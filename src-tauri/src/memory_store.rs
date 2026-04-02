use crate::ai_engine::ChatMessage;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemanticSlot {
    pub id: String,
    pub category: String, // e.g., "code", "lesson", "doc"
    pub content: String,
    pub tags: Vec<String>,
    pub metadata: Option<Value>,
    pub timestamp: u64,
}

pub struct MemoryStore {
    messages: Arc<RwLock<Vec<ChatMessage>>>,
    slots: Arc<RwLock<Vec<SemanticSlot>>>,
    entities: Arc<RwLock<HashMap<String, Vec<String>>>>, // Tag -> List of related IDs
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(RwLock::new(Vec::new())),
            slots: Arc::new(RwLock::new(Vec::new())),
            entities: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn store_conversation(&self, messages: &[ChatMessage]) {
        let mut lock = self.messages.write().await;
        lock.clear();
        lock.extend_from_slice(messages);
    }

    pub async fn store_message(&self, message: &ChatMessage) {
        let mut lock = self.messages.write().await;
        lock.push(message.clone());
    }

    pub async fn clear(&self) {
        let mut msg_lock = self.messages.write().await;
        msg_lock.clear();
        let mut slot_lock = self.slots.write().await;
        slot_lock.clear();
    }

    pub async fn store_slot(&self, slot: SemanticSlot) {
        let mut lock = self.slots.write().await;
        lock.push(slot);
    }

    pub async fn add_relationship(&self, tag: &str, id: &str) {
        let mut lock = self.entities.write().await;
        lock.entry(tag.to_string())
            .or_default()
            .push(id.to_string());
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
        for (tag, ids) in lock.iter() {
            if ids.contains(&id.to_string()) {
                related.extend(ids.clone());
            }
        }
        related.retain(|x| x != id);
        related.dedup();
        related
    }

    pub async fn get_messages(&self) -> Vec<ChatMessage> {
        let lock = self.messages.read().await;
        lock.clone()
    }
}
