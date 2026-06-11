use tauri::State;
use crate::EditorState;
use serde_json::Value;
use crate::apex_red_team;

/// Red Team: Full security scan using BugTraceAI-Apex-G4-26B
#[tauri::command]
pub async fn apex_red_team_scan(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    language: String,
    depth: Option<String>,
) -> Result<Value, String> {
    let depth = match depth.as_deref() {
        Some("quick") => apex_red_team::ScanDepth::Quick,
        Some("deep") => apex_red_team::ScanDepth::Deep,
        _ => apex_red_team::ScanDepth::Standard,
    };
    let report = state.ai.apex.red_team().scan(apex_red_team::RedTeamScanRequest {
        target_code: code,
        file_path,
        language,
        scan_depth: depth,
        focus_areas: vec![],
    }).await?;
    serde_json::to_value(&report).map_err(|e| e.to_string())
}

/// Red Team: Quick vulnerability check
#[tauri::command]
pub async fn apex_quick_check(
    state: State<'_, EditorState>,
    code: String,
    language: String,
) -> Result<Value, String> {
    let findings = state.ai.apex.red_team().quick_check(&code, &language).await?;
    serde_json::to_value(&findings).map_err(|e| e.to_string())
}

/// Red Team: Execute a LIVE attack chain (real curl/nmap/audits — not LLM simulation)
#[tauri::command]
pub async fn apex_simulate_attack(
    state: State<'_, EditorState>,
    target: String,
    attack_type: String,
) -> Result<Value, String> {
    state
        .ai.tools
        .call_tool(
            "apex_simulate_attack",
            serde_json::json!({ "target": target, "attack_type": attack_type }),
        )
        .await
        .map_err(|e| e.to_string())
}

/// Red Team: Generate pentest report
#[tauri::command]
pub async fn apex_pentest_report(
    state: State<'_, EditorState>,
    files: Vec<Vec<String>>,
) -> Result<Value, String> {
    let file_pairs: Vec<(String, String)> = files.into_iter()
        .filter_map(|pair| {
            if pair.len() == 2 { Some((pair[0].clone(), pair[1].clone())) } else { None }
        })
        .collect();
    state.ai.apex.red_team().pentest_report(file_pairs).await
}

/// Engine 1: Autonomous System Architect — design complete system
#[tauri::command]
pub async fn apex_architect_design(
    state: State<'_, EditorState>,
    description: String,
) -> Result<Value, String> {
    let rec = state.ai.apex.architect_design(&description).await?;
    serde_json::to_value(&rec).map_err(|e| e.to_string())
}

/// Engine 1: Scaffold a project from architecture spec
#[tauri::command]
pub async fn apex_architect_scaffold(
    state: State<'_, EditorState>,
    architecture: String,
) -> Result<Value, String> {
    state.ai.apex.architect_scaffold(&architecture).await
}

/// Engine 2: Threat Anticipation — predict future vulnerabilities
#[tauri::command]
pub async fn apex_threat_anticipate(
    state: State<'_, EditorState>,
    code: String,
    context: String,
) -> Result<Value, String> {
    state.ai.apex.threat_anticipate(&code, &context).await
}

/// Engine 2: Simulate attack with concurrent users
#[tauri::command]
pub async fn apex_threat_simulate(
    state: State<'_, EditorState>,
    endpoint: String,
    attack_type: String,
    concurrent_users: u32,
) -> Result<Value, String> {
    state.ai.apex.threat_simulate(&endpoint, &attack_type, concurrent_users).await
}

/// Engine 3: Performance Optimizer — find and fix bottlenecks
#[tauri::command]
pub async fn apex_perf_optimize(
    state: State<'_, EditorState>,
    code: String,
    language: String,
) -> Result<Value, String> {
    let suggestions = state.ai.apex.perf_optimize(&code, &language).await?;
    serde_json::to_value(&suggestions).map_err(|e| e.to_string())
}

/// Engine 3: Deep profile a specific function
#[tauri::command]
pub async fn apex_perf_profile(
    state: State<'_, EditorState>,
    function_code: String,
    language: String,
) -> Result<Value, String> {
    state.ai.apex.perf_profile_function(&function_code, &language).await
}

