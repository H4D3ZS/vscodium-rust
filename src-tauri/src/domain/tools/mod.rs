//! Tools domain: the agent tool registry, schemas, dispatcher, and per-area
//! implementations. Split from the 8.5K-LOC ai_tools.rs — one struct
//! (`AiTools`), impl blocks distributed across submodules.

pub mod browser_tools;
pub mod dispatch;
pub mod file_edit_tools;
pub mod fs_tools;
pub mod git_tools;
pub mod registry;
pub mod schemas;
pub mod search_tools;
pub mod security_tools;
pub mod shell;
pub mod task_store;
pub mod terminal_tools;
pub mod web_edit;
pub mod web_tools;
pub mod websearch;
pub mod workflow_tools;

pub use registry::{AiTools, ToolDefinition};
pub use shell::ShellTranslator;
