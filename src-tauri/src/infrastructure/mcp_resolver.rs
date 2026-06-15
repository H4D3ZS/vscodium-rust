//! Resolve MCP toolchain binaries and validate reverse-engineering install paths.

use anyhow::{anyhow, Result};
use std::path::{Path, PathBuf};

use crate::mcp_registry::McpServerConfig;

fn is_ida_root(p: &Path) -> bool {
    p.join("ida.exe").exists()
        && (p.join("idalib.dll").exists() || p.join("libidalib.so").exists())
}

/// Resolve user input (e.g. `E:\IDA Professional 9.1`) to the IDA install root.
pub fn resolve_ida_install_dir(input: &str) -> Result<String> {
    let raw = input.trim().trim_matches('"');
    if raw.is_empty() {
        return Err(anyhow!("IDA path is empty"));
    }
    let p = PathBuf::from(raw);
    if !p.exists() {
        return Err(anyhow!(
            "Path does not exist: {raw}. Point to your IDA Pro folder (contains ida.exe and idalib.dll). IDA Free is not supported."
        ));
    }
    if is_ida_root(&p) {
        return Ok(normalize_path(&p));
    }
    if p.is_dir() {
        if let Ok(rd) = std::fs::read_dir(&p) {
            for entry in rd.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    let child = entry.path();
                    if is_ida_root(&child) {
                        return Ok(normalize_path(&child));
                    }
                }
            }
        }
        return Err(anyhow!(
            "No IDA Pro install under {raw}. Expected ida.exe + idalib.dll (paid IDA 8.3+, not IDA Free)."
        ));
    }
    Err(anyhow!("Not a directory: {raw}"))
}

/// Write `%APPDATA%\\Hex-Rays\\IDA Pro\\ida-config.json` via Hex-Rays' activation script.
pub fn ensure_idalib_activated(ida_dir: &str) -> Result<()> {
    let script = PathBuf::from(ida_dir)
        .join("idalib")
        .join("python")
        .join("py-activate-idalib.py");
    if !script.exists() {
        return Err(anyhow!(
            "Missing {}. Reinstall IDA Pro (paid) with idalib support.",
            script.display()
        ));
    }
    let py = resolve_executable("py")?;
    let output = std::process::Command::new(&py)
        .args([
            "-3",
            script.to_str().unwrap_or_default(),
            "-d",
            ida_dir,
        ])
        .output()
        .map_err(|e| anyhow!("Failed to run idalib activation: {e}"))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        let out = String::from_utf8_lossy(&output.stdout);
        return Err(anyhow!(
            "idalib activation failed: {err}{out}. Run manually: py -3 \"{}\" -d \"{ida_dir}\"",
            script.display()
        ));
    }
    Ok(())
}

fn resolve_python_console_script(name: &str) -> Result<String> {
    if let Ok(p) = which::which(name) {
        return Ok(p.to_string_lossy().to_string());
    }
    if cfg!(windows) {
        if let Ok(p) = which::which(format!("{name}.exe")) {
            return Ok(p.to_string_lossy().to_string());
        }
        for base in python_scripts_dirs() {
            let exe = base.join(format!("{name}.exe"));
            if exe.exists() {
                return Ok(exe.to_string_lossy().to_string());
            }
            let bare = base.join(name);
            if bare.exists() {
                return Ok(bare.to_string_lossy().to_string());
            }
        }
    }
    Err(anyhow!(
        "'{name}' not found after pip install. Restart the IDE or add Python Scripts to PATH."
    ))
}

fn python_scripts_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if cfg!(windows) {
        for p in [r"C:\Python314\Scripts", r"C:\Python312\Scripts"] {
            dirs.push(PathBuf::from(p));
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            dirs.push(PathBuf::from(format!(
                "{local}\\Programs\\Python\\Python314\\Scripts"
            )));
            dirs.push(PathBuf::from(format!(
                "{local}\\Programs\\Python\\Python312\\Scripts"
            )));
        }
    }
    dirs
}

fn pip_install_package(py: &str, spec: &str) -> Result<()> {
    let output = std::process::Command::new(py)
        .args(["-3", "-m", "pip", "install", "--upgrade", spec])
        .output()
        .map_err(|e| anyhow!("pip install failed to start: {e}"))?;
    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("pip install {spec} failed: {err}"));
    }
    Ok(())
}

fn prepend_path_env(env: &mut std::collections::HashMap<String, String>, dir: &str) {
    let base = std::env::var("PATH").unwrap_or_default();
    let merged = if base.is_empty() {
        dir.to_string()
    } else {
        format!("{dir};{base}")
    };
    env.insert("PATH".to_string(), merged);
}

fn is_idalib_mcp_launcher(command: &str, args: &[String]) -> bool {
    command.contains("idalib-mcp")
        || args.iter().any(|a| a == "idalib-mcp" || a.contains("ida-pro-mcp"))
}

fn is_pyghidra_mcp_launcher(command: &str, args: &[String]) -> bool {
    command.contains("pyghidra-mcp")
        || args
            .iter()
            .any(|a| a.contains("pyghidra") || a.contains("clearbluejar"))
}

