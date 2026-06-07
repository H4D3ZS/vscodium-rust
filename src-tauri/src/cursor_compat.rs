//! Cursor IDE project-layout compatibility.
//!
//! Mirrors conventions from Cursor 3.x bundled extensions (`cursor-retrieval`,
//! `cursor-always-local`, `cursor-mcp`, `cursor-ndjson-ingest`, etc.) so
//! workspaces opened from Cursor keep working without migration.

use anyhow::{anyhow, Context, Result};
use ignore::gitignore::{Gitignore, GitignoreBuilder};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

// ── Ignore files (`.cursorignore` vs `.cursorindexingignore`) ───────────────

#[derive(Clone, Debug)]
pub struct CursorIgnoreSet {
    root: PathBuf,
    matchers: Vec<Gitignore>,
}

#[derive(Clone, Copy, Debug)]
pub enum IgnoreScope {
    /// Indexing only: `.cursorindexingignore`, `.gitignore`, `.hadesignore`
    Indexing,
    /// AI tool access: indexing ignores + `.cursorignore`
    AiAccess,
}

/// Strip Windows extended-length `\\?\` prefix so `C:\foo` and `\\?\C:\foo` compare equal.
fn strip_verbatim_prefix(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    if let Some(rest) = s.strip_prefix(r"\\?\") {
        PathBuf::from(rest)
    } else {
        path.to_path_buf()
    }
}

/// Path relative to workspace root for gitignore matching. None when outside root.
fn relative_to_root(root: &Path, path: &Path) -> Option<PathBuf> {
    let root = strip_verbatim_prefix(root);
    let path = strip_verbatim_prefix(path);
    path.strip_prefix(&root).ok().map(|p| p.to_path_buf())
}

impl CursorIgnoreSet {
    pub fn load(root: &Path, scope: IgnoreScope) -> Self {
        let mut names: Vec<&str> = match scope {
            IgnoreScope::Indexing => vec![".hadesignore", ".cursorindexingignore", ".gitignore"],
            IgnoreScope::AiAccess => vec![
                ".hadesignore",
                ".cursorindexingignore",
                ".cursorignore",
                ".gitignore",
            ],
        };
        // Deduplicate while preserving order
        names.dedup();
        let mut matchers = Vec::new();
        for name in names {
            let p = root.join(name);
            if !p.is_file() {
                continue;
            }
            let mut b = GitignoreBuilder::new(root);
            let _ = b.add(&p);
            if let Ok(gi) = b.build() {
                matchers.push(gi);
            }
        }
        Self {
            root: strip_verbatim_prefix(root),
            matchers,
        }
    }

    pub fn is_ignored(&self, path: &Path) -> bool {
        if self.matchers.is_empty() {
            return false;
        }
        // ignore::Gitignore panics if the path is not under its root (common when
        // canonicalize() adds `\\?\` on Windows or the agent touches paths outside
        // the workspace during bug-bounty recon). Treat out-of-root as not ignored.
        let Some(rel) = relative_to_root(&self.root, path) else {
            return false;
        };
        let is_dir = path.is_dir();
        for gi in &self.matchers {
            match gi.matched_path_or_any_parents(&rel, is_dir) {
                ignore::Match::Ignore(_) => return true,
                _ => {}
            }
        }
        false
    }
}

// ── `.cursor/rules/*.mdc` frontmatter ───────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorRuleFrontmatter {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub globs: Option<String>,
    #[serde(default, rename = "alwaysApply")]
    pub always_apply: Option<bool>,
}

#[derive(Debug, Clone, Serialize)]
pub struct CursorRule {
    pub name: String,
    pub content: String,
    pub file_path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frontmatter: Option<CursorRuleFrontmatter>,
}

