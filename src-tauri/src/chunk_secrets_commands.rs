//! Tauri IPC for JS chunk / bundle secret scanning.

use crate::chunk_secrets::{scan_directory, scan_url, ChunkScanSummary};
use std::path::PathBuf;

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
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(8))
        .build()
        .map_err(|e| e.to_string())?;
    scan_url(&url, &client).await
}
