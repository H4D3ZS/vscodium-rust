//! APEX Vega — JavaScript module host (Phase 2).
//!
//! Subgraph Vega's detection logic lives in ~46 self-contained JS modules. We
//! run them **unchanged** by embedding `boa_engine` (pure-Rust JS) and providing
//! the `ctx` / `request` / `response` objects the modules expect.
//!
//! Design: rather than bind native Rust closures into JS (which fights boa's
//! lifetime model), we define the API surface as **pure-JS shims** in a prelude
//! and marshal data across the boundary as JSON. The module pushes findings into
//! a global array; Rust reads it back after execution. This keeps the boundary
//! tiny, robust, and easy to extend as later phases add more `ctx`/`ps` methods.
//!
//! Phase 2 implements passive ("response-processor") modules that expose
//! `run(request, response, ctx)`. Injection modules (initialize/process +
//! request submission) are driven by the HTTP engine in Phase 3, which builds
//! on this host.

use crate::vega::alerts::AlertRegistry;
use crate::vega::model::{Alert, HttpRequest, HttpResponse, Severity};
use boa_engine::{Context, Source};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

/// What kind of Vega module this is, derived from its `var module = {...}` decl.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleKind {
    /// `type: "response-processor"` — passive, implements `run(req, res, ctx)`.
    ResponseProcessor,
    /// `category: "Injection Modules"` — active, implements `initialize/process`.
    Injection,
    /// Anything we don't classify yet.
    Unknown,
}

/// Metadata read from a module's `var module = {...}` object.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ModuleMeta {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default, rename = "type")]
    pub type_field: Option<String>,
    #[serde(default)]
    pub differential: Option<bool>,
    /// Vega marks slow/aggressive modules (timing sleeps, format-string) as
    /// `defaultDisabled` — they run only when explicitly selected.
    #[serde(default, rename = "defaultDisabled")]
    pub default_disabled: Option<bool>,
}

impl ModuleMeta {
    pub fn kind(&self) -> ModuleKind {
        if self.type_field.as_deref() == Some("response-processor") {
            ModuleKind::ResponseProcessor
        } else if self.category.as_deref() == Some("Injection Modules") {
            ModuleKind::Injection
        } else {
            ModuleKind::Unknown
        }
    }
}

/// The pure-JS API shim injected before every module. Defines `ctx`, and the
/// `__mkRequest` / `__mkResponse` factories the runner uses. Findings accumulate
/// in `__vega_alerts`; highlights in `__vega_highlights`.
pub const VEGA_JS_PRELUDE: &str = r#"
var __vega_alerts = [];
var __vega_highlights = [];
var __vega_seen_keys = {};

var ctx = {
  alert: function(type, req, res, opts) {
    opts = opts || {};
    __vega_alerts.push({
      type_key: type,
      output: (opts.output != null) ? String(opts.output) : "",
      resource: (opts.resource != null) ? String(opts.resource) : "",
      key: (opts.key != null) ? String(opts.key) : "",
      detection_type: (opts.detectiontype != null) ? String(opts.detectiontype) : null
    });
    if (opts.key) { __vega_seen_keys[opts.key] = true; }
  },
  alertExists: function(key) { return !!__vega_seen_keys[key]; },
  addStringHighlight: function(s) { __vega_highlights.push(String(s)); },
  addRegexCaseInsensitiveHighlight: function(s) { __vega_highlights.push(String(s)); },
  // No-op stubs so passive modules that touch these don't throw. The HTTP
  // engine (Phase 3) provides the real implementations for injection modules.
  error: function() {},
  setModuleFailed: function() {},
  hasModuleFailed: function() { return false; }
};

function __mkResponse(o) {
  return {
    bodyAsString: o.body || "",
    _headers: o.headers || [],
    fetchFail: !!o.fetch_fail,
    statusCode: o.status || 0,
    milliseconds: o.elapsed_ms || 0,
    hasHeader: function(n) {
      n = String(n).toLowerCase();
      for (var i = 0; i < this._headers.length; i++) {
        if (String(this._headers[i][0]).toLowerCase() === n) return true;
      }
      return false;
    },
    getFirstHeader: function(n) {
      n = String(n).toLowerCase();
      for (var i = 0; i < this._headers.length; i++) {
        if (String(this._headers[i][0]).toLowerCase() === n) {
          return { name: this._headers[i][0], value: this._headers[i][1] };
        }
      }
      return null;
    }
  };
}

