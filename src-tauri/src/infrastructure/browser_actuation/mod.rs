// Register and export ClaudeBridge, GeminiBridge, and ChatGPTBridge state in the Tauri builder.
pub mod claude_bridge;
pub mod gemini_bridge;
pub mod chatgpt_bridge;

pub use claude_bridge::ClaudeBridge;
pub use gemini_bridge::GeminiBridge;
pub use chatgpt_bridge::ChatGPTBridge;

// Remove the #[cfg(target_devel)] and manager initialization to avoid unused managers and cfg warnings
// If you need to initialize bridges on startup, you can do it elsewhere or keep a simple init.
// For now, we keep it simple and just export the bridges.
// If you want to keep warm-up, you can add a function that initializes the bridges via State.
// But for simplicity, we remove the manager-related code.
