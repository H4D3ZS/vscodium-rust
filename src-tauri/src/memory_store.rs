use crate::ai_engine::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;

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
}

pub struct MemoryStore {
    messages: Arc<RwLock<Vec<ChatMessage>>>,
    pub slots: Arc<RwLock<Vec<SemanticSlot>>>,
    entities: Arc<RwLock<HashMap<String, Vec<String>>>>,
    symbol_graph: Arc<RwLock<SymbolGraph>>,
    aim_path: Arc<RwLock<Option<PathBuf>>>,
    binary_body: Arc<RwLock<Vec<u8>>>, // Cache the binary suffix of the .aim file
    binary_header_raw: Arc<RwLock<Value>>, // Cache the non-kortex parts of the header
    app_handle: Arc<RwLock<Option<tauri::AppHandle>>>,
    is_dirty: Arc<AtomicBool>,
    vfs_bridge: Arc<RwLock<Option<crate::vfs_bridge::VfsBridge>>>,
    events: Arc<tokio::sync::Mutex<Vec<Value>>>,
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
            app_handle: Arc::new(RwLock::new(None)),
            is_dirty: is_dirty.clone(),
            vfs_bridge: Arc::new(RwLock::new(None)),
            events: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        };

        // Spawn background persistence task (Phase 24: Emergency Performance)
        let messages = store.messages.clone();
        let slots = store.slots.clone();
        let entities = store.entities.clone();
        let aim_path = store.aim_path.clone();
        let binary_body = store.binary_body.clone();
        let header_raw = store.binary_header_raw.clone();
        let app_handle = store.app_handle.clone();
        let symbol_graph = store.symbol_graph.clone();
        let dirty = is_dirty.clone();
        
        // Singleton background flusher: ensures only one task ever handles disk I/O per process
        static FLUSHER_ACTIVE: AtomicBool = AtomicBool::new(false);
        if !FLUSHER_ACTIVE.swap(true, Ordering::SeqCst) {
            tauri::async_runtime::spawn(async move {
                loop {
                    // Optimized Flush Interval: 10 seconds for performance balance
                    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
                    
                    if dirty.load(Ordering::SeqCst) {
                        // Silent persistence to protect logs; only error if I/O fails
                        let path_lock = aim_path.read().await;
                        if let Some(path) = path_lock.as_ref() {
                            let mut header = header_raw.read().await.clone();
                            let snapshot = KortexSnapshot {
                                slots: slots.read().await.clone(),
                                entities: entities.read().await.clone(),
                                session_messages: messages.read().await.clone(),
                                symbol_graph: symbol_graph.read().await.clone(),
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
                                
                                // Perform atomic-like write to the same .aim file (zero disk growth, one file)
                                if let Err(e) = tokio::fs::write(path, final_bytes).await {
                                    eprintln!("[Kortex-AIM] Critical Persistence Error: {}", e);
                                } else {
                                    dirty.store(false, Ordering::SeqCst);
                                    
                                    // Telemetry only: no log spam
                                    let app_lock = app_handle.read().await;
                                    if let Some(handle) = app_lock.as_ref() {
                                        use tauri::Emitter;
                                        let _ = handle.emit("memory-update", json!({
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
            });
        }

        store
    }

    pub async fn set_app_handle(&self, handle: tauri::AppHandle) {
        let mut lock = self.app_handle.write().await;
        *lock = Some(handle);
    }

    pub async fn emit_event(&self, event: &str, payload: Value) {
        let lock = self.app_handle.read().await;
        if let Some(handle) = lock.as_ref() {
            use tauri::Emitter;
            let _ = handle.emit(event, payload);
        }
    }

    pub async fn mount(&self, pp: Option<PathBuf>) {
        self.mount_project(pp).await;
    }

    /// Mount Kortex persistent storage directly into a .aim file.
    /// Surgical extraction of JSON header and binary tensor body.
    pub async fn mount_project(&self, project_path: Option<PathBuf>) {
        if let Some(pp) = project_path {
            let path = pp.join(".aim").join("memory.aim");
            self.load_from_path(path).await;
        }
    }

    pub async fn load_from_path(&self, path: PathBuf) {
        {
            let mut lock = self.aim_path.write().await;
            *lock = Some(path.clone());
        }

        if path.exists() {
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

                        // Store binary body for later persistence
                        let mut body_lock = self.binary_body.write().await;
                        *body_lock = body_bytes.to_vec();

                        // Extract Kortex data if present
                        if let Some(kortex_val) = header_json.get_mut("kortex") {
                            if let Ok(snapshot) = serde_json::from_value::<KortexSnapshot>(kortex_val.take()) {
                                let mut slots = self.slots.write().await;
                                *slots = snapshot.slots;
                                let mut entities = self.entities.write().await;
                                *entities = snapshot.entities;
                                let mut messages = self.messages.write().await;
                                *messages = snapshot.session_messages;
                                println!(
                                    "[Kortex-AIM] Restored: {} slots, {} messages from .aim",
                                    slots.len(),
                                    messages.len()
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
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn clear(&self) {
        let mut msg_lock = self.messages.write().await;
        msg_lock.clear();
        drop(msg_lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn store_slot(&self, slot: SemanticSlot) {
        let mut lock = self.slots.write().await;
        lock.retain(|s| s.id != slot.id);
        lock.push(slot);
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn add_relationship(&self, tag: &str, id: &str) {
        let mut lock = self.entities.write().await;
        lock.entry(tag.to_string())
            .or_default()
            .push(id.to_string());
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn store_symbol(&self, symbol: SymbolDefinition) {
        let mut lock = self.symbol_graph.write().await;
        lock.definitions.retain(|d| !(d.name == symbol.name && d.path == symbol.path));
        lock.definitions.push(symbol);
        drop(lock);
        self.is_dirty.store(true, Ordering::SeqCst);
    }

    pub async fn query_slots(&self, category: &str) -> Vec<SemanticSlot> {
        let lock = self.slots.read().await;
        lock.iter()
            .filter(|s| s.category == category)
            .cloned()
            .collect()
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

        let mut relevant: Vec<(&SemanticSlot, f32)> = slots
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
                
                // Confidence score based on keyword density vs total keywords
                let confidence = if keywords.is_empty() { 0.0 } else { matching_keywords as f32 / keywords.len() as f32 };
                (s, confidence)
            })
            .filter(|(_, confidence)| *confidence > 0.0)
            .collect();

        relevant.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top: Vec<_> = relevant.iter().take(5).collect();

        // Check for Page-Fault (Confidence < 0.95)
        let mut final_context = String::new();
        let vfs_lock = self.vfs_bridge.read().await;

        for (slot, confidence) in top {
            let mut content = slot.content.clone();
            
            if *confidence < 0.95 {
                // Page-Fault trigger
                if let (Some(bridge), Some(metadata)) = (vfs_lock.as_ref(), &slot.metadata) {
                    if let Some(path_str) = metadata.get("path").and_then(|v| v.as_str()) {
                        println!("[PAGE-FAULT] Low confidence ({:.2}) for {}. Fetching L2 verbatim source...", confidence, path_str);
                        if let Ok(raw_source) = bridge.fetch_raw(std::path::Path::new(path_str)) {
                            content = format!("(L2 VERBATIM SOURCE RESOLVED VIA PAGE-FAULT)\n{}", raw_source);
                        }
                    }
                }
            }

            final_context.push_str(&format!(
                "\n[{}] {}: {}\n",
                slot.category,
                slot.tags.join(", "),
                &content[..content.len().min(800)] // Increased limit for richer L2 context
            ));
        }

        // Emit telemetry for real-time visualization of context retrieval
        let active_ids: Vec<String> = relevant.iter().take(5).map(|(s, _)| s.id.clone()).collect();
        if !active_ids.is_empty() {
            self.emit_event("context-active", json!({
                "ids": active_ids,
                "query": query
            })).await;
        }

        final_context
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
                        if path.extension().and_then(|s| s.to_str()) == Some("aim") {
                            if let Ok(bytes) = std::fs::read(&path) {
                                let mut header_end = 0;
                                for (i, &b) in bytes.iter().enumerate() {
                                    if b == b'}' {
                                        header_end = i + 1;
                                        break;
                                    }
                                }
                                if header_end > 0 {
                                    if let Ok(header) = serde_json::from_slice::<Value>(&bytes[0..header_end]) {
                                        let updated_at = header.get("updated_at").and_then(|v| v.as_u64()).unwrap_or(0);
                                        let message_count = header.get("kortex")
                                            .and_then(|k| k.get("session_messages"))
                                            .and_then(|m| m.as_array())
                                            .map(|a| a.len())
                                            .unwrap_or(0);
                                        
                                        sessions.push(json!({
                                            "name": path.file_name().and_then(|s| s.to_str()).unwrap_or("unknown"),
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
        }
        sessions.sort_by(|a, b| {
            let a_val = a["updated_at"].as_u64().unwrap_or(0);
            let b_val = b["updated_at"].as_u64().unwrap_or(0);
            b_val.cmp(&a_val)
        });
        sessions
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
            "messages": 0
        })).await;
    }

    pub async fn store_event(&self, event_type: &str, data: Value) -> anyhow::Result<()> {
        let mut events = self.events.lock().await;
        events.push(json!({
            "type": event_type,
            "data": data,
            "timestamp": chrono::Utc::now().timestamp()
        }));
        Ok(())
    }
}

