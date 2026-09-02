//! AiTools: struct, constructor, entitlement gating, shared activity helpers.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

#[derive(Clone)]
pub struct AiTools {
    pub root_path: Arc<tokio::sync::Mutex<PathBuf>>,
    /// Contention-free mirror of `root_path` for the SYNC `get_root_path()`.
    /// Without it, `try_lock()` on the tokio mutex loses the race during an
    /// active agent run and falls back to `current_dir()` (the IDE's launch dir,
    /// not the open project) — making the agent read the wrong directory.
    pub(crate) root_cache: Arc<std::sync::RwLock<PathBuf>>,
    /// Weak back-reference to the owning EditorState. Replaces the old
    /// `tauri::AppHandle`: tools read config/engine/terminal through this and
    /// emit via `EditorState::emit`, so the engine no longer depends on Tauri.
    pub(crate) editor_state: Arc<std::sync::RwLock<std::sync::Weak<crate::EditorState>>>,
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
    /// Shared YOLO mode flag — replaces the unsafe `std::env::set_var` pattern.
    /// Set by `Sentient::set_yolo_mode`, read by `ToolInvoker` for permission bypass.
    pub yolo_flag: Arc<std::sync::atomic::AtomicBool>,
    /// In-memory task store with disk persistence.
    pub task_store: Arc<tokio::sync::RwLock<super::task_store::TaskStore>>,
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
        yolo_flag: Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        Self {
            root_cache: Arc::new(std::sync::RwLock::new(root_path.clone())),
            root_path: Arc::new(tokio::sync::Mutex::new(root_path)),
            editor_state: Arc::new(std::sync::RwLock::new(std::sync::Weak::new())),
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
            yolo_flag,
            task_store: Arc::new(tokio::sync::RwLock::new(super::task_store::TaskStore::new())),
        }
    }

    pub fn set_editor_state(&self, weak: std::sync::Weak<crate::EditorState>) {
        if let Ok(mut g) = self.editor_state.write() {
            *g = weak;
        }
    }

    /// Upgrade the back-reference to the live EditorState, if still alive.
    pub(crate) fn editor_state(&self) -> Option<Arc<crate::EditorState>> {
        self.editor_state.read().ok().and_then(|w| w.upgrade())
    }

    /// Snapshot the current event sink (Send+Sync) for moving into worker
    /// threads that emit streamed output without holding the EditorState.
    pub(crate) fn sink(&self) -> Option<Arc<dyn crate::EventSink>> {
        self.editor_state()
            .and_then(|s| s.event_sink.read().ok().and_then(|g| g.clone()))
    }

    pub(crate) async fn load_keys_value(&self) -> Value {
        if let Some(state) = self.editor_state() {
            let path = state.config_dir.join("api_keys.json");
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(v) = serde_json::from_str(&content) {
                    return v;
                }
            }
        }
        json!({})
    }

    pub(crate) async fn config_dir(&self) -> Option<std::path::PathBuf> {
        self.editor_state().map(|s| s.config_dir.clone())
    }

    /// Backend gate — no entitlement checks in OSS edition.
    pub(crate) async fn gate_tool_entitlement(&self, _tool: &str) -> Result<()> {
        Ok(())
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
        if let Ok(mut c) = self.root_cache.write() { *c = root_path.clone(); }
        let mut r = self.root_path.lock().await;
        *r = root_path;
    }

    /// Emit `agent_editing_file` Tauri event so the frontend editor can show
    /// an animated "agent hands" cursor at the file being written.
    pub(crate) fn emit_agent_editing(&self, path: &str) {
        if let Some(state) = self.editor_state() {
            state.emit("agent_editing_file", serde_json::json!({ "path": path }));
        }
    }

    pub fn get_root_path(&self) -> PathBuf {
        // Read the contention-free cache, NOT a try_lock() on the tokio mutex.
        // The old code fell back to current_dir() on lock contention — which is
        // constant during an agent run — so the agent intermittently operated on
        // the IDE's launch dir instead of the open project. The cache always
        // holds the last root set via set_root_path (open folder / set_active_root).
        self.root_cache
            .read()
            .map(|g| g.clone())
            .unwrap_or_else(|_| PathBuf::from("."))
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
        if let Some(state) = self.editor_state() {
            state.emit(event, payload);
        }
    }
}
