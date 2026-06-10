//! Injection-module host — two-phase execute (collect plan → fetch → process).
//!
//! Vega injection modules call `initialize(ctx)` which submits altered requests,
//! then `process(req, res, ctx)` per response. Network I/O stays in Rust; JS stays
//! synchronous. See `.planning/vega-integration/PROGRESS.md` Phase 3.

use crate::vega::alerts::AlertRegistry;
use crate::vega::js_runtime::RawAlert;
use crate::vega::fingerprint::ResponseFingerprint;
use crate::vega::js_runtime::{JsModuleHost, ModuleRunResult};
use crate::vega::model::{HttpRequest, HttpResponse, ParamLocation, PathState};
use boa_engine::{Context, Source};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// One step collected from `initialize()`.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "kind")]
pub enum PlanStep {
    #[serde(rename = "altered")]
    Altered {
        payload: String,
        append: bool,
        index: u32,
    },
    #[serde(rename = "request")]
    Request {
        index: u32,
        method: String,
        uri: String,
        #[serde(default)]
        headers: Vec<(String, String)>,
        #[serde(default)]
        body: String,
    },
}

#[derive(Debug, Clone, Serialize)]
struct PsBase {
    uri: String,
    method: String,
    is_post: bool,
    param_name: String,
    param_value: String,
}

const INJECTION_COLLECT_PRELUDE: &str = r#"
var __vega_plan = [];
var __xss_id = 0;

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
  if (!found) {
    parts.push(encodeURIComponent(param) + '=' + encodeURIComponent(value));
  }
  return base + '?' + parts.join('&');
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

var __vega_ps = {
  isParametric: function() { return true; },
  getFuzzableParameter: function() { return { name: __vega_ps_base.param_name, value: __vega_ps_base.param_value }; },
  getPath: function() {
    return {
      getUri: function() { return __vega_ps_base.uri; },
      isPostTarget: function() { return !!__vega_ps_base.is_post; }
    };
  },
  getPathFingerprint: function() { return __vega_baseline_fp; },
  allocateXssId: function() { __xss_id++; return __xss_id; },
  createXssTag: function(payload, xid) {
    payload = payload || '';
    return 'vega-xss-' + xid + '-' + payload;
  },
  createAlteredRequest: function(payload, append) {
    var val = __vega_ps_base.param_value;
    if (append) { val = val + payload; } else { val = payload; }
    var uri = __mutateUri(__vega_ps_base.uri, __vega_ps_base.param_name, val);
    return __mkRequest({ method: __vega_ps_base.method, uri: uri, headers: [] });
  },
  registerXssRequest: function(req, xid) {},
  incrementFuzzCounter: function() {},
  decrementFuzzCounter: function() {}
};

var ctx = {
  getPathState: function() { return __vega_ps; },
  submitAlteredRequest: function(cb, payload, append, index) {
    __vega_plan.push({ kind: 'altered', payload: String(payload), append: !!append, index: index|0 });
  },
  submitRequest: function(req, cb, index) {
    var s = __serializeReq(req);
    __vega_plan.push({ kind: 'request', index: index|0, method: s.method, uri: s.uri, headers: s.headers, body: s.body });
  },
  alertExists: function(k) { return !!__vega_seen_keys[k]; },
  setModuleFailed: function() { __vega_module_failed = true; },
  hasModuleFailed: function() { return !!__vega_module_failed; },
  error: function() {},
  alert: function(type, req, res, opts) {
    opts = opts || {};
    __vega_alerts.push({
      type_key: type,
      output: (opts.output != null) ? String(opts.output) : '',
      resource: (opts.resource != null) ? String(opts.resource) : '',
      key: (opts.key != null) ? String(opts.key) : '',
      detection_type: (opts.detectiontype != null) ? String(opts.detectiontype) : null
    });
    if (opts.key) { __vega_seen_keys[opts.key] = true; }
  }
};
"#;

const INJECTION_PROCESS_PRELUDE: &str = r#"
function __fpMatch(i, j) {
  return __vega_fps[i|0] === __vega_fps[j|0];
}

var __vega_ps = {
  isParametric: function() { return true; },
  getFuzzableParameter: function() { return { name: __vega_ps_base.param_name }; },
  getPath: function() {
    return {
      getUri: function() { return __vega_ps_base.uri; },
      isPostTarget: function() { return !!__vega_ps_base.is_post; }
    };
  },
  getPathFingerprint: function() { return __vega_baseline_fp; }
};

var __response_count = 0;
var __vega_module_failed = false;

