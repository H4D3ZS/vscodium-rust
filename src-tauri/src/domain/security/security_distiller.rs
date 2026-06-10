use serde_json::{json, Value};

pub struct SecurityDistiller;

impl SecurityDistiller {
    /// Audits a file content for high-risk patterns and returns a "Risk Weight".
    /// This weight is used to adjust the Pythagorean geometric coordinates' priority.
    pub fn audit_content(content: &str) -> f32 {
        let mut risk_score: f32 = 0.0;

        // Pattern 1: Unsafe Usage (Traditional CVE source)
        if content.contains("unsafe {") {
            risk_score += 0.4;
        }

        // Pattern 2: Panic Prone Logic
        if content.contains(".unwrap()") || content.contains(".expect(") {
            risk_score += 0.2;
        }

        // Pattern 3: Sensitive Data Handling (Mock pattern)
        if content.contains("password") || content.contains("secret") || content.contains("token") {
            risk_score += 0.3;
        }

        // Pattern 4: Memory Transmutation
        if content.contains("mem::transmute") {
            risk_score += 0.5;
        }

        risk_score.min(1.0)
    }

    /// Generates security metadata for the context indexer.
    pub fn get_security_metadata(content: &str) -> serde_json::Value {
        let risk = Self::audit_content(content);
        json!({
            "risk_level": risk,
            "security_tier": if risk > 0.7 { "CRITICAL" } else if risk > 0.3 { "ELEVATED" } else { "STANDARD" },
            "audit_needed": risk > 0.5
        })
    }

    /// Runs semgrep on a file/directory and returns the output.
    pub fn run_semgrep(path: &std::path::Path) -> Result<Value, String> {
        let output = std::process::Command::new("semgrep")
            .arg("scan")
            .arg("--json")
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to execute semgrep: {}. Is it installed?", e))?;

        if !output.status.success() && output.stderr.len() > 0 {
             return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        serde_json::from_slice(&output.stdout).map_err(|e| format!("Failed to parse semgrep JSON: {}", e))
    }

    /// Runs cargo audit on the current workspace and returns the output.
    pub fn run_cargo_audit(workspace_root: &std::path::Path) -> Result<Value, String> {
        let output = std::process::Command::new("cargo")
            .arg("audit")
            .arg("--json")
            .current_dir(workspace_root)
            .output()
            .map_err(|e| format!("Failed to execute cargo audit: {}. Is it installed?", e))?;

        serde_json::from_slice(&output.stdout).map_err(|e| format!("Failed to parse cargo audit JSON: {}", e))
    }
}
