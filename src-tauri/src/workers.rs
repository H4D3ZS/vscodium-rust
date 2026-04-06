use crate::specs_db::{SpecDb, WorkItem};
use crate::ai_engine::Sentient;
use crate::ai_prompts::*;
use std::sync::Arc;
use std::fs;
use std::path::{PathBuf};
use tokio::time::{sleep, Duration};

pub struct WorkerManager {
    db: Arc<SpecDb>,
    ai: Arc<Sentient>,
    root_path: PathBuf,
}

impl WorkerManager {
    pub fn new(db: Arc<SpecDb>, ai: Arc<Sentient>, root_path: PathBuf) -> Self {
        Self { db, ai, root_path }
    }

    pub async fn start_loop(self: Arc<Self>) {
        loop {
            if let Ok(Some(work)) = self.db.fetch_pending_work() {
                let _ = self.handle_work_item(work).await;
            } else {
                sleep(Duration::from_secs(1)).await;
            }
        }
    }

    async fn handle_work_item(&self, work: WorkItem) -> Result<(), String> {
        let result = match work.work_type.as_str() {
            "Analyzation" => self.handle_analyzation(&work).await,
            "Structure" => self.handle_structure(&work).await,
            "Design" => self.handle_design(&work).await,
            "Backend" => self.handle_backend(&work).await,
            "Frontend" => self.handle_frontend(&work).await,
            // Fallbacks for legacy/compatibility
            "GenerateLayout" => self.handle_analyzation(&work).await, 
            "GenerateTest" => self.handle_backend(&work).await,
            "GenerateCode" => self.handle_backend(&work).await,
            _ => Err("Unknown work type".to_string()),
        };

        match result {
            Ok(logs) => {
                let _ = self.db.update_work_status(work.id, "Done", &logs);
            }
            Err(e) => {
                if work.retries < 3 {
                    let _ = self.db.increment_retry(work.id);
                } else {
                    let _ = self.db.update_work_status(work.id, "Failed", &e);
                }
            }
        }
        Ok(())
    }

    async fn handle_analyzation(&self, work: &WorkItem) -> Result<String, String> {
        let project = self.db.get_project(work.data_id).map_err(|e| e.to_string())?;
        self.db.update_work_log(work.id, "Phase 1: Deep Analysis of Specifications (Kortex Augmented)...").map_err(|e| e.to_string())?;
        
        let (kortex, skills) = self.retrieve_relevant_context(&project.specs).await;
        let prompt = ANALYZATION_PROMPT
            .replace("{specs}", &project.specs)
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let _response = self.stream_ai_to_log(work.id, &prompt, project.preferred_provider.clone()).await?;
        
        // Progress to Phase 2: Structure
        self.db.add_work_item("Structure", project.id).map_err(|e| e.to_string())?;
        Ok("Analysis complete. Kortex insights integrated.".to_string())
    }

    async fn handle_structure(&self, work: &WorkItem) -> Result<String, String> {
        let project = self.db.get_project(work.data_id).map_err(|e| e.to_string())?;
        // Get analysis from Phase 1 logs (simplified approach: just use specs for now)
        self.db.update_work_log(work.id, "Phase 2: Defining Project File System Structure...").map_err(|e| e.to_string())?;
        
        let (kortex, skills) = self.retrieve_relevant_context(&project.specs).await;
        let prompt = STRUCTURE_PROMPT
            .replace("{specs}", &project.specs)
            .replace("{analysis}", "Existing specifications")
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let response = self.stream_ai_to_log(work.id, &prompt, project.preferred_provider.clone()).await?;
        
        let json_str = extract_json(&response).ok_or("No JSON structure found")?;
        let files: Vec<String> = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
        
        for path in files {
            self.db.add_file(project.id, &path).map_err(|e| e.to_string())?;
        }
        
        // SYNC: Create folders and empty files immediately
        let _ = self.sync_project_to_disk(project.id).await;
        
        // Progress to Phase 3: Design
        self.db.add_work_item("Design", project.id).map_err(|e| e.to_string())?;
        Ok("Structure defined. Proceeding to MVC Design phase.".to_string())
    }

