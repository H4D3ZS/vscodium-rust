//! Auto vision sidecar — when the agent model is text-only but the user attached
//! images, discover a local VL model (qwen2.5-vl, moondream, …) and inject a text
//! description (Cursor-style routing without swapping the main agent model).

use serde::{Deserialize, Serialize};
use tauri::State;

use crate::EditorState;

/// Priority-ordered substrings for picking the best installed vision model.
const VISION_MODEL_PRIORITY: &[(&str, i32)] = &[
    ("qwen2.5-vl", 100),
    ("qwen2.5vl", 100),
    ("qwen3-vl", 95),
    ("qwen-vl", 90),
    ("qwen2-vl", 88),
    ("llava", 72),
    ("bakllava", 72),
    ("minicpm-v", 68),
    ("moondream", 65),
    ("gemma-4", 55),
    ("gemma4", 55),
    ("gemma3", 50),
    ("phi", 45),
    ("nemotron-vl", 80),
    ("nemotron3-vl", 80),
];

pub fn is_vision_capable_model(model: &str) -> bool {
    let m = model.to_lowercase();
    if m.contains("gemma4") || m.contains("gemma-4") {
        return true;
    }
    if (m.contains("gemma3") || m.contains("gemma-3")) && m.contains("vision") {
        return true;
    }
    if m.contains("llava") || m.contains("bakllava") || m.contains("moondream") {
        return true;
    }
    if m.contains("minicpm-v") || m.contains("mimo-vl") {
        return true;
    }
    if (m.contains("qwen") && m.contains("-vl")) || m.contains("qwen-vl") || m.contains("qwen2.5vl") {
        return true;
    }
    if m.contains("phi") && m.contains("vision") {
        return true;
    }
    if m.contains("nemotron") && m.contains("vl") {
        return true;
    }
    false
}

fn vision_match_score(name: &str) -> i32 {
    let n = name.to_lowercase();
    if (n.contains("gemma3") || n.contains("gemma-3")) && !n.contains("vision") && !n.contains("vl") {
        return 0;
    }
    VISION_MODEL_PRIORITY
        .iter()
        .filter(|(pat, _)| n.contains(pat))
        .map(|(_, score)| *score)
        .max()
        .unwrap_or(0)
}

pub async fn discover_vision_models(ollama_base: &str, bearer: &str) -> Vec<String> {
    let base = normalize_ollama_base(ollama_base);
    let url = format!("{}/api/tags", base.trim_end_matches('/'));
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(8))
        .build()
        .unwrap_or_else(|_| reqwest::Client::new());
    let mut req = client.get(&url);
    if !bearer.trim().is_empty() {
        req = req.bearer_auth(bearer.trim());
    }
    let Ok(resp) = req.send().await else {
        return Vec::new();
    };
    let Ok(json) = resp.json::<serde_json::Value>().await else {
        return Vec::new();
    };
    let Some(models) = json.get("models").and_then(|m| m.as_array()) else {
        return Vec::new();
    };
    let mut scored: Vec<(String, i32)> = models
        .iter()
        .filter_map(|m| m.get("name").and_then(|n| n.as_str()))
        .filter_map(|name| {
            let score = vision_match_score(name);
            if score > 0 {
                Some((name.to_string(), score))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    scored.into_iter().map(|(n, _)| n).collect()
}

pub async fn discover_best_vision_model(ollama_base: &str, bearer: &str) -> Option<String> {
    discover_vision_models(ollama_base, bearer)
        .await
        .into_iter()
        .next()
}

fn normalize_ollama_base(url: &str) -> String {
    url.trim()
        .trim_end_matches('/')
        .trim_end_matches("/v1/chat/completions")
        .trim_end_matches("/chat/completions")
        .trim_end_matches("/v1")
        .trim_end_matches("/api/chat")
        .to_string()
}

fn ollama_bearer_for(state: &EditorState, ollama_base: &str) -> String {
    if let Ok(keys) = std::fs::read_to_string(state.config_dir.join("api_keys.json")) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&keys) {
            if let Some(k) = v.get("ollama").and_then(|x| x.as_str()) {
                if !k.trim().is_empty() {
                    return k.trim().to_string();
                }
            }
        }
    }
    let base = ollama_base.to_lowercase();
    if base.contains("cyberifrit.xyz") {
        if let Some(s) = crate::auth::load_session(&state.config_dir) {
            if !s.access_token.is_empty() {
                return s.access_token;
            }
        }
    }
    String::new()
}

