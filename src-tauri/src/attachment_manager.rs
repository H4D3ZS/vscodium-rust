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
}

pub struct AttachmentManager {
    pub gist_injector: GistInjector,
    pub processing_lock: Mutex<()>,
}

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
        
        let is_image = path.extension()
            .map(|ext| {
                let ext_str = ext.to_string_lossy().to_lowercase();
                ext_str == "png" || ext_str == "jpg" || ext_str == "jpeg" || ext_str == "webp"
            })
            .unwrap_or(false);

        if is_image {
             println!("[DEBUG] Visual Asset detected. Routing to Spatial Gist engine: {:?}", path);
             let result = self.gist_injector.inject_visual_knowledge(&path).await;
             
             match result {
                 Ok(_) => {
                    let current_gist = self.gist_injector.get_gist_token().await;
                    let gist_str = general_purpose::STANDARD.encode(serde_json::to_vec(&current_gist).unwrap_or_default());
                    return Ok(AttachmentInfo {
                        path: path.to_string_lossy().into_owned(),
                        name,
                        gist: Some(gist_str),
                    });
                 },
                 Err(e) => {
                    println!("[ERROR] Visual Gist injection failed: {}. section 315", e);
                    // Fallback to raw path if encoding fails
                    return Ok(AttachmentInfo {
                        path: path.to_string_lossy().into_owned(),
                        name,
                        gist: None,
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
                })
            },
            Err(e) => {
                println!("[WARN] Neuralization failed, falling back to raw attachment: {}", e);
                // Return Info without Gist. The frontend/agent will handle raw content.
                Ok(AttachmentInfo {
                    path: path.to_string_lossy().into_owned(),
                    name,
                    gist: None,
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
