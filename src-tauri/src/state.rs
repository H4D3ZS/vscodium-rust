use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::io::Write;
use std::fs;
use ropey::Rope;

use portable_pty::{Child, MasterPty};
use crate::domain::{Settings};
use crate::lsp;
use crate::context_key::{ContextKeyRegistry};
#[cfg(feature = "tauri")]
use crate::extension_host::ExtensionHostManager;
use crate::keybindings::KeybindingRegistry;
#[cfg(feature = "tauri")]
use crate::debug_adapter::DebugManager;
#[cfg(feature = "tauri")]
use crate::activation::ActivationManager;
use crate::performance::PerformanceMonitor;
use crate::ai_engine::Sentient;
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
use crate::semantic_firewall::SemanticFirewall;
use crate::ghost_runtime;
use crate::kairos;
use crate::mcp_server;
use crate::vfs_bridge;
use crate::shadow_workspace;
use crate::memory_layer;
use crate::hades_harness;
use crate::code_bloat_enforcer::CodeBloatEnforcer;
use crate::context_indexer::ContextIndexer;
use crate::structural_blueprints::StructuralBlueprints;
use crate::vector_indexer::VectorIndexer;
use crate::git_checkpoints::GitCheckpoint;
#[cfg(feature = "tauri")]
use crate::iphone_emulator::IPhoneEmulatorManager;
#[cfg(feature = "tauri")]
use crate::hades_vision;
use crate::apex_orchestrator::ApexOrchestrator;
#[cfg(feature = "tauri")]
use tauri::Manager;

/// Windows blocks writes under `Program Files`; the MSI default launch cwd is
/// often the install dir — use AppData instead.
fn is_protected_install_path(path: &PathBuf) -> bool {
    let s = path.to_string_lossy().to_lowercase();
    #[cfg(windows)]
    {
        s.contains("\\program files") || s.contains("\\program files (x86)")
    }
    #[cfg(not(windows))]
    {
        let _ = s;
        false
    }
}

fn paths_same(a: &PathBuf, b: &PathBuf) -> bool {
    match (a.canonicalize(), b.canonicalize()) {
        (Ok(a), Ok(b)) => a == b,
        _ => a == b,
    }
}

fn is_dir_writable(path: &PathBuf) -> bool {
    if is_protected_install_path(path) {
        return false;
    }
    if std::fs::create_dir_all(path).is_err() {
        return false;
    }
    let probe = path.join(".write_probe");
    match std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&probe)
    {
        Ok(_) => {
            let _ = std::fs::remove_file(probe);
            true
        }
        Err(_) => false,
    }
}

/// Startup workspace root: never use a read-only install directory.
fn resolve_startup_root(config_dir: &PathBuf) -> PathBuf {
    let mut root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

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

    let exe_install = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()));

    if let Some(ref install) = exe_install {
        if paths_same(&root, install) {
            let fb = config_dir.join("default_workspace");
            let _ = fs::create_dir_all(&fb);
            return fb;
        }
    }

    if !is_dir_writable(&root) {
        let fb = config_dir.join("default_workspace");
        let _ = fs::create_dir_all(&fb);
        return fb;
    }

    root
}

/// Default local model. Lemonade is the only local backend (real llama.cpp).
/// Measured on this hardware: ~15.9 tok/s, 8/8 tool calls, and proven over a
/// 12-hour agentic session. See the Lemonade notes in MEMORY.md.
pub const DEFAULT_LOCAL_MODEL: &str = "Qwen3.6-35B-A3B-Abliterated-Heretic-GGUF-Q4_K_M";

/// PTY terminal state. `pending` is the PRIMARY terminal transport — the
/// frontend polls it via `terminal_take_pending` (global `terminal-data`
/// events don't reliably reach the webview). std::sync::Mutex so the blocking
/// reader thread can append without dropping bytes.
pub struct TerminalState {
    pub masters: tokio::sync::Mutex<HashMap<String, Box<dyn MasterPty + Send>>>,
    pub writers: tokio::sync::Mutex<HashMap<String, Box<dyn Write + Send>>>,
    pub processes: tokio::sync::Mutex<HashMap<String, Box<dyn Child + Send>>>,
    pub buffers: tokio::sync::Mutex<HashMap<String, Vec<String>>>,
    pub pending: std::sync::Mutex<HashMap<String, String>>,
}

