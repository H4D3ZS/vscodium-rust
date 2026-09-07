//! Sentient: the AI engine struct, lifecycle, events, and reasoning entry.

use anyhow::Result;

use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::Semaphore;

use crate::ai_tools::AiTools;
use crate::mcp_registry::{McpRegistry, McpServerConfig};
use crate::memory_store::MemoryStore;
use crate::task_planner::TaskPlanner;
use crate::tool_invoker::ToolInvoker;
use crate::rules_engine::RulesEngine;
use crate::workflow_engine::WorkflowEngine;

/// Tools always exposed to local local models (which have small context
/// windows and degrade with large tool lists). This is the single source of
/// truth — the autonomous loop's tool filter references it.
///
/// IMPORTANT: the offensive-security tools at the end are a CORE IDE strength.
/// Removing them would gut the red-team / pentest / bug-bounty playbooks. A
/// regression test (`tests::local_essentials_keep_offensive_tools`) enforces
/// their presence.
pub(crate) const LOCAL_ESSENTIAL_TOOLS: &[&str] = &[
    // Frontend/Cursor-style schemas passed in from src/tool_registry.ts.
    "bash", "file_read", "file_write", "file_edit", "glob",
    "list_directory", "web_fetch", "web_search", "git_status", "git_diff",
    "git_add", "git_commit", "git_log",
    "terminal_read_output", "terminal_send_data", "replace_file_content",
    "multi_replace_file_content", "create_directory",
    "todo_write", "task_create", "task_update",
    "grep", "get_system_health", "aim_query_spans", "aim_pack_context",
    "ag_mark_task_done", "ag_phase_wrap", "ag_get_next_task", "ag_list_tasks",
    // Native backend tool names.
    "view_file", "list_files", "write_to_file", "str_replace",
    "search_replace_edit", "apply_shadow_patch", "fast_apply", "patch_file_content",
    "find_by_name", "get_directory_structure",
    "search_codebase", "find_symbols", "run_command",
    "verify_implementation",
    "dev_cargo_diagnostics", "get_lsp_diagnostics", "save_knowledge_brief",
    "secrets_scan", "project_rules",
    // AIM VFS zero-grep tools — always available for local models
    "aim_pack_context", "aim_query_spans",
    // Offensive security tools — CORE IDE STRENGTH. Always available
    // even on small local models. Stripping these would gut the
    // red-team/pentest/bug-bounty playbooks documented in the
    // system prompt's CYBERSECURITY OPERATIONS section.
    "weaponize_env", "deep_security_audit", "sec_distro_inventory", "apex_red_team_scan", "apex_quick_check",
    "apex_threat_anticipate", "apex_simulate_attack", "apex_pentest_report",
    "binary_mach_o_scanner", "file_entropy_analysis", "network_port_scanner",
    "extract_strings", "hex_dump", "apex_scan_url",
    // Pentest generators + recon — must stay visible to local models
    "reverse_shell_generate", "security_listener_generate", "payload_encode",
    "shellcode_recipe_generate", "csp_bypass_analyze", "exploit_lookup", "network_scan",
    "security_scan", "audit_dependencies", "disassemble", "get_binary_info",
    // Common model alias names (resolved via tool_aliases.rs, kept in schema filter)
    "run_terminal_cmd", "run_command", "nmap", "searchsploit", "vuln_hunt",
    // FastContext repository explorer — dedicated exploration subagent
    "explore_repository",
    // Live web pentest / bug-bounty against a target URL. Without these, a local
    // security model (e.g. sec-eng-neuraldevil) gets NO web tooling — the first
    // essential-tools retain stripped them before the domain filter could keep
    // them, so "find security bugs in <url>" had nothing to drive the browser.
    "web_security_audit", "browser_open", "browser_navigate",
    "browser_screenshot", "browser_read_dom",
    // 3-stage AI vuln-hunting pipeline (HackerOne #1-KR methodology).
    "ai_vuln_hunt",
];

use super::types::*;