pub fn parse_mdc(raw: &str) -> (Option<CursorRuleFrontmatter>, String) {
    let trimmed = raw.trim_start();
    if !trimmed.starts_with("---") {
        return (None, raw.to_string());
    }
    let rest = &trimmed[3..];
    let end = rest.find("\n---");
    if end.is_none() {
        return (None, raw.to_string());
    }
    let end = end.unwrap();
    let yaml_block = rest[..end].trim();
    let body = rest[end + 4..].trim_start_matches('\n').trim_start();
    let mut fm = CursorRuleFrontmatter::default();
    for line in yaml_block.lines() {
        let line = line.trim();
        if let Some((k, v)) = line.split_once(':') {
            let key = k.trim();
            let val = v.trim().trim_matches('"').trim_matches('\'');
            match key {
                "description" => fm.description = Some(val.to_string()),
                "globs" => fm.globs = Some(val.to_string()),
                "alwaysApply" => {
                    fm.always_apply = Some(val == "true" || val == "1");
                }
                _ => {}
            }
        }
    }
    let has_fm = fm.description.is_some() || fm.globs.is_some() || fm.always_apply.is_some();
    (
        if has_fm { Some(fm) } else { None },
        body.to_string(),
    )
}

// ── `.cursor/environment.json` ────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CursorEnvironment {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub install: Option<String>,
    #[serde(default)]
    pub start: Option<String>,
    #[serde(default)]
    pub ports: Option<Vec<CursorPort>>,
    #[serde(default)]
    pub terminals: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CursorPort {
    pub port: u16,
    #[serde(default)]
    pub name: Option<String>,
}

pub fn load_environment(root: &Path) -> Option<CursorEnvironment> {
    let p = root.join(".cursor").join("environment.json");
    if !p.is_file() {
        return None;
    }
    let raw = fs::read_to_string(&p).ok()?;
    serde_json::from_str(&raw).ok()
}

pub fn format_environment_for_prompt(env: &CursorEnvironment) -> String {
    let mut out = String::from("\n### CURSOR DEV ENVIRONMENT (.cursor/environment.json):\n");
    if let Some(n) = &env.name {
        out.push_str(&format!("Name: {n}\n"));
    }
    if let Some(i) = &env.install {
        out.push_str(&format!("Install: `{i}`\n"));
    }
    if let Some(s) = &env.start {
        out.push_str(&format!("Start: `{s}`\n"));
    }
    if let Some(ports) = &env.ports {
        out.push_str("Ports:\n");
        for p in ports {
            let label = p.name.as_deref().unwrap_or("service");
            out.push_str(&format!("  - {label}: {}\n", p.port));
        }
    }
    if let Some(terminals) = &env.terminals {
        out.push_str(&format!("Agent terminals: {}\n", terminals));
    }
    out
}

// ── Project scan ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CursorProjectScan {
    pub root: String,
    pub has_cursor_dir: bool,
    pub rules_count: usize,
    pub rules: Vec<CursorRule>,
    pub has_environment: bool,
    pub environment: Option<CursorEnvironment>,
    pub has_mcp: bool,
    pub mcp_server_count: usize,
    pub has_cursorignore: bool,
    pub has_cursorindexingignore: bool,
    pub worktree_count: usize,
    pub skills_count: usize,
    pub commands_count: usize,
    pub hooks_count: usize,
}

pub fn scan_project(root: &Path) -> Result<CursorProjectScan> {
    let cursor_dir = root.join(".cursor");
    let rules = load_cursor_rules(root)?;
    let env = load_environment(root);
    let mcp_path = cursor_dir.join("mcp.json");
    let mcp_count = if mcp_path.is_file() {
        fs::read_to_string(&mcp_path)
            .ok()
            .and_then(|s| serde_json::from_str::<Value>(&s).ok())
            .and_then(|v| v.get("mcpServers").and_then(|m| m.as_object()).map(|o| o.len()))
            .unwrap_or(0)
    } else {
        0
    };

    Ok(CursorProjectScan {
        root: root.to_string_lossy().to_string(),
        has_cursor_dir: cursor_dir.is_dir(),
        rules_count: rules.len(),
        rules,
        has_environment: env.is_some(),
        environment: env,
        has_mcp: mcp_path.is_file(),
        mcp_server_count: mcp_count,
        has_cursorignore: root.join(".cursorignore").is_file(),
        has_cursorindexingignore: root.join(".cursorindexingignore").is_file(),
        worktree_count: count_entries(&cursor_dir.join("worktrees")),
        skills_count: count_entries(&cursor_dir.join("skills")),
        commands_count: count_entries(&cursor_dir.join("commands")),
        hooks_count: count_hook_files(&cursor_dir.join("hooks")),
    })
}

fn count_entries(dir: &Path) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    fs::read_dir(dir).map(|e| e.flatten().count()).unwrap_or(0)
}

