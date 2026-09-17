pub mod agent;
pub mod editor;
pub mod file_tree;
pub mod hooks;
pub mod specs;

pub use agent::*;
pub use editor::*;
pub use file_tree::*;
pub use hooks::*;
pub use specs::*;

use crate::theme::Theme;
use crate::ui::context_menu::ContextMenuState;
use crate::ui::inline_edit::InlineEditState;
use crate::ui::toast::ToastManager;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use vscode_rust_app::services::AppServices;

pub use crate::domain::canvas::*;
pub use crate::domain::layout::{
    ActivityTab, BottomPanelTab, EmulatorSubTab, FocusedPanel, RightSidebarTab,
};

#[derive(Clone, Debug)]
pub struct ToolPermissionRequest {
    pub id: String,
    pub tool: String,
    pub args: String,
    pub level: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct EditorPreferences {
    pub tab_size: usize,
    pub word_wrap: bool,
    pub minimap: bool,
    pub bracket_pairs: bool,
    pub indent_guides: bool,
    pub sticky_scroll: bool,
    pub format_on_save: bool,
    pub inlay_hints: bool,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_font_size")]
    pub font_size: usize,
    #[serde(default = "default_line_numbers")]
    pub line_numbers: bool,
    #[serde(default = "default_telemetry")]
    pub telemetry: bool,
    #[serde(default = "default_shell")]
    pub default_shell: String,
    #[serde(default = "default_auto_approve_terminal")]
    pub auto_approve_terminal: bool,
    #[serde(default = "default_auto_approve_file_write")]
    pub auto_approve_file_write: bool,
    #[serde(default = "default_reasoning_effort")]
    pub reasoning_effort: String,
}

fn default_theme() -> String {
    "Cursor Dark".to_string()
}
fn default_font_size() -> usize {
    14
}
fn default_line_numbers() -> bool {
    true
}
fn default_telemetry() -> bool {
    false
}
fn default_shell() -> String {
    "PowerShell".to_string()
}
fn default_auto_approve_terminal() -> bool {
    false
}
fn default_auto_approve_file_write() -> bool {
    true
}
fn default_reasoning_effort() -> String {
    "High".to_string()
}

impl Default for EditorPreferences {
    fn default() -> Self {
        Self {
            tab_size: 4,
            word_wrap: true,
            minimap: true,
            bracket_pairs: true,
            indent_guides: true,
            sticky_scroll: true,
            format_on_save: true,
            inlay_hints: true,
            theme: default_theme(),
            font_size: default_font_size(),
            line_numbers: default_line_numbers(),
            telemetry: default_telemetry(),
            default_shell: default_shell(),
            auto_approve_terminal: default_auto_approve_terminal(),
            auto_approve_file_write: default_auto_approve_file_write(),
            reasoning_effort: default_reasoning_effort(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenTabSession {
    pub path: String,
    #[serde(default)]
    pub cursor_row: usize,
    #[serde(default)]
    pub cursor_col: usize,
    #[serde(default)]
    pub scroll_row: usize,
    #[serde(default)]
    pub is_pinned: bool,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct WorkspaceSession {
    pub open_tabs: Vec<OpenTabSession>,
    pub active_tab_idx: Option<usize>,
    #[serde(default)]
    pub workspace_root: Option<PathBuf>,
    #[serde(default)]
    pub secondary_open_tabs: Vec<OpenTabSession>,
    #[serde(default)]
    pub secondary_active_tab_idx: Option<usize>,
    #[serde(default)]
    pub is_split_editor: bool,
    #[serde(default)]
    pub split_horizontal: bool,
    #[serde(default)]
    pub left_sidebar_open: bool,
    #[serde(default)]
    pub right_sidebar_open: bool,
    #[serde(default)]
    pub iphone_preview_open: bool,
    #[serde(default)]
    pub bottom_panel_open: bool,
    #[serde(default = "default_sidebar_width")]
    pub sidebar_width: f32,
    #[serde(default = "default_right_sidebar_width")]
    pub right_sidebar_width: f32,
    #[serde(default = "default_iphone_preview_width")]
    pub iphone_preview_width: f32,
    #[serde(default = "default_bottom_panel_height")]
    pub bottom_panel_height: f32,
    #[serde(default = "default_activity_tab")]
    pub active_activity: ActivityTab,
    #[serde(default = "default_right_tab")]
    pub active_right_tab: RightSidebarTab,
    #[serde(default = "default_bottom_tab")]
    pub active_bottom_tab: BottomPanelTab,
}

fn default_sidebar_width() -> f32 {
    260.0
}
fn default_right_sidebar_width() -> f32 {
    460.0
}
fn default_iphone_preview_width() -> f32 {
    390.0
}
fn default_bottom_panel_height() -> f32 {
    240.0
}
fn default_activity_tab() -> ActivityTab {
    ActivityTab::Explorer
}
fn default_right_tab() -> RightSidebarTab {
    RightSidebarTab::AgentChat
}
fn default_bottom_tab() -> BottomPanelTab {
    BottomPanelTab::Terminal
}

#[derive(Clone)]
pub struct HadesNativeState {
    pub services: Option<AppServices>,
    pub theme: Theme,

    // Layout visibility toggles
    pub left_sidebar_open: bool,
    pub right_sidebar_open: bool,
    pub iphone_preview_open: bool,
    pub bottom_panel_open: bool,

    // Active tab selectors
    pub active_activity: ActivityTab,
    pub active_right_tab: RightSidebarTab,
    pub active_bottom_tab: BottomPanelTab,
    pub active_emulator_subtab: EmulatorSubTab,
    pub focused_panel: FocusedPanel,

    // Editor tabs & buffer state
    pub tabs: Vec<OpenTab>,
    pub active_tab_idx: Option<usize>,
    pub is_split_editor: bool,
    pub active_pane: usize,
    pub secondary_tabs: Vec<OpenTab>,
    pub active_secondary_tab_idx: Option<usize>,
    pub split_ratio: f32,
    pub split_horizontal: bool,

    // Editor Find Bar
    pub find_bar_open: bool,
    pub find_query: String,
    pub find_match_count: usize,
    pub find_case_sensitive: bool,
    pub find_regex: bool,
    pub find_whole_word: bool,
    pub replace_query: String,
    pub find_replace_mode: bool,

    // Steering Rules & Hooks
    pub global_rules: String,
    pub agent_hooks: Vec<AgentHook>,

    // Specs & Spec-to-Code
    pub specs_prompt: String,
    pub specs_projects: Vec<SpecsProject>,
    pub active_specs_project_idx: Option<usize>,

    // Kortex Services
    pub kortex_chunks_indexed: usize,
    pub kortex_indexing_status: String,
    pub kortex_kv_running: bool,

    // AI Agent (Cursor-like) state
    pub agent_mode: String,
    pub agent_model: String,
    pub available_ai_models: Vec<String>,
    pub anthropic_api_key: String,
    pub openai_api_key: String,
    pub gemini_api_key: String,
    pub deepseek_api_key: String,
    pub modelscope_api_key: String,
    pub reasoning_active: bool,
    pub composer_input: String,
    pub chat_messages: Vec<ChatMessage>,
    pub is_agent_thinking: bool,
    pub ai_turn: Arc<Mutex<AiTurnState>>,
    pub agent_threads: Vec<AgentThread>,
    pub active_thread_idx: usize,
    pub is_yolo_mode: bool,
    pub is_continuous_mode: bool,
    pub model_picker_open: bool,
    pub mode_picker_open: bool,
    pub expanded_thoughts: std::collections::HashSet<usize>,
    pub live_thought_expanded: bool,
    pub copied_msg_idx: Option<usize>,
    pub editing_msg_idx: Option<usize>,
    pub edit_msg_input: String,
    pub thinking_start_instant: Option<std::time::Instant>,

    // File tree & Workspace
    pub has_workspace: bool,
    pub workspace_root: PathBuf,
    pub recent_workspaces: Vec<PathBuf>,
    pub file_tree: Vec<FileNode>,
    pub explorer_create: Option<ExplorerCreateState>,

    // SCM and Search
    pub search: crate::panels::SearchState,
    pub scm: crate::panels::ScmState,

    // Problems, Output, and Debug Subsystems
    pub problems: crate::panels::ProblemsState,
    pub output: crate::panels::OutputState,
    pub debug: crate::panels::DebugState,
    pub test_explorer: crate::panels::TestExplorerState,
    pub markdown_preview: crate::panels::MarkdownPreviewState,
    pub run_configs: crate::panels::RunConfigsState,
    pub ports: crate::panels::PortsState,
    pub jobs: crate::panels::JobsState,
    pub debug_console: crate::panels::DebugConsoleState,
    pub logcat: crate::panels::LogcatState,
    pub browser: crate::panels::BrowserPanelState,
    pub vector_search: crate::panels::VectorSearchState,
    pub mcp_store: crate::panels::McpStoreState,
    pub outline: crate::panels::OutlineState,
    pub keybindings: crate::panels::KeybindingsState,
    pub canvases: Vec<CanvasSpec>,

    // Security & Trust
    pub workspace_trusted: bool,
    pub workspace_trust_dismissed: bool,
    pub pending_tool_permission: Option<ToolPermissionRequest>,

    // Terminal
    pub terminal_lines: Vec<String>,
    pub terminal_input: String,
    pub terminal_active_prompt: String,
    pub pty: Option<Arc<crate::terminal_pty::NativePtySession>>,
    pub terminal_sessions: Vec<Arc<crate::terminal_pty::NativePtySession>>,
    pub active_terminal_idx: usize,
    pub status_message: String,

    // Context Menu, Toast Notifications & Inline Edit
    pub context_menu: ContextMenuState,
    pub toast_manager: ToastManager,
    pub inline_edit: InlineEditState,
    pub current_thought: Option<crate::ui::AiThought>,
    pub inference_health: crate::ui::InferenceHealthState,
    pub settings_category: usize,
    pub settings_item: usize,
    pub settings_search: String,
    pub default_tab_size: usize,
    pub format_on_save: bool,
    pub inlay_hints_enabled: bool,
    pub active_theme_name: String,
    pub editor_font_size: usize,
    pub line_numbers_enabled: bool,
    pub telemetry_enabled: bool,
    pub default_shell: String,
    pub agent_auto_approve_terminal: bool,
    pub agent_auto_approve_file_write: bool,
    pub agent_reasoning_effort: String,

    // Panel Dimensions & Splitter Resizing
    pub sidebar_width: f32,
    pub right_sidebar_width: f32,
    pub iphone_preview_width: f32,
    pub bottom_panel_height: f32,
    pub is_resizing_sidebar: bool,
    pub is_resizing_right_sidebar: bool,
    pub is_resizing_iphone_preview: bool,
    pub is_resizing_bottom_panel: bool,
    pub minimap_hover: Option<(usize, f32)>,

    // Hover Tooltip State
    pub hover_info: Option<crate::editor::engine::vsx::languages::HoverInfo>,
    pub hover_row: usize,

    // Open VSX & Extensions Subsystem
    pub vsx: crate::editor::engine::vsx::manager::VsxManager,
    pub extensions_category: usize,

    // Standalone Pure Rust LSP Engine
    pub lsp: Arc<Mutex<crate::editor::engine::lsp::LspManager>>,

    // Ext-Host Sidecar Bridge
    pub ext_host: Arc<Mutex<crate::editor::engine::vsx::ext_host::ExtHostBridge>>,

    // LSP Advanced Language Features
    pub code_actions: crate::editor::engine::code_actions::CodeActionState,
    pub rename: crate::editor::engine::rename::RenameState,
    pub references: crate::editor::engine::references::ReferencesState,
    pub sticky_scroll: crate::editor::engine::sticky_scroll::StickyScrollModel,
    pub minimap: crate::editor::engine::minimap::MinimapModel,
    pub bracket_pairs: crate::editor::engine::bracket_pairs::BracketPairState,
    pub multi_cursor: crate::editor::engine::multi_cursor::MultiCursorState,
}

fn resolve_workspace_root(explicit: Option<PathBuf>) -> (Option<PathBuf>, bool) {
    if let Some(path) = explicit {
        if path.is_dir() {
            return (Some(path), true);
        } else if path.is_file() {
            return (path.parent().map(|p| p.to_path_buf()), true);
        }
    }
    // Check if we are running in active repository development mode
    if let Ok(current) = std::env::current_dir() {
        if current.join("src-native").exists()
            && (current.join("src-tauri").exists() || current.join("package.json").exists())
        {
            return (Some(current), true);
        }
    }
    // In installed release mode or when launched without CLI folder arguments, do not mount install directory
    (None, false)
}

pub fn seed_canvases() -> Vec<CanvasSpec> {
    use crate::domain::canvas::*;
    vec![
        CanvasSpec {
            id: "getting-started".to_string(),
            title: "Agent Mission Control".to_string(),
            subtitle: Some("Live status of the Hades agent loop".to_string()),
            updated_at: chrono::Utc::now().timestamp_millis(),
            blocks: vec![
                CanvasBlock::Stats {
                    items: vec![
                        CanvasStatItem {
                            label: "Tasks completed".to_string(),
                            value: "128".to_string(),
                            tone: Some(CanvasTone::Success),
                            hint: Some("+12 today".to_string()),
                        },
                        CanvasStatItem {
                            label: "Tokens spent".to_string(),
                            value: "1.4M".to_string(),
                            tone: Some(CanvasTone::Info),
                            hint: None,
                        },
                        CanvasStatItem {
                            label: "Active watchers".to_string(),
                            value: "3".to_string(),
                            tone: Some(CanvasTone::Warning),
                            hint: None,
                        },
                    ],
                },
                CanvasBlock::Progress {
                    title: Some("Port progress".to_string()),
                    items: vec![
                        CanvasProgressItem {
                            label: "Panels ported".to_string(),
                            value: 14.0,
                            max: Some(26.0),
                            tone: Some(CanvasTone::Success),
                        },
                        CanvasProgressItem {
                            label: "Editor features".to_string(),
                            value: 6.0,
                            max: Some(10.0),
                            tone: Some(CanvasTone::Info),
                        },
                    ],
                },
                CanvasBlock::Todo {
                    title: Some("Next up".to_string()),
                    items: vec![
                        CanvasTodoItem {
                            text: "Wire JS controller events".to_string(),
                            done: false,
                        },
                        CanvasTodoItem {
                            text: "Ship status bar cosmetics".to_string(),
                            done: false,
                        },
                        CanvasTodoItem {
                            text: "Fix XR jailbreak freeze".to_string(),
                            done: true,
                        },
                    ],
                },
            ],
        },
        CanvasSpec {
            id: "speedrun-stats".to_string(),
            title: "Speedrun Leaderboard".to_string(),
            subtitle: Some("Community best times".to_string()),
            updated_at: chrono::Utc::now().timestamp_millis(),
            blocks: vec![
                CanvasBlock::Chart {
                    chart: CanvasChartKind::Bar,
                    title: Some("Time vs. goal".to_string()),
                    labels: vec![
                        "M1".to_string(),
                        "M2".to_string(),
                        "M3".to_string(),
                        "M4".to_string(),
                        "M5".to_string(),
                    ],
                    series: vec![
                        CanvasChartSeries {
                            name: Some("Best".to_string()),
                            values: vec![94.2, 88.5, 79.1, 70.7, 61.0],
                            tone: Some(CanvasTone::Success),
                        },
                        CanvasChartSeries {
                            name: Some("Goal".to_string()),
                            values: vec![96.0, 90.0, 84.0, 78.0, 72.0],
                            tone: Some(CanvasTone::Info),
                        },
                    ],
                },
                CanvasBlock::Table {
                    title: Some("Top splits".to_string()),
                    columns: vec!["Run".to_string(), "Time".to_string(), "Diff".to_string()],
                    rows: vec![
                        vec![
                            "Current PB".to_string(),
                            "61.0s".to_string(),
                            "-2.1s".to_string(),
                        ],
                        vec![
                            "Field run".to_string(),
                            "63.4s".to_string(),
                            "+0.3s".to_string(),
                        ],
                    ],
                },
            ],
        },
    ]
}

impl HadesNativeState {
    pub fn load_persisted_kortex_messages(workspace_root: &Path) -> Vec<ChatMessage> {
        let aim_path = workspace_root.join(".aim").join("memory.aim");
        if !aim_path.exists() {
            return Vec::new();
        }
        if let Ok(bytes) = std::fs::read(&aim_path) {
            let mut header_end = 0;
            let mut depth = 0;
            let mut in_string = false;
            let mut escaped = false;
            for (i, &b) in bytes.iter().enumerate() {
                if escaped { escaped = false; continue; }
                match b {
                    b'\\' => escaped = true,
                    b'"' => in_string = !in_string,
                    b'{' if !in_string => depth += 1,
                    b'}' if !in_string => {
                        depth -= 1;
                        if depth == 0 {
                            header_end = i + 1;
                            break;
                        }
                    }
                    _ => {}
                }
            }
            if header_end > 0 {
                if let Ok(val) = serde_json::from_slice::<serde_json::Value>(&bytes[..header_end]) {
                    if let Some(msgs_val) = val.get("kortex").and_then(|k| k.get("session_messages")).and_then(|m| m.as_array()) {
                        let mut msgs = Vec::new();
                        for item in msgs_val {
                            let role = item.get("role").and_then(|r| r.as_str()).unwrap_or("user");
                            if role == "system" {
                                continue;
                            }
                            let content_str = if let Some(s) = item.get("content").and_then(|c| c.as_str()) {
                                s.to_string()
                            } else if let Some(obj) = item.get("content").and_then(|c| c.as_object()) {
                                obj.get("Text").or_else(|| obj.get("text")).and_then(|v| v.as_str()).unwrap_or("").to_string()
                            } else {
                                String::new()
                            };
                            let trimmed = content_str.trim();
                            if trimmed.is_empty() {
                                continue;
                            }
                            // Guard: Skip if content is internal prompt preamble
                            if trimmed.starts_with("PROJECT ROOT:") || trimmed.starts_with("You are a coding agent") || trimmed.contains("Behaviour\n- Do the work") {
                                continue;
                            }
                            let is_user = role == "user";
                            let m = if is_user {
                                ChatMessage::user(content_str)
                            } else {
                                ChatMessage::assistant(content_str)
                            };
                            msgs.push(m);
                        }
                        return msgs;
                    }
                }
            }
        }
        Vec::new()
    }

    pub fn new() -> Self {
        Self::with_initial_path(None)
    }

    pub fn with_initial_path(initial_path: Option<PathBuf>) -> Self {
        let (resolved_root, has_workspace) = resolve_workspace_root(initial_path.clone());
        let root = resolved_root.unwrap_or_default();
        let mut file_tree = Vec::new();
        if has_workspace && root.is_dir() {
            FileNode::populate_tree(&root, &mut file_tree, 1);
        }
        let recent_workspaces = Self::load_recent_workspaces();

        let restored_messages = if has_workspace && root.is_dir() {
            Self::load_persisted_kortex_messages(&root)
        } else {
            Vec::new()
        };
        let active_messages = restored_messages;

        let initial_thread = AgentThread {
            id: "thread-1".to_string(),
            title: "Chat 1".to_string(),
            created_at: "Just now".to_string(),
            messages: active_messages.clone(),
        };

        let term_display = if has_workspace && root.is_dir() {
            root.display().to_string()
        } else {
            dirs::home_dir()
                .map(|h| h.display().to_string())
                .unwrap_or_else(|| "C:\\".to_string())
        };

        let terminal_lines = vec![
            "Microsoft Windows [Version 10.0.26100.3194]".to_string(),
            "(c) Microsoft Corporation. All rights reserved.".to_string(),
            "".to_string(),
            format!("PS {}> [Terminal Ready — Session initializes on first command]", term_display),
        ];

        // Zero-lag startup: PTY and local AI services stay dormant until explicitly started
        let pty: Option<Arc<crate::terminal_pty::NativePtySession>> = None;
        let terminal_sessions: Vec<Arc<crate::terminal_pty::NativePtySession>> = Vec::new();

        let mut state = Self {
            services: None,
            theme: Theme::cursor_dark(),
            left_sidebar_open: true,
            right_sidebar_open: true,
            iphone_preview_open: false,
            bottom_panel_open: false,
            active_activity: ActivityTab::Explorer,
            active_right_tab: RightSidebarTab::AgentChat,
            active_bottom_tab: BottomPanelTab::Terminal,
            active_emulator_subtab: EmulatorSubTab::Device,
            focused_panel: FocusedPanel::Editor,
            tabs: Vec::new(),
            active_tab_idx: None,
            is_split_editor: false,
            active_pane: 0,
            secondary_tabs: Vec::new(),
            active_secondary_tab_idx: None,
            split_ratio: 0.5,
            split_horizontal: false,

            // Editor Find Bar
            find_bar_open: false,
            find_query: String::new(),
            find_match_count: 0,
            find_case_sensitive: false,
            find_regex: false,
            find_whole_word: false,
            replace_query: String::new(),
            find_replace_mode: false,

            // Steering Rules & Hooks
            global_rules: "// Project Steering Rules\n// 1. Prioritize zero heap allocations and memory safety in hot paths.\n// 2. Hardware-accelerated Direct3D 12 rendering at 120 FPS.\n// 3. Zero mockups: Full in-process bindings with native services.".to_string(),
            agent_hooks: vec![
                AgentHook {
                    id: "hook-1".to_string(),
                    pattern: "src/**/*.rs".to_string(),
                    prompt: "Ensure idiomatic Rust, zero unwraps, and run cargo check".to_string(),
                    enabled: true,
                },
                AgentHook {
                    id: "hook-2".to_string(),
                    pattern: "**/*.toml".to_string(),
                    prompt: "Verify dependencies and features consistency".to_string(),
                    enabled: true,
                },
            ],

            // Specs & Spec-to-Code
            specs_prompt: String::new(),
            specs_projects: vec![
                SpecsProject {
                    id: 1,
                    name: "Native GPUI Migration".to_string(),
                    specs: "THE SYSTEM SHALL provide hardware-accelerated Direct3D 12 rendering at 120 FPS.\nWHEN a user opens a file, THE SYSTEM SHALL display syntax highlighting via Tree-Sitter.\nWHEN SCM view is active, THE SYSTEM SHALL parse git status and git graph in-process.".to_string(),
                    tasks: vec![
                        SpecsTask {
                            id: "task-1".to_string(),
                            title: "Hardware-accelerated window and layout shell".to_string(),
                            status: "done".to_string(),
                            complexity: "Medium".to_string(),
                            estimated_hours: 4,
                            details: "Direct3D 12 presentation with GPUI".to_string(),
                        },
                        SpecsTask {
                            id: "task-2".to_string(),
                            title: "In-process Git Graph and SCM diff viewer".to_string(),
                            status: "done".to_string(),
                            complexity: "Hard".to_string(),
                            estimated_hours: 8,
                            details: "Parse git log --graph and render inline colored hunks".to_string(),
                        },
                        SpecsTask {
                            id: "task-3".to_string(),
                            title: "Specs-to-Code and Rules engine".to_string(),
                            status: "in_progress".to_string(),
                            complexity: "Medium".to_string(),
                            estimated_hours: 6,
                            details: "Interactive requirements, tasks breakdown, and hooks".to_string(),
                        },
                    ],
                }
            ],
            active_specs_project_idx: Some(0),

            // Kortex Services
            kortex_chunks_indexed: 1420,
            kortex_indexing_status: "Active · 1,420 chunks indexed (Qwen3-0.6B)".to_string(),
            kortex_kv_running: true,

            agent_mode: "Ask".to_string(),
            agent_model: "Qwen3.8-27B-Uncensored-Cyber-agentic-imatrix-GGUF-Qwen3.8-27B-Uncensored-Cyber-IQ4_XS-imatrix-fromq8".to_string(),
            available_ai_models: vec![
                // Local Models (Primary · GPU Accelerated via Lemonade)
                "Qwen3.8-27B-Uncensored-Cyber-agentic-imatrix-GGUF-Qwen3.8-27B-Uncensored-Cyber-IQ4_XS-imatrix-fromq8".to_string(),
                "Qwen3.8-35B-A3B-Q4_K_M.gguf".to_string(),
                "Qwen3.8-27B-GGUF-IQ3_XXS".to_string(),
                "Qwen3.5-4B-GGUF-Q5_K_M".to_string(),
                "Escha-W2-35B-A3B-ROCmFP2-Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf".to_string(),
                "Qwen3-Embedding-0.6B-GGUF".to_string(),
                // ModelScope (Alibaba Qwen Ambassador)
                "Qwen 2.5 Coder (ModelScope)".to_string(),
                "Qwen Max (ModelScope)".to_string(),
                "Qwen Plus (ModelScope)".to_string(),
                "DeepSeek V3 (ModelScope)".to_string(),
                // Cloud BYOK (Bring Your Own Key)
                "Claude 3.5 Sonnet".to_string(),
                "Gemini 1.5 Pro".to_string(),
                "GPT-4o".to_string(),
                "DeepSeek V3".to_string(),
            ],
            anthropic_api_key: Self::read_initial_api_key("anthropic", "ANTHROPIC_API_KEY"),
            openai_api_key: Self::read_initial_api_key("openai", "OPENAI_API_KEY"),
            gemini_api_key: Self::read_initial_api_key("google", "GEMINI_API_KEY"),
            deepseek_api_key: Self::read_initial_api_key("deepseek", "DEEPSEEK_API_KEY"),
            modelscope_api_key: Self::read_initial_api_key("modelscope", "MODELSCOPE_API_KEY"),
            reasoning_active: false,
            composer_input: String::new(),
            chat_messages: active_messages,
            is_agent_thinking: false,
            ai_turn: Arc::new(Mutex::new(AiTurnState::default())),
            agent_threads: vec![initial_thread],
            active_thread_idx: 0,
            is_yolo_mode: true,
            is_continuous_mode: false,
            model_picker_open: false,
            mode_picker_open: false,
            expanded_thoughts: std::collections::HashSet::new(),
            live_thought_expanded: false,
            copied_msg_idx: None,
            editing_msg_idx: None,
            edit_msg_input: String::new(),
            thinking_start_instant: None,
            has_workspace,
            workspace_root: root.clone(),
            recent_workspaces,
            file_tree,
            explorer_create: None,
            search: crate::panels::SearchState::default(),
            scm: crate::panels::ScmState::default(),
            problems: crate::panels::ProblemsState::default(),
            output: crate::panels::OutputState::default(),
            debug: crate::panels::DebugState::default(),
            test_explorer: crate::panels::TestExplorerState::default(),
            markdown_preview: crate::panels::MarkdownPreviewState::default(),
            run_configs: crate::panels::RunConfigsState::new(&root),
            ports: crate::panels::PortsState::default(),
            jobs: crate::panels::JobsState::default(),
            debug_console: crate::panels::DebugConsoleState::default(),
            logcat: crate::panels::LogcatState::default(),
            browser: crate::panels::BrowserPanelState::default(),
            vector_search: crate::panels::VectorSearchState::default(),
            mcp_store: crate::panels::McpStoreState::default(),
            outline: crate::panels::OutlineState::default(),
            keybindings: crate::panels::KeybindingsState::new(),
            canvases: seed_canvases(),
            workspace_trusted: true,
            workspace_trust_dismissed: false,
            pending_tool_permission: None,
            terminal_lines,
            terminal_input: String::new(),
            terminal_active_prompt: String::new(),
            pty,
            terminal_sessions,
            active_terminal_idx: 0,
            status_message: "Ready (GPUI Native 120 FPS)".to_string(),

            context_menu: ContextMenuState::default(),
            toast_manager: ToastManager::default(),
            inline_edit: InlineEditState::default(),
            current_thought: None,
            inference_health: crate::ui::InferenceHealthState::default(),
            settings_category: 0,
            settings_item: 0,
            settings_search: String::new(),
            default_tab_size: 4,
            format_on_save: true,
            inlay_hints_enabled: true,
            active_theme_name: "Cursor Dark".to_string(),
            editor_font_size: 14,
            line_numbers_enabled: true,
            telemetry_enabled: false,
            default_shell: "PowerShell".to_string(),
            agent_auto_approve_terminal: false,
            agent_auto_approve_file_write: true,
            agent_reasoning_effort: "High".to_string(),
            sidebar_width: 260.0,
            right_sidebar_width: 360.0,
            iphone_preview_width: 390.0,
            bottom_panel_height: 240.0,
            is_resizing_sidebar: false,
            is_resizing_right_sidebar: false,
            is_resizing_iphone_preview: false,
            is_resizing_bottom_panel: false,
            minimap_hover: None,

            hover_info: None,
            hover_row: 0,

            vsx: {
                let extensions_dir = dirs::home_dir()
                    .map(|h| h.join(".vscodium-rust").join("extensions"))
                    .unwrap_or_else(|| root.join("extensions"));
                crate::editor::engine::vsx::manager::VsxManager::new(extensions_dir)
            },
            extensions_category: 0,
            lsp: Arc::new(Mutex::new(crate::editor::engine::lsp::LspManager::new(root.clone()))),
            ext_host: Arc::new(Mutex::new(crate::editor::engine::vsx::ext_host::ExtHostBridge::spawn(&root))),
            code_actions: crate::editor::engine::code_actions::CodeActionState::new(),
            rename: crate::editor::engine::rename::RenameState::new(),
            references: crate::editor::engine::references::ReferencesState::new(),
            sticky_scroll: crate::editor::engine::sticky_scroll::StickyScrollModel::new(),
            minimap: crate::editor::engine::minimap::MinimapModel::new(),
            bracket_pairs: crate::editor::engine::bracket_pairs::BracketPairState::default(),
            multi_cursor: crate::editor::engine::multi_cursor::MultiCursorState::default(),
        };
        state.vsx.language_services.active_ext_host = Some(state.ext_host.clone());

        state.load_preferences();
        let mut opened_initial_file = false;
        if let Some(ref p) = initial_path {
            if p.is_file() {
                state.open_file(&p.to_string_lossy());
                opened_initial_file = true;
            }
        }
        if !opened_initial_file {
            let restored = state.load_session();
            if !restored && state.has_workspace && state.workspace_root.join("src-native").exists() {
                let main_rs = state.workspace_root.join("src-native").join("src").join("main.rs");
                if main_rs.exists() {
                    state.open_file(&main_rs.to_string_lossy());
                }
            }
        }
        state
    }

    pub fn workspace_display_name(&self) -> String {
        if !self.has_workspace || self.workspace_root.as_os_str().is_empty() {
            "NO FOLDER OPENED".to_string()
        } else {
            self.workspace_root
                .file_name()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| self.workspace_root.to_string_lossy().to_string())
        }
    }

    pub fn load_recent_workspaces() -> Vec<PathBuf> {
        if let Some(home) = dirs::home_dir() {
            let file = home.join(".vscodium-rust").join("recent_workspaces.json");
            if let Ok(data) = std::fs::read_to_string(&file) {
                if let Ok(list) = serde_json::from_str::<Vec<PathBuf>>(&data) {
                    return list.into_iter().filter(|p| p.is_dir()).collect();
                }
            }
        }
        Vec::new()
    }

    pub fn add_recent_workspace(&mut self, path: &Path) {
        let p = path.to_path_buf();
        self.recent_workspaces.retain(|existing| existing != &p);
        self.recent_workspaces.insert(0, p);
        if self.recent_workspaces.len() > 10 {
            self.recent_workspaces.truncate(10);
        }
        if let Some(home) = dirs::home_dir() {
            let dir = home.join(".vscodium-rust");
            let _ = std::fs::create_dir_all(&dir);
            let file = dir.join("recent_workspaces.json");
            if let Ok(json) = serde_json::to_string_pretty(&self.recent_workspaces) {
                let _ = std::fs::write(file, json);
            }
        }
    }

    pub fn populate_tree(dir: &Path, out: &mut Vec<FileNode>, depth_limit: usize) {
        FileNode::populate_tree(dir, out, depth_limit);
    }

    pub fn refresh_file_tree(&mut self) {
        if !self.has_workspace || !self.workspace_root.is_dir() {
            self.file_tree.clear();
            return;
        }
        let root = self.workspace_root.clone();
        let mut tree = Vec::new();
        FileNode::populate_tree(&root, &mut tree, 1);
        self.file_tree = tree;
    }

    pub fn open_workspace(&mut self, path: PathBuf) {
        if path.is_dir() {
            self.has_workspace = true;
            self.workspace_root = path.clone();
            self.add_recent_workspace(&path);
            self.refresh_file_tree();
            self.tabs.clear();
            self.active_tab_idx = None;
            self.toast_manager.push_info(&format!("Opened folder: {}", self.workspace_root.display()));
            self.save_session();
        }
    }

    pub fn create_new_file(&mut self, relative_name: &str) {
        let full_path = if self.has_workspace && self.workspace_root.is_dir() {
            self.workspace_root.join(relative_name)
        } else if let Some(home) = dirs::home_dir() {
            home.join(relative_name)
        } else {
            PathBuf::from(relative_name)
        };
        if !full_path.exists() {
            let _ = std::fs::write(&full_path, "");
        }
        self.refresh_file_tree();
        self.open_file(&full_path.to_string_lossy());
    }

    pub fn clear_terminal(&mut self) {
        self.terminal_lines.clear();
        self.terminal_input.clear();
        self.terminal_active_prompt.clear();
        if let Some(pty) = self.active_pty() {
            pty.clear();
        }
    }

    pub fn clear_ai_chat(&mut self) {
        self.chat_messages.clear();
    }

    pub fn set_thought(&mut self, logic: &str, action: &str, confidence: Option<f32>) {
        self.current_thought = Some(crate::ui::AiThought::new(logic, action, confidence));
    }

    pub fn clear_thought(&mut self) {
        self.current_thought = None;
    }

    pub fn toggle_inference_health(&mut self) {
        self.inference_health.toggle_visibility();
    }

    pub fn active_pty(&self) -> Option<Arc<crate::terminal_pty::NativePtySession>> {
        self.terminal_sessions
            .get(self.active_terminal_idx)
            .cloned()
            .or_else(|| self.pty.clone())
    }

    pub fn ensure_pty(&mut self) -> Option<Arc<crate::terminal_pty::NativePtySession>> {
        if self.pty.is_none() && self.terminal_sessions.is_empty() {
            self.spawn_terminal(None);
        }
        self.active_pty()
    }

    pub fn ensure_services(&mut self) -> &AppServices {
        if self.services.is_none() {
            let config_dir = dirs::config_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("vscodium-rust");
            self.services = Some(AppServices::init(config_dir));
        }
        self.services.as_ref().unwrap()
    }

    pub fn spawn_terminal(&mut self, shell: Option<&str>) {
        let root = self.workspace_root.clone();
        let idx = self.terminal_sessions.len() + 1;
        let shell_cmd = shell.unwrap_or_else(|| {
            if cfg!(windows) {
                "powershell.exe"
            } else {
                "bash"
            }
        });
        let title = format!(
            "{idx}: {}",
            if shell_cmd.contains("powershell") || shell_cmd.contains("pwsh") {
                "pwsh"
            } else if shell_cmd.contains("cmd") {
                "cmd"
            } else {
                "bash"
            }
        );
        if let Ok(sess) =
            crate::terminal_pty::NativePtySession::spawn_with_shell(&root, shell_cmd, &title)
        {
            let arc = Arc::new(sess);
            self.terminal_sessions.push(arc);
            self.active_terminal_idx = self.terminal_sessions.len() - 1;
            self.pty = Some(self.terminal_sessions[self.active_terminal_idx].clone());
            self.terminal_lines.clear();
        }
    }

    pub fn kill_active_terminal(&mut self) {
        if self.terminal_sessions.len() > 1 {
            self.terminal_sessions.remove(self.active_terminal_idx);
            if self.active_terminal_idx >= self.terminal_sessions.len() {
                self.active_terminal_idx = self.terminal_sessions.len().saturating_sub(1);
            }
            self.pty = self
                .terminal_sessions
                .get(self.active_terminal_idx)
                .cloned();
            self.terminal_lines.clear();
            if let Some(pty) = &self.pty {
                self.terminal_lines = pty.poll_lines();
            }
        } else if let Some(sess) = self.terminal_sessions.first() {
            sess.clear();
            self.terminal_lines.clear();
        }
    }

    pub fn select_terminal(&mut self, idx: usize) {
        if idx < self.terminal_sessions.len() {
            self.active_terminal_idx = idx;
            self.pty = Some(self.terminal_sessions[idx].clone());
            self.terminal_lines.clear();
            if let Some(pty) = &self.pty {
                self.terminal_lines = pty.poll_lines();
            }
        }
    }

    pub fn create_agent_thread(&mut self) {
        // Save current messages to active thread before creating new one
        if let Some(t) = self.agent_threads.get_mut(self.active_thread_idx) {
            t.messages = self.chat_messages.clone();
        }

        let count = self.agent_threads.len() + 1;
        let thread = AgentThread {
            id: format!("thread-{count}"),
            title: format!("New Conversation {count}"),
            created_at: chrono::Local::now().format("%H:%M:%S").to_string(),
            messages: Vec::new(),
        };
        self.agent_threads.push(thread);
        self.active_thread_idx = self.agent_threads.len() - 1;
        self.chat_messages.clear();
    }

    pub fn select_agent_thread(&mut self, idx: usize) {
        if idx < self.agent_threads.len() {
            if let Some(t) = self.agent_threads.get_mut(self.active_thread_idx) {
                t.messages = self.chat_messages.clone();
            }
            self.active_thread_idx = idx;
            self.chat_messages = self.agent_threads[idx].messages.clone();
        }
    }

    pub fn sync_terminal_lines(&mut self) {
        if let Some(pty) = self.active_pty() {
            if let Ok(mut pty_lines) = pty.lines.lock() {
                if !pty_lines.is_empty() {
                    self.terminal_lines.append(&mut *pty_lines);
                    if self.terminal_lines.len() > 2000 {
                        let excess = self.terminal_lines.len() - 2000;
                        self.terminal_lines.drain(0..excess);
                    }
                }
            }
            if let Ok(prompt) = pty.active_prompt.lock() {
                self.terminal_active_prompt = prompt.clone();
            }
        }
    }

    pub fn current_streaming_text(&self) -> String {
        if let Ok(t) = self.ai_turn.lock() {
            if let Ok(sc) = t.streamed_content.lock() {
                return sc.clone();
            }
        }
        String::new()
    }

    pub fn current_streaming_thoughts(&self) -> String {
        if let Ok(t) = self.ai_turn.lock() {
            if let Ok(st) = t.streamed_thoughts.lock() {
                return st.clone();
            }
        }
        String::new()
    }

    pub fn current_live_metrics(&self) -> (usize, f64, f64, Option<f64>) {
        if let Ok(t) = self.ai_turn.lock() {
            let tokens = t.live_tokens_count.lock().map(|c| *c).unwrap_or(0);
            let tps = t.live_tokens_per_sec.lock().map(|s| *s).unwrap_or(0.0);
            let elapsed = t.live_elapsed_secs.lock().map(|e| *e).unwrap_or(0.0);
            let mtp = t.live_mtp_rate.lock().ok().and_then(|m| *m);
            return (tokens, tps, elapsed, mtp);
        }
        (0, 0.0, 0.0, None)
    }

    pub fn start_edit_message(&mut self, idx: usize) {
        if idx < self.chat_messages.len() && self.chat_messages[idx].is_user {
            self.editing_msg_idx = Some(idx);
            self.edit_msg_input = self.chat_messages[idx].text.clone();
        }
    }

    pub fn cancel_edit_message(&mut self) {
        self.editing_msg_idx = None;
        self.edit_msg_input.clear();
    }

    pub fn save_and_resend_message(&mut self, idx: usize) {
        let new_text = self.edit_msg_input.trim().to_string();
        if new_text.is_empty() || self.is_agent_thinking {
            return;
        }
        self.cancel_edit_message();
        if idx < self.chat_messages.len() {
            self.chat_messages.truncate(idx);
        }
        self.send_ai_prompt(&new_text);
    }

    pub fn toggle_markdown_preview(&mut self) {
        self.markdown_preview.is_open = !self.markdown_preview.is_open;
    }

    pub fn trust_workspace(&mut self) {
        self.workspace_trusted = true;
        self.workspace_trust_dismissed = true;
        self.status_message = "Workspace trusted. Full features enabled.".to_string();
    }

    pub fn dismiss_workspace_trust(&mut self) {
        self.workspace_trust_dismissed = true;
    }

    pub fn request_tool_permission(
        &mut self,
        id: String,
        tool: String,
        args: String,
        level: String,
    ) {
        if self.is_yolo_mode {
            return;
        }
        self.pending_tool_permission = Some(ToolPermissionRequest {
            id,
            tool,
            args,
            level,
        });
    }

    pub fn respond_tool_permission(&mut self, _id: &str, allow: bool) {
        self.pending_tool_permission = None;
        if allow {
            self.status_message = "Tool execution permitted.".to_string();
        } else {
            self.status_message = "Tool execution denied by user.".to_string();
        }
    }

    pub fn poll_ai_turn(&mut self) {
        if let Ok(t) = self.ai_turn.try_lock() {
            self.is_agent_thinking = t.is_running;
            if let Ok(mut fin) = t.finished_turn.try_lock() {
                if let Some(mut msg) = fin.take() {
                    if let Some(start) = self.thinking_start_instant.take() {
                        msg.thought_duration_ms = Some(start.elapsed().as_millis() as u64);
                    }
                    self.chat_messages.push(msg);
                    self.is_agent_thinking = false;
                    if let Some(thread) = self.agent_threads.get_mut(self.active_thread_idx) {
                        thread.messages = self.chat_messages.clone();
                    }
                }
            }
        }
    }

    pub fn toggle_model_picker(&mut self) {
        self.model_picker_open = !self.model_picker_open;
        if self.model_picker_open {
            self.mode_picker_open = false;
        }
    }

    pub fn toggle_mode_picker(&mut self) {
        self.mode_picker_open = !self.mode_picker_open;
        if self.mode_picker_open {
            self.model_picker_open = false;
        }
    }

    pub fn set_agent_model(&mut self, model: &str) {
        self.agent_model = model.to_string();
        self.inference_health.select_model(model);
        self.model_picker_open = false;
        self.toast_manager.push_info(&format!("Model switched: {model}"));
    }

    pub fn set_agent_mode(&mut self, mode: &str) {
        self.agent_mode = mode.to_string();
        self.mode_picker_open = false;
        self.toast_manager.push_info(&format!("Mode switched: {mode}"));
    }

    pub fn toggle_message_thoughts(&mut self, idx: usize) {
        if self.expanded_thoughts.contains(&idx) {
            self.expanded_thoughts.remove(&idx);
        } else {
            self.expanded_thoughts.insert(idx);
        }
    }

    pub fn toggle_live_thought(&mut self) {
        self.live_thought_expanded = !self.live_thought_expanded;
    }

    pub fn cancel_ai_turn(&mut self) {
        if let Ok(mut t) = self.ai_turn.lock() {
            t.is_running = false;
        }
        self.is_agent_thinking = false;
        self.status_message = "AI generation cancelled.".to_string();
    }

    pub fn clear_chat(&mut self) {
        self.chat_messages.clear();
        self.status_message = "Chat history cleared.".to_string();
    }

    pub fn set_activity(&mut self, tab: ActivityTab) {
        if tab == ActivityTab::Settings {
            self.open_settings_tab();
            return;
        }
        if self.active_activity == tab {
            self.left_sidebar_open = !self.left_sidebar_open;
        } else {
            self.active_activity = tab;
            self.left_sidebar_open = true;
        }
        self.save_session();
    }

    pub fn toggle_left_sidebar(&mut self) {
        self.left_sidebar_open = !self.left_sidebar_open;
        self.save_session();
    }

    pub fn toggle_right_sidebar(&mut self) {
        self.right_sidebar_open = !self.right_sidebar_open;
        self.save_session();
    }

    pub fn toggle_iphone_preview(&mut self) {
        self.iphone_preview_open = !self.iphone_preview_open;
        self.save_session();
    }

    pub fn toggle_dual_sidebar(&mut self) {
        // If either is closed, open both. If both are open, close both.
        if self.right_sidebar_open && self.iphone_preview_open {
            self.right_sidebar_open = false;
            self.iphone_preview_open = false;
        } else {
            self.right_sidebar_open = true;
            self.iphone_preview_open = true;
        }
        self.save_session();
    }

    pub fn toggle_bottom_panel(&mut self) {
        self.bottom_panel_open = !self.bottom_panel_open;
        self.save_session();
    }

    pub fn resize_sidebar(&mut self, delta: f32) {
        self.sidebar_width = (self.sidebar_width + delta).clamp(160.0, 600.0);
    }

    pub fn resize_right_sidebar(&mut self, delta: f32) {
        self.right_sidebar_width = (self.right_sidebar_width + delta).clamp(220.0, 700.0);
    }

    pub fn resize_bottom_panel(&mut self, delta: f32) {
        self.bottom_panel_height = (self.bottom_panel_height + delta).clamp(100.0, 600.0);
    }

    pub fn set_bottom_tab(&mut self, tab: BottomPanelTab) {
        if self.active_bottom_tab == tab && self.bottom_panel_open {
            self.bottom_panel_open = false;
        } else {
            self.active_bottom_tab = tab;
            self.bottom_panel_open = true;
        }
    }

    /// Stream log lines into the native multi-channel output panel
    pub fn log_output(&mut self, channel: &str, line: impl Into<String>) {
        self.output.log(channel, line);
    }

    pub fn open_file(&mut self, path: &str) {
        let p = Path::new(path);
        let resolved = if p.is_absolute() {
            p.to_path_buf()
        } else if self.has_workspace {
            self.workspace_root.join(p)
        } else {
            p.to_path_buf()
        };
        let resolved_str = resolved.to_string_lossy().to_string();

        if let Some(idx) = self
            .tabs
            .iter()
            .position(|t| t.path == resolved_str || t.path == path)
        {
            self.active_tab_idx = Some(idx);
            self.sync_lsp_for_active_tab();
            self.save_session();
            return;
        }

        match std::fs::read_to_string(&resolved) {
            Ok(content) => {
                let title = resolved
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();
                let mut tab = OpenTab::new(resolved_str, title, content);
                tab.model.tab_size = self.default_tab_size;
                self.tabs.push(tab);
                self.active_tab_idx = Some(self.tabs.len() - 1);
                self.sync_lsp_for_active_tab();
                self.save_session();
            }
            Err(e) => {
                self.toast_manager
                    .push_error(&format!("Cannot open {}: {}", path, e));
            }
        }
    }

    pub fn open_settings_tab(&mut self) {
        if let Some(idx) = self
            .tabs
            .iter()
            .position(|t| t.is_settings || t.path == "hades://settings")
        {
            self.active_tab_idx = Some(idx);
            self.save_session();
            return;
        }
        let tab = OpenTab::new_settings();
        self.tabs.push(tab);
        self.active_tab_idx = Some(self.tabs.len() - 1);
        self.save_session();
    }

    pub fn open_canvas(&mut self, id: &str) {
        let Some(canvas) = self.canvases.iter().find(|c| c.id == id) else {
            self.toast_manager
                .push_error(&format!("Canvas '{id}' not found"));
            return;
        };
        let path = format!("canvas://{id}");
        if let Some(idx) = self.tabs.iter().position(|t| t.path == path) {
            self.active_tab_idx = Some(idx);
            self.save_session();
            return;
        }
        let title = canvas.title.clone();
        let tab = OpenTab::new(path, title, spec_to_json(canvas));
        self.tabs.push(tab);
        self.active_tab_idx = Some(self.tabs.len() - 1);
        self.save_session();
    }

    pub fn refresh_canvases(&mut self) {
        let dir = self.workspace_root.join(".agent").join("canvases");
        let mut fresh: Vec<CanvasSpec> = Vec::new();
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| e.eq_ignore_ascii_case("json"))
                    .unwrap_or(false)
                {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        if let Ok(raw) = serde_json::from_str::<serde_json::Value>(&content) {
                            if let Some(spec) = normalize_canvas_spec(&raw) {
                                fresh.push(spec);
                            }
                        }
                    }
                }
            }
        }
        fresh.sort_by(|a, b| a.title.cmp(&b.title));
        self.canvases = fresh;
    }

