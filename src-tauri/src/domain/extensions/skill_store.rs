//! HADES Skill Store — user-installed agentskills.io / skills.sh compatible skills.
//!
//! Installed under `{HADES_HOME}/skills/installed/` with registry + security audit.

use crate::ide_shell;
use crate::skill_audit::{audit_report_to_json, audit_skill_tree, SkillAuditReport};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillInstallRecord {
    pub id: String,
    pub name: String,
    pub description: String,
    pub source: String,
    pub install_source: String,
    pub installed_at: String,
    pub path: String,
    pub red_team_skill: bool,
    pub safe_to_use: bool,
    pub blocked: bool,
    pub audit_summary: String,
    pub warning_count: usize,
    pub critical_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct SkillRegistry {
    version: u32,
    skills: Vec<SkillInstallRecord>,
}

pub fn skills_home() -> PathBuf {
    ide_shell::hades_home().join("skills")
}

pub fn installed_dir() -> PathBuf {
    skills_home().join("installed")
}

fn registry_path() -> PathBuf {
    skills_home().join("registry.json")
}

fn load_registry() -> SkillRegistry {
    let path = registry_path();
    if !path.is_file() {
        return SkillRegistry {
            version: 1,
            ..Default::default()
        };
    }
    fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

fn save_registry(reg: &SkillRegistry) -> Result<(), String> {
    fs::create_dir_all(skills_home()).map_err(|e| e.to_string())?;
    let data = serde_json::to_string_pretty(reg).map_err(|e| e.to_string())?;
    fs::write(registry_path(), data).map_err(|e| e.to_string())
}

fn parse_frontmatter_name_desc(content: &str) -> (String, String) {
    let mut name = String::new();
    let mut desc = String::new();
    let body = content.trim();
    if !body.starts_with("---") {
        return (name, desc);
    }
    if let Some(end) = body[3..].find("\n---") {
        for line in body[3..3 + end].lines() {
            if let Some((k, v)) = line.split_once(':') {
                match k.trim().to_lowercase().as_str() {
                    "name" => name = v.trim().trim_matches('"').to_string(),
                    "description" => desc = v.trim().trim_matches('"').to_string(),
                    _ => {}
                }
            }
        }
    }
    (name, desc)
}

fn find_skill_md_root(dir: &Path) -> Option<PathBuf> {
    let direct = dir.join("SKILL.md");
    if direct.is_file() {
        return Some(dir.to_path_buf());
    }
    for entry in walkdir::WalkDir::new(dir)
        .max_depth(4)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_name().to_str() == Some("SKILL.md") {
            return entry.path().parent().map(|p| p.to_path_buf());
        }
    }
    None
}

fn slugify_id(raw: &str) -> String {
    raw.trim()
        .to_lowercase()
        .replace('\\', "/")
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '/' || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .trim_matches('-')
        .to_string()
}

fn git_url_from_source(source: &str) -> Result<String, String> {
    let s = source.trim();
    if s.is_empty() {
        return Err("Empty install source".into());
    }
    if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("git@") {
        return Ok(s.to_string());
    }
    if s.contains('/') && !s.contains('\\') && !Path::new(s).exists() {
        let parts: Vec<_> = s.split('/').collect();
        if parts.len() == 2 {
            return Ok(format!("https://github.com/{}/{}.git", parts[0], parts[1]));
        }
        if parts.len() >= 3 && parts[0].eq_ignore_ascii_case("github.com") {
            return Ok(format!("https://github.com/{}/{}.git", parts[1], parts[2]));
        }
    }
    Err(format!(
        "Unrecognized source `{s}`. Use owner/repo, https://github.com/..., or a local folder path."
    ))
}

fn resolve_git_exe() -> Option<PathBuf> {
    if let Some(bash) = ide_shell::resolve_git_bash_exe() {
        if let Some(git_root) = bash.parent().and_then(|p| p.parent()) {
            let git = git_root.join("cmd").join("git.exe");
            if git.is_file() {
                return Some(git);
            }
            let git2 = git_root.join("bin").join("git.exe");
            if git2.is_file() {
                return Some(git2);
            }
        }
    }
    which::which("git").ok()
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<(), String> {
    fs::create_dir_all(dst).map_err(|e| e.to_string())?;
    for entry in walkdir::WalkDir::new(src)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let rel = entry
            .path()
            .strip_prefix(src)
            .map_err(|e| e.to_string())?;
        if rel.as_os_str().is_empty() {
            continue;
        }
        if rel
            .components()
            .any(|c| c.as_os_str() == std::ffi::OsStr::new(".git"))
        {
            continue;
        }
        let target = dst.join(rel);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target).map_err(|e| e.to_string())?;
        } else {
            if let Some(parent) = target.parent() {
                let _ = fs::create_dir_all(parent);
            }
            fs::copy(entry.path(), &target).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

fn remove_dir_all(path: &Path) -> Result<(), String> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn record_from_audit(
    id: &str,
    skill_root: &Path,
    install_source: &str,
    report: &SkillAuditReport,
    name: &str,
    description: &str,
) -> SkillInstallRecord {
    SkillInstallRecord {
        id: id.to_string(),
        name: if name.is_empty() {
            id.split('/').next_back().unwrap_or(id).to_string()
        } else {
            name.to_string()
        },
        description: description.to_string(),
        source: "skill-store".to_string(),
        install_source: install_source.to_string(),
        installed_at: chrono::Utc::now().to_rfc3339(),
        path: skill_root.to_string_lossy().to_string(),
        red_team_skill: report.red_team_skill,
        safe_to_use: report.safe_to_use,
        blocked: report.blocked,
        audit_summary: report.summary.clone(),
        warning_count: report.warning_count,
        critical_count: report.critical_count,
    }
}

pub fn install_skill_from_path(
    src: &Path,
    id_hint: Option<&str>,
    force: bool,
) -> Result<(SkillInstallRecord, SkillAuditReport), String> {
    if !src.is_dir() {
        return Err(format!("Not a directory: {}", src.display()));
    }
    let skill_root = find_skill_md_root(src).ok_or("No SKILL.md found in source")?;
    let content = fs::read_to_string(skill_root.join("SKILL.md")).map_err(|e| e.to_string())?;
    let (name, description) = parse_frontmatter_name_desc(&content);
    let default_id = skill_root
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("skill");
    let id = slugify_id(id_hint.unwrap_or(default_id));
    if id.is_empty() {
        return Err("Could not derive skill id".into());
    }

    let report = audit_skill_tree(&skill_root, &id);
    if report.blocked && !force {
        return Err(format!(
            "Install blocked: {} critical finding(s). Set force=true to override after manual review.",
            report.critical_count
        ));
    }

    fs::create_dir_all(installed_dir()).map_err(|e| e.to_string())?;
    let dest = installed_dir().join(&id);
    remove_dir_all(&dest)?;
    copy_dir_all(&skill_root, &dest)?;

    let final_report = audit_skill_tree(&dest, &id);
    let record = record_from_audit(
        &id,
        &dest,
        &src.to_string_lossy(),
        &final_report,
        &name,
        &description,
    );

    let mut reg = load_registry();
    reg.skills.retain(|s| s.id != id);
    reg.skills.push(record.clone());
    reg.version = 1;
    save_registry(&reg)?;
    crate::hermes_skills::invalidate_skill_cache();

    Ok((record, final_report))
}

pub fn install_skill_from_git(
    source: &str,
    id_hint: Option<&str>,
    force: bool,
) -> Result<(SkillInstallRecord, SkillAuditReport), String> {
    let git_url = git_url_from_source(source)?;
    let git_exe = resolve_git_exe().ok_or(
        "Git not available. Bundled PortableGit installs on first IDE launch — restart and retry.",
    )?;

    let tmp = skills_home().join("_tmp_clone");
    remove_dir_all(&tmp)?;
    fs::create_dir_all(&tmp).map_err(|e| e.to_string())?;

    let status = Command::new(&git_exe)
        .args(["clone", "--depth", "1", &git_url, &tmp.to_string_lossy()])
        .status()
        .map_err(|e| format!("git clone failed: {e}"))?;
    if !status.success() {
        remove_dir_all(&tmp)?;
        return Err(format!("git clone failed for {git_url}"));
    }

    let id = id_hint.map(|s| s.to_string()).or_else(|| {
        git_url
            .trim_end_matches(".git")
            .rsplit('/')
            .next()
            .map(slugify_id)
    });
    let result = install_skill_from_path(&tmp, id.as_deref(), force);
    remove_dir_all(&tmp)?;
    result
}

pub fn uninstall_skill(id: &str) -> Result<(), String> {
    let id = slugify_id(id);
    let dest = installed_dir().join(&id);
    remove_dir_all(&dest)?;
    let mut reg = load_registry();
    reg.skills.retain(|s| s.id != id);
    save_registry(&reg)?;
    crate::hermes_skills::invalidate_skill_cache();
    Ok(())
}

pub fn list_installed() -> Vec<SkillInstallRecord> {
    load_registry().skills
}

pub fn skill_store_roots() -> Vec<PathBuf> {
    let dir = installed_dir();
    if dir.is_dir() {
        vec![dir]
    } else {
        vec![]
    }
}

#[tauri::command]
pub fn skill_store_status() -> Result<Value, String> {
    let installed = list_installed();
    Ok(json!({
        "storeDir": skills_home().to_string_lossy(),
        "installedDir": installed_dir().to_string_lossy(),
        "installedCount": installed.len(),
        "registryPath": registry_path().to_string_lossy(),
        "agentskillsSpec": "https://agentskills.io/",
        "skillsSh": "https://www.skills.sh/",
    }))
}

#[tauri::command]
pub fn skill_store_list() -> Result<Value, String> {
    Ok(json!({ "skills": list_installed() }))
}

#[tauri::command]
pub fn skill_store_audit(id: String) -> Result<Value, String> {
    let id = slugify_id(&id);
    let path = installed_dir().join(&id);
    if !path.is_dir() {
        return Err(format!("Skill not installed: {id}"));
    }
    let report = audit_skill_tree(&path, &id);
    Ok(audit_report_to_json(&report))
}

#[tauri::command]
pub fn skill_store_install(source: String, id: Option<String>, force: Option<bool>) -> Result<Value, String> {
    let force = force.unwrap_or(false);
    let path = PathBuf::from(source.trim());
    let (record, report) = if path.is_dir() {
        install_skill_from_path(&path, id.as_deref(), force)?
    } else {
        install_skill_from_git(&source, id.as_deref(), force)?
    };
    Ok(json!({
        "ok": true,
        "skill": record,
        "audit": audit_report_to_json(&report),
    }))
}

#[tauri::command]
pub fn skill_store_uninstall(id: String) -> Result<Value, String> {
    uninstall_skill(&id)?;
    Ok(json!({ "ok": true, "id": slugify_id(&id) }))
}

#[tauri::command]
pub fn skill_store_refresh() -> Result<Value, String> {
    crate::hermes_skills::invalidate_skill_cache();
    let mut reg = load_registry();
    let mut updated = Vec::new();
    for mut rec in reg.skills.clone() {
        let path = PathBuf::from(&rec.path);
        if !path.join("SKILL.md").is_file() {
            continue;
        }
        let report = audit_skill_tree(&path, &rec.id);
        rec.red_team_skill = report.red_team_skill;
        rec.safe_to_use = report.safe_to_use;
        rec.blocked = report.blocked;
        rec.audit_summary = report.summary.clone();
        rec.warning_count = report.warning_count;
        rec.critical_count = report.critical_count;
        updated.push(rec);
    }
    reg.skills = updated;
    save_registry(&reg)?;
    Ok(json!({ "ok": true, "count": reg.skills.len() }))
}
