use std::process::{Child, Command, Stdio};
use std::io::{Write, BufReader, BufRead};
use std::thread;
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionMetadata {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub version: String,
    pub main: String,
    pub path: PathBuf,
    #[serde(default)]
    pub activation_events: Vec<String>,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

pub struct ExtensionHostManager {
    child: Option<Child>,
    stdin: Option<std::process::ChildStdin>,
    pub extensions: Vec<ExtensionMetadata>,
}

impl ExtensionHostManager {
    pub fn new() -> Self {
        Self {
            child: None,
            stdin: None,
            extensions: Vec::new(),
        }
    }

    pub fn scan_extensions(&mut self, base_dir: PathBuf) -> std::io::Result<()> {
        if !base_dir.exists() {
            std::fs::create_dir_all(&base_dir)?;
        }

        self.extensions.clear();
        for entry in std::fs::read_dir(base_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let package_json_path = path.join("package.json");
                if package_json_path.exists() {
                    let content = std::fs::read_to_string(&package_json_path)?;
                    if let Ok(mut meta) = serde_json::from_str::<ExtensionMetadata>(&content) {
                        meta.path = path.clone();
                        // Construct ID if not present
                        if meta.id.is_empty() {
                            let publisher = meta.publisher.clone().unwrap_or_else(|| "undefined".to_string());
                            meta.id = format!("{}.{}", publisher, meta.name);
                        }
                        self.extensions.push(meta);
                    }
                }
            }
        }
        Ok(())
    }

    pub fn start(&mut self, app_handle: tauri::AppHandle) -> std::io::Result<()> {
        let mut child = Command::new("node")
            .arg("ext-host/index.js")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let mut stdin = child.stdin.take().expect("Failed to open stdin");
        let stdout = child.stdout.take().expect("Failed to open stdout");
        let stderr = child.stderr.take().expect("Failed to open stderr");
        
        // Send initial bootstrap with extensions
        let bootstrap = serde_json::json!({
            "type": "bootstrap",
            "extensions": self.extensions
        });
        stdin.write_all(serde_json::to_string(&bootstrap).unwrap().as_bytes())?;
        stdin.write_all(b"\n")?;
        stdin.flush()?;

        self.stdin = Some(stdin);
        self.child = Some(child);

        // Read from stdout in a separate thread
        let app_handle_clone = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stdout);
            for line in reader.lines() {
                if let Ok(l) = line {
                    app_handle_clone.emit("ext-host-message", l).unwrap();
                }
            }
        });

        let app_handle_err = app_handle.clone();
        thread::spawn(move || {
            let reader = BufReader::new(stderr);
            for line in reader.lines() {
                if let Ok(l) = line {
                    app_handle_err.emit("ext-host-log", l).unwrap();
                }
            }
        });

        println!("Extension Host started");
        Ok(())
    }

    pub fn send_message(&mut self, msg: String) -> std::io::Result<()> {
        if let Some(stdin) = &mut self.stdin {
            stdin.write_all(msg.as_bytes())?;
            stdin.write_all(b"\n")?;
            stdin.flush()?;
        }
        Ok(())
    }
}
