//! Subscription account commands.
//!
//! Local inference (Lemonade/Ollama) is free and needs no account. Signing in to
//! a (configurable) remote auth endpoint returns a JWT that unlocks the cloud
//! subscription models (served via the Cyber-Ifrit gateway). The JWT is persisted
//! as the `cyberifrit` provider key in the engine's `api_keys.json` — read fresh
//! on every request by `get_key_for_provider`, so no restart is needed.

use serde::Serialize;
use serde_json::json;
use tauri::State;

use super::api_keys::{get_api_keys_at, save_api_keys_at};

#[derive(Debug, Serialize)]
pub struct SubscriptionSession {
    pub token: String,
    pub tier: String,
    pub email: String,
    /// Expiry as epoch milliseconds, or null when the server doesn't say.
    pub expires: Option<i64>,
}

/// Persist (or clear) the Cyber-Ifrit cloud token at the path the engine reads.
fn store_cloud_token(state: &State<'_, std::sync::Arc<crate::EditorState>>, token: &str) -> Result<(), String> {
    let dir = state.config_dir.clone();
    let mut keys = get_api_keys_at(&dir);
    keys.cyberifrit = if token.is_empty() { None } else { Some(token.to_string()) };
    save_api_keys_at(&dir, &keys)
}

/// Authenticate against the subscription auth endpoint and unlock cloud models.
///
/// `mock=true` produces an offline Pro session so the gate can be exercised
/// without a live auth server (toggle via `subscription.mockAuth` on the client).
#[tauri::command]
pub async fn subscription_login(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    auth_url: String,
    email: String,
    password: String,
    mock: Option<bool>,
) -> Result<SubscriptionSession, String> {
    let session = if mock.unwrap_or(false) {
        SubscriptionSession {
            token: format!("mock.{}.jwt", email.len()),
            tier: "pro".to_string(),
            email: email.clone(),
            expires: Some(chrono::Utc::now().timestamp_millis() + 30 * 24 * 3600 * 1000),
        }
    } else {
        let url = format!("{}/login", auth_url.trim_end_matches('/'));
        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(&json!({ "email": email, "password": password }))
            .timeout(std::time::Duration::from_secs(20))
            .send()
            .await
            .map_err(|e| format!("Auth request failed: {}", e))?;
        if !resp.status().is_success() {
            let code = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(format!("Sign-in rejected ({}): {}", code, body.chars().take(200).collect::<String>()));
        }
        let v: serde_json::Value = resp
            .json()
            .await
            .map_err(|e| format!("Auth response not JSON: {}", e))?;
        let token = v.get("token").and_then(|t| t.as_str()).unwrap_or("").to_string();
        if token.is_empty() {
            return Err("Auth server returned no token".to_string());
        }
        SubscriptionSession {
            token,
            tier: v.get("tier").and_then(|t| t.as_str()).unwrap_or("pro").to_string(),
            email: v.get("email").and_then(|t| t.as_str()).unwrap_or(&email).to_string(),
            expires: v.get("expires").and_then(|t| t.as_i64()),
        }
    };

    store_cloud_token(&state, &session.token)?;
    Ok(session)
}

/// Clear the cloud subscription token (re-locks cloud models). Local stays free.
#[tauri::command]
pub async fn subscription_logout(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    store_cloud_token(&state, "")
}
