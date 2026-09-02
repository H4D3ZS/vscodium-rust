//! Tauri IPC for security generators.

use serde_json::{json, Value};
use tauri::State;

use crate::security_generators::{
    analyze_csp, encode_payload, listener_config, reverse_shell, shellcode_recipe,
};

/// Security lens: per-line vulnerability findings for live inline decoration.
/// Fast enough for the keystroke path — the frontend debounces and renders the
/// results as Monaco markers (gutter + squiggle). This is a beyond-Cursor moat:
/// Cursor has no native as-you-type security overlay.
#[tauri::command]
pub async fn security_lens_scan(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    content: String,
) -> Result<Value, String> {
    let findings = crate::security_distiller::SecurityDistiller::scan_lines(&content);
    let critical = findings.iter().filter(|f| f.severity == "CRITICAL").count();
    let high = findings.iter().filter(|f| f.severity == "HIGH").count();
    Ok(json!({
        "findings": findings,
        "total": findings.len(),
        "critical": critical,
        "high": high,
    }))
}

#[tauri::command]
pub async fn security_reverse_shell(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    language: String,
    host: String,
    port: u16,
    shell: Option<String>,
) -> Result<Value, String> {
    let payload = reverse_shell(&language, &host, port, shell.as_deref())?;
    Ok(json!({ "language": language, "host": host, "port": port, "payload": payload }))
}

#[tauri::command]
pub async fn security_listener(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    kind: String,
    host: String,
    port: u16,
) -> Result<Value, String> {
    let cmd = listener_config(&kind, &host, port)?;
    Ok(json!({ "kind": kind, "command": cmd }))
}

#[tauri::command]
pub async fn security_csp_analyze(_state: State<'_, std::sync::Arc<crate::EditorState>>, header: String) -> Result<Value, String> {
    Ok(analyze_csp(&header))
}

#[tauri::command]
pub async fn security_shellcode_recipe(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    platform: String,
    arch: String,
    payload: Option<String>,
) -> Result<Value, String> {
    Ok(shellcode_recipe(
        &platform,
        &arch,
        payload.as_deref().unwrap_or("shell_reverse_tcp"),
    ))
}

#[tauri::command]
pub async fn security_encode_payload(
    _state: State<'_, std::sync::Arc<crate::EditorState>>,
    payload: String,
    encoding: String,
) -> Result<Value, String> {
    encode_payload(&payload, &encoding)
}
