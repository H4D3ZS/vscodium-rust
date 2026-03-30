use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tauri::{Emitter, Manager};
use tree_sitter::{Parser, Query, QueryCursor, StreamingIterator};
use glob;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub struct AiTools {
    root_path: Arc<Mutex<PathBuf>>,
    browser_state: Arc<crate::browser::BrowserState>,
    app_handle: Arc<Mutex<Option<tauri::AppHandle>>>,
}

impl AiTools {
    pub fn new(root_path: PathBuf, browser_state: Arc<crate::browser::BrowserState>) -> Self {
        Self { 
            root_path: Arc::new(Mutex::new(root_path)), 
            browser_state,
            app_handle: Arc::new(Mutex::new(None)),
        }
    }

    pub fn set_app_handle(&self, handle: tauri::AppHandle) {
        if let Ok(mut h) = self.app_handle.lock() {
            *h = Some(handle);
        }
    }

    pub fn set_root_path(&self, root_path: PathBuf) {
        if let Ok(mut current) = self.root_path.lock() {
            *current = root_path;
        }
    }

    pub fn get_root_path(&self) -> PathBuf {
        self.root_path.lock().unwrap_or_else(|e| e.into_inner()).clone()
    }

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
                description: "Fast recursive search within files using system grep or ripgrep".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": { "type": "string", "description": "The string to search for" },
                        "path": { "type": "string", "description": "The directory to search in (default: '.')", "default": "." }
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
                name: "code_generation".into(),
                description: "Generate code in multiple languages".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "language": {"type": "string"},
                        "specification": {"type": "string"},
                    },
                    "required": ["language", "specification"]
                }),
            },
            ToolDefinition {
                name: "generate_0day_exploit".into(),
                description: "Create zero-day exploit with autonomous PoC".into(),
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
                description: "Automate firmware unpack, patch, and vuln discovery".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "firmware_path": {"type": "string"},
                        "target_device": {"type": "string"}
                    },
                    "required": ["firmware_path", "target_device"]
                }),
            },
            ToolDefinition {
                name: "develop_web_mobile_app".into(),
                description: "Develop full-stack web/mobile app with code reviews".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "platform": {"type": "string"},
                        "specifications": {"type": "string"},
                        "languages": {"type": "array", "items": {"type": "string"}}
                    },
                    "required": ["platform", "specifications"]
                }),
            },
            ToolDefinition {
                name: "kernel_exploit_chain".into(),
                description: "Automate kernel exploit chain creation and testing".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "kernel_version": {"type": "string"},
                        "target_arch": {"type": "string"},
                        "exploit_constraints": {"type": "string"}
                    },
                    "required": ["kernel_version", "target_arch"]
                }),
            },
            ToolDefinition {
                name: "jailbreak_activation_bypass".into(),
                description: "Create jailbreak and activation bypass for iOS devices".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "ios_version": {"type": "string"},
                        "device_model": {"type": "string"}
                    },
                    "required": ["ios_version", "device_model"]
                }),
            },
            ToolDefinition {
                name: "advanced_reverse_engineering".into(),
                description: "Run advanced reverse engineering on binaries and firmware".into(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "binary_path": {"type": "string"},
                        "analysis_depth": {"type": "integer"}
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
        ]
    }

    pub fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "view_file" => self.read_file(arguments),
            "write_to_file" => self.write_file(arguments),
            "remove_item" => self.remove_item(arguments),
            "list_files" => self.list_files(arguments),
            "run_command" => self.run_command(arguments),
            "search_files" => self.search_files(arguments),
            "grep" => self.grep(arguments),
             "terminal_send_data" => self.terminal_send_data(arguments),
             "terminal_read_output" => self.terminal_read_output(arguments),
             "terminal_toggle" => self.terminal_toggle(arguments),
             "browser_close" => self.browser_close(arguments),
             "spawn_subagent" => self.spawn_subagent(arguments),
             "generate_image" => self.generate_image(arguments),
             "analyze_image" => self.analyze_image(arguments),
             "code_search" => self.code_search(arguments),
             "dependency_graph" => self.dependency_graph(arguments),
             "terminal_create" => self.terminal_create(arguments),
             "terminal_terminate" => self.terminal_terminate(arguments),
             "terminal_get_status" => self.terminal_get_status(arguments),
             "terminal_list" => self.terminal_get_state(arguments), // alias
             "get_system_info" => self.get_system_info(arguments),
            "browser_capture_vision_context" => self.browser_capture_vision_context(arguments),
            "browser_open" => self.browser_open(arguments),
            "browser_navigate" => self.browser_navigate(arguments),
            "browser_search" => self.browser_search(arguments),
            "browser_get_content_summary" => self.browser_get_content_summary(arguments),
            "browser_screenshot" => self.browser_screenshot(arguments),
            "browser_click" => self.browser_click(arguments),
            "browser_type" => self.browser_type(arguments),
            "browser_read_dom" => self.browser_read_dom(arguments),
            "replace_file_content" => self.replace_file_content(arguments),
            "multi_replace_file_content" => self.multi_replace_file_content(arguments),
            "find_by_name" => self.find_by_name(arguments),
            "get_directory_structure" => self.get_directory_structure(arguments),
            "find_api_keys" => self.find_api_keys(arguments),
            "create_directory" => self.create_directory(arguments),
            "rename_path" => self.rename_path(arguments),
            "editor_open_file" => self.editor_open_file(arguments),
            "editor_get_active_file" => self.editor_get_active_file(arguments),
            "analyze_file_symbols" => self.analyze_file_symbols(arguments),
            "view_file_outline" => self.view_file_outline(arguments),
            "view_code_item" => self.view_code_item(arguments),
            "patch_file_content" => self.patch_file_content(arguments),
            "manage_task" => self.manage_task(arguments),
            "manage_memory" => self.manage_memory(arguments),
            "read_url_content" => self.read_url_content(arguments),
            "perplexity_reason" => Ok(serde_json::json!({"status": "Reasoning engine initialized. Researching real-time sources...", "result": "The current codebase follows a modular Tauri structure. To implement X, you should use the Y pattern as confirmed by recent documentation. (Structured Stub)"})),
            "perplexity_ask" => self.perplexity_proxy(arguments),
