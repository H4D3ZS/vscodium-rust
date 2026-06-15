//! Tool schema catalog: list_tools() JSON definitions + canonical naming.
use serde_json::json;
use super::registry::AiTools;
use super::registry::ToolDefinition;

impl AiTools {
    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "view_file".to_string(),
                description: "Read the content of a file".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "write_to_file".to_string(),
                description: "Write content to a file. Overwrites if exists, creates if not."
                    .to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" },
                        "content": { "type": "string", "description": "Content to write" }
                    },
                    "required": ["path", "content"]
                }),
            },
            ToolDefinition {
                name: "str_replace".to_string(),
                description: "Replace an exact string in a file. Reads the file, replaces old_str with new_str, writes back. Use this for surgical code edits.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" },
                        "old_str": { "type": "string", "description": "Exact string to find (must exist in file)" },
                        "new_str": { "type": "string", "description": "Replacement string" }
                    },
                    "required": ["path", "old_str", "new_str"]
                }),
            },
            ToolDefinition {
                name: "fast_apply".to_string(),
                description: "Apply a short Cursor-style edit sketch to a file. Provide ONLY the changed regions plus elision markers (`// ... existing code ...`, `# ... existing code ...`, `<!-- ... -->`, or just `... existing code ...` on its own line) for unchanged regions. The tool deterministically stitches the sketch back into the full file using line-anchor matching. Use this when you want to make targeted edits without re-emitting the whole file and the SEARCH/REPLACE format is too verbose.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path":   { "type": "string", "description": "Relative path to the file" },
                        "edit":   { "type": "string", "description": "Short sketch with new code + elision markers for unchanged regions" },
                        "dry_run": { "type": "boolean", "description": "If true, return merged result without writing", "default": false }
                    },
                    "required": ["path", "edit"]
                }),
            },
            ToolDefinition {
                name: "search_replace_edit".to_string(),
                description: "Surgically edit a file using SEARCH/REPLACE blocks. Format:\n<<<< SEARCH\n<exact existing code>\n====\n<new code>\n>>>>".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" },
                        "content": { "type": "string", "description": "SEARCH/REPLACE block(s)" },
                        "direct_apply": { "type": "boolean", "description": "If true, skip shadow buffer and write immediately to disk", "default": false }
                    },
                    "required": ["path", "content"]
                }),
            },
            ToolDefinition {
                name: "preview_shadow_diff".to_string(),
                description: "Preview the diff of uncommitted (shadow) changes for a specific file.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "apply_shadow_patch".to_string(),
                description: "Commit the staged (shadow) changes for a file to the actual filesystem.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "ghost_test".to_string(),
                description: "Run a command in the background Ghost Runtime to verify changes (e.g. 'cargo test', 'npm test').".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "Command to execute" }
                    },
                    "required": ["command"]
                }),
            },
            ToolDefinition {
                name: "remove_item".to_string(),
                description: "Delete a file or directory recursively".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to delete" },
                        "recursive": { "type": "boolean", "description": "Whether to delete recursively (required for directories)", "default": true }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "create_directory".to_string(),
                description: "Create a new directory (and parents if needed)".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the directory" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "rename_path".to_string(),
                description: "Rename or move a file or directory".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "old_path": { "type": "string", "description": "Current relative path" },
                        "new_path": { "type": "string", "description": "New relative path" }
                    },
                    "required": ["old_path", "new_path"]
                }),
            },
            ToolDefinition {
                name: "list_files".to_string(),
                description: "List files in a directory".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the directory (default: '.')", "default": "." },
                        "recursive": { "type": "boolean", "description": "Whether to list recursively", "default": false }
                    }
                }),
            },
             ToolDefinition {
                 name: "run_command".to_string(),
                 description: "Run a shell command. Supports 'background: true' to run without blocking and return a status ID.".to_string(),
                 input_schema: serde_json::json!({
                     "type": "object",
                     "properties": {
                         "command": { "type": "string", "description": "The command to run" },
                         "background": { "type": "boolean", "description": "Run in background?", "default": false }
                     },
                     "required": ["command"]
                 }),
             },
             ToolDefinition {
                 name: "editor_open_file".to_string(),
                 description: "Open a file in the main editor window.".to_string(),
                 input_schema: serde_json::json!({
                     "type": "object",
                     "properties": {
                         "path": { "type": "string", "description": "Relative path to the file to open" }
                     },
                     "required": ["path"]
                 }),
             },
             ToolDefinition {
                 name: "terminal_terminate".to_string(),
                 description: "Terminate a terminal process by its ID.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "term_id": { "type": "string", "description": "The ID of the terminal to terminate" }
                     },
                     "required": ["term_id"]
                 }),
             },
             ToolDefinition {
                 name: "terminal_get_status".to_string(),
                 description: "Get the execution status and exit code of a terminal process.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "term_id": { "type": "string", "description": "The ID of the terminal to check" }
                     },
                     "required": ["term_id"]
                 }),
             },
             ToolDefinition {
                 name: "terminal_list".to_string(),
                 description: "List all active terminal sessions and their handles.".to_string(),
                 input_schema: json!({ "type": "object", "properties": {} }),
             },
             ToolDefinition {
                 name: "get_system_info".to_string(),
                 description: "Get information about the current system (OS, architecture, user, etc).".to_string(),
                 input_schema: json!({ "type": "object", "properties": {} }),
             },
             ToolDefinition {
                 name: "terminal_get_state".to_string(),
                 description: "Get the current state of terminals, including list of active terminal IDs.".to_string(),
                 input_schema: json!({ "type": "object", "properties": {} }),
             },
             ToolDefinition {
                 name: "terminal_create".to_string(),
                 description: "Create a new terminal instance.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "shell": { "type": "string", "description": "Optional shell path" }
                     }
                 }),
             },
            ToolDefinition {
                name: "grep".to_string(),
                description: "INSTANT ripgrep (rg) search — regex or literal patterns across files. ALWAYS use this instead of shell grep/rg. Supports single files (e.g. index_bundle.js) and directories.".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Regex pattern to search for (alias: pattern)" },
                        "pattern": { "type": "string", "description": "Same as query — regex pattern" },
                        "path": { "type": "string", "description": "File or directory (default: project root)", "default": "." },
                        "include": { "type": "string", "description": "Optional glob filter (e.g. *.js)" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "search_files".to_string(),
                description: "Search for a string across all files in the project".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "The string or regex to search for" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "generate_0day_exploit".into(),
                description: "Build a LIVE exploit scaffold: runs exploit_lookup (searchsploit/NVD), generates reverse-shell/listener payloads, writes PoC report to disk. Not an LLM fantasy PoC.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "target_os": {"type": "string"},
                        "vulnerability_desc": {"type": "string"},
                        "constraints": {"type": "string"}
                    },
                    "required": ["target_os", "vulnerability_desc"]
                }),
            },
            ToolDefinition {
                name: "reverse_engineer_firmware".into(),
                description: "LIVE firmware/binary analysis: get_binary_info + extract_strings + entropy + secrets_scan (+ disassembly at depth≥2). Real file analysis, not simulated.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "firmware_path": {"type": "string"},
                        "target_device": {"type": "string"},
                        "analysis_depth": {"type": "integer", "default": 1}
                    },
                    "required": ["firmware_path"]
                }),
            },
            ToolDefinition {
                name: "network_scan".into(),
                description: "LIVE network recon: runs real nmap (or curl/ping fallback) against the target. Returns actual command stdout/stderr.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "target": {"type": "string", "description": "The target IP or hostname"},
                        "intensity": {"type": "string", "enum": ["light", "normal", "aggressive"], "default": "normal"}
                    },
                    "required": ["target"]
                }),
            },
            ToolDefinition {
                name: "exploit_lookup".into(),
                description: "LIVE exploit/CVE lookup via searchsploit or NVD API curl. Returns real search results, not fake EDB rows.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {"type": "string", "description": "Search query (e.g., 'vsftpd 2.3.4' or 'CVE-2021-44228')"}
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "advanced_reverse_engineering".into(),
                description: "LIVE binary RE chain (alias of reverse_engineer_firmware): metadata, strings, entropy, secrets, optional disassembly.".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "binary_path": {"type": "string"},
                        "analysis_depth": {"type": "integer", "default": 2}
                    },
                    "required": ["binary_path"]
                }),
            },
            ToolDefinition {
                name: "terminal_send_data".into(),
                description: "Send raw data/commands to the active terminal session".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "data": {"type": "string", "description": "The string/command to send (include \\n for enter)"},
                        "term_id": {"type": "string", "description": "The ID of the terminal to send to (optional, defaults to first)"}
                    },
                    "required": ["data"]
                }),
            },
            ToolDefinition {
                name: "terminal_read_output".into(),
                description: "Read the character buffer of a specific terminal session".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "term_id": {"type": "string", "description": "Terminal ID to read from"}
                    },
                    "required": ["term_id"]
                }),
            },
            ToolDefinition {
                name: "terminal_toggle".into(),
                description: "Toggle terminal panel visibility in the IDE".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "visible": {"type": "boolean", "description": "true to show, false to hide"}
                    },
                    "required": ["visible"]
                }),
            },
            ToolDefinition {
                name: "browser_capture_vision_context".into(),
                description: "Capture the current browser state (URL, Title, Screenshot, simplified DOM)".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "browser_open".into(),
                description: "Open a new headless browser instance".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "browser_navigate".into(),
                description: "Navigate the browser to a URL".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": {"type": "string", "description": "The URL to navigate to"}
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "browser_screenshot".into(),
                description: "Capture a screenshot of the current page and return as base64".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "browser_click".into(),
                description: "Click an element on the page using a CSS selector".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "selector": {"type": "string", "description": "The CSS selector to click"}
                    },
                    "required": ["selector"]
                }),
            },
            ToolDefinition {
                name: "browser_type".into(),
                description: "Type text into an element on the page using a CSS selector".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "selector": {"type": "string", "description": "The CSS selector to type into"},
                        "text": {"type": "string", "description": "The text to type"}
                    },
                    "required": ["selector", "text"]
                }),
            },
            ToolDefinition {
                name: "browser_read_dom".into(),
                description: "Read the full HTML content of the current page".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "browser_close".into(),
                description: "Close the headless browser instance".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "editor_open_file".into(),
                description: "Open a file in the main editor".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "The relative path to the file to open"}
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "editor_get_active_file".into(),
                description: "Get the path of the currently active/focused file in the editor".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "replace_file_content".into(),
                description: "Replace a specific string in a file with a new string. Use for precise edits.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "Relative path to the file"},
                        "target": {"type": "string", "description": "The exact string to replace"},
                        "replacement": {"type": "string", "description": "The replacement string"}
                    },
                    "required": ["path", "target", "replacement"]
                }),
            },
            ToolDefinition {
                name: "multi_replace_file_content".into(),
                description: "Perform multiple non-contiguous replacements in a single file.".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "Relative path to the file"},
                        "replacements": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "target": {"type": "string"},
                                    "replacement": {"type": "string"}
                                },
                                "required": ["target", "replacement"]
                            }
                        }
                    },
                    "required": ["path", "replacements"]
                }),
            },
            ToolDefinition {
                name: "find_by_name".into(),
                description: "Find files by name pattern (case-insensitive glob).".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": {"type": "string", "description": "The glob pattern to search for (e.g. *.tsx)"},
                        "path": {"type": "string", "description": "Directory to search in (default: '.')", "default": "."}
                    },
                    "required": ["pattern"]
                }),
            },
            ToolDefinition {
                name: "get_directory_structure".into(),
                description: "Get a high-level overview of the directory structure (recursive with depth).".into(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": {"type": "string", "description": "Path to explore", "default": "."},
                        "depth": {"type": "integer", "description": "Maximum depth (default: 2)", "default": 2}
                    }
                }),
            },
            ToolDefinition {
                name: "find_api_keys".into(),
                description: "Search the codebase for leaked API keys using common dorking patterns (OpenAI, GitHub, Google)".into(),
                input_schema: json!({"type": "object", "properties": {}}),
            },
            ToolDefinition {
                name: "analyze_file_symbols".to_string(),
                description: "Extract classes, functions, interfaces, and variables from a source file for semantic analysis.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "read_url_content".to_string(),
                description: "Lite web fetcher that returns the content of a URL as markdown-friendly text.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "The URL to read" }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "perplexity_reason".to_string(),
                description: "Deep reasoning tool grounded in real-time web research and documentation.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "The question or research task" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "perplexity_ask".to_string(),
                description: "Direct Q&A tool for real-time documentation and web queries.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "The question to ask" }
                    },
                    "required": ["query"]
                }),
            },
             ToolDefinition {
                 name: "spawn_subagent".to_string(),
                 description: "Spawn a specialized sub-agent to handle a sub-task. USE THIS FOR CONCURRENT RESEARCH OR COMPLEX SUB-STEPS.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "task": { "type": "string", "description": "The specific task for the sub-agent" }
                     },
                     "required": ["task"]
                 }),
             },
             ToolDefinition {
                 name: "explore_repository".to_string(),
                 description: "FastContext repository explorer — lightweight subagent that does parallel READ/GLOB/GREP and returns compact file citations. Use this INSTEAD of doing your own exploration when you need to find relevant code across a large codebase. Returns file paths, line ranges, and key snippets. Much faster and cheaper than manual exploration.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "query": { "type": "string", "description": "What to find in the repository (e.g. 'authentication middleware', 'error handling in parser')" },
                         "max_results": { "type": "number", "description": "Maximum number of file citations to return (default 10)", "default": 10 },
                         "file_pattern": { "type": "string", "description": "Optional glob pattern to scope search (e.g. '*.rs', 'src/**/*.ts')" }
                     },
                     "required": ["query"]
                 }),
             },
             ToolDefinition {
                 name: "generate_image".to_string(),
                 description: "Generate a custom UI asset or image based on a prompt.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "prompt": { "type": "string", "description": "The description of the image to generate" },
                         "path": { "type": "string", "description": "Local path to save the generated image" }
                     },
                     "required": ["prompt", "path"]
                 }),
             },
             ToolDefinition {
                 name: "analyze_image".to_string(),
                 description: "Analyze an image file and return structural or semantic information.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "path": { "type": "string", "description": "Path to the image file" },
                         "question": { "type": "string", "description": "Specific question about the image (optional)" }
                     },
                     "required": ["path"]
                 }),
             },
             ToolDefinition {
                 name: "code_search".to_string(),
                 description: "Perform a high-speed global text search across the codebase.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "query": { "type": "string", "description": "The search term" },
                         "file_pattern": { "type": "string", "description": "Optional glob pattern for files (e.g., *.rs)" }
                     },
                     "required": ["query"]
                 }),
             },
             ToolDefinition {
                 name: "dependency_graph".to_string(),
                 description: "Map the dependencies (imports/exports) of a specific file or directory.".to_string(),
                 input_schema: json!({
                     "type": "object",
                     "properties": {
                         "path": { "type": "string", "description": "The file or directory path" }
                     },
                     "required": ["path"]
                 }),
             },
            ToolDefinition {
                name: "browser_subagent".to_string(),
                description: "One-shot autonomous orchestrator for multi-step browser tasks.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "task": { "type": "string", "description": "High-level goal for the browser (e.g., 'Find the latest Rust async patterns')" }
                    },
                    "required": ["task"]
                }),
            },
            ToolDefinition {
                name: "get_symbol_graph".to_string(),
                description: "Uses LSP to find all usages and definitions of a function or class. The agent understands the impact of a change project-wide.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "symbol": { "type": "string", "description": "Name of the function or class to analyze" },
                        "path": { "type": "string", "description": "Relative path to the file containing the symbol" }
                    },
                    "required": ["symbol", "path"]
                }),
            },
            ToolDefinition {
                name: "run_command_safe".to_string(),
                description: "A Rust-native command wrapper that maps Unix commands to Windows-native equivalents (e.g., ls -> dir, rm -> del).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "The command string to execute (Unix-style allowed)" }
                    },
                    "required": ["command"]
                }),
            },
            ToolDefinition {
                name: "verify_implementation".to_string(),
                description: "Spawns a background cargo check, cargo test, or npm test. The agent knows if it broke the build before the user sees the code.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "Verification command (default: 'cargo check')", "default": "cargo check" }
                    }
                }),
            },
            ToolDefinition {
                name: "create_mission_plan".to_string(),
                description: "Forces the AI to write a Markdown checklist first. Visualizes the 'Thought Process' in real-time.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "plan": { "type": "string", "description": "A Markdown checklist of steps to accomplish the task" }
                    },
                    "required": ["plan"]
                }),
            },
            ToolDefinition {
                name: "revert_checkpoint".to_string(),
                description: "Restores files from the last known-good state in .hades_cache if the current implementation failed verification.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file to revert" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "patch_file_content".to_string(),
                description: "Precisely edit a file by replacing a specific line range with new content".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" },
                        "StartLine": { "type": "integer", "description": "1-indexed starting line" },
                        "EndLine": { "type": "integer", "description": "1-indexed ending line" },
                        "ReplacementContent": { "type": "string", "description": "The new content" }
                    },
                    "required": ["path", "StartLine", "EndLine", "ReplacementContent"]
                }),
            },
            ToolDefinition {
                name: "ai_propose_edit".to_string(),
                description: "Propose a full-file code modification for user review. Does NOT write to disk immediately.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" },
                        "new_content": { "type": "string", "description": "The proposed full content of the file" },
                        "description": { "type": "string", "description": "Brief summary of the changes for the user" }
                    },
                    "required": ["path", "new_content"]
                }),
            },
            ToolDefinition {
                name: "git_status".to_string(),
                description: "View the status of the git repository (staged, unstaged, untracked files).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            ToolDefinition {
                name: "git_add".to_string(),
                description: "Stage a file or directory for commit.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "The path to the file or directory to stage." }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "git_commit".to_string(),
                description: "Commit staged changes with a message.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "message": { "type": "string", "description": "The commit message." }
                    },
                    "required": ["message"]
                }),
            },
            ToolDefinition {
                name: "git_diff".to_string(),
                description: "View changes in the working directory, staged area, or for a specific commit hash.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Optional path to show diff for." },
                        "staged": { "type": "boolean", "description": "Whether to show staged changes.", "default": false },
                        "hash": { "type": "string", "description": "Optional commit hash to show the diff for." }
                    }
                }),
            },
            ToolDefinition {
                name: "git_log".to_string(),
                description: "View the git commit history.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "limit": { "type": "integer", "description": "Limit the number of commits shown.", "default": 10 }
                    }
                }),
            },
            ToolDefinition {
                name: "get_system_health".to_string(),
                description: "Check the health and status of system dependencies (Git, Node, Rust, MCP).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {}
                }),
            },
            ToolDefinition {
                name: "task_boundary".to_string(),
                description: "Signal the start or update of a task. Use this to report progress to the UI.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "TaskName": { "type": "string", "description": "Human readable name of the task" },
                        "Mode": { "type": "string", "description": "PLANNING, EXECUTION, or VERIFICATION" },
                        "TaskSummary": { "type": "string", "description": "Concise summary of accomplished work" },
                        "TaskStatus": { "type": "string", "description": "What you are going to do next" },
                        "PredictedTaskSize": { "type": "integer", "description": "Estimated steps remaining" }
                    },
                    "required": ["TaskName", "Mode", "TaskSummary", "TaskStatus", "PredictedTaskSize"]
                }),
            },
            ToolDefinition {
                name: "create_canvas".to_string(),
                description: "Render an interactive visual canvas (dashboard) in the IDE instead of a wall of text. Use for data-heavy results: scan findings, audits, comparisons, progress, metrics, plans. Block types: stats, table, chart (bar|line|pie), callout, progress, todo, kv, timeline, markdown, code. Reuse the same 'id' to update a canvas in place.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "id": { "type": "string", "description": "Stable canvas id (slug). Reuse to update in place. Omit to derive from title." },
                        "title": { "type": "string", "description": "Dashboard heading / tab name" },
                        "subtitle": { "type": "string", "description": "Optional one-line context" },
                        "blocks": {
                            "type": "array",
                            "description": "Ordered block objects. e.g. {\"type\":\"stats\",\"items\":[{\"label\":\"Critical\",\"value\":3,\"tone\":\"danger\"}]}, {\"type\":\"table\",\"columns\":[...],\"rows\":[[...]]}, {\"type\":\"chart\",\"chart\":\"bar\",\"labels\":[...],\"series\":[{\"values\":[...]}]}",
                            "items": { "type": "object" }
                        }
                    },
                    "required": ["title", "blocks"]
                }),
            },
            ToolDefinition {
                name: "notify_user".to_string(),
                description: "Communicate with the user. Can be used to ask questions or request reviews.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "Message": { "type": "string", "description": "The message to the user" },
                        "BlockedOnUser": { "type": "boolean", "description": "Whether to pause and wait for user response", "default": false },
                        "PathsToReview": { "type": "array", "items": { "type": "string" }, "description": "Optional file paths for review" }
                    },
                    "required": ["Message"]
                }),
            },
            ToolDefinition {
                name: "use_skill".to_string(),
                description: "Load and activate a specific skill from the .agent/skills library.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "SkillName": { "type": "string", "description": "The name of the skill to use (e.g. 'rust-pro')" }
                    },
                    "required": ["SkillName"]
                }),
            },
            ToolDefinition {
                name: "search_skills".to_string(),
                description: "Search for available skills in the .agent/skills library based on keywords.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "Query": { "type": "string", "description": "Keywords to search for (e.g. 'react', 'postgres')" }
                    },
                    "required": ["Query"]
                }),
            },
            ToolDefinition {
                name: "save_knowledge_brief".to_string(),
                description: "Proactively save session findings or architectural decisions to the persistent brain (.kortex/knowledge). section 318".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "title": { "type": "string", "description": "Short mnemonic title for the finding" },
                        "date": { "type": "string", "description": "Current date/time" },
                        "findings": { "type": "string", "description": "Detailed explanation of the solution or architecture" },
                        "affected_files": { "type": "array", "items": { "type": "string" }, "description": "Paths involved" }
                    },
                    "required": ["title", "date", "findings", "affected_files"]
                }),
            },
            ToolDefinition {
                name: "verify_claim".to_string(),
                description: "Verify an AI claim against stored project knowledge and source code symbols. Use this before making factual statements to prevent hallucinations.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "claim": { "type": "string", "description": "The factual claim to verify (e.g. 'the function foo takes 3 arguments')" }
                    },
                    "required": ["claim"]
                }),
            },
            ToolDefinition {
                name: "see_the_screen".to_string(),
                description: "Proactively capture a screenshot of the IDE preview to visually verify UI changes or layout issues. section 318".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {},
                    "required": []
                }),
            },
            ToolDefinition {
                name: "semantic_search".to_string(),
                description: "Semantic @codebase search: AIM memory slots plus ranked vector index chunks (run Index in Settings → Cursor Parity first).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Keyword or topic to search for" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "aim_pack_context".to_string(),
                description: "Load the entire pre-indexed codebase as a compact semantic map (~6 gist tokens). \
                              Call this ONCE at the start of any task involving an unfamiliar codebase. \
                              Returns the project structure, key symbols, and indexed file summaries. \
                              ZERO-GREP MODE: after calling this you do NOT need list_files or grep to orient yourself.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Optional: focus the context on a specific topic" },
                        "max_slots": { "type": "integer", "description": "Max number of memory slots to return (default 20)" }
                    }
                }),
            },
            ToolDefinition {
                name: "aim_query_spans".to_string(),
                description: "Find the exact file and line number for a function, class, or concept using the AIM index. \
                              Faster and more precise than grep — uses the pre-built codebase knowledge graph. \
                              Returns file paths, line numbers, and code previews for each match.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Symbol name, concept, or code pattern to locate" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "find_symbols".to_string(),
                description: "List all indexed code symbols (functions, classes) found in the project.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "pattern": { "type": "string", "description": "Optional substring to filter symbols" }
                    }
                }),
            },
            ToolDefinition {
                name: "read_file_lines".to_string(),
                description: "Read a specific line range from a file. Essential for large source files.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to file" },
                        "start_line": { "type": "integer", "description": "1-indexed start line" },
                        "end_line": { "type": "integer", "description": "1-indexed end line" }
                    },
                    "required": ["path", "start_line", "end_line"]
                }),
            },
            ToolDefinition {
                name: "reindex_project".to_string(),
                description: "Force a full background re-indexing of the current project structure and symbols.".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "list_dir_tree".to_string(),
                description: "Get a formatted tree view of the project structure (max depth 3).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to root directory", "default": "." }
                    }
                }),
            },
            ToolDefinition {
                name: "list_mcp_ops".to_string(),
                description: "List all available MCP server tools and their specific capabilities.".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "hex_dump".to_string(),
                description: "Generate a hex dump of a binary file for low-level research.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to binary" },
                        "offset": { "type": "integer", "description": "Starting byte offset", "default": 0 },
                        "length": { "type": "integer", "description": "Number of bytes (default 256)", "default": 256 }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "extract_strings".to_string(),
                description: "Extract printable ASCII strings from a binary file (min length 4).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "list_active_processes".to_string(),
                description: "List currently running system processes (PID and Name).".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "apply_patch".to_string(),
                description: "Apply a unified diff / patch to a file for complex changes.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to file" },
                        "patch": { "type": "string", "description": "Unified diff format patch content" }
                    },
                    "required": ["path", "patch"]
                }),
            },
            ToolDefinition {
                name: "get_file_metadata".to_string(),
                description: "Get detailed file metadata (size, mod time, permissions).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "ide_get_state".to_string(),
                description: "Get the current IDE state (active file, open tabs, active terminal).".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "network_port_scanner".to_string(),
                description: "Perform a native TCP port scan on a target host.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target": { "type": "string", "description": "IP or logical hostname" },
                        "ports": { "type": "array", "items": { "type": "integer" }, "description": "List of ports to scan" }
                    },
                    "required": ["target", "ports"]
                }),
            },
            ToolDefinition {
                name: "binary_mach_o_scanner".to_string(),
                description: "Deep scanner for Mach-O binaries (XNU Kernel / MacOS) to find sections and symbols.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to target binary" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "secrets_scan".to_string(),
                description: "Red-team / blue-team secrets sweep. Recursively scan a file or directory for hard-coded credentials, API keys, tokens, JWTs, private keys, cloud credentials, DSNs, and database URLs. Returns structured findings: {type, severity, path, line, redacted_preview}. Use during security audits, pre-commit checks, leak hunts, and bug bounty recon.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File or directory path (relative to workspace). Use '.' to scan everything." },
                        "max_findings": { "type": "integer", "description": "Maximum number of findings to return (default 200)" },
                        "include_low": { "type": "boolean", "description": "Include LOW-severity heuristic matches (default false)" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "deep_security_audit".to_string(),
                description: "Deep multi-pass security audit of a file or directory. Runs a secrets sweep, a heuristic CWE source scan (OS command injection, SQLi, XSS, unsafe deserialization, weak crypto/randomness, disabled TLS verification, path traversal, eval, unsafe blocks, panic-prone unwrap/expect), and a dependency-posture check. Consolidates everything into CWE-tagged findings {id, title, severity, cwe, category, path, line, evidence, remediation, confidence}, sorts by severity, and writes a Markdown report to reports/. Use this FIRST for any 'audit my code', 'find vulnerabilities', blue-team, or bug-bounty recon request — it is fast (filesystem-only) and gives a structured starting point.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File or directory to audit (relative to workspace). Use '.' for the whole project. Default '.'." },
                        "depth": { "type": "string", "enum": ["standard", "deep"], "description": "'deep' also includes INFO-level findings (unwrap/expect) and LOW-severity secrets. Default 'standard'." },
                        "write_report": { "type": "boolean", "description": "Write a Markdown report to reports/ (default true)." }
                    }
                }),
            },
            ToolDefinition {
                name: "web_security_audit".to_string(),
                description: "DYNAMIC web-application security audit against a LIVE url (authorized pentest / bug-bounty only). Drives the stealth browser to fetch the target, then audits: response security headers (CSP/HSTS/X-Frame-Options/X-Content-Type-Options/Referrer-Policy/Permissions-Policy), information-disclosure headers (Server/X-Powered-By/version banners), cookie flags (Secure/HttpOnly/SameSite), HTML forms (password-over-HTTP, GET-with-credentials, missing CSRF token), and mixed content (http resources on an https page). Consolidates into CWE-tagged findings {id, title, severity, cwe, category, path, evidence, remediation, confidence}, sorts by severity, and writes a Markdown report to reports/. Use for 'audit this website/url', web pentest, and BugBounty recon of a live target (complements deep_security_audit, which is static/code-only).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Target URL to audit (http/https). Authorized targets only." },
                        "write_report": { "type": "boolean", "description": "Write a Markdown report to reports/ (default true)." }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "ai_vuln_hunt".to_string(),
                description: "AI vulnerability-hunting PIPELINE (3-stage, tiered models) — the HackerOne #1-KR methodology, the strongest white-box bug-finder in the toolbox. Stage 1 chunks the codebase into analysis units; stage 2 runs a CHEAP model over every chunk for high-recall candidate generation (missing authz/IDOR, injection, SSRF, input-handling, business-logic); stage 3 validates in two passes — a MID model culls obvious false positives, then a STRONG (security-specialized if available) model confirms each real bug and writes severity/CWE/impact/PoC/remediation, honestly flagging policy_dependent and cross_component cases. Model tiers auto-resolve from installed Ollama + keyed cloud models (overridable). Writes a CWE-tagged Markdown report to reports/ and returns structured findings. Use this FIRST for 'find vulnerabilities / audit this repo / bug-bounty recon' on source you can read (complements deep_security_audit which is pattern-only, and web_security_audit which is for live URLs).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "File or directory to hunt (relative to workspace). Use '.' for the whole project. Default '.'." },
                        "cheap_model": { "type": "string", "description": "Override the high-recall stage-2 model, as 'provider|model' or bare model (defaults to ollama). Optional — auto-detected if omitted." },
                        "mid_model": { "type": "string", "description": "Override the false-positive-filter (stage 3a) model. Optional." },
                        "strong_model": { "type": "string", "description": "Override the final confirm (stage 3b) model. Optional." },
                        "chunk_bytes": { "type": "integer", "description": "Max bytes per analysis chunk (default 24000). Smaller = more calls but finer detail." },
                        "max_files": { "type": "integer", "description": "Cap files scanned (cost control, default 400)." },
                        "max_chunks": { "type": "integer", "description": "Cap chunks analyzed (cost control, default 60)." },
                        "write_report": { "type": "boolean", "description": "Write a Markdown report to reports/ (default true)." }
                    }
                }),
            },
            ToolDefinition {
                name: "vega_dast_scan".to_string(),
                description: "DYNAMIC web vuln scan against a LIVE url (authorized pentest / bug-bounty only) using the native Vega DAST engine. Crawls the target (BFS), then runs passive + active injection modules (SQLi, XSS, command injection, SSRF, path traversal, etc.) over discovered parametric paths with differential detection. Returns {target, paths_scanned, modules_run, alerts:[{type_key,title,severity,resource,output}], duration_ms}. Fully offline (no cloud). Set ai_triage:true to add local-LLM false-positive verdicts. Use for live web pentest / bounty triage; complements ai_vuln_hunt (static source) and chunk_secret_scan (bundle leaks).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Target URL (http/https). Authorized targets only." },
                        "max_pages": { "type": "integer", "description": "Crawl page cap (default 24)." },
                        "max_depth": { "type": "integer", "description": "Crawl depth (default 2)." },
                        "ai_triage": { "type": "boolean", "description": "Run local-LLM false-positive triage on findings (default false; degrades to heuristic offline)." }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "chunk_secret_scan".to_string(),
                description: "Scan JavaScript bundles / webpack-vite chunks / source maps for leaked secrets (API keys, tokens, OpenAI/Anthropic keys, DSNs, cloud creds). Works on a LIVE url (fetches HTML, extracts + fetches script chunks and .map files) OR a local path (parallel filesystem scan). Returns {files_scanned, bytes_scanned, source_maps_found, findings:[{kind,severity,bounty_hint,file,preview}]}. Rust-native, fully offline for path mode. Use for bounty recon on deployed front-ends and pre-deploy leak checks.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Live origin URL to fetch + scan its JS chunks. Provide this OR path." },
                        "path": { "type": "string", "description": "Local file/dir to scan (relative to workspace). Provide this OR url." },
                        "max_files": { "type": "integer", "description": "Path-mode file cap (default 2000)." }
                    }
                }),
            },
            ToolDefinition {
                name: "bounty_scan".to_string(),
                description: "Combined one-shot bounty recon on a LIVE url (authorized only): runs the chunk/source-map secret scanner AND a native XSS reflection probe on URL parameters, returning {chunk:{...}, xss:{hits:[...]}}. Fastest way to triage a deployed target for high-signal bounty findings before deeper Vega/Moxy runs. Rust-native, no cloud.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "Target origin URL (http/https). Authorized targets only." },
                        "include_xss": { "type": "boolean", "description": "Also run the native XSS reflection probe (default true)." }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "oast_payload".to_string(),
                description: "Mint an out-of-band (OAST/Collaborator) callback payload to catch BLIND vulnerabilities — blind SSRF, blind RCE, blind XXE, blind XSS, DNS/HTTP exfil. Returns {token, http_url, authority}. Inject the http_url (or authority for non-HTTP probes) into the target, then call oast_interactions with the token to see if the target called back. Requires the OAST server to be running (auto-starts on first use). Authorized testing only.".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "oast_interactions".to_string(),
                description: "Poll the OAST/Collaborator server for callbacks (interactions). Pass the token from oast_payload to check one probe, or omit to list all. Returns interactions [{token, protocol, remote_addr, method, path, host_header, user_agent, timestamp_ms}]. A non-empty result for your token CONFIRMS a blind vulnerability (the target reached your callback host). Authorized testing only.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "token": { "type": "string", "description": "Correlation token from oast_payload. Omit to list all interactions." }
                    }
                }),
            },
            ToolDefinition {
                name: "weaponize_env".to_string(),
                description: "Red-team weaponization assessment of a .env / env-export file. Parses KEY=VALUE pairs, classifies each variable (secret / endpoint / telemetry / runtime), and produces a structured weaponization plan: which secrets are immediately actionable (DB URLs, admin passwords, API tokens, Sentry/OTLP DSNs), which endpoints are pivot targets, what the blast radius is, and what an attacker would do next. Pair with `secrets_scan` for full coverage. Output is JSON suitable for the agent to drive follow-up actions.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to a .env file or shell-export style file (relative to workspace)" },
                        "raw": { "type": "string", "description": "Alternative to `path`: raw env content as a string" }
                    }
                }),
            },
            ToolDefinition {
                name: "reverse_shell_generate".to_string(),
                description: "Generate one-liner reverse shell payloads for authorized pentest labs. Supports bash, python, powershell, php, ruby, nc, node, go, rust, java, csharp, and more. Template only — no network I/O.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "language": { "type": "string", "description": "Target language/runtime (bash, python, powershell, php, …)" },
                        "host": { "type": "string", "description": "Attacker/listener IP or hostname (LHOST)" },
                        "port": { "type": "integer", "description": "Listener port (LPORT)" },
                        "shell": { "type": "string", "description": "Optional shell binary path (/bin/bash, cmd.exe, …)" }
                    },
                    "required": ["host", "port"]
                }),
            },
            ToolDefinition {
                name: "security_listener_generate".to_string(),
                description: "Generate listener commands for authorized red-team labs: netcat, ncat, socat, Metasploit handler, pwncat.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "kind": { "type": "string", "description": "nc, ncat, socat_tcp, socat_udp, msf, pwncat" },
                        "host": { "type": "string", "description": "Bind address (0.0.0.0 default)" },
                        "port": { "type": "integer", "description": "Listen port" }
                    },
                    "required": ["port"]
                }),
            },
            ToolDefinition {
                name: "csp_bypass_analyze".to_string(),
                description: "Parse a Content-Security-Policy header and return directive breakdown, weaknesses, and bypass research angles (JSONP, unsafe-inline, base-uri, etc.).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "header": { "type": "string", "description": "Raw CSP header value" },
                        "csp": { "type": "string", "description": "Alias for header" }
                    }
                }),
            },
            ToolDefinition {
                name: "shellcode_recipe_generate".to_string(),
                description: "Return msfvenom / assembly recipes for shellcode generation on authorized lab machines. Does not emit executable shellcode bytes.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "platform": { "type": "string", "description": "windows, linux, osx" },
                        "arch": { "type": "string", "description": "x64, x86, elf, macho" },
                        "payload": { "type": "string", "description": "Metasploit payload name (default shell_reverse_tcp)" }
                    }
                }),
            },
            ToolDefinition {
                name: "payload_encode".to_string(),
                description: "Encode a payload string for authorized testing: base64, URL, hex, or double-URL encoding.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "payload": { "type": "string", "description": "Raw payload text" },
                        "encoding": { "type": "string", "description": "base64, url, hex, double_url" }
                    },
                    "required": ["payload", "encoding"]
                }),
            },
            ToolDefinition {
                name: "sec_distro_inventory".to_string(),
                description: "Detect Kali Linux / Parrot OS / Debian security distro and probe PATH for curated offensive tools (nmap, nuclei, sqlmap, ffuf, bloodhound, impacket, anonsurf, etc.). Call at the START of every red-team / bug-bounty engagement on GNU/Linux — then use available native tools via run_command instead of reinventing scripts. Returns JSON: distro, tools_available by category, tools_missing, install hints.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "category": {
                            "type": "string",
                            "description": "Optional filter: recon_osint, web_app, network_sniff, wireless, exploitation, credentials, post_exploit_ad, reverse_engineering, parrot_privacy_ops, container_cloud"
                        }
                    }
                }),
            },
            ToolDefinition {
                name: "file_entropy_analysis".to_string(),
                description: "Calculate bit-level entropy to detect packed or encrypted sections (Malware Research).".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Path to target file" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "dev_cargo_diagnostics".to_string(),
                description: "Run 'cargo check' and return high-fidelity structured error messages. Call this after editing any Rust file to verify correctness before moving on.".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "search_codebase".to_string(),
                description: "Search the entire codebase for a query: returns matching file paths, line numbers, snippet previews, AND any matching symbols (functions/structs/classes). Faster than grep for symbol-level queries.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Text or symbol name to search for" },
                        "file_types": { "type": "string", "description": "Optional comma-separated extensions to limit search, e.g. 'rs,ts,tsx'" },
                        "max_results": { "type": "integer", "description": "Max results to return (default 30)" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "web_search".to_string(),
                description: "Search the web for information. Returns instant answers, summaries, and related links. Use for: API documentation, error messages, library usage, current events, technical questions.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "Search query (e.g. 'rust tokio spawn timeout example')" },
                        "num_results": { "type": "integer", "description": "Max results to return (1-10, default 5)" }
                    },
                    "required": ["query"]
                }),
            },
            ToolDefinition {
                name: "web_fetch".to_string(),
                description: "Fetch a URL and return its body as text (HTTP GET). Use for reading pages, APIs, robots.txt, JS bundles, etc.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "URL to fetch" }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "security_scan".to_string(),
                description: "Run a deep static analysis (semgrep) on a file or directory to find security vulnerabilities.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to scan" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "audit_dependencies".to_string(),
                description: "Scan the project dependencies for known vulnerabilities (cargo audit).".to_string(),
                input_schema: json!({ "type": "object", "properties": {} }),
            },
            ToolDefinition {
                name: "disassemble".to_string(),
                description: "Disassemble a binary file using radare2 or objdump. Essential for reverse engineering.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to binary" }
                    },
                    "required": ["path"]
                }),
            },
            ToolDefinition {
                name: "get_binary_info".to_string(),
                description: "Get metadata about a binary file (architecture, type) using system 'file' command.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to binary" }
                    },
                    "required": ["path"]
                }),
            },
            // ═══════════════════════════════════════════════════════════════
            // APEX INTELLIGENCE FRAMEWORK TOOLS
            // ═══════════════════════════════════════════════════════════════
            ToolDefinition {
                name: "apex_red_team_scan".to_string(),
                description: "Run a BugTraceAI-Apex red team security scan on code. Uses the uncensored BugTraceAI-Apex-G4-26B offensive security model to find vulnerabilities, generate exploit chains, and map to MITRE ATT&CK. Use for: penetration testing, vulnerability assessment, security audit of any code.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "The source code to scan" },
                        "file_path": { "type": "string", "description": "Path of the file being scanned" },
                        "language": { "type": "string", "description": "Programming language (rust, python, javascript, etc.)" },
                        "depth": { "type": "string", "description": "Scan depth: quick, standard, or deep" }
                    },
                    "required": ["code", "file_path", "language"]
                }),
            },
            ToolDefinition {
                name: "apex_scan_url".to_string(),
                description: "Scan a live website URL for security vulnerabilities. Fetches the page, analyzes headers, forms, cookies, JavaScript, and generates attack vectors. Use when the user asks to hack, pentest, or audit a website or URL.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "url": { "type": "string", "description": "The URL to scan (e.g. https://example.com)" },
                        "depth": { "type": "string", "description": "Scan depth: quick, standard, or deep" }
                    },
                    "required": ["url"]
                }),
            },
            ToolDefinition {
                name: "apex_threat_anticipate".to_string(),
                description: "Predict FUTURE vulnerabilities that don't exist yet. Simulates high traffic, hostile networks, supply chain attacks, and insider threats against the provided code. Use when discussing security architecture, threat modeling, or asking 'what could go wrong'.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "Code to analyze for future threats" },
                        "context": { "type": "string", "description": "Context about the system (scale, users, deployment)" }
                    },
                    "required": ["code", "context"]
                }),
            },
            ToolDefinition {
                name: "apex_perf_optimize".to_string(),
                description: "Analyze code for performance bottlenecks and suggest optimizations. Identifies O(n²) loops, excessive memory allocation, cache-unfriendly patterns, and provides optimized replacement code. Use when code is slow or user asks to optimize.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "Code to optimize" },
                        "language": { "type": "string", "description": "Programming language" }
                    },
                    "required": ["code", "language"]
                }),
            },
            ToolDefinition {
                name: "apex_self_improve".to_string(),
                description: "Iteratively improve code through multiple AI passes. Takes code through 3-5 refinement rounds for correctness, security, performance, and readability. Returns version history showing the evolution.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "Code to iteratively improve" },
                        "language": { "type": "string", "description": "Programming language" },
                        "iterations": { "type": "integer", "description": "Number of improvement passes (1-5, default 3)" }
                    },
                    "required": ["code", "language"]
                }),
            },
            ToolDefinition {
                name: "apex_security_explain".to_string(),
                description: "Explain a security vulnerability and its fix in plain English. Provides: what the vulnerability was, why it's dangerous, how the fix works, related CVEs, and prevention tips. Use after fixing security issues to educate.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "vulnerability": { "type": "string", "description": "Description of the vulnerability" },
                        "fix_diff": { "type": "string", "description": "The diff/code change that fixes it" }
                    },
                    "required": ["vulnerability", "fix_diff"]
                }),
            },
            ToolDefinition {
                name: "apex_predict_failures".to_string(),
                description: "Predict system failures before they happen. Analyzes code and optional runtime logs to predict OOM crashes, deadlocks, connection pool exhaustion, and cascading failures with probability scores and time horizons.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "Code or system description to analyze" },
                        "logs": { "type": "string", "description": "Optional runtime/server logs for pattern analysis" }
                    },
                    "required": ["code"]
                }),
            },
            ToolDefinition {
                name: "apex_full_sweep".to_string(),
                description: "Run ALL APEX intelligence engines in parallel on a target. Performs red team scan + performance analysis + failure prediction simultaneously. Use for comprehensive security and quality assessment.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "code": { "type": "string", "description": "Code to analyze" },
                        "file_path": { "type": "string", "description": "File path" },
                        "language": { "type": "string", "description": "Programming language" }
                    },
                    "required": ["code", "file_path", "language"]
                }),
            },
            ToolDefinition {
                name: "apex_simulate_attack".to_string(),
                description: "Execute a LIVE attack chain against an authorized target — runs real curl/nmap/web_security_audit/deep_security_audit probes and returns actual command output (NOT an LLM simulation). Use for sqli, xss, ssrf, lfi, rce, auth_bypass on in-scope bug-bounty/pentest targets.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "target": { "type": "string", "description": "Target code, endpoint, or system description" },
                        "attack_type": { "type": "string", "description": "Attack type: sqli, xss, rce, csrf, ssrf, lfi, auth_bypass, etc." }
                    },
                    "required": ["target", "attack_type"]
                }),
            },
            ToolDefinition {
                name: "apex_architect_design".to_string(),
                description: "Design a complete system architecture from a project description. Chooses optimal frontend, backend, database, and infrastructure. Use when starting a new project or planning architecture.".to_string(),
                input_schema: json!({
                    "type": "object",
                    "properties": {
                        "description": { "type": "string", "description": "Description of the project to architect" }
                    },
                    "required": ["description"]
                }),
            },
        ]
    }

    /// Map frontend / model alias names to canonical backend handlers.
    pub(crate) fn canonical_tool_name(name: &str) -> &str {
        crate::tool_aliases::canonical_tool_name(name)
    }

}
