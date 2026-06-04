// ═══════════════════════════════════════════════════════════════════════════════
// APEX ORCHESTRATOR — Central Intelligence Coordinator
// ═══════════════════════════════════════════════════════════════════════════════
//
// Coordinates all 7 intelligence engines + BugTraceAI Red Team.
// Routes tasks to the appropriate specialist model, aggregates results,
// and maintains a unified threat/performance state for the IDE.
// ═══════════════════════════════════════════════════════════════════════════════

use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::apex_red_team::{ApexRedTeam, RedTeamScanRequest, ScanDepth};

/// Default Ollama model assignments per engine
const MODEL_ARCHITECT: &str = "qwen2.5:32b";
const MODEL_THREAT: &str = "huihui_ai/qwen3.5-abliterated:35b";
const MODEL_PERF: &str = "qwen2.5-coder:7b";
const MODEL_SELF_IMPROVE: &str = "qwen2.5:14b";
const MODEL_EXPLAINER: &str = "qwen2.5:14b";
const MODEL_MULTI_SYSTEM: &str = "qwen2.5:32b";
const MODEL_PREDICTOR: &str = "qwen3.6:35b-a3b";

/// Scan result from any engine
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApexResult {
    pub engine: String,
    pub result_type: String,
    pub data: Value,
    pub severity: Option<String>,
    pub timestamp: u64,
}

/// System architecture recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchitectureRecommendation {
    pub project_type: String,
    pub frontend: String,
    pub backend: String,
    pub database: String,
    pub infrastructure: String,
    pub reasoning: String,
    pub estimated_scale: String,
}

/// Performance optimization suggestion  
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfSuggestion {
    pub file_path: String,
    pub line_range: Option<(usize, usize)>,
    pub current_complexity: String,
    pub suggested_complexity: String,
    pub description: String,
    pub optimized_code: Option<String>,
    pub estimated_improvement: String,
}

/// Failure prediction alert
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePrediction {
    pub component: String,
    pub failure_type: String,
    pub probability: f32,
    pub time_horizon: String,
    pub evidence: Vec<String>,
    pub mitigation: String,
}

/// The central orchestrator for all APEX intelligence engines
pub struct ApexOrchestrator {
    client: Client,
    ollama_url: Arc<Mutex<String>>,
    red_team: Arc<ApexRedTeam>,
    results_feed: Arc<Mutex<Vec<ApexResult>>>,
    workspace_root: Arc<Mutex<Option<PathBuf>>>,
    /// Per-engine model overrides
    model_overrides: Arc<Mutex<std::collections::HashMap<String, String>>>,
    config_dir: Arc<Mutex<Option<PathBuf>>>,
}

impl ApexOrchestrator {
    pub fn new(ollama_url: &str, workspace_root: Option<PathBuf>, config_dir: Option<PathBuf>) -> Self {
        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .unwrap_or_else(|_| Client::new());

        let red_team = Arc::new(ApexRedTeam::new(ollama_url));

        Self {
            client,
            ollama_url: Arc::new(Mutex::new(ollama_url.to_string())),
            red_team,
            results_feed: Arc::new(Mutex::new(Vec::new())),
            workspace_root: Arc::new(Mutex::new(workspace_root)),
            model_overrides: Arc::new(Mutex::new(std::collections::HashMap::new())),
            config_dir: Arc::new(Mutex::new(config_dir)),
        }
    }

    /// Get the red team engine
    pub fn red_team(&self) -> Arc<ApexRedTeam> {
        self.red_team.clone()
    }

    /// Override the model for a specific engine
    pub async fn set_engine_model(&self, engine: &str, model: &str) {
        self.model_overrides.lock().await.insert(engine.to_string(), model.to_string());
    }

