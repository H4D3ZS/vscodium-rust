//! Multi-key API probe engine — brutecat methodology.
//!
//! Core innovation: fire every request across all collected API keys simultaneously,
//! then group responses by SHA-256 hash. Identical responses collapse to one entry,
//! dramatically cutting token usage when reporting to the AI. Divergent responses
//! (different per-key auth, per-key scoping, per-key visibility labels) stand out
//! immediately.
//!
//! Error normalization converts raw HTTP status+body into typed `ApiError` variants
//! so the AI always receives structured signal — not raw 404 JSON that varies per endpoint.
//!
//! Authorized testing / bug bounty use only.

use reqwest::{header::HeaderMap, Method};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::OnceLock,
    time::{Duration, Instant},
};
use tokio::sync::Semaphore;
use std::sync::Arc;

// ── Key restriction types ─────────────────────────────────────────────────────

/// How the API key is scoped. Determines which extra headers the probe injects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum KeyRestriction {
    #[default]
    None,
    /// HTTP Referer validation — inject `Referer: <origin>`.
    Browser { origin: String },
    /// Android package + signing fingerprint.
    Android { package: String, certificate_hash: String },
    /// iOS bundle identifier.
    Ios { bundle_id: String },
    /// IP whitelist — non-bypassable; probe anyway but expect 403.
    Server,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    /// The key value itself (e.g. `AIza...`).
    pub value: String,
    /// Human label for display (project name, apk source, etc.).
    pub label: String,
    pub restriction: KeyRestriction,
}

// ── Error taxonomy ────────────────────────────────────────────────────────────

/// Typed API error — prevents AI from treating all 4xx identically.
/// Maps the patterns seen in Google Discovery API responses; generalises to others.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ApiError {
    /// HTML body (or no Content-Type) — endpoint does not exist in this API version.
    MethodNotFound,
    /// JSON 404 with `visibilityLabel` field — endpoint hidden behind label.
    VisibilityLabelRequired { label: String },
    /// 400 with detail — invalid argument (field missing or wrong type).
    InvalidArgument { detail: String },
    /// 401 / `UNAUTHENTICATED` — key missing or expired.
    Unauthenticated,
    /// 403 / `PERMISSION_DENIED` — key valid but lacks required IAM role.
    PermissionDenied,
    /// 429 — rate limited.
    RateLimited,
    /// 5xx.
    ServerError { code: u16 },
    /// Catch-all for anything else.
    Unknown { code: u16, snippet: String },
}

impl ApiError {
    fn classify(status: u16, body: &str) -> Self {
        match status {
            200..=299 => unreachable!("classify only called on errors"),
            400 => {
                // Try to pull a useful detail string from common JSON error formats.
                let detail = if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
                    v["error"]["message"].as_str()
                        .or_else(|| v["message"].as_str())
                        .unwrap_or("no_detail")
                        .chars()
                        .take(200)
                        .collect()
                } else {
                    "no_detail".into()
                };
                Self::InvalidArgument { detail }
            }
            401 => Self::Unauthenticated,
            403 => Self::PermissionDenied,
            404 => {
                // Distinguish HTML 404 (method not found) from JSON 404 (visibility gate).
                if body.trim_start().starts_with('<') {
                    return Self::MethodNotFound;
                }
                if let Ok(v) = serde_json::from_str::<serde_json::Value>(body) {
                    if let Some(label) = v["visibilityLabel"]
                        .as_str()
                        .or_else(|| v["error"]["visibilityLabel"].as_str())
                    {
                        return Self::VisibilityLabelRequired {
                            label: label.to_string(),
                        };
                    }
                    // Generic JSON 404 — no visibility label.
                    return Self::MethodNotFound;
                }
                Self::MethodNotFound
            }
            429 => Self::RateLimited,
            500..=599 => Self::ServerError { code: status },
            code => Self::Unknown {
                code,
                snippet: body.chars().take(120).collect(),
            },
        }
    }
}

// ── Request / response types ──────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeRequest {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
    /// Optional visibility label to inject (iterates known internal labels).
    #[serde(default)]
    pub visibility_label: Option<String>,
    #[serde(default)]
    pub follow_redirects: bool,
}

