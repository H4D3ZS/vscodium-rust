//! SaaS account, subscription, entitlement & Terms-of-Service model.
//!
//! This is the local-first foundation for the subscription IDE. Today it
//! persists to `<config_dir>/account.json`; the same `AccountManager` API is the
//! seam where a real auth/billing backend (Stripe + a user service) plugs in
//! later — commands and entitlement checks stay identical, only the storage
//! source changes.
//!
//! Pricing/tiers mirror cyberifrit.xyz:
//!   Community (free) · Pro Developer ($30) · Security Researcher ($75) · Enterprise ($225)
//!
//! The offensive-security ("bug bounty") tooling is gated behind a versioned ToS
//! acceptance that is recorded **on the account** — so consent is auditable and
//! travels with the user, not the machine.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::State;

use crate::EditorState;

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0)
}

// ─────────────────────────────────────────────────────────────────────────────
// Model
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum Tier {
    #[default]
    Community,
    ProDeveloper,
    SecurityResearcher,
    Enterprise,
}

impl Tier {
    pub fn label(&self) -> &'static str {
        match self {
            Tier::Community => "Community",
            Tier::ProDeveloper => "Pro Developer",
            Tier::SecurityResearcher => "Security Researcher",
            Tier::Enterprise => "Enterprise",
        }
    }
    pub fn monthly_price_usd(&self) -> u32 {
        match self {
            Tier::Community => 0,
            Tier::ProDeveloper => 30,
            Tier::SecurityResearcher => 75,
            Tier::Enterprise => 225,
        }
    }
}

/// A signed acceptance of a versioned Terms-of-Service document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TosAcceptance {
    pub doc_id: String,   // e.g. "bug-bounty"
    pub version: String,  // e.g. "1.0"
    pub accepted_at: u64, // unix secs
}

/// A one-time / add-on entitlement purchased outside the base tier
/// (e.g. the MiMo pre-model promo).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Addon {
    pub id: String,        // "mimo_pro"
    pub label: String,     // "MiMo Pro model"
    pub acquired_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub created_at: u64,
    pub tier: Tier,
    #[serde(default)]
    pub tos: Vec<TosAcceptance>,
    #[serde(default)]
    pub addons: Vec<Addon>,
    /// Unix secs when the 1-day free trial ends (None = no trial).
    #[serde(default)]
    pub trial_ends_at: Option<u64>,
    /// True once the one-time trial has been started (prevents re-trial).
    #[serde(default)]
    pub trial_used: bool,
}

impl Default for Account {
    fn default() -> Self {
        Account {
            id: uuid::Uuid::new_v4().to_string(),
            email: None,
            display_name: None,
            created_at: now(),
            tier: Tier::Community,
            tos: Vec::new(),
            addons: Vec::new(),
            trial_ends_at: None,
            trial_used: false,
        }
    }
}

/// Is the 1-day free trial currently active?
fn trial_active(account: &Account) -> bool {
    account.trial_ends_at.map(|t| now() < t).unwrap_or(false)
}

/// Derived feature + quota entitlements for a given tier (+ add-ons). Kept as a
/// pure function of the account so it can be recomputed anywhere (and later
/// overridden by the billing backend) without storing stale data.
#[derive(Debug, Clone, Serialize)]
pub struct Entitlements {
    pub daily_requests: u32,
    pub monthly_requests: u32,
    /// Monthly token budget (input+output). 0 == unlimited.
    pub monthly_tokens: u64,
    pub features: Vec<String>,
}

