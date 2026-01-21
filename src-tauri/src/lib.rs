use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use portable_pty::{native_pty_system, CommandBuilder, PtySize, MasterPty};
use std::io::{Read, Write};
use tauri::{Emitter, State};
use ropey::Rope;
use serde::{Serialize, Deserialize};
use tree_sitter::{Parser, Query, QueryCursor};
use tree_sitter_rust::language;

mod lsp;
use lsp::LspClient;

mod context_key;
use context_key::{ContextKeyRegistry, ContextValue};

mod extension_host;
use extension_host::ExtensionHostManager;

mod git;
use git::GitManager;

mod performance;
use performance::{PerformanceMonitor, ProcessStats};

// ... existing imports ...

// Removed duplicate run and git_status

mod keybindings;
use keybindings::KeybindingRegistry;

mod debug_adapter;
use debug_adapter::DebugManager;

mod activation;
use activation::ActivationManager;

use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    theme: String,
    font_size: u32,
}

struct EditorState {
    buffers: Mutex<HashMap<String, Rope>>,
    active_path: Mutex<Option<String>>,
    settings: Mutex<Settings>,
    terminal_masters: Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    terminal_writers: Mutex<HashMap<String, Box<dyn Write + Send>>>,
    lsp_client: Arc<Mutex<LspClient>>,
    context_keys: Arc<ContextKeyRegistry>,
    ext_host: Arc<Mutex<ExtensionHostManager>>,
    keybindings: Arc<Mutex<KeybindingRegistry>>,
    debug_manager: Arc<Mutex<DebugManager>>,
    activation_manager: Arc<Mutex<ActivationManager>>,
    perf_monitor: Arc<PerformanceMonitor>,
    config_dir: PathBuf,
}

#[derive(Serialize, Deserialize)]
struct Highlight {
    start: usize,
    end: usize,
    kind: String,
}

#[derive(Serialize, Deserialize)]
struct FileEntry {
    name: String,
    path: String,
    is_dir: bool,
    children: Option<Vec<FileEntry>>,
}

#[tauri::command]
fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = std::fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut result = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry.metadata().map_err(|e| format!("Failed to read metadata: {}", e))?;
        let name = entry.file_name().to_string_lossy().to_string();
        
        // Skip hidden files to keep UI clean initialy
        if name.starts_with(".") {
            continue;
        }

        result.push(FileEntry {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            children: None, // Flat for now, UI will request deeper ones
        });
    }

    // Sort: Directories first, then alphabetically
    result.sort_by(|a, b| {
        if a.is_dir != b.is_dir {
            b.is_dir.cmp(&a.is_dir)
        } else {
            a.name.to_lowercase().cmp(&b.name.to_lowercase())
        }
    });

    Ok(result)
}

#[tauri::command]
fn open_file(state: tauri::State<'_, EditorState>, path: String) -> Result<String, String> {
    let mut buffers = state.buffers.lock().unwrap();
    let mut active = state.active_path.lock().unwrap();
    
    if let Some(rope) = buffers.get(&path) {
        *active = Some(path.clone());
        return Ok(rope.to_string());
    }

    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))?;
    
    buffers.insert(path.clone(), Rope::from_str(&content));
    *active = Some(path);

    Ok(content)
}

#[tauri::command]
fn switch_to_buffer(state: tauri::State<'_, EditorState>, path: String) -> Result<String, String> {
    let buffers = state.buffers.lock().unwrap();
    let mut active = state.active_path.lock().unwrap();
    
    if let Some(rope) = buffers.get(&path) {
        *active = Some(path);
        Ok(rope.to_string())
    } else {
        Err("Buffer not found".to_string())
    }
}

#[tauri::command]
fn save_file(state: tauri::State<'_, EditorState>, content: String) -> Result<(), String> {
    let active = state.active_path.lock().unwrap();
    let path = active.as_ref().ok_or("No active file")?;

    std::fs::write(path, &content)
        .map_err(|e| format!("Failed to save file: {}", e))?;

    let mut buffers = state.buffers.lock().unwrap();
    buffers.insert(path.clone(), Rope::from_str(&content));

    Ok(())
}

#[tauri::command]
fn sync_content(state: tauri::State<'_, EditorState>, content: String) {
    let active = state.active_path.lock().unwrap();
    if let Some(path) = active.as_ref() {
        let mut buffers = state.buffers.lock().unwrap();
        buffers.insert(path.clone(), Rope::from_str(&content));
    }
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    path: String,
    line: usize,
    content: String,
}

#[tauri::command]
fn get_git_branch() -> Result<String, String> {
    use std::process::Command;
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|_| "Git not found".to_string())?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        Err("Not a git repository".to_string())
    }
}

#[tauri::command]
fn git_status(path: String) -> Result<Vec<git::GitFileStatus>, String> {
    let manager = GitManager::new();
    manager.get_status(path)
}

#[tauri::command]
fn git_stage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.stage(path, &file_path)
}

#[tauri::command]
fn git_unstage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.unstage(path, &file_path)
}

