use tauri::State;
use serde_json::{json, Value};
use crate::model_manager::{ModelInfo, get_context_window, is_suitable_for_offline, parse_param_count};

/// List the local text-generation models Lemonade serves.
///
/// Lemonade is the only local backend — it runs real llama.cpp. The catalog at
/// `/api/v1/models` also lists image/speech recipes (`sd-cpp`, `whispercpp`,
/// `kokoro`), which are filtered out here so the model picker only offers LLMs.
#[tauri::command]
pub async fn list_local_models(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<Vec<ModelInfo>, String> {
    let base = state.ai.engine.lemonade_base().await;
    let base = base.trim_end_matches('/');

    let client = reqwest::Client::new();
    let response = client
        .get(format!("{}/api/v1/models", base))
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
        .map_err(|e| format!("Failed to connect to Lemonade at {}: {}", base, e))?;

    let data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse Lemonade response: {}", e))?;

    let rows = data
        .get("data")
        .and_then(|d| d.as_array())
        .or_else(|| data.as_array())
        .cloned()
        .unwrap_or_default();

    let mut models = Vec::new();
    for model in &rows {
        if !model.get("downloaded").and_then(|d| d.as_bool()).unwrap_or(false) {
            continue;
        }
        if model.get("recipe").and_then(|r| r.as_str()) != Some("llamacpp") {
            continue;
        }
        let Some(name) = model.get("id").and_then(|n| n.as_str()) else {
            continue;
        };
        let context_window = get_context_window(name);
        let suitable = is_suitable_for_offline(name);

        models.push(ModelInfo {
            name: name.to_string(),
            context_window,
            recommended: suitable && context_window >= 16_384,
            supports_12b_and_below: suitable,
        });
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
        return Err(format!(
            "No models found on the Lemonade server at {}. Pull one with `lemonade pull <hf-repo>:<quant>`.",
            base
        ));
    }

    Ok(models)
}

/// Get currently selected model
#[tauri::command]
pub async fn get_current_model(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<String, String> {
    let model = state.ai.current_model.lock().await.clone();
    Ok(model)
}

/// Set the model for future inference
#[tauri::command]
pub async fn set_current_model(
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    model_name: String,
) -> Result<ModelInfo, String> {
    // Verify the model is actually served locally
    let models = list_local_models(state.clone()).await?;

    let found = models.iter()
        .find(|m| m.name == model_name)
        .cloned()
        .ok_or_else(|| format!("Model '{}' is not served by Lemonade", model_name))?;

    // Update current model
    {
        let mut current = state.ai.current_model.lock().await;
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
    state: State<'_, std::sync::Arc<crate::EditorState>>,
) -> Result<String, String> {
    let models = list_local_models(state.clone()).await?;

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
    state: State<'_, std::sync::Arc<crate::EditorState>>,
    model_name: String,
) -> Result<Value, String> {
    // Verify the model is actually served locally
    let models = list_local_models(state.clone()).await?;
    let _found = models.iter()
        .find(|m| m.name == model_name)
        .ok_or_else(|| format!("Model '{}' is not served by Lemonade", model_name))?;

    // Apply to all APEX engines
    let engines = vec!["architect", "threat", "perf", "self_improve", "explainer", "multi_system", "predictor"];

    for engine in &engines {
        state.ai.apex.set_engine_model(engine, &model_name).await;
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
