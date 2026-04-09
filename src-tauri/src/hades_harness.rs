use anyhow::Result;
use std::sync::{Arc, Mutex};
use crate::memory_layer::MemoryLayer;
use crate::patch_engine::PatchEngine;
use crate::ghost_runtime::GhostRuntime;
use crate::shadow_workspace::ShadowWorkspace;
use crate::ai_engine::Sentient;
use serde_json::{json, Value};

pub struct HadesHarness {
    sentient: Arc<Sentient>,
    memory: Arc<MemoryLayer>,
    shadow: Arc<ShadowWorkspace>,
    patch: Arc<tokio::sync::Mutex<PatchEngine>>,
    ghost: Arc<GhostRuntime>,
}

impl HadesHarness {
    pub fn new(
        sentient: Arc<Sentient>,
        memory: Arc<MemoryLayer>,
        shadow: Arc<ShadowWorkspace>,
        patch: Arc<tokio::sync::Mutex<PatchEngine>>,
        ghost: Arc<GhostRuntime>,
    ) -> Self {
        Self {
            sentient,
            memory,
            shadow,
            patch,
            ghost,
        }
    }

    /// The "Sentient" Execution Wrapper (ULTRAPLAN orchestration)
    pub async fn execute_agent_step(&self, task_description: &str) -> Result<Value> {
        println!("[Harness] Starting mission step: {}", task_description);

        // 1. Pre-computation: The Architect reads memory and maps dependencies
        let _context = self.memory.get_aggregate_context()?;
        // (Simulated Architect phase - reasoning is inside Sentient::reason)
        
        // 2. Execution: The Implementer applies patch to SHADOW buffer
        // Note: In this version, we trigger the agent reasoning which uses the patch_file tool
        let _result = self.sentient.clone().reason(task_description).await?;
        
        // 3. Verification: The Auditor runs background 'cargo check'
        // If the agent used 'apply_shadow_patch', it's already in the shadow files.
        println!("[Harness] Initiating Auditor verification...");
        let verify_result = self.ghost.execute("cargo check", 60).await?;
        
        if verify_result.success {
            // 4. Persistence: AUTO-UPDATE memory.md and task.md on success
            println!("[Harness] Verification SUCCESS. Syncing synaptic pins.");
            self.memory.record_decision(
                "Auto-verified step completion",
                task_description,
                "Verified via cargo check in Ghost Runtime"
            )?;
            
            self.memory.update_state("Idle", "Mission segment complete")?;
            
            Ok(json!({
                "status": "success",
                "message": "Step verified and memory synced.",
                "auditor_report": verify_result
            }))
        } else {
            // 5. Re-reasoning: The Architect analyzes the failure
            println!("[Harness] Verification FAILED. Triggering re-reasoning loop.");
            let error_msg = if !verify_result.stderr.is_empty() { &verify_result.stderr } else { &verify_result.stdout };
            let error_msg = if error_msg.is_empty() { "Unknown build error" } else { error_msg };
            
            let re_reason_prompt = format!(
                "The previous implementation failed verification with error: {}. Please analyze and fix.", 
                error_msg
            );
            
            let fixed_result = self.sentient.clone().reason(&re_reason_prompt).await?;
            
            Ok(json!({
                "status": "re_reasoned",
                "original_error": error_msg,
                "re_reason_result": fixed_result
            }))
        }
    }
}
