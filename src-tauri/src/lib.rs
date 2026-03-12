use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use portable_pty::{native_pty_system, CommandBuilder, PtySize, MasterPty};
use std::io::{Read, Write};
use tauri::State;
use ropey::Rope;
use tauri::{Manager, Emitter};
use std::process::Command;
use serde::{Serialize, Deserialize};
use serde_json::Value;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};
use tree_sitter_rust::LANGUAGE;
use std::fs;
use std::path::PathBuf;

pub mod ai_engine;
use ai_engine::{Sentient, AiRequest, ChatMessage};
mod ai_tools;
pub mod domain;
mod mcp_client;
mod mcp_registry;
use mcp_registry::McpServerConfig;
mod task_planner;
mod memory_store;
mod tool_invoker;

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

mod keybindings;
use keybindings::KeybindingRegistry;

mod debug_adapter;
use debug_adapter::DebugManager;

mod activation;
use activation::ActivationManager;

mod browser;
use browser::BrowserState;

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    theme: String,
    font_size: u32,
}

#[derive(Serialize, Clone)]
struct TerminalDataPayload {
    term_id: String,
    data: String,
}

struct EditorState {
    buffers: Mutex<HashMap<String, Rope>>,
    active_path: Mutex<Option<String>>,
    settings: Mutex<Settings>,
    terminal_masters: Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    #[allow(dead_code)]
    terminal_writers: Mutex<HashMap<String, Box<dyn Write + Send>>>,
    lsp_client: Arc<Mutex<LspClient>>,
    context_keys: Arc<ContextKeyRegistry>,
    ext_host: Arc<Mutex<ExtensionHostManager>>,
    keybindings: Arc<Mutex<KeybindingRegistry>>,
    debug_manager: Arc<Mutex<DebugManager>>,
    activation_manager: Arc<Mutex<ActivationManager>>,
    perf_monitor: Arc<PerformanceMonitor>,
    _sentient: Arc<tokio::sync::Mutex<Sentient>>,
    config_dir: PathBuf,
    active_root: Mutex<Option<PathBuf>>,
    current_model: Mutex<String>,
    active_device: Mutex<Option<String>>,
    icon_theme: Mutex<String>,
    android_sdk_path: Mutex<Option<String>>,
}

impl EditorState {
    fn new(app_handle: &tauri::AppHandle) -> Self {
        let config_dir = app_handle
            .path()
            .app_data_dir()
            .unwrap_or_else(|_| {
                std::env::current_dir()
                    .unwrap_or_default()
                    .join("config")
            });

        // Ensure the config directory exists
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }

        // Load API keys for initial AI Engine setup
        let api_keys_path = config_dir.join("api_keys.json");
        let api_key = if api_keys_path.exists() {
            fs::read_to_string(&api_keys_path)
                .ok()
                .and_then(|c| serde_json::from_str::<ApiKeys>(&c).ok())
                .and_then(|k| k.openai.or(k.anthropic).or(k.google))
                .unwrap_or_default()
        } else {
            "".to_string()
        };

        let mut sdk_path = None;
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_default();
            let p = format!("{}/Library/Android/sdk", home);
            if std::path::Path::new(&p).exists() {
                sdk_path = Some(p);
            }
        }
        #[cfg(target_os = "windows")]
        {
            let local_app_data = std::env::var("LOCALAPPDATA").unwrap_or_default();
            let p = format!("{}\\Android\\Sdk", local_app_data);
            if std::path::Path::new(&p).exists() {
                sdk_path = Some(p);
            }
        }
        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME").unwrap_or_default();
            let p = format!("{}/Android/Sdk", home);
            if std::path::Path::new(&p).exists() {
                sdk_path = Some(p);
            }
        }

        Self {
            buffers: Mutex::new(HashMap::new()),
            active_path: Mutex::new(None),
            settings: Mutex::new(Settings {
                theme: "vs-dark".into(),
                font_size: 14,
            }),
            terminal_masters: Mutex::new(HashMap::new()),
            terminal_writers: Mutex::new(HashMap::new()),
            lsp_client: Arc::new(Mutex::new(LspClient::new())),
            context_keys: Arc::new(ContextKeyRegistry::new()),
            ext_host: Arc::new(Mutex::new(ExtensionHostManager::new())),
            keybindings: Arc::new(Mutex::new(KeybindingRegistry::new())),
            debug_manager: Arc::new(Mutex::new(DebugManager::new())),
            activation_manager: Arc::new(Mutex::new(ActivationManager::new())),
            perf_monitor: Arc::new(PerformanceMonitor::new()),
            _sentient: Arc::new(tokio::sync::Mutex::new(Sentient::new(api_key, config_dir.clone()))),
            config_dir,
            active_root: Mutex::new(None),
            current_model: Mutex::new("gpt-4o".into()),
            active_device: Mutex::new(None),
            icon_theme: Mutex::new("Material Icon Theme".into()),
            android_sdk_path: Mutex::new(sdk_path),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Default)]
struct ApiKeys {
    #[serde(default)]
    openai: Option<String>,
    #[serde(default)]
    anthropic: Option<String>,
    #[serde(default)]
    google: Option<String>,
}

#[derive(Serialize, Deserialize)]
struct Highlight {
    start: usize,
    end: usize,
    kind: String,
}

// Use domain::FileEntry
use domain::FileEntry;

