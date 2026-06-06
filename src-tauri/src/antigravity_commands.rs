use std::fs;
use std::path::{Path, PathBuf};
use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::workspace_compat::resolve_specs_dir;
use crate::antigravity_compat::{
    append_trajectory_step, apply_autonomy_preset, dispatch_lifecycle_hooks,
    ensure_antigravity_scaffold, list_brain_artifacts, list_brain_cascades, list_trajectories,
    load_autonomy_policies, load_lifecycle_hooks, save_autonomy_policies, save_brain_artifact,
    save_brain_media, save_lifecycle_hooks, save_trajectory, upsert_subagent, AutonomyPolicies,
    BrainArtifactInfo, LifecycleHookResult, SubagentState, TrajectoryRecord, TrajectoryStep,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgTask {
    pub task_id: String,
    pub description: String,
    pub file_ref: Option<String>,
    pub done: bool,
    pub phase: String,
    pub tasks_path: String,
    pub spec_dir: String,
    pub line_index: usize,
}

fn scan_tasks_file(path: &std::path::Path, spec_dir: &str) -> Vec<AgTask> {
    let Ok(content) = fs::read_to_string(path) else {
        return vec![];
    };
    let mut tasks = Vec::new();
    let mut current_phase = String::from("Phase 1");

    for (i, line) in content.lines().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            current_phase = trimmed[3..].to_string();
        } else if trimmed.starts_with("- [ ]") || trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
            let done = !trimmed.starts_with("- [ ]");
            let rest = trimmed[5..].trim(); // skip "- [ ]" or "- [x]"

            let (task_id, desc_part) = if rest.starts_with("TASK-") {
                if let Some(colon_pos) = rest.find(':') {
                    (rest[..colon_pos].to_string(), rest[colon_pos + 1..].trim().to_string())
                } else {
                    (format!("TASK-{:03}", tasks.len() + 1), rest.to_string())
                }
            } else {
                (format!("TASK-{:03}", tasks.len() + 1), rest.to_string())
            };

            // Split on em dash " — " or " -- "
            let (description, file_ref) = if let Some(pos) = desc_part.find(" \u{2014} ") {
                (
                    desc_part[..pos].trim().to_string(),
                    Some(desc_part[pos + 3..].trim().to_string()),
                )
            } else if let Some(pos) = desc_part.find(" -- ") {
                (
                    desc_part[..pos].trim().to_string(),
                    Some(desc_part[pos + 4..].trim().to_string()),
                )
            } else {
                (desc_part, None)
            };

            tasks.push(AgTask {
                task_id,
                description,
                file_ref,
                done,
                phase: current_phase.clone(),
                tasks_path: path.to_string_lossy().to_string(),
                spec_dir: spec_dir.to_string(),
                line_index: i,
            });
        }
    }
    tasks
}

#[tauri::command]
pub fn ag_list_all_tasks(root: String) -> Result<Vec<AgTask>, String> {
    let root_path = PathBuf::from(&root);
    let mut all_tasks = Vec::new();

    for specs_dir in crate::workspace_compat::spec_roots(&root_path) {
        if !specs_dir.exists() {
            continue;
        }
        let mut entries: Vec<_> = fs::read_dir(&specs_dir)
            .map_err(|e| e.to_string())?
            .flatten()
            .filter(|e| e.path().is_dir())
            .collect();
        entries.sort_by_key(|e| e.file_name());
        for entry in entries {
            let spec_path = entry.path();
            let tasks_path = spec_path.join("tasks.md");
            if !tasks_path.exists() {
                // Kiro uses tasks.md at spec root — also try requirements/design layout without tasks
                continue;
            }
            let spec_dir = spec_path.to_string_lossy().to_string();
            all_tasks.extend(scan_tasks_file(&tasks_path, &spec_dir));
        }
    }

    Ok(all_tasks)
}

#[tauri::command]
pub fn ag_get_next_task(root: String) -> Result<Option<AgTask>, String> {
    let tasks = ag_list_all_tasks(root)?;
    Ok(tasks.into_iter().find(|t| !t.done))
}