fn count_hook_files(dir: &Path) -> usize {
    if !dir.is_dir() {
        return 0;
    }
    fs::read_dir(dir)
        .map(|entries| {
            entries
                .flatten()
                .filter(|e| e.path().is_file())
                .count()
        })
        .unwrap_or(0)
}

pub fn load_cursor_rules(root: &Path) -> Result<Vec<CursorRule>> {
    let rules_dir = root.join(".cursor").join("rules");
    let mut rules = Vec::new();
    if rules_dir.is_dir() {
        scan_rules_dir(&rules_dir, &mut rules)?;
    }
    // Legacy single-file at repo root
    for fname in [".cursorrules"] {
        let p = root.join(fname);
        if p.is_file() {
            let content = fs::read_to_string(&p)?;
            rules.push(CursorRule {
                name: "cursorrules".into(),
                content,
                file_path: p,
                frontmatter: None,
            });
        }
    }
    Ok(rules)
}

fn scan_rules_dir(dir: &Path, rules: &mut Vec<CursorRule>) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            scan_rules_dir(&path, rules)?;
            continue;
        }
        let ext_ok = path
            .extension()
            .and_then(|s| s.to_str())
            .map(|s| s == "md" || s == "mdc")
            .unwrap_or(false);
        if !ext_ok {
            continue;
        }
        let raw = fs::read_to_string(&path)?;
        let (frontmatter, content) = parse_mdc(&raw);
        let name = path.file_stem().unwrap_or_default().to_string_lossy().to_string();
        rules.push(CursorRule {
            name,
            content,
            file_path: path,
            frontmatter,
        });
    }
    Ok(())
}

pub fn format_rules_for_prompt(rules: &[CursorRule], active_file: Option<&str>) -> String {
    if rules.is_empty() {
        return String::new();
    }
    let mut out = String::from("\n### CURSOR WORKSPACE RULES:\n");
    for rule in rules {
        let always = rule
            .frontmatter
            .as_ref()
            .and_then(|f| f.always_apply)
            .unwrap_or(true);
        if !always {
            if let (Some(globs), Some(file)) = (
                rule.frontmatter.as_ref().and_then(|f| f.globs.as_deref()),
                active_file,
            ) {
                if !glob_matches_file(globs, file) {
                    continue;
                }
            } else if !always {
                continue;
            }
        }
        let title = rule
            .frontmatter
            .as_ref()
            .and_then(|f| f.description.as_ref())
            .map(|s| s.as_str())
            .unwrap_or(&rule.name);
        out.push_str(&format!("\n#### {title}\n{}\n", rule.content));
    }
    out
}

fn glob_matches_file(globs: &str, file: &str) -> bool {
    for part in globs.split(',') {
        let g = part.trim();
        if g.is_empty() {
            continue;
        }
        if glob::Pattern::new(g)
            .map(|p| p.matches(file))
            .unwrap_or(false)
        {
            return true;
        }
    }
    false
}

// ── NDJSON debug log (cursor-ndjson-ingest convention) ──────────────────────

pub fn append_debug_log(root: &Path, event: Value) -> Result<()> {
    let dir = root.join(".cursor");
    fs::create_dir_all(&dir)?;
    let path = dir.join("debug.log");
    let line = serde_json::to_string(&event)?;
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)?;
    writeln!(f, "{line}")?;
    Ok(())
}

// ── `.cursor/mcp.json` merge ────────────────────────────────────────────────

pub fn load_workspace_mcp(root: &Path) -> Option<Value> {
    let p = root.join(".cursor").join("mcp.json");
    if !p.is_file() {
        return None;
    }
    fs::read_to_string(&p).ok().and_then(|s| serde_json::from_str(&s).ok())
}

// ── Worktrees under `.cursor/worktrees/` ────────────────────────────────────

#[derive(Debug, Clone, Serialize)]
pub struct CursorWorktreeInfo {
    pub name: String,
    pub path: String,
    pub branch: Option<String>,
}

pub fn list_worktrees(root: &Path) -> Vec<CursorWorktreeInfo> {
    let base = root.join(".cursor").join("worktrees");
    if !base.is_dir() {
        return Vec::new();
    }
    fs::read_dir(&base)
        .into_iter()
        .flatten()
        .flatten()
        .filter(|e| e.path().is_dir())
        .map(|e| CursorWorktreeInfo {
            name: e.file_name().to_string_lossy().to_string(),
            path: e.path().to_string_lossy().to_string(),
            branch: read_worktree_branch(&e.path()),
        })
        .collect()
}

