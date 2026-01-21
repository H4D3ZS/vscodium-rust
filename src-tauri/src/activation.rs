use std::sync::{Arc, Mutex};
use crate::extension_host::ExtensionHostManager;

pub struct ActivationManager {
    // We might keep a cache of which extensions are activated here
    activated_extensions: Vec<String>,
}

impl ActivationManager {
    pub fn new() -> Self {
        Self {
            activated_extensions: Vec::new(),
        }
    }

    pub fn check_activation_requests(
        &mut self,
        event: &str,
        ext_host: Arc<Mutex<ExtensionHostManager>>,
    ) {
        // In a real implementation, we would query the ExtensionHostManager
        // for extensions that match the 'event' in their activationEvents.
        // For now, we'll do a simple scan.
        
        let extensions_to_activate = {
            let host = ext_host.lock().unwrap();
            let mut to_activate = Vec::new();
            
            for ext in &host.extensions {
                if self.activated_extensions.contains(&ext.id) {
                    continue;
                }

                for event_pattern in &ext.activation_events {
                    if self.matches_activation_event(event_pattern, event) {
                        println!("Activating extension {} due to event {}", ext.id, event);
                        to_activate.push(ext.id.clone());
                        break;
                    }
                }
            }
            to_activate
        };

        let mut host = ext_host.lock().unwrap();
        for id in extensions_to_activate {
            if let Err(e) = host.send_message(format!(
                r#"{{"type": "activateExtension", "id": "{}"}}"#,
                id
            )) {
                eprintln!("Failed to activate extension {}: {}", id, e);
            } else {
                self.activated_extensions.push(id);
            }
        }
    }

    fn matches_activation_event(&self, pattern: &str, event: &str) -> bool {
         if pattern == "*" {
             return true;
         }
         // Simple exact match or prefix match (e.g. onLanguage:rust)
         if pattern == event {
             return true;
         }
         // TODO: Implement more complex glob matching
         false
    }
}
