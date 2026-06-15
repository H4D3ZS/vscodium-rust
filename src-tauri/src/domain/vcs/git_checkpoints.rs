// =============================================================================
// Git Checkpoints - Auto-Snapshot & Rollback for AI Edits
// Similar to Cursor's checkpoint system for safe AI-driven changes
// =============================================================================

use anyhow::{anyhow, Result};
use chrono::{DateTime, Local, TimeZone};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: String,
    pub name: String,
    pub description: String,
    pub timestamp: u64,
    pub datetime: String,
    pub commit_hash: String,
    pub parent_hash: Option<String>,
    pub files_changed: usize,
    pub is_ai_generated: bool,
    pub can_rollback: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckpointDiff {
    pub files: Vec<FileDiff>,
    pub total_additions: usize,
    pub total_deletions: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileDiff {
    pub path: String,
    pub status: String, // "added", "modified", "deleted", "renamed"
    pub additions: usize,
    pub deletions: usize,
    pub patch: Option<String>,
}

pub struct GitCheckpoint {
    repo_path: PathBuf,
}

impl GitCheckpoint {
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    // ── Create Checkpoint Before AI Edit ─────────────────────────────────

    pub fn create_checkpoint(&self, description: &str, is_ai: bool) -> Result<Checkpoint> {
        // Ensure we're in a git repo
        self.ensure_git_repo()?;

        // Stash any uncommitted changes first
        self.stash_uncommitted_changes()?;

        // Create a unique checkpoint name
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let datetime: DateTime<Local> = Local::now();
        let checkpoint_name = if is_ai {
            format!("checkpoint/ai-{}-{}", timestamp, description.replace(" ", "-"))
        } else {
            format!("checkpoint/manual-{}-{}", timestamp, description.replace(" ", "-"))
        };

        // Get current HEAD hash
        let parent_hash = self.get_head_hash().ok();

        // Create the checkpoint (git commit with special message)
        let status = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["add", "-A"])
            .output()?;

        if !status.status.success() {
            return Err(anyhow!("Failed to stage files"));
        }

        let commit_status = Command::new("git")
            .current_dir(&self.repo_path)
            .args([
                "commit",
                "-m",
                &checkpoint_name,
                "-m",
                &format!("Description: {}", description),
                "-m",
                &format!("AI-Generated: {}", is_ai),
                "-m",
                &format!("Timestamp: {}", timestamp),
            ])
            .output()?;

        if !commit_status.status.success() {
            // If nothing to commit, create a tag anyway
            let err_msg = String::from_utf8_lossy(&commit_status.stderr);
            if err_msg.contains("nothing to commit") {
                // Create a lightweight tag at current HEAD
                let head = self.get_head_hash()?;
                Command::new("git")
                    .current_dir(&self.repo_path)
                    .args(["tag", &checkpoint_name, &head])
                    .output()?;

                return Ok(Checkpoint {
                    id: checkpoint_name.clone(),
                    name: description.to_string(),
                    description: description.to_string(),
                    timestamp,
                    datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
                    commit_hash: head,
                    parent_hash,
                    files_changed: 0,
                    is_ai_generated: is_ai,
                    can_rollback: true,
                });
            }
            return Err(anyhow!("Failed to create checkpoint: {}", err_msg));
        }

        // Get the new commit hash
        let commit_hash = self.get_head_hash()?;

        // Get files changed
        let files_changed = self.get_files_changed_since(parent_hash.as_deref())?;

        // Create a git tag for easy identification
        Command::new("git")
            .current_dir(&self.repo_path)
            .args(["tag", &checkpoint_name, &commit_hash])
            .output()?;

        Ok(Checkpoint {
            id: checkpoint_name.clone(),
            name: description.to_string(),
            description: description.to_string(),
            timestamp,
            datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            commit_hash: commit_hash.clone(),
            parent_hash,
            files_changed,
            is_ai_generated: is_ai,
            can_rollback: true,
        })
    }

    // ── Rollback to Checkpoint ───────────────────────────────────────────

    pub fn rollback_to_checkpoint(&self, checkpoint_id: &str) -> Result<String> {
        // Verify the checkpoint/tag exists
        let tag_exists = self.tag_exists(checkpoint_id)?;
        if !tag_exists {
            return Err(anyhow!("Checkpoint '{}' not found", checkpoint_id));
        }

        // Get the commit hash for this tag
        let commit_hash = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["rev-list", "-n", "1", checkpoint_id])
            .output()?;

        let hash = String::from_utf8_lossy(&commit_hash.stdout).trim().to_string();

        // Perform a soft reset to the checkpoint (keeps changes in staging)
        let reset_status = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["reset", "--soft", &hash])
            .output()?;

        if !reset_status.status.success() {
            return Err(anyhow!("Failed to rollback to checkpoint"));
        }

        // Create a new commit documenting the rollback
        let _rollback_commit = Command::new("git")
            .current_dir(&self.repo_path)
            .args([
                "commit",
                "-m",
                &format!("rollback/to-{} - Manual rollback", checkpoint_id),
            ])
            .output()?;

        let new_hash = self.get_head_hash()?;

        Ok(format!(
            "Successfully rolled back to checkpoint '{}'. New commit: {}",
            checkpoint_id, new_hash
        ))
    }

    // ── List All Checkpoints ─────────────────────────────────────────────

    pub fn list_checkpoints(&self, limit: Option<usize>) -> Result<Vec<Checkpoint>> {
        // Get all tags that start with "checkpoint/"
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["tag", "-l", "checkpoint/*", "--sort=-creatordate"])
            .output()?;

        let tags: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        let limited_tags = if let Some(lim) = limit {
            tags.into_iter().take(lim).collect::<Vec<_>>()
        } else {
            tags
        };

        let mut checkpoints = Vec::new();

        for tag in &limited_tags {
            if let Ok(checkpoint) = self.get_checkpoint_info(tag) {
                checkpoints.push(checkpoint);
            }
        }

        Ok(checkpoints)
    }

    // ── Get Checkpoint Info ──────────────────────────────────────────────

    fn get_checkpoint_info(&self, tag: &str) -> Result<Checkpoint> {
        // Get commit hash
        let hash_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["rev-list", "-n", "1", tag])
            .output()?;

        let commit_hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();

        // Get commit message
        let message_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["log", "-1", "--format=%B", &commit_hash])
            .output()?;

        let message = String::from_utf8_lossy(&message_output.stdout);
        let lines: Vec<&str> = message.lines().collect();

        let name = lines.first().unwrap_or(&"").to_string();
        let description = lines
            .iter()
            .find(|l| l.starts_with("Description: "))
            .map(|l| l.trim_start_matches("Description: "))
            .unwrap_or("")
            .to_string();

        let is_ai = lines
            .iter()
            .find(|l| l.starts_with("AI-Generated: "))
            .map(|l| *l == "AI-Generated: true")
            .unwrap_or(false);

        let timestamp = lines
            .iter()
            .find(|l| l.starts_with("Timestamp: "))
            .and_then(|l| l.trim_start_matches("Timestamp: ").parse().ok())
            .unwrap_or(0);

        let datetime: DateTime<Local> = Local
            .timestamp_opt(timestamp as i64, 0)
            .single()
            .unwrap_or(Local::now());

        // Get parent hash
        let parent_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["log", "-1", "--format=%P", &commit_hash])
            .output()?;

        let parent_hash_str = String::from_utf8_lossy(&parent_output.stdout).trim().to_string();
        let parent_hash = if parent_hash_str.is_empty() {
            None
        } else {
            Some(parent_hash_str)
        };

        // Get files changed from parent
        let files_changed = self.get_files_changed_since(parent_hash.as_deref())?;

        Ok(Checkpoint {
            id: tag.to_string(),
            name,
            description,
            timestamp,
            datetime: datetime.format("%Y-%m-%d %H:%M:%S").to_string(),
            commit_hash,
            parent_hash,
            files_changed,
            is_ai_generated: is_ai,
            can_rollback: true,
        })
    }

    // ── Get Diff Between Checkpoint and Current State ────────────────────

    pub fn get_checkpoint_diff(&self, checkpoint_id: &str) -> Result<CheckpointDiff> {
        // Get the commit hash
        let hash_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["rev-list", "-n", "1", checkpoint_id])
            .output()?;

        let commit_hash = String::from_utf8_lossy(&hash_output.stdout).trim().to_string();

        // Get diff stats
        let diff_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["diff", "--stat", &commit_hash])
            .output()?;

        let diff_stat = String::from_utf8_lossy(&diff_output.stdout);

        // Get actual diff
        let patch_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["diff", "--unified=3", &commit_hash])
            .output()?;

        let _patch = String::from_utf8_lossy(&patch_output.stdout);

        // Parse the diff
        let mut files = Vec::new();
        let mut total_additions = 0;
        let mut total_deletions = 0;

        for line in diff_stat.lines() {
            if let Some(sep_pos) = line.rfind('|') {
                let file_path = line[..sep_pos].trim();
                let changes = &line[sep_pos + 1..];

                let additions = changes.chars().filter(|&c| c == '+').count();
                let deletions = changes.chars().filter(|&c| c == '-').count();

                total_additions += additions;
                total_deletions += deletions;

                let status = if line.contains("Bin") {
                    "modified".to_string()
                } else {
                    "modified".to_string()
                };

                files.push(FileDiff {
                    path: file_path.to_string(),
                    status,
                    additions,
                    deletions,
                    patch: None,
                });
            }
        }

        Ok(CheckpointDiff {
            files,
            total_additions,
            total_deletions,
        })
    }

    // ── Delete Checkpoint ────────────────────────────────────────────────

    pub fn delete_checkpoint(&self, checkpoint_id: &str) -> Result<()> {
        if !self.tag_exists(checkpoint_id)? {
            return Err(anyhow!("Checkpoint not found"));
        }

        Command::new("git")
            .current_dir(&self.repo_path)
            .args(["tag", "-d", checkpoint_id])
            .output()?;

        Ok(())
    }

    // ── Auto-Checkpoint Before AI Action ─────────────────────────────────

    pub fn auto_checkpoint_before_ai_edit(&self, description: &str) -> Result<Option<Checkpoint>> {
        // Only create checkpoint if there are staged or modified files
        let status_output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["status", "--porcelain"])
            .output()?;

        let status = String::from_utf8_lossy(&status_output.stdout);
        if status.trim().is_empty() {
            return Ok(None); // Nothing to checkpoint
        }

        let checkpoint = self.create_checkpoint(description, true)?;
        Ok(Some(checkpoint))
    }

    // ── Helper Methods ───────────────────────────────────────────────────

    fn ensure_git_repo(&self) -> Result<()> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["rev-parse", "--git-dir"])
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Not a git repository"));
        }

        Ok(())
    }

    fn stash_uncommitted_changes(&self) -> Result<()> {
        // Check if there are uncommitted changes
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["status", "--porcelain"])
            .output()?;

        if !String::from_utf8_lossy(&output.stdout).trim().is_empty() {
            // Stage all changes before checkpoint
            Command::new("git")
                .current_dir(&self.repo_path)
                .args(["add", "-A"])
                .output()?;
        }

        Ok(())
    }

    fn get_head_hash(&self) -> Result<String> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["rev-parse", "HEAD"])
            .output()?;

        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if hash.is_empty() {
            Err(anyhow!("No HEAD found"))
        } else {
            Ok(hash)
        }
    }

    fn tag_exists(&self, tag: &str) -> Result<bool> {
        let output = Command::new("git")
            .current_dir(&self.repo_path)
            .args(["tag", "-l", tag])
            .output()?;

        let tags = String::from_utf8_lossy(&output.stdout);
        Ok(tags.lines().any(|t| t.trim() == tag))
    }

    fn get_files_changed_since(&self, since_hash: Option<&str>) -> Result<usize> {
        let mut cmd = Command::new("git");
        cmd.current_dir(&self.repo_path);

        if let Some(hash) = since_hash {
            cmd.args(["diff", "--name-only", hash]);
        } else {
            cmd.args(["ls-files"]);
        }

        let output = cmd.output()?;
        let files = String::from_utf8_lossy(&output.stdout);
        Ok(files.lines().filter(|l| !l.trim().is_empty()).count())
    }
}
