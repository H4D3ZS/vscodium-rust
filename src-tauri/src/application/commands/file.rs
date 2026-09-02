use crate::{EditorState, domain::FileEntry};
use tauri::State;
use std::path::{Path, PathBuf};
use std::fs;
use ropey::Rope;

const LARGE_FILE_BYTES: u64 = 2 * 1024 * 1024;
const PAGED_VIEWER_WINDOW: u64 = 4096;

#[derive(serde::Serialize)]
pub struct FileStat {
    pub size: u64,
    pub lines: usize,
    pub is_large: bool,
}

#[derive(serde::Serialize)]
pub struct LargeFileMarker {
    pub large: bool,
    pub size: u64,
    pub lines: usize,
}

/// Validate that a path is within the project root and not a blocked directory.
fn validate_path_against_root(path: &str, root: &Path) -> Result<PathBuf, String> {
    let p = PathBuf::from(path);
    let full = if p.is_absolute() { p } else { root.join(&p) };
    // Normalize separators for comparison
    let full_str = full.to_string_lossy().replace('\\', "/");
    let root_str = root.to_string_lossy().replace('\\', "/");
    // Check if the file is within the project root (allow root itself)
    if !full_str.starts_with(&root_str) && !root_str.starts_with(&full_str.trim_end_matches('/')) {
        return Err(format!("Path escapes project root: {}", path));
    }
    Ok(full)
}

pub async fn is_path_valid(state: &EditorState, path: &PathBuf) -> Result<(), String> {
    let root = state.editor.active_root.lock().await
        .clone()
        .ok_or_else(|| "No project root set".to_string())?;
    validate_path_against_root(&path.to_string_lossy(), &root)?;
    Ok(())
}