pub struct Sentient {
    pub(crate) client: Client,
    pub(crate) api_key: String,
    pub(crate) mcp_registry: Arc<McpRegistry>,
    pub ai_tools: Arc<AiTools>,
    pub(crate) task_planner: Arc<TaskPlanner>,
    pub memory_store: Arc<MemoryStore>,
    pub rules_engine: Arc<RulesEngine>,
    pub workflow_engine: Arc<WorkflowEngine>,
    pub(crate) tool_invoker: Arc<ToolInvoker>,
    pub(crate) conversation_state: AsyncMutex<Vec<ChatMessage>>,
    pub(crate) editor_state: std::sync::RwLock<std::sync::Weak<crate::EditorState>>,
    /// Lemonade server base URL. Set from the frontend (`set_lemonade_url`)
    /// whenever the user picks the Lemonade inference backend or changes its
    /// port in Settings. Defaults to `http://localhost:13305`.
    pub(crate) lemonade_url: tokio::sync::Mutex<String>,
    /// Caps concurrent local backend HTTP calls from this process so one desktop seat
    /// does not trip nginx `limit_conn` on a shared reverse proxy.
    pub(crate) local_http_sem: Arc<Semaphore>,
    pub(crate) _browser_state: Arc<crate::browser::BrowserState>,
    pub(crate) stop_signal: Arc<AtomicBool>,
    pub(crate) pause_signal: Arc<AtomicBool>,
    pub(crate) brain_dir: PathBuf,
    pub(crate) advisor_model: tokio::sync::Mutex<Option<String>>,
    pub(crate) memory_optimizer: Arc<crate::memory_optimizer::MemoryOptimizer>,
    pub(crate) perf_monitor: Arc<crate::performance::PerformanceMonitor>,
    pub ghost_runtime: Arc<crate::ghost_runtime::GhostRuntime>,
    pub shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>,
    pub yolo_mode: Arc<AtomicBool>,
    /// While > 0, `emit_event` is a no-op. Used by `ai_chat_oneshot` so
    /// background agents don't paint over the foreground chat with
    /// `task-phase-update`, `ai-content-delta`, etc. The counter form lets
    /// nested oneshot calls (and the inner phase wrappers) compose safely.
    pub(crate) silent_emits: Arc<AtomicUsize>,
    /// Mirror of UI activity events (tool calls/results/actions) as JSON lines,
    /// drained by the frontend via `agent_activity_drain`. The global event
    /// stream does not reach the webview here, so the live activity terminal
    /// polls this instead.
    pub activity_log: Arc<std::sync::Mutex<Vec<String>>>,
    /// Reviewable edit proposals from the autonomous loop. Each is a JSON
    /// `{path, oldContent, newContent, description, additions, deletions}`.
    /// Drained by the frontend via `agent_proposals_drain` → the diff-review
    /// panel. Same dead-event workaround as `activity_log`: edits are applied to
    /// disk, surfaced here for accept (keep) / reject (revert to oldContent).
    pub pending_proposals: Arc<std::sync::Mutex<Vec<Value>>>,
    /// Live chat token stream buffer. The `ai-content-delta` event is dead in the
    /// webview, so the frontend polls `chat_stream_drain` to render streamed
    /// tokens in real time. Appended in the streaming parser, drained per poll.
    pub chat_stream_buf: Arc<std::sync::Mutex<String>>,
    pub(crate) session_id: String,
    pub ane_engine: Arc<tokio::sync::Mutex<Option<crate::ane::AneEngine>>>,
    pub attachment_manager: Arc<crate::attachment_manager::AttachmentManager>,
    pub knowledge_distiller: Arc<crate::knowledge_distiller::KnowledgeDistiller>,
    // High-speed RAM Caches to resolve regression
    pub(crate) project_files_cache: tokio::sync::Mutex<Option<String>>,
    pub(crate) workspace_memory_cache: tokio::sync::Mutex<Option<String>>,
    pub(crate) global_brain_cache: tokio::sync::Mutex<Option<String>>,
    pub harness: Arc<hades_harness::ReasoningLoop>,
    pub airi: Arc<tokio::sync::Mutex<Option<crate::airi_bridge::AiriBridge>>>,
    /// Shared with EditorState::tool_permission_senders.
    /// The frontend resolves dangerous-tool approvals by sending the bool here.
    pub permission_senders: Arc<std::sync::Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<bool>>>>,
}

