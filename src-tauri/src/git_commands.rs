use tauri::State;
use crate::EditorState;
use crate::git::{self, GitManager};
use std::process::Command;
use crate::process_ext::CommandExtHidden;
use serde_json::{json, Value};

#[tauri::command]
pub async fn git_status(path: String) -> Result<Vec<git::GitFileStatus>, String> {
    let manager = GitManager::new();
    manager.get_status(path)
}

#[tauri::command]
pub async fn git_stage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.stage(path, &file_path)
}

#[tauri::command]
pub async fn git_unstage(path: String, file_path: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.unstage(path, &file_path)
}

#[tauri::command]
pub async fn git_commit(path: String, message: String) -> Result<(), String> {
    let manager = GitManager::new();
    manager.commit(path, &message)
}

#[tauri::command]
pub async fn get_git_branch() -> Result<String, String> {
    let output = Command::new("git")
        .hidden()
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .map_err(|_| "Git not found".to_string())?;
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tauri::command]
pub async fn git_get_history(path: String) -> Result<Vec<git::GitCommitInfo>, String> {
    let manager = GitManager::new();
    manager.get_history(path)
}

#[tauri::command]
pub async fn git_diff(path: String, hash: String) -> Result<String, String> {
    let manager = GitManager::new();
    manager.get_commit_diff(path, &hash)
}

