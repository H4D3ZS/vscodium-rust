use crate::ai_tools::AiTools;
use crate::mcp_registry::McpRegistry;
use anyhow::Result;
use tracing::instrument;

use serde_json::Value;
use std::sync::Arc;
use tauri::Emitter;

pub struct ToolInvoker {
    ai_tools: Arc<AiTools>,
    mcp_registry: Arc<McpRegistry>,
}

/// Tool permission level:
/// - Safe: read-only (never prompts)
/// - Caution: writes files (prompts when `request_review` policy is set)
/// - Dangerous: executes code, deletes, git operations (always prompts)
pub enum ToolLevel {
    Safe,
    Caution,
    Dangerous,
}

pub fn classify_tool(name: &str) -> ToolLevel {
    if is_mcp_mutation_tool(name) {
        return ToolLevel::Dangerous;
    }
    if is_mcp_write_tool(name) {
        return ToolLevel::Caution;
    }
    match name {
        // Destructive / irreversible — shell execution, deletions, git history changes
        "remove_item" | "run_command" | "ghost_test" | "terminal_send_data"
        | "git_commit" | "git_push" | "git_reset"
        | "spawn_subagent" | "browser_subagent" => ToolLevel::Dangerous,

        // Write operations
        "write_to_file" | "str_replace" | "search_replace_edit" | "fast_apply"
        | "patch_file_content" | "apply_shadow_patch" | "replace_file_content"
        | "multi_replace_file_content" | "apply_patch" | "create_directory"
        | "rename_path" | "git_add" | "git_stash" => ToolLevel::Caution,

        // Offensive security analysis tools — read-only by design.
        // These produce reports/findings but do NOT execute attacks. Misclassifying
        // them as Dangerous would gate every red-team scan behind a dialog and
        // gut the IDE's core offensive security strength. Keep Safe.
        //   - apex_red_team_scan: BugTraceAI vulnerability analysis (report only)
        //   - weaponize_env: parses .env files, returns attack_plan JSON (no execution)
        //   - secrets_scan: regex over file content (read-only)
        //   - binary_mach_o_scanner, file_entropy_analysis, network_port_scanner:
        //       all produce data, no exploitation
        // The actual attack execution happens via `run_command` which IS Dangerous.

        // Everything else is read-only / safe
        _ => ToolLevel::Safe,
    }
}

/// IDA Pro / Ghidra MCP tools that mutate the IDB or run arbitrary code.
fn is_mcp_mutation_tool(name: &str) -> bool {
    matches!(
        name,
        "patch_asm"
            | "patch"
            | "put_int"
            | "py_eval"
            | "dbg_write"
            | "dbg_start"
            | "dbg_exit"
            | "dbg_continue"
            | "dbg_run_to"
            | "dbg_step_into"
            | "dbg_step_over"
            | "idb_save"
    ) || (name.starts_with("dbg_") && !matches!(name, "dbg_regs" | "dbg_regs_all" | "dbg_gpregs" | "dbg_stacktrace" | "dbg_bps" | "dbg_read"))
}

/// MCP tools that rename/annotate/type — reversible but write to the database.
fn is_mcp_write_tool(name: &str) -> bool {
    matches!(
        name,
        "set_comments"
            | "define_func"
            | "define_code"
            | "undefine"
            | "declare_type"
            | "declare_stack"
            | "delete_stack"
            | "set_type"
            | "rename"
            | "add_bookmark"
            | "infer_types"
    )
}

impl ToolInvoker {
    pub fn new(ai_tools: Arc<AiTools>, mcp_registry: Arc<McpRegistry>) -> Self {
        Self { ai_tools, mcp_registry }
    }

    /// Execute a tool, optionally requesting user permission for dangerous operations.
    /// `permission_senders` is the shared map from EditorState; `app_handle` is used
    /// to emit the `tool_permission_request` event.
    #[instrument(skip(self))]
    pub async fn execute_tool(&self, name: &str, args: &str) -> Result<Value> {
        let arguments: Value = serde_json::from_str(args)?;
        match self.ai_tools.call_tool(name, arguments.clone()).await {
            Ok(result) => Ok(result),
            Err(e) if Self::is_unknown_tool_error(&e) => {
                self.mcp_registry.call_tool(name, arguments).await
            }
            Err(e) => Err(e),
        }
    }