    pub fn select_tab(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.active_tab_idx = Some(idx);
            self.sync_lsp_for_active_tab();
            self.save_session();
        }
    }

    pub fn close_tab(&mut self, idx: usize) {
        if idx < self.tabs.len() {
            self.tabs.remove(idx);
            if self.tabs.is_empty() {
                self.active_tab_idx = None;
            } else if let Some(cur) = self.active_tab_idx {
                if cur >= self.tabs.len() {
                    self.active_tab_idx = Some(self.tabs.len() - 1);
                }
            }
            self.sync_lsp_for_active_tab();
            self.save_session();
        }
    }

    /// Synchronize active LSP client and notify didOpen for active tab
    pub fn sync_lsp_for_active_tab(&mut self) {
        let (lang, path, content) = match self.active_tab() {
            Some(tab) => {
                if tab.model.language == "plaintext" || tab.model.language.is_empty() {
                    self.vsx.language_services.active_lsp = None;
                    return;
                }
                (
                    tab.model.language.clone(),
                    PathBuf::from(&tab.path),
                    tab.lines.join("\n"),
                )
            }
            None => {
                self.vsx.language_services.active_lsp = None;
                return;
            }
        };

        if lang == "plaintext" || content.len() > 1_000_000 {
            self.vsx.language_services.active_lsp = None;
            return;
        }

        if let Ok(mut mgr) = self.lsp.lock() {
            if let Some(client) = mgr.ensure_client(&lang) {
                let _ = client.did_open(&path, &lang, &content);
                self.vsx.language_services.active_lsp = Some(client);
            } else {
                self.vsx.language_services.active_lsp = None;
            }
        }

        if let Ok(bridge) = self.ext_host.lock() {
            bridge.document_opened(&path, &lang, &content);
        }
    }

