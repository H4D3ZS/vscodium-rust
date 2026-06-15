//! Rust-native intercepting proxy — a lightweight Burp/Hetty alternative.
//!
//! Design goals: zero new dependencies, fully offline, snappy. Built directly on
//! tokio TCP so it runs anywhere the IDE does.
//!
//! Capabilities:
//! - **Plain HTTP**: full request/response capture (method, URL, headers, body
//!   previews, status, timing, byte counts) with transparent forwarding.
//! - **HTTPS (CONNECT)**: tunneled end-to-end with flow metadata (host, bytes,
//!   duration). Payloads stay encrypted — TLS decryption (on-the-fly CA) is a
//!   planned upgrade and intentionally not done here to avoid a trust-store UX.
//! - In-memory ring buffer of flows, queryable + replayable from the UI/agent.
//!
//! Authorized testing only.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_FLOWS: usize = 2_000;
const BODY_PREVIEW_CAP: usize = 8_192;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub id: u64,
    pub timestamp_ms: u64,
    pub scheme: String, // "http" | "https"
    pub method: String,
    pub host: String,
    pub url: String,
    /// False for HTTPS CONNECT tunnels (payload encrypted, metadata only).
    pub intercepted: bool,
    pub req_headers: Vec<(String, String)>,
    pub req_body_preview: String,
    pub status: u16,
    pub resp_headers: Vec<(String, String)>,
    pub resp_body_preview: String,
    pub bytes_up: u64,
    pub bytes_down: u64,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyStatus {
    pub running: bool,
    pub port: u16,
    pub flow_count: usize,
}

struct ProxyInner {
    flows: Mutex<VecDeque<Flow>>,
    running: AtomicBool,
    port: AtomicU64,
    next_id: AtomicU64,
}

fn state() -> &'static ProxyInner {
    static S: OnceLock<ProxyInner> = OnceLock::new();
    S.get_or_init(|| ProxyInner {
        flows: Mutex::new(VecDeque::with_capacity(256)),
        running: AtomicBool::new(false),
        port: AtomicU64::new(0),
        next_id: AtomicU64::new(1),
    })
}

/// Pooled upstream client. Reused across every forwarded request and replay so
/// we keep keep-alive connections warm instead of rebuilding a pool per call.
fn upstream_client() -> &'static reqwest::Client {
    static C: OnceLock<reqwest::Client> = OnceLock::new();
    C.get_or_init(|| {
        reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .pool_max_idle_per_host(16)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new())
    })
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

fn record_flow(f: Flow) {
    let inner = state();
    if let Ok(mut q) = inner.flows.lock() {
        q.push_back(f);
        while q.len() > MAX_FLOWS {
            q.pop_front();
        }
    }
}

pub fn status() -> ProxyStatus {
    let inner = state();
    ProxyStatus {
        running: inner.running.load(Ordering::Relaxed),
        port: inner.port.load(Ordering::Relaxed) as u16,
        flow_count: inner.flows.lock().map(|q| q.len()).unwrap_or(0),
    }
}

pub fn list_flows(limit: usize) -> Vec<Flow> {
    let inner = state();
    let q = match inner.flows.lock() {
        Ok(q) => q,
        Err(_) => return vec![],
    };
    q.iter().rev().take(limit).cloned().collect()
}

pub fn clear_flows() {
    if let Ok(mut q) = state().flows.lock() {
        q.clear();
    }
}