function __mkRequest(o) {
  return {
    requestLine: { uri: o.uri || "", method: o.method || "GET" },
    _headers: o.headers || [],
    addHeader: function(n, v) { this._headers.push([String(n), String(v)]); }
  };
}
"#;

/// A finding as emitted by the JS shim, before resolution against the registry.
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct RawAlert {
    type_key: String,
    #[serde(default)]
    output: String,
    #[serde(default)]
    resource: String,
    #[serde(default)]
    key: String,
    #[serde(default)]
    detection_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct RunOutput {
    #[serde(default)]
    alerts: Vec<RawAlert>,
    #[serde(default)]
    highlights: Vec<String>,
}

/// Result of running one module against one request/response pair.
#[derive(Debug, Clone, Default)]
pub struct ModuleRunResult {
    pub alerts: Vec<Alert>,
    pub highlights: Vec<String>,
}

/// Hosts and executes Vega JS modules.
pub struct JsModuleHost {
    registry: AlertRegistry,
}

impl JsModuleHost {
    pub fn new(registry: AlertRegistry) -> Self {
        Self { registry }
    }

    /// Read a module's `var module = {...}` metadata without running its logic.
    pub fn read_meta(source: &str) -> Result<ModuleMeta, String> {
        let mut ctx = Context::default();
        eval(&mut ctx, source).map_err(|e| format!("eval module source: {e}"))?;
        let json = eval(&mut ctx, "JSON.stringify(typeof module !== 'undefined' ? module : {})")
            .map_err(|e| format!("read module meta: {e}"))?;
        serde_json::from_str(&json).map_err(|e| format!("parse module meta '{json}': {e}"))
    }

    /// Run a passive (`response-processor`) module's `run(request, response, ctx)`
    /// against the given request/response. Returns resolved alerts + highlights.
    pub fn run_response_module(
        &self,
        source: &str,
        request: &HttpRequest,
        response: &HttpResponse,
    ) -> Result<ModuleRunResult, String> {
        let mut ctx = Context::default();

        // 1) API shim.
        eval(&mut ctx, VEGA_JS_PRELUDE).map_err(|e| format!("eval prelude: {e}"))?;

        // 2) The module itself (defines `module`, `run`).
        eval(&mut ctx, source).map_err(|e| format!("eval module: {e}"))?;

        // 3) Inject the request/response as JSON globals.
        let req_json = serde_json::to_string(&JsReq::from(request))
            .map_err(|e| format!("serialize request: {e}"))?;
        let res_json = serde_json::to_string(&JsRes::from(response))
            .map_err(|e| format!("serialize response: {e}"))?;
        let inject = format!("var __vega_request = {req_json}; var __vega_response = {res_json};");
        eval(&mut ctx, &inject).map_err(|e| format!("inject io: {e}"))?;

        // 4) Run and collect.
        let runner = r#"
            (function() {
              __vega_alerts = [];
              __vega_highlights = [];
              var request = __mkRequest(__vega_request);
              var response = __mkResponse(__vega_response);
              if (typeof run === 'function') { run(request, response, ctx); }
              return JSON.stringify({ alerts: __vega_alerts, highlights: __vega_highlights });
            })()
        "#;
        let out_json = eval(&mut ctx, runner).map_err(|e| format!("run module: {e}"))?;
        let out: RunOutput =
            serde_json::from_str(&out_json).map_err(|e| format!("parse run output: {e}"))?;

        let ts = now_ms();
        let alerts = out
            .alerts
            .into_iter()
            .map(|raw| self.resolve_alert(raw, ts))
            .collect();

        Ok(ModuleRunResult {
            alerts,
            highlights: out.highlights,
        })
    }

    /// Resolve a raw JS alert against the registry, filling in title/severity.
    pub fn resolve_alert_public(&self, raw: RawAlert, ts_ms: u64) -> Alert {
        self.resolve_alert(raw, ts_ms)
    }

    /// Resolve a raw JS alert against the registry, filling in title/severity.
    fn resolve_alert(&self, raw: RawAlert, ts_ms: u64) -> Alert {
        let (title, severity) = match self.registry.get(&raw.type_key) {
            Some(def) => (def.title.clone(), def.severity),
            None => (raw.type_key.clone(), Severity::Info),
        };
        Alert {
            type_key: raw.type_key,
            title,
            severity,
            resource: raw.resource,
            key: raw.key,
            output: raw.output,
            detection_type: raw.detection_type,
            request_id: None,
            response_id: None,
            extra: Default::default(),
            timestamp_ms: ts_ms,
        }
    }
}

/// Eval helper: run `code`, return its result coerced to a Rust String.
fn eval(context: &mut Context, code: &str) -> Result<String, String> {
    match context.eval(Source::from_bytes(code)) {
        Ok(v) => match v.to_string(context) {
            Ok(s) => Ok(s.to_std_string_escaped()),
            Err(e) => Err(format!("{e:?}")),
        },
        Err(e) => Err(format!("{e:?}")),
    }
}

fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

// ── JSON marshalling shapes (match what the JS shim factories expect) ──────────
#[derive(serde::Serialize)]
struct JsReq<'a> {
    method: &'a str,
    uri: &'a str,
    headers: &'a [(String, String)],
}
impl<'a> From<&'a HttpRequest> for JsReq<'a> {
    fn from(r: &'a HttpRequest) -> Self {
        JsReq {
            method: &r.method,
            uri: &r.uri,
            headers: &r.headers,
        }
    }
}

