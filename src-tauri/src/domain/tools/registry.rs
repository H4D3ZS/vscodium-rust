//! AiTools: struct, constructor, entitlement gating, shared activity helpers.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use tree_sitter::StreamingIterator;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Clone)]
pub struct AiTools {
    pub root_path: Arc<tokio::sync::Mutex<PathBuf>>,
    pub app_handle: Arc<tokio::sync::Mutex<Option<tauri::AppHandle>>>,
    pub browser_state: Arc<crate::browser::BrowserState>,
    pub(crate) git_manager: Arc<crate::git::GitManager>,
    pub(crate) mcp_registry: Arc<crate::mcp_registry::McpRegistry>,
    pub(crate) memory_store: Arc<crate::memory_store::MemoryStore>,
    pub knowledge_distiller: Arc<crate::knowledge_distiller::KnowledgeDistiller>,
    pub patch_engine: Arc<tokio::sync::Mutex<crate::patch_engine::PatchEngine>>,
    pub ghost_runtime: Arc<crate::ghost_runtime::GhostRuntime>,
    pub shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>,
    pub apex: Arc<tokio::sync::Mutex<Option<Arc<crate::apex_orchestrator::ApexOrchestrator>>>>,
    pub(crate) vector_indexer: Arc<tokio::sync::Mutex<Option<Arc<crate::vector_indexer::VectorIndexer>>>>,
    /// Shared live-activity buffer (same Arc as `Sentient.activity_log`). The
    /// streamed-command reader threads push `{kind,payload}` JSON lines here so
    /// the activity terminal can drain them — `h.emit` is dead in the webview.
    pub activity_log: Arc<std::sync::Mutex<Vec<String>>>,
}

/// Append one `{kind,payload}` activity line to the shared buffer (capped at
/// 1000 lines). Mirrors `Sentient::emit_event` so streamed command output shows
/// up in the activity terminal, which polls this buffer.
pub(crate) fn push_activity(log: &Arc<std::sync::Mutex<Vec<String>>>, kind: &str, payload: Value) {
    if let Ok(mut l) = log.lock() {
        if let Ok(line) = serde_json::to_string(&json!({ "kind": kind, "payload": payload })) {
            l.push(line);
            if l.len() > 1000 {
                let drop = l.len() - 1000;
                l.drain(0..drop);
            }
        }
    }
}

/// Best-effort JSON extraction from a model reply that may wrap the payload in
/// ```json fences or surround it with prose. Tries the whole string, then the
/// widest `[...]` array slice, then the widest `{...}` object slice. Used by the
/// AI vuln-hunt pipeline where small local models rarely return clean JSON.
pub(crate) fn extract_json_loose(text: &str) -> Option<Value> {
    let mut t = text.trim();
    if let Some(rest) = t.strip_prefix("```json") { t = rest; }
    else if let Some(rest) = t.strip_prefix("```") { t = rest; }
    let t = t.trim_start_matches("```").trim_end_matches("```").trim();
    if let Ok(v) = serde_json::from_str::<Value>(t) { return Some(v); }
    if let (Some(s), Some(e)) = (t.find('['), t.rfind(']')) {
        if e > s {
            if let Ok(v) = serde_json::from_str::<Value>(&t[s..=e]) { return Some(v); }
        }
    }
    if let (Some(s), Some(e)) = (t.find('{'), t.rfind('}')) {
        if e > s {
            if let Ok(v) = serde_json::from_str::<Value>(&t[s..=e]) { return Some(v); }
        }
    }
    None
}

/// Rough parameter-size hint (in billions) parsed from a model id, e.g.
/// `qwen2.5:32b` → 32, `llama3:8b` → 8, `BugTraceAI-Apex-G4-26B-Q4` → 26.
/// Used to rank local models into cheap/mid/strong tiers. 0 when unknown.
pub(crate) fn model_size_hint(model: &str) -> u32 {
    let lc = model.to_lowercase();
    let bytes = lc.as_bytes();
    let mut best = 0u32;
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
            // followed by 'b' (optionally after nothing) → a parameter count
            if i < bytes.len() && bytes[i] == b'b' {
                if let Ok(n) = lc[start..i].parse::<u32>() {
                    if n <= 1000 && n > best { best = n; }
                }
            }
        } else {
            i += 1;
        }
    }
    best
}

/// True if a model id looks security-specialized (preferred for the strong
/// confirm tier): BugTrace, APEX, SecurityEngineer, red-team, abliterated, etc.
pub(crate) fn is_security_model(model: &str) -> bool {
    let lc = model.to_lowercase();
    ["bugtrace", "apex", "securityengineer", "security", "redteam", "red-team",
     "pentest", "abliterat", "neuraldevil", "hacker", "exploit"]
        .iter().any(|k| lc.contains(k))
}

