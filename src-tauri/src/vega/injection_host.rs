//! Injection-module host — Vega's event-driven submit/process loop.
//!
//! Faithful to how Subgraph Vega drives an injection module:
//!
//! 1. `initialize(ctx)` submits one or more altered requests.
//! 2. As each response arrives, `process(req, res, ctx)` is called — and it may
//!    **submit further requests** (e.g. timing modules escalate, differential
//!    modules submit confirmation probes). The loop continues until no module
//!    submits anything new.
//!
//! boa's `Context` is not `Send` and JS is synchronous, so we can't hold a JS
//! context across an `await`. Instead each "round" runs in a fresh context with
//! the full accumulated state (saved requests/responses, fingerprints, and a
//! JSON scratch blob of the module's JS globals) re-injected. Rust owns the
//! network I/O and the saved-response store; JS owns detection logic. This gives
//! us the event-driven semantics modules expect without async-in-boa.
//!
//! See `.planning/vega-integration/01-VEGA-INVENTORY.md` for the full `ctx`/`ps`
//! API contract this implements.

use crate::vega::alerts::AlertRegistry;
use crate::vega::js_runtime::RawAlert;
use crate::vega::js_runtime::{JsModuleHost, ModuleRunResult};
use crate::vega::model::{HttpRequest, HttpResponse, PathState};
use boa_engine::{Context, Source};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::BTreeMap;

/// One request a module asked the engine to send.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum PlanStep {
    /// Fuzz the active parameter: mutate it by `payload`, send the full param set.
    #[serde(rename = "altered")]
    Altered {
        payload: String,
        append: bool,
        index: u32,
    },
    /// A fully-formed request the module built itself (e.g. XSS probes).
    #[serde(rename = "request")]
    Request {
        index: u32,
        #[serde(default)]
        method: String,
        uri: String,
        #[serde(default)]
        headers: Vec<(String, String)>,
        #[serde(default)]
        body: String,
    },
}