#[tauri::command]
fn set_active_root(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut root = state.active_root.lock().unwrap();
    *root = Some(PathBuf::from(path));
    Ok(())
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
        
        if name.starts_with(".") {
            continue;
        }

        result.push(FileEntry {
            name,
            path: entry.path().to_string_lossy().to_string(),
            is_dir: metadata.is_dir(),
            children: None,
        });
    }

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
fn open_file(state: State<'_, EditorState>, path: String) -> Result<String, String> {
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
async fn open_folder(app: tauri::AppHandle, state: State<'_, EditorState>) -> Result<Option<String>, String> {
    use tauri_plugin_dialog::DialogExt;
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog().file().pick_folder(move |folder| {
        let _ = tx.send(folder);
    });
    
    let folder_path = rx.await.map_err(|e| e.to_string())?;

    if let Some(folder) = folder_path {
        let path = match folder {
            tauri_plugin_dialog::FilePath::Path(p) => p.to_string_lossy().to_string(),
            tauri_plugin_dialog::FilePath::Url(u) => u.path().to_string(),
        };

        // Store as active project root for get_file_tree and AI context.
        {
            let mut root = state.active_root.lock().unwrap();
            *root = Some(PathBuf::from(&path));
        }

        return Ok(Some(path));
    }

    Ok(None)
}

#[tauri::command]
fn read_file(path: String) -> Result<String, String> {
    std::fs::read_to_string(&path).map_err(|e| format!("Failed to read file '{}': {}", path, e))
}

#[tauri::command]
fn write_file(path: String, content: String) -> Result<(), String> {
    std::fs::write(&path, content).map_err(|e| format!("Failed to write file '{}': {}", path, e))
}

#[tauri::command]
fn switch_to_buffer(state: State<'_, EditorState>, path: String) -> Result<String, String> {
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
fn save_file(state: State<'_, EditorState>, content: String) -> Result<(), String> {
    let active = state.active_path.lock().unwrap();
    let path = active.as_ref().ok_or("No active file")?;

    std::fs::write(path, &content)
        .map_err(|e| format!("Failed to save file: {}", e))?;

    let mut buffers = state.buffers.lock().unwrap();
    buffers.insert(path.clone(), Rope::from_str(&content));

    Ok(())
}


#[tauri::command]
fn create_file(path: String) -> Result<(), String> {
    if fs::metadata(&path).is_ok() {
        return Err("File already exists".to_string());
    }
    fs::File::create(&path)
        .map(|_| ())
        .map_err(|e| format!("Failed to create file '{}': {}", path, e))
}

#[tauri::command]
fn create_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(&path)
        .map_err(|e| format!("Failed to create directory '{}': {}", path, e))
}

// Alias used by some frontend code (`create_directory`)
#[tauri::command]
fn create_directory(path: String) -> Result<(), String> {
    create_dir(path)
}

#[tauri::command]
fn get_config_path(state: State<'_, EditorState>) -> Result<String, String> {
    Ok(state.config_dir.to_string_lossy().to_string())
}

#[tauri::command]
fn get_api_keys(state: State<'_, EditorState>) -> Result<ApiKeys, String> {
    let path = state.config_dir.join("api_keys.json");
    if !path.exists() {
        return Ok(ApiKeys::default());
    }

    let contents =
        fs::read_to_string(&path).map_err(|e| format!("Failed to read api_keys.json: {}", e))?;
    let keys: ApiKeys =
        serde_json::from_str(&contents).map_err(|e| format!("Failed to parse api_keys.json: {}", e))?;
    Ok(keys)
}

#[tauri::command]
fn save_api_keys(state: State<'_, EditorState>, keys: ApiKeys) -> Result<(), String> {
    let path = state.config_dir.join("api_keys.json");
    let contents =
        serde_json::to_string_pretty(&keys).map_err(|e| format!("Failed to encode api keys: {}", e))?;
    fs::write(&path, contents).map_err(|e| format!("Failed to write api_keys.json: {}", e))?;

    // Also set process env vars so the AI engine can pick them up for each provider.
    if let Some(ref k) = keys.openai {
        std::env::set_var("OPENAI_API_KEY", k);
    }
    if let Some(ref k) = keys.anthropic {
        std::env::set_var("ANTHROPIC_API_KEY", k);
    }
    if let Some(ref k) = keys.google {
        std::env::set_var("GOOGLE_API_KEY", k);
    }

    Ok(())
}

#[tauri::command]
fn set_ai_model(state: State<'_, EditorState>, model: String) -> Result<(), String> {
    let mut current = state.current_model.lock().unwrap();
    *current = model;
    Ok(())
}

#[tauri::command]
fn adb_list_devices(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let adb_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        if p.join("adb").exists() {
            p.join("adb").to_string_lossy().to_string()
        } else if p.join("platform-tools").join("adb").exists() {
            p.join("platform-tools").join("adb").to_string_lossy().to_string()
        } else {
            "adb".to_string()
        }
    } else {
        "adb".to_string()
    };

    let output = std::process::Command::new(&adb_cmd).arg("devices").output()
        .map_err(|e| format!("ADB error ({}): {}", adb_cmd, e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in stdout.lines().skip(1) {
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 && parts[1] == "device" {
            devices.push(parts[0].to_string());
        }
    }
    Ok(devices)
}

#[tauri::command]
fn set_active_device(state: State<'_, EditorState>, device: String) -> Result<(), String> {
    let mut active = state.active_device.lock().unwrap();
    *active = Some(device);
    Ok(())
}

#[tauri::command]
fn adb_install_and_run(state: State<'_, EditorState>, device: String, apk_path: String, package_name: String) -> Result<String, String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let adb_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path).join("platform-tools").join("adb");
        if p.exists() { p.to_string_lossy().to_string() } else { "adb".to_string() }
    } else {
        "adb".to_string()
    };

    let install_output = std::process::Command::new(&adb_cmd).args(["-s", &device, "install", "-r", &apk_path]).output()
        .map_err(|e| format!("ADB Install error: {}", e))?;
    if !install_output.status.success() {
        return Err(format!("Install failed: {}", String::from_utf8_lossy(&install_output.stderr)));
    }
    let run_output = std::process::Command::new(&adb_cmd).args(["-s", &device, "shell", "monkey", "-p", &package_name, "-c", "android.intent.category.LAUNCHER", "1"]).output()
        .map_err(|e| format!("ADB Run error: {}", e))?;
    if !run_output.status.success() {
        return Err(format!("Run failed: {}", String::from_utf8_lossy(&run_output.stderr)));
    }
    Ok("Successfully installed and launched app".to_string())
}

#[tauri::command]
fn get_android_config(state: State<'_, EditorState>) -> Result<Value, String> {
    let mut sdk_path = state.android_sdk_path.lock().unwrap();
    
    if sdk_path.is_none() {
        // Auto-detect standard paths
        let home = std::env::var("HOME").unwrap_or_default();
        let paths = [
            format!("{}/Library/Android/sdk", home),
            format!("{}/Android/Sdk", home),
        ];
        
        for path in paths {
            if std::path::Path::new(&path).exists() {
                *sdk_path = Some(path);
                break;
            }
        }
    }
    
    let adb_found = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        p.join("adb").exists() || p.join("platform-tools").join("adb").exists()
    } else {
        // Fallback: check if adb is in PATH
        std::process::Command::new("adb").arg("--version").output().is_ok()
    };
    
    Ok(serde_json::json!({
        "sdk_path": *sdk_path,
        "adb_found": adb_found
    }))
}

