use headless_chrome::{Browser, LaunchOptions};
use std::sync::Mutex;
use base64::{Engine as _, engine::general_purpose};
use serde_json::{Value, json};

pub struct BrowserState {
    pub browser: Mutex<Option<Browser>>,
}

impl BrowserState {
    pub fn new() -> Self {
        Self {
            browser: Mutex::new(None),
        }
    }
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_open(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let mut browser_lock = state.browser.lock().unwrap();
    if browser_lock.is_some() {
        return Ok("Browser already open".to_string());
    }

    let mut builder = LaunchOptions::default_builder();
    builder.headless(true);

    // Hardening for Windows: specifically search for common paths if default fails
    if cfg!(target_os = "windows") {
        let common_paths = [
            "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files (x86)\\Google\\Chrome\\Application\\chrome.exe",
            "C:\\Program Files\\Microsoft\\Edge\\Application\\msedge.exe",
        ];
        for path in common_paths {
            if std::path::Path::new(path).exists() {
                builder.path(Some(path.into()));
                break;
            }
        }
    }

    let options = builder.build().map_err(|e| e.to_string())?;
    let browser = Browser::new(options).map_err(|e| e.to_string())?;
    *browser_lock = Some(browser);

    Ok("Browser launched successfully".to_string())
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_navigate(state: tauri::State<'_, BrowserState>, url: String) -> Result<String, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    // Optimization: Reuse the first available tab if it exists to avoid new process overhead
    let tab = if let Some(existing_tab) = browser.get_tabs().lock().unwrap().first() {
        existing_tab.clone()
    } else {
        browser.new_tab().map_err(|e| e.to_string())?
    };

    tab.navigate_to(&url).map_err(|e| e.to_string())?;
    tab.wait_until_navigated().map_err(|e| e.to_string())?;

    Ok(format!("Navigated to {}", url))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_screenshot(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();
    let jpeg_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Jpeg,
        None,
        None,
        true
    ).map_err(|e| e.to_string())?;

    Ok(general_purpose::STANDARD.encode(jpeg_data))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_click(state: tauri::State<'_, BrowserState>, selector: String) -> Result<String, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();
    let element = tab.wait_for_element(&selector).map_err(|e| e.to_string())?;
    element.click().map_err(|e| e.to_string())?;

    Ok(format!("Clicked element: {}", selector))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_type(state: tauri::State<'_, BrowserState>, selector: String, text: String) -> Result<String, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();
    let element = tab.wait_for_element(&selector).map_err(|e| e.to_string())?;
    element.type_into(&text).map_err(|e| e.to_string())?;

    Ok(format!("Typed into {}: {}", selector, text))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_read_dom(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();
    let content = tab.get_content().map_err(|e| e.to_string())?;

    Ok(content)
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_capture_vision_context(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    capture_vision_context_internal(&state).await
}

pub async fn capture_vision_context_internal(state: &BrowserState) -> Result<Value, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;

    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();
    
    let url = tab.get_url();
    let title = tab.get_title().unwrap_or_default();
    
    // Capture screenshot as base64
    let jpeg_data = tab.capture_screenshot(
        headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Jpeg,
        Some(80), // Quality
        None,
        true
    ).map_err(|e| e.to_string())?;
    let screenshot_b64 = general_purpose::STANDARD.encode(jpeg_data);

    // Get simplified DOM (focused on interactive elements and text)
    let dom_script = r#"
        (function() {
            const result = [];
            const elements = document.querySelectorAll('button, a, input, select, textarea, h1, h2, h3, [role="button"]');
            elements.forEach(el => {
                const rect = el.getBoundingClientRect();
                if (rect.width > 0 && rect.height > 0) {
                    result.push({
                        tag: el.tagName.toLowerCase(),
                        text: el.innerText.substring(0, 100).trim(),
                        id: el.id,
                        placeholder: el.placeholder,
                        role: el.getAttribute('role'),
                        aria_label: el.getAttribute('aria-label')
                    });
                }
            });
            return JSON.stringify(result);
        })()
    "#;
    
    let dom_summary = tab.evaluate(dom_script, false)
        .map_err(|e| e.to_string())?
        .value.ok_or("Failed to get DOM summary")?;

    Ok(json!({
        "url": url,
        "title": title,
        "screenshot": screenshot_b64,
        "dom_summary": dom_summary
    }))
}

#[tauri::command]
pub async fn browser_get_content_summary(state: tauri::State<'_, BrowserState>) -> Result<Value, String> {
    let browser_lock = state.browser.lock().unwrap();
    let browser = browser_lock.as_ref().ok_or("Browser not launched")?;
    let tab = browser.get_tabs().lock().unwrap().first().ok_or("No tabs open")?.clone();

    let extraction_script = r#"
        (function() {
            const bodyText = document.body.innerText.substring(0, 5000);
            const links = Array.from(document.querySelectorAll('a')).map(a => ({
                text: a.innerText.trim(),
                href: a.href
            })).filter(l => l.text.length > 5 && l.href.startsWith('http')).slice(0, 15);
            
            const headers = Array.from(document.querySelectorAll('h1, h2, h3')).map(h => h.innerText.trim()).filter(t => t.length > 0);
            
            return JSON.stringify({
                text: bodyText,
                links: links,
                headers: headers
            });
        })()
    "#;

    let result = tab.evaluate(extraction_script, false)
        .map_err(|e| e.to_string())?
        .value.ok_or("Failed to get content summary")?;

    Ok(serde_json::from_str(result.as_str().unwrap_or("{}")).unwrap_or(json!({})))
}

#[tauri::command]
#[allow(dead_code)]
pub async fn browser_close(state: tauri::State<'_, BrowserState>) -> Result<String, String> {
    let mut browser_lock = state.browser.lock().unwrap();
    *browser_lock = None;
    Ok("Browser closed".to_string())
}