/// True if `want` matches a model in `catalog`, tolerating the ways local model
/// ids drift: exact, prefix either way (a `:latest`/quant suffix on one side),
/// or a shared stem before the first `:` tag. Case-insensitive. Pure so the
/// connectivity-preflight matching is unit-tested without a live server.
pub(crate) fn model_in_catalog(want: &str, catalog: &[String]) -> bool {
    let want = want.to_lowercase();
    let want_stem = want.split(':').next().unwrap_or(&want);
    catalog.iter().any(|m| {
        let ml = m.to_lowercase();
        ml == want
            || ml.starts_with(&want)
            || want.starts_with(&ml)
            || ml.split(':').next().unwrap_or(&ml) == want_stem
    })
}

impl Sentient {
    /// Directory that holds `api_keys.json` — the brain dir's parent, falling
    /// back to the brain dir itself when it is a filesystem root (no parent).
    /// Never panics, unlike the former `brain_dir.parent().unwrap()`.
    pub(crate) fn brain_parent(&self) -> &Path {
        self.brain_dir.parent().unwrap_or(self.brain_dir.as_path())
    }

    /// Canonical path to the API-keys store. Collapses the repeated
    /// `brain_dir.parent().unwrap().join("api_keys.json")` into one safe call.
    pub(crate) fn api_keys_path(&self) -> PathBuf {
        self.brain_parent().join("api_keys.json")
    }

