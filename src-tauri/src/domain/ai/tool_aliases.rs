//! Single source of truth for AI tool name normalization.
//! Used by `ai_tools::call_tool`, `ai_engine` streaming loop, and kept in sync
//! with `src/domain/agent/toolAliases.ts` on the frontend.

fn norm_key(name: &str) -> String {
    name.trim().replace('-', "_").to_lowercase()
}

/// Map model / Cursor / Claude / MCP alias names to canonical backend tool ids.
pub fn canonical_tool_name(name: &str) -> &str {
    let key = norm_key(name);
    match key.as_str() {
        // ── Terminal ──
        "bash" | "sh" | "shell" | "exec" | "execute" | "cmd" | "run" | "terminal"
        | "run_command" | "run_terminal_cmd" | "run_terminal_command"
        | "execute_command" | "execute_bash" | "shell_command" | "run_shell_command"
        | "terminal_command"         | "run_shell" | "invoke_shell" | "powershell" => "run_command",

        // ── Read ──
        "view_file" | "file_read" | "read_file" | "cat" | "read" | "view"
        | "get_file" | "readfile" | "open_file" => "view_file",

        // ── Write ──
        "write_to_file" | "file_write" | "write_file" | "create_file" | "save_file"
        | "write" | "writefile" | "save" => "write_to_file",

        // ── Simple in-file replace ──
        "str_replace" | "string_replace" | "replace_in_file" | "replace_code"
        | "find_and_replace" => "str_replace",

        // ── Surgical SEARCH/REPLACE blocks (preferred for agent edits) ──
        "search_replace_edit" | "edit_file" | "file_edit" | "edit" | "code_edit"
        | "search_replace" | "replace" | "str_replace_editor" | "apply_edit"
        | "multi_edit" | "patch_edit" => "search_replace_edit",

        // ── Line / patch edits ──
        "patch_file_content" | "patch_lines" | "line_edit" | "replace_lines"
        | "line_replace" => "patch_file_content",
        "apply_patch" | "patch" | "apply_diff" | "unified_patch" => "apply_patch",
        "fast_apply" | "sketch_apply" => "fast_apply",
        "preview_shadow_diff" => "preview_shadow_diff",
        "apply_shadow_patch" | "commit_patch" => "apply_shadow_patch",
        "ai_propose_edit" | "propose_edit" | "suggest_edit" | "draft_edit" => "ai_propose_edit",
        "replace_file_content" => "replace_file_content",
        "multi_replace_file_content" => "multi_replace_file_content",

        // ── Filesystem ──
        "glob" | "find" | "find_files" | "glob_file_search" | "file_glob"
        | "glob_files" | "find_by_name" | "file_search" => "find_by_name",
        "list_files" | "list_directory" | "list_dir" | "ls" | "dir" | "files"
        | "list" | "listdir" | "get_directory_structure" => "list_files",
        "create_directory" | "mkdir" | "md" | "create_dir" | "make_dir" => "create_directory",
        "rename_path" | "rename" | "move_file" | "move" => "rename_path",
        "remove_item" | "delete_file" | "delete" | "rm" | "unlink" => "remove_item",
        "get_file_metadata" | "file_metadata" | "stat" => "get_file_metadata",
        "list_dir_tree" | "directory_tree" => "list_dir_tree",

        // ── Search / grep ──
        "grep" | "ripgrep" | "rg" | "grep_search" | "find_string" | "find_in_files"
        | "search" | "code_search" => "grep",
        "search_files" | "filename_search" => "search_files",
        "search_codebase" | "codebase_search" | "codebasesearch" => "search_codebase",
        "semantic_search" | "search_index" | "find_context" | "vector_search" => "semantic_search",
        "find_symbols" | "lookup_symbols" | "symbols" | "symbol_search" => "find_symbols",
        "read_file_lines" | "read_range" | "file_lines" | "head" | "tail"
        | "read_lines" => "read_file_lines",

        // ── Web ──
        "web_fetch" | "read_url_content" | "fetch_url" | "http_get" | "curl_fetch" => "web_fetch",
        "web_search" | "internet_search" | "google_search" | "research" | "browse"
        | "web_research" => "web_search",
        "browser_subagent" | "browser_agent" => "browser_subagent",
        "perplexity_ask" | "ask" | "query_web" | "perplexity" => "perplexity_ask",
        "perplexity_reason" => "perplexity_reason",

        // ── Browser automation ──
        "browser_open" | "open_browser" => "browser_open",
        "browser_navigate" | "navigate" | "goto_url" => "browser_navigate",
        "browser_screenshot" | "screenshot" | "capture_page" => "browser_screenshot",
        "browser_close" | "close_browser" => "browser_close",
        "browser_read_dom" | "read_dom" | "get_dom" => "browser_read_dom",
        "browser_click" | "click" => "browser_click",
        "browser_type" | "type_text" => "browser_type",
        "browser_search" => "browser_search",

        // ── Terminal panel ──
        "terminal_send_data" | "terminal_send" | "send_terminal" | "term_send"
        | "type_to_terminal" => "terminal_send_data",
        "terminal_read_output" | "terminal_read" | "read_terminal" | "term_read"
        | "get_output" => "terminal_read_output",
        "terminal_list" | "terminal_get_state" | "list_terminals" => "terminal_list",
        "terminal_create" | "new_terminal" => "terminal_create",
        "terminal_terminate" | "kill_terminal" => "terminal_terminate",
        "terminal_get_status" | "terminal_status" => "terminal_get_status",

        // ── Git ──
        "git_status" | "status" => "git_status",
        "git_diff" | "git_diff_tool" => "git_diff",
        "git_add" | "stage" => "git_add",
        "git_commit" | "commit" => "git_commit",
        "git_log" | "log" => "git_log",

        // ── Dev / verify ──
        "dev_cargo_diagnostics" | "cargo_check" | "cargo_check_diagnostics"
        | "check" | "diagnostics" | "rustc_check" => "dev_cargo_diagnostics",
        "verify_implementation" | "verify" | "build" | "cargo_build" | "test"
        | "run_tests" => "verify_implementation",
        "get_lsp_diagnostics" | "lsp_diagnostics" | "lint" => "get_lsp_diagnostics",
        "reindex_project" | "reindex" => "reindex_project",
        "ide_get_state" | "get_ide_state" | "state" | "editor_state" => "ide_get_state",
        "editor_open_file" => "editor_open_file",

        // ── Binary / RE ──
        "hex_dump" | "hexdump" | "hex" => "hex_dump",
        "extract_strings" | "strings" | "get_strings" => "extract_strings",
        "list_active_processes" | "ps" | "processes" | "top" => "list_active_processes",
        "binary_mach_o_scanner" | "macho_scan" | "kernel_scan" | "mach_o" => "binary_mach_o_scanner",
        "file_entropy_analysis" | "entropy" | "packer_check" => "file_entropy_analysis",
        "disassemble" | "disasm" | "disassemble_binary" => "disassemble",
        "get_binary_info" | "bin_info" | "file_info" => "get_binary_info",

        // ── Pentest / offensive security ──
        "network_port_scanner" | "nmap" | "port_scan" | "scan_ports" | "portscan" => "network_port_scanner",
        "network_scan" | "network_recon" | "host_discovery" => "network_scan",
        "exploit_lookup" | "exploit_search" | "searchsploit" | "cve_lookup" => "exploit_lookup",
        "secrets_scan" | "find_secrets" | "secret_scan" | "scan_secrets" => "secrets_scan",
        "weaponize_env" | "weaponize" | "env_weaponize" => "weaponize_env",
        "sec_distro_inventory" | "kali_inventory" | "kali_tools" | "parrot_tools"
        | "sec_distro" | "distro_inventory" => "sec_distro_inventory",
        "deep_security_audit" | "security_audit" | "audit_code" | "code_audit"
        | "vuln_scan" | "vulnerability_scan" | "static_analysis" => "deep_security_audit",
        "web_security_audit" | "web_audit" | "url_audit" | "pentest_web"
        | "scan_website" => "web_security_audit",
        "ai_vuln_hunt" | "vuln_hunt" | "bug_hunt" | "hunt_vulns" => "ai_vuln_hunt",
        "security_scan" | "security_check" => "security_scan",
        "audit_dependencies" | "dependency_audit" | "npm_audit" | "cargo_audit" => "audit_dependencies",
        "reverse_shell_generate" | "reverse_shell" | "gen_reverse_shell"
        | "rev_shell" => "reverse_shell_generate",
        "security_listener_generate" | "listener_generate" | "bind_shell"
        | "listener" => "security_listener_generate",
        "csp_bypass_analyze" | "csp_analyze" | "csp_bypass" => "csp_bypass_analyze",
        "shellcode_recipe_generate" | "shellcode" | "gen_shellcode" => "shellcode_recipe_generate",
        "payload_encode" | "encode_payload" | "obfuscate_payload" => "payload_encode",

        // ── APEX engines ──
        "apex_red_team_scan" | "red_team_scan" | "red_team" | "pentest_scan"
        | "apex_scan" => "apex_red_team_scan",
        "apex_quick_check" | "quick_scan" | "quick_security_check" => "apex_quick_check",
        "apex_scan_url" | "scan_url" | "url_scan" => "apex_scan_url",
        "apex_threat_anticipate" | "threat_model" | "threat_anticipate" => "apex_threat_anticipate",
        "apex_simulate_attack" | "apex_execute_attack" | "simulate_attack" | "attack_sim" | "attack_simulation" | "live_attack_chain" => "apex_simulate_attack",
        "apex_pentest_report" | "pentest_report" | "generate_report" => "apex_pentest_report",
        "apex_security_explain" | "explain_vuln" => "apex_security_explain",
        "apex_perf_optimize" | "perf_optimize" => "apex_perf_optimize",
        "apex_self_improve" => "apex_self_improve",
        "apex_predict_failures" => "apex_predict_failures",
        "apex_full_sweep" | "full_sweep" => "apex_full_sweep",
        "apex_architect_design" | "architect_design" => "apex_architect_design",

        // ── AIM / AG / workflow ──
        "aim_query_spans" | "query_spans" => "aim_query_spans",
        "aim_pack_context" | "pack_context" => "aim_pack_context",
        "ag_get_next_task" => "ag_get_next_task",
        "ag_mark_task_done" => "ag_mark_task_done",
        "ag_phase_wrap" => "ag_phase_wrap",
        "ag_list_tasks" => "ag_list_tasks",
        "todo_write" => "todo_write",
        "task_create" => "task_create",
        "task_update" => "task_update",
        "project_rules" | "get_rules" => "project_rules",
        "save_knowledge_brief" | "save_brief" => "save_knowledge_brief",
        "verify_claim" => "verify_claim",
        "use_skill" | "skill_execute" | "run_skill" => "use_skill",
        "search_skills" | "find_skill" => "search_skills",
        "spawn_subagent" | "subagent" => "spawn_subagent",
        "task_boundary" | "boundary" => "task_boundary",
        "notify_user" | "ask_user" => "notify_user",
        "get_system_health" | "system_health" => "get_system_health",
        "get_system_info" | "system_info" => "get_system_info",
        "see_the_screen" | "screen_capture" => "see_the_screen",
        "ghost_test" => "ghost_test",
        "find_api_keys" => "find_api_keys",
        "analyze_file_symbols" => "analyze_file_symbols",
        "generate_image" => "generate_image",
        "analyze_image" => "analyze_image",
        "dependency_graph" => "dependency_graph",
        "get_symbol_graph" => "get_symbol_graph",
        "create_mission_plan" | "mission_plan" => "create_mission_plan",
        "revert_checkpoint" | "restore_checkpoint" => "revert_checkpoint",
        "list_mcp_ops" | "mcp_call" => "list_mcp_ops",

        // Already canonical — pass through trimmed
        other if is_known_backend_tool(other) => name.trim(),

        _ => name.trim(),
    }
}