impl PlanStep {
    pub fn index(&self) -> u32 {
        match self {
            PlanStep::Altered { index, .. } | PlanStep::Request { index, .. } => *index,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct PsBase {
    uri: String,
    method: String,
    is_post: bool,
    param_name: String,
    param_value: String,
}

/// The output of one JS round: any newly-submitted requests, plus the opaque
/// scratch blob (module JS globals) to thread into the next round.
pub struct RoundOutput {
    pub plan: Vec<PlanStep>,
    pub scratch: String,
}

/// Whether this round runs `initialize()` or `process()` for given indices.
pub enum RoundKind {
    Init,
    Process(Vec<u32>),
}

/// The full pure-JS API surface (`ctx.*` / `ps.*`) for injection modules. This
/// is the contract from `01-VEGA-INVENTORY.md`. Every method the 18 injection
/// modules call is implemented or safely stubbed so none throw at runtime.
const INJECTION_PRELUDE: &str = r#"
var __vega_plan = [];

function __fpVal(x) {
  x = x | 0;
  return (x < 0) ? __vega_baseline_fp : __vega_fps[x];
}
function __fpMatch(i, j) { return __fpVal(i) === __fpVal(j); }

function __mutateUri(uri, param, value) {
  var q = uri.indexOf('?');
  var base = q >= 0 ? uri.substring(0, q) : uri;
  var qs = q >= 0 ? uri.substring(q + 1) : '';
  var parts = qs ? qs.split('&') : [];
  var found = false;
  for (var i = 0; i < parts.length; i++) {
    var kv = parts[i].split('=');
    if (decodeURIComponent(kv[0] || '') === param) {
      parts[i] = encodeURIComponent(param) + '=' + encodeURIComponent(value);
      found = true;
      break;
    }
  }
  if (!found) parts.push(encodeURIComponent(param) + '=' + encodeURIComponent(value));
  return base + '?' + parts.join('&');
}

// Serialize the full parameter set with the fuzzed value substituted — matches
// the Rust build_altered_request so module-built requests carry sibling fields.
function __encodeParams(fuzzVal) {
  var out = [];
  for (var i = 0; i < __vega_params.length; i++) {
    var p = __vega_params[i];
    var v = (i === __vega_fuzz_index) ? fuzzVal : p.value;
    out.push(encodeURIComponent(p.name) + '=' + encodeURIComponent(v));
  }
  return out.join('&');
}

function __serializeReq(req) {
  var rl = req.requestLine || {};
  return {
    method: rl.method || 'GET',
    uri: rl.uri || '',
    headers: req._headers || [],
    body: req._body || ''
  };
}

function __origin(u) { var m = /^(https?:\/\/[^\/]+)/i.exec(u); return m ? m[1] : ''; }
function __hostOf(u) { var m = /^https?:\/\/([^\/]+)/i.exec(u); return m ? m[1] : ''; }
function __pathOf(u) { var m = /^https?:\/\/[^\/]+(\/[^#]*)?/i.exec(u); return (m && m[1]) ? m[1] : '/'; }

function __rawRequest(host, method, path) {
  var uri;
  if (host && path) { uri = 'http://' + host + path; }
  else if (path) { uri = __origin(__vega_ps_base.uri) + path; }
  else { uri = __vega_ps_base.uri; }
  return {
    requestLine: { uri: uri, method: method || 'GET' },
    _headers: [], _body: '',
    addHeader: function(n, v) { this._headers.push([String(n), String(v)]); }
  };
}

var __vega_ps = {
  isParametric: function() { return __vega_params.length > 0; },
  getFuzzableParameter: function() {
    return { name: __vega_ps_base.param_name, value: __vega_ps_base.param_value };
  },
  getPath: function() {
    return {
      getUri: function() { return __vega_ps_base.uri; },
      isPostTarget: function() { return !!__vega_ps_base.is_post; },
      getHttpHost: function() { return __hostOf(__vega_ps_base.uri); },
      getFullPath: function() { return __pathOf(__vega_ps_base.uri); }
    };
  },
  getPathFingerprint: function() { return -1; },
  allocateXssId: function() { __xss_id++; return __xss_id; },
  createXssTag: function(a, b) {
    return (b === undefined) ? ('vega-xss-' + a) : ('vega-xss-' + b + '-' + a);
  },
  createAlteredRequest: function(payload, append) {
    var base = __vega_ps_base;
    var fuzzVal = append ? (base.param_value + payload) : String(payload);
    var enc = __encodeParams(fuzzVal);
    var stripped = base.uri.split('?')[0];
    if (base.is_post) {
      return {
        requestLine: { uri: stripped, method: 'POST' },
        _headers: [['Content-Type', 'application/x-www-form-urlencoded']],
        _body: enc,
        addHeader: function(n, v) { this._headers.push([String(n), String(v)]); }
      };
    }
    return {
      requestLine: { uri: stripped + '?' + enc, method: 'GET' },
      _headers: [],
      _body: '',
      addHeader: function(n, v) { this._headers.push([String(n), String(v)]); }
    };
  },
  registerXssRequest: function(req, xid) {},
  incrementFuzzCounter: function() {},
  decrementFuzzCounter: function() {},
  isRootPath: function() { return __pathOf(__vega_ps_base.uri) === '/'; },
  getResponse: function() { return __mkResponse(__vega_orig_res); },
  createRequest: function() { return __rawRequest(null, 'GET', __pathOf(__vega_ps_base.uri)); },
  createRawRequest: function(host, method, path) { return __rawRequest(host, method, path); },
  length: 0
};
__vega_ps.path = __vega_ps.getPath();

var ctx = {
  getPathState: function() { return __vega_ps; },
  submitAlteredRequest: function(cb, payload, append, index) {
    __vega_plan.push({ kind: 'altered', payload: String(payload), append: !!append, index: index | 0 });
  },
  submitMultipleAlteredRequests: function(cb, payloads, append) {
    for (var i = 0; i < payloads.length; i++) {
      __vega_plan.push({ kind: 'altered', payload: String(payloads[i]), append: !!append, index: i });
    }
  },
  submitRequest: function(req, cb, index) {
    var s = __serializeReq(req);
    __vega_plan.push({ kind: 'request', index: index | 0, method: s.method, uri: s.uri, headers: s.headers, body: s.body });
  },
  getSavedRequest: function(i) { return __mkRequest(__vega_saved_req[i | 0] || {}); },
  getSavedResponse: function(i) {
    var r = __mkResponse(__vega_saved_res[i | 0] || {});
    r.fingerprint = i | 0;
    return r;
  },
  getOrigResponse: function() {
    // `.fingerprint` is the baseline handle (-1) so modules that compare against
    // ctx.getOrigResponse().fingerprint diff against the true baseline.
    var r = __mkResponse(__vega_orig_res);
    r.fingerprint = -1;
    return r;
  },
  isFingerprintMatch: function(i, j) { return __fpMatch(i, j); },
  incrementResponseCount: function() { __vega_response_count++; return __vega_response_count; },
  addRequestResponse: function(req, res) {},
  allResponsesReceived: function() { return __vega_plan.length === 0; },
  getCurrentIndex: function() { return __vega_current_index; },
  setModuleFailed: function() { __vega_module_failed = true; },
  hasModuleFailed: function() { return !!__vega_module_failed; },
  alertExists: function(k) { return !!__vega_seen_keys[k]; },
  error: function() {},
  responseChecks: function(i) { __vega_response_checks.push(i | 0); },
  contentChecks: function(req, res) {
    var body = (res && res.bodyAsString) ? res.bodyAsString : '';
    var uri = (req && req.requestLine && req.requestLine.uri) ? req.requestLine.uri : '';
    if (body.indexOf('vega-xss-') >= 0) {
      ctx.alert('vinfo-xss', req, res, {
        output: body.substring(0, 200), resource: uri, key: 'xss:' + uri,
        detectiontype: 'Reflected Cross-Site Scripting'
      });
    }
  },
  addStringHighlight: function(s) { __vega_highlights.push(String(s)); },
  addRegexCaseInsensitiveHighlight: function(s) { __vega_highlights.push(String(s)); },
  setIntegerProperty: function(k, v) { __vega_props[k] = v | 0; },
  getIntegerProperty: function(k) { return __vega_props[k] || 0; },
  isValidInternetDomainName: function(s) { return /^[a-z0-9.-]+\.[a-z]{2,}$/i.test(String(s)); },
  internetDomainName: function(s) { return String(s); },
  alert: function(type, req, res, opts) {
    opts = opts || {};
    __vega_alerts.push({
      type_key: type,
      output: (opts.output != null) ? String(opts.output) : '',
      resource: (opts.resource != null) ? String(opts.resource) : '',
      key: (opts.key != null) ? String(opts.key) : '',
      detection_type: (opts.detectiontype != null) ? String(opts.detectiontype) : null
    });
    if (opts.key) __vega_seen_keys[opts.key] = true;
  }
};
"#;

/// Drives injection modules round-by-round.
pub struct InjectionModuleHost {
    inner: JsModuleHost,
}

impl InjectionModuleHost {
    pub fn new(registry: AlertRegistry) -> Self {
        Self {
            inner: JsModuleHost::new(registry),
        }
    }

    /// Run one round (init or process). Rust owns the saved-state stores; this
    /// injects them into a fresh JS context, runs the requested entry point, and
    /// returns the new submissions plus the updated scratch blob.
    #[allow(clippy::too_many_arguments)]
    pub fn run_round(
        &self,
        source: &str,
        ps: &PathState,
        baseline_fp: u64,
        orig_res: &HttpResponse,
        saved_req: &BTreeMap<u32, HttpRequest>,
        saved_res: &BTreeMap<u32, HttpResponse>,
        fps: &BTreeMap<u32, u64>,
        scratch_in: &str,
        kind: RoundKind,
    ) -> Result<RoundOutput, String> {
        // A fuzzable parameter is optional: differential modules self-gate on
        // `ps.isParametric()`, while root-path probes (http-trace) run on
        // parameterless paths. Default to empty so neither path errors.
        let (param_name, param_value) = match ps.fuzzable_parameter() {
            Some(p) => (p.name.clone(), p.value.clone()),
            None => (String::new(), String::new()),
        };

        let ps_base = PsBase {
            uri: ps.uri.clone(),
            method: if ps.is_post_target { "POST".into() } else { "GET".into() },
            is_post: ps.is_post_target,
            param_name,
            param_value,
        };
        let params: Vec<Value> = ps
            .params
            .iter()
            .map(|p| json!({ "name": p.name, "value": p.value }))
            .collect();

        let mut ctx = Context::default();
        eval(&mut ctx, crate::vega::js_runtime::VEGA_JS_PRELUDE)?;
        eval(&mut ctx, INJECTION_PRELUDE)?;

        // Inject the immutable per-path facts.
        let setup = format!(
            "var __vega_ps_base = {}; var __vega_params = {}; var __vega_fuzz_index = {}; \
             var __vega_baseline_fp = {}; var __vega_orig_res = {}; \
             var __vega_saved_req = {}; var __vega_saved_res = {}; var __vega_fps = {}; \
             var __vega_current_index = -1;",
            serde_json::to_string(&ps_base).map_err(|e| e.to_string())?,
            serde_json::to_string(&params).map_err(|e| e.to_string())?,
            ps.fuzz_index.unwrap_or(0),
            baseline_fp,
            serde_json::to_string(&res_json(orig_res)).map_err(|e| e.to_string())?,
            serde_json::to_string(&map_json(saved_req, req_json)).map_err(|e| e.to_string())?,
            serde_json::to_string(&map_json(saved_res, res_json)).map_err(|e| e.to_string())?,
            serde_json::to_string(&fps_json(fps)).map_err(|e| e.to_string())?,
        );
        eval(&mut ctx, &setup)?;

        // Re-hydrate the module's JS scratch globals from the prior round.
        let scratch = if scratch_in.trim().is_empty() { "{}" } else { scratch_in };
        eval(
            &mut ctx,
            &format!(
                "var __s = {scratch}; \
                 var __vega_props = __s.props || {{}}; \
                 var __vega_seen_keys = __s.seen || {{}}; \
                 var __vega_response_count = __s.count || 0; \
                 var __xss_id = __s.xss || 0; \
                 var __vega_alerts = __s.alerts || []; \
                 var __vega_highlights = __s.highlights || []; \
                 var __vega_response_checks = __s.checks || []; \
                 var __vega_module_failed = __s.failed || false; \
                 __vega_plan = [];"
            ),
        )?;

        // Define the module (re-runs its top-level setup; redefines initialize/process).
        eval(&mut ctx, source)?;

        match kind {
            RoundKind::Init => {
                eval(&mut ctx, "if (typeof initialize === 'function') initialize(ctx);")?;
            }
            RoundKind::Process(indices) => {
                for i in indices {
                    eval(
                        &mut ctx,
                        &format!(
                            "(function() {{ __vega_current_index = {i}; \
                              if (typeof process === 'function') {{ \
                                process(__mkRequest(__vega_saved_req[{i}] || {{}}), \
                                        __mkResponse(__vega_saved_res[{i}] || {{}}), ctx); }} }})()"
                        ),
                    )?;
                }
            }
        }

        let plan_json = eval(&mut ctx, "JSON.stringify(__vega_plan)")?;
        let plan: Vec<PlanStep> =
            serde_json::from_str(&plan_json).map_err(|e| format!("parse plan: {e}"))?;

        let scratch_out = eval(
            &mut ctx,
            "JSON.stringify({ props: __vega_props, seen: __vega_seen_keys, \
             count: __vega_response_count, xss: __xss_id, alerts: __vega_alerts, \
             highlights: __vega_highlights, checks: __vega_response_checks, \
             failed: __vega_module_failed })",
        )?;

        Ok(RoundOutput {
            plan,
            scratch: scratch_out,
        })
    }

    /// Parse the final scratch blob into resolved alerts + the response-check
    /// queue (saved-response indices the module asked passive modules to run on).
    pub fn finalize(&self, scratch: &str) -> (ModuleRunResult, Vec<u32>) {
        #[derive(Deserialize, Default)]
        struct Final {
            #[serde(default)]
            alerts: Vec<RawAlert>,
            #[serde(default)]
            highlights: Vec<String>,
            #[serde(default)]
            checks: Vec<u32>,
        }
        let parsed: Final = serde_json::from_str(scratch).unwrap_or_default();
        let ts = now_ms();
        let alerts = parsed
            .alerts
            .into_iter()
            .map(|raw| self.inner.resolve_alert_public(raw, ts))
            .collect();
        (
            ModuleRunResult {
                alerts,
                highlights: parsed.highlights,
            },
            parsed.checks,
        )
    }
}

fn map_json<T>(map: &BTreeMap<u32, T>, f: fn(&T) -> Value) -> Value {
    let mut obj = serde_json::Map::new();
    for (k, v) in map {
        obj.insert(k.to_string(), f(v));
    }
    Value::Object(obj)
}

fn fps_json(map: &BTreeMap<u32, u64>) -> Value {
    let mut obj = serde_json::Map::new();
    for (k, v) in map {
        obj.insert(k.to_string(), json!(v));
    }
    Value::Object(obj)
}

fn req_json(r: &HttpRequest) -> Value {
    json!({ "method": r.method, "uri": r.uri, "headers": r.headers, "body": r.body })
}

fn res_json(r: &HttpResponse) -> Value {
    json!({
        "status": r.status,
        "body": r.body,
        "headers": r.headers,
        "fetch_fail": r.fetch_fail,
        "elapsed_ms": r.elapsed_ms,
    })
}

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
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// Build an altered HTTP request by mutating the active fuzzable parameter while
/// preserving every sibling field. Login and search forms reject a request that
/// drops their other inputs (password, CSRF token, submit button) — so we always
/// serialize the full parameter set and only swap the value at `fuzz_index`.
pub fn build_altered_request(
    ps: &PathState,
    payload: &str,
    append: bool,
) -> Result<HttpRequest, String> {
    let idx = ps
        .fuzz_index
        .ok_or_else(|| "path state has no fuzz index".to_string())?;
    let param = ps
        .params
        .get(idx)
        .ok_or_else(|| "fuzz index out of range".to_string())?;

    let new_value = if append {
        format!("{}{}", param.value, payload)
    } else {
        payload.to_string()
    };

    let encoded = encode_params(ps, idx, &new_value);

    if ps.is_post_target {
        let mut req = HttpRequest {
            method: "POST".into(),
            uri: strip_query(&ps.uri).to_string(),
            body: encoded,
            ..Default::default()
        };
        req.add_header("Content-Type", "application/x-www-form-urlencoded");
        Ok(req)
    } else {
        Ok(HttpRequest {
            method: "GET".into(),
            uri: format!("{}?{}", strip_query(&ps.uri), encoded),
            ..Default::default()
        })
    }
}

/// URL-encode the full parameter set, substituting `value` at position `idx`.
fn encode_params(ps: &PathState, idx: usize, value: &str) -> String {
    ps.params
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let v = if i == idx { value } else { p.value.as_str() };
            format!("{}={}", urlencoding_encode(&p.name), urlencoding_encode(v))
        })
        .collect::<Vec<_>>()
        .join("&")
}

fn strip_query(uri: &str) -> &str {
    uri.split('?').next().unwrap_or(uri)
}

fn urlencoding_encode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vega::model::{FuzzableParam, ParamLocation};

    #[test]
    fn mutates_query_param() {
        let ps = PathState {
            uri: "http://t.test/page?id=1".into(),
            params: vec![FuzzableParam {
                name: "id".into(),
                value: "1".into(),
                location: ParamLocation::Query,
            }],
            fuzz_index: Some(0),
            ..Default::default()
        };
        let req = build_altered_request(&ps, "' AND 1=1 -- ", true).unwrap();
        assert!(req.uri.contains("id="));
        let decoded = urlencoding::decode(&req.uri).unwrap_or_else(|_| req.uri.clone().into());
        let decoded = decoded.to_string();
        assert!(decoded.contains("1' AND 1=1") || decoded.contains("1%27 AND 1=1"));
    }

    #[test]
    fn post_sends_all_sibling_params() {
        let ps = PathState {
            uri: "http://t.test/login".into(),
            is_post_target: true,
            params: vec![
                FuzzableParam { name: "username".into(), value: "admin".into(), location: ParamLocation::Post },
                FuzzableParam { name: "password".into(), value: "pw".into(), location: ParamLocation::Post },
            ],
            fuzz_index: Some(0),
            ..Default::default()
        };
        let req = build_altered_request(&ps, "'", true).unwrap();
        assert_eq!(req.method, "POST");
        assert!(req.body.contains("username=admin%27"));
        assert!(req.body.contains("password=pw"), "sibling field must be sent");
    }
}
