// LSP Subsystem module exports and manager.

pub mod client;
pub mod detector;

pub use client::{
    path_to_uri, uri_to_path, DocumentSymbol, LspCodeAction, LspPosition, LspRange, LspTextEdit,
    NativeLspClient, SymbolKind, WorkspaceEdit,
};
pub use detector::{detect_language_servers, DetectedLsp};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;

pub struct LspManager {
    pub workspace_root: PathBuf,
    pub clients: HashMap<String, Arc<NativeLspClient>>,
    pub detected_servers: Vec<DetectedLsp>,
}

impl LspManager {
    pub fn new(workspace_root: PathBuf) -> Self {
        let detected = detect_language_servers();
        Self {
            workspace_root,
            clients: HashMap::new(),
            detected_servers: detected,
        }
    }

    /// Ensure LSP client is running for a given language
    pub fn ensure_client(&mut self, language: &str) -> Option<Arc<NativeLspClient>> {
        let lang_key = language.to_lowercase();
        if lang_key == "plaintext" || lang_key.is_empty() {
            return None;
        }
        if let Some(existing) = self.clients.get(&lang_key) {
            return Some(existing.clone());
        }

        // Try to match against detected servers
        let matched = self.detected_servers.iter().find(|s| {
            s.language.eq_ignore_ascii_case(&lang_key)
                || (lang_key == "rs" && s.language == "rust")
                || (lang_key == "py" && s.language == "python")
                || (lang_key == "ts" && s.language == "typescript")
                || (lang_key == "js" && s.language == "typescript")
        })?;

        if let Ok(client) =
            NativeLspClient::new(&matched.command, &matched.args, &self.workspace_root)
        {
            let client_arc = Arc::new(client);
            self.clients.insert(lang_key, client_arc.clone());
            Some(client_arc)
        } else {
            None
        }
    }

    pub fn client_for(&self, language: &str) -> Option<Arc<NativeLspClient>> {
        self.clients.get(&language.to_lowercase()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_manager_init() {
        let root = PathBuf::from(r"C:\test\workspace");
        let mgr = LspManager::new(root);
        // Ensure detector runs without panic
        let _ = mgr.detected_servers.len();
    }
}
