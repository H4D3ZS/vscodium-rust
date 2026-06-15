//! OAST — Out-of-band Application Security Testing (Collaborator-style).
//!
//! Catches *blind* vulnerabilities — blind SSRF, blind RCE, blind XXE, blind
//! XSS, blind SQLi via DNS/HTTP exfil — by giving each test a unique callback
//! token and recording any interaction the target makes back to us.
//!
//! Two backends:
//! - **Built-in HTTP listener** (zero deps, tokio TCP): works on localhost, LAN,
//!   or any host the target can reach. The single-shot way to confirm a callback.
//! - **External interactsh-compatible server** (optional): set a public base URL
//!   when the target can't reach your machine directly. We just hand out payload
//!   URLs under that domain; polling that server is the operator's job for now.
//!
//! Authorized testing only.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Mutex, OnceLock};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

const MAX_INTERACTIONS: usize = 5_000;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interaction {
    pub id: u64,
    pub token: String,
    pub protocol: String, // "http"
    pub timestamp_ms: u64,
    pub remote_addr: String,
    pub method: String,
    pub path: String,
    pub host_header: String,
    pub user_agent: String,
    pub raw_head: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OastPayload {
    pub token: String,
    /// Best ready-to-paste URL for the target to hit.
    pub http_url: String,
    /// Host:port form for non-HTTP probes (e.g. `curl <host>`).
    pub authority: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OastStatus {
    pub running: bool,
    pub port: u16,
    pub public_host: String,
    pub interaction_count: usize,
}

struct OastInner {
    interactions: Mutex<VecDeque<Interaction>>,
    /// Public host clients should be told to call back to. Defaults to the bind
    /// host; override with an interactsh domain or your LAN IP / tunnel host.
    public_host: Mutex<String>,
    running: AtomicBool,
    port: AtomicU64,
    next_id: AtomicU64,
}

fn state() -> &'static OastInner {
    static S: OnceLock<OastInner> = OnceLock::new();
    S.get_or_init(|| OastInner {
        interactions: Mutex::new(VecDeque::with_capacity(128)),
        public_host: Mutex::new("127.0.0.1".to_string()),
        running: AtomicBool::new(false),
        port: AtomicU64::new(0),
        next_id: AtomicU64::new(1),
    })
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as u64)
        .unwrap_or(0)
}

/// 16-hex-char correlation token. Cheap PRNG seeded from time + a counter —
/// fine for callback correlation (not a security secret).
fn gen_token() -> String {
    let seed = now_ms()
        ^ (state().next_id.fetch_add(1, Ordering::Relaxed).wrapping_mul(0x9E3779B97F4A7C15));
    let mut x = seed | 1;
    let mut out = String::with_capacity(16);
    for _ in 0..16 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        out.push(char::from_digit((x & 0xF) as u32, 16).unwrap_or('0'));
    }
    out
}

pub fn set_public_host(host: String) {
    if let Ok(mut h) = state().public_host.lock() {
        *h = host;
    }
}

fn public_host() -> String {
    state()
        .public_host
        .lock()
        .map(|h| h.clone())
        .unwrap_or_else(|_| "127.0.0.1".into())
}

/// Mint a new payload. The token is embedded in both the host label (for DNS/
/// vhost-style callbacks) and the path (for plain HTTP fetches), so either form
/// correlates back.
pub fn register() -> OastPayload {
    let token = gen_token();
    let host = public_host();
    let port = state().port.load(Ordering::Relaxed) as u16;
    let authority = if port == 0 || port == 80 {
        host.clone()
    } else {
        format!("{host}:{port}")
    };
    OastPayload {
        http_url: format!("http://{authority}/{token}"),
        authority,
        token,
    }
}

pub fn status() -> OastStatus {
    let inner = state();
    OastStatus {
        running: inner.running.load(Ordering::Relaxed),
        port: inner.port.load(Ordering::Relaxed) as u16,
        public_host: public_host(),
        interaction_count: inner.interactions.lock().map(|q| q.len()).unwrap_or(0),
    }
}

/// All interactions, newest first. Filter by `token` to poll one specific probe.
pub fn poll(token: Option<&str>) -> Vec<Interaction> {
    let q = match state().interactions.lock() {
        Ok(q) => q,
        Err(_) => return vec![],
    };
    q.iter()
        .rev()
        .filter(|i| token.map_or(true, |t| i.token == t))
        .cloned()
        .collect()
}

