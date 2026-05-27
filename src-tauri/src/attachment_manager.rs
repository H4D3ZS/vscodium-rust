use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use base64::{engine::general_purpose, Engine as _};
use tauri_plugin_dialog::DialogExt;
use std::path::PathBuf;
use daemon::gist::GistInjector;
use daemon::neural_math::VECTOR_DIM;

use tokio::sync::Mutex;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttachmentInfo {
    pub path: String,
    pub name: String,
    pub gist: Option<String>, // Base64 or string representation of the token
    pub thumbnail: Option<String>, // Base64 data URL for preview
    pub data: Option<String>, // Textual summary or content
}

pub struct AttachmentManager {
    pub gist_injector: GistInjector,
    pub processing_lock: Mutex<()>,
}

const VISION_MODELS: &[&str] = &["gemma3", "moondream", "llava", "bakllava", "minicpm-v"];

impl AttachmentManager {
    pub fn new() -> Self {
        Self {
            gist_injector: GistInjector::new(),
            processing_lock: Mutex::new(()),
        }
    }

    pub async fn process_file(&self, path: PathBuf, model: &str) -> Result<AttachmentInfo, String> {
        let _lock = self.processing_lock.lock().await;
        
        let name = path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned();
        
        let extension = path.extension()
            .map(|ext| ext.to_string_lossy().to_lowercase())
            .unwrap_or_default();
            
        let is_image = extension == "png" || extension == "jpg" || extension == "jpeg" || extension == "webp";

        if is_image {
             println!("[DEBUG] Visual Asset detected. Checking for Ollama vision models...");
             
             // 1. Find a vision-capable model
             let vision_model = self.find_vision_model().await;
             let mut visual_summary = String::new();
             
             if let Some(vm) = vision_model {
                 println!("[DEBUG] Using vision model: {} for visual neuralization.", vm);
                 match self.generate_visual_summary(&path, &vm).await {
                     Ok(summary) => {
                         visual_summary = summary;
                         println!("[DEBUG] Visual Summary generated ({} chars).", visual_summary.len());
                     },
                     Err(e) => {
                         println!("[WARN] Visual summary failed: {}. Falling back to default Spatial Gist.", e);
                     }
                 }
             }

             // Generate Thumbnail for UX preview (Fast, Local) - Stubbed to drop heavy 'image' dependency
             let thumbnail_data = None;

             // 2. Neuralize the visual state
             let gist_injector_res = if !visual_summary.is_empty() {
                 println!("[DEBUG] Visual Summary generated. Using internal vector for Gist to save GPU swapping.");
                 // Instead of using an embed model (double GPU load), we use the internal encoder for the vector
                 // and provide the visual_summary as text for reasoning.
                 self.gist_injector.inject_visual_knowledge(&path).await
                     .map(|_| ())
             } else {
                 // Standard fallback to local engine (CLIP/SigLIP)
                 self.gist_injector.inject_visual_knowledge(&path).await
                     .map(|_| ())
             };
             
             match gist_injector_res {
                 Ok(_) => {
                    let current_gist = self.gist_injector.get_gist_token().await;
                    let gist_str = general_purpose::STANDARD.encode(serde_json::to_vec(&current_gist).unwrap_or_default());
                    return Ok(AttachmentInfo {
                        path: path.to_string_lossy().into_owned(),
                        name,
                        gist: Some(gist_str),
                        thumbnail: thumbnail_data,
                        data: if visual_summary.is_empty() { None } else { Some(visual_summary) },
                    });
                 },
                 Err(e) => {
                    println!("[ERROR] Visual Gist injection failed: {}. section 325", e);
                    return Ok(AttachmentInfo {
                        path: path.to_string_lossy().into_owned(),
                        name,
                        gist: None,
                        thumbnail: thumbnail_data,
                        data: if visual_summary.is_empty() { None } else { Some(visual_summary) },
                    });
                 }
             }
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        // 1. Compute Embedding via Ollama (Now optional)
        let embedding_res = if model.is_empty() {
             Err("No model specified for neuralization. Falling back to raw attachment.".to_string())
        } else {
             self.compute_embedding(&content, model).await
        };
        
        match embedding_res {
            Ok(embedding) => {
                // 2. Inject into Kortex Gist logic
                let mut fixed_vec = [0.0f32; VECTOR_DIM];
                for i in 0..VECTOR_DIM {
                    if i < embedding.len() {
                        fixed_vec[i] = embedding[i];
                    }
                }
                
                self.gist_injector.inject_knowledge(&fixed_vec).await;
                
                // 3. Return info with Gist
                let current_gist = self.gist_injector.get_gist_token().await;
                let gist_str = general_purpose::STANDARD.encode(serde_json::to_vec(&current_gist).unwrap_or_default());

                Ok(AttachmentInfo {
                    path: path.to_string_lossy().into_owned(),
                    name,
                    gist: Some(gist_str),
                    thumbnail: None,
                    data: Some(content),
                })
            },
            Err(e) => {
                println!("[WARN] Neuralization failed, falling back to raw attachment: {}", e);
                // Return Info without Gist. The frontend/agent will handle raw content.
                Ok(AttachmentInfo {
                    path: path.to_string_lossy().into_owned(),
                    name,
                    gist: None,
                    thumbnail: None,
                    data: Some(content),
                })
            }
        }
    }

    async fn compute_embedding(&self, text: &str, model: &str) -> Result<Vec<f32>, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        println!("[DEBUG] Requesting embedding from Ollama for model: {}", model);
        
        let res = client.post("http://127.0.0.1:11434/api/embeddings")
            .json(&serde_json::json!({
                "model": model,
                "prompt": text,
                "input": text
            }))
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    println!("[ERROR] Ollama request timed out (60s)");
                    "Ollama embedding request timed out. The model might be loading or the file is too large.".to_string()
                } else {
                    println!("[ERROR] Ollama connection failed: {:?}", e);
                    format!("Ollama connection failed: {}. Ensure Ollama is running.", e)
                }
            })?;

        let status = res.status();
        let body = res.text().await.map_err(|e| e.to_string())?;
        println!("[DEBUG] Ollama response status: {}, body truncate: {}", status, if body.len() > 100 { &body[..100] } else { &body });

        if !status.is_success() {
            return Err(format!("Ollama error ({}): {}", status, body));
        }

        let json: serde_json::Value = serde_json::from_str(&body).map_err(|e| e.to_string())?;
        
        // Try 'embedding' (older) or 'embeddings' (newer) or 'data[0].embedding'
        if let Some(arr) = json.get("embedding").and_then(|e| e.as_array()) {
            let floats: Vec<f32> = arr.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect();
            return Ok(floats);
        }
        
        if let Some(embeddings) = json.get("embeddings").and_then(|e| e.as_array()) {
             if let Some(first) = embeddings.first().and_then(|e| e.as_array()) {
                 let floats: Vec<f32> = first.iter().filter_map(|v| v.as_f64().map(|f| f as f32)).collect();
                 return Ok(floats);
             }
        }
        
        Err(format!("Ollama embedding format not recognized. Response: {}", body))
    }

    async fn find_vision_model(&self) -> Option<String> {
        let client = reqwest::Client::new();
        let res = client.get("http://127.0.0.1:11434/api/tags").send().await.ok()?;
        let json: serde_json::Value = res.json().await.ok()?;
        
        if let Some(models) = json.get("models").and_then(|m| m.as_array()) {
            for v_model in VISION_MODELS {
                for m in models {
                    if let Some(name) = m.get("name").and_then(|n| n.as_str()) {
                        if name.contains(v_model) {
                            return Some(name.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    async fn generate_visual_summary(&self, image_path: &std::path::Path, model: &str) -> Result<String, String> {
        let img_bytes = std::fs::read(image_path).map_err(|e| e.to_string())?;
        let b64_img = general_purpose::STANDARD.encode(img_bytes);
        
        let client = reqwest::Client::new();
        let payload = serde_json::json!({
            "model": model,
            "prompt": "Describe this image in detail, focusing on UI elements, code structures, or relevant visual context for a software developer. Be concise but thorough.",
            "images": [b64_img],
            "stream": false,
            "keep_alive": 0 // Force GPU free after call
        });

        let res = client.post("http://127.0.0.1:11434/api/generate")
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: serde_json::Value = res.json().await.map_err(|e| e.to_string())?;
        json.get("response")
            .and_then(|r| r.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| "Failed to get response from vision model".to_string())
    }
    
    pub async fn unload_model(&self, model: &str) -> Result<(), String> {
        let client = reqwest::Client::new();
        println!("[DEBUG] Requesting Ollama to unload model: {}", model);
        let payload = serde_json::json!({
            "model": model,
            "keep_alive": 0
        });
        
        let _ = client.post("http://127.0.0.1:11434/api/generate")
            .json(&payload)
            .send()
            .await;
            
        Ok(())
    }
}

use std::sync::Arc;

#[tauri::command]
pub async fn select_and_process_attachment(
    app: AppHandle,
    manager: tauri::State<'_, Arc<AttachmentManager>>,
    model: String,
) -> Result<Vec<AttachmentInfo>, String> {
    println!("[DEBUG] select_and_process_attachment (multi) called with model: {}", model);
    let (tx, rx) = tokio::sync::oneshot::channel();
    
    app.dialog()
        .file()
        .pick_files(move |paths| {
            println!("[DEBUG] pick_files callback triggered with {:?} items", paths.as_ref().map(|p| p.len()));
            let _ = tx.send(paths);
        });

    let paths = rx.await
        .map_err(|e| {
            println!("[ERROR] oneshot recv error: {:?}", e);
            e.to_string()
        })?
        .ok_or_else(|| {
            println!("[DEBUG] Selection cancelled by user");
            "No files selected".to_string()
        })?;

    let mut results = Vec::new();
    for path in paths {
        println!("[DEBUG] Neuralizing: {:?} with model: {}", path, model);
        let path_buf = PathBuf::from(path.to_string());
        match manager.process_file(path_buf, &model).await {
            Ok(info) => {
                // Broadcast update for each file
                let _ = app.emit("memory-gist-updated", &info);
                results.push(info);
            },
            Err(e) => {
                println!("[ERROR] Failed to process {:?}: {}", path, e);
                // Continue to next file
            }
        }
    }
    
    Ok(results)
}