    pub fn new(
        api_key: String,
        root_path: PathBuf,
        browser_state: Arc<crate::browser::BrowserState>,
        git_manager: Arc<crate::git::GitManager>,
        config_dir: PathBuf,
        memory_optimizer: Arc<crate::memory_optimizer::MemoryOptimizer>,
        perf_monitor: Arc<crate::performance::PerformanceMonitor>,
        attachment_manager: Arc<crate::attachment_manager::AttachmentManager>,
        knowledge_distiller: Arc<crate::knowledge_distiller::KnowledgeDistiller>,
        patch_engine: Arc<tokio::sync::Mutex<crate::patch_engine::PatchEngine>>,
        ghost_runtime: Arc<crate::ghost_runtime::GhostRuntime>,
        shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>,
    ) -> Self {
        let brain_dir = config_dir.join("brain");
        if !brain_dir.exists() {
            let _ = std::fs::create_dir_all(&brain_dir);
        }

        // Mount Kortex persistent memory (initial sync with brain dir)
        let memory_store = Arc::new(MemoryStore::new());
        let vfs_bridge = crate::vfs_bridge::VfsBridge::new(root_path.clone());
        {
            let ms = memory_store.clone();
            let rp = root_path.clone();
            let vb = vfs_bridge.clone();
            crate::event_sink::spawn_detached(async move {
                ms.mount(Some(rp)).await;
                ms.set_vfs_bridge(vb).await;
            });
        }

        let mcp_registry = Arc::new(McpRegistry::new(config_dir.join("mcp_servers.json")));
        // Shared activity buffer: the live agent feed the webview polls. AiTools
        // gets the SAME Arc so streamed command stdout/stderr lands in it too
        // (those emits bypass `emit_event`, and the event stream is dead anyway).
        let activity_log: Arc<std::sync::Mutex<Vec<String>>> =
            Arc::new(std::sync::Mutex::new(Vec::new()));
        let yolo_flag: Arc<std::sync::atomic::AtomicBool> =
            Arc::new(std::sync::atomic::AtomicBool::new(false));
        let ai_tools = Arc::new(AiTools::new(
            root_path.clone(),
            browser_state.clone(),
            git_manager.clone(),
            mcp_registry.clone(),
            memory_store.clone(),
            knowledge_distiller.clone(),
            patch_engine.clone(),
            ghost_runtime.clone(),
            shadow_workspace.clone(),
            None,
            activity_log.clone(),
            yolo_flag.clone(),
        ));
        
        let task_planner = Arc::new(TaskPlanner::new());
        let rules_engine = Arc::new(RulesEngine::new(root_path.clone()));
        let workflow_engine = Arc::new(WorkflowEngine::new(root_path.clone()));
        let tool_invoker = Arc::new(ToolInvoker::new(
            ai_tools.clone(),
            mcp_registry.clone(),
            config_dir.clone(),
        ));
        let ane_engine = Arc::new(tokio::sync::Mutex::new(None));

        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            // 24/7 coding: a long local generation (or a cold model load) must NOT be
            // killed by a total-request cap. The real hang guard is the per-chunk
            // inter-token timeout in the streaming loop (no bytes for N s → error).
            // Per-request `.timeout()` still tightens this for cloud/advisor calls.
            .timeout(std::time::Duration::from_secs(3600))
            .no_proxy()
            .build()
            .unwrap_or_else(|_| Client::new());

        Self {
            client,
            api_key,
            mcp_registry,
            ai_tools,
            task_planner,
            memory_store,
            rules_engine,
            workflow_engine,
            tool_invoker,
            conversation_state: AsyncMutex::new(Vec::new()),
            editor_state: std::sync::RwLock::new(std::sync::Weak::new()),
            // Seed from the env var (if present) so a non-default port works on
            // the very first request, before the frontend calls set_lemonade_url.
            lemonade_url: tokio::sync::Mutex::new(
                std::env::var("LEMONADE_URL")
                    .ok()
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "http://localhost:13305".to_string()),
            ),
            local_http_sem: Arc::new(Semaphore::new(4)),
            _browser_state: browser_state.clone(),
            stop_signal: Arc::new(AtomicBool::new(false)),
            pause_signal: Arc::new(AtomicBool::new(false)),
            brain_dir,
            advisor_model: tokio::sync::Mutex::new(None),
            memory_optimizer,
            perf_monitor,
            session_id: uuid::Uuid::new_v4().to_string(),
            ane_engine,
            attachment_manager,
            knowledge_distiller: Arc::new(crate::knowledge_distiller::KnowledgeDistiller::new(&root_path)),
            ghost_runtime,
            shadow_workspace,
            yolo_mode: yolo_flag.clone(),
            silent_emits: Arc::new(AtomicUsize::new(0)),
            activity_log,
            pending_proposals: Arc::new(std::sync::Mutex::new(Vec::new())),
            chat_stream_buf: Arc::new(std::sync::Mutex::new(String::new())),
            project_files_cache: tokio::sync::Mutex::new(None),
            workspace_memory_cache: tokio::sync::Mutex::new(None),
            global_brain_cache: tokio::sync::Mutex::new(None),
            harness: Arc::new(hades_harness::ReasoningLoop::new(&root_path)),
            airi: Arc::new(tokio::sync::Mutex::new(None)),
            permission_senders: Arc::new(std::sync::Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// RAII guard for silent-emit mode. Drop the returned guard to restore
    /// foreground events. See `ai_chat_oneshot`.
    pub fn enter_silent(self: &Arc<Self>) -> SilentEmitGuard {
        self.silent_emits.fetch_add(1, Ordering::SeqCst);
        SilentEmitGuard { counter: self.silent_emits.clone() }
    }

    /// Resolve the active Lemonade base URL: the stored value (set from the
    /// frontend), else `LEMONADE_URL` env var, else the default port.
    /// Pre-run connectivity check for LOCAL providers. Converts the two silent-
    /// failure modes into immediate, actionable feedback instead of a stalled turn:
    ///   - server unreachable → hard `Err` (fail fast with the URL + underlying error)
    ///   - model not in catalog → `Ok(Some(warning))` (the server may still load it
    ///     on demand, so this is a heads-up, not a block).
    /// `Ok(None)` means the server answered and the model looks present. Reuses
    /// `list_models`, which already returns actionable auth/gate errors.
    pub(crate) async fn local_connectivity_preflight(
        &self,
        provider: &str,
        model: &str,
    ) -> anyhow::Result<Option<String>> {
        let p = provider.to_lowercase();
        let is_local = matches!(
            p.as_str(),
            "lemonade" | "huggingface" | "antigravity"
                | "deepseek-ane" | "deepseek_ane" | "ds2-ane"
        );
        if !is_local {
            return Ok(None);
        }

        let models = match self.list_models(provider).await {
            Ok(m) => m,
            Err(e) => {
                let base = match p.as_str() {
                    "lemonade" => self.lemonade_base().await,
                    _ => String::new(),
                };
                return Err(anyhow::anyhow!(
                    "{} server is not reachable{} — start it and confirm the URL in \
                     Settings, then retry. ({})",
                    provider,
                    if base.is_empty() { String::new() } else { format!(" at {}", base) },
                    e
                ));
            }
        };
        // Empty catalog: some gateways don't list; can't verify, so don't block.
        if models.is_empty() {
            return Ok(None);
        }
        if model_in_catalog(model, &models) {
            return Ok(None);
        }
        let sample: Vec<String> = models.iter().take(8).cloned().collect();
        let more = models.len().saturating_sub(sample.len());
        Ok(Some(format!(
            "Model '{}' is not in {}'s loaded catalog. Available: {}{}. The server may \
             load it on demand; if the run stalls, pull or select one of these.",
            model,
            provider,
            sample.join(", "),
            if more > 0 { format!(", … (+{} more)", more) } else { String::new() }
        )))
    }

    pub(crate) async fn lemonade_base(&self) -> String {
        let stored = self.lemonade_url.lock().await.clone();
        let raw = if !stored.trim().is_empty() {
            stored
        } else {
            std::env::var("LEMONADE_URL")
                .ok()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "http://localhost:13305".to_string())
        };
        // Return a normalized ROOT (no trailing slash, no trailing `/v1`). Callers
        // append `/v1/models`, `/v1/chat/completions`, etc. The user often types
        // the base WITH `/v1` (it's the documented client base URL), which would
        // otherwise produce `/v1/v1/models`.
        Self::strip_lemonade_v1(&raw)
    }

    /// Synchronous version of `lemonade_base` using `blocking_lock` to resolve
    /// the base URL in synchronous contexts like `get_endpoint`.
    pub(crate) fn lemonade_base_blocking(&self) -> String {
        let stored = self.lemonade_url.blocking_lock().clone();
        let raw = if !stored.trim().is_empty() {
            stored
        } else {
            std::env::var("LEMONADE_URL")
                .ok()
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "http://localhost:13305".to_string())
        };
        Self::strip_lemonade_v1(&raw)
    }

