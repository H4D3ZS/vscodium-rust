use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::io::Write;
use std::fs;
use ropey::Rope;

use portable_pty::{Child, MasterPty};
use crate::domain::{Settings};
use crate::lsp::{self, LspClient};
use crate::context_key::{ContextKeyRegistry};
use crate::extension_host::ExtensionHostManager;
use crate::keybindings::KeybindingRegistry;
use crate::debug_adapter::DebugManager;
use crate::activation::ActivationManager;
use crate::performance::PerformanceMonitor;
use crate::ai_engine::Sentient;
use crate::ai_auth;
use crate::browser;
use crate::mcp_registry::McpRegistry;
use crate::ai_tools;
use crate::memory_store;
use crate::memory_optimizer;
use crate::specs_db;
use crate::workers;
use crate::attachment_manager::{AttachmentManager};
use crate::knowledge_distiller::KnowledgeDistiller;
use crate::patch_engine;
use crate::ghost_runtime;
use crate::kairos;
use crate::mcp_server;
use crate::vfs_bridge;
use crate::shadow_workspace;
use crate::memory_layer;
use crate::hades_harness;
use crate::context_indexer::ContextIndexer;
use crate::vector_indexer::VectorIndexer;
use crate::git_checkpoints::GitCheckpoint;
use crate::iphone_emulator::IPhoneEmulatorManager;
use crate::hades_vision;
use crate::apex_orchestrator::ApexOrchestrator;
use tauri::Manager;

pub struct EditorState {
    pub buffers: tokio::sync::Mutex<HashMap<String, Rope>>,
    pub active_path: tokio::sync::Mutex<Option<String>>,
    pub settings: tokio::sync::Mutex<Settings>,
    pub terminal_masters: tokio::sync::Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    pub terminal_writers: tokio::sync::Mutex<HashMap<String, Box<dyn Write + Send>>>,
    pub terminal_processes: tokio::sync::Mutex<HashMap<String, Box<dyn Child + Send>>>,
    pub lsp_client: Arc<tokio::sync::Mutex<LspClient>>,
    pub lsp_diagnostics: lsp::DiagnosticsMap,
    pub context_keys: Arc<ContextKeyRegistry>,
    pub ext_host: Arc<tokio::sync::Mutex<ExtensionHostManager>>,
    pub keybindings: Arc<tokio::sync::Mutex<KeybindingRegistry>>,
    pub debug_manager: Arc<tokio::sync::Mutex<DebugManager>>,
    pub activation_manager: Arc<tokio::sync::Mutex<ActivationManager>>,
    pub perf_monitor: Arc<PerformanceMonitor>,
    pub ai_engine: Arc<Sentient>,
    pub ollama_url: tokio::sync::Mutex<String>,
    pub config_dir: PathBuf,
    pub active_root: tokio::sync::Mutex<Option<PathBuf>>,
    pub current_model: tokio::sync::Mutex<String>,
    pub active_device: tokio::sync::Mutex<Option<String>>,
    pub android_sdk_path: tokio::sync::Mutex<Option<String>>,
    pub auth_state: Arc<ai_auth::AuthState>,
    pub browser_state: Arc<browser::BrowserState>,
    pub mcp_registry: Arc<McpRegistry>,
    pub terminal_buffers: tokio::sync::Mutex<HashMap<String, Vec<String>>>,
    /// Unread PTY output, drained by the frontend via `terminal_take_pending`.
    /// This is the PRIMARY terminal transport — the global `terminal-data`
    /// event stream does not reliably reach the webview, so the UI polls this
    /// instead. std::sync::Mutex so the blocking reader thread can append
    /// without dropping bytes (best-effort try_lock would lose output).
    pub terminal_pending: std::sync::Mutex<HashMap<String, String>>,
    pub ai_tools: Arc<ai_tools::AiTools>,
    pub memory_store: Arc<memory_store::MemoryStore>,
    pub memory_optimizer: Arc<memory_optimizer::MemoryOptimizer>,
    pub advisor_model: tokio::sync::Mutex<Option<String>>,
    pub specs_db: Arc<specs_db::SpecDb>,
    pub worker_manager: Arc<workers::WorkerManager>,
    pub attachment_manager: Arc<AttachmentManager>,
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
    pub iphone_manager: Arc<IPhoneEmulatorManager>,
    pub hades_vision: Arc<hades_vision::HadesVision>,
    pub apex: Arc<ApexOrchestrator>,
    /// Pending tool-permission approvals: tool_id → oneshot sender.
    /// Backend emits `tool_permission_request`, then awaits the sender.
    /// Frontend responds via `respond_tool_permission` command.
    pub tool_permission_senders: Arc<std::sync::Mutex<HashMap<String, tokio::sync::oneshot::Sender<bool>>>>,
}

