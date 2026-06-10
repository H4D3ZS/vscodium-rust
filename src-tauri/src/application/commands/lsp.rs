use crate::EditorState;
use tauri::{AppHandle, Emitter, State};
use serde_json::{json, Value};

fn root_uri(root: &str) -> String {
    if root.starts_with('/') {
        format!("file://{root}")
    } else {
        format!("file:///{}", root.replace('\\', "/"))
    }
}

#[tauri::command]
pub async fn lsp_start(
    state: State<'_, EditorState>,
    app: AppHandle,
    command: String,
    args: Option<Vec<String>>,
) -> Result<(), String> {
    let launch = crate::lsp_bundle::ResolvedLaunch {
        id: "manual".into(),
        command,
        args: args.unwrap_or_default(),
        source: "manual".into(),
    };
    let root = std::env::current_dir()
        .unwrap_or_else(|_| state.config_dir.clone())
        .to_string_lossy()
        .to_string();
    let mut router = state.lsp_router.lock().await;
    router
        .ensure_server("manual", &launch, &root, app)
        .await?;
    Ok(())
}

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

    let _ = app.emit(
        "lsp-bundle-progress",
        json!({ "phase": "ensure", "message": "Preparing IDE language server…" }),
    );

    let config_dir = state.config_dir.clone();
    let spec = crate::lsp_manager::detect_workspace_lsp_async(&root_path, &config_dir)
        .await
        .map_err(|e| format!("{e} (run scripts/fetch-lsp-binaries.ps1 for offline bundles)"))?;

    let launch = crate::lsp_bundle::ResolvedLaunch {
        id: spec.id.clone(),
        command: spec.command,
        args: spec.args,
        source: "bundled".into(),
    };

    let mut router = state.lsp_router.lock().await;
    router.ensure_server(&launch.id, &launch, &root, app).await?;

    Ok(json!({
        "status": "started",
        "id": launch.id,
        "command": launch.command,
        "managed": true,
    }))
}

#[tauri::command]
pub async fn lsp_ensure_for_file(
    state: State<'_, EditorState>,
    app: AppHandle,
    root: String,
    path: String,
    language_id: String,
    version: i32,
    text: String,
) -> Result<Value, String> {
    let root_path = std::path::PathBuf::from(&root);
    let file_path = std::path::PathBuf::from(&path);
    let mut router = state.lsp_router.lock().await;
    router
        .ensure_for_file(
            &file_path,
            &root_path,
            &state.config_dir,
            &language_id,
            version,
            &text,
            app,
        )
        .await
}

#[tauri::command]
pub async fn lsp_detect_workspace(
    state: State<'_, EditorState>,
    root: String,
) -> Result<Value, String> {
    let root_path = std::path::PathBuf::from(&root);
    Ok(crate::lsp_router::detect_workspace_lsp_json(
        &root_path,
        &state.config_dir,
    ))
}

#[tauri::command]
pub async fn lsp_start_server(
    state: State<'_, EditorState>,
    app: AppHandle,
    root: String,
    server_id: String,
) -> Result<Value, String> {
    let root_path = std::path::PathBuf::from(&root);
    let spec = crate::lsp_store::resolve_launch_by_server_id(
        &server_id,
        &state.config_dir,
        Some(&root_path),
    )
    .ok_or_else(|| {
        if crate::lsp_store::find_user_server(&server_id)
            .map(|r| !r.enabled)
            .unwrap_or(false)
        {
            format!(
                "Language server '{server_id}' is disabled. Enable it in Settings → Language Servers."
            )
        } else {
            format!("Could not launch language server: {server_id}")
        }
    })?;

    let mut router = state.lsp_router.lock().await;
    router
        .ensure_server(&spec.id, &spec, &root, app)
        .await?;

    Ok(json!({
        "status": "started",
        "id": spec.id,
        "command": spec.command,
        "args": spec.args,
    }))
}

