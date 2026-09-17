//! Sentinel agent tools: expose the FlutterSentinel/SecuritySentinel high-value
//! unit tools to the agent loop. Pure analysis (jwt / crypto) runs anywhere;
//! engine-backed tools (secrets scan, PoC/report write, sidecars, work DB) use
//! the globally registered `SentinelEngine`.

use super::registry::AiTools;
use anyhow::{anyhow, Result};
use serde_json::{json, Value};

fn arg<'a>(a: &'a Value, key: &str) -> Option<&'a str> {
    a.get(key).and_then(|v| v.as_str())
}

fn arg_owned(a: &Value, key: &str) -> Option<String> {
    arg(a, key).map(|s| s.to_string())
}

impl AiTools {
    pub(crate) async fn handle_sentinel_tool(&self, name: &str, args: Value) -> Result<Value> {
        match name {
            // ── Pure analysis (no engine needed) ──
            "sentinel_analyze_jwt" => {
                let token = arg(&args, "token").ok_or_else(|| anyhow!("missing token"))?;
                let secret = arg_owned(&args, "provided_secret");
                crate::domain::sentinel::jwt::analyze_jwt(token, secret.as_deref())
                    .map_err(|e| anyhow!(e))
            }

            "sentinel_forge_jwt" => {
                let payload = args.get("payload").cloned().unwrap_or(json!({}));
                let secret = arg(&args, "secret").unwrap_or("");
                let alg = arg(&args, "alg").unwrap_or("HS256");
                crate::domain::sentinel::jwt::forge_jwt(payload, secret, alg)
                    .map_err(|e| anyhow!(e))
            }

            "sentinel_rsa_recover" => {
                let n_hex = arg(&args, "n_hex").ok_or_else(|| anyhow!("missing n_hex"))?;
                let e_hex = arg(&args, "e_hex").unwrap_or("10001");
                crate::domain::sentinel::crypto::rsa_recover_private_key(n_hex, e_hex)
                    .map_err(|e| anyhow!(e))
            }

            "sentinel_analyze_crypto" => {
                let text = arg(&args, "text").ok_or_else(|| anyhow!("missing text"))?;
                Ok(json!({ "hits": crate::domain::sentinel::crypto::analyze_crypto_text(text) }))
            }

            // ── Engine-backed tools ──
            "sentinel_scan_secrets" => {
                let eng = need_engine()?;
                let text = arg(&args, "text").ok_or_else(|| anyhow!("missing text"))?;
                Ok(json!({ "secrets": eng.scan_secrets(text) }))
            }

            "sentinel_validate_secret" => {
                let eng = need_engine()?;
                let kind = crate::domain::sentinel::secrets::SecretKind::from_label(
                    arg(&args, "kind").unwrap_or("generic"),
                )
                .unwrap_or(crate::domain::sentinel::secrets::SecretKind::Generic);
                let value = arg(&args, "value").ok_or_else(|| anyhow!("missing value"))?;
                Ok(eng.validate_secret(kind, value))
            }

            "sentinel_gen_poc" => {
                let eng = need_engine()?;
                let kind = crate::domain::sentinel::poc::PoCKinds::from_label(
                    arg(&args, "kind").ok_or_else(|| anyhow!("missing kind"))?,
                )
                .ok_or_else(|| anyhow!("unsupported poc kind"))?;
                let opts = crate::domain::sentinel::poc::PoCOptions {
                    target: arg_owned(&args, "target")
                        .ok_or_else(|| anyhow!("missing target (scheme://host)"))?,
                    endpoint: arg_owned(&args, "endpoint").unwrap_or_else(|| "/".into()),
                    method: arg_owned(&args, "method").unwrap_or_else(|| "GET".into()),
                    param: arg_owned(&args, "param").unwrap_or_else(|| "q".into()),
                    auth_header: arg_owned(&args, "auth_header"),
                    extra: arg_owned(&args, "extra"),
                };
                eng.gen_poc(kind, &opts).map_err(|e| anyhow!(e))
            }

            "sentinel_make_report" => {
                let eng = need_engine()?;
                let platform = crate::domain::sentinel::report::ReportPlatform::from_label(
                    arg(&args, "platform").unwrap_or("generic"),
                )
                .unwrap_or(crate::domain::sentinel::report::ReportPlatform::Generic);
                let input = crate::domain::sentinel::report::ReportInput {
                    title: arg_owned(&args, "title").ok_or_else(|| anyhow!("missing title"))?,
                    severity: arg_owned(&args, "severity").unwrap_or_else(|| "medium".into()),
                    target: arg_owned(&args, "target").ok_or_else(|| anyhow!("missing target"))?,
                    cwe: arg_owned(&args, "cwe").unwrap_or_default(),
                    owasp: arg_owned(&args, "owasp").unwrap_or_default(),
                    description: arg_owned(&args, "description").unwrap_or_default(),
                    evidence: arg_owned(&args, "evidence").unwrap_or_default(),
                    remediation: arg_owned(&args, "remediation").unwrap_or_default(),
                    recommendation: arg_owned(&args, "recommendation").unwrap_or_default(),
                };
                eng.make_report(platform, &input).map_err(|e| anyhow!(e))
            }

            "sentinel_sidecar_status" => {
                let eng = need_engine()?;
                Ok(json!({ "sidecars": eng.all_sidecar_status() }))
            }

            "sentinel_sidecar_start" | "sentinel_sidecar_stop" => {
                let eng = need_engine()?;
                let label = arg(&args, "kind").ok_or_else(|| anyhow!("missing kind"))?;
                let kind = crate::domain::sentinel::sidecars::SidecarKind::from_label(label)
                    .ok_or_else(|| anyhow!("unknown sidecar: {label}"))?;
                if name == "sentinel_sidecar_start" {
                    eng.spawn_sidecar(kind).map_err(|e| anyhow!(e))
                } else {
                    eng.stop_sidecar(kind).map_err(|e| anyhow!(e))
                }
            }

            "sentinel_stats" => Ok(need_engine()?.stats()),

            "sentinel_list_findings" => {
                let eng = need_engine()?;
                let rows = eng
                    .db()
                    .lock()
                    .unwrap()
                    .list_findings(arg(&args, "target_id"))
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "findings": rows }))
            }

            "sentinel_list_targets" => {
                let eng = need_engine()?;
                let rows = eng
                    .db()
                    .lock()
                    .unwrap()
                    .list_targets()
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "targets": rows }))
            }

            "sentinel_add_finding" => {
                let eng = need_engine()?;
                use crate::domain::sentinel::db::FindingRow;
                let mut f = FindingRow::new(
                    arg_owned(&args, "target_id").ok_or_else(|| anyhow!("missing target_id"))?,
                    arg_owned(&args, "title").ok_or_else(|| anyhow!("missing title"))?,
                );
                f.severity = arg_owned(&args, "severity").unwrap_or_else(|| "info".into());
                f.cwe = arg_owned(&args, "cwe").unwrap_or_default();
                f.owasp = arg_owned(&args, "owasp").unwrap_or_default();
                f.description = arg_owned(&args, "description").unwrap_or_default();
                f.evidence = arg_owned(&args, "evidence").unwrap_or_default();
                let row = eng
                    .db()
                    .lock()
                    .unwrap()
                    .create_finding(&f)
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "finding": row }))
            }

            "sentinel_create_target" => {
                let eng = need_engine()?;
                let name = arg_owned(&args, "name").ok_or_else(|| anyhow!("missing name"))?;
                let row = eng
                    .db()
                    .lock()
                    .unwrap()
                    .create_target(
                        &name,
                        arg(&args, "root_domain"),
                        arg(&args, "platform"),
                        arg(&args, "scope"),
                        arg(&args, "program"),
                        arg(&args, "scope_type"),
                    )
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "target": row }))
            }

            // ── Mobile bounty drive ──
            "sentinel_mobile_discover" => {
                let eng = need_engine()?;
                Ok(json!({ "uploads": eng.discover_mobile_apps() }))
            }

            "sentinel_mobile_assets" => {
                let eng = need_engine()?;
                Ok(json!({ "assets": eng.list_mobile_assets() }))
            }

            "sentinel_mobsf_status" => {
                let eng = need_engine()?;
                Ok(json!({ "port_open": eng.mobsf_up(), "api": eng.mobsf_ping().await }))
            }

            "sentinel_mobile_analyze" => {
                let eng = need_engine()?;
                let apk = arg(&args, "apk").ok_or_else(|| anyhow!("missing apk path"))?;
                eng.analyze_mobile(apk).await.map_err(|e| anyhow!(e))
            }

            "sentinel_mobile_pull" => {
                let eng = need_engine()?;
                let package = arg(&args, "package").ok_or_else(|| anyhow!("missing package"))?;
                eng.pull_mobile_apk(package).map_err(|e| anyhow!(e))
            }

            "sentinel_mobile_delete" => {
                let eng = need_engine()?;
                let id = arg(&args, "id").ok_or_else(|| anyhow!("missing asset id"))?;
                eng.delete_mobile_asset(id).map_err(|e| anyhow!(e))
            }

            "sentinel_gen_frida_script" => {
                let eng = need_engine()?;
                use crate::domain::sentinel::mobile::FridaScriptKind;
                let kind = FridaScriptKind::from_label(arg(&args, "kind").unwrap_or("ssl_unpin"))
                    .ok_or_else(|| anyhow!("unknown frida kind"))?;
                eng.gen_frida_script(kind).map_err(|e| anyhow!(e))
            }

            "sentinel_jb_status" => Ok(need_engine()?.jb_status().await),

            "sentinel_jb_devices" => need_engine()?.jb_devices().await.map_err(|e| anyhow!(e)),

            "sentinel_jb_apps" => {
                let eng = need_engine()?;
                let udid = arg(&args, "udid").map(|s| s.to_string());
                eng.jb_apps(udid).await.map_err(|e| anyhow!(e))
            }

            "sentinel_jb_live_scan" => {
                let eng = need_engine()?;
                use crate::domain::sentinel::jbops::LiveScriptKind;
                let target = arg(&args, "target")
                    .ok_or_else(|| anyhow!("missing target (process or bundle id)"))?;
                let kind = LiveScriptKind::from_label(arg(&args, "kind").unwrap_or("enum_classes"))
                    .ok_or_else(|| anyhow!("unknown live scan kind"))?;
                let filter = arg(&args, "filter").unwrap_or(".*");
                eng.jb_live_scan(target, kind, filter)
                    .await
                    .map_err(|e| anyhow!(e))
            }

            "sentinel_jb_dump" => {
                let eng = need_engine()?;
                let bundle = arg(&args, "bundle").ok_or_else(|| anyhow!("missing bundle id"))?;
                let udid = arg(&args, "udid").map(|s| s.to_string());
                eng.jb_dump_to_ipa(udid, bundle)
                    .await
                    .map_err(|e| anyhow!(e))
            }

            other => Err(anyhow!("Unknown sentinel tool: {other}")),
        }
    }
}

/// The engine must have been booted (Tauri setup / gpui main) for engine-backed tools.
fn need_engine() -> Result<std::sync::Arc<crate::domain::sentinel::SentinelEngine>> {
    crate::domain::sentinel::engine().ok_or_else(|| {
        anyhow!("sentinel engine not booted — start the IDE normally (setup boots it)")
    })
}