    /// Extended execute with permission check. Dangerous tools emit a
    /// `tool_permission_request` Tauri event and await frontend approval.
    /// Yolo mode (passed by caller via app_handle managed state) bypasses prompts.
    pub async fn execute_tool_with_permission(
        &self,
        name: &str,
        args: &str,
        app_handle: Option<&tauri::AppHandle>,
        permission_senders: Option<&Arc<std::sync::Mutex<std::collections::HashMap<String, tokio::sync::oneshot::Sender<bool>>>>>,
    ) -> Result<Value> {
        let level = classify_tool(name);
        let arguments: Value = serde_json::from_str(args)?;

        // Yolo bypass: if YOLO_MODE env var is set, skip all permission gates.
        // This is how the autonomous loop opts out of dialogs during long missions.
        let yolo = std::env::var("AIRI_YOLO_MODE").map(|v| v == "1").unwrap_or(false);

        // For dangerous tools (and yolo is off): emit permission request and wait for response
        if matches!(level, ToolLevel::Dangerous) && !yolo {
            if let (Some(handle), Some(senders)) = (app_handle, permission_senders) {
                let tool_id = format!("{}-{}", name, uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>());
                let (tx, rx) = tokio::sync::oneshot::channel::<bool>();

                {
                    let mut map = senders.lock().unwrap_or_else(|e| e.into_inner());
                    map.insert(tool_id.clone(), tx);
                }

                let _ = handle.emit("tool_permission_request", serde_json::json!({
                    "id": tool_id,
                    "tool": name,
                    "args": arguments,
                    "level": "dangerous"
                }));

                // Await approval with 60s timeout
                let approved = tokio::time::timeout(
                    std::time::Duration::from_secs(60),
                    rx,
                ).await
                    .unwrap_or(Ok(false))
                    .unwrap_or(false);

                if !approved {
                    return Ok(serde_json::json!({
                        "status": "denied",
                        "message": format!("Tool '{}' was denied by user", name)
                    }));
                }
            }
        }

        match self.ai_tools.call_tool(name, arguments.clone()).await {
            Ok(result) => Ok(result),
            Err(e) if Self::is_unknown_tool_error(&e) => {
                self.mcp_registry.call_tool(name, arguments).await
            }
            Err(e) => Err(e),
        }
    }

    /// Only fall back to MCP when the name is absent from AiTools — not when a
    /// registered tool fails mid-execution (e.g. str_replace miss, browser crash).
    fn is_unknown_tool_error(err: &anyhow::Error) -> bool {
        let msg = err.to_string();
        msg.starts_with("Unknown tool:") || msg.starts_with("Tool not found:")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn is_safe(name: &str) -> bool { matches!(classify_tool(name), ToolLevel::Safe) }
    fn is_caution(name: &str) -> bool { matches!(classify_tool(name), ToolLevel::Caution) }
    fn is_dangerous(name: &str) -> bool { matches!(classify_tool(name), ToolLevel::Dangerous) }

    /// REGRESSION GUARD: the offensive-security analysis tools are a core IDE
    /// strength and MUST stay Safe (analysis-only, no execution). If anyone
    /// re-classifies them as Dangerous, every red-team scan would block behind
    /// a permission dialog — this test fails first.
    #[test]
    fn offensive_security_tools_stay_safe() {
        for tool in [
            "apex_red_team_scan",
            "weaponize_env",
            "secrets_scan",
            "binary_mach_o_scanner",
            "file_entropy_analysis",
            "network_port_scanner",
            "extract_strings",
            "hex_dump",
            "apex_quick_check",
            "apex_threat_anticipate",
            "apex_simulate_attack",
            "apex_pentest_report",
            "apex_scan_url",
        ] {
            assert!(is_safe(tool), "{tool} must be Safe (analysis-only), not gated");
        }
    }

    /// Code-executing / irreversible tools must always prompt (Dangerous).
    #[test]
    fn destructive_tools_are_dangerous() {
        for tool in [
            "run_command",       // shell execution — the real attack vector
            "remove_item",
            "ghost_test",
            "terminal_send_data",
            "git_commit",
            "git_push",
            "git_reset",
            "spawn_subagent",
            "browser_subagent",
        ] {
            assert!(is_dangerous(tool), "{tool} must be Dangerous (executes/irreversible)");
        }
    }

    /// File-mutating tools are Caution (prompt only under request_review policy).
    #[test]
    fn write_tools_are_caution() {
        for tool in [
            "write_to_file",
            "str_replace",
            "search_replace_edit",
            "fast_apply",
            "patch_file_content",
            "apply_patch",
            "create_directory",
            "rename_path",
            "git_add",
            "git_stash",
        ] {
            assert!(is_caution(tool), "{tool} must be Caution (file write)");
        }
    }

    /// IDA/Ghidra MCP mutation tools must prompt before patching IDB or running py_eval.
    #[test]
    fn ida_mcp_mutation_tools_are_gated() {
        assert!(is_dangerous("patch_asm"));
        assert!(is_dangerous("py_eval"));
        assert!(is_dangerous("dbg_write"));
        assert!(is_caution("set_comments"));
        assert!(is_caution("rename"));
        assert!(is_safe("decompile"));
        assert!(is_safe("xrefs_to"));
        assert!(is_safe("int_convert"));
    }

    /// Read-only tools (and unknown tools) default to Safe.
    #[test]
    fn read_only_tools_default_safe() {
        for tool in ["read_file", "list_files", "grep", "view_file", "some_unknown_tool"] {
            assert!(is_safe(tool), "{tool} should default to Safe");
        }
    }
}
