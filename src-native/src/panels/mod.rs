#![allow(unused_imports)]

pub mod agent_chat;
pub mod agent_studio;
pub mod apex;
pub mod browser;
pub mod canvases;
pub mod debug;
pub mod daemon_supervisor;
pub mod debug_console;
pub mod device_window;
pub mod diff_viewer;
pub mod explorer;
pub mod extensions;
pub mod git_graph;
pub mod jobs;
pub mod keybindings;
pub mod kortex;
pub mod logcat;
pub mod markdown_preview;
pub mod mcp_store;
pub mod mobile;
pub mod outline;
pub mod output;
pub mod ports;
pub mod problems;
pub mod pytorch_studio;
pub mod registry;
pub mod rules;
pub mod run_configs;
pub mod scm;
pub mod search;
pub mod settings;
pub mod specs;
pub mod terminal;
pub mod test_explorer;
pub mod traits;
pub mod vector_search;
pub mod vision;

pub use agent_chat::{
    render_agent_chat_body, render_agent_chat_panel, AgentChatPanel, ComposerTab,
};
pub use agent_studio::{render_right_sidebar_panel, AgentStudioTab};
pub use apex::{render_apex_panel, ApexPanel, ApexSubTab};
pub use browser::{render_browser_panel, BrowserPanel, BrowserPanelState};
pub use canvases::{render_canvas_view, render_canvases_panel, CanvasesPanel};
pub use debug::{render_debug_panel, DebugPanel, DebugState};
pub use debug_console::{DebugConsolePanel, DebugConsoleState};
pub use diff_viewer::{render_diff_viewer, DiffViewerState};
pub use explorer::{render_explorer_panel, ExplorerPanel};
pub use extensions::{render_extensions_panel, ExtensionsPanel};
pub use git_graph::{render_git_graph, GitCommit, GitGraphState};
pub use jobs::{BackgroundJob, JobStatus, JobsPanel, JobsState};
pub use keybindings::{
    render_keybindings_panel, KeybindingItem, KeybindingSource, KeybindingsPanel, KeybindingsState,
};
pub use kortex::{render_kortex_panel, KortexPanel};
pub use logcat::{list_adb_devices, parse_logcat_line, LogcatEntry, LogcatPanel, LogcatState};
pub use markdown_preview::{render_markdown_preview, MarkdownPreviewState};
pub use mcp_store::{
    render_mcp_store_panel, McpCatalogCategory, McpCatalogEntry, McpServerConfig, McpStorePanel,
    McpStoreState, McpStoreView,
};
pub use mobile::{render_mobile_panel, MobilePanel};
pub use outline::{render_outline_panel, OutlinePanel, OutlineState};
pub use output::{OutputPanel, OutputState};
pub use ports::{PortsPanel, PortsState};
pub use problems::{ProblemsPanel, ProblemsState};
pub use pytorch_studio::{render_pytorch_panel, PyTorchStudioPanel};
pub use registry::PanelRegistry;
pub use rules::{render_rules_panel, RulesPanel};
pub use run_configs::{
    render_run_configs_section, RunConfigsState, VsCodeLaunchConfig, VsCodeTask,
};
pub use scm::{render_scm_panel, ScmPanel, ScmState};
pub use search::{render_search_panel, SearchPanel, SearchState};
pub use settings::{render_settings_panel, SettingsPanel};
pub use specs::{render_specs_panel, SpecsPanel};
pub use terminal::{render_terminal_panel, TerminalPanel};
pub use test_explorer::{render_test_explorer_panel, TestExplorerPanel, TestExplorerState};
pub use traits::{AuxiliaryTab, BottomPanelTab, WorkbenchPanel};
pub use vector_search::{execute_search, refresh_stats, VectorSearchPanel, VectorSearchState};
pub use vision::{render_vision_panel, VisionPanel};
