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
        }
    }
}

/// Derived feature + quota entitlements for a given tier (+ add-ons). Kept as a
/// pure function of the account so it can be recomputed anywhere (and later
/// overridden by the billing backend) without storing stale data.
#[derive(Debug, Clone, Serialize)]
pub struct Entitlements {
    pub daily_requests: u32,
    pub monthly_requests: u32,
    pub features: Vec<String>,
}

fn entitlements_for(account: &Account) -> Entitlements {
    let (daily, monthly, mut features): (u32, u32, Vec<&str>) = match account.tier {
        Tier::Community => (50, 1_500, vec!["editor", "agentic_basic", "local_models"]),
        Tier::ProDeveloper => (0, 5_000, vec!["editor", "agentic", "neural_vfs", "local_models", "cloud_models"]),
        Tier::SecurityResearcher => (0, 20_000, vec![
            "editor", "agentic", "neural_vfs", "local_models", "cloud_models",
            "bug_bounty", "vuln_hunt", "web_audit",
        ]),
        Tier::Enterprise => (0, 0 /* custom */, vec![
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

/// Full account view (account + derived entitlements + ToS status flags).
#[tauri::command]
pub async fn account_get(state: State<'_, EditorState>) -> Result<serde_json::Value, String> {
    let acc = AccountManager::load(&state.config_dir);
    let ent = entitlements_for(&acc);
    Ok(serde_json::json!({
        "account": acc,
        "tier_label": acc.tier.label(),
        "tier_price_usd": acc.tier.monthly_price_usd(),
        "entitlements": ent,
    }))
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
