use tauri::webview::{WebviewWindow, WebviewWindowBuilder};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct GeminiBridge {
    window: Arc<Mutex<Option<WebviewWindow>>>,
}

impl GeminiBridge {
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let window = WebviewWindowBuilder::new(
            app_handle,
            "gemini_bridge",
            tauri::WebviewUrl::External("https://gemini.google.com/app".parse().unwrap())
        )
        .transparent(true)
        .decorations(false)
        .build()?;
        
        // Inject automation script
        let window_clone = window.clone();
        tauri::async_runtime::spawn(async move {
            // Wait for webview to be ready
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            
            let script = r#"
                window.onload = () => {
                    const promptBox = document.querySelector('div[contenteditable]');
                    if (!promptBox) {
                        console.error('Gemini prompt box not found');
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
            
            // Use eval instead of evaluate_script
            let _ = window_clone.eval(script);
        });

        Ok(Self { window: Arc::new(Mutex::new(Some(window))) })
    }

    pub async fn send_prompt(&self, _prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let lock = self.window.lock().await;
        if let Some(ref window) = *lock {
            // Send prompt to Gemini using eval with argument passing
            let script = r#"
                (function(prompt) {
                    const promptBox = document.querySelector('div[contenteditable]');
                    if (!promptBox) {
                        console.error('Gemini prompt box not found');
                        return;
                    }
                    promptBox.focus();
                    document.execCommand('insertText', false, prompt);
                    const sendBtn = document.querySelector('button[aria-label="Send message"]');
                    if (sendBtn) sendBtn.click();
                })
            "#;
            
            // Execute script with prompt argument
            window.eval(script);
            
            // Wait for response
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;
            
            Ok("Response received".to_string())
        } else {
            Err("Gemini bridge window not available".into())
        }
    }
}
