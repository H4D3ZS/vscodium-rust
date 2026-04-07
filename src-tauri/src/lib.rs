use base64::{engine::general_purpose, Engine as _};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tauri::State;
use tauri::{Emitter, Listener};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};
use tree_sitter_rust::LANGUAGE;

mod hunter;
use hunter::ApiRadarHunter;
mod ai_auth;

pub mod ai_engine;
use ai_engine::{AiRequest, ChatMessage, Sentient};
mod ai_tools;
pub mod ane;
pub mod process_ext;
use crate::process_ext::CommandExtHidden;
pub mod context_quantizer;
pub mod domain;
mod mcp_client;
mod mcp_registry;
pub mod memory_optimizer;
mod memory_store;
use crate::memory_store::SemanticSlot;
mod task_planner;
// mod browser_bridge; // Redundant, functionality in browser.rs
mod context_indexer;
mod tool_invoker;
mod visual_lab;
mod attachment_manager;
use attachment_manager::{AttachmentManager, select_and_process_attachment};
mod knowledge_distiller;
use knowledge_distiller::KnowledgeDistiller;
mod vision_bridge;
mod workflow_engine;
use mcp_registry::McpRegistry;
use context_indexer::ContextIndexer;

mod lsp;
use lsp::LspClient;

mod context_key;
use context_key::{ContextKeyRegistry, ContextValue};

mod extension_host;
use extension_host::ExtensionHostManager;

mod git;
use git::GitManager;

mod performance;
use performance::PerformanceMonitor;

mod keybindings;
use keybindings::KeybindingRegistry;

mod debug_adapter;
use debug_adapter::DebugManager;

#[allow(dead_code)]
extern "system" {
    fn GetCurrentProcess() -> isize;
    fn SetProcessWorkingSetSize(
        hProcess: isize,
        dwMinimumWorkingSetSize: usize,
        dwMaximumWorkingSetSize: usize,
    ) -> i32;
}

mod activation;
use activation::ActivationManager;

mod marketplace;

mod browser;

pub mod specs_db;
mod workers;
mod ai_prompts;
mod specs_commands;
mod rules_engine;

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

#[derive(Serialize, Clone)]
struct HuntProgress {
    msg: String,
}

pub(crate) struct EditorState {
    buffers: Mutex<HashMap<String, Rope>>,
    active_path: Mutex<Option<String>>,
    settings: Mutex<Settings>,
    terminal_masters: Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    terminal_writers: Mutex<HashMap<String, Box<dyn Write + Send>>>,
    terminal_processes: Mutex<HashMap<String, Box<dyn Child + Send>>>,
    lsp_client: Arc<Mutex<LspClient>>,
    context_keys: Arc<ContextKeyRegistry>,
    ext_host: Arc<Mutex<ExtensionHostManager>>,
    keybindings: Arc<Mutex<KeybindingRegistry>>,
    debug_manager: Arc<Mutex<DebugManager>>,
    activation_manager: Arc<Mutex<ActivationManager>>,
    perf_monitor: Arc<PerformanceMonitor>,
    pub ai_engine: Arc<Sentient>,
    ollama_url: Mutex<String>,
    config_dir: PathBuf,
    active_root: Mutex<Option<PathBuf>>,
    current_model: Mutex<String>,
    active_device: Mutex<Option<String>>,
    android_sdk_path: Mutex<Option<String>>,
    auth_state: Arc<ai_auth::AuthState>,
    browser_state: Arc<browser::BrowserState>,
    mcp_registry: Arc<mcp_registry::McpRegistry>,
    terminal_buffers: Mutex<HashMap<String, Vec<String>>>,
    memory_optimizer: Arc<memory_optimizer::MemoryOptimizer>,
    advisor_model: Mutex<Option<String>>,
    pub specs_db: Arc<specs_db::SpecDb>,
    #[allow(dead_code)]
    pub worker_manager: Arc<workers::WorkerManager>,
    pub attachment_manager: Arc<AttachmentManager>,
    #[allow(dead_code)]
    pub knowledge_distiller: Arc<KnowledgeDistiller>,
}

impl EditorState {
    pub fn terminal_read_output(&self, id: String) -> Result<String, String> {
        let buffers = self.terminal_buffers.lock().map_err(|e| e.to_string())?;
        let history = buffers
            .get(&id)
            .ok_or_else(|| "Terminal not found".to_string())?;
        Ok(history.join(""))
    }
}

impl EditorState {
    fn new(app: &tauri::AppHandle) -> Self {
        println!("[DEBUG] Initializing EditorState...");
        let config_dir = app
            .path()
            .app_config_dir()
            .unwrap_or_else(|_| PathBuf::from(".config"));
        println!("[DEBUG] Config dir: {:?}", config_dir);
        if !config_dir.exists() {
            let _ = fs::create_dir_all(&config_dir);
        }

        let mut root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

        // Find project root by searching for package.json or .git up to 3 levels
        let mut check_path = root.clone();
        for _ in 0..3 {
            if check_path.join("package.json").exists() || check_path.join(".git").exists() {
                root = check_path;
                break;
            }
            if let Some(parent) = check_path.parent() {
                check_path = parent.to_path_buf();
            } else {
                break;
            }
        }
        let auth_state = Arc::new(ai_auth::AuthState::new());
        let browser_state = Arc::new(browser::BrowserState::new());
        let memory_optimizer = Arc::new(memory_optimizer::MemoryOptimizer::new());

        let git_manager = Arc::new(crate::git::GitManager::new());
        let perf_monitor = Arc::new(crate::performance::PerformanceMonitor::new());
        let attachment_manager = Arc::new(AttachmentManager::new());
        let knowledge_distiller = Arc::new(KnowledgeDistiller::new(&root));
        
        let sentient = Arc::new(Sentient::new(
            "".to_string(), // Initial empty API key
            root.clone(),
            auth_state.clone(),
            browser_state.clone(),
            git_manager.clone(),
            config_dir.clone(),
            memory_optimizer.clone(),
            perf_monitor.clone(),
            attachment_manager.clone(),
            knowledge_distiller.clone(),
        ));
        println!("[DEBUG] Sentient initialized");
        sentient.set_app_handle(app.clone());

        // Initialize and start Omni-Context Indexer (Phase 44)
        let context_indexer = Arc::new(ContextIndexer::new(
            sentient.memory_store.clone(),
            root.clone(),
        ));
        let ci_for_spawn = context_indexer.clone();
        tauri::async_runtime::spawn(async move {
            ci_for_spawn.start_background_indexing().await;
        });

        app.manage(context_indexer);

        let mut ext_dirs = vec![config_dir.join("extensions")];
        let builtin_ext_dir = root.join("vscode").join("extensions");
        if builtin_ext_dir.exists() {
            ext_dirs.push(builtin_ext_dir);
        }

        let specs_db = Arc::new(specs_db::SpecDb::new(config_dir.join("specs.db")).expect("Failed to init specs DB"));
        let worker_manager = Arc::new(workers::WorkerManager::new(specs_db.clone(), sentient.clone(), root.clone()));

        // Start worker loop
        let wm_clone = worker_manager.clone();
        tauri::async_runtime::spawn(async move {
            wm_clone.start_loop().await;
        });

        Self {
            buffers: Mutex::new(HashMap::new()),
            active_path: Mutex::new(None),
            settings: Mutex::new(Settings {
                theme: "vs-dark".to_string(),
                font_size: 14,
            }),
            terminal_masters: Mutex::new(HashMap::new()),
            terminal_writers: Mutex::new(HashMap::new()),
            terminal_processes: Mutex::new(HashMap::new()),
            lsp_client: Arc::new(Mutex::new(LspClient::new())),
            context_keys: Arc::new(ContextKeyRegistry::new()),
            ext_host: Arc::new(Mutex::new(ExtensionHostManager::new(ext_dirs))),
            keybindings: Arc::new(Mutex::new(KeybindingRegistry::new())),
            debug_manager: Arc::new(Mutex::new(DebugManager::new())),
            activation_manager: Arc::new(Mutex::new(ActivationManager::new())),
            perf_monitor: Arc::new(PerformanceMonitor::new()),
            ai_engine: sentient,
            ollama_url: Mutex::new("http://127.0.0.1:11434".to_string()),
            config_dir: config_dir.clone(),
            active_root: Mutex::new(None),
            current_model: Mutex::new("gpt-4o".to_string()),
            active_device: Mutex::new(None),
            android_sdk_path: Mutex::new(None),
            auth_state,
            browser_state,
            mcp_registry: Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json"))),
            terminal_buffers: Mutex::new(HashMap::new()),
            memory_optimizer,
            advisor_model: Mutex::new(None),
            specs_db,
            worker_manager,
            attachment_manager,
            knowledge_distiller,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
    pub is_expanded: Option<bool>,
    pub children: Option<Vec<FileEntry>>,
}

#[tauri::command]
async fn list_chat_sessions(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.ai_engine.memory_store.list_sessions().await))
}

#[tauri::command]
async fn load_chat_session(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    state.ai_engine.memory_store.load_from_path(PathBuf::from(path)).await;
    Ok(())
}

#[tauri::command]
async fn archive_chat_session(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.memory_store.archive_current_session().await;
    Ok(())
}

#[tauri::command]
async fn create_new_session(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.memory_store.create_new_session().await;
    Ok(())
}

#[tauri::command]
async fn get_agent_messages(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.ai_engine.memory_store.get_messages().await))
}

#[tauri::command]
async fn get_brain_telemetry(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(state.ai_engine.memory_store.get_brain_telemetry().await)
}

#[derive(Serialize, Deserialize)]
struct AiResponse {
    content: String,
}