/// Engine 4: Self-Improving Code — iteratively improve generated code
#[tauri::command]
pub async fn apex_self_improve(
    state: State<'_, EditorState>,
    code: String,
    language: String,
    iterations: Option<u32>,
) -> Result<Value, String> {
    state.ai.apex.self_improve(&code, &language, iterations.unwrap_or(3)).await
}

/// Engine 5: Explainable Security — explain fixes in plain English
#[tauri::command]
pub async fn apex_security_explain(
    state: State<'_, EditorState>,
    vulnerability: String,
    fix_diff: String,
) -> Result<Value, String> {
    state.ai.apex.security_explain(&vulnerability, &fix_diff).await
}

/// Engine 5: Security audit with educational annotations
#[tauri::command]
pub async fn apex_security_audit(
    state: State<'_, EditorState>,
    code: String,
    language: String,
) -> Result<Value, String> {
    state.ai.apex.security_audit_explain(&code, &language).await
}

/// Engine 6: Multi-System Control — scan multiple systems
#[tauri::command]
pub async fn apex_multi_system_scan(
    state: State<'_, EditorState>,
    systems: Vec<Vec<String>>,
) -> Result<Value, String> {
    let system_pairs: Vec<(String, String)> = systems.into_iter()
        .filter_map(|pair| {
            if pair.len() == 2 { Some((pair[0].clone(), pair[1].clone())) } else { None }
        })
        .collect();
    state.ai.apex.multi_system_scan(system_pairs).await
}

/// Engine 7: Failure Prediction — predict system crashes
#[tauri::command]
pub async fn apex_predict_failures(
    state: State<'_, EditorState>,
    code: String,
    logs: Option<String>,
) -> Result<Value, String> {
    let predictions = state.ai.apex.predict_failures(&code, logs.as_deref()).await?;
    serde_json::to_value(&predictions).map_err(|e| e.to_string())
}

/// Engine 7: Predict from server logs
#[tauri::command]
pub async fn apex_predict_from_logs(
    state: State<'_, EditorState>,
    logs: String,
) -> Result<Value, String> {
    state.ai.apex.predict_from_logs(&logs).await
}

/// FULL SWEEP: Run all APEX engines in parallel on a target
#[tauri::command]
pub async fn apex_full_sweep(
    state: State<'_, EditorState>,
    code: String,
    file_path: String,
    language: String,
) -> Result<Value, String> {
    state.ai.apex.full_sweep(&code, &file_path, &language).await
}

/// Get the APEX results feed (latest intelligence findings)
#[tauri::command]
pub async fn apex_get_results_feed(
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    let feed = state.ai.apex.get_results_feed().await;
    serde_json::to_value(&feed).map_err(|e| e.to_string())
}

/// Set the model for a specific APEX engine
#[tauri::command]
pub async fn apex_set_engine_model(
    state: State<'_, EditorState>,
    engine: String,
    model: String,
) -> Result<(), String> {
    state.ai.apex.set_engine_model(&engine, &model).await;
    Ok(())
}

/// Get red team findings history
#[tauri::command]
pub async fn apex_get_findings_history(
    state: State<'_, EditorState>,
) -> Result<Value, String> {
    let history = state.ai.apex.red_team().get_findings_history().await;
    serde_json::to_value(&history).map_err(|e| e.to_string())
}

/// Auto-downgrade all APEX engines to small models for local/offline use
/// Recommended for M1 Macs with limited RAM (8GB+)
#[tauri::command]
pub async fn apex_set_local_mode(
    state: State<'_, EditorState>,
    small_model: Option<String>,
) -> Result<(), String> {
    let model = small_model.unwrap_or_else(|| "qwen3.5:2b".to_string());
    let engines = vec![
        "architect",
        "threat",
        "perf",
        "self_improve",
        "explainer",
        "multi_system",
        "predictor",
    ];
    for engine in engines {
        state.ai.apex.set_engine_model(engine, &model).await;
    }
    Ok(())
}
