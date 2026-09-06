//! Agent skills — `SKILL.md` files with YAML frontmatter, discovered from the
//! workspace and made available to the agent via the `search_skills` / `skill`
//! tools.
//!
//! Layout (either directory is scanned):
//! ```text
//! <workspace>/.claude/skills/<name>/SKILL.md
//! <workspace>/.agent/skills/<name>/SKILL.md
//! ```
//!
//! Frontmatter is a leading `---` block. Only `name` and `description` are read;
//! `name` falls back to the directory name.

use std::fs;
use std::path::{Path, PathBuf};

const SKILL_DIRS: &[&str] = &[".claude/skills", ".agent/skills"];

#[derive(Debug, Clone, serde::Serialize)]
pub struct SkillMeta {
    pub name: String,
    pub description: String,
    /// Absolute path to the SKILL.md.
    pub path: String,
}

/// All skills discoverable under `workspace_root`. De-duplicated by name
/// (`.claude/skills` wins over `.agent/skills` on a clash).
pub fn discover(workspace_root: &Path) -> Vec<SkillMeta> {
    let mut out: Vec<SkillMeta> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

    for rel in SKILL_DIRS {
        let base = workspace_root.join(rel);
        let Ok(entries) = fs::read_dir(&base) else { continue };
        for entry in entries.filter_map(Result::ok) {
            if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                continue;
            }
            let dir_name = entry.file_name().to_string_lossy().to_string();
            let md = entry.path().join("SKILL.md");
            let Ok(text) = fs::read_to_string(&md) else { continue };
            let (name, description) = parse_frontmatter(&text, &dir_name);
            if seen.insert(name.clone()) {
                out.push(SkillMeta {
                    name,
                    description,
                    path: md.to_string_lossy().into_owned(),
                });
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));
    out
}

/// Skills whose name or description contains `query` (case-insensitive).
/// Empty query returns everything.
pub fn search(workspace_root: &Path, query: &str) -> Vec<SkillMeta> {
    let q = query.trim().to_lowercase();
    discover(workspace_root)
        .into_iter()
        .filter(|s| {
            q.is_empty()
                || s.name.to_lowercase().contains(&q)
                || s.description.to_lowercase().contains(&q)
        })
        .collect()
}

/// The SKILL.md body (frontmatter stripped) for the named skill.
pub fn load_body(workspace_root: &Path, name: &str) -> Option<String> {
    let want = name.trim();
    let meta = discover(workspace_root).into_iter().find(|s| s.name == want)?;
    let text = fs::read_to_string(PathBuf::from(&meta.path)).ok()?;
    Some(strip_frontmatter(&text).trim().to_string())
}

/// One `skill(name) — description` line per skill, for a system-prompt catalog.
pub fn catalog(workspace_root: &Path) -> String {
    let skills = discover(workspace_root);
    if skills.is_empty() {
        return String::new();
    }
    let mut s = String::from("## Skills\n\nCall `skill({\"name\": \"<name>\"})` to load one:\n\n");
    for k in &skills {
        s.push_str("  ");
        s.push_str(&k.name);
        if !k.description.is_empty() {
            s.push_str(" — ");
            s.push_str(first_line(&k.description));
        }
        s.push('\n');
    }
    s
}

// ── frontmatter parsing ─────────────────────────────────────────────────────

fn parse_frontmatter(text: &str, dir_name: &str) -> (String, String) {
    let mut name = dir_name.to_string();
    let mut description = String::new();
    if let Some(fm) = frontmatter_block(text) {
        for line in fm.lines() {
            let line = line.trim();
            if let Some(v) = line.strip_prefix("name:") {
                name = unquote(v.trim());
            } else if let Some(v) = line.strip_prefix("description:") {
                description = unquote(v.trim());
            }
        }
    }
    (name, description)
}

/// Contents between the leading `---` fences, if present.
fn frontmatter_block(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---")?.trim_start_matches(['\r', '\n']);
    // `---` alone stripped; now find the closing fence.
    let end = rest.find("\n---")?;
    Some(&rest[..end])
}

fn strip_frontmatter(text: &str) -> &str {
    if let Some(rest) = text.strip_prefix("---") {
        if let Some(pos) = rest.find("\n---") {
            let after = &rest[pos + 4..];
            return after.trim_start_matches(['-', '\r', '\n']);
        }
    }
    text
}

fn unquote(s: &str) -> String {
    let s = s.trim();
    s.strip_prefix('"')
        .and_then(|s| s.strip_suffix('"'))
        .or_else(|| s.strip_prefix('\'').and_then(|s| s.strip_suffix('\'')))
        .unwrap_or(s)
        .to_string()
}

fn first_line(s: &str) -> &str {
    s.split(['\n', '.']).next().unwrap_or(s).trim()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn write_skill(root: &Path, rel: &str, name: &str, body: &str) {
        let dir = root.join(rel).join(name);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("SKILL.md"),
            format!("---\nname: {name}\ndescription: \"does {name} things\"\n---\n\n{body}\n"),
        )
        .unwrap();
    }

    #[test]
    fn discover_parse_load() {
        let tmp = std::env::temp_dir().join(format!("kortex-skills-{}", uuid::Uuid::new_v4()));
        write_skill(&tmp, ".claude/skills", "alpha", "# Alpha\ndo the alpha.");
        write_skill(&tmp, ".agent/skills", "beta", "# Beta\ndo the beta.");

        let found = discover(&tmp);
        assert_eq!(found.len(), 2);
        assert_eq!(found[0].name, "alpha");
        assert_eq!(found[0].description, "does alpha things");

        let body = load_body(&tmp, "beta").unwrap();
        assert!(body.starts_with("# Beta"));
        assert!(!body.contains("description:"));

        assert!(catalog(&tmp).contains("alpha — does alpha things"));
        assert_eq!(search(&tmp, "beta").len(), 1);
        assert_eq!(search(&tmp, "").len(), 2);

        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn dedup_prefers_claude_dir() {
        let tmp = std::env::temp_dir().join(format!("kortex-skills-{}", uuid::Uuid::new_v4()));
        write_skill(&tmp, ".agent/skills", "dup", "agent version");
        write_skill(&tmp, ".claude/skills", "dup", "claude version");
        let found = discover(&tmp);
        assert_eq!(found.len(), 1);
        assert!(load_body(&tmp, "dup").unwrap().contains("claude version"));
        let _ = fs::remove_dir_all(&tmp);
    }

    #[test]
    fn missing_dirs_are_fine() {
        let tmp = std::env::temp_dir().join(format!("kortex-skills-none-{}", uuid::Uuid::new_v4()));
        assert!(discover(&tmp).is_empty());
        assert!(catalog(&tmp).is_empty());
        assert!(load_body(&tmp, "x").is_none());
    }

    #[test]
    fn name_falls_back_to_dir() {
        let tmp = std::env::temp_dir().join(format!("kortex-skills-{}", uuid::Uuid::new_v4()));
        let dir = tmp.join(".claude/skills/nofm");
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("SKILL.md"), "# No frontmatter\njust body").unwrap();
        let found = discover(&tmp);
        assert_eq!(found[0].name, "nofm");
        assert_eq!(found[0].description, "");
        let _ = fs::remove_dir_all(&tmp);
    }
}