impl EditorState {
    pub async fn terminal_read_output(&self, id: String) -> Result<String, String> {
        let buffers = self.terminal_buffers.lock().await;
        let history = buffers
            .get(&id)
            .ok_or_else(|| "Terminal not found".to_string())?;
        Ok(history.join(""))
    }

    pub fn new(app: &tauri::AppHandle) -> Self {
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
        let perf_monitor = Arc::new(PerformanceMonitor::new());
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

        let sentient_airi = sentient.clone();
        tauri::async_runtime::spawn(async move {
            let mut lock = sentient_airi.airi.lock().await;
            *lock = Some(airi_bridge);
            println!("[DEBUG] Sentient initialized");
        });
        
        let memory_layer = {
            let mut ml = memory_layer::MemoryLayer::new(root.clone());
            ml.set_memory_store(sentient.memory_store.clone());
            Arc::new(ml)
        };
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

        // Initialize Vector Indexer (Cursor-like semantic search).
        // Memory optimization: do NOT index at startup. The context_indexer
        // already provides AIM-based codebase awareness. Vector indexing only
        // runs when the user explicitly invokes vector_search, which loads
        // the SQLite-backed embeddings on demand. Saves 50-100MB at startup.
        let vector_indexer = Arc::new(VectorIndexer::new(root.clone()).expect("Failed to init vector indexer"));

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

        // KairosEngine uses interior mutability (RwLock) so no outer Mutex needed
        let kairos = Arc::new(kairos::KairosEngine::new(
            context_indexer.clone(),
            sentient.memory_store.clone(),
            Arc::new(tokio::sync::Mutex::new(Some(root.clone()))),
        ));
        // Wire up app_handle so Kairos can emit frontend events (kairos://suggestion)
        kairos.set_app_handle(app.clone());
        let k_clone = kairos.clone();
        tauri::async_runtime::spawn(async move {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                k_clone.tick().await;
            }
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

        // Initialize global static vision for background tasks
        hades_vision::init_hades_vision(
            1,
            "http://localhost:11434",
            "moondream",
            "gpt-4o",
            false
        );

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
            ollama_url: tokio::sync::Mutex::new("http://localhost:11434".to_string()),
            config_dir: config_dir.clone(),
            active_root: tokio::sync::Mutex::new(Some(root.clone())),
            current_model: tokio::sync::Mutex::new("gpt-4o".to_string()),
            active_device: tokio::sync::Mutex::new(None),
            android_sdk_path: tokio::sync::Mutex::new(None),
            auth_state,
            browser_state,
            mcp_registry: Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json"))),
            terminal_buffers: tokio::sync::Mutex::new(HashMap::new()),
            terminal_pending: std::sync::Mutex::new(HashMap::new()),
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
            iphone_manager: Arc::new(IPhoneEmulatorManager::new()),
            hades_vision: Arc::new(hades_vision::HadesVision::new(
                "http://localhost:1536",
                "qwen2.5vl",
                "gpt-4o",
                false
            )),
            apex: {
                let apex_inst = Arc::new(ApexOrchestrator::new("http://localhost:1536", Some(root), Some(config_dir.clone())));
                let apex_for_tools = apex_inst.clone();
                let tools = sentient.ai_tools.clone();
                tauri::async_runtime::spawn(async move {
                    tools.set_apex(apex_for_tools).await;
                });
                apex_inst
            },
            // Share the same Arc as Sentient so respond_tool_permission resolves
            // the correct oneshot sender that the autonomous_loop is waiting on.
            tool_permission_senders: sentient.permission_senders.clone(),
        }
    }
}
