use crate::ai_engine::ChatMessage;
use tokio::sync::Mutex;
use std::sync::Arc;

pub struct MemoryStore {
    messages: Arc<Mutex<Vec<ChatMessage>>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub async fn store_conversation(&self, messages: &[ChatMessage]) {
        let mut lock = self.messages.lock().await;
        lock.clear();
        lock.extend_from_slice(messages);
    }

    pub async fn store_message(&self, message: &ChatMessage) {
        let mut lock = self.messages.lock().await;
        lock.push(message.clone());
    }
}
