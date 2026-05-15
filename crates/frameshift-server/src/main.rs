//! Entry point for the `frameshift-server` binary.
//!
//! Parses configuration from environment variables, initializes tracing, wires
//! up the Postgres catalog adapter and filesystem object store, and calls
//! [`frameshift_server::run`] to serve until SIGTERM/SIGINT.

use std::sync::Arc;
use std::time::Duration;

use mimalloc::MiMalloc;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;

use frameshift_catalog_postgres::{PostgresCatalog, PostgresCatalogConfig};
use frameshift_objects_fs::{FsPackStore, FsPackStoreConfig};
use frameshift_server::{AppState, LogFormat, ServerConfig, ServerError};

/// Use mimalloc as the global allocator for improved throughput on
/// allocation-heavy workloads (many small async tasks).
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Initialize the `tracing` subscriber based on the resolved [`ServerConfig`].
///
/// Applies an [`tracing_subscriber::EnvFilter`] from `config.log_level`.
/// Falls back to `info` if the level string is invalid. Emits either
/// structured JSON or compact text output depending on `config.log_format`.
fn init_tracing(config: &ServerConfig) {
    let env_filter = tracing_subscriber::EnvFilter::try_new(&config.log_level)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"));

    let registry = tracing_subscriber::registry().with(env_filter);

    match config.log_format {
        LogFormat::Json => registry
            .with(tracing_subscriber::fmt::layer().json())
            .init(),
        LogFormat::Text => registry.with(tracing_subscriber::fmt::layer()).init(),
    }
}

/// Build [`AppState`] by initializing the Postgres catalog and filesystem
/// object store from the resolved config.
///
/// Both backends are initialized before the TCP socket is bound so that startup
/// errors (bad connection string, unwritable directory) are surfaced immediately
/// as `ServerError::Startup` rather than causing runtime failures after the
/// server is already accepting connections.
async fn build_state(config: Arc<ServerConfig>) -> Result<AppState, ServerError> {
    use secrecy::ExposeSecret as _;

    let catalog_config = PostgresCatalogConfig {
        url: secrecy::SecretString::new(config.postgres_url.expose_secret().to_string()),
        pool_size: 10,
        connect_timeout: Duration::from_secs(5),
        statement_timeout: Duration::from_secs(30),
    };

    let catalog = PostgresCatalog::new(catalog_config)
        .await
        .map_err(|e| ServerError::Startup(e.to_string()))?;

    let objects_config = FsPackStoreConfig {
        root: config.object_store_root.clone(),
        verify_on_read: true,
        max_bytes: None,
        fsync_on_put: true,
    };

    let objects = FsPackStore::new(objects_config)
        .await
        .map_err(|e| ServerError::Startup(e.to_string()))?;

    Ok(AppState {
        catalog: Arc::new(catalog),
        objects: Arc::new(objects),
        runtime: None,
        config,
    })
}

#[tokio::main]
async fn main() {
    let config = match ServerConfig::from_env() {
        Ok(c) => Arc::new(c),
        Err(e) => {
            eprintln!("configuration error: {e}");
            std::process::exit(2);
        }
    };
    // Note: `from_env` returns `Box<figment::Error>` to avoid large Err variants.

    init_tracing(&config);
    tracing::debug!(?config, "resolved server configuration");

    let state = match build_state(Arc::clone(&config)).await {
        Ok(s) => s,
        Err(e) => {
            tracing::error!("startup failed: {e}");
            std::process::exit(3);
        }
    };

    if let Err(e) = frameshift_server::run(state).await {
        tracing::error!("server error: {e}");
        let code = match e {
            ServerError::Bind(_) => 2,
            ServerError::Startup(_) => 3,
            ServerError::Shutdown(_) => 1,
        };
        std::process::exit(code);
    }
}