/// Replay a captured HTTP flow through reqwest and return the fresh response.
/// HTTPS tunnels can't be replayed (we never saw the plaintext).
pub async fn replay_flow(id: u64) -> Result<serde_json::Value, String> {
    let flow = {
        let q = state().flows.lock().map_err(|_| "lock")?;
        q.iter().find(|f| f.id == id).cloned()
    }
    .ok_or_else(|| format!("flow {id} not found"))?;

    if !flow.intercepted {
        return Err("cannot replay an HTTPS tunnel (payload was encrypted)".into());
    }

    let client = upstream_client();
    let method = reqwest::Method::from_bytes(flow.method.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut req = client.request(method, &flow.url);
    for (k, v) in &flow.req_headers {
        // Hop-by-hop headers must not be forwarded on replay.
        if is_hop_by_hop(k) {
            continue;
        }
        req = req.header(k, v);
    }
    if !flow.req_body_preview.is_empty() {
        req = req.body(flow.req_body_preview.clone());
    }

    let started = std::time::Instant::now();
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let body = resp.text().await.unwrap_or_default();
    let preview: String = body.chars().take(BODY_PREVIEW_CAP).collect();

    Ok(serde_json::json!({
        "status": status,
        "headers": headers,
        "body_preview": preview,
        "duration_ms": started.elapsed().as_millis() as u64,
    }))
}

fn is_hop_by_hop(name: &str) -> bool {
    let n = name.to_lowercase();
    matches!(
        n.as_str(),
        "connection"
            | "proxy-connection"
            | "keep-alive"
            | "transfer-encoding"
            | "te"
            | "trailer"
            | "upgrade"
            | "host"
    )
}

/// Start the proxy on `port` (0 = OS-chosen). Spawns the accept loop and returns
/// the bound port. Idempotent-ish: refuses to start if already running.
pub async fn start_proxy(port: u16) -> Result<u16, String> {
    let inner = state();
    if inner.running.load(Ordering::Relaxed) {
        return Err("proxy already running".into());
    }

    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .map_err(|e| format!("bind 127.0.0.1:{port}: {e}"))?;
    let bound = listener.local_addr().map_err(|e| e.to_string())?.port();

    inner.running.store(true, Ordering::Relaxed);
    inner.port.store(bound as u64, Ordering::Relaxed);

    tokio::spawn(async move {
        loop {
            if !state().running.load(Ordering::Relaxed) {
                break;
            }
            match listener.accept().await {
                Ok((sock, _)) => {
                    tokio::spawn(async move {
                        let _ = handle_conn(sock).await;
                    });
                }
                Err(_) => break,
            }
        }
    });

    Ok(bound)
}

pub fn stop_proxy() {
    state().running.store(false, Ordering::Relaxed);
    state().port.store(0, Ordering::Relaxed);
}

/// Read the request head (up to the blank line) plus whatever body bytes arrived
/// with it. Returns (head_bytes, leftover_after_head).
async fn read_head(stream: &mut TcpStream) -> std::io::Result<(Vec<u8>, Vec<u8>)> {
    let mut buf = Vec::with_capacity(4096);
    let mut tmp = [0u8; 4096];
    loop {
        let n = stream.read(&mut tmp).await?;
        if n == 0 {
            break;
        }
        buf.extend_from_slice(&tmp[..n]);
        if let Some(pos) = find_double_crlf(&buf) {
            let leftover = buf.split_off(pos + 4);
            return Ok((buf, leftover));
        }
        if buf.len() > 64 * 1024 {
            break; // oversized head — bail
        }
    }
    Ok((buf, Vec::new()))
}

fn find_double_crlf(b: &[u8]) -> Option<usize> {
    b.windows(4).position(|w| w == b"\r\n\r\n")
}

async fn handle_conn(mut client: TcpStream) -> std::io::Result<()> {
    let (head, leftover) = read_head(&mut client).await?;
    if head.is_empty() {
        return Ok(());
    }
    let head_str = String::from_utf8_lossy(&head).to_string();
    let mut lines = head_str.lines();
    let request_line = lines.next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let target = parts.next().unwrap_or("").to_string();

    if method.eq_ignore_ascii_case("CONNECT") {
        return handle_connect(client, &target).await;
    }
    handle_http(client, &head_str, &method, &target, leftover).await
}

/// HTTPS: reply 200 then splice both directions, capturing byte counts only.
async fn handle_connect(mut client: TcpStream, authority: &str) -> std::io::Result<()> {
    let host = authority.to_string();
    let mut upstream = match TcpStream::connect(authority).await {
        Ok(s) => s,
        Err(_) => {
            let _ = client
                .write_all(b"HTTP/1.1 502 Bad Gateway\r\n\r\n")
                .await;
            return Ok(());
        }
    };
    client
        .write_all(b"HTTP/1.1 200 Connection Established\r\n\r\n")
        .await?;

    let id = state().next_id.fetch_add(1, Ordering::Relaxed);
    let started = std::time::Instant::now();
    let (up, down) = tokio::io::copy_bidirectional(&mut client, &mut upstream)
        .await
        .unwrap_or((0, 0));

    record_flow(Flow {
        id,
        timestamp_ms: now_ms(),
        scheme: "https".into(),
        method: "CONNECT".into(),
        host: host.clone(),
        url: format!("https://{host}"),
        intercepted: false,
        req_headers: vec![],
        req_body_preview: String::new(),
        status: 200,
        resp_headers: vec![],
        resp_body_preview: String::new(),
        bytes_up: up,
        bytes_down: down,
        duration_ms: started.elapsed().as_millis() as u64,
    });
    Ok(())
}

/// Plain HTTP: forward via reqwest, capture full request + response.
async fn handle_http(
    mut client: TcpStream,
    head_str: &str,
    method: &str,
    target: &str,
    mut leftover: Vec<u8>,
) -> std::io::Result<()> {
    // Parse headers from the head.
    let mut headers: Vec<(String, String)> = Vec::new();
    let mut content_length = 0usize;
    let mut host_hdr = String::new();
    for line in head_str.lines().skip(1) {
        if line.is_empty() {
            break;
        }
        if let Some((k, v)) = line.split_once(':') {
            let k = k.trim().to_string();
            let v = v.trim().to_string();
            if k.eq_ignore_ascii_case("content-length") {
                content_length = v.parse().unwrap_or(0);
            }
            if k.eq_ignore_ascii_case("host") {
                host_hdr = v.clone();
            }
            headers.push((k, v));
        }
    }

    // Absolute-form (proxy) target preferred; else reconstruct from Host.
    let url = if target.starts_with("http://") || target.starts_with("https://") {
        target.to_string()
    } else {
        format!("http://{host_hdr}{target}")
    };

    // Read remaining body bytes up to content-length.
    while leftover.len() < content_length {
        let mut tmp = [0u8; 4096];
        let n = client.read(&mut tmp).await?;
        if n == 0 {
            break;
        }
        leftover.extend_from_slice(&tmp[..n]);
    }
    let body = leftover;

    let id = state().next_id.fetch_add(1, Ordering::Relaxed);
    let started = std::time::Instant::now();

    let forward = forward_http(method, &url, &headers, &body).await;
    match forward {
        Ok((status, resp_headers, resp_body)) => {
            // Write a clean HTTP/1.1 response back to the client.
            let mut out = format!("HTTP/1.1 {status}\r\n");
            for (k, v) in &resp_headers {
                if is_hop_by_hop(k) || k.eq_ignore_ascii_case("content-length") {
                    continue;
                }
                out.push_str(&format!("{k}: {v}\r\n"));
            }
            out.push_str(&format!("Content-Length: {}\r\n", resp_body.len()));
            out.push_str("Connection: close\r\n\r\n");
            client.write_all(out.as_bytes()).await?;
            client.write_all(&resp_body).await?;

            record_flow(Flow {
                id,
                timestamp_ms: now_ms(),
                scheme: "http".into(),
                method: method.to_string(),
                host: host_hdr,
                url,
                intercepted: true,
                req_headers: headers,
                req_body_preview: preview_bytes(&body),
                status,
                resp_headers,
                resp_body_preview: preview_bytes(&resp_body),
                bytes_up: body.len() as u64,
                bytes_down: resp_body.len() as u64,
                duration_ms: started.elapsed().as_millis() as u64,
            });
        }
        Err(e) => {
            let msg = format!("HTTP/1.1 502 Bad Gateway\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", e.len(), e);
            let _ = client.write_all(msg.as_bytes()).await;
        }
    }
    Ok(())
}

async fn forward_http(
    method: &str,
    url: &str,
    headers: &[(String, String)],
    body: &[u8],
) -> Result<(u16, Vec<(String, String)>, Vec<u8>), String> {
    let client = upstream_client();
    let m = reqwest::Method::from_bytes(method.as_bytes()).map_err(|e| e.to_string())?;
    let mut req = client.request(m, url);
    for (k, v) in headers {
        if is_hop_by_hop(k) {
            continue;
        }
        req = req.header(k, v);
    }
    if !body.is_empty() {
        req = req.body(body.to_vec());
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    let resp_headers: Vec<(String, String)> = resp
        .headers()
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
        .collect();
    let bytes = resp.bytes().await.map_err(|e| e.to_string())?.to_vec();
    Ok((status, resp_headers, bytes))
}

fn preview_bytes(b: &[u8]) -> String {
    let take = b.len().min(BODY_PREVIEW_CAP);
    String::from_utf8_lossy(&b[..take]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hop_by_hop_detection() {
        assert!(is_hop_by_hop("Connection"));
        assert!(is_hop_by_hop("Proxy-Connection"));
        assert!(!is_hop_by_hop("X-Api-Key"));
    }

    #[test]
    fn double_crlf_found() {
        assert_eq!(find_double_crlf(b"GET / HTTP/1.1\r\nHost: x\r\n\r\n"), Some(23));
    }

    #[test]
    fn preview_truncates() {
        let big = vec![b'a'; BODY_PREVIEW_CAP + 100];
        assert_eq!(preview_bytes(&big).len(), BODY_PREVIEW_CAP);
    }
}
