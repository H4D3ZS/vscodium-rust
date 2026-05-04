use crate::EditorState;
use tauri::{State, AppHandle};
use serde_json::{Value, json};

#[tauri::command]
pub async fn lsp_start(
    state: State<'_, EditorState>,
    app: AppHandle,
    command: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.start(&command, app).map_err(|e| e.to_string())
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
