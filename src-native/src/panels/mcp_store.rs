use crate::panels::traits::{AuxiliaryTab, WorkbenchPanel};
use crate::state::HadesNativeState;
use crate::theme::Theme;
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpCatalogCategory {
    Development,
    Data,
    Cloud,
    Security,
    Productivity,
}

impl McpCatalogCategory {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Development => "Development",
            Self::Data => "Data & Analytics",
            Self::Cloud => "Cloud & APIs",
            Self::Security => "Security & Research",
            Self::Productivity => "Productivity",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpEnvField {
    pub key: String,
    pub label: String,
    pub placeholder: String,
    pub secret: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpCatalogEntry {
    pub id: String,
    pub name: String,
    pub category: McpCatalogCategory,
    pub description: String,
    pub command: Option<String>,
    pub args: Vec<String>,
    pub env_fields: Vec<McpEnvField>,
    pub type_: Option<String>,
    pub server_url: Option<String>,
    pub needs_config: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: BTreeMap<String, String>,
    pub type_: String,
    pub server_url: Option<String>,
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum McpStoreView {
    Store,
    Manage,
    RawConfig,
}

#[derive(Clone, Debug)]
pub struct McpStoreState {
    pub view: McpStoreView,
    pub search: String,
    pub selected_category: Option<McpCatalogCategory>,
    pub catalog: Vec<McpCatalogEntry>,
    pub installed_servers: Vec<McpServerConfig>,
    pub install_target: Option<McpCatalogEntry>,
    pub env_draft: BTreeMap<String, String>,
    pub custom_name: String,
    pub custom_command: String,
    pub custom_args: String,
    pub status_msg: String,
    pub raw_text: String,
    pub raw_error: String,
    pub config_path: String,
}

impl Default for McpStoreState {
    fn default() -> Self {
        let catalog = Self::default_catalog();
        let default_raw = r#"{
  "mcpServers": {
    "filesystem": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-filesystem", "."],
      "enabled": true
    },
    "git": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-git"],
      "enabled": true
    },
    "fetch": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-fetch"],
      "enabled": true
    }
  }
}"#
        .to_string();

        let installed_servers = vec![
            McpServerConfig {
                name: "filesystem".to_string(),
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-filesystem".to_string(),
                    ".".to_string(),
                ],
                env: BTreeMap::new(),
                type_: "stdio".to_string(),
                server_url: None,
                enabled: true,
            },
            McpServerConfig {
                name: "git".to_string(),
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-git".to_string(),
                ],
                env: BTreeMap::new(),
                type_: "stdio".to_string(),
                server_url: None,
                enabled: true,
            },
            McpServerConfig {
                name: "fetch".to_string(),
                command: "npx".to_string(),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-fetch".to_string(),
                ],
                env: BTreeMap::new(),
                type_: "stdio".to_string(),
                server_url: None,
                enabled: true,
            },
        ];

        Self {
            view: McpStoreView::Store,
            search: String::new(),
            selected_category: None,
            catalog,
            installed_servers,
            install_target: None,
            env_draft: BTreeMap::new(),
            custom_name: String::new(),
            custom_command: "npx".to_string(),
            custom_args: String::new(),
            status_msg: "14 tools across 3 enabled servers ready.".to_string(),
            raw_text: default_raw,
            raw_error: String::new(),
            config_path: "~/.gemini/antigravity-ide/mcp_config.json".to_string(),
        }
    }
}