pub async fn describe_image_b64(
    ollama_base: &str,
    bearer: &str,
    model: &str,
    b64: &str,
    user_prompt: Option<&str>,
) -> Result<String, String> {
    let base = normalize_ollama_base(ollama_base);
    let ctx = user_prompt
        .map(|p| p.trim())
        .filter(|p| !p.is_empty())
        .unwrap_or("the user's request");
    let prompt = format!(
        "Describe this image in detail for a software developer / security researcher. \
         Focus on UI elements, text, URLs, forms, errors, code, and security-relevant details. \
         User context: {ctx}. Be concise but complete."
    );
    let payload = serde_json::json!({
        "model": model,
        "prompt": prompt,
        "images": [b64],
        "stream": false,
        "keep_alive": 0
    });
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(120))
        .build()
        .map_err(|e| e.to_string())?;
    let url = format!("{}/api/generate", base);
    let mut req = client.post(&url).json(&payload);
    if !bearer.trim().is_empty() {
        req = req.bearer_auth(bearer.trim());
    }
    let resp = req.send().await.map_err(|e| e.to_string())?;
    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("vision model HTTP {status}: {body}"));
    }
    let json: serde_json::Value = resp.json().await.map_err(|e| e.to_string())?;
    json.get("response")
        .and_then(|r| r.as_str())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| "vision model returned empty response".to_string())
}

