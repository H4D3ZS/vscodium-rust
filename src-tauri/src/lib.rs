use base64::{engine::general_purpose, Engine as _};
use portable_pty::{native_pty_system, Child, CommandBuilder, MasterPty, PtySize};
use ropey::Rope;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::Ordering;
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
mod ghost_runtime;
// mod browser_bridge; // Redundant, functionality in browser.rs
mod context_indexer;
mod patch_engine;
mod tool_invoker;
mod visual_lab;
mod attachment_manager;
use attachment_manager::{AttachmentManager, select_and_process_attachment};
mod knowledge_distiller;
use knowledge_distiller::KnowledgeDistiller;
mod vision_bridge;
mod workflow_engine;
mod kairos;
mod vfs_bridge;
mod mcp_server;
mod memory_layer;
mod hades_harness;
mod airi_bridge;
mod security_distiller;
mod binary_analyzer;
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

mod emulator_stream;
use emulator_stream::{
    list_available_avds,
    spawn_emulator_by_name,
    list_running_emulators,
    start_emulator_stream,
    stop_emulator_stream,
    get_stream_status,
};

mod scrcpy;
use scrcpy::{
    start_scrcpy_stream,
    stop_scrcpy_stream,
    capture_emulator_frame,
    send_emulator_tap,
    send_emulator_swipe,
    send_emulator_text,
    send_emulator_key,
    get_scrcpy_status,
    spawn_emulator_headless,
};

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
mod rules_engine;
mod shadow_workspace;
mod specs_commands;
mod ai_prompts;

mod vector_indexer;
use vector_indexer::VectorIndexer;

mod git_checkpoints;
use git_checkpoints::GitCheckpoint;

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
    buffers: tokio::sync::Mutex<HashMap<String, Rope>>,
    active_path: tokio::sync::Mutex<Option<String>>,
    settings: tokio::sync::Mutex<Settings>,
    terminal_masters: tokio::sync::Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    terminal_writers: tokio::sync::Mutex<HashMap<String, Box<dyn Write + Send>>>,
    terminal_processes: tokio::sync::Mutex<HashMap<String, Box<dyn Child + Send>>>,
    lsp_client: Arc<tokio::sync::Mutex<LspClient>>,
    pub lsp_diagnostics: lsp::DiagnosticsMap,
    context_keys: Arc<ContextKeyRegistry>,
    ext_host: Arc<tokio::sync::Mutex<ExtensionHostManager>>,
    keybindings: Arc<tokio::sync::Mutex<KeybindingRegistry>>,
    debug_manager: Arc<tokio::sync::Mutex<DebugManager>>,
    activation_manager: Arc<tokio::sync::Mutex<ActivationManager>>,
    perf_monitor: Arc<PerformanceMonitor>,
    pub ai_engine: Arc<Sentient>,
    ollama_url: tokio::sync::Mutex<String>,
    config_dir: PathBuf,
    active_root: tokio::sync::Mutex<Option<PathBuf>>,
    current_model: tokio::sync::Mutex<String>,
    active_device: tokio::sync::Mutex<Option<String>>,
    android_sdk_path: tokio::sync::Mutex<Option<String>>,
    auth_state: Arc<ai_auth::AuthState>,
    browser_state: Arc<browser::BrowserState>,
    mcp_registry: Arc<mcp_registry::McpRegistry>,
    terminal_buffers: tokio::sync::Mutex<HashMap<String, Vec<String>>>,
    pub ai_tools: Arc<ai_tools::AiTools>,
    pub memory_store: Arc<memory_store::MemoryStore>,
    memory_optimizer: Arc<memory_optimizer::MemoryOptimizer>,
    advisor_model: tokio::sync::Mutex<Option<String>>,
    pub specs_db: Arc<specs_db::SpecDb>,
    pub worker_manager: Arc<workers::WorkerManager>,
    pub attachment_manager: Arc<AttachmentManager>,
    #[allow(dead_code)]
    pub knowledge_distiller: Arc<KnowledgeDistiller>,
    pub patch_engine: Arc<tokio::sync::Mutex<patch_engine::PatchEngine>>,
    pub ghost_runtime: Arc<ghost_runtime::GhostRuntime>,
    pub kairos: Arc<kairos::KairosEngine>,
    pub mcp_server: Arc<mcp_server::McpServer>,
    pub vfs_bridge: Arc<vfs_bridge::VfsBridge>,
    pub shadow_workspace: Arc<shadow_workspace::ShadowWorkspace>,
    pub memory_layer: Arc<memory_layer::MemoryLayer>,
    pub hades_harness: Arc<hades_harness::HadesHarness>,
    pub context_indexer: Arc<ContextIndexer>,
    pub vector_indexer: Arc<VectorIndexer>,
    pub git_checkpoints: Arc<GitCheckpoint>,
}

impl EditorState {
    pub async fn terminal_read_output(&self, id: String) -> Result<String, String> {
        let buffers = self.terminal_buffers.lock().await;
        let history = buffers
            .get(&id)
            .ok_or_else(|| "Terminal not found".to_string())?;
        Ok(history.join(""))
    }
}

