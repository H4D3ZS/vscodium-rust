use serde_json::{Value, json};
use regex::Regex;

pub struct BrowserSession {
    pub url: String,
    pub html: String,
    pub text: String,
    pub title: String,
}

pub struct SendBrowser(pub BrowserSession);
unsafe impl Send for SendBrowser {}
unsafe impl Sync for SendBrowser {}

pub struct BrowserState {
    pub browser: tokio::sync::Mutex<Option<SendBrowser>>,
}

impl BrowserState {
    pub fn new() -> Self {
        Self {
            browser: tokio::sync::Mutex::new(None),
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

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_open(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let mut browser_lock = state.browser.lock().await;
    if browser_lock.is_some() {
        return Ok("Browser already open".to_string());
    }

    *browser_lock = Some(SendBrowser(BrowserSession {
        url: "about:blank".to_string(),
        html: "<html><body></body></html>".to_string(),
        text: "".to_string(),
        title: "".to_string(),
    }));

    Ok("Browser launched successfully".to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_navigate(state: tauri::State<'_, BrowserState>, url: String) -> Result<String, String> {
    let mut browser_lock = state.browser.lock().await;
    let browser_wrapper = browser_lock.as_mut().ok_or("Browser not launched")?;
    let session = &mut browser_wrapper.0;

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| format!("Failed to build HTTP client: {}", e))?;

    let response = client.get(&url)
        .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch URL: {}", e))?;

    let html = response.text().await.map_err(|e| format!("Failed to read response body: {}", e))?;
    
    session.text = strip_html(&html);
    session.title = extract_title(&html).unwrap_or_else(|| url.clone());
    session.url = url.clone();
    session.html = html;

    Ok(format!("Navigated to {}", url))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_screenshot(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let browser_lock = state.browser.lock().await;
    let _browser_wrapper = browser_lock.as_ref().ok_or("Browser not launched")?;
    Ok("/9j/4AAQSkZJRgABAQEASABIAAD/2wBDAP//////////////////////////////////////////////////////////////////////////////////////wgALCAABAAEBAREA/8QAFBABAAAAAAAAAAAAAAAAAAAAAP/aAAgBAQABPxA=".to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_click(_state: tauri::State<'_, BrowserState>, selector: String) -> Result<String, String> {
    Ok(format!("Clicked element (lightweight mock): {}", selector))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_type(_state: tauri::State<'_, BrowserState>, selector: String, text: String) -> Result<String, String> {
    Ok(format!("Typed into (lightweight mock) {}: {}", selector, text))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_read_dom(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let browser_lock = state.browser.lock().await;
    let browser_wrapper = browser_lock.as_ref().ok_or("Browser not launched")?;
    let session = &browser_wrapper.0;

    Ok(session.html.clone())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_capture_vision_context(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    capture_vision_context_internal(&state).await
}

pub async fn capture_vision_context_internal(state: &BrowserState) -> Result<Value, String> {
    let browser_lock = state.browser.lock().await;
    let browser_wrapper = browser_lock.as_ref().ok_or("Browser not launched")?;
    let session = &browser_wrapper.0;
    
    let screenshot_b64 = "/9j/4AAQSkZJRgABAQEASABIAAD/2wBDAP//////////////////////////////////////////////////////////////////////////////////////wgALCAABAAEBAREA/8QAFBABAAAAAAAAAAAAAAAAAAAAAP/aAAgBAQABPxA=".to_string();
    let dom_summary = get_dom_summary(&session.html);

    Ok(json!({
        "url": session.url,
        "title": session.title,
        "screenshot": screenshot_b64,
        "dom_summary": dom_summary
    }))
}

#[tauri::command]
pub async fn browser_get_content_summary(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    let browser_lock = state.browser.lock().await;
    let browser_wrapper = browser_lock.as_ref().ok_or("Browser not launched")?;
    let session = &browser_wrapper.0;

    Ok(get_content_summary_internal(&session.html, &session.text))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_close(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let mut browser_lock = state.browser.lock().await;
    *browser_lock = None;
    Ok("Browser closed".to_string())
}

