//! Native Hermes skills integration — loads SKILL.md from vendored `hermes-agent/` tree,
//! bundled installer resources, and user Skill Store installs.

use crate::ide_shell;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

static EXCLUDED: &[&str] = &[
    ".git", ".github", ".venv", "venv", "node_modules", "__pycache__", "site-packages",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegratedSkill {
    pub id: String,
    pub name: String,
    pub description: String,
    pub path: String,
    pub category: String,
    pub source: String,
}

static SKILL_CACHE: RwLock<Option<Vec<IntegratedSkill>>> = RwLock::new(None);

pub fn invalidate_skill_cache() {
    if let Ok(mut g) = SKILL_CACHE.write() {
        *g = None;
    }
}

fn repo_roots() -> Vec<PathBuf> {
    let mut roots = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.to_path_buf());
            roots.push(dir.join("resources"));
        }
    }
    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        if let Some(root) = PathBuf::from(manifest).parent() {
            roots.push(root.to_path_buf());
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.clone());
        if let Some(p) = cwd.parent() {
            roots.push(p.to_path_buf());
        }
    }
    roots
}

pub fn hermes_skills_roots() -> Vec<(PathBuf, &'static str)> {
    let mut out = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for root in repo_roots() {
        for (rel, tag) in [
            ("hermes-agent/skills", "hermes-bundled"),
            ("hermes-agent/optional-skills", "hermes-optional"),
            ("bundles/hermes-skills/skills", "bundled"),
            ("bundles/hermes-skills/optional-skills", "bundled-optional"),
            ("resources/bundles/hermes-skills/skills", "bundled"),
        ] {
            let p = root.join(rel);
            let key = p.to_string_lossy().to_string();
            if p.is_dir() && seen.insert(key) {
                out.push((p, tag));
            }
        }
    }

    if let Some(p) = skill_store_installed_root() {
        out.push((p, "skill-store"));
    }
    out
}

fn skill_store_installed_root() -> Option<PathBuf> {
    let dir = ide_shell::hades_home().join("skills").join("installed");
    if dir.is_dir() {
        Some(dir)
    } else {
        None
    }
}

fn skill_store_installed_count() -> usize {
    let dir = ide_shell::hades_home().join("skills").join("installed");
    if !dir.is_dir() {
        return 0;
    }
    std::fs::read_dir(&dir)
        .map(|rd| {
            rd.filter_map(|e| e.ok())
                .filter(|e| e.path().join("SKILL.md").is_file())
                .count()
        })
        .unwrap_or(0)
}

fn is_excluded(path: &Path) -> bool {
    path.components().any(|c| {
        EXCLUDED
            .iter()
            .any(|e| c.as_os_str() == std::ffi::OsStr::new(e))
    })
}

fn parse_frontmatter(content: &str) -> (HashMap<String, String>, String) {
    let mut meta = HashMap::new();
    let body = content.trim();
    if !body.starts_with("---") {
        return (meta, body.to_string());
    }
    if let Some(end) = body[3..].find("\n---") {
        let fm = &body[3..3 + end];
        for line in fm.lines() {
            if let Some((k, v)) = line.split_once(':') {
                meta.insert(k.trim().to_lowercase(), v.trim().trim_matches('"').to_string());
            }
        }
        let rest = body[3 + end + 4..].trim_start();
        return (meta, rest.to_string());
    }
    (meta, body.to_string())
}

fn skill_id_from_path(skill_md: &Path, root: &Path, source: &str) -> String {
    if source == "skill-store" {
        return skill_md
            .parent()
            .and_then(|p| p.file_name())
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "skill".to_string());
    }
    skill_md
        .parent()
        .and_then(|p| p.strip_prefix(root).ok())
        .map(|rel| rel.to_string_lossy().replace('\\', "/"))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            skill_md
                .parent()
                .and_then(|p| p.file_name())
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "skill".to_string())
        })
}

pub fn scan_integrated_skills() -> Vec<IntegratedSkill> {
    let mut skills: Vec<IntegratedSkill> = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (root, source) in hermes_skills_roots() {
        let walker = walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok());
        for entry in walker {
            let path = entry.path();
            if path.file_name().and_then(|n| n.to_str()) != Some("SKILL.md") {
                continue;
            }
            if is_excluded(path) {
                continue;
            }
            let Ok(content) = std::fs::read_to_string(path) else {
                continue;
            };
            let (meta, _) = parse_frontmatter(&content);
            let id = skill_id_from_path(path, &root, source);
            if !seen.insert(id.clone()) {
                continue;
            }
            let name = meta
                .get("name")
                .cloned()
                .unwrap_or_else(|| id.split('/').next_back().unwrap_or(&id).to_string());
            let description = meta.get("description").cloned().unwrap_or_default();
            let category = id.split('/').next().unwrap_or("general").to_string();
            skills.push(IntegratedSkill {
                id: id.clone(),
                name,
                description,
                path: path.to_string_lossy().to_string(),
                category,
                source: source.to_string(),
            });
        }
    }
    skills.sort_by(|a, b| a.id.cmp(&b.id));
    skills
}

