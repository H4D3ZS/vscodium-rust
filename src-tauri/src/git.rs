use std::process::Command;
use std::path::Path;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct GitFileStatus {
    pub path: String,
    pub status: String, // "M", "A", "D", "??"
}

pub struct GitManager;

impl GitManager {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status<P: AsRef<Path>>(&self, repo_path: P) -> Result<Vec<GitFileStatus>, String> {
        let output = Command::new("git")
            .arg("status")
            .arg("--porcelain")
            .current_dir(repo_path)
            .output()
            .map_err(|e| format!("Failed to execute git: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut results = Vec::new();

        for line in stdout.lines() {
            if line.len() > 3 {
                let status = line[0..2].trim().to_string();
                let path = line[3..].to_string();
                results.push(GitFileStatus { path, status });
            }
        }

        Ok(results)
    }

    pub fn stage<P: AsRef<Path>>(&self, repo_path: P, file_path: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("add")
            .arg(file_path)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() { Ok(()) } else { Err("Git add failed".to_string()) }
    }

    pub fn unstage<P: AsRef<Path>>(&self, repo_path: P, file_path: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("reset")
            .arg("HEAD")
            .arg(file_path)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() { Ok(()) } else { Err("Git reset failed".to_string()) }
    }

    pub fn commit<P: AsRef<Path>>(&self, repo_path: P, message: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() { Ok(()) } else { Err("Git commit failed".to_string()) }
    }
}