fn resolve_idalib_launcher(
    cmd: &str,
    args: &[String],
    env: &mut std::collections::HashMap<String, String>,
) -> Result<(String, Vec<String>)> {
    const PKG: &str = "git+https://github.com/mrexodia/ida-pro-mcp";
    if resolve_executable("uvx").is_ok() && (cmd == "uvx" || cmd.ends_with("uvx.cmd")) {
        return Ok((resolve_executable("uvx")?, args.to_vec()));
    }
    if let Ok(script) = resolve_python_console_script("idalib-mcp") {
        let mut out_args = vec!["--stdio".to_string()];
        append_max_workers_arg(&mut out_args, env);
        return Ok((script, out_args));
    }
    let py = resolve_executable("py")?;
    pip_install_package(&py, PKG)?;
    let script = resolve_python_console_script("idalib-mcp")?;
    let mut out_args = vec!["--stdio".to_string()];
    append_max_workers_arg(&mut out_args, env);
    Ok((script, out_args))
}

fn resolve_pyghidra_launcher(_cmd: &str, proj: &Path) -> Result<(String, Vec<String>)> {
    const PKG: &str = "git+https://github.com/clearbluejar/pyghidra-mcp";
    let proj_s = proj.to_string_lossy().to_string();
    if let Ok(script) = resolve_python_console_script("pyghidra-mcp") {
        return Ok((
            script,
            vec![
                "-t".to_string(),
                "stdio".to_string(),
                "--project-path".to_string(),
                proj_s,
            ],
        ));
    }
    let py = resolve_executable("py")?;
    pip_install_package(&py, PKG)?;
    let script = resolve_python_console_script("pyghidra-mcp")?;
    Ok((
        script,
        vec![
            "-t".to_string(),
            "stdio".to_string(),
            "--project-path".to_string(),
            proj_s,
        ],
    ))
}

fn append_max_workers_arg(args: &mut Vec<String>, env: &std::collections::HashMap<String, String>) {
    if let Some(w) = env.get("IDA_MCP_MAX_WORKERS") {
        if !w.trim().is_empty() && !args.iter().any(|a| a == "--max-workers") {
            args.push("--max-workers".to_string());
            args.push(w.trim().to_string());
        }
    }
}

fn is_ghidra_root(p: &Path) -> bool {
    p.join("ghidraRun.bat").exists()
        || p.join("ghidraRun").exists()
        || p.join("support").join("launch.properties").exists()
}

/// Resolve user input (e.g. `E:\Ghidra`) to the directory containing `ghidraRun.bat`.
pub fn resolve_ghidra_install_dir(input: &str) -> Result<String> {
    let raw = input.trim().trim_matches('"');
    if raw.is_empty() {
        return Err(anyhow!("Ghidra path is empty"));
    }
    let p = PathBuf::from(raw);
    if !p.exists() {
        return Err(anyhow!(
            "Path does not exist: {raw}. Install Ghidra or point to the folder that contains ghidraRun.bat (e.g. ghidra_12.0_PUBLIC)."
        ));
    }
    if is_ghidra_root(&p) {
        return Ok(normalize_path(&p));
    }
    if p.is_dir() {
        let mut candidates: Vec<PathBuf> = Vec::new();
        if let Ok(rd) = std::fs::read_dir(&p) {
            for entry in rd.flatten() {
                if entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
                    candidates.push(entry.path());
                }
            }
        }
        candidates.sort_by(|a, b| {
            b.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .len()
                .cmp(
                    &a.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("")
                        .len(),
                )
        });
        for child in candidates {
            if is_ghidra_root(&child) {
                return Ok(normalize_path(&child));
            }
        }
        let entries: Vec<String> = std::fs::read_dir(&p)
            .ok()
            .map(|rd| {
                rd.flatten()
                    .take(8)
                    .filter_map(|e| e.file_name().into_string().ok())
                    .collect()
            })
            .unwrap_or_default();
        if entries.is_empty() {
            return Err(anyhow!(
                "{raw} is empty. Extract Ghidra there (you need ghidra_*_PUBLIC with ghidraRun.bat inside)."
            ));
        }
        return Err(anyhow!(
            "No Ghidra install under {raw}. Found: {}. Expected a ghidra_*_PUBLIC folder with ghidraRun.bat.",
            entries.join(", ")
        ));
    }
    Err(anyhow!("Not a directory: {raw}"))
}

fn normalize_path(p: &Path) -> String {
    p.to_string_lossy().replace('/', "\\")
}

