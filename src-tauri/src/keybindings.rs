
// Removed unused HashMap
use serde::{Serialize, Deserialize};
use crate::context_key::ContextKeyRegistry;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Keybinding {
    pub key: String,
    pub command: String,
    pub when: Option<String>,
}

pub struct KeybindingRegistry {
    bindings: Vec<Keybinding>,
}

impl KeybindingRegistry {
    pub fn new() -> Self {
        // Initial default bindings for parity
        let mut bindings = Vec::new();
        bindings.push(Keybinding {
            key: "ctrl+b".to_string(),
            command: "workbench.action.toggleSidebar".to_string(),
            when: None,
        });
        bindings.push(Keybinding {
            key: "ctrl+s".to_string(),
            command: "editor.action.save".to_string(),
            when: Some("editorFocus".to_string()),
        });
        
        Self { bindings }
    }

    pub fn resolve_key(&self, key_combo: &str, context: &ContextKeyRegistry) -> Option<String> {
        // Find the most specific binding that matches the key and context
        // Higher index overrides lower index (last binding wins)
        for binding in self.bindings.iter().rev() {
            if binding.key.to_lowercase() == key_combo.to_lowercase() {
                match &binding.when {
                    Some(when_clause) => {
                        if context.evaluate(when_clause) {
                            return Some(binding.command.clone());
                        }
                    }
                    None => return Some(binding.command.clone()),
                }
            }
        }
        None
    }

    // Removed unused add_binding
}
