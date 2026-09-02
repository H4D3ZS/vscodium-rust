//! Tool schema catalog: list_tools() JSON definitions + canonical naming.
use serde_json::json;
use super::registry::AiTools;
use super::registry::ToolDefinition;

/// Construct a ToolDefinition from name, description, and JSON schema.
fn td(name: &str, desc: &str, schema: serde_json::Value) -> ToolDefinition {
    ToolDefinition {
        name: name.to_string(),
        description: desc.to_string(),
        input_schema: schema,
    }
}

/// Helper: build a JSON Schema object from required fields + properties object.
fn obj_schema(required: &[&str], props: serde_json::Value) -> serde_json::Value {
    let mut s = json!({ "type": "object", "properties": props });
    if !required.is_empty() {
        s.as_object_mut().unwrap().insert(
            "required".to_string(),
            json!(required),
        );
    }
    s
}

/// Helper: a simple string property.
fn str_prop(desc: &str) -> serde_json::Value {
    json!({ "type": "string", "description": desc })
}

/// Helper: an optional string property.
fn opt_str(desc: &str) -> serde_json::Value {
    json!({ "type": "string", "description": desc })
}

/// Helper: an integer property.
fn int_prop(desc: &str) -> serde_json::Value {
    json!({ "type": "integer", "description": desc })
}

/// Helper: an integer property with default.
fn int_default(desc: &str, default: i64) -> serde_json::Value {
    json!({ "type": "integer", "description": desc, "default": default })
}

/// Helper: a boolean property with default.
fn bool_default(desc: &str, default: bool) -> serde_json::Value {
    json!({ "type": "boolean", "description": desc, "default": default })
}

/// Helper: an array property.
fn arr_prop(item_schema: serde_json::Value) -> serde_json::Value {
    json!({ "type": "array", "items": item_schema })
}