#[derive(serde::Serialize)]
struct JsRes<'a> {
    status: u16,
    body: &'a str,
    headers: &'a [(String, String)],
    fetch_fail: bool,
    elapsed_ms: u64,
}
impl<'a> From<&'a HttpResponse> for JsRes<'a> {
    fn from(r: &'a HttpResponse) -> Self {
        JsRes {
            status: r.status,
            body: &r.body,
            headers: &r.headers,
            fetch_fail: r.fetch_fail,
            elapsed_ms: r.elapsed_ms,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn module_path(rel: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("resources/vega/scripts/modules")
            .join(rel)
    }

    fn load(rel: &str) -> Option<String> {
        let p = module_path(rel);
        std::fs::read_to_string(&p).ok()
    }

    #[test]
    fn reads_module_metadata() {
        let Some(src) = load("response/vinfo-headers.js") else {
            eprintln!("[vega] module resources missing — skipping");
            return;
        };
        let meta = JsModuleHost::read_meta(&src).expect("read meta");
        assert_eq!(meta.kind(), ModuleKind::ResponseProcessor);
        assert!(meta.name.to_lowercase().contains("header"));
    }

    #[test]
    fn runs_real_vega_header_module_and_raises_alert() {
        let Some(src) = load("response/vinfo-headers.js") else {
            eprintln!("[vega] module resources missing — skipping");
            return;
        };
        let registry = AlertRegistry::load_default().unwrap_or_default();
        let host = JsModuleHost::new(registry);

        let req = HttpRequest {
            method: "GET".into(),
            uri: "http://target.test/page?id=1".into(),
            ..Default::default()
        };
        // X-XSS-Protection: 0 → vinfo-headers.js raises vinfo-xss-filter-disabled.
        let res = HttpResponse {
            status: 200,
            headers: vec![("X-XSS-Protection".into(), "0".into())],
            body: "<html></html>".into(),
            fetch_fail: false,
            ..Default::default()
        };

        let result = host
            .run_response_module(&src, &req, &res)
            .expect("module run");
        assert!(
            result
                .alerts
                .iter()
                .any(|a| a.type_key == "vinfo-xss-filter-disabled"),
            "expected xss-filter-disabled alert, got: {:?}",
            result.alerts.iter().map(|a| &a.type_key).collect::<Vec<_>>()
        );
    }

    #[test]
    fn insecure_cors_header_is_detected() {
        let Some(src) = load("response/vinfo-headers.js") else {
            return;
        };
        let host = JsModuleHost::new(AlertRegistry::load_default().unwrap_or_default());
        let req = HttpRequest {
            method: "GET".into(),
            uri: "http://target.test/api".into(),
            ..Default::default()
        };
        let res = HttpResponse {
            status: 200,
            headers: vec![("Access-Control-Allow-Origin".into(), "*".into())],
            ..Default::default()
        };
        let result = host.run_response_module(&src, &req, &res).expect("run");
        assert!(result
            .alerts
            .iter()
            .any(|a| a.type_key == "vinfo-insecure-cors-ac"));
    }
}