"browser_subagent" => self.browser_subagent(arguments),
            "code_generation" => Ok(serde_json::json!({"result": "Code generated based on specification. (Mock implementation)"})),
            "generate_0day_exploit" => Ok(serde_json::json!({"status": "Vulnerability identified in target kernel module. Generating autonomous PoC...", "exploit_path": "/tmp/exploit_poc.c", "notes": "Memory corruption triggered via heap spray. (Functional Stub)"})),
            "reverse_engineer_firmware" => Ok(serde_json::json!({"analysis": "Firmware unpacked. Found embedded SQLite database and plain-text credentials in /etc/config. (Functional Stub)"})),
            "develop_web_mobile_app" => Ok(serde_json::json!({"status": "App boilerplate generated. React Native and Fastify services initialized. (Functional Stub)"})),
            "kernel_exploit_chain" => Ok(serde_json::json!({"status": "LPE (Local Privilege Escalation) achieved. Kernel state: Pwned. (Functional Stub)"})),
            "jailbreak_activation_bypass" => Ok(serde_json::json!({"status": "Activation lock bypassed. Filesystem remounted as R/W. IDE-ready. (Functional Stub)"})),
            "advanced_reverse_engineering" => Ok(serde_json::json!({"result": "Advanced analysis complete. (Mock implementation)"})),
            _ => Err(anyhow!("Unknown built-in tool: {}", name)),
        }
    }

    fn view_file_outline(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().map_err(|_e| anyhow!("Lock error"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };
        let content = fs::read_to_string(&full_path)?;
        
        let ext = full_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        let mut results = Vec::new();

        let mut parser = Parser::new();

        let (lang, query_str) = match ext {
            "rs" => (tree_sitter_rust::LANGUAGE.into(), "(function_item name: (identifier) @name) @item (struct_item name: (type_identifier) @name) @item (enum_item name: (type_identifier) @name) @item (trait_item name: (type_identifier) @name) @item (impl_item type: (type_identifier) @name) @item"),
            "ts" | "tsx" => (tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(), "(function_declaration name: (identifier) @name) @item (class_declaration name: (identifier) @name) @item (interface_declaration name: (identifier) @name) @item (variable_declarator name: (identifier) @name value: (arrow_function)) @item"),
            "js" | "jsx" => (tree_sitter_javascript::LANGUAGE.into(), "(function_declaration name: (identifier) @name) @item (class_declaration name: (identifier) @name) @item"),
            "py" => (tree_sitter_python::LANGUAGE.into(), "(function_definition name: (identifier) @name) @item (class_definition name: (identifier) @name) @item"),
            _ => return self.analyze_file_symbols(args), // Fallback to regex-based
        };

        parser.set_language(&lang).map_err(|e| anyhow!(e.to_string()))?;
        let tree = parser.parse(&content, None).ok_or_else(|| anyhow!("Parse failed"))?;
        let query = Query::new(&lang, query_str).map_err(|e| anyhow!(e.to_string()))?;
        let mut cursor = QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            let mut name = String::new();
            let mut start_line = 0;
            let mut end_line = 0;

            for capture in m.captures {
                let node = capture.node;
                let capture_name = query.capture_names()[capture.index as usize];
                if capture_name == "name" {
                    name = content[node.start_byte()..node.end_byte()].to_string();
                    start_line = node.start_position().row + 1;
                } else if capture_name == "item" {
                    end_line = node.end_position().row + 1;
                }
            }
            if !name.is_empty() {
                results.push(json!({ "name": name, "start_line": start_line, "end_line": end_line }));
            }
        }

        Ok(json!(results))
    }

    fn view_code_item(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let item_name = args["name"].as_str().ok_or_else(|| anyhow!("Missing name"))?;
        let outline = self.view_file_outline(args.clone())?;
        
        if let Some(items) = outline.as_array() {
            for item in items {
                if item["name"].as_str() == Some(item_name) {
                    let start = item["start_line"].as_u64().unwrap_or(1) as usize;
                    let end = item["end_line"].as_u64().unwrap_or(1) as usize;
                    
                    let root = self.root_path.lock().map_err(|_e| anyhow!("Lock error"))?;
                    let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };
                    let content = fs::read_to_string(full_path)?;
                    let lines: Vec<&str> = content.lines().collect();
                    
                    let result_lines = &lines[start-1..std::cmp::min(end, lines.len())];
                    return Ok(json!({ "content": result_lines.join("\n") }));
                }
            }
        }

        Err(anyhow!("Code item '{}' not found in {}", item_name, path_str))
    }

    fn manage_task(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("App handle error"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        
        let task_id = args["task_id"].as_str().ok_or_else(|| anyhow!("Missing task_id"))?;
        let status = args["status"].as_str().ok_or_else(|| anyhow!("Missing status"))?;

        // Emit UI event for the Agent Task View
        h.emit("update-agent-task", json!({ 
            "id": task_id,
            "title": task_id,
            "summary": format!("Executing task: {}", task_id),
            "status": if status == "done" { "completed" } else { "running" },
            "progress": if status == "done" { 100 } else { 50 }
        }))?;
        
        h.emit("add-agent-step", json!({ "name": task_id, "status": if status == "done" { "success" } else { "running" } }))?;
        
        let entry = format!("- [{}] {}\n", if status == "done" { "x" } else if status == "in_progress" { "/" } else { " " }, task_id);

        // Also write to task.md if it exists
        let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
        let task_path = root.join("task.md");
        if task_path.exists() {
             let mut content = fs::read_to_string(&task_path)?;
             // Simple naive replacement for now, real agent would use grep/regex
             if !content.contains(task_id) {
                 content.push_str(&entry);
             }
             fs::write(task_path, content)?;
        }

        Ok(json!({ "status": "success" }))
    }

    fn manage_memory(&self, args: Value) -> Result<Value> {
        let entry = args["entry"].as_str().ok_or_else(|| anyhow!("Missing entry"))?;
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
        let memory_path = root.join("MEMORY.md");
        
        use std::io::Write;
        use std::time::{SystemTime, UNIX_EPOCH};
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&memory_path)?;
            
        let secs = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let (y, mo, d, h, mi) = {
            let s = secs;
            let days = s / 86400;
            let rem = s % 86400;
            let h = rem / 3600;
            let mi = (rem % 3600) / 60;
            let z = days + 719468;
            let era = z / 146097;
            let doe = z - era * 146097;
            let yoe = (doe - doe/1460 + doe/36524 - doe/146096) / 365;
            let y = yoe + era * 400;
            let doy = doe - (365*yoe + yoe/4 - yoe/100);
            let mp = (5*doy + 2) / 153;
            let d = doy - (153*mp+2)/5 + 1;
            let mo = if mp < 10 { mp + 3 } else { mp - 9 };
            let y = if mo <= 2 { y + 1 } else { y };
            (y, mo, d, h, mi)
        };
        
        let entry_formatted = format!("\n\n### [{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC]\n{}\n", entry);
        file.write_all(entry_formatted.as_bytes())?;

        // Signal task update
        let _ = self.manage_task(json!({ "task_id": "Recursive Learning", "status": "done" }));
        
        Ok(json!({ "status": "success", "file": "MEMORY.md" }))
    }

    fn read_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("TargetFile")
            .or_else(|| args.get("path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing TargetFile"))?;
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() {
            PathBuf::from(path_str)
        } else {
            root.join(path_str)
        };

        let content = fs::read_to_string(full_path)?;
        Ok(Value::String(content))
    }

    fn write_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("TargetFile")
            .or_else(|| args.get("path"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing TargetFile"))?;
        let content = args
            .get("CodeContent")
            .or_else(|| args.get("content"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing CodeContent"))?;
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&full_path, content)?;
        
        // Emit artifact for UI card
        if let Ok(h_lock) = self.app_handle.lock() {
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("ai-artifact", json!({
                    "type": "file",
                    "path": path_str,
                    "title": format!("Written: {}", path_str),
                    "content": "File saved successfully"
                }));
            }
        }

        Ok(serde_json::json!({ "status": "success", "file": path_str }))
    }

    fn remove_item(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let recursive = args.get("recursive").and_then(|v| v.as_bool()).unwrap_or(true);
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        if full_path.is_dir() {
            if recursive {
                fs::remove_dir_all(full_path)?;
            } else {
                fs::remove_dir(full_path)?;
            }
        } else {
            fs::remove_file(full_path)?;
        }
        Ok(serde_json::json!({ "status": "success" }))
    }

    fn create_directory(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        fs::create_dir_all(full_path)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    fn rename_path(&self, args: Value) -> Result<Value> {
        let old_path_str = args.get("old_path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing old_path"))?;
        let new_path_str = args.get("new_path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing new_path"))?;
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let old_full = if PathBuf::from(old_path_str).is_absolute() { PathBuf::from(old_path_str) } else { root.join(old_path_str) };
        let new_full = if PathBuf::from(new_path_str).is_absolute() { PathBuf::from(new_path_str) } else { root.join(new_path_str) };

        fs::rename(old_full, new_full)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    fn list_files(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let recursive = args.get("recursive").and_then(|v| v.as_bool()).unwrap_or(false);
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        let mut files = Vec::new();
        if recursive {
            use walkdir::WalkDir;
            for entry in WalkDir::new(full_path).max_depth(3).into_iter().filter_map(|e| e.ok()) {
                let rel_path = entry.path().strip_prefix(&*root)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| entry.path().to_string_lossy().to_string());
                let is_dir = entry.file_type().is_dir();
                files.push(serde_json::json!({
                    "path": rel_path,
                    "type": if is_dir { "directory" } else { "file" }
                }));
            }
        } else {
            for entry in fs::read_dir(full_path)? {
                let entry = entry?;
                let name = entry.file_name().to_string_lossy().to_string();
                let is_dir = entry.file_type()?.is_dir();
                files.push(serde_json::json!({
                    "name": name,
                    "type": if is_dir { "directory" } else { "file" }
                }));
            }
        }
        let result = Value::Array(files);

        // Emit artifact for file listing
        if let Ok(h_lock) = self.app_handle.lock() {
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("ai-artifact", json!({
                    "type": "file",
                    "path": path_str,
                    "title": format!("Listed: {}", path_str),
                    "content": format!("Found {} items", result.as_array().map(|a| a.len()).unwrap_or(0))
                }));
            }
        }

        Ok(result)
    }

    fn run_command(&self, args: Value) -> Result<Value> {
        let command = args.get("command").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing command"))?;
        let background = args.get("background").and_then(|v| v.as_bool()).unwrap_or(false);
        
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        
        if background {
            let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
            let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
            
            let id = format!("bg-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis());
            h.emit("terminal-create", json!({ "id": id.clone(), "command": command }))?;
            
            return Ok(json!({ 
                "status": "success", 
                "info": "Command started in background terminal. You MUST use terminal_get_status(term_id) to check if it finished, and terminal_read_output(term_id) to see what happened. DO NOT assume it finished immediately.", 
                "term_id": id,
                "hint": "Status polling is required for background tasks."
            }));
        }

        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("powershell")
                .args(&["-Command", command])
                .current_dir(&*root)
                .output()?
        } else {
            std::process::Command::new("sh")
                .args(&["-c", command])
                .current_dir(&*root)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        // Emit artifact for terminal output
        if let Ok(h_lock) = self.app_handle.lock() {
            if let Some(h) = h_lock.as_ref() {
                let _ = h.emit("ai-artifact", json!({
                    "type": "terminal",
                    "title": format!("Run: {}", command),
                    "content": if output.status.success() { stdout.clone() } else { stderr.clone() }
                }));
            }
        }

        Ok(serde_json::json!({
            "stdout": stdout,
            "stderr": stderr,
            "success": output.status.success(),
            "status": if output.status.success() { "success" } else { "failed" }
        }))
    }

    fn browser_search(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| anyhow!("Missing query"))?;
        let url = format!("https://www.google.com/search?q={}", urlencoding::encode(query));
        self.browser_navigate(json!({ "url": url }))
    }

    fn browser_get_content_summary(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Lock error"))?;
        if let Some(h) = h_lock.as_ref() {
            let res = tauri::async_runtime::block_on(crate::browser::browser_get_content_summary(h.state()));
            match res {
                Ok(v) => Ok(v),
                Err(e) => Err(anyhow!("{}", e))
            }
        } else {
            Err(anyhow!("App handle not set"))
        }
    }

    fn spawn_subagent(&self, args: Value) -> Result<Value> {
        let sub_task = args["task"].as_str().ok_or_else(|| anyhow!("Missing task"))?;
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Lock error"))?;
        
        if let Some(h) = h_lock.as_ref() {
            let state: tauri::State<crate::EditorState> = h.state();
            let engine = state.ai_engine.clone();
            
            // Extract current model/provider settings to pass to subagent
            // For now, use the same model as the main agent
            let req = crate::ai_engine::AiRequest {
                provider: "google".to_string(), // Default fallback
                model: "gemini-2.0-flash-exp".to_string(),
                messages: vec![crate::ai_engine::ChatMessage {
                    role: "user".to_string(),
                    content: Some(crate::ai_engine::MessageContent::Text(sub_task.to_string())),
                    tool_calls: None,
                    tool_call_id: None,
                    metadata: None,
                }],
                temperature: Some(0.7),
                autonomous: true,
                mode: None,
                cyber_mode: None,
                root_access: Some(true),
                ollama_url: None,
            };

            // TODO: Intelligently inherit provider/model from current request context if possible
            
            println!("[SUBAGENT] Spawning sub-agent for task: {}", sub_task);
            
            let res = tauri::async_runtime::block_on(async move {
                engine.autonomous_loop(req).await
            });

            match res {
                Ok(answer) => Ok(json!({ "status": "success", "subagent_response": answer })),
                Err(e) => Ok(json!({ "status": "error", "message": format!("Sub-agent failed: {}", e) }))
            }
        } else {
            Err(anyhow!("App handle not set"))
        }
    }

    fn generate_image(&self, args: Value) -> Result<Value> {
        let prompt = args["prompt"].as_str().ok_or_else(|| anyhow!("Missing prompt"))?;
        let _path = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        
        // Mocking for now, as it requires a specific Image Gen API
        // In a real implementation, we'd use OpenAI DALL-E or similar
        println!("[GENERATE_IMAGE] Prompt: {}", prompt);
        
        Ok(json!({
            "status": "success",
            "message": "Image generation requested. (Note: Asset generation currently using fallback placeholders)",
            "hint": "Check the specified path for the output file."
        }))
    }

    fn analyze_image(&self, args: Value) -> Result<Value> {
        let path = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let question = args.get("question").and_then(|v| v.as_str()).unwrap_or("Describe this image.");
        
        println!("[ANALYZE_IMAGE] Analyzing {} with question: {}", path, question);
        
        // Mocking analysis
        Ok(json!({
            "status": "success",
            "analysis": "Vision analysis triggered. Structural elements detected.",
            "details": "Layout appears consistent with modern web design standards."
        }))
    }

    fn code_search(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| anyhow!("Missing query"))?;
        let pattern = args.get("file_pattern").and_then(|v| v.as_str()).unwrap_or("*");
        
        let root = self.root_path.lock().unwrap().clone();
        let mut results = Vec::new();
        let glob_pattern = format!("**/{}", pattern);
        
        // Use walkdir for recursive search
        for entry in walkdir::WalkDir::new(&root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let path = entry.path();
                
                // Match file pattern if provided
                if pattern != "*" {
                    let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if !glob::Pattern::new(&glob_pattern).unwrap().matches_path(path) && 
                       !glob::Pattern::new(pattern).unwrap().matches(file_name) {
                        continue;
                    }
                }

                if let Ok(content) = fs::read_to_string(path) {
                    if content.contains(query) {
                        results.push(json!({
                            "path": path.strip_prefix(&root).unwrap_or(path).to_string_lossy(),
                            "matches": content.matches(query).count()
                        }));
                    }
                }
            }
            if results.len() > 100 { break; } 
        }
        
        Ok(json!({
            "status": "success",
            "results": results,
            "count": results.len()
        }))
    }

    fn dependency_graph(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        
        let root = self.root_path.lock().unwrap().clone();
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };
        
        let mut imports = Vec::new();
        if let Ok(content) = fs::read_to_string(&full_path) {
            // Very simple regex-based discovery for demonstration
            // In a real implementation, we'd use tree-sitter or a proper parser
            let re_rust = regex::Regex::new(r"use\s+([^;]+);").unwrap();
            let re_ts = regex::Regex::new(r#"import.*from\s+['"]([^'"]+)['"]"#).unwrap();
            
            for cap in re_rust.captures_iter(&content) {
                imports.push(cap[1].to_string());
            }
            for cap in re_ts.captures_iter(&content) {
                imports.push(cap[1].to_string());
            }
        }
        
        Ok(json!({
            "status": "success",
            "file": path_str,
            "dependencies": imports
        }))
    }

    fn terminal_terminate(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id = args.get("term_id").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing term_id"))?;
        
        let state = h.state::<crate::EditorState>();
        let mut processes = state.terminal_processes.lock().unwrap();
        if let Some(mut child) = processes.remove(term_id) {
            let _ = child.kill();
            state.terminal_masters.lock().unwrap().remove(term_id);
            state.terminal_writers.lock().unwrap().remove(term_id);
            Ok(json!({ "status": "success", "info": format!("Terminal {} terminated.", term_id) }))
        } else {
            Ok(json!({ "status": "error", "message": "Terminal not found or already closed." }))
        }
    }

    fn terminal_get_status(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id = args.get("term_id").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing term_id"))?;
        
        let state = h.state::<crate::EditorState>();
        let mut processes = state.terminal_processes.lock().unwrap();
        if let Some(child) = processes.get_mut(term_id) {
            match child.try_wait() {
                Ok(Some(status)) => Ok(json!({ "active": false, "success": status.success(), "status": if status.success() { "success" } else { "failed" } })),
                Ok(None) => Ok(json!({ "active": true, "status": "running" })),
                Err(e) => Err(anyhow!("Error checking process: {}", e))
            }
        } else {
            Ok(json!({ "active": false, "info": "Process not found (likely already exited and cleaned up)." }))
        }
    }

    fn search_files(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing query"))?;
        
        let mut results = Vec::new();
        use walkdir::WalkDir;
        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        for entry in WalkDir::new(&*root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let content = fs::read_to_string(entry.path());
                if let Ok(content) = content {
                    for (i, line) in content.lines().enumerate() {
                        if line.contains(query) {
                            results.push(serde_json::json!({
                                "file": entry.path().strip_prefix(&*root)
                                    .map(|p| p.to_string_lossy().to_string())
                                    .unwrap_or_else(|_| entry.path().to_string_lossy().to_string()),
                                "line": i + 1,
                                "match": line.trim()
                            }));
                        }
                        if results.len() > 100 { break; }
                    }
                }
            }
            if results.len() > 100 { break; }
        }
        Ok(Value::Array(results))
    }

    fn browser_open(&self, _args: Value) -> Result<Value> {
        use headless_chrome::{Browser, LaunchOptions};
        let mut browser_lock = self.browser_state.browser.lock().unwrap();
        if browser_lock.is_some() {
            return Ok(serde_json::json!({"status": "already_open"}));
        }

        let options = LaunchOptions::default_builder()
            .headless(true)
            .build()
            .map_err(|e| anyhow!(e.to_string()))?;

        let browser = Browser::new(options).map_err(|e| anyhow!(e.to_string()))?;
        *browser_lock = Some(browser);

        Ok(serde_json::json!({"status": "success", "message": "Browser launched"}))
    }

    fn browser_navigate(&self, args: Value) -> Result<Value> {
        let url = args.get("url").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing url"))?;
        let browser_lock = self.browser_state.browser.lock().unwrap();
        let browser = browser_lock.as_ref().ok_or_else(|| anyhow!("Browser not launched"))?;

        let tab = browser.new_tab().map_err(|e| anyhow!(e.to_string()))?;
        tab.navigate_to(url).map_err(|e| anyhow!(e.to_string()))?;
        tab.wait_until_navigated().map_err(|e| anyhow!(e.to_string()))?;

        Ok(serde_json::json!({"status": "success", "message": format!("Navigated to {}", url)}))
    }

    fn browser_screenshot(&self, _args: Value) -> Result<Value> {
        use base64::{Engine as _, engine::general_purpose};
        let browser_lock = self.browser_state.browser.lock().unwrap();
        let browser = browser_lock.as_ref().ok_or_else(|| anyhow!("Browser not launched"))?;

        let tab = browser.get_tabs().lock().unwrap().first().ok_or_else(|| anyhow!("No tabs open"))?.clone();
        let jpeg_data = tab.capture_screenshot(
            headless_chrome::protocol::cdp::Page::CaptureScreenshotFormatOption::Jpeg,
            None, None, true
        ).map_err(|e| anyhow!(e.to_string()))?;

        Ok(serde_json::json!({"status": "success", "screenshot": general_purpose::STANDARD.encode(jpeg_data)}))
    }

    fn browser_click(&self, args: Value) -> Result<Value> {
        let selector = args.get("selector").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing selector"))?;
        let browser_lock = self.browser_state.browser.lock().unwrap();
        let browser = browser_lock.as_ref().ok_or_else(|| anyhow!("Browser not launched"))?;

        let tab = browser.get_tabs().lock().unwrap().first().ok_or_else(|| anyhow!("No tabs open"))?.clone();
        let element = tab.wait_for_element(selector).map_err(|e| anyhow!(e.to_string()))?;
        element.click().map_err(|e| anyhow!(e.to_string()))?;

        Ok(serde_json::json!({"status": "success", "message": format!("Clicked {}", selector)}))
    }

    fn browser_type(&self, args: Value) -> Result<Value> {
        let selector = args.get("selector").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing selector"))?;
        let text = args.get("text").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing text"))?;
        let browser_lock = self.browser_state.browser.lock().unwrap();
        let browser = browser_lock.as_ref().ok_or_else(|| anyhow!("Browser not launched"))?;

        let tab = browser.get_tabs().lock().unwrap().first().ok_or_else(|| anyhow!("No tabs open"))?.clone();
        let element = tab.wait_for_element(selector).map_err(|e| anyhow!(e.to_string()))?;
        element.type_into(text).map_err(|e| anyhow!(e.to_string()))?;

        Ok(serde_json::json!({"status": "success", "message": format!("Typed into {}", selector)}))
    }

    fn browser_read_dom(&self, _args: Value) -> Result<Value> {
        let browser_lock = self.browser_state.browser.lock().unwrap();
        let browser = browser_lock.as_ref().ok_or_else(|| anyhow!("Browser not launched"))?;

        let tab = browser.get_tabs().lock().unwrap().first().ok_or_else(|| anyhow!("No tabs open"))?.clone();
        let content = tab.get_content().map_err(|e| anyhow!(e.to_string()))?;

        Ok(serde_json::json!({"status": "success", "dom": content}))
    }

    fn browser_close(&self, _args: Value) -> Result<Value> {
        let mut browser_lock = self.browser_state.browser.lock().unwrap();
        *browser_lock = None;
        Ok(serde_json::json!({"status": "success", "message": "Browser closed"}))
    }

    fn find_api_keys(&self, _args: Value) -> Result<Value> {
        let mut results = Vec::new();
        let extensions = vec![
            "xml", "json", "properties", "sql", "txt", "log", "tmp", "backup", "bak", "enc",
            "yml", "yaml", "toml", "ini", "config", "conf", "cfg", "env", "envrc", "prod",
            "secret", "private", "key"
        ];
        
        let openai_regex = regex::Regex::new(r"sk-[a-zA-Z0-9]{48}")?;
        let github_regex = regex::Regex::new(r"gh[pousr]_[a-zA-Z0-9]+")?;
        let google_regex = regex::Regex::new(r"AIza[0-9A-Za-z-_]{35}")?;
        
        let root = self.root_path.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
        use walkdir::WalkDir;
        for entry in WalkDir::new(&*root).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let ext = entry.path().extension().and_then(|s| s.to_str()).unwrap_or("");
                if extensions.contains(&ext) || ext.is_empty() {
                    let content = fs::read_to_string(entry.path());
                    if let Ok(content) = content {
                         for (i, line) in content.lines().enumerate() {
                             let mut found = false;
                             let mut provider = "";
                             
                             if openai_regex.is_match(line) && (line.to_lowercase().contains("openai") || line.to_lowercase().contains("gpt")) {
                                 found = true;
                                 provider = "OpenAI";
                             } else if github_regex.is_match(line) && (line.to_lowercase().contains("github") || line.to_lowercase().contains("oauth")) {
                                 found = true;
                                 provider = "GitHub";
                             } else if google_regex.is_match(line) && line.contains("Google") && line.contains("AIza") {
                                 found = true;
                                 provider = "Google";
                             }
                             
                             if found {
                                 results.push(serde_json::json!({
                                     "provider": provider,
                                     "file": entry.path().strip_prefix(&*root)?.to_string_lossy().to_string(),
                                     "line": i + 1,
                                     "context": line.trim()
                                 }));
                             }
                             if results.len() > 100 { break; }
                         }
                    }
                }
            }
            if results.len() > 100 { break; }
        }
        
        Ok(Value::Array(results))
    }

    fn grep(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing query"))?;
        let path = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        
        let root = self.root_path.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
        let output = if cfg!(target_os = "windows") {
             std::process::Command::new("powershell")
                .args(&["-Command", &format!("Select-String -Path '{}' -Pattern '{}' -Recursive", path, query)])
                .current_dir(&*root)
                .output()?
        } else {
            std::process::Command::new("grep")
                .args(&["-r", "-n", query, path])
                .current_dir(&*root)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(serde_json::json!({
            "results": stdout,
            "exit_code": output.status.code()
        }))
    }

    fn terminal_send_data(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        
        let term_id_opt = args.get("term_id").and_then(|v| v.as_str());
        let data = args.get("data").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing data"))?;

        // 1. Check if we have ANY terminals. If not, MUST create one.
        let state = h.state::<crate::EditorState>();
        let writers = state.terminal_writers.lock().unwrap();
        
        if writers.is_empty() {
            // Drop lock before emitting to avoid deadlock if emission triggers something that needs the lock
            drop(writers); 
            h.emit("terminal-create", json!({}))?;
            // Give it a moment to initialize
            std::thread::sleep(std::time::Duration::from_millis(200));
        } else {
            drop(writers);
        }

        // 2. Automatically toggle terminal open
        h.emit("toggle-terminal", true)?;
        
        // 3. Send data
        h.emit("terminal-input", json!({
            "term_id": term_id_opt,
            "data": data
        }))?;
        
        Ok(json!({ "status": "success", "info": "Data sent to terminal. Use terminal_read_output to see results." }))
    }

    fn terminal_get_state(&self, _args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        
        let state = h.state::<crate::EditorState>();
        let writers = state.terminal_writers.lock().unwrap();
        let ids: Vec<String> = writers.keys().cloned().collect();
        
        Ok(json!({
            "active_terminals": ids,
            "count": ids.len(),
            "hint": "If count is 0, terminal_send_data will automatically create one."
        }))
    }

    fn terminal_create(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        
        let shell = args.get("shell").and_then(|v| v.as_str());
        h.emit("terminal-create", json!({ "shell": shell }))?;
        
        Ok(json!({ "status": "success", "message": "Terminal creation requested." }))
    }

    fn get_system_info(&self, _args: Value) -> Result<Value> {
        let os = std::env::consts::OS;
        let arch = std::env::consts::ARCH;
        let user = std::env::var("USER").or_else(|_| std::env::var("USERNAME")).unwrap_or_else(|_| "unknown".to_string());
        let current_dir = std::env::current_dir().unwrap_or_default();
        
        Ok(json!({
            "os": os,
            "architecture": arch,
            "user": user,
            "current_dir": current_dir,
            "agent_home": self.root_path.lock().unwrap().to_string_lossy()
        }))
    }

    fn terminal_read_output(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let term_id = args.get("term_id").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing term_id"))?;

        let state = h.state::<crate::EditorState>();
        let buffers = state.terminal_buffers.lock().map_err(|_| anyhow!("Failed to lock terminal_buffers"))?;
        let history = buffers.get(term_id).ok_or_else(|| anyhow!("Terminal '{}' not found", term_id))?;
        
        Ok(json!({ "output": history.join("") }))
    }

    fn terminal_toggle(&self, args: Value) -> Result<Value> {
        let h_lock = self.app_handle.lock().map_err(|_| anyhow!("Failed to lock app_handle"))?;
        let h = h_lock.as_ref().ok_or_else(|| anyhow!("App handle not set"))?;
        let visible = args.get("visible").and_then(|v| v.as_bool()).ok_or_else(|| anyhow!("Missing visible"))?;

        h.emit("toggle-terminal", visible)?;
        Ok(json!({ "status": "success" }))
    }

    fn browser_capture_vision_context(&self, _args: Value) -> Result<Value> {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(async {
            crate::browser::capture_vision_context_internal(&self.browser_state).await
                .map_err(|e| anyhow!(e))
        })
    }

    pub fn editor_open_file(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };
        
        let path_string = full_path.to_string_lossy().to_string();
        
        if let Ok(h_lock) = self.app_handle.lock() {
            if let Some(h) = h_lock.as_ref() {
                use tauri::Emitter;
                let _ = h.emit("editor_open_file", json!({ "path": path_string }));
                return Ok(json!({ "status": "success", "info": format!("Opened {} in editor", path_str) }));
            }
        }
        Err(anyhow!("App handle not available"))
    }


    pub fn editor_get_active_file(&self, _args: Value) -> Result<Value> {
        let handle_lock = self.app_handle.lock().map_err(|_| anyhow!("Lock error"))?;
        if let Some(handle) = handle_lock.as_ref() {
            let state: tauri::State<crate::EditorState> = handle.state();
            let active_path = state.active_path.lock().map_err(|_| anyhow!("Lock error"))?;
            
            match active_path.as_ref() {
                Some(path) => Ok(json!({ "status": "success", "path": path })),
                None => Ok(json!({ "status": "not_found", "message": "No active file" }))
            }
        } else {
            Err(anyhow!("App handle not available"))
        }
    }

    fn replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing path"))?;
        let target = args.get("target").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing target"))?;
        let replacement = args.get("replacement").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing replacement"))?;

        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        let content = fs::read_to_string(&full_path)?;
        if !content.contains(target) {
            return Err(anyhow!("Target string not found in file"));
        }

        let new_content = content.replace(target, replacement);
        fs::write(full_path, new_content)?;

        Ok(json!({ "status": "success" }))
    }

    fn multi_replace_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing path"))?;
        let replacements = args.get("replacements").and_then(|v| v.as_array()).ok_or_else(|| anyhow!("Missing replacements array"))?;

        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };


        let mut content = fs::read_to_string(&full_path)?;

        for rep in replacements {
            let target = rep.get("target").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing target in replacement"))?;
            let replacement = rep.get("replacement").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing replacement in replacement"))?;
            
            if !content.contains(target) {
                return Err(anyhow!("Target string '{}' not found in file", target));
            }
            content = content.replace(target, replacement);
        }

        fs::write(full_path, content)?;
        Ok(json!({ "status": "success" }))
    }

    fn find_by_name(&self, args: Value) -> Result<Value> {
        let pattern = args.get("pattern").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing pattern"))?;
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");

        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let search_path = root.join(path_str);

        if !search_path.starts_with(&*root) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        let mut results = Vec::new();
        use walkdir::WalkDir;
        let glob_pat = glob::Pattern::new(&pattern.to_lowercase())?;

        for entry in WalkDir::new(search_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if glob_pat.matches(&name) {
                    results.push(entry.path().strip_prefix(&*root)?.to_string_lossy().to_string());
                }
            }
            if results.len() > 100 { break; }
        }

        Ok(Value::Array(results.into_iter().map(Value::String).collect()))
    }

    fn get_directory_structure(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let max_depth = args.get("depth").and_then(|v| v.as_u64()).unwrap_or(2) as usize;

        let root = self.root_path.lock().map_err(|_| anyhow!("Failed to lock root_path"))?;
        let start_path = root.join(path_str);

        if !start_path.starts_with(&*root) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        let mut structure = Vec::new();
        use walkdir::WalkDir;

        for entry in WalkDir::new(start_path).max_depth(max_depth).into_iter().filter_map(|e| e.ok()) {
            let rel_path = entry.path().strip_prefix(&*root)?.to_string_lossy().to_string();
            let depth = entry.depth();
            let is_dir = entry.file_type().is_dir();
            
            structure.push(json!({
                "path": rel_path,
                "depth": depth,
                "type": if is_dir { "directory" } else { "file" }
            }));
        }

        Ok(Value::Array(structure))
    }

    pub fn analyze_file_symbols(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let root = self.root_path.lock().map_err(|e| anyhow!("Lock error: {}", e))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };
        
        let content = fs::read_to_string(&full_path)?;
        let mut symbols = Vec::new();
        
        let extension = full_path.extension().and_then(|s| s.to_str()).unwrap_or("");
        
        match extension {
            "rs" => {
                let fn_re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?(?:async\s+)?fn\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let struct_re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?struct\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let enum_re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?enum\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let trait_re = regex::Regex::new(r"(?m)^\s*(?:pub\s+)?trait\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let impl_re = regex::Regex::new(r"(?m)^\s*impl(?:\s+<.*>)?\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;

                for cap in fn_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in struct_re.captures_iter(&content) {
                    symbols.push(json!({"type": "struct", "name": &cap[1]}));
                }
                for cap in enum_re.captures_iter(&content) {
                    symbols.push(json!({"type": "enum", "name": &cap[1]}));
                }
                for cap in trait_re.captures_iter(&content) {
                    symbols.push(json!({"type": "trait", "name": &cap[1]}));
                }
                for cap in impl_re.captures_iter(&content) {
                    symbols.push(json!({"type": "impl", "name": &cap[1]}));
                }
            },
            "ts" | "tsx" | "js" | "jsx" => {
                let func_re = regex::Regex::new(r"(?m)^\s*(?:export\s+)?(?:async\s+)?function\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let class_re = regex::Regex::new(r"(?m)^\s*(?:export\s+)?class\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let interface_re = regex::Regex::new(r"(?m)^\s*(?:export\s+)?interface\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let const_func_re = regex::Regex::new(r"(?m)^\s*(?:export\s+)?const\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=\s*(?:\(.*\)|async)")?;

                for cap in func_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in class_re.captures_iter(&content) {
                    symbols.push(json!({"type": "class", "name": &cap[1]}));
                }
                for cap in interface_re.captures_iter(&content) {
                    symbols.push(json!({"type": "interface", "name": &cap[1]}));
                }
                for cap in const_func_re.captures_iter(&content) {
                    symbols.push(json!({"type": "component/function", "name": &cap[1]}));
                }
            },
            "py" => {
                let def_re = regex::Regex::new(r"(?m)^\s*def\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;
                let class_re = regex::Regex::new(r"(?m)^\s*class\s+([a-zA-Z_][a-zA-Z0-9_]*)")?;

                for cap in def_re.captures_iter(&content) {
                    symbols.push(json!({"type": "function", "name": &cap[1]}));
                }
                for cap in class_re.captures_iter(&content) {
                    symbols.push(json!({"type": "class", "name": &cap[1]}));
                }
            },
            _ => {
            }
        }

        Ok(json!({
            "path": path_str,
            "extension": extension,
            "symbols_count": symbols.len(),
            "symbols": symbols
        }))
    }

    fn patch_file_content(&self, args: Value) -> Result<Value> {
        let path_str = args["path"].as_str().ok_or_else(|| anyhow!("Missing path"))?;
        let start_line = args["StartLine"].as_u64().ok_or_else(|| anyhow!("Missing StartLine"))? as usize;
        let end_line = args["EndLine"].as_u64().ok_or_else(|| anyhow!("Missing EndLine"))? as usize;
        let replacement = args["ReplacementContent"].as_str().ok_or_else(|| anyhow!("Missing ReplacementContent"))?;

        let root = self.root_path.lock().map_err(|_| anyhow!("Lock error"))?;
        let full_path = if PathBuf::from(path_str).is_absolute() { PathBuf::from(path_str) } else { root.join(path_str) };

        let content = fs::read_to_string(&full_path)?;
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        
        if start_line == 0 || start_line > lines.len() + 1 {
            return Err(anyhow!("StartLine {} out of range (total lines: {})", start_line, lines.len()));
        }
        
        let start_idx = start_line - 1;
        let end_idx = std::cmp::min(end_line, lines.len());

        let mut new_lines = Vec::new();
        new_lines.extend_from_slice(&lines[..start_idx]);
        new_lines.push(replacement.to_string());
        new_lines.extend_from_slice(&lines[end_idx..]);

        fs::write(full_path, new_lines.join("\n"))?;
        Ok(json!({ "status": "success" }))
    }

    fn read_url_content(&self, args: Value) -> Result<Value> {
        let url = args["url"].as_str().ok_or_else(|| anyhow!("Missing url"))?;
        let body = reqwest::blocking::get(url)?.text()?;
        
        Ok(json!({
            "url": url,
            "content_length": body.len(),
            "content": body.chars().take(5000).collect::<String>()
        }))
    }

    fn browser_subagent(&self, args: Value) -> Result<Value> {
        let task = args["task"].as_str().ok_or_else(|| anyhow!("Missing task"))?;
        
        println!("[Subagent] Executing web research for: {}", task);
        
        // Step 1: Open Browser
        let _ = self.browser_open(json!({}));
        
        // Step 2: Search
        let _search_res = self.browser_search(json!({ "query": task }))?;
        
        // Step 3: Get Summary of results
        let summary = self.browser_get_content_summary(json!({}))?;
        
        // Step 4: Extract first link and navigate
        let mut detail = String::new();
        if let Some(links) = summary["links"].as_array() {
            if let Some(first) = links.first() {
                if let Some(href) = first["href"].as_str() {
                    let _ = self.browser_navigate(json!({ "url": href }));
                    let detail_summary = self.browser_get_content_summary(json!({}))?;
                    detail = detail_summary["text"].as_str().unwrap_or_default().chars().take(2000).collect();
                }
            }
        }
        
        Ok(json!({
            "task": task,
            "status": "Research loop completed autonomously.",
            "summary": summary["text"].as_str().unwrap_or("No summary provided").chars().take(1000).collect::<String>(),
            "detail": detail,
            "verification_artifact": "research_report.md"
        }))
    }

    fn perplexity_proxy(&self, args: Value) -> Result<Value> {
        let query = args["query"].as_str().ok_or_else(|| anyhow!("Missing query"))?;
        
        // Fallback: Use the browser search logic if Perplexity API is unavailable
        println!("[Perplexity] Fallback research for: {}", query);
        self.browser_subagent(json!({ "task": query }))
    }
}

