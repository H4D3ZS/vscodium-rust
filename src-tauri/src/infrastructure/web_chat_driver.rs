//! Headless web-chat driver — turns a logged-in free web chat (Claude.ai,
//! DeepSeek) into a usable LLM "model" without any API key.
//!
//! The native agent panel selects a provider like `webchat-claude`; the agent
//! loop then POSTs an OpenAI-style request to the local shim (`webchat_openai_shim`),
//! which calls [`WebChatDriver::complete`]. The driver drives a **dedicated**
//! stealth-Firefox sidecar (separate from the agent's own browser tools, so the
//! agent can still navigate pages without hijacking the chat session):
//!
//!   ensure session → start a fresh chat → type the flattened prompt → submit →
//!   poll the assistant message until it stops growing → return the text.
//!
//! Login is one-time and visible (headful), after which cookies + localStorage are
//! persisted to `config_dir/webchat/<provider>.json` and reused headless for 24/7 use.

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use tokio::sync::Mutex;

use crate::infrastructure::browser::BrowserState;

/// Static per-provider config: URLs + DOM selectors. Selectors mirror the ones in
/// `application::commands::auth` so both paths stay in sync. Keep them here as the
/// single source for the headless driver.
#[derive(Clone)]
pub struct WebProvider {
    pub key: &'static str,
    pub origin: &'static str,
    pub new_chat_url: &'static str,
    pub login_url: &'static str,
    /// CSS for the prompt input (textarea or contenteditable).
    pub input_selector: &'static str,
    /// CSS for the send button (clicked if present; Enter is the fallback).
    pub send_selector: &'static str,
    /// CSS matching assistant message containers (the LAST match is the reply).
    pub assistant_selector: &'static str,
}

/// Resolve a provider key (`webchat-claude`, `claude`, `deepseek`, …) to its config.
pub fn provider_config(key: &str) -> Option<WebProvider> {
    let k = key.to_lowercase();
    let k = k.strip_prefix("webchat-").unwrap_or(&k);
    match k {
        "claude" => Some(WebProvider {
            key: "claude",
            origin: "https://claude.ai",
            new_chat_url: "https://claude.ai/new",
            login_url: "https://claude.ai/login",
            input_selector: "div[contenteditable=\"true\"]",
            send_selector:
                "button[aria-label=\"Send Message\"], button[aria-label=\"Send message\"], button[data-testid=\"send-button\"]",
            assistant_selector: "[data-testid*=\"message\"], div.font-claude-message, div.prose",
        }),
        "deepseek" => Some(WebProvider {
            key: "deepseek",
            origin: "https://chat.deepseek.com",
            new_chat_url: "https://chat.deepseek.com/",
            login_url: "https://chat.deepseek.com/sign_in",
            input_selector: "textarea, div[contenteditable=\"true\"]",
            send_selector: "button[type=\"submit\"], button[aria-label*=\"Send\"], div[role=\"button\"]:has(svg)",
            assistant_selector: ".ds-markdown, [class*=\"markdown\"], [class*=\"message-content\"]",
        }),
        _ => None,
    }
}

/// The headless web-chat driver. One dedicated browser sidecar, shared across all
/// web providers (navigation switches between them); a `busy` lock serialises
/// requests so two completions never fight over the single page.
pub struct WebChatDriver {
    browser: Arc<BrowserState>,
    profile_dir: PathBuf,
    busy: Mutex<()>,
    /// Providers whose persisted session was already restored this process.
    restored: Mutex<HashSet<String>>,
}

impl WebChatDriver {
    pub fn new(config_dir: &PathBuf) -> Arc<Self> {
        let profile_dir = config_dir.join("webchat");
        let _ = std::fs::create_dir_all(&profile_dir);
        Arc::new(Self {
            // Dedicated sidecar — NOT services.browser — so the agent's own browser
            // tools never navigate the chat page out from under us.
            browser: Arc::new(BrowserState::new()),
            profile_dir,
            busy: Mutex::new(()),
            restored: Mutex::new(HashSet::new()),
        })
    }

    fn state_path(&self, provider: &str) -> PathBuf {
        self.profile_dir.join(format!("{}.json", provider))
    }

    /// True if a saved session file exists for this provider.
    pub fn has_session(&self, provider: &str) -> bool {
        let cfg = match provider_config(provider) {
            Some(c) => c,
            None => return false,
        };
        self.state_path(cfg.key).exists()
    }

    /// One-time visible login. Opens a REAL Firefox window at the provider's login
    /// page and polls until the composer appears (= logged in), then persists the
    /// session (cookies + localStorage). Returns once login is detected or times out.
    pub async fn login(&self, provider: &str) -> Result<String, String> {
        let cfg = provider_config(provider)
            .ok_or_else(|| format!("unknown web provider: {}", provider))?;
        let _guard = self.busy.lock().await;
        self.login_inner(&cfg).await
    }

