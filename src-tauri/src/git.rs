use serde::Serialize;
use std::path::Path;
use std::process::Command;

#[derive(Serialize, Debug)]
pub struct GitFileStatus {
    pub path: String,
    pub status: String, // "M", "A", "D", "??"
}

#[derive(Serialize, Debug)]
pub struct GitCommitInfo {
    pub hash: String,
    pub author: String,
    pub date: String,
    pub message: String,
    pub parents: Vec<String>,
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
        if status.success() {
            Ok(())
        } else {
            Err("Git add failed".to_string())
        }
    }

    pub fn unstage<P: AsRef<Path>>(&self, repo_path: P, file_path: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("reset")
            .arg("HEAD")
            .arg(file_path)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git reset failed".to_string())
        }
    }

    pub fn commit<P: AsRef<Path>>(&self, repo_path: P, message: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git commit failed".to_string())
        }
    }

    pub fn get_history<P: AsRef<Path>>(&self, repo_path: P) -> Result<Vec<GitCommitInfo>, String> {
        let output = Command::new("git")
            .arg("log")
            .arg("--format=%H|%an|%ai|%s|%P")
            .arg("-n")
            .arg("50")
            .current_dir(repo_path)
            .output()
            .map_err(|e| format!("Failed to execute git log: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut history = Vec::new();

        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                history.push(GitCommitInfo {
                    hash: parts[0].to_string(),
                    author: parts[1].to_string(),
                    date: parts[2].to_string(),
                    message: parts[3].to_string(),
                    parents: parts
                        .get(4)
                        .unwrap_or(&"")
                        .split_whitespace()
                        .map(|s| s.to_string())
                        .collect(),
                });
            }
        }

        Ok(history)
    }
    pub fn revert_commit<P: AsRef<Path>>(&self, repo_path: P, hash: &str) -> Result<(), String> {
        let status = Command::new("git")
            .arg("revert")
            .arg("--no-edit")
            .arg(hash)
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git revert failed".to_string())
        }
    }

    pub fn stash_changes<P: AsRef<Path>>(&self, repo_path: P) -> Result<(), String> {
        let status = Command::new("git")
            .arg("stash")
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git stash failed".to_string())
        }
    }

    pub fn pop_stash<P: AsRef<Path>>(&self, repo_path: P) -> Result<(), String> {
        let status = Command::new("git")
            .arg("stash")
            .arg("pop")
            .current_dir(repo_path)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git stash pop failed".to_string())
        }
    }

    pub fn get_unmerged_files<P: AsRef<Path>>(&self, repo_path: P) -> Result<Vec<String>, String> {
        let output = Command::new("git")
            .arg("diff")
            .arg("--name-only")
            .arg("--diff-filter=U")
            .current_dir(repo_path)
            .output()
            .map_err(|e| format!("Failed to execute git: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.lines().map(|s| s.to_string()).collect())
    }

    pub fn clone<P: AsRef<Path>>(&self, url: &str, dest: P) -> Result<(), String> {
        let status = Command::new("git")
            .arg("clone")
            .arg(url)
            .arg(".")
            .current_dir(dest)
            .status()
            .map_err(|e| e.to_string())?;
        if status.success() {
            Ok(())
        } else {
            Err("Git clone failed".to_string())
        }
    }

    pub fn get_commit_diff<P: AsRef<Path>>(
        &self,
        repo_path: P,
        hash: &str,
    ) -> Result<String, String> {
        let output = Command::new("git")
            .arg("show")
            .arg("--format=")
            .arg(hash)
            .current_dir(repo_path)
            .output()
            .map_err(|e| format!("Failed to execute git show: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}