fn cached_skills() -> Vec<IntegratedSkill> {
    {
        let read = SKILL_CACHE.read().unwrap();
        if let Some(ref v) = *read {
            return v.clone();
        }
    }
    let skills = scan_integrated_skills();
    *SKILL_CACHE.write().unwrap() = Some(skills.clone());
    skills
}

pub fn find_skill(id_or_name: &str) -> Option<IntegratedSkill> {
    let q = id_or_name.trim().to_lowercase();
    cached_skills().into_iter().find(|s| {
        s.id.to_lowercase() == q
            || s.name.to_lowercase() == q
            || s.id.to_lowercase().ends_with(&format!("/{q}"))
            || s.id
                .split('/')
                .next_back()
                .map(|t| t.to_lowercase() == q)
                .unwrap_or(false)
    })
}

pub fn load_skill_body(id_or_name: &str) -> Option<String> {
    let skill = find_skill(id_or_name)?;
    let content = std::fs::read_to_string(&skill.path).ok()?;
    let (_, body) = parse_frontmatter(&content);
    Some(body)
}

/// Compact catalog for system prompt (Hermes-style progressive disclosure).
pub fn build_skills_catalog_for_prompt(max_entries: usize) -> String {
    let skills = cached_skills();
    if skills.is_empty() {
        return String::new();
    }
    let home = ide_shell::hades_home().to_string_lossy().to_string();
    let mut out = format!(
        "\n### HADES INTEGRATED SKILLS (agentskills.io — native, no subprocess)\n\
         Skill Store: `{home}/skills/installed`. Install via Settings → Skill Store or \
         `use_skill` with id/folder name. Full SKILL.md loads on invoke.\n",
    );
    for s in skills.iter().take(max_entries) {
        let desc = if s.description.len() > 120 {
            format!("{}…", s.description.chars().take(120).collect::<String>())
        } else {
            s.description.clone()
        };
        out.push_str(&format!("- **{}** ({}) — {}\n", s.id, s.source, desc));
    }
    if skills.len() > max_entries {
        out.push_str(&format!(
            "… and {} more — use `search_skills` to find by keyword.\n",
            skills.len() - max_entries
        ));
    }
    out
}

pub fn search_skills(query: &str, limit: usize) -> Vec<IntegratedSkill> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return cached_skills().into_iter().take(limit).collect();
    }
    cached_skills()
        .into_iter()
        .filter(|s| {
            s.id.to_lowercase().contains(&q)
                || s.name.to_lowercase().contains(&q)
                || s.description.to_lowercase().contains(&q)
                || s.category.to_lowercase().contains(&q)
        })
        .take(limit)
        .collect()
}

#[cfg(feature = "tauri")]
#[cfg(feature = "tauri")]
#[tauri::command]
pub fn hermes_skills_list(limit: Option<usize>) -> Result<Value, String> {
    let all = cached_skills();
    let lim = limit.unwrap_or(200).min(500);
    let list: Vec<_> = all.iter().take(lim).collect();
    Ok(json!({
        "count": all.len(),
        "skills": list,
        "roots": hermes_skills_roots().iter().map(|(p, s)| json!({ "path": p.to_string_lossy(), "source": s })).collect::<Vec<_>>(),
    }))
}

#[cfg(feature = "tauri")]
#[cfg(feature = "tauri")]
#[tauri::command]
pub fn hermes_skills_get(id: String) -> Result<Value, String> {
    let skill = find_skill(&id).ok_or_else(|| format!("Skill not found: {id}"))?;
    let body = load_skill_body(&id).unwrap_or_default();
    Ok(json!({
        "skill": skill,
        "body": body,
    }))
}

#[cfg(feature = "tauri")]
#[cfg(feature = "tauri")]
#[tauri::command]
pub fn hermes_skills_search(query: String, limit: Option<usize>) -> Result<Value, String> {
    let hits = search_skills(&query, limit.unwrap_or(20).min(50));
    Ok(json!({ "query": query, "results": hits }))
}

#[cfg(feature = "tauri")]
#[cfg(feature = "tauri")]
#[tauri::command]
pub fn hermes_integration_status() -> Result<Value, String> {
    let roots = hermes_skills_roots();
    let installed = skill_store_installed_count();
    Ok(json!({
        "mode": "native",
        "subprocess": false,
        "skillsCount": cached_skills().len(),
        "storeInstalledCount": installed,
        "skillStoreDir": ide_shell::hades_home().join("skills").to_string_lossy(),
        "skillRoots": roots.iter().map(|(p, s)| json!({ "path": p.to_string_lossy(), "source": s, "exists": p.is_dir() })).collect::<Vec<_>>(),
        "license": "MIT (hermes-agent bundled content); user skills per upstream license",
        "spec": "https://agentskills.io/",
        "marketplace": "https://www.skills.sh/",
    }))
}