    /// The actual login flow. Assumes the `busy` lock is already held (so it can be
    /// called both from `login` and from `complete` on first use without deadlock).
    async fn login_inner(&self, cfg: &WebProvider) -> Result<String, String> {
        self.browser
            .ensure_started_with(false) // headful — the user must see + complete login
            .await?;
        self.browser
            .cmd("navigate", json!({ "url": cfg.login_url, "timeout": 60000 }), 70)
            .await?;

        // Poll up to 5 min for the composer to exist on the provider origin.
        let deadline = Instant::now() + Duration::from_secs(300);
        let check = format!(
            "(() => {{ try {{ return !!document.querySelector({sel:?}) && location.href.indexOf({origin:?}) === 0; }} catch (e) {{ return false; }} }})()",
            sel = cfg.input_selector,
            origin = cfg.origin,
        );
        loop {
            if Instant::now() > deadline {
                return Err("login timed out (composer never appeared)".into());
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
            if let Ok(v) = self.browser.cmd("eval", json!({ "script": check }), 15).await {
                if v.get("value").and_then(|x| x.as_bool()).unwrap_or(false) {
                    break;
                }
            }
        }

        self.save_session(cfg).await?;
        self.restored.lock().await.insert(cfg.key.to_string());
        Ok(format!("{} session captured and saved", cfg.key))
    }

    /// Dump cookies + localStorage for the current origin to disk.
    async fn save_session(&self, cfg: &WebProvider) -> Result<(), String> {
        let cookies = self
            .browser
            .cmd("cookies", json!({}), 20)
            .await?
            .get("cookies")
            .cloned()
            .unwrap_or(json!([]));
        let ls = self
            .browser
            .cmd(
                "eval",
                json!({ "script": "JSON.stringify(Object.fromEntries(Object.entries(localStorage)))" }),
                15,
            )
            .await
            .ok()
            .and_then(|v| v.get("value").and_then(|x| x.as_str()).map(|s| s.to_string()))
            .unwrap_or_else(|| "{}".to_string());

        let state = json!({ "cookies": cookies, "local_storage": ls, "origin": cfg.origin });
        std::fs::write(
            self.state_path(cfg.key),
            serde_json::to_vec_pretty(&state).unwrap_or_default(),
        )
        .map_err(|e| format!("save session: {}", e))?;
        Ok(())
    }

    /// Restore a persisted session into the (headless) browser once per process.
    async fn ensure_restored(&self, cfg: &WebProvider) -> Result<(), String> {
        if self.restored.lock().await.contains(cfg.key) {
            return Ok(());
        }
        let path = self.state_path(cfg.key);
        if !path.exists() {
            return Err(format!(
                "no saved session for '{}'. Run the one-time web login first.",
                cfg.key
            ));
        }
        let raw = std::fs::read_to_string(&path).map_err(|e| format!("read session: {}", e))?;
        let state: Value = serde_json::from_str(&raw).map_err(|e| format!("parse session: {}", e))?;
        let cookies = state.get("cookies").cloned().unwrap_or(json!([]));
        let ls = state
            .get("local_storage")
            .and_then(|x| x.as_str())
            .unwrap_or("{}")
            .to_string();

        // origin must be loaded before cookies/localStorage apply.
        self.browser
            .cmd("navigate", json!({ "url": cfg.origin, "timeout": 45000 }), 50)
            .await?;
        self.browser
            .cmd("add_cookies", json!({ "cookies": cookies }), 20)
            .await?;
        let restore_ls = format!(
            "(() => {{ try {{ const o = JSON.parse({ls}); for (const k in o) localStorage.setItem(k, o[k]); return true; }} catch (e) {{ return false; }} }})()",
            ls = serde_json::to_string(&ls).unwrap_or_else(|_| "\"{}\"".into()),
        );
        let _ = self.browser.cmd("eval", json!({ "script": restore_ls }), 15).await;

        self.restored.lock().await.insert(cfg.key.to_string());
        Ok(())
    }

    /// Run one completion: start a fresh chat, send `prompt`, stream growth via
    /// `on_delta`, and return the final assistant text. `on_delta` receives only
    /// the NEW suffix each time (so the caller can emit OpenAI-style deltas).
    pub async fn complete<F>(
        &self,
        provider: &str,
        prompt: &str,
        mut on_delta: F,
    ) -> Result<String, String>
    where
        F: FnMut(&str),
    {
        let cfg = provider_config(provider)
            .ok_or_else(|| format!("unknown web provider: {}", provider))?;
        let _guard = self.busy.lock().await;

        if self.state_path(cfg.key).exists() {
            // Returning user: restore the saved session and run headless.
            self.browser.ensure_started_with(true).await?;
            self.ensure_restored(&cfg).await?;
        } else {
            // First use: pop a VISIBLE browser so the user can sign in once. After
            // login the session is saved and every later run is headless.
            self.login_inner(&cfg).await?;
        }

        // Fresh chat each request — keeps provider rotation trivial and avoids
        // relying on the web UI's own conversation memory.
        self.browser
            .cmd("navigate", json!({ "url": cfg.new_chat_url, "timeout": 45000 }), 50)
            .await?;
        // Give the SPA a moment to mount the composer.
        tokio::time::sleep(Duration::from_millis(1200)).await;

        // Inject the prompt into the composer (textarea OR contenteditable).
        let inject = format!(
            r#"(() => {{
                const input = document.querySelector({sel:?});
                if (!input) return 'NO_INPUT';
                input.focus();
                const text = {prompt};
                if ('value' in input && input.tagName === 'TEXTAREA') {{
                    const setter = Object.getOwnPropertyDescriptor(window.HTMLTextAreaElement.prototype, 'value').set;
                    setter.call(input, text);
                    input.dispatchEvent(new InputEvent('input', {{ bubbles: true }}));
                }} else {{
                    input.textContent = text;
                    input.dispatchEvent(new InputEvent('input', {{ bubbles: true, inputType: 'insertText', data: text }}));
                }}
                return 'OK';
            }})()"#,
            sel = cfg.input_selector,
            prompt = serde_json::to_string(prompt).unwrap_or_else(|_| "\"\"".into()),
        );
        let r = self.browser.cmd("eval", json!({ "script": inject }), 20).await?;
        if r.get("value").and_then(|v| v.as_str()) == Some("NO_INPUT") {
            return Err(format!(
                "composer not found for '{}' — the session may be logged out or the page layout changed.",
                cfg.key
            ));
        }

        // Submit: click the send button if present, else press Enter.
        let send = format!(
            r#"(() => {{
                const b = document.querySelector({sel:?});
                if (b && !b.disabled) {{ b.click(); return 'CLICKED'; }}
                return 'NO_SEND';
            }})()"#,
            sel = cfg.send_selector,
        );
        let sr = self.browser.cmd("eval", json!({ "script": send }), 15).await?;
        if sr.get("value").and_then(|v| v.as_str()) != Some("CLICKED") {
            let _ = self.browser.cmd("press", json!({ "key": "Enter" }), 15).await;
        }

        // Poll the last assistant message until it stops growing.
        let read = format!(
            r#"(() => {{
                try {{
                    const nodes = Array.from(document.querySelectorAll({sel:?}));
                    const txts = nodes.map(n => (n.innerText || n.textContent || '').trim()).filter(t => t.length > 0);
                    return txts.length ? txts[txts.length - 1] : '';
                }} catch (e) {{ return ''; }}
            }})()"#,
            sel = cfg.assistant_selector,
        );

        let hard_deadline = Instant::now() + Duration::from_secs(240);
        let stable_for = Duration::from_millis(2500);
        // `sent` is exactly the text already streamed to the caller; because it is
        // always a prefix of `cur` when we slice, `sent.len()` is a valid char
        // boundary in `cur` (no UTF-8 panic).
        let mut sent = String::new();
        let mut last_seen = String::new();
        let mut last_growth = Instant::now();

        loop {
            tokio::time::sleep(Duration::from_millis(700)).await;
            let now = Instant::now();
            let cur = self
                .browser
                .cmd("eval", json!({ "script": read }), 15)
                .await
                .ok()
                .and_then(|v| v.get("value").and_then(|x| x.as_str()).map(|s| s.to_string()))
                .unwrap_or_default();

            if cur != last_seen && !cur.is_empty() {
                last_growth = now;
                last_seen = cur.clone();
                // Stream the new suffix only when `cur` extends what we've sent.
                if cur.len() > sent.len() && cur.starts_with(&sent) {
                    on_delta(&cur[sent.len()..]);
                    sent = cur;
                } else {
                    // Re-render replaced the node — resync silently; the full final
                    // text is still returned below.
                    sent = cur;
                }
            }

            let settled = !sent.is_empty() && now.duration_since(last_growth) >= stable_for;
            if settled || now > hard_deadline {
                break;
            }
        }

        if sent.trim().is_empty() {
            return Err(format!("no reply captured from '{}' (timeout)", cfg.key));
        }
        Ok(sent)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_known_providers() {
        assert_eq!(provider_config("webchat-claude").unwrap().key, "claude");
        assert_eq!(provider_config("deepseek").unwrap().key, "deepseek");
        assert!(provider_config("gemini").is_none());
    }
}
