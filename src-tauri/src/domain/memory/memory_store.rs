        use crate::ai_engine::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

/// Find the end of the JSON header in an `.aim` file (brace-balanced).
fn aim_header_end(bytes: &[u8]) -> usize {
    let mut header_end = 0;
    let mut depth = 0;
    let mut in_string = false;
    let mut escaped = false;
    for (i, &b) in bytes.iter().enumerate() {
        if escaped {
            escaped = false;
            continue;
        }
        match b {
            b'\\' => escaped = true,
            b'"' => in_string = !in_string,
            b'{' if !in_string => depth += 1,
            b'}' if !in_string => {
                depth -= 1;
                if depth == 0 {
                    header_end = i + 1;
                    break;
                }
            }
            _ => {}
        }
    }
    header_end
}

fn chat_content_preview(content: &Value) -> String {
    if let Some(s) = content.as_str() {
        return s.chars().take(140).collect();
    }
    if let Some(obj) = content.as_object() {
        if let Some(t) = obj
            .get("Text")
            .or_else(|| obj.get("text"))
            .and_then(|v| v.as_str())
        {
            return t.chars().take(140).collect();
        }
    }
    if let Some(arr) = content.as_array() {
        let mut out = String::new();
        for part in arr {
            if let Some(t) = part
                .get("text")
                .or_else(|| part.get("Text"))
                .and_then(|v| v.as_str())
            {
                out.push_str(t);
            }
        }
        return out.chars().take(140).collect();
    }
    String::new()
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticSlot {
    pub id: String,
    pub category: String, // e.g., "code", "lesson", "doc", "task", "decision"
    pub content: String,
    pub tags: Vec<String>,
    pub metadata: Option<Value>,
    pub timestamp: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SymbolDefinition {
    pub name: String,
    pub path: String,
    pub kind: String, // e.g., "function", "struct", "trait"
    pub line_range: (usize, usize),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SymbolGraph {
    pub definitions: Vec<SymbolDefinition>,
    pub relations: Vec<(String, String, String)>, // (subject_id, predicate, object_id)
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
struct KortexSnapshot {
    slots: Vec<SemanticSlot>,
    entities: HashMap<String, Vec<String>>,
    session_messages: Vec<ChatMessage>,
    #[serde(default)]
    symbol_graph: SymbolGraph,
    #[serde(default)]
    pub project_tree: Vec<String>,
    #[serde(default)]
    pub project_metadata: HashMap<String, Value>,
}

pub struct MemoryStore {
    pub messages: Arc<RwLock<Vec<ChatMessage>>>, // Made pub for bridge sync
    pub slots: Arc<RwLock<Vec<SemanticSlot>>>,
    entities: Arc<RwLock<HashMap<String, Vec<String>>>>,
    symbol_graph: Arc<RwLock<SymbolGraph>>,
    aim_path: Arc<RwLock<Option<PathBuf>>>,
    binary_body: Arc<RwLock<Vec<u8>>>, // Cache the binary suffix of the .aim file
    binary_header_raw: Arc<RwLock<Value>>, // Cache the non-kortex parts of the header
    editor_state: Arc<std::sync::RwLock<std::sync::Weak<crate::EditorState>>>,
    pub is_dirty: Arc<AtomicBool>, // Made pub for bridge sync
    vfs_bridge: Arc<RwLock<Option<crate::vfs_bridge::VfsBridge>>>,
    events: Arc<tokio::sync::Mutex<Vec<Value>>>,
    vfs_cache: Arc<RwLock<HashMap<PathBuf, (String, std::time::SystemTime)>>>,
    pub project_tree: Arc<RwLock<Vec<String>>>,
    pub project_metadata: Arc<RwLock<HashMap<String, Value>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        let is_dirty = Arc::new(AtomicBool::new(false));
        
        let store = Self {
            messages: Arc::new(RwLock::new(Vec::new())),
            slots: Arc::new(RwLock::new(Vec::new())),
            entities: Arc::new(RwLock::new(HashMap::new())),
            symbol_graph: Arc::new(RwLock::new(SymbolGraph::default())),
            aim_path: Arc::new(RwLock::new(None)),
            binary_body: Arc::new(RwLock::new(Vec::new())),
            binary_header_raw: Arc::new(RwLock::new(json!({}))),
            editor_state: Arc::new(std::sync::RwLock::new(std::sync::Weak::new())),
            is_dirty: is_dirty.clone(),
            vfs_bridge: Arc::new(RwLock::new(None)),
            events: Arc::new(tokio::sync::Mutex::new(Vec::new())),
            vfs_cache: Arc::new(RwLock::new(HashMap::new())),
            project_tree: Arc::new(RwLock::new(Vec::new())),
            project_metadata: Arc::new(RwLock::new(HashMap::new())),
        };

        // Spawn background persistence task (Phase 24: Emergency Performance)
        let messages = store.messages.clone();
        let slots = store.slots.clone();
        let entities = store.entities.clone();
        let aim_path = store.aim_path.clone();
        let binary_body = store.binary_body.clone();
        let header_raw = store.binary_header_raw.clone();
        let editor_state = store.editor_state.clone();
        let symbol_graph = store.symbol_graph.clone();
        let project_tree = store.project_tree.clone();
        let project_metadata = store.project_metadata.clone();
        let dirty = is_dirty.clone();
        
        // Singleton background flusher: ensures only one task ever handles disk I/O per process
        static FLUSHER_ACTIVE: AtomicBool = AtomicBool::new(false);
        if !FLUSHER_ACTIVE.swap(true, Ordering::SeqCst) {
            let flusher = async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(30)).await;
                    
                    if dirty.load(Ordering::SeqCst) {
                        let path_lock = aim_path.read().await;
                        if let Some(path) = path_lock.as_ref() {
                            let mut header = header_raw.read().await.clone();
                            let snapshot = KortexSnapshot {
                                slots: slots.read().await.clone(),
                                entities: entities.read().await.clone(),
                                session_messages: messages.read().await.clone(),
                                symbol_graph: symbol_graph.read().await.clone(),
                                project_tree: project_tree.read().await.clone(),
                                project_metadata: project_metadata.read().await.clone(),
                            };
                            header["kortex"] = json!(snapshot);
                            header["updated_at"] = json!(std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap_or_default()
                                .as_secs());

                            if let Ok(header_json) = serde_json::to_string(&header) {
                                let mut final_bytes = header_json.into_bytes();
                                let body = binary_body.read().await;
                                final_bytes.extend_from_slice(&body);
                                
                                if let Err(e) = tokio::fs::write(path, final_bytes).await {
                                    eprintln!("[Kortex-AIM] Critical Persistence Error: {}", e);
                                } else {
                                    dirty.store(false, Ordering::SeqCst);
                                    
                                    if let Some(es) = editor_state.read().ok().and_then(|w| w.upgrade()) {
                                        es.emit("memory-update", json!({
                                            "slots": snapshot.slots.len(),
                                            "entities": snapshot.entities.len(),
                                            "messages": snapshot.session_messages.len()
                                        }));
                                    }
                                }
                            }
                        }
                    }
                }
            };

            // Try to spawn on the current tokio runtime; if none exists (e.g. during
            // early EditorState::new before the Tauri runtime is fully up), fall back
            // to a dedicated std::thread with its own mini-runtime.
            match tokio::runtime::Handle::try_current() {
                Ok(handle) => { handle.spawn(flusher); }
                Err(_) => {
                    std::thread::Builder::new()
                        .name("kortex-flusher".into())
                        .spawn(move || {
                            let rt = tokio::runtime::Builder::new_current_thread()
                                .enable_all()
                                .build()
                                .expect("failed to create flusher runtime");
                            rt.block_on(flusher);
                        })
                        .ok();
                }
            }
        }

        store
    }

    pub fn set_editor_state(&self, weak: std::sync::Weak<crate::EditorState>) {
        if let Ok(mut g) = self.editor_state.write() {
            *g = weak;
        }
    }

    pub async fn emit_event(&self, event: &str, payload: Value) {
        if let Some(es) = self.editor_state.read().ok().and_then(|w| w.upgrade()) {
            es.emit(event, payload);
        }
    }

    pub async fn mount(&self, pp: Option<PathBuf>) {
        self.mount_project(pp).await;
    }

    /// Mount Kortex persistent storage directly into a .aim file.
    /// Surgical extraction of JSON header and binary tensor body.
    /// Guarded against multiple mounts to prevent OOM from loading the same data repeatedly.
    pub async fn mount_project(&self, project_path: Option<PathBuf>) {
        if let Some(pp) = project_path {
            let path = pp.join(".aim").join("memory.aim");
            // Guard: skip if already mounted the same path
            {
                let lock = self.aim_path.read().await;
                if let Some(ref existing) = *lock {
                    if *existing == path && path.exists() {
                        println!("[Kortex-AIM] Already mounted: {} — skipping duplicate load", path.display());
                        return;
                    }
                }
            }
            self.load_from_path(path).await;
        }
    }

    pub async fn load_from_path(&self, path: PathBuf) {
        {
            let mut lock = self.aim_path.write().await;
            *lock = Some(path.clone());
        }

        if path.exists() {
            // Cap .aim file read at 10MB to prevent OOM on corrupted files
            const MAX_AIM_FILE_SIZE: u64 = 10 * 1024 * 1024;
            if let Ok(meta) = std::fs::metadata(&path) {
                if meta.len() > MAX_AIM_FILE_SIZE {
                    eprintln!("[Kortex-AIM] File too large ({}MB) — skipping load to prevent OOM", meta.len() / 1_000_000);
                    return;
                }
            }
            if let Ok(bytes) = tokio::fs::read(&path).await {
                // Manually find the JSON boundary to bypass serde_json version issues
                let mut header_end = 0;
                let mut depth = 0;
                let mut in_string = false;
                let mut escaped = false;
                
                for (i, &b) in bytes.iter().enumerate() {
                    if escaped { escaped = false; continue; }
                    match b {
                        b'\\' => escaped = true,
                        b'"' => in_string = !in_string,
                        b'{' if !in_string => depth += 1,
                        b'}' if !in_string => {
                            depth -= 1;
                            if depth == 0 {
                                header_end = i + 1;
                                break;
                            }
                        }
                        _ => {}
                    }
                }

                if header_end > 0 {
                    if let Ok(mut header_json) = serde_json::from_slice::<Value>(&bytes[0..header_end]) {
                        let body_bytes = &bytes[header_end..];

                        // Store binary body for later persistence — cap at 4MB to bound RSS.
                        // Beyond 4MB the tensor data is stale; the flusher regenerates it from slots.
                        let mut body_lock = self.binary_body.write().await;
                        *body_lock = body_bytes.iter().take(4 * 1024 * 1024).cloned().collect();

                        // Extract Kortex data if present
                        if let Some(kortex_val) = header_json.get_mut("kortex") {
                            if let Ok(snapshot) = serde_json::from_value::<KortexSnapshot>(kortex_val.take()) {
                                let mut slots = self.slots.write().await;
                                *slots = snapshot.slots;
                                let mut entities = self.entities.write().await;
                                *entities = snapshot.entities;
                                let mut messages = self.messages.write().await;
                                *messages = snapshot.session_messages;
                                if let Ok(mut tree) = self.project_tree.try_write() {
                                    *tree = snapshot.project_tree;
                                }
                                if let Ok(mut meta) = self.project_metadata.try_write() {
                                    *meta = snapshot.project_metadata;
                                }
                                println!(
                                    "[Kortex-AIM] Restored: {} slots, {} messages, {} indexed files from .aim",
                                    slots.len(),
                                    messages.len(),
                                    self.project_tree.read().await.len()
                                );
                            }
                        }
                        // Store the rest of the header to avoid data loss
                        let mut raw_lock = self.binary_header_raw.write().await;
                        *raw_lock = header_json;

                        // Emit telemetry for real-time visualization
                        let slots_count = self.slots.read().await.len();
                        let entities_count = self.entities.read().await.len();
                        let messages_count = self.messages.read().await.len();
                        
                        let _ = self.emit_event("memory-update", json!({
                            "slots": slots_count,
                            "entities": entities_count,
                            "messages": messages_count
                        })).await;
                    }
                }
            }
        } else {
            println!("[Kortex-AIM] Initialized fresh .aim container at {:?}", path);
            // Ensure directory exists
            if let Some(parent) = path.parent() {
                let _ = tokio::fs::create_dir_all(parent).await;
            }
        }
    }

    /// Surgically update the .aim file: rewrite header with kortex data + existing binary body
    async fn persist(&self) {
        let path_lock = self.aim_path.read().await;
        if let Some(path) = path_lock.as_ref() {
            let mut header = self.binary_header_raw.read().await.clone();
            
            // Inject current Kortex snapshot into the header
            let snapshot = KortexSnapshot {
                slots: self.slots.read().await.clone(),
                entities: self.entities.read().await.clone(),
                session_messages: self.messages.read().await.clone(),
                symbol_graph: self.symbol_graph.read().await.clone(),
                project_tree: self.project_tree.read().await.clone(),
                project_metadata: self.project_metadata.read().await.clone(),
            };
            
            header["kortex"] = json!(snapshot);
            header["updated_at"] = json!(std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs());

            if let Ok(header_json) = serde_json::to_string(&header) {
                let mut final_bytes = header_json.into_bytes();
                let body = self.binary_body.read().await;
                final_bytes.extend_from_slice(&body);

                if let Err(e) = tokio::fs::write(path, final_bytes).await {
                    eprintln!("[Kortex-AIM] Persistence failed: {}", e);
                } else {
                    // Emit telemetry for real-time visualization
                    self.emit_event("memory-update", json!({
                        "slots": snapshot.slots.len(),
                        "entities": snapshot.entities.len(),
                        "messages": snapshot.session_messages.len()
                    })).await;
                }
            }
        }
    }

    pub async fn store_conversation(&self, messages: &[ChatMessage]) {
        let mut lock = self.messages.write().await;
        lock.clear();
        lock.extend_from_slice(messages);
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn store_message(&self, message: &ChatMessage) {
        let mut lock = self.messages.write().await;
        lock.push(message.clone());
        // Cap at 500 messages to prevent unbounded growth
        const MAX_MESSAGES: usize = 500;
        if lock.len() > MAX_MESSAGES {
            let drop = lock.len() - MAX_MESSAGES;
            lock.drain(0..drop);
        }
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn store_message_params(&self, role: String, content: String, timestamp: i64) {
        let mut lock = self.messages.write().await;
        
        // Phase 25: Enhanced Upsert Logic for Streaming
        let mut found = false;
        if let Some(last) = lock.last_mut() {
            if last.role == role && (timestamp - (last.metadata.as_ref().and_then(|m| m["timestamp"].as_i64()).unwrap_or(0)) < 1000) {
                last.content = Some(crate::ai_engine::MessageContent::Text(content.clone()));
                found = true;
            }
        }
        
        if !found {
            lock.push(ChatMessage {
                role: role.clone(),
                content: Some(crate::ai_engine::MessageContent::Text(content)),
                tool_calls: None,
                tool_call_id: None,
                metadata: Some(json!({ "timestamp": timestamp })),
            });
            // Cap at 500 messages
            const MAX_MESSAGES: usize = 500;
            if lock.len() > MAX_MESSAGES {
                let drop = lock.len() - MAX_MESSAGES;
                lock.drain(0..drop);
            }
        }
        
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn clear(&self) {
        let mut msg_lock = self.messages.write().await;
        msg_lock.clear();
        drop(msg_lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    /// Hard cap: keep at most this many slots. Evict oldest "code" slots first
    /// since they're regenerated on the next index cycle, preserving decisions/tasks.
    const MAX_SLOTS: usize = 800;

    pub async fn store_slot(&self, slot: SemanticSlot) {
        let mut lock = self.slots.write().await;
        lock.retain(|s| s.id != slot.id);
        lock.push(slot);
        // Evict when over cap: drop oldest code-category slots first, then oldest overall
        if lock.len() > Self::MAX_SLOTS {
            let excess = lock.len() - Self::MAX_SLOTS;
            // Find indices of oldest "code" slots
            let mut code_indices: Vec<usize> = lock.iter().enumerate()
                .filter(|(_, s)| s.category == "code")
                .map(|(i, _)| i)
                .collect();
            code_indices.sort_unstable_by(|a, b| b.cmp(a)); // reverse so removal is safe
            let to_remove = code_indices.into_iter().take(excess);
            for i in to_remove { lock.remove(i); }
            // If still over cap, drop the very oldest entries
            if lock.len() > Self::MAX_SLOTS {
                let extra = lock.len() - Self::MAX_SLOTS;
                lock.drain(..extra);
            }
        }
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    /// Synchronous version for high-performance bulk operations (e.g. indexing)
    pub fn store_slot_sync(&self, slot: SemanticSlot) {
        if let Ok(mut lock) = self.slots.try_write() {
            lock.retain(|s| s.id != slot.id);
            lock.push(slot);
            // Enforce cap synchronously too
            if lock.len() > Self::MAX_SLOTS {
                let excess = lock.len() - Self::MAX_SLOTS;
                // Collect code indices FIRST (ends immutable borrow), then remove in reverse
                let mut code_indices: Vec<usize> = lock.iter().enumerate()
                    .filter(|(_, s)| s.category == "code")
                    .map(|(i, _)| i)
                    .collect();
                code_indices.sort_unstable_by(|a, b| b.cmp(a)); // reverse for safe removal
                let to_remove: Vec<usize> = code_indices.into_iter().take(excess).collect();
                for i in to_remove { lock.remove(i); }
                if lock.len() > Self::MAX_SLOTS {
                    let extra = lock.len() - Self::MAX_SLOTS;
                    lock.drain(..extra);
                }
            }
            self.is_dirty.store(true, Ordering::SeqCst);
        }
    }

    pub async fn add_relationship(&self, tag: &str, id: &str) {
        const MAX_ENTITIES: usize = 2000;
        let mut lock = self.entities.write().await;
        lock.entry(tag.to_string())
            .or_default()
            .push(id.to_string());
        // Evict oldest entries if over cap
        if lock.len() > MAX_ENTITIES {
            let keys_to_remove: Vec<_> = lock.keys().take(lock.len() - MAX_ENTITIES).cloned().collect();
            for key in keys_to_remove {
                lock.remove(&key);
            }
        }
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn store_symbol(&self, symbol: SymbolDefinition) {
        const MAX_SYMBOLS: usize = 5000;
        let mut lock = self.symbol_graph.write().await;
        lock.definitions.retain(|d| !(d.name == symbol.name && d.path == symbol.path));
        lock.definitions.push(symbol);
        // Evict oldest if over cap
        if lock.definitions.len() > MAX_SYMBOLS {
            let excess = lock.definitions.len() - MAX_SYMBOLS;
            lock.definitions.drain(0..excess);
        }
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    /// Synchronous version for high-performance bulk operations (e.g. indexing)
    pub fn store_symbol_sync(&self, symbol: SymbolDefinition) {
        const MAX_SYMBOLS: usize = 5000;
        if let Ok(mut lock) = self.symbol_graph.try_write() {
            lock.definitions.retain(|d| !(d.name == symbol.name && d.path == symbol.path));
            lock.definitions.push(symbol);
            if lock.definitions.len() > MAX_SYMBOLS {
                let excess = lock.definitions.len() - MAX_SYMBOLS;
                lock.definitions.drain(0..excess);
            }
            self.is_dirty.store(true, Ordering::SeqCst);
        }
    }

    /// Search symbol definitions by name substring (case-insensitive). Returns up to `limit` results.
    pub async fn query_symbols(&self, query: &str, limit: usize) -> Vec<SymbolDefinition> {
        let q = query.to_lowercase();
        let lock = self.symbol_graph.read().await;
        lock.definitions.iter()
            .filter(|s| s.name.to_lowercase().contains(&q))
            .take(limit)
            .cloned()
            .collect()
    }

    pub async fn query_slots(&self, category: &str) -> Vec<SemanticSlot> {
        let lock = self.slots.read().await;
        lock.iter()
            .filter(|s| s.category == category)
            .cloned()
            .collect()
    }

    /// Total-recall codebase map: EVERY indexed file grouped with EVERY symbol
    /// (kind name @line). This is the "100% memory" map — not lossy top-k RAG —
    /// so the agent knows the whole codebase structure at once and can answer
    /// "where is X / what's in file Y" instantly, then pull exact lines on demand.
    ///
    /// `max_chars` bounds the output so a huge repo can't blow the context
    /// window; 0 = unbounded. Returns markdown grouped by file path.
    pub async fn build_full_codebase_map(&self, max_chars: usize) -> String {
        use std::collections::BTreeMap;
        let graph = self.symbol_graph.read().await;
        let tree = self.project_tree.read().await;

        // Group symbols by file path (BTreeMap → stable, sorted output).
        let mut by_file: BTreeMap<String, Vec<&SymbolDefinition>> = BTreeMap::new();
        for sym in graph.definitions.iter() {
            by_file.entry(sym.path.clone()).or_default().push(sym);
        }

        let file_count = by_file.len().max(tree.len());
        let sym_count = graph.definitions.len();
        let mut out = format!(
            "## CODEBASE MEMORY (AIM) — {} files, {} symbols (complete map, not RAG)\n\
             You already KNOW this codebase. Use this map directly; do not grep/list to orient.\n",
            file_count, sym_count
        );

        for (path, syms) in by_file.iter() {
            let mut line = format!("{}: ", path);
            // kind abbreviations keep it compact: fn/struct/trait/class/enum/impl
            let mut parts: Vec<String> = syms.iter()
                .map(|s| format!("{} {}@{}", s.kind, s.name, s.line_range.0))
                .collect();
            parts.sort();
            line.push_str(&parts.join(", "));
            out.push('\n');
            out.push_str(&line);
            if max_chars > 0 && out.len() >= max_chars {
                out.push_str("\n… (map truncated — call aim_query_spans for the rest)");
                break;
            }
        }

        // Files with no extracted symbols still belong in the map (config, md, …).
        if max_chars == 0 || out.len() < max_chars {
            for f in tree.iter() {
                if !by_file.contains_key(f) {
                    out.push('\n');
                    out.push_str(f);
                    if max_chars > 0 && out.len() >= max_chars { break; }
                }
            }
        }
        out
    }

    pub async fn query_by_tag(&self, tag: &str) -> Vec<SemanticSlot> {
        let lock = self.slots.read().await;
        lock.iter()
            .filter(|s| s.tags.contains(&tag.to_string()))
            .cloned()
            .collect()
    }

    pub async fn query_related_entities(&self, id: &str) -> Vec<String> {
        let lock = self.entities.read().await;
        let mut related = Vec::new();
        for (_tag, ids) in lock.iter() {
            if ids.contains(&id.to_string()) {
                related.extend(ids.clone());
            }
        }
        related.retain(|x| x != id);
        related.dedup();
        related
    }

    pub async fn set_vfs_bridge(&self, bridge: crate::vfs_bridge::VfsBridge) {
        let mut lock = self.vfs_bridge.write().await;
        *lock = Some(bridge);
    }

    pub async fn retrieve_context(&self, query: &str) -> String {
        let slots = self.slots.read().await;
        let query_lower = query.to_lowercase();
        let keywords: Vec<&str> = query_lower.split_whitespace().collect();

        let now_secs = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let mut relevant: Vec<(&SemanticSlot, f32, f32)> = slots
            .iter()
            .map(|s| {
                let content_lower = s.content.to_lowercase();
                let tags_lower: Vec<String> = s.tags.iter().map(|t| t.to_lowercase()).collect();
                let matching_keywords = keywords
                    .iter()
                    .filter(|kw| {
                        content_lower.contains(*kw)
                            || tags_lower.iter().any(|t| t.contains(*kw))
                            || s.category.to_lowercase().contains(*kw)
                    })
                    .count();

                // Confidence: keyword density vs total keywords. Kept raw because
                // the page-fault path below thresholds on it (< 0.95).
                let confidence = if keywords.is_empty() { 0.0 } else { matching_keywords as f32 / keywords.len() as f32 };

                // Composite rank so memory *compounds* across sessions rather than
                // being flat keyword recall (Cursor forgets between sessions):
                //   score = confidence × category_weight × recency_decay
                // Durable knowledge (fix lessons, decisions, agent memory) is worth
                // surfacing even when slightly less keyword-dense; stale entries
                // decay toward — but never below — half weight, so old lessons still
                // count.
                let cat = s.category.to_lowercase();
                let category_weight = if cat.contains("lesson") || cat.contains("knowledge")
                    || cat.contains("decision") || cat.contains("memory") || cat.contains("brief") {
                    1.5
                } else if cat.contains("file_map") {
                    0.9
                } else {
                    1.0
                };
                // Half-life ~14 days. recency ∈ [0.5, 1.0].
                const HALF_LIFE_SECS: f32 = 14.0 * 24.0 * 3600.0;
                let age = now_secs.saturating_sub(s.timestamp) as f32;
                let recency = 0.5 + 0.5 * (2.0f32).powf(-age / HALF_LIFE_SECS);

                let score = confidence * category_weight * recency;
                (s, confidence, score)
            })
            .filter(|(_, confidence, _)| *confidence > 0.0)
            .collect();

        // Rank by composite score; keep raw confidence for the page-fault decision.
        relevant.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal));
        let top: Vec<_> = relevant.iter().take(5).collect();

        // Check for Page-Fault (Confidence < 0.95)
        let mut final_context = String::new();
        let vfs_lock = self.vfs_bridge.read().await;
        for (slot, confidence, _score) in top {
            let mut content = slot.content.clone();
            let metadata = slot.metadata.as_ref().cloned().unwrap_or(json!({}));

            if *confidence < 0.95 {
                // Page-Fault trigger: Check cache first, then bridge
                let mut cached_result = None;
                {
                    let cache_lock = self.vfs_cache.read().await;
                    if let Some(path_str) = metadata.get("path").and_then(|v| v.as_str()) {
                        let path_pb = PathBuf::from(path_str);
                        if let Some((cached_content, _mtime)) = cache_lock.get(&path_pb) {
                            println!("[PAGE-FAULT] Resolved via VFS-CACHE: {}", path_str);
                            cached_result = Some(cached_content.clone());
                        }
                    }
                }
                
                if let Some(res) = cached_result {
                    content = format!("(L2 VERBATIM SOURCE RESOLVED VIA VFS-CACHE)\n{}", res);
                } else if let (Some(bridge), Some(path_str)) = (vfs_lock.as_ref(), metadata.get("path").and_then(|v| v.as_str())) {
                    println!("[PAGE-FAULT] Low confidence ({:.2}) for {}. Fetching L2 verbatim source...", confidence, path_str);
                    if let Ok(raw_source) = bridge.fetch_raw(std::path::Path::new(path_str)) {
                        content = format!("(L2 VERBATIM SOURCE RESOLVED VIA PAGE-FAULT)\n{}", raw_source);
                        // Populate cache for future hits (respects size/entry caps)
                        self.update_vfs_cache(PathBuf::from(path_str), raw_source).await;
                    }
                }
            }

            final_context.push_str(&format!(
                "\n[{}] {}: {}\n",
                slot.category,
                slot.tags.join(", "),
                if content.len() > 800 { format!("{}...", &content[..800]) } else { content }
            ));
        }

        // Emit telemetry for real-time visualization of context retrieval
        let active_ids: Vec<String> = relevant.iter().take(5).map(|(s, _, _)| s.id.clone()).collect();
        if !active_ids.is_empty() {
            self.emit_event("context-active", json!({
                "ids": active_ids,
                "query": query
            })).await;
        }

        final_context
    }

    /// Verify an AI-generated claim against stored memory and source code.
    /// Returns supporting/contradicting evidence with confidence scores.
    pub async fn verify_claim(&self, claim: &str) -> Value {
        use serde_json::json;

        // Extract keywords from claim (split on whitespace/punctuation)
        let keywords: Vec<&str> = claim.split(|c: char| c.is_whitespace() || c == ',' || c == '.' || c == ';' || c == ':' || c == '!' || c == '?')
            .filter(|s| s.len() > 2)
            .collect();

        if keywords.is_empty() {
            return json!({
                "claim": claim,
                "supported": false,
                "confidence": 0.0,
                "evidence": [],
                "reason": "Claim too short to verify"
            });
        }

        // 1. Query memory slots using retrieve_context
        let context = self.retrieve_context(claim).await;

        // 2. Query symbols for matching definitions
        let symbols = self.query_symbols(claim, 10).await;

        // 3. Build evidence list
        let mut evidence: Vec<Value> = Vec::new();

        // Check context for keyword matches
        let ctx_lower = context.to_lowercase();
        let mut matching_keywords = 0;
        for kw in &keywords {
            if ctx_lower.contains(kw) {
                matching_keywords += 1;
                // Find a snippet around the match
                if let Some(pos) = ctx_lower.find(kw) {
                    let start = pos.saturating_sub(40);
                    let end = (pos + kw.len() + 40).min(context.len());
                    evidence.push(json!({
                        "source": "memory_store",
                        "keyword": kw,
                        "snippet": &context[start..end],
                        "type": "keyword_match"
                    }));
                }
            }
        }

        // Add symbol matches as evidence
        for sym in &symbols {
            evidence.push(json!({
                "source": "symbol_index",
                "name": sym.name,
                "kind": sym.kind,
                "path": sym.path,
                "lines": format!("{}-{}", sym.line_range.0, sym.line_range.1),
                "type": "symbol_match"
            }));
        }

        // Calculate confidence
        let keyword_confidence = if keywords.is_empty() { 0.0 } else { matching_keywords as f32 / keywords.len() as f32 };
        let symbol_confidence = (symbols.len() as f32).min(5.0) / 5.0 * 0.3; // up to 0.3 bonus
        let confidence = (keyword_confidence * 0.7 + symbol_confidence).min(1.0);

        json!({
            "claim": claim,
            "supported": confidence > 0.3,
            "confidence": (confidence * 100.0).round() / 100.0,
            "evidence": evidence,
            "keyword_match_ratio": keyword_confidence,
            "symbol_matches": symbols.len()
        })
    }

    pub async fn get_messages(&self) -> Vec<ChatMessage> {
        let lock = self.messages.read().await;
        lock.clone()
    }

    pub async fn get_all_slots(&self) -> Vec<SemanticSlot> {
        let lock = self.slots.read().await;
        lock.clone()
    }

    pub async fn get_knowledge_summary(&self) -> String {
        let slots = self.slots.read().await;
        if slots.is_empty() {
            return String::new();
        }

        let mut summary = String::from("\n### KORTEX PERSISTENT MEMORY (LOADED FROM .AIM):\n");
        let mut categories: HashMap<String, Vec<&SemanticSlot>> = HashMap::new();
        for slot in slots.iter() {
            categories.entry(slot.category.clone()).or_default().push(slot);
        }

        for (cat, items) in &categories {
            summary.push_str(&format!("\n#### {} ({} items):\n", cat, items.len()));
            for item in items.iter().rev().take(5) {
                let preview = &item.content[..item.content.len().min(150)];
                summary.push_str(&format!("- [{}] {}\n", item.tags.join(","), preview));
            }
        }
        summary
    }

    /// Compact brain gist — fits in ~100 tokens. Used as the "1 Gist Token" context injection.
    /// Replaces verbose knowledge_summary for small models and Phase-Wrap context resets.
    pub async fn build_compact_gist(&self) -> String {
        let slots = self.slots.read().await;
        if slots.is_empty() {
            return String::new();
        }

        let total = slots.len();
        // Prioritize: phase_wrap > decision > code > task — last 3 of each
        let mut lines: Vec<String> = Vec::new();

        let priority_cats = ["phase_wrap", "decision", "code", "fix", "task"];
        for cat in &priority_cats {
            let cat_slots: Vec<&SemanticSlot> = slots.iter()
                .filter(|s| s.category == *cat)
                .rev()
                .take(2)
                .collect();
            for s in cat_slots {
                let snippet = s.content.chars().take(80).collect::<String>().replace('\n', " ");
                lines.push(format!("[{}] {}", cat, snippet));
            }
        }
        // Fill remaining budget with any other recent slots
        for s in slots.iter().rev().take(4) {
            if !priority_cats.contains(&s.category.as_str()) {
                let snippet = s.content.chars().take(60).collect::<String>().replace('\n', " ");
                lines.push(format!("[{}] {}", s.category, snippet));
            }
        }

        if lines.is_empty() {
            return String::new();
        }
        format!("[KORTEX:{} slots]\n{}", total, lines.join("\n"))
    }

    /// Token-efficient, query-relevant AIM span pack for CLOUD context injection.
    /// Scores indexed slots by keyword overlap with the query (id/tag matches weigh
    /// more than content) and returns a compact `file:line [kind] preview` list — the
    /// AIM brain compressed to only what the current task needs, instead of the full
    /// codebase map. Empty string when nothing relevant (caller falls back to a
    /// structure summary). Restores AIM's token-saving on metered cloud APIs.
    pub async fn query_relevant_spans(&self, query: &str, limit: usize, preview_chars: usize) -> String {
        let keywords: Vec<String> = query
            .to_lowercase()
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .filter(|w| w.len() > 3)
            .map(|w| w.to_string())
            .collect();
        if keywords.is_empty() {
            return String::new();
        }
        let slots = self.slots.read().await;
        let mut scored: Vec<(usize, String)> = Vec::new();
        for slot in slots.iter() {
            let id_l = slot.id.to_lowercase();
            let tags_l = slot.tags.join(" ").to_lowercase();
            let content_l = slot.content.to_lowercase();
            let score: usize = keywords.iter().map(|k| {
                if id_l.contains(k) { 3 }
                else if tags_l.contains(k) { 2 }
                else if content_l.contains(k) { 1 }
                else { 0 }
            }).sum();
            if score == 0 {
                continue;
            }
            let file = slot.metadata.as_ref().and_then(|m| m.get("path")).and_then(|v| v.as_str()).unwrap_or("");
            let line = slot.metadata.as_ref().and_then(|m| m.get("line")).and_then(|v| v.as_u64()).unwrap_or(0);
            let kind = slot.tags.iter().find(|t| t.starts_with("symbol:")).map(|t| t[7..].to_string())
                .unwrap_or_else(|| slot.category.clone());
            let loc = if line > 0 { format!("{}:{}", file, line) } else { file.to_string() };
            let entry = if preview_chars == 0 {
                format!("- {} [{}]", loc, kind)
            } else {
                let preview = slot.content.chars().take(preview_chars).collect::<String>().replace('\n', " ");
                format!("- {} [{}] {}", loc, kind, preview)
            };
            scored.push((score, entry));
        }
        scored.sort_by(|a, b| b.0.cmp(&a.0));
        scored.into_iter().take(limit).map(|(_, s)| s).collect::<Vec<_>>().join("\n")
    }

    /// Store a Phase-Wrap outcome — called automatically after every context compression cycle.
    pub async fn store_phase_outcome(&self, iteration: u32, summary: String, files_written: Vec<String>) {
        let tags = {
            let mut t = vec![format!("iter_{}", iteration)];
            for f in &files_written {
                t.push(f.split('/').last().unwrap_or(f).to_string());
            }
            t
        };
        self.store_slot(SemanticSlot {
            id: uuid::Uuid::new_v4().to_string(),
            category: "phase_wrap".to_string(),
            content: summary,
            tags,
            metadata: Some(serde_json::json!({ "files": files_written, "iteration": iteration })),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }).await;
    }

    /// Auto-save a code knowledge brief after a successful file write/patch.
    pub async fn auto_learn_from_write(&self, file_path: &str, operation: &str) {
        let brief = format!("{} modified via {} — pattern recorded", file_path, operation);
        self.store_slot(SemanticSlot {
            id: uuid::Uuid::new_v4().to_string(),
            category: "code".to_string(),
            content: brief,
            tags: vec![
                operation.to_string(),
                file_path.split('/').last().unwrap_or(file_path).to_string(),
            ],
            metadata: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }).await;
    }

    pub async fn get_brain_telemetry(&self) -> Value {
        let slots = self.slots.read().await;
        let messages = self.messages.read().await;
        let entities = self.entities.read().await;
        
        // Structured telemetry for the Summary View
        json!({
            "slot_count": slots.len(),
            "message_count": messages.len(),
            "entity_count": entities.len(),
            "last_slots": slots.iter().rev().take(10).cloned().collect::<Vec<_>>(),
            "categories": slots.iter().fold(HashMap::new(), |mut acc, s| {
                *acc.entry(s.category.clone()).or_insert(0) += 1;
                acc
            }),
            "recent_context": messages.iter().rev().take(5).cloned().collect::<Vec<_>>()
        })
    }

    pub async fn list_sessions(&self) -> Vec<Value> {
        let mut sessions = Vec::new();
        let path_lock = self.aim_path.read().await;
        if let Some(current_path) = path_lock.as_ref() {
            if let Some(parent) = current_path.parent() {
                if let Ok(entries) = std::fs::read_dir(parent) {
                    for entry in entries.filter_map(|e| e.ok()) {
                        let path = entry.path();
                        let fname = path
                            .file_name()
                            .and_then(|s| s.to_str())
                            .unwrap_or("");
                        // Live workspace memory — not an archived conversation.
                        if fname == "memory.aim" {
                            continue;
                        }
                        if path.extension().and_then(|s| s.to_str()) != Some("aim") {
                            continue;
                        }
                        if let Ok(bytes) = std::fs::read(&path) {
                            let header_end = aim_header_end(&bytes);
                            if header_end > 0 {
                                if let Ok(header) =
                                    serde_json::from_slice::<Value>(&bytes[0..header_end])
                                {
                                    let updated_at = header
                                        .get("updated_at")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0);
                                    let session_messages = header
                                        .get("kortex")
                                        .and_then(|k| k.get("session_messages"))
                                        .and_then(|m| m.as_array());
                                    let message_count =
                                        session_messages.map(|a| a.len()).unwrap_or(0);
                                    if message_count == 0 {
                                        continue;
                                    }
                                    let (title, preview) = session_messages
                                        .and_then(|msgs| {
                                            msgs.iter()
                                                .find(|m| {
                                                    m.get("role")
                                                        .and_then(|r| r.as_str())
                                                        == Some("user")
                                                })
                                                .map(|m| {
                                                    let p = chat_content_preview(
                                                        m.get("content").unwrap_or(&Value::Null),
                                                    );
                                                    let t = p.chars().take(48).collect::<String>();
                                                    (t, p)
                                                })
                                        })
                                        .unwrap_or_else(|| {
                                            (
                                                fname.replace("session_", "Chat "),
                                                String::new(),
                                            )
                                        });

                                    sessions.push(json!({
                                        "name": fname,
                                        "title": title,
                                        "preview": preview,
                                        "path": path.to_string_lossy(),
                                        "updated_at": updated_at,
                                        "messages": message_count
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
        sessions.sort_by(|a, b| {
            let a_val = a["updated_at"].as_u64().unwrap_or(0);
            let b_val = b["updated_at"].as_u64().unwrap_or(0);
            b_val.cmp(&a_val)
        });

        // Live workspace conversation (memory.aim) — shown in History until archived.
        let live_messages = self.messages.read().await;
        if !live_messages.is_empty() {
            if let Some(current_path) = path_lock.as_ref() {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let (title, preview) = live_messages
                    .iter()
                    .find(|m| m.role == "user")
                    .map(|m| {
                        let content_val = match &m.content {
                            Some(c) => serde_json::to_value(c).unwrap_or(Value::Null),
                            None => Value::Null,
                        };
                        let p = chat_content_preview(&content_val);
                        let t = if p.is_empty() {
                            "Current conversation".to_string()
                        } else {
                            p.chars().take(48).collect::<String>()
                        };
                        (t, p)
                    })
                    .unwrap_or_else(|| ("Current conversation".to_string(), String::new()));

                sessions.insert(
                    0,
                    json!({
                        "name": "memory.aim",
                        "title": title,
                        "preview": preview,
                        "path": current_path.to_string_lossy(),
                        "updated_at": now,
                        "messages": live_messages.len(),
                        "is_current": true
                    }),
                );
            }
        }

        sessions
    }

    /// Read `session_messages` from an `.aim` archive without repointing `aim_path`.
    async fn read_session_messages_from_file(path: &PathBuf) -> Vec<ChatMessage> {
        if !path.exists() {
            return Vec::new();
        }
        let Ok(bytes) = tokio::fs::read(path).await else {
            return Vec::new();
        };
        let header_end = aim_header_end(&bytes);
        if header_end == 0 {
            return Vec::new();
        }
        let Ok(header) = serde_json::from_slice::<Value>(&bytes[0..header_end]) else {
            return Vec::new();
        };
        let Some(msgs_val) = header
            .get("kortex")
            .and_then(|k| k.get("session_messages"))
        else {
            return Vec::new();
        };
        serde_json::from_value(msgs_val.clone()).unwrap_or_default()
    }

    /// Restore a saved conversation into the live workspace memory (`memory.aim`)
    /// so the chat panel AND the next `ai_chat` turn share the same context.
    pub async fn restore_session_from_path(&self, path: PathBuf) -> Vec<ChatMessage> {
        let messages = Self::read_session_messages_from_file(&path).await;
        if messages.is_empty() {
            return messages;
        }
        {
            let mut lock = self.messages.write().await;
            *lock = messages.clone();
        }
        self.is_dirty.store(true, Ordering::SeqCst);
        self.persist().await;
        self.is_dirty.store(false, Ordering::SeqCst);
        messages
    }

    pub async fn flush_to_disk(&self) {
        if self.is_dirty.load(Ordering::SeqCst) {
            self.persist().await;
            self.is_dirty.store(false, Ordering::SeqCst);
        }
    }

    pub async fn archive_current_session(&self) {
        let path_lock = self.aim_path.read().await;
        if let Some(current_path) = path_lock.as_ref() {
            if let Some(parent) = current_path.parent() {
                let timestamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let archive_path = parent.join(format!("session_{}.aim", timestamp));
                
                let mut header = self.binary_header_raw.read().await.clone();
                let snapshot = KortexSnapshot {
                    slots: self.slots.read().await.clone(),
                    entities: self.entities.read().await.clone(),
                    session_messages: self.messages.read().await.clone(),
                    symbol_graph: self.symbol_graph.read().await.clone(),
                    project_tree: self.project_tree.read().await.clone(),
                    project_metadata: self.project_metadata.read().await.clone(),
                };
                header["kortex"] = json!(snapshot);
                header["updated_at"] = json!(timestamp);

                if let Ok(header_json) = serde_json::to_string(&header) {
                    let mut final_bytes = header_json.into_bytes();
                    let body = self.binary_body.read().await;
                    final_bytes.extend_from_slice(&body);
                    let _ = tokio::fs::write(archive_path, final_bytes).await;
                }
            }
        }
    }

    pub async fn create_new_session(&self) {
        // Save current session to history before clearing
        self.archive_current_session().await;

        // Clear memory
        {
            let mut slots = self.slots.write().await;
            slots.clear();
            let mut entities = self.entities.write().await;
            entities.clear();
            let mut messages = self.messages.write().await;
            messages.clear();
            if let Ok(mut tree) = self.project_tree.try_write() {
                tree.clear();
            }
            if let Ok(mut meta) = self.project_metadata.try_write() {
                meta.clear();
            }
        }

        // Reset to default memory.aim for the new session
        let path_lock = self.aim_path.read().await.clone();
        if let Some(current_path) = path_lock {
            if let Some(parent) = current_path.parent() {
                let default_path = parent.join("memory.aim");
                let mut aim_path_write = self.aim_path.write().await;
                *aim_path_write = Some(default_path);
            }
        }

        // Persist the empty state
        self.persist().await;

        // Emit telemetry update
        let _ = self.emit_event("memory-update", json!({
            "slots": 0,
            "entities": 0,
            "messages": 0,
            "tree": 0
        })).await;
    }

    pub async fn set_project_tree(&self, tree: Vec<String>) {
        let mut lock = self.project_tree.write().await;
        *lock = tree;
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn get_project_tree(&self) -> Vec<String> {
        self.project_tree.read().await.clone()
    }

    /// Drop indexed paths that do not exist under `root` (stale cross-project .aim bleed).
    /// Returns (before_count, after_count).
    pub async fn prune_project_tree_to_workspace(&self, root: &std::path::Path) -> (usize, usize) {
        let tree = self.project_tree.read().await.clone();
        let before = tree.len();
        if before == 0 {
            return (0, 0);
        }
        let kept: Vec<String> = tree
            .into_iter()
            .filter(|p| {
                let full = if std::path::Path::new(p).is_absolute() {
                    std::path::PathBuf::from(p)
                } else {
                    root.join(p)
                };
                full.exists()
            })
            .collect();
        let after = kept.len();
        if after != before {
            let mut lock = self.project_tree.write().await;
            *lock = kept;
            self.is_dirty.store(true, Ordering::SeqCst);
        }
        (before, after)
    }

    /// Closest indexed paths when a requested file is missing (basename match + prefix).
    pub async fn suggest_similar_paths(&self, requested: &str, limit: usize) -> Vec<String> {
        let req = requested.replace('\\', "/");
        let base = std::path::Path::new(&req)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        let tree = self.project_tree.read().await.clone();
        if tree.is_empty() || base.is_empty() {
            return Vec::new();
        }
        let mut scored: Vec<(i32, String)> = tree
            .into_iter()
            .filter_map(|p| {
                let norm = p.replace('\\', "/");
                let fname = std::path::Path::new(&norm)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("")
                    .to_ascii_lowercase();
                let mut score = 0i32;
                if fname == base {
                    score += 100;
                } else if fname.contains(&base) || base.contains(&fname) {
                    score += 50;
                }
                if norm.to_ascii_lowercase().contains(&base) {
                    score += 10;
                }
                if score > 0 {
                    Some((score, p))
                } else {
                    None
                }
            })
            .collect();
        scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
        scored
            .into_iter()
            .take(limit)
            .map(|(_, p)| p)
            .collect()
    }

    /// Returns true if the workspace has never been indexed — signals that we
    /// should trigger an immediate background index cycle on first chat.
    pub async fn needs_initial_index(&self) -> bool {
        self.project_tree.read().await.is_empty()
    }

    /// Build a short human-readable project structure summary from the indexed
    /// file tree — used for the ### PROJECT STRUCTURE section in the BRAIN prompt.
    /// Groups files by top-level directory and shows counts.
    pub async fn get_project_tree_summary(&self) -> String {
        let tree = self.project_tree.read().await;
        if tree.is_empty() {
            return "(not yet indexed — run 'Index Workspace' in Kortex panel)".to_string();
        }
        // Group by top-level component
        let mut groups: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
        for path in tree.iter() {
            let top = path.split(['/', '\\']).next().unwrap_or("root").to_string();
            *groups.entry(top).or_insert(0) += 1;
        }
        let summary: Vec<String> = groups.iter()
            .map(|(dir, count)| format!("{}/({} files)", dir, count))
            .collect();
        format!("{} total files: {}", tree.len(), summary.join(", "))
    }

    pub async fn store_event(&self, event_type: &str, data: Value) -> anyhow::Result<()> {
        let mut events = self.events.lock().await;
        events.push(json!({
            "type": event_type,
            "data": data,
            "timestamp": chrono::Utc::now().timestamp()
        }));
        // Cap event log at 200 — telemetry should not pin RAM
        const MAX_EVENTS: usize = 200;
        if events.len() > MAX_EVENTS {
            let drop = events.len() - MAX_EVENTS;
            events.drain(..drop);
        }
        Ok(())
    }

    pub async fn update_vfs_cache(&self, path: PathBuf, content: String) {
        let mut lock = self.vfs_cache.write().await;
        // Cap each cached entry to 8KB and the total cache to 64 entries.
        // The full file is always re-readable from disk; this is just a hot-path cache.
        const MAX_CACHE_ENTRIES: usize = 64;
        const MAX_CACHE_BYTES: usize = 8192;
        let trimmed = if content.len() > MAX_CACHE_BYTES {
            content.chars().take(MAX_CACHE_BYTES).collect::<String>()
        } else {
            content
        };
        lock.insert(path, (trimmed, std::time::SystemTime::now()));
        if lock.len() > MAX_CACHE_ENTRIES {
            // Evict the oldest entry (lowest SystemTime)
            if let Some((oldest_key, _)) = lock.iter()
                .min_by_key(|(_, (_, t))| *t)
                .map(|(k, v)| (k.clone(), v.clone()))
            {
                lock.remove(&oldest_key);
            }
        }
    }


    pub async fn get_vfs_cache(&self, path: &PathBuf) -> Option<String> {
        let lock = self.vfs_cache.read().await;
        lock.get(path).map(|(c, _)| c.clone())
    }

    pub async fn generate_knowledge_graph(&self) -> anyhow::Result<crate::visual_lab::VisualGraph> {
        let slots = self.slots.read().await;
        Ok(crate::visual_lab::generate_neural_omni_graph(slots.clone()))
    }

}