    /// Pull latest diagnostics from LSP and language registry into active tab model
    pub fn refresh_active_tab_diagnostics(&mut self) {
        let path = match self.active_tab() {
            Some(tab) => PathBuf::from(&tab.path),
            None => return,
        };
        let diags = self.vsx.language_services.get_diagnostics(&path);
        if let Some(tab) = self.active_tab_mut() {
            tab.model.diagnostics = diags;
        }
    }

    /// Format document for active tab using LSP formatting or native toolchain
    pub fn format_active_tab(&mut self) -> bool {
        let (path, content, tab_size) = match self.active_tab() {
            Some(t) => (PathBuf::from(&t.path), t.lines.join("\n"), t.model.tab_size),
            None => return false,
        };

        if let Some(edits) = self
            .vsx
            .language_services
            .format_document(&path, &content, tab_size, true)
        {
            if !edits.is_empty() {
                if let Some(t) = self.active_tab_mut() {
                    t.model.apply_text_edits(&edits);
                    t.lines = t.model.buffer.lines().to_vec();
                    t.refresh_line_tokens();
                    t.dirty = true;
                    self.status_message = format!("Formatted {}", t.title);
                    return true;
                }
            }
        }
        false
    }

    /// Query and open Code Actions / Quick Fixes menu for the active cursor/selection
    pub fn trigger_code_actions_for_active_tab(&mut self) {
        let (path, row, col) = match self.active_tab() {
            Some(t) => (PathBuf::from(&t.path), t.cursor_row, t.cursor_col),
            None => return,
        };

        let actions = self
            .vsx
            .language_services
            .get_code_actions(&path, row, 0, row, col.max(1));
        if !actions.is_empty() {
            self.code_actions.open(actions, row, col);
        } else {
            self.status_message = "No code actions available".to_string();
        }
    }

