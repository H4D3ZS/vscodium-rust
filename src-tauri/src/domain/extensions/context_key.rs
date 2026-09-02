use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ContextValue {
    Bool(bool),
    String(String),
    Int(i32),
}

pub struct ContextKeyRegistry {
    keys: Mutex<HashMap<String, ContextValue>>,
}

impl ContextKeyRegistry {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        // Initial defaults
        keys.insert("editorFocus".to_string(), ContextValue::Bool(false));
        keys.insert("isReadOnly".to_string(), ContextValue::Bool(false));
        
        Self {
            keys: Mutex::new(keys),
        }
    }

    pub fn set(&self, key: String, value: ContextValue) {
        let mut keys = self.keys.lock().unwrap();
        keys.insert(key, value);
    }

    // Removed unused get

    /// Evaluates a simple "when" clause.
    /// Simplified for now: only supports single keys or "!" prefix for negation.
    /// Future: Add full boolean algebraic parser (AND/OR).
    pub fn evaluate(&self, clause: &str) -> bool {
        let keys = self.keys.lock().unwrap();
        
        if clause.starts_with('!') {
            let key = &clause[1..];
            match keys.get(key) {
                Some(ContextValue::Bool(b)) => !*b,
                None => true, // !undefined is true
                _ => false,
            }
        } else {
            match keys.get(clause) {
                Some(ContextValue::Bool(b)) => *b,
                _ => false,
            }
        }
    }
}
