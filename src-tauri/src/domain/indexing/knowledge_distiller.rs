use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::{Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct KnowledgeBrief {
    pub title: String,
    pub date: String,
    pub findings: String,
    pub affected_files: Vec<String>,
}

pub struct KnowledgeDistiller {
    base_path: PathBuf,
}

impl KnowledgeDistiller {
    pub fn new(project_root: &Path) -> Self {
        let knowledge_path = project_root.join(".kortex").join("knowledge");
        if !knowledge_path.exists() {
            let _ = fs::create_dir_all(&knowledge_path);
        }
        Self {
            base_path: knowledge_path,
        }
    }

    pub fn save_finding(&self, brief: KnowledgeBrief) -> Result<String, String> {
        let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("{}_{}.md", timestamp, brief.title.replace(" ", "_").to_lowercase());
        let file_path = self.base_path.join(filename);

        let content = format!(
            "# {}\n\n**Date:** {}\n**Affected Files:** {:?}\n\n## Findings\n{}\n",
            brief.title, brief.date, brief.affected_files, brief.findings
        );

        fs::write(&file_path, content)
            .map_err(|e| format!("Failed to write knowledge file: {}", e))?;

        Ok(file_path.to_string_lossy().to_string())
    }

    /// Query-relevant, recency-ranked brief selection. This is what makes
    /// cross-session memory *compound* instead of degrade: rather than dumping
    /// every brief and truncating in arbitrary directory order (which drops the
    /// useful ones once history grows), we score each brief by keyword overlap
    /// with the current task × a recency decay, then pack the best ones until the
    /// char budget is spent. Cursor forgets between sessions; this remembers the
    /// *relevant* past fixes.
    pub fn load_relevant(&self, query: &str, char_cap: usize) -> Result<String, String> {
        if !self.base_path.exists() {
            return Ok(String::new());
        }
        let q = query.to_lowercase();
        let keywords: Vec<&str> = q.split_whitespace().filter(|w| w.len() > 2).collect();

        let now = std::time::SystemTime::now();
        const HALF_LIFE_SECS: f32 = 30.0 * 24.0 * 3600.0; // ~30 days

        // Score every brief but retain only (score, path) — never all contents
        // at once. On a large knowledge dir the old code held every file's full
        // text in `scored` simultaneously (peak RAM ∝ dir size); here peak stays
        // at one file, since each `content` is dropped at the end of its
        // iteration and the packing pass below re-reads only the winners.
        let mut scored: Vec<(f32, std::path::PathBuf)> = Vec::new();
        for entry in fs::read_dir(&self.base_path).map_err(|e| format!("Failed to read knowledge dir: {}", e))?.flatten() {
            if !entry.file_type().map(|t| t.is_file()).unwrap_or(false) {
                continue;
            }
            let content = match fs::read_to_string(entry.path()) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let content_lower = content.to_lowercase();
            let overlap = if keywords.is_empty() {
                1
            } else {
                keywords.iter().filter(|kw| content_lower.contains(**kw)).count()
            };

            let age_secs = entry
                .metadata()
                .and_then(|m| m.modified())
                .ok()
                .and_then(|t| now.duration_since(t).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0) as f32;
            let recency = 0.5 + 0.5 * (2.0f32).powf(-age_secs / HALF_LIFE_SECS);

            // Keyword match dominates; recency breaks ties and keeps fresh lessons
            // ahead of stale ones. An empty query falls back to pure recency.
            let score = (overlap as f32 + 0.1) * recency;
            scored.push((score, entry.path()));
        }

        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

        let mut out = String::from("\n=== RELEVANT PERSISTENT KNOWLEDGE (past sessions) ===\n");
        for (_, path) in scored {
            if out.len() >= char_cap {
                break;
            }
            let content = match fs::read_to_string(&path) {
                Ok(c) => c,
                Err(_) => continue,
            };
            if out.len() + content.len() + 8 > char_cap {
                let remaining = char_cap.saturating_sub(out.len());
                out.push_str(&content.chars().take(remaining).collect::<String>());
                break;
            }
            out.push_str(&content);
            out.push_str("\n\n---\n\n");
        }
        // Nothing but the header means no briefs — signal empty to the caller.
        if out.trim_end().ends_with("past sessions) ===") {
            return Ok(String::new());
        }
        Ok(out)
    }

    pub fn load_all_knowledge(&self) -> Result<String, String> {
        if !self.base_path.exists() {
            return Ok(String::new());
        }

        let mut combined_knowledge = String::from("\n=== PERSISTENT KNOWLEDGE (Session History) ===\n");
        let entries = fs::read_dir(&self.base_path)
            .map_err(|e| format!("Failed to read knowledge dir: {}", e))?;

        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        combined_knowledge.push_str(&content);
                        combined_knowledge.push_str("\n\n---\n\n");
                    }
                }
            }
        }

        Ok(combined_knowledge)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_root() -> PathBuf {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let dir = std::env::temp_dir().join(format!("kd_test_{}_{}", std::process::id(), nanos));
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    fn brief(title: &str, findings: &str) -> KnowledgeBrief {
        KnowledgeBrief {
            title: title.to_string(),
            date: "2026-07-09".to_string(),
            findings: findings.to_string(),
            affected_files: vec![],
        }
    }

    #[test]
    fn load_relevant_ranks_by_keyword_overlap() {
        let root = temp_root();
        let kd = KnowledgeDistiller::new(&root);
        kd.save_finding(brief("SQL Injection Fix", "fixed sql injection in the login query via parameterization")).unwrap();
        kd.save_finding(brief("Perf Tune", "optimized buffer allocation loop for throughput")).unwrap();

        let out = kd.load_relevant("sql injection login query", 10_000).unwrap();
        assert!(out.contains("parameterization"), "relevant brief must be included: {out}");
        // The matching brief should rank ahead of the unrelated one.
        let sql_pos = out.find("parameterization").unwrap();
        if let Some(perf_pos) = out.find("throughput") {
            assert!(sql_pos < perf_pos, "sql brief should rank before perf brief");
        }
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_relevant_empty_when_no_briefs() {
        let root = temp_root();
        let kd = KnowledgeDistiller::new(&root);
        let out = kd.load_relevant("anything", 5_000).unwrap();
        assert!(out.is_empty(), "no briefs should yield empty string, got: {out:?}");
        std::fs::remove_dir_all(&root).ok();
    }

    #[test]
    fn load_relevant_respects_char_cap() {
        let root = temp_root();
        let kd = KnowledgeDistiller::new(&root);
        for i in 0..5 {
            kd.save_finding(brief(&format!("Note {i}"), &"x".repeat(500))).unwrap();
        }
        let cap = 600;
        let out = kd.load_relevant("note", cap).unwrap();
        assert!(out.len() <= cap, "output {} exceeded cap {}", out.len(), cap);
        std::fs::remove_dir_all(&root).ok();
    }
}