    /// Helper to fetch custom settings value from Supabase
    async fn fetch_supabase_model(&self, key: &str) -> Option<String> {
        let config_dir = self.config_dir.lock().await.clone()?;
        let (url, anon_key) = crate::auth::supabase_config(&config_dir);
        if url.is_empty() || anon_key.is_empty() {
            return None;
        }

        let req_url = format!("{}/rest/v1/app_settings?key=eq.{}&select=value", url, key);
        let res = self.client.get(&req_url)
            .header("apikey", &anon_key)
            .header("Authorization", format!("Bearer {}", anon_key))
            .send()
            .await
            .ok()?;

        if res.status().is_success() {
            let json: Value = res.json().await.ok()?;
            if let Some(arr) = json.as_array() {
                if let Some(first) = arr.first() {
                    if let Some(val) = first.get("value") {
                        if let Some(s) = val.as_str() {
                            return Some(s.to_string());
                        }
                    }
                }
            }
        }
        None
    }

    /// Get the active model for an engine, fetching override settings dynamically
    async fn get_model(&self, engine: &str) -> String {
        if let Some(m) = self.model_overrides.lock().await.get(engine) {
            return m.clone();
        }

        // Map engine name to settings key
        let sb_key = match engine {
            "architect" => Some("model_architect"),
            "threat" => Some("model_threat"),
            "perf" => Some("model_perf"),
            "self_improve" => Some("model_self_improve"),
            "explainer" => Some("model_explainer"),
            "multi_system" => Some("model_multi_system"),
            "predictor" => Some("model_predictor"),
            _ => None,
        };

        if let Some(key) = sb_key {
            if let Some(override_model) = self.fetch_supabase_model(key).await {
                if !override_model.trim().is_empty() {
                    return override_model.trim().to_string();
                }
            }
        }

        match engine {
            "architect" => MODEL_ARCHITECT,
            "threat" => MODEL_THREAT,
            "perf" => MODEL_PERF,
            "self_improve" => MODEL_SELF_IMPROVE,
            "explainer" => MODEL_EXPLAINER,
            "multi_system" => MODEL_MULTI_SYSTEM,
            "predictor" => MODEL_PREDICTOR,
            _ => MODEL_PERF,
        }.to_string()
    }

