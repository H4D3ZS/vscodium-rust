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

fn now_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

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

    /// Full injection-module pipeline for one path state, driven the way Vega
    /// drives modules: `initialize()` submits requests, the engine fetches them,
    /// `process()` runs per response and may submit *more* requests, looping
    /// until the module stops submitting. Bounded by `MAX_FETCHES`/`MAX_ROUNDS`
    /// so a runaway module can't scan forever.
    pub async fn run_injection_module(
        &self,
        source: &str,
        ps: &PathState,
    ) -> Result<ModuleRunResult, String> {
        let baseline_res = self.fetch(&baseline_request(ps)?).await?;
        self.run_injection_with_baseline(source, ps, &baseline_res).await
    }

    /// Run every injection module against one path, fetching the baseline once
    /// and sharing it across modules (instead of one redundant baseline fetch per
    /// module). Returns the merged alerts.
    pub async fn run_injection_modules(
        &self,
        sources: &[(String, String)],
        ps: &PathState,
    ) -> Vec<Alert> {
        let Ok(baseline_res) = self.fetch(&match baseline_request(ps) {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        }).await else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for (_rel, src) in sources {
            if let Ok(r) = self.run_injection_with_baseline(src, ps, &baseline_res).await {
                out.extend(r.alerts);
            }
        }
        out
    }

    /// The event-driven core, given an already-fetched baseline response.
    async fn run_injection_with_baseline(
        &self,
        source: &str,
        ps: &PathState,
        baseline_res: &HttpResponse,
    ) -> Result<ModuleRunResult, String> {
        use crate::vega::injection_host::RoundKind;
        use std::collections::BTreeMap;

        const MAX_FETCHES: usize = 96;
        const MAX_ROUNDS: usize = 64;

        let baseline_fp = ResponseFingerprint::compute(baseline_res).raw();

        let host = InjectionModuleHost::new(self.registry.clone());

        let mut saved_req: BTreeMap<u32, HttpRequest> = BTreeMap::new();
        let mut saved_res: BTreeMap<u32, HttpResponse> = BTreeMap::new();
        let mut fps: BTreeMap<u32, u64> = BTreeMap::new();

        // Round 0: initialize() collects the first batch of submissions.
        let mut round = host.run_round(
            source,
            ps,
            baseline_fp,
            baseline_res,
            &saved_req,
            &saved_res,
            &fps,
            "{}",
            RoundKind::Init,
        )?;

        let mut fetched = 0usize;
        for _ in 0..MAX_ROUNDS {
            // Fetch every newly-submitted request that hasn't been sent yet.
            let mut new_indices: Vec<u32> = Vec::new();
            for step in &round.plan {
                let idx = step.index();
                if saved_res.contains_key(&idx) {
                    continue;
                }
                if fetched >= MAX_FETCHES {
                    break;
                }
                let req = match step {
                    PlanStep::Altered { payload, append, .. } => {
                        build_altered_request(ps, payload, *append)?
                    }
                    PlanStep::Request { method, uri, headers, body, .. } => HttpRequest {
                        method: if method.is_empty() { "GET".into() } else { method.clone() },
                        uri: uri.clone(),
                        headers: headers.clone(),
                        body: body.clone(),
                    },
                };
                let res = self.fetch(&req).await?;
                fps.insert(idx, ResponseFingerprint::compute(&res).raw());
                saved_req.insert(idx, req);
                saved_res.insert(idx, res);
                new_indices.push(idx);
                fetched += 1;
            }

            if new_indices.is_empty() {
                break;
            }

            // Deliver those responses to process(); it may submit more.
            round = host.run_round(
                source,
                ps,
                baseline_fp,
                baseline_res,
                &saved_req,
                &saved_res,
                &fps,
                &round.scratch,
                RoundKind::Process(new_indices),
            )?;
        }

        let (mut result, response_checks) = host.finalize(&round.scratch);

        // Honor ctx.responseChecks(i): run passive response modules on the named
        // saved responses, exactly as Vega folds passive checks into an injection
        // scan. Best-effort and deduped against the module's own alerts.
        if !response_checks.is_empty() {
            let passive = self.passive_sources();
            if !passive.is_empty() {
                let mut seen: std::collections::HashSet<String> =
                    result.alerts.iter().map(|a| a.key.clone()).collect();
                for i in response_checks {
                    let (Some(req), Some(res)) = (saved_req.get(&i), saved_res.get(&i)) else {
                        continue;
                    };
                    for a in self.run_all_passive(
                        &passive.iter().map(|(n, s)| (n.as_str(), s.as_str())).collect::<Vec<_>>(),
                        req,
                        res,
                    ) {
                        if a.key.is_empty() || seen.insert(a.key.clone()) {
                            result.alerts.push(a);
                        }
                    }
                }
            }
        }

        Ok(result)
    }

    /// Load passive (response-processor) module sources, skipping any that need
    /// the Rhino `importPackage` shim. Cached read from `resources/vega`.
    fn passive_sources(&self) -> Vec<(String, String)> {
        crate::vega::campaign::passive_module_sources()
    }

    /// Built-in, deterministic error-based SQL injection check for one path
    /// state. Sends the unaltered request once, then a handful of
    /// syntax-breaking payloads on the active parameter, and raises a
    /// high-confidence alert if a DB error string appears that the baseline
    /// lacked. Independent of the JS differential modules — this is what catches
    /// the common DVWA-style case reliably. Stops at the first hit per param.
    pub async fn run_error_based_sqli(&self, ps: &PathState) -> Option<Alert> {
        use crate::vega::error_based::{detect_sql_error, sql_alert, SQL_PROBE_PAYLOADS};

        let param = ps.fuzzable_parameter()?.name.clone();
        let baseline = self.fetch(&baseline_request(ps).ok()?).await.ok()?;
        if baseline.fetch_fail {
            return None;
        }
        let resource = ps.uri.split('?').next().unwrap_or(&ps.uri).to_string();

        for payload in SQL_PROBE_PAYLOADS {
            let Ok(req) = build_altered_request(ps, payload, true) else {
                continue;
            };
            let Ok(res) = self.fetch(&req).await else { continue };
            if let Some(sig) = detect_sql_error(&baseline, &res) {
                let ts = now_ms();
                return Some(sql_alert(&resource, &param, sig, &res.body, ts));
            }
        }
        None
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
        let method = if req.method.is_empty() { "GET".to_string() } else { req.method.to_uppercase() };
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

        let started = std::time::Instant::now();
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
                    elapsed_ms: started.elapsed().as_millis() as u64,
                })
            }
            Err(e) => Ok(HttpResponse {
                status: 0,
                fetch_fail: true,
                body: e.to_string(),
                elapsed_ms: started.elapsed().as_millis() as u64,
                ..Default::default()
            }),
        }
    }
}

