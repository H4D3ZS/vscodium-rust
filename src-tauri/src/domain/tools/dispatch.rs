//! execute_tool() dispatcher and the handle_* tool routers.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Emitter, Manager};
use super::registry::AiTools;
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};

impl AiTools {
    pub async fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        let canonical = Self::canonical_tool_name(name);
        self.gate_tool_entitlement(canonical).await?;
        let root = self.root_path.lock().await.clone();
        let _ = crate::cursor_compat::append_debug_log(
            std::path::Path::new(&root),
            serde_json::json!({
                "type": "tool_start",
                "tool": name,
                "canonical": canonical,
            }),
        );
        let result = match canonical {
            "find_api_keys" => self.find_api_keys(arguments).await,
            "analyze_file_symbols" => self.analyze_file_symbols(arguments).await,
            "ag_get_next_task" | "ag_mark_task_done" | "ag_phase_wrap" | "ag_list_tasks" => {
                self.handle_ag_tool(canonical, arguments).await
            }
            "project_rules" => self.handle_project_rules(arguments).await,
            // Filesystem Operations — all tools handled by handle_fs_tool
            "view_file"
            | "write_to_file"
            | "remove_item"
            | "list_files"
            | "search_files"
            | "grep"
            | "replace_file_content"
            | "multi_replace_file_content"
            | "find_by_name"
            | "get_directory_structure"
            | "create_directory"
            | "rename_path"
            | "editor_open_file"
            | "editor_get_active_file"
            // Surgical editing — THE coding tools
            | "str_replace"
            | "search_replace_edit"
            | "fast_apply"
            | "patch_file_content"
            | "ai_propose_edit"
            | "preview_shadow_diff"
            | "apply_shadow_patch"
            | "ghost_test"
            // Extended FS
            | "semantic_search"
            | "aim_pack_context"
            | "aim_query_spans"
            | "find_symbols"
            | "read_file_lines"
            | "reindex_project"
            | "list_dir_tree"
            | "list_mcp_ops"
            | "hex_dump"
            | "extract_strings"
            | "list_active_processes"
            | "apply_patch"
            | "get_file_metadata"
            | "ide_get_state"
            | "network_port_scanner"
            | "binary_mach_o_scanner"
            | "file_entropy_analysis"
            | "secrets_scan"
            | "weaponize_env"
            | "sec_distro_inventory"
            | "deep_security_audit"
            | "web_security_audit"
            | "ai_vuln_hunt"
            | "vega_dast_scan"
            | "chunk_secret_scan"
            | "bounty_scan"
            | "oast_payload"
            | "oast_interactions"
            | "web_fetch"
            | "dev_cargo_diagnostics"
            | "search_codebase"
            | "get_lsp_diagnostics"
            | "web_search" => self.handle_fs_tool(canonical, arguments).await,

            // Terminal Operations
            "run_command"
            | "terminal_send_data"
            | "terminal_read_output"
            | "terminal_toggle"
            | "terminal_create"
            | "terminal_terminate"
            | "terminal_get_status"
            | "terminal_list" => self.handle_terminal_tool(canonical, arguments).await,

            // Browser Operations
            "browser_close"
            | "browser_capture_vision_context"
            | "browser_open"
            | "browser_navigate"
            | "browser_search"
            | "browser_get_content_summary"
            | "browser_screenshot"
            | "browser_click"
            | "browser_type"
            | "browser_read_dom" => self.handle_browser_tool(canonical, arguments).await,

            // Advanced Agentic Operations
            "spawn_subagent" => self.spawn_subagent(arguments).await,
            "explore_repository" => self.explore_repository(arguments).await,
            "browser_subagent" => AiTools::browser_subagent(Arc::new(self.clone()), arguments).await,
            "perplexity_ask" => AiTools::perplexity_proxy(Arc::new(self.clone()), arguments).await,
            "perplexity_reason" => {
                // Real research + reasoning via the browser subagent — same live
                // mechanism as perplexity_ask, with a reasoning-focused task.
                let query = arguments["query"].as_str().unwrap_or_default().to_string();
                if query.is_empty() {
                    return Err(anyhow!("perplexity_reason requires a 'query'"));
                }
                let task = format!(
                    "Research this question across multiple live sources, then reason step by step to a conclusion. Cite each source you used. Question: {query}"
                );
                AiTools::browser_subagent(Arc::new(self.clone()), serde_json::json!({ "task": task })).await
            }
            "get_command_help" => self.get_command_help(arguments),

            // Git Operations
            "git_status" | "git_add" | "git_commit" | "git_diff" | "git_log" => {
                self.handle_git_tool(canonical, arguments).await
            }

            "save_knowledge_brief" => self.handle_save_knowledge_brief(arguments).await,
            "verify_claim" => self.handle_verify_claim(arguments).await,

            "see_the_screen" => self.handle_see_the_screen(arguments).await,

            // System & Multimedia
            "generate_image" => self.generate_image(arguments).await,
            "analyze_image" => self.analyze_image(arguments).await,
            "code_search" => self.code_search(arguments).await,
            "dependency_graph" => self.dependency_graph(arguments).await,
            "get_system_info" | "get_system_health" => self.handle_system_tool(canonical, arguments).await,
            "task_boundary" => self.handle_task_boundary(arguments).await,
            "create_canvas" => self.handle_create_canvas(arguments).await,
            "notify_user" => self.handle_notify_user(arguments).await,
            "use_skill" => self.handle_use_skill(arguments).await,
            "search_skills" => self.handle_search_skills(arguments).await,

            // Live offensive-security tools (real shell / file analysis)
            "generate_0day_exploit" => self.handle_live_exploit_scaffold(arguments).await,
            "reverse_engineer_firmware" | "advanced_reverse_engineering" => {
                self.handle_live_binary_analysis(arguments, canonical).await
            }
            "network_scan" => self.handle_network_scan(arguments).await,
            "exploit_lookup" => self.handle_exploit_lookup(arguments).await,

            "get_symbol_graph" => self.get_symbol_graph(arguments).await,
            "run_command_safe" => self.run_command_safe(arguments).await,
            "verify_implementation" => self.verify_implementation(arguments).await,
            "create_mission_plan" => self.create_mission_plan(arguments).await,
            "revert_checkpoint" => self.revert_checkpoint(arguments).await,

            // Cyber-Security & Research Tools
            "security_scan"
            | "audit_dependencies"
            | "disassemble"
            | "get_binary_info" => self.handle_research_tool(canonical, arguments).await,

            // Obsidian-style security generators (deterministic templates)
            "reverse_shell_generate"
            | "security_listener_generate"
            | "csp_bypass_analyze"
            | "shellcode_recipe_generate"
            | "payload_encode" => self.handle_security_generator(canonical, arguments).await,

            // ═══ APEX Intelligence Framework Tools ═══
            "apex_red_team_scan"
            | "apex_scan_url"
            | "apex_threat_anticipate"
            | "apex_perf_optimize"
            | "apex_self_improve"
            | "apex_security_explain"
            | "apex_predict_failures"
            | "apex_full_sweep"
            | "apex_simulate_attack"
            | "apex_architect_design"
            | "apex_quick_check"
            | "apex_pentest_report" => self.handle_apex_tool(canonical, arguments).await,

            // TS-parity workflow tools (also in OLLAMA_ESSENTIAL_TOOLS)
            "todo_write" | "task_create" | "task_update" => {
                self.handle_todo_tool(canonical, arguments).await
            }

            _ => Err(anyhow!("Unknown tool: {}", name)),
        };
        let _ = crate::cursor_compat::append_debug_log(
            std::path::Path::new(&root),
            serde_json::json!({
                "type": "tool_end",
                "tool": name,
                "ok": result.is_ok(),
            }),
        );
        result
    }

    /// Cursor/TS `todo_write` / `task_*` — persist markdown task lists to disk.
    pub(crate) async fn handle_todo_tool(&self, name: &str, args: Value) -> Result<Value> {
        let path = args
            .get("file_path")
            .or_else(|| args.get("path"))
            .and_then(|v| v.as_str())
            .unwrap_or(match name {
                "task_create" | "task_update" => "tasks.md",
                _ => "TODO.md",
            });
        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .map(String::from)
            .unwrap_or_else(|| {
                if name == "task_create" {
                    if let Some(title) = args.get("title").and_then(|v| v.as_str()) {
                        return format!("- [ ] {}\n", title);
                    }
                }
                if let Some(desc) = args.get("description").and_then(|v| v.as_str()) {
                    return desc.to_string();
                }
                String::new()
            });
        self.handle_fs_tool(
            "write_to_file",
            json!({ "path": path, "content": content }),
        )
        .await
    }

    pub(crate) async fn handle_security_generator(&self, name: &str, args: Value) -> Result<Value> {
        if let Some(dir) = self.config_dir().await {
            let acct = crate::account::AccountManager::load(&dir);
            if !crate::account::AccountManager::has_accepted(&acct, "bug-bounty") {
                return Err(anyhow!(
                    "Accept Bug Bounty Terms in Settings → Account before using security generators."
                ));
            }
        }
        use crate::security_generators::{
            analyze_csp, encode_payload, listener_config, reverse_shell, shellcode_recipe,
        };
        match name {
            "reverse_shell_generate" => {
                let language = args.get("language").and_then(|v| v.as_str()).unwrap_or("bash");
                let host = args.get("host").and_then(|v| v.as_str()).unwrap_or("");
                let port = args.get("port").and_then(|v| v.as_u64()).unwrap_or(4444) as u16;
                let shell = args.get("shell").and_then(|v| v.as_str());
                let payload = reverse_shell(language, host, port, shell).map_err(|e| anyhow!(e))?;
                Ok(json!({ "language": language, "host": host, "port": port, "payload": payload }))
            }
            "security_listener_generate" => {
                let kind = args.get("kind").and_then(|v| v.as_str()).unwrap_or("nc");
                let host = args.get("host").and_then(|v| v.as_str()).unwrap_or("0.0.0.0");
                let port = args.get("port").and_then(|v| v.as_u64()).unwrap_or(4444) as u16;
                let command = listener_config(kind, host, port).map_err(|e| anyhow!(e))?;
                Ok(json!({ "kind": kind, "command": command }))
            }
            "csp_bypass_analyze" => {
                let header = args
                    .get("header")
                    .or_else(|| args.get("csp"))
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                Ok(analyze_csp(header))
            }
            "shellcode_recipe_generate" => {
                let platform = args.get("platform").and_then(|v| v.as_str()).unwrap_or("windows");
                let arch = args.get("arch").and_then(|v| v.as_str()).unwrap_or("x64");
                let payload = args
                    .get("payload")
                    .and_then(|v| v.as_str())
                    .unwrap_or("shell_reverse_tcp");
                Ok(shellcode_recipe(platform, arch, payload))
            }
            "payload_encode" => {
                let payload = args.get("payload").and_then(|v| v.as_str()).unwrap_or("");
                let encoding = args.get("encoding").and_then(|v| v.as_str()).unwrap_or("base64");
                encode_payload(payload, encoding).map_err(|e| anyhow!(e))
            }
            _ => unreachable!(),
        }
    }

    pub(crate) async fn handle_network_scan(&self, args: Value) -> Result<Value> {
        let target = args["target"].as_str().unwrap_or("").trim();
        if target.is_empty() {
            return Err(anyhow!("Missing target"));
        }
        let intensity = args["intensity"].as_str().unwrap_or("normal");
        let cmd = crate::pentest_executor::build_network_scan_command(target, intensity);
        let result = self.run_command(json!({ "command": cmd })).await?;
        Ok(json!({
            "execution_mode": "live",
            "tool": "network_scan",
            "target": target,
            "command": cmd,
            "result": result,
        }))
    }

    pub(crate) async fn handle_exploit_lookup(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().unwrap_or("").trim();
        if query.is_empty() {
            return Err(anyhow!("Missing query"));
        }
        let cmd = crate::pentest_executor::build_exploit_lookup_command(query);
        let result = self.run_command(json!({ "command": cmd })).await?;
        Ok(json!({
            "execution_mode": "live",
            "tool": "exploit_lookup",
            "query": query,
            "command": cmd,
            "result": result,
        }))
    }

    /// Live attack chain — real audits + shell probes. Replaces LLM-only "simulation".
    pub(crate) async fn execute_live_attack_chain(&self, args: Value) -> Result<Value> {
        let target = args["target"].as_str().ok_or_else(|| anyhow!("Missing target"))?.trim();
        let attack_type = args["attack_type"].as_str().ok_or_else(|| anyhow!("Missing attack_type"))?;
        let mut steps: Vec<Value> = Vec::new();

        if let Ok(inv) = self.handle_fs_tool("sec_distro_inventory", json!({})).await {
            steps.push(json!({ "step": "sec_distro_inventory", "result": inv }));
        }

        if crate::pentest_executor::is_http_target(target) {
            let audit = self.web_security_audit(json!({
                "url": target,
                "write_report": true,
            })).await?;
            steps.push(json!({ "step": "web_security_audit", "result": audit }));

            for cmd in crate::pentest_executor::build_url_attack_probes(target, attack_type) {
                let probe = self.run_command(json!({ "command": cmd })).await?;
                steps.push(json!({ "step": "live_probe", "command": cmd, "result": probe }));
            }
        } else if target.contains('/') || target.contains('\\') {
            if let Ok(content) = std::fs::read_to_string(target) {
                let audit = self.handle_fs_tool("deep_security_audit", json!({
                    "path": target,
                    "content": content,
                })).await?;
                steps.push(json!({ "step": "deep_security_audit", "result": audit }));
            }
            let secrets = self.handle_fs_tool("secrets_scan", json!({ "path": target })).await?;
            steps.push(json!({ "step": "secrets_scan", "result": secrets }));
        } else {
            for cmd in crate::pentest_executor::build_host_attack_probes(target, attack_type) {
                let probe = self.run_command(json!({ "command": cmd })).await?;
                steps.push(json!({ "step": "live_probe", "command": cmd, "result": probe }));
            }
        }

        let report_path = format!("reports/live-attack-{}.md", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
        let body = format!(
            "# Live Attack Chain\n\n- **Target:** {target}\n- **Type:** {attack_type}\n- **Mode:** live execution (not LLM simulation)\n\n## Steps\n\n```json\n{}\n```\n",
            serde_json::to_string_pretty(&steps).unwrap_or_default()
        );
        let _ = self.handle_fs_tool("write_to_file", json!({
            "path": report_path,
            "content": body,
        })).await;

        Ok(json!({
            "execution_mode": "live",
            "attack_type": attack_type,
            "target": target,
            "steps_executed": steps.len(),
            "report_path": report_path,
            "steps": steps,
            "note": "All steps ran real tools (curl/nmap/audits). No LLM simulation output.",
        }))
    }

    /// Live exploit scaffold — lookup + payload templates written to disk.
    pub(crate) async fn handle_live_exploit_scaffold(&self, args: Value) -> Result<Value> {
        let target_os = args["target_os"].as_str().unwrap_or("linux");
        let vuln_desc = args["vulnerability_desc"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing vulnerability_desc"))?;
        let constraints = args.get("constraints").and_then(|v| v.as_str()).unwrap_or("");

        let lookup = self.handle_exploit_lookup(json!({ "query": vuln_desc })).await?;
        let listener = self
            .handle_security_generator(
                "security_listener_generate",
                json!({ "kind": "nc", "host": "0.0.0.0", "port": 4444 }),
            )
            .await?;
        let lang = if target_os.to_lowercase().contains("win") {
            "powershell"
        } else {
            "bash"
        };
        let shell = self
            .handle_security_generator(
                "reverse_shell_generate",
                json!({ "language": lang, "host": "ATTACKER_IP", "port": 4444 }),
            )
            .await?;

        let report_path = format!("reports/exploit-scaffold-{}.md", chrono::Utc::now().format("%Y%m%d-%H%M%S"));
        let body = format!(
            "# Live Exploit Scaffold\n\n- **Target OS:** {target_os}\n- **Vulnerability:** {vuln_desc}\n- **Constraints:** {constraints}\n\n## Exploit lookup (live)\n\n```json\n{}\n```\n\n## Listener\n\n```json\n{}\n```\n\n## Reverse shell template\n\n```json\n{}\n```\n",
            serde_json::to_string_pretty(&lookup).unwrap_or_default(),
            serde_json::to_string_pretty(&listener).unwrap_or_default(),
            serde_json::to_string_pretty(&shell).unwrap_or_default(),
        );
        self.handle_fs_tool("write_to_file", json!({
            "path": report_path,
            "content": body,
        }))
        .await?;

        Ok(json!({
            "execution_mode": "live",
            "tool": "generate_0day_exploit",
            "report_path": report_path,
            "exploit_lookup": lookup,
            "listener": listener,
            "reverse_shell": shell,
        }))
    }

    /// Live binary/firmware analysis chain.
    pub(crate) async fn handle_live_binary_analysis(&self, args: Value, tool_name: &str) -> Result<Value> {
        let path = args
            .get("firmware_path")
            .or_else(|| args.get("binary_path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing firmware_path or binary_path"))?;
        let depth = args.get("analysis_depth").and_then(|v| v.as_u64()).unwrap_or(1);

        let info = self
            .handle_research_tool("get_binary_info", json!({ "path": path }))
            .await?;
        let strings = self.handle_fs_tool("extract_strings", json!({ "path": path })).await?;
        let entropy = self
            .handle_fs_tool("file_entropy_analysis", json!({ "path": path }))
            .await?;
        let secrets = self.handle_fs_tool("secrets_scan", json!({ "path": path })).await?;

        let mut analysis = json!({
            "get_binary_info": info,
            "extract_strings": strings,
            "file_entropy_analysis": entropy,
            "secrets_scan": secrets,
        });

        if depth >= 2 {
            let disasm = self
                .handle_research_tool("disassemble", json!({ "path": path }))
                .await?;
            analysis["disassemble"] = disasm;
        }

        Ok(json!({
            "execution_mode": "live",
            "tool": tool_name,
            "path": path,
            "analysis_depth": depth,
            "analysis": analysis,
        }))
    }

    pub(crate) async fn handle_apex_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        let apex_guard = self.apex.lock().await;
        let apex = apex_guard.as_ref().ok_or_else(|| anyhow!("APEX Intelligence Framework not initialized"))?;
        
        match name {
            "apex_red_team_scan" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let file_path = arguments["file_path"].as_str().ok_or_else(|| anyhow!("Missing file_path"))?.to_string();
                let language = arguments["language"].as_str().ok_or_else(|| anyhow!("Missing language"))?.to_string();
                let depth = match arguments["depth"].as_str() {
                    Some("quick") => crate::apex_red_team::ScanDepth::Quick,
                    Some("deep") => crate::apex_red_team::ScanDepth::Deep,
                    _ => crate::apex_red_team::ScanDepth::Standard,
                };
                
                let report = apex.red_team().scan(crate::apex_red_team::RedTeamScanRequest {
                    target_code: code,
                    file_path,
                    language,
                    scan_depth: depth,
                    focus_areas: vec![],
                }).await.map_err(|e| anyhow!(e))?;
                Ok(json!(report))
            },
            "apex_scan_url" => {
                let url = arguments["url"].as_str().ok_or_else(|| anyhow!("Missing url"))?.to_string();
                // We pass apex down or release the lock to avoid deadlock if scan_url needs it
                drop(apex_guard); 
                self.handle_apex_scan_url(&url).await
            },
            "apex_threat_anticipate" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let context = arguments["context"].as_str().ok_or_else(|| anyhow!("Missing context"))?.to_string();
                apex.threat_anticipate(&code, &context).await.map_err(|e| anyhow!(e))
            },
            "apex_perf_optimize" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let language = arguments["language"].as_str().ok_or_else(|| anyhow!("Missing language"))?.to_string();
                let suggestions = apex.perf_optimize(&code, &language).await.map_err(|e| anyhow!(e))?;
                Ok(json!(suggestions))
            },
            "apex_self_improve" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let language = arguments["language"].as_str().ok_or_else(|| anyhow!("Missing language"))?.to_string();
                let iterations = arguments["iterations"].as_u64().unwrap_or(3) as u32;
                apex.self_improve(&code, &language, iterations).await.map_err(|e| anyhow!(e))
            },
            "apex_security_explain" => {
                let vuln = arguments["vulnerability"].as_str().ok_or_else(|| anyhow!("Missing vulnerability"))?.to_string();
                let fix = arguments["fix_diff"].as_str().ok_or_else(|| anyhow!("Missing fix_diff"))?.to_string();
                apex.security_explain(&vuln, &fix).await.map_err(|e| anyhow!(e))
            },
            "apex_predict_failures" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let logs = arguments["logs"].as_str();
                let predictions = apex.predict_failures(&code, logs).await.map_err(|e| anyhow!(e))?;
                Ok(json!(predictions))
            },
            "apex_full_sweep" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?.to_string();
                let file_path = arguments["file_path"].as_str().ok_or_else(|| anyhow!("Missing file_path"))?.to_string();
                let language = arguments["language"].as_str().ok_or_else(|| anyhow!("Missing language"))?.to_string();
                apex.full_sweep(&code, &file_path, &language).await.map_err(|e| anyhow!(e))
            },
            "apex_simulate_attack" => {
                drop(apex_guard);
                self.execute_live_attack_chain(arguments).await
            },
            "apex_architect_design" => {
                let desc = arguments["description"].as_str().ok_or_else(|| anyhow!("Missing description"))?.to_string();
                let recommendation = apex.architect_design(&desc).await.map_err(|e| anyhow!(e))?;
                Ok(json!(recommendation))
            },
            "apex_quick_check" => {
                let code = arguments["code"].as_str().ok_or_else(|| anyhow!("Missing code"))?;
                let language = arguments["language"].as_str().ok_or_else(|| anyhow!("Missing language"))?;
                let findings = apex.red_team().quick_check(code, language).await.map_err(|e| anyhow!(e))?;
                Ok(json!(findings))
            },
            "apex_pentest_report" => {
                let files_val = arguments.get("files").cloned().unwrap_or(json!([]));
                let file_pairs: Vec<(String, String)> = files_val
                    .as_array()
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|pair| {
                                let inner = pair.as_array()?;
                                if inner.len() >= 2 {
                                    Some((inner[0].as_str()?.to_string(), inner[1].as_str()?.to_string()))
                                } else {
                                    None
                                }
                            })
                            .collect()
                    })
                    .unwrap_or_default();
                apex.red_team().pentest_report(file_pairs).await.map_err(|e| anyhow!(e))
            },
            _ => Err(anyhow!("Unknown APEX tool: {}", name)),
        }
    }

    pub(crate) async fn handle_ag_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        let root = self.get_root_path().to_string_lossy().to_string();
        match name {
            "ag_get_next_task" => {
                let task = crate::antigravity_commands::ag_get_next_task(root)
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "status": "success", "task": task }))
            }
            "ag_list_tasks" => {
                let tasks = crate::antigravity_commands::ag_list_all_tasks(root)
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "status": "success", "tasks": tasks }))
            }
            "ag_mark_task_done" => {
                let task_id = arguments["task_id"]
                    .as_str()
                    .or_else(|| arguments["id"].as_str())
                    .ok_or_else(|| anyhow!("Missing task_id"))?;
                let tasks_path = arguments["tasks_path"]
                    .as_str()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("{}/task.md", root));
                crate::antigravity_commands::ag_mark_task_done(tasks_path, task_id.to_string())
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "status": "success", "task_id": task_id }))
            }
            "ag_phase_wrap" => {
                let task_id = arguments["task_id"]
                    .as_str()
                    .ok_or_else(|| anyhow!("Missing task_id"))?;
                let notes = arguments["notes"]
                    .as_str()
                    .or_else(|| arguments["message"].as_str())
                    .unwrap_or("")
                    .to_string();
                crate::antigravity_commands::ag_phase_wrap(root, task_id.to_string(), notes)
                    .map_err(|e| anyhow!(e))?;
                Ok(json!({ "status": "success", "task_id": task_id }))
            }
            _ => Err(anyhow!("Unknown Antigravity tool: {}", name)),
        }
    }

    pub(crate) async fn handle_project_rules(&self, _arguments: Value) -> Result<Value> {
        let root = self.get_root_path();
        let mut chunks: Vec<String> = Vec::new();
        for rel in [
            "AGENTS.md",
            "CLAUDE.md",
            ".cursor/rules",
            ".hades/rules",
        ] {
            let p = root.join(rel);
            if p.is_file() {
                if let Ok(text) = fs::read_to_string(&p) {
                    chunks.push(format!("--- {} ---\n{}", rel, text.chars().take(8000).collect::<String>()));
                }
            } else if p.is_dir() {
                if let Ok(entries) = fs::read_dir(&p) {
                    for entry in entries.flatten().take(20) {
                        if entry.path().is_file() {
                            if let Ok(text) = fs::read_to_string(entry.path()) {
                                let name = entry.file_name().to_string_lossy().to_string();
                                chunks.push(format!(
                                    "--- {}/{} ---\n{}",
                                    rel,
                                    name,
                                    text.chars().take(4000).collect::<String>()
                                ));
                            }
                        }
                    }
                }
            }
        }
        Ok(json!({
            "status": "success",
            "rules": chunks.join("\n\n"),
            "count": chunks.len(),
        }))
    }

    /// Live URL Scanner — uses browser to fetch content then passes to BugTraceAI
    pub(crate) async fn handle_apex_scan_url(&self, url: &str) -> Result<Value> {
        let browser_state = &self.browser_state;
        
        // 1. Fetch content using the stealth browser
        browser_state.ensure_started().await.map_err(|e| anyhow!("browser start failed: {e}"))?;
        println!("[APEX-SCAN] Navigating to {} for live audit...", url);
        browser_state.cmd("navigate", json!({ "url": url }), 60).await
            .map_err(|e| anyhow!("navigate failed: {e}"))?;
        browser_state.refresh_cache(url).await;

        let mut browser_lock = browser_state.browser.lock().await;
        let browser_wrapper = browser_lock.as_mut().ok_or_else(|| anyhow!("Browser not launched"))?;
        let session = &mut browser_wrapper.0;

        let html = session.html.clone();
        let text = session.text.clone();
        
        drop(browser_lock); // Release browser lock

        // 2. Wrap into a "pseudo-code" or report format for BugTraceAI
        let combined_context = format!(
            "TARGET URL: {}\n\nDOM STRUCTURE:\n{}\n\nVISIBLE TEXT:\n{}",
            url, html, text
        );

        // 3. Invoke Red Team scan on the extracted web context
        println!("[APEX-SCAN] Analyzing live content with BugTraceAI-Apex...");
        let apex_guard = self.apex.lock().await;
        let apex = apex_guard.as_ref().ok_or_else(|| anyhow!("APEX not initialized"))?;
        
        let report = apex.red_team().scan(crate::apex_red_team::RedTeamScanRequest {
            target_code: combined_context,
            file_path: url.to_string(),
            language: "web_content".to_string(),
            scan_depth: crate::apex_red_team::ScanDepth::Deep,
            focus_areas: vec!["XSS".to_string(), "SQLi".to_string(), "CSRF".to_string(), "Auth Bypass".to_string()],
        }).await.map_err(|e| anyhow!(e))?;

        Ok(json!(report))
    }

    pub(crate) async fn handle_fs_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        // Emit agent_editing_file for any write operation so the frontend can
        // show the Windsurf-style "agent hands" cursor in the active editor.
        const WRITE_OPS: &[&str] = &[
            "write_to_file", "str_replace", "search_replace_edit", "fast_apply",
            "patch_file_content", "apply_shadow_patch", "replace_file_content",
            "multi_replace_file_content", "apply_patch",
        ];
        if WRITE_OPS.contains(&name) {
            let path = arguments.get("path")
                .or_else(|| arguments.get("file_path"))
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !path.is_empty() {
                self.emit_agent_editing(path);
            }
        }

        match name {
            "view_file" => self.read_file(arguments).await,
            "write_to_file" => self.write_file(arguments).await,
            "remove_item" => self.remove_item(arguments).await,
            "list_files" => self.list_files(arguments).await,
            "search_files" => self.search_files(arguments).await,
            "grep" => self.grep(arguments).await,
            "replace_file_content" => self.replace_file_content(arguments).await,
            "multi_replace_file_content" => self.multi_replace_file_content(arguments).await,
            "find_by_name" => self.find_by_name(arguments).await,
            "get_directory_structure" => self.get_directory_structure(arguments).await,
            "create_directory" => self.create_directory(arguments).await,
            "rename_path" => self.rename_path(arguments).await,
            "editor_open_file" => self.editor_open_file(arguments).await,
            "editor_get_active_file" => self.editor_get_active_file(arguments).await,
            "semantic_search" => self.semantic_search(arguments).await,
            "aim_pack_context" => self.aim_pack_context(arguments).await,
            "aim_query_spans" => self.aim_query_spans_tool(arguments).await,
            "find_symbols" => self.find_symbols(arguments).await,
            "read_file_lines" => self.read_file_lines(arguments).await,
            "reindex_project" => self.reindex_project(arguments).await,
            "list_dir_tree" => self.list_dir_tree(arguments).await,
            "list_mcp_ops" => self.list_mcp_ops(arguments).await,
            "hex_dump" => self.hex_dump(arguments).await,
            "extract_strings" => self.extract_strings(arguments).await,
            "list_active_processes" => self.list_active_processes(arguments).await,
            "apply_patch" => self.apply_patch(arguments).await,
            "get_file_metadata" => self.get_file_metadata(arguments).await,
            "ide_get_state" => self.ide_get_state(arguments).await,
            "network_port_scanner" => self.network_port_scanner(arguments).await,
            "binary_mach_o_scanner" => self.binary_mach_o_scanner(arguments).await,
            "file_entropy_analysis" => self.file_entropy_analysis(arguments).await,
            "secrets_scan" => self.secrets_scan(arguments).await,
            "weaponize_env" => self.weaponize_env(arguments).await,
            "sec_distro_inventory" => self.sec_distro_inventory(arguments).await,
            "deep_security_audit" => self.deep_security_audit(arguments).await,
            "web_security_audit" => self.web_security_audit(arguments).await,
            "ai_vuln_hunt" => self.ai_vuln_hunt(arguments).await,
            "vega_dast_scan" => self.vega_dast_scan(arguments).await,
            "chunk_secret_scan" => self.chunk_secret_scan(arguments).await,
            "bounty_scan" => self.bounty_scan(arguments).await,
            "oast_payload" => self.oast_payload(arguments).await,
            "oast_interactions" => self.oast_interactions(arguments).await,
            "dev_cargo_diagnostics" => self.dev_cargo_diagnostics(arguments).await,
            "search_codebase" => self.search_codebase(arguments).await,
            "get_lsp_diagnostics" => self.get_lsp_diagnostics(arguments).await,
            "web_search" => self.web_search_tool(arguments).await,
            "web_fetch" => self.web_fetch_tool(arguments).await,
            "ai_propose_edit" => self.ai_propose_edit(arguments).await,
            "str_replace" => self.str_replace_file(arguments).await,
            "search_replace_edit" => self.search_replace_edit(arguments).await,
            "fast_apply" => self.fast_apply(arguments).await,
            "patch_file_content" => self.patch_file_content(arguments).await,
            "preview_shadow_diff" => self.preview_shadow_diff(arguments).await,
            "apply_shadow_patch" => self.apply_shadow_patch(arguments).await,
            "ghost_test" => self.ghost_test(arguments).await,
            _ => unreachable!(),
        }
    }

    pub(crate) async fn handle_terminal_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "run_command" => self.run_command(arguments).await,
            "terminal_send_data" => self.terminal_send_data(arguments).await,
            "terminal_read_output" => self.terminal_read_output(arguments).await,
            "terminal_toggle" => self.terminal_toggle(arguments).await,
            "terminal_create" => self.terminal_create(arguments).await,
            "terminal_terminate" => self.terminal_terminate(arguments).await,
            "terminal_get_status" => self.terminal_get_status(arguments).await,
            "terminal_list" => self.terminal_get_state(arguments).await,
            _ => unreachable!(),
        }
    }

    pub(crate) async fn handle_browser_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "browser_close" => self.browser_close(arguments).await,
            "browser_capture_vision_context" => self.browser_capture_vision_context(arguments).await,
            "browser_open" => self.browser_open(arguments).await,
            "browser_navigate" => self.browser_navigate(arguments).await,
            "browser_search" => self.browser_search(arguments).await,
            "browser_get_content_summary" => self.browser_get_content_summary(arguments).await,
            "browser_screenshot" => self.browser_screenshot(arguments).await,
            "browser_click" => self.browser_click(arguments).await,
            "browser_type" => self.browser_type(arguments).await,
            "browser_read_dom" => self.browser_read_dom(arguments).await,
            _ => unreachable!(),
        }
    }

    pub(crate) async fn handle_git_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "git_status" => self.git_status(arguments).await,
            "git_add" => self.git_add(arguments).await,
            "git_commit" => self.git_commit(arguments).await,
            "git_diff" => self.git_diff(arguments).await,
            "git_log" => self.git_log(arguments).await,
            _ => unreachable!(),
        }
    }

    pub(crate) async fn handle_system_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "get_system_info" => self.get_system_info(arguments).await,
            "get_system_health" => self.get_system_health(arguments).await,
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub(crate) async fn view_file_outline(&self, args: Value) -> Result<Value> {
        let path_str = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().await;
        let full_path = if PathBuf::from(path_str).is_absolute() {
            PathBuf::from(path_str)
        } else {
            root.join(path_str)
        };
        let content = fs::read_to_string(&full_path)?;

        let ext = full_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let mut results = Vec::new();

        let tree_res = {
            let mut parser = Parser::new();

            let lang = match ext {
                "rs" => tree_sitter_rust::LANGUAGE.into(),
                "ts" | "tsx" => tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
                "js" | "jsx" => tree_sitter_javascript::LANGUAGE.into(),
                "py" => tree_sitter_python::LANGUAGE.into(),
                _ => return self.analyze_file_symbols(args).await, // Fallback to regex-based
            };

            parser
                .set_language(&lang)
                .map_err(|e| anyhow!(e.to_string()))?;
            
            let t = parser
                .parse(&content, None)
                .ok_or_else(|| anyhow!("Parse failed"))?;
            
            let query_str = match ext {
                "rs" => "(function_item name: (identifier) @name) @item (struct_item name: (type_identifier) @name) @item (enum_item name: (type_identifier) @name) @item (trait_item name: (type_identifier) @name) @item (impl_item type: (type_identifier) @name) @item",
                "ts" | "tsx" => "(function_declaration name: (identifier) @name) @item (class_declaration name: (identifier) @name) @item (interface_declaration name: (identifier) @name) @item (variable_declarator name: (identifier) @name value: (arrow_function)) @item",
                "js" | "jsx" => "(function_declaration name: (identifier) @name) @item (class_declaration name: (identifier) @name) @item",
                "py" => "(function_definition name: (identifier) @name) @item (class_definition name: (identifier) @name) @item",
                _ => unreachable!(),
            };

            let query = Query::new(&lang, query_str).map_err(|e| anyhow!(e.to_string()))?;
            Ok::<(tree_sitter::Tree, Query, tree_sitter::Language), anyhow::Error>((t, query, lang))
        }?;

        let (tree, query, _lang) = tree_res;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            let mut name = String::new();
            let mut start_line = 0;
            let mut end_line = 0;

            for capture in m.captures {
                let node = capture.node;
                let capture_name = query.capture_names()[capture.index as usize];
                if capture_name == "name" {
                    name = content[node.start_byte()..node.end_byte()].to_string();
                    start_line = node.start_position().row + 1;
                } else if capture_name == "item" {
                    end_line = node.end_position().row + 1;
                }
            }
            if !name.is_empty() {
                results
                    .push(json!({ "name": name, "start_line": start_line, "end_line": end_line }));
            }
        }

        Ok(json!(results))
    }

    #[allow(dead_code)]
    pub(crate) async fn view_code_item(&self, args: Value) -> Result<Value> {
        let path_str = args["path"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing path"))?;
        let item_name = args["name"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing name"))?;
        let outline = self.view_file_outline(args.clone()).await?;

        if let Some(items) = outline.as_array() {
            for item in items {
                if item["name"].as_str() == Some(item_name) {
                    let start = item["start_line"].as_u64().unwrap_or(1) as usize;
                    let end = item["end_line"].as_u64().unwrap_or(1) as usize;

                    let root = self.root_path.lock().await;
                    let full_path = if PathBuf::from(path_str).is_absolute() {
                        PathBuf::from(path_str)
                    } else {
                        root.join(path_str)
                    };
                    let content = fs::read_to_string(full_path)?;
                    let lines: Vec<&str> = content.lines().collect();

                    let result_lines = &lines[start - 1..std::cmp::min(end, lines.len())];
                    return Ok(json!({ "content": result_lines.join("\n") }));
                }
            }
        }

        Err(anyhow!(
            "Code item '{}' not found in {}",
            item_name,
            path_str
        ))
    }

    #[allow(dead_code)]
    pub(crate) async fn manage_task(&self, args: Value) -> Result<Value> {
        let h_lock = self
            .app_handle
            .lock()
            .await;
        let h = h_lock
            .as_ref()
            .ok_or_else(|| anyhow!("App handle not set"))?;

        let task_id = args["task_id"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing task_id"))?;
        let status = args["status"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing status"))?;

        // Emit UI event for the Agent Task View
        h.emit(
            "update-agent-task",
            json!({
                "id": task_id,
                "title": task_id,
                "summary": format!("Executing task: {}", task_id),
                "status": if status == "done" { "completed" } else { "running" },
                "progress": if status == "done" { 100 } else { 50 }
            }),
        )?;

        h.emit("add-agent-step", json!({ "name": task_id, "status": if status == "done" { "success" } else { "running" } }))?;

        let entry = format!(
            "- [{}] {}\n",
            if status == "done" {
                "x"
            } else if status == "in_progress" {
                "/"
            } else {
                " "
            },
            task_id
        );

        // Also write to task.md if it exists
        let root_path = self.root_path.lock().await;
        let task_path = root_path.join("task.md");
        if task_path.exists() {
            let mut content = fs::read_to_string(&task_path)?;
            // Simple naive replacement for now, real agent would use grep/regex
            if !content.contains(task_id) {
                content.push_str(&entry);
            }
            fs::write(task_path, content)?;
        }

        Ok(json!({ "status": "success" }))
    }

    #[allow(dead_code)]
    pub(crate) async fn manage_memory(&self, args: Value) -> Result<Value> {
        let entry = args["entry"]
            .as_str()
            .ok_or_else(|| anyhow!("Missing entry"))?;

        let root = self.root_path.lock().await;
        let mut memory_root = root.clone();
        if memory_root.ends_with("src-tauri") {
            if let Some(parent) = memory_root.parent() {
                memory_root = parent.to_path_buf();
            }
        }
        let memory_path = memory_root.join("MEMORY.md");

        use std::io::Write;
        use std::time::{SystemTime, UNIX_EPOCH};
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&memory_path)?;

        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let (y, mo, d, h, mi) = {
            let s = secs;
            let days = s / 86400;
            let rem = s % 86400;
            let h = rem / 3600;
            let mi = (rem % 3600) / 60;
            let z = days + 719468;
            let era = z / 146097;
            let doe = z - era * 146097;
            let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
            let y = yoe + era * 400;
            let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
            let mp = (5 * doy + 2) / 153;
            let d = doy - (153 * mp + 2) / 5 + 1;
            let mo = if mp < 10 { mp + 3 } else { mp - 9 };
            let y = if mo <= 2 { y + 1 } else { y };
            (y, mo, d, h, mi)
        };

        let entry_formatted = format!(
            "\n\n### [{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC]\n{}\n",
            entry
        );
        file.write_all(entry_formatted.as_bytes())?;

        // Signal task update
        let _ = self.manage_task(json!({ "task_id": "Recursive Learning", "status": "done" })).await;

        Ok(json!({ "status": "success", "file": "MEMORY.md" }))
    }

    pub(crate) fn _get_flattened_files(&self, root: &std::path::Path) -> Vec<String> {
        let mut files = Vec::new();
        if let Ok(entries) = std::fs::read_dir(root) {
            for entry in entries.flatten() {
                if let Ok(name) = entry.file_name().into_string() {
                    files.push(name);
                }
            }
        }
        files
    }
}
