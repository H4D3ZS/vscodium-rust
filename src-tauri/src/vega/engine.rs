//! APEX Vega HTTP scan engine (Phase 3).
//!
//! Orchestrates baseline fetch → injection plan collection → concurrent fuzz
//! requests → JS differential analysis.

use crate::vega::alerts::AlertRegistry;
use crate::vega::fingerprint::ResponseFingerprint;
use crate::vega::injection_host::{
    build_altered_request, InjectionModuleHost, PlanStep,
};
use crate::vega::js_runtime::{JsModuleHost, ModuleRunResult};
use crate::vega::model::{Alert, HttpRequest, HttpResponse, PathState};
use std::time::Duration;

const DEFAULT_TIMEOUT: Duration = Duration::from_secs(15);

/// Active/passive scan orchestrator.
pub struct ScanEngine {
    registry: AlertRegistry,
    client: reqwest::Client,
}

impl ScanEngine {
    pub fn new(registry: AlertRegistry) -> Self {
        let client = reqwest::Client::builder()
            .timeout(DEFAULT_TIMEOUT)
            .redirect(reqwest::redirect::Policy::limited(5))
            .user_agent("APEX-Vega/1.0 (authorized security testing)")
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        Self { registry, client }
    }

    pub fn with_client(registry: AlertRegistry, client: reqwest::Client) -> Self {
        Self { registry, client }
    }

    /// Run a passive response-processor module against one request/response pair.
    pub fn run_passive_module(
        &self,
        source: &str,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Result<ModuleRunResult, String> {
        JsModuleHost::new(self.registry.clone()).run_response_module(source, request, response)
    }

    /// Full injection-module pipeline for one path state.
    pub async fn run_injection_module(
        &self,
        source: &str,
        ps: &PathState,
    ) -> Result<ModuleRunResult, String> {
        let baseline_req = baseline_request(ps)?;
        let baseline_res = self.fetch(&baseline_req).await?;
        let baseline_fp = ResponseFingerprint::compute(&baseline_res);

        let host = InjectionModuleHost::new(self.registry.clone());
        let plan = host.collect_plan(source, ps, baseline_fp)?;

        if plan.is_empty() {
            return Ok(ModuleRunResult::default());
        }

        let max_index = plan
            .iter()
            .map(|s| match s {
                PlanStep::Altered { index, .. } | PlanStep::Request { index, .. } => *index,
            })
            .max()
            .unwrap_or(0);

        let mut saved_req = vec![HttpRequest::default(); (max_index as usize) + 1];
        let mut saved_res = vec![HttpResponse::default(); (max_index as usize) + 1];

        // Index 0 reserved for baseline when modules reference fingerprint comparisons.
        saved_req[0] = baseline_req;
        saved_res[0] = baseline_res;

        for step in &plan {
            match step {
                PlanStep::Altered {
                    payload,
                    append,
                    index,
                } => {
                    let req = build_altered_request(ps, payload, *append)?;
                    let res = self.fetch(&req).await?;
                    let i = *index as usize;
                    if i < saved_req.len() {
                        saved_req[i] = req;
                        saved_res[i] = res;
                    }
                }
                PlanStep::Request {
                    index,
                    method,
                    uri,
                    headers,
                    body,
                } => {
                    let mut req = HttpRequest {
                        method: method.clone(),
                        uri: uri.clone(),
                        headers: headers.clone(),
                        body: body.clone(),
                    };
                    if req.method.is_empty() {
                        req.method = "GET".into();
                    }
                    let res = self.fetch(&req).await?;
                    let i = *index as usize;
                    if i < saved_req.len() {
                        saved_req[i] = req;
                        saved_res[i] = res;
                    }
                }
            }
        }

        host.run_process_phase(source, ps, baseline_fp, &saved_req, &saved_res)
    }

    /// Run all response-processor modules against one pair; merge alerts.
    pub fn run_all_passive(
        &self,
        modules: &[(&str, &str)],
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Vec<Alert> {
        let host = JsModuleHost::new(self.registry.clone());
        let mut out = Vec::new();
        for (_name, source) in modules {
            if let Ok(r) = host.run_response_module(source, request, response) {
                out.extend(r.alerts);
            }
        }
        out
    }

    pub async fn fetch(&self, req: &HttpRequest) -> Result<HttpResponse, String> {
        let method = req.method.to_uppercase();
        let mut builder = self.client.request(
            reqwest::Method::from_bytes(method.as_bytes())
                .map_err(|e| format!("bad method: {e}"))?,
            &req.uri,
        );
        for (k, v) in &req.headers {
            builder = builder.header(k.as_str(), v.as_str());
        }
        if !req.body.is_empty() {
            builder = builder.body(req.body.clone());
        }

        match builder.send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let headers: Vec<(String, String)> = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| {
                        (
                            k.to_string(),
                            v.to_str().unwrap_or("").to_string(),
                        )
                    })
                    .collect();
                let body = resp.text().await.unwrap_or_default();
                Ok(HttpResponse {
                    status,
                    headers,
                    body,
                    fetch_fail: false,
                })
            }
            Err(e) => Ok(HttpResponse {
                status: 0,
                fetch_fail: true,
                body: e.to_string(),
                ..Default::default()
            }),
        }
    }
}

