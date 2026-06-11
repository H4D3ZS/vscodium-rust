//! Sentient: the AI engine struct, lifecycle, events, and reasoning entry.

use anyhow::Result;
use futures::StreamExt;
use tauri::{Manager, Emitter};

use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex as AsyncMutex;
use tokio::sync::Semaphore;

use crate::ai_auth::AuthState;
use crate::ai_tools::AiTools;
use crate::mcp_registry::{McpRegistry, McpServerConfig};
use crate::memory_store::MemoryStore;
use crate::task_planner::TaskPlanner;
use crate::tool_invoker::ToolInvoker;
use crate::rules_engine::RulesEngine;
use crate::workflow_engine::WorkflowEngine;

/// Tools always exposed to local Ollama models (which have small context
/// windows and degrade with large tool lists). This is the single source of
/// truth — the autonomous loop's tool filter references it.
///
/// IMPORTANT: the offensive-security tools at the end are a CORE IDE strength.
/// Removing them would gut the red-team / pentest / bug-bounty playbooks. A
/// regression test (`tests::ollama_essentials_keep_offensive_tools`) enforces
/// their presence.
pub(crate) const OLLAMA_ESSENTIAL_TOOLS: &[&str] = &[
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
    pub(crate) app_handle: std::sync::RwLock<Option<AppHandle>>,
    pub(crate) auth_state: Arc<AuthState>,
    pub(crate) ollama_url: tokio::sync::Mutex<String>,
    /// Caps concurrent Ollama HTTP calls from this process so one desktop seat
    /// does not trip nginx `limit_conn` on a shared reverse proxy.
    pub(crate) ollama_http_sem: Arc<Semaphore>,
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

impl Sentient {
    pub fn new(
        api_key: String,
        root_path: PathBuf,
        auth_state: Arc<AuthState>,
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
            tauri::async_runtime::spawn(async move {
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
            .timeout(std::time::Duration::from_secs(600)) // 10 minute total timeout for heavy local inference
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
            app_handle: std::sync::RwLock::new(None),
            auth_state,
            ollama_url: tokio::sync::Mutex::new("http://localhost:11434".to_string()),
            ollama_http_sem: Arc::new(Semaphore::new(4)),
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
            yolo_mode: Arc::new(AtomicBool::new(false)),
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

    pub async fn set_ollama_url(&self, url: String) {
        let normalized = normalize_ollama_base_url(&url);
        let mut u = self.ollama_url.lock().await;
        *u = normalized;
    }

    pub(crate) async fn ollama_http_permit(&self) -> tokio::sync::OwnedSemaphorePermit {
        self.ollama_http_sem
            .clone()
            .acquire_owned()
            .await
            .expect("ollama_http_sem not closed")
    }

    pub async fn set_advisor_model(&self, model: Option<String>) {
        let mut m = self.advisor_model.lock().await;
        *m = model;
    }

    pub fn get_hades_harness(&self) -> Option<Arc<crate::hades_harness::HadesHarness>> {
        let h_lock = self.app_handle.read().ok()?;
        h_lock.as_ref().map(|h| {
            let state: tauri::State<crate::EditorState> = h.state();
            state.ai.harness.clone()
        })
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

    pub async fn set_app_handle(&self, handle: AppHandle) {
        if let Ok(mut h) = self.app_handle.write() {
            *h = Some(handle.clone());
        }
        self.ai_tools.set_app_handle(handle.clone()).await;
        
        let ms = self.memory_store.clone();
        tauri::async_runtime::spawn(async move {
            ms.set_app_handle(handle).await;
        });
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

        tauri::async_runtime::spawn(async move {
            tools.set_root_path(rp_tools).await;
        });

        tauri::async_runtime::spawn(async move {
            ms.mount(Some(rp_mount)).await;
        });

        tauri::async_runtime::spawn(async move {
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
        // Mirror to env var so ToolInvoker bypasses permission dialogs in yolo mode.
        unsafe { std::env::set_var("AIRI_YOLO_MODE", if enabled { "1" } else { "0" }); }
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
        use tauri::Emitter;
        if let Ok(guard) = self.app_handle.read() {
            if let Some(handle) = guard.as_ref() {
                let _ = handle.emit(event, payload);
            } else {
                println!("[WARN] emit_event: app_handle is None for event '{}'", event);
            }
        } else {
            println!("[WARN] emit_event: COULD NOT ACQUIRE READ LOCK for event '{}'", event);
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
    use super::OLLAMA_ESSENTIAL_TOOLS;

    /// REGRESSION GUARD: offensive-security tools must remain available to
    /// local Ollama models. They are a core IDE strength; dropping them from
    /// the essential set would hide them from small models and break the
    /// red-team / pentest / bug-bounty workflows.
    #[test]
    pub(crate) fn ollama_essentials_keep_offensive_tools() {
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
                OLLAMA_ESSENTIAL_TOOLS.contains(&tool),
                "{tool} must stay in OLLAMA_ESSENTIAL_TOOLS (offensive-security strength)"
            );
        }
    }

    /// The AIM zero-grep tools must also stay available to local models.
    #[test]
    pub(crate) fn ollama_essentials_keep_aim_tools() {
        assert!(OLLAMA_ESSENTIAL_TOOLS.contains(&"aim_pack_context"));
        assert!(OLLAMA_ESSENTIAL_TOOLS.contains(&"aim_query_spans"));
    }
}