var ctx = {
  getPathState: function() { return __vega_ps; },
  getSavedRequest: function(i) { return __mkRequest(__vega_saved_req[i|0]); },
  getSavedResponse: function(i) { return __mkResponse(__vega_saved_res[i|0]); },
  isFingerprintMatch: function(i, j) { return __fpMatch(i, j); },
  incrementResponseCount: function() { __response_count++; return __response_count; },
  addRequestResponse: function(req, res) {},
  allResponsesReceived: function() { return true; },
  getCurrentIndex: function() { return __vega_current_index; },
  getOrigResponse: function() { return __mkResponse(__vega_saved_res[0]); },
  setModuleFailed: function() { __vega_module_failed = true; },
  hasModuleFailed: function() { return __vega_module_failed; },
  alertExists: function(k) { return !!__vega_seen_keys[k]; },
  error: function() {},
  alert: function(type, req, res, opts) {
    opts = opts || {};
    __vega_alerts.push({
      type_key: type,
      output: (opts.output != null) ? String(opts.output) : '',
      resource: (opts.resource != null) ? String(opts.resource) : '',
      key: (opts.key != null) ? String(opts.key) : '',
      detection_type: (opts.detectiontype != null) ? String(opts.detectiontype) : null
    });
    if (opts.key) { __vega_seen_keys[opts.key] = true; }
  },
  addStringHighlight: function(s) { __vega_highlights.push(String(s)); },
  addRegexCaseInsensitiveHighlight: function(s) { __vega_highlights.push(String(s)); },
  responseChecks: function(i) {},
  contentChecks: function(req, res) {
    var body = res.bodyAsString || '';
    var uri = (req.requestLine && req.requestLine.uri) ? req.requestLine.uri : '';
    if (body.indexOf('vega-xss-') >= 0) {
      ctx.alert('vinfo-xss', req, res, { output: body.substring(0, 200), resource: uri, key: 'xss:' + uri });
    }
  },
  setIntegerProperty: function(k, v) { __vega_props[k] = v|0; },
  getIntegerProperty: function(k) { return __vega_props[k] || 0; }
};
"#;

/// Host for injection modules (Phase 3).
pub struct InjectionModuleHost {
    inner: JsModuleHost,
}

impl InjectionModuleHost {
    pub fn new(registry: AlertRegistry) -> Self {
        Self {
            inner: JsModuleHost::new(registry),
        }
    }

    /// Phase A: run `initialize()` and collect the request plan.
    pub fn collect_plan(
        &self,
        source: &str,
        ps: &PathState,
        baseline_fp: ResponseFingerprint,
    ) -> Result<Vec<PlanStep>, String> {
        let param = ps
            .fuzzable_parameter()
            .ok_or_else(|| "path state has no fuzzable parameter".to_string())?;

        let ps_base = PsBase {
            uri: ps.uri.clone(),
            method: if ps.is_post_target { "POST".into() } else { "GET".into() },
            is_post: ps.is_post_target,
            param_name: param.name.clone(),
            param_value: param.value.clone(),
        };

        let mut ctx = Context::default();
        eval(&mut ctx, crate::vega::js_runtime::VEGA_JS_PRELUDE)?;
        eval(&mut ctx, INJECTION_COLLECT_PRELUDE)?;

        let ps_json = serde_json::to_string(&ps_base).map_err(|e| e.to_string())?;
        eval(
            &mut ctx,
            &format!(
                "var __vega_ps_base = {ps_json}; var __vega_baseline_fp = {}; __vega_module_failed = false;",
                baseline_fp.raw()
            ),
        )?;

        eval(&mut ctx, source)?;
        eval(&mut ctx, "if (typeof initialize === 'function') initialize(ctx);")?;

        let plan_json = eval(&mut ctx, "JSON.stringify(__vega_plan)")?;
        serde_json::from_str(&plan_json).map_err(|e| format!("parse plan: {e}"))
    }

