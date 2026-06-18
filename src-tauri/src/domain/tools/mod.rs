//! Tools domain: the agent tool registry, schemas, dispatcher, and per-area
//! implementations. Split from the 8.5K-LOC ai_tools.rs — one struct
//! (`AiTools`), impl blocks distributed across submodules.

pub mod dispatch;
pub mod fs_tools;
pub mod registry;
pub mod schemas;
pub mod shell;
pub mod web_edit;

pub use registry::{AiTools, ToolDefinition};
pub use shell::ShellTranslator;