/// Find an executable on PATH or common install locations (Windows GUI apps often miss user PATH).
pub fn resolve_executable(name: &str) -> Result<String> {
    if cfg!(windows) {
        let candidates = match name {
            "uvx" | "uv" => vec![
                name.to_string(),
                format!("{name}.cmd"),
                format!("{name}.exe"),
            ],
            "npx" | "npm" | "node" => vec![format!("{name}.cmd"), name.to_string()],
            "py" => vec!["py.exe".to_string(), "py".to_string()],
            "python" => vec!["python.exe".to_string(), "python".to_string()],
            _ => vec![name.to_string(), format!("{name}.exe")],
        };
        for c in &candidates {
            if let Ok(p) = which::which(c) {
                return Ok(p.to_string_lossy().to_string());
            }
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            for sub in [
                format!("{local}\\Programs\\uv\\{name}.exe"),
                format!("{local}\\Programs\\Python\\Python314\\{name}.exe"),
            ] {
                if Path::new(&sub).exists() {
                    return Ok(sub);
                }
            }
        }
        if name == "py" {
            for p in [r"C:\Python314\python.exe", r"C:\Python312\python.exe"] {
                if Path::new(p).exists() {
                    return Ok("py".to_string());
                }
            }
        }
    } else if let Ok(p) = which::which(name) {
        return Ok(p.to_string_lossy().to_string());
    }
    Err(anyhow!(
        "'{name}' not found on PATH. Install it or add it to your user PATH, then restart the IDE."
    ))
}

/// Absolute writable MCP workspace under app config (replaces `./ghidra-projects`).
pub fn mcp_workspace_dir(config_dir: &Path, sub: &str) -> PathBuf {
    let dir = config_dir.join("mcp-workspaces").join(sub);
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Normalize catalog installs before spawn: resolve binaries, Ghidra path, cwd, env.
pub fn prepare_server_config(
    config: &McpServerConfig,
    config_dir: &Path,
) -> Result<McpServerConfig> {
    match config {
        McpServerConfig::Http { .. } => Ok(config.clone()),
        McpServerConfig::Stdio {
            command,
            args,
            env,
            cwd,
            enabled,
        } => {
            let mut cmd = command.clone();
            let mut out_args = args.clone();
            let mut out_env: std::collections::HashMap<String, String> = env
                .iter()
                .filter(|(_, v)| !v.trim().is_empty())
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect();

            // IDA headless (idalib-mcp): validate install, activate idalib, resolve launcher
            if out_env.contains_key("IDA_INSTALL_DIR") || is_idalib_mcp_launcher(&cmd, &out_args) {
                let ida_in = out_env
                    .get("IDA_INSTALL_DIR")
                    .cloned()
                    .unwrap_or_else(|| "E:\\IDA Professional 9.1".to_string());
                let resolved = resolve_ida_install_dir(&ida_in)?;
                ensure_idalib_activated(&resolved)?;
                out_env.insert("IDA_INSTALL_DIR".to_string(), resolved.clone());
                prepend_path_env(&mut out_env, &resolved);

                if is_idalib_mcp_launcher(&cmd, &out_args) {
                    let (new_cmd, new_args) = resolve_idalib_launcher(&cmd, &out_args, &mut out_env)?;
                    cmd = new_cmd;
                    out_args = new_args;
                }
            } else if out_env.contains_key("GHIDRA_INSTALL_DIR")
                || is_pyghidra_mcp_launcher(&cmd, &out_args)
            {
                let ghidra_in = out_env
                    .get("GHIDRA_INSTALL_DIR")
                    .cloned()
                    .unwrap_or_else(|| "E:\\Ghidra".to_string());
                let resolved = resolve_ghidra_install_dir(&ghidra_in)?;
                out_env.insert("GHIDRA_INSTALL_DIR".to_string(), resolved);

                let proj = mcp_workspace_dir(config_dir, "ghidra-projects");
                out_args = rewrite_project_path_arg(&out_args, &proj);

                if is_pyghidra_mcp_launcher(&cmd, &out_args) && cmd == "uvx" {
                    if resolve_executable("uvx").is_ok() {
                        cmd = resolve_executable("uvx")?;
                    } else {
                        let (new_cmd, new_args) = resolve_pyghidra_launcher(&cmd, &proj)?;
                        cmd = new_cmd;
                        out_args = new_args;
                    }
                }
            } else if cmd == "uvx" || cmd == "uv" || cmd == "npx" || cmd == "npm" {
                cmd = resolve_executable(&cmd)?;
            } else if cmd == "py" || cmd == "python" {
                cmd = resolve_executable(if cmd == "py" { "py" } else { "python" })?;
            }

            let out_cwd = cwd.clone().or_else(|| {
                Some(mcp_workspace_dir(config_dir, "cwd").to_string_lossy().to_string())
            });

            Ok(McpServerConfig::Stdio {
                command: cmd,
                args: out_args,
                env: out_env,
                cwd: out_cwd,
                enabled: *enabled,
            })
        }
    }
}

fn rewrite_project_path_arg(args: &[String], proj: &Path) -> Vec<String> {
    let mut out = args.to_vec();
    if let Some(i) = out.iter().position(|a| a == "--project-path") {
        if i + 1 < out.len() {
            out[i + 1] = proj.to_string_lossy().to_string();
        }
    } else {
        out.push("--project-path".to_string());
        out.push(proj.to_string_lossy().to_string());
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ghidra_root_detection() {
        assert!(!is_ghidra_root(Path::new("/nonexistent")));
    }

    #[test]
    fn ida_root_detection() {
        assert!(!is_ida_root(Path::new("/nonexistent")));
    }
}