impl EditorState {
    fn new(app: &tauri::AppHandle) -> Self {
        // DIAGNOSTIC: Capture panics to a log file before process exits
        std::panic::set_hook(Box::new(|info| {
            let msg = format!(
                "[CRASH] PANIC: {}\nLocation: {:?}\nTime: {:?}\n",
                info,
                info.location(),
                std::time::SystemTime::now()
            );
            eprintln!("{}", msg);
            let _ = std::fs::write("crash.log", &msg);
            // Also try to append to a persistent log
            let _ = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open("vscodium_crash.log")
                .and_then(|mut f| { use std::io::Write; writeln!(f, "{}", msg) });
        }));
        println!("[DEBUG] Panic hook installed. Initializing EditorState...");
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
        let shadow_workspace = Arc::new(crate::shadow_workspace::ShadowWorkspace::new(root.clone()));
        let patch_engine = Arc::new(tokio::sync::Mutex::new(patch_engine::PatchEngine::new(shadow_workspace.clone())));
        let ghost_runtime = Arc::new(ghost_runtime::GhostRuntime::new(root.clone()));
        
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
            patch_engine.clone(),
            ghost_runtime.clone(),
            shadow_workspace.clone(),
        ));
        
        let airi_bridge = crate::airi_bridge::AiriBridge::new();
        let airi_clone = airi_bridge.clone();
        let app_airi = app.clone();
        
        tauri::async_runtime::spawn(async move {
            airi_clone.init(app_airi).await;
        });

        let mut lock = sentient.airi.blocking_lock();
        *lock = Some(airi_bridge);
        drop(lock);

        println!("[DEBUG] Sentient initialized");
        
        let memory_layer = Arc::new(memory_layer::MemoryLayer::new(root.clone()));
        let hades_harness = Arc::new(hades_harness::HadesHarness::new(
            sentient.clone(),
            memory_layer.clone(),
            shadow_workspace.clone(),
            patch_engine.clone(),
            ghost_runtime.clone(),
        ));
        
        let sentient_clone = sentient.clone();
        let app_sentient = app.clone();
        tauri::async_runtime::spawn(async move {
            sentient_clone.set_app_handle(app_sentient).await;
        });

        let pe = patch_engine.clone();
        let app_clone = app.clone();
        tauri::async_runtime::spawn(async move {
            let engine = pe.lock().await;
            engine.set_app_handle(app_clone).await;
        });

        // Initialize and start Omni-Context Indexer (Phase 44)
        let context_indexer = Arc::new(ContextIndexer::new(
            sentient.memory_store.clone(),
            root.clone(),
        ));
        let ci_for_spawn = context_indexer.clone();
        tauri::async_runtime::spawn(async move {
            ci_for_spawn.start_background_indexing().await;
        });

        app.manage(context_indexer.clone());

        // Initialize Vector Indexer (Cursor-like codebase semantic search)
        let vector_indexer = Arc::new(VectorIndexer::new(root.clone()).expect("Failed to init vector indexer"));
        let vi_clone = vector_indexer.clone();
        tauri::async_runtime::spawn(async move {
            let _ = vi_clone.index_codebase().await;
        });

        // Initialize Git Checkpoints (auto-snapshot before AI edits)
        let git_checkpoints = Arc::new(GitCheckpoint::new(root.clone()));

        let mut ext_dirs = vec![config_dir.join("extensions")];
        let builtin_ext_dir = root.join("vscode").join("extensions");
        if builtin_ext_dir.exists() {
            ext_dirs.push(builtin_ext_dir);
        }

        let specs_db = Arc::new(specs_db::SpecDb::new(config_dir.join("specs.db")).expect("Failed to init specs DB"));
        let worker_manager = Arc::new(workers::WorkerManager::new(specs_db.clone(), sentient.clone(), root.clone()));

        let wm_clone = worker_manager.clone();
        tauri::async_runtime::spawn(async move {
            wm_clone.start_loop().await;
        });

        let kairos = Arc::new(kairos::KairosEngine::new(
            context_indexer.clone(),
            sentient.memory_store.clone(),
            Arc::new(tokio::sync::Mutex::new(Some(root.clone()))),
        ));
        let k_clone = kairos.clone();
        tauri::async_runtime::spawn(async move {
            k_clone.start_loop().await;
        });

        let vfs_bridge = Arc::new(vfs_bridge::VfsBridge::new(root.clone()));
        let mcp_server = Arc::new(mcp_server::McpServer::new(sentient.ai_tools.clone()));
        let mcp_server_clone = mcp_server.clone();
        tauri::async_runtime::spawn(async move {
            mcp_server_clone.start(1537).await;
        });

        // Shared diagnostics map — owned by EditorState, borrowed by LspClient
        let shared_lsp_diags: lsp::DiagnosticsMap =
            Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
        let lsp_client_inst = LspClient::with_diagnostics(shared_lsp_diags.clone());

        Self {
            buffers: tokio::sync::Mutex::new(HashMap::new()),
            active_path: tokio::sync::Mutex::new(None),
            settings: tokio::sync::Mutex::new(Settings {
                theme: "vs-dark".to_string(),
                font_size: 14,
            }),
            terminal_masters: tokio::sync::Mutex::new(HashMap::new()),
            terminal_writers: tokio::sync::Mutex::new(HashMap::new()),
            terminal_processes: tokio::sync::Mutex::new(HashMap::new()),
            lsp_client: Arc::new(tokio::sync::Mutex::new(lsp_client_inst)),
            lsp_diagnostics: shared_lsp_diags,
            context_keys: Arc::new(ContextKeyRegistry::new()),
            ext_host: Arc::new(tokio::sync::Mutex::new(ExtensionHostManager::new(ext_dirs))),
            keybindings: Arc::new(tokio::sync::Mutex::new(KeybindingRegistry::new())),
            debug_manager: Arc::new(tokio::sync::Mutex::new(DebugManager::new())),
            activation_manager: Arc::new(tokio::sync::Mutex::new(ActivationManager::new())),
            perf_monitor: Arc::new(PerformanceMonitor::new()),
            ai_engine: sentient.clone(),
            ollama_url: tokio::sync::Mutex::new("http://127.0.0.1:1536".to_string()),
            config_dir: config_dir.clone(),
            active_root: tokio::sync::Mutex::new(Some(root)),
            current_model: tokio::sync::Mutex::new("gpt-4o".to_string()),
            active_device: tokio::sync::Mutex::new(None),
            android_sdk_path: tokio::sync::Mutex::new(None),
            auth_state,
            browser_state,
            mcp_registry: Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json"))),
            terminal_buffers: tokio::sync::Mutex::new(HashMap::new()),
            memory_optimizer,
            advisor_model: tokio::sync::Mutex::new(None),
            specs_db,
            worker_manager,
            attachment_manager,
            knowledge_distiller,
            patch_engine,
            ghost_runtime,
            kairos,
            mcp_server,
            vfs_bridge,
            shadow_workspace,
            memory_store: sentient.memory_store.clone(),
            ai_tools: sentient.ai_tools.clone(),
            memory_layer,
            hades_harness,
            context_indexer,
            vector_indexer,
            git_checkpoints,
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

#[tauri::command]
async fn store_message(
    state: State<'_, EditorState>,
    role: String,
    content: String,
    timestamp: i64,
) -> Result<(), String> {
    state.ai_engine.memory_store.store_message_params(role, content, timestamp).await;
    Ok(())
}

#[tauri::command]
async fn sync_agent_messages(
    state: State<'_, EditorState>,
    messages: Vec<crate::ai_engine::ChatMessage>,
) -> Result<(), String> {
    let mut store_messages = state.ai_engine.memory_store.messages.write().await;
    *store_messages = messages;
    state.ai_engine.memory_store.is_dirty.store(true, Ordering::Relaxed);
    Ok(())
}

#[derive(Serialize, Deserialize)]
struct AiResponse {
    content: String,
}

#[tauri::command]
async fn mount_neural_project(
    state: State<'_, EditorState>,
    indexer: State<'_, Arc<ContextIndexer>>,
) -> Result<String, String> {
    let root = state.ai_engine.get_tools().get_root_path();
    let aim_dir = root.join(".aim");
    
    println!("[CONTEXT] Manually neuralizing project at: {:?}", root);
    
    if !aim_dir.exists() {
        std::fs::create_dir_all(&aim_dir).map_err(|e| format!("Failed to create .aim dir: {}", e))?;
    }
    
    // Remount to ensure memory.aim creation and loading
    state.ai_engine.memory_store.mount(Some(root.clone())).await;
    
    // Trigger immediate indexing
    let _ = indexer.trigger_index_cycle().await;
    
    Ok("Project Neuralization Started! .aim detected and indexing triggered.".to_string())
}

#[tauri::command]
async fn open_file(state: State<'_, EditorState>, path: String) -> Result<String, String> {
    let content = fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

    let mut buffers = state.buffers.lock().await;
    buffers.insert(path.clone(), Rope::from_str(&content));

    let mut active = state.active_path.lock().await;
    *active = Some(path);

    Ok(content)
}

#[tauri::command]
async fn save_file(state: State<'_, EditorState>, path: String, content: String) -> Result<(), String> {
    fs::write(&path, &content).map_err(|e| format!("Failed to write file: {}", e))?;
    let mut buffers = state.buffers.lock().await;
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
        let mut root = state.active_root.lock().await;
        *root = Some(path.clone());
        state.ai_engine.set_root_path(path.clone());
        return Ok(Some(path.to_string_lossy().to_string()));
    }
    Ok(None)
}

#[tauri::command]
async fn switch_to_buffer(state: State<'_, EditorState>, path: String) -> Result<String, String> {
    let buffers = state.buffers.lock().await;
    if let Some(rope) = buffers.get(&path) {
        let mut active = state.active_path.lock().await;
        *active = Some(path);
        Ok(rope.to_string())
    } else {
        Err("Buffer not found".to_string())
    }
}

#[tauri::command]
async fn get_settings(state: State<'_, EditorState>) -> Result<Settings, String> {
    // Try loading from disk first; fall back to in-memory defaults
    let settings_path = state.config_dir.join("settings.json");
    if settings_path.exists() {
        if let Ok(raw) = fs::read_to_string(&settings_path) {
            if let Ok(loaded) = serde_json::from_str::<Settings>(&raw) {
                let mut s = state.settings.lock().await;
                *s = loaded.clone();
                return Ok(loaded);
            }
        }
    }
    Ok(state.settings.lock().await.clone())
}

#[tauri::command]
async fn update_settings(state: State<'_, EditorState>, settings: Settings) -> Result<(), String> {
    // Persist to disk
    let settings_path = state.config_dir.join("settings.json");
    let raw = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(&settings_path, raw).map_err(|e| e.to_string())?;
    // Update in-memory
    let mut s = state.settings.lock().await;
    *s = settings;
    Ok(())
}

#[tauri::command]
async fn lsp_start(
    state: State<'_, EditorState>,
    app: tauri::AppHandle,
    command: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.start(&command, app).map_err(|e| e.to_string())
}

#[tauri::command]
async fn lsp_send_request(
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
async fn lsp_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.stop();
    Ok(())
}

#[tauri::command]
async fn lsp_initialized(state: State<'_, EditorState>) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.send_initialized().map_err(|e| e.to_string())
}