impl AiTools {
    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![
            // ── File Operations ──
            td("view_file", "Read the content of a file",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("write_to_file", "Write content to a file. Overwrites if exists, creates if not.",
               obj_schema(&["path", "content"], json!({
                   "path": str_prop("Relative path to the file"),
                   "content": str_prop("Content to write")
               }))),
            td("str_replace", "Replace an exact string in a file. Reads the file, replaces old_str with new_str, writes back. Use this for surgical code edits.",
               obj_schema(&["path", "old_str", "new_str"], json!({
                   "path": str_prop("Relative path to the file"),
                   "old_str": str_prop("Exact string to find (must exist in file)"),
                   "new_str": str_prop("Replacement string")
               }))),
            td("fast_apply", "Apply a short Cursor-style edit sketch to a file. Provide ONLY the changed regions plus elision markers for unchanged regions.",
               obj_schema(&["path", "edit"], json!({
                   "path": str_prop("Relative path to the file"),
                   "edit": str_prop("Short sketch with new code + elision markers for unchanged regions"),
                   "dry_run": bool_default("If true, return merged result without writing", false)
               }))),
            td("search_replace_edit", "Surgically edit a file using SEARCH/REPLACE blocks. Format:\n<<<< SEARCH\n<exact existing code>\n====\n<new code>\n>>>>",
               obj_schema(&["path", "content"], json!({
                   "path": str_prop("Relative path to the file"),
                   "content": str_prop("SEARCH/REPLACE block(s)"),
                   "direct_apply": bool_default("If true, skip shadow buffer and write immediately to disk", false)
               }))),
            td("preview_shadow_diff", "Preview the diff of uncommitted (shadow) changes for a specific file.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("apply_shadow_patch", "Apply pending shadow changes to disk.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("replace_file_content", "Replace all occurrences of a string in a file.",
               obj_schema(&["path", "old_str", "new_str"], json!({
                   "path": str_prop("Relative path to the file"),
                   "old_str": str_prop("String to find"),
                   "new_str": str_prop("Replacement string")
               }))),
            td("multi_replace_file_content", "Replace multiple strings in a file in a single operation.",
               obj_schema(&["path", "replacements"], json!({
                   "path": str_prop("Relative path to the file"),
                   "replacements": arr_prop(json!({ "type": "object" }))
               }))),
            td("patch_file_content", "Edit a file by line range (start_line, end_line, content).",
               obj_schema(&["path", "start_line", "end_line", "content"], json!({
                   "path": str_prop("Relative path to the file"),
                   "start_line": int_prop("Start line (1-indexed)"),
                   "end_line": int_prop("End line (1-indexed)"),
                   "content": str_prop("Replacement content")
               }))),
            td("apply_patch", "Apply a unified diff patch to a file.",
               obj_schema(&["path", "patch"], json!({
                   "path": str_prop("Relative path to the file"),
                   "patch": str_prop("Unified diff patch content")
               }))),

            // ── File Discovery ──
            td("remove_item", "Remove a file or directory.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file or directory") }))),
            td("list_files", "List files in a directory.",
               obj_schema(&["path"], json!({
                   "path": str_prop("Relative path to directory"),
                   "pattern": opt_str("Optional glob pattern")
               }))),
            td("list_dir_tree", "List directory tree structure.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to directory") }))),
            td("search_files", "Search for files matching a pattern.",
               obj_schema(&["query"], json!({
                   "query": str_prop("Search query or pattern"),
                   "path": opt_str("Directory to search in")
               }))),
            td("find_by_name", "Find files by name pattern.",
               obj_schema(&["query"], json!({
                   "query": str_prop("File name pattern"),
                   "path": opt_str("Directory to search in")
               }))),
            td("get_directory_structure", "Get directory structure as JSON tree.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to directory") }))),
            td("create_directory", "Create a directory (and parents if needed).",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to create") }))),
            td("rename_path", "Rename a file or directory.",
               obj_schema(&["old_path", "new_path"], json!({
                   "old_path": str_prop("Current path"),
                   "new_path": str_prop("New path")
               }))),
            td("grep", "Search file contents with regex.",
               obj_schema(&["query"], json!({
                   "query": str_prop("Regex pattern"),
                   "path": opt_str("Directory to search in"),
                   "file_types": opt_str("Comma-separated extensions")
               }))),
            td("get_file_metadata", "Get file metadata (size, modified, permissions).",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("read_file_lines", "Read specific line ranges from a file.",
               obj_schema(&["path", "start_line", "end_line"], json!({
                   "path": str_prop("Relative path to the file"),
                   "start_line": int_prop("Start line (1-indexed)"),
                   "end_line": int_prop("End line (1-indexed)")
               }))),
            td("hex_dump", "Display hex dump of a binary file.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("extract_strings", "Extract printable strings from a binary file.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),

            // ── Editor Integration ──
            td("editor_open_file", "Open a file in the IDE editor.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the file") }))),
            td("editor_get_active_file", "Get the currently active file in the editor.",
               obj_schema(&[], json!({}))),

            // ── Indexing & Search ──
            td("semantic_search", "Semantic code search using vector embeddings.",
               obj_schema(&["query"], json!({
                   "query": str_prop("Search query"),
                   "max_results": int_default("Max results", 10)
               }))),
            td("search_codebase", "Full-text codebase search with symbol lookup.",
               obj_schema(&["query"], json!({
                   "query": str_prop("Search query"),
                   "max_results": int_default("Max results", 30),
                   "file_types": opt_str("Comma-separated extensions")
               }))),
            td("find_symbols", "Find symbols by name in the project.",
               obj_schema(&["query"], json!({ "query": str_prop("Symbol name or pattern") }))),
            td("codebase_map", "Get a compact one-line-per-directory map of the whole codebase for orientation. Cheap; call once when you need to understand the repo layout.",
               obj_schema(&[], json!({ "max_chars": int_default("Max map size in characters", 12000) }))),
            td("get_file_signatures", "Get a file's function/type/trait signatures and doc comments with bodies stripped — a token-cheap way to understand a file without reading its full source. Use after codebase_map to drill into a specific file.",
               obj_schema(&["path"], json!({ "path": str_prop("Relative path to the source file") }))),
            td("get_symbol_graph", "Get the project symbol dependency graph.",
               obj_schema(&[], json!({ "max_depth": int_default("Max graph depth", 3) }))),
            td("aim_pack_context", "Pack current context into .aim binary format.",
               obj_schema(&["query"], json!({ "query": str_prop("Context query") }))),
            td("aim_query_spans", "Query .aim semantic spans.",
               obj_schema(&["query"], json!({ "query": str_prop("Query string") }))),
            td("reindex_project", "Force re-index the entire project.",
               obj_schema(&[], json!({}))),
            td("list_mcp_ops", "List available MCP operations.",
               obj_schema(&[], json!({}))),

            // ── Processes ──
            td("list_active_processes", "List currently running processes.",
               obj_schema(&[], json!({}))),

            // ── Git ──
            td("git_status", "Show git working tree status.",
               obj_schema(&[], json!({}))),
            td("git_add", "Stage files for commit.",
               obj_schema(&["path"], json!({ "path": str_prop("File path (or '.' for all)") }))),
            td("git_commit", "Create a git commit.",
               obj_schema(&["message"], json!({ "message": str_prop("Commit message") }))),
            td("git_diff", "Show git diff.",
               obj_schema(&[], json!({}))),
            td("git_log", "Show git log.",
               obj_schema(&[], json!({ "count": int_default("Number of commits", 10) }))),

            // ── Terminal ──
            td("run_command", "Execute a shell command.",
               obj_schema(&["command"], json!({ "command": str_prop("Shell command to execute") }))),
            td("terminal_create", "Create a new terminal instance.",
               obj_schema(&[], json!({ "name": opt_str("Terminal name") }))),
            td("terminal_send_data", "Send input to a terminal.",
               obj_schema(&["id", "data"], json!({
                   "id": str_prop("Terminal ID"),
                   "data": str_prop("Data to send")
               }))),
            td("terminal_read_output", "Read terminal output.",
               obj_schema(&["id"], json!({ "id": str_prop("Terminal ID") }))),
            td("terminal_toggle", "Toggle terminal visibility.",
               obj_schema(&["id"], json!({ "id": str_prop("Terminal ID") }))),
            td("terminal_terminate", "Terminate a terminal.",
               obj_schema(&["id"], json!({ "id": str_prop("Terminal ID") }))),
            td("terminal_get_status", "Get terminal status.",
               obj_schema(&["id"], json!({ "id": str_prop("Terminal ID") }))),
            td("terminal_list", "List all terminals.",
               obj_schema(&[], json!({}))),

            // ── Browser ──
            td("browser_open", "Open a URL in the browser.",
               obj_schema(&["url"], json!({ "url": str_prop("URL to open") }))),
            td("browser_navigate", "Navigate to a URL.",
               obj_schema(&["url"], json!({ "url": str_prop("URL to navigate to") }))),
            td("browser_close", "Close the browser.",
               obj_schema(&[], json!({}))),
            td("browser_screenshot", "Take a screenshot of the current page.",
               obj_schema(&[], json!({}))),
            td("browser_click", "Click an element on the page.",
               obj_schema(&["selector"], json!({ "selector": str_prop("CSS selector") }))),
            td("browser_type", "Type text into an input field.",
               obj_schema(&["selector", "text"], json!({
                   "selector": str_prop("CSS selector"),
                   "text": str_prop("Text to type")
               }))),
            td("browser_read_dom", "Read the current page DOM.",
               obj_schema(&[], json!({}))),
            td("browser_search", "Search using the browser.",
               obj_schema(&["query"], json!({ "query": str_prop("Search query") }))),
            td("browser_get_content_summary", "Get a summary of the current page content.",
               obj_schema(&[], json!({}))),

            // ── Security Tools ──
            td("network_port_scanner", "Scan for open ports on a target.",
               obj_schema(&["target", "ports"], json!({
                   "target": str_prop("Target IP or hostname"),
                   "ports": arr_prop(json!({ "type": "integer" }))
               }))),
            td("binary_mach_o_scanner", "Analyze a Mach-O binary file.",
               obj_schema(&["path"], json!({ "path": str_prop("Path to binary") }))),
            td("file_entropy_analysis", "Analyze file entropy to detect packed/encrypted content.",
               obj_schema(&["path"], json!({ "path": str_prop("Path to file") }))),
            td("secrets_scan", "Scan for hardcoded secrets and API keys.",
               obj_schema(&["path"], json!({ "path": str_prop("Directory to scan") }))),
            td("weaponize_env", "Generate a security test environment.",
               obj_schema(&["path"], json!({ "path": str_prop("Target path") }))),
            td("sec_distro_inventory", "Inventory security-relevant files in the project.",
               obj_schema(&[], json!({}))),
            td("deep_security_audit", "Perform a deep security audit of the codebase.",
               obj_schema(&["path"], json!({ "path": str_prop("Directory to audit") }))),
            td("web_security_audit", "Perform a web application security audit.",
               obj_schema(&["url"], json!({ "url": str_prop("Target URL") }))),
            td("ai_vuln_hunt", "AI-powered vulnerability hunting.",
               obj_schema(&["path"], json!({ "path": str_prop("Target path") }))),
            td("vega_dast_scan", "Run a DAST security scan.",
               obj_schema(&["url"], json!({ "url": str_prop("Target URL") }))),
            td("chunk_secret_scan", "Scan for secrets using chunked analysis.",
               obj_schema(&["path"], json!({ "path": str_prop("Path to scan") }))),
            td("bounty_scan", "Bug bounty reconnaissance scan.",
               obj_schema(&["target"], json!({ "target": str_prop("Target domain") }))),
            td("oast_payload", "Generate an OAST payload for blind vulnerability detection.",
               obj_schema(&["type"], json!({ "type": str_prop("Payload type") }))),
            td("oast_interactions", "Check for OAST interactions.",
               obj_schema(&[], json!({}))),

            // ── Web ──
            td("web_fetch", "Fetch content from a URL.",
               obj_schema(&["url"], json!({ "url": str_prop("URL to fetch") }))),
            td("web_search", "Search the web.",
               obj_schema(&["query"], json!({ "query": str_prop("Search query") }))),

            // ── Diagnostics ──
            td("dev_cargo_diagnostics", "Get Rust compiler diagnostics.",
               obj_schema(&[], json!({}))),
            td("get_lsp_diagnostics", "Get LSP diagnostics for a file.",
               obj_schema(&["path"], json!({ "path": str_prop("File path") }))),

            // ── AI ──
            td("ai_propose_edit", "Propose a code edit via the AI engine.",
               obj_schema(&["path", "instruction"], json!({
                   "path": str_prop("File path"),
                   "instruction": str_prop("Edit instruction")
               }))),
            td("ghost_test", "Run a test via the ghost runtime.",
               obj_schema(&["command"], json!({ "command": str_prop("Test command") }))),
            td("verify_implementation", "Verify implementation correctness.",
               obj_schema(&["task"], json!({ "task": str_prop("Task description") }))),
            td("create_mission_plan", "Create a structured mission plan.",
               obj_schema(&["task"], json!({ "task": str_prop("Task description") }))),
            td("revert_checkpoint", "Revert to a git checkpoint.",
               obj_schema(&["checkpoint_id"], json!({ "checkpoint_id": str_prop("Checkpoint ID") }))),

            // ── Knowledge ──
            td("save_knowledge_brief", "Save a knowledge brief for future reference.",
               obj_schema(&["topic", "content"], json!({
                   "topic": str_prop("Topic"),
                   "content": str_prop("Knowledge content")
               }))),
            td("verify_claim", "Verify a factual claim.",
               obj_schema(&["claim"], json!({ "claim": str_prop("Claim to verify") }))),
            td("see_the_screen", "Capture and analyze the current screen.",
               obj_schema(&[], json!({}))),

            // ── IDE State ──
            td("ide_get_state", "Get current IDE state.",
               obj_schema(&[], json!({}))),

            // ── Image Generation ──
            td("generate_image", "Generate an image from a text prompt.",
               obj_schema(&["prompt"], json!({ "prompt": str_prop("Image description") }))),
            td("analyze_image", "Analyze an image.",
               obj_schema(&["path"], json!({ "path": str_prop("Image path") }))),

            // ── Code Analysis ──
            td("code_search", "Search for code patterns.",
               obj_schema(&["query"], json!({ "query": str_prop("Search query") }))),
            td("dependency_graph", "Get the project dependency graph.",
               obj_schema(&[], json!({}))),
            td("analyze_file_symbols", "Analyze symbols in a file.",
               obj_schema(&["path"], json!({ "path": str_prop("File path") }))),

            // ── System ──
            td("get_system_info", "Get system information.",
               obj_schema(&[], json!({}))),
            td("get_system_health", "Get system health status.",
               obj_schema(&[], json!({}))),

            // ── Agentic ──
            td("spawn_subagent", "Spawn a sub-agent for a task.",
               obj_schema(&["task"], json!({ "task": str_prop("Task description") }))),
            td("browser_subagent", "Spawn a browser sub-agent.",
               obj_schema(&["task"], json!({ "task": str_prop("Task description") }))),
            td("perplexity_ask", "Ask Perplexity a question.",
               obj_schema(&["query"], json!({ "query": str_prop("Question") }))),
            td("perplexity_reason", "Reason about a question using Perplexity.",
               obj_schema(&["query"], json!({ "query": str_prop("Question") }))),

            // ── Task Management ──
            td("todo_write", "Write a TODO item.",
               obj_schema(&["content"], json!({
                   "content": str_prop("TODO content"),
                   "file_path": opt_str("File to write to")
               }))),
            td("task_create", "Create a task.",
               obj_schema(&["title"], json!({
                   "title": str_prop("Task title"),
                   "description": opt_str("Task description"),
                   "parent_id": opt_str("Parent task ID for subtasks")
               }))),
            td("task_update", "Update a task.",
               obj_schema(&["id", "status"], json!({
                   "id": str_prop("Task ID"),
                   "status": str_prop("New status: todo|in_progress|done|blocked"),
                   "description": opt_str("Updated description")
               }))),
            td("task_list", "List all tasks with optional status filter.",
               obj_schema(&[], json!({
                   "status": opt_str("Filter by status: todo|in_progress|done|blocked")
               }))),
            td("task_get", "Get a specific task by ID.",
               obj_schema(&["id"], json!({
                   "id": str_prop("Task ID")
               }))),
            td("tool_search", "Search available tools by keyword. Use when you have too many tools and need to find the right one.",
               obj_schema(&["query"], json!({
                   "query": str_prop("Search query (e.g. 'file edit', 'git commit', 'search')")
               }))),

            // ── Workflow ──
            td("task_boundary", "Create a task boundary marker.",
               obj_schema(&["description"], json!({ "description": str_prop("Boundary description") }))),
            td("create_canvas", "Create a visual canvas.",
               obj_schema(&["title"], json!({ "title": str_prop("Canvas title") }))),
            td("notify_user", "Notify the user with a message.",
               obj_schema(&["message"], json!({ "message": str_prop("Notification message") }))),
            td("use_skill", "Use a registered skill.",
               obj_schema(&["skill"], json!({
                   "skill": str_prop("Skill name"),
                   "args": opt_str("Skill arguments")
               }))),
            td("search_skills", "Search for available skills.",
               obj_schema(&["query"], json!({ "query": str_prop("Search query") }))),

            // ── Offensive / Exploit ──
            td("generate_0day_exploit", "Generate an exploit scaffold (educational).",
               obj_schema(&["target"], json!({ "target": str_prop("Target vulnerability") }))),
            td("reverse_engineer_firmware", "Reverse engineer firmware.",
               obj_schema(&["path"], json!({ "path": str_prop("Firmware path") }))),
            td("advanced_reverse_engineering", "Advanced binary analysis.",
               obj_schema(&["path"], json!({ "path": str_prop("Binary path") }))),
            td("network_scan", "Scan a network target.",
               obj_schema(&["target"], json!({
                   "target": str_prop("Target IP/CIDR"),
                   "intensity": opt_str("Scan intensity")
               }))),
            td("exploit_lookup", "Look up known exploits.",
               obj_schema(&["query"], json!({ "query": str_prop("Search query") }))),

            // ── Security Generators ──
            td("security_scan", "Run a security scan.",
               obj_schema(&["path"], json!({ "path": str_prop("Target path") }))),
            td("audit_dependencies", "Audit project dependencies.",
               obj_schema(&[], json!({}))),
            td("disassemble", "Disassemble a binary.",
               obj_schema(&["path"], json!({ "path": str_prop("Binary path") }))),
            td("get_binary_info", "Get binary file information.",
               obj_schema(&["path"], json!({ "path": str_prop("Binary path") }))),
            td("reverse_shell_generate", "Generate a reverse shell payload (educational).",
               obj_schema(&["host"], json!({
                   "language": opt_str("Language"),
                   "host": str_prop("Lhost"),
                   "port": int_default("Lport", 4444)
               }))),
            td("security_listener_generate", "Generate a listener command (educational).",
               obj_schema(&[], json!({
                   "kind": opt_str("Listener type"),
                   "host": opt_str("Bind host"),
                   "port": int_default("Port", 4444)
               }))),
            td("csp_bypass_analyze", "Analyze a Content-Security-Policy header.",
               obj_schema(&["header"], json!({ "header": str_prop("CSP header value") }))),
            td("shellcode_recipe_generate", "Generate shellcode recipes (educational).",
               obj_schema(&[], json!({
                   "platform": opt_str("Target platform"),
                   "arch": opt_str("Architecture"),
                   "payload": opt_str("Payload type")
               }))),
            td("payload_encode", "Encode a payload.",
               obj_schema(&["payload"], json!({
                   "payload": str_prop("Payload to encode"),
                   "encoding": opt_str("Encoding type")
               }))),

            // ── APEX Intelligence ──
            td("apex_red_team_scan", "Run APEX red team scan.",
               obj_schema(&["target"], json!({ "target": str_prop("Target") }))),
            td("apex_scan_url", "Scan a URL with APEX.",
               obj_schema(&["url"], json!({ "url": str_prop("URL") }))),
            td("apex_threat_anticipate", "Anticipate threats with APEX.",
               obj_schema(&["context"], json!({ "context": str_prop("Context") }))),
            td("apex_perf_optimize", "Optimize performance with APEX.",
               obj_schema(&["code"], json!({ "code": str_prop("Code to optimize") }))),
            td("apex_self_improve", "APEX self-improvement analysis.",
               obj_schema(&[], json!({}))),
            td("apex_security_explain", "Explain security implications.",
               obj_schema(&["code"], json!({ "code": str_prop("Code to analyze") }))),
            td("apex_predict_failures", "Predict potential failures.",
               obj_schema(&["code"], json!({ "code": str_prop("Code to analyze") }))),
            td("apex_full_sweep", "Run a full APEX sweep.",
               obj_schema(&[], json!({}))),
            td("apex_simulate_attack", "Simulate an attack scenario.",
               obj_schema(&["target"], json!({ "target": str_prop("Target") }))),
            td("apex_architect_design", "Architectural design review.",
               obj_schema(&["spec"], json!({ "spec": str_prop("Specification") }))),
            td("apex_quick_check", "Quick APEX check.",
               obj_schema(&[], json!({}))),
            td("apex_pentest_report", "Generate a penetration test report.",
               obj_schema(&["findings"], json!({ "findings": str_prop("Findings") }))),
            td("generate_exploit_artifact",
               "Generate a complete, runnable security artifact (Nuclei template, CVE PoC, JWT cracker, webshell/upload bypass, kernel exploit, recon script) via the BugTrace CORE-Ultra tooling model. Returns ready-to-run code, not prose. Use when the task needs a working exploit/tool rather than analysis.",
               obj_schema(&["task"], json!({
                   "task": str_prop("What artifact to build, e.g. 'Nuclei template for CVE-2021-44228 with interactsh OOB detection'"),
                   "target_context": str_prop("Optional target details (endpoint, version, stack) to specialize the artifact")
               }))),

            // ── Project Rules ──
            td("project_rules", "Get project rules and conventions.",
               obj_schema(&[], json!({}))),
            td("get_command_help", "Get help for a command.",
               obj_schema(&["command"], json!({ "command": str_prop("Command name") }))),
            td("find_api_keys", "Find API keys in the project.",
               obj_schema(&[], json!({}))),
            td("run_command_safe", "Run a command with safety checks.",
               obj_schema(&["command"], json!({ "command": str_prop("Command to run") }))),

            // ── Agentic Workflow ──
            td("ag_get_next_task", "Get the next task from the agentic workflow.",
               obj_schema(&[], json!({}))),
            td("ag_mark_task_done", "Mark a task as done.",
               obj_schema(&["task_id"], json!({ "task_id": str_prop("Task ID") }))),
            td("ag_phase_wrap", "Wrap the current phase.",
               obj_schema(&[], json!({}))),
            td("ag_list_tasks", "List all tasks.",
               obj_schema(&[], json!({}))),
        ]
    }

    /// Canonical tool name mapping (aliases → canonical name).
    pub(crate) fn canonical_tool_name(name: &str) -> &str {
        match name {
            "read_file" | "view_file" => "view_file",
            "edit_file" | "str_replace" | "search_replace_edit" => "search_replace_edit",
            "create_file" | "write_to_file" => "write_to_file",
            "delete_file" | "remove_item" => "remove_item",
            "list_directory" | "list_files" => "list_files",
            "terminal_execute" | "run_command" => "run_command",
            other => other,
        }
    }
}