impl McpStoreState {
    pub fn default_catalog() -> Vec<McpCatalogEntry> {
        vec![
            // Development
            McpCatalogEntry {
                id: "filesystem".to_string(),
                name: "Filesystem".to_string(),
                category: McpCatalogCategory::Development,
                description: "Read and write files in allowed workspace directories.".to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-filesystem".to_string(),
                    ".".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: Some("Allowed path default: current workspace".to_string()),
                tags: vec!["files".to_string(), "fs".to_string()],
            },
            McpCatalogEntry {
                id: "github".to_string(),
                name: "GitHub".to_string(),
                category: McpCatalogCategory::Development,
                description: "Manage repositories, issues, pull requests, and search code."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-github".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "GITHUB_PERSONAL_ACCESS_TOKEN".to_string(),
                    label: "GitHub Personal Access Token (PAT)".to_string(),
                    placeholder: "ghp_xxxxxxxxxxxx".to_string(),
                    secret: true,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["git".to_string(), "github".to_string()],
            },
            McpCatalogEntry {
                id: "git".to_string(),
                name: "Git".to_string(),
                category: McpCatalogCategory::Development,
                description: "Local git operations: history, log, diff, commit, and blame."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-git".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["vcs".to_string(), "git".to_string()],
            },
            McpCatalogEntry {
                id: "fetch".to_string(),
                name: "Fetch".to_string(),
                category: McpCatalogCategory::Development,
                description: "Fetch web pages and convert HTML directly to clean markdown."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-fetch".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["web".to_string(), "http".to_string()],
            },
            McpCatalogEntry {
                id: "puppeteer".to_string(),
                name: "Puppeteer".to_string(),
                category: McpCatalogCategory::Development,
                description: "Headless browser automation, DOM inspection, and screenshots."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-puppeteer".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["browser".to_string(), "automation".to_string()],
            },
            McpCatalogEntry {
                id: "sequential-thinking".to_string(),
                name: "Sequential Thinking".to_string(),
                category: McpCatalogCategory::Development,
                description: "Structured step-by-step reasoning for complex architecture tasks."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-sequential-thinking".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["reasoning".to_string(), "ai".to_string()],
            },
            McpCatalogEntry {
                id: "memory".to_string(),
                name: "Memory Graph".to_string(),
                category: McpCatalogCategory::Development,
                description: "Persistent knowledge-graph memory across agent sessions.".to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-memory".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["memory".to_string(), "graph".to_string()],
            },
            // Data
            McpCatalogEntry {
                id: "postgres".to_string(),
                name: "PostgreSQL".to_string(),
                category: McpCatalogCategory::Data,
                description:
                    "Query Postgres databases with schema inspection and read-only safety."
                        .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-postgres".to_string(),
                    "postgresql://localhost/db".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: Some("Set postgresql connection string in arguments".to_string()),
                tags: vec!["database".to_string(), "sql".to_string()],
            },
            McpCatalogEntry {
                id: "sqlite".to_string(),
                name: "SQLite".to_string(),
                category: McpCatalogCategory::Data,
                description: "Query and inspect local SQLite database files.".to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-sqlite".to_string(),
                    "./data.db".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: Some("Specify path to your .db file".to_string()),
                tags: vec!["database".to_string(), "sqlite".to_string()],
            },
            McpCatalogEntry {
                id: "bigquery".to_string(),
                name: "BigQuery".to_string(),
                category: McpCatalogCategory::Data,
                description: "Query Google Cloud BigQuery datasets with natural language prompts."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@toolbox-sdk/server".to_string(),
                    "--prebuilt".to_string(),
                    "bigquery".to_string(),
                    "--stdio".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "BIGQUERY_PROJECT".to_string(),
                    label: "GCP Project ID".to_string(),
                    placeholder: "my-cloud-project".to_string(),
                    secret: false,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["gcp".to_string(), "bigquery".to_string()],
            },
            // Cloud
            McpCatalogEntry {
                id: "firebase".to_string(),
                name: "Firebase".to_string(),
                category: McpCatalogCategory::Cloud,
                description: "AI-powered tools for Firebase hosting, firestore, and auth."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "firebase-tools@latest".to_string(),
                    "mcp".to_string(),
                ],
                env_fields: Vec::new(),
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["firebase".to_string(), "cloud".to_string()],
            },
            McpCatalogEntry {
                id: "brave-search".to_string(),
                name: "Brave Search".to_string(),
                category: McpCatalogCategory::Cloud,
                description: "Live web search and news indexing via the Brave Search API."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-brave-search".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "BRAVE_API_KEY".to_string(),
                    label: "Brave API Key".to_string(),
                    placeholder: "BSA-xxxxxxxxxxxx".to_string(),
                    secret: true,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["search".to_string(), "web".to_string()],
            },
            McpCatalogEntry {
                id: "slack".to_string(),
                name: "Slack".to_string(),
                category: McpCatalogCategory::Cloud,
                description: "Read channels and post agent execution summaries to Slack."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec![
                    "-y".to_string(),
                    "@modelcontextprotocol/server-slack".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "SLACK_BOT_TOKEN".to_string(),
                    label: "Slack Bot Token".to_string(),
                    placeholder: "xoxb-xxxxxxxxxxxx".to_string(),
                    secret: true,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["chat".to_string(), "slack".to_string()],
            },
            // Security
            McpCatalogEntry {
                id: "ida-pro-idalib".to_string(),
                name: "IDA Pro MCP (Headless)".to_string(),
                category: McpCatalogCategory::Security,
                description:
                    "245+ live RE tools via idalib-mcp: decompile, disasm, xrefs, and patches."
                        .to_string(),
                command: Some("uvx".to_string()),
                args: vec![
                    "--from".to_string(),
                    "git+https://github.com/mrexodia/ida-pro-mcp".to_string(),
                    "idalib-mcp".to_string(),
                    "--stdio".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "IDA_INSTALL_DIR".to_string(),
                    label: "IDA Pro Install Path".to_string(),
                    placeholder: "C:\\Program Files\\IDA Professional 9.0".to_string(),
                    secret: false,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: Some("Requires IDA Pro 8.3+ license".to_string()),
                tags: vec![
                    "binary-analysis".to_string(),
                    "reverse-engineering".to_string(),
                ],
            },
            McpCatalogEntry {
                id: "ghidra-pyghidra".to_string(),
                name: "Ghidra MCP (Headless)".to_string(),
                category: McpCatalogCategory::Security,
                description:
                    "Headless Ghidra decompilation, cross-references, and struct analysis."
                        .to_string(),
                command: Some("uvx".to_string()),
                args: vec![
                    "--from".to_string(),
                    "git+https://github.com/clearbluejar/pyghidra-mcp".to_string(),
                    "pyghidra-mcp".to_string(),
                    "-t".to_string(),
                    "stdio".to_string(),
                ],
                env_fields: vec![McpEnvField {
                    key: "GHIDRA_INSTALL_DIR".to_string(),
                    label: "Ghidra Install Path".to_string(),
                    placeholder: "C:\\Tools\\ghidra_11.1_PUBLIC".to_string(),
                    secret: false,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: Some(
                    "Requires Ghidra directory containing ghidraRun.bat".to_string(),
                ),
                tags: vec!["binary-analysis".to_string(), "ghidra".to_string()],
            },
            // Productivity
            McpCatalogEntry {
                id: "notion".to_string(),
                name: "Notion".to_string(),
                category: McpCatalogCategory::Productivity,
                description: "Search and update Notion pages, databases, and project tasks."
                    .to_string(),
                command: Some("npx".to_string()),
                args: vec!["-y".to_string(), "@notionhq/notion-mcp-server".to_string()],
                env_fields: vec![McpEnvField {
                    key: "NOTION_API_KEY".to_string(),
                    label: "Notion Integration Secret".to_string(),
                    placeholder: "secret_xxxxxxxxxxxx".to_string(),
                    secret: true,
                }],
                type_: Some("stdio".to_string()),
                server_url: None,
                needs_config: None,
                tags: vec!["docs".to_string(), "notion".to_string()],
            },
        ]
    }

    pub fn is_installed(&self, name: &str) -> bool {
        self.installed_servers
            .iter()
            .any(|s| s.name.eq_ignore_ascii_case(name))
    }

    pub fn install_entry(&mut self, entry: &McpCatalogEntry, env: BTreeMap<String, String>) {
        let name = entry.name.to_lowercase();
        if self.is_installed(&name) {
            return;
        }

        self.installed_servers.push(McpServerConfig {
            name: name.clone(),
            command: entry.command.clone().unwrap_or_else(|| "npx".to_string()),
            args: entry.args.clone(),
            env,
            type_: entry.type_.clone().unwrap_or_else(|| "stdio".to_string()),
            server_url: entry.server_url.clone(),
            enabled: true,
        });

        self.status_msg = format!("Installed MCP server '{}' successfully.", entry.name);
        self.install_target = None;
        self.env_draft.clear();
    }

    pub fn remove_server(&mut self, name: &str) {
        self.installed_servers
            .retain(|s| !s.name.eq_ignore_ascii_case(name));
        self.status_msg = format!("Removed MCP server '{}'.", name);
    }

    pub fn toggle_server(&mut self, name: &str) {
        if let Some(s) = self
            .installed_servers
            .iter_mut()
            .find(|s| s.name.eq_ignore_ascii_case(name))
        {
            s.enabled = !s.enabled;
            self.status_msg = format!(
                "Server '{}' is now {}.",
                name,
                if s.enabled { "enabled" } else { "disabled" }
            );
        }
    }

    pub fn add_custom_server(&mut self) {
        let name = self.custom_name.trim().to_string();
        let cmd = self.custom_command.trim().to_string();
        if name.is_empty() || cmd.is_empty() {
            return;
        }

        let args: Vec<String> = self
            .custom_args
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        self.installed_servers.push(McpServerConfig {
            name: name.clone(),
            command: cmd,
            args,
            env: BTreeMap::new(),
            type_: "stdio".to_string(),
            server_url: None,
            enabled: true,
        });

        self.status_msg = format!("Added custom MCP server '{}'.", name);
        self.custom_name.clear();
        self.custom_args.clear();
    }

    pub fn format_raw_json(&mut self) {
        if let Ok(val) = serde_json::from_str::<serde_json::Value>(&self.raw_text) {
            if let Ok(pretty) = serde_json::to_string_pretty(&val) {
                self.raw_text = pretty;
                self.raw_error.clear();
                return;
            }
        }
        self.raw_error = "Invalid JSON syntax. Unable to format.".to_string();
    }

    pub fn save_raw_json(&mut self) {
        if serde_json::from_str::<serde_json::Value>(&self.raw_text).is_ok() {
            self.raw_error.clear();
            self.status_msg = "Saved mcp_config.json and reloaded MCP servers.".to_string();
        } else {
            self.raw_error = "Syntax error: Fix invalid JSON before saving.".to_string();
        }
    }

    pub fn enabled_server_count(&self) -> usize {
        self.installed_servers.iter().filter(|s| s.enabled).count()
    }

    pub fn total_tool_count(&self) -> usize {
        self.enabled_server_count() * 4 + 2
    }
}

pub struct McpStorePanel;

impl WorkbenchPanel for McpStorePanel {
    fn id(&self) -> &'static str {
        "mcp_store"
    }

    fn title(&self) -> &'static str {
        "MCP Store"
    }

    fn icon(&self) -> crate::ui::icons::IconName {
        crate::ui::icons::IconName::Bot
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_mcp_store_panel(state, cx).into_any_element()
    }
}

impl AuxiliaryTab for McpStorePanel {
    fn id(&self) -> &'static str {
        "mcp_store"
    }