    /// Strip a trailing slash and a trailing `/v1` segment from a Lemonade base.
    pub(crate) fn strip_lemonade_v1(url: &str) -> String {
        let t = url.trim().trim_end_matches('/');
        t.strip_suffix("/v1").unwrap_or(t).trim_end_matches('/').to_string()
    }

    pub async fn set_lemonade_url(&self, url: String) {
        let trimmed = url.trim().trim_end_matches('/').to_string();
        let lower = trimmed.to_lowercase();
        let normalized = if trimmed.is_empty() {
            "http://localhost:13305".to_string()
        } else if lower.starts_with("http://") || lower.starts_with("https://") {
            trimmed
        } else {
            format!("http://{}", trimmed)
        };
        let mut u = self.lemonade_url.lock().await;
        *u = normalized;
    }

    pub(crate) async fn local_http_permit(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.local_http_sem
            .clone()
            .acquire_owned()
            .await
            .expect("local_http_sem not closed")
    }

    pub async fn set_advisor_model(&self, model: Option<String>) {
        let mut m = self.advisor_model.lock().await;
        *m = model;
    }

    pub fn get_hades_harness(&self) -> Option<Arc<crate::hades_harness::HadesHarness>> {
        self.editor_state().map(|s| s.ai.harness.clone())
    }

    /// Upgrade the back-reference to the live EditorState, if still alive.
    pub(crate) fn editor_state(&self) -> Option<Arc<crate::EditorState>> {
        self.editor_state.read().ok().and_then(|w| w.upgrade())
    }

    /// Verifies if a code modification (patch or write) is valid according to the compiler.
    pub(crate) async fn verify_code_change(&self, tool_name: &str, args: &Value) -> Result<Option<Vec<hades_harness::Diagnostic>>> {
        let path_str = match tool_name {
            "patch_file_content" | "write_to_file" | "search_replace_edit" => args["path"].as_str(),
            _ => return Ok(None),
        };

        if let Some(p) = path_str {
            let relative_path = Path::new(p);
            let root: PathBuf = self.ai_tools.get_root_path();
            let full_path = root.join(relative_path);

            // Only verify Rust files for now
            if relative_path.extension().map(|e| e != "rs").unwrap_or(true) {
                return Ok(None);
            }

            let new_content = if tool_name == "write_to_file" {
                args["content"].as_str().unwrap_or("").to_string()
            } else if tool_name == "patch_file_content" {
                let content = fs::read_to_string(&full_path).unwrap_or_default();
                let start_line = args["StartLine"].as_u64().unwrap_or(1) as usize;
                let end_line = args["EndLine"].as_u64().unwrap_or(1) as usize;
                let replacement = args["ReplacementContent"].as_str().unwrap_or("");
                
                let lines: Vec<String> = content.lines().map(|s: &str| s.to_string()).collect();
                if start_line > 0 && start_line <= lines.len() + 1 {
                    let mut new_lines = Vec::new();
                    new_lines.extend_from_slice(&lines[..start_line - 1]);
                    new_lines.push(replacement.to_string());
                    new_lines.extend_from_slice(&lines[std::cmp::min(end_line, lines.len())..]);
                    new_lines.join("\n")
                } else {
                    content
                }
            } else {
                return Ok(None); 
            };

            println!("[Harness] Verifying patch for {:?}...", relative_path);
            let diags = self.harness.verify_candidate(relative_path, &new_content)?;
            
            let errors: Vec<_> = diags.into_iter().filter(|d| d.level == "error").collect();
            if !errors.is_empty() {
                return Ok(Some(errors));
            }
        }

        Ok(None)
    }

