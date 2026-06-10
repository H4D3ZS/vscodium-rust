use tauri::State;
use crate::EditorState;
use serde_json::{json, Value};
use crate::model_manager::{ModelInfo, get_context_window, is_suitable_for_offline, parse_param_count};

/// List all available models from local Ollama
#[tauri::command]
pub async fn list_ollama_models(
    state: State<'_, EditorState>,
) -> Result<Vec<ModelInfo>, String> {
    let ollama_url = state.ollama_url.lock().await.clone();

    // Query Ollama /api/tags endpoint
    let client = reqwest::Client::new();
    let url = format!("{}/api/tags", ollama_url.trim_end_matches('/'));

    let response = client
        .get(&url)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Ollama: {}", e))?;

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Ollama response: {}", e))?;

    let mut models = Vec::new();

    if let Some(model_list) = data.get("models").and_then(|m| m.as_array()) {
        for model in model_list {
            if let Some(name) = model.get("name").and_then(|n| n.as_str()) {
                let context_window = get_context_window(name);
                let suitable = is_suitable_for_offline(name);

                models.push(ModelInfo {
                    name: name.to_string(),
                    context_window,
                    recommended: suitable && context_window >= 16_384,
                    supports_12b_and_below: suitable,
                });
            }
        }
    }

    // Sort: recommended first, then by size
    models.sort_by(|a, b| {
        if a.recommended != b.recommended {
            b.recommended.cmp(&a.recommended)
        } else {
            b.context_window.cmp(&a.context_window)
        }
    });

    if models.is_empty() {
        return Err("No models found in Ollama. Run 'ollama pull qwen3.5:12b'".to_string());
    }

    Ok(models)
}

/// Get currently selected model
#[tauri::command]
pub async fn get_current_model(
    state: State<'_, EditorState>,
) -> Result<String, String> {
    let model = state.current_model.lock().await.clone();
    Ok(model)
}

/// Set the model for future inference
#[tauri::command]
pub async fn set_current_model(
    state: State<'_, EditorState>,
    model_name: String,
) -> Result<ModelInfo, String> {
    // Verify model exists in Ollama
    let models = list_ollama_models(state.clone()).await?;

    let found = models.iter()
        .find(|m| m.name == model_name)
        .cloned()
        .ok_or_else(|| format!("Model '{}' not found in Ollama", model_name))?;

    // Update current model
    {
        let mut current = state.current_model.lock().await;
        *current = model_name.clone();
    }

    println!("[Model] Switched to: {} (context: {}K)",
        model_name,
        found.context_window / 1024
    );

    Ok(found)
}

/// Get model info (context window, capabilities)
#[tauri::command]
pub async fn get_model_info(
    model_name: String,
) -> Result<Value, String> {
    let context_window = get_context_window(&model_name);
    let param_count = parse_param_count(&model_name);
    let suitable = is_suitable_for_offline(&model_name);

    Ok(json!({
        "name": model_name,
        "context_window": context_window,
        "param_count": param_count,
        "suitable_for_offline_12b_and_below": suitable,
        "estimated_speed_relative": if suitable { "fast" } else { "slow" },
        "recommended": suitable && context_window >= 16_384,
    }))
}

/// Auto-detect best available model for offline work
#[tauri::command]
pub async fn detect_best_model(
    state: State<'_, EditorState>,
) -> Result<String, String> {
    let models = list_ollama_models(state.clone()).await?;

    // Find first recommended model
    if let Some(best) = models.iter().find(|m| m.recommended) {
        set_current_model(state.clone(), best.name.clone()).await?;
        return Ok(best.name.clone());
    }

    // Fallback: use first available 12b-and-below
    if let Some(fallback) = models.iter().find(|m| m.supports_12b_and_below) {
        set_current_model(state.clone(), fallback.name.clone()).await?;
        return Ok(fallback.name.clone());
    }

    Err("No suitable models found for offline 12b-and-below work".to_string())
}

/// Apply a model to all APEX specialist engines
#[tauri::command]
pub async fn apply_model_to_all_engines(
    state: State<'_, EditorState>,
    model_name: String,
) -> Result<Value, String> {
    // Verify model exists in Ollama
    let models = list_ollama_models(state.clone()).await?;
    let _found = models.iter()
        .find(|m| m.name == model_name)
        .ok_or_else(|| format!("Model '{}' not found in Ollama", model_name))?;

    // Apply to all APEX engines
    let engines = vec!["architect", "threat", "perf", "self_improve", "explainer", "multi_system", "predictor"];

    for engine in &engines {
        state.apex.set_engine_model(engine, &model_name).await;
    }

    // Also set as current model
    set_current_model(state.clone(), model_name.clone()).await?;

    println!("[Model] Applied '{}' to all {} APEX engines", model_name, engines.len());

    Ok(json!({
        "status": "success",
        "model": model_name,
        "engines_updated": engines.len(),
        "message": format!("Model '{}' applied to all specialist engines", model_name)
    }))
}