#[tauri::command]
async fn lsp_did_open(
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
async fn lsp_did_change(
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
async fn lsp_did_save(state: State<'_, EditorState>, uri: String) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.did_save(&uri).map_err(|e| e.to_string())
}

#[tauri::command]
async fn lsp_set_workspace(
    state: State<'_, EditorState>,
    root_uri: String,
) -> Result<(), String> {
    let mut lsp = state.lsp_client.lock().await;
    lsp.set_workspace_root(&root_uri).map_err(|e| e.to_string())
}

#[tauri::command]
async fn lsp_get_diagnostics(
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
async fn lsp_is_running(state: State<'_, EditorState>) -> Result<bool, String> {
    let lsp = state.lsp_client.lock().await;
    Ok(lsp.is_running())
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
async fn ext_host_init(state: State<'_, EditorState>, app: tauri::AppHandle) -> Result<(), String> {
    let mut eh = state.ext_host.lock().await;
    eh.scan_extensions().map_err(|e| e.to_string())?;
    eh.start(app).map_err(|e| e.to_string())
}

#[tauri::command]
async fn ext_host_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut eh = state.ext_host.lock().await;
    eh.send_message(msg).map_err(|e| e.to_string())
}

#[tauri::command]
async fn resolve_keybinding(state: State<'_, EditorState>, key: String) -> Result<Option<String>, String> {
    let kb = state.keybindings.lock().await;
    Ok(kb.resolve_key(&key, &state.context_keys))
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
        let eh = state.ext_host.lock().await;
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
            let mut eh = state.ext_host.lock().await;
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
    let mut eh = state.ext_host.lock().await;
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
async fn get_installed_extensions(
    state: State<'_, EditorState>,
) -> Result<Vec<extension_host::ExtensionMetadata>, String> {
    let eh = state.ext_host.lock().await;
    Ok(eh.extensions.clone())
}

#[tauri::command]
fn install_vsix(_state: State<'_, EditorState>, path: String) -> Result<(), String> {
    // Basic stub for manual VSIX installation
    println!("Installing VSIX from {}", path);
    Ok(())
}

#[tauri::command]
async fn get_running_extensions(state: State<'_, EditorState>) -> Result<Vec<extension_host::ExtensionMetadata>, String> {
    let eh = state.ext_host.lock().await;
    Ok(eh.extensions.clone())
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
async fn get_process_stats(state: State<'_, EditorState>) -> Result<performance::ProcessStats, String> {
    Ok(state
        .perf_monitor
        .get_stats()
        .await
        .unwrap_or(performance::ProcessStats {
            memory_mb: 0,
            cpu_usage: 0.0,
            total_ram_gb: 0,
            available_ram_gb: 0,
        }))
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
        ("elevenlabs_api_key", "ELEVENLABS_API_KEY"),
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

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct ApiKeys {
    pub openai: Option<String>,
    pub anthropic: Option<String>,
    pub google: Option<String>,
    pub groq: Option<String>,
    pub openrouter: Option<String>,
    pub mistral: Option<String>,
    pub xai: Option<String>,
    pub alibaba: Option<String>,
    pub apiradar: Option<String>,
    pub elevenlabs_api_key: Option<String>,
    pub elevenlabs_voice_id: Option<String>, // Selected ElevenLabs voice ID
}

#[tauri::command]
async fn save_api_keys(
    state: State<'_, EditorState>,
    keys: Value,
) -> Result<HashMap<String, String>, String> {
    // ── Step 1: Load existing saved keys from disk ──
    let path = state.config_dir.join("api_keys.json");
    let existing_keys: ApiKeys = if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        serde_json::from_str(&content).unwrap_or_default()
    } else {
        ApiKeys::default()
    };

    // ── Step 2: Parse incoming keys ──
    let incoming: ApiKeys =
        serde_json::from_value(keys).map_err(|e| format!("Invalid keys format: {}", e))?;

    // ── Step 3: Merge — only overwrite non-None incoming values ──
    let mut merged = existing_keys.clone();
    if incoming.openai.is_some() { merged.openai = incoming.openai; }
    if incoming.anthropic.is_some() { merged.anthropic = incoming.anthropic; }
    if incoming.google.is_some() { merged.google = incoming.google; }
    if incoming.groq.is_some() { merged.groq = incoming.groq; }
    if incoming.openrouter.is_some() { merged.openrouter = incoming.openrouter; }
    if incoming.mistral.is_some() { merged.mistral = incoming.mistral; }
    if incoming.xai.is_some() { merged.xai = incoming.xai; }
    if incoming.alibaba.is_some() { merged.alibaba = incoming.alibaba; }
    if incoming.apiradar.is_some() { merged.apiradar = incoming.apiradar; }
    if incoming.elevenlabs_api_key.is_some() { merged.elevenlabs_api_key = incoming.elevenlabs_api_key; }
    if incoming.elevenlabs_voice_id.is_some() { merged.elevenlabs_voice_id = incoming.elevenlabs_voice_id; }

    let mut results = HashMap::new();
    let hunter = ApiRadarHunter::new();

    // Validate OpenAI
    if let Some(ref k) = merged.openai {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("openai_api_key", k).await;
            if !alive {
                results.insert("openai".to_string(), format!("Dead: {}", details));
                merged.openai = None;
            } else {
                results.insert("openai".to_string(), "Alive".to_string());
                std::env::set_var("OPENAI_API_KEY", k);
            }
        }
    }
    // Validate Anthropic
    if let Some(ref k) = merged.anthropic {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("anthropic_api_key", k).await;
            if !alive {
                results.insert("anthropic".to_string(), format!("Dead: {}", details));
                merged.anthropic = None;
            } else {
                results.insert("anthropic".to_string(), "Alive".to_string());
                std::env::set_var("ANTHROPIC_API_KEY", k);
            }
        }
    }
    // Validate Google
    if let Some(ref k) = merged.google {
        if !k.is_empty() {
            let (alive, details) = hunter.validate_key("google_api_key", k).await;
            if !alive {
                results.insert("google".to_string(), format!("Dead: {}", details));
                merged.google = None;
            } else {
                results.insert("google".to_string(), "Alive".to_string());
                std::env::set_var("GOOGLE_API_KEY", k);
            }
        }
    }
    // Groq — no validation endpoint, just save and set env
    if let Some(ref k) = merged.groq {
        if !k.is_empty() {
            std::env::set_var("GROQ_API_KEY", k);
            results.insert("groq".to_string(), "Saved".to_string());
        }
    }
    // OpenRouter — no validation endpoint, just save and set env
    if let Some(ref k) = merged.openrouter {
        if !k.is_empty() {
            std::env::set_var("OPENROUTER_API_KEY", k);
            results.insert("openrouter".to_string(), "Saved".to_string());
        }
    }
    // Mistral
    if let Some(ref k) = merged.mistral {
        if !k.is_empty() {
            std::env::set_var("MISTRAL_API_KEY", k);
            results.insert("mistral".to_string(), "Saved".to_string());
        }
    }

    // ElevenLabs — validate by checking format
    if let Some(ref k) = merged.elevenlabs_api_key {
        if !k.is_empty() {
            if k.starts_with("sk_") {
                std::env::set_var("ELEVENLABS_API_KEY", k);
                results.insert("elevenlabs_api_key".to_string(), "Saved".to_string());
            } else {
                results.insert("elevenlabs_api_key".to_string(), "Dead: Invalid format (must start with sk_)".to_string());
                merged.elevenlabs_api_key = None;
            }
        }
    }

    // ── Step 4: Write merged keys to disk ──
    let contents = serde_json::to_string_pretty(&merged)
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
async fn set_ai_model(state: State<'_, EditorState>, model: String) -> Result<(), String> {
    let mut current = state.current_model.lock().await;
    *current = model;
    Ok(())
}

#[tauri::command]
async fn set_advisor_model(state: State<'_, EditorState>, model: Option<String>) -> Result<(), String> {
    let mut current = state.advisor_model.lock().await;
    *current = model;
    Ok(())
}

#[tauri::command]
async fn adb_list_devices(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().await;
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
async fn set_active_device(state: State<'_, EditorState>, device: String) -> Result<(), String> {
    let mut active = state.active_device.lock().await;
    *active = Some(device);
    Ok(())
}

#[tauri::command]
fn adb_install_and_run(_state: State<'_, EditorState>, _apk_path: String) -> Result<(), String> {
    // Stub
    Ok(())
}

#[tauri::command]
async fn get_android_config(state: State<'_, EditorState>) -> Result<Value, String> {
    let sdk_path = state.android_sdk_path.lock().await;
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
async fn set_android_sdk_path(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut sdk = state.android_sdk_path.lock().await;
    *sdk = Some(path);
    Ok(())
}

#[tauri::command]
async fn adb_list_emulators(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let sdk_path = state.android_sdk_path.lock().await;
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
async fn spawn_emulator(state: State<'_, EditorState>, avd: String) -> Result<(), String> {
    let sdk_path = state.android_sdk_path.lock().await;
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
async fn set_active_root(state: State<'_, EditorState>, path: Option<String>) -> Result<(), String> {
    let mut root = state.active_root.lock().await;
    if let Some(p) = path {
        let path_buf = PathBuf::from(p);
        *root = Some(path_buf.clone());
        state.ai_engine.set_root_path(path_buf);
    } else {
        *root = None;
    }
    Ok(())
}

/// Returns the backend's current active project root path.
/// Used by the frontend to restore state after hot-reload or app restart.
#[tauri::command]
async fn get_active_root(state: State<'_, EditorState>) -> Result<Option<String>, String> {
    let root = state.active_root.lock().await;
    Ok(root.as_ref().map(|p| p.to_string_lossy().to_string()))
}

#[tauri::command]
async fn rename_path(old_path: String, new_path: String) -> Result<(), String> {
    fs::rename(old_path, new_path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_path(path: String) -> Result<(), String> {
    let p = std::path::Path::new(&path);
    if p.is_dir() {
        std::fs::remove_dir_all(p).map_err(|e| format!("Failed to delete directory: {}", e))?;
    } else {
        std::fs::remove_file(p).map_err(|e| format!("Failed to delete file: {}", e))?;
    }
    Ok(())
}

#[tauri::command]
async fn create_file(path: String) -> Result<(), String> {
    fs::File::create(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn create_dir(path: String) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn create_directory(path: String) -> Result<(), String> {
    fs::create_dir_all(path).map_err(|e| e.to_string())?;
    Ok(())
}

async fn is_path_valid(_state: &EditorState, _path: &PathBuf) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
async fn validate_path(state: State<'_, EditorState>, path: PathBuf) -> Result<(), String> {
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
async fn list_dir_flat(path: PathBuf) -> Result<Vec<FileEntry>, String> {
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

            if name.is_empty() {
                continue;
            }

            // Skip ignored patterns at the high level to keep UI clean
            if ignore_list.iter().any(|&p| name == p) {
                continue;
            }

            // Use entry.metadata() which avoids an extra syscall and handles
            // Windows symlinks/junctions gracefully. Skip on any error instead
            // of hard-failing the entire tree scan.
            let is_dir = match entry.metadata().or_else(|_| fs::metadata(&entry_path)) {
                Ok(m) => m.is_dir(),
                Err(_) => continue, // inaccessible entry — skip silently
            };

            tree.push(FileEntry {
                name,
                path: entry_path.to_string_lossy().to_string(),
                is_dir,
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
        let root_guard = state.active_root.lock().await;
        root_guard
            .clone()
            .ok_or_else(|| "No project open".to_string())?
    };

    // EXTREME SCALE FIX: Never walk recursively on initial load.
    // Only return the top-level files/folders of the root.
    list_dir_flat(root).await
}

#[tauri::command]
async fn get_directory_contents(
    state: tauri::State<'_, EditorState>,
    path: String,
) -> Result<Vec<FileEntry>, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;
    list_dir_flat(path_buf).await
}

#[tauri::command]
async fn read_file(state: tauri::State<'_, EditorState>, path: String) -> Result<String, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;
    fs::read_to_string(path).map_err(|e| e.to_string())
}

#[tauri::command]
async fn write_file(
    state: tauri::State<'_, EditorState>,
    path: String,
    content: String,
) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;
    fs::write(path, content).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_project_memory(
    state: tauri::State<'_, EditorState>,
    content: String,
) -> Result<(), String> {
    let root = state
        .active_root
        .lock()
        .await
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
async fn git_status(path: String) -> Result<Vec<git::GitFileStatus>, String> {
    let manager = GitManager::new();
    manager.get_status(path)
}

#[tauri::command]
async fn git_stage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.stage(path, &file_path)
}

#[tauri::command]
async fn git_unstage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.unstage(path, &file_path)
}

#[tauri::command]
async fn git_commit(path: String, message: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.commit(path, &message)
}

#[tauri::command]
async fn get_git_branch() -> Result<String, String> {
    let output = Command::new("git")
        .hidden()
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|_| "Git not found".to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
async fn get_git_history(path: String) -> Result<Vec<git::GitCommitInfo>, String> {
    let manager = GitManager::new();
    manager.get_history(path)
}

#[tauri::command]
async fn git_diff(path: String, hash: String) -> Result<String, String> {
    let manager = GitManager::new();
    manager.get_commit_diff(path, &hash)
}

/// Git blame for a file — returns one line per source line: "hash|author|date|summary"
#[tauri::command]
async fn git_blame(path: String, file_path: String) -> Result<Vec<String>, String> {
    use crate::process_ext::CommandExtHidden;
    let output = Command::new("git")
        .hidden()
        .args(&["blame", "--porcelain", "--", &file_path])
        .current_dir(&path)
        .output()
        .map_err(|e| e.to_string())?;

    let raw = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<String> = Vec::new();
    let mut current_hash = String::new();
    let mut current_author = String::new();
    let mut current_date = String::new();
    let mut current_summary = String::new();

    for line in raw.lines() {
        if line.starts_with('\t') {
            // Source line — emit blame entry
            lines.push(format!("{}|{}|{}|{}", current_hash, current_author, current_date, current_summary));
        } else if line.len() >= 40 && line.chars().next().map(|c| c.is_ascii_hexdigit()).unwrap_or(false) {
            current_hash = line[..40].to_string();
        } else if let Some(rest) = line.strip_prefix("author ") {
            current_author = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("author-time ") {
            // Unix timestamp → YYYY-MM-DD
            if let Ok(ts) = rest.trim().parse::<i64>() {
                let secs = ts;
                // Simple date calculation without external crate
                let days = secs / 86400;
                let y400 = days / 146097;
                let r400 = days % 146097;
                let y100 = (r400 / 36524).min(3);
                let r100 = r400 - y100 * 36524;
                let y4 = r100 / 1461;
                let r4 = r100 % 1461;
                let y1 = (r4 / 365).min(3);
                let year = y400 * 400 + y100 * 100 + y4 * 4 + y1 + 1970;
                let doy = r4 - y1 * 365 + 1;
                let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                let month_days: [i64; 12] = if leap {
                    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                } else {
                    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                };
                let mut month = 1i64;
                let mut d = doy;
                for md in &month_days {
                    if d <= *md { break; }
                    d -= md;
                    month += 1;
                }
                current_date = format!("{}-{:02}-{:02}", year, month, d);
            }
        } else if let Some(rest) = line.strip_prefix("summary ") {
            current_summary = rest.trim().chars().take(60).collect();
        }
    }

    Ok(lines)
}

/// Get unstaged diff for a specific file (for inline diff view in SCM)
#[tauri::command]
async fn git_diff_file(path: String, file_path: String) -> Result<String, String> {
    use std::process::Command;
    use crate::process_ext::CommandExtHidden;
    let output = Command::new("git")
        .hidden()
        .args(&["diff", "HEAD", "--", &file_path])
        .current_dir(&path)
        .output()
        .map_err(|e| e.to_string())?;
    if output.stdout.is_empty() {
        // Try unstaged diff (not yet committed)
        let output2 = Command::new("git")
            .hidden()
            .args(&["diff", "--", &file_path])
            .current_dir(&path)
            .output()
            .map_err(|e| e.to_string())?;
        return Ok(String::from_utf8_lossy(&output2.stdout).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
async fn search_project(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Vec<SearchResult>, String> {
    let root = state
        .active_root
        .lock()
        .await
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
async fn glob_files(
    state: State<'_, EditorState>,
    pattern: String,
    path: Option<String>,
) -> Result<Vec<String>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state.active_root.lock().await.clone().unwrap_or_else(|| PathBuf::from("."))
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
async fn grep_files(
    state: State<'_, EditorState>,
    pattern: String,
    path: Option<String>,
    include: Option<String>,
) -> Result<Vec<SearchResult>, String> {
    let root = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        state.active_root.lock().await.clone().unwrap_or_else(|| PathBuf::from("."))
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
async fn write_file_content(path: String, content: String) -> Result<(), String> {
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
async fn spawn_terminal(
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
        let root = state.active_root.lock().await;
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
                    {
                        let mut buffers = state.terminal_buffers.blocking_lock();
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
        .await
        .insert(id.clone(), pair.master);
    state
        .terminal_writers
        .lock()
        .await
        .insert(id.clone(), writer);
    state.terminal_processes.lock().await.insert(id, child);
    Ok(())
}

#[tauri::command]
async fn close_terminal(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    // Drop the master, writer, and kill the process
    state.terminal_writers.lock().await.remove(&id);
    state.terminal_masters.lock().await.remove(&id);
    if let Some(mut child) = state.terminal_processes.lock().await.remove(&id) {
        let _ = child.kill();
    }
    Ok(())
}

#[tauri::command]
async fn get_available_shells() -> Vec<String> {
    let mut shells = Vec::new();
    if cfg!(target_os = "windows") {
        // pwsh.exe = PowerShell 7+ (modern, preferred)
        let pwsh_paths = [
            "pwsh.exe",
            r"C:\Program Files\PowerShell\7\pwsh.exe",
            r"C:\Program Files\PowerShell\pwsh.exe",
        ];
        for p in &pwsh_paths {
            if std::path::Path::new(p).exists() || which_on_path(p) {
                shells.push(p.to_string());
                break;
            }
        }
        // PowerShell 5 (built-in Windows)
        shells.push("powershell.exe".to_string());
        // cmd.exe
        shells.push("cmd.exe".to_string());
        // Git Bash (very common on Windows dev machines)
        let git_bash_paths = [
            r"C:\Program Files\Git\bin\bash.exe",
            r"C:\Program Files (x86)\Git\bin\bash.exe",
        ];
        for p in &git_bash_paths {
            if std::path::Path::new(p).exists() {
                shells.push(p.to_string());
                break;
            }
        }
        // WSL bash
        if std::path::Path::new(r"C:\Windows\System32\wsl.exe").exists() {
            shells.push("wsl.exe".to_string());
        }
    } else {
        for path in &[
            "/bin/zsh",
            "/bin/bash",
            "/usr/bin/zsh",
            "/usr/bin/bash",
            "/bin/sh",
            "/usr/bin/fish",
        ] {
            if std::path::Path::new(path).exists() {
                shells.push(path.to_string());
            }
        }
    }
    shells
}

fn which_on_path(name: &str) -> bool {
    if let Ok(path_var) = std::env::var("PATH") {
        for dir in std::env::split_paths(&path_var) {
            if dir.join(name).exists() { return true; }
        }
    }
    false
}

#[tauri::command]
async fn write_to_terminal(
    state: State<'_, EditorState>,
    id: String,
    data: String,
) -> Result<(), String> {
    let mut writers = state.terminal_writers.lock().await;
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
async fn resize_terminal(
    state: State<'_, EditorState>,
    id: String,
    rows: u16,
    cols: u16,
) -> Result<(), String> {
    let masters = state.terminal_masters.lock().await;
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
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
async fn ai_chat(state: State<'_, EditorState>, request: AiRequest) -> Result<String, String> {
    // DIAGNOSTIC: Log to file before any async work so we know if we get here
    let log_entry = format!(
        "[ai_chat] START: provider={}, model={}, msg_count={}\n",
        request.provider, request.model, request.messages.len()
    );
    eprintln!("{}", log_entry.trim());
    let _ = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(log_entry.as_bytes()) });

    let result = state
        .ai_engine
        .clone()
        .autonomous_loop(request, None)
        .await
        .map_err(|e| {
            let err_log = format!("[ai_chat] ERROR: {}\n", e);
            eprintln!("{}", err_log.trim());
            let _ = std::fs::OpenOptions::new()
                .create(true).append(true)
                .open("ai_chat.log")
                .and_then(|mut f| { use std::io::Write; f.write_all(err_log.as_bytes()) });
            e.to_string()
        })?;

    let done_log = format!("[ai_chat] DONE: response_len={}\n", result.len());
    eprintln!("{}", done_log.trim());
    let _ = std::fs::OpenOptions::new()
        .create(true).append(true)
        .open("ai_chat.log")
        .and_then(|mut f| { use std::io::Write; f.write_all(done_log.as_bytes()) });

    // Satisfy AiResponse usage warning
    let _response = AiResponse { content: result.clone() };
    Ok(result)
}

#[tauri::command]
async fn ai_inline_complete(
    state: State<'_, EditorState>,
    prefix: String,
    suffix: String,
    language: String,
    file_path: String,
) -> Result<String, String> {
    // Use active provider/model from state for completions
    let current_model = state.current_model.lock().await.clone();
    let ollama_url_val = state.ollama_url.lock().await.clone();

    // Detect provider from model string
    let (comp_provider, comp_model, comp_ollama_url) = {
        let m = current_model.as_str();
        if m.contains(':') || (!m.contains('.') && m.contains('/')) || m.to_lowercase().starts_with("llama") || m.to_lowercase().starts_with("qwen") || m.to_lowercase().starts_with("deepseek") || m.to_lowercase().starts_with("gemma") || m.to_lowercase().starts_with("mistral") || m.to_lowercase().starts_with("phi") || m.to_lowercase().starts_with("codellama") {
            ("ollama".to_string(), m.to_string(), Some(ollama_url_val))
        } else if m.to_lowercase().contains("claude") {
            ("anthropic".to_string(), m.to_string(), None)
        } else if m.to_lowercase().contains("gemini") {
            ("google".to_string(), m.to_string(), None)
        } else if m.to_lowercase().contains("gpt") || m.to_lowercase().contains("o1") || m.to_lowercase().contains("o3") {
            ("openai".to_string(), m.to_string(), None)
        } else {
            // Fallback: try as Ollama since this is a self-hosted-first IDE
            ("ollama".to_string(), m.to_string(), Some(ollama_url_val))
        }
    };

    // For Ollama models that support FIM tokens (qwen2.5-coder, deepseek-coder, codellama)
    let uses_fim_tokens = comp_provider == "ollama" && (
        comp_model.to_lowercase().contains("coder") ||
        comp_model.to_lowercase().contains("codellama") ||
        comp_model.to_lowercase().contains("deepseek")
    );

    let fim_prompt = if uses_fim_tokens {
        format!("<fim_prefix>{}<fim_suffix>{}<fim_middle>", prefix, suffix)
    } else {
        format!(
            "Complete the following {} code. Return ONLY the completion text, no explanation, no markdown fencing, no extra whitespace.\n\n<prefix>\n{}\n</prefix>\n<suffix>\n{}\n</suffix>",
            language, prefix, suffix
        )
    };

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
        provider: comp_provider,
        model: comp_model,
        messages,
        temperature: Some(0.1), // Very low temp for precise completions
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Completion".to_string()),
        ollama_url: comp_ollama_url,
        tools: None,
    };

    // Use ai_engine's single-shot (non-autonomous) call
    let result = state.ai_engine.clone()
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
async fn unload_ollama_model(state: State<'_, EditorState>, name: String) -> Result<(), String> {
    state.attachment_manager.unload_model(&name).await
}

#[tauri::command]
async fn stop_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai_engine.stop();
    
    // Unload the current model from Ollama if it's an Ollama model
    let model = {
        let m = state.current_model.lock().await;
        m.clone()
    };
    
    println!("[DEBUG] stop_ai_agent: stopping reasoning and unloading model: {}", model);
    
    // Check if it's an Ollama model (either explicitly stated or a standard local model name)
    if model.to_lowercase().contains("ollama") || (!model.contains("|") && !model.is_empty()) {
        // Extract model name from format "Ollama|model-name" or "model-name"
        let model_name = model.split('|').last().unwrap_or(&model);
        let _ = state.attachment_manager.unload_model(model_name).await;
    }
    
    Ok(())
}

#[tauri::command]
async fn backend_ping() -> String {
    "System Pulse: ACTIVE".to_string()
}

#[tauri::command]
async fn get_ollama_ps(state: State<'_, EditorState>) -> Result<Value, String> {
    let ollama_url = {
        let u = state.ollama_url.lock().await;
        u.clone()
    };
    
    let client = reqwest::Client::new();
    let res = client.get(format!("{}/api/ps", ollama_url))
        .send()
        .await
        .map_err(|e| e.to_string())?;
        
    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    Ok(json)
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
        .clone()
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
async fn get_installed_themes(state: State<'_, EditorState>) -> Result<Vec<Value>, String> {
    let host = state.ext_host.lock().await;
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
async fn register_ida_pro(state: State<'_, EditorState>) -> Result<(), String> {
    let name = "ida-pro".to_string();
    let config = crate::mcp_registry::McpServerConfig::Stdio {
        command: "npx".to_string(),
        args: vec!["-y".to_string(), "@modelcontextprotocol/server-ida".to_string()],
        env: std::collections::HashMap::new(),
    };

    state
        .mcp_registry
        .add_server(name, config)
        .await
        .map_err(|e| e.to_string())
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
async fn propose_file_change(
    state: tauri::State<'_, EditorState>,
    path: String,
    content: String,
    description: String,
) -> Result<serde_json::Value, String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;

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
async fn accept_sentient_patch(state: tauri::State<'_, EditorState>, path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;
    
    let mut engine = state.patch_engine.lock().await;
    engine.commit_shadow(&path_buf).map_err(|e| e.to_string())
}

#[tauri::command]
async fn reject_sentient_patch(state: tauri::State<'_, EditorState>, path: String) -> Result<(), String> {
    let path_buf = PathBuf::from(&path);
    is_path_valid(&state, &path_buf).await?;

    let mut engine = state.patch_engine.lock().await;
    engine.discard_shadow(&path_buf);
    Ok(())
}

#[tauri::command]
async fn get_icon_theme_mapping(state: State<'_, EditorState>) -> Result<Value, String> {
    let host = state.ext_host.lock().await;

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
async fn get_extension_contributions(state: State<'_, EditorState>) -> Result<Value, String> {
    let host = state.ext_host.lock().await;
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
fn set_yolo_mode(state: State<'_, EditorState>, enabled: bool) -> Result<String, String> {
    state.ai_engine.set_yolo_mode(enabled);
    Ok(if enabled {
        "YOLO MODE ENGAGED — full sentient autonomy, no blockers.".to_string()
    } else {
        "Yolo mode disengaged.".to_string()
    })
}

#[tauri::command]
fn get_yolo_mode(state: State<'_, EditorState>) -> bool {
    state.ai_engine.is_yolo_mode()
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
async fn debug_start(
    state: State<'_, EditorState>,
    app: tauri::AppHandle,
    adapter_path: String,
) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().await;
    debug.start_session(&adapter_path, app)
}

#[tauri::command]
async fn debug_send(state: State<'_, EditorState>, msg: String) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().await;
    debug.send_message(msg)
}

#[tauri::command]
async fn debug_stop(state: State<'_, EditorState>) -> Result<(), String> {
    let mut debug = state.debug_manager.lock().await;
    debug.stop_session()
}

#[tauri::command]
async fn check_activation_event(state: State<'_, EditorState>, event: String) -> Result<(), String> {
    let mut am = state.activation_manager.lock().await;
    am.check_activation_requests(&event, state.ext_host.clone()).await;
    Ok(())
}

#[tauri::command]
async fn terminal_read_output(state: State<'_, EditorState>, id: String) -> Result<String, String> {
    state.terminal_read_output(id).await
}

#[tauri::command]
fn terminal_toggle(app: tauri::AppHandle, visible: bool) -> Result<(), String> {
    app.emit("toggle-terminal", visible)
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn terminal_terminate(state: State<'_, EditorState>, id: String) -> Result<(), String> {
    let mut processes = state.terminal_processes.lock().await;
    if let Some(mut child) = processes.remove(&id) {
        let _ = child.kill();
    }
    state.terminal_masters.lock().await.remove(&id);
    state.terminal_writers.lock().await.remove(&id);
    Ok(())
}

#[tauri::command]
async fn editor_get_active_file(
    state: tauri::State<'_, EditorState>,
) -> Result<serde_json::Value, String> {
    let sentient = state.ai_engine.clone();
    let tools = sentient.get_tools();
    tools
        .editor_get_active_file(serde_json::json!({}))
        .await
        .map_err(|e: anyhow::Error| e.to_string())
}

#[tauri::command]
async fn terminal_get_status(
    state: State<'_, EditorState>,
    id: String,
) -> Result<serde_json::Value, String> {
    let mut processes = state.terminal_processes.lock().await;
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
async fn analyze_file_symbols(
    state: State<'_, EditorState>,
    path: String,
) -> Result<serde_json::Value, String> {
    let sentient = state.ai_engine.clone();
    let tools = sentient.get_tools();
    tools
        .analyze_file_symbols(serde_json::json!({ "path": path }))
        .await
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
    crate::ai_auth::save_session(&state.auth_state, session).await;
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
        let mut current = state.ollama_url.lock().await;
        *current = url.clone();
    }

    state.ai_engine.set_ollama_url(url).await;
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
            .record_inference("CPU".to_string(), (eval_us / 1000) as u64)
            .await;

        return Ok(json!({
            "status": "success",
            "eval_us": eval_us,
            "device": "Intel/PC CPU"
        }));
    }

    if mode == "GPU" {
        let start_eval = std::time::Instant::now();
        
        // High-Speed Compute Path Check using Candle
        use candle_core::{Device, Tensor, DType};
        
        // Try obtaining the strongest available device (CUDA/Metal/Cpu)
        let device = if let Ok(d) = Device::new_cuda(0) { d } 
                    else if let Ok(d) = Device::new_metal(0) { d } 
                    else { Device::Cpu };
        
        // Run a lightweight matrix multiplication to measure kernel latency
        let res = (|| -> candle_core::Result<Tensor> {
            let a = Tensor::ones((512, 512), DType::F32, &device)?;
            let b = Tensor::ones((512, 512), DType::F32, &device)?;
            a.matmul(&b)
        })();

        let eval_us = start_eval.elapsed().as_micros();

        // Record the physical metric
        state
            .perf_monitor
            .record_inference("GPU".to_string(), (eval_us / 1000) as u64)
            .await;

        let device_name = match device {
            Device::Cuda(_) => "H4RDW4RE GPU (NVIDIA CUDA)",
            Device::Metal(_) => "H4RDW4RE GPU (Apple Metal)",
            Device::Cpu => "PC GPU (Compute Path/CPU Fallback)",
        };

        return Ok(json!({
            "status": if res.is_ok() { "success" } else { "warning" },
            "eval_us": eval_us,
            "device": device_name
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
            .record_inference("ANE".to_string(), (eval_us / 1000) as u64)
            .await;

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
            .record_inference("ANE".to_string(), (eval_us / 1000) as u64)
            .await;

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
    Ok(json!(state.perf_monitor.get_inference_history().await))
}

#[tauri::command]
async fn query_performance_history(state: State<'_, EditorState>) -> Result<Value, String> {
    let stats = state.perf_monitor.get_stats().await;
    Ok(json!({ "history": [stats] }))
}

#[tauri::command]
async fn git_revert(state: State<'_, EditorState>, hash: String) -> Result<(), String> {
    let root_lock = state
        .active_root
        .lock().await;
    let root = root_lock
        .clone()
        .ok_or("No active project")?;
    GitManager::new().revert_commit(root, &hash)
}

#[tauri::command]
async fn git_stash(state: State<'_, EditorState>) -> Result<(), String> {
    let root_lock = state
        .active_root
        .lock().await;
    let root = root_lock
        .clone()
        .ok_or("No active project")?;
    GitManager::new().stash_changes(root)
}

#[tauri::command]
async fn git_stash_pop(state: State<'_, EditorState>) -> Result<(), String> {
    let root_lock = state
        .active_root
        .lock().await;
    let root = root_lock
        .clone()
        .ok_or("No active project")?;
    GitManager::new().pop_stash(root)
}

#[tauri::command]
async fn git_get_unmerged(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let root_lock = state
        .active_root
        .lock().await;
    let root = root_lock
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
                    let mut writers = state.terminal_writers.blocking_lock();

                    // HADES SYNERGY: Final Synaptic Sync if mission complete
                    let final_text = ""; // Placeholder if needed, or get from context
                    let has_completion_keyword = final_text.contains("MISSION_ACCOMPLISHED");
                    if has_completion_keyword {
                    }
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
            get_ollama_ps,
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
            list_available_avds,
            spawn_emulator_by_name,
            list_running_emulators,
            start_emulator_stream,
            stop_emulator_stream,
            get_stream_status,
            // scrcpy commands
            spawn_emulator_headless,
            start_scrcpy_stream,
            stop_scrcpy_stream,
            capture_emulator_frame,
            send_emulator_tap,
            send_emulator_swipe,
            send_emulator_text,
            send_emulator_key,
            get_scrcpy_status,
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
            set_yolo_mode,
            get_yolo_mode,
            register_ida_pro,
            ai_execute_command,
            ai_modify_file,
            propose_file_change,
            get_icon_theme_mapping,
            get_brain_telemetry,
            get_agent_messages,
            store_message,
            sync_agent_messages,
            archive_chat_session,
            create_new_session,
            mount_neural_project,
            open_ai_login,
            save_ai_session,
            list_provider_models,
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
            get_active_root,
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
            lsp_initialized,
            lsp_did_open,
            lsp_did_change,
            lsp_did_save,
            lsp_set_workspace,
            lsp_get_diagnostics,
            lsp_is_running,
            lsp_completion,
            lsp_hover,
            lsp_goto_definition,
            lsp_find_references,
            lsp_rename_symbol,
            lsp_document_symbols,
            lsp_workspace_symbols,
            lsp_code_lens,
            replace_in_files,
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
            // NEW: AI Code Tools (Cursor-like features)
            ai_explain_code,
            ai_document_code,
            ai_generate_code,
            ai_refactor_code,
            ai_debug_code,
            ai_multi_cursor_edit,
            ai_pr_review,
            ai_get_context,
            select_and_process_attachment,
            unload_ollama_model,
            vision_bridge::capture_preview_screenshot,
            get_ollama_ps,
            mount_neural_project,
            accept_sentient_patch,
            reject_sentient_patch,
            web_search,
            search_codebase_files,
            get_directory_tree,
            get_git_file_hunks,
            git_diff_file,
            git_blame,
            list_project_files,
            lsp_format_document,
            // NEW: Vector Indexer (Cursor-like codebase search)
            vector_index_codebase,
            vector_search_codebase,
            vector_find_symbol,
            vector_get_index_stats,
            vector_get_file_chunks,
            // NEW: Git Checkpoints (auto-snapshot & rollback)
            git_create_checkpoint,
            git_list_checkpoints,
            git_rollback_checkpoint,
            git_get_checkpoint_diff,
            git_delete_checkpoint,
            git_auto_checkpoint,
            elevenlabs_get_voices,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ── ElevenLabs Voice Picker ────────────────────────────────────────────────

#[derive(Serialize, Deserialize, Debug)]
pub struct ElevenLabsVoice {
    pub voice_id: String,
    pub name: String,
    pub labels: Option<serde_json::Value>,
    pub preview_url: Option<String>,
    pub category: Option<String>,
    pub gender: Option<String>,
    pub age: Option<String>,
    pub accent: Option<String>,
}

#[tauri::command]
async fn elevenlabs_get_voices(state: State<'_, EditorState>) -> Result<Vec<ElevenLabsVoice>, String> {
    // Get API key from stored keys
    let path = state.config_dir.join("api_keys.json");
    let api_key: String = if path.exists() {
        let content = fs::read_to_string(&path).map_err(|e| e.to_string())?;
        let keys: ApiKeys = serde_json::from_str(&content).unwrap_or_default();
        keys.elevenlabs_api_key.unwrap_or_default()
    } else {
        String::new()
    };

    if api_key.is_empty() {
        return Err("ElevenLabs API key not configured".to_string());
    }

    // Fetch voices from ElevenLabs API
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.elevenlabs.io/v1/voices")
        .header("xi-api-key", &api_key)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !response.status().is_success() {
        let err = response.text().await.unwrap_or_default();
        return Err(format!("API error: {}", err));
    }

    let body: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    // Parse voices array
    let voices: Vec<ElevenLabsVoice> = body["voices"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .filter_map(|v| {
            let labels = v.get("labels").cloned();
            let gender = labels.as_ref().and_then(|l| l.get("gender")).and_then(|g| g.as_str()).map(String::from);
            let age = labels.as_ref().and_then(|l| l.get("age")).and_then(|a| a.as_str()).map(String::from);
            let accent = labels.as_ref().and_then(|l| l.get("accent")).and_then(|a| a.as_str()).map(String::from);
            let category = v.get("category").and_then(|c| c.as_str()).map(String::from);

            Some(ElevenLabsVoice {
                voice_id: v.get("voice_id").and_then(|id| id.as_str()).map(String::from)?,
                name: v.get("name").and_then(|n| n.as_str()).map(String::from)?,
                labels,
                preview_url: v.get("preview_url").and_then(|u| u.as_str()).map(String::from),
                category,
                gender,
                age,
                accent,
            })
        })
        .collect();

    Ok(voices)
}

// ── LSP Request-Response Commands (completions, hover, definition, references, rename) ─────────

#[tauri::command]
async fn lsp_completion(
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
async fn lsp_hover(
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
async fn lsp_goto_definition(
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
async fn lsp_find_references(
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
async fn lsp_rename_symbol(
    state: State<'_, EditorState>,
    uri: String,
    line: u32,
    character: u32,
    new_name: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() {
        return Err("LSP not running".to_string());
    }
    let result = client.request_with_response("textDocument/rename", json!({
        "textDocument": { "uri": uri },
        "position": { "line": line, "character": character },
        "newName": new_name
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

/// List all non-binary files in the active project root (for Ctrl+P Quick Open).
#[tauri::command]
async fn list_project_files(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let root = state.active_root.lock().await.clone()
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

/// Format the document at the given URI using LSP textDocument/formatting.
#[tauri::command]
async fn lsp_format_document(state: State<'_, EditorState>, uri: String) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() { return Err("LSP not running".to_string()); }
    let result = client.request_with_response("textDocument/formatting", json!({
        "textDocument": { "uri": uri },
        "options": { "tabSize": 4, "insertSpaces": true }
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

/// Replace all occurrences of `query` with `replacement` across the project.
/// Returns number of files modified.
#[tauri::command]
async fn replace_in_files(
    state: State<'_, EditorState>,
    query: String,
    replacement: String,
    case_sensitive: bool,
) -> Result<usize, String> {
    let root = state.active_root.lock().await.clone()
        .unwrap_or_else(|| PathBuf::from("."));
    let mut count = 0usize;
    for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
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
            // Case-insensitive replace preserving original case structure
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
async fn lsp_workspace_symbols(
    state: State<'_, EditorState>,
    query: String,
) -> Result<Value, String> {
    let mut client = state.lsp_client.lock().await;
    if !client.is_running() { return Ok(json!([])); }
    let result = client.request_with_response("workspace/symbol", json!({
        "query": query
    })).await.map_err(|e| e.to_string())?;
    Ok(result["result"].clone())
}

#[tauri::command]
async fn lsp_code_lens(
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
async fn lsp_document_symbols(
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

// ── Web Search via DuckDuckGo Instant Answer API ──────────────────────────────
#[tauri::command]
async fn web_search(query: String, num_results: Option<usize>) -> Result<Value, String> {
    let limit = num_results.unwrap_or(6).min(10);
    let encoded = urlencoding::encode(&query);
    let ddg_url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
        encoded
    );
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) vscodium-rust/1.0")
        .build()
        .map_err(|e| e.to_string())?;

    let resp = client.get(&ddg_url).send().await.map_err(|e| e.to_string())?;
    let body: Value = resp.json().await.map_err(|e| e.to_string())?;

    let mut results: Vec<Value> = Vec::new();

    if let Some(abstract_text) = body["Abstract"].as_str() {
        if !abstract_text.is_empty() {
            results.push(json!({
                "title": body["Heading"].as_str().unwrap_or(""),
                "url": body["AbstractURL"].as_str().unwrap_or(""),
                "snippet": abstract_text,
                "source": body["AbstractSource"].as_str().unwrap_or("DuckDuckGo"),
            }));
        }
    }

    if let Some(answer) = body["Answer"].as_str() {
        if !answer.is_empty() {
            results.push(json!({
                "title": format!("Answer: {}", query),
                "url": "",
                "snippet": answer,
                "source": "DuckDuckGo Instant",
            }));
        }
    }

    if let Some(topics) = body["RelatedTopics"].as_array() {
        for topic in topics.iter().take(limit.saturating_sub(results.len())) {
            if let Some(text) = topic["Text"].as_str() {
                if !text.is_empty() {
                    results.push(json!({
                        "title": text.chars().take(80).collect::<String>(),
                        "url": topic["FirstURL"].as_str().unwrap_or(""),
                        "snippet": text,
                        "source": "DuckDuckGo",
                    }));
                }
            }
        }
    }

    if results.is_empty() {
        results.push(json!({
            "title": format!("Search: {}", query),
            "url": format!("https://duckduckgo.com/?q={}", encoded),
            "snippet": "No instant answer found. See URL for full search.",
            "source": "fallback",
        }));
    }

    Ok(json!({ "query": query, "results": &results[..results.len().min(limit)], "count": results.len() }))
}

// ── Git gutter decorations: return added/modified/deleted lines for a file ──
#[tauri::command]
async fn get_git_file_hunks(path: String) -> Result<Value, String> {
    use std::process::Command;
    // git diff -U0 HEAD -- <path>  gives us exact line ranges
    let output = Command::new("git")
        .args(["diff", "-U0", "HEAD", "--", &path])
        .output()
        .map_err(|e| e.to_string())?;

    let diff = String::from_utf8_lossy(&output.stdout);
    let mut added: Vec<u32> = Vec::new();
    let mut modified: Vec<u32> = Vec::new();
    let mut deleted: Vec<u32> = Vec::new();

    // Parse unified diff @@ -a,b +c,d @@ headers
    for line in diff.lines() {
        if let Some(hunk) = line.strip_prefix("@@ ") {
            // Format: -<old_start>[,<old_count>] +<new_start>[,<new_count>]
            let parts: Vec<&str> = hunk.split_whitespace().collect();
            let old_part = parts.first().unwrap_or(&"-0");
            let new_part = parts.get(1).unwrap_or(&"+0");

            let parse_range = |s: &str| -> (u32, u32) {
                let s = s.trim_start_matches(['+', '-']);
                if let Some((start, count)) = s.split_once(',') {
                    (start.parse().unwrap_or(0), count.parse().unwrap_or(1))
                } else {
                    (s.parse().unwrap_or(0), 1)
                }
            };

            let (_old_start, old_count) = parse_range(old_part);
            let (new_start, new_count) = parse_range(new_part);

            if old_count == 0 {
                // Pure addition
                for l in new_start..new_start + new_count { added.push(l); }
            } else if new_count == 0 {
                // Pure deletion — mark the line before deletion
                deleted.push(if new_start == 0 { 1 } else { new_start });
            } else {
                // Modification
                for l in new_start..new_start + new_count { modified.push(l); }
            }
        }
    }

    // Also check for untracked (new file) — git diff shows nothing for staged new files
    if diff.is_empty() {
        // Check if file is untracked
        let status = Command::new("git")
            .args(["status", "--porcelain", &path])
            .output()
            .map_err(|e| e.to_string())?;
        let st = String::from_utf8_lossy(&status.stdout);
        if st.starts_with("??") || st.starts_with("A ") {
            // Whole file is new — mark all lines as added
            // Return a sentinel so frontend knows to mark all
            return Ok(json!({ "new_file": true, "added": [], "modified": [], "deleted": [] }));
        }
    }

    Ok(json!({ "new_file": false, "added": added, "modified": modified, "deleted": deleted }))
}

// ── @codebase search: grep codebase for files matching a query ─────────────
#[tauri::command]
async fn search_codebase_files(query: String, root: String) -> Result<Value, String> {
    use std::process::Command;
    let root_path = std::path::Path::new(&root);
    if !root_path.exists() {
        return Ok(json!({ "files": [] }));
    }

    // Use ripgrep if available, otherwise fallback to walkdir + contains
    let rg_result = Command::new("rg")
        .args(["--files-with-matches", "--max-count", "1", "--ignore-case",
               "--glob", "!target", "--glob", "!node_modules", "--glob", "!.git",
               &query, &root])
        .output();

    let files: Vec<String> = if let Ok(out) = rg_result {
        String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
            .take(10)
            .collect()
    } else {
        // Fallback: plain grep
        let grep_result = Command::new("grep")
            .args(["-ril", "--include=*.rs", "--include=*.ts", "--include=*.tsx",
                   "--include=*.js", "--include=*.py", "--include=*.go",
                   &query, &root])
            .output();
        if let Ok(out) = grep_result {
            String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.to_string())
                .take(10)
                .collect()
        } else {
            vec![]
        }
    };

    Ok(json!({ "files": files }))
}

// ── Directory tree for @codebase context ────────────────────────────────────
#[tauri::command]
async fn get_directory_tree(root: String, max_depth: Option<usize>) -> Result<String, String> {
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

    let mut out = format!("{}/\n", root_path.file_name().unwrap_or_default().to_string_lossy());
    walk(root_path, 1, depth, &mut out, 1);
    Ok(out)
}

// ── AI Code Tools (Cursor-like features) ──────────────────────────────────────

#[tauri::command]
async fn ai_explain_code(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    detail_level: String,
) -> Result<String, String> {
    let prompt = format!(
        "Explain what this {} code does in {} detail level:\n\n```\n{}\n```\n\nProvide a clear explanation covering:\n1. What the code does (plain English)\n2. Key logic flow\n3. Any important patterns or concepts used",
        file_path.split('.').last().unwrap_or("code"),
        detail_level,
        code
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a code explanation assistant. Explain code clearly in plain English.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Explain".to_string()),
        ollama_url: None,
        tools: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn ai_document_code(
    state: State<'_, EditorState>,
    code: String,
    _file_path: String,
    format: String,
    language: String,
) -> Result<String, String> {
    let prompt = format!(
        "Generate {} documentation for this {} code:\n\n```{}\n```\n\nInclude:\n- Function/class descriptions\n- Parameter explanations\n- Return value descriptions\n- Usage examples if helpful",
        format, language, code
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a documentation generator. Generate clean, professional code documentation.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Document".to_string()),
        ollama_url: None,
        tools: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn ai_generate_code(
    state: State<'_, EditorState>,
    prompt: String,
    language: String,
    framework: Option<String>,
    _file_path: Option<String>,
) -> Result<String, String> {
    let full_prompt = if let Some(fw) = framework {
        format!("Generate {} code using {} framework for: {}\n\nInclude proper imports, error handling, and best practices.", language, fw, prompt)
    } else {
        format!("Generate {} code for: {}\n\nInclude proper imports, error handling, and best practices.", language, prompt)
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a code generation assistant. Generate clean, functional, production-ready code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(full_prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.4),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Generate".to_string()),
        ollama_url: None,
        tools: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn ai_refactor_code(
    state: State<'_, EditorState>,
    _code: String,
    file_path: String,
    start_line: Option<usize>,
    end_line: Option<usize>,
    refactor_type: String,
    target_name: Option<String>,
) -> Result<String, String> {
    let range = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        (Some(s), None) => format!(" (starting from line {})", s),
        _ => String::new(),
    };
    
    let prompt = format!(
        "Refactor this {} code{} using {} refactoring approach.{}\n\nProvide improved code with better readability, performance, and best practices.",
        file_path.split('.').last().unwrap_or("code"),
        range,
        refactor_type,
        target_name.map(|n| format!(" Target name: {}", n)).unwrap_or_default()
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a code refactoring assistant. Improve code quality while preserving functionality.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Refactor".to_string()),
        ollama_url: None,
        tools: None,
    };

    state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn ai_debug_code(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    error_message: Option<String>,
    start_line: Option<usize>,
    end_line: Option<usize>,
) -> Result<Value, String> {
    let code_section = match (start_line, end_line) {
        (Some(s), Some(e)) => format!(" (lines {} to {})", s, e),
        _ => String::new(),
    };
    
    let prompt = if let Some(err) = error_message {
        format!(
            "Debug this {} code{} that has error: {}\n\nError: {}\n\nProvide:\n1. Diagnosis of the problem\n2. List of specific issues found\n3. Fixed code\n4. Suggestions for prevention",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            err,
            code
        )
    } else {
        format!(
            "Debug this {} code{} for bugs, errors, and issues.\n\nProvide:\n1. Diagnosis of problems found\n2. List of specific issues (logic errors, race conditions, security issues, etc.)\n3. Fixed code\n4. Suggestions for improvement\n\nCode:\n```\n{}\n```",
            file_path.split('.').last().unwrap_or("code"),
            code_section,
            code
        )
    };
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a code debugging assistant. Find and fix bugs, errors, and issues in code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.2),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("Debug".to_string()),
        ollama_url: None,
        tools: None,
    };

    let response = state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    // Parse the response to extract diagnosis, issues, and fixed code
    // For now, return a structured response with the full response
    Ok(json!({
        "diagnosis": "Analysis complete",
        "issues": ["See fixed code below"],
        "fixed_code": response,
        "suggestions": ["Review the fixed code and apply any needed adjustments"]
    }))
}

#[tauri::command]
async fn ai_multi_cursor_edit(
    state: State<'_, EditorState>,
    _code: String,
    file_path: String,
    pattern: String,
    replacement: String,
    match_scope: String,
    apply: bool,
) -> Result<Value, String> {
    let prompt = format!(
        "Find all occurrences of '{}' in this {} code and {} them.\n\nPattern: {}\nReplacement: {}\nMatch scope: {}\n\nReturn the modified code with all changes applied. If apply=false, show preview of changes.",
        pattern,
        file_path.split('.').last().unwrap_or("code"),
        if apply { "replace" } else { "preview replacement for" },
        pattern,
        replacement,
        match_scope
    );
    
    let request = AiRequest {
        provider: "google".to_string(),
        model: state.current_model.lock().await.clone(),
        messages: vec![
            ai_engine::ChatMessage {
                role: "system".to_string(),
                content: Some(ai_engine::MessageContent::Text(
                    "You are a multi-cursor editing assistant. Find patterns and edit them consistently across code.".to_string()
                )),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
            ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(ai_engine::MessageContent::Text(prompt)),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            },
        ],
        temperature: Some(0.3),
        autonomous: false,
        cyber_mode: None,
        root_access: Some(false),
        mode: Some("MultiEdit".to_string()),
        ollama_url: None,
        tools: None,
    };

    let modified = state.ai_engine.clone().autonomous_loop(request, None).await.map_err(|e| e.to_string())?;
    
    Ok(json!({
        "matches": [format!("Found occurrences of: {}", pattern)],
        "modified_code": modified,
        "preview_only": !apply
    }))
}

#[tauri::command]
async fn ai_pr_review(
    _state: State<'_, EditorState>,
    _pr_url: Option<String>,
    diff_content: Option<String>,
    focus_areas: Option<Vec<String>>,
) -> Result<Value, String> {
    // For now, require diff_content since we can't fetch from GitHub directly
    let diff = diff_content.ok_or("diff_content required for PR review")?;

    let focus = focus_areas.unwrap_or_else(|| vec!["security".to_string(), "performance".to_string(), "style".to_string()]);

    let _prompt = format!(
        "Perform an AI code review of these changes. Focus on: {}.\n\nReview the diff and provide:\n1. Summary of changes\n2. Potential issues found\n3. Suggestions for improvement\n4. Security considerations\n5. Performance impact\n\nDiff:\n```\n{}\n```",
        focus.join(", "),
        diff
    );
    
    // Return a placeholder - in production this would call the AI engine
    Ok(json!({
        "summary": "Review requires AI engine integration",
        "issues": ["Awaiting full AI integration"],
        "suggestions": ["Full PR review coming soon"],
        "security": "Manual review recommended",
        "performance": "Manual review recommended"
    }))
}

#[tauri::command]
async fn ai_get_context(
    state: State<'_, EditorState>,
    query: String,
    max_files: Option<usize>,
    _include_types: Option<Vec<String>>,
) -> Result<Value, String> {
    let max = max_files.unwrap_or(5);
    
    // Use grep_files to find relevant files based on the query
    let results = grep_files(state.clone(), query.clone(), None, None)
        .await
        .unwrap_or_default();
    
    // Extract unique files from grep results
    let mut unique_files: Vec<Value> = Vec::new();
    let mut seen_paths: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    for r in results.into_iter().take(max * 3) {
        let path = r.path.clone();
        if !seen_paths.contains(&path) && unique_files.len() < max {
            seen_paths.insert(path.clone());
            unique_files.push(json!({
                "path": path,
                "line": r.line,
                "snippet": r.content.chars().take(100).collect::<String>(),
            }));
        }
    }
    
    Ok(json!({
        "query": query,
        "files": unique_files,
        "count": unique_files.len()
    }))
}

// ── Vector Indexer Commands (Cursor-like Codebase Search) ─────────────────

#[tauri::command]
async fn vector_index_codebase(state: State<'_, EditorState>) -> Result<String, String> {
    state.vector_indexer
        .index_codebase()
        .await
        .map(|_| "Codebase indexing started".to_string())
        .map_err(|e| format!("Failed to start indexing: {}", e))
}

#[tauri::command]
async fn vector_search_codebase(
    state: State<'_, EditorState>,
    query: String,
    limit: Option<usize>,
) -> Result<Value, String> {
    let limit = limit.unwrap_or(10);
    let results = state.vector_indexer
        .search_codebase(&query, limit)
        .await
        .map_err(|e| format!("Search failed: {}", e))?;

    Ok(json!({
        "query": query,
        "results": results,
        "count": results.len()
    }))
}

#[tauri::command]
async fn vector_find_symbol(
    state: State<'_, EditorState>,
    symbol_name: String,
) -> Result<Value, String> {
    let results = state.vector_indexer
        .find_symbol(&symbol_name)
        .await
        .map_err(|e| format!("Symbol search failed: {}", e))?;

    Ok(json!({
        "symbol": symbol_name,
        "results": results,
        "count": results.len()
    }))
}

#[tauri::command]
async fn vector_get_index_stats(state: State<'_, EditorState>) -> Result<Value, String> {
    let stats = state.vector_indexer
        .get_index_stats()
        .await
        .map_err(|e| format!("Failed to get stats: {}", e))?;

    Ok(json!(stats))
}

#[tauri::command]
async fn vector_get_file_chunks(
    state: State<'_, EditorState>,
    file_path: String,
) -> Result<Value, String> {
    let chunks = state.vector_indexer
        .get_file_chunks(&file_path)
        .await
        .map_err(|e| format!("Failed to get chunks: {}", e))?;

    Ok(json!({
        "file": file_path,
        "chunks": chunks,
        "count": chunks.len()
    }))
}

// ── Git Checkpoint Commands (Auto-Snapshot & Rollback) ────────────────────

#[tauri::command]
async fn git_create_checkpoint(
    state: State<'_, EditorState>,
    description: String,
    is_ai: Option<bool>,
) -> Result<Value, String> {
    let is_ai = is_ai.unwrap_or(false);
    let checkpoint = state.git_checkpoints
        .create_checkpoint(&description, is_ai)
        .map_err(|e| format!("Failed to create checkpoint: {}", e))?;

    // If this was an AI checkpoint, automatically trigger auto-checkpoint before AI edits
    if is_ai {
        println!("[CHECKPOINT] ✅ AI checkpoint created: {}", checkpoint.id);
    }

    Ok(json!(checkpoint))
}

#[tauri::command]
async fn git_list_checkpoints(
    state: State<'_, EditorState>,
    limit: Option<usize>,
) -> Result<Value, String> {
    let checkpoints = state.git_checkpoints
        .list_checkpoints(limit)
        .map_err(|e| format!("Failed to list checkpoints: {}", e))?;

    Ok(json!(checkpoints))
}

#[tauri::command]
async fn git_rollback_checkpoint(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<String, String> {
    state.git_checkpoints
        .rollback_to_checkpoint(&checkpoint_id)
        .map_err(|e| format!("Failed to rollback: {}", e))
}

#[tauri::command]
async fn git_get_checkpoint_diff(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<Value, String> {
    let diff = state.git_checkpoints
        .get_checkpoint_diff(&checkpoint_id)
        .map_err(|e| format!("Failed to get diff: {}", e))?;

    Ok(json!(diff))
}

#[tauri::command]
async fn git_delete_checkpoint(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<(), String> {
    state.git_checkpoints
        .delete_checkpoint(&checkpoint_id)
        .map_err(|e| format!("Failed to delete checkpoint: {}", e))
}

#[tauri::command]
async fn git_auto_checkpoint(
    state: State<'_, EditorState>,
    description: String,
) -> Result<Value, String> {
    let result = state.git_checkpoints
        .auto_checkpoint_before_ai_edit(&description)
        .map_err(|e| format!("Failed to auto-checkpoint: {}", e))?;

    Ok(json!({
        "checkpoint": result,
        "created": result.is_some()
    }))
}