/// Single per-key probe result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeResult {
    pub op_id: String,
    pub key_label: String,
    pub status: u16,
    pub headers: Vec<(String, String)>,
    /// Body truncated to 8 KB.
    pub body: String,
    /// SHA-256 hex of the raw body. Used for deduplication.
    pub body_hash: String,
    pub duration_ms: u64,
    pub error: Option<ApiError>,
}

/// Deduplicated probe results grouped by response hash.
/// All keys that produced the same response collapse into one `ProbeGroup` entry.
/// Divergent responses (key-scoped data, different auth levels) each get their own entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProbeGroup {
    /// Hash → (sample_result, keys_that_matched)
    pub by_hash: HashMap<String, GroupEntry>,
    /// Total requests fired.
    pub fired: usize,
    /// Any per-key errors (network-level, not HTTP-level).
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupEntry {
    /// Representative result for this hash group.
    pub sample: ProbeResult,
    /// Labels of all keys that produced this exact response.
    pub key_labels: Vec<String>,
    pub count: usize,
}

impl ProbeGroup {
    fn new() -> Self {
        Self {
            by_hash: HashMap::new(),
            fired: 0,
            errors: Vec::new(),
        }
    }

    fn insert(&mut self, result: ProbeResult) {
        self.fired += 1;
        let hash = result.body_hash.clone();
        let label = result.key_label.clone();
        let entry = self.by_hash.entry(hash).or_insert_with(|| GroupEntry {
            sample: result,
            key_labels: Vec::new(),
            count: 0,
        });
        entry.key_labels.push(label);
        entry.count += 1;
    }
}

// ── HTTP client ───────────────────────────────────────────────────────────────

const MAX_BODY: usize = 8_192;
const TIMEOUT_SECS: u64 = 20;

fn http_client(follow: bool) -> &'static reqwest::Client {
    static FOLLOW: OnceLock<reqwest::Client> = OnceLock::new();
    static NOFOLLOW: OnceLock<reqwest::Client> = OnceLock::new();
    let build = |policy: reqwest::redirect::Policy| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(TIMEOUT_SECS))
            .redirect(policy)
            .pool_max_idle_per_host(20)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    };
    if follow {
        FOLLOW.get_or_init(|| build(reqwest::redirect::Policy::limited(5)))
    } else {
        NOFOLLOW.get_or_init(|| build(reqwest::redirect::Policy::none()))
    }
}

fn hash_body(body: &[u8]) -> String {
    let mut h = Sha256::new();
    h.update(body);
    format!("{:x}", h.finalize())
}

fn inject_key_headers(
    headers: &mut HeaderMap,
    key: &ApiKey,
    visibility_label: Option<&str>,
) {
    // Inject restriction headers.
    match &key.restriction {
        KeyRestriction::Browser { origin } => {
            headers.insert(
                reqwest::header::REFERER,
                origin.parse().unwrap_or_else(|_| "https://example.com".parse().unwrap()),
            );
        }
        KeyRestriction::Android { package, certificate_hash } => {
            headers.insert(
                "X-Android-Package",
                package.parse().unwrap_or_else(|_| "com.google.android.apps.unknown".parse().unwrap()),
            );
            headers.insert(
                "X-Android-Cert",
                certificate_hash.parse().unwrap_or_else(|_| "0".parse().unwrap()),
            );
        }
        KeyRestriction::Ios { bundle_id } => {
            headers.insert(
                "X-Ios-Bundle-Identifier",
                bundle_id.parse().unwrap_or_else(|_| "com.google.unknown".parse().unwrap()),
            );
        }
        KeyRestriction::None | KeyRestriction::Server => {}
    }
    // Inject visibility label header if the caller is iterating labels.
    if let Some(label) = visibility_label {
        headers.insert(
            "X-Goog-Fieldmask",
            label.parse().unwrap_or_else(|_| "GOOGLE_INTERNAL".parse().unwrap()),
        );
    }
}

// ── Engine ────────────────────────────────────────────────────────────────────

pub struct ProbeEngine {
    /// Hard cap: at most 20 concurrent in-flight requests.
    semaphore: Arc<Semaphore>,
}