fn baseline_request(ps: &PathState) -> Result<HttpRequest, String> {
    Ok(HttpRequest {
        method: if ps.is_post_target {
            "POST".into()
        } else {
            "GET".into()
        },
        uri: ps.uri.clone(),
        body: if ps.is_post_target {
            ps.params
                .iter()
                .map(|p| {
                    format!(
                        "{}={}",
                        urlencoding::encode(&p.name),
                        urlencoding::encode(&p.value)
                    )
                })
                .collect::<Vec<_>>()
                .join("&")
        } else {
            String::new()
        },
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vega::model::{FuzzableParam, ParamLocation};
    use axum::{
        extract::Query,
        routing::get,
        Router,
    };
    use std::collections::HashMap;
    use std::net::SocketAddr;

    async fn sqli_handler(Query(params): Query<HashMap<String, String>>) -> String {
        let id = params.get("id").cloned().unwrap_or_default();
        if id.contains("1=1") {
            return "users: alice,bob,charlie".repeat(8);
        }
        if id.contains("1=2") {
            return "error: no results".into();
        }
        if id.contains("vega-xss-") {
            return format!("<html>{id}</html>");
        }
        format!("ok id={id}")
    }

    async fn start_test_server() -> (SocketAddr, tokio::task::JoinHandle<()>) {
        let app = Router::new().route("/sqli", get(sqli_handler));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind");
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        (addr, handle)
    }

    fn load_module(rel: &str) -> Option<String> {
        let p = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/vega/scripts/modules")
            .join(rel);
        std::fs::read_to_string(p).ok()
    }

    #[tokio::test]
    async fn sql_text_injection_detects_differential() {
        let Some(src) = load_module("injection/sql-text-injection.js") else {
            eprintln!("[vega] resources missing — skip");
            return;
        };

        let (addr, _srv) = start_test_server().await;
        let base = format!("http://{addr}/sqli?id=1");

        let ps = PathState {
            uri: base,
            is_post_target: false,
            params: vec![FuzzableParam {
                name: "id".into(),
                value: "1".into(),
                location: ParamLocation::Query,
            }],
            fuzz_index: Some(0),
            ..Default::default()
        };

        let registry = AlertRegistry::load_default().unwrap_or_default();
        let engine = ScanEngine::new(registry);
        let result = engine
            .run_injection_module(&src, &ps)
            .await
            .expect("injection run");

        assert!(
            result
                .alerts
                .iter()
                .any(|a| a.type_key == "vinfo-sql-inject"),
            "expected SQL injection alert, got: {:?}",
            result.alerts.iter().map(|a| &a.type_key).collect::<Vec<_>>()
        );
    }

    #[tokio::test]
    async fn xss_injection_detects_reflection() {
        let Some(src) = load_module("injection/xss-injection.js") else {
            return;
        };

        let (addr, _srv) = start_test_server().await;
        let base = format!("http://{addr}/sqli?id=1");

        let ps = PathState {
            uri: base,
            is_post_target: false,
            params: vec![FuzzableParam {
                name: "id".into(),
                value: "1".into(),
                location: ParamLocation::Query,
            }],
            fuzz_index: Some(0),
            ..Default::default()
        };

        let registry = AlertRegistry::load_default().unwrap_or_default();
        let engine = ScanEngine::new(registry);
        let result = engine
            .run_injection_module(&src, &ps)
            .await
            .expect("xss run");

        assert!(
            result.alerts.iter().any(|a| a.type_key.contains("xss")),
            "expected XSS alert, got: {:?}",
            result.alerts.iter().map(|a| &a.type_key).collect::<Vec<_>>()
        );
    }

    #[test]
    fn all_response_modules_have_metadata() {
        let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/vega/scripts/modules/response");
        let Ok(entries) = std::fs::read_dir(&dir) else {
            return;
        };
        let mut ok = 0;
        let mut rhino_only = 0;
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("js") {
                continue;
            }
            let src = std::fs::read_to_string(&path).expect("read module");
            assert!(src.contains("var module"), "{} missing module decl", path.display());
            match crate::vega::js_runtime::JsModuleHost::read_meta(&src) {
                Ok(meta) => {
                    assert!(!meta.name.is_empty(), "{} missing name", path.display());
                    ok += 1;
                }
                Err(_) if src.contains("importPackage") => {
                    rhino_only += 1; // needs Rhino prelude — host in Phase 6
                }
                Err(e) => panic!("{}: {e}", path.display()),
            }
        }
        assert!(ok >= 25, "expected most response modules to parse, got {ok} ok, {rhino_only} rhino-only");
    }
}