/// Core editor state: open buffers, active file/root, settings, LSP plumbing.
pub struct EditorCore {
    pub buffers: tokio::sync::Mutex<HashMap<String, Rope>>,
    pub active_path: tokio::sync::Mutex<Option<String>>,
    pub settings: tokio::sync::Mutex<Settings>,
    pub lsp_router: Arc<tokio::sync::Mutex<crate::lsp_router::LspRouter>>,
    pub lsp_diagnostics: lsp::DiagnosticsMap,
    pub active_root: tokio::sync::Mutex<Option<PathBuf>>,
}

/// AI state: the Sentient engine, tools, model selection, APEX, ANE, harnesses.
pub struct AiState {
    pub engine: Arc<Sentient>,
    pub tools: Arc<ai_tools::AiTools>,
    pub current_model: tokio::sync::Mutex<String>,
    pub advisor_model: tokio::sync::Mutex<Option<String>>,
    pub ane: Arc<crate::ane_inference::AneInferenceOptimizer>,
    pub apex: Arc<ApexOrchestrator>,
    #[cfg(feature = "tauri")]
    pub vision: Arc<hades_vision::HadesVision>,
    pub harness: Arc<hades_harness::HadesHarness>,
}

/// Mobile tooling: Android SDK/Gradle/logcat, iOS simulator manager.
#[cfg(feature = "tauri")]
pub struct MobileState {
    pub active_device: tokio::sync::Mutex<Option<String>>,
    pub android_sdk_path: tokio::sync::Mutex<Option<String>>,
    pub android: Arc<crate::architecture::application::android_service::AndroidService>,
    pub gradle: Arc<crate::architecture::application::gradle_service::GradleService>,
    pub logcat: Arc<crate::logcat_service::LogcatService>,
    pub iphone: Arc<IPhoneEmulatorManager>,
}

/// Extension-host state: sidecar host, keybindings, activation, context keys.
#[cfg(feature = "tauri")]
pub struct ExtensionState {
    pub host: Arc<tokio::sync::Mutex<ExtensionHostManager>>,
    pub keybindings: Arc<tokio::sync::Mutex<KeybindingRegistry>>,
    pub activation: Arc<tokio::sync::Mutex<ActivationManager>>,
    pub context_keys: Arc<ContextKeyRegistry>,
    pub debug: Arc<tokio::sync::Mutex<DebugManager>>,
}

/// Memory + indexing: .aim layers, stores, codebase indexers, distiller.
pub struct MemoryState {
    pub store: Arc<memory_store::MemoryStore>,
    pub optimizer: Arc<memory_optimizer::MemoryOptimizer>,
    pub layer: Arc<memory_layer::MemoryLayer>,
    pub context_indexer: Arc<ContextIndexer>,
    pub vector_indexer: Arc<VectorIndexer>,
    pub bloat_enforcer: Arc<tokio::sync::Mutex<CodeBloatEnforcer>>,
    pub blueprints: Arc<StructuralBlueprints>,
    pub distiller: Arc<KnowledgeDistiller>,
    pub attachments: Arc<AttachmentManager>,
}