fn read_worktree_branch(path: &Path) -> Option<String> {
    let head = path.join(".git");
    if head.is_file() {
        let content = fs::read_to_string(&head).ok()?;
        if content.contains("worktrees") {
            return content.lines().last().map(|s| s.trim().to_string());
        }
    }
    None
}

pub fn create_worktree(root: &Path, branch: &str, name: Option<&str>) -> Result<CursorWorktreeInfo> {
    let wt_name = name.unwrap_or(branch).replace('/', "-");
    let base = root.join(".cursor").join("worktrees");
    fs::create_dir_all(&base)?;
    let target = base.join(&wt_name);
    if target.exists() {
        return Err(anyhow!("Worktree directory already exists: {}", target.display()));
    }

    let output = Command::new("git")
        .current_dir(root)
        .args(["worktree", "add", target.to_str().unwrap(), "-b", branch])
        .output()
        .context("failed to spawn git")?;

    if !output.status.success() {
        // Branch may exist — try attaching without -b
        let output2 = Command::new("git")
            .current_dir(root)
            .args(["worktree", "add", target.to_str().unwrap(), branch])
            .output()
            .context("failed to spawn git")?;
        if !output2.status.success() {
            let err = String::from_utf8_lossy(&output2.stderr);
            return Err(anyhow!("git worktree add failed: {err}"));
        }
    }

    Ok(CursorWorktreeInfo {
        name: wt_name,
        path: target.to_string_lossy().to_string(),
        branch: Some(branch.to_string()),
    })
}

// ── Scaffold default `.cursor/` layout ──────────────────────────────────────

pub fn ensure_scaffold(root: &Path) -> Result<Value> {
    let cursor = root.join(".cursor");
    fs::create_dir_all(cursor.join("rules"))?;
    fs::create_dir_all(cursor.join("skills"))?;
    fs::create_dir_all(cursor.join("commands"))?;
    fs::create_dir_all(cursor.join("hooks"))?;
    fs::create_dir_all(cursor.join("worktrees"))?;

    let gitignore_path = root.join(".cursorignore");
    if !gitignore_path.is_file() {
        fs::write(
            &gitignore_path,
            "# Cursor-compatible ignore — hidden from agent context\n\
             node_modules/\n\
             target/\n\
             dist/\n\
             .git/\n",
        )?;
    }

    let rules_readme = cursor.join("rules").join("README.mdc");
    if !rules_readme.is_file() {
        fs::write(
            &rules_readme,
            "---\n\
             description: Example workspace rule\n\
             alwaysApply: true\n\
             ---\n\
             # Project rules\n\
             \n\
             Add `.mdc` or `.md` files here. Cursor and vscodium-rust both load them.\n",
        )?;
    }

    Ok(json!({
        "status": "ok",
        "cursor_dir": cursor.to_string_lossy(),
        "created": ["rules", "skills", "commands", "hooks", "worktrees"]
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_mdc_frontmatter() {
        let raw = "---\ndescription: Test rule\nalwaysApply: true\n---\n\nHello world";
        let (fm, body) = parse_mdc(raw);
        assert!(fm.is_some());
        assert_eq!(body.trim(), "Hello world");
    }

    #[test]
    fn is_ignored_accepts_verbatim_canonical_paths() {
        let dir = std::env::temp_dir().join("cursor_compat_ignore_test");
        let _ = fs::create_dir_all(&dir);
        let ignore_file = dir.join(".gitignore");
        fs::write(&ignore_file, "secret.txt\n").unwrap();
        let set = CursorIgnoreSet::load(&dir, IgnoreScope::Indexing);
        let secret = dir.join("secret.txt");
        fs::write(&secret, "x").unwrap();
        let verbatim = PathBuf::from(format!(r"\\?\{}", secret.display()));
        assert!(set.is_ignored(&secret));
        assert!(set.is_ignored(&verbatim));
        assert!(!set.is_ignored(&PathBuf::from(r"C:\totally\outside\file.txt")));
        let _ = fs::remove_dir_all(&dir);
    }
}
