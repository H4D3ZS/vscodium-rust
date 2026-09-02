

use serde_json::{json, Value};
use reqwest::Client;

#[tauri::command]
pub async fn web_fetch(url: String) -> Result<String, String> {
    let client = Client::new();
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    res.text().await.map_err(|e| e.to_string())
}

/// Generate a real text embedding via Ollama's /api/embeddings endpoint.
/// Used by the AIRI memory system for genuine semantic similarity instead of
/// the previous hash-based placeholder vector. Defaults to `nomic-embed-text`
/// (a small, fast embedding model). Returns the embedding vector.
#[tauri::command]
pub async fn embed_text(text: String, model: Option<String>) -> Result<Vec<f32>, String> {
    // Delegates to the shared embedding client rather than calling Ollama
    // directly. This used to POST to :11434 with Ollama's payload shape, which
    // on a Lemonade-only machine failed every call — taking AIRI's semantic
    // memory down with it, silently, since callers treat an error as "no match".
    crate::embeddings::embed_text_async(&text, model.as_deref()).await
}

/// Real HTTP probe for security testing: returns status, response headers,
/// body, and round-trip time. Used by the offensive-security module to do
/// genuine header/XSS/SQLi-timing/IDOR checks instead of simulated ones.
#[tauri::command]
pub async fn http_probe(url: String, method: Option<String>) -> Result<Value, String> {
    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .timeout(std::time::Duration::from_secs(20))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())?;

    let m = method.unwrap_or_else(|| "GET".to_string()).to_uppercase();
    let req = match m.as_str() {
        "HEAD" => client.head(&url),
        "POST" => client.post(&url),
        _ => client.get(&url),
    };

    let started = std::time::Instant::now();
    let res = req.send().await.map_err(|e| e.to_string())?;
    let status = res.status().as_u16();

    let mut headers = serde_json::Map::new();
    for (k, v) in res.headers().iter() {
        headers.insert(k.as_str().to_lowercase(), json!(v.to_str().unwrap_or("")));
    }
    // Cap body so a huge response can't blow memory.
    let body = res.text().await.unwrap_or_default();
    let elapsed_ms = started.elapsed().as_millis() as u64;
    let capped: String = body.chars().take(200_000).collect();

    Ok(json!({
        "status": status,
        "headers": Value::Object(headers),
        "body": capped,
        "elapsed_ms": elapsed_ms,
    }))
}

#[tauri::command]
pub async fn web_search(query: String, num_results: Option<usize>) -> Result<Value, String> {
    let limit = num_results.unwrap_or(6).min(10);
    let encoded = urlencoding::encode(&query);
    let ddg_url = format!(
        "https://api.duckduckgo.com/?q={}&format=json&no_html=1&skip_disambig=1",
        encoded
    );

    let client = Client::new();
    let res = client.get(ddg_url).send().await.map_err(|e| e.to_string())?;
    let json: Value = res.json().await.map_err(|e| e.to_string())?;
    
    // Process results to a standard format
    let mut results = Vec::new();
    if let Some(abs) = json.get("AbstractText").and_then(|v| v.as_str()) {
        if !abs.is_empty() {
             results.push(json!({
                 "title": json.get("Heading").and_then(|v| v.as_str()).unwrap_or("Result"),
                 "snippet": abs,
                 "url": json.get("AbstractURL").and_then(|v| v.as_str()).unwrap_or("")
             }));
        }
    }
    
    if let Some(related) = json.get("RelatedTopics").and_then(|v| v.as_array()) {
        for item in related.iter().take(limit) {
            if let Some(text) = item.get("Text").and_then(|v| v.as_str()) {
                results.push(json!({
                    "title": text.chars().take(50).collect::<String>(),
                    "snippet": text,
                    "url": item.get("FirstURL").and_then(|v| v.as_str()).unwrap_or("")
                }));
            }
        }
    }

    Ok(json!(results))
}