fn extract_b64_from_data_url(data: &str) -> Option<String> {
    if data.starts_with("data:image/") {
        data.split_once(',').map(|(_, b64)| b64.to_string())
    } else if !data.contains(' ') && data.len() > 64 {
        Some(data.to_string())
    } else {
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionAttachmentIn {
    pub name: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub gist: Option<String>,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionAttachmentOut {
    pub name: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub data: Option<String>,
    #[serde(default)]
    pub thumbnail: Option<String>,
    #[serde(default)]
    pub gist: Option<String>,
    #[serde(rename = "type", default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub vision_model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisionSidecarResult {
    pub attachments: Vec<VisionAttachmentOut>,
    pub skipped: bool,
    pub vision_model: Option<String>,
    pub analyzed_count: u32,
    #[serde(default)]
    pub message: Option<String>,
}

#[tauri::command]
pub async fn discover_vision_models_cmd(
    state: State<'_, EditorState>,
) -> Result<Vec<String>, String> {
    let ollama_url = state.ollama_url.lock().await.clone();
    let bearer = ollama_bearer_for(&state, &ollama_url);
    Ok(discover_vision_models(&ollama_url, &bearer).await)
}

#[tauri::command]
pub async fn vision_sidecar_process_attachments(
    state: State<'_, EditorState>,
    agent_model: String,
    attachments: Vec<VisionAttachmentIn>,
    user_prompt: Option<String>,
    ollama_url: Option<String>,
) -> Result<VisionSidecarResult, String> {
    if attachments.is_empty() {
        return Ok(VisionSidecarResult {
            attachments: attachments
                .into_iter()
                .map(|a| VisionAttachmentOut {
                    name: a.name,
                    path: a.path,
                    data: a.data,
                    thumbnail: a.thumbnail,
                    gist: a.gist,
                    kind: a.kind,
                    vision_model: None,
                })
                .collect(),
            skipped: true,
            vision_model: None,
            analyzed_count: 0,
            message: None,
        });
    }

    if is_vision_capable_model(&agent_model) {
        return Ok(VisionSidecarResult {
            attachments: attachments
                .into_iter()
                .map(|a| VisionAttachmentOut {
                    name: a.name,
                    path: a.path,
                    data: a.data,
                    thumbnail: a.thumbnail,
                    gist: a.gist,
                    kind: a.kind,
                    vision_model: None,
                })
                .collect(),
            skipped: true,
            vision_model: None,
            analyzed_count: 0,
            message: Some(format!(
                "Agent model {} is vision-capable — images pass through natively",
                agent_model
            )),
        });
    }

    let primary_url = ollama_url
        .filter(|u| !u.trim().is_empty())
        .unwrap_or_else(|| {
            // block_on not available — caller should pass URL; fall back to state
            String::new()
        });
    let primary_url = if primary_url.is_empty() {
        state.ollama_url.lock().await.clone()
    } else {
        primary_url
    };
    let bearer = ollama_bearer_for(&state, &primary_url);
    let (vision_model, vision_base) =
        resolve_vision_endpoint(&primary_url, &bearer).await;
    let Some(vision_model) = vision_model else {
        return Ok(VisionSidecarResult {
            attachments: attachments
                .into_iter()
                .map(|a| VisionAttachmentOut {
                    name: a.name,
                    path: a.path,
                    data: a.data,
                    thumbnail: a.thumbnail,
                    gist: a.gist,
                    kind: a.kind,
                    vision_model: None,
                })
                .collect(),
            skipped: false,
            vision_model: None,
            analyzed_count: 0,
            message: Some(
                "No vision model found on Ollama (try: ollama pull qwen2.5vl:7b or moondream)".into(),
            ),
        });
    };

    let prompt_ref = user_prompt.as_deref();
    let mut out = Vec::with_capacity(attachments.len());
    let mut analyzed = 0u32;

    for att in attachments {
        let mut item = VisionAttachmentOut {
            name: att.name.clone(),
            path: att.path.clone(),
            data: att.data.clone(),
            thumbnail: att.thumbnail.clone(),
            gist: att.gist.clone(),
            kind: att.kind.clone(),
            vision_model: None,
        };

        let data = att.data.as_deref().unwrap_or("");
        let already_text = !data.starts_with("data:image/") && data.len() > 20;
        let is_image = data.starts_with("data:image/")
            || att
                .thumbnail
                .as_deref()
                .map(|t| t.starts_with("data:image/"))
                .unwrap_or(false);

        if is_image && !already_text {
            let src = if data.starts_with("data:image/") {
                data
            } else {
                att.thumbnail.as_deref().unwrap_or("")
            };
            if let Some(b64) = extract_b64_from_data_url(src) {
                match describe_image_b64(&vision_base, &bearer, &vision_model, &b64, prompt_ref)
                    .await
                {
                    Ok(summary) => {
                        item.data = Some(summary);
                        item.vision_model = Some(vision_model.clone());
                        analyzed += 1;
                    }
                    Err(e) => {
                        eprintln!("[vision-sidecar] {} failed: {e}", att.name);
                    }
                }
            }
        }

        out.push(item);
    }

    Ok(VisionSidecarResult {
        message: if analyzed > 0 {
            Some(format!(
                "Analyzed {analyzed} image(s) with {vision_model} (agent stays on {agent_model})"
            ))
        } else {
            None
        },
        attachments: out,
        skipped: false,
        vision_model: Some(vision_model),
        analyzed_count: analyzed,
    })
}

const LOCAL_OLLAMA: &str = "http://127.0.0.1:11434";

async fn resolve_vision_endpoint(primary: &str, bearer: &str) -> (Option<String>, String) {
    if let Some(m) = discover_best_vision_model(primary, bearer).await {
        return (Some(m), normalize_ollama_base(primary));
    }
    let local = normalize_ollama_base(LOCAL_OLLAMA);
    if normalize_ollama_base(primary) != local {
        if let Some(m) = discover_best_vision_model(LOCAL_OLLAMA, "").await {
            return (Some(m), local);
        }
    }
    (None, normalize_ollama_base(primary))
}
