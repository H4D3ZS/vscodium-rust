//! Tauri IPC for APEX Vega DAST campaigns.

use crate::vega::campaign::{list_modules, run_campaign, VegaModuleInfo, VegaScanOptions, VegaScanResult};
use crate::vega::{report, Alert};

#[tauri::command]
pub fn vega_list_modules() -> Vec<VegaModuleInfo> {
    list_modules()
}

#[tauri::command]
pub async fn vega_scan(options: VegaScanOptions) -> Result<VegaScanResult, String> {
    run_campaign(options).await
}

/// Export a completed scan result to SARIF 2.1.0 (`sarif`) or Markdown (`markdown`).
/// Fully offline and deterministic — no model required.
#[tauri::command]
pub fn vega_export_report(
    target: String,
    alerts: Vec<Alert>,
    triage: Option<Vec<String>>,
    format: String,
) -> Result<String, String> {
    let triage = triage.unwrap_or_default();
    match format.as_str() {
        "sarif" => serde_json::to_string_pretty(&report::to_sarif(&target, &alerts, &triage))
            .map_err(|e| e.to_string()),
        "markdown" | "md" => Ok(report::to_markdown(&target, &alerts, &triage)),
        other => Err(format!("unknown report format: {other}")),
    }
}

#[tauri::command]
pub fn vega_disciplines() -> serde_json::Value {
    serde_json::json!({
        "disciplines": [
            {
                "id": "web-pentest",
                "label": "Web Penetration Testing",
                "icon": "globe",
                "native": ["vega-dast", "chunk-secret-scanner", "zero-day-hunter"],
                "external": ["moxy-dast", "hetty-proxy", "dalfox-xss"]
            },
            {
                "id": "red-team",
                "label": "Red Team & Phishing",
                "icon": "flame",
                "native": ["security-arsenal"],
                "external": ["dissectx-pro"]
            },
            {
                "id": "malware-re",
                "label": "Malware Reverse Engineering",
                "icon": "circuit-board",
                "native": ["code-review", "secrets-scan"],
                "external": ["exploit-db-search"]
            },
            {
                "id": "mobile-pentest",
                "label": "Mobile Penetration Testing",
                "icon": "device-mobile",
                "native": [],
                "external": ["flutter-sentinel", "frida-runner", "mobsf-integration", "apk-analyzer"]
            }
        ]
    })
}
