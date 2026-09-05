use std::sync::Arc;
use crate::ai_engine::Sentient;
use crate::memory_layer::MemoryLayer;
use crate::shadow_workspace::ShadowWorkspace;
use crate::patch_engine::PatchEngine;
use crate::ghost_runtime::GhostRuntime;

pub struct HadesHarness {
    pub sentient: Arc<Sentient>,
    pub memory_layer: Arc<MemoryLayer>,
    pub shadow_workspace: Arc<ShadowWorkspace>,
    pub patch_engine: Arc<tokio::sync::Mutex<PatchEngine>>,
    pub ghost_runtime: Arc<GhostRuntime>,
}

/// Result of running the correction loop.
#[derive(serde::Serialize)]
pub struct CorrectionResult {
    pub success: bool,
    pub attempts: u32,
    pub committed: bool,
    pub message: String,
}

impl HadesHarness {
    pub fn new(
        sentient: Arc<Sentient>,
        memory_layer: Arc<MemoryLayer>,
        shadow_workspace: Arc<ShadowWorkspace>,
        patch_engine: Arc<tokio::sync::Mutex<PatchEngine>>,
        ghost_runtime: Arc<GhostRuntime>,
    ) -> Self {
        Self {
            sentient,
            memory_layer,
            shadow_workspace,
            patch_engine,
            ghost_runtime,
        }
    }

    pub async fn execute_agent_step(&self, step: &str) -> anyhow::Result<()> {
        println!("[HadesHarness] Executing mission step: {}", step);
        let calculative_seed = hades_harness::vedic::VedicBrain::fold_address(0xDEADBEEF, 0x1337);
        println!("[Hades-Vedic] Computational seed for step: 0x{:X}", calculative_seed);
        Ok(())
    }

    /// Validates a code candidate using the Stoic Katalepsis filter.
    pub async fn validate_verity(&self, diagnostics: &[hades_harness::Diagnostic]) -> f32 {
        let verity_score = hades_harness::KatalepsisFilter::evaluate_verity(diagnostics);
        self.sentient.emit_event("hades://verity", serde_json::json!({
            "score": verity_score,
            "status": if verity_score >= 1.0 { "Katalepsis" } else { "Doxa" },
            "count": diagnostics.len()
        }));
        verity_score
    }

    /// Run the kortex CorrectionLoop: apply a patch to a shadow workspace,
    /// compile, and on failure ask the Lemonade model for corrections —
    /// retrying until it compiles or attempts run out. The real workspace is
    /// only touched after a patch compiles.
    pub async fn run_correction_loop(
        &self,
        workspace_root: &std::path::Path,
        file_path: &str,
        content: String,
        lemonade_url: &str,
        model: &str,
    ) -> CorrectionResult {
        use hades_harness::verify::{CodePatch, CorrectionLoop, LoopConfig};
        use hades_harness::lemonade::LemonadeAgent;

        let patch = CodePatch::new(file_path, content);
        let loop_config = LoopConfig {
            max_attempts: 3,
            max_errors_in_prompt: 5,
            commit_on_success: false,
        };
        let root = workspace_root.to_path_buf();
        let url = lemonade_url.to_string();
        let mdl = model.to_string();

        let result = tokio::task::spawn_blocking(move || -> Result<hades_harness::verify::Outcome, String> {
            let mut agent = LemonadeAgent::new(&url, &mdl)
                .map_err(|e| format!("Lemonade agent: {e}"))?;
            CorrectionLoop::new(root)
                .with_config(loop_config)
                .run(patch, &mut agent)
                .map_err(|e| format!("Correction loop: {e}"))
        })
        .await;

        match result {
            Ok(Ok(outcome)) => {
                use hades_harness::verify::Outcome;
                match outcome {
                    Outcome::Compiles { attempts, warnings, committed, .. } => {
                        println!("[HadesHarness] Correction succeeded in {attempts} attempt(s), {} warnings", warnings.len());
                        CorrectionResult {
                            success: true,
                            attempts: attempts as u32,
                            committed,
                            message: format!("Patch applied successfully in {attempts} attempt(s)"),
                        }
                    }
                    Outcome::Exhausted { attempts, diagnostics, .. } => {
                        let error_msgs: Vec<String> = diagnostics.iter()
                            .filter(|d| d.level == "error")
                            .map(|d| d.message.clone())
                            .collect();
                        CorrectionResult {
                            success: false,
                            attempts: attempts as u32,
                            committed: false,
                            message: format!("Exhausted {attempts} attempts. Last errors: {}", error_msgs.join("; ")),
                        }
                    }
                }
            }
            Ok(Err(e)) => CorrectionResult {
                success: false,
                attempts: 0,
                committed: false,
                message: e,
            },
            Err(e) => CorrectionResult {
                success: false,
                attempts: 0,
                committed: false,
                message: format!("Correction loop panicked: {e}"),
            },
        }
    }
}

/// Run the Lemonade-backed correction loop on a proposed patch, standalone.
///
/// Verifies the patch compiles in a shadow workspace, asking Lemonade for fixes
/// until it builds or attempts run out; the real tree is only written on
/// success. No `HadesHarness` instance is needed — the loop only wants the
/// workspace root — so this is a plain command. Defaults to Lemonade on :13305.
#[cfg(feature = "tauri")]
#[tauri::command]
pub async fn kortex_correction_run(
    workspace_root: String,
    file_path: String,
    content: String,
    model: Option<String>,
    lemonade_url: Option<String>,
) -> Result<CorrectionResult, String> {
    use hades_harness::lemonade::LemonadeAgent;
    use hades_harness::verify::{CodePatch, CorrectionLoop, LoopConfig, Outcome};

    let url = lemonade_url
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "http://localhost:13305".to_string());
    let mdl = model
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "Qwen3.6-27B-Fable-5".to_string());
    let root = std::path::PathBuf::from(workspace_root);
    let patch = CodePatch::new(&file_path, content);
    let cfg = LoopConfig { max_attempts: 3, max_errors_in_prompt: 5, commit_on_success: false };

    let outcome = tokio::task::spawn_blocking(move || -> Result<Outcome, String> {
        let mut agent = LemonadeAgent::new(&url, &mdl).map_err(|e| format!("Lemonade agent: {e}"))?;
        CorrectionLoop::new(root)
            .with_config(cfg)
            .run(patch, &mut agent)
            .map_err(|e| format!("Correction loop: {e}"))
    })
    .await
    .map_err(|e| format!("correction task join failed: {e}"))??;

    Ok(match outcome {
        Outcome::Compiles { attempts, warnings, committed, .. } => CorrectionResult {
            success: true,
            attempts: attempts as u32,
            committed,
            message: format!("Compiled in {attempts} attempt(s), {} warning(s)", warnings.len()),
        },
        Outcome::Exhausted { attempts, diagnostics, .. } => {
            let errs: Vec<String> = diagnostics
                .iter()
                .filter(|d| d.level == "error")
                .map(|d| d.message.clone())
                .collect();
            CorrectionResult {
                success: false,
                attempts: attempts as u32,
                committed: false,
                message: format!("Exhausted {attempts} attempt(s). Errors: {}", errs.join("; ")),
            }
        }
    })
}