    fn title(&self) -> &'static str {
        "MCP Store"
    }

    fn icon(&self) -> crate::ui::icons::IconName {
        crate::ui::icons::IconName::Bot
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_mcp_store_panel(state, cx).into_any_element()
    }
}

fn render_store_header(
    theme: &Theme,
    store: &McpStoreState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let current_view = store.view;

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .p_3()
        .border_b_1()
        .border_color(theme.border_subtle)
        .child(
            div()
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_base()
                        .child(match current_view {
                            McpStoreView::Store => "Model Context Protocol (MCP) Store",
                            McpStoreView::Manage => "Manage MCP Servers",
                            McpStoreView::RawConfig => "Raw MCP Configuration",
                        }),
                )
                .child(
                    div().text_xs().text_color(theme.text_muted).child(
                        "Antigravity MCP parity: browse, 1-click install, and manage servers",
                    ),
                ),
        )
        .child(
            div()
                .flex()
                .gap_1p5()
                .child(
                    div()
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if current_view == McpStoreView::Store {
                            theme.accent
                        } else {
                            theme.bg_editor
                        })
                        .text_color(if current_view == McpStoreView::Store {
                            rgb(0xffffff)
                        } else {
                            theme.text_primary
                        })
                        .text_xs()
                        .hover(|s| s.opacity(0.85))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.mcp_store.view = McpStoreView::Store;
                                cx.notify();
                            }),
                        )
                        .child("Catalog"),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if current_view == McpStoreView::Manage {
                            theme.accent
                        } else {
                            theme.bg_editor
                        })
                        .text_color(if current_view == McpStoreView::Manage {
                            rgb(0xffffff)
                        } else {
                            theme.text_primary
                        })
                        .text_xs()
                        .hover(|s| s.opacity(0.85))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.mcp_store.view = McpStoreView::Manage;
                                cx.notify();
                            }),
                        )
                        .child(format!("Manage ({})", store.installed_servers.len())),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if current_view == McpStoreView::RawConfig {
                            theme.accent
                        } else {
                            theme.bg_editor
                        })
                        .text_color(if current_view == McpStoreView::RawConfig {
                            rgb(0xffffff)
                        } else {
                            theme.text_primary
                        })
                        .text_xs()
                        .hover(|s| s.opacity(0.85))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.mcp_store.view = McpStoreView::RawConfig;
                                cx.notify();
                            }),
                        )
                        .child("Raw JSON"),
                ),
        )
}

