/// Core plugin types — the loaded-plugin record and related definitions.
use crate::manifest::{PluginHooksConfig, PluginManifest};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

// ---------------------------------------------------------------------------
// Source
// ---------------------------------------------------------------------------

/// Where a plugin came from (mirrors the TS `source` field on `LoadedPlugin`).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PluginSource {
    /// `~/.claurst/plugins/`
    User,
    /// `<project>/.claurst/plugins/`
    Project,
    /// An extra path provided at runtime (e.g. `--plugin-dir` CLI flag).
    Extra(String),
    /// Provided programmatically at session start (inline / SDK).
    Inline,
    /// Built-in plugins bundled with the CLI.
    Builtin,
}

impl PluginSource {
    pub fn label(&self) -> &str {
        match self {
            PluginSource::User => "user",
            PluginSource::Project => "project",
            PluginSource::Extra(label) => label.as_str(),
            PluginSource::Inline => "inline",
            PluginSource::Builtin => "builtin",
        }
    }
}

// ---------------------------------------------------------------------------
// Loaded plugin
// ---------------------------------------------------------------------------

/// A fully loaded and validated plugin record.
///
/// This is what the `PluginRegistry` stores and what the rest of the crate
/// operates on.
#[derive(Debug, Clone)]
pub struct LoadedPlugin {
    /// The plugin's unique name (from `manifest.name`).
    pub name: String,
    /// Absolute path to the plugin root directory.
    pub path: PathBuf,
    /// Where this plugin was loaded from.
    pub source: PluginSource,
    /// Combined identifier: `"name@source"`.
    pub source_id: String,
    /// Parsed plugin.json / plugin.toml manifest.
    pub manifest: PluginManifest,
    /// Whether the plugin is currently enabled.
    pub enabled: bool,
    /// Path to the `commands/` subdirectory, if present.
    pub commands_path: Option<PathBuf>,
    /// Path to the `agents/` subdirectory, if present.
    pub agents_path: Option<PathBuf>,
    /// Path to the `skills/` subdirectory, if present.
    pub skills_path: Option<PathBuf>,
    /// Path to the `output-styles/` subdirectory, if present.
    pub output_styles_path: Option<PathBuf>,
    /// Parsed hooks configuration (from `hooks/hooks.json` or inline manifest field).
    pub hooks_config: Option<PluginHooksConfig>,
}

// ---------------------------------------------------------------------------
// Command definitions
// ---------------------------------------------------------------------------

/// How a plugin command produces its output.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandRunAction {
    /// Read a markdown file and use its content as the model prompt.
    MarkdownPrompt {
        file_path: String,
        plugin_root: String,
    },
    /// Run a shell command and return its stdout as the response.
    ShellCommand {
        command: String,
        plugin_root: String,
    },
    /// Return a static string response.
    StaticResponse(String),
}

impl CommandRunAction {
    /// Return the capability category string this action requires, or `None`
    /// if the action needs no special capability grant.
    ///
    /// | Variant              | Required capability |
    /// |----------------------|---------------------|
    /// | `StaticResponse`     | none                |
    /// | `MarkdownPrompt`     | `"read_files"`      |
    /// | `ShellCommand`       | `"shell"`           |
    pub fn required_capability(&self) -> Option<&'static str> {
        match self {
            CommandRunAction::StaticResponse(_) => None,
            CommandRunAction::MarkdownPrompt { .. } => Some("read_files"),
            CommandRunAction::ShellCommand { .. } => Some("shell"),
        }
    }
}

/// A plugin-defined slash command, ready for registration in the command system.
///
/// `cc-plugins` does NOT implement `claurst_commands::SlashCommand` directly (that
/// would create a circular dependency).  Instead `cc-commands` wraps
/// `PluginCommandDef` in a thin adapter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginCommandDef {
    /// Command name without leading `/` (e.g. `"myplugin:build"`).
    pub name: String,
    /// One-line description shown in `/help`.
    pub description: String,
    /// The plugin this command belongs to.
    pub plugin_name: String,
    /// Combined plugin source identifier.
    pub plugin_source_id: String,
    /// How to produce the command response.
    pub run_action: CommandRunAction,
    /// Capability grants declared in the plugin manifest.
    ///
    /// `None`  → manifest predates capability enforcement → all capabilities allowed
    ///           (backwards-compatible default).
    /// `Some(v)` → only the listed capability strings are permitted.
    pub plugin_capabilities: Option<Vec<String>>,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors that can occur while loading a plugin.
#[derive(Debug, Clone, thiserror::Error, Serialize, Deserialize)]
pub enum PluginError {
    #[error("IO error reading '{path}': {message}")]
    Io { path: String, message: String },

    #[error("Invalid manifest '{path}': {message}")]
    InvalidManifest { path: String, message: String },

    #[error("Duplicate plugin name '{name}': found in both '{first}' and '{second}'")]
    DuplicateName {
        name: String,
        first: String,
        second: String,
    },

    #[error("Plugin '{name}' validation error: {message}")]
    Validation { name: String, message: String },
}

impl PluginError {
    pub fn message(&self) -> String {
        self.to_string()
    }
}

// ---------------------------------------------------------------------------
// Reload diff
// ---------------------------------------------------------------------------

/// Summary of changes after a reload.
#[derive(Debug, Clone, Default)]
pub struct ReloadDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub updated: Vec<String>,
    pub error_count: usize,
}

impl ReloadDiff {
    pub fn is_empty(&self) -> bool {
        self.added.is_empty() && self.removed.is_empty() && self.updated.is_empty()
    }
}
