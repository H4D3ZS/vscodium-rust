use anyhow::{anyhow, Result};
use std::path::PathBuf;
use std::process::Command;
use crate::process_ext::CommandExtHidden;
use serde::{Deserialize, Serialize};
use std::time::Instant;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandResult {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
    pub exit_code: Option<i32>,
    pub duration_ms: u128,
}

pub struct GhostRuntime {
    project_root: PathBuf,
}

impl GhostRuntime {
    pub fn new(project_root: PathBuf) -> Self {
        Self { project_root }
    }

    /// Executes a command in a hidden window, capturing output.
    /// Commands are scoped to the project root.
    pub async fn execute(&self, cmd_str: &str, _timeout_secs: u64) -> Result<CommandResult> {
        let start = Instant::now();
        
        // Security check: simple check for destructive commands
        if self.is_destructive(cmd_str) {
            return Err(anyhow!("Command is flagged as potentially destructive and rejected by Ghost Runtime security."));
        }

        let final_cmd = if cfg!(target_os = "windows") {
            self.map_command(cmd_str)
        } else {
            cmd_str.to_string()
        };

        let child = if cfg!(target_os = "windows") {
            Command::new("cmd")
                .args(["/C", &final_cmd])
                .current_dir(&self.project_root)
                .hidden()
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?
        } else {
            Command::new("sh")
                .args(["-c", &final_cmd])
                .current_dir(&self.project_root)
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?
        };

        let output = child.wait_with_output()?;
        let duration = start.elapsed().as_millis();

        Ok(CommandResult {
            success: output.status.success(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            exit_code: output.status.code(),
            duration_ms: duration,
        })
    }

    fn is_destructive(&self, cmd: &str) -> bool {
        let destructive_patterns = [
            "rm -rf", "rd /s", "del /s", "format", "mkfs", "dd",
            "> /dev/", "chmod -R 777", "chown -R"
        ];
        
        let cmd_lower = cmd.to_lowercase();
        destructive_patterns.iter().any(|pattern| cmd_lower.contains(pattern))
    }

    fn map_command(&self, cmd: &str) -> String {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        if parts.is_empty() { return cmd.to_string(); }
        
        match parts[0] {
            "ls" => cmd.replace("ls", "dir /B"),
            "cat" => cmd.replace("cat", "type"),
            "grep" => cmd.replace("grep", "findstr"),
            "rm" if parts.get(1) == Some(&"-rf") => cmd.replace("rm -rf", "rd /s /q"),
            "rm" => cmd.replace("rm", "del /q"),
            "cp" => cmd.replace("cp", "copy"),
            "mv" => cmd.replace("mv", "move"),
            "sudo" => cmd.replacen("sudo", "", 1).trim().to_string(),
            _ => cmd.to_string()
        }
    }
}
