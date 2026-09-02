//! Services domain: auth/account lifecycle.
//! (ai_auth webui-session + enterprise governance/audit modules exist on disk but
//! are intentionally left unwired — out of scope for the account restore.)

#[cfg(feature = "tauri")]
pub mod account;
#[cfg(feature = "tauri")]
pub mod auth;
#[cfg(feature = "tauri")]
pub mod enterprise_audit;
#[cfg(feature = "tauri")]
pub mod enterprise_governance;