#[tauri::command]
fn mount_aim_memory(project_path: String) -> Result<String, String> {
    let aim_path = format!("{}\\.aim\\memory.aim", project_path);
    let file = std::fs::File::open(&aim_path).map_err(|e| format!("Failed to load memory.aim: {}", e))?;
    
    // Execute Native Zero-Copy RAM Mapping directly inside the Core IDE execution structure
    let mmap = unsafe { memmap2::Mmap::map(&file).map_err(|e| format!("Mmap core failure: {}", e))? };
    let bytes = &mmap[..];
    
    let mut header_end = 0;
    for (i, &b) in bytes.iter().enumerate() {
        if b == b'}' {
            header_end = i + 1;
            break;
        }
    }
    
    if header_end == 0 || header_end + (1536 * 4) > bytes.len() {
        return Err("Invalid .aim structural limits".to_string());
    }
    
    let tensor_start = &bytes[header_end];
    let vector_ptr = tensor_start as *const u8 as *const f32;
    
    Ok(format!("🧠 [ANTIGRAVITY CORE] Neural .aim VFS Mounted! OS RAM physically bound to IDE dynamically. Zero-Copy Pointer Extracted: {:?}", vector_ptr))
}

#[tauri::command]
fn open_file(state: State<'_, EditorState>, path: String) -> Result<String, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut buffers = state.buffers.lock().unwrap();
    buffers.insert(path.clone(), Rope::from_str(&content));

    let mut active = state.active_path.lock().unwrap();
    *active = Some(path);

    Ok(content)
}

#[tauri::command]
fn save_file(state: State<'_, EditorState>, path: String, content: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| format!("Failed to write file: {}", e))?;
    let mut buffers = state.buffers.lock().unwrap();
    buffers.insert(path, Rope::from_str(&content));
    Ok(())
}

#[tauri::command]
fn get_highlights(code: String) -> Result<Value, String> {
    let mut parser = Parser::new();
    let lang: tree_sitter::Language = LANGUAGE.into(); // Convert for 0.26 API
    parser.set_language(&lang).map_err(|e| e.to_string())?;
    let tree = parser.parse(&code, None).ok_or("Failed to parse code")?;

    let query = Query::new(&lang, "(function_item) @function (struct_item) @struct")
        .map_err(|e| e.to_string())?;
    let mut cursor = QueryCursor::new();
    let mut matches = cursor.matches(&query, tree.root_node(), code.as_bytes());

    let mut highlights = Vec::new();
    while let Some(m) = matches.next() {
        for capture in m.captures {
            highlights.push(json!({
                "start": capture.node.start_byte(),
                "end": capture.node.end_byte(),
                "tag": query.capture_names()[capture.index as usize]
            }));
        }
    }
    Ok(json!(highlights))
}

#[tauri::command]
fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let entries = fs::read_dir(&path).map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut results = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        let meta = entry.metadata().map_err(|e| e.to_string())?;
        results.push(FileEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path().to_string_lossy().to_string(),
            is_dir: meta.is_dir(),
            is_expanded: Some(false),
            children: None,
        });
    }
    Ok(results)
}

#[tauri::command]
async fn open_folder(
    app: tauri::AppHandle,
    state: State<'_, EditorState>,
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
        let mut root = state.active_root.lock().unwrap();
        *root = Some(path.clone());
        state.ai_engine.set_root_path(path.clone());
        return Ok(Some(path.to_string_lossy().to_string()));
    }
    Ok(None)
}

#[tauri::command]
fn switch_to_buffer(state: State<'_, EditorState>, path: String) -> Result<String, String> {
    let buffers = state.buffers.lock().unwrap();
    if let Some(rope) = buffers.get(&path) {
        let mut active = state.active_path.lock().unwrap();
        *active = Some(path);
        Ok(rope.to_string())
    } else {
        Err("Buffer not found".to_string())
    }
}

#[tauri::command]
fn get_settings(state: State<'_, EditorState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(state: State<'_, EditorState>, settings: Settings) {
    let mut s = state.settings.lock().unwrap();
    *s = settings;
}

#[tauri::command]
fn lsp_start(
    state: State<'_, EditorState>,
    app: tauri::AppHandle,
    command: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.start(&command, app).map_err(|e| e.to_string())
}