    async fn handle_design(&self, work: &WorkItem) -> Result<String, String> {
        let project = self.db.get_project(work.data_id).map_err(|e| e.to_string())?;
        let db_files = self.db.get_project_files(project.id).map_err(|e| e.to_string())?;
        let file_paths = db_files.iter().map(|f| f["path"].as_str().unwrap_or("")).collect::<Vec<_>>().join(", ");
        
        self.db.update_work_log(work.id, "Phase 3: Designing MVC Interfaces (Rules Applied)...").map_err(|e| e.to_string())?;
        
        let (kortex, skills) = self.retrieve_relevant_context(&project.specs).await;
        let prompt = MVC_DESIGN_PROMPT
            .replace("{specs}", &project.specs)
            .replace("{files}", &file_paths)
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let response = self.stream_ai_to_log(work.id, &prompt, project.preferred_provider.clone()).await?;
        
        let json_str = extract_json(&response).ok_or("No JSON design found")?;
        let layout: Vec<serde_json::Value> = serde_json::from_str(&json_str).map_err(|e| e.to_string())?;
        
        let mut first_func_id = None;
        for file_val in layout {
            let path = file_val["path"].as_str().unwrap_or("unknown.rs");
            // Find file ID by path (optimization: assuming we have get_file_by_path or similar)
            let file_id = self.db.get_project_files(project.id).unwrap().into_iter()
                .find(|f| f["path"] == path).map(|f| f["id"].as_i64().unwrap_or(0)).unwrap_or(0);
            
            if file_id == 0 { continue; }

            if let Some(funcs) = file_val["functions"].as_array() {
                for func_val in funcs {
                    let name = func_val["name"].as_str().unwrap_or("unknown");
                    let signature = func_val["signature"].as_str().unwrap_or("");
                    let func_id = self.db.add_function(file_id, name, signature).map_err(|e| e.to_string())?;
                    if first_func_id.is_none() { first_func_id = Some(func_id); }
                }
            }
        }
        
        // SYNC: Write back stubs (signatures) to physical disk
        let _ = self.sync_project_to_disk(project.id).await;
        
        // Progress to Phase 4: Backend
        if let Some(fid) = first_func_id {
            self.db.add_work_item("Backend", fid).map_err(|e| e.to_string())?;
        } else {
             // Fallback to Frontend if no functions
             self.db.add_work_item("Frontend", project.id).map_err(|e| e.to_string())?;
        }
        
        Ok("Design complete. Proceeding to Backend development.".to_string())
    }

    async fn handle_backend(&self, work: &WorkItem) -> Result<String, String> {
        let func = self.db.get_function(work.data_id).map_err(|e| e.to_string())?;
        let file = self.db.get_file(func.file_id).map_err(|e| e.to_string())?;
        let project = self.db.get_project(file.project_id).map_err(|e| e.to_string())?;
        
        let _ = self.db.update_work_log(work.id, &format!("Phase 4: Developing Logic and Tests for {}...", func.name)).map_err(|e| e.to_string());
        
        // 1. Generate Tests
        let (kortex, skills) = self.retrieve_relevant_context(&func.signature).await;
        let test_prompt = TEST_GENERATION_PROMPT
            .replace("{signature}", &func.signature)
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let test_response = self.stream_ai_to_log(work.id, &test_prompt, project.preferred_provider.clone()).await?;
        let tests_json = extract_json(&test_response).ok_or("No tests found in AI response")?;
        self.db.save_test_code(func.id, &tests_json).map_err(|e| e.to_string())?;
        
        // 2. Generate Implementation (with tests as context)
        let (kortex, skills) = self.retrieve_relevant_context(&func.signature).await;
        let code_prompt = CODE_GENERATION_PROMPT
            .replace("{signature}", &func.signature)
            .replace("{tests}", &tests_json)
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let code_response = self.stream_ai_to_log(work.id, &code_prompt, project.preferred_provider.clone()).await?;
        
        self.db.update_preview_code(work.id, &code_response).map_err(|e| e.to_string())?;
        self.db.save_function_code(func.id, &code_response).map_err(|e| e.to_string())?;
        self.db.update_function_status(func.id, "Done").map_err(|e| e.to_string())?;

        // SYNC: Write back to physical disk
        let _ = self.sync_project_to_disk(project.id).await;

        // 3. Queue NEXT Backend task or Frontend
        let next_func = self.db.get_next_pending_function(project.id).map_err(|e| e.to_string())?;
        if let Some(nf) = next_func {
            self.db.add_work_item("Backend", nf.id).map_err(|e| e.to_string())?;
        } else {
             self.db.add_work_item("Frontend", project.id).map_err(|e| e.to_string())?;
        }

        Ok(format!("Backend implementation for {} complete.", func.name))
    }

    async fn handle_frontend(&self, work: &WorkItem) -> Result<String, String> {
        let project = self.db.get_project(work.data_id).map_err(|e| e.to_string())?;
        self.db.update_work_log(work.id, "Phase 5: Generating UI Stubs (Skill Augmented)...").map_err(|e| e.to_string())?;
        
        let (kortex, skills) = self.retrieve_relevant_context(&project.specs).await;
        let prompt = FRONTEND_STUB_PROMPT
            .replace("{specs}", &project.specs)
            .replace("{files}", "Full MVC layout")
            .replace("{kortex}", &kortex)
            .replace("{skills}", &skills);

        let _response = self.stream_ai_to_log(work.id, &prompt, project.preferred_provider.clone()).await?;
        
        // SYNC: Force write back for frontend stubs
        let _ = self.sync_project_to_disk(project.id).await;

        Ok("Frontend stubs generated. Assembly cycle complete.".to_string())
    }

