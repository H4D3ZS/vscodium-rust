//! Supabase email + password auth for the IDE.
//!
//! Makes an account a *person*, not a machine: after sign-in the session
//! (access + refresh token) is stored in `<config_dir>/session.json`, and
//! `account.rs` uses the JWT to sync the user's real tier/entitlements from
//! Supabase. Only **public** config lives here — `SUPABASE_URL` + the **anon**
//! key are safe to ship. The Supabase **service-role** key NEVER appears in the
//! client; it is a server-only secret used by the Netlify webhook.
//!
//! Config resolution (so no secret is hardcoded in the repo):
//!   1. env `SUPABASE_URL` / `SUPABASE_ANON_KEY`
//!   2. `<config_dir>/api_keys.json` keys `supabase_url` / `supabase_anon_key`
//!   3. compiled fallback (empty → auth simply stays disabled until configured)

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;


fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

// Public values — safe to ship (the anon key is RLS-protected by design).
// Overridable via env (SUPABASE_URL / SUPABASE_ANON_KEY) or api_keys.json.
const SUPABASE_URL_FALLBACK: &str = "https://ktufvjkvejjshtndmjze.supabase.co";
const SUPABASE_ANON_FALLBACK: &str = "" /* sanitized for OSS: provide via runtime config / env */;

/// Resolve the public Supabase URL + anon key. Returns trimmed `(url, anon)`.
pub fn supabase_config(config_dir: &Path) -> (String, String) {
    let mut url = std::env::var("SUPABASE_URL").unwrap_or_default();
    let mut anon = std::env::var("SUPABASE_ANON_KEY").unwrap_or_default();
    if url.is_empty() || anon.is_empty() {
        if let Ok(txt) = std::fs::read_to_string(config_dir.join("api_keys.json")) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) {
                if url.is_empty() {
                    url = v.get("supabase_url").and_then(|x| x.as_str()).unwrap_or("").to_string();
                }
                if anon.is_empty() {
                    anon = v.get("supabase_anon_key").and_then(|x| x.as_str()).unwrap_or("").to_string();
                }
            }
        }
    }
    if url.is_empty() {
        url = SUPABASE_URL_FALLBACK.to_string();
    }
    if anon.is_empty() {
        anon = SUPABASE_ANON_FALLBACK.to_string();
    }
    (url.trim_end_matches('/').to_string(), anon)
}

// ─────────────────────────────────────────────────────────────────────────────
// Session
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Session {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_at: u64, // unix secs (already includes a 60s safety margin)
    pub user_id: String,
    pub email: Option<String>,
}

fn session_path(config_dir: &Path) -> PathBuf {
    config_dir.join("session.json")
}

pub fn load_session(config_dir: &Path) -> Option<Session> {
    std::fs::read_to_string(session_path(config_dir))
        .ok()
        .and_then(|c| serde_json::from_str::<Session>(&c).ok())
        .filter(|s| !s.access_token.is_empty())
}

fn save_session(config_dir: &Path, s: &Session) -> Result<(), String> {
    let _ = std::fs::create_dir_all(config_dir);
    let json = serde_json::to_string_pretty(s).map_err(|e| e.to_string())?;
    std::fs::write(session_path(config_dir), json).map_err(|e| e.to_string())?;
    // Mirror the access token into api_keys.json as the `cyberifrit` key — that's
    // the ONLY place the AI engine looks for the cloud JWT (get_key_for_provider),
    // so cloud Lemonade requests carry it. Without this, a fully signed-in
    // user still gets 401 from the JWT gate. Runs on sign-in, sign-up, and refresh.
    mirror_cloud_token(config_dir, Some(&s.access_token));
    Ok(())
}

fn clear_session(config_dir: &Path) {
    let _ = std::fs::remove_file(session_path(config_dir));
    mirror_cloud_token(config_dir, None); // re-lock cloud on sign-out
}

/// Write (or remove) the `cyberifrit` cloud token in api_keys.json without
/// disturbing other keys. `None` removes it.
fn mirror_cloud_token(config_dir: &Path, token: Option<&str>) {
    let path = config_dir.join("api_keys.json");
    let mut keys: serde_json::Value = std::fs::read_to_string(&path)
        .ok()
        .and_then(|c| serde_json::from_str(&c).ok())
        .unwrap_or_else(|| serde_json::json!({}));
    if !keys.is_object() {
        keys = serde_json::json!({});
    }
    if let Some(obj) = keys.as_object_mut() {
        match token {
            Some(t) if !t.is_empty() => { obj.insert("cyberifrit".into(), serde_json::json!(t)); }
            _ => { obj.remove("cyberifrit"); }
        }
    }
    if let Ok(json) = serde_json::to_string_pretty(&keys) {
        let _ = std::fs::write(&path, json);
    }
}

// Supabase GoTrue token / user shapes.
#[derive(Deserialize)]
struct TokenResp {
    access_token: String,
    refresh_token: String,
    #[serde(default)]
    expires_in: u64,
    user: Option<GoTrueUser>,
}

#[derive(Deserialize)]
struct GoTrueUser {
    id: String,
    #[serde(default)]
    email: Option<String>,
}

fn session_from_token(t: TokenResp) -> Session {
    let (user_id, email) = t.user.map(|u| (u.id, u.email)).unwrap_or_default();
    Session {
        access_token: t.access_token,
        refresh_token: t.refresh_token,
        // Refresh 60s early so a request never rides an about-to-expire token.
        expires_at: now() + t.expires_in.saturating_sub(60),
        user_id,
        email,
    }
}