#[tauri::command]
fn lsp_send_request(
    state: State<'_, EditorState>,
    id: i32,
    method: String,
    params: Value,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.send_request(id, &method, params)
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn lsp_stop(state: State<'_, EditorState>) {
    let mut lsp = state.lsp_client.lock().unwrap();
    lsp.stop();
}

#[tauri::command]
fn set_context_key(state: State<'_, EditorState>, key: String, value: Value) {
    let context_val = match value {
        Value::Bool(b) => ContextValue::Bool(b),
        Value::String(s) => ContextValue::String(s),
        Value::Number(n) => ContextValue::Int(n.as_i64().unwrap_or(0) as i32),
        _ => ContextValue::Bool(false),
    };
    state.context_keys.set(key, context_val);
}

#[tauri::command]
fn evaluate_when_clause(state: State<'_, EditorState>, clause: String) -> bool {
    state.context_keys.evaluate(&clause)
}

#[tauri::command]
fn ext_host_init(state: State<'_, EditorState>, app: tauri::AppHandle) -> Result<(), String> {
    let mut eh = state.ext_host.lock().unwrap();
    eh.scan_extensions().map_err(|e| e.to_string())?;
    eh.start(app).map_err(|e| e.to_string())
}

#[tauri::command]
fn ext_host_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut eh = state.ext_host.lock().unwrap();
    eh.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Option<String> {
    let kb = state.keybindings.lock().unwrap();
    kb.resolve_key(&key, &state.context_keys)
}

#[tauri::command]
async fn search_extensions(
    query: String,
) -> Result<Vec<marketplace::MarketplaceExtension>, String> {
    marketplace::search_extensions(query)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn install_extension(
    state: State<'_, EditorState>,
    publisher: String,
    name: String,
    version: String,
) -> Result<extension_host::ExtensionMetadata, String> {
    let extensions_dir = {
        let eh = state.ext_host.lock().unwrap();
        eh.primary_extensions_dir()
    };

    let _id = marketplace::install_extension(
        publisher.clone(),
        name.clone(),
        version.clone(),
        extensions_dir.clone(),
    )
    .await
    .map_err(|e| e.to_string())?;

    // Dynamically load the extension
    let target_dir = extensions_dir.join(format!("{}.{}-{}", publisher, name, version));
    if !target_dir.exists() {
        fs::create_dir_all(&target_dir).map_err(|e| e.to_string())?;
    }
    let mut package_json_path = target_dir.join("package.json");
    if !package_json_path.exists() {
        package_json_path = target_dir.join("extension").join("package.json");
    }

    if package_json_path.exists() {
        let content = fs::read_to_string(&package_json_path).map_err(|e| e.to_string())?;
        if let Ok(mut meta) = serde_json::from_str::<extension_host::ExtensionMetadata>(&content) {
            meta.extension_path = package_json_path.parent().unwrap().to_path_buf();
            if meta.id.is_empty() {
                meta.id = format!("{}.{}", publisher, name);
            }
            let mut eh = state.ext_host.lock().unwrap();
            let _ = eh.add_extension(meta.clone());
            return Ok(meta);
        }
    }

    Err("Failed to load installed extension metadata".to_string())
}

#[tauri::command]
async fn get_popular_extensions() -> Result<Vec<marketplace::MarketplaceExtension>, String> {
    marketplace::get_popular_extensions()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_extension_details(id: String) -> Result<Value, String> {
    let parts: Vec<&str> = id.split('.').collect();
    if parts.len() < 2 {
        return Err("Invalid extension ID".to_string());
    }
    let publisher = parts[0].to_string();
    let name = parts[1..].join(".").to_string();
    marketplace::get_extension_details(publisher, name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn uninstall_extension(
    state: State<'_, EditorState>,
    publisher: String,
    name: String,
    version: Option<String>,
) -> Result<(), String> {
    let mut eh = state.ext_host.lock().unwrap();
    let extensions_dir = eh.primary_extensions_dir();

    let target_prefix = format!("{}.{}", publisher, name);
    println!("Uninstalling extension: {}.{} (v:{:?})", publisher, name, version);

    if let Ok(entries) = std::fs::read_dir(&extensions_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_dir() {
                let folder_name = path.file_name().unwrap_or_default().to_string_lossy();
                let matches = if folder_name.starts_with(&target_prefix) {
                    if let Some(ref v) = version {
                        folder_name.contains(v)
                    } else {
                        true
                    }
                } else {
                    false
                };

                if matches {
                    println!("Found matching extension folder: {}", folder_name);
                    // Try to delete. If it fails (file lock), try to rename it to hide it.
                    if let Err(e) = std::fs::remove_dir_all(&path) {
                        println!("Failed to delete extension dir {}: {}. Attempting rename fallback...", folder_name, e);
                        let deprecated_path = path.with_extension(format!("deprecated-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()));
                        if let Err(re) = std::fs::rename(&path, &deprecated_path) {
                            println!("Rename fallback failed for {}: {}", folder_name, re);
                            return Err(format!("Failed to uninstall extension: Folder is locked and cannot be removed or renamed. Please close any background processes or restart the IDE. Error: {}", e));
                        } else {
                            println!("Successfully renamed {} to {} to hide it.", folder_name, deprecated_path.display());
                        }
                    } else {
                        println!("Successfully deleted extension folder: {}", folder_name);
                    }
                }
            }
        }
    }

    let _ = eh.scan_extensions();
    println!("Scan extensions complete. Total running/installed: {}", eh.extensions.len());
    Ok(())
}

#[tauri::command]
fn get_installed_extensions(
    state: State<'_, EditorState>,
) -> Vec<extension_host::ExtensionMetadata> {
    let eh = state.ext_host.lock().unwrap();
    eh.extensions.clone()
}

#[tauri::command]
fn install_vsix(_state: State<'_, EditorState>, path: String) -> Result<(), String> {
    // Basic stub for manual VSIX installation
    println!("Installing VSIX from {}", path);
    Ok(())
}

#[tauri::command]
fn get_running_extensions(state: State<'_, EditorState>) -> Vec<extension_host::ExtensionMetadata> {
    let eh = state.ext_host.lock().unwrap();
    eh.extensions.clone()
}

#[tauri::command]
async fn compress_session_data(
    state: State<'_, EditorState>,
    key: String,
    data: String,
) -> Result<(), String> {
    state
        .memory_optimizer
        .compress_and_store(&key, &data)
        .await
        .map_err(|e: anyhow::Error| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn get_memory_savings(state: State<'_, EditorState>) -> Result<(usize, usize), String> {
    Ok(state.memory_optimizer.get_savings_report().await)
}

#[tauri::command]
fn get_process_stats(state: State<'_, EditorState>) -> performance::ProcessStats {
    state
        .perf_monitor
        .get_stats()
        .unwrap_or(performance::ProcessStats {
            memory_mb: 0,
            cpu_usage: 0.0,
            total_ram_gb: 0,
            available_ram_gb: 0,
        })
}

#[tauri::command]
fn get_config_path(state: State<'_, EditorState>) -> String {
    state.config_dir.to_string_lossy().to_string()
}

#[tauri::command]
fn get_api_keys(state: State<'_, EditorState>) -> Result<Value, String> {
    let path = state.config_dir.join("api_keys.json");
    let mut keys: serde_json::Map<String, Value> = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        serde_json::Map::new()
    };

    // Merge env var overrides (from hunt_api_keys or manual export)
    let env_mappings = vec![
        ("google", "GOOGLE_API_KEY"),
        ("anthropic", "ANTHROPIC_API_KEY"),
        ("openai", "OPENAI_API_KEY"),
        ("openrouter", "OPENROUTER_API_KEY"),
        ("mistral", "MISTRAL_API_KEY"),
        ("xai", "XAI_API_KEY"),
        ("groq", "GROQ_API_KEY"),
        ("alibaba", "ALIBABA_API_KEY"),
    ];

    for (field, env_var) in env_mappings {
        if let Ok(val) = std::env::var(env_var) {
            if !val.is_empty() && !keys.contains_key(field) {
                keys.insert(field.to_string(), json!(val));
            }
        }
    }

    Ok(json!(keys))
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeys {
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub google: Option<String>,
    pub alibaba: Option<String>,
    pub apiradar: Option<String>,
}

#[tauri::command]
async fn save_api_keys(
    state: State<'_, EditorState>,
    keys: Value,
) -> Result<HashMap<String, String>, String> {
    let mut keys: ApiKeys =
        serde_json::from_value(keys).map_err(|e| format!("Invalid keys format: {}", e))?;
    let mut results = HashMap::new();
    let hunter = ApiRadarHunter::new();

    // Validate OpenAI
    if let Some(ref k) = keys.openai {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("openai_api_key", k).await;
            if !alive {
                results.insert("openai".to_string(), format!("Dead: {}", details));
                keys.openai = None;
            } else {
                results.insert("openai".to_string(), "Alive".to_string());
                std::env::set_var("OPENAI_API_KEY", k);
            }
        }
    }
    // Validate Anthropic
    if let Some(ref k) = keys.anthropic {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("anthropic_api_key", k).await;
            if !alive {
                results.insert("anthropic".to_string(), format!("Dead: {}", details));
                keys.anthropic = None;
            } else {
                results.insert("anthropic".to_string(), "Alive".to_string());
                std::env::set_var("ANTHROPIC_API_KEY", k);
            }
        }
    }
    // Validate Google
    if let Some(ref k) = keys.google {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("google_api_key", k).await;
            if !alive {
                results.insert("google".to_string(), format!("Dead: {}", details));
                keys.google = None;
            } else {
                results.insert("google".to_string(), "Alive".to_string());
                std::env::set_var("GOOGLE_API_KEY", k);
            }
        }
    }

    // Save filtered keys
    let path = state.config_dir.join("api_keys.json");
    let contents = serde_json::to_string_pretty(&keys)
        .map_err(|e| format!("Failed to encode api keys: {}", e))?;
    fs::write(&path, contents).map_err(|e| format!("Failed to write api_keys.json: {}", e))?;

    Ok(results)
}

#[tauri::command]
async fn list_mcp_servers(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.mcp_registry.list_servers().await))
}

#[tauri::command]
async fn add_mcp_server(
    state: State<'_, EditorState>,
    name: String,
    config: mcp_registry::McpServerConfig,
) -> Result<(), String> {
    state
        .mcp_registry
        .add_server(name, config)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn remove_mcp_server(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state
        .mcp_registry
        .remove_server(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn set_ai_model(state: State<'_, EditorState>, model: String) -> Result<(), String> {
    let mut current = state.current_model.lock().unwrap();
    *current = model;
    Ok(())
}

#[tauri::command]
fn set_advisor_model(state: State<'_, EditorState>, model: Option<String>) -> Result<(), String> {
    let mut current = state.advisor_model.lock().unwrap();
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
            p.join("platform-tools")
                .join("adb")
                .to_string_lossy()
                .to_string()
        } else {
            "adb".to_string()
        }
    } else {
        "adb".to_string()
    };

    let output = std::process::Command::new(&adb_cmd)
        .hidden()
        .arg("devices")
        .output()
        .map_err(|e| format!("ADB error ({}): {}", adb_cmd, e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut devices = Vec::new();
    for line in stdout.lines().skip(1) {
        if line.is_empty() {
            continue;
        }
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
fn adb_install_and_run(_state: State<'_, EditorState>, _apk_path: String) -> Result<(), String> {
    // Stub
    Ok(())
}

#[tauri::command]
fn get_android_config(state: State<'_, EditorState>) -> Result<Value, String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let adb_found = if let Some(path) = sdk_path.as_ref() {
        std::path::PathBuf::from(path)
            .join("platform-tools/adb")
            .exists()
    } else {
        false
    };

    Ok(json!({
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
        if p.join("emulator/emulator").exists() {
            p.join("emulator/emulator").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    let output = std::process::Command::new(emulator_cmd)
        .hidden()
        .arg("-list-avds")
        .output()
        .map_err(|e| format!("Emulator error: {}", e))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.lines().map(|s| s.to_string()).collect())
}

#[tauri::command]
fn spawn_emulator(state: State<'_, EditorState>, avd: String) -> Result<(), String> {
    let sdk_path = state.android_sdk_path.lock().unwrap();
    let emulator_cmd = if let Some(path) = sdk_path.as_ref() {
        let p = std::path::PathBuf::from(path);
        if p.join("emulator/emulator").exists() {
            p.join("emulator/emulator").to_string_lossy().to_string()
        } else {
            "emulator".to_string()
        }
    } else {
        "emulator".to_string()
    };

    std::process::Command::new(emulator_cmd)
        .hidden()
        .arg("-avd")
        .arg(avd)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn set_active_root(state: State<'_, EditorState>, path: Option<String>) {
    let mut root = state.active_root.lock().unwrap();
    if let Some(p) = path {
        let path_buf = PathBuf::from(p);
        *root = Some(path_buf.clone());
        state.ai_engine.set_root_path(path_buf);
    } else {
        *root = None;
    }
}

#[tauri::command]
fn rename_path(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(old_path, new_path).map_err(|e| e.to_string())
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
fn create_file(path: String) -> Result<(), String> {
    fs::File::create(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn create_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn create_directory(path: String) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

fn is_path_valid(state: &EditorState, path: &PathBuf) -> Result<(), String> {
    let root = state.active_root.lock().unwrap();
    if let Some(ref r) = *root {
        if !path.starts_with(r) {
            return Err("Access Denied: Path is outside of project root".to_string());
        }
    } else {
        return Err("No project open".to_string());
    }
    Ok(())
}

#[tauri::command]
fn validate_path(state: State<'_, EditorState>, path: PathBuf) -> Result<(), String> {
    is_path_valid(&state, &path)
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
fn list_dir_flat(path: PathBuf) -> Result<Vec<FileEntry>, String> {
    let mut tree = Vec::new();
    let ignore_list = get_ignore_patterns();

    // Use a flat read_dir for the current expansion level (lazy loading)
    if let Ok(entries) = fs::read_dir(&path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let name = entry_path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();

            // Skip ignored patterns at the high level to keep UI clean
            if ignore_list.iter().any(|&p| name == p) {
                continue;
            }

            let meta = fs::metadata(&entry_path).map_err(|e| e.to_string())?;
            tree.push(FileEntry {
                name,
                path: entry_path.to_string_lossy().to_string(),
                is_dir: meta.is_dir(),
                is_expanded: Some(false),
                children: None,
            });
        }
    }

    // Sort: directories first, then alphabetically
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
async fn get_file_tree(state: tauri::State<'_, EditorState>) -> Result<Vec<FileEntry>, String> {
    let root = {
        let root_guard = state.active_root.lock().unwrap();
        root_guard
            .clone()
            .ok_or_else(|| "No project open".to_string())?
    };

    // EXTREME SCALE FIX: Never walk recursively on initial load.
    // Only return the top-level files/folders of the root.
    list_dir_flat(root)
}

#[tauri::command]
async fn get_directory_contents(
    state: tauri::State<'_, EditorState>,
    path: String,
) -> Result<Vec<FileEntry>, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf)?;
    list_dir_flat(path_buf)
}

#[tauri::command]
fn read_file(state: tauri::State<'_, EditorState>, path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf)?;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_file(
    state: tauri::State<'_, EditorState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf)?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_project_memory(
    state: tauri::State<'_, EditorState>,
    content: String,
) -> Result<(), String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let memory_path = root.join("MEMORY.md");
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&memory_path)
        .map_err(|e| e.to_string())?;
    use std::io::Write;
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    // Format as YYYY-MM-DD HH:MM (UTC) from unix seconds
    let (y, mo, d, h, mi) = {
        let s = secs;
        let days = s / 86400;
        let rem = s % 86400;
        let h = rem / 3600;
        let mi = (rem % 3600) / 60;
        // Approximate Gregorian date from epoch days
        let z = days + 719468;
        let era = z / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let mo = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if mo <= 2 { y + 1 } else { y };
        (y, mo, d, h, mi)
    };
    let entry = format!("\n\n### [{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC]\n{content}\n");
    file.write_all(entry.as_bytes()).map_err(|e| e.to_string())
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
fn get_git_branch() -> Result<String, String> {
    let output = Command::new("git")
        .hidden()
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|_| "Git not found".to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
fn get_git_history(path: String) -> Result<Vec<git::GitCommitInfo>, String> {
    let manager = GitManager::new();
    manager.get_history(path)
}

#[tauri::command]
fn git_diff(path: String, hash: String) -> Result<String, String> {
    let manager = GitManager::new();
    manager.get_commit_diff(path, &hash)
}

#[tauri::command]
fn search_project(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .unwrap_or_else(|| PathBuf::from("."));

    // Use native grep for high performance on Unix-like systems (macOS/Linux)
    if cfg!(not(target_os = "windows")) {
        let output = std::process::Command::new("grep")
            .hidden()
            .args(&["-r", "-n", "-i", "--exclude-dir=.git", &query, "."])
            .current_dir(&root)
            .output()
            .map_err(|e| e.to_string())?;

        let mut results = Vec::new();
        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() == 3 {
                if let Ok(line_num) = parts[1].parse::<usize>() {
                    results.push(SearchResult {
                        path: root.join(parts[0]).to_string_lossy().to_string(),
                        line: line_num,
                        content: parts[2].trim().to_string(),
                    });
                }
            }
            if results.len() > 100 {
                break;
            }
        }

        if !results.is_empty() {
            return Ok(results);
        }
    }

    // Fallback to WalkDir for Windows or if grep returns nothing/fails
    let mut results = Vec::new();
    for entry in walkdir::WalkDir::new(&root)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_file() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for (i, line) in content.lines().enumerate() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        results.push(SearchResult {
                            path: entry.path().to_string_lossy().to_string(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() > 100 {
                            break;
                        }
                    }
                }
            }
        }
        if results.len() > 100 {
            break;
        }
    }
    Ok(results)
}

#[derive(Serialize, Deserialize)]
struct SearchResult {
    path: String,
    line: usize,
    content: String,
}

// =============================================================================
// NEW: Tool Registry Backend Commands — supports structured function calling
// =============================================================================

/// Glob file search — find files matching a pattern (like Claude Code's GlobTool)
#[tauri::command]
fn glob_files(
    state: State<'_, EditorState>,
    pattern: String,
    path: Option<String>,
) -> Result<Vec<String>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
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

/// Grep file search — search content in files (like Claude Code's GrepTool backed by ripgrep)
#[tauri::command]
fn grep_files(
    state: State<'_, EditorState>,
    pattern: String,
    path: Option<String>,
    include: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state.active_root.lock().unwrap().clone().unwrap_or_else(|| PathBuf::from("."))
    };

    let mut results = Vec::new();

    // 1. Try ripgrep (Optimized for speed)
    let rg_result = std::process::Command::new("rg")
        .hidden()
        .args(&["-n", "--no-heading", "--max-count=100", "--color=never"])
        .args(include.as_ref().map(|i| vec!["-g", i]).unwrap_or_default())
        .arg(&pattern)
        .current_dir(&root)
        .output();

    if let Ok(output) = rg_result {
        if output.status.success() || !output.stdout.is_empty() {
             let stdout = String::from_utf8_lossy(&output.stdout);
             for line in stdout.lines().take(100) {
                 let parts: Vec<&str> = line.splitn(3, ':').collect();
                 if parts.len() == 3 {
                     if let Ok(ln) = parts[1].parse::<usize>() {
                         results.push(SearchResult {
                             path: parts[0].to_string(),
                             line: ln,
                             content: parts[2].trim().to_string(),
                         });
                     }
                 }
             }
             if !results.is_empty() { return Ok(results); }
        }
    }

    // 2. Fallback: Pure-Rust Resident Search (WalkDir + Regex)
    println!("[DEBUG] Ripgrep unavailable. Activating internal search engine for: {}", pattern);
    let re = regex::RegexBuilder::new(&pattern)
        .case_insensitive(true)
        .build()
        .map_err(|e| format!("Invalid regex pattern: {}", e))?;

    let walker = ignore::WalkBuilder::new(&root)
        .standard_filters(true)
        .max_depth(Some(10))
        .build();

    for entry in walker.flatten() {
        let is_file = entry.file_type().map(|t| t.is_file()).unwrap_or(false);
        if is_file {
            let path = entry.path();
            if let Ok(content) = std::fs::read_to_string(path) {
                if content.len() > 1_000_000 { continue; } // Skip huge binaries
                for (i, line) in content.lines().enumerate() {
                    if re.is_match(line) {
                        results.push(SearchResult {
                            path: path.to_string_lossy().to_string(),
                            line: i + 1,
                            content: line.trim().to_string(),
                        });
                        if results.len() >= 100 { break; }
                    }
                }
            }
        }
        if results.len() >= 100 { break; }
    }

    Ok(results)
}

/// Write file content with auto-mkdir — the tool_registry's file_write tool backend
#[tauri::command]
fn write_file_content(path: String, content: String) -> Result<(), String> {
    let p = PathBuf::from(&path);
    // Auto-create parent directories
    if let Some(parent) = p.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("Failed to create directory: {}", e))?;
        }
    }
    fs::write(&p, &content).map_err(|e| format!("Failed to write file: {}", e))
}

/// Web fetch — fetch URL content (the tool_registry's web_fetch tool backend)
#[tauri::command]
async fn web_fetch(url: String) -> Result<String, String> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .map_err(|e| format!("HTTP client error: {}", e))?;

    let response = client
        .get(&url)
        .header("User-Agent", "Antigravity/1.0")
        .send()
        .await
        .map_err(|e| format!("Fetch error: {}", e))?;

    let text = response
        .text()
        .await
        .map_err(|e| format!("Read error: {}", e))?;

    // Truncate to prevent overwhelming the AI context
    Ok(if text.len() > 50000 {
        text[..50000].to_string()
    } else {
        text
    })
}

/// AI tool result callback — frontend sends back tool execution results to the backend
#[tauri::command]
async fn ai_tool_result(
    _state: State<'_, EditorState>,
    call_id: String,
    result: String,
) -> Result<(), String> {
    // Store the tool result for the AI engine to pick up
    eprintln!(
        "[Tool Result] call_id={}, result_len={}",
        call_id,
        result.len()
    );
    Ok(())
}

#[tauri::command]
fn spawn_terminal(
    state: State<'_, EditorState>,
    app: tauri::AppHandle,
    id: String,
    shell: Option<String>,
) -> Result<(), String> {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    // Determine the shell
    let shell_exe = if let Some(s) = shell {
        if s.is_empty() {
            if cfg!(target_os = "windows") {
                std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
            } else {
                std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
            }
        } else {
            s
        }
    } else {
        if cfg!(target_os = "windows") {
            std::env::var("COMSPEC").unwrap_or_else(|_| "powershell.exe".to_string())
        } else {
            std::env::var("SHELL").unwrap_or_else(|_| "/bin/zsh".to_string())
        }
    };

    let mut cmd = CommandBuilder::new(shell_exe);

    // Set CWD to active project root if available
    {
        let root = state.active_root.lock().unwrap();
        if let Some(ref r) = *root {
            let r_owned: String = r.display().to_string();
            cmd.cwd(r_owned);
        }
    }

    let child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let mut reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;

    // Spawn reader thread
    let app_handle = app.clone();
    let term_id_clone = id.clone();
    std::thread::spawn(move || {
        let state = app_handle.state::<EditorState>();
        let mut buf = [0u8; 8192];
        let mut pending_data = String::new();
        let mut last_emit = std::time::Instant::now();

        loop {
            // Check if we should emit pending data due to timeout (50ms)
            if !pending_data.is_empty()
                && last_emit.elapsed() >= std::time::Duration::from_millis(50)
            {
                let _ = app_handle.emit(
                    "terminal-data",
                    TerminalDataPayload {
                        term_id: term_id_clone.clone(),
                        data: pending_data.clone(),
                    },
                );
                pending_data.clear();
                last_emit = std::time::Instant::now();
            }

            // Non-blocking read or small timeout to allow the loop to check last_emit
            // Note: reader.read is typically blocking. For high-perf throttle,
            // we could use non-blocking or just accept that the NEXT read will trigger the emit.
            // Given the architectural constraints, we'll append to pending and emit if burst or time.

            match reader.read(&mut buf) {
                Ok(0) => break, // EOF
                Ok(n) => {
                    let data = String::from_utf8_lossy(&buf[..n]).to_string();

                    // Update persistent buffer
                    if let Ok(mut buffers) = state.terminal_buffers.lock() {
                        let history = buffers
                            .entry(term_id_clone.clone())
                            .or_insert_with(Vec::<String>::new);
                        history.push(data.clone());
                        if history.len() > 1000 {
                            history.remove(0);
                        }
                    }

                    pending_data.push_str(&data);

                    // If we have a lot of data or 50ms passed, emit
                    if pending_data.len() > 2048
                        || last_emit.elapsed() >= std::time::Duration::from_millis(50)
                    {
                        let _ = app_handle.emit(
                            "terminal-data",
                            TerminalDataPayload {
                                term_id: term_id_clone.clone(),
                                data: pending_data.clone(),
                            },
                        );
                        pending_data.clear();
                        last_emit = std::time::Instant::now();
                    }
                }
                Err(e) => {
                    println!("[Term] Error reading from {}: {:?}", term_id_clone, e);
                    break;
                }
            }
        }
    });

    state
        .terminal_masters
        .lock()
        .unwrap()
        .insert(id.clone(), pair.master);
    state
        .terminal_writers
        .lock()
        .unwrap()
        .insert(id.clone(), writer);
    state.terminal_processes.lock().unwrap().insert(id, child);
    Ok(())
}

#[tauri::command]
fn close_terminal(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    // Drop the master, writer, and kill the process
    state.terminal_writers.lock().unwrap().remove(&id);
    state.terminal_masters.lock().unwrap().remove(&id);
    if let Some(mut child) = state.terminal_processes.lock().unwrap().remove(&id) {
        let _ = child.kill();
    }
    Ok(())
}

#[tauri::command]
fn get_available_shells() -> Vec<String> {
    let mut shells = Vec::new();
    if cfg!(target_os = "windows") {
        shells.push("powershell.exe".to_string());
        shells.push("cmd.exe".to_string());
    } else {
        for path in &[
            "/bin/zsh",
            "/bin/bash",
            "/usr/bin/zsh",
            "/usr/bin/bash",
            "/bin/sh",
        ] {
            if std::path::Path::new(path).exists() {
                shells.push(path.to_string());
            }
        }
    }
    shells
}

#[tauri::command]
fn write_to_terminal(
    state: State<'_, EditorState>,
    id: String,
    data: String,
) -> Result<(), String> {
    let mut writers = state.terminal_writers.lock().unwrap();
    if let Some(writer) = writers.get_mut(&id) {
        writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
fn resize_terminal(
    state: State<'_, EditorState>,
    id: String,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let masters = state.terminal_masters.lock().unwrap();
    if let Some(master) = masters.get(&id) {
        master
            .resize(PtySize {
                rows,
                cols,
                pixel_width: 0,
                pixel_height: 0,
            })
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("Terminal not found".to_string())
    }
}

#[tauri::command]
async fn get_system_health(state: State<'_, EditorState>) -> Result<Value, String> {
    state
        .ai_engine
        .get_tools()
        .get_system_health(json!({}))
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
async fn ai_chat(state: State<'_, EditorState>, request: AiRequest) -> Result<String, String> {
    let content = state
        .ai_engine
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;
    // Satisfy AiResponse usage warning
    let _response = AiResponse {
        content: content.clone(),
    };
    Ok(content)
}

#[tauri::command]
async fn ai_inline_complete(
    state: State<'_, EditorState>,
    prefix: String,
    suffix: String,
    language: String,
    file_path: String,
) -> Result<String, String> {
    // Build a FIM (fill-in-the-middle) completion request
    let fim_prompt = format!(
        "Complete the following {} code. Return ONLY the completion text, no explanation, no markdown fencing, no extra whitespace.\n\n<prefix>\n{}\n</prefix>\n<suffix>\n{}\n</suffix>",
        language, prefix, suffix
    );

    let messages = vec![
        ai_engine::ChatMessage {
            role: "system".to_string(),
            content: Some(ai_engine::MessageContent::Text(
                format!("You are an inline code completion engine for file '{}' (language: {}). Return ONLY the exact code that should be inserted at the cursor position. No explanation, no markdown, no comments. Just the raw code completion.", file_path, language)
            )),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
        ai_engine::ChatMessage {
            role: "user".to_string(),
            content: Some(ai_engine::MessageContent::Text(fim_prompt)),
            tool_calls: None,
            tool_call_id: None,
            metadata: None,
        },
    ];

    let request = AiRequest {
        provider: "apiradar".to_string(),
        model: "gpt-4o-mini".to_string(), // Use a fast model for completions
        messages,
        temperature: Some(0.2), // Low temperature for precise completions
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url: None,
        tools: None,
    };

    // Use ai_engine's single-shot (non-autonomous) call
    let result = state.ai_engine
        .autonomous_loop(request, None)
        .await
        .map_err(|e| e.to_string())?;

    // Strip any markdown fences the model might add despite instructions
    let cleaned = result
        .trim()
        .trim_start_matches("```")
        .trim_start_matches(&language)
        .trim_start_matches('\n')
        .trim_end_matches("```")
        .trim()
        .to_string();

    Ok(cleaned)
}

#[tauri::command]
fn stop_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.stop();
    Ok(())
}

#[tauri::command]
fn backend_ping() -> String {
    "System Pulse: ACTIVE".to_string()
}

#[tauri::command]
async fn list_provider_models(
    state: State<'_, EditorState>,
    provider: String,
) -> Result<Vec<String>, String> {
    state
        .ai_engine
        .list_models(&provider)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_visual_graph(data: Value, format: String) -> Result<visual_lab::VisualGraph, String> {
    match format.as_str() {
        "json" => Ok(visual_lab::parse_json_to_graph(data)),
        "sql" => {
            let sql = data.as_str().unwrap_or("");
            Ok(visual_lab::parse_sql_to_graph(sql))
        }
        "mongodb" => {
            let content = data.as_str().unwrap_or("");
            Ok(visual_lab::parse_mongodb_to_graph(content))
        }
        _ => Err(format!("Unsupported format: {}", format)),
    }
}

#[tauri::command]
async fn get_neural_omni_graph(
    state: tauri::State<'_, EditorState>,
) -> Result<visual_lab::VisualGraph, String> {
    let slots = state.ai_engine.memory_store.get_all_slots().await;
    Ok(visual_lab::generate_neural_omni_graph(slots))
}

#[tauri::command]
async fn get_all_memory_slots(
    state: tauri::State<'_, EditorState>,
) -> Result<Vec<crate::memory_store::SemanticSlot>, String> {
    Ok(state.ai_engine.memory_store.get_all_slots().await)
}

#[tauri::command]
async fn generate_visual_graph(
    state: State<'_, EditorState>,
    prompt: String,
) -> Result<visual_lab::VisualGraph, String> {
    // This will use the AI engine to generate a graph structure
    // For now, we'll implement a mock that shows the potential
    let ai_prompt = format!("Generate a visual diagram for: {}. Return ONLY a JSON object compatible with ReactFlow (nodes and edges).", prompt);
    let response = state
        .ai_engine
        .autonomous_loop(AiRequest {
            provider: "Anthropic".to_string(), // Default
            model: "claude-3-5-sonnet-latest".to_string(),
            messages: vec![ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(ai_prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            }],
            temperature: Some(0.3),
            autonomous: true,
            mode: Some("Fast".to_string()),
            cyber_mode: None,
            root_access: None,
            ollama_url: None,
            tools: None,
        }, None)
        .await
        .map_err(|e| e.to_string())?;

    // Parse the AI response into a VisualGraph
    let graph: visual_lab::VisualGraph =
        serde_json::from_str(&response).map_err(|e| e.to_string())?;
    Ok(graph)
}

// Duplicates removed

fn load_jsonc(path: &std::path::Path) -> Result<Value, String> {
    if !path.exists() {
        return Err(format!("File not found: {:?}", path));
    }
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;

    // Strip comments (JSONC)
    let re_block = regex::Regex::new(r"/\*[\s\S]*?\*/").unwrap();
    let re_line = regex::Regex::new(r"//.*").unwrap();
    let sanitized = re_block.replace_all(&content, "");
    let sanitized = re_line.replace_all(&sanitized, "");

    let json: Value =
        serde_json::from_str(&sanitized).map_err(|e| format!("JSON Error in {:?}: {}", path, e))?;
    Ok(json)
}

fn load_theme_recursive(path: &std::path::Path) -> Result<Value, String> {
    let mut json = load_jsonc(path)?;

    // Handle 'include'
    if let Some(include_path) = json.get("include").and_then(|v| v.as_str()) {
        let parent = path.parent().unwrap();
        let included_full_path = parent.join(include_path);
        let included_json = load_theme_recursive(&included_full_path)?;
        // Correct merge logic below
        if let Some(included_obj) = included_json.as_object() {
            for (key, val) in included_obj {
                if key == "colors" {
                    if let Some(target_colors) =
                        json.get_mut("colors").and_then(|v| v.as_object_mut())
                    {
                        for (ckey, cval) in val.as_object().unwrap() {
                            if !target_colors.contains_key(ckey) {
                                target_colors.insert(ckey.clone(), cval.clone());
                            }
                        }
                    } else {
                        json.as_object_mut()
                            .unwrap()
                            .insert("colors".to_string(), val.clone());
                    }
                } else if key == "tokenColors" {
                    // Aggregate token colors (array)
                    if let Some(target_tokens) =
                        json.get_mut("tokenColors").and_then(|v| v.as_array_mut())
                    {
                        if let Some(src_tokens) = val.as_array() {
                            target_tokens.extend(src_tokens.iter().cloned());
                        }
                    } else {
                        json.as_object_mut()
                            .unwrap()
                            .insert("tokenColors".to_string(), val.clone());
                    }
                }
            }
        }
    }

    Ok(json)
}

#[tauri::command]
fn get_installed_themes(state: State<'_, EditorState>) -> Result<Vec<Value>, String> {
    let host = state.ext_host.lock().map_err(|e| e.to_string())?;
    let mut themes = Vec::new();

    for ext in &host.extensions {
        if let Some(contributes) = &ext.contributes {
            if let Some(contributed_themes) = contributes.get("themes").and_then(|v| v.as_array()) {
                for theme in contributed_themes {
                    if let Some(label) = theme.get("label").and_then(|v| v.as_str()) {
                        if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
                            let extension_path = &ext.extension_path;
                            let theme_file_path = extension_path.join(path);

                            themes.push(json!({
                                "id": format!("{}-{}", ext.name, label),
                                "label": label,
                                "path": theme_file_path.to_string_lossy().to_string(),
                                "uiTheme": theme.get("uiTheme").and_then(|v| v.as_str()).unwrap_or("vs-dark"),
                                "extensionName": ext.name
                            }));
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
    let p = std::path::Path::new(&path);
    load_theme_recursive(p)
}

#[tauri::command]
fn register_ida_pro() -> Result<(), String> {
    Ok(())
}
#[tauri::command]
async fn ai_execute_command(command: String, cwd: Option<String>, _timeout: Option<u64>) -> Result<String, String> {
    println!("[DEBUG] ai_execute_command: {}", command);
    
    let working_dir = cwd.map(PathBuf::from).unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    
    let mut cmd = if cfg!(target_os = "windows") {
        let mut c = std::process::Command::new("cmd");
        c.args(&["/C", &command]);
        c
    } else {
        let mut c = std::process::Command::new("sh");
        c.args(&["-c", &command]);
        c
    };

    cmd.current_dir(working_dir);
    
    // Use tokio for async execution if we want to handle timeouts easily, 
    // but we'll stick to a simple synchronous wait with a timeout thread for now
    // to match the existing non-tokio command structure in lib.rs if it's simpler.
    // Actually, let's use standard output capture.
    
    let output = cmd.output().map_err(|e| format!("Failed to spawn command: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    if output.status.success() {
        if stdout.is_empty() && !stderr.is_empty() {
             Ok(format!("Command succeeded (stderr only):\n{}", stderr))
        } else if stdout.is_empty() {
             Ok("Command succeeded (no output)".to_string())
        } else {
             Ok(stdout)
        }
    } else {
        Err(format!("Command failed (exit {}):\nSTDOUT: {}\nSTDERR: {}", 
            output.status.code().unwrap_or(-1),
            stdout,
            stderr))
    }
}
#[tauri::command]
fn propose_file_change(
    state: tauri::State<'_, EditorState>,
    path: String,
    content: String,
    description: String,
) -> Result<serde_json::Value, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf)?;

    let old_content = if path_buf.exists() {
        fs::read_to_string(&path_buf).unwrap_or_default()
    } else {
        String::new()
    };

    Ok(serde_json::json!({
        "path": path,
        "oldContent": old_content,
        "newContent": content,
        "description": description
    }))
}

#[tauri::command]
fn ai_modify_file(
    _state: tauri::State<'_, EditorState>,
    path: String,
    instruction: String,
) -> Result<(), String> {
    // This is a stub for the AI to "think" about a modification
    // The actual modification will come back as a write_file which we intercept
    println!(
        "AI requested modification for path: {}, instruction: {}",
        path, instruction
    );
    Ok(())
}
#[tauri::command]
fn get_icon_theme_mapping(state: State<'_, EditorState>) -> Result<Value, String> {
    let host = state.ext_host.lock().map_err(|e| e.to_string())?;

    // Find vs-seti-icon-theme (usually in theme-seti extension)
    for ext in &host.extensions {
        if let Some(contributes) = &ext.contributes {
            if let Some(icon_themes) = contributes.get("iconThemes").and_then(|v| v.as_array()) {
                for theme in icon_themes {
                    if let Some(path) = theme.get("path").and_then(|v| v.as_str()) {
                        let full_path = ext.extension_path.join(path);
                        if let Ok(mut mapping) = load_jsonc(&full_path) {
                            // Convert font paths to full paths
                            if let Some(fonts) =
                                mapping.get_mut("fonts").and_then(|v| v.as_array_mut())
                            {
                                for font in fonts {
                                    if let Some(srcs) =
                                        font.get_mut("src").and_then(|v| v.as_array_mut())
                                    {
                                        for src in srcs {
                                            if let Some(path_val) =
                                                src.get_mut("path").and_then(|v| v.as_str())
                                            {
                                                let font_path =
                                                    full_path.parent().unwrap().join(path_val);
                                                *src = json!({ "path": font_path.to_string_lossy().to_string(), "format": "woff" });
                                            }
                                        }
                                    }
                                }
                            }
                            // Add extension path for relative URI resolution in frontend
                            if let Some(obj) = mapping.as_object_mut() {
                                obj.insert(
                                    "extensionPath".to_string(),
                                    json!(ext.extension_path.to_string_lossy().to_string()),
                                );
                            }
                            return Ok(mapping);
                        }
                    }
                }
            }
        }
    }
    Ok(json!({}))
}

#[tauri::command]
fn get_extension_contributions(state: State<'_, EditorState>) -> Result<Value, String> {
    let host = state.ext_host.lock().map_err(|e| e.to_string())?;
    let mut contribs = json!({
        "snippets": [],
        "keybindings": [],
        "grammars": [],
        "languages": [],
        "viewsContainers": { "activitybar": [] },
        "views": {}
    });

    for ext in &host.extensions {
        if let Some(contributes) = &ext.contributes {
            // Snippets
            if let Some(ext_snippets) = contributes.get("snippets").and_then(|v| v.as_array()) {
                for snippet in ext_snippets {
                    let mut s = snippet.clone();
                    if let Some(spath) = s.get("path").and_then(|v| v.as_str()) {
                        let full_spath = ext.extension_path.join(spath.replace("./", ""));
                        if let Some(obj) = s.as_object_mut() {
                            obj.insert(
                                "absolutePath".to_string(),
                                json!(full_spath.to_string_lossy().to_string()),
                            );
                        }
                    }
                    contribs["snippets"].as_array_mut().unwrap().push(s);
                }
            }
            // Languages
            if let Some(ext_langs) = contributes.get("languages").and_then(|v| v.as_array()) {
                for lang in ext_langs {
                    contribs["languages"]
                        .as_array_mut()
                        .unwrap()
                        .push(lang.clone());
                }
            }
            // Grammars
            if let Some(ext_grammars) = contributes.get("grammars").and_then(|v| v.as_array()) {
                for grammar in ext_grammars {
                    let mut g = grammar.clone();
                    if let Some(gpath) = g.get("path").and_then(|v| v.as_str()) {
                        let full_gpath = ext.extension_path.join(gpath.replace("./", ""));
                        if let Some(obj) = g.as_object_mut() {
                            obj.insert(
                                "absolutePath".to_string(),
                                json!(full_gpath.to_string_lossy().to_string()),
                            );
                        }
                    }
                    contribs["grammars"].as_array_mut().unwrap().push(g);
                }
            }
            // Views Containers (Activity Bar)
            if let Some(containers) = contributes.get("viewsContainers") {
                if let Some(activitybar) = containers.get("activitybar").and_then(|v| v.as_array())
                {
                    for container in activitybar {
                        let mut c = container.clone();
                        if let Some(obj) = c.as_object_mut() {
                            obj.insert(
                                "extensionPath".to_string(),
                                json!(ext.extension_path.to_string_lossy().to_string()),
                            );
                            obj.insert("extensionId".to_string(), json!(ext.id));

                            // Handle icons
                            if let Some(icon_val) = obj.get("icon").and_then(|v| v.as_str()) {
                                if icon_val.starts_with("$(") && icon_val.ends_with(")") {
                                    // Codicon reference: $(references) -> references
                                    let icon_name = &icon_val[2..icon_val.len() - 1];
                                    obj.insert("icon".to_string(), json!(icon_name));
                                } else {
                                    // File path icon
                                    let full_icon_path =
                                        ext.extension_path.join(icon_val.replace("./", ""));
                                    if let Ok(icon_data) = std::fs::read(&full_icon_path) {
                                        let b64 = general_purpose::STANDARD.encode(icon_data);
                                        let mime = if icon_val.ends_with(".svg") {
                                            "image/svg+xml"
                                        } else {
                                            "image/png"
                                        };
                                        obj.insert(
                                            "base64_icon".to_string(),
                                            json!(format!("data:{};base64,{}", mime, b64)),
                                        );
                                    }
                                }
                            }
                        }
                        contribs["viewsContainers"]["activitybar"]
                            .as_array_mut()
                            .unwrap()
                            .push(c);
                    }
                }
            }
            // Views (Sidebars)
            if let Some(views) = contributes.get("views").and_then(|v| v.as_object()) {
                for (location, view_list) in views {
                    if let Some(arr) = view_list.as_array() {
                        let target_arr = contribs["views"]
                            .as_object_mut()
                            .unwrap()
                            .entry(location.clone())
                            .or_insert(json!([]))
                            .as_array_mut()
                            .unwrap();
                        for view in arr {
                            let mut v = view.clone();
                            if let Some(obj) = v.as_object_mut() {
                                obj.insert(
                                    "extensionPath".to_string(),
                                    json!(ext.extension_path.to_string_lossy().to_string()),
                                );
                                obj.insert("extensionId".to_string(), json!(ext.id));
                            }
                            target_arr.push(v);
                        }
                    }
                }
            }
        }
    }

    Ok(contribs)
}
#[tauri::command]
async fn hunt_api_keys(
    app: tauri::AppHandle,
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    use tauri::Emitter;

    let _ = app.emit(
        "hunt-progress",
        HuntProgress {
            msg: "Initializing ApiRadar Hunter...".to_string(),
        },
    );
    let hunter = crate::hunter::ApiRadarHunter::new();

    let _ = app.emit(
        "hunt-progress",
        HuntProgress {
            msg: "Fetching recent leaks from ApiRadar...".to_string(),
        },
    );
    let leaks = hunter.fetch_recent_leaks("all").await.unwrap_or_default();

    if leaks.is_empty() {
        let _ = app.emit(
            "hunt-progress",
            HuntProgress {
                msg: "No leaks found from ApiRadar. Try again later.".to_string(),
            },
        );
        return Ok(json!([]));
    }

    let _ = app.emit(
        "hunt-progress",
        HuntProgress {
            msg: format!("Found {} repositories to scan...", leaks.len()),
        },
    );

    let mut found_keys: Vec<Value> = Vec::new();
    let mut persisted_keys: std::collections::HashMap<String, String> =
        std::collections::HashMap::new();

    for (_i, leak) in leaks.iter().enumerate() {
        if !hunter.is_relevant_file(&leak.file_path) {
            continue;
        }

        let content = hunter
            .fetch_raw_content(&leak.repo_url, &leak.file_path)
            .await
            .unwrap_or_default();
        if content.is_empty() || !hunter.contains_key_indicator(&content) {
            continue;
        }

        let extracted = hunter.extract_keys(&content);
        for (key_type, key_value) in extracted {
            let _ = app.emit(
                "hunt-progress",
                HuntProgress {
                    msg: format!("Validating {} key...", key_type),
                },
            );
            let (is_live, details) = hunter.validate_key(&key_type, &key_value).await;

            if is_live {
                let result = crate::hunter::HuntResult {
                    provider: leak.provider.clone(),
                    key: key_value.clone(),
                    key_type: key_type.clone(),
                    source: "ApiRadar".to_string(),
                    repo_url: leak.repo_url.clone(),
                    is_live: true,
                    details: details.clone(),
                };
                found_keys.push(serde_json::to_value(result).unwrap_or_default());

                let _ = app.emit(
                    "hunt-found",
                    json!({"msg": format!("✅ LIVE {} from {}", key_type, leak.repo_url)}),
                );

                // Set env var AND track for persistence
                let (env_var, json_field) = match key_type.as_str() {
                    "openrouter_key" => ("OPENROUTER_API_KEY", "openrouter"),
                    "openai_key" => ("OPENAI_API_KEY", "openai"),
                    "anthropic_api_key" => ("ANTHROPIC_API_KEY", "anthropic"),
                    "google_api_key" => ("GOOGLE_API_KEY", "google"),
                    "mistral_api_key" => ("MISTRAL_API_KEY", "mistral"),
                    "xai_key" => ("XAI_API_KEY", "xai"),
                    "groq_key" => ("GROQ_API_KEY", "groq"),
                    _ => ("", ""),
                };
                if !env_var.is_empty() {
                    std::env::set_var(env_var, &key_value);
                    persisted_keys.insert(json_field.to_string(), key_value.clone());
                }
            } else {
                let _ = app.emit(
                    "hunt-progress",
                    json!({"msg": format!("❌ {} dead: {}", key_type, details)}),
                );
            }
        }
    }

    // Persist found keys to api_keys.json so refreshAvailableModels can find them
    if !persisted_keys.is_empty() {
        let path = state.config_dir.join("api_keys.json");
        let mut existing: serde_json::Map<String, Value> = if path.exists() {
            let content = fs::read_to_string(&path).unwrap_or_default();
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            serde_json::Map::new()
        };

        for (field, key) in &persisted_keys {
            existing.insert(field.clone(), json!(key));
        }

        let _ = fs::write(
            &path,
            serde_json::to_string_pretty(&existing).unwrap_or_default(),
        );
        let _ = app.emit(
            "hunt-progress",
            json!({"msg": format!("Persisted {} keys to config.", persisted_keys.len())}),
        );
    }

    let _ = app.emit(
        "hunt-progress",
        json!({"msg": format!("Hunt complete. Found {} live keys.", found_keys.len())}),
    );
    Ok(json!(found_keys))
}
#[tauri::command]
async fn optimize_memory(state: State<'_, EditorState>) -> Result<(), String> {
    state
        .ai_engine
        .optimize_memory()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
fn pause_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.pause();
    Ok(())
}

#[tauri::command]
fn resume_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.resume();
    Ok(())
}
#[tauri::command]
fn start_mitm_server() -> Result<(), String> {
    Ok(())
}
#[tauri::command]
fn stop_mitm_server() -> Result<(), String> {
    Ok(())
}
#[tauri::command]
fn get_mitm_status() -> String {
    "idle".to_string()
}
#[tauri::command]
fn debug_start(
    state: State<'_, EditorState>,
    app: tauri::AppHandle,
    adapter_path: String,
) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.start_session(&adapter_path, app)
}

#[tauri::command]
fn debug_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.send_message(msg)
}

#[tauri::command]
fn debug_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().unwrap();
    debug.stop_session()
}

#[tauri::command]
fn check_activation_event(state: State<'_, EditorState>, event: String) -> Result<(), String> {
    let mut am = state.activation_manager.lock().unwrap();
    am.check_activation_requests(&event, state.ext_host.clone());
    Ok(())
}

#[tauri::command]
fn terminal_read_output(state: State<'_, EditorState>, id: String) -> Result<String, String> {
    state.terminal_read_output(id)
}

#[tauri::command]
fn terminal_toggle(app: tauri::AppHandle, visible: bool) -> Result<(), String> {
    app.emit("toggle-terminal", visible)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn terminal_terminate(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    let mut processes = state.terminal_processes.lock().unwrap();
    if let Some(mut child) = processes.remove(&id) {
        let _ = child.kill();
    }
    state.terminal_masters.lock().unwrap().remove(&id);
    state.terminal_writers.lock().unwrap().remove(&id);
    Ok(())
}

#[tauri::command]
fn editor_get_active_file(
    state: tauri::State<'_, EditorState>,
) -> Result<serde_json::Value, String> {
    let sentient = state.ai_engine.clone();
    let tools = sentient.get_tools();
    tools
        .editor_get_active_file(serde_json::json!({}))
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
fn terminal_get_status(
    state: State<'_, EditorState>,
    id: String,
) -> Result<serde_json::Value, String> {
    let mut processes = state.terminal_processes.lock().unwrap();
    if let Some(child) = processes.get_mut(&id) {
        match child.try_wait() {
            Ok(Some(status)) => {
                Ok(serde_json::json!({ "active": false, "success": status.success() }))
            }
            Ok(None) => Ok(serde_json::json!({ "active": true })),
            Err(e) => Err(e.to_string()),
        }
    } else {
        Ok(serde_json::json!({ "active": false, "info": "Process not found or already exited" }))
    }
}

#[tauri::command]
fn analyze_file_symbols(
    state: State<'_, EditorState>,
    path: String,
) -> Result<serde_json::Value, String> {
    let sentient = state.ai_engine.clone();
    let tools = sentient.get_tools();
    tools
        .analyze_file_symbols(serde_json::json!({ "path": path }))
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
async fn open_ai_login(app: tauri::AppHandle, provider: String) -> Result<(), String> {
    crate::ai_auth::open_login_window(app, provider).await
}

#[tauri::command]
async fn save_ai_session(
    session: crate::ai_auth::AiSession,
    state: State<'_, EditorState>,
) -> Result<(), String> {
    crate::ai_auth::save_session(&state.auth_state, session);
    Ok(())
}

#[tauri::command]
async fn capture_ai_session(
    app: tauri::AppHandle,
    provider: String,
) -> Result<crate::ai_auth::AiSession, String> {
    crate::ai_auth::capture_session(app, provider).await
}

#[tauri::command]
fn get_emulator_screenshot() -> Result<String, String> {
    Ok("".to_string())
}
#[tauri::command]
fn emulator_tap(_x: i32, _y: i32) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
async fn check_ollama_status(state: State<'_, EditorState>) -> Result<bool, String> {
    state
        .ai_engine
        .check_ollama_status()
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn pull_ollama_model(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state
        .ai_engine
        .pull_model(&name)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn set_ollama_url(state: State<'_, EditorState>, url: String) -> Result<(), String> {
    {
        let mut current = state.ollama_url.lock().unwrap();
        *current = url.clone();
    }

    state.ai_engine.set_ollama_url(url);
    Ok(())
}

#[tauri::command]
async fn benchmark_ane(
    state: State<'_, EditorState>,
    device: Option<String>,
) -> Result<Value, String> {
    let mode = device.unwrap_or_else(|| "ANE".to_string());

    if mode == "CPU" {
        let start_eval = std::time::Instant::now();
        // Simulate CPU workload
        let mut sum = 0u64;
        for i in 0..10_000_000 {
            sum = sum.wrapping_add(i);
        }
        std::hint::black_box(sum);
        let eval_us = start_eval.elapsed().as_micros();
        state
            .perf_monitor
            .record_inference("CPU".to_string(), (eval_us / 1000) as u64);

        return Ok(json!({
            "status": "success",
            "eval_us": eval_us,
            "device": "Intel/PC CPU"
        }));
    }

    if mode == "GPU" {
        let start_eval = std::time::Instant::now();
        // Simulate massive parallel throughput (approx 2ms)
        std::thread::sleep(std::time::Duration::from_millis(2));
        let eval_us = start_eval.elapsed().as_micros();
        state
            .perf_monitor
            .record_inference("GPU".to_string(), (eval_us / 1000) as u64);

        return Ok(json!({
            "status": "success",
            "eval_us": eval_us,
            "device": "H4RDW4RE GPU (Async)"
        }));
    }

    let mut ane = state.ai_engine.ane_engine.lock().await;
    if ane.is_none() {
        // Initialize with a simple 110M Transformer block template
        let mil = crate::ane::AneEngine::gen_dyn_matmul_mil(768, 768, 256);
        let weights = vec![0u8; 768 * 768 * 2]; // Dummy weights for benchmark
        let input_sizes = vec![1 * 768 * 1 * (256 + 768) * 2];
        let output_sizes = vec![1 * 768 * 1 * 256 * 2];

        let start_compile = std::time::Instant::now();
        let engine = crate::ane::AneEngine::new(&mil, &weights, &input_sizes, &output_sizes)?;
        let compile_ms = start_compile.elapsed().as_millis();

        *ane = Some(engine);

        // Run one evaluation
        let input = vec![vec![0u8; input_sizes[0]]];
        let start_eval = std::time::Instant::now();
        ane.as_ref().unwrap().execute(&input, &output_sizes)?;
        let eval_us = start_eval.elapsed().as_micros();
        state
            .perf_monitor
            .record_inference("ANE".to_string(), (eval_us / 1000) as u64);

        Ok(json!({
            "status": "success",
            "compile_ms": compile_ms,
            "eval_us": eval_us,
            "device": "Apple Neural Engine"
        }))
    } else {
        // Already initialized, just benchmark eval
        let input_sizes = vec![1 * 768 * 1 * (256 + 768) * 2];
        let output_sizes = vec![1 * 768 * 1 * 256 * 2];
        let input = vec![vec![0u8; input_sizes[0]]];

        let start_eval = std::time::Instant::now();
        ane.as_ref().unwrap().execute(&input, &output_sizes)?;
        let eval_us = start_eval.elapsed().as_micros();
        state
            .perf_monitor
            .record_inference("ANE".to_string(), (eval_us / 1000) as u64);

        Ok(json!({
            "status": "success",
            "eval_us": eval_us,
            "device": "Apple Neural Engine"
        }))
    }
}

#[tauri::command]
#[allow(dead_code)]
async fn get_inference_history(state: State<'_, EditorState>) -> Result<Value, String> {
    Ok(json!(state.perf_monitor.get_inference_history()))
}

#[tauri::command]
async fn query_performance_history(state: State<'_, EditorState>) -> Result<Value, String> {
    let stats = state.perf_monitor.get_stats();
    Ok(json!({ "history": [stats] }))
}

#[tauri::command]
async fn git_revert(state: State<'_, EditorState>, hash: String) -> Result<(), String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .ok_or("No active project")?;
    GitManager::new().revert_commit(root, &hash)
}

#[tauri::command]
async fn git_stash(state: State<'_, EditorState>) -> Result<(), String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .ok_or("No active project")?;
    GitManager::new().stash_changes(root)
}

#[tauri::command]
async fn git_stash_pop(state: State<'_, EditorState>) -> Result<(), String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .ok_or("No active project")?;
    GitManager::new().pop_stash(root)
}

#[tauri::command]
async fn git_get_unmerged(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let root = state
        .active_root
        .lock()
        .unwrap()
        .clone()
        .ok_or("No active project")?;
    GitManager::new().get_unmerged_files(root)
}

#[tauri::command]
async fn git_clone(url: String, path: String) -> Result<(), String> {
    GitManager::new().clone(&url, path)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileContext {
    pub symbols: Vec<String>,
    pub related_files: Vec<String>,
    pub relevant_lessons: Vec<SemanticSlot>,
}

#[tauri::command]
async fn query_workspace_memory(
    state: State<'_, EditorState>,
    category: String,
) -> Result<Vec<SemanticSlot>, String> {
    let memory = state.ai_engine.memory_store.query_slots(&category).await;
    Ok(memory)
}

#[tauri::command]
async fn get_file_context(
    state: State<'_, EditorState>,
    file_path: String,
) -> Result<FileContext, String> {
    let memory = &state.ai_engine.memory_store;

    // 1. Get symbols for this file
    let slots = memory.query_slots("file_map").await;
    let mut symbols = Vec::new();
    if let Some(slot) = slots.iter().find(|s| s.content == file_path) {
        for tag in &slot.tags {
            if let Some(sym) = tag.strip_prefix("symbol:") {
                symbols.push(sym.to_string());
            }
        }
    }

    // 2. Query related entities
    let related_files = memory
        .query_related_entities(&format!("file_map:{}", file_path))
        .await;

    // 3. Query relevant lessons
    let relevant_lessons = memory.query_slots("fix_lessons").await;

    Ok(FileContext {
        symbols,
        related_files,
        relevant_lessons,
    })
}

pub fn run() {
    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    std::panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.as_str()
        } else {
            "Unknown panic"
        };
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());
        let panic_msg = format!("[CRITICAL PANIC] {} at {}\n", message, location);
        eprintln!("{}", panic_msg);
        let _ = std::fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open("log.txt")
            .map(|mut f| {
                use std::io::Write;
                let _ = f.write_all(panic_msg.as_bytes());
            });
    }));

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};
            let ctrl_alt_a = Shortcut::new(
                Some(
                    tauri_plugin_global_shortcut::Modifiers::CONTROL
                        | tauri_plugin_global_shortcut::Modifiers::ALT,
                ),
                tauri_plugin_global_shortcut::Code::KeyA,
            );

            let h = app.handle().clone();
            app.global_shortcut()
                .on_shortcut(ctrl_alt_a, move |_app, shortcut, _event| {
                    if shortcut == &ctrl_alt_a {
                        if let Some(window) = h.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .ok();

            println!("[DEBUG] Tauri setup starting...");
            let state = EditorState::new(app.handle());
            println!("[DEBUG] EditorState created successfully");

            // Listen for terminal-input events from AI
            let h = app.handle().clone();
            app.listen("terminal-input", move |event: tauri::Event| {
                if let Ok(args) = serde_json::from_str::<Value>(event.payload()) {
                    let data = args["data"].as_str().unwrap_or_default().to_string();
                    let term_id = args["term_id"].as_str().map(|s| s.to_string());

                    let state = h.state::<EditorState>();
                    let mut writers = state.terminal_writers.lock().unwrap();

                    // Use specified ID or find first available
                    let target_id = term_id.or_else(|| writers.keys().next().cloned());

                    if let Some(id) = target_id {
                        if let Some(writer) = writers.get_mut(&id) {
                            let _ = writer.write_all(data.as_bytes());
                            let _ = writer.flush();
                        }
                    }
                }
            });

            app.manage(state.browser_state.clone());
            let mcp_registry = state.mcp_registry.clone();
            app.manage(state.attachment_manager.clone());
            app.manage(state);

            // Initialize MCP servers in background
            tauri::async_runtime::spawn(async move {
                let _ = mcp_registry.initialize_servers().await;
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            mount_aim_memory,
            open_folder,
            get_file_tree,
            get_directory_contents,
            read_file,
            write_file,
            update_project_memory,
            create_file,
            create_directory,
            rename_path,
            delete_path,
            list_dir_flat,
            search_project,
            get_git_branch,
            git_status,
            git_stage,
            git_unstage,
            git_unstage,
            git_commit,
            git_revert,
            git_stash,
            git_stash_pop,
            git_get_unmerged,
            get_api_keys,
            save_api_keys,
            list_provider_models,
            ai_chat,
            ai_inline_complete,
            spawn_terminal,
            write_to_terminal,
            resize_terminal,
            close_terminal,
            get_available_shells,
            search_extensions,
            install_extension,
            get_popular_extensions,
            get_installed_extensions,
            get_extension_details,
            uninstall_extension,
            get_installed_themes,
            load_extension_theme,
            get_extension_contributions,
            ext_host_init,
            ext_host_send,
            debug_start,
            debug_send,
            debug_stop,
            check_activation_event,
            adb_list_devices,
            adb_list_emulators,
            spawn_emulator,
            set_active_device,
            set_ai_model,
            set_advisor_model,
            list_mcp_servers,
            add_mcp_server,
            remove_mcp_server,
            backend_ping,
            get_git_history,
            git_diff,
            adb_install_and_run,
            set_ollama_url,
            check_ollama_status,
            pull_ollama_model,
            stop_ai_agent,
            pause_ai_agent,
            resume_ai_agent,
            register_ida_pro,
            ai_execute_command,
            ai_modify_file,
            propose_file_change,
            get_icon_theme_mapping,
            hunt_api_keys,
            optimize_memory,
            git_clone,
            start_mitm_server,
            stop_mitm_server,
            get_mitm_status,
            editor_get_active_file,
            analyze_file_symbols,
            terminal_read_output,
            terminal_toggle,
            terminal_terminate,
            terminal_get_status,
            open_ai_login,
            save_ai_session,
            capture_ai_session,
            get_emulator_screenshot,
            emulator_tap,
            get_process_stats,
            resolve_keybinding,
            set_context_key,
            evaluate_when_clause,
            get_settings,
            update_settings,
            get_config_path,
            get_android_config,
            set_android_sdk_path,
            set_active_root,
            install_vsix,
            get_running_extensions,
            open_file,
            save_file,
            get_highlights,
            list_directory,
            switch_to_buffer,
            lsp_start,
            lsp_send_request,
            lsp_stop,
            validate_path,
            create_dir,
            browser::browser_open,
            browser::browser_navigate,
            browser::browser_screenshot,
            browser::browser_click,
            browser::browser_type,
            browser::browser_read_dom,
            browser::browser_capture_vision_context,
            browser::browser_close,
            // NEW: Tool Registry Backend Commands
            glob_files,
            grep_files,
            write_file_content,
            web_fetch,
            specs_commands::cmd_specs_create_project,
            specs_commands::cmd_specs_get_projects,
            specs_commands::cmd_specs_generate_layout,
            specs_commands::cmd_specs_get_project_tasks,
            specs_commands::cmd_specs_get_project_files,
            specs_commands::cmd_specs_get_extended_project_layout,
            specs_commands::cmd_specs_retry_task,
            specs_commands::cmd_specs_set_project_provider,
            specs_commands::cmd_specs_get_project_by_name,
            specs_commands::cmd_specs_delete_project,
            specs_commands::cmd_specs_clear_history,
            specs_commands::cmd_specs_delete_task,
            ai_tool_result,
            get_system_health,
            get_visual_graph,
            generate_visual_graph,
            query_workspace_memory,
            compress_session_data,
            get_memory_savings,
            benchmark_ane,
            query_performance_history,
            get_file_context,
            get_neural_omni_graph,
            get_all_memory_slots,
            list_chat_sessions,
            load_chat_session,
            archive_chat_session,
            create_new_session,
            get_agent_messages,
            get_brain_telemetry,
            select_and_process_attachment,
            vision_bridge::capture_preview_screenshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
