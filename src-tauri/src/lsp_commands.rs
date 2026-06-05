use crate::EditorState;
use tauri::{State, AppHandle, Emitter};
use serde_json::{Value, json};

#[tauri::command]
pub async fn lsp_start(
    state: State<'_, EditorState>,
    app: AppHandle,
    command: String,
    args: Option<Vec<String>>,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    let a = args.unwrap_or_default();
    lsp.start(&command, &a, app).map_err(|e| e.to_string())
}

/// Auto-detect and start a language server for the active workspace root.
#[tauri::command]
pub async fn lsp_auto_start(
    state: State<'_, EditorState>,
    app: AppHandle,
    root: String,
) -> Result<Value, String> {
    let root_path = std::path::PathBuf::from(&root);
    if !root_path.is_dir() {
        return Err(format!("Workspace root not found: {root}"));
    }

    {
        let lsp = state.lsp_client.lock().await;
        if lsp.is_running() {
            return Ok(json!({ "status": "already_running" }));
        }
    }

    let config_dir = state.config_dir.clone();
    let _ = app.emit("lsp-bundle-progress", json!({ "phase": "ensure", "message": "Preparing IDE language server…" }));

    let spec = crate::lsp_manager::detect_workspace_lsp_async(&root_path, &config_dir)
        .await
        .map_err(|e| format!("{e} (run scripts/fetch-lsp-binaries.ps1 for offline TS/Python bundles)"))?;

    {
        let mut lsp = state.lsp_client.lock().await;
        lsp.start(&spec.command, &spec.args, app.clone())
            .map_err(|e| e.to_string())?;
    }

    // Give the server a moment to answer initialize before initialized notification.
    tokio::time::sleep(std::time::Duration::from_millis(400)).await;

    {
        let mut lsp = state.lsp_client.lock().await;
        lsp.send_initialized().map_err(|e| e.to_string())?;
        let root_uri = if root.starts_with('/') {
            format!("file://{root}")
        } else {
            format!("file:///{}", root.replace('\\', "/"))
        };
        lsp.set_workspace_root(&root_uri).map_err(|e| e.to_string())?;
    }

    Ok(json!({
        "status": "started",
        "id": spec.id,
        "command": spec.command,
        "args": spec.args,
        "managed": true,
    }))
}

#[tauri::command]
pub async fn lsp_bundle_status(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(crate::lsp_bundle::bundle_status(&state.config_dir))
}

#[tauri::command]
pub async fn lsp_ensure_bundle(
    state: State<'_, EditorState>,
    root: String,
) -> Result<Value, String> {
    let root_path = std::path::PathBuf::from(&root);
    let launch = crate::lsp_bundle::ensure_workspace_lsp(&root_path, &state.config_dir).await?;
    Ok(json!({
        "id": launch.id,
        "command": launch.command,
        "source": launch.source,
        "installed": true,
    }))
}

#[tauri::command]
pub async fn lsp_send_request(
    state: State<'_, EditorState>,
    id: i32,
    method: String,
    params: Value,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.send_request(id, &method, params)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.stop();
    Ok(())
}

#[tauri::command]
pub async fn lsp_initialized(state: State<'_, EditorState>) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.send_initialized().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_did_open(
    state: State<'_, EditorState>,
    uri: String,
    language_id: String,
    version: i32,
    text: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.did_open(&uri, &language_id, version, &text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_did_change(
    state: State<'_, EditorState>,
    uri: String,
    version: i32,
    text: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.did_change(&uri, version, &text)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_did_save(state: State<'_, EditorState>, uri: String) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.did_save(&uri).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_set_workspace(
    state: State<'_, EditorState>,
    root_uri: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.set_workspace_root(&root_uri).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_change_workspace_folders(
    state: State<'_, EditorState>,
    folders: Vec<Value>,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    if !lsp.is_running() {
        return Ok(());
    }
    let pairs: Vec<(String, String)> = folders
        .iter()
        .filter_map(|f| {
            let uri = f.get("uri")?.as_str()?.to_string();
            let name = f.get("name").and_then(|n| n.as_str()).unwrap_or("workspace").to_string();
            Some((uri, name))
        })
        .collect();
    let refs: Vec<(&str, &str)> = pairs.iter().map(|(u, n)| (u.as_str(), n.as_str())).collect();
    lsp.sync_workspace_folders(&refs, &[]).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_get_diagnostics(
    state: State<'_, EditorState>,
    path: Option<String>,
) -> Result<Value, String> {
    let diags = state.lsp_diagnostics.read().await;
    let result: Vec<Value> = diags.iter()
        .filter(|(uri, _)| {
            path.as_deref().map(|p| uri.contains(p)).unwrap_or(true)
        })
        .map(|(uri, items)| json!({ "uri": uri, "diagnostics": items }))
        .collect();
    Ok(json!(result))
}

#[tauri::command]
pub async fn lsp_is_running(state: State<'_, EditorState>) -> Result<bool, String> {
    let lsp = state.lsp_client.lock().await;
    Ok(lsp.is_running())
}

#[tauri::command]
pub async fn lsp_completion(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!({ "items": [] }));
    }
    let result = client.request_with_response("textDocument/completion", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character },
        "context": { "triggerKind": 1 }
    })).await.map_err(|e| e.to_string())?;

    let items = result["result"]["items"].as_array()
        .cloned()
        .or_else(|| result["result"].as_array().cloned())
        .unwrap_or_default();
    Ok(json!({ "items": items }))
}

#[tauri::command]
pub async fn lsp_hover(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!(null));
    }
    let result = client.request_with_response("textDocument/hover", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_goto_definition(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!(null));
    }
    let result = client.request_with_response("textDocument/definition", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_find_references(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!([]));
    }
    let result = client.request_with_response("textDocument/references", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character },
        "context": { "includeDeclaration": true }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_rename_symbol(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
    new_name: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!(null));
    }
    let result = client.request_with_response("textDocument/rename", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character },
        "newName": new_name
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_format_document(state: State<'_, EditorState>, uri: String) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!(null));
    }
    let result = client.request_with_response("textDocument/formatting", json!({
        "textDocument": { "uri": uri },
        "options": { "tabSize": 4, "insertSpaces": true }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_workspace_symbols(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!([]));
    }
    let result = client.request_with_response("workspace/symbol", json!({
        "query": query
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_code_lens(
    state: State<'_, EditorState>,
    uri: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() { return Ok(json!([])); }
    let result = client.request_with_response("textDocument/codeLens", json!({
        "textDocument": { "uri": uri }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_document_symbols(
    state: State<'_, EditorState>,
    uri: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Ok(json!([]));
    }
    let result = client.request_with_response("textDocument/documentSymbol", json!({
        "textDocument": { "uri": uri }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}
