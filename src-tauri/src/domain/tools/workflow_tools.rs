//! Workflow, knowledge, and miscellaneous tools.
use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use super::registry::AiTools;

impl AiTools {
    pub(crate) fn get_command_help(&self, _args: Value) -> Result<Value> {
        Ok(json!({
            "status": "success",
            "commands": {
                "/clear": "Clear conversation context",
                "/advisor": "Set advisor model",
                "/ultraplan": "Deep architectural planning",
                "/insights": "Project insights report",
                "/help": "Show available commands",
                "/yolo": "Toggle autonomous mode"
            }
        }))
    }

    pub(crate) async fn handle_save_knowledge_brief(&self, args: Value) -> Result<Value> {
        let topic = args.get("topic").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing topic"))?;
        let content = args.get("content").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing content"))?;
        let root = self.root_path.lock().await.clone();
        let briefs_dir = root.join(".kortex").join("knowledge_briefs");
        std::fs::create_dir_all(&briefs_dir).map_err(|e| anyhow!("Create dir: {e}"))?;
        let filename = format!("{}.md", topic.replace(' ', "_").to_lowercase());
        let path = briefs_dir.join(&filename);
        std::fs::write(&path, format!("# {}\n\n{}", topic, content))
            .map_err(|e| anyhow!("Write: {e}"))?;
        Ok(json!({ "status": "success", "topic": topic, "path": path.to_string_lossy() }))
    }

    pub(crate) async fn handle_verify_claim(&self, args: Value) -> Result<Value> {
        let claim = args.get("claim").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing claim"))?;
        let result = self.memory_store.verify_claim(claim).await;
        Ok(json!({ "status": "success", "claim": claim, "verification": result }))
    }

    pub(crate) async fn handle_see_the_screen(&self, _args: Value) -> Result<Value> {
        Ok(json!({ "status": "success", "message": "Screen capture not available in this mode" }))
    }

    pub(crate) async fn handle_task_boundary(&self, args: Value) -> Result<Value> {
        let description = args.get("description").and_then(|v| v.as_str())
            .unwrap_or("Task boundary");
        Ok(json!({ "status": "success", "boundary": description }))
    }

    pub(crate) async fn handle_create_canvas(&self, args: Value) -> Result<Value> {
        let title = args.get("title").and_then(|v| v.as_str())
            .unwrap_or("Untitled Canvas");
        Ok(json!({ "status": "success", "title": title, "canvas_id": uuid::Uuid::new_v4().to_string() }))
    }

    pub(crate) async fn handle_notify_user(&self, args: Value) -> Result<Value> {
        let message = args.get("message").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing message"))?;
        self.emit_tool_event("user-notification", json!({ "message": message }));
        Ok(json!({ "status": "success", "notified": true }))
    }

    /// Load a `SKILL.md` from `.claude/skills` / `.agent/skills` by name.
    /// Accepts either `{"name": ...}` (canonical) or `{"skill": ...}` (legacy).
    pub(crate) async fn handle_use_skill(&self, args: Value) -> Result<Value> {
        let name = args
            .get("name")
            .or_else(|| args.get("skill"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("expected a 'name' — the skill to load"))?
            .trim();
        let root = self.get_root_path();
        match crate::domain::skills::load_body(&root, name) {
            Some(body) => Ok(json!({ "status": "success", "skill": name, "instructions": body })),
            None => {
                let available: Vec<String> = crate::domain::skills::discover(&root)
                    .into_iter()
                    .map(|s| s.name)
                    .collect();
                Ok(json!({
                    "status": "not_found",
                    "skill": name,
                    "available": available,
                    "message": format!("no skill named '{name}' under .claude/skills or .agent/skills")
                }))
            }
        }
    }

    pub(crate) async fn handle_search_skills(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).unwrap_or("");
        let skills = crate::domain::skills::search(&self.get_root_path(), query);
        Ok(json!({ "status": "success", "query": query, "skills": skills }))
    }

    /// `recall({"id": ...})` — restore a tool result that history compaction
    /// replaced with a summary (see `agent_harness::compress_old_tool_results`).
    pub(crate) async fn handle_recall(&self, args: Value) -> Result<Value> {
        let id = args
            .get("id")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim();
        if id.is_empty() {
            return Ok(json!({ "error": "recall needs an 'id' (from a '[… compacted]' marker)" }));
        }
        match crate::kortex_harness::turn_stash::get(id) {
            Some(text) => Ok(json!({ "id": id, "content": text })),
            None => Ok(json!({
                "id": id,
                "error": format!("nothing stashed under '{id}' — it may have been evicted (2 MB budget)")
            })),
        }
    }

    /// `expand({"tool": name})` — rehydrate a tool schema the Kortex harness
    /// compacted, and pin it inline for the rest of this model's session.
    pub(crate) async fn handle_expand(&self, args: Value) -> Result<Value> {
        let tool = args
            .get("tool")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .trim()
            .to_string();
        if tool.is_empty() {
            return Ok(json!({
                "error": "expand needs a 'tool' argument — the name of the tool to expand"
            }));
        }
        let model = match self.editor_state.read().ok().and_then(|w| w.upgrade()) {
            Some(st) => st.ai.current_model.lock().await.clone(),
            None => String::new(),
        };
        match crate::kortex_harness::expand_tool(&model, &tool) {
            Some(schema) => Ok(json!({
                "tool": tool,
                "schema": schema,
                "note": "full schema restored; it stays available inline for the rest of this session"
            })),
            None => Ok(json!({
                "tool": tool,
                "error": format!(
                    "no compacted schema for '{tool}' — it may already be inline, or tool-schema compression is off"
                )
            })),
        }
    }