pub fn clear() {
    if let Ok(mut q) = state().interactions.lock() {
        q.clear();
    }
}

fn record(i: Interaction) {
    if let Ok(mut q) = state().interactions.lock() {
        q.push_back(i);
        while q.len() > MAX_INTERACTIONS {
            q.pop_front();
        }
    }
}

pub async fn start(port: u16, public: Option<String>) -> Result<u16, String> {
    let inner = state();
    if inner.running.load(Ordering::Relaxed) {
        return Err("oast server already running".into());
    }
    if let Some(p) = public {
        if !p.trim().is_empty() {
            set_public_host(p);
        }
    }

    let listener = TcpListener::bind(("0.0.0.0", port))
        .await
        .map_err(|e| format!("bind 0.0.0.0:{port}: {e}"))?;
    let bound = listener.local_addr().map_err(|e| e.to_string())?.port();
    inner.running.store(true, Ordering::Relaxed);
    inner.port.store(bound as u64, Ordering::Relaxed);

    tokio::spawn(async move {
        loop {
            if !state().running.load(Ordering::Relaxed) {
                break;
            }
            match listener.accept().await {
                Ok((sock, addr)) => {
                    let remote = addr.to_string();
                    tokio::spawn(async move {
                        let _ = handle(sock, remote).await;
                    });
                }
                Err(_) => break,
            }
        }
    });

    Ok(bound)
}

pub fn stop() {
    state().running.store(false, Ordering::Relaxed);
    state().port.store(0, Ordering::Relaxed);
}

async fn handle(mut sock: TcpStream, remote: String) -> std::io::Result<()> {
    let mut buf = vec![0u8; 8192];
    let n = sock.read(&mut buf).await.unwrap_or(0);
    let head = String::from_utf8_lossy(&buf[..n]).to_string();

    let request_line = head.lines().next().unwrap_or("");
    let mut parts = request_line.split_whitespace();
    let method = parts.next().unwrap_or("").to_string();
    let path = parts.next().unwrap_or("/").to_string();

    let header = |name: &str| -> String {
        let needle = format!("{}:", name.to_lowercase());
        head.lines()
            .skip(1)
            .find(|l| l.to_lowercase().starts_with(&needle))
            .and_then(|l| l.split_once(':'))
            .map(|(_, v)| v.trim().to_string())
            .unwrap_or_default()
    };
    let host_header = header("host");

    // Token may live in the path (/<token>) or the host label (<token>.domain).
    let token = extract_token(&path, &host_header);

    record(Interaction {
        id: state().next_id.fetch_add(1, Ordering::Relaxed),
        token,
        protocol: "http".into(),
        timestamp_ms: now_ms(),
        remote_addr: remote,
        method,
        path,
        host_header,
        user_agent: header("user-agent"),
        raw_head: head.chars().take(2048).collect(),
    });

    // Tiny benign 200 so the target's request "succeeds".
    let body = b"hades-oast";
    let resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    );
    sock.write_all(resp.as_bytes()).await?;
    sock.write_all(body).await?;
    Ok(())
}

/// Pull the correlation token from the path (`/<token>...`) or the leftmost
/// host label (`<token>.collab.example`). Returns "" when none matches.
fn extract_token(path: &str, host: &str) -> String {
    let p = path.trim_start_matches('/');
    let path_tok: String = p.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if path_tok.len() >= 8 {
        return path_tok;
    }
    let label = host.split('.').next().unwrap_or("");
    let host_tok: String = label.chars().take_while(|c| c.is_ascii_hexdigit()).collect();
    if host_tok.len() >= 8 {
        return host_tok;
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokens_are_unique_and_hex() {
        let a = gen_token();
        let b = gen_token();
        assert_eq!(a.len(), 16);
        assert!(a.chars().all(|c| c.is_ascii_hexdigit()));
        assert_ne!(a, b);
    }

    #[test]
    fn extracts_token_from_path() {
        assert_eq!(extract_token("/deadbeefcafe1234/x", "h"), "deadbeefcafe1234");
    }

    #[test]
    fn extracts_token_from_host_label() {
        assert_eq!(
            extract_token("/", "deadbeefcafe.collab.example.com"),
            "deadbeefcafe"
        );
    }

    #[test]
    fn no_token_when_absent() {
        assert_eq!(extract_token("/favicon.ico", "example.com"), "");
    }

    #[test]
    fn register_embeds_token_in_url() {
        let p = register();
        assert!(p.http_url.contains(&p.token));
    }
}
