use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub input_schema: Value,
}

pub struct AiTools {
    root_path: PathBuf,
}

impl AiTools {
    pub fn new(root_path: PathBuf) -> Self {
        Self { root_path }
    }

    pub fn list_tools(&self) -> Vec<ToolDefinition> {
        vec![
            ToolDefinition {
                name: "read_file".to_string(),
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
                name: "write_file".to_string(),
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
                name: "delete_file".to_string(),
                description: "Delete a file".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "path": { "type": "string", "description": "Relative path to the file" }
                    },
                    "required": ["path"]
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
                description: "Run a shell command in the project root and get output".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "command": { "type": "string", "description": "The command to run" }
                    },
                    "required": ["command"]
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
        ]
    }

    pub fn call_tool(&self, name: &str, arguments: Value) -> Result<Value> {
        match name {
            "read_file" => self.read_file(arguments),
            "write_file" => self.write_file(arguments),
            "delete_file" => self.delete_file(arguments),
            "list_files" => self.list_files(arguments),
            "run_command" => self.run_command(arguments),
            "search_files" => self.search_files(arguments),
            "code_generation" => Ok(serde_json::json!({"result": "Code generated based on specification. (Mock implementation)"})),
            "generate_0day_exploit" => Ok(serde_json::json!({"status": "Exploit generated and verified in sandbox environment. (Mock implementation)"})),
            "reverse_engineer_firmware" => Ok(serde_json::json!({"analysis": "Firmware unpacked. No critical vulnerabilities found in first pass. (Mock implementation)"})),
            "develop_web_mobile_app" => Ok(serde_json::json!({"status": "App boilerplate generated and ready for review. (Mock implementation)"})),
            "kernel_exploit_chain" => Ok(serde_json::json!({"status": "Kernel exploit chain completed. (Mock implementation)"})),
            "jailbreak_activation_bypass" => Ok(serde_json::json!({"status": "Jailbreak sequence prepared. (Mock implementation)"})),
            "advanced_reverse_engineering" => Ok(serde_json::json!({"result": "Advanced analysis complete. (Mock implementation)"})),
            _ => Err(anyhow!("Unknown built-in tool: {}", name)),
        }
    }

    fn read_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let full_path = self.root_path.join(path_str);

        // Security check: ensure path is within root
        if !full_path.starts_with(&self.root_path) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        let content = fs::read_to_string(full_path)?;
        Ok(Value::String(content))
    }

    fn write_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let content = args
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing content"))?;
        let full_path = self.root_path.join(path_str);

        if !full_path.starts_with(&self.root_path) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(full_path, content)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    fn delete_file(&self, args: Value) -> Result<Value> {
        let path_str = args
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow!("Missing path"))?;
        let full_path = self.root_path.join(path_str);

        if !full_path.starts_with(&self.root_path) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        fs::remove_file(full_path)?;
        Ok(serde_json::json!({ "status": "success" }))
    }

    fn list_files(&self, args: Value) -> Result<Value> {
        let path_str = args.get("path").and_then(|v| v.as_str()).unwrap_or(".");
        let recursive = args.get("recursive").and_then(|v| v.as_bool()).unwrap_or(false);
        let full_path = self.root_path.join(path_str);

        if !full_path.starts_with(&self.root_path) {
            return Err(anyhow!("Access denied: path outside project root"));
        }

        let mut files = Vec::new();
        if recursive {
            use walkdir::WalkDir;
            for entry in WalkDir::new(full_path).max_depth(3).into_iter().filter_map(|e| e.ok()) {
                let rel_path = entry.path().strip_prefix(&self.root_path)?.to_string_lossy().to_string();
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
        Ok(Value::Array(files))
    }

    fn run_command(&self, args: Value) -> Result<Value> {
        let command = args.get("command").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing command"))?;
        
        let output = if cfg!(target_os = "windows") {
            std::process::Command::new("powershell")
                .args(&["-Command", command])
                .current_dir(&self.root_path)
                .output()?
        } else {
            std::process::Command::new("sh")
                .args(&["-c", command])
                .current_dir(&self.root_path)
                .output()?
        };

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        Ok(serde_json::json!({
            "stdout": stdout,
            "stderr": stderr,
            "exit_code": output.status.code()
        }))
    }

    fn search_files(&self, args: Value) -> Result<Value> {
        let query = args.get("query").and_then(|v| v.as_str()).ok_or_else(|| anyhow!("Missing query"))?;
        
        let mut results = Vec::new();
        use walkdir::WalkDir;
        for entry in WalkDir::new(&self.root_path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                let content = fs::read_to_string(entry.path());
                if let Ok(content) = content {
                    for (i, line) in content.lines().enumerate() {
                        if line.contains(query) {
                            results.push(serde_json::json!({
                                "file": entry.path().strip_prefix(&self.root_path)?.to_string_lossy().to_string(),
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
}