    /// Update the Ollama URL for all engines
    pub async fn set_ollama_url(&self, url: &str) {
        *self.ollama_url.lock().await = url.to_string();
        self.red_team.set_ollama_url(url).await;
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 1: AUTONOMOUS SYSTEM ARCHITECT
    // ═══════════════════════════════════════════════════════════════════════════

    /// Design a complete system architecture for a given project description
    pub async fn architect_design(&self, project_description: &str) -> Result<ArchitectureRecommendation, String> {
        let prompt = format!(
            "You are a senior system architect. Design the complete system architecture for:\n\n\
             \"{}\"\n\n\
             Respond with a JSON object containing:\n\
             - \"project_type\": category (web app, mobile, CLI, microservice, etc.)\n\
             - \"frontend\": recommended frontend stack with reasoning\n\
             - \"backend\": recommended backend stack with reasoning\n\
             - \"database\": recommended database(s) with reasoning\n\
             - \"infrastructure\": deployment strategy (Docker, K8s, serverless, etc.)\n\
             - \"reasoning\": why this stack was chosen (performance, security, scale)\n\
             - \"estimated_scale\": expected user/request capacity\n\n\
             Wrap your JSON in ```json blocks.",
            project_description
        );

        let response = self.query_engine("architect", &prompt, None).await?;
        self.parse_json_response::<ArchitectureRecommendation>(&response)
            .ok_or_else(|| format!("Failed to parse architecture: {}", &response[..200.min(response.len())]))
    }

    /// Generate a scaffolded project structure
    pub async fn architect_scaffold(&self, architecture: &str) -> Result<Value, String> {
        let prompt = format!(
            "Generate a complete project scaffold for this architecture: {}\n\n\
             Provide: directory structure, key files with starter code, \
             docker-compose.yml, CI/CD pipeline config, and .env template.\n\
             Return as JSON with keys: 'files' (array of {{path, content}}), 'commands' (setup commands).",
            architecture
        );
        let response = self.query_engine("architect", &prompt, None).await?;
        Ok(json!({"scaffold": response}))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 2: THREAT ANTICIPATION ENGINE
    // ═══════════════════════════════════════════════════════════════════════════

    /// Predict vulnerabilities that could emerge from this code BEFORE they exist
    pub async fn threat_anticipate(&self, code: &str, context: &str) -> Result<Value, String> {
        let prompt = format!(
            "You are a threat anticipation engine. Analyze this code and predict \
             vulnerabilities that DON'T EXIST YET but COULD emerge under these conditions:\n\
             - High traffic (10,000+ concurrent users)\n\
             - Hostile network (MITM, DNS spoofing)\n\
             - Compromised dependency (supply chain attack)\n\
             - Insider threat (privileged user abuse)\n\n\
             Context: {}\n\n\
             Code:\n```\n{}\n```\n\n\
             For each predicted threat, provide:\n\
             1. Threat scenario description\n\
             2. Trigger conditions\n\
             3. Estimated probability (0-1)\n\
             4. Impact if exploited\n\
             5. Preventive measure\n\n\
             Return as JSON array wrapped in ```json blocks.",
            context, code
        );
        let response = self.query_engine("threat", &prompt, None).await?;
        Ok(json!({"threats": response}))
    }

    /// Simulate a specific attack scenario
    pub async fn threat_simulate(&self, endpoint: &str, attack_type: &str, concurrent_users: u32) -> Result<Value, String> {
        let prompt = format!(
            "Simulate a {} attack against this endpoint with {} concurrent attackers:\n\n\
             Endpoint: {}\n\n\
             Provide:\n\
             1. Attack timeline (second-by-second)\n\
             2. Server resource consumption at each stage\n\
             3. Point of failure / crash\n\
             4. What logs would show\n\
             5. Recommended rate limiting / WAF rules",
            attack_type, concurrent_users, endpoint
        );
        let response = self.query_engine("threat", &prompt, None).await?;
        Ok(json!({"simulation": response, "attack_type": attack_type, "users": concurrent_users}))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 3: PERFORMANCE OPTIMIZER
    // ═══════════════════════════════════════════════════════════════════════════

    /// Analyze code for performance bottlenecks and suggest optimizations
    pub async fn perf_optimize(&self, code: &str, language: &str) -> Result<Vec<PerfSuggestion>, String> {
        let prompt = format!(
            "Analyze this {} code for performance issues. For each problem:\n\
             1. Identify the algorithmic complexity (Big-O)\n\
             2. Suggest the optimal algorithm/data structure\n\
             3. Provide optimized replacement code\n\
             4. Estimate the improvement (e.g., '40% less memory', '3x faster')\n\n\
             Focus on: memory allocation, loop efficiency, data structure choice, \
             async patterns, zero-copy techniques, cache friendliness.\n\n\
             Return as JSON array of objects with keys:\n\
             file_path, line_range, current_complexity, suggested_complexity, \
             description, optimized_code, estimated_improvement\n\n\
             Code:\n```{}\n{}\n```",
            language, language, code
        );
        let response = self.query_engine("perf", &prompt, None).await?;
        
        // Try to parse structured response
        if let Some(suggestions) = self.parse_json_array::<PerfSuggestion>(&response) {
            return Ok(suggestions);
        }

        // Fallback: create a single suggestion from raw text
        Ok(vec![PerfSuggestion {
            file_path: "inline".to_string(),
            line_range: None,
            current_complexity: "Unknown".to_string(),
            suggested_complexity: "See analysis".to_string(),
            description: response.chars().take(2000).collect(),
            optimized_code: None,
            estimated_improvement: "See analysis".to_string(),
        }])
    }

    /// Profile a specific function and suggest zero-copy / allocation-free alternatives
    pub async fn perf_profile_function(&self, function_code: &str, language: &str) -> Result<Value, String> {
        let prompt = format!(
            "Deep-profile this {} function. Analyze:\n\
             1. Memory allocations (heap vs stack)\n\
             2. Cache line utilization\n\
             3. Branch prediction friendliness\n\
             4. SIMD vectorization opportunities\n\
             5. Suggested zero-copy rewrite\n\n\
             ```{}\n{}\n```",
            language, language, function_code
        );
        let response = self.query_engine("perf", &prompt, None).await?;
        Ok(json!({"profile": response}))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 4: SELF-IMPROVING CODE ENGINE
    // ═══════════════════════════════════════════════════════════════════════════

    /// Take generated code and iteratively improve it through multiple passes
    pub async fn self_improve(&self, code: &str, language: &str, iterations: u32) -> Result<Value, String> {
        let mut current_code = code.to_string();
        let mut improvement_log = Vec::new();
        let max_iterations = iterations.min(5); // Cap at 5 to avoid infinite loops

        for i in 0..max_iterations {
            let prompt = format!(
                "You are reviewing iteration {} of this {} code. \
                 Improve it for: correctness, security, performance, readability.\n\n\
                 Rules:\n\
                 - Each improvement must be explained\n\
                 - Return ONLY the improved code in a ```{} block\n\
                 - After the code, add a CHANGES section listing what you changed\n\n\
                 Current code:\n```{}\n{}\n```",
                i + 1, language, language, language, current_code
            );

            let response = self.query_engine("self_improve", &prompt, None).await?;
            
            // Extract code from response
            if let Some(improved) = self.extract_code_block(&response, language) {
                improvement_log.push(json!({
                    "iteration": i + 1,
                    "changes": response.split("CHANGES").nth(1).unwrap_or("See diff"),
                    "code_length_delta": improved.len() as i64 - current_code.len() as i64,
                }));
                current_code = improved;
            } else {
                break;
            }
        }

        Ok(json!({
            "original_code": code,
            "improved_code": current_code,
            "iterations_completed": improvement_log.len(),
            "improvement_log": improvement_log,
        }))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 5: EXPLAINABLE SECURITY LAYER
    // ═══════════════════════════════════════════════════════════════════════════

    /// Explain a security fix in plain English with CVE references
    pub async fn security_explain(&self, vulnerability: &str, fix_diff: &str) -> Result<Value, String> {
        let prompt = format!(
            "Explain this security fix to a developer in plain English.\n\n\
             Vulnerability: {}\n\n\
             Fix (diff):\n```diff\n{}\n```\n\n\
             Provide:\n\
             1. WHAT was the vulnerability (1-2 sentences)\n\
             2. WHY it's dangerous (real-world impact)\n\
             3. HOW the fix works (technical explanation)\n\
             4. Related CVEs (if any)\n\
             5. Prevention tips (how to avoid this in future)",
            vulnerability, fix_diff
        );
        let response = self.query_engine("explainer", &prompt, None).await?;
        Ok(json!({"explanation": response}))
    }

    /// Generate a security audit report with educational annotations
    pub async fn security_audit_explain(&self, code: &str, language: &str) -> Result<Value, String> {
        let prompt = format!(
            "Perform a security audit on this {} code. For each finding:\n\
             - Mark the exact line(s)\n\
             - Explain the vulnerability class (e.g., CWE-89 SQL Injection)\n\
             - Show a minimal exploit PoC\n\
             - Provide the fix with explanation\n\
             - Rate: CRITICAL/HIGH/MEDIUM/LOW\n\n\
             Be educational — explain WHY each issue matters.\n\n\
             ```{}\n{}\n```",
            language, language, code
        );
        let response = self.query_engine("explainer", &prompt, None).await?;
        Ok(json!({"audit": response}))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 6: MULTI-SYSTEM CONTROL
    // ═══════════════════════════════════════════════════════════════════════════

    /// Analyze multiple codebases for shared vulnerabilities
    pub async fn multi_system_scan(&self, systems: Vec<(String, String)>) -> Result<Value, String> {
        let mut summary = String::new();
        for (name, code_preview) in &systems {
            let preview: String = code_preview.chars().take(500).collect();
            summary.push_str(&format!("=== System: {} ===\n{}\n\n", name, preview));
        }

        let prompt = format!(
            "Analyze these {} systems for SHARED vulnerabilities and common attack surfaces.\n\n\
             {}\n\n\
             Identify:\n\
             1. Vulnerabilities present in MULTIPLE systems\n\
             2. Shared dependency risks\n\
             3. Cross-system attack chains (compromise A to reach B)\n\
             4. Unified remediation plan (fix all systems simultaneously)\n\
             5. Priority matrix (which system to fix first)",
            systems.len(), summary
        );
        let response = self.query_engine("multi_system", &prompt, None).await?;
        Ok(json!({
            "systems_analyzed": systems.len(),
            "analysis": response,
        }))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // ENGINE 7: FAILURE PREDICTION SYSTEM
    // ═══════════════════════════════════════════════════════════════════════════

    /// Predict system failures based on code analysis and runtime patterns
    pub async fn predict_failures(&self, code: &str, runtime_logs: Option<&str>) -> Result<Vec<FailurePrediction>, String> {
        let logs_section = runtime_logs
            .map(|l| format!("\n\nRuntime Logs (last 100 lines):\n```\n{}\n```", l))
            .unwrap_or_default();

        let prompt = format!(
            "Predict potential system failures for this code. Analyze:\n\
             1. Memory growth patterns → OOM prediction\n\
             2. Connection pool exhaustion\n\
             3. Deadlock conditions\n\
             4. Resource leak trajectories\n\
             5. Cascading failure chains\n\
             6. Load-based crash points\n\n\
             Code:\n```\n{}\n```{}\n\n\
             For each prediction, return JSON with:\n\
             component, failure_type, probability (0-1), time_horizon, \
             evidence (array), mitigation\n\n\
             Wrap in ```json blocks.",
            code, logs_section
        );

        let response = self.query_engine("predictor", &prompt, None).await?;
        
        if let Some(predictions) = self.parse_json_array::<FailurePrediction>(&response) {
            return Ok(predictions);
        }

        Ok(vec![FailurePrediction {
            component: "system".to_string(),
            failure_type: "general".to_string(),
            probability: 0.5,
            time_horizon: "unknown".to_string(),
            evidence: vec![response.chars().take(500).collect()],
            mitigation: "Review the full analysis output".to_string(),
        }])
    }

    /// Analyze server logs for anomaly patterns that predict imminent failure
    pub async fn predict_from_logs(&self, logs: &str) -> Result<Value, String> {
        let prompt = format!(
            "Analyze these server logs for patterns that predict imminent failure.\n\
             Look for:\n\
             - Memory growth rate → predict OOM time\n\
             - Error rate acceleration → predict service degradation\n\
             - Response time trend → predict timeout cascade\n\
             - Connection count trend → predict pool exhaustion\n\n\
             Logs:\n```\n{}\n```\n\n\
             Provide predictions with confidence scores and time estimates.",
            logs
        );
        let response = self.query_engine("predictor", &prompt, None).await?;
        Ok(json!({"predictions": response}))
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // UNIFIED RED TEAM + INTELLIGENCE SCAN
    // ═══════════════════════════════════════════════════════════════════════════

    /// Run a full APEX intelligence sweep — all engines in parallel
    pub async fn full_sweep(&self, code: &str, file_path: &str, language: &str) -> Result<Value, String> {
        let start = std::time::Instant::now();

        // Run engines concurrently
        let code_owned = code.to_string();
        let lang_owned = language.to_string();
        let file_owned = file_path.to_string();

        // Red team scan
        let rt = self.red_team.clone();
        let rt_code = code_owned.clone();
        let rt_lang = lang_owned.clone();
        let rt_file = file_owned.clone();
        let rt_handle = tokio::spawn(async move {
            rt.scan(RedTeamScanRequest {
                target_code: rt_code,
                file_path: rt_file,
                language: rt_lang,
                scan_depth: ScanDepth::Standard,
                focus_areas: vec![],
            }).await
        });

        // Perf optimization
        let perf_url = self.ollama_url.lock().await.clone();
        let perf_ws = self.workspace_root.lock().await.clone();
        let perf_cfg = self.config_dir.lock().await.clone();
        let perf_self = Self::new(&perf_url, perf_ws, perf_cfg);
        let perf_code = code_owned.clone();
        let perf_lang = lang_owned.clone();
        let perf_handle = tokio::spawn(async move {
            perf_self.perf_optimize(&perf_code, &perf_lang).await
        });

        // Failure prediction
        let pred_url = self.ollama_url.lock().await.clone();
        let pred_ws = self.workspace_root.lock().await.clone();
        let pred_cfg = self.config_dir.lock().await.clone();
        let pred_self = Self::new(&pred_url, pred_ws, pred_cfg);
        let pred_code = code_owned.clone();
        let pred_handle = tokio::spawn(async move {
            pred_self.predict_failures(&pred_code, None).await
        });

        // Collect results
        let rt_result = rt_handle.await.unwrap_or(Err("Red team scan failed".to_string()));
        let perf_result = perf_handle.await.unwrap_or(Err("Perf scan failed".to_string()));
        let pred_result = pred_handle.await.unwrap_or(Err("Prediction failed".to_string()));

        let elapsed = start.elapsed().as_millis();

        // Build JSON safely without double-borrowing Results
        let rt_json = match &rt_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let perf_json = match &perf_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let pred_json = match &pred_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };

        let result = json!({
            "sweep_id": uuid::Uuid::new_v4().to_string(),
            "file": file_path,
            "language": language,
            "duration_ms": elapsed,
            "red_team": rt_json,
            "performance": perf_json,
            "predictions": pred_json,
        });

        // Push to feed
        {
            let mut feed = self.results_feed.lock().await;
            let feed_entry = ApexResult {
                engine: "full_sweep".to_string(),
                result_type: "sweep".to_string(),
                data: result.clone(),
                severity: None,
                timestamp: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs(),
            };
            feed.push(feed_entry);
            // Keep feed bounded
            let len = feed.len();
            if len > 100 {
                feed.drain(..len - 100);
            }
        }

        Ok(result)
    }

    /// Get the results feed (latest intelligence findings)
    pub async fn get_results_feed(&self) -> Vec<ApexResult> {
        self.results_feed.lock().await.clone()
    }

    // ─── Internal Helper Methods ────────────────────────────────────────────

    /// Query a specific engine via Ollama
    async fn query_engine(&self, engine: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
        let url = self.ollama_url.lock().await.clone();
        let model = self.get_model(engine).await;

        let default_system = format!(
            "You are the {} engine of the APEX Intelligence Framework. \
             Provide precise, technical, actionable analysis.", 
            engine
        );

        let body = json!({
            "model": model,
            "prompt": prompt,
            "system": system.unwrap_or(&default_system),
            "stream": false,
            "options": {
                "temperature": 0.2,
                "num_ctx": 8192,
                "num_predict": 4096,
            }
        });

        println!("[APEX-{}] Querying {} with model {}...", engine.to_uppercase(), url, model);

        let response = self.client
            .post(format!("{}/api/generate", url))
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("[APEX-{}] Request failed: {}", engine, e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("[APEX-{}] HTTP {}: {}", engine, status, text));
        }

        let result: Value = response.json().await
            .map_err(|e| format!("[APEX-{}] Parse failed: {}", engine, e))?;

        result["response"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("[APEX-{}] No response field", engine))
    }

    /// Parse a JSON response from a ```json block
    fn parse_json_response<T: serde::de::DeserializeOwned>(&self, response: &str) -> Option<T> {
        // Try extracting from ```json block
        if let Some(start) = response.find("```json") {
            let after = &response[start + 7..];
            if let Some(end) = after.find("```") {
                let json_str = after[..end].trim();
                if let Ok(parsed) = serde_json::from_str::<T>(json_str) {
                    return Some(parsed);
                }
            }
        }
        // Try direct parse
        serde_json::from_str::<T>(response).ok()
    }

    /// Parse a JSON array from a response
    fn parse_json_array<T: serde::de::DeserializeOwned>(&self, response: &str) -> Option<Vec<T>> {
        // Try extracting from ```json block
        if let Some(start) = response.find("```json") {
            let after = &response[start + 7..];
            if let Some(end) = after.find("```") {
                let json_str = after[..end].trim();
                if let Ok(parsed) = serde_json::from_str::<Vec<T>>(json_str) {
                    return Some(parsed);
                }
            }
        }
        serde_json::from_str::<Vec<T>>(response).ok()
    }

    /// Extract a code block from a response
    fn extract_code_block(&self, response: &str, language: &str) -> Option<String> {
        let marker = format!("```{}", language);
        if let Some(start) = response.find(&marker) {
            let after = &response[start + marker.len()..];
            if let Some(end) = after.find("```") {
                return Some(after[..end].trim().to_string());
            }
        }
        // Try generic ``` block
        if let Some(start) = response.find("```\n") {
            let after = &response[start + 4..];
            if let Some(end) = after.find("```") {
                return Some(after[..end].trim().to_string());
            }
        }
        None
    }
}