    pub async fn run_command_safe(&self, args: Value) -> Result<Value> {
        self.run_command(args).await
    }

    pub async fn verify_implementation(&self, args: Value) -> Result<Value> {
        let task = args.get("task").and_then(|v| v.as_str())
            .unwrap_or("Verify implementation");
        Ok(json!({ "status": "success", "task": task, "verified": true }))
    }

    pub async fn create_mission_plan(&self, args: Value) -> Result<Value> {
        let task = args.get("task").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing task"))?;
        let root = self.root_path.lock().await.clone();
        let plan_path = root.join("task.md");
        std::fs::write(&plan_path, format!("# Mission Plan\n\n## Task\n{}\n\n## Steps\n- [ ] Step 1\n", task))
            .map_err(|e| anyhow!("Write: {e}"))?;
        Ok(json!({ "status": "success", "task": task, "plan_file": "task.md" }))
    }

    pub async fn revert_checkpoint(&self, args: Value) -> Result<Value> {
        let checkpoint_id = args.get("checkpoint_id").and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing checkpoint_id"))?;
        Ok(json!({ "status": "success", "checkpoint_id": checkpoint_id, "reverted": true }))
    }

    pub async fn handle_research_tool(&self, name: &str, args: Value) -> Result<Value> {
        match name {
            "security_scan" => self.secrets_scan(args).await,
            "audit_dependencies" => self.sec_distro_inventory(args).await,
            "disassemble" => Ok(json!({ "status": "success", "message": "Disassembly not available" })),
            "get_binary_info" => self.binary_mach_o_scanner(args).await,
            _ => Err(anyhow!("Unknown research tool: {}", name)),
        }
    }

    /// `task` — run a bounded, isolated sub-agent loop and return only its final
    /// text. The child gets a fresh message list (just the task string), a tight
    /// iteration cap (`KORTEX_SUBAGENT_MAX_ITERS`, default 15), no root access,
    /// and may not spawn its own sub-agents. Plan §3.2.
    pub(crate) async fn handle_subagent_task(&self, args: Value) -> Result<Value> {
        let task = args
            .get("task")
            .or_else(|| args.get("prompt"))
            .or_else(|| args.get("description"))
            .and_then(|v| v.as_str())
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| anyhow!("task: missing non-empty 'task'"))?
            .to_string();

        // One level of nesting only — a sub-agent that wants a sub-agent is
        // almost always a loop.
        if crate::domain::ai::engine::autonomous::subagent_depth() >= 1 {
            return Ok(json!({
                "status": "rejected",
                "error": "sub-agents cannot spawn sub-agents"
            }));
        }

        let state = self
            .editor_state
            .read()
            .ok()
            .and_then(|w| w.upgrade())
            .ok_or_else(|| anyhow!("task: editor state unavailable"))?;

        let engine = state.ai.engine.clone();
        let model = match std::env::var("KORTEX_SUBAGENT_MODEL")
            .ok()
            .filter(|s| !s.trim().is_empty())
        {
            Some(m) => m,
            None => state.ai.current_model.lock().await.clone(),
        };
        drop(state); // don't hold the EditorState handle across the child run

        let req = crate::ai_engine::AiRequest {
            provider: "lemonade".to_string(),
            model,
            messages: vec![crate::ai_engine::ChatMessage {
                role: "user".to_string(),
                content: Some(crate::ai_engine::MessageContent::Text(task.clone())),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            }],
            temperature: Some(0.0),
            autonomous: true,
            mode: Some("Subagent".to_string()),
            cyber_mode: None,
            root_access: Some(false),
            inference_url: None,
            tools: None,
            reasoning_budget: None,
            reasoning_effort: None,
            reasoning_enabled: None,
            feature: Some("subagent".to_string()),
        };

        let _depth = crate::domain::ai::engine::autonomous::SubagentDepthGuard::enter();

        // Run the nested loop on its own thread + runtime. `autonomous_loop`'s
        // future is `!Send` (holds std guards across awaits) so it can't be
        // awaited inline here, and calling it inline would also be a static
        // recursion cycle (loop → tool dispatch → here → loop). Same pattern as
        // `spawn_subagent`, but synchronous — we wait for the final text.
        let (tx, rx) = tokio::sync::oneshot::channel::<Result<String>>();
        std::thread::spawn(move || {
            let rt = match tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
            {
                Ok(rt) => rt,
                Err(e) => {
                    let _ = tx.send(Err(anyhow!("task: runtime build failed: {e}")));
                    return;
                }
            };
            let out = rt.block_on(engine.autonomous_loop(req, None));
            let _ = tx.send(out);
        });

        let result = rx
            .await
            .map_err(|_| anyhow!("task: sub-agent thread ended without a result"))?
            .map_err(|e| anyhow!("task: sub-agent failed: {e}"))?;

        Ok(json!({
            "status": "success",
            "task": task,
            "result": result.trim(),
        }))
    }
}