#[tauri::command]
fn git_commit(path: String, message: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.commit(path, &message)
}

#[tauri::command]
fn get_process_stats(state: State<'_, EditorState>) -> Result<ProcessStats, String> {
    state.perf_monitor.get_stats().ok_or("Failed to get process stats".to_string())
}

#[tauri::command]
fn search_project(query: String) -> Result<Vec<SearchResult>, String> {
    use walkdir::WalkDir;
    let mut results = Vec::new();

    for entry in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            // Skip binary and large files for simplicity
            if let Ok(content) = std::fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if line.contains(&query) {
                        results.push(SearchResult {
                            path: path.to_string_lossy().to_string(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                    }
                }
            }
        }
        if results.len() > 1000 { break; } // Cap results
    }

    Ok(results)
}

#[tauri::command]
fn get_highlights(state: tauri::State<'_, EditorState>) -> Vec<Highlight> {
    let active = state.active_path.lock().unwrap();
    let path = match active.as_ref() {
        Some(p) => p,
        None => return Vec::new(),
    };

    let buffers = state.buffers.lock().unwrap();
    let buffer = match buffers.get(path) {
        Some(b) => b,
        None => return Vec::new(),
    };
    
    let code = buffer.to_string();
    let mut parser = Parser::new();
    parser.set_language(language()).expect("Error loading Rust grammar");
    
    let tree = parser.parse(&code, None).unwrap();
    
    // Simple query for keywords and functions for demonstration
    let query_str = "(keyword) @keyword (function_item name: (identifier) @function)";
    let query = Query::new(language(), query_str).unwrap();
    let mut cursor = QueryCursor::new();
    
    let mut highlights = Vec::new();
    let matches = cursor.matches(&query, tree.root_node(), code.as_bytes());
    
    for m in matches {
        for capture in m.captures {
            let node = capture.node;
            highlights.push(Highlight {
                start: node.start_byte(),
                end: node.end_byte(),
                kind: query.capture_names()[capture.index as usize].clone(),
            });
        }
    }
    
    highlights
}

#[tauri::command]
fn get_settings(state: tauri::State<'_, EditorState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(state: tauri::State<'_, EditorState>, new_settings: Settings) -> Result<(), String> {
    *state.settings.lock().unwrap() = new_settings.clone();
    
    // Persist to disk
    let config_dir = state.config_dir.clone();
    let path = config_dir.join("settings.json");
    let content = serde_json::to_string_pretty(&new_settings).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Option<String> {
    let keybindings = state.keybindings.lock().unwrap();
    keybindings.resolve_key(&key, &state.context_keys)
}
#[tauri::command]
fn spawn_terminal(state: State<'_, EditorState>, window: tauri::Window, term_id: String) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system.open_pty(PtySize {
        rows: 24,
        cols: 80,
        pixel_width: 0,
        pixel_height: 0,
    }).map_err(|e| e.to_string())?;

    #[cfg(target_os = "windows")]
    let cmd = CommandBuilder::new("powershell.exe");
    #[cfg(not(target_os = "windows"))]
    let cmd = CommandBuilder::new("zsh");

    let mut _child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    
    let master = pair.master;
    let mut reader = master.try_clone_reader().map_err(|e| e.to_string())?;
    let writer = master.take_writer().map_err(|e| e.to_string())?;
    
    state.terminal_masters.lock().unwrap().insert(term_id.clone(), master);
    state.terminal_writers.lock().unwrap().insert(term_id.clone(), writer);

    let term_id_clone = term_id.clone();
    std::thread::spawn(move || {
        let mut buf = [0u8; 1024];
        loop {
            match reader.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();
                    let _ = window.emit("terminal-data", serde_json::json!({ "term_id": term_id_clone, "data": data }));
                }
                Err(_) => break,
            }
        }
    });

    Ok(())
}

