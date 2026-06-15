use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::thread;
use tauri::Emitter;

fn resolve_ext_host_script() -> PathBuf {
    let script = "index.js";

    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for root in [
                dir.join("resources").join("ext-host"),
                dir.join("ext-host"),
            ] {
                let cand = root.join(script);
                if cand.is_file() {
                    return cand;
                }
            }
        }
    }

    if let Ok(manifest) = std::env::var("CARGO_MANIFEST_DIR") {
        let cand = PathBuf::from(manifest).join("ext-host").join(script);
        if cand.is_file() {
            return cand;
        }
    }

    if let Ok(cwd) = std::env::current_dir() {
        for root in [cwd.join("src-tauri").join("ext-host"), cwd.join("ext-host")] {
            let cand = root.join(script);
            if cand.is_file() {
                return cand;
            }
        }
    }

    PathBuf::from("ext-host").join(script)
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionMetadata {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub version: String,
    #[serde(default)]
    pub main: Option<String>,
    #[serde(default)]
    pub extension_path: PathBuf,
    #[serde(default, rename = "activationEvents")]
    pub activation_events: Vec<String>,
    #[serde(default, rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(default)]
    pub publisher: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub contributes: Option<serde_json::Value>,
    /// HADES native-API manifest ("hades" field): { capabilities: [...], contributes?: {...} }.
    /// Validated at scan/install time; the sidecar enforces capability gating.
    #[serde(default)]
    pub hades: Option<serde_json::Value>,
}

/// Known v1 capabilities — anything else in the manifest is rejected so a
/// typo'd capability fails loudly at install, not silently at runtime.
const HADES_CAPABILITIES_V1: &[&str] = &["commands", "window", "fs", "languages", "settings"];

/// Validate the optional `hades` manifest block. Returns Err with a
/// human-readable reason; extensions with an invalid block still load as
/// plain VSIX extensions, but the hades field is stripped.
pub fn validate_hades_manifest(manifest: &serde_json::Value) -> Result<(), String> {
    let obj = manifest
        .as_object()
        .ok_or("\"hades\" must be an object")?;
    let caps = obj
        .get("capabilities")
        .and_then(|c| c.as_array())
        .ok_or("\"hades.capabilities\" must be an array")?;
    for cap in caps {
        let name = cap.as_str().ok_or("capability entries must be strings")?;
        if !HADES_CAPABILITIES_V1.contains(&name) {
            return Err(format!(
                "unknown capability \"{name}\" (v1 allows: {})",
                HADES_CAPABILITIES_V1.join(", ")
            ));
        }
    }
    if let Some(contributes) = obj.get("contributes") {
        if !contributes.is_object() {
            return Err("\"hades.contributes\" must be an object".into());
        }
        for (key, val) in contributes.as_object().unwrap() {
            match key.as_str() {
                "commands" | "keybindings" | "themes" | "views" | "settings" => {
                    if !val.is_array() {
                        return Err(format!("\"hades.contributes.{key}\" must be an array"));
                    }
                }
                other => return Err(format!("unknown contribution point \"{other}\"")),
            }
        }
    }
    Ok(())
}

pub struct ExtensionHostManager {
    child: Option<Child>,
    stdin: Option<std::process::ChildStdin>,
    pub extensions: Vec<ExtensionMetadata>,
    pub extensions_dirs: Vec<PathBuf>,
}

impl ExtensionHostManager {
    pub fn new(extensions_dirs: Vec<PathBuf>) -> Self {
        Self {
            child: None,
            stdin: None,
            extensions: Vec::new(),
            extensions_dirs,
        }
    }

    pub fn primary_extensions_dir(&self) -> PathBuf {
        self.extensions_dirs[0].clone()
    }

    #[allow(dead_code)]
    pub fn scan_extensions(&mut self) -> std::io::Result<()> {
        self.extensions.clear();

        for base_dir in self.extensions_dirs.clone() {
            if !base_dir.exists() {
                continue;
            }

            for entry in std::fs::read_dir(base_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    let mut package_json_path = path.join("package.json");
                    if !package_json_path.exists() {
                        package_json_path = path.join("extension").join("package.json");
                    }

                    if package_json_path.exists() {
                        if let Ok(content) = std::fs::read_to_string(&package_json_path) {
                            if let Ok(mut meta) = serde_json::from_str::<ExtensionMetadata>(&content) {
                                meta.extension_path =
                                    package_json_path.parent().unwrap().to_path_buf();
                                if let Some(h) = &meta.hades {
                                    if let Err(reason) = validate_hades_manifest(h) {
                                        eprintln!(
                                            "[ext-host] {}: invalid hades manifest ({reason}) — native API disabled",
                                            meta.name
                                        );
                                        meta.hades = None;
                                    }
                                }
                                // Construct ID if not present
                                if meta.id.is_empty() {
                                    let publisher = meta
                                        .publisher
                                        .clone()
                                        .unwrap_or_else(|| "undefined".to_string());
                                    meta.id = format!("{}.{}", publisher, meta.name);
                                }
                                self.extensions.push(meta);
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn start(&mut self, app_handle: tauri::AppHandle) -> std::io::Result<()> {
        use crate::process_ext::CommandExtHidden;

        let script = resolve_ext_host_script();
        if !script.is_file() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Extension host script not found: {}", script.display()),
            ));
        }

        let ext_host_dir = script
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from("."));

        let mut child = Command::new("node")
            .hidden()
            .current_dir(&ext_host_dir)
            .arg(&script)
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

        println!("Extension Host started ({})", script.display());
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

    pub fn add_extension(&mut self, mut meta: ExtensionMetadata) -> std::io::Result<()> {
        if let Some(h) = &meta.hades {
            if let Err(reason) = validate_hades_manifest(h) {
                eprintln!(
                    "[ext-host] {}: invalid hades manifest ({reason}) — native API disabled",
                    meta.name
                );
                meta.hades = None;
            }
        }
        self.extensions.push(meta.clone());
        let msg = serde_json::json!({
            "type": "load_extension",
            "metadata": meta
        });
        self.send_message(serde_json::to_string(&msg).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn valid_manifest_passes() {
        let m = json!({
            "capabilities": ["commands", "window"],
            "contributes": { "commands": [{ "id": "x.y", "title": "X" }] }
        });
        assert!(validate_hades_manifest(&m).is_ok());
    }

    #[test]
    fn unknown_capability_rejects() {
        let m = json!({ "capabilities": ["commands", "network"] });
        let err = validate_hades_manifest(&m).unwrap_err();
        assert!(err.contains("network"), "{err}");
    }

    #[test]
    fn unknown_contribution_point_rejects() {
        let m = json!({ "capabilities": [], "contributes": { "telemetry": [] } });
        assert!(validate_hades_manifest(&m).is_err());
    }

    #[test]
    fn non_array_capabilities_rejects() {
        let m = json!({ "capabilities": "commands" });
        assert!(validate_hades_manifest(&m).is_err());
    }
}
