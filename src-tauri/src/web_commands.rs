

use serde_json::{json, Value};
use reqwest::Client;

#[tauri::command]
pub async fn web_fetch(url: String) -> Result<String, String> {
    let client = Client::new();
    let res = client.get(url).send().await.map_err(|e| e.to_string())?;
    res.text().await.map_err(|e| e.to_string())
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
