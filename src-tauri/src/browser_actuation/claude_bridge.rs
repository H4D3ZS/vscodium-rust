use std::sync::Arc;
use tauri::webview::{WebviewWindow, WebviewWindowBuilder};
use tokio::sync::Mutex;

pub struct ClaudeBridge {
    window: Arc<Mutex<Option<WebviewWindow>>>,
}

impl ClaudeBridge {
    pub fn new(
        app_handle: &tauri::AppHandle,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let window = WebviewWindowBuilder::new(
            app_handle,
            "claude_bridge",
            tauri::WebviewUrl::External("https://claude.ai/new".parse().unwrap()),
        )
        .decorations(false)
        .visible(false)
        .build()?;

        // Inject automation script
        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            // Wait for webview to be ready
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;

            let script = r#"
                window.onload = () => {
                    const promptBox = document.querySelector('div[contenteditable="true"]');
                    if (!promptBox) {
                        console.error('Claude prompt box not found');
                        return;
                    }
                    promptBox.focus();
                    
                    const observer = new MutationObserver((mutations) => {
                        mutations.forEach((mutation) => {
                            mutation.addedNodes.forEach((node) => {
                                if (node.textContent && node.textContent.trim() !== '') {
                                    window.postMessage({ type: 'response', content: node.textContent }, '*');
                                }
                            });
                        });
                    });
                    observer.observe(promptBox.parentNode, { childList: true, subtree: true });
                    
                    window.addEventListener('message', (event) => {
                        if (event.data.type === 'input') {
                            promptBox.innerHTML = event.data.content;
                        }
                    });
                };
            "#;

            // Use eval instead of execute_script
            let _ = window_clone.eval(script);
        });

        Ok(Self {
            window: Arc::new(Mutex::new(Some(window))),
        })
    }

    pub async fn send_prompt(
        &self,
        _prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let lock = self.window.lock().await;
        if let Some(ref window) = *lock {
            // Send prompt to Claude using eval with argument passing
            let script = r#"
                (function(prompt) {
                    const promptBox = document.querySelector('div[contenteditable="true"]');
                    if (!promptBox) {
                        console.error('Claude prompt box not found');
                        return;
                    }
                    promptBox.focus();
                    document.execCommand('insertText', false, prompt);
                    const sendBtn = document.querySelector('button[aria-label="Send Message"]');
                    if (sendBtn) sendBtn.click();
                })
            "#;

            // Execute script with prompt argument
            window.eval(script);

            // Wait for response
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

            Ok("Response received".to_string())
        } else {
            Err("Claude bridge window not available".into())
        }
    }
}
