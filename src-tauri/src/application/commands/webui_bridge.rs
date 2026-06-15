//! Tauri commands for the WebUI→MCP bridge (`infrastructure::webui_mcp_bridge`).
//!
//! Thin wrappers: read the bridge handle off `EditorState` and call into it. The bridge
//! itself runs autonomously (the extension drives tools without these commands); these
//! exist for the Mission Control status strip and for manually kicking a task onto a tab.

use crate::infrastructure::webui_mcp_bridge::TabStatus;
use crate::EditorState;

/// Connected browser-chat tabs and their health (for the status strip).
#[tauri::command]
pub async fn webui_bridge_status(
    state: tauri::State<'_, EditorState>,
) -> Result<Vec<TabStatus>, String> {
    Ok(state.webui_bridge.status().await)
}

/// Distinct providers with at least one healthy connected tab.
#[tauri::command]
pub async fn webui_bridge_providers(
    state: tauri::State<'_, EditorState>,
) -> Result<Vec<String>, String> {
    Ok(state.webui_bridge.connected_providers().await)
}

/// Inject the protocol + a task onto a specific tab to start a session manually.
/// Pass `tab_id` from `webui_bridge_status`, or omit to use any healthy tab.
#[tauri::command]
pub async fn webui_bridge_start_task(
    state: tauri::State<'_, EditorState>,
    tab_id: Option<String>,
    provider: Option<String>,
    task: String,
) -> Result<bool, String> {
    let target = match tab_id {
        Some(id) => id,
        None => state
            .webui_bridge
            .pick_tab(provider.as_deref())
            .await
            .ok_or_else(|| "no connected browser-chat tab available".to_string())?,
    };
    Ok(state.webui_bridge.start_task_on_tab(&target, &task).await)
}

/// Type arbitrary text into a tab's composer and submit (a nudge / follow-up).
#[tauri::command]
pub async fn webui_bridge_inject(
    state: tauri::State<'_, EditorState>,
    tab_id: String,
    text: String,
) -> Result<bool, String> {
    Ok(state.webui_bridge.inject(&tab_id, &text).await)
}

// ── Headless web-chat-as-model (webchat_openai_shim) ─────────────────────────

/// One-time visible login for a headless web-chat provider (`claude` | `deepseek`).
/// Opens a real Firefox window; resolves once the composer appears and the session
/// (cookies + localStorage) has been persisted for headless 24/7 reuse.
#[tauri::command]
pub async fn webchat_login(
    driver: tauri::State<'_, std::sync::Arc<crate::infrastructure::web_chat_driver::WebChatDriver>>,
    provider: String,
) -> Result<String, String> {
    driver.login(&provider).await
}

/// Which web-chat providers already have a saved (logged-in) session on disk.
#[tauri::command]
pub async fn webchat_sessions(
    driver: tauri::State<'_, std::sync::Arc<crate::infrastructure::web_chat_driver::WebChatDriver>>,
) -> Result<Vec<String>, String> {
    let mut out = Vec::new();
    for p in ["claude", "deepseek"] {
        if driver.has_session(p) {
            out.push(p.to_string());
        }
    }
    Ok(out)
}