    /// Apply the currently selected Code Action
    pub fn apply_selected_code_action(&mut self) -> bool {
        let action = match self.code_actions.selected_action() {
            Some(a) => a.clone(),
            None => return false,
        };

        let mut applied = false;
        if let Some(ws_edit) = action.edit {
            for (path, edits) in ws_edit.changes {
                for tab in &mut self.tabs {
                    if PathBuf::from(&tab.path) == path {
                        tab.model.apply_text_edits(&edits);
                        tab.lines = tab.model.buffer.lines().to_vec();
                        tab.refresh_line_tokens();
                        tab.dirty = true;
                        applied = true;
                    }
                }
            }
        }

        self.code_actions.close();
        if applied {
            self.status_message = format!("Applied: {}", action.title);
        }
        applied
    }

    /// Trigger rename symbol session for the word under the cursor
    pub fn trigger_rename_for_active_tab(&mut self) {
        let (path, row, col, word) = match self.active_tab_mut() {
            Some(t) => {
                let w = t.model.current_word_at_cursor();
                (PathBuf::from(&t.path), t.cursor_row, t.cursor_col, w)
            }
            None => return,
        };

        if word.is_empty() {
            self.status_message = "No symbol at cursor to rename".to_string();
            return;
        }

        self.rename.open(word, row, col, path);
    }

    /// Commit rename symbol across workspace
    pub fn commit_rename(&mut self) -> bool {
        if !self.rename.has_changed() {
            self.rename.close();
            return false;
        }

        let path = self.rename.file_path.clone();
        let row = self.rename.row;
        let col = self.rename.col;
        let new_name = self.rename.new_name.clone();

        let ws_edit = self
            .vsx
            .language_services
            .rename_symbol(&path, row, col, &new_name);
        self.rename.close();

        if let Some(edit) = ws_edit {
            let mut edit_count = 0;
            for (edit_path, edits) in edit.changes {
                for tab in &mut self.tabs {
                    if PathBuf::from(&tab.path) == edit_path {
                        tab.model.apply_text_edits(&edits);
                        tab.lines = tab.model.buffer.lines().to_vec();
                        tab.refresh_line_tokens();
                        tab.dirty = true;
                        edit_count += edits.len();
                    }
                }
            }
            self.status_message =
                format!("Renamed symbol to '{}' ({} edits)", new_name, edit_count);
            true
        } else {
            self.status_message = "Rename failed: LSP returned no edits".to_string();
            false
        }
    }

    /// Query all references to symbol under cursor and open references peek panel
    pub fn find_references_for_active_tab(&mut self) {
        let (lines, path, row, col, word) = match self.active_tab_mut() {
            Some(t) => {
                let w = t.model.current_word_at_cursor();
                (
                    t.lines.clone(),
                    PathBuf::from(&t.path),
                    t.cursor_row,
                    t.cursor_col,
                    w,
                )
            }
            None => return,
        };

        if word.is_empty() {
            self.status_message = "No symbol at cursor for references".to_string();
            return;
        }

        let refs = self
            .vsx
            .language_services
            .find_references(&lines, &path, row, col);
        if !refs.is_empty() {
            let count = refs.len();
            self.references.open(word.clone(), refs, row, col);
            self.status_message = format!("Found {} reference(s) for '{}'", count, word);
        } else {
            self.status_message = format!("No references found for '{}'", word);
        }
    }

    pub fn toggle_folder(&mut self, path: &str) {
        FileNode::toggle_in_nodes(&mut self.file_tree, path);
    }

    pub fn active_tab(&self) -> Option<&OpenTab> {
        if self.is_split_editor && self.active_pane == 1 {
            if let Some(i) = self.active_secondary_tab_idx {
                return self.secondary_tabs.get(i);
            }
        }
        self.active_tab_idx.and_then(|i| self.tabs.get(i))
    }

    pub fn active_tab_mut(&mut self) -> Option<&mut OpenTab> {
        if self.is_split_editor && self.active_pane == 1 {
            if let Some(i) = self.active_secondary_tab_idx {
                return self.secondary_tabs.get_mut(i);
            }
        }
        self.active_tab_idx.and_then(|i| self.tabs.get_mut(i))
    }

    pub fn split_editor(&mut self) {
        if !self.is_split_editor {
            self.is_split_editor = true;
            self.active_pane = 1;
            if let Some(cur_tab) = self.tabs.get(self.active_tab_idx.unwrap_or(0)).cloned() {
                self.secondary_tabs = vec![cur_tab];
                self.active_secondary_tab_idx = Some(0);
            }
        } else {
            self.split_horizontal = !self.split_horizontal;
        }
        self.sync_lsp_for_active_tab();
        self.save_session();
    }

    pub fn close_split_editor(&mut self) {
        self.is_split_editor = false;
        self.active_pane = 0;
        self.secondary_tabs.clear();
        self.active_secondary_tab_idx = None;
        self.sync_lsp_for_active_tab();
        self.save_session();
    }

    pub fn focus_pane(&mut self, pane: usize) {
        self.active_pane = pane;
        self.sync_lsp_for_active_tab();
    }

    pub fn select_secondary_tab(&mut self, idx: usize) {
        if idx < self.secondary_tabs.len() {
            self.active_secondary_tab_idx = Some(idx);
            self.active_pane = 1;
            self.sync_lsp_for_active_tab();
            self.save_session();
        }
    }

    pub fn close_secondary_tab(&mut self, idx: usize) {
        if idx < self.secondary_tabs.len() {
            self.secondary_tabs.remove(idx);
            if self.secondary_tabs.is_empty() {
                self.active_secondary_tab_idx = None;
                self.is_split_editor = false;
                self.active_pane = 0;
            } else if let Some(cur) = self.active_secondary_tab_idx {
                if cur >= self.secondary_tabs.len() {
                    self.active_secondary_tab_idx = Some(self.secondary_tabs.len() - 1);
                }
            }
            self.sync_lsp_for_active_tab();
            self.save_session();
        }
    }

    pub fn toggle_sticky_scroll(&mut self) {
        self.sticky_scroll.enabled = !self.sticky_scroll.enabled;
        self.save_preferences();
    }

    pub fn toggle_minimap(&mut self) -> bool {
        let res = self.minimap.toggle();
        self.save_preferences();
        res
    }

    pub fn toggle_inlay_hints(&mut self) -> bool {
        self.inlay_hints_enabled = !self.inlay_hints_enabled;
        self.save_preferences();
        self.inlay_hints_enabled
    }

    pub fn set_tab_size(&mut self, size: usize) {
        self.default_tab_size = size;
        for tab in &mut self.tabs {
            tab.model.tab_size = size;
        }
        for tab in &mut self.secondary_tabs {
            tab.model.tab_size = size;
        }
        self.save_preferences();
    }

    pub fn toggle_word_wrap(&mut self) -> bool {
        let mut new_state = true;
        if let Some(tab) = self.active_tab_mut() {
            new_state = tab.model.toggle_word_wrap();
        }
        for tab in &mut self.tabs {
            tab.model.wrap.enabled = new_state;
            if new_state {
                tab.model.wrap.wrap_width = 100;
            }
            tab.model.wrap.recompute(tab.model.buffer.lines());
        }
        for tab in &mut self.secondary_tabs {
            tab.model.wrap.enabled = new_state;
            if new_state {
                tab.model.wrap.wrap_width = 100;
            }
            tab.model.wrap.recompute(tab.model.buffer.lines());
        }
        self.save_preferences();
        new_state
    }

    pub fn save_preferences(&self) {
        if let Some(home) = dirs::home_dir() {
            let dir = home.join(".vscodium-rust");
            let _ = std::fs::create_dir_all(&dir);
            let prefs = EditorPreferences {
                tab_size: self.default_tab_size,
                word_wrap: self
                    .active_tab()
                    .map(|t| t.model.wrap.enabled)
                    .unwrap_or(true),
                minimap: self.minimap.enabled,
                bracket_pairs: self.bracket_pairs.colorization_enabled,
                indent_guides: self.bracket_pairs.indent_guides.enabled,
                sticky_scroll: self.sticky_scroll.enabled,
                format_on_save: self.format_on_save,
                inlay_hints: self.inlay_hints_enabled,
                theme: self.active_theme_name.clone(),
                font_size: self.editor_font_size,
                line_numbers: self.line_numbers_enabled,
                telemetry: self.telemetry_enabled,
                default_shell: self.default_shell.clone(),
                auto_approve_terminal: self.agent_auto_approve_terminal,
                auto_approve_file_write: self.agent_auto_approve_file_write,
                reasoning_effort: self.agent_reasoning_effort.clone(),
            };
            if let Ok(json) = serde_json::to_string_pretty(&prefs) {
                let _ = std::fs::write(dir.join("settings.json"), json);
            }
        }
    }

    pub fn set_theme(&mut self, theme_name: &str) {
        self.active_theme_name = theme_name.to_string();
        self.theme = match theme_name {
            "Tokyo Night" => crate::theme::Theme::tokyo_night(),
            "Obsidian" => crate::theme::Theme::obsidian(),
            _ => crate::theme::Theme::cursor_dark(),
        };
        self.save_preferences();
    }