    pub fn get_tools(&self) -> Arc<AiTools> {
        self.ai_tools.clone()
    }

    /// Set the EditorState back-reference. `ai_tools`/`memory_store`/`patch_engine`
    /// are wired separately by `EditorState::wire_back_refs`.
    pub fn set_editor_state(&self, weak: std::sync::Weak<crate::EditorState>) {
        if let Ok(mut g) = self.editor_state.write() {
            *g = weak;
        }
    }

    pub fn set_root_path(&self, root_path: PathBuf) {
        let tools = self.ai_tools.clone();
        let rules = self.rules_engine.clone();
        let mcp = self.mcp_registry.clone();
        let ms = self.memory_store.clone();
        let rp_tools = root_path.clone();
        let rp_mount = root_path.clone();
        let rp_mcp = root_path.clone();
        rules.set_root(root_path);

        crate::event_sink::spawn_detached(async move {
            tools.set_root_path(rp_tools).await;
        });

        crate::event_sink::spawn_detached(async move {
            ms.mount(Some(rp_mount)).await;
        });

        crate::event_sink::spawn_detached(async move {
            if let Err(e) = mcp.merge_workspace_config(&rp_mcp).await {
                eprintln!("[cursor] MCP merge failed: {e}");
            }
        });
    }

    /// Reload Cursor + Kiro + Antigravity workspace layout.
    pub async fn reload_workspace(&self, root: &std::path::Path) -> Result<serde_json::Value> {
        use crate::workspace_compat::{scan_workspace, load_environment, format_environment_for_prompt, append_debug_log, format_steering_for_prompt, load_steering_docs};
        self.rules_engine.set_root(root.to_path_buf());
        self.mcp_registry.merge_workspace_config(root).await?;
        let scan = scan_workspace(root)?;
        let steering_prompt = format_steering_for_prompt(&load_steering_docs(root));
        let _ = append_debug_log(root, serde_json::json!({
            "kind": "workspace_reload",
            "cursor_rules": scan.cursor.rules_count,
            "steering": scan.steering_count,
            "kiro_hooks": scan.kiro_hooks_count,
            "mcp_servers": scan.cursor.mcp_server_count + scan.kiro_mcp_count,
        }));
        Ok(serde_json::json!({
            "scan": scan,
            "environment_prompt": load_environment(root).map(|e| format_environment_for_prompt(&e)),
            "steering_prompt": steering_prompt,
        }))
    }

    /// Back-compat alias.
    pub async fn reload_cursor_workspace(&self, root: &std::path::Path) -> Result<serde_json::Value> {
        self.reload_workspace(root).await
    }

    /// Main autonomous reasoning loop with iterative tool invocation and task planning.
    pub async fn register_mcp_server(&self, name: String, config: McpServerConfig) -> Result<()> {
        self.mcp_registry.add_server(name, config).await
    }

    pub async fn list_mcp_servers(&self) -> Result<Vec<Value>> {
        Ok(self.mcp_registry.list_servers().await)
    }

    /// Yolo mode: disables pre-flight cargo check, auto-applies shadow patches,
    /// and raises iteration ceiling to 200. Full sentient autonomy.
    pub fn set_yolo_mode(&self, enabled: bool) {
        self.yolo_mode.store(enabled, Ordering::SeqCst);
        println!("[Sentient] Yolo mode: {}", if enabled { "ENGAGED" } else { "OFF" });
    }

    pub fn is_yolo_mode(&self) -> bool {
        self.yolo_mode.load(Ordering::SeqCst)
    }