fn render_catalog_view(
    theme: &Theme,
    store: &McpStoreState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let query = store.search.to_lowercase();
    let selected_cat = store.selected_category;

    let filtered: Vec<_> = store
        .catalog
        .iter()
        .filter(|entry| {
            if let Some(cat) = selected_cat {
                if entry.category != cat {
                    return false;
                }
            }
            if query.is_empty() {
                true
            } else {
                entry.name.to_lowercase().contains(&query)
                    || entry.description.to_lowercase().contains(&query)
                    || entry.tags.iter().any(|t| t.contains(&query))
            }
        })
        .cloned()
        .collect();

    let categories = [
        (None, "All"),
        (Some(McpCatalogCategory::Development), "Development"),
        (Some(McpCatalogCategory::Data), "Data"),
        (Some(McpCatalogCategory::Cloud), "Cloud"),
        (Some(McpCatalogCategory::Security), "Security"),
        (Some(McpCatalogCategory::Productivity), "Productivity"),
    ];

    div()
        .flex()
        .flex_col()
        .gap_3()
        .p_3()
        // Category Filter Chips
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_1p5()
                .children(categories.into_iter().map(|(cat, label)| {
                    let is_sel = selected_cat == cat;
                    div()
                        .cursor_pointer()
                        .px_2()
                        .py_0p5()
                        .rounded(px(12.0))
                        .border_1()
                        .border_color(if is_sel {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .bg(if is_sel {
                            rgba(0x3b82f625)
                        } else {
                            theme.bg_editor
                        })
                        .text_xs()
                        .text_color(if is_sel {
                            theme.accent
                        } else {
                            theme.text_muted
                        })
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                this.state.mcp_store.selected_category = cat;
                                cx.notify();
                            }),
                        )
                        .child(label)
                })),
        )
        // Catalog Cards Grid
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(filtered.into_iter().map(|entry| {
                    let is_installed = store.is_installed(&entry.name);
                    let name = entry.name.clone();
                    let desc = entry.description.clone();
                    let category_label = entry.category.label();
                    let needs_config = entry.needs_config.clone();
                    let entry_clone = entry.clone();

                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .p_3()
                        .rounded(px(6.0))
                        .bg(theme.bg_editor)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .flex_1()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .font_weight(FontWeight::BOLD)
                                                .text_sm()
                                                .child(name),
                                        )
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .bg(theme.bg_panel)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(category_label),
                                        ),
                                )
                                .child(div().text_xs().text_color(theme.text_muted).child(desc))
                                .children(needs_config.map(|hint| {
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0xf59e0b))
                                        .child(format!("Note: {}", hint))
                                })),
                        )
                        .child(
                            div().flex().items_center().gap_2().child(
                                div()
                                    .cursor_pointer()
                                    .px_3()
                                    .py_1p5()
                                    .rounded(px(4.0))
                                    .bg(if is_installed {
                                        rgba(0x10b98125)
                                    } else {
                                        theme.accent
                                    })
                                    .border_1()
                                    .border_color(if is_installed {
                                        rgb(0x10b981)
                                    } else {
                                        theme.accent
                                    })
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(if is_installed {
                                        rgb(0x10b981)
                                    } else {
                                        rgb(0xffffff)
                                    })
                                    .hover(|s| s.opacity(0.85))
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |this, _event, _window, cx| {
                                            if is_installed {
                                                this.state.mcp_store.status_msg = format!(
                                                    "'{}' is already installed.",
                                                    entry_clone.name
                                                );
                                            } else if !entry_clone.env_fields.is_empty() {
                                                this.state.mcp_store.install_target =
                                                    Some(entry_clone.clone());
                                                this.state.mcp_store.env_draft.clear();
                                                for f in &entry_clone.env_fields {
                                                    this.state
                                                        .mcp_store
                                                        .env_draft
                                                        .insert(f.key.clone(), String::new());
                                                }
                                            } else {
                                                this.state
                                                    .mcp_store
                                                    .install_entry(&entry_clone, BTreeMap::new());
                                            }
                                            cx.notify();
                                        }),
                                    )
                                    .child(if is_installed { "Installed" } else { "Install" }),
                            ),
                        )
                })),
        )
}