/// Baseline (unaltered) request — sends the full parameter set exactly as
/// harvested, so the baseline response is directly comparable to the fuzzed ones.
fn baseline_request(ps: &PathState) -> Result<HttpRequest, String> {
    let encoded = ps
        .params
        .iter()
        .map(|p| {
            format!(
                "{}={}",
                urlencoding::encode(&p.name),
                urlencoding::encode(&p.value)
            )
        })
        .collect::<Vec<_>>()
        .join("&");

    let base = ps.uri.split('?').next().unwrap_or(&ps.uri);

    if ps.is_post_target {
        let mut req = HttpRequest {
            method: "POST".into(),
            uri: base.to_string(),
            body: encoded,
            ..Default::default()
        };
        req.add_header("Content-Type", "application/x-www-form-urlencoded");
        Ok(req)
    } else if encoded.is_empty() {
        Ok(HttpRequest {
            method: "GET".into(),
            uri: ps.uri.clone(),
            ..Default::default()
        })
    } else {
        Ok(HttpRequest {
            method: "GET".into(),
            uri: format!("{}?{}", base, encoded),
            ..Default::default()
        })
    }
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

    /// Every default-enabled injection module must run end-to-end against a live
    /// target without the JS host throwing — proves the full `ctx`/`ps` API
    /// surface is implemented (no `submitMultipleAlteredRequests is not a
    /// function`-style failures that silently kill detection).
    #[tokio::test]
    async fn all_default_injection_modules_run_without_error() {
        let mods = crate::vega::campaign::default_injection_modules();
        if mods.is_empty() {
            eprintln!("[vega] resources missing — skip");
            return;
        }

        let (addr, _srv) = start_test_server().await;
        let registry = AlertRegistry::load_default().unwrap_or_default();
        let engine = ScanEngine::new(registry);

        let ps = PathState {
            uri: format!("http://{addr}/sqli?id=1"),
            is_post_target: false,
            params: vec![FuzzableParam {
                name: "id".into(),
                value: "1".into(),
                location: ParamLocation::Query,
            }],
            fuzz_index: Some(0),
            ..Default::default()
        };

        for rel in &mods {
            let Some(src) = load_module(rel) else { continue };
            let res = engine.run_injection_module(&src, &ps).await;
            assert!(res.is_ok(), "module {rel} failed to run: {:?}", res.err());
        }
    }

    /// A module that submits a follow-up request from inside `process()` must be
    /// driven iteratively (Vega's event loop), not collect-once.
    #[tokio::test]
    async fn iterative_submission_during_process_is_supported() {
        // Inline module: initialize submits idx 0; process submits idx 1 once,
        // then alerts on idx 1. Exercises the submit-during-process path.
        let src = r#"
            var module = { name: "iter", category: "Injection Modules" };
            function initialize(ctx) {
                ctx.submitAlteredRequest(process, "a", true, 0);
            }
            function process(req, res, ctx) {
                var i = ctx.getCurrentIndex();
                if (i === 0) { ctx.submitAlteredRequest(process, "b", true, 1); return; }
                if (i === 1) {
                    ctx.alert("vinfo-headers", req, res, { key: "iter:1", resource: "/x" });
                }
            }
        "#;
        let (addr, _srv) = start_test_server().await;
        let registry = AlertRegistry::load_default().unwrap_or_default();
        let engine = ScanEngine::new(registry);
        let ps = PathState {
            uri: format!("http://{addr}/sqli?id=1"),
            params: vec![FuzzableParam {
                name: "id".into(),
                value: "1".into(),
                location: ParamLocation::Query,
            }],
            fuzz_index: Some(0),
            ..Default::default()
        };
        let result = engine.run_injection_module(src, &ps).await.expect("run");
        assert!(
            result.alerts.iter().any(|a| a.key == "iter:1"),
            "follow-up request submitted during process() was not delivered"
        );
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
