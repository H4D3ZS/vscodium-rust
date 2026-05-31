use serde_json::{json, Value};
use std::process::Command;
use std::path::Path;

pub struct BinaryAnalyzer;

impl BinaryAnalyzer {
    /// Disassemble a binary using radare2 (if available) or objdump.
    pub fn disassemble(path: &Path) -> Result<String, String> {
        if !path.exists() {
            return Err("Binary file not found".to_string());
        }

        // Try radare2 first (more powerful for research)
        if let Ok(output) = Command::new("r2")
            .arg("-qc")
            .arg("pd 100") // Print 100 instructions from entry
            .arg(path)
            .output()
        {
            if output.status.success() {
                return Ok(String::from_utf8_lossy(&output.stdout).to_string());
            }
        }

        // Fallback to objdump
        let output = Command::new("objdump")
            .arg("-d")
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to execute objdump: {}", e))?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }

    /// Get basic info about a binary (file type, architecture, etc.)
    pub fn get_info(path: &Path) -> Result<Value, String> {
        let output = Command::new("file")
            .arg(path)
            .output()
            .map_err(|e| format!("Failed to execute 'file' command: {}", e))?;

        if output.status.success() {
            let result = String::from_utf8_lossy(&output.stdout).to_string();
            Ok(json!({ "info": result.trim() }))
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}
