use tauri::State;
use serde_json::{json, Value};
use std::path::PathBuf;

#[tauri::command]
pub async fn mount_project(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
) -> Result<(), String> {
    let path_buf = PathBuf::from(path);
    state.memory.layer.mount_workspace(path_buf.clone()).await.map_err(|e| e.to_string())?;
    let mut root = state.editor.active_root.lock().await;
    *root = Some(path_buf);
    Ok(())
}

#[tauri::command]
pub async fn unmount_project(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<(), String> {
    state.memory.layer.unmount().await;
    let mut root = state.editor.active_root.lock().await;
    *root = None;
    Ok(())
}

#[tauri::command]
pub async fn get_project_memory(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Value, String> {
    let mem = state.memory.layer.get_all_memory().await.map_err(|e| e.to_string())?;
    Ok(json!(mem))
}

#[tauri::command]
pub async fn clear_project_memory(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<(), String> {
    state.memory.layer.clear_memory().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_project_memory(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    key: String,
    value: String,
) -> Result<(), String> {
    state.memory.layer.set_memory(&key, &value).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_project(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    query: String,
) -> Result<Value, String> {
    let results = state.memory.layer.search(&query).await.map_err(|e| e.to_string())?;
    Ok(json!(results))
}

#[tauri::command]
pub async fn query_workspace_memory(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    query: String,
) -> Result<Value, String> {
    let results = state.memory.layer.query_context(&query).await.map_err(|e| e.to_string())?;
    Ok(json!(results))
}

#[tauri::command]
pub async fn get_file_context(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
) -> Result<Value, String> {
    let ctx = state.memory.layer.get_file_context(&path).await.map_err(|e| e.to_string())?;
    Ok(json!(ctx))
}

/// Clear the AI's working memory: in-flight conversation history AND the
/// persistent Kortex semantic slots. Wired to the IDE "clear chat / new
/// session" control (RightSidebar + agent.ts).
#[tauri::command]
pub async fn clear_ai_memory(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<(), String> {
    state.ai.engine.clear_conversation().await;
    state.memory.store.clear().await;
    Ok(())
}

/// Search files in the active workspace by query. Matches both file *names*
/// and file *contents* (case-insensitive substring), returning relative paths
/// ranked name-matches-first. Used by the agent's keyword sweep and the Test
/// Explorer. Skips common build/vendor dirs and binary-ish files.
#[tauri::command]
pub async fn search_codebase_files(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    query: String,
    root: Option<String>,
) -> Result<Value, String> {
    let root_path = match root {
        Some(r) if !r.trim().is_empty() => PathBuf::from(r),
        _ => state.editor.active_root.lock().await.clone()
            .ok_or_else(|| "No active workspace root".to_string())?,
    };
    let needle = query.to_lowercase();
    if needle.is_empty() {
        return Ok(json!({ "query": query, "results": [] }));
    }

    const SKIP_DIRS: &[&str] = &[
        ".git", "node_modules", "target", "dist", "build", ".next",
        "out", ".cache", "vendor", "__pycache__", ".venv",
    ];
    let mut name_hits: Vec<String> = Vec::new();
    let mut content_hits: Vec<String> = Vec::new();

    let mut stack = vec![root_path.clone()];
    let mut scanned = 0usize;
    while let Some(dir) = stack.pop() {
        if name_hits.len() + content_hits.len() >= 200 { break; }
        let entries = match std::fs::read_dir(&dir) { Ok(e) => e, Err(_) => continue };
        for entry in entries.flatten() {
            let path = entry.path();
            let fname = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                if !SKIP_DIRS.contains(&fname.as_str()) && !fname.starts_with('.') {
                    stack.push(path);
                }
                continue;
            }
            let rel = path.strip_prefix(&root_path).unwrap_or(&path)
                .to_string_lossy().replace('\\', "/");
            if fname.to_lowercase().contains(&needle) {
                name_hits.push(rel);
                continue;
            }
            // Content match — only for reasonably-sized text files.
            if scanned < 4000 {
                if let Ok(meta) = std::fs::metadata(&path) {
                    if meta.len() <= 512 * 1024 {
                        scanned += 1;
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if content.to_lowercase().contains(&needle) {
                                content_hits.push(rel);
                            }
                        }
                    }
                }
            }
        }
    }

    name_hits.extend(content_hits);
    name_hits.truncate(200);
    Ok(json!({ "query": query, "root": root_path.to_string_lossy(), "results": name_hits }))
}
