//! Frameshift MCP server: reads JSON-RPC from stdin, writes to stdout.
//! Tracing output goes to stderr to avoid corrupting the MCP protocol.

use frameshift_client::Client;
use frameshift_mcp::protocol::{error_response, success_response, JsonRpcMessage, JsonRpcResponse};
use frameshift_mcp::tools;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter};

/// Main entry point. Initializes tracing, creates the client, then
/// runs the stdin JSON-RPC read loop writing responses to stdout.
#[tokio::main]
async fn main() {
    // Tracing to stderr -- stdout is reserved for MCP protocol
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let client = Client::with_default_data_root().expect("failed to initialize client");

    let reader = BufReader::new(tokio::io::stdin());
    let mut stdout = BufWriter::new(tokio::io::stdout());
    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        if let Some(response) = handle_message(&line, &client) {
            let json = serde_json::to_string(&response).unwrap_or_default();
            let _ = stdout.write_all(json.as_bytes()).await;
            let _ = stdout.write_all(b"\n").await;
            let _ = stdout.flush().await;
        }
    }
}

/// Handle a single JSON-RPC message line.
///
/// Returns None for notifications (no id present) so no response is written.
/// Returns Some(response) for requests that require a reply.
fn handle_message(line: &str, client: &Client) -> Option<JsonRpcResponse> {
    let msg: JsonRpcMessage = match serde_json::from_str(line) {
        Ok(m) => m,
        Err(e) => {
            return Some(error_response(None, -32700, format!("parse error: {e}")));
        }
    };

    // Notifications have no id -- do not respond to them.
    if msg.id.is_none() {
        return None;
    }

    let id = msg.id.clone();

    match msg.method.as_str() {
        "initialize" => {
            let result = serde_json::json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": {
                    "name": "frameshift-mcp",
                    "version": "0.1.0"
                },
                "capabilities": {
                    "tools": {}
                }
            });
            Some(success_response(id, result))
        }
        "tools/list" => {
            let defs = tools::tool_definitions();
            let result = serde_json::json!({"tools": defs});
            Some(success_response(id, result))
        }
        "tools/call" => {
            let params = msg.params.unwrap_or(serde_json::Value::Null);
            let name = params.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let arguments = params
                .get("arguments")
                .cloned()
                .unwrap_or(serde_json::Value::Object(Default::default()));
            let tool_result = tools::call_tool(name, &arguments, client);
            Some(success_response(
                id,
                serde_json::to_value(tool_result).unwrap_or_default(),
            ))
        }
        _ => Some(error_response(
            id,
            -32601,
            format!("method not found: {}", msg.method),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use frameshift_client::{Client, ClientOptions, InstallRequest, InstallSource, PersonaSpec};
    use std::fs;

    /// Build a Client backed by a temporary data root.
    fn make_client(data_root: &std::path::Path) -> Client {
        Client::new(ClientOptions {
            data_root: data_root.to_path_buf(),
            config_root: None,
        })
    }

    /// Verify that a JSON notification (no id field) produces no response.
    #[test]
    fn notification_returns_none() {
        let tmp = tempfile::tempdir().unwrap();
        let client = make_client(tmp.path());
        // A notification has no "id" field.
        let line = r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#;
        let result = handle_message(line, &client);
        assert!(result.is_none(), "notifications must not produce a response");
    }

    /// Verify that an initialize request returns serverInfo.name.
    #[test]
    fn initialize_returns_server_info() {
        let tmp = tempfile::tempdir().unwrap();
        let client = make_client(tmp.path());
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}"#;
        let response = handle_message(line, &client).expect("should produce a response");
        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized["result"]["serverInfo"]["name"], "frameshift-mcp");
    }

    /// Verify that an unknown method returns a -32601 error.
    #[test]
    fn unknown_method_returns_error() {
        let tmp = tempfile::tempdir().unwrap();
        let client = make_client(tmp.path());
        let line = r#"{"jsonrpc":"2.0","id":2,"method":"bogus/method"}"#;
        let response = handle_message(line, &client).expect("should produce a response");
        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized["error"]["code"], -32601);
    }

    /// Verify that tools/list returns the expected seven tool names.
    #[test]
    fn tools_list_returns_four_tools() {
        let tmp = tempfile::tempdir().unwrap();
        let client = make_client(tmp.path());
        let line = r#"{"jsonrpc":"2.0","id":3,"method":"tools/list"}"#;
        let response = handle_message(line, &client).expect("should produce a response");
        let serialized = serde_json::to_value(&response).unwrap();
        let tools = serialized["result"]["tools"].as_array().unwrap();
        assert_eq!(tools.len(), 7);
    }

    /// Verify tools/call with frameshift_install succeeds end-to-end.
    #[test]
    fn tools_call_install_end_to_end() {
        let tmp = tempfile::tempdir().unwrap();
        let data_root = tmp.path().join("data");
        let pack_dir = tmp.path().join("pack");
        fs::create_dir_all(&pack_dir).unwrap();
        fs::write(
            pack_dir.join("pack.toml"),
            "schema_version = 1\nname = \"mcp-test\"\nversion = \"0.1.0\"\nauthor_handle = \"test\"\nauthor_pubkey = \"local-unsigned\"\n",
        )
        .unwrap();
        fs::write(pack_dir.join("AGENTS.md"), "# MCP Test\n").unwrap();

        let project_root = tmp.path().join("project");
        fs::create_dir_all(&project_root).unwrap();

        let client = make_client(&data_root);

        let args = serde_json::json!({
            "name": "frameshift_install",
            "arguments": {
                "spec": "mcp-test@0.1.0",
                "project_root": project_root.to_str().unwrap(),
                "from_path": pack_dir.to_str().unwrap()
            }
        });
        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 10,
            "method": "tools/call",
            "params": args
        });
        let response = handle_message(&msg.to_string(), &client).expect("should produce a response");
        let serialized = serde_json::to_value(&response).unwrap();
        // Should be a success result (no error field).
        assert!(serialized.get("error").is_none() || serialized["error"].is_null());
        let content = &serialized["result"]["content"][0]["text"];
        assert!(content.as_str().unwrap().contains("mcp-test@0.1.0"));
    }

    /// Verify that a malformed JSON line produces a parse error response.
    #[test]
    fn malformed_json_returns_parse_error() {
        let tmp = tempfile::tempdir().unwrap();
        let client = make_client(tmp.path());
        let response = handle_message("not json {{{{", &client).expect("should produce error response");
        let serialized = serde_json::to_value(&response).unwrap();
        assert_eq!(serialized["error"]["code"], -32700);
    }

    /// Verify grow_append integration through the full message handler.
    #[test]
    fn tools_call_grow_append_integration() {
        let tmp = tempfile::tempdir().unwrap();
        let data_root = tmp.path().join("data");
        let pack_dir = tmp.path().join("pack");
        fs::create_dir_all(&pack_dir).unwrap();
        fs::write(
            pack_dir.join("pack.toml"),
            "schema_version = 1\nname = \"growpersona\"\nversion = \"0.1.0\"\nauthor_handle = \"test\"\nauthor_pubkey = \"local-unsigned\"\n",
        )
        .unwrap();
        fs::write(pack_dir.join("AGENTS.md"), "# Grow Persona\n").unwrap();

        let project_root = tmp.path().join("project");
        fs::create_dir_all(&project_root).unwrap();

        let client = make_client(&data_root);

        // Install persona first so growth dir exists.
        client
            .install(InstallRequest {
                project_root: project_root.clone(),
                spec: PersonaSpec {
                    name: "growpersona".to_string(),
                    version: "0.1.0".to_string(),
                },
                source: InstallSource::LocalPath(pack_dir),
            })
            .unwrap();

        let msg = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 20,
            "method": "tools/call",
            "params": {
                "name": "frameshift_grow_append",
                "arguments": {
                    "project_root": project_root.to_str().unwrap(),
                    "persona": "growpersona",
                    "text": "Learned something useful."
                }
            }
        });

        let response = handle_message(&msg.to_string(), &client).expect("should produce a response");
        let serialized = serde_json::to_value(&response).unwrap();
        assert!(serialized.get("error").is_none() || serialized["error"].is_null());
        let content_text = serialized["result"]["content"][0]["text"].as_str().unwrap();
        let parsed: serde_json::Value = serde_json::from_str(content_text).unwrap();
        assert_eq!(parsed["appended"], true);
    }
}
