use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersonaSummary {
    pub name: String,
    pub description: String,
    pub version: String,
    pub active: bool,
    pub capabilities: Vec<String>,
    pub installed_at: String,
}

#[tauri::command]
pub fn list_personas() -> Result<Vec<PersonaSummary>, String> {
    // Placeholder data -- will wire to frameshift-client when workspace is set up
    let personas = vec![
        PersonaSummary {
            name: "security".to_string(),
            description: "Security-focused persona with threat modeling and audit capabilities".to_string(),
            version: "0.3.1".to_string(),
            active: true,
            capabilities: vec![
                "threat-model".to_string(),
                "audit".to_string(),
                "vuln-scan".to_string(),
            ],
            installed_at: "2026-05-01T00:00:00Z".to_string(),
        },
        PersonaSummary {
            name: "cryptographic".to_string(),
            description: "Cryptographic systems expert -- key management, protocol design".to_string(),
            version: "0.2.0".to_string(),
            active: false,
            capabilities: vec![
                "key-derivation".to_string(),
                "protocol-review".to_string(),
            ],
            installed_at: "2026-05-05T00:00:00Z".to_string(),
        },
        PersonaSummary {
            name: "systems".to_string(),
            description: "Low-level systems programming, kernel interfaces, memory safety".to_string(),
            version: "0.4.2".to_string(),
            active: false,
            capabilities: vec![
                "memory-analysis".to_string(),
                "perf-profiling".to_string(),
                "kernel-debug".to_string(),
            ],
            installed_at: "2026-04-20T00:00:00Z".to_string(),
        },
        PersonaSummary {
            name: "frontend".to_string(),
            description: "Frontend engineering -- React, accessibility, performance".to_string(),
            version: "0.1.8".to_string(),
            active: false,
            capabilities: vec![
                "a11y-audit".to_string(),
                "bundle-analysis".to_string(),
            ],
            installed_at: "2026-05-10T00:00:00Z".to_string(),
        },
    ];

    Ok(personas)
}

#[tauri::command]
pub fn active_persona() -> Result<Option<String>, String> {
    // Returns the name of the currently active persona
    Ok(Some("security".to_string()))
}

#[tauri::command]
pub fn activate_persona(name: String) -> Result<(), String> {
    // Placeholder -- will call frameshift-client to swap active persona
    if name.is_empty() {
        return Err("persona name cannot be empty".to_string());
    }
    // TODO: wire to frameshift-client::PersonaManager::activate
    Ok(())
}

#[tauri::command]
pub fn install_persona(name: String, source: String) -> Result<(), String> {
    // Placeholder -- will fetch and install a persona from source
    if name.is_empty() {
        return Err("persona name cannot be empty".to_string());
    }
    if source.is_empty() {
        return Err("source cannot be empty".to_string());
    }
    // TODO: wire to frameshift-client::PersonaInstaller::install
    Ok(())
}