#[tauri::command]
pub async fn git_blame(path: String, file_path: String) -> Result<Vec<String>, String> {
    let output = Command::new("git")
        .hidden()
        .args(&["blame", "--porcelain", "--", &file_path])
        .current_dir(&path)
        .output()
        .map_err(|e| e.to_string())?;

    let raw = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<String> = Vec::new();
    let mut current_hash = String::new();
    let mut current_author = String::new();
    let mut current_date = String::new();
    let mut current_summary = String::new();

    for line in raw.lines() {
        if line.starts_with('\t') {
            lines.push(format!("{}|{}|{}|{}", current_hash, current_author, current_date, current_summary));
        } else if line.len() >= 40 && line.chars().next().map(|c| c.is_ascii_hexdigit()).unwrap_or(false) {
            current_hash = line[..40].to_string();
        } else if let Some(rest) = line.strip_prefix("author ") {
            current_author = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("author-time ") {
            if let Ok(ts) = rest.trim().parse::<i64>() {
                let secs = ts;
                let days = secs / 86400;
                let y400 = days / 146097;
                let r400 = days % 146097;
                let y100 = (r400 / 36524).min(3);
                let r100 = r400 - y100 * 36524;
                let y4 = r100 / 1461;
                let r4 = r100 % 1461;
                let y1 = (r4 / 365).min(3);
                let year = y400 * 400 + y100 * 100 + y4 * 4 + y1 + 1970;
                let doy = r4 - y1 * 365 + 1;
                let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
                let month_days: [i64; 12] = if leap {
                    [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                } else {
                    [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
                };
                let mut month = 1i64;
                let mut d = doy;
                for md in &month_days {
                    if d <= *md { break; }
                    d -= md;
                    month += 1;
                }
                current_date = format!("{}-{:02}-{:02}", year, month, d);
            }
        } else if let Some(rest) = line.strip_prefix("summary ") {
            current_summary = rest.trim().chars().take(60).collect();
        }
    }

    Ok(lines)
}

#[tauri::command]
pub async fn git_diff_file(path: String, file_path: String) -> Result<String, String> {
    let output = Command::new("git")
        .hidden()
        .args(&["diff", "HEAD", "--", &file_path])
        .current_dir(&path)
        .output()
        .map_err(|e| e.to_string())?;
    if output.stdout.is_empty() {
        let output2 = Command::new("git")
            .hidden()
            .args(&["diff", "--", &file_path])
            .current_dir(&path)
            .output()
            .map_err(|e| e.to_string())?;
        return Ok(String::from_utf8_lossy(&output2.stdout).to_string());
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

#[tauri::command]
pub async fn git_revert(state: State<'_, EditorState>, hash: String) -> Result<(), String> {
    let root_lock = state.active_root.lock().await;
    let root = root_lock.clone().ok_or("No active project")?;
    GitManager::new().revert_commit(root, &hash)
}

#[tauri::command]
pub async fn git_stash(state: State<'_, EditorState>) -> Result<(), String> {
    let root_lock = state.active_root.lock().await;
    let root = root_lock.clone().ok_or("No active project")?;
    GitManager::new().stash_changes(root)
}

#[tauri::command]
pub async fn git_stash_pop(state: State<'_, EditorState>) -> Result<(), String> {
    let root_lock = state.active_root.lock().await;
    let root = root_lock.clone().ok_or("No active project")?;
    GitManager::new().pop_stash(root)
}

#[tauri::command]
pub async fn git_get_unmerged(state: State<'_, EditorState>) -> Result<Vec<String>, String> {
    let root_lock = state.active_root.lock().await;
    let root = root_lock.clone().ok_or("No active project")?;
    GitManager::new().get_unmerged_files(root)
}

#[tauri::command]
pub async fn git_clone(url: String, path: String) -> Result<(), String> {
    GitManager::new().clone(&url, path)
}
#[tauri::command]
pub async fn git_create_checkpoint(
    state: State<'_, EditorState>,
    description: String,
    is_ai: Option<bool>,
) -> Result<Value, String> {
    let is_ai = is_ai.unwrap_or(false);
    let checkpoint = state.git_checkpoints
        .create_checkpoint(&description, is_ai)
        .map_err(|e| format!("Failed to create checkpoint: {}", e))?;

    if is_ai {
        println!("[CHECKPOINT] ✅ AI checkpoint created: {}", checkpoint.id);
    }

    Ok(json!(checkpoint))
}

#[tauri::command]
pub async fn git_list_checkpoints(
    state: State<'_, EditorState>,
    limit: Option<usize>,
) -> Result<Value, String> {
    let checkpoints = state.git_checkpoints
        .list_checkpoints(limit)
        .map_err(|e| format!("Failed to list checkpoints: {}", e))?;

    Ok(json!(checkpoints))
}

#[tauri::command]
pub async fn git_rollback_checkpoint(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<String, String> {
    state.git_checkpoints
        .rollback_to_checkpoint(&checkpoint_id)
        .map_err(|e| format!("Failed to rollback: {}", e))
}

#[tauri::command]
pub async fn git_get_checkpoint_diff(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<Value, String> {
    let diff = state.git_checkpoints
        .get_checkpoint_diff(&checkpoint_id)
        .map_err(|e| format!("Failed to get diff: {}", e))?;

    Ok(json!(diff))
}

#[tauri::command]
pub async fn git_delete_checkpoint(
    state: State<'_, EditorState>,
    checkpoint_id: String,
) -> Result<(), String> {
    state.git_checkpoints
        .delete_checkpoint(&checkpoint_id)
        .map_err(|e| format!("Failed to delete checkpoint: {}", e))
}

#[tauri::command]
pub async fn git_auto_checkpoint(
    state: State<'_, EditorState>,
    description: String,
) -> Result<Value, String> {
    let result = state.git_checkpoints
        .auto_checkpoint_before_ai_edit(&description)
        .map_err(|e| format!("Failed to auto-checkpoint: {}", e))?;

    Ok(json!({
        "checkpoint": result,
        "created": result.is_some()
    }))
}
#[tauri::command]
pub async fn get_git_file_hunks(path: String) -> Result<Value, String> {
    // git diff -U0 HEAD -- <path>  gives us exact line ranges
    let output = Command::new("git")
        .hidden()
        .args(["diff", "-U0", "HEAD", "--", &path])
        .output()
        .map_err(|e| e.to_string())?;

    let diff = String::from_utf8_lossy(&output.stdout);
    let mut added: Vec<u32> = Vec::new();
    let mut modified: Vec<u32> = Vec::new();
    let mut deleted: Vec<u32> = Vec::new();

    // Parse unified diff @@ -a,b +c,d @@ headers
    for line in diff.lines() {
        if let Some(hunk) = line.strip_prefix("@@ ") {
            // Format: -<old_start>[,<old_count>] +<new_start>[,<new_count>]
            let parts: Vec<&str> = hunk.split_whitespace().collect();
            let old_part = parts.first().unwrap_or(&"-0");
            let new_part = parts.get(1).unwrap_or(&"+0");

            let parse_range = |s: &str| -> (u32, u32) {
                let s = s.trim_start_matches(['+', '-']);
                if let Some((start, count)) = s.split_once(',') {
                    (start.parse().unwrap_or(0), count.parse().unwrap_or(1))
                } else {
                    (s.parse().unwrap_or(0), 1)
                }
            };

            let (_old_start, old_count) = parse_range(old_part);
            let (new_start, new_count) = parse_range(new_part);

            if old_count == 0 {
                // Pure addition
                for l in new_start..new_start + new_count { added.push(l); }
            } else if new_count == 0 {
                // Pure deletion — mark the line before deletion
                deleted.push(if new_start == 0 { 1 } else { new_start });
            } else {
                // Modification
                for l in new_start..new_start + new_count { modified.push(l); }
            }
        }
    }

    // Also check for untracked (new file) — git diff shows nothing for staged new files
    if diff.is_empty() {
        // Check if file is untracked
        let status = Command::new("git")
            .hidden()
            .args(["status", "--porcelain", &path])
            .output()
            .map_err(|e| e.to_string())?;
        let st = String::from_utf8_lossy(&status.stdout);
        if st.starts_with("??") || st.starts_with("A ") {
            // Whole file is new — mark all lines as added
            // Return a sentinel so frontend knows to mark all
            return Ok(json!({ "new_file": true, "added": [], "modified": [], "deleted": [] }));
        }
    }

    Ok(json!({ "new_file": false, "added": added, "modified": modified, "deleted": deleted }))
}