    pub async fn sync_project_to_disk(&self, project_id: i64) -> Result<(), String> {
        let project = self.db.get_project(project_id).map_err(|e| e.to_string())?;
        let files = self.db.get_project_files(project_id).map_err(|e| e.to_string())?;
        
        let base_path = self.root_path.join("specs_projects").join(&project.name);
        
        if !base_path.exists() {
            fs::create_dir_all(&base_path).map_err(|e| e.to_string())?;
        }

        for file_val in files {
            let file_id = file_val["id"].as_i64().unwrap_or(0);
            let path = file_val["path"].as_str().unwrap_or("unknown.rs");
            
            let full_file_path = base_path.join(path);
            if let Some(parent) = full_file_path.parent() {
                let _ = fs::create_dir_all(parent);
            }

            if let Ok(content) = self.db.get_file_full_content(file_id) {
                let _ = fs::write(full_file_path, content);
            }
        }

        Ok(())
    }

    async fn stream_ai_to_log(&self, work_id: i64, prompt: &str, provider: Option<String>) -> Result<String, String> {
        let db_clone = self.db.clone();
        let buffer = Arc::new(std::sync::Mutex::new(String::new()));
        let cb_buffer = buffer.clone();
        let last_update = Arc::new(std::sync::Mutex::new(std::time::Instant::now()));
        
        let db_for_closure = db_clone.clone();
        let on_chunk = Some(Arc::new(move |chunk: &str| {
            let mut b = cb_buffer.lock().unwrap();
            b.push_str(chunk);
            
            let mut last = last_update.lock().unwrap();
            if last.elapsed() >= Duration::from_millis(200) {
                let _ = db_for_closure.update_work_log(work_id, &b);
                if b.contains("fn ") || b.contains("import ") {
                    let _ = db_for_closure.update_preview_code(work_id, &b);
                }
                *last = std::time::Instant::now();
            }
        }) as Arc<dyn Fn(&str) + Send + Sync>);

        let response = self.ai.chat_complete(
            prompt, 
            None, 
            provider, 
            None,
            on_chunk
        ).await.map_err(|e: anyhow::Error| e.to_string())?;
        
        // Final update to catch the end of the stream
        let b = buffer.lock().unwrap();
        let _ = db_clone.update_work_log(work_id, &b);
        if b.contains("fn ") || b.contains("import ") {
            let _ = db_clone.update_preview_code(work_id, &b);
        }
        
        Ok(response.content)
    }

    async fn retrieve_relevant_context(&self, query: &str) -> (String, String) {
        // 1. Retrieve Kortex morsels
        let kortex_morsels = self.ai.memory_store.retrieve_context(query).await;
        let kortex_text = if kortex_morsels.is_empty() {
            String::new()
        } else {
            format!("### PRIOR PROJECT KNOWLEDGE (KORTEX):\n{}\n", kortex_morsels)
        };

        // 2. Retrieve Relevant Skills from .agent/skills/
        let mut skill_text = String::new();
        let skills_root = self.root_path.join(".agent").join("skills");
        if skills_root.exists() {
            let query_lower = query.to_lowercase();
            if let Ok(entries) = fs::read_dir(&skills_root) {
                let mut found_skills = Vec::new();
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        let dir_name = path.file_name().unwrap_or_default().to_string_lossy();
                        // Simple keyword matching for demo/MVP
                        if query_lower.contains(&dir_name.to_lowercase()) {
                            let skill_file = path.join("SKILL.md");
                            if skill_file.exists() {
                                if let Ok(content) = fs::read_to_string(&skill_file) {
                                    found_skills.push(format!("#### Skill: {}\n{}\n", dir_name, content));
                                }
                            }
                        }
                    }
                }
                if !found_skills.is_empty() {
                    skill_text = format!("### APPLICABLE SPECIALIZED SKILLS:\n{}", found_skills.join("\n"));
                }
            }
        }

        (kortex_text, skill_text)
    }
}

fn extract_json(content: &str) -> Option<String> {
    // 1. Try markdown blocks and triple-quote blocks
    let labels = [
        "```json", "```javascript", "```typescript", "```",
        "\"\"\"json", "\"\"\"javascript", "\"\"\"typescript", "\"\"\""
    ];
    for label in &labels {
        if let Some(start) = content.find(label) {
            let after_start = &content[start + label.len()..];
            let end_delimiter = if label.starts_with("```") { "```" } else { "\"\"\"" };
            if let Some(end) = after_start.find(end_delimiter) {
                let inner = after_start[..end].trim();
                if !inner.is_empty() { return Some(inner.to_string()); }
            }
        }
    }
    // 2. Try raw array search
    if let Some(start) = content.find('[') {
        if let Some(end) = content.rfind(']') {
            let inner = &content[start..=end];
            if serde_json::from_str::<serde_json::Value>(inner).is_ok() {
                return Some(inner.to_string());
            }
        }
    }
    // 3. Try raw object search
    if let Some(start) = content.find('{') {
        if let Some(end) = content.rfind('}') {
             let inner = &content[start..=end];
             if serde_json::from_str::<serde_json::Value>(inner).is_ok() {
                return Some(inner.to_string());
            }
        }
    }
    None
}
