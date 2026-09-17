//! Tauri IPC layer for the Sentinel (FlutterSentinel) integration. Thin
//! wrappers: parse args, call `crate::domain::sentinel`, map errors to String.

use crate::domain::sentinel::poc::{PoCKinds, PoCOptions};
use crate::domain::sentinel::report::{ReportInput, ReportPlatform};
use crate::domain::sentinel::secrets::SecretKind;
use crate::domain::sentinel::sidecars::SidecarKind;
use crate::domain::sentinel::{SentinelEngine, SentinelSettings};
use serde_json::{json, Value};
use std::sync::Arc;
use tauri::State;

/// Managed engine (set up in `lib.rs` setup). Commands never construct it.
pub type Sent<'a> = State<'a, Arc<SentinelEngine>>;

fn engine(state: &Sent<'_>) -> Arc<SentinelEngine> {
    state.inner().clone()
}

// ═══ Settings ═══

#[tauri::command]
pub async fn sentinel_get_settings(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    serde_json::to_value(eng.settings()).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_save_settings(state: Sent<'_>, patch: Value) -> Result<Value, String> {
    let eng = engine(&state);
    let mut settings = eng.settings();
    if let Some(obj) = patch.as_object() {
        for (k, v) in obj {
            let s = v.as_str().map(|s| s.to_string());
            match k.as_str() {
                "sentinel_root" => settings.sentinel_root = s,
                "work_dir" => settings.work_dir = s.unwrap_or_default(),
                "fbhbot_dir" => settings.fbhbot_dir = s.unwrap_or_default(),
                "backend_dir" => settings.backend_dir = s.unwrap_or_default(),
                "mobsf_dir" => settings.mobsf_dir = s.unwrap_or_default(),
                "mcp_dir" => settings.mcp_dir = s.unwrap_or_default(),
                "ai_hunter_dir" => settings.ai_hunter_dir = s.unwrap_or_default(),
                "node_bin" => settings.node_bin = s,
                "python_bin" => settings.python_bin = s,
                "auto_start_sidecars" => {
                    settings.auto_start_sidecars = v.as_bool().unwrap_or(false)
                }
                "live_secret_check" => settings.live_secret_check = v.as_bool().unwrap_or(false),
                "default_program" => settings.default_program = s.unwrap_or_default(),
                _ => {}
            }
        }
    }
    eng.update_settings(|s| {
        *s = settings;
    })
    .map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

// ═══ Dashboard / DB ═══

#[tauri::command]
pub async fn sentinel_stats(state: Sent<'_>) -> Result<Value, String> {
    Ok(engine(&state).stats())
}

#[tauri::command]
pub async fn sentinel_list_targets(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    let rows = guard.list_targets().map_err(|e| e.to_string())?;
    Ok(json!({ "targets": rows }))
}

#[tauri::command]
pub async fn sentinel_create_target(
    state: Sent<'_>,
    name: String,
    root_domain: Option<String>,
    platform: Option<String>,
    scope: Option<String>,
    program: Option<String>,
    scope_type: Option<String>,
) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    guard
        .create_target(
            &name,
            root_domain.as_deref(),
            platform.as_deref(),
            scope.as_deref(),
            program.as_deref(),
            scope_type.as_deref(),
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_update_target(
    state: Sent<'_>,
    id: String,
    name: Option<String>,
    root_domain: Option<String>,
    program: Option<String>,
    status: Option<String>,
) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    guard
        .update_target(
            &id,
            name.as_deref(),
            root_domain.as_deref(),
            program.as_deref(),
            status.as_deref(),
        )
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_delete_target(state: Sent<'_>, id: String) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    guard.delete_target(&id).map_err(|e| e.to_string())?;
    Ok(json!({ "deleted": id }))
}

#[tauri::command]
pub async fn sentinel_list_findings(
    state: Sent<'_>,
    target_id: Option<String>,
) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    let rows = guard
        .list_findings(target_id.as_deref())
        .map_err(|e| e.to_string())?;
    Ok(json!({ "findings": rows }))
}

#[tauri::command]
pub async fn sentinel_create_finding(
    state: Sent<'_>,
    target_id: String,
    title: String,
    severity: Option<String>,
    cwe: Option<String>,
    owasp: Option<String>,
    description: Option<String>,
    evidence: Option<String>,
) -> Result<Value, String> {
    use crate::domain::sentinel::db::FindingRow;
    let mut f = FindingRow::new(target_id, title);
    f.severity = severity.unwrap_or_else(|| "info".into());
    f.cwe = cwe.unwrap_or_default();
    f.owasp = owasp.unwrap_or_default();
    f.description = description.unwrap_or_default();
    f.evidence = evidence.unwrap_or_default();
    f.status = "open".into();
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    guard.create_finding(&f).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_update_finding_status(
    state: Sent<'_>,
    id: String,
    status: String,
) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    guard
        .update_finding_status(&id, &status)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_list_reports(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    let db = eng.db();
    let guard = db.lock().unwrap();
    let rows = guard.list_reports().map_err(|e| e.to_string())?;
    Ok(json!({ "reports": rows }))
}

// ═══ JWT ═══

#[tauri::command]
pub async fn sentinel_analyze_jwt(
    _state: Sent<'_>,
    token: String,
    provided_secret: Option<String>,
) -> Result<Value, String> {
    crate::domain::sentinel::jwt::analyze_jwt(&token, provided_secret.as_deref())
}

#[tauri::command]
pub async fn sentinel_forge_jwt(
    _state: Sent<'_>,
    payload: Value,
    secret: String,
    alg: String,
) -> Result<Value, String> {
    crate::domain::sentinel::jwt::forge_jwt(payload, &secret, &alg)
}

// ═══ Crypto ═══

#[tauri::command]
pub async fn sentinel_analyze_crypto_text(_state: Sent<'_>, text: String) -> Result<Value, String> {
    let hits = crate::domain::sentinel::crypto::analyze_crypto_text(&text);
    Ok(json!({ "hits": hits }))
}

#[tauri::command]
pub async fn sentinel_rsa_recover(
    _state: Sent<'_>,
    n_hex: String,
    e_hex: Option<String>,
) -> Result<Value, String> {
    let e = e_hex.unwrap_or_else(|| "10001".to_string());
    crate::domain::sentinel::crypto::rsa_recover_private_key(&n_hex, &e)
}

// ═══ Secrets ═══

fn parse_secret_kind(s: &str) -> Result<SecretKind, String> {
    match s.to_lowercase().as_str() {
        "github" => Ok(SecretKind::Github),
        "slack" => Ok(SecretKind::Slack),
        "stripe" => Ok(SecretKind::Stripe),
        "aws" => Ok(SecretKind::Aws),
        "openai" => Ok(SecretKind::Openai),
        "google" => Ok(SecretKind::Google),
        "twilio" => Ok(SecretKind::Twilio),
        "generic" => Ok(SecretKind::Generic),
        _ => Err(format!("unknown secret kind: {s}")),
    }
}

#[tauri::command]
pub async fn sentinel_scan_secrets(state: Sent<'_>, text: String) -> Result<Value, String> {
    Ok(json!({ "secrets": engine(&state).scan_secrets(&text) }))
}

#[tauri::command]
pub async fn sentinel_validate_secret(
    state: Sent<'_>,
    kind: String,
    value: String,
) -> Result<Value, String> {
    let k = parse_secret_kind(&kind)?;
    Ok(engine(&state).validate_secret(k, &value))
}

#[tauri::command]
pub async fn sentinel_live_secret_check(
    state: Sent<'_>,
    kind: String,
    value: String,
) -> Result<Value, String> {
    let eng = engine(&state);
    if !eng.settings().live_secret_check {
        return Err("live_secret_check is disabled in sentinel settings (opt-in only)".to_string());
    }
    let k = parse_secret_kind(&kind)?;
    let verdict = crate::domain::sentinel::secrets::check_live(k, &value).await;
    Ok(json!({ "kind": k.to_string(), "verdict": verdict }))
}

// ═══ PoC + Reports ═══

#[tauri::command]
pub async fn sentinel_gen_poc(
    state: Sent<'_>,
    kind: String,
    opts: PoCOptions,
) -> Result<Value, String> {
    let k = PoCKinds::from_label(&kind).ok_or_else(|| format!("unknown poc kind: {kind}"))?;
    engine(&state).gen_poc(k, &opts).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_make_report(
    state: Sent<'_>,
    platform: String,
    input: ReportInput,
) -> Result<Value, String> {
    let p = ReportPlatform::from_label(&platform)
        .ok_or_else(|| format!("unknown platform: {platform}"))?;
    engine(&state)
        .make_report(p, &input)
        .map_err(|e| e.to_string())
}

// ═══ Sidecars ═══

fn parse_sidecar(s: &str) -> Result<SidecarKind, String> {
    SidecarKind::from_label(s).ok_or_else(|| format!("unknown sidecar: {s}"))
}

#[tauri::command]
pub async fn sentinel_sidecars_status(state: Sent<'_>) -> Result<Value, String> {
    Ok(json!({ "sidecars": engine(&state).all_sidecar_status() }))
}

#[tauri::command]
pub async fn sentinel_sidecar_start(state: Sent<'_>, kind: String) -> Result<Value, String> {
    let k = parse_sidecar(&kind)?;
    engine(&state).spawn_sidecar(k).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_sidecar_stop(state: Sent<'_>, kind: String) -> Result<Value, String> {
    let k = parse_sidecar(&kind)?;
    engine(&state).stop_sidecar(k).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_bootstrap_sidecars(state: Sent<'_>) -> Result<Value, String> {
    engine(&state)
        .bootstrap_sidecars()
        .map_err(|e| e.to_string())
}

/// Sentinel domain boot report (config dir, detected root, work dir).
#[tauri::command]
pub async fn sentinel_boot_info(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    let settings = eng.settings();
    Ok(json!({
        "config_dir": eng.config_dir().to_string_lossy(),
        "work_dir": eng.work_dir().to_string_lossy(),
        "logs_dir": eng.logs_dir().to_string_lossy(),
        "sentinel_root": settings.sentinel_root,
        "auto_start_sidecars": settings.auto_start_sidecars,
    }))
}

#[tauri::command]
pub async fn sentinel_reload_settings(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    let loaded = SentinelSettings::load(eng.config_dir());
    eng.update_settings(|s| {
        *s = loaded;
    })
    .map_err(|e| e.to_string())?;
    Ok(json!({ "ok": true }))
}

// ═══ Mobile bounty drive (acquire → MobSF/local scan → findings) ═══

#[tauri::command]
pub async fn sentinel_mobile_discover(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    Ok(json!({ "uploads": eng.discover_mobile_apps() }))
}

#[tauri::command]
pub async fn sentinel_mobile_assets(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    Ok(json!({ "assets": eng.list_mobile_assets() }))
}

#[tauri::command]
pub async fn sentinel_mobile_analyze(state: Sent<'_>, apk: String) -> Result<Value, String> {
    let eng = engine(&state);
    eng.analyze_mobile(&apk).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_mobile_pull(state: Sent<'_>, package: String) -> Result<Value, String> {
    let eng = engine(&state);
    eng.pull_mobile_apk(&package).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_mobile_delete(state: Sent<'_>, id: String) -> Result<Value, String> {
    let eng = engine(&state);
    eng.delete_mobile_asset(&id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_mobsf_status(state: Sent<'_>) -> Result<Value, String> {
    let eng = engine(&state);
    let ping = eng.mobsf_ping().await;
    Ok(json!({ "port_open": eng.mobsf_up(), "api": ping }))
}

#[tauri::command]
pub async fn sentinel_gen_frida_script(state: Sent<'_>, kind: String) -> Result<Value, String> {
    use crate::domain::sentinel::mobile::FridaScriptKind;
    let k =
        FridaScriptKind::from_label(&kind).ok_or_else(|| format!("unknown frida kind: {kind}"))?;
    engine(&state)
        .gen_frida_script(k)
        .map_err(|e| e.to_string())
}

// ── Jailbreak ops: live iPhone XR over usbmuxd/frida ──

#[tauri::command]
pub async fn sentinel_jb_status(state: Sent<'_>) -> Result<Value, String> {
    Ok(engine(&state).jb_status().await)
}

#[tauri::command]
pub async fn sentinel_jb_devices(state: Sent<'_>) -> Result<Value, String> {
    engine(&state).jb_devices().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_jb_apps(state: Sent<'_>, udid: Option<String>) -> Result<Value, String> {
    engine(&state)
        .jb_apps(udid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_jb_live_scan(
    state: Sent<'_>,
    target: String,
    kind: String,
    filter: String,
) -> Result<Value, String> {
    use crate::domain::sentinel::jbops::LiveScriptKind;
    let k =
        LiveScriptKind::from_label(&kind).ok_or_else(|| format!("unknown live scan: {kind}"))?;
    engine(&state)
        .jb_live_scan(&target, k, &filter)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_jb_tunnel(state: Sent<'_>, udid: String) -> Result<Value, String> {
    engine(&state)
        .jb_ensure_tunnel(&udid)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_jb_ssh(state: Sent<'_>, command: String) -> Result<Value, String> {
    engine(&state)
        .jb_ssh(&command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn sentinel_jb_dump(
    state: Sent<'_>,
    udid: Option<String>,
    bundle: String,
) -> Result<Value, String> {
    engine(&state)
        .jb_dump_to_ipa(udid, &bundle)
        .await
        .map_err(|e| e.to_string())
}
