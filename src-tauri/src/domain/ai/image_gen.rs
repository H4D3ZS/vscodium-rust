use base64::{engine::general_purpose::STANDARD as B64, Engine as _};
use reqwest::Client;
use serde_json::{json, Value};
use std::path::Path;

const DEFAULT_OLLAMA: &str = "http://127.0.0.1:11434";
const DEFAULT_VISION_MODEL: &str = "llava";
const DEFAULT_IMAGE_MODEL: &str = "x/flux2-klein";

fn ollama_base() -> String {
    std::env::var("OLLAMA_HOST")
        .or_else(|_| std::env::var("OLLAMA_BASE_URL"))
        .unwrap_or_else(|_| DEFAULT_OLLAMA.to_string())
}

fn vision_model() -> String {
    std::env::var("OLLAMA_VISION_MODEL").unwrap_or_else(|_| DEFAULT_VISION_MODEL.to_string())
}

fn image_gen_model() -> String {
    std::env::var("OLLAMA_IMAGE_MODEL").unwrap_or_else(|_| DEFAULT_IMAGE_MODEL.to_string())
}

/// Analyze an image with Ollama vision model (llava, moondream, etc.).
pub async fn analyze_with_ollama(image_path: &Path, question: &str) -> Result<String, String> {
    let bytes = std::fs::read(image_path).map_err(|e| e.to_string())?;
    let b64 = B64.encode(&bytes);
    let client = Client::new();
    let url = format!("{}/api/chat", ollama_base().trim_end_matches('/'));
    let body = json!({
        "model": vision_model(),
        "stream": false,
        "messages": [{
            "role": "user",
            "content": question,
            "images": [b64]
        }]
    });
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama vision request failed: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("Ollama vision HTTP {}", resp.status()));
    }
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    Ok(v["message"]["content"]
        .as_str()
        .or_else(|| v["response"].as_str())
        .unwrap_or("(no response)")
        .to_string())
}

/// Analyze image with Gemini multimodal.
pub async fn analyze_with_gemini(api_key: &str, image_path: &Path, question: &str) -> Result<String, String> {
    let bytes = std::fs::read(image_path).map_err(|e| e.to_string())?;
    let b64 = B64.encode(&bytes);
    let ext = image_path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("png")
        .to_lowercase();
    let mime = match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "gif" => "image/gif",
        _ => "image/png",
    };
    let model = std::env::var("GEMINI_VISION_MODEL")
        .unwrap_or_else(|_| "gemini-2.0-flash".to_string());
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );
    let body = json!({
        "contents": [{
            "parts": [
                { "text": question },
                { "inline_data": { "mime_type": mime, "data": b64 } }
            ]
        }]
    });
    let client = Client::new();
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let err = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini vision error: {err}"));
    }
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    let text = v["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("(no response)");
    Ok(text.to_string())
}

/// Generate image via Ollama image-capable model; writes PNG to dest_path.
pub async fn generate_with_ollama(prompt: &str, dest_path: &Path) -> Result<String, String> {
    let client = Client::new();
    let url = format!("{}/api/generate", ollama_base().trim_end_matches('/'));
    let body = json!({
        "model": image_gen_model(),
        "prompt": prompt,
        "stream": false
    });
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("Ollama image gen failed: {e}"))?;
    if !resp.status().is_success() {
        let err = resp.text().await.unwrap_or_default();
        return Err(format!(
            "Ollama image model '{}' unavailable: {err}. Pull with: ollama pull {}",
            image_gen_model(),
            image_gen_model()
        ));
    }
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    if let Some(img_b64) = v["image"].as_str().or_else(|| v["images"].as_array()?.first()?.as_str()) {
        let bytes = B64.decode(img_b64).map_err(|e| e.to_string())?;
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        std::fs::write(dest_path, &bytes).map_err(|e| e.to_string())?;
        return Ok(dest_path.to_string_lossy().to_string());
    }
    Err("Ollama response contained no image data — use a model that supports image output (e.g. x/flux2-klein)".into())
}

/// Generate image via Gemini native image generation.
pub async fn generate_with_gemini(api_key: &str, prompt: &str, dest_path: &Path) -> Result<String, String> {
    let model = std::env::var("GEMINI_IMAGE_MODEL")
        .unwrap_or_else(|_| "gemini-2.0-flash-preview-image-generation".to_string());
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, api_key
    );
    let body = json!({
        "contents": [{ "parts": [{ "text": prompt }] }],
        "generationConfig": { "responseModalities": ["TEXT", "IMAGE"] }
    });
    let client = Client::new();
    let resp = client
        .post(&url)
        .json(&body)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    if !resp.status().is_success() {
        let err = resp.text().await.unwrap_or_default();
        return Err(format!("Gemini image gen error: {err}"));
    }
    let v: Value = resp.json().await.map_err(|e| e.to_string())?;
    let parts = v["candidates"][0]["content"]["parts"]
        .as_array()
        .ok_or("Gemini returned no parts")?;
    for part in parts {
        if let Some(data) = part["inlineData"]["data"].as_str() {
            let bytes = B64.decode(data).map_err(|e| e.to_string())?;
            if let Some(parent) = dest_path.parent() {
                std::fs::create_dir_all(parent).ok();
            }
            std::fs::write(dest_path, &bytes).map_err(|e| e.to_string())?;
            return Ok(dest_path.to_string_lossy().to_string());
        }
    }
    Err("Gemini response contained no image".into())
}
