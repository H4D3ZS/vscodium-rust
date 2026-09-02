//! Repeater — manual request workbench backend.
//!
//! Send an arbitrary HTTP request and get the full response back for inspection.
//! The frontend keeps the editable request/response tabs; this is the wire.
//! Authorized testing only.

use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualRequest {
    pub method: String,
    pub url: String,
    #[serde(default)]
    pub headers: Vec<(String, String)>,
    #[serde(default)]
    pub body: String,
    /// Follow 3xx redirects (off by default — pentesters usually want to see them).
    #[serde(default)]
    pub follow_redirects: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManualResponse {
    pub status: u16,
    pub status_text: String,
    pub headers: Vec<(String, String)>,
    pub body: String,
    pub body_bytes: usize,
    pub duration_ms: u64,
    pub truncated: bool,
}

const MAX_BODY: usize = 1_000_000;

fn client(follow: bool) -> &'static reqwest::Client {
    static FOLLOW: OnceLock<reqwest::Client> = OnceLock::new();
    static NOFOLLOW: OnceLock<reqwest::Client> = OnceLock::new();
    let build = |policy: reqwest::redirect::Policy| {
        reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .redirect(policy)
            .pool_max_idle_per_host(8)
            .danger_accept_invalid_certs(true) // pentest targets often have bad/self-signed TLS
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    };
    if follow {
        FOLLOW.get_or_init(|| build(reqwest::redirect::Policy::limited(10)))
    } else {
        NOFOLLOW.get_or_init(|| build(reqwest::redirect::Policy::none()))
    }
}

pub async fn send(req: ManualRequest) -> Result<ManualResponse, String> {
    let method = reqwest::Method::from_bytes(req.method.trim().as_bytes())
        .map_err(|e| format!("bad method: {e}"))?;
    let mut rb = client(req.follow_redirects).request(method, req.url.trim());
    for (k, v) in &req.headers {
        if k.trim().is_empty() {
            continue;
        }
        rb = rb.header(k.trim(), v);
    }
    if !req.body.is_empty() {
        rb = rb.body(req.body);
    }

    let started = Instant::now();
    let resp = rb.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let full = resp.bytes().await.map_err(|e| e.to_string())?;
    let body_bytes = full.len();
    let truncated = body_bytes > MAX_BODY;
    let slice = &full[..body_bytes.min(MAX_BODY)];

    Ok(ManualResponse {
        status: status.as_u16(),
        status_text: status.canonical_reason().unwrap_or("").to_string(),
        headers,
        body: String::from_utf8_lossy(slice).to_string(),
        body_bytes,
        duration_ms: started.elapsed().as_millis() as u64,
        truncated,
    })
}