#[tauri::command]
pub fn ag_mark_task_done(tasks_path: String, task_id: String) -> Result<(), String> {
    let content = fs::read_to_string(&tasks_path).map_err(|e| e.to_string())?;

    let new_lines: Vec<String> = content
        .lines()
        .map(|line| {
            let trimmed = line.trim();
            if trimmed.starts_with("- [ ]") && trimmed.contains(&task_id) {
                let indent_len = line.len() - line.trim_start().len();
                let indent = &line[..indent_len];
                format!("{}- [x]{}", indent, &trimmed[5..])
            } else {
                line.to_string()
            }
        })
        .collect();

    let mut final_content = new_lines.join("\n");
    if content.ends_with('\n') {
        final_content.push('\n');
    }

    fs::write(&tasks_path, final_content).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_create_spec(root: String, slug: String, description: String) -> Result<String, String> {
    let date = Local::now().format("%Y-%m-%d");
    let safe_slug = slug
        .replace(' ', "-")
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '-')
        .collect::<String>();
    let dir_name = format!("{}-{}", date, safe_slug);
    let spec_root = resolve_specs_dir(Path::new(&root));
    fs::create_dir_all(&spec_root).map_err(|e| e.to_string())?;
    let spec_dir = spec_root.join(&dir_name);

    let spec_md = format!(
        "# {}\n\n{}\n\n## Background\n\nTODO\n\n## Goals\n\n- TODO\n\n## Non-Goals\n\n- TODO\n\n## Acceptance Criteria\n\n- [ ] TODO\n",
        slug, description
    );
    let plan_md = format!(
        "# Implementation Plan: {}\n\n## Architecture\n\nTODO\n\n## Phases\n\n### Phase 1: Foundation\n\nTODO\n\n### Phase 2: Core Logic\n\nTODO\n",
        slug
    );
    let tasks_md = format!(
        "# Tasks: {}\n\n## Phase 1: Foundation\n\n- [ ] TASK-001: Scaffold initial structure \u{2014} TBD\n- [ ] TASK-002: Write failing tests \u{2014} tests/\n\n## Phase 2: Core Logic\n\n- [ ] TASK-003: Implement core functionality \u{2014} src/\n- [ ] TASK-004: Wire integration tests \u{2014} tests/\n",
        slug
    );

    fs::write(spec_dir.join("spec.md"), spec_md).map_err(|e| e.to_string())?;
    fs::write(spec_dir.join("plan.md"), plan_md).map_err(|e| e.to_string())?;
    fs::write(spec_dir.join("tasks.md"), tasks_md).map_err(|e| e.to_string())?;

    Ok(spec_dir.to_string_lossy().to_string())
}

