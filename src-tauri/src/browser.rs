use serde_json::{Value, json};
use regex::Regex;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};

pub struct BrowserSession {
    pub url: String,
    pub html: String,
    pub text: String,
    pub title: String,
}

pub struct SendBrowser(pub BrowserSession);
unsafe impl Send for SendBrowser {}
unsafe impl Sync for SendBrowser {}

// ───────────────────────── Real browser engine ──────────────────────────────
// A long-lived Python sidecar drives invisible_playwright (stealth Firefox).
// We talk to it over stdio with line-delimited JSON. The `browser` session
// cache below mirrors the last page so existing readers keep working.

/// The embedded sidecar script, written to a temp file and run on first use so
/// it works in both `tauri dev` and a bundled build without path juggling.
const SIDECAR_PY: &str = include_str!("../sidecars/browser_agent.py");

pub struct BrowserProc {
    child: Child,
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
}

/// Politely close the browser then kill the sidecar process. Exposed so other
/// modules (e.g. ai_tools) can shut the live browser down without touching the
/// private process fields.
pub async fn shutdown_proc(p: &mut BrowserProc) -> Result<(), String> {
    let _ = send_cmd(p, "close", json!({}), 10).await;
    p.child.kill().await.map_err(|e| e.to_string())
}

fn sidecar_path() -> std::path::PathBuf {
    std::env::temp_dir().join("vscodium_browser_agent.py")
}

/// Pick a python interpreter that exists. invisible_playwright must be importable
/// from it (the user confirmed `python -c "import invisible_playwright"` works).
fn python_exe() -> String {
    for cand in ["python", "py", "python3"] {
        let ok = std::process::Command::new(cand)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if ok {
            return cand.to_string();
        }
    }
    "python".to_string()
}

/// Locate a bundled, frozen sidecar shipped with the installer (PyInstaller
/// `browser-agent.exe` with invisible_playwright baked in) — so a released build
/// drives the browser WITHOUT the user installing Python. Searched next to the
/// IDE exe and in `resources/` / `binaries/` siblings. None in dev.
fn bundled_sidecar() -> Option<std::path::PathBuf> {
    let dir = std::env::current_exe().ok()?.parent()?.to_path_buf();
    let names = ["browser-agent.exe", "browser-agent", "browser_agent.exe", "browser_agent"];
    let mut roots = vec![dir.clone()];
    for sub in ["resources", "binaries"] {
        roots.push(dir.join(sub));
    }
    for root in roots {
        for n in names {
            let p = root.join(n);
            if p.exists() {
                return Some(p);
            }
        }
    }
    None
}

async fn start_sidecar() -> Result<BrowserProc, String> {
    // Prefer the bundled frozen sidecar (no Python needed); fall back to system
    // Python + the embedded script for dev / source runs.
    let (cmd, arg): (String, Option<std::path::PathBuf>) = match bundled_sidecar() {
        Some(exe) => (exe.to_string_lossy().to_string(), None),
        None => {
            let path = sidecar_path();
            std::fs::write(&path, SIDECAR_PY).map_err(|e| format!("write sidecar: {e}"))?;
            (python_exe(), Some(path))
        }
    };
    let mut command = Command::new(&cmd);
    if let Some(p) = &arg {
        command.arg(p);
    }
    let mut child = command
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null()) // discard Playwright logs / download progress
        .kill_on_drop(true)
        .spawn()
        .map_err(|e| format!("spawn browser sidecar ({cmd}): {e}. For source runs: pip install playwright invisible_playwright"))?;
    let stdin = child.stdin.take().ok_or("sidecar: no stdin")?;
    let stdout = child.stdout.take().ok_or("sidecar: no stdout")?;
    Ok(BrowserProc { child, stdin, stdout: BufReader::new(stdout) })
}