    pub fn load_preferences(&mut self) {
        if let Some(home) = dirs::home_dir() {
            let file = home.join(".vscodium-rust").join("settings.json");
            if let Ok(content) = std::fs::read_to_string(file) {
                if let Ok(prefs) = serde_json::from_str::<EditorPreferences>(&content) {
                    self.default_tab_size = prefs.tab_size;
                    self.minimap.enabled = prefs.minimap;
                    self.bracket_pairs.colorization_enabled = prefs.bracket_pairs;
                    self.bracket_pairs.indent_guides.enabled = prefs.indent_guides;
                    self.sticky_scroll.enabled = prefs.sticky_scroll;
                    self.format_on_save = prefs.format_on_save;
                    self.inlay_hints_enabled = prefs.inlay_hints;
                    self.active_theme_name = prefs.theme.clone();
                    self.theme = match self.active_theme_name.as_str() {
                        "Tokyo Night" => crate::theme::Theme::tokyo_night(),
                        "Obsidian" => crate::theme::Theme::obsidian(),
                        _ => crate::theme::Theme::cursor_dark(),
                    };
                    self.editor_font_size = prefs.font_size;
                    self.line_numbers_enabled = prefs.line_numbers;
                    self.telemetry_enabled = prefs.telemetry;
                    self.default_shell = prefs.default_shell.clone();
                    self.agent_auto_approve_terminal = prefs.auto_approve_terminal;
                    self.agent_auto_approve_file_write = prefs.auto_approve_file_write;
                    self.agent_reasoning_effort = prefs.reasoning_effort.clone();
                    for tab in &mut self.tabs {
                        tab.model.tab_size = prefs.tab_size;
                        tab.model.wrap.enabled = prefs.word_wrap;
                        if prefs.word_wrap {
                            tab.model.wrap.wrap_width = 100;
                        }
                        tab.model.wrap.recompute(tab.model.buffer.lines());
                    }
                    for tab in &mut self.secondary_tabs {
                        tab.model.tab_size = prefs.tab_size;
                        tab.model.wrap.enabled = prefs.word_wrap;
                        if prefs.word_wrap {
                            tab.model.wrap.wrap_width = 100;
                        }
                        tab.model.wrap.recompute(tab.model.buffer.lines());
                    }
                }
            }
        }
    }

    pub fn save_session(&self) {
        if let Some(home) = dirs::home_dir() {
            let dir = home.join(".vscodium-rust");
            let _ = std::fs::create_dir_all(&dir);
            self.save_session_to(&dir.join("session.json"));
        }
    }

    pub fn save_session_to(&self, file: &Path) {
        if let Some(parent) = file.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let session = WorkspaceSession {
            open_tabs: self
                .tabs
                .iter()
                .map(|t| OpenTabSession {
                    path: t.path.clone(),
                    cursor_row: t.cursor_row,
                    cursor_col: t.cursor_col,
                    scroll_row: t.scroll_row,
                    is_pinned: t.is_pinned,
                })
                .collect(),
            active_tab_idx: self.active_tab_idx,
            workspace_root: if self.has_workspace && self.workspace_root.is_dir() {
                Some(self.workspace_root.clone())
            } else {
                None
            },
            secondary_open_tabs: self
                .secondary_tabs
                .iter()
                .map(|t| OpenTabSession {
                    path: t.path.clone(),
                    cursor_row: t.cursor_row,
                    cursor_col: t.cursor_col,
                    scroll_row: t.scroll_row,
                    is_pinned: t.is_pinned,
                })
                .collect(),
            secondary_active_tab_idx: self.active_secondary_tab_idx,
            is_split_editor: self.is_split_editor,
            split_horizontal: self.split_horizontal,
            left_sidebar_open: self.left_sidebar_open,
            right_sidebar_open: self.right_sidebar_open,
            iphone_preview_open: self.iphone_preview_open,
            bottom_panel_open: self.bottom_panel_open,
            sidebar_width: self.sidebar_width,
            right_sidebar_width: self.right_sidebar_width,
            iphone_preview_width: self.iphone_preview_width,
            bottom_panel_height: self.bottom_panel_height,
            active_activity: self.active_activity,
            active_right_tab: self.active_right_tab,
            active_bottom_tab: self.active_bottom_tab,
        };
        if let Ok(json) = serde_json::to_string_pretty(&session) {
            let _ = std::fs::write(file, json);
        }
    }

    pub fn load_session(&mut self) -> bool {
        if let Some(home) = dirs::home_dir() {
            let file = home.join(".vscodium-rust").join("session.json");
            return self.load_session_from(&file);
        }
        false
    }

    pub fn load_session_from(&mut self, file: &Path) -> bool {
        if let Ok(content) = std::fs::read_to_string(file) {
            if let Ok(session) = serde_json::from_str::<WorkspaceSession>(&content) {
                if let Some(ref ws) = session.workspace_root {
                    if ws.is_dir() {
                        self.workspace_root = ws.clone();
                        self.has_workspace = true;
                        self.refresh_file_tree();
                    }
                }
                let mut restored_tabs = Vec::new();
                for tab_sess in session.open_tabs {
                    let p = Path::new(&tab_sess.path);
                    let resolved = if p.is_absolute() {
                        p.to_path_buf()
                    } else if self.has_workspace {
                        self.workspace_root.join(p)
                    } else {
                        p.to_path_buf()
                    };
                    if resolved.exists() {
                        if let Ok(content) = std::fs::read_to_string(&resolved) {
                            let title = resolved
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let mut tab = OpenTab::new(
                                resolved.to_string_lossy().to_string(),
                                title,
                                content,
                            );
                            tab.model.tab_size = self.default_tab_size;
                            tab.cursor_row = tab_sess.cursor_row;
                            tab.cursor_col = tab_sess.cursor_col;
                            tab.scroll_row = tab_sess.scroll_row;
                            tab.is_pinned = tab_sess.is_pinned;
                            tab.model
                                .set_cursor(tab_sess.cursor_row, tab_sess.cursor_col);
                            restored_tabs.push(tab);
                        }
                    }
                }

                let mut restored_sec = Vec::new();
                for tab_sess in session.secondary_open_tabs {
                    let p = Path::new(&tab_sess.path);
                    let resolved = if p.is_absolute() {
                        p.to_path_buf()
                    } else {
                        self.workspace_root.join(p)
                    };
                    if resolved.exists() {
                        if let Ok(content) = std::fs::read_to_string(&resolved) {
                            let title = resolved
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string();
                            let mut tab = OpenTab::new(
                                resolved.to_string_lossy().to_string(),
                                title,
                                content,
                            );
                            tab.model.tab_size = self.default_tab_size;
                            tab.cursor_row = tab_sess.cursor_row;
                            tab.cursor_col = tab_sess.cursor_col;
                            tab.scroll_row = tab_sess.scroll_row;
                            tab.is_pinned = tab_sess.is_pinned;
                            tab.model
                                .set_cursor(tab_sess.cursor_row, tab_sess.cursor_col);
                            restored_sec.push(tab);
                        }
                    }
                }

                if !restored_tabs.is_empty() {
                    self.tabs = restored_tabs;
                    if let Some(idx) = session.active_tab_idx {
                        if idx < self.tabs.len() {
                            self.active_tab_idx = Some(idx);
                        } else {
                            self.active_tab_idx = Some(0);
                        }
                    } else {
                        self.active_tab_idx = Some(0);
                    }

                    if !restored_sec.is_empty() {
                        self.secondary_tabs = restored_sec;
                        self.is_split_editor = session.is_split_editor;
                        self.split_horizontal = session.split_horizontal;
                        if let Some(idx) = session.secondary_active_tab_idx {
                            if idx < self.secondary_tabs.len() {
                                self.active_secondary_tab_idx = Some(idx);
                            } else {
                                self.active_secondary_tab_idx = Some(0);
                            }
                        } else {
                            self.active_secondary_tab_idx = Some(0);
                        }
                    }
                }

                self.left_sidebar_open = session.left_sidebar_open;
                self.right_sidebar_open = session.right_sidebar_open;
                self.iphone_preview_open = session.iphone_preview_open;
                self.bottom_panel_open = session.bottom_panel_open;
                self.sidebar_width = session.sidebar_width;
                self.right_sidebar_width = session.right_sidebar_width.max(360.0);
                self.iphone_preview_width = session.iphone_preview_width.max(320.0);
                self.active_activity = if session.active_activity == ActivityTab::Settings {
                    ActivityTab::Explorer
                } else {
                    session.active_activity
                };
                self.active_right_tab = session.active_right_tab;
                self.active_bottom_tab = session.active_bottom_tab;

                self.sync_lsp_for_active_tab();
                return true;
            }
        }
        false
    }

    pub fn pin_tab(&mut self, idx: usize) {
        if let Some(tab) = self.tabs.get_mut(idx) {
            tab.is_pinned = true;
            self.save_session();
        }
    }

    pub fn unpin_tab(&mut self, idx: usize) {
        if let Some(tab) = self.tabs.get_mut(idx) {
            tab.is_pinned = false;
            self.save_session();
        }
    }

    pub fn toggle_pin_tab(&mut self, idx: usize) {
        if let Some(tab) = self.tabs.get_mut(idx) {
            tab.is_pinned = !tab.is_pinned;
            self.save_session();
        }
    }