#[tauri::command]
pub fn ag_phase_wrap(root: String, task_id: String, notes: String) -> Result<(), String> {
    let state_dir = PathBuf::from(&root).join(".hades");
    fs::create_dir_all(&state_dir).map_err(|e| e.to_string())?;

    let state_path = state_dir.join("state.md");
    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");

    let entry = format!(
        "\n## {} \u{2014} {}\n\n{}\n",
        timestamp, task_id, notes
    );

    let existing = fs::read_to_string(&state_path).unwrap_or_else(|_| {
        format!("# Hades Phase-Wrap State Log\n\nAuto-generated by the Antigravity workflow engine.\n")
    });
    let new_content = existing + &entry;
    fs::write(&state_path, new_content).map_err(|e| e.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowInfo {
    pub name: String,
    pub description: String,
    pub path: String,
    pub content: String,
}

#[tauri::command]
pub fn ag_get_workflows(root: String) -> Result<Vec<WorkflowInfo>, String> {
    let mut result = Vec::new();
    let dirs = [".agent/workflows", ".agents/workflows"];

    for dir in &dirs {
        let path = PathBuf::from(&root).join(dir);
        if !path.exists() {
            continue;
        }
        let Ok(entries) = fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_file() && p.extension().map(|e| e == "md").unwrap_or(false) {
                let name = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                let content = fs::read_to_string(&p).unwrap_or_default();
                let description = parse_frontmatter_field(&content, "description")
                    .or_else(|| extract_first_heading(&content))
                    .unwrap_or_else(|| name.clone());
                result.push(WorkflowInfo {
                    name,
                    description,
                    path: p.to_string_lossy().to_string(),
                    content,
                });
            }
        }
    }

    Ok(result)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RuleInfo {
    pub name: String,
    pub description: String,
    pub trigger: Option<String>,
    pub path: String,
    pub content: String,
}

#[tauri::command]
pub fn ag_get_rules(root: String) -> Result<Vec<RuleInfo>, String> {
    let mut result = Vec::new();
    let dirs = [
        ".agent/rules",
        ".agents/rules",
        ".cursor/rules",
        ".kiro/steering",
        ".agent/steering",
    ];

    for dir in &dirs {
        let path = PathBuf::from(&root).join(dir);
        if !path.exists() {
            continue;
        }
        let Ok(entries) = fs::read_dir(&path) else {
            continue;
        };
        for entry in entries.flatten() {
            let p = entry.path();
            let ext_ok = p
                .extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "md" || e == "mdc")
                .unwrap_or(false);
            if p.is_file() && ext_ok {
                let name = p.file_stem().unwrap_or_default().to_string_lossy().to_string();
                let content = fs::read_to_string(&p).unwrap_or_default();
                let description = parse_frontmatter_field(&content, "description")
                    .or_else(|| extract_first_heading(&content))
                    .unwrap_or_else(|| name.clone());
                let trigger = parse_frontmatter_field(&content, "trigger");
                result.push(RuleInfo {
                    name,
                    description,
                    trigger,
                    path: p.to_string_lossy().to_string(),
                    content,
                });
            }
        }
    }

    Ok(result)
}

fn parse_frontmatter_field(content: &str, field: &str) -> Option<String> {
    if !content.starts_with("---") {
        return None;
    }
    // find closing ---
    let after_open = &content[3..];
    let end = after_open.find("\n---")?;
    let frontmatter = &after_open[..end];
    for line in frontmatter.lines() {
        let trimmed = line.trim();
        let prefix = format!("{}:", field);
        if trimmed.starts_with(&prefix) {
            return Some(trimmed[prefix.len()..].trim().to_string());
        }
    }
    None
}

fn extract_first_heading(content: &str) -> Option<String> {
    content
        .lines()
        .find(|l| l.starts_with("# "))
        .map(|l| l[2..].trim().to_string())
}

// ── Antigravity brain / trajectory / hooks / autonomy ─────────────────────────

#[tauri::command]
pub fn ag_init_layout(root: String) -> Result<(), String> {
    ensure_antigravity_scaffold(Path::new(&root)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_brain_list(root: String, cascade_id: String) -> Result<Vec<BrainArtifactInfo>, String> {
    Ok(list_brain_artifacts(Path::new(&root), &cascade_id))
}

#[tauri::command]
pub fn ag_brain_list_cascades(root: String) -> Result<Vec<String>, String> {
    Ok(list_brain_cascades(Path::new(&root)))
}

#[tauri::command]
pub fn ag_brain_save_artifact(
    root: String,
    cascade_id: String,
    filename: String,
    content: String,
    artifact_type: String,
    summary: Option<String>,
) -> Result<String, String> {
    save_brain_artifact(
        Path::new(&root),
        &cascade_id,
        &filename,
        &content,
        &artifact_type,
        summary.as_deref(),
    )
    .map(|p| p.to_string_lossy().to_string())
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_brain_save_media(
    root: String,
    cascade_id: String,
    png_base64: String,
    summary: Option<String>,
) -> Result<String, String> {
    save_brain_media(
        Path::new(&root),
        &cascade_id,
        &png_base64,
        summary.as_deref(),
    )
    .map(|p| p.to_string_lossy().to_string())
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_list_trajectories(root: String) -> Result<Vec<TrajectoryRecord>, String> {
    Ok(list_trajectories(Path::new(&root)))
}

#[tauri::command]
pub fn ag_get_trajectory(root: String, cascade_id: String) -> Result<Option<TrajectoryRecord>, String> {
    Ok(crate::antigravity_compat::load_trajectory(Path::new(&root), &cascade_id))
}

#[tauri::command]
pub fn ag_save_trajectory(root: String, record: TrajectoryRecord) -> Result<String, String> {
    save_trajectory(Path::new(&root), &record)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_append_trajectory_step(
    root: String,
    cascade_id: String,
    step: TrajectoryStep,
) -> Result<TrajectoryRecord, String> {
    append_trajectory_step(Path::new(&root), &cascade_id, step).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_upsert_subagent(
    root: String,
    cascade_id: String,
    subagent: SubagentState,
) -> Result<(), String> {
    upsert_subagent(Path::new(&root), &cascade_id, subagent).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_load_lifecycle_hooks(root: String) -> Result<serde_json::Value, String> {
    Ok(load_lifecycle_hooks(Path::new(&root)))
}

#[tauri::command]
pub fn ag_save_lifecycle_hooks(root: String, hooks: serde_json::Value) -> Result<String, String> {
    save_lifecycle_hooks(Path::new(&root), &hooks)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_dispatch_lifecycle_hooks(
    root: String,
    event: String,
    context: String,
) -> Result<Vec<LifecycleHookResult>, String> {
    dispatch_lifecycle_hooks(Path::new(&root), &event, &context).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_get_autonomy_policies(root: String) -> Result<AutonomyPolicies, String> {
    Ok(load_autonomy_policies(Path::new(&root)))
}

#[tauri::command]
pub fn ag_save_autonomy_policies(root: String, policies: AutonomyPolicies) -> Result<String, String> {
    save_autonomy_policies(Path::new(&root), &policies)
        .map(|p| p.to_string_lossy().to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn ag_apply_autonomy_preset(
    root: String,
    preset: String,
    secure_mode: bool,
) -> Result<AutonomyPolicies, String> {
    let policies = apply_autonomy_preset(&preset, secure_mode);
    save_autonomy_policies(Path::new(&root), &policies).map_err(|e| e.to_string())?;
    Ok(policies)
}