/// Send one command and read exactly one JSON response. Commands are serialized
/// by the `proc` mutex, so strict request/response is correct here.
async fn send_cmd(proc: &mut BrowserProc, action: &str, args: Value, timeout_s: u64) -> Result<Value, String> {
    let line = format!("{}\n", json!({ "action": action, "args": args }));
    proc.stdin.write_all(line.as_bytes()).await.map_err(|e| format!("sidecar write: {e}"))?;
    proc.stdin.flush().await.map_err(|e| format!("sidecar flush: {e}"))?;

    let read_fut = async {
        loop {
            let mut buf = String::new();
            let n = proc.stdout.read_line(&mut buf).await.map_err(|e| format!("sidecar read: {e}"))?;
            if n == 0 {
                return Err("sidecar closed (python/invisible_playwright missing or crashed)".to_string());
            }
            let t = buf.trim();
            if t.is_empty() { continue; }
            match serde_json::from_str::<Value>(t) {
                Ok(v) if v.get("ok").is_some() => return Ok(v),
                _ => continue, // skip any stray stdout noise
            }
        }
    };
    let v = tokio::time::timeout(std::time::Duration::from_secs(timeout_s), read_fut)
        .await
        .map_err(|_| format!("browser action '{action}' timed out after {timeout_s}s"))??;

    if v.get("ok").and_then(|b| b.as_bool()).unwrap_or(false) {
        Ok(v.get("result").cloned().unwrap_or_else(|| json!({})))
    } else {
        Err(v.get("error").and_then(|e| e.as_str()).unwrap_or("unknown sidecar error").to_string())
    }
}

pub struct BrowserState {
    /// Cached view of the current page (kept in sync so existing readers work).
    pub browser: tokio::sync::Mutex<Option<SendBrowser>>,
    /// The live stealth-browser sidecar process.
    pub proc: tokio::sync::Mutex<Option<BrowserProc>>,
}

impl BrowserState {
    pub fn new() -> Self {
        Self {
            browser: tokio::sync::Mutex::new(None),
            proc: tokio::sync::Mutex::new(None),
        }
    }

    /// Start the stealth browser if it isn't running yet. First launch downloads
    /// the patched Firefox, so the open timeout is generous.
    pub async fn ensure_started(&self) -> Result<(), String> {
        let mut guard = self.proc.lock().await;
        if guard.is_none() {
            let mut p = start_sidecar().await?;
            send_cmd(&mut p, "open", json!({}), 180).await?;
            *guard = Some(p);
        }
        Ok(())
    }

    /// Run a browser command against the live sidecar (auto-starts it).
    pub async fn cmd(&self, action: &str, args: Value, timeout_s: u64) -> Result<Value, String> {
        self.ensure_started().await?;
        let mut guard = self.proc.lock().await;
        let p = guard.as_mut().ok_or("browser sidecar missing")?;
        send_cmd(p, action, args, timeout_s).await
    }

    /// Refresh the cached session from the live page so DOM/text readers see
    /// the real rendered content.
    pub async fn refresh_cache(&self, url_hint: &str) {
        if let Ok(content) = self.cmd("content", json!({}), 30).await {
            let get = |k: &str| content.get(k).and_then(|v| v.as_str()).unwrap_or("").to_string();
            let url = {
                let u = get("url");
                if u.is_empty() { url_hint.to_string() } else { u }
            };
            *self.browser.lock().await = Some(SendBrowser(BrowserSession {
                url,
                html: get("html"),
                text: get("text"),
                title: get("title"),
            }));
        }
    }
}

fn strip_html(html: &str) -> String {
    let mut stripped = String::new();
    let mut in_tag = false;
    let mut in_script_or_style = false;
    let mut tag_buffer = String::new();
    
    let mut chars = html.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '<' {
            in_tag = true;
            tag_buffer.clear();
        } else if c == '>' && in_tag {
            in_tag = false;
            let tag_lower = tag_buffer.to_lowercase();
            if tag_lower.starts_with("script") {
                in_script_or_style = true;
            } else if tag_lower.starts_with("/script") {
                in_script_or_style = false;
            } else if tag_lower.starts_with("style") {
                in_script_or_style = true;
            } else if tag_lower.starts_with("/style") {
                in_script_or_style = false;
            }
            if tag_lower.starts_with("div") || tag_lower.starts_with("/div") || 
               tag_lower.starts_with("p") || tag_lower.starts_with("/p") || 
               tag_lower.starts_with("li") || tag_lower.starts_with("/li") ||
               tag_lower.starts_with("br") || tag_lower.starts_with("h") {
                stripped.push(' ');
            }
        } else if in_tag {
            tag_buffer.push(c);
        } else if !in_script_or_style {
            stripped.push(c);
        }
    }
    
    let mut clean = String::new();
    let mut last_was_space = false;
    for c in stripped.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                clean.push(' ');
                last_was_space = true;
            }
        } else {
            clean.push(c);
            last_was_space = false;
        }
    }
    clean.trim().to_string()
}

