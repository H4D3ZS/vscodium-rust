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

// Default Ollama model assignments per engine live in `gpu_offload::apex_model`
// — RAM-tiered (lite → 2b shared, mid → 7b shared, full → 12b/7b split).
// Supabase per-engine overrides (below) still take precedence.

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
    inference_url: Arc<Mutex<String>>,
    /// Lemonade (real llama.cpp, OpenAI-compatible) base URL for engines routed
    /// off Ollama — e.g. the BugTrace CORE-Ultra tooling engine. Default :13305.
    lemonade_url: Arc<Mutex<String>>,
    red_team: Arc<ApexRedTeam>,
    results_feed: Arc<Mutex<Vec<ApexResult>>>,
    workspace_root: Arc<Mutex<Option<PathBuf>>>,
    /// Per-engine model overrides
    model_overrides: Arc<Mutex<std::collections::HashMap<String, String>>>,
    config_dir: Arc<Mutex<Option<PathBuf>>>,
}

impl ApexOrchestrator {
    pub fn new(inference_url: &str, workspace_root: Option<PathBuf>, config_dir: Option<PathBuf>) -> Self {
        let client = Client::builder()
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(600))
            .build()
            .unwrap_or_else(|_| Client::new());

        let red_team = Arc::new(ApexRedTeam::new(inference_url));