    pub fn stop(&self) {
        self.stop_signal.store(true, Ordering::SeqCst);
        if let Ok(mut b) = self.chat_stream_buf.lock() {
            b.clear();
        }
        // Kill any foreground command still running (e.g. a long scan). The loop's
        // stop check only fires between tools, so without this a running child
        // (nmap/ffuf/sqlmap) would keep going and wedge the turn until it exits.
        crate::process_registry::kill_all();
        self.emit_event("ai-stopped", json!({ "reason": "user" }));
    }

    pub fn reset_stop_signal(&self) {
        self.stop_signal.store(false, Ordering::SeqCst);
    }

    pub fn is_stopped(&self) -> bool {
        self.stop_signal.load(Ordering::SeqCst)
    }

    /// If the user pressed Stop, return the canonical early-exit message.
    pub(crate) fn stopped_message(&self) -> Option<String> {
        if self.is_stopped() {
            Some("Execution stopped by user.".to_string())
        } else {
            None
        }
    }

    pub fn pause(&self) {
        self.pause_signal.store(true, Ordering::SeqCst);
    }

    pub fn resume(&self) {
        self.pause_signal.store(false, Ordering::SeqCst);
        self.reset_stop_signal(); // Resume also clears stop if we want to restart a soft-stop (optional)
    }

    pub fn is_paused(&self) -> bool {
        self.pause_signal.load(Ordering::SeqCst)
    }

    pub async fn wait_if_paused(&self) {
        while self.is_paused() && !self.is_stopped() {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        }
    }

}

impl Sentient {
    pub fn emit_event(&self, event: &str, payload: Value) {
        // Suppress all UI events while a `ai_chat_oneshot` (background)
        // run is in progress. The counter form lets nested or recursive
        // background calls remain silent until the outermost finishes.
        if self.silent_emits.load(Ordering::SeqCst) > 0 {
            return;
        }
        // Mirror activity events into the pollable log (the webview can't receive
        // the live event stream, so the activity terminal drains this instead).
        if event == "ai-content-delta" {
            if let Some(delta) = payload.get("delta").and_then(|v| v.as_str()) {
                if !delta.is_empty() {
                    if let Ok(mut b) = self.chat_stream_buf.lock() {
                        b.push_str(delta);
                    }
                }
            }
        }
        if matches!(
            event,
            "ai-action"
                | "ai-tool-call"
                | "ai-tool-result"
                | "ai-tool-stdout-start"
                | "ai-tool-stdout"
                | "ai-tool-stdout-end"
        ) {
            if let Ok(mut log) = self.activity_log.lock() {
                if let Ok(line) = serde_json::to_string(&json!({ "kind": event, "payload": payload })) {
                    log.push(line);
                    if log.len() > 1000 {
                        let drop = log.len() - 1000;
                        log.drain(0..drop);
                    }
                }
            }
        }
        if let Some(es) = self.editor_state() {
            es.emit(event, payload);
        }
    }

    pub(crate) fn try_parse_task_plan(&self, content: &str) -> Option<Vec<crate::task_planner::TaskStep>> {
        if let Some(start) = content.find("```task-plan") {
            let rest = &content[start + 12..];
            if let Some(end) = rest.find("```") {
                let json_str = rest[..end].trim();
                if let Ok(steps) = serde_json::from_str::<Vec<crate::task_planner::TaskStep>>(json_str) {
                    return Some(steps);
                }
            }
        }
        None
    }

    pub async fn reason(self: Arc<Self>, prompt: &str) -> anyhow::Result<String> {
        let resp = self.clone().chat_complete(prompt, None, None, None, None).await?;
        Ok(resp.content)
    }
    /// Attempts to fix common tool argument hallucinations from small models.
    pub(crate) fn repair_tool_arguments(&self, tool_name: &str, args: &mut Value) {
        if let Some(obj) = args.as_object_mut() {
             // Case 1: Model uses the pattern/query as a KEY instead of a value
             // e.g. {"**/*.cpp": "TODO"} for grep/glob
             if obj.len() == 1 {
                 let key = obj.keys().next().unwrap().clone();
                 // If the key looks like a path or pattern (contains . / *), it's likely a hallucination
                 if key.contains('.') || key.contains('*') || key.contains('/') || key.contains('\\') {
                      let value = obj.get(&key).unwrap().clone();
                      match tool_name {
                          "grep" | "search_files" | "search_project" => {
                              obj.clear();
                              obj.insert("query".to_string(), value);
                              obj.insert("path".to_string(), json!(key));
                          },
                          "find_by_name" | "glob" | "list_files" => {
                              obj.clear();
                              obj.insert("pattern".to_string(), json!(key));
                          },
                          "view_file" | "read_file" | "file_read" => {
                              obj.clear();
                              obj.insert("path".to_string(), json!(key));
                          },
                          _ => {}
                      }
                 }
             }
        }
    }

