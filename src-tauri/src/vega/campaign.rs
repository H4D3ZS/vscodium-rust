//! Vega scan campaign — crawl → injection + passive modules → deduped alerts.

use crate::vega::alerts::AlertRegistry;
use crate::vega::crawler::{crawl, CrawlConfig};
use crate::vega::engine::ScanEngine;
use crate::vega::js_runtime::{JsModuleHost, ModuleKind};
use crate::vega::model::{Alert, PathState};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Semaphore;

/// Fast default injection pack for web pentest triage.
const DEFAULT_INJECTION: &[&str] = &[
    "injection/sql-text-injection.js",
    "injection/xss-injection.js",
    "injection/command-injection.js",
    "modern/ssrf-probe.js",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegaModuleInfo {
    pub id: String,
    pub name: String,
    pub category: String,
    pub kind: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VegaScanOptions {
    pub target_url: String,
    pub authorized: bool,
    pub max_pages: Option<usize>,
    pub max_depth: Option<u32>,
    pub injection_modules: Option<Vec<String>>,
    pub run_passive: Option<bool>,
    /// Opt-in: run a local-LLM second pass to flag likely false positives.
    pub ai_triage: Option<bool>,
    pub ai_model: Option<String>,
    pub ollama_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VegaScanProgress {
    pub phase: String,
    pub paths_total: usize,
    pub paths_done: usize,
    pub alerts_found: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VegaScanResult {
    pub target: String,
    pub paths_scanned: usize,
    pub modules_run: usize,
    pub alerts: Vec<Alert>,
    pub duration_ms: u64,
    /// Per-alert LLM triage verdicts (CONFIRMED / LIKELY / FALSE_POSITIVE),
    /// keyed by the same order as `alerts`. Empty when AI triage is off.
    #[serde(default)]
    pub ai_triage: Vec<String>,
}

pub fn modules_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/vega/scripts/modules")
}

pub fn list_modules() -> Vec<VegaModuleInfo> {
    let root = modules_root();
    let mut out = Vec::new();
    collect_modules(&root, &root, &mut out);
    out.sort_by(|a, b| a.category.cmp(&b.category).then(a.name.cmp(&b.name)));
    out
}

fn collect_modules(root: &PathBuf, dir: &PathBuf, out: &mut Vec<VegaModuleInfo>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for ent in entries.flatten() {
        let path = ent.path();
        if path.is_dir() {
            collect_modules(root, &path, out);
            continue;
        }
        if path.extension().and_then(|e| e.to_str()) != Some("js") {
            continue;
        }
        let Ok(src) = std::fs::read_to_string(&path) else { continue };
        let Ok(meta) = JsModuleHost::read_meta(&src) else { continue };
        let rel = path.strip_prefix(root).unwrap_or(&path);
        let id = rel.to_string_lossy().replace('\\', "/");
        let kind = match meta.kind() {
            ModuleKind::Injection => "injection",
            ModuleKind::ResponseProcessor => "passive",
            ModuleKind::Unknown => "other",
        };
        out.push(VegaModuleInfo {
            id: id.clone(),
            name: if meta.name.is_empty() {
                id
            } else {
                meta.name.clone()
            },
            category: meta.category.unwrap_or_else(|| "Modules".into()),
            kind: kind.into(),
            path: path.to_string_lossy().to_string(),
        });
    }
}

fn load_module_source(rel: &str) -> Option<String> {
    let p = modules_root().join(rel);
    std::fs::read_to_string(p).ok()
}

fn assert_authorized(opts: &VegaScanOptions) -> Result<(), String> {
    if !opts.authorized {
        return Err(
            "Authorized testing required — confirm you have permission to scan this target."
                .into(),
        );
    }
    if !opts.target_url.starts_with("http://") && !opts.target_url.starts_with("https://") {
        return Err("Target must be a valid http(s) URL".into());
    }
    Ok(())
}

pub async fn run_campaign(opts: VegaScanOptions) -> Result<VegaScanResult, String> {
    assert_authorized(&opts)?;
    let started = std::time::Instant::now();

    let registry = AlertRegistry::load_default().unwrap_or_default();
    let engine = Arc::new(ScanEngine::new(registry));

    let http = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .redirect(reqwest::redirect::Policy::limited(5))
        .user_agent("APEX-Vega/1.0 (authorized security testing)")
        .build()
        .map_err(|e| e.to_string())?;

    let crawl_cfg = CrawlConfig {
        seed: opts.target_url.clone(),
        max_pages: opts.max_pages.unwrap_or(32),
        max_depth: opts.max_depth.unwrap_or(2),
        same_host_only: true,
    };

    let paths = crawl(&http, &crawl_cfg).await?;
    let parametric: Vec<PathState> = paths.into_iter().filter(|p| p.is_parametric()).collect();

    let injection_rels: Vec<String> = opts
        .injection_modules
        .clone()
        .unwrap_or_else(|| DEFAULT_INJECTION.iter().map(|s| s.to_string()).collect());

    let mut sources: Vec<(String, String)> = Vec::new();
    for rel in &injection_rels {
        if let Some(src) = load_module_source(rel) {
            sources.push((rel.clone(), src));
        }
    }

    let sem = Arc::new(Semaphore::new(4));
    let mut alert_keys = HashSet::new();
    let mut all_alerts = Vec::new();
    let paths_scanned = parametric.len();

    let mut tasks = Vec::new();
    for ps in parametric {
        let eng = engine.clone();
        let mods = sources.clone();
        let permit = sem.clone().acquire_owned().await.map_err(|e| e.to_string())?;
        tasks.push(tokio::spawn(async move {
            let _p = permit;
            let mut local = Vec::new();
            for (rel, src) in mods {
                if let Ok(r) = eng.run_injection_module(&src, &ps).await {
                    local.extend(r.alerts);
                }
                let _ = rel;
            }
            local
        }));
    }

    for t in tasks {
        if let Ok(alerts) = t.await {
            for a in alerts {
                let dedupe = if a.key.is_empty() {
                    format!("{}:{}", a.type_key, a.resource)
                } else {
                    a.key.clone()
                };
                if alert_keys.insert(dedupe) {
                    all_alerts.push(a);
                }
            }
        }
    }

    if opts.run_passive.unwrap_or(true) {
        let resp = engine
            .fetch(&crate::vega::model::HttpRequest {
                uri: opts.target_url.clone(),
                ..Default::default()
            })
            .await?;
        let req = crate::vega::model::HttpRequest {
            uri: opts.target_url.clone(),
            ..Default::default()
        };
        let passive_dir = modules_root().join("response");
        if passive_dir.is_dir() {
            let mut passive_mods = Vec::new();
            if let Ok(entries) = std::fs::read_dir(&passive_dir) {
                for ent in entries.flatten().take(12) {
                    let p = ent.path();
                    if p.extension().and_then(|e| e.to_str()) == Some("js") {
                        if let Ok(src) = std::fs::read_to_string(&p) {
                            if !src.contains("importPackage") {
                                passive_mods.push((
                                    p.file_name().unwrap_or_default().to_string_lossy().into_owned(),
                                    src,
                                ));
                            }
                        }
                    }
                }
            }
            let passive = engine.run_all_passive(
                &passive_mods.iter().map(|(n, s)| (n.as_str(), s.as_str())).collect::<Vec<_>>(),
                &req,
                &resp,
            );
            for a in passive {
                let dedupe = if a.key.is_empty() {
                    format!("{}:{}", a.type_key, a.resource)
                } else {
                    a.key.clone()
                };
                if alert_keys.insert(dedupe) {
                    all_alerts.push(a);
                }
            }
        }
    }

    all_alerts.sort_by(|a, b| b.severity.cmp(&a.severity));

    // Optional local-LLM second pass: triage each finding as CONFIRMED / LIKELY /
    // FALSE_POSITIVE so the operator can prioritise real bugs first. Bounded and
    // best-effort — a model timeout never fails the scan.
    let ai_triage = if opts.ai_triage.unwrap_or(false) && !all_alerts.is_empty() {
        run_ai_triage(&all_alerts, &opts).await
    } else {
        Vec::new()
    };

    Ok(VegaScanResult {
        target: opts.target_url,
        paths_scanned,
        modules_run: sources.len(),
        alerts: all_alerts,
        duration_ms: started.elapsed().as_millis() as u64,
        ai_triage,
    })
}

/// Best-effort LLM triage of findings. Caps work to the top 25 alerts so a big
/// scan can't stall behind hundreds of model calls.
async fn run_ai_triage(alerts: &[Alert], opts: &VegaScanOptions) -> Vec<String> {
    use crate::vega::modern::{AiAssistConfig, VegaAiAssist};

    let mut cfg = AiAssistConfig::default();
    if let Some(m) = opts.ai_model.clone() {
        if !m.trim().is_empty() {
            cfg.model = m;
        }
    }
    if let Some(u) = opts.ollama_url.clone() {
        if !u.trim().is_empty() {
            cfg.ollama_url = u;
        }
    }
    let mut assist = VegaAiAssist::new(cfg);

    // Probe once. If Ollama is unreachable (complete-offline), disable the model
    // path so triage_finding uses the deterministic heuristic without paying a
    // failed-request round trip per alert.
    if !assist.reachable().await {
        assist.set_enabled(false);
    }

    let mut verdicts = Vec::with_capacity(alerts.len());
    for (i, a) in alerts.iter().enumerate() {
        if i >= 25 {
            verdicts.push("SKIPPED".into());
            continue;
        }
        let snippet: String = a.output.chars().take(600).collect();
        let verdict = assist
            .triage_finding(&a.type_key, &a.output, &snippet)
            .await
            .unwrap_or_else(|_| "UNKNOWN".into());
        verdicts.push(verdict);
    }
    verdicts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_injection_modules() {
        let mods = list_modules();
        assert!(mods.iter().any(|m| m.id.contains("sql-text-injection")));
    }
}