/// Cross-cutting services: browser, MCP, VFS, specs, vcs helpers.
pub struct ServiceState {
    pub browser: Arc<browser::BrowserState>,
    pub mcp_registry: Arc<McpRegistry>,
    pub mcp_server: Arc<mcp_server::McpServer>,
    pub perf_monitor: Arc<PerformanceMonitor>,
    pub vfs: Arc<vfs_bridge::VfsBridge>,
    pub specs_db: Arc<specs_db::SpecDb>,
    pub workers: Arc<workers::WorkerManager>,
    pub test_runner: Arc<crate::test_runner_service::TestRunnerService>,
    pub git_checkpoints: Arc<GitCheckpoint>,
    pub patch_engine: Arc<tokio::sync::Mutex<patch_engine::PatchEngine>>,
    pub shadow_workspace: Arc<shadow_workspace::ShadowWorkspace>,
    pub firewall: Arc<tokio::sync::Mutex<SemanticFirewall>>,
    pub ghost_runtime: Arc<ghost_runtime::GhostRuntime>,
    pub kairos: Arc<kairos::KairosEngine>,
    /// Pending tool-permission approvals: tool_id → oneshot sender.
    /// Backend emits `tool_permission_request`, then awaits the sender.
    /// Frontend responds via `respond_tool_permission` command.
    pub tool_permissions: Arc<std::sync::Mutex<HashMap<String, tokio::sync::oneshot::Sender<bool>>>>,
}

pub struct EditorState {
    pub editor: EditorCore,
    pub terminal: TerminalState,
    pub ai: AiState,
    #[cfg(feature = "tauri")]
    pub mobile: MobileState,
    #[cfg(feature = "tauri")]
    pub ext: ExtensionState,
    pub memory: MemoryState,
    pub services: ServiceState,
    pub config_dir: PathBuf,
    #[cfg(feature = "tauri")]
    #[cfg(feature = "tauri")]
    /// Renderer-agnostic UI event sink. The Tauri shell sets a `TauriSink`; a
    /// native (gpui) shell sets its own (often a no-op — the engine also exposes
    /// pollable `activity_log`/`chat_stream_buf`/`pending_proposals` buffers).
    /// Engine subsystems emit through `EditorState::emit` via their
    /// `Weak<EditorState>` back-reference instead of holding a `tauri::AppHandle`.
    pub event_sink: std::sync::RwLock<Option<std::sync::Arc<dyn crate::EventSink>>>,
}

impl EditorState {
    pub async fn terminal_read_output(&self, id: String) -> Result<String, String> {
        let buffers = self.terminal.buffers.lock().await;
        let history = buffers
            .get(&id)
            .ok_or_else(|| "Terminal not found".to_string())?;
        let joined = history.join("");
        if joined.len() > crate::domain::safe_io::MAX_TERM_BUFFER {
            Ok(joined[joined.len() - crate::domain::safe_io::MAX_TERM_BUFFER..].to_string())
        } else {
            Ok(joined)
        }
    }

