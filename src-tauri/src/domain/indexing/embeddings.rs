//! Text embeddings — shared by the vector indexer, semantic search, AIRI memory.
//!
//! Served by **Lemonade**. This machine runs Lemonade on :13305 and has
//! no raw server; pointing at :11434 meant every embed call failed, which silently took
//! `@codebase`, `semantic_search` and `search_codebase` with it. Those features
//! reported "done" while returning nothing, because a failed embed degrades to an
//! empty result rather than an error the user ever sees.
//!
//! Lemonade exposes the OpenAI shape: `POST /api/v1/embeddings` with `{model, input}`
//! returning `{data: [{embedding: [...]}]}`. the local backend historically used `/api/embeddings` with
//! `{model, prompt}` returning `{embedding: [...]}` — both response shapes are still
//! parsed, so an the local backend backend keeps working if someone points `base_url` at one.

use serde_json::json;

/// Lemonade's default endpoint. Overridable with `LEMONADE_URL` so a non-default
/// port does not silently fall back to a dead host.
pub fn default_embed_base_url() -> String {
    std::env::var("LEMONADE_URL").unwrap_or_else(|_| "http://localhost:13305".to_string())
}

/// Qwen3-Embedding-0.6B: 1024 dimensions, 609MB, strong on code retrieval.
///
/// **Changing this invalidates every stored vector.** Dimensions differ between
/// models (the old `nomic-embed-text` default was 768, this is 1024) and
/// [`cosine_similarity`] returns 0.0 for a length mismatch — so a stale index does
/// not error, it just silently ranks everything as unrelated. Re-index after any
/// change here.
pub fn default_embed_model() -> &'static str {
    "Qwen3-Embedding-0.6B-GGUF"
}

/// Dimensions produced by [`default_embed_model`]. Used to detect a stale index
/// built with a different model.
pub const DEFAULT_EMBED_DIMS: usize = 1024;

pub async fn embed_text_async(text: &str, model: Option<&str>) -> Result<Vec<f32>, String> {
    embed_text_at(text, model, &default_embed_base_url()).await
}

pub async fn embed_text_at(
    text: &str,
    model: Option<&str>,
    base_url: &str,
) -> Result<Vec<f32>, String> {
    let model = model.unwrap_or(default_embed_model());
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(60))
        .build()
        .map_err(|e| e.to_string())?;
    let trimmed: String = text.chars().take(4000).collect();

    // An the local backend base URL keeps its own endpoint and payload shape, so an existing
    // the local backend install still works if configured explicitly.
    let is_native_api = base_url.contains("11434");
    let (url, payload) = if is_native_api {
        (
            format!("{}/api/embeddings", base_url.trim_end_matches('/')),
            json!({ "model": model, "prompt": trimmed }),
        )
    } else {
        (
            format!("{}/api/v1/embeddings", base_url.trim_end_matches('/')),
            json!({ "model": model, "input": trimmed }),
        )
    };

    let res = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .map_err(|e| {
            format!(
                "Embeddings request to {url} failed: {e}. \
                 Is Lemonade running? Install the model with: lemonade pull {model}"
            )
        })?;

    let status = res.status();
    if !status.is_success() {
        let body = res.text().await.unwrap_or_default();
        return Err(format!(
            "Embeddings backend returned {status} for model {model}: {}",
            body.chars().take(200).collect::<String>()
        ));
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

/// Blocking embed at a specific backend URL.
pub fn embed_text_blocking_at(
    text: &str,
    model: Option<&str>,
    base_url: &str,
) -> Result<Vec<f32>, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| e.to_string())?;
    rt.block_on(embed_text_at(text, model, base_url))
}

/// Extract the vector from either backend's response shape.
///
/// Lemonade/OpenAI: `{"data": [{"embedding": [...]}]}`
/// the local backend:          `{"embedding": [...]}`
pub fn parse_embedding_array(body: &serde_json::Value) -> Result<Vec<f32>, String> {
    let arr = body
        .get("data")
        .and_then(|d| d.as_array())
        .and_then(|a| a.first())
        .and_then(|e| e.get("embedding"))
        .and_then(|v| v.as_array())
        .or_else(|| body.get("embedding").and_then(|v| v.as_array()));

    let Some(arr) = arr else {
        return Err(format!(
            "No embedding in response (keys: {:?})",
            body.as_object().map(|o| o.keys().collect::<Vec<_>>())
        ));
    };
    if arr.is_empty() {
        return Err("Embedding response contained an empty vector".to_string());
    }
    Ok(arr
        .iter()
        .filter_map(|v| v.as_f64().map(|f| f as f32))
        .collect())
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

#[cfg(test)]
mod embeddings_tests {
    use super::*;

    /// Lemonade's OpenAI-shaped response is the default path now.
    #[test]
    fn parses_the_openai_data_array_shape() {
        let body = json!({"data": [{"embedding": [0.1, 0.2, 0.3]}], "model": "x"});
        assert_eq!(parse_embedding_array(&body).unwrap(), vec![0.1f32, 0.2, 0.3]);
    }

    /// The old the local backend shape must still parse, so pointing at an the local backend backend
    /// keeps working.
    #[test]
    fn still_parses_the_native_flat_shape() {
        let body = json!({"embedding": [0.4, 0.5]});
        assert_eq!(parse_embedding_array(&body).unwrap(), vec![0.4f32, 0.5]);
    }

    /// An empty vector must be an error, not a silently-useless all-zero result:
    /// `cosine_similarity` scores mismatched lengths as 0.0, so a bad embed would
    /// otherwise look like "nothing in the codebase is relevant".
    #[test]
    fn empty_and_missing_embeddings_are_errors() {
        assert!(parse_embedding_array(&json!({"data": [{"embedding": []}]})).is_err());
        assert!(parse_embedding_array(&json!({"error": "nope"})).is_err());
    }

    /// The default must not point at the local backend. This machine has no the local backend, and the
    /// failure mode is silent — semantic search returns nothing rather than erroring.
    #[test]
    fn default_backend_is_lemonade_not_native() {
        let url = default_embed_base_url();
        assert!(!url.contains("11434"), "default must not be a raw :11434 server, got {url}");
        assert!(url.contains("13305"), "expected the Lemonade port, got {url}");
    }

    /// A dimension mismatch scores 0.0 rather than erroring, which is why changing
    /// the embedding model without re-indexing presents as "search finds nothing".
    #[test]
    fn mismatched_dimensions_score_zero_not_panic() {
        assert_eq!(cosine_similarity(&[1.0, 0.0], &[1.0, 0.0, 0.0]), 0.0);
        assert!((cosine_similarity(&[1.0, 0.0], &[1.0, 0.0]) - 1.0).abs() < 1e-6);
    }
}