    /// Phase C: run `process()` once per saved response index; return alerts.
    pub fn run_process_phase(
        &self,
        source: &str,
        ps: &PathState,
        baseline_fp: ResponseFingerprint,
        saved_req: &[HttpRequest],
        saved_res: &[HttpResponse],
    ) -> Result<ModuleRunResult, String> {
        let param = ps
            .fuzzable_parameter()
            .ok_or_else(|| "path state has no fuzzable parameter".to_string())?;

        let ps_base = PsBase {
            uri: ps.uri.clone(),
            method: if ps.is_post_target { "POST".into() } else { "GET".into() },
            is_post: ps.is_post_target,
            param_name: param.name.clone(),
            param_value: param.value.clone(),
        };

        let fps: Vec<u64> = saved_res
            .iter()
            .map(|r| ResponseFingerprint::compute(r).raw())
            .collect();

        let req_json: Vec<serde_json::Value> = saved_req
            .iter()
            .map(|r| {
                serde_json::json!({
                    "method": r.method,
                    "uri": r.uri,
                    "headers": r.headers,
                    "body": r.body,
                })
            })
            .collect();
        let res_json: Vec<serde_json::Value> = saved_res
            .iter()
            .map(|r| {
                serde_json::json!({
                    "status": r.status,
                    "body": r.body,
                    "headers": r.headers,
                    "fetch_fail": r.fetch_fail,
                })
            })
            .collect();

        let mut ctx = Context::default();
        eval(&mut ctx, crate::vega::js_runtime::VEGA_JS_PRELUDE)?;
        eval(&mut ctx, INJECTION_PROCESS_PRELUDE)?;

        let inject = format!(
            "var __vega_ps_base = {}; var __vega_baseline_fp = {}; var __vega_fps = {}; \
             var __vega_saved_req = {}; var __vega_saved_res = {}; var __vega_props = {{}}; \
             __vega_alerts = []; __vega_highlights = []; __vega_seen_keys = {{}};",
            serde_json::to_string(&ps_base).map_err(|e| e.to_string())?,
            baseline_fp.raw(),
            serde_json::to_string(&fps).map_err(|e| e.to_string())?,
            serde_json::to_string(&req_json).map_err(|e| e.to_string())?,
            serde_json::to_string(&res_json).map_err(|e| e.to_string())?,
        );
        eval(&mut ctx, &inject)?;
        eval(&mut ctx, source)?;

        let max_idx = saved_res.len().saturating_sub(1);
        for i in 0..=max_idx {
            let runner = format!(
                r#"(function() {{
                  __vega_current_index = {i};
                  if (typeof process !== 'function') return;
                  var request = __mkRequest(__vega_saved_req[{i}]);
                  var response = __mkResponse(__vega_saved_res[{i}]);
                  process(request, response, ctx);
                }})()"#
            );
            eval(&mut ctx, &runner)?;
        }

        let out_json = eval(
            &mut ctx,
            "JSON.stringify({ alerts: __vega_alerts, highlights: __vega_highlights })",
        )?;
        let out: ProcessOutput =
            serde_json::from_str(&out_json).map_err(|e| format!("parse process output: {e}"))?;

        let ts = now_ms();
        let alerts = out
            .alerts
            .into_iter()
            .map(|raw| self.inner.resolve_alert_public(raw, ts))
            .collect();

        Ok(ModuleRunResult {
            alerts,
            highlights: out.highlights,
        })
    }
}

#[derive(Debug, Deserialize)]
struct ProcessOutput {
    #[serde(default)]
    alerts: Vec<RawAlert>,
    #[serde(default)]
    highlights: Vec<String>,
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

/// Build an altered HTTP request by mutating the active fuzzable parameter.
pub fn build_altered_request(
    ps: &PathState,
    payload: &str,
    append: bool,
) -> Result<HttpRequest, String> {
    let param = ps
        .fuzzable_parameter()
        .ok_or_else(|| "no fuzzable parameter".to_string())?;

    let new_value = if append {
        format!("{}{}", param.value, payload)
    } else {
        payload.to_string()
    };

    match param.location {
        ParamLocation::Query => {
            let uri = mutate_query_param(&ps.uri, &param.name, &new_value)?;
            Ok(HttpRequest {
                method: if ps.is_post_target {
                    "POST".to_string()
                } else {
                    "GET".to_string()
                },
                uri,
                ..Default::default()
            })
        }
        ParamLocation::Post => {
            let body = mutate_form_body(&param.name, &new_value, &ps.uri)?;
            let mut req = HttpRequest {
                method: "POST".into(),
                uri: ps.uri.clone(),
                body,
                ..Default::default()
            };
            req.add_header("Content-Type", "application/x-www-form-urlencoded");
            Ok(req)
        }
        _ => Err(format!("unsupported param location: {:?}", param.location)),
    }
}

fn mutate_query_param(uri: &str, param: &str, value: &str) -> Result<String, String> {
    let (base, qs) = match uri.split_once('?') {
        Some((b, q)) => (b, q),
        None => (uri, ""),
    };
    let mut pairs: HashMap<String, String> = HashMap::new();
    if !qs.is_empty() {
        for part in qs.split('&') {
            if part.is_empty() {
                continue;
            }
            let (k, v) = part
                .split_once('=')
                .map(|(a, b)| (a.to_string(), b.to_string()))
                .unwrap_or((part.to_string(), String::new()));
            pairs.insert(urlencoding_decode(&k), urlencoding_decode(&v));
        }
    }
    pairs.insert(param.to_string(), value.to_string());
    let mut out = pairs
        .iter()
        .map(|(k, v)| format!("{}={}", urlencoding_encode(k), urlencoding_encode(v)))
        .collect::<Vec<_>>();
    out.sort();
    Ok(format!("{base}?{}", out.join("&")))
}

fn mutate_form_body(param: &str, value: &str, _uri: &str) -> Result<String, String> {
    Ok(format!(
        "{}={}",
        urlencoding_encode(param),
        urlencoding_encode(value)
    ))
}

fn urlencoding_encode(s: &str) -> String {
    urlencoding::encode(s).into_owned()
}

fn urlencoding_decode(s: &str) -> String {
    urlencoding::decode(s)
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vega::model::FuzzableParam;

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
}