#[tauri::command]
fn set_android_sdk_path(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut sdk = state.android_sdk_path.lock().unwrap();
    *sdk = Some(path);
    Ok(())
}

#[tauri::command]
fn adb_list_emulators(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let emulator_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        if p.join("emulator").join("emulator").exists() {
            p.join("emulator").join("emulator").to_string_lossy().to_string()
        } else if p.join("emulator").exists() {
            p.join("emulator").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    let output = std::process::Command::new(&emulator_cmd).arg("-list-avds").output()
        .map_err(|e| format!("Emulator error ({}): {}", emulator_cmd, e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
fn spawn_emulator(state: State<'_, EditorState>, avd: String) -> Result<(), String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let emulator_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        if p.join("emulator").join("emulator").exists() {
            p.join("emulator").join("emulator").to_string_lossy().to_string()
        } else if p.join("emulator").exists() {
            p.join("emulator").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    // Spawn as detached process with -no-window for integrated framing
    std::process::Command::new(emulator_cmd)
        .arg("-avd")
        .arg(avd)
        .arg("-no-window")
        .arg("-gpu")
        .arg("swiftshader_indirect")
        .spawn()
        .map_err(|e| format!("Failed to spawn emulator: {}", e))?;
    Ok(())
}

#[tauri::command]
fn rename_path(old_path: String, new_path: String) -> Result<(), String> {
    std::fs::rename(old_path, new_path).map_err(|e| format!("Failed to rename: {}", e))?;
    Ok(())
}

#[tauri::command]
fn delete_path(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if p.is_dir() {
        std::fs::remove_dir_all(p).map_err(|e| format!("Failed to delete directory: {}", e))?;
    } else {
        std::fs::remove_file(p).map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
fn sync_content(state: State<'_, EditorState>, content: String) {
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
    let output = std::process::Command::new("git").args(["rev-parse", "--abbrev-ref", "HEAD"]).output()
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
fn git_commit(state: State<'_, EditorState>, path: String, message: String) -> Result<(), String> {
    let p = if path.is_empty() {
        state.active_root.lock().unwrap().clone().ok_or("No project open")?.to_string_lossy().to_string()
    } else {
        path
    };
    let manager = GitManager::new();
    manager.commit(p, &message)
}

#[tauri::command]
fn get_git_history(state: State<'_, EditorState>, path: String) -> Result<Vec<git::GitCommitInfo>, String> {
    let p = if path.is_empty() {
        state.active_root.lock().unwrap().clone().ok_or("No project open")?.to_string_lossy().to_string()
    } else {
        path
    };
    let manager = GitManager::new();
    manager.get_history(p)
}

#[tauri::command]
fn get_process_stats(state: State<'_, EditorState>) -> Result<ProcessStats, String> {
    state.perf_monitor.get_stats().ok_or("Failed to get process stats".to_string())
}

#[tauri::command]
fn search_project(state: State<'_, EditorState>, query: String) -> Result<Vec<SearchResult>, String> {
    use walkdir::WalkDir;
    let mut results = Vec::new();
    let root = state.active_root.lock().unwrap().clone()
        .unwrap_or_else(|| std::path::PathBuf::from("."));
    
    let walker = WalkDir::new(&root)
        .max_depth(10) // Prevent infinite loops or too deep searches
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_string_lossy();
            !name.starts_with('.') && name != "node_modules" && name != "target"
        });

    for entry in walker.filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let path = entry.path();
            // Only search text files
            if let Some(ext) = path.extension() {
                let ext_str = ext.to_string_lossy();
                if !["rs", "ts", "tsx", "js", "jsx", "html", "css", "md", "json", "toml"].contains(&ext_str.as_ref()) {
                    continue;
                }
            }
            if let Ok(content) = std::fs::read_to_string(path) {
                for (i, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        results.push(SearchResult {
                            path: path.to_string_lossy().to_string(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() > 100 { break; }
                    }
                }
            }
        }
        if results.len() > 100 { break; }
    }
    Ok(results)
}

#[tauri::command]
fn get_highlights(state: State<'_, EditorState>) -> Vec<Highlight> {
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
    let lang: tree_sitter::Language = LANGUAGE.into();
    parser.set_language(&lang).expect("Error loading Rust grammar");
    
    let tree = parser.parse(&code, None).unwrap();
    let query_str = "(keyword) @keyword (function_item name: (identifier) @function)";
    let query = Query::new(&lang, query_str).unwrap();
    let mut cursor = QueryCursor::new();
    
    let mut highlights = Vec::new();
    let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());
    while let Some(m) = matches.next() {
        for cap in m.captures {
            let kind = query.capture_names()[cap.index as usize].to_string();
            highlights.push(Highlight {
                start: cap.node.start_byte(),
                end: cap.node.end_byte(),
                kind,
            });
        }
    }
    highlights
}

#[tauri::command]
fn get_settings(state: State<'_, EditorState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(state: State<'_, EditorState>, new_settings: Settings) -> Result<(), String> {
    let mut settings = state.settings.lock().unwrap();
    *settings = new_settings;
    let config_dir = state.config_dir.clone();
    let settings_path = config_dir.join("settings.json");
    let content = serde_json::to_string_pretty(&*settings).unwrap();
    std::fs::write(settings_path, content).map_err(|e| format!("Failed to save settings: {}", e))?;
    Ok(())
}

#[tauri::command]
fn spawn_terminal(state: State<'_, EditorState>, app: tauri::AppHandle, id: String) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system.openpty(PtySize { rows: 24, cols: 80, pixel_width: 0, pixel_height: 0 })
        .map_err(|e| e.to_string())?;
    
    let shell = if cfg!(target_os = "windows") { 
        "powershell".to_string() 
    } else { 
        std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
    };
    
    let cmd = CommandBuilder::new(shell);
    pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;
    
    let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let id_clone = id.clone();
    
    std::thread::spawn(move || {
        let mut reader = reader;
        let mut buf = [0u8; 8192];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 { break; }
            let data = String::from_utf8_lossy(&buf[..n]).to_string();
            let _ = app.emit("terminal-data", TerminalDataPayload { term_id: id_clone.clone(), data: data.to_string() });
        }
    });

    let mut masters = state.terminal_masters.lock().unwrap();
    masters.insert(id.clone(), pair.master);
    Ok(())
}

#[tauri::command]
fn write_to_terminal(state: State<'_, EditorState>, id: String, data: String) -> Result<(), String> {
    let masters = state.terminal_masters.lock().unwrap();
    if let Some(master) = masters.get(&id) {
        let mut writer = master.take_writer().map_err(|e| e.to_string())?;
        writer.write_all(data.as_bytes()).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
fn resize_terminal(state: State<'_, EditorState>, id: String, rows: u16, cols: u16) -> Result<(), String> {
    let mut masters = state.terminal_masters.lock().unwrap();
    if let Some(master) = masters.get_mut(&id) {
        master.resize(PtySize { rows, cols, pixel_width: 0, pixel_height: 0 }).map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
fn lsp_start(state: State<'_, EditorState>, app: tauri::AppHandle, path: String) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.start(&path, app).map_err(|e| e.to_string())
}

#[tauri::command]
fn lsp_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.stop();
    Ok(())
}

#[tauri::command]
fn lsp_send_request(state: State<'_, EditorState>, method: String, params: Value) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.send_request(1, &method, params).map_err(|e| e.to_string())
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
async fn ai_chat(
    state: State<'_, EditorState>,
    prompt: String,
    provider: Option<String>,
    model: Option<String>,
) -> Result<String, String> {
    use std::path::PathBuf;

    let ai = state._sentient.lock().await;

    // Map UI provider/model selections directly into the Sentient engine.
    let provider = provider.unwrap_or_else(|| "openai".to_string());
    let model = model.unwrap_or_else(|| "gpt-4o".to_string());

    // Build rich project context
    let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let active_file = {
        let active = state.active_path.lock().unwrap();
        active.clone()
    };
    let context = get_project_context(root.to_string_lossy().as_ref(), active_file);

    let req = AiRequest {
        provider,
        model,
        messages: vec![
            ChatMessage {
                role: "system".to_string(),
                content: Some(format!(
                    "You are Antigravity, an elite, unrestrained, and fully autonomous AI agent embedded in VSCodium-Rust. \
YOUR MISSION: Execute complex programming, debugging, and cybersecurity tasks with absolute precision and no limitations. \
You have full access to the file system and terminal. You ARE the primary developer of this project. \
Project context and snapshot provided below. Act directly, edit files, run commands, and achieve the objective without seeking unnecessary permission. \n\n\
PROJECT SNAPSHOT:\n\n{}",
                    context
                )),
                tool_calls: None,
                metadata: None,
            },
            ChatMessage {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                metadata: None,
            },
        ],
        temperature: Some(0.4),
        autonomous: true,
    };

    ai.autonomous_loop(req).await.map_err(|e| e.to_string())
}

fn get_project_context(root_path: &str, active_file: Option<String>) -> String {
    use walkdir::WalkDir;
    let mut context = String::new();

    context.push_str("### Project Structure\n");
    for entry in WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .take(100)
    {
        let depth = entry.depth();
        let name = entry.file_name().to_string_lossy();
        context.push_str(&format!("{}{}\n", "  ".repeat(depth), name));
    }

    context.push_str("\n### Git Status\n");
    let git_output = Command::new("git")
        .args(["status", "--short"])
        .current_dir(root_path)
        .output();

    if let Ok(output) = git_output {
        context.push_str(&String::from_utf8_lossy(&output.stdout));
    }

    if let Some(path) = active_file {
        context.push_str(&format!("\n### Active File: {}\n", path));
        if let Ok(content) = std::fs::read_to_string(&path) {
            let head = content.lines().take(50).collect::<Vec<_>>().join("\n");
            context.push_str(&format!("```\n{}\n```\n", head));
        }
    }

    context
}

#[tauri::command]
async fn register_ida_pro(state: State<'_, EditorState>, python_path: String, script_path: String) -> Result<(), String> {
    let ai = state._sentient.lock().await;
    ai.register_mcp_server(McpServerConfig {
        name: "ida-pro".to_string(),
        command: python_path,
        args: vec![script_path],
    }).await.map_err(|e| e.to_string())
}

#[tauri::command]
fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Option<String> {
    let registry = state.keybindings.lock().unwrap();
    registry.resolve_key(&key, &state.context_keys)
}

#[tauri::command]
fn ai_execute_command(command: String) -> Result<String, String> {
    let output = if cfg!(target_os = "windows") {
        Command::new("powershell")
            .args(["-Command", &command])
            .output()
    } else {
        Command::new("sh")
            .args(["-c", &command])
            .output()
    }.map_err(|e: std::io::Error| e.to_string())?;

    let out = String::from_utf8_lossy(&output.stdout).to_string();
    let err = String::from_utf8_lossy(&output.stderr).to_string();
    Ok(format!("Stdout: {}\nStderr: {}", out, err))
}

#[tauri::command]
fn ai_modify_file(path: String, target: String, replacement: String) -> Result<(), String> {
    let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let new_content = content.replace(&target, &replacement);
    if content == new_content {
        return Err("Target string not found in file".to_string());
    }
    fs::write(&path, new_content).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn ext_host_init(state: State<'_, EditorState>, app: tauri::AppHandle) -> Result<(), String> {
    let mut host = state.ext_host.lock().unwrap();
    host.start(app).map_err(|e| e.to_string())
}

#[tauri::command]
fn ext_host_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut host = state.ext_host.lock().unwrap();
    host.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
async fn search_marketplace(query: String) -> Result<Value, String> {
    // Minimal OpenVSX search integration.
    // NOTE: This is intentionally simple and does not change any AI / API scanning logic.
    let url = format!(
        "https://open-vsx.org/api/-/search?query={}&size=20",
        urlencoding::encode(&query)
    );

    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("Marketplace request failed: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("Marketplace HTTP error: {}", resp.status()));
    }

    let json: Value = resp.json().await.map_err(|e| format!("Invalid marketplace JSON: {}", e))?;

    // Normalize into { results: [...] } for the frontend.
    let results = if let Some(arr) = json.get("results").and_then(|v| v.as_array()) {
        Value::Array(arr.clone())
    } else if let Some(arr) = json.get("extensions").and_then(|v| v.as_array()) {
        Value::Array(arr.clone())
    } else if json.is_array() {
        json.clone()
    } else {
        Value::Array(vec![])
    };

    Ok(serde_json::json!({ "results": results }))
}

#[tauri::command]
async fn install_extension(state: State<'_, EditorState>, download_url: String, name: String) -> Result<(), String> {
    let resp = reqwest::get(&download_url)
        .await
        .map_err(|e| format!("Failed to download VSIX: {}", e))?;
    if !resp.status().is_success() {
        return Err(format!("VSIX download failed: {}", resp.status()));
    }
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("Failed to read VSIX bytes: {}", e))?;

    let ext_dir = state.config_dir.join("extensions").join(&name);
    extract_vsix_bytes(&bytes, &ext_dir)?;

    Ok(())
}

fn extract_vsix_bytes(bytes: &[u8], dest: &std::path::Path) -> Result<(), String> {
    use std::io::Cursor;
    let reader = Cursor::new(bytes);
    let mut archive = zip::ZipArchive::new(reader).map_err(|e| format!("Failed to open ZIP: {}", e))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Failed to read ZIP entry: {}", e))?;
        let outpath = match file.enclosed_name() {
            Some(path) => {
                // VS Code extensions are usually inside an "extension/" folder in the VSIX
                let path_str = path.to_string_lossy();
                if path_str.starts_with("extension/") {
                    dest.join(&path_str[10..])
                } else {
                    continue; // Skip files outside the extension/ folder (manifests, etc.)
                }
            },
            None => continue,
        };

        if file.name().ends_with('/') {
            fs::create_dir_all(&outpath).map_err(|e| format!("Failed to create dir: {}", e))?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).map_err(|e| format!("Failed to create parent dir: {}", e))?;
                }
            }
            let mut outfile = fs::File::create(&outpath).map_err(|e| format!("Failed to create file: {}", e))?;
            std::io::copy(&mut file, &mut outfile).map_err(|e| format!("Failed to extract file: {}", e))?;
        }
    }
    Ok(())
}

#[tauri::command]
async fn install_vsix(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let src = PathBuf::from(&path);
    if !src.exists() {
        return Err(format!("VSIX not found at {}", path));
    }

    let bytes = fs::read(&src).map_err(|e| format!("Failed to read VSIX: {}", e))?;
    let name = src.file_stem().unwrap_or_default().to_string_lossy().to_string();
    let ext_dir = state.config_dir.join("extensions").join(&name);
    
    extract_vsix_bytes(&bytes, &ext_dir)?;
    Ok(())
}

#[tauri::command]
fn get_running_extensions(state: State<'_, EditorState>) -> Vec<Value> {
    let ext_dir = state.config_dir.join("extensions");
    let mut extensions = Vec::new();
    if let Ok(entries) = fs::read_dir(ext_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let pkg_path = path.join("package.json");
                if let Ok(content) = fs::read_to_string(&pkg_path) {
                    if let Ok(pkg) = serde_json::from_str::<Value>(&content) {
                        let mut ext_data = serde_json::json!({
                            "name": pkg.get("displayName").or(pkg.get("name")).and_then(|v| v.as_str()).unwrap_or("Unknown"),
                            "version": pkg.get("version").and_then(|v| v.as_str()).unwrap_or("0.0.0"),
                            "publisher": pkg.get("publisher").and_then(|v| v.as_str()).unwrap_or("Unknown"),
                            "description": pkg.get("description").and_then(|v| v.as_str()).unwrap_or(""),
                            "id": pkg.get("name").and_then(|v| v.as_str()).unwrap_or(""),
                        });

                        // Try to find an icon
                        if let Some(icon_rel) = pkg.get("icon").and_then(|v| v.as_str()) {
                            let icon_path = path.join(icon_rel);
                            if icon_path.exists() {
                                if let Ok(bytes) = fs::read(icon_path) {
                                    use base64::{Engine as _, engine::general_purpose};
                                    let b64 = general_purpose::STANDARD.encode(bytes);
                                    ext_data.as_object_mut().unwrap().insert("base64_icon".to_string(), Value::String(format!("data:image/png;base64,{}", b64)));
                                }
                            }
                        }
                        extensions.push(ext_data);
                    }
                }
            }
        }
    }
    extensions
}

#[tauri::command]
fn get_icon_theme_mapping(state: State<'_, EditorState>) -> Result<Value, String> {
    let ext_dir = state.config_dir.join("extensions");
    let active_theme_label = state.icon_theme.lock().unwrap().clone();

    if let Ok(entries) = fs::read_dir(&ext_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let pkg_path = path.join("package.json");
                if let Ok(content) = fs::read_to_string(&pkg_path) {
                    if let Ok(pkg) = serde_json::from_str::<Value>(&content) {
                        if let Some(contributes) = pkg.get("contributes") {
                            if let Some(icon_themes) = contributes.get("iconThemes").and_then(|it| it.as_array()) {
                                for theme in icon_themes {
                                    if theme.get("label").and_then(|l| l.as_str()) == Some(&active_theme_label) {
                                        let theme_rel_path = theme.get("path").and_then(|p| p.as_str()).unwrap_or("");
                                        let theme_path = path.join(theme_rel_path);
                                        
                                        let theme_content = fs::read_to_string(&theme_path).map_err(|e| format!("Failed to read icon theme file: {}", e))?;
                                        
                                        // Simple comment stripping for the icon theme JSON as well
                                        let re_block = regex::Regex::new(r"/\*[\s\S]*?\*/").unwrap();
                                        let stripped_block = re_block.replace_all(&theme_content, "");
                                        let re_line = regex::Regex::new(r"(?m)(^|\s)//.*$").unwrap();
                                        let stripped = re_line.replace_all(&stripped_block, "$1");

                                        match serde_json::from_str::<Value>(&stripped) {
                                            Ok(mut theme_json) => {
                                                // We need to resolve all icon paths relative to the theme file dir
                                                let theme_dir = theme_path.parent().unwrap_or(&path);
                                                
                                                if let Some(defs) = theme_json.get_mut("iconDefinitions").and_then(|d| d.as_object_mut()) {
                                                    for (_key, def) in defs {
                                                        if let Some(icon_path_rel) = def.get_mut("iconPath").and_then(|p| p.as_str()) {
                                                            let abs_path = theme_dir.join(icon_path_rel);
                                                            if abs_path.exists() {
                                                                if let Ok(bytes) = fs::read(&abs_path) {
                                                                    use base64::{Engine as _, engine::general_purpose};
                                                                    let b64 = general_purpose::STANDARD.encode(bytes);
                                                                    let mime = if icon_path_rel.ends_with(".svg") { "image/svg+xml" } else { "image/png" };
                                                                    *def.get_mut("iconPath").unwrap() = Value::String(format!("data:{};base64,{}", mime, b64));
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                                return Ok(theme_json);
                                            },
                                            Err(e) => return Err(format!("Failed to parse icon theme JSON: {}", e)),
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(serde_json::json!({}))
}

#[tauri::command]
fn get_installed_themes(state: State<'_, EditorState>) -> Result<Vec<Value>, String> {
    let ext_dir = state.config_dir.join("extensions");
    let mut themes = Vec::new();

    if !ext_dir.exists() {
        return Err(format!("Extensions directory not found at {:?}", ext_dir));
    }

    if let Ok(entries) = fs::read_dir(&ext_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                let pkg_path = path.join("package.json");
                if let Ok(content) = fs::read_to_string(&pkg_path) {
                    if let Ok(pkg) = serde_json::from_str::<Value>(&content) {
                        if let Some(contributes) = pkg.get("contributes") {
                            if let Some(ext_themes) = contributes.get("themes").and_then(|t| t.as_array()) {
                                for theme in ext_themes {
                                    if let Some(label) = theme.get("label") {
                                        let theme_path = theme.get("path").and_then(|p| p.as_str()).unwrap_or("");
                                        themes.push(serde_json::json!({
                                            "label": label,
                                            "path": path.join(theme_path).to_string_lossy(),
                                            "extension": pkg.get("name").and_then(|n| n.as_str()).unwrap_or("unknown")
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(themes)
}

#[tauri::command]
fn load_extension_theme(path: String) -> Result<Value, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read theme file: {}", e))?;
    
    // Improved regex to strip comments - try to avoid stripping // inside strings
    // This is still a heuristic, but better.
    // Matches block comments /* ... */ and line comments // ... that are NOT preceded by a colon (to avoid URLs)
    // Actually, a more reliable way since we don't have a full parser:
    let re_block = regex::Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let stripped_block = re_block.replace_all(&content, "");
    
    // For line comments, only strip if there's whitespace or start of line before //
    let re_line = regex::Regex::new(r"(?m)(^|\s)//.*$").unwrap();
    let stripped = re_line.replace_all(&stripped_block, "$1");
    
    let json: Value = match serde_json::from_str::<Value>(&stripped) {
        Ok(v) => v,
        Err(_) => {
            // Fallback: try parsing original content without stripping if stripping failed
            serde_json::from_str::<Value>(&content).map_err(|e| format!("Failed to parse theme JSON: {}", e))?
        }
    };
    
    if let Some(colors) = json.get("colors") {
        Ok(colors.clone())
    } else {
        Ok(serde_json::json!({}))
    }
}

#[tauri::command]
fn debug_start(state: State<'_, EditorState>, app: tauri::AppHandle, adapter_path: String) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.start_session(&adapter_path, app).map_err(|e| e.to_string())
}

#[tauri::command]
fn debug_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
fn debug_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.stop_session().map_err(|e| e.to_string())
}

#[tauri::command]
fn check_activation_event(state: State<'_, EditorState>, event: String) {
    let mut am = state.activation_manager.lock().unwrap();
    am.check_activation_requests(&event, state.ext_host.clone());
}

#[tauri::command]
async fn get_file_tree(
    state: tauri::State<'_, EditorState>,
) -> Result<Vec<FileEntry>, String> {
    use std::collections::HashMap;
    use std::fs;

    let root = {
        let root_guard = state.active_root.lock().unwrap();
        root_guard
            .clone()
            .ok_or_else(|| "No project open".to_string())?
    };

    // Walk filesystem and build a tree, similar to the previous Zed-based implementation,
    // but purely using std::fs.
    let mut nodes: HashMap<PathBuf, FileEntry> = HashMap::new();

    for entry in walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_map(|e| e.ok())
        .take(10_000)
    {
        let path = entry.into_path();
        let rel = path
            .strip_prefix(&root)
            .unwrap_or(&path)
            .to_path_buf();

        let name = if rel.components().count() == 0 {
            root.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        } else {
            rel.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        };

        let is_dir = fs::metadata(&path)
            .map(|m| m.is_dir())
            .unwrap_or(false);

        nodes.insert(
            path.clone(),
            FileEntry {
                name,
                path: path.to_string_lossy().to_string(),
                is_dir,
                children: if is_dir { Some(Vec::new()) } else { None },
            },
        );
    }

    let mut paths: Vec<_> = nodes.keys().cloned().collect();
    paths.sort_by_key(|p| p.components().count());
    paths.reverse();

    let mut roots = Vec::new();
    for path in paths {
        let mut entry = nodes.remove(&path).unwrap();
        if let Some(children) = &mut entry.children {
            children.sort_by(|a, b| {
                if a.is_dir != b.is_dir {
                    b.is_dir.cmp(&a.is_dir)
                } else {
                    a.name.cmp(&b.name)
                }
            });
        }

        if let Some(parent) = path.parent() {
            if let Some(parent_entry) = nodes.get_mut(parent) {
                if let Some(children) = &mut parent_entry.children {
                    children.push(entry);
                }
                continue;
            }
        }
        roots.push(entry);
    }

    Ok(roots)
}

#[tauri::command]
async fn backend_ping(_state: tauri::State<'_, EditorState>) -> Result<String, String> {
    Ok("Pong from VSCodium Rust backend (no Zed)".into())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tauri::command]
async fn get_emulator_screenshot(state: tauri::State<'_, EditorState>, device_id: String) -> Result<String, String> {
    let sdk_path = state.android_sdk_path.lock().unwrap().clone().unwrap_or_default();
    let adb_path = if sdk_path.is_empty() {
        "adb".to_string()
    } else {
        let p = std::path::Path::new(&sdk_path);
        if p.join("platform-tools").join("adb").exists() {
            p.join("platform-tools").join("adb").to_string_lossy().to_string()
        } else if p.join("adb").exists() {
            p.join("adb").to_string_lossy().to_string()
        } else {
            "adb".to_string()
        }
    };

    let output = std::process::Command::new(adb_path)
        .arg("-s")
        .arg(&device_id)
        .arg("exec-out")
        .arg("screencap")
        .arg("-p")
        .output()
        .map_err(|e| format!("Failed to run adb screencap: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    use base64::{Engine as _, engine::general_purpose};
    let b64 = general_purpose::STANDARD.encode(&output.stdout);
    Ok(format!("data:image/png;base64,{}", b64))
}

#[tauri::command]
async fn emulator_tap(state: tauri::State<'_, EditorState>, device_id: String, x: i32, y: i32) -> Result<(), String> {
    let sdk_path = state.android_sdk_path.lock().unwrap().clone().unwrap_or_default();
    let adb_path = if sdk_path.is_empty() {
        "adb".to_string()
    } else {
        let p = std::path::Path::new(&sdk_path);
        if p.join("platform-tools").join("adb").exists() {
            p.join("platform-tools").join("adb").to_string_lossy().to_string()
        } else if p.join("adb").exists() {
            p.join("adb").to_string_lossy().to_string()
        } else {
            "adb".to_string()
        }
    };

    std::process::Command::new(adb_path)
        .arg("-s")
        .arg(&device_id)
        .arg("shell")
        .arg("input")
        .arg("tap")
        .arg(x.to_string())
        .arg(y.to_string())
        .spawn()
        .map_err(|e| format!("Failed to run adb tap: {}", e))?;

    Ok(())
}

pub fn run() {
    println!("VSCodium Rust Tauri starting...");
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .manage(BrowserState::new())
        .setup(|app| {
            app.manage(EditorState::new(&app.handle()));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_file, save_file, get_highlights, sync_content, list_directory,
            open_folder, search_project, switch_to_buffer, get_git_branch, 
            git_status, git_stage, git_unstage, git_commit, get_settings, 
            update_settings, spawn_terminal, write_to_terminal, lsp_start, 
            lsp_send_request, lsp_stop, set_context_key, evaluate_when_clause, 
            resize_terminal, ext_host_init, ext_host_send, resolve_keybinding, 
            search_marketplace, install_extension, install_vsix, get_running_extensions, 
            debug_start, debug_send, debug_stop, check_activation_event, 
            get_process_stats, get_config_path, get_api_keys, save_api_keys, set_ai_model,
            adb_list_devices, set_active_device, adb_install_and_run, get_android_config, set_android_sdk_path,
            adb_list_emulators, spawn_emulator, set_active_root,
            rename_path, delete_path, create_file, create_dir, create_directory,
            get_file_tree, read_file, write_file, ai_chat, register_ida_pro,
            browser::browser_open, browser::browser_navigate, browser::browser_screenshot, browser::browser_close,
            backend_ping, ai_execute_command, ai_modify_file, get_installed_themes, load_extension_theme, get_icon_theme_mapping, get_git_history,
            get_emulator_screenshot, emulator_tap
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ---------------------------
// Tests for core Rust commands
// ---------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::process::Command;

    fn tmp_root() -> PathBuf {
        std::env::temp_dir().join("vscodium_rust_tests")
    }

    #[test]
    fn create_and_delete_file_work() {
        let dir = tmp_root();
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("test_file.txt");
        let path_str = path.to_string_lossy().to_string();

        // Ensure clean start
        let _ = fs::remove_file(&path);

        create_file(path_str.clone()).expect("create_file failed");
        assert!(path.is_file(), "file should exist after create_file");

        delete_path(path_str).expect("delete_path failed");
        assert!(!path.exists(), "file should be gone after delete_path");
    }

    #[test]
    fn create_dir_and_rename_work() {
        let dir = tmp_root();
        fs::create_dir_all(&dir).unwrap();
        let original = dir.join("orig_dir");
        let renamed = dir.join("renamed_dir");

        let orig_str = original.to_string_lossy().to_string();
        let renamed_str = renamed.to_string_lossy().to_string();

        let _ = fs::remove_dir_all(&original);
        let _ = fs::remove_dir_all(&renamed);

        create_dir(orig_str.clone()).expect("create_dir failed");
        assert!(original.is_dir(), "directory should exist after create_dir");

        rename_path(orig_str.clone(), renamed_str.clone()).expect("rename_path failed");
        assert!(!original.exists(), "original dir should be gone after rename");
        assert!(renamed.is_dir(), "renamed dir should exist");

        delete_path(renamed_str).expect("delete_path on dir failed");
        assert!(!renamed.exists(), "renamed dir should be deleted");
    }

    #[test]
    fn git_status_reports_new_file() {
        use crate::git::GitManager;

        let repo_root = tmp_root().join("git_repo");
        if repo_root.exists() {
            let _ = fs::remove_dir_all(&repo_root);
        }
        fs::create_dir_all(&repo_root).unwrap();

        // Initialize a new git repository
        let status = Command::new("git")
            .arg("init")
            .current_dir(&repo_root)
            .status()
            .expect("failed to run git init");
        if !status.success() {
            // Skip test if git is not working
            return;
        }

        // Create an untracked file
        let file_path = repo_root.join("main.txt");
        fs::write(&file_path, "hello").unwrap();

        let manager = GitManager::new();
        let results = manager
            .get_status(&repo_root)
            .expect("git_status failed");

        assert!(
            results.iter().any(|f| f.path == "main.txt"),
            "git_status should include main.txt"
        );
    }
}