#[allow(dead_code)]
fn extract_title(html: &str) -> Option<String> {
    let title_start = html.to_lowercase().find("<title>")?;
    let title_end = html.to_lowercase().find("</title>")?;
    if title_end > title_start {
        Some(html[title_start + 7..title_end].trim().to_string())
    } else {
        None
    }
}

fn get_dom_summary(html: &str) -> String {
    let mut summary = Vec::new();
    
    if let Ok(re_a) = Regex::new(r#"<a\s+[^>]*?href=["']([^"']*)["'][^>]*>(.*?)</a>"#) {
        for cap in re_a.captures_iter(html) {
            let href = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let text = strip_html(cap.get(2).map(|m| m.as_str()).unwrap_or(""));
            summary.push(json!({
                "tag": "a",
                "text": text.chars().take(100).collect::<String>(),
                "id": "",
                "placeholder": href,
                "role": "link",
                "aria_label": ""
            }));
        }
    }
    
    if let Ok(re_btn) = Regex::new(r#"<button[^>]*>(.*?)</button>"#) {
        for cap in re_btn.captures_iter(html) {
            let text = strip_html(cap.get(1).map(|m| m.as_str()).unwrap_or(""));
            summary.push(json!({
                "tag": "button",
                "text": text.chars().take(100).collect::<String>(),
                "id": "",
                "placeholder": "",
                "role": "button",
                "aria_label": ""
            }));
        }
    }

    if let Ok(re_input) = Regex::new(r#"<input\s+([^>]*?)>"#) {
        for cap in re_input.captures_iter(html) {
            let attrs = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            let placeholder = if attrs.contains("placeholder=") {
                attrs.split("placeholder=")
                    .nth(1)
                    .unwrap_or("")
                    .trim_matches(|c| c == '"' || c == '\'' || c == ' ')
                    .split(' ')
                    .next()
                    .unwrap_or("")
                    .to_string()
            } else {
                "".to_string()
            };
            summary.push(json!({
                "tag": "input",
                "text": "",
                "id": "",
                "placeholder": placeholder,
                "role": "textbox",
                "aria_label": ""
            }));
        }
    }

    serde_json::to_string(&summary).unwrap_or_else(|_| "[]".to_string())
}

fn get_content_summary_internal(html: &str, text: &str) -> Value {
    let mut links = Vec::new();
    if let Ok(re_a) = Regex::new(r#"<a\s+[^>]*?href=["']([^"']*)["'][^>]*>(.*?)</a>"#) {
        for cap in re_a.captures_iter(html) {
            let href = cap.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
            let text_val = strip_html(cap.get(2).map(|m| m.as_str()).unwrap_or(""));
            if text_val.len() > 5 && href.starts_with("http") {
                links.push(json!({
                    "text": text_val,
                    "href": href
                }));
                if links.len() >= 15 {
                    break;
                }
            }
        }
    }

    let mut headers = Vec::new();
    if let Ok(re_h) = Regex::new(r#"<h[1-3][^>]*>(.*?)</h[1-3]>"#) {
        for cap in re_h.captures_iter(html) {
            let val = strip_html(cap.get(1).map(|m| m.as_str()).unwrap_or(""));
            if !val.is_empty() {
                headers.push(val);
            }
        }
    }

    json!({
        "text": text.chars().take(5000).collect::<String>(),
        "links": links,
        "headers": headers
    })
}

