use crate::state::EditorState;
use std::path::PathBuf;
use std::sync::Arc;

/// Unified facade for backend services, usable by both Tauri and native (gpui) shells
/// without coupling to `tauri::AppHandle`.
#[derive(Clone)]
pub struct AppServices {
    pub state: Arc<EditorState>,
}

impl AppServices {
    /// Initialize services in headless mode (for gpui native shell, CLI, or test runners)
    #[cfg(not(feature = "tauri"))]
    pub fn init(config_dir: PathBuf) -> Self {
        let state = Arc::new(EditorState::new_headless(config_dir));
        state.wire_back_refs();
        Self { state }
    }

    /// Initialize services with an existing EditorState
    pub fn from_state(state: Arc<EditorState>) -> Self {
        Self { state }
    }

    /// Access the underlying `Arc<EditorState>`
    #[inline]
    pub fn state(&self) -> &Arc<EditorState> {
        &self.state
    }
}

impl std::ops::Deref for AppServices {
    type Target = EditorState;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}