    /// Detects if a prompt is an imperative action (write/run/fix) vs descriptive/chat.
    pub fn looks_like_action_request(text: &str) -> bool {
        let t = text.to_lowercase();
        // Action verbs that usually require a tool call
        let verbs = [
            "write", "create", "generate", "make", "build", "implement", "add", "edit", "patch", "fix", 
            "refactor", "delete", "remove", "run", "execute", "launch", "invoke", "fuzz", "exploit", 
            "scan", "recon", "enumerate", "inject", "craft", "attack", "brute", "crack", "deploy", 
            "install", "compile", "test", "verify", "push", "commit", "merge", "rebase", "checkout", 
            "shell", "apply", "save", "persist"
        ];
        for v in &verbs {
            if t.contains(v) { return true; }
        }
        // Coding markers/paths that suggest action
        if t.contains('/') || t.contains('\\') || t.contains("```") || t.contains('.') {
            return true;
        }
        false
    }
}

fn _assert_sentient_sync() {
    fn is_sync<T: Sync>() {}
    is_sync::<Sentient>();
}

#[cfg(test)]
mod tests {
    use super::LOCAL_ESSENTIAL_TOOLS;

    /// REGRESSION GUARD: offensive-security tools must remain available to
    /// local local models. They are a core IDE strength; dropping them from
    /// the essential set would hide them from small models and break the
    /// red-team / pentest / bug-bounty workflows.
    #[test]
    pub(crate) fn local_essentials_keep_offensive_tools() {
        for tool in [
            "weaponize_env",
            "apex_red_team_scan",
            "apex_quick_check",
            "apex_threat_anticipate",
            "apex_simulate_attack",
            "apex_pentest_report",
            "apex_scan_url",
            "binary_mach_o_scanner",
            "file_entropy_analysis",
            "network_port_scanner",
            "extract_strings",
            "hex_dump",
            "secrets_scan",
            "reverse_shell_generate",
            "exploit_lookup",
            "network_scan",
            "web_security_audit",
            "ai_vuln_hunt",
            "deep_security_audit",
        ] {
            assert!(
                LOCAL_ESSENTIAL_TOOLS.contains(&tool),
                "{tool} must stay in LOCAL_ESSENTIAL_TOOLS (offensive-security strength)"
            );
        }
    }

    /// The AIM zero-grep tools must also stay available to local models.
    #[test]
    pub(crate) fn local_essentials_keep_aim_tools() {
        assert!(LOCAL_ESSENTIAL_TOOLS.contains(&"aim_pack_context"));
        assert!(LOCAL_ESSENTIAL_TOOLS.contains(&"aim_query_spans"));
    }

    use super::model_in_catalog;

    #[test]
    fn catalog_match_exact_and_case_insensitive() {
        let cat = vec!["Qwen2.5-Coder:7b".to_string(), "gemma-4-12b".to_string()];
        assert!(model_in_catalog("qwen2.5-coder:7b", &cat));
        assert!(model_in_catalog("GEMMA-4-12B", &cat));
    }

    #[test]
    fn catalog_match_tolerates_tag_suffix_either_side() {
        // Requested bare stem, catalog has a :tag — and vice versa.
        let cat = vec!["llama3.1:8b-instruct-q4_0".to_string()];
        assert!(model_in_catalog("llama3.1", &cat));
        let cat2 = vec!["llama3.1".to_string()];
        assert!(model_in_catalog("llama3.1:8b", &cat2));
    }

    #[test]
    fn catalog_match_rejects_absent_model() {
        let cat = vec!["qwen2.5-coder:7b".to_string(), "gemma-4-12b".to_string()];
        assert!(!model_in_catalog("deepseek-r1:32b", &cat));
        assert!(!model_in_catalog("mistral", &cat));
    }
}