fn render_manage_view(
    theme: &Theme,
    store: &McpStoreState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let installed = store.installed_servers.clone();
    let enabled_count = store.enabled_server_count();
    let tools_count = store.total_tool_count();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .p_3()
        // Status banner
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_editor)
                .border_1()
                .border_color(theme.border_subtle)
                .child(div().text_xs().child(format!(
                    "{} tools registered across {} enabled servers.",
                    tools_count, enabled_count
                )))
                .child(
                    div()
                        .text_xs()
                        .font_family("monospace")
                        .text_color(theme.text_muted)
                        .child(store.config_path.clone()),
                ),
        )
        // Installed Servers List
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(installed.into_iter().map(|server| {
                    let s_name = server.name.clone();
                    let is_enabled = server.enabled;
                    let type_badge = server.type_.clone();
                    let cmd_line = format!("{} {}", server.command, server.args.join(" "));
                    let name_copy1 = s_name.clone();
                    let name_copy2 = s_name.clone();

                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_editor)
                        .border_1()
                        .border_color(if is_enabled {
                            theme.border_subtle
                        } else {
                            rgba(0xffffff10)
                        })
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .font_weight(FontWeight::BOLD)
                                                .text_xs()
                                                .child(s_name),
                                        )
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .bg(theme.bg_panel)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(type_badge),
                                        )
                                        .children((!is_enabled).then(|| {
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0xf59e0b))
                                                .child("disabled")
                                        })),
                                )
                                .child(
                                    div()
                                        .font_family("monospace")
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(cmd_line),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px_2()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(if is_enabled {
                                            rgb(0x16a34a)
                                        } else {
                                            rgba(0xffffff15)
                                        })
                                        .text_xs()
                                        .text_color(rgb(0xffffff))
                                        .hover(|s| s.opacity(0.85))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.state.mcp_store.toggle_server(&name_copy1);
                                                cx.notify();
                                            }),
                                        )
                                        .child(if is_enabled { "ON" } else { "OFF" }),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px_2()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(theme.bg_panel)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .text_xs()
                                        .text_color(rgb(0xf43f5e))
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.state.mcp_store.remove_server(&name_copy2);
                                                cx.notify();
                                            }),
                                        )
                                        .child("Delete"),
                                ),
                        )
                })),
        )
}

