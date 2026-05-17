use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowthEntry {
    pub timestamp: String,
    pub event: String,
    pub delta: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GrowthReport {
    pub persona: String,
    pub total_sessions: u32,
    pub total_tokens_processed: u64,
    pub capability_scores: Vec<CapabilityScore>,
    pub log: Vec<GrowthEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CapabilityScore {
    pub capability: String,
    pub score: f32,
    pub delta_7d: f32,
}

#[tauri::command]
pub fn get_growth(name: String) -> Result<String, String> {
    // Returns growth data as JSON string -- will wire to frameshift-client
    if name.is_empty() {
        return Err("persona name cannot be empty".to_string());
    }

    let report = GrowthReport {
        persona: name.clone(),
        total_sessions: 47,
        total_tokens_processed: 1_284_930,
        capability_scores: vec![
            CapabilityScore {
                capability: "threat-model".to_string(),
                score: 0.82,
                delta_7d: 0.04,
            },
            CapabilityScore {
                capability: "audit".to_string(),
                score: 0.74,
                delta_7d: 0.02,
            },
            CapabilityScore {
                capability: "vuln-scan".to_string(),
                score: 0.68,
                delta_7d: -0.01,
            },
        ],
        log: vec![
            GrowthEntry {
                timestamp: "2026-05-17T10:00:00Z".to_string(),
                event: "session completed -- threat model review".to_string(),
                delta: 0.02,
            },
            GrowthEntry {
                timestamp: "2026-05-16T14:30:00Z".to_string(),
                event: "capability unlocked: advanced-audit".to_string(),
                delta: 0.05,
            },
            GrowthEntry {
                timestamp: "2026-05-15T09:15:00Z".to_string(),
                event: "session completed -- CVE analysis".to_string(),
                delta: 0.01,
            },
        ],
    };

    serde_json::to_string(&report).map_err(|e| e.to_string())
}
