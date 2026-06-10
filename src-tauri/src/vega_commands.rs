//! Tauri IPC for APEX Vega DAST campaigns.

use crate::vega::campaign::{list_modules, run_campaign, VegaModuleInfo, VegaScanOptions, VegaScanResult};

#[tauri::command]
pub fn vega_list_modules() -> Vec<VegaModuleInfo> {
    list_modules()
}

#[tauri::command]
pub async fn vega_scan(options: VegaScanOptions) -> Result<VegaScanResult, String> {
    run_campaign(options).await
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