    #[cfg(feature = "tauri")]
    pub fn new(app: &tauri::AppHandle) -> Self {
        let boot_t0 = std::time::Instant::now(); // Milestone D metric: EditorState::new duration
        // DIAGNOSTIC: Capture panics before `panic = "abort"` kills the process.
        let crash_log_dir = app
            .path()
            .app_log_dir()
            .or_else(|_| app.path().app_data_dir())
            .unwrap_or_else(|_| PathBuf::from(".config"));
        let _ = fs::create_dir_all(&crash_log_dir);
        std::panic::set_hook(Box::new(move |info| {
            let msg = format!(
                "[CRASH] PANIC: {}\nLocation: {:?}\nTime: {:?}\n",
                info,
                info.location(),
                std::time::SystemTime::now()
            );
            eprintln!("{}", msg);
            let mut targets: Vec<PathBuf> = vec![
                PathBuf::from("crash.log"),
                PathBuf::from("vscodium_crash.log"),
                crash_log_dir.join("crash.log"),
                crash_log_dir.join("vscodium_crash.log"),
            ];
            if let Ok(exe) = std::env::current_exe() {
                if let Some(dir) = exe.parent() {
                    targets.push(dir.join("crash.log"));
                    targets.push(dir.join("vscodium_crash.log"));
                }
            }
            targets.dedup();
            for p in targets {
                let _ = std::fs::write(&p, &msg);
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(&p)
                    .and_then(|mut f| {
                        use std::io::Write;
                        writeln!(f, "{}", msg)
                    });
            }
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

        let root = resolve_startup_root(&config_dir);
        let browser_state = Arc::new(browser::BrowserState::new());
        // Tauri commands use State<Arc<BrowserState>> — must register or browser_open panics (IDE exit).
        app.manage(Arc::clone(&browser_state));
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
        
        crate::event_sink::spawn_detached(async move {
            airi_clone.init(app_airi).await;
        });

        let sentient_airi = sentient.clone();
        crate::event_sink::spawn_detached(async move {
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
        
        // NOTE: engine subsystems (sentient/ai_tools/memory_store/patch_engine)
        // get their EditorState back-reference via `wire_back_refs()` in lib.rs,
        // right after the state is wrapped in an Arc. No AppHandle plumbing here.

        // Initialize and start Omni-Context Indexer (Phase 44)
        let context_indexer = Arc::new(ContextIndexer::new(
            sentient.memory_store.clone(),
            root.clone(),
        ));
        let ci_for_spawn = context_indexer.clone();
        crate::event_sink::spawn_detached(async move {
            // Milestone D: let the UI paint and first interactions settle
            // before competing for CPU/RAM on 8GB machines.
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            ci_for_spawn.start_background_indexing().await;
        });

        app.manage(context_indexer.clone());

        // Initialize Vector Indexer (Cursor-like semantic search).
        // Memory optimization: do NOT index at startup. The context_indexer
        // already provides AIM-based codebase awareness. Vector indexing only
        // runs when the user explicitly invokes vector_search, which loads
        // the SQLite-backed embeddings on demand. Saves 50-100MB at startup.
        let vector_indexer = Arc::new(
            VectorIndexer::new(root.clone(), config_dir.clone())
                .unwrap_or_else(|e| {
                    eprintln!("[VectorIndexer] init failed ({e}); using config dir fallback");
                    VectorIndexer::new(config_dir.join("default_workspace"), config_dir.clone())
                        .expect("Failed to init vector indexer in config dir")
                }),
        );
        {
            let vi_for_tools = vector_indexer.clone();
            let tools = sentient.ai_tools.clone();
            crate::event_sink::spawn_detached(async move {
                tools.set_vector_indexer(vi_for_tools).await;
            });
        }

        // Initialize Code Bloat Enforcer for structural analysis
        let bloat_enforcer = Arc::new(tokio::sync::Mutex::new(CodeBloatEnforcer::new()));
        let blueprints = Arc::new(StructuralBlueprints::new(root.clone()));

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
        crate::event_sink::spawn_detached(async move {
            wm_clone.start_loop().await;
        });

        // KairosEngine uses interior mutability (RwLock) so no outer Mutex needed
        let kairos = Arc::new(kairos::KairosEngine::new(
            context_indexer.clone(),
            sentient.memory_store.clone(),
            Arc::new(tokio::sync::Mutex::new(Some(root.clone()))),
        ));
        // NOTE: kairos gets its EditorState back-ref via wire_back_refs() in lib.rs.
        // Potato mode: Kairos idle ticks can shell out (e.g. `cargo` on Rust repos) —
        // a luxury 4–8GB machines can't afford. Suggestions stay available on demand.
        if !crate::system_profile::is_lite() {
            let k_clone = kairos.clone();
            crate::event_sink::spawn_detached(async move {
                loop {
                    // 300s: was 60s, which spawned `cargo` every minute and held
                    // the process at "High" GPU/CPU power even when idle.
                    tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
                    k_clone.tick().await;
                }
            });
        } else {
            println!("[profile] lite: Kairos background tick disabled");
        }

        let vfs_bridge = Arc::new(vfs_bridge::VfsBridge::new(root.clone()));
        let mcp_server = Arc::new(mcp_server::McpServer::new(sentient.ai_tools.clone()));
        // The built-in MCP listener (:1537) only matters when an *external* MCP
        // client wants to reach this IDE's tools — uncommon. Opt-in: it auto-
        // starts only if `<config>/mcp_builtin.enabled` exists (created by the
        // MCP Store's "Expose IDE tools" toggle). Skipped in lite mode regardless.
        let mcp_builtin_enabled = config_dir.join("mcp_builtin.enabled").exists();
        if !crate::system_profile::is_lite() && mcp_builtin_enabled {
            let mcp_server_clone = mcp_server.clone();
            crate::event_sink::spawn_detached(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(20)).await;
                mcp_server_clone.start(1537).await;
            });
        } else {
            println!("[profile] built-in MCP listener (:1537) not auto-started (opt-in via mcp_builtin.enabled)");
        }


        // Shared diagnostics map — owned by EditorState, borrowed by LspClient
        let shared_lsp_diags: lsp::DiagnosticsMap =
            Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
        let lsp_router_inst = crate::lsp_router::LspRouter::new(shared_lsp_diags.clone());

        // Vision stack — init on first command, not at boot (saves ~10–20MB).
        // Potato mode: never auto-init; commands init lazily when invoked.
        if !crate::system_profile::is_lite() {
            crate::event_sink::spawn_detached(async {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                hades_vision::init_hades_vision(
                    1,
                    "http://localhost:13305",
                    "moondream",
                    DEFAULT_LOCAL_MODEL,
                    false,
                );
            });
        }

        tracing::info!(elapsed_ms = boot_t0.elapsed().as_millis() as u64, "EditorState::new complete");
        Self {
            editor: EditorCore {
                buffers: tokio::sync::Mutex::new(HashMap::new()),
                active_path: tokio::sync::Mutex::new(None),
                settings: tokio::sync::Mutex::new(Settings {
                    theme: "vs-dark".to_string(),
                    font_size: 14,
                }),
                lsp_router: Arc::new(tokio::sync::Mutex::new(lsp_router_inst)),
                lsp_diagnostics: shared_lsp_diags.clone(),
                active_root: tokio::sync::Mutex::new(Some(root.clone())),
            },
            terminal: TerminalState {
                masters: tokio::sync::Mutex::new(HashMap::new()),
                writers: tokio::sync::Mutex::new(HashMap::new()),
                processes: tokio::sync::Mutex::new(HashMap::new()),
                buffers: tokio::sync::Mutex::new(HashMap::new()),
                pending: std::sync::Mutex::new(HashMap::new()),
            },
            ai: AiState {
                engine: sentient.clone(),
                tools: sentient.ai_tools.clone(),
                current_model: tokio::sync::Mutex::new(DEFAULT_LOCAL_MODEL.to_string()),
                advisor_model: tokio::sync::Mutex::new(None),
                ane: {
                    let ane = Arc::new(crate::ane_inference::AneInferenceOptimizer::new());
                    // Register globally so sync indexing code (ann_index) can offload
                    // similarity scoring to the ANE without plumbing Tauri state.
                    crate::ane_inference::set_global(ane.clone());
                    ane
                },
                apex: {
                    let apex_inst = Arc::new(ApexOrchestrator::new("http://localhost:1536", Some(root), Some(config_dir.clone())));
                    let apex_for_tools = apex_inst.clone();
                    let tools = sentient.ai_tools.clone();
                    crate::event_sink::spawn_detached(async move {
                        tools.set_apex(apex_for_tools).await;
                    });
                    apex_inst
                },
                vision: Arc::new(hades_vision::HadesVision::new(
                    "http://localhost:1536",
                    "qwen2.5vl",
                    DEFAULT_LOCAL_MODEL,
                    false
                )),
                harness: hades_harness,
            },
            mobile: MobileState {
                active_device: tokio::sync::Mutex::new(None),
                android_sdk_path: tokio::sync::Mutex::new(None),
                android: Arc::new(crate::architecture::application::android_service::AndroidService::new()),
                gradle: Arc::new(crate::architecture::application::gradle_service::GradleService::new()),
                logcat: Arc::new(crate::logcat_service::LogcatService::new()),
                iphone: Arc::new(IPhoneEmulatorManager::new()),
            },
            ext: ExtensionState {
                host: Arc::new(tokio::sync::Mutex::new(ExtensionHostManager::new(ext_dirs))),
                keybindings: Arc::new(tokio::sync::Mutex::new(KeybindingRegistry::new())),
                activation: Arc::new(tokio::sync::Mutex::new(ActivationManager::new())),
                context_keys: Arc::new(ContextKeyRegistry::new()),
                debug: Arc::new(tokio::sync::Mutex::new(DebugManager::new())),
            },
            memory: MemoryState {
                store: sentient.memory_store.clone(),
                optimizer: memory_optimizer,
                layer: memory_layer,
                context_indexer,
                vector_indexer,
                bloat_enforcer,
                blueprints,
                distiller: knowledge_distiller,
                attachments: attachment_manager,
            },
            services: ServiceState {
                browser: browser_state,
                mcp_registry: Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json"))),
                mcp_server,
                perf_monitor: Arc::new(PerformanceMonitor::new()),
                vfs: vfs_bridge,
                specs_db,
                workers: worker_manager,
                test_runner: Arc::new(crate::test_runner_service::TestRunnerService::new()),
                git_checkpoints,
                patch_engine,
                shadow_workspace,
                firewall: Arc::new(tokio::sync::Mutex::new(
                    SemanticFirewall::new(shared_lsp_diags.clone()),
                )),
                ghost_runtime,
                kairos,
                // Share the same Arc as Sentient so respond_tool_permission resolves
                // the correct oneshot sender that the autonomous_loop is waiting on.
                tool_permissions: sentient.permission_senders.clone(),
            },
            config_dir: config_dir.clone(),
            // Tauri shell: emit engine UI events to the WebView.
            event_sink: std::sync::RwLock::new(Some(std::sync::Arc::new(
                crate::event_sink::TauriSink(app.clone()),
            ))),
        }
    }

    /// Headless constructor for the gpui shell — no Tauri, no WebView, no mobile
    /// or extension state. Builds the same engine core (Sentient, tools, memory,
    /// harness, indexing) so the real agent runs identically.
    #[cfg(not(feature = "tauri"))]
    pub fn new_headless(config_dir: PathBuf) -> Self {
        let root = resolve_startup_root(&config_dir);
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
            "".to_string(),
            root.clone(),
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

        let context_indexer = Arc::new(ContextIndexer::new(
            sentient.memory_store.clone(),
            root.clone(),
        ));
        let ci_for_spawn = context_indexer.clone();
        crate::event_sink::spawn_detached(async move {
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
            ci_for_spawn.start_background_indexing().await;
        });

        let vector_indexer = Arc::new(
            VectorIndexer::new(root.clone(), config_dir.clone())
                .unwrap_or_else(|e| {
                    eprintln!("[VectorIndexer] init failed ({e}); using config dir fallback");
                    VectorIndexer::new(config_dir.join("default_workspace"), config_dir.clone())
                        .expect("Failed to init vector indexer in config dir")
                }),
        );
        {
            let vi_for_tools = vector_indexer.clone();
            let tools = sentient.ai_tools.clone();
            crate::event_sink::spawn_detached(async move {
                tools.set_vector_indexer(vi_for_tools).await;
            });
        }

        let bloat_enforcer = Arc::new(tokio::sync::Mutex::new(CodeBloatEnforcer::new()));
        let blueprints = Arc::new(StructuralBlueprints::new(root.clone()));
        let git_checkpoints = Arc::new(GitCheckpoint::new(root.clone()));

        let specs_db = Arc::new(specs_db::SpecDb::new(config_dir.join("specs.db")).expect("Failed to init specs DB"));
        let worker_manager = Arc::new(workers::WorkerManager::new(specs_db.clone(), sentient.clone(), root.clone()));
        let wm_clone = worker_manager.clone();
        crate::event_sink::spawn_detached(async move {
            wm_clone.start_loop().await;
        });

        let kairos = Arc::new(kairos::KairosEngine::new(
            context_indexer.clone(),
            sentient.memory_store.clone(),
            Arc::new(tokio::sync::Mutex::new(Some(root.clone()))),
        ));

        let vfs_bridge = Arc::new(vfs_bridge::VfsBridge::new(root.clone()));
        let mcp_server = Arc::new(mcp_server::McpServer::new(sentient.ai_tools.clone()));

        let shared_lsp_diags: lsp::DiagnosticsMap =
            Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new()));
        let lsp_router_inst = crate::lsp_router::LspRouter::new(shared_lsp_diags.clone());

        Self {
            editor: EditorCore {
                buffers: tokio::sync::Mutex::new(HashMap::new()),
                active_path: tokio::sync::Mutex::new(None),
                settings: tokio::sync::Mutex::new(Settings {
                    theme: "vs-dark".to_string(),
                    font_size: 14,
                }),
                lsp_router: Arc::new(tokio::sync::Mutex::new(lsp_router_inst)),
                lsp_diagnostics: shared_lsp_diags.clone(),
                active_root: tokio::sync::Mutex::new(Some(root.clone())),
            },
            terminal: TerminalState {
                masters: tokio::sync::Mutex::new(HashMap::new()),
                writers: tokio::sync::Mutex::new(HashMap::new()),
                processes: tokio::sync::Mutex::new(HashMap::new()),
                buffers: tokio::sync::Mutex::new(HashMap::new()),
                pending: std::sync::Mutex::new(HashMap::new()),
            },
            ai: AiState {
                engine: sentient.clone(),
                tools: sentient.ai_tools.clone(),
                current_model: tokio::sync::Mutex::new(DEFAULT_LOCAL_MODEL.to_string()),
                advisor_model: tokio::sync::Mutex::new(None),
                ane: {
                    let ane = Arc::new(crate::ane_inference::AneInferenceOptimizer::new());
                    crate::ane_inference::set_global(ane.clone());
                    ane
                },
                apex: {
                    let apex_inst = Arc::new(ApexOrchestrator::new("http://localhost:1536", Some(root), Some(config_dir.clone())));
                    let apex_for_tools = apex_inst.clone();
                    let tools = sentient.ai_tools.clone();
                    crate::event_sink::spawn_detached(async move {
                        tools.set_apex(apex_for_tools).await;
                    });
                    apex_inst
                },
                harness: hades_harness,
            },
            memory: MemoryState {
                store: sentient.memory_store.clone(),
                optimizer: memory_optimizer,
                layer: memory_layer,
                context_indexer,
                vector_indexer,
                bloat_enforcer,
                blueprints,
                distiller: knowledge_distiller,
                attachments: attachment_manager,
            },
            services: ServiceState {
                browser: browser_state,
                mcp_registry: Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json"))),
                mcp_server,
                perf_monitor: Arc::new(PerformanceMonitor::new()),
                vfs: vfs_bridge,
                specs_db,
                workers: worker_manager,
                test_runner: Arc::new(crate::test_runner_service::TestRunnerService::new()),
                git_checkpoints,
                patch_engine,
                shadow_workspace,
                firewall: Arc::new(tokio::sync::Mutex::new(
                    SemanticFirewall::new(shared_lsp_diags.clone()),
                )),
                ghost_runtime,
                kairos,
                tool_permissions: sentient.permission_senders.clone(),
            },
            config_dir: config_dir.clone(),
            event_sink: std::sync::RwLock::new(None),
        }
    }

    /// Emit a UI event through the configured sink (Tauri WebView or native).
    /// Fire-and-forget; a missing sink is a no-op.
    pub fn emit(&self, event: &str, payload: serde_json::Value) {
        if let Ok(guard) = self.event_sink.read() {
            if let Some(sink) = guard.as_ref() {
                sink.emit(event, payload);
            }
        }
    }

    /// Distribute a `Weak<EditorState>` back-reference into the engine subsystems
    /// so they can reach state + emit without holding a `tauri::AppHandle`.
    /// Call once, right after wrapping the state in an `Arc`, before `manage`.
    pub fn wire_back_refs(self: &std::sync::Arc<Self>) {
        let weak = std::sync::Arc::downgrade(self);
        self.ai.engine.set_editor_state(weak.clone());
        self.ai.engine.ai_tools.set_editor_state(weak.clone());
        self.ai.engine.memory_store.set_editor_state(weak.clone());
        if let Ok(pe) = self.services.patch_engine.try_lock() {
            pe.set_editor_state(weak.clone());
        }
        self.services.kairos.set_editor_state(weak);
    }
}