/// Quick security-header audit used to enrich navigate results for pentest.
fn missing_security_headers(headers: &Value) -> Vec<String> {
    const SEC: &[&str] = &[
        "content-security-policy",
        "strict-transport-security",
        "x-frame-options",
        "x-content-type-options",
        "referrer-policy",
        "permissions-policy",
    ];
    SEC.iter()
        .filter(|h| headers.get(**h).is_none())
        .map(|h| h.to_string())
        .collect()
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_open(app: tauri::AppHandle) -> Result<String, String> {
    // FIRE-AND-FORGET. Launching the stealth Firefox — and DOWNLOADING it on first
    // run — can take minutes. Awaiting it here blocked the UI's invoke and looked
    // like the IDE force-quit/hung. Spawn it in the background so the UI returns
    // instantly; the Firefox window appears when ready. Failures are logged, never
    // fatal to the app.
    use tauri::Manager;
    tauri::async_runtime::spawn(async move {
        let state = app.state::<BrowserState>();
        if let Err(e) = state.ensure_started().await {
            eprintln!("[browser] open failed (install: pip install playwright invisible_playwright): {e}");
        }
    });
    Ok("Browser launching… a stealth-Firefox window will open shortly (first run downloads it).".to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_navigate(state: tauri::State<'_, BrowserState>, url: String) -> Result<String, String> {
    let r = state.cmd("navigate", json!({ "url": url }), 60).await?;
    state.refresh_cache(&url).await;
    let status = r.get("status").map(|s| s.to_string()).unwrap_or_else(|| "?".into());
    let headers = r.get("headers").cloned().unwrap_or_else(|| json!({}));
    let missing = missing_security_headers(&headers);
    let missing_str = if missing.is_empty() { "none".to_string() } else { missing.join(", ") };
    Ok(format!("Navigated to {url} (HTTP {status}). Missing security headers: {missing_str}"))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_screenshot(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let r = state.cmd("screenshot", json!({}), 30).await?;
    Ok(r.get("screenshot").and_then(|v| v.as_str()).unwrap_or("").to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_click(state: tauri::State<'_, BrowserState>, selector: String) -> Result<String, String> {
    state.cmd("click", json!({ "selector": selector }), 20).await?;
    state.refresh_cache("").await;
    Ok(format!("Clicked {}", selector))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_type(state: tauri::State<'_, BrowserState>, selector: String, text: String) -> Result<String, String> {
    state.cmd("fill", json!({ "selector": selector, "text": text }), 20).await?;
    Ok(format!("Typed into {}", selector))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_read_dom(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let r = state.cmd("content", json!({}), 30).await?;
    Ok(r.get("html").and_then(|v| v.as_str()).unwrap_or("").to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_capture_vision_context(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    capture_vision_context_internal(&state).await
}

pub async fn capture_vision_context_internal(state: &BrowserState) -> Result<Value, String> {
    let content = state.cmd("content", json!({}), 30).await?;
    let shot = state.cmd("screenshot", json!({}), 30).await.unwrap_or_else(|_| json!({}));
    let html = content.get("html").and_then(|v| v.as_str()).unwrap_or("");

    Ok(json!({
        "url": content.get("url").cloned().unwrap_or_else(|| json!("")),
        "title": content.get("title").cloned().unwrap_or_else(|| json!("")),
        "screenshot": shot.get("screenshot").cloned().unwrap_or_else(|| json!("")),
        "dom_summary": get_dom_summary(html)
    }))
}

#[tauri::command]
pub async fn browser_get_content_summary(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    let content = state.cmd("content", json!({}), 30).await?;
    let html = content.get("html").and_then(|v| v.as_str()).unwrap_or("");
    let text = content.get("text").and_then(|v| v.as_str()).unwrap_or("");
    Ok(get_content_summary_internal(html, text))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_close(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let mut guard = state.proc.lock().await;
    if let Some(mut p) = guard.take() {
        let _ = send_cmd(&mut p, "close", json!({}), 10).await;
        let _ = p.child.kill().await;
    }
    *state.browser.lock().await = None;
    Ok("Browser closed".to_string())
}

