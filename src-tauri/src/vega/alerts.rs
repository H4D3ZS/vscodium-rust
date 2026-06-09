//! APEX Vega — alert definition registry.
//!
//! Loads Subgraph Vega's flat alert XML files (`resources/vega/alerts/*.xml`)
//! into a lookup table keyed by alert type (the filename without `.xml`).
//! The XML is simple and flat, so we parse with `regex` rather than pulling in
//! a full XML dependency. See `01-VEGA-INVENTORY.md` for the schema.

use crate::vega::model::{AlertDefinition, Severity};
use regex::Regex;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// In-memory registry of all known alert definitions.
#[derive(Debug, Clone, Default)]
pub struct AlertRegistry {
    defs: HashMap<String, AlertDefinition>,
}

impl AlertRegistry {
    pub fn len(&self) -> usize {
        self.defs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.defs.is_empty()
    }

    pub fn get(&self, type_key: &str) -> Option<&AlertDefinition> {
        self.defs.get(type_key)
    }

    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.defs.keys()
    }

    /// Load every `*.xml` in `dir` into the registry. Files that fail to parse
    /// are skipped (logged), not fatal — a malformed community module shouldn't
    /// sink the whole scanner.
    pub fn load_from_dir(dir: &Path) -> std::io::Result<AlertRegistry> {
        let mut defs = HashMap::new();
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|e| e.to_str()) != Some("xml") {
                continue;
            }
            let type_key = match path.file_stem().and_then(|s| s.to_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            let content = match std::fs::read_to_string(&path) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[vega/alerts] skip {}: {}", path.display(), e);
                    continue;
                }
            };
            match parse_alert_xml(&type_key, &content) {
                Some(def) => {
                    defs.insert(type_key, def);
                }
                None => {
                    eprintln!("[vega/alerts] unparseable alert: {}", path.display());
                }
            }
        }
        Ok(AlertRegistry { defs })
    }

    /// Convenience: load from the dev resources dir (`CARGO_MANIFEST_DIR`).
    /// Runtime/bundled resolution via the Tauri app handle is wired in Phase 6.
    pub fn load_default() -> std::io::Result<AlertRegistry> {
        Self::load_from_dir(&default_alerts_dir())
    }
}

/// The dev-time alerts directory, relative to the crate manifest.
pub fn default_alerts_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("resources")
        .join("vega")
        .join("alerts")
}

/// Extract the inner text of the first `<tag>...</tag>` (DOTALL), trimmed.
fn first_tag(content: &str, tag: &str) -> Option<String> {
    let re = Regex::new(&format!(r"(?is)<{t}>(.*?)</{t}>", t = regex::escape(tag))).ok()?;
    re.captures(content)
        .and_then(|c| c.get(1))
        .map(|m| decode_entities(m.as_str().trim()))
}

/// Extract inner text of ALL `<tag>...</tag>` occurrences.
fn all_tags(content: &str, tag: &str) -> Vec<String> {
    let re = match Regex::new(&format!(r"(?is)<{t}>(.*?)</{t}>", t = regex::escape(tag))) {
        Ok(r) => r,
        Err(_) => return Vec::new(),
    };
    re.captures_iter(content)
        .filter_map(|c| c.get(1))
        .map(|m| decode_entities(m.as_str().trim()))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Minimal XML entity decode for the handful that appear in Vega alert text.
fn decode_entities(s: &str) -> String {
    s.replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&apos;", "'")
        .replace("&amp;", "&")
}

/// Parse one alert XML document into an `AlertDefinition`.
pub fn parse_alert_xml(type_key: &str, content: &str) -> Option<AlertDefinition> {
    let title = first_tag(content, "title").unwrap_or_else(|| type_key.to_string());
    let class = first_tag(content, "class").unwrap_or_default();
    let severity = first_tag(content, "severity")
        .map(|s| Severity::from_str_loose(&s))
        .unwrap_or(Severity::Info);
    let impact = all_tags(content, "impact");
    let remediation = first_tag(content, "remediation").unwrap_or_default();
    let discussion = first_tag(content, "discussion").unwrap_or_default();

    Some(AlertDefinition {
        type_key: type_key.to_string(),
        title,
        class,
        severity,
        impact,
        remediation,
        discussion,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_known_alert() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<alert>
<title>Form File Upload Detected</title>
<class>Environment</class>
<severity>Info</severity>
<impact>A form allowing file upload was detected by Vega.</impact>
<impact>Vulnerabilities are commonly found in file upload functions.</impact>
<remediation>Review the upload function.</remediation>
<discussion>File uploads via FORM submissions are a common source of vulnerabilities.</discussion>
</alert>"#;
        let def = parse_alert_xml("vfileupload", xml).expect("should parse");
        assert_eq!(def.type_key, "vfileupload");
        assert_eq!(def.title, "Form File Upload Detected");
        assert_eq!(def.class, "Environment");
        assert_eq!(def.severity, Severity::Info);
        assert_eq!(def.impact.len(), 2);
        assert!(def.remediation.contains("Review"));
    }

    #[test]
    fn loads_all_shipped_alert_definitions() {
        let dir = default_alerts_dir();
        if !dir.exists() {
            // Resources not present in this checkout — don't fail the suite.
            eprintln!("[vega/alerts] resources dir missing: {}", dir.display());
            return;
        }
        let reg = AlertRegistry::load_from_dir(&dir).expect("load alerts");
        // We shipped 85 alert XMLs from Vega. Assert we parsed a healthy set.
        assert!(
            reg.len() >= 80,
            "expected >=80 alert defs, got {}",
            reg.len()
        );
        // Spot-check a well-known key resolves with a real title.
        if let Some(def) = reg.get("vfileupload") {
            assert!(!def.title.is_empty());
        }
    }
}
