//! Semantic Firewall: Compiler-constrained gatekeeper for AI-driven development.
//!
//! Intercepts all AI code-generation tool calls, enforces exact diff matching,
//! validates via LSP diagnostics, and loops self-correction up to 3 iterations.
//! No unchecked atomic file write reaches persistent storage.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::sync::Mutex;

use super::patch_engine::PatchBlock;
use crate::lsp::DiagnosticsMap;
use crate::lsp_router::LspRouter;

/// Maximum self-correction iterations before halting and presenting errors to the human.
const MAX_CORRECTION_ITERATIONS: u32 = 3;

/// Placeholder detection patterns — the AI model must never emit these in SEARCH blocks.
const PLACEHOLDER_PATTERNS: &[&str] = &[
    "// ... rest of code stays the same",
    "// ...",
    "/* ... */",
    "/* remaining code */",
    "// remaining code",
    "<!-- ... -->",
    "# ...",
    "// TODO: keep existing code",
    "// rest of function unchanged",
    "// ... other methods ...",
    "// ... other imports ...",
];

/// Severity level that constitutes a blocking error in LSP diagnostics.
const ERROR_SEVERITY: i64 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FirewallVerdict {
    Accepted,
    RejectedPlaceholder,
    RejectedExactMatch,
    RejectedLspErrors,
    RejectedBloat,
    HaltedMaxIterations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallResult {
    pub verdict: FirewallVerdict,
    pub iteration: u32,
    pub message: String,
    pub diagnostics: Vec<LspDiagnosticReport>,
    pub diff: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspDiagnosticReport {
    pub uri: String,
    pub range_start_line: i64,
    pub range_start_col: i64,
    pub range_end_line: i64,
    pub range_end_col: i64,
    pub severity: i64,
    pub message: String,
    pub code: Option<String>,
    pub source: Option<String>,
}

impl LspDiagnosticReport {
    pub fn is_error(&self) -> bool {
        self.severity == ERROR_SEVERITY
    }
}

pub struct SemanticFirewall {
    lsp_diagnostics: DiagnosticsMap,
    /// Tracks iteration count per (path, generation_session_id)
    iteration_counts: HashMap<(PathBuf, String), u32>,
}

impl SemanticFirewall {
    pub fn new(lsp_diagnostics: DiagnosticsMap) -> Self {
        Self {
            lsp_diagnostics,
            iteration_counts: HashMap::new(),
        }
    }

    // ── TASK 1: Speculative CoW Sandbox & Diff Engine ─────────────────────

    /// Validate that a SEARCH/REPLACE patch contains no placeholder patterns.
    /// Returns Err if any placeholder is detected.
    pub fn validate_no_placeholders(patches: &[PatchBlock]) -> Result<()> {
        for (i, patch) in patches.iter().enumerate() {
            let search_lower = patch.search.to_lowercase();
            for pattern in PLACEHOLDER_PATTERNS {
                if search_lower.contains(&pattern.to_lowercase()) {
                    return Err(anyhow!(
                        "PLACEHOLDER DETECTED in patch block {}: \
                         '{}' is not allowed in SEARCH blocks. \
                         The SEARCH block must contain the EXACT code to be replaced. \
                         No abbreviations, ellipsis, or placeholder comments.",
                        i + 1,
                        pattern
                    ));
                }
            }
        }
        Ok(())
    }

    /// Verify exact bitwise match of SEARCH block against target document content.
    /// The "Zero-Loss" rule: if the text doesn't match exactly, reject immediately.
    pub fn verify_exact_match(content: &str, patches: &[PatchBlock]) -> Result<()> {
        for (i, patch) in patches.iter().enumerate() {
            let search_str = patch.search.trim();
            if search_str.is_empty() {
                continue;
            }
            if !content.contains(search_str) {
                return Err(anyhow!(
                    "SEARCH block {} does not match target document. \
                     Exact bitwise match required. The SEARCH block text \
                     ({} chars) was not found in the file. \
                     Re-read the file and provide the EXACT current code.",
                    i + 1,
                    search_str.len()
                ));
            }
        }
        Ok(())
    }

    // ── TASK 2: Universal LSP-Driven Validation Gate ──────────────────────

    /// Query the LSP for diagnostics on a given URI and extract error-severity items.
    pub async fn query_lsp_diagnostics(
        &self,
        uri: &str,
        wait_ms: u64,
    ) -> Vec<LspDiagnosticReport> {
        tokio::time::sleep(std::time::Duration::from_millis(wait_ms)).await;

        let map = self.lsp_diagnostics.read().await;
        let items = match map.get(uri) {
            Some(diags) => diags.clone(),
            None => return Vec::new(),
        };

        items
            .iter()
            .filter_map(|d| {
                let severity = d.get("severity")?.as_i64()?;
                let range = d.get("range")?;
                let start = range.get("start")?;
                let end = range.get("end")?;
                Some(LspDiagnosticReport {
                    uri: uri.to_string(),
                    range_start_line: start.get("line")?.as_i64().unwrap_or(0),
                    range_start_col: start.get("character")?.as_i64().unwrap_or(0),
                    range_end_line: end.get("line")?.as_i64().unwrap_or(0),
                    range_end_col: end.get("character")?.as_i64().unwrap_or(0),
                    severity,
                    message: d.get("message")?.as_str()?.to_string(),
                    code: d.get("code").and_then(|c| c.as_str().map(|s| s.to_string())),
                    source: d.get("source").and_then(|s| s.as_str().map(|s| s.to_string())),
                })
            })
            .filter(|r| r.is_error())
            .collect()
    }

    /// Build a self-correction prompt from LSP diagnostics to inject back into the AI agent.
    pub fn build_correction_prompt(
        diagnostics: &[LspDiagnosticReport],
        file_path: &str,
        iteration: u32,
    ) -> String {
        let mut prompt = format!(
            "COMPILATION ERROR in {} (correction attempt {}/{}):\n\n",
            file_path, iteration, MAX_CORRECTION_ITERATIONS
        );
        for (i, diag) in diagnostics.iter().enumerate() {
            prompt.push_str(&format!(
                "  ERROR {} (line {}, col {}): {}\n",
                i + 1,
                diag.range_start_line + 1,
                diag.range_start_col + 1,
                diag.message
            ));
            if let Some(code) = &diag.code {
                prompt.push_str(&format!("    code: {}\n", code));
            }
        }
        prompt.push_str(
            "\nFix ONLY the reported errors. Use precise SEARCH/REPLACE patches. \
             Do NOT emit placeholder comments like '// ... rest of code stays the same'.",
        );
        prompt
    }

    // ── Orchestrator: Full Firewall Pipeline ──────────────────────────────

    /// Run the complete firewall validation pipeline on a proposed edit.
    ///
    /// Steps:
    /// 1. Validate no placeholders in SEARCH blocks
    /// 2. Verify exact bitwise match of SEARCH blocks against file
    /// 3. Inject shadow content into LSP, query diagnostics
    /// 4. If errors: build correction prompt, return for AI retry
    /// 5. If clean: pass through for commit
    pub async fn validate_proposal(
        &mut self,
        file_path: &Path,
        original_content: &str,
        proposed_content: &str,
        patches: &[PatchBlock],
        session_id: &str,
        lsp_router: &Arc<Mutex<LspRouter>>,
    ) -> FirewallResult {
        let key = (file_path.to_path_buf(), session_id.to_string());
        let iteration = self.iteration_counts.entry(key.clone()).or_insert(0);
        *iteration += 1;
        let current_iter = *iteration;

        if current_iter > MAX_CORRECTION_ITERATIONS {
            self.iteration_counts.remove(&key);
            return FirewallResult {
                verdict: FirewallVerdict::HaltedMaxIterations,
                iteration: current_iter,
                message: format!(
                    "HALTED: Maximum self-correction iterations ({}) exceeded. \
                     Reverting all changes. Human architect intervention required.",
                    MAX_CORRECTION_ITERATIONS
                ),
                diagnostics: Vec::new(),
                diff: None,
            };
        }

        // Step 1: Placeholder detection
        if let Err(e) = Self::validate_no_placeholders(patches) {
            self.iteration_counts.remove(&key);
            return FirewallResult {
                verdict: FirewallVerdict::RejectedPlaceholder,
                iteration: current_iter,
                message: e.to_string(),
                diagnostics: Vec::new(),
                diff: None,
            };
        }

        // Step 2: Exact match verification
        if let Err(e) = Self::verify_exact_match(original_content, patches) {
            self.iteration_counts.remove(&key);
            return FirewallResult {
                verdict: FirewallVerdict::RejectedExactMatch,
                iteration: current_iter,
                message: e.to_string(),
                diagnostics: Vec::new(),
                diff: None,
            };
        }

        // Step 3: LSP diagnostic validation
        let uri = path_to_uri(file_path);
        {
            let mut router = lsp_router.lock().await;
            let _ = router
                .did_change(&uri, current_iter as i32, proposed_content)
                .await;
        }

        // Wait for LSP to process the change and emit diagnostics
        let errors = self.query_lsp_diagnostics(&uri, 500).await;

        if !errors.is_empty() {
            // Errors found — build correction prompt for AI retry
            let correction = Self::build_correction_prompt(
                &errors,
                &file_path.to_string_lossy(),
                current_iter,
            );
            return FirewallResult {
                verdict: FirewallVerdict::RejectedLspErrors,
                iteration: current_iter,
                message: correction,
                diagnostics: errors,
                diff: None,
            };
        }

        // Step 4: Clean — compute diff for audit trail
        let diff = diffy::create_patch(original_content, proposed_content);
        let diff_str = format!("{}", diff);

        // Clean up iteration counter on success
        self.iteration_counts.remove(&key);

        FirewallResult {
            verdict: FirewallVerdict::Accepted,
            iteration: current_iter,
            message: "All firewall checks passed. Proposal is safe to commit.".to_string(),
            diagnostics: Vec::new(),
            diff: Some(diff_str),
        }
    }

    /// Reset iteration counter for a session (e.g., on new chat turn or manual reset).
    pub fn reset_session(&mut self, session_id: &str) {
        self.iteration_counts
            .retain(|(_, sid), _| sid != session_id);
    }

    pub fn iteration_count(&self, file_path: &Path, session_id: &str) -> u32 {
        self.iteration_counts
            .get(&(file_path.to_path_buf(), session_id.to_string()))
            .copied()
            .unwrap_or(0)
    }
}

fn path_to_uri(path: &Path) -> String {
    let normalized = path.to_string_lossy().replace('\\', "/");
    if normalized.starts_with('/') {
        format!("file://{normalized}")
    } else {
        format!("file:///{normalized}")
    }
}
