use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::editor::engine::completion::{CompletionItem, CompletionKind};
use crate::theme::Theme;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContributedTheme {
    pub label: String,
    #[serde(rename = "uiTheme")]
    pub ui_theme: Option<String>,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContributedSnippet {
    pub language: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContributedLanguage {
    pub id: String,
    #[serde(default)]
    pub extensions: Vec<String>,
    #[serde(default)]
    pub aliases: Vec<String>,
    pub configuration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContributedCommand {
    pub command: String,
    pub title: String,
    pub category: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExtensionContributes {
    #[serde(default)]
    pub themes: Vec<ContributedTheme>,
    #[serde(default)]
    pub snippets: Vec<ContributedSnippet>,
    #[serde(default)]
    pub languages: Vec<ContributedLanguage>,
    #[serde(default)]
    pub commands: Vec<ContributedCommand>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    pub version: String,
    #[serde(default = "default_publisher")]
    pub publisher: String,
    pub description: Option<String>,
    pub main: Option<String>,
    pub icon: Option<String>,
    pub contributes: Option<ExtensionContributes>,
    #[serde(skip)]
    pub dir_path: Option<PathBuf>,
    #[serde(skip)]
    pub is_enabled: bool,
}

fn default_publisher() -> String {
    "unknown".to_string()
}

impl ExtensionManifest {
    pub fn id(&self) -> String {
        format!("{}.{}", self.publisher, self.name)
    }

    pub fn display_title(&self) -> &str {
        self.display_name.as_deref().unwrap_or(&self.name)
    }
}

/// Helper to parse hex colors into GPUI Rgba (#RGB, #RRGGBB, #RRGGBBAA)
pub fn parse_hex_color(hex: &str) -> Option<gpui_kit::gpui::Rgba> {
    let clean = hex.trim().trim_start_matches('#');
    if clean.len() == 6 {
        u32::from_str_radix(clean, 16).ok().map(gpui_kit::gpui::rgb)
    } else if clean.len() == 8 {
        u32::from_str_radix(clean, 16)
            .ok()
            .map(gpui_kit::gpui::rgba)
    } else if clean.len() == 3 {
        let mut expanded = String::with_capacity(6);
        for c in clean.chars() {
            expanded.push(c);
            expanded.push(c);
        }
        u32::from_str_radix(&expanded, 16)
            .ok()
            .map(gpui_kit::gpui::rgb)
    } else {
        None
    }
}

/// Load and parse extension manifest from a directory containing package.json
pub fn load_manifest_from_dir(dir: &Path) -> Result<ExtensionManifest> {
    let pkg_path = dir.join("package.json");
    let content = fs::read_to_string(&pkg_path)
        .with_context(|| format!("Failed to read package.json from {:?}", pkg_path))?;

    let mut manifest: ExtensionManifest = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON in {:?}", pkg_path))?;

    manifest.dir_path = Some(dir.to_path_buf());
    manifest.is_enabled = true;
    Ok(manifest)
}

/// Scan an extensions directory and return all discovered extension manifests
pub fn scan_installed_extensions(extensions_dir: &Path) -> Vec<ExtensionManifest> {
    let mut installed = Vec::new();
    if !extensions_dir.exists() {
        return installed;
    }

    if let Ok(entries) = fs::read_dir(extensions_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(manifest) = load_manifest_from_dir(&path) {
                    installed.push(manifest);
                }
            }
        }
    }

    installed
}

/// Parse a VS Code snippets JSON file into Monaco IntelliSense CompletionItems
pub fn load_snippets_from_file(file_path: &Path) -> Result<Vec<CompletionItem>> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read snippets file {:?}", file_path))?;

    let raw_snippets: HashMap<String, serde_json::Value> = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse snippets JSON {:?}", file_path))?;

    let mut completions = Vec::new();

    for (name, val) in raw_snippets {
        let desc = val
            .get("description")
            .and_then(|d| d.as_str())
            .unwrap_or(&name)
            .to_string();

        let body_str = match val.get("body") {
            Some(serde_json::Value::String(s)) => s.clone(),
            Some(serde_json::Value::Array(arr)) => {
                let lines: Vec<String> = arr
                    .iter()
                    .filter_map(|l| l.as_str().map(|s| s.to_string()))
                    .collect();
                lines.join("\n")
            }
            _ => continue,
        };

        // Prefix can be a string or array of strings
        let prefixes: Vec<String> = match val.get("prefix") {
            Some(serde_json::Value::String(p)) => vec![p.clone()],
            Some(serde_json::Value::Array(arr)) => arr
                .iter()
                .filter_map(|p| p.as_str().map(|s| s.to_string()))
                .collect(),
            _ => vec![name.clone()],
        };

        for prefix in prefixes {
            completions.push(CompletionItem {
                label: prefix,
                kind: CompletionKind::Snippet,
                detail: desc.clone(),
                insert_text: body_str.clone(),
            });
        }
    }

    Ok(completions)
}

/// Parse a VS Code Theme JSON file into color overrides and apply to Theme
pub fn load_theme_from_file(file_path: &Path, base_theme: &Theme) -> Result<Theme> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read theme file {:?}", file_path))?;

    let data: serde_json::Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse theme JSON {:?}", file_path))?;

    let mut theme = base_theme.clone();

    if let Some(colors) = data.get("colors").and_then(|c| c.as_object()) {
        if let Some(c) = colors
            .get("editor.background")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.bg_editor = c;
        }
        if let Some(c) = colors
            .get("editor.foreground")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.text_primary = c;
        }
        if let Some(c) = colors
            .get("activityBar.background")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.bg_activity_bar = c;
        }
        if let Some(c) = colors
            .get("sideBar.background")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.bg_sidebar = c;
        }
        if let Some(c) = colors
            .get("statusBar.background")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.bg_status_bar = c;
        }
        if let Some(c) = colors
            .get("titleBar.activeBackground")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.bg_titlebar = c;
        }
        if let Some(c) = colors
            .get("editorLineNumber.foreground")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.text_subtle = c;
        }
        if let Some(c) = colors
            .get("editorCursor.foreground")
            .and_then(|v| v.as_str())
            .and_then(parse_hex_color)
        {
            theme.accent = c;
        }
    }

    Ok(theme)
}
