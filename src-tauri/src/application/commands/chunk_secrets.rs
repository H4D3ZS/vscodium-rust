//! Tauri IPC for Rust-native security scanning (chunks, XSS probe, bounty stack).

use crate::chunk_secrets::{scan_directory, scan_url, ChunkScanSummary};
use crate::security_native::{bounty_scan_url, xss_probe_url, BountyScanSummary, XssProbeSummary};
use std::path::PathBuf;
use std::sync::OnceLock;

fn http_client() -> &'static reqwest::Client {
    static CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
    CLIENT.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(8))
            .pool_max_idle_per_host(8)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

#[tauri::command]
pub async fn chunk_secrets_scan_path(
    path: String,
    max_files: Option<usize>,
) -> Result<ChunkScanSummary, String> {
    let root = PathBuf::from(&path);
    let cap = max_files.unwrap_or(2_000);
    scan_directory(&root, cap)
}

#[tauri::command]
pub async fn chunk_secrets_scan_url(url: String) -> Result<ChunkScanSummary, String> {
    scan_url(&url, http_client()).await
}

#[tauri::command]
pub async fn security_xss_probe_url(url: String) -> Result<XssProbeSummary, String> {
    xss_probe_url(&url, http_client()).await
}

#[tauri::command]
pub async fn security_bounty_scan_url(
    url: String,
    include_xss: Option<bool>,
) -> Result<BountyScanSummary, String> {
    bounty_scan_url(&url, http_client(), include_xss.unwrap_or(true)).await
}

#[tauri::command]
pub fn security_native_tools() -> serde_json::Value {
    serde_json::json!({
        "tools": [
            {
                "id": "chunk-secret-scanner",
                "name": "JS Chunk Secret Scanner",
                "native_rust": true,
                "replaces": ["hawk-eye", "trufflehog-lite"],
                "commands": ["chunk_secrets_scan_path", "chunk_secrets_scan_url"]
            },
            {
                "id": "xss-probe-native",
                "name": "Native XSS Reflection Probe",
                "native_rust": true,
                "replaces": ["dalfox-xss-lite"],
                "commands": ["security_xss_probe_url"]
            },
            {
                "id": "vega-dast",
                "name": "Vega DAST Engine",
                "native_rust": true,
                "replaces": ["zap-lite"],
                "commands": ["vega_scan"]
            },
            {
                "id": "bounty-stack",
                "name": "Combined Bounty Scan",
                "native_rust": true,
                "commands": ["security_bounty_scan_url"]
            }
        ],
        "external": [
            { "id": "moxy-dast", "requires": ["docker"], "note": "Full agentic MITM — use when Docker available" },
            { "id": "hetty-proxy", "requires": ["go"], "note": "Burp alternative — build from source or brew install hetty" },
            { "id": "dissectx-pro", "requires": ["python3", "pip", "npm"], "note": "Red team phishing suite" },
            { "id": "flutter-sentinel", "requires": ["npm"], "note": "Mobile RE for Flutter/RN" },
            { "id": "dalfox-xss", "requires": ["go"], "note": "Optional; native XSS probe built-in" },
            { "id": "nuclei-templates", "requires": ["go"], "note": "CVE templates at scale" }
        ]
    })
}
