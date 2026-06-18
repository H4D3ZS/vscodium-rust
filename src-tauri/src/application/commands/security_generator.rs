//! Tauri IPC for security generators.

use serde_json::{json, Value};
use tauri::State;

use crate::security_generators::{
    analyze_csp, encode_payload, listener_config, reverse_shell, shellcode_recipe,
};
use crate::EditorState;

#[tauri::command]
pub async fn security_reverse_shell(
    state: State<'_, EditorState>,
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
    state: State<'_, EditorState>,
    kind: String,
    host: String,
    port: u16,
) -> Result<Value, String> {
    let cmd = listener_config(&kind, &host, port)?;
    Ok(json!({ "kind": kind, "command": cmd }))
}

#[tauri::command]
pub async fn security_csp_analyze(state: State<'_, EditorState>, header: String) -> Result<Value, String> {
    Ok(analyze_csp(&header))
}

#[tauri::command]
pub async fn security_shellcode_recipe(
    state: State<'_, EditorState>,
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
    state: State<'_, EditorState>,
    payload: String,
    encoding: String,
) -> Result<Value, String> {
    encode_payload(&payload, &encoding)
}
