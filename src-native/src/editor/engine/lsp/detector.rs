// LSP Server Auto-Detection on Host System.

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectedLsp {
    pub language: String,
    pub command: String,
    pub args: Vec<String>,
}

/// Detect available language servers on the host system PATH
pub fn detect_language_servers() -> Vec<DetectedLsp> {
    let mut servers = Vec::new();

    // Check rust-analyzer
    if is_command_available("rust-analyzer") {
        servers.push(DetectedLsp {
            language: "rust".to_string(),
            command: "rust-analyzer".to_string(),
            args: Vec::new(),
        });
    }

    // Check Python servers
    if is_command_available("pyright-langserver") {
        servers.push(DetectedLsp {
            language: "python".to_string(),
            command: "pyright-langserver".to_string(),
            args: vec!["--stdio".to_string()],
        });
    } else if is_command_available("pylsp") {
        servers.push(DetectedLsp {
            language: "python".to_string(),
            command: "pylsp".to_string(),
            args: Vec::new(),
        });
    }

    // Check TypeScript / JavaScript
    if is_command_available("typescript-language-server") {
        servers.push(DetectedLsp {
            language: "typescript".to_string(),
            command: "typescript-language-server".to_string(),
            args: vec!["--stdio".to_string()],
        });
    }

    // Check Go
    if is_command_available("gopls") {
        servers.push(DetectedLsp {
            language: "go".to_string(),
            command: "gopls".to_string(),
            args: Vec::new(),
        });
    }

    // Check C/C++
    if is_command_available("clangd") {
        servers.push(DetectedLsp {
            language: "cpp".to_string(),
            command: "clangd".to_string(),
            args: Vec::new(),
        });
    }

    servers
}

fn is_command_available(cmd: &str) -> bool {
    let path_var = match std::env::var_os("PATH") {
        Some(p) => p,
        None => return false,
    };

    let extensions: &[&str] = if cfg!(windows) {
        &[".exe", ".cmd", ".bat", ""]
    } else {
        &[""]
    };

    for dir in std::env::split_paths(&path_var) {
        for ext in extensions {
            let candidate = dir.join(format!("{cmd}{ext}"));
            if candidate.is_file() {
                return true;
            }
        }
    }
    false
}