#[tauri::command]
pub async fn lsp_bundle_status(state: State<'_, EditorState>) -> Result<Value, String> {
    let mut status = crate::lsp_bundle::bundle_status(&state.config_dir);
    let router = state.lsp_router.lock().await;
    if let Some(obj) = status.as_object_mut() {
        obj.insert(
            "userServers".into(),
            json!(crate::lsp_store::list_user_servers()),
        );
        obj.insert("pool".into(), router.pool_status());
        obj.insert(
            "running".into(),
            json!(router.running_server_ids().await),
        );
    }
    Ok(status)
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
    let uri = params
        .get("textDocument")
        .and_then(|t| t.get("uri"))
        .and_then(|u| u.as_str())
        .unwrap_or("")
        .to_string();
    let router = state.lsp_router.lock().await;
    let client = router
        .client_for_uri(&uri)
        .await
        .ok_or_else(|| "No LSP client for document".to_string())?;
    let mut guard = client.lock().await;
    guard
        .send_request(id, &method, params)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn lsp_stop(state: State<'_, EditorState>) -> Result<(), String> {
    state.lsp_router.lock().await.stop_all().await;
    Ok(())
}

#[tauri::command]
pub async fn lsp_initialized(_state: State<'_, EditorState>) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn lsp_did_open(
    state: State<'_, EditorState>,
    uri: String,
    language_id: String,
    version: i32,
    text: String,
) -> Result<(), String> {
    state
        .lsp_router
        .lock()
        .await
        .did_open(&uri, &language_id, version, &text)
        .await
}

#[tauri::command]
pub async fn lsp_did_change(
    state: State<'_, EditorState>,
    uri: String,
    version: i32,
    text: String,
) -> Result<(), String> {
    state
        .lsp_router
        .lock()
        .await
        .did_change(&uri, version, &text)
        .await
}

#[tauri::command]
pub async fn lsp_did_save(state: State<'_, EditorState>, uri: String) -> Result<(), String> {
    state.lsp_router.lock().await.did_save(&uri).await
}

#[tauri::command]
pub async fn lsp_set_workspace(
    state: State<'_, EditorState>,
    root_uri: String,
) -> Result<(), String> {
    state
        .lsp_router
        .lock()
        .await
        .set_workspace(&root_uri)
        .await
}

#[tauri::command]
pub async fn lsp_change_workspace_folders(
    state: State<'_, EditorState>,
    folders: Vec<Value>,
) -> Result<(), String> {
    let pairs: Vec<(String, String)> = folders
        .iter()
        .filter_map(|f| {
            let uri = f.get("uri")?.as_str()?.to_string();
            let name = f
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("workspace")
                .to_string();
            Some((uri, name))
        })
        .collect();
    state
        .lsp_router
        .lock()
        .await
        .change_workspace_folders(pairs, &[])
        .await
}

#[tauri::command]
pub async fn lsp_get_diagnostics(
    state: State<'_, EditorState>,
    path: Option<String>,
) -> Result<Value, String> {
    let diags = state.lsp_diagnostics.read().await;
    let result: Vec<Value> = diags
        .iter()
        .filter(|(uri, _)| path.as_deref().map(|p| uri.contains(p)).unwrap_or(true))
        .map(|(uri, items)| json!({ "uri": uri, "diagnostics": items }))
        .collect();
    Ok(json!(result))
}

#[tauri::command]
pub async fn lsp_is_running(state: State<'_, EditorState>) -> Result<bool, String> {
    Ok(state.lsp_router.lock().await.is_any_running().await)
}

#[tauri::command]
pub async fn lsp_completion(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/completion",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character },
                "context": { "triggerKind": 1 }
            }),
        )
        .await
        .unwrap_or(json!({ "result": { "items": [] } }));

    let items = result["result"]["items"]
        .as_array()
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
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/hover",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }),
        )
        .await
        .unwrap_or(json!({ "result": null }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_goto_definition(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/definition",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character }
            }),
        )
        .await
        .unwrap_or(json!({ "result": null }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_find_references(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/references",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character },
                "context": { "includeDeclaration": true }
            }),
        )
        .await
        .unwrap_or(json!({ "result": [] }));
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
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/rename",
            json!({
                "textDocument": { "uri": uri },
                "position": { "line": line, "character": character },
                "newName": new_name
            }),
        )
        .await
        .unwrap_or(json!({ "result": null }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_format_document(
    state: State<'_, EditorState>,
    uri: String,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/formatting",
            json!({
                "textDocument": { "uri": uri },
                "options": { "tabSize": 4, "insertSpaces": true }
            }),
        )
        .await
        .unwrap_or(json!({ "result": null }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_workspace_symbols(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Value, String> {
    let uri = "file:///workspace";
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            uri,
            "workspace/symbol",
            json!({ "query": query }),
        )
        .await
        .unwrap_or(json!({ "result": [] }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_code_lens(
    state: State<'_, EditorState>,
    uri: String,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/codeLens",
            json!({ "textDocument": { "uri": uri } }),
        )
        .await
        .unwrap_or(json!({ "result": [] }));
    Ok(result["result"].clone())
}

#[tauri::command]
pub async fn lsp_document_symbols(
    state: State<'_, EditorState>,
    uri: String,
) -> Result<Value, String> {
    let mut router = state.lsp_router.lock().await;
    let result = router
        .request_with_response(
            &uri,
            "textDocument/documentSymbol",
            json!({ "textDocument": { "uri": uri } }),
        )
        .await
        .unwrap_or(json!({ "result": [] }));
    Ok(result["result"].clone())
}