/// Return a currently-valid session, transparently refreshing via the refresh
/// token when the access token has expired. `None` = not signed in / refresh
/// failed (caller falls back to the local Community account). Used by both the
/// `auth_session` command and `account.rs` to authorize Supabase reads.
pub async fn valid_session(config_dir: &Path) -> Option<Session> {
    let s = load_session(config_dir)?;
    if now() < s.expires_at {
        return Some(s);
    }
    let (url, anon) = supabase_config(config_dir);
    if url.is_empty() || anon.is_empty() {
        return None;
    }
    let resp = reqwest::Client::new()
        .post(format!("{url}/auth/v1/token?grant_type=refresh_token"))
        .header("apikey", &anon)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "refresh_token": s.refresh_token }))
        .send()
        .await
        .ok()?;
    if !resp.status().is_success() {
        clear_session(config_dir); // refresh token dead → force re-login
        return None;
    }
    let t: TokenResp = resp.json().await.ok()?;
    let ns = session_from_token(t);
    let _ = save_session(config_dir, &ns);
    Some(ns)
}

fn err_message(body: &serde_json::Value, fallback: &str) -> String {
    body.get("msg")
        .or_else(|| body.get("error_description"))
        .or_else(|| body.get("error"))
        .or_else(|| body.get("message"))
        .and_then(|v| v.as_str())
        .unwrap_or(fallback)
        .to_string()
}

// ─────────────────────────────────────────────────────────────────────────────
// Commands
// ─────────────────────────────────────────────────────────────────────────────

/// Create a Supabase account. If the project has email-confirmation disabled,
/// the response carries a session and we sign the user in immediately; if
/// confirmation is required, we report `needs_confirmation` so the UI can tell
/// the user to check their email.
#[tauri::command]
pub async fn auth_sign_up(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    email: String,
    password: String,
) -> Result<serde_json::Value, String> {
    let (url, anon) = supabase_config(&state.config_dir);
    if url.is_empty() || anon.is_empty() {
        return Err("Supabase not configured. Set supabase_url + supabase_anon_key in api_keys.json (or SUPABASE_URL / SUPABASE_ANON_KEY env).".into());
    }
    let resp = reqwest::Client::new()
        .post(format!("{url}/auth/v1/signup"))
        .header("apikey", &anon)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    let body: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    if !status.is_success() {
        let msg = err_message(&body, "sign-up failed");
        // Registration is CAPTCHA-gated on the website; the Supabase instance has
        // public sign-ups disabled so bots can't mass-create accounts. Point the
        // user there instead of surfacing a raw "signups not allowed" error.
        if msg.to_lowercase().contains("signup") || msg.to_lowercase().contains("not allowed") {
            return Err("Create your account at https://cyberifrit.xyz/account (one-time, takes a few seconds), then sign in here.".into());
        }
        return Err(msg);
    }
    // Session returned inline (confirmations off) → persist + signed in.
    if let Some(at) = body.get("access_token").and_then(|v| v.as_str()) {
        let sess = Session {
            access_token: at.to_string(),
            refresh_token: body.get("refresh_token").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            expires_at: now() + body.get("expires_in").and_then(|v| v.as_u64()).unwrap_or(3600).saturating_sub(60),
            user_id: body.pointer("/user/id").and_then(|v| v.as_str()).unwrap_or("").to_string(),
            email: body.pointer("/user/email").and_then(|v| v.as_str()).map(|s| s.to_string()),
        };
        save_session(&state.config_dir, &sess)?;
        return Ok(serde_json::json!({ "ok": true, "signed_in": true, "email": sess.email }));
    }
    Ok(serde_json::json!({ "ok": true, "signed_in": false, "needs_confirmation": true }))
}

/// Email + password sign-in (Supabase GoTrue password grant). Stores the
/// session locally on success.
#[tauri::command]
pub async fn auth_sign_in(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    email: String,
    password: String,
) -> Result<serde_json::Value, String> {
    let (url, anon) = supabase_config(&state.config_dir);
    if url.is_empty() || anon.is_empty() {
        return Err("Supabase not configured. Set supabase_url + supabase_anon_key in api_keys.json (or SUPABASE_URL / SUPABASE_ANON_KEY env).".into());
    }
    let resp = reqwest::Client::new()
        .post(format!("{url}/auth/v1/token?grant_type=password"))
        .header("apikey", &anon)
        .header("Content-Type", "application/json")
        .json(&serde_json::json!({ "email": email, "password": password }))
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let status = resp.status();
    if !status.is_success() {
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        return Err(err_message(&body, "invalid email or password"));
    }
    let t: TokenResp = resp.json().await.map_err(|e| e.to_string())?;
    let sess = session_from_token(t);
    save_session(&state.config_dir, &sess)?;
    Ok(serde_json::json!({ "ok": true, "signed_in": true, "email": sess.email, "user_id": sess.user_id }))
}

/// Current sign-in state (auto-refreshes an expired access token).
#[tauri::command]
pub async fn auth_session(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<serde_json::Value, String> {
    match valid_session(&state.config_dir).await {
        Some(s) => Ok(serde_json::json!({ "signed_in": true, "email": s.email, "user_id": s.user_id })),
        None => Ok(serde_json::json!({ "signed_in": false })),
    }
}

/// Sign out — clears the stored session.
#[tauri::command]
pub async fn auth_sign_out(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    clear_session(&state.config_dir);
    Ok(())
}