    pub fn toggle_active_markdown_reader_mode(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            tab.markdown_reader_mode = !tab.markdown_reader_mode;
        }
    }

    pub fn add_cursor_above(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            tab.add_cursor_above();
            let count = tab.model.cursors.all_selections().len();
            self.status_message = format!("Multi-Cursor: {} selections", count);
        }
    }

    pub fn add_cursor_below(&mut self) {
        if let Some(tab) = self.active_tab_mut() {
            tab.add_cursor_below();
            let count = tab.model.cursors.all_selections().len();
            self.status_message = format!("Multi-Cursor: {} selections", count);
        }
    }

    pub fn add_next_occurrence(&mut self) -> bool {
        if let Some(tab) = self.active_tab_mut() {
            let res = tab.add_next_occurrence();
            if res {
                let count = tab.model.cursors.all_selections().len();
                self.status_message = format!("Multi-Cursor: {} selections", count);
            }
            res
        } else {
            false
        }
    }

    pub fn select_all_occurrences(&mut self) -> usize {
        if let Some(tab) = self.active_tab_mut() {
            let count = tab.select_all_occurrences();
            if count > 0 {
                self.status_message = format!("Selected all {} occurrences", count);
            }
            count
        } else {
            0
        }
    }

    pub fn cursor_undo(&mut self) -> bool {
        if let Some(tab) = self.active_tab_mut() {
            let res = tab.cursor_undo();
            if res {
                let count = tab.model.cursors.all_selections().len();
                self.status_message = format!("Multi-Cursor: {} selections", count);
            }
            res
        } else {
            false
        }
    }

    pub fn toggle_word_highlight(&mut self) -> bool {
        let res = self.multi_cursor.toggle_highlights();
        self.status_message = format!(
            "Word Occurrence Highlights: {}",
            if res { "On" } else { "Off" }
        );
        res
    }

    pub fn execute_terminal_command(&mut self, cmd: &str) {
        let trimmed = cmd.trim();
        if trimmed.is_empty() {
            return;
        }

        if trimmed == "clear" || trimmed == "cls" {
            self.clear_terminal();
            return;
        }

        if self.active_pty().is_none() {
            self.ensure_pty();
        }

        if let Some(pty) = self.active_pty() {
            let _ = pty.send_line(trimmed);
        }

        self.active_bottom_tab = crate::domain::layout::BottomPanelTab::Terminal;
        self.bottom_panel_open = true;
        self.focused_panel = crate::domain::layout::FocusedPanel::Terminal;
        self.terminal_input.clear();
    }

    pub fn write_terminal_char(&mut self, ch: char) {
        self.terminal_input.push(ch);
    }

    pub fn delete_terminal_char(&mut self) {
        self.terminal_input.pop();
    }

    pub fn submit_terminal_input(&mut self) {
        let input = std::mem::take(&mut self.terminal_input);
        self.execute_terminal_command(&input);
    }

    pub fn send_ai_prompt(&mut self, prompt: &str) {
        let trimmed = prompt.trim();
        if trimmed.is_empty() || self.is_agent_thinking {
            return;
        }

        let now = chrono::Local::now().format("%H:%M:%S").to_string();

        // 1. Add user message
        let mut user_msg = ChatMessage::user(trimmed.to_string());
        user_msg.timestamp = now.clone();
        self.chat_messages.push(user_msg);

        self.composer_input.clear();
        self.is_agent_thinking = true;
        self.thinking_start_instant = Some(std::time::Instant::now());

        // 2. Scan for @file mentions and resolve content
        let mut context_preamble = String::new();
        for word in trimmed.split_whitespace() {
            if let Some(rest) = word.strip_prefix('@') {
                let clean_path = rest.trim_matches(|c: char| {
                    !c.is_alphanumeric() && c != '.' && c != '/' && c != '\\' && c != '_'
                });
                let full_path = self.workspace_root.join(clean_path);
                if full_path.exists() && full_path.is_file() {
                    if let Ok(c) = std::fs::read_to_string(&full_path) {
                        let snippet = if c.len() > 4000 {
                            format!("{}...\n[truncated]", &c[..4000])
                        } else {
                            c
                        };
                        context_preamble.push_str(&format!(
                            "\n\n[Referenced File: {}]\n```\n{}\n```\n",
                            clean_path, snippet
                        ));
                    }
                }
            }
        }

        // 3. Prepare conversation history for backend Sentient engine
        let mut engine_messages: Vec<vscode_rust_app::domain::ai::engine::types::ChatMessage> =
            Vec::new();
        for msg in &self.chat_messages {
            let role = if msg.is_user {
                "user".to_string()
            } else {
                "assistant".to_string()
            };
            let content = msg.text.clone();
            engine_messages.push(vscode_rust_app::domain::ai::engine::types::ChatMessage {
                role,
                content: Some(
                    vscode_rust_app::domain::ai::engine::types::MessageContent::Text(content),
                ),
                tool_calls: None,
                tool_call_id: None,
                metadata: None,
            });
        }

        if !context_preamble.is_empty() {
            if let Some(last) = engine_messages.last_mut() {
                if let Some(vscode_rust_app::domain::ai::engine::types::MessageContent::Text(
                    ref mut t,
                )) = last.content
                {
                    t.push_str(&context_preamble);
                }
            }
        }

        let engine = self.ensure_services().state.ai.engine.clone();
        let display_model = self.agent_model.clone();
        let model = Self::resolve_ai_model_id(&display_model);
        let mode = self.agent_mode.clone();
        let reasoning_enabled = self.reasoning_active;

        let (provider, inference_url) = Self::resolve_ai_provider_and_url(&display_model);
        let is_chat = mode == "Chat" || mode == "Ask";

        let req = vscode_rust_app::domain::ai::engine::types::AiRequest {
            provider,
            model,
            messages: engine_messages,
            temperature: Some(if is_chat { 0.7 } else { 0.2 }),
            autonomous: !is_chat,
            mode: Some(mode),
            cyber_mode: None,
            root_access: None,
            inference_url,
            tools: None,
            reasoning_budget: Some(4096),
            reasoning_effort: Some("medium".to_string()),
            reasoning_enabled: Some(reasoning_enabled),
            feature: Some("Chat".to_string()),
        };

        // 4. Initialize AI turn state
        let turn = self.ai_turn.clone();
        {
            if let Ok(mut t) = turn.lock() {
                t.reset();
            }
        }

        let streamed_content = turn.lock().unwrap().streamed_content.clone();
        let streamed_thoughts = turn.lock().unwrap().streamed_thoughts.clone();
        let tool_calls = turn.lock().unwrap().tool_calls.clone();
        let live_tokens_count = turn.lock().unwrap().live_tokens_count.clone();
        let live_tokens_per_sec = turn.lock().unwrap().live_tokens_per_sec.clone();
        let live_elapsed_secs = turn.lock().unwrap().live_elapsed_secs.clone();
        let live_mtp_rate = turn.lock().unwrap().live_mtp_rate.clone();
        let final_metrics = turn.lock().unwrap().final_metrics.clone();

        let in_thinking = Arc::new(std::sync::atomic::AtomicBool::new(false));

        let on_chunk = Some(Arc::new(move |chunk: &str| {
            if chunk.is_empty() {
                return;
            }

            // 1. Intercept [METRICS:{...}]
            if let Some(start_pos) = chunk.find("[METRICS:") {
                if let Some(end_pos) = chunk[start_pos..].find(']') {
                    let json_str = &chunk[start_pos + 9..start_pos + end_pos];
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let pred_tps = v.get("predicted_per_second").and_then(|x| x.as_f64());
                        let pred_n = v.get("predicted_n").and_then(|x| x.as_u64()).map(|n| n as usize);
                        let pred_ms = v.get("predicted_ms").and_then(|x| x.as_f64());
                        let prompt_tps = v.get("prompt_per_second").and_then(|x| x.as_f64());
                        let prompt_ms = v.get("prompt_ms").and_then(|x| x.as_f64());
                        let cache_n = v.get("cache_n").and_then(|x| x.as_u64()).map(|n| n as usize);
                        let draft_n = v.get("draft_n").and_then(|x| x.as_u64()).map(|n| n as usize);
                        let draft_accepted = v.get("draft_n_accepted").and_then(|x| x.as_u64()).map(|n| n as usize);
                        let mtp_rate = if let (Some(acc), Some(d)) = (draft_accepted, draft_n) {
                            if d > 0 { Some((acc as f64 / d as f64) * 100.0) } else { None }
                        } else { None };

                        if let Some(tps) = pred_tps {
                            if let Ok(mut s) = live_tokens_per_sec.lock() { *s = tps; }
                        }
                        if let Some(rate) = mtp_rate {
                            if let Ok(mut m) = live_mtp_rate.lock() { *m = Some(rate); }
                        }
                        if let Some(n) = pred_n {
                            if let Ok(mut c) = live_tokens_count.lock() { *c = n; }
                        }

                        let metrics = TokenMetrics {
                            predicted_per_second: pred_tps,
                            predicted_tokens: pred_n,
                            predicted_ms: pred_ms,
                            prompt_per_second: prompt_tps,
                            prompt_ms: prompt_ms,
                            cache_n,
                            draft_tokens: draft_n,
                            draft_accepted,
                            mtp_acceptance_rate: mtp_rate,
                        };
                        if let Ok(mut fm) = final_metrics.lock() {
                            *fm = Some(metrics);
                        }
                    }
                }
                return;
            }

            // 2. Intercept [STREAM_STATS:{...}]
            if let Some(start_pos) = chunk.find("[STREAM_STATS:") {
                if let Some(end_pos) = chunk[start_pos..].find(']') {
                    let json_str = &chunk[start_pos + 14..start_pos + end_pos];
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
                        if let Some(cnt) = v.get("tokens").and_then(|x| x.as_u64()) {
                            if let Ok(mut c) = live_tokens_count.lock() { *c = cnt as usize; }
                        }
                        if let Some(tps) = v.get("tps").and_then(|x| x.as_f64()) {
                            if let Ok(mut s) = live_tokens_per_sec.lock() { *s = tps; }
                        }
                        if let Some(el) = v.get("elapsed").and_then(|x| x.as_f64()) {
                            if let Ok(mut e) = live_elapsed_secs.lock() { *e = el; }
                        }
                    }
                }
                return;
            }

            // 3. Intercept [TOOL_START:{...}]
            if let Some(start_pos) = chunk.find("[TOOL_START:") {
                if let Some(end_pos) = chunk[start_pos..].find(']') {
                    let json_str = &chunk[start_pos + 12..start_pos + end_pos];
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("").to_string();
                        let args = v.get("args").and_then(|x| x.as_str()).unwrap_or("").to_string();
                        if !name.is_empty() {
                            if let Ok(mut tc) = tool_calls.lock() {
                                tc.push(ChatToolCall {
                                    name,
                                    args,
                                    output: None,
                                    completed: false,
                                    duration_ms: None,
                                });
                            }
                        }
                    }
                }
                return;
            }

            // 4. Intercept [TOOL_END:{...}]
            if let Some(start_pos) = chunk.find("[TOOL_END:") {
                if let Some(end_pos) = chunk[start_pos..].find(']') {
                    let json_str = &chunk[start_pos + 10..start_pos + end_pos];
                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(json_str) {
                        let name = v.get("name").and_then(|x| x.as_str()).unwrap_or("");
                        if let Ok(mut tc) = tool_calls.lock() {
                            if let Some(tool) = tc.iter_mut().rev().find(|t| t.name == name && !t.completed) {
                                tool.completed = true;
                            }
                        }
                    }
                }
                return;
            }

            // 5. Stateful thinking tags & reasoning extraction
            let mut remaining = chunk;
            while !remaining.is_empty() {
                if in_thinking.load(std::sync::atomic::Ordering::SeqCst) {
                    if let Some(end_pos) = remaining.find("</think>") {
                        let thought_part = &remaining[..end_pos];
                        if !thought_part.is_empty() {
                            if let Ok(mut st) = streamed_thoughts.lock() {
                                st.push_str(thought_part);
                            }
                        }
                        in_thinking.store(false, std::sync::atomic::Ordering::SeqCst);
                        remaining = &remaining[end_pos + 8..];
                    } else {
                        if let Ok(mut st) = streamed_thoughts.lock() {
                            st.push_str(remaining);
                        }
                        break;
                    }
                } else {
                    if let Some(start_pos) = remaining.find("<think>") {
                        let content_part = &remaining[..start_pos];
                        if !content_part.is_empty() {
                            if let Ok(mut sc) = streamed_content.lock() {
                                sc.push_str(content_part);
                            }
                        }
                        in_thinking.store(true, std::sync::atomic::Ordering::SeqCst);
                        remaining = &remaining[start_pos + 7..];
                    } else {
                        if let Ok(mut sc) = streamed_content.lock() {
                            sc.push_str(remaining);
                        }
                        break;
                    }
                }
            }
        }) as Arc<dyn Fn(&str) + Send + Sync>);

        // 5. Spawn autonomous loop execution on background thread with Tokio runtime
        let turn_clone = turn.clone();
        std::thread::Builder::new()
            .name("ai-autonomous-loop".to_string())
            .spawn(move || {
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build();

                let now = chrono::Local::now().format("%H:%M:%S").to_string();

                if let Ok(rt) = rt {
                    let result = rt.block_on(async { engine.autonomous_loop(req, on_chunk).await });

                    if let Ok(mut t) = turn_clone.lock() {
                        t.is_running = false;
                        let thoughts = t
                            .streamed_thoughts
                            .lock()
                            .map(|s| s.clone())
                            .unwrap_or_default();
                        let tool_calls =
                            t.tool_calls.lock().map(|tc| tc.clone()).unwrap_or_default();
                        let token_metrics =
                            t.final_metrics.lock().ok().and_then(|m| m.clone());

                        let final_msg = match result {
                            Ok(content) => {
                                let stream_txt = t
                                    .streamed_content
                                    .lock()
                                    .map(|s| s.clone())
                                    .unwrap_or_default();
                                let text = if content.trim().is_empty() {
                                    stream_txt
                                } else {
                                    content
                                };
                                let mut msg = ChatMessage::assistant(text);
                                msg.thoughts = if thoughts.trim().is_empty() {
                                    None
                                } else {
                                    Some(thoughts)
                                };
                                msg.tool_calls = tool_calls;
                                msg.token_metrics = token_metrics;
                                msg.timestamp = now;
                                msg
                            }
                            Err(e) => {
                                let err_str = e.to_string();
                                let advice = if err_str.contains("refused")
                                    || err_str.contains("connect")
                                    || err_str.contains("error sending request")
                                    || err_str.contains("timeout")
                                {
                                    "\nTip: Local models are prioritized. Verify Lemonade daemon on 127.0.0.1:13305 or llama-server on :8001 and allow GPU VRAM loading."
                                } else {
                                    ""
                                };
                                let mut msg =
                                    ChatMessage::assistant(format!("[Local AI notice]: {err_str}{advice}"));
                                msg.timestamp = now;
                                msg
                            }
                        };

                        if let Ok(mut fin) = t.finished_turn.lock() {
                            *fin = Some(final_msg);
                        }
                    }
                }
            })
            .expect("Failed to spawn ai agent loop thread");
    }

    pub fn api_keys_file_path() -> PathBuf {
        let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
        PathBuf::from(appdata).join("vscodium-rust").join("api_keys.json")
    }

    pub fn read_initial_api_key(json_key: &str, env_var: &str) -> String {
        if let Ok(val) = std::env::var(env_var) {
            let trimmed = val.trim();
            if !trimmed.is_empty() {
                return trimmed.to_string();
            }
        }
        if env_var == "GEMINI_API_KEY" {
            if let Ok(val) = std::env::var("GOOGLE_API_KEY") {
                let trimmed = val.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
        if env_var == "MODELSCOPE_API_KEY" {
            if let Ok(val) = std::env::var("MODELSCOPE_TOKEN") {
                let trimmed = val.trim();
                if !trimmed.is_empty() {
                    return trimmed.to_string();
                }
            }
        }
        let p = Self::api_keys_file_path();
        if let Ok(content) = std::fs::read_to_string(&p) {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(k) = v.get(json_key).and_then(|s| s.as_str()) {
                    let trimmed = k.trim();
                    if !trimmed.is_empty() {
                        return trimmed.to_string();
                    }
                }
                if json_key == "google" {
                    if let Some(k) = v.get("gemini").and_then(|s| s.as_str()) {
                        let trimmed = k.trim();
                        if !trimmed.is_empty() {
                            return trimmed.to_string();
                        }
                    }
                }
                if json_key == "modelscope" {
                    if let Some(k) = v.get("modelscope_api_key").and_then(|s| s.as_str()) {
                        let trimmed = k.trim();
                        if !trimmed.is_empty() {
                            return trimmed.to_string();
                        }
                    }
                }
            }
        }
        String::new()
    }

    pub fn set_and_save_api_key(&mut self, provider: &str, key: String) {
        let trimmed = key.trim().to_string();
        let (env_var, json_key) = match provider.to_lowercase().as_str() {
            "anthropic" | "claude" => ("ANTHROPIC_API_KEY", "anthropic"),
            "openai" | "gpt" => ("OPENAI_API_KEY", "openai"),
            "google" | "gemini" => ("GEMINI_API_KEY", "google"),
            "deepseek" => ("DEEPSEEK_API_KEY", "deepseek"),
            "modelscope" | "qwen" => ("MODELSCOPE_API_KEY", "modelscope"),
            _ => ("OPENAI_API_KEY", "openai"),
        };

        match json_key {
            "anthropic" => self.anthropic_api_key = trimmed.clone(),
            "openai" => self.openai_api_key = trimmed.clone(),
            "google" => self.gemini_api_key = trimmed.clone(),
            "deepseek" => self.deepseek_api_key = trimmed.clone(),
            "modelscope" => self.modelscope_api_key = trimmed.clone(),
            _ => {}
        }

        std::env::set_var(env_var, &trimmed);
        if json_key == "google" {
            std::env::set_var("GOOGLE_API_KEY", &trimmed);
        }
        if json_key == "modelscope" {
            std::env::set_var("MODELSCOPE_TOKEN", &trimmed);
        }

        let p = Self::api_keys_file_path();
        let mut json_obj = if let Ok(content) = std::fs::read_to_string(&p) {
            serde_json::from_str::<serde_json::Value>(&content).unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        if let Some(map) = json_obj.as_object_mut() {
            map.insert(json_key.to_string(), serde_json::Value::String(trimmed));
            if json_key == "google" {
                map.insert("gemini".to_string(), serde_json::Value::String(self.gemini_api_key.clone()));
            }
            if json_key == "modelscope" {
                map.insert("modelscope_api_key".to_string(), serde_json::Value::String(self.modelscope_api_key.clone()));
            }
        }

        if let Some(parent) = p.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(formatted) = serde_json::to_string_pretty(&json_obj) {
            let _ = std::fs::write(&p, formatted);
        }
    }

    pub fn remove_and_save_api_key(&mut self, provider: &str) {
        let (env_var, json_key) = match provider.to_lowercase().as_str() {
            "anthropic" | "claude" => ("ANTHROPIC_API_KEY", "anthropic"),
            "openai" | "gpt" => ("OPENAI_API_KEY", "openai"),
            "google" | "gemini" => ("GEMINI_API_KEY", "google"),
            "deepseek" => ("DEEPSEEK_API_KEY", "deepseek"),
            "modelscope" | "qwen" => ("MODELSCOPE_API_KEY", "modelscope"),
            _ => ("OPENAI_API_KEY", "openai"),
        };

        match json_key {
            "anthropic" => self.anthropic_api_key.clear(),
            "openai" => self.openai_api_key.clear(),
            "google" => self.gemini_api_key.clear(),
            "deepseek" => self.deepseek_api_key.clear(),
            "modelscope" => self.modelscope_api_key.clear(),
            _ => {}
        }

        std::env::remove_var(env_var);
        if json_key == "google" {
            std::env::remove_var("GOOGLE_API_KEY");
        }
        if json_key == "modelscope" {
            std::env::remove_var("MODELSCOPE_TOKEN");
        }

        let p = Self::api_keys_file_path();
        if let Ok(content) = std::fs::read_to_string(&p) {
            if let Ok(mut json_obj) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(map) = json_obj.as_object_mut() {
                    map.remove(json_key);
                    if json_key == "google" {
                        map.remove("gemini");
                    }
                    if json_key == "modelscope" {
                        map.remove("modelscope_api_key");
                    }
                }
                if let Ok(formatted) = serde_json::to_string_pretty(&json_obj) {
                    let _ = std::fs::write(&p, formatted);
                }
            }
        }
    }

    pub fn resolve_ai_model_id(display_name: &str) -> String {
        let m = display_name.trim();
        if m.eq_ignore_ascii_case("claude 3.5 sonnet") || m.eq_ignore_ascii_case("claude-3.5-sonnet") {
            return "claude-3-5-sonnet-20241022".to_string();
        }
        if m.eq_ignore_ascii_case("claude 3.7 sonnet") || m.eq_ignore_ascii_case("claude-3.7-sonnet") {
            return "claude-3-7-sonnet-20250219".to_string();
        }
        if m.eq_ignore_ascii_case("gemini 1.5 pro") || m.eq_ignore_ascii_case("gemini-1.5-pro") {
            return "gemini-1.5-pro".to_string();
        }
        if m.eq_ignore_ascii_case("gemini 2.0 flash") || m.eq_ignore_ascii_case("gemini-2.0-flash") {
            return "gemini-2.0-flash".to_string();
        }
        if m.eq_ignore_ascii_case("gpt-4o") {
            return "gpt-4o".to_string();
        }
        if m.eq_ignore_ascii_case("gpt-4o-mini") {
            return "gpt-4o-mini".to_string();
        }
        if m.eq_ignore_ascii_case("deepseek v3") || m.eq_ignore_ascii_case("deepseek-v3") {
            return "deepseek-chat".to_string();
        }
        if m.eq_ignore_ascii_case("deepseek r1") || m.eq_ignore_ascii_case("deepseek-r1") {
            return "deepseek-reasoner".to_string();
        }
        if m.eq_ignore_ascii_case("qwen 2.5 coder (modelscope)")
            || m.eq_ignore_ascii_case("qwen-2.5-coder-32b")
            || m.eq_ignore_ascii_case("qwen2.5-coder-32b")
            || m.eq_ignore_ascii_case("qwen 2.5 coder 32b")
        {
            return "Qwen/Qwen2.5-Coder-32B-Instruct".to_string();
        }
        if m.eq_ignore_ascii_case("qwen max (modelscope)")
            || m.eq_ignore_ascii_case("qwen max")
            || m.eq_ignore_ascii_case("qwen-max")
        {
            return "qwen-max".to_string();
        }
        if m.eq_ignore_ascii_case("qwen plus (modelscope)")
            || m.eq_ignore_ascii_case("qwen plus")
            || m.eq_ignore_ascii_case("qwen-plus")
        {
            return "qwen-plus".to_string();
        }
        if m.eq_ignore_ascii_case("qwen turbo (modelscope)")
            || m.eq_ignore_ascii_case("qwen turbo")
            || m.eq_ignore_ascii_case("qwen-turbo")
        {
            return "qwen-turbo".to_string();
        }
        if m.eq_ignore_ascii_case("qwen 3.8 27b")
            || m.eq_ignore_ascii_case("qwen 3.8 · lemonade")
            || m.eq_ignore_ascii_case("qwen 3.8")
            || m.eq_ignore_ascii_case("qwen3.8")
            || m.eq_ignore_ascii_case("qwen-3.8-27b")
            || m.eq_ignore_ascii_case("qwen 3.8 27b cyber agentic")
            || m.eq_ignore_ascii_case("qwen 3.8 cyber")
            || m.eq_ignore_ascii_case("qwen 3.8 uncensored")
        {
            return "Qwen3.8-27B-Uncensored-Cyber-IQ4_XS-imatrix-fromq8.gguf".to_string();
        }
        if m.eq_ignore_ascii_case("qwen 3.5 4b")
            || m.eq_ignore_ascii_case("qwen 3.5 · lemonade")
            || m.eq_ignore_ascii_case("qwen 3.5")
            || m.eq_ignore_ascii_case("qwen3.5")
            || m.eq_ignore_ascii_case("qwen-3.5-4b")
        {
            return "Qwen3.5-4B-GGUF-Q5_K_M".to_string();
        }
        if m.eq_ignore_ascii_case("qwen 3.8 35b")
            || m.eq_ignore_ascii_case("qwen 3.8 35b a3b")
            || m.eq_ignore_ascii_case("qwen 3.8 35b · lemonade")
            || m.eq_ignore_ascii_case("qwen3.8-35b-a3b")
            || m.eq_ignore_ascii_case("qwen3.8-35b-a3b-distill")
            || m.eq_ignore_ascii_case("qwen 3.8 35b a3b distill")
            || m.eq_ignore_ascii_case("qwen3.8-35b")
        {
            return "Qwen3.8-35B-A3B-Q4_K_M.gguf".to_string();
        }
        if m.eq_ignore_ascii_case("escha 35b")
            || m.eq_ignore_ascii_case("escha 35b deep reasoning")
            || m.eq_ignore_ascii_case("escha 35b · lemonade")
            || m.eq_ignore_ascii_case("escha")
        {
            return "Escha-W2-35B-A3B-ROCmFP2-Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf".to_string();
        }
        if m.eq_ignore_ascii_case("deepseek v3 (modelscope)") {
            return "deepseek-ai/DeepSeek-V3".to_string();
        }
        if m.eq_ignore_ascii_case("deepseek r1 (modelscope)") {
            return "deepseek-ai/DeepSeek-R1".to_string();
        }
        m.to_string()
    }

    pub fn resolve_ai_provider_and_url(model: &str) -> (String, Option<String>) {
        let m = model.to_lowercase();
        if m.contains("gguf") || m.contains("qwen3.") || m.contains("escha") || m.contains("cyber") || m.contains("uncensored") || m.contains("lemonade") {
            return ("lemonade".to_string(), Some("http://127.0.0.1:13305".to_string()));
        }
        if m.contains("modelscope")
            || m == "qwen-max"
            || m == "qwen-plus"
            || m == "qwen-turbo"
            || (m.starts_with("qwen-") && !m.contains("gguf"))
        {
            return ("modelscope".to_string(), None);
        }
        if m.contains("claude") {
            return ("anthropic".to_string(), None);
        }
        if m.contains("gemini") {
            return ("google".to_string(), None);
        }
        if m.contains("gpt-") || m.contains("o1-") || m.contains("o3-") {
            return ("openai".to_string(), None);
        }
        if m.contains("deepseek") && !m.contains("local") {
            return ("deepseek".to_string(), None);
        }

        if let Ok(u) = std::env::var("LEMONADE_URL") {
            if !u.trim().is_empty() {
                return ("lemonade".to_string(), Some(u));
            }
        }

        ("lemonade".to_string(), Some("http://127.0.0.1:13305".to_string()))
    }

    pub fn toggle_find_bar(&mut self) {
        self.find_bar_open = !self.find_bar_open;
        if self.find_bar_open {
            self.update_find_matches();
        }
    }

    pub fn update_find_matches(&mut self) {
        if self.find_query.is_empty() {
            self.find_match_count = 0;
            return;
        }

        if let Some(tab) = self.active_tab() {
            let options = crate::editor::engine::find_replace::FindOptions {
                case_sensitive: self.find_case_sensitive,
                whole_word: self.find_whole_word,
                is_regex: self.find_regex,
            };
            let matches = crate::editor::engine::find_replace::FindReplaceEngine::find_all(
                &tab.lines,
                &self.find_query,
                &options,
            );
            self.find_match_count = matches.len();
        } else {
            self.find_match_count = 0;
        }
    }

    pub fn find_next(&mut self) {
        let (query, options, cursor_pos) = match self.active_tab() {
            Some(t) => (
                self.find_query.clone(),
                crate::editor::engine::find_replace::FindOptions {
                    case_sensitive: self.find_case_sensitive,
                    whole_word: self.find_whole_word,
                    is_regex: self.find_regex,
                },
                (t.cursor_row, t.cursor_col),
            ),
            None => return,
        };

        if let Some(tab) = self.active_tab_mut() {
            if let Some(m) = crate::editor::engine::find_replace::FindReplaceEngine::find_next(
                &tab.lines,
                &query,
                &options,
                cursor_pos.0,
                cursor_pos.1,
            ) {
                tab.cursor_row = m.row;
                tab.cursor_col = m.end_col;
                tab.model.set_cursor(m.row, m.end_col);
                if m.row < tab.scroll_row || m.row >= tab.scroll_row + 45 {
                    tab.scroll_row = m.row.saturating_sub(10);
                }
            }
        }
    }

    pub fn find_prev(&mut self) {
        let (query, options, cursor_pos) = match self.active_tab() {
            Some(t) => (
                self.find_query.clone(),
                crate::editor::engine::find_replace::FindOptions {
                    case_sensitive: self.find_case_sensitive,
                    whole_word: self.find_whole_word,
                    is_regex: self.find_regex,
                },
                (t.cursor_row, t.cursor_col),
            ),
            None => return,
        };

        if let Some(tab) = self.active_tab_mut() {
            if let Some(m) = crate::editor::engine::find_replace::FindReplaceEngine::find_prev(
                &tab.lines,
                &query,
                &options,
                cursor_pos.0,
                cursor_pos.1,
            ) {
                tab.cursor_row = m.row;
                tab.cursor_col = m.end_col;
                tab.model.set_cursor(m.row, m.end_col);
                if m.row < tab.scroll_row || m.row >= tab.scroll_row + 45 {
                    tab.scroll_row = m.row.saturating_sub(10);
                }
            }
        }
    }

    pub fn replace_next(&mut self) {
        let (query, repl, options, cursor_pos) = match self.active_tab() {
            Some(t) => (
                self.find_query.clone(),
                self.replace_query.clone(),
                crate::editor::engine::find_replace::FindOptions {
                    case_sensitive: self.find_case_sensitive,
                    whole_word: self.find_whole_word,
                    is_regex: self.find_regex,
                },
                (t.cursor_row, t.cursor_col),
            ),
            None => return,
        };

        if let Some(tab) = self.active_tab_mut() {
            if let Some(m) = crate::editor::engine::find_replace::FindReplaceEngine::find_next(
                &tab.lines,
                &query,
                &options,
                cursor_pos.0,
                cursor_pos.1.saturating_sub(1),
            ) {
                if crate::editor::engine::find_replace::FindReplaceEngine::replace_one(
                    &mut tab.lines,
                    &query,
                    &repl,
                    &options,
                    &m,
                ) {
                    tab.dirty = true;
                    tab.cursor_row = m.row;
                    tab.cursor_col = m.start_col + repl.len();
                    tab.model.set_cursor(tab.cursor_row, tab.cursor_col);
                    tab.model.recompute_inlays_and_lenses();
                }
            }
        }
        self.update_find_matches();
    }

    pub fn replace_all(&mut self) {
        let (query, repl, options) = (
            self.find_query.clone(),
            self.replace_query.clone(),
            crate::editor::engine::find_replace::FindOptions {
                case_sensitive: self.find_case_sensitive,
                whole_word: self.find_whole_word,
                is_regex: self.find_regex,
            },
        );

        if let Some(tab) = self.active_tab_mut() {
            let replaced_count =
                crate::editor::engine::find_replace::FindReplaceEngine::replace_all(
                    &mut tab.lines,
                    &query,
                    &repl,
                    &options,
                );
            if replaced_count > 0 {
                tab.dirty = true;
                tab.model.recompute_inlays_and_lenses();
                self.toast_manager
                    .push_success(&format!("Replaced {replaced_count} occurrences"));
            }
        }
        self.update_find_matches();
    }

    pub fn toggle_hook(&mut self, id: &str) {
        if let Some(hook) = self.agent_hooks.iter_mut().find(|h| h.id == id) {
            hook.enabled = !hook.enabled;
        }
    }

    pub fn add_hook(&mut self, pattern: &str, prompt: &str) {
        let count = self.agent_hooks.len() + 1;
        self.agent_hooks.push(AgentHook {
            id: format!("hook-{count}"),
            pattern: pattern.to_string(),
            prompt: prompt.to_string(),
            enabled: true,
        });
    }

    pub fn delete_hook(&mut self, id: &str) {
        self.agent_hooks.retain(|h| h.id != id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_tabs_lifecycle() {
        let mut state = HadesNativeState::new();
        let initial_tabs = state.tabs.len();

        // Create a temporary tab
        state.tabs.push(OpenTab::new(
            "foo.rs".into(),
            "foo.rs".into(),
            "hello world".into(),
        ));
        let new_idx = state.tabs.len() - 1;
        state.select_tab(new_idx);
        assert_eq!(state.active_tab_idx, Some(new_idx));
        assert_eq!(state.active_tab().unwrap().content(), "hello world");

        // Close tab
        state.close_tab(new_idx);
        assert_eq!(state.tabs.len(), initial_tabs);
    }

    #[test]
    fn test_state_find_bar() {
        let mut state = HadesNativeState::new();
        assert!(!state.find_bar_open);

        state.toggle_find_bar();
        assert!(state.find_bar_open);

        // Put a tab with text to find
        state.tabs.push(OpenTab::new(
            "test.rs".into(),
            "test.rs".into(),
            "fn foo() { foo(); }".into(),
        ));
        state.active_tab_idx = Some(state.tabs.len() - 1);

        state.find_query = "foo".into();
        state.update_find_matches();
        assert_eq!(state.find_match_count, 2);
    }

    #[test]
    fn test_state_hooks() {
        let mut state = HadesNativeState::new();
        let initial_len = state.agent_hooks.len();

        state.add_hook("*.rs", "lint rules");
        assert_eq!(state.agent_hooks.len(), initial_len + 1);

        let hook_id = state.agent_hooks.last().unwrap().id.clone();
        assert!(state.agent_hooks.last().unwrap().enabled);

        state.toggle_hook(&hook_id);
        assert!(
            !state
                .agent_hooks
                .iter()
                .find(|h| h.id == hook_id)
                .unwrap()
                .enabled
        );

        state.delete_hook(&hook_id);
        assert_eq!(state.agent_hooks.len(), initial_len);
    }

    #[test]
    fn test_state_activity_and_sidebars() {
        let mut state = HadesNativeState::new();
        state.set_activity(ActivityTab::Search);
        assert_eq!(state.active_activity, ActivityTab::Search);
        assert!(state.left_sidebar_open);

        // Setting same activity toggles sidebar closed
        state.set_activity(ActivityTab::Search);
        assert!(!state.left_sidebar_open);

        // Setting different activity opens it again
        state.set_activity(ActivityTab::Explorer);
        assert!(state.left_sidebar_open);
        assert_eq!(state.active_activity, ActivityTab::Explorer);
    }

    #[test]
    fn test_state_resizing() {
        let mut state = HadesNativeState::new();
        assert_eq!(state.sidebar_width, 260.0);
        assert_eq!(state.right_sidebar_width, 360.0);
        assert_eq!(state.bottom_panel_height, 240.0);

        // Sidebar resizing
        state.resize_sidebar(50.0);
        assert_eq!(state.sidebar_width, 310.0);
        state.resize_sidebar(-300.0); // clamped to min 160.0
        assert_eq!(state.sidebar_width, 160.0);
        state.resize_sidebar(1000.0); // clamped to max 600.0
        assert_eq!(state.sidebar_width, 600.0);

        // Right sidebar resizing
        state.resize_right_sidebar(100.0);
        assert_eq!(state.right_sidebar_width, 460.0);
        state.resize_right_sidebar(-500.0); // clamped to min 220.0
        assert_eq!(state.right_sidebar_width, 220.0);
        state.resize_right_sidebar(1000.0); // clamped to max 700.0
        assert_eq!(state.right_sidebar_width, 700.0);

        // Bottom panel resizing
        state.resize_bottom_panel(60.0);
        assert_eq!(state.bottom_panel_height, 300.0);
        state.resize_bottom_panel(-400.0); // clamped to min 100.0
        assert_eq!(state.bottom_panel_height, 100.0);
        state.resize_bottom_panel(900.0); // clamped to max 600.0
        assert_eq!(state.bottom_panel_height, 600.0);
    }

    #[test]
    fn test_state_extensions_integration() {
        let mut state = HadesNativeState::new();
        assert_eq!(state.extensions_category, 0); // Default to Marketplace
        assert!(!state.vsx.marketplace_results.is_empty());

        // Switch categories
        state.extensions_category = 1; // Installed
        assert_eq!(state.extensions_category, 1);
        state.extensions_category = 2; // MCP
        assert_eq!(state.extensions_category, 2);
        state.extensions_category = 3; // Skills
        assert_eq!(state.extensions_category, 3);

        // Search query and hover fields
        state.vsx.search_query = "rust".to_string();
        assert_eq!(state.vsx.search_query, "rust");
        assert!(state.hover_info.is_none());
    }

    #[test]
    fn test_state_split_editor() {
        let mut state = HadesNativeState::new();
        state.tabs.push(OpenTab::new(
            "main.rs".into(),
            "main.rs".into(),
            "fn main() {}".into(),
        ));
        state.active_tab_idx = Some(0);

        assert!(!state.is_split_editor);
        assert_eq!(state.active_pane, 0);

        // Split editor (Ctrl+\)
        state.split_editor();
        assert!(state.is_split_editor);
        assert_eq!(state.active_pane, 1);
        assert_eq!(state.secondary_tabs.len(), 1);
        assert_eq!(state.secondary_tabs[0].title, "main.rs");
        assert_eq!(state.active_secondary_tab_idx, Some(0));

        // Active tab should point to secondary tab while in pane 1
        assert_eq!(state.active_tab().unwrap().title, "main.rs");

        // Focus pane 0 (Ctrl+1)
        state.focus_pane(0);
        assert_eq!(state.active_pane, 0);
        assert_eq!(state.active_tab().unwrap().title, "main.rs");

        // Focus pane 1 (Ctrl+2)
        state.focus_pane(1);
        assert_eq!(state.active_pane, 1);

        // Toggle split orientation
        assert!(!state.split_horizontal);
        state.split_editor(); // already split -> toggles orientation
        assert!(state.split_horizontal);

        // Close secondary tab collapses split back to single editor
        state.close_secondary_tab(0);
        assert!(!state.is_split_editor);
        assert_eq!(state.active_pane, 0);
        assert!(state.secondary_tabs.is_empty());
    }

    #[test]
    fn test_state_multi_cursor_and_word_highlight() {
        let mut state = HadesNativeState::new();
        state.tabs.push(OpenTab::new(
            "test.rs".into(),
            "test.rs".into(),
            "let apple = 1;\nlet orange = apple + 2;\nlet total = apple;".into(),
        ));
        let tab_idx = state.tabs.len() - 1;
        state.active_tab_idx = Some(tab_idx);

        // Word occurrence highlights toggle
        assert!(state.multi_cursor.highlight_occurrences);
        state.toggle_word_highlight();
        assert!(!state.multi_cursor.highlight_occurrences);
        state.toggle_word_highlight();
        assert!(state.multi_cursor.highlight_occurrences);

        // Put cursor inside 'apple' on line 0
        if let Some(tab) = state.active_tab_mut() {
            tab.model.set_cursor(0, 6);
        }

        // Add next occurrence (Ctrl+D): 1st selects 'apple'
        assert!(state.add_next_occurrence());
        // 2nd adds 'apple' on line 1
        assert!(state.add_next_occurrence());
        assert_eq!(state.active_tab().unwrap().model.cursors.secondary.len(), 1);

        // 3rd adds 'apple' on line 2
        assert!(state.add_next_occurrence());
        assert_eq!(state.active_tab().unwrap().model.cursors.secondary.len(), 2);

        // Cursor undo (Ctrl+U)
        assert!(state.cursor_undo());
        assert_eq!(state.active_tab().unwrap().model.cursors.secondary.len(), 1);

        // Clear secondary cursors
        if let Some(tab) = state.active_tab_mut() {
            tab.clear_secondary_cursors();
        }
        assert_eq!(state.active_tab().unwrap().model.cursors.secondary.len(), 0);

        // Select all occurrences (Ctrl+Shift+L)
        if let Some(tab) = state.active_tab_mut() {
            tab.model.set_cursor(0, 6);
        }
        let count = state.select_all_occurrences();
        assert_eq!(count, 3);
        assert_eq!(
            state
                .active_tab()
                .unwrap()
                .model
                .cursors
                .all_selections()
                .len(),
            3
        );
    }

    #[test]
    fn test_state_output_channel_logging() {
        let mut state = HadesNativeState::new();
        state.log_output("Tasks", "Building workspace crate...");
        state.log_output("Tasks", "Build succeeded with zero errors.");

        state.output.active_channel = "Tasks".to_string();
        let lines = state.output.active_lines();
        assert!(lines
            .iter()
            .any(|l| l.contains("Building workspace crate...")));
        assert!(lines
            .iter()
            .any(|l| l.contains("Build succeeded with zero errors.")));

        // Clear channel
        state.output.clear_active();
        assert!(state.output.active_lines().is_empty());
    }

    #[test]
    fn test_state_debug_breakpoints_and_watch() {
        let mut state = HadesNativeState::new();
        let file = "src-native/src/main.rs";

        // Toggle breakpoint
        let added = state.debug.toggle_breakpoint(file, 200);
        assert!(added);
        assert!(state.debug.has_breakpoint_at(file, 200));

        // Toggle off
        let added_again = state.debug.toggle_breakpoint(file, 200);
        assert!(!added_again);
        assert!(!state.debug.has_breakpoint_at(file, 200));

        // Watch expressions
        state.debug.add_watch("state.active_pane");
        assert_eq!(
            state.debug.watch_expressions.last().unwrap().expression,
            "state.active_pane"
        );
        let initial_len = state.debug.watch_expressions.len();
        state.debug.remove_watch(initial_len - 1);
        assert_eq!(state.debug.watch_expressions.len(), initial_len - 1);
    }

    #[test]
    fn test_state_problems_aggregation() {
        let mut state = HadesNativeState::new();
        state.tabs.push(OpenTab::new(
            "src/example.rs".into(),
            "example.rs".into(),
            "fn main() { let x = 5; }".into(),
        ));
        let tab_idx = state.tabs.len() - 1;
        state.tabs[tab_idx].model.diagnostics.push(
            crate::editor::engine::decorations::DiagnosticMarker {
                row: 0,
                start_col: 17,
                end_col: 18,
                severity: crate::editor::engine::decorations::DiagnosticSeverity::Warning,
                message: "unused variable: `x`".to_string(),
            },
        );

        let diags = crate::panels::problems::collect_all_diagnostics(&state);
        assert!(diags
            .iter()
            .any(|d| d.message.contains("unused variable: `x`")));
    }

    #[test]
    fn test_test_explorer_lifecycle() {
        let mut state = HadesNativeState::new();
        assert!(state.test_explorer.suites.len() >= 4);
        let total_initial = state
            .test_explorer
            .suites
            .iter()
            .map(|s| s.tests.len())
            .sum::<usize>();
        assert!(total_initial >= 8);

        // Run all
        state.test_explorer.run_all();
        assert_eq!(state.test_explorer.total_failed, 0);
        assert_eq!(state.test_explorer.total_passed, total_initial);
    }

    #[test]
    fn test_workspace_trust_and_permission_lifecycle() {
        let mut state = HadesNativeState::new();
        assert!(state.workspace_trusted);
        assert!(!state.workspace_trust_dismissed);
        assert!(state.pending_tool_permission.is_none());

        // Revoke trust
        state.workspace_trusted = false;
        assert!(!state.workspace_trusted);

        // Trust folder
        state.trust_workspace();
        assert!(state.workspace_trusted);
        assert!(state.workspace_trust_dismissed);

        // Tool permission request in non-YOLO mode
        state.is_yolo_mode = false;
        state.request_tool_permission(
            "req_1".to_string(),
            "run_command".to_string(),
            "cargo build".to_string(),
            "dangerous".to_string(),
        );
        assert!(state.pending_tool_permission.is_some());
        assert_eq!(
            state.pending_tool_permission.as_ref().unwrap().tool,
            "run_command"
        );

        // Allow permission
        state.respond_tool_permission("req_1", true);
        assert!(state.pending_tool_permission.is_none());
        assert!(state.status_message.contains("permitted"));
    }

    #[test]
    fn test_markdown_preview_state_toggle() {
        let mut state = HadesNativeState::new();
        assert!(!state.markdown_preview.is_open);

        state.toggle_markdown_preview();
        assert!(state.markdown_preview.is_open);

        state.toggle_markdown_preview();
        assert!(!state.markdown_preview.is_open);
    }

    #[test]
    fn test_run_configs_state_lifecycle() {
        let mut state = HadesNativeState::new();
        // Discovered default tasks or parsed workspace tasks should exist
        assert!(!state.run_configs.tasks.is_empty());
        assert!(!state.run_configs.launch_configs.is_empty());

        let initial_selected = state.run_configs.selected_launch_config();
        assert!(initial_selected.is_some());

        // Cycle through launch configurations
        let count = state.run_configs.launch_configs.len();
        state.run_configs.selected_launch_index =
            (state.run_configs.selected_launch_index + 1) % count;
        assert!(state.run_configs.selected_launch_config().is_some());

        // Collapse toggles
        assert!(!state.run_configs.is_tasks_collapsed);
        state.run_configs.is_tasks_collapsed = true;
        assert!(state.run_configs.is_tasks_collapsed);

        // Status recording
        state.run_configs.last_status = Some(("build".to_string(), true, "ok".to_string()));
        assert_eq!(state.run_configs.last_status.as_ref().unwrap().0, "build");
    }

    #[test]
    fn test_ports_state_and_bottom_tab_lifecycle() {
        let mut state = HadesNativeState::new();
        assert!(!state.ports.listening_ports.is_empty());

        // Forward a port
        let initial_forward_count = state.ports.forwarded_ports.len();
        state
            .ports
            .forward_port(9999, Some("Custom Service".to_string()));
        assert_eq!(state.ports.forwarded_ports.len(), initial_forward_count + 1);

        // Switch bottom tab to Ports
        state.set_bottom_tab(BottomPanelTab::Ports);
        assert_eq!(state.active_bottom_tab, BottomPanelTab::Ports);
        assert_eq!(state.active_bottom_tab.id(), "ports");

        // Remove forward
        state.ports.unforward_port(9999);
        assert_eq!(state.ports.forwarded_ports.len(), initial_forward_count);
    }

    #[test]
    fn test_jobs_state_and_bottom_tab_lifecycle() {
        let mut state = HadesNativeState::new();
        let running_before = state.jobs.running_count();

        // Register job
        let id = state
            .jobs
            .register("Compile Shaders", "DX12 compute shaders", true);
        assert_eq!(state.jobs.running_count(), running_before + 1);

        // Switch bottom tab to Jobs
        state.set_bottom_tab(BottomPanelTab::Jobs);
        assert_eq!(state.active_bottom_tab, BottomPanelTab::Jobs);
        assert_eq!(state.active_bottom_tab.id(), "jobs");

        // Cancel job
        state.jobs.cancel(&id);
        assert_eq!(state.jobs.running_count(), running_before);
    }

    #[test]
    fn test_debug_console_state_and_bottom_tab_lifecycle() {
        let mut state = HadesNativeState::new();
        state.set_bottom_tab(BottomPanelTab::DebugConsole);
        assert_eq!(state.active_bottom_tab, BottomPanelTab::DebugConsole);
        assert_eq!(state.active_bottom_tab.id(), "debug_console");

        let status = state.debug.session_status.clone();
        state.debug_console.evaluate("40 + 2", &status);
        assert_eq!(state.debug_console.entries.last().unwrap().content, "42");
    }

    #[test]
    fn test_thought_process_state_lifecycle() {
        let mut state = HadesNativeState::new();
        assert!(state.current_thought.is_none());

        state.set_thought("Testing AST tree", "Generate code", Some(0.92));
        assert!(state.current_thought.is_some());
        let thought = state.current_thought.as_ref().unwrap();
        assert_eq!(thought.logic, "Testing AST tree");
        assert_eq!(thought.action, "Generate code");
        assert_eq!(thought.confidence, Some(0.92));

        state.clear_thought();
        assert!(state.current_thought.is_none());
    }

    #[test]
    fn test_inference_health_state_lifecycle() {
        let mut state = HadesNativeState::new();
        assert!(!state.inference_health.visible);

        state.toggle_inference_health();
        assert!(state.inference_health.visible);

        state.toggle_inference_health();
        assert!(!state.inference_health.visible);
    }

    #[test]
    fn test_mcp_store_state_lifecycle() {
        let mut state = HadesNativeState::new();
        assert_eq!(state.mcp_store.view, crate::panels::McpStoreView::Store);
        assert!(!state.mcp_store.catalog.is_empty());

        state.mcp_store.view = crate::panels::McpStoreView::Manage;
        assert_eq!(state.mcp_store.view, crate::panels::McpStoreView::Manage);

        assert!(state.mcp_store.is_installed("filesystem"));
        assert_eq!(state.mcp_store.enabled_server_count(), 3);
    }

    #[test]
    fn test_workspace_session_serialization_and_restore() {
        let session = WorkspaceSession {
            open_tabs: vec![
                OpenTabSession {
                    path: "src/main.rs".to_string(),
                    cursor_row: 10,
                    cursor_col: 4,
                    scroll_row: 5,
                    is_pinned: true,
                },
                OpenTabSession {
                    path: "Cargo.toml".to_string(),
                    cursor_row: 2,
                    cursor_col: 0,
                    scroll_row: 0,
                    is_pinned: false,
                },
            ],
            active_tab_idx: Some(1),
            secondary_open_tabs: vec![],
            secondary_active_tab_idx: None,
            is_split_editor: false,
            split_horizontal: false,
            left_sidebar_open: true,
            right_sidebar_open: false,
            iphone_preview_open: true,
            bottom_panel_open: true,
            sidebar_width: 280.0,
            right_sidebar_width: 350.0,
            iphone_preview_width: 380.0,
            bottom_panel_height: 250.0,
            active_activity: ActivityTab::Search,
            active_right_tab: RightSidebarTab::AgentStudio,
            active_bottom_tab: BottomPanelTab::Problems,
        };

        let json = serde_json::to_string_pretty(&session).expect("Serialize session");
        let restored: WorkspaceSession = serde_json::from_str(&json).expect("Deserialize session");

        assert_eq!(restored.open_tabs.len(), 2);
        assert_eq!(restored.open_tabs[0].path, "src/main.rs");
        assert_eq!(restored.open_tabs[0].cursor_row, 10);
        assert_eq!(restored.open_tabs[0].cursor_col, 4);
        assert_eq!(restored.open_tabs[0].scroll_row, 5);
        assert_eq!(restored.active_tab_idx, Some(1));
        assert_eq!(restored.sidebar_width, 280.0);
        assert_eq!(restored.bottom_panel_height, 250.0);
        assert_eq!(restored.active_activity, ActivityTab::Search);
        assert_eq!(restored.active_bottom_tab, BottomPanelTab::Problems);
        assert!(!restored.right_sidebar_open);
        assert!(restored.bottom_panel_open);
    }

    #[test]
    fn test_workspace_session_save_and_load_roundtrip() {
        let nanos = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let temp_dir = std::env::temp_dir().join(format!(
            "vscodium_test_session_{}_{}",
            std::process::id(),
            nanos
        ));
        let _ = std::fs::create_dir_all(&temp_dir);
        let session_file = temp_dir.join("test_session.json");

        let mut state = HadesNativeState::new();
        state.sidebar_width = 310.0;
        state.right_sidebar_width = 390.0;
        state.bottom_panel_height = 275.0;
        state.set_activity(ActivityTab::SourceControl);
        state.set_bottom_tab(BottomPanelTab::Output);

        // Save session directly to temp file
        state.save_session_to(&session_file);
        assert!(session_file.exists());

        // Load session back into fresh state
        let mut fresh = HadesNativeState::new();
        let restored = fresh.load_session_from(&session_file);
        assert!(restored);
        assert_eq!(fresh.sidebar_width, 310.0);
        assert_eq!(fresh.right_sidebar_width, 390.0);
        assert_eq!(fresh.bottom_panel_height, 275.0);
        assert_eq!(fresh.active_activity, ActivityTab::SourceControl);
        assert_eq!(fresh.active_bottom_tab, BottomPanelTab::Output);

        let _ = std::fs::remove_file(&session_file);
        let _ = std::fs::remove_dir(&temp_dir);
    }

    #[test]
    fn test_tab_pinning_lifecycle_and_persistence() {
        let mut state = HadesNativeState::new();
        state.tabs.push(OpenTab::new(
            "src/main.rs".to_string(),
            "main.rs".to_string(),
            "fn main() {}".to_string(),
        ));
        state.tabs.push(OpenTab::new(
            "src/lib.rs".to_string(),
            "lib.rs".to_string(),
            "pub fn run() {}".to_string(),
        ));
        assert!(!state.tabs[0].is_pinned);
        assert!(!state.tabs[1].is_pinned);

        state.pin_tab(0);
        assert!(state.tabs[0].is_pinned);
        assert!(!state.tabs[1].is_pinned);

        state.toggle_pin_tab(1);
        assert!(state.tabs[1].is_pinned);

        state.unpin_tab(0);
        assert!(!state.tabs[0].is_pinned);
        assert!(state.tabs[1].is_pinned);
    }
}