impl AiTools {
    pub fn new(
        root_path: PathBuf,
        browser_state: Arc<crate::browser::BrowserState>,
        git_manager: Arc<crate::git::GitManager>,
        mcp_registry: Arc<crate::mcp_registry::McpRegistry>,
        memory_store: Arc<crate::memory_store::MemoryStore>,
        knowledge_distiller: Arc<crate::knowledge_distiller::KnowledgeDistiller>,
        patch_engine: Arc<tokio::sync::Mutex<crate::patch_engine::PatchEngine>>,
        ghost_runtime: Arc<crate::ghost_runtime::GhostRuntime>,
        shadow_workspace: Arc<crate::shadow_workspace::ShadowWorkspace>,
        apex: Option<Arc<crate::apex_orchestrator::ApexOrchestrator>>,
        activity_log: Arc<std::sync::Mutex<Vec<String>>>,
    ) -> Self {
        Self {
            root_path: Arc::new(tokio::sync::Mutex::new(root_path)),
            app_handle: Arc::new(tokio::sync::Mutex::new(None)),
            browser_state,
            git_manager,
            mcp_registry,
            memory_store,
            knowledge_distiller,
            patch_engine,
            ghost_runtime,
            shadow_workspace,
            apex: Arc::new(tokio::sync::Mutex::new(apex)),
            vector_indexer: Arc::new(tokio::sync::Mutex::new(None)),
            activity_log,
        }
    }

    pub async fn set_app_handle(&self, handle: tauri::AppHandle) {
        let mut h = self.app_handle.lock().await;
        *h = Some(handle);
    }

    pub(crate) async fn load_keys_value(&self) -> Value {
        if let Some(handle) = self.app_handle.lock().await.clone() {
            if let Some(state) = handle.try_state::<crate::EditorState>() {
                let path = state.config_dir.join("api_keys.json");
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(v) = serde_json::from_str(&content) {
                        return v;
                    }
                }
            }
        }
        json!({})
    }

    pub(crate) async fn config_dir(&self) -> Option<std::path::PathBuf> {
        let handle = self.app_handle.lock().await.clone()?;
        let state: tauri::State<crate::EditorState> = handle.state();
        Some(state.config_dir.clone())
    }

    /// Backend SaaS gate — complements frontend checks in `agent.ts`.
    pub(crate) async fn gate_tool_entitlement(&self, tool: &str) -> Result<()> {
        let Some(dir) = self.config_dir().await else { return Ok(()); };
        match tool {
            "aim_pack_context" | "aim_query_spans" => {
                crate::account::require_feature_at(&dir, "neural_vfs").map_err(|e| anyhow!(e))
            }
            "reverse_shell_generate" | "security_listener_generate" | "csp_bypass_analyze"
            | "shellcode_recipe_generate" | "payload_encode"
            | "ai_vuln_hunt" | "web_security_audit" | "deep_security_audit" | "weaponize_env"
            | "network_port_scanner" | "binary_mach_o_scanner" | "generate_0day_exploit"
            | "apex_red_team_scan" | "apex_scan_url" | "apex_simulate_attack"
            | "apex_full_sweep" | "apex_pentest_report" | "apex_quick_check" => {
                crate::account::require_security_suite(&dir).map_err(|e| anyhow!(e))
            }
            "apex_architect_design" | "apex_threat_anticipate" | "apex_perf_optimize"
            | "apex_self_improve" | "apex_security_explain" | "apex_predict_failures"
            | "spawn_subagent" | "browser_subagent" => {
                crate::account::require_feature_at(&dir, "agentic").map_err(|e| anyhow!(e))
            }
            _ => Ok(()),
        }
    }

    pub async fn set_apex(&self, apex: Arc<crate::apex_orchestrator::ApexOrchestrator>) {
        let mut a = self.apex.lock().await;
        *a = Some(apex);
    }

    pub async fn set_vector_indexer(&self, indexer: Arc<crate::vector_indexer::VectorIndexer>) {
        let mut v = self.vector_indexer.lock().await;
        *v = Some(indexer);
    }

    pub async fn set_root_path(&self, root_path: PathBuf) {
        let mut r = self.root_path.lock().await;
        *r = root_path;
    }

    /// Emit `agent_editing_file` Tauri event so the frontend editor can show
    /// an animated "agent hands" cursor at the file being written.
    pub(crate) fn emit_agent_editing(&self, path: &str) {
        if let Ok(guard) = self.app_handle.try_lock() {
            if let Some(handle) = guard.as_ref() {
                let _ = handle.emit("agent_editing_file", serde_json::json!({ "path": path }));
            }
        }
    }

    pub fn get_root_path(&self) -> PathBuf {
        // Use try_lock() instead of blocking_lock() — blocking_lock() PANICS
        // when called from within a tokio async runtime (which is always the case
        // since autonomous_loop is async). try_lock() is safe in all contexts.
        self.root_path
            .try_lock()
            .map(|guard| guard.clone())
            .unwrap_or_else(|_| {
                // Lock briefly contended — fall back to cwd. This is safe and
                // won't crash the process.
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            })
    }

    pub async fn get_root_path_async(&self) -> PathBuf {
        self.root_path.lock().await.clone()
    }

    /// Resolve a file path argument against the project root, with path traversal protection.
    pub(crate) async fn resolve_path(&self, args: &Value, key: &str) -> Result<(PathBuf, PathBuf)> {
        let path_str = args.get(key)
            .or_else(|| args.get("path"))
            .or_else(|| args.get("TargetFile"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing '{}'", key))?;
        let root = self.root_path.lock().await.clone();
        let full = self.validate_path(&root, path_str)?;
        Ok((root, full))
    }

    /// Emit a Tauri event if app handle is available. Fire-and-forget.
    pub(crate) fn emit_tool_event(&self, event: &str, payload: Value) {
        if let Ok(guard) = self.app_handle.try_lock() {
            if let Some(handle) = guard.as_ref() {
                let _ = handle.emit(event, payload);
            }
        }
    }
}