#[tauri::command]
fn lsp_start(state: State<'_, EditorState>, app_handle: tauri::AppHandle, command: String) -> Result<(), String> {
    let mut client = state.lsp_client.lock().unwrap();
    client.start(&command, app_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn lsp_send_request(state: State<'_, EditorState>, id: i32, method: String, params: Value) -> Result<(), String> {
    let mut client = state.lsp_client.lock().unwrap();
    client.send_request(id, &method, params).map_err(|e| e.to_string())
}

#[tauri::command]
fn lsp_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut client = state.lsp_client.lock().unwrap();
    client.stop();
    Ok(())
}

#[tauri::command]
fn write_to_terminal(state: State<'_, EditorState>, term_id: String, data: String) -> Result<(), String> {
    if let Some(writer) = state.terminal_writers.lock().unwrap().get_mut(&term_id) {
        writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn resize_terminal(state: State<'_, EditorState>, term_id: String, cols: u16, rows: u16) -> Result<(), String> {
    if let Some(master) = state.terminal_masters.lock().unwrap().get(&term_id) {
        master.resize(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        }).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn set_context_key(state: State<'_, EditorState>, key: String, value: ContextValue) {
    state.context_keys.set(key, value);
}

#[tauri::command]
fn evaluate_when_clause(state: State<'_, EditorState>, clause: String) -> bool {
    state.context_keys.evaluate(&clause)
}

#[tauri::command]
fn ext_host_init(state: State<'_, EditorState>, app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut ext_host = state.ext_host.lock().unwrap();
    let extensions_dir = state.config_dir.join("extensions");
    ext_host.scan_extensions(extensions_dir).map_err(|e| e.to_string())?;
    ext_host.start(app_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn ext_host_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    state.ext_host.lock().unwrap().send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_marketplace(query: String) -> Result<serde_json::Value, String> {
    let url = format!("https://open-vsx.org/api/-/search?q={}", query);
    let client = reqwest::Client::new();
    let res = client.get(url)
        .header("User-Agent", "VSCodium-Rust-Rewrite")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    let json = res.json::<serde_json::Value>().await.map_err(|e| e.to_string())?;
    Ok(json)
}

#[tauri::command]
async fn install_extension(state: State<'_, EditorState>, download_url: String, name: String) -> Result<(), String> {
    let client = reqwest::Client::new();
    let res = client.get(download_url)
        .header("User-Agent", "VSCodium-Rust-Rewrite")
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let bytes = res.bytes().await.map_err(|e| e.to_string())?;
    let extensions_dir = state.config_dir.join("extensions");
    if !extensions_dir.exists() {
        fs::create_dir_all(&extensions_dir).map_err(|e| e.to_string())?;
    }

    let target_dir = extensions_dir.join(&name);
    if target_dir.exists() {
        fs::remove_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }
    fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;

    let reader = std::io::Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| e.to_string())?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| e.to_string())?;
        let outpath = match file.enclosed_name() {
            Some(path) => target_dir.join(path),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| e.to_string())?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| e.to_string())?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| e.to_string())?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| e.to_string())?;
        }
    }

    Ok(())
}

#[tauri::command]
fn get_running_extensions(state: State<'_, EditorState>) -> Vec<extension_host::ExtensionMetadata> {
    state.ext_host.lock().unwrap().extensions.clone()
}

#[tauri::command]
fn debug_start(state: State<'_, EditorState>, app_handle: tauri::AppHandle, adapter_path: String) -> Result<(), String> {
    state.debug_manager.lock().unwrap().start_session(&adapter_path, app_handle)
}

#[tauri::command]
fn debug_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    state.debug_manager.lock().unwrap().send_message(msg)
}

#[tauri::command]
fn debug_stop(state: State<'_, EditorState>) -> Result<(), String> {
    state.debug_manager.lock().unwrap().stop_session()
}

#[tauri::command]
fn check_activation_event(state: State<'_, EditorState>, event: String) -> Result<(), String> {
    state.activation_manager.lock().unwrap().check_activation_requests(&event, state.ext_host.clone());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // In Tauri 2.0, we use setup() to handle paths properly.
    // For now, we use a simple home-based path or current dir.
    let config_dir = std::env::current_dir().unwrap().join("config");
    
    if !config_dir.exists() {
        fs::create_dir_all(&config_dir).ok();
    }

    // Load settings from disk
    let settings_path = config_dir.join("settings.json");
    let initial_settings = if settings_path.exists() {
        let content = fs::read_to_string(&settings_path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or(Settings { theme: "dark".to_string(), font_size: 14 })
    } else {
        Settings { theme: "dark".to_string(), font_size: 14 }
    };

    tauri::Builder::default()
        .manage(EditorState {
            buffers: Mutex::new(HashMap::new()),
            active_path: Mutex::new(None),
            settings: Mutex::new(initial_settings),
            terminal_masters: Mutex::new(HashMap::new()),
            terminal_writers: Mutex::new(HashMap::new()),
            lsp_client: Arc::new(Mutex::new(LspClient::new())),
            context_keys: Arc::new(ContextKeyRegistry::new()),
            ext_host: Arc::new(Mutex::new(extension_host::ExtensionHostManager::new())),
            keybindings: Arc::new(Mutex::new(keybindings::KeybindingRegistry::new())),
            debug_manager: Arc::new(Mutex::new(DebugManager::new())),
            activation_manager: Arc::new(Mutex::new(ActivationManager::new())),
            perf_monitor: Arc::new(PerformanceMonitor::new()),
            config_dir: config_dir,
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            open_file,
            save_file,
            get_highlights,
            sync_content,
            list_directory,
            search_project,
            switch_to_buffer,
            get_git_branch,
            git_status,
            git_stage,
            git_unstage,
            git_commit,
            get_settings,
            update_settings,
            spawn_terminal,
            write_to_terminal,
            lsp_start,
            lsp_send_request,
            lsp_stop,
            set_context_key,
            evaluate_when_clause,
            resize_terminal,
            ext_host_init,
            ext_host_send,
            resolve_keybinding,
            search_marketplace,
            install_extension,
            get_running_extensions,
            debug_start,
            debug_send,
            debug_stop,
            check_activation_event,
            get_process_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
