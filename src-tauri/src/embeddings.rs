//! Ollama text embeddings — shared by vector indexer, semantic search, AIRI memory.

use serde_json::json;

pub fn default_embed_model() -> &'static str {
    "nomic-embed-text"
}

pub async fn embed_text_async(text: &str, model: Option<&str>) -> Result<Vec<f32>, String> {
    let model = model.unwrap_or(default_embed_model());
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;
    let trimmed: String = text.chars().take(4000).collect();
    let res = client
        .post("http://127.0.0.1:11434/api/embeddings")
        .json(&json!({ "model": model, "prompt": trimmed }))
        .send()
        .await
        .map_err(|e| format!("Ollama embeddings failed: {e}. Run: ollama pull {model}"))?;
    if !res.status().is_success() {
        return Err(format!("Ollama returned {} for model {model}", res.status()));
    }
    let body: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
    parse_embedding_array(&body)
}

/// Blocking embed for sync indexing loops.
pub fn embed_text_blocking(text: &str, model: Option<&str>) -> Result<Vec<f32>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    rt.block_on(embed_text_async(text, model))
}

pub fn parse_embedding_array(body: &serde_json::Value) -> Result<Vec<f32>, String> {
    body.get("embedding")
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect())
        .ok_or_else(|| "No embedding in response".to_string())
}

pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    if a.len() != b.len() || a.is_empty() {
        return 0.0;
    }
    let mut dot = 0.0f32;
    let mut na = 0.0f32;
    let mut nb = 0.0f32;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    if na <= 0.0 || nb <= 0.0 {
        return 0.0;
    }
    dot / (na.sqrt() * nb.sqrt())
}