fn entitlements_for(account: &Account) -> Entitlements {
    // 1-day free trial: unlimited prompts + EVERY feature (security included), so
    // the user gets the full agentic experience before committing.
    if trial_active(account) {
        return Entitlements {
            daily_requests: 0,   // 0 == unlimited
            monthly_requests: 0, // 0 == unlimited
            monthly_tokens: 0,   // 0 == unlimited
            features: [
                "editor", "agentic", "neural_vfs", "local_models", "cloud_models",
                "bug_bounty", "vuln_hunt", "web_audit", "mimo_pro", "trial",
            ].into_iter().map(String::from).collect(),
        };
    }
    let (daily, monthly, tokens, mut features): (u32, u32, u64, Vec<&str>) = match account.tier {
        Tier::Community => (50, 1_500, 200_000, vec!["editor", "agentic_basic", "local_models"]),
        Tier::ProDeveloper => (0, 5_000, 5_000_000, vec!["editor", "agentic", "neural_vfs", "local_models", "cloud_models"]),
        Tier::SecurityResearcher => (0, 20_000, 20_000_000, vec![
            "editor", "agentic", "neural_vfs", "local_models", "cloud_models",
            "bug_bounty", "vuln_hunt", "web_audit",
        ]),
        Tier::Enterprise => (0, 0 /* custom */, 0 /* unlimited */, vec![
            "editor", "agentic", "neural_vfs", "local_models", "cloud_models",
            "bug_bounty", "vuln_hunt", "web_audit", "team", "amd_backend",
        ]),
    };
    // Add-ons unlock extra features regardless of tier.
    for a in &account.addons {
        match a.id.as_str() {
            "mimo_pro" => features.push("mimo_pro"),
            "bug_bounty_pack" => features.push("bug_bounty"),
            _ => {}
        }
    }
    Entitlements {
        daily_requests: daily,
        monthly_requests: monthly,
        monthly_tokens: tokens,
        features: features.into_iter().map(|s| s.to_string()).collect(),
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Storage (local now, remote-ready)
// ─────────────────────────────────────────────────────────────────────────────

pub struct AccountManager;

impl AccountManager {
    fn path(config_dir: &Path) -> PathBuf {
        config_dir.join("account.json")
    }

    pub fn load(config_dir: &Path) -> Account {
        let p = Self::path(config_dir);
        match std::fs::read_to_string(&p).ok().and_then(|c| serde_json::from_str::<Account>(&c).ok()) {
            Some(a) => a,
            None => {
                // First run — create + persist a default Community account.
                let a = Account::default();
                let _ = Self::save(config_dir, &a);
                a
            }
        }
    }

    pub fn save(config_dir: &Path, account: &Account) -> Result<(), String> {
        let _ = std::fs::create_dir_all(config_dir);
        let p = Self::path(config_dir);
        let json = serde_json::to_string_pretty(account).map_err(|e| e.to_string())?;
        std::fs::write(&p, json).map_err(|e| e.to_string())
    }

    pub fn has_accepted(account: &Account, doc_id: &str) -> bool {
        account.tos.iter().any(|t| t.doc_id == doc_id)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Commands
// ─────────────────────────────────────────────────────────────────────────────

/// Full account view (account + derived entitlements + subscription status).
/// When signed in, the tier/status are synced live from Supabase (the billing
/// source of truth) and cached to `account.json`; offline / signed-out falls
/// back to the local cache (Community by default).
#[tauri::command]
pub async fn account_get(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    Ok(build_view(&state.config_dir).await)
}

/// Record a versioned ToS acceptance on the account (idempotent per doc — the
/// newest acceptance wins; older versions are kept for audit).
#[tauri::command]
pub async fn account_accept_tos(
    state: State<'_, EditorState>,
    doc_id: String,
    version: String,
) -> Result<(), String> {
    let mut acc = AccountManager::load(&state.config_dir);
    acc.tos.retain(|t| !(t.doc_id == doc_id && t.version == version));
    acc.tos.push(TosAcceptance { doc_id, version, accepted_at: now() });
    AccountManager::save(&state.config_dir, &acc)
}

/// Has the account accepted a given ToS document (any version)?
#[tauri::command]
pub async fn account_tos_status(state: State<'_, EditorState>, doc_id: String) -> Result<bool, String> {
    let acc = AccountManager::load(&state.config_dir);
    Ok(AccountManager::has_accepted(&acc, &doc_id))
}

/// Entitlement check used to gate features (e.g. "bug_bounty").
#[tauri::command]
pub async fn account_has_feature(state: State<'_, EditorState>, feature: String) -> Result<bool, String> {
    let acc = AccountManager::load(&state.config_dir);
    Ok(entitlements_for(&acc).features.iter().any(|f| f == &feature))
}

/// Set the subscription tier. LOCAL/dev path for now — in production this is
/// driven by the billing backend (Stripe webhook → account service), never the
/// client. Exposed so the IDE can reflect a tier the backend reports.
#[tauri::command]
pub async fn account_set_tier(state: State<'_, EditorState>, tier: String) -> Result<(), String> {
    let mut acc = AccountManager::load(&state.config_dir);
    acc.tier = match tier.to_lowercase().replace([' ', '-'], "_").as_str() {
        "community" => Tier::Community,
        "pro_developer" | "pro" => Tier::ProDeveloper,
        "security_researcher" | "security" => Tier::SecurityResearcher,
        "enterprise" => Tier::Enterprise,
        other => return Err(format!("unknown tier: {other}")),
    };
    AccountManager::save(&state.config_dir, &acc)
}

/// Acquire an add-on (e.g. the MiMo pre-model promo). LOCAL/dev path — real
/// purchases settle through billing; this records the entitlement locally so the
/// feature unlocks immediately and can be reconciled with the backend later.
#[tauri::command]
pub async fn account_acquire_addon(
    state: State<'_, EditorState>,
    id: String,
    label: String,
) -> Result<(), String> {
    let mut acc = AccountManager::load(&state.config_dir);
    if !acc.addons.iter().any(|a| a.id == id) {
        acc.addons.push(Addon { id, label, acquired_at: now() });
        AccountManager::save(&state.config_dir, &acc)?;
    }
    Ok(())
}

// ─────────────────────────────────────────────────────────────────────────────
// Billing sync (Supabase = source of truth, PayMongo = money)
// ─────────────────────────────────────────────────────────────────────────────

/// Website base where the Netlify billing functions live. Resolution:
/// env `CYBERIFRIT_SITE` → api_keys.json `site_url` → `https://cyberifrit.xyz`.
fn site_base(config_dir: &Path) -> String {
    if let Ok(v) = std::env::var("CYBERIFRIT_SITE") {
        if !v.is_empty() {
            return v.trim_end_matches('/').to_string();
        }
    }
    if let Ok(txt) = std::fs::read_to_string(config_dir.join("api_keys.json")) {
        if let Ok(j) = serde_json::from_str::<serde_json::Value>(&txt) {
            if let Some(s) = j.get("site_url").and_then(|x| x.as_str()) {
                if !s.is_empty() {
                    return s.trim_end_matches('/').to_string();
                }
            }
        }
    }
    "https://cyberifrit.xyz".to_string()
}

fn tier_from_str(s: &str) -> Tier {
    match s.to_lowercase().replace([' ', '-'], "_").as_str() {
        "pro_developer" | "pro" => Tier::ProDeveloper,
        "security_researcher" | "security" => Tier::SecurityResearcher,
        "enterprise" => Tier::Enterprise,
        _ => Tier::Community,
    }
}

struct RemoteState {
    tier: Tier,
    status: String,
    current_period_end: Option<String>,
    addons: Vec<Addon>,
    email: Option<String>,
}

/// Pull the user's subscription + add-ons from Supabase (RLS-scoped by the
/// signed-in JWT). `None` when signed out / unreachable.
async fn sync_from_supabase(config_dir: &Path) -> Option<RemoteState> {
    let sess = crate::auth::valid_session(config_dir).await?;
    let (url, anon) = crate::auth::supabase_config(config_dir);
    if url.is_empty() {
        return None;
    }
    let client = reqwest::Client::new();

    // Subscription row (at most one per user).
    let rows: Vec<serde_json::Value> = client
        .get(format!(
            "{url}/rest/v1/subscriptions?user_id=eq.{}&select=tier,status,current_period_end",
            sess.user_id
        ))
        .header("apikey", &anon)
        .bearer_auth(&sess.access_token)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let (tier, status, current_period_end) = match rows.first() {
        Some(r) => {
            let st = r.get("status").and_then(|v| v.as_str()).unwrap_or("active").to_string();
            // Only grant the paid tier while the sub is active (past_due = grace).
            let t = if matches!(st.as_str(), "active" | "past_due" | "trialing") {
                tier_from_str(r.get("tier").and_then(|v| v.as_str()).unwrap_or("community"))
            } else {
                Tier::Community
            };
            (t, st, r.get("current_period_end").and_then(|v| v.as_str()).map(|s| s.to_string()))
        }
        None => (Tier::Community, "none".to_string(), None),
    };

    // Add-ons (e.g. mimo_pro).
    let mut addons = Vec::new();
    if let Ok(resp) = client
        .get(format!("{url}/rest/v1/addons?user_id=eq.{}&select=addon_id,acquired_at", sess.user_id))
        .header("apikey", &anon)
        .bearer_auth(&sess.access_token)
        .send()
        .await
    {
        if let Ok(arows) = resp.json::<Vec<serde_json::Value>>().await {
            for ar in arows {
                if let Some(id) = ar.get("addon_id").and_then(|v| v.as_str()) {
                    addons.push(Addon { id: id.to_string(), label: id.to_string(), acquired_at: now() });
                }
            }
        }
    }

    Some(RemoteState { tier, status, current_period_end, addons, email: sess.email })
}

/// Build the account view, syncing from Supabase when signed in and caching the
/// result to `account.json`.
async fn build_view(config_dir: &Path) -> serde_json::Value {
    let mut acc = AccountManager::load(config_dir);
    let mut status = "local".to_string();
    let mut current_period_end: Option<String> = None;
    let mut signed_in = false;

    if let Some(rs) = sync_from_supabase(config_dir).await {
        signed_in = true;
        acc.tier = rs.tier;
        status = rs.status;
        current_period_end = rs.current_period_end;
        if rs.email.is_some() {
            acc.email = rs.email;
        }
        for ra in rs.addons {
            if !acc.addons.iter().any(|a| a.id == ra.id) {
                acc.addons.push(ra);
            }
        }
        let _ = AccountManager::save(config_dir, &acc); // cache for offline
    }

    let ent = entitlements_for(&acc);
    let on_trial = trial_active(&acc);
    serde_json::json!({
        "account": acc,
        "tier_label": if on_trial { "Free Trial".to_string() } else { acc.tier.label().to_string() },
        "tier_price_usd": acc.tier.monthly_price_usd(),
        "entitlements": ent,
        "status": if on_trial { "trialing".to_string() } else { status },
        "current_period_end": current_period_end,
        "signed_in": signed_in,
        "trial_active": on_trial,
        "trial_ends_at": acc.trial_ends_at,
        "trial_used": acc.trial_used,
    })
}

/// Start the one-time 1-day free trial — unlimited prompts + all features
/// (including security) for 24h. Idempotent: refuses if already used.
/// Also syncs Supabase `subscriptions.status=trialing` so the AMD cloud gateway
/// grants access (otherwise the VPS returns HTTP 402 while the IDE looks entitled).
#[tauri::command]
pub async fn account_start_trial(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let mut acc = AccountManager::load(&state.config_dir);
    if acc.trial_used {
        return Err("Your free trial has already been used.".to_string());
    }

    if let Some(sess) = crate::auth::valid_session(&state.config_dir).await {
        let site = site_base(&state.config_dir);
        let resp = reqwest::Client::new()
            .post(format!("{site}/api/start-trial"))
            .bearer_auth(&sess.access_token)
            .header("Content-Type", "application/json")
            .send()
            .await
            .map_err(|e| format!("Could not reach billing API: {e}"))?;
        let ok = resp.status().is_success();
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        if !ok {
            let msg = body
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Could not start trial on server.");
            return Err(msg.to_string());
        }
    }

    acc.trial_ends_at = Some(now() + 86_400); // 24 hours
    acc.trial_used = true;
    AccountManager::save(&state.config_dir, &acc)?;
    Ok(build_view(&state.config_dir).await)
}

/// Force a re-sync from Supabase (call after returning from checkout).
#[tauri::command]
pub async fn account_sync(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    Ok(build_view(&state.config_dir).await)
}

/// Open the website QR Ph checkout for `tier` (live billing path). Requires sign-in
/// so `/pay` can use the same Supabase session as the browser account page.
#[tauri::command]
pub async fn account_subscribe(
    state: State<'_, EditorState>,
    tier: String,
) -> Result<serde_json::Value, String> {
    let _sess = crate::auth::valid_session(&state.config_dir)
        .await
        .ok_or_else(|| "Sign in first to subscribe.".to_string())?;
    let allowed = ["pro_developer", "security_researcher", "enterprise"];
    if !allowed.contains(&tier.as_str()) {
        return Err(format!("unknown tier: {tier}"));
    }
    let site = site_base(&state.config_dir).trim_end_matches('/').to_string();
    // Website checkout uses cf_session (browser) — not the IDE session. Send users
    // through /account first so they sign in on the site, then land on /pay.
    let pay_path = format!("/pay?kind=subscription&tier={tier}");
    let url = format!(
        "{site}/account?next={}",
        urlencoding::encode(&pay_path)
    );
    let _ = open_in_browser(&url);
    Ok(serde_json::json!({ "checkout_url": url, "method": "qrph" }))
}

// ─────────────────────────────────────────────────────────────────────────────
// Usage metering / quota (local-authoritative MVP; best-effort Supabase mirror)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct Usage {
    month: String, // "YYYY-MM"
    month_count: u32,
    day: String, // "YYYY-MM-DD"
    day_count: u32,
    #[serde(default)]
    tokens_month: u64, // input+output tokens this month
}

fn usage_path(config_dir: &Path) -> PathBuf {
    config_dir.join("usage.json")
}

fn load_usage(config_dir: &Path) -> Usage {
    std::fs::read_to_string(usage_path(config_dir))
        .ok()
        .and_then(|c| serde_json::from_str::<Usage>(&c).ok())
        .unwrap_or_default()
}

fn save_usage(config_dir: &Path, u: &Usage) {
    let _ = std::fs::create_dir_all(config_dir);
    if let Ok(j) = serde_json::to_string_pretty(u) {
        let _ = std::fs::write(usage_path(config_dir), j);
    }
}

/// Add the token count of a completed AI turn to the monthly token meter. Called
/// by the frontend after each turn (input+output). Best-effort Supabase mirror.
#[tauri::command]
pub async fn account_add_tokens(state: State<'_, EditorState>, tokens: u64) -> Result<(), String> {
    let month = chrono::Local::now().format("%Y-%m").to_string();
    let mut u = load_usage(&state.config_dir);
    if u.month != month {
        u.month = month.clone();
        u.month_count = 0;
        u.tokens_month = 0;
    }
    u.tokens_month = u.tokens_month.saturating_add(tokens);
    save_usage(&state.config_dir, &u);

    if let Some(sess) = crate::auth::valid_session(&state.config_dir).await {
        let (url, anon) = crate::auth::supabase_config(&state.config_dir);
        if !url.is_empty() {
            let (period, toks, uid, token) = (month, u.tokens_month, sess.user_id.clone(), sess.access_token.clone());
            tauri::async_runtime::spawn(async move {
                let _ = reqwest::Client::new()
                    .post(format!("{url}/rest/v1/usage_counters?on_conflict=user_id,period_yyyymm"))
                    .header("apikey", &anon)
                    .bearer_auth(&token)
                    .header("Content-Type", "application/json")
                    .header("Prefer", "resolution=merge-duplicates")
                    .json(&serde_json::json!([{ "user_id": uid, "period_yyyymm": period, "tokens": toks }]))
                    .send()
                    .await;
            });
        }
    }
    Ok(())
}

/// Check the current quota and, if allowed, increment usage. Called by the
/// frontend before each AI turn (same pattern as the Bug-Bounty ToS gate). Tier
/// limits come from the cached account (fast, no network on the hot path); a
/// best-effort Supabase mirror runs in the background when signed in.
#[tauri::command]
pub async fn account_check_and_count(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let acc = AccountManager::load(&state.config_dir);
    let ent = entitlements_for(&acc); // 0 == unlimited
    let (monthly, daily) = (ent.monthly_requests, ent.daily_requests);

    let month = chrono::Local::now().format("%Y-%m").to_string();
    let day = chrono::Local::now().format("%Y-%m-%d").to_string();
    let mut u = load_usage(&state.config_dir);
    if u.month != month {
        u.month = month.clone();
        u.month_count = 0;
    }
    if u.day != day {
        u.day = day.clone();
        u.day_count = 0;
    }

    if daily > 0 && u.day_count >= daily {
        return Ok(serde_json::json!({
            "allowed": false, "reason": "daily",
            "used_day": u.day_count, "limit_day": daily,
            "used_month": u.month_count, "limit_month": monthly,
            "tier": acc.tier.label(),
        }));
    }
    if monthly > 0 && u.month_count >= monthly {
        return Ok(serde_json::json!({
            "allowed": false, "reason": "monthly",
            "used_month": u.month_count, "limit_month": monthly,
            "tier": acc.tier.label(),
        }));
    }

    u.day_count += 1;
    u.month_count += 1;
    save_usage(&state.config_dir, &u);

    // Best-effort Supabase mirror (does not gate the turn).
    if let Some(sess) = crate::auth::valid_session(&state.config_dir).await {
        let (url, anon) = crate::auth::supabase_config(&state.config_dir);
        if !url.is_empty() {
            let (period, count, uid, token) = (month.clone(), u.month_count, sess.user_id.clone(), sess.access_token.clone());
            tauri::async_runtime::spawn(async move {
                let _ = reqwest::Client::new()
                    .post(format!("{url}/rest/v1/usage_counters?on_conflict=user_id,period_yyyymm"))
                    .header("apikey", &anon)
                    .bearer_auth(&token)
                    .header("Content-Type", "application/json")
                    .header("Prefer", "resolution=merge-duplicates")
                    .json(&serde_json::json!([{ "user_id": uid, "period_yyyymm": period, "count": count }]))
                    .send()
                    .await;
            });
        }
    }

    Ok(serde_json::json!({
        "allowed": true,
        "used_day": u.day_count, "limit_day": daily,
        "used_month": u.month_count, "limit_month": monthly,
        "tier": acc.tier.label(),
    }))
}

/// Open the website billing/account page in the system browser. This is where
/// card capture + add-on (MiMo) purchases complete reliably (PayMongo.js).
#[tauri::command]
pub async fn account_open_billing(state: State<'_, EditorState>) -> Result<(), String> {
    let url = format!("{}/account", site_base(&state.config_dir));
    open_in_browser(&url)
}

/// Read-only usage + tier snapshot for the status-bar chip (does NOT increment,
/// unlike `account_check_and_count`). Rolls stale counters in the view only.
#[tauri::command]
pub async fn account_usage(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let acc = AccountManager::load(&state.config_dir);
    let ent = entitlements_for(&acc);
    let month = chrono::Local::now().format("%Y-%m").to_string();
    let day = chrono::Local::now().format("%Y-%m-%d").to_string();
    let u = load_usage(&state.config_dir);
    Ok(serde_json::json!({
        "tier": if trial_active(&acc) { "Free Trial".to_string() } else { acc.tier.label().to_string() },
        "trial_active": trial_active(&acc),
        "used_month": if u.month == month { u.month_count } else { 0 },
        "limit_month": ent.monthly_requests,
        "used_day": if u.day == day { u.day_count } else { 0 },
        "limit_day": ent.daily_requests,
        "used_tokens": if u.month == month { u.tokens_month } else { 0 },
        "limit_tokens": ent.monthly_tokens,
    }))
}

/// Cross-platform "open URL in the system browser" (default browser, not the
/// in-app webview) — used to send the user to the PayMongo checkout page.
fn open_in_browser(url: &str) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn().map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open").arg(url).spawn().map_err(|e| e.to_string())?;
    }
    Ok(())
}