/// Backend tools that exist in `ai_tools::call_tool` — used to preserve casing on pass-through.
fn is_known_backend_tool(key: &str) -> bool {
    matches!(
        key,
        "view_file"
            | "write_to_file"
            | "str_replace"
            | "search_replace_edit"
            | "fast_apply"
            | "patch_file_content"
            | "apply_patch"
            | "apply_shadow_patch"
            | "preview_shadow_diff"
            | "ai_propose_edit"
            | "replace_file_content"
            | "multi_replace_file_content"
            | "list_files"
            | "find_by_name"
            | "grep"
            | "search_files"
            | "search_codebase"
            | "semantic_search"
            | "run_command"
            | "browser_open"
            | "browser_navigate"
            | "browser_screenshot"
            | "browser_close"
            | "browser_read_dom"
            | "web_security_audit"
            | "deep_security_audit"
            | "ai_vuln_hunt"
            | "secrets_scan"
            | "weaponize_env"
            | "sec_distro_inventory"
            | "network_port_scanner"
            | "network_scan"
            | "exploit_lookup"
            | "reverse_shell_generate"
            | "apex_red_team_scan"
            | "apex_quick_check"
            | "apex_pentest_report"
            | "manage_task"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_aliases() {
        assert_eq!(canonical_tool_name("run_terminal_cmd"), "run_command");
        assert_eq!(canonical_tool_name("Bash"), "run_command");
    }

    #[test]
    fn pentest_aliases() {
        assert_eq!(canonical_tool_name("nmap"), "network_port_scanner");
        assert_eq!(canonical_tool_name("searchsploit"), "exploit_lookup");
        assert_eq!(canonical_tool_name("apex_execute_attack"), "apex_simulate_attack");
        assert_eq!(canonical_tool_name("live_attack_chain"), "apex_simulate_attack");
        assert_eq!(canonical_tool_name("vuln_hunt"), "ai_vuln_hunt");
    }

    #[test]
    fn edit_aliases() {
        assert_eq!(canonical_tool_name("edit"), "search_replace_edit");
        assert_eq!(canonical_tool_name("str_replace"), "str_replace");
    }

    #[test]
    fn grep_not_search_files() {
        assert_eq!(canonical_tool_name("search"), "grep");
        assert_eq!(canonical_tool_name("find_in_files"), "grep");
    }
}