impl ProbeEngine {
    pub fn new() -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(20)),
        }
    }

    /// Fire `req` against every key in `keys` concurrently, deduplicate by hash.
    /// `op_id_prefix` is prepended to per-key IDs: `<prefix>-k0`, `<prefix>-k1`, …
    pub async fn probe_all(
        &self,
        req: ProbeRequest,
        keys: &[ApiKey],
        op_id_prefix: &str,
    ) -> ProbeGroup {
        let mut tasks = Vec::with_capacity(keys.len());
        let req = Arc::new(req);

        for (i, key) in keys.iter().enumerate() {
            let req = req.clone();
            let key = key.clone();
            let op_id = format!("{op_id_prefix}-k{i}");
            let sem = self.semaphore.clone();

            tasks.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                fire_single(&req, &key, &op_id).await
            }));
        }

        // If no keys supplied, fire once without a key param.
        if keys.is_empty() {
            let req_clone = req.clone();
            let op_id = format!("{op_id_prefix}-k0");
            let bare_key = ApiKey {
                value: String::new(),
                label: "no-key".into(),
                restriction: KeyRestriction::None,
            };
            let sem = self.semaphore.clone();
            tasks.push(tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                fire_single(&req_clone, &bare_key, &op_id).await
            }));
        }

        let mut group = ProbeGroup::new();
        for t in tasks {
            match t.await {
                Ok(Ok(result)) => group.insert(result),
                Ok(Err(e)) => group.errors.push(e),
                Err(e) => group.errors.push(e.to_string()),
            }
        }
        group
    }
}

async fn fire_single(
    req: &ProbeRequest,
    key: &ApiKey,
    op_id: &str,
) -> Result<ProbeResult, String> {
    let method = Method::from_bytes(req.method.trim().as_bytes())
        .map_err(|e| format!("bad method: {e}"))?;

    // Build URL — append key as query param if non-empty.
    let url = if key.value.is_empty() {
        req.url.clone()
    } else {
        let sep = if req.url.contains('?') { '&' } else { '?' };
        format!("{}{}key={}", req.url, sep, key.value)
    };

    let client = http_client(req.follow_redirects);
    let mut rb = client.request(method, url.trim());

    // Base headers from caller.
    let mut header_map = HeaderMap::new();
    for (k, v) in &req.headers {
        if let (Ok(name), Ok(val)) = (
            reqwest::header::HeaderName::from_bytes(k.as_bytes()),
            reqwest::header::HeaderValue::from_str(v),
        ) {
            header_map.insert(name, val);
        }
    }

    inject_key_headers(&mut header_map, key, req.visibility_label.as_deref());
    rb = rb.headers(header_map);

    if !req.body.is_empty() {
        rb = rb
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(req.body.clone());
    }

    let t0 = Instant::now();
    let resp = rb.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let resp_headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let raw = resp.bytes().await.map_err(|e| e.to_string())?;
    let duration_ms = t0.elapsed().as_millis() as u64;

    let body_hash = hash_body(&raw);
    let truncated = raw.len() > MAX_BODY;
    let body = String::from_utf8_lossy(if truncated { &raw[..MAX_BODY] } else { &raw }).to_string();

    let error = if status < 200 || status >= 300 {
        Some(ApiError::classify(status, &body))
    } else {
        None
    };

    Ok(ProbeResult {
        op_id: op_id.to_string(),
        key_label: key.label.clone(),
        status,
        headers: resp_headers,
        body,
        body_hash,
        duration_ms,
        error,
    })
}

// ── Known visibility labels (Google Discovery API) ────────────────────────────

/// Visibility labels seen in the wild. Pass each to `visibility_label` field
/// to probe endpoints gated behind them.
pub const KNOWN_VISIBILITY_LABELS: &[&str] = &[
    "GOOGLE_INTERNAL",
    "GOOGLE_INTERNAL_CLOUD",
    "TRUSTED_TESTER",
    "GOOGLE_ALPHA",
    "GOOGLE_BETA",
    "GOOGLE_PARTNER",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_visibility_label() {
        let body = r#"{"visibilityLabel":"GOOGLE_INTERNAL","error":{"code":404}}"#;
        let err = ApiError::classify(404, body);
        assert!(matches!(err, ApiError::VisibilityLabelRequired { label } if label == "GOOGLE_INTERNAL"));
    }

    #[test]
    fn classify_html_404_as_method_not_found() {
        let err = ApiError::classify(404, "<html><body>Not Found</body></html>");
        assert!(matches!(err, ApiError::MethodNotFound));
    }

    #[test]
    fn hash_deterministic() {
        let a = hash_body(b"hello");
        let b = hash_body(b"hello");
        assert_eq!(a, b);
        let c = hash_body(b"world");
        assert_ne!(a, c);
    }
}
