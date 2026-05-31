use std::fs;
use std::path::PathBuf;
use chrono::Local;
use serde::{Deserialize, Serialize};

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
    let specs_dir = PathBuf::from(&root).join("specs");
    if !specs_dir.exists() {
        return Ok(vec![]);
    }

    let mut all_tasks = Vec::new();
    let mut entries: Vec<_> = fs::read_dir(&specs_dir)
        .map_err(|e| e.to_string())?
        .flatten()
        .filter(|e| e.path().is_dir())
        .collect();

    // Sort by directory name (date-based slugs sort chronologically)
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let spec_path = entry.path();
        let tasks_path = spec_path.join("tasks.md");
        if tasks_path.exists() {
            let spec_dir = spec_path.to_string_lossy().to_string();
            let tasks = scan_tasks_file(&tasks_path, &spec_dir);
            all_tasks.extend(tasks);
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
    let spec_dir = PathBuf::from(&root).join("specs").join(&dir_name);

    fs::create_dir_all(&spec_dir).map_err(|e| e.to_string())?;

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
    let dirs = [".agent/rules", ".agents/rules", ".cursor/rules"];

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