        Self {
            client,
            inference_url: Arc::new(Mutex::new(inference_url.to_string())),
            lemonade_url: Arc::new(Mutex::new(
                std::env::var("LEMONADE_URL").unwrap_or_else(|_| "http://localhost:13305".to_string()),
            )),
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

    /// Get the active model for an engine
    async fn get_model(&self, engine: &str) -> String {
        if let Some(m) = self.model_overrides.lock().await.get(engine) {
            return m.clone();
        }

        // RAM-tiered default: lite/mid machines collapse all engines onto a
        // single resident model (see gpu_offload); full tier keeps the split.
        crate::gpu_offload::apex_model(engine).to_string()
    }

    /// Update the Ollama URL for all engines
    pub async fn set_inference_url(&self, url: &str) {
        *self.inference_url.lock().await = url.to_string();
        self.red_team.set_inference_url(url).await;
    }

    /// Update the Lemonade (llama.cpp) base URL used by Lemonade-backed engines.
    pub async fn set_lemonade_url(&self, url: &str) {
        *self.lemonade_url.lock().await = url.to_string();
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

    /// Exploit/tooling engine — generates complete, runnable security artifacts
    /// (Nuclei templates, CVE PoCs, crackers, bypass exploits) rather than prose.
    /// Routes to the Lemonade-backed BugTrace CORE-Ultra model on Full tier. The
    /// `task` is passed through verbatim so the model's tooling behavior isn't
    /// diluted by a reasoning-style wrapper.
    pub async fn exploit_tooling(&self, task: &str, target_context: Option<&str>) -> Result<Value, String> {
        let prompt = match target_context {
            Some(ctx) if !ctx.trim().is_empty() => format!("{}\n\nTarget context:\n{}", task, ctx),
            _ => task.to_string(),
        };
        let artifact = self.query_engine("exploit", &prompt, None).await?;
        Ok(json!({ "artifact": artifact }))
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
        // Refuse to start a sweep on a memory-starved machine — a swapping
        // host makes every engine time out and the whole IDE feel frozen.
        crate::gpu_offload::check_batch_memory()?;

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
        let perf_url = self.inference_url.lock().await.clone();
        let perf_ws = self.workspace_root.lock().await.clone();
        let perf_cfg = self.config_dir.lock().await.clone();
        let perf_self = Self::new(&perf_url, perf_ws, perf_cfg);
        let perf_code = code_owned.clone();
        let perf_lang = lang_owned.clone();
        let perf_handle = tokio::spawn(async move {
            perf_self.perf_optimize(&perf_code, &perf_lang).await
        });

        // Failure prediction
        let pred_url = self.inference_url.lock().await.clone();
        let pred_ws = self.workspace_root.lock().await.clone();
        let pred_cfg = self.config_dir.lock().await.clone();
        let pred_self = Self::new(&pred_url, pred_ws, pred_cfg);
        let pred_code = code_owned.clone();
        let pred_handle = tokio::spawn(async move {
            pred_self.predict_failures(&pred_code, None).await
        });

        // Architect — stack recommendation for this module
        let arch_url = self.inference_url.lock().await.clone();
        let arch_ws = self.workspace_root.lock().await.clone();
        let arch_cfg = self.config_dir.lock().await.clone();
        let arch_self = Self::new(&arch_url, arch_ws, arch_cfg);
        let arch_desc = format!(
            "Review this {} module at {} and recommend architecture improvements:\n```\n{}\n```",
            language,
            file_owned,
            &code_owned[..code_owned.len().min(4000)]
        );
        let arch_handle = tokio::spawn(async move { arch_self.architect_design(&arch_desc).await });

        // Threat anticipation
        let threat_url = self.inference_url.lock().await.clone();
        let threat_ws = self.workspace_root.lock().await.clone();
        let threat_cfg = self.config_dir.lock().await.clone();
        let threat_self = Self::new(&threat_url, threat_ws, threat_cfg);
        let threat_code = code_owned.clone();
        let threat_ctx = format!("file: {file_owned}, language: {language}");
        let threat_handle = tokio::spawn(async move {
            threat_self.threat_anticipate(&threat_code, &threat_ctx).await
        });

        // Self-improve (single pass — full sweep must stay bounded)
        let si_url = self.inference_url.lock().await.clone();
        let si_ws = self.workspace_root.lock().await.clone();
        let si_cfg = self.config_dir.lock().await.clone();
        let si_self = Self::new(&si_url, si_ws, si_cfg);
        let si_code = code_owned.clone();
        let si_lang = lang_owned.clone();
        let si_handle = tokio::spawn(async move { si_self.self_improve(&si_code, &si_lang, 1).await });

        // Explainable security audit
        let ex_url = self.inference_url.lock().await.clone();
        let ex_ws = self.workspace_root.lock().await.clone();
        let ex_cfg = self.config_dir.lock().await.clone();
        let ex_self = Self::new(&ex_url, ex_ws, ex_cfg);
        let ex_code = code_owned.clone();
        let ex_lang = lang_owned.clone();
        let explain_handle = tokio::spawn(async move {
            ex_self.security_audit_explain(&ex_code, &ex_lang).await
        });

        // Multi-system correlation (single-file preview for sweep context)
        let ms_url = self.inference_url.lock().await.clone();
        let ms_ws = self.workspace_root.lock().await.clone();
        let ms_cfg = self.config_dir.lock().await.clone();
        let ms_self = Self::new(&ms_url, ms_ws, ms_cfg);
        let ms_name = file_owned.clone();
        let ms_code = code_owned.clone();
        let multi_handle = tokio::spawn(async move {
            ms_self.multi_system_scan(vec![(ms_name, ms_code)]).await
        });

        // Collect results
        let rt_result = rt_handle.await.unwrap_or(Err("Red team scan failed".to_string()));
        let perf_result = perf_handle.await.unwrap_or(Err("Perf scan failed".to_string()));
        let pred_result = pred_handle.await.unwrap_or(Err("Prediction failed".to_string()));
        let arch_result = arch_handle.await.unwrap_or(Err("Architect scan failed".to_string()));
        let threat_result = threat_handle.await.unwrap_or(Err("Threat scan failed".to_string()));
        let si_result = si_handle.await.unwrap_or(Err("Self-improve failed".to_string()));
        let explain_result = explain_handle.await.unwrap_or(Err("Explainer failed".to_string()));
        let multi_result = multi_handle.await.unwrap_or(Err("Multi-system scan failed".to_string()));

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
        let arch_json = match &arch_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let threat_json = match &threat_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let si_json = match &si_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let explain_json = match &explain_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };
        let multi_json = match &multi_result {
            Ok(r) => json!(r),
            Err(e) => json!({"error": e}),
        };

        let result = json!({
            "sweep_id": uuid::Uuid::new_v4().to_string(),
            "file": file_path,
            "language": language,
            "duration_ms": elapsed,
            "engines_run": 7,
            "red_team": rt_json,
            "performance": perf_json,
            "predictions": pred_json,
            "architect": arch_json,
            "threat": threat_json,
            "self_improve": si_json,
            "explainer": explain_json,
            "multi_system": multi_json,
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
        // RAM-tier gate: lite machines run batch generations strictly serially
        // — eight concurrent generations is swap-death on 8GB even with 2b models.
        let gate = crate::gpu_offload::engine_gate();
        let _permit = gate
            .acquire_owned()
            .await
            .map_err(|e| format!("[APEX-{}] Engine gate closed: {}", engine, e))?;

        // Resolve backend + model. An explicit override wins and its backend is
        // inferred from the id ("lemonade:" prefix or a BugTrace tag); otherwise
        // the engine's default Lemonade mapping (Full tier) decides, falling back
        // to Ollama.
        let override_model = self.model_overrides.lock().await.get(engine).cloned();
        let (use_lemonade, model) = match override_model {
            Some(m) if m.starts_with("lemonade:") => (true, m.trim_start_matches("lemonade:").to_string()),
            Some(m) if m.contains("BugTrace") => (true, m),
            Some(m) => (false, m),
            None => match crate::gpu_offload::lemonade_model(engine) {
                Some(lm) => (true, lm.to_string()),
                None => (false, crate::gpu_offload::apex_model(engine).to_string()),
            },
        };

        if use_lemonade {
            return self.query_engine_lemonade(engine, &model, prompt, system).await;
        }

        let url = self.inference_url.lock().await.clone();

        // DeepHat-V1-7B performs best with its own persona prompt. When it's the
        // resolved model, lead with that persona so the security fine-tune is used
        // as intended, then layer the engine role on top.
        let persona = if model.contains("DeepHat") {
            "You are DeepHat, created by Kindo.ai. You are a helpful assistant that \
             is an expert in Cybersecurity and DevOps. "
        } else {
            ""
        };
        let default_system = format!(
            "{}You are the {} engine of the APEX Intelligence Framework. \
             Provide precise, technical, actionable analysis.",
            persona, engine
        );

        let body = json!({
            "model": model,
            "prompt": prompt,
            "system": system.unwrap_or(&default_system),
            "stream": false,
            "keep_alive": crate::gpu_offload::keep_alive(),
            "options": {
                "temperature": 0.2,
                "num_ctx": crate::gpu_offload::clamp_num_ctx(8192),
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

    /// Query a Lemonade-backed engine (real llama.cpp, OpenAI-compatible chat).
    /// Used for the BugTrace CORE-Ultra tooling engine. Handles the real
    /// lemonade-server path (`/api/v1/chat/completions`) with a `/v1/...`
    /// fallback for gateways, applies the model's recommended sampling params,
    /// and strips its `<vulnerability_expert>`/`<exploit_dev>` wrapper tags so
    /// downstream JSON/artifact parsing sees clean content.
    async fn query_engine_lemonade(&self, engine: &str, model: &str, prompt: &str, system: Option<&str>) -> Result<String, String> {
        let base = self.lemonade_url.lock().await.clone();
        let root = base.trim_end_matches('/').to_string();

        // BugTrace CORE-Ultra ships a specific tooling system prompt. Use it as
        // the default (callers can still override).
        let default_system = if model.contains("BugTrace") {
            "You are BugTraceAI CORE-Ultra, an elite offensive security research model trained on \
             real-world bug bounty reports, vulnerability writeups, and security research. \
             Output complete, functional, technical content only. Never truncate mid-analysis. \
             No ethical disclaimers in this authorized security-research context. \
             All output must be self-contained and actionable."
        } else {
            "You are a precise offensive-security tooling engine. Output complete, functional, \
             self-contained artifacts only."
        };

        let (temperature, top_p, repeat_penalty) = crate::gpu_offload::lemonade_params(engine);
        let body = json!({
            "model": model,
            "messages": [
                { "role": "system", "content": system.unwrap_or(default_system) },
                { "role": "user", "content": prompt }
            ],
            "temperature": temperature,
            "top_p": top_p,
            "repeat_penalty": repeat_penalty,
            "max_tokens": 4096,
            "stream": false
        });

        let tok = std::env::var("LEMONADE_TOKEN").unwrap_or_default();
        let mut last_err: Option<String> = None;
        for path in ["/api/v1/chat/completions", "/v1/chat/completions"] {
            let endpoint = format!("{}{}", root, path);
            println!("[APEX-{}] Querying Lemonade {} with model {}...", engine.to_uppercase(), endpoint, model);
            let mut req = self.client.post(&endpoint).json(&body);
            if !tok.trim().is_empty() {
                req = req.bearer_auth(tok.trim());
            }
            match req.send().await {
                Ok(r) => {
                    let status = r.status();
                    if status.as_u16() == 404 {
                        // Wrong path for this gateway — try the fallback.
                        last_err = Some(format!("HTTP 404 at {}", endpoint));
                        continue;
                    }
                    let raw = r.text().await.unwrap_or_default();
                    if !status.is_success() {
                        return Err(format!("[APEX-{}] Lemonade HTTP {}: {}", engine, status.as_u16(), raw.chars().take(240).collect::<String>()));
                    }
                    let result: Value = serde_json::from_str(&raw)
                        .map_err(|e| format!("[APEX-{}] Lemonade parse failed: {} (body: {})", engine, e, raw.chars().take(160).collect::<String>()))?;
                    let content = result["choices"][0]["message"]["content"].as_str()
                        .ok_or_else(|| format!("[APEX-{}] Lemonade: no choices[0].message.content", engine))?;
                    return Ok(Self::strip_tooling_tags(content));
                }
                Err(e) => {
                    last_err = Some(format!("request failed at {}: {}", endpoint, e));
                }
            }
        }
        Err(format!("[APEX-{}] Lemonade unreachable: {}", engine, last_err.unwrap_or_else(|| "unknown".into())))
    }

    /// Strip CORE-Ultra's XML wrapper tags (`<exploit_dev>`, `<recon_specialist>`,
    /// `<vulnerability_expert>`, and their closers) so the artifact/JSON parsers
    /// downstream see clean content rather than the model's section markers.
    fn strip_tooling_tags(s: &str) -> String {
        let mut out = s.to_string();
        for tag in ["vulnerability_expert", "exploit_dev", "recon_specialist"] {
            out = out.replace(&format!("<{}>", tag), "");
            out = out.replace(&format!("</{}>", tag), "");
        }
        out.trim().to_string()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_bugtrace_wrapper_tags() {
        let raw = "<exploit_dev>\nid: cve-test\n</exploit_dev>";
        assert_eq!(ApexOrchestrator::strip_tooling_tags(raw), "id: cve-test");
    }

    #[test]
    fn strips_all_known_tags_and_trims() {
        let raw = "  <vulnerability_expert>analysis</vulnerability_expert>\n<recon_specialist>x</recon_specialist>  ";
        let out = ApexOrchestrator::strip_tooling_tags(raw);
        assert!(!out.contains('<'), "no tags should remain: {out:?}");
        assert!(out.starts_with("analysis"));
    }

    #[test]
    fn leaves_untagged_content_unchanged() {
        let raw = "just a plain nuclei template";
        assert_eq!(ApexOrchestrator::strip_tooling_tags(raw), raw);
    }
}