#[tauri::command]
pub async fn create_dir(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<(), String> {
    let root = state.editor.active_root.lock().await
        .clone()
        .ok_or_else(|| "No project root set".to_string())?;
    let full = validate_path_against_root(&path, &root)?;
    fs::create_dir_all(&full).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_directory(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<(), String> {
    let root = state.editor.active_root.lock().await
        .clone()
        .ok_or_else(|| "No project root set".to_string())?;
    let full = validate_path_against_root(&path, &root)?;
    fs::create_dir_all(&full).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn create_file(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<(), String> {
    let root = state.editor.active_root.lock().await
        .clone()
        .ok_or_else(|| "No project root set".to_string())?;
    let full = validate_path_against_root(&path, &root)?;
    fs::File::create(&full).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub async fn validate_path(state: State<'_, std::sync::Arc<crate::EditorState>>, path: PathBuf) -> Result<(), String> {
    is_path_valid(&state, &path).await
}

fn get_ignore_patterns() -> Vec<&'static str> {
    vec![
        ".git",
        "node_modules",
        "target",
        ".DS_Store",
        "__pycache__",
        ".next",
        "dist",
        "build",
        ".svelte-kit",
        ".turbo",
    ]
}

#[tauri::command]
pub async fn list_dir_flat(path: PathBuf) -> Result<Vec<FileEntry>, String> {
    let mut tree = Vec::new();
    let ignore_list = get_ignore_patterns();

    if !path.is_dir() {
        return Err(format!("Not a directory: {}", path.display()));
    }

    let entries = fs::read_dir(&path).map_err(|e| {
        format!("Failed to read directory {}: {}", path.display(), e)
    })?;

    for entry in entries.filter_map(|e| e.ok()) {
        let entry_path = entry.path();
        let name = entry_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        if name.is_empty() || ignore_list.iter().any(|&p| name == p) {
            continue;
        }

        let is_dir = match entry.metadata().or_else(|_| fs::metadata(&entry_path)) {
            Ok(m) => m.is_dir(),
            Err(_) => continue,
        };

        tree.push(FileEntry {
            name,
            path: entry_path.to_string_lossy().to_string(),
            is_dir,
            is_expanded: Some(false),
            children: None,
        });
    }

    tree.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(tree)
}

#[tauri::command]
pub async fn get_file_tree(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: Option<String>,
) -> Result<Vec<FileEntry>, String> {
    let root = if let Some(raw) = path {
        let cleaned = raw.split('\0').next().unwrap_or("").trim().to_string();
        if cleaned.is_empty() {
            return Err("Empty workspace path".to_string());
        }
        let path_buf = PathBuf::from(&cleaned);
        if !path_buf.is_dir() {
            return Err(format!("Not a directory: {}", cleaned));
        }
        {
            let mut root_guard = state.editor.active_root.lock().await;
            *root_guard = Some(path_buf.clone());
        }
        state.ai.engine.set_root_path(path_buf.clone());
        path_buf
    } else {
        state
            .editor.active_root
            .lock()
            .await
            .clone()
            .ok_or_else(|| "No project open".to_string())?
    };
    list_dir_flat(root).await
}

#[tauri::command]
pub async fn refresh_file_tree(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Vec<FileEntry>, String> {
    get_file_tree(state, None).await
}

#[tauri::command]
pub async fn get_directory_contents(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
) -> Result<Vec<FileEntry>, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;
    list_dir_flat(path_buf).await
}

#[tauri::command]
pub async fn file_stat(path: String) -> Result<FileStat, String> {
    let meta = fs::metadata(&path).map_err(|e| e.to_string())?;
    let size = meta.len();
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let lines = content.lines().count();
    Ok(FileStat { size, lines, is_large: size > LARGE_FILE_BYTES })
}

#[tauri::command]
pub async fn read_file_range(path: String, start_byte: u64, len: u64) -> Result<String, String> {
    use std::io::{Read, Seek, SeekFrom};
    let mut file = fs::File::open(&path).map_err(|e| e.to_string())?;
    file.seek(SeekFrom::Start(start_byte)).map_err(|e| e.to_string())?;
    let read_len = len.min(PAGED_VIEWER_WINDOW * 200);
    let mut buf = vec![0u8; read_len as usize];
    let n = file.read(&mut buf).map_err(|e| e.to_string())?;
    buf.truncate(n);
    String::from_utf8(buf).map_err(|e| format!("Invalid UTF-8: {}", e))
}

#[tauri::command]
pub async fn open_file(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<serde_json::Value, String> {
    if let Some(root) = state.editor.active_root.lock().await.clone() {
        validate_path_against_root(&path, &root)?;
    }
    let meta = fs::metadata(&path).map_err(|e| format!("Failed to read file: {}", e))?;
    let size = meta.len();

    if size > LARGE_FILE_BYTES {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let lines = content.lines().count();
        let preview: String = content.lines().take(PAGED_VIEWER_WINDOW as usize).collect::<Vec<_>>().join("\n");
        let mut buffers = state.editor.buffers.lock().await;
        buffers.insert(path.clone(), Rope::from_str(&preview));
        let mut active = state.editor.active_path.lock().await;
        *active = Some(path);
        return Ok(serde_json::json!({
            "large": true,
            "size": size,
            "lines": lines,
            "preview": preview
        }));
    }

    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let mut buffers = state.editor.buffers.lock().await;
    buffers.insert(path.clone(), Rope::from_str(&content));
    let mut active = state.editor.active_path.lock().await;
    *active = Some(path);
    Ok(serde_json::json!({ "large": false, "content": content }))
}

#[tauri::command]
pub async fn save_file(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String, content: String) -> Result<(), String> {
    if let Some(root) = state.editor.active_root.lock().await.clone() {
        validate_path_against_root(&path, &root)?;
    }
    fs::write(&path, &content).map_err(|e| format!("Failed to write file: {}", e))?;
    let mut buffers = state.editor.buffers.lock().await;
    buffers.insert(path, Rope::from_str(&content));
    // Signal Kairos that the user is active — prevents background tasks from running mid-edit
    state.services.kairos.report_activity().await;
    Ok(())
}

#[tauri::command]
pub fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut results = Vec::new();
    for entry in entries.filter_map(|e| e.ok()) {
        let name = entry.file_name().to_string_lossy().to_string();
        if name.is_empty() { continue; }
        // Skip on metadata errors (Windows junctions, restricted files) instead of failing
        let is_dir = match entry.metadata() {
            Ok(m) => m.is_dir(),
            Err(_) => continue,
        };
        results.push(FileEntry {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir,
            is_expanded: Some(false),
            children: None,
        });
    }
    Ok(results)
}

#[tauri::command]
pub async fn open_folder(
    app: tauri::AppHandle,
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_folder(move |folder| {
        let _ = tx.send(folder);
    });

    let folder_path = rx.await.map_err(|e| e.to_string())?;

    if let Some(folder) = folder_path {
        let path = match folder {
            tauri_plugin_dialog::FilePath::Path(p) => p,
            tauri_plugin_dialog::FilePath::Url(u) => {
                u.to_file_path().unwrap_or(PathBuf::from(u.path()))
            }
        };
        let mut root = state.editor.active_root.lock().await;
        *root = Some(path.clone());
        state.ai.engine.set_root_path(path.clone());
        return Ok(Some(path.to_string_lossy().to_string()));
    }
    Ok(None)
}

#[tauri::command]
pub async fn list_project_files(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<Vec<String>, String> {
    let root = state.editor.active_root.lock().await.clone()
        .ok_or_else(|| "No folder open".to_string())?;
    let skip_dirs = ["node_modules", ".git", "target", "dist", ".next", "build", "__pycache__", ".cache"];
    let skip_exts = ["png","jpg","jpeg","gif","bmp","ico","woff","woff2","ttf","eot","bin","exe","dll","so","a","lib","pdf","zip","tar","gz","lock"];
    let mut files = Vec::new();
    for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        // Skip hidden and known large dirs
        if path.components().any(|c| {
            let s = c.as_os_str().to_str().unwrap_or("");
            s.starts_with('.') || skip_dirs.contains(&s)
        }) { continue; }
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if skip_exts.contains(&ext.as_str()) { continue; }
        let rel = path.strip_prefix(&root).unwrap_or(path);
        files.push(rel.to_string_lossy().replace('\\', "/"));
        if files.len() >= 5000 { break; }
    }
    files.sort();
    Ok(files)
}

#[tauri::command]
pub async fn read_file(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<String, String> {
    if let Some(root) = state.editor.active_root.lock().await.clone() {
        validate_path_against_root(&path, &root)?;
    }
    fs::read_to_string(&path).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    path: String,
    content: String,
) -> Result<(), String> {
    if let Some(root) = state.editor.active_root.lock().await.clone() {
        validate_path_against_root(&path, &root)?;
    }
    fs::write(&path, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_path(state: State<'_, std::sync::Arc<crate::EditorState>>, path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    if let Some(root) = state.editor.active_root.lock().await.clone() {
        validate_path_against_root(&path, &root)?;
    }
    
    if path_buf.is_dir() {
        fs::remove_dir_all(path_buf).map_err(|e| e.to_string())
    } else {
        fs::remove_file(path_buf).map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub async fn rename_path(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    old_path: String,
    new_path: String,
) -> Result<(), String> {
    let old_buf = PathBuf::from(&old_path);
    let new_buf = PathBuf::from(&new_path);
    
    is_path_valid(&state, &old_buf).await?;
    is_path_valid(&state, &new_buf).await?;
    
    fs::rename(old_buf, new_buf).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn write_file_content(path: String, content: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
    }
    fs::write(p, content).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn glob_files(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    pattern: String,
    path: Option<String>,
) -> Result<Vec<String>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state.editor.active_root.lock().await.clone().unwrap_or_else(|| PathBuf::from("."))
    };

    // Correctly normalize the pattern for Windows
    let clean_pattern = pattern.replace("\\", "/");
    let full_pattern = if std::path::Path::new(&pattern).is_absolute() {
        clean_pattern
    } else {
        root.join(pattern).to_string_lossy().to_string().replace("\\", "/")
    };

    let mut results = Vec::new();
    if let Ok(entries) = glob::glob(&full_pattern) {
        for entry in entries {
            if let Ok(path) = entry {
                let rel = path.strip_prefix(&root).unwrap_or(&path);
                results.push(rel.to_string_lossy().to_string());
                if results.len() >= 100 { break; }
            }
        }
    }
    Ok(results)
}
#[tauri::command]
pub async fn editor_get_active_file(
    state: tauri::State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<serde_json::Value, String> {
    let sentient = state.ai.engine.clone();
    let tools = sentient.get_tools();
    tools
        .editor_get_active_file(serde_json::json!({}))
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
pub async fn replace_in_files(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    query: String,
    replacement: String,
    case_sensitive: bool,
) -> Result<usize, String> {
    let root = state.editor.active_root.lock().await.clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let mut count = 0usize;
    let walker = walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok());
    
    for entry in walker {
        if !entry.file_type().is_file() { continue; }
        let path = entry.path();
        // Skip binary-like extensions and hidden dirs
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        if ["png","jpg","jpeg","gif","bmp","ico","woff","woff2","ttf","eot","bin","exe","dll","so","a","lib","pdf","zip","tar","gz"].contains(&ext.as_str()) { continue; }
        if path.components().any(|c| c.as_os_str().to_str().map(|s| s.starts_with('.')).unwrap_or(false)) { continue; }
        
        let content = match fs::read_to_string(path) { Ok(c) => c, Err(_) => continue };
        let new_content = if case_sensitive {
            content.replace(&query, &replacement)
        } else {
            let lower_content = content.to_lowercase();
            let lower_query = query.to_lowercase();
            if !lower_content.contains(&lower_query) { continue; }
            
            let mut result = String::with_capacity(content.len());
            let mut last = 0usize;
            let bytes = content.as_bytes();
            let lbytes = lower_content.as_bytes();
            let qbytes = lower_query.as_bytes();
            let mut i = 0usize;
            while i + qbytes.len() <= bytes.len() {
                if &lbytes[i..i+qbytes.len()] == qbytes {
                    result.push_str(&content[last..i]);
                    result.push_str(&replacement);
                    i += qbytes.len();
                    last = i;
                } else {
                    i += 1;
                }
            }
            result.push_str(&content[last..]);
            result
        };

        if new_content != content {
            fs::write(path, new_content).map_err(|e| e.to_string())?;
            count += 1;
        }
    }
    Ok(count)
}

#[tauri::command]
pub async fn get_directory_tree(root: String, max_depth: Option<usize>) -> Result<String, String> {
    let depth = max_depth.unwrap_or(3);
    let root_path = std::path::Path::new(&root);
    if !root_path.exists() {
        return Ok("(directory not found)".to_string());
    }

    fn walk(path: &std::path::Path, depth: usize, max: usize, out: &mut String, indent: usize) {
        if depth > max { return; }
        let Ok(entries) = std::fs::read_dir(path) else { return };
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') || name == "target" || name == "node_modules" || name == "dist" { continue; }
            let prefix = "  ".repeat(indent);
            let ft = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
            if ft {
                out.push_str(&format!("{}{}/\n", prefix, name));
                walk(&entry.path(), depth + 1, max, out, indent + 1);
            } else {
                out.push_str(&format!("{}{}\n", prefix, name));
            }
        }
    }

    let mut output = String::new();
    walk(root_path, 0, depth, &mut output, 0);
    Ok(output)
}