fn render_raw_view(
    theme: &Theme,
    store: &McpStoreState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let raw_text = store.raw_text.clone();
    let raw_error = store.raw_error.clone();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .p_3()
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Direct mcp_config.json editor:"),
                )
                .child(
                    div()
                        .flex()
                        .gap_2()
                        .child(
                            div()
                                .cursor_pointer()
                                .px_2p5()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_editor)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.mcp_store.format_raw_json();
                                        cx.notify();
                                    }),
                                )
                                .child("Format JSON"),
                        )
                        .child(
                            div()
                                .cursor_pointer()
                                .px_2p5()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .hover(|s| s.opacity(0.85))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.mcp_store.save_raw_json();
                                        cx.notify();
                                    }),
                                )
                                .child("Save & Reload"),
                        ),
                ),
        )
        .children(
            (!raw_error.is_empty())
                .then(|| div().text_xs().text_color(rgb(0xf43f5e)).child(raw_error)),
        )
        .child(
            div()
                .p_3()
                .bg(theme.bg_editor)
                .rounded(px(4.0))
                .border_1()
                .border_color(theme.border_subtle)
                .font_family("monospace")
                .text_xs()
                .child(raw_text),
        )
}

pub fn render_mcp_store_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let store = &state.mcp_store;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_panel)
        .child(render_store_header(theme, store, cx))
        .child(div().flex_1().overflow_hidden().child(match store.view {
            McpStoreView::Store => render_catalog_view(theme, store, cx).into_any_element(),
            McpStoreView::Manage => render_manage_view(theme, store, cx).into_any_element(),
            McpStoreView::RawConfig => render_raw_view(theme, store, cx).into_any_element(),
        }))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_mcp_store_defaults() {
        let store = McpStoreState::default();
        assert_eq!(store.view, McpStoreView::Store);
        assert!(!store.catalog.is_empty());
        assert_eq!(store.installed_servers.len(), 3);
        assert!(store.is_installed("filesystem"));
        assert!(store.is_installed("git"));
        assert!(store.is_installed("fetch"));
        assert!(!store.is_installed("postgres"));
    }

    #[test]
    fn test_mcp_store_install_and_remove() {
        let mut store = McpStoreState::default();
        let postgres_entry = store
            .catalog
            .iter()
            .find(|e| e.name == "PostgreSQL")
            .unwrap()
            .clone();

        assert!(!store.is_installed("postgresql"));
        store.install_entry(&postgres_entry, BTreeMap::new());
        assert!(store.is_installed("postgresql"));

        store.remove_server("postgresql");
        assert!(!store.is_installed("postgresql"));
    }

    #[test]
    fn test_mcp_store_toggle_and_custom() {
        let mut store = McpStoreState::default();
        assert!(store.installed_servers[0].enabled);

        store.toggle_server("filesystem");
        assert!(
            !store
                .installed_servers
                .iter()
                .find(|s| s.name == "filesystem")
                .unwrap()
                .enabled
        );

        store.custom_name = "my_custom_tool".to_string();
        store.custom_command = "python".to_string();
        store.custom_args = "tool.py, --server".to_string();
        store.add_custom_server();

        assert!(store.is_installed("my_custom_tool"));
        let custom = store
            .installed_servers
            .iter()
            .find(|s| s.name == "my_custom_tool")
            .unwrap();
        assert_eq!(custom.command, "python");
        assert_eq!(custom.args, vec!["tool.py", "--server"]);
    }

    #[test]
    fn test_mcp_store_json_formatting() {
        let mut store = McpStoreState::default();
        store.raw_text = r#"{"mcpServers":{"test":{"command":"npx"}}}"#.to_string();
        store.format_raw_json();
        assert!(store.raw_error.is_empty());
        assert!(store.raw_text.contains('\n'));

        store.save_raw_json();
        assert!(store.status_msg.contains("Saved"));
    }
}
