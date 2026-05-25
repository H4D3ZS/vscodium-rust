// ... existing imports ...
use crate::browser_actuation::{ClaudeBridgeManager, GeminiBridgeManager};

// ... in the setup phase after creating EditorState ...

// Initialize bridge managers
let claude_bridge = ClaudeBridgeManager::new()?;
let gemini_bridge = GeminiBridgeManager::new()?;

// Manage them in the Tauri app state
app.manage(claude_bridge);
app.manage(gemini_bridge);

// ... rest of the code ...