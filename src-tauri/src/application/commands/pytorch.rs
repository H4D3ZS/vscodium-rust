//! PyTorch detect / install / verify — NVIDIA CUDA + AMD ROCm + CPU.

use serde_json::json;
use std::process::Stdio;

use crate::process_ext::hidden_command;

fn python_candidates() -> Vec<&'static str> {
    if cfg!(windows) {
        vec!["python", "py", "python3"]
    } else {
        vec!["python3", "python"]
    }
}

pub fn resolve_python() -> Option<String> {
    for cand in python_candidates() {
        let ok = hidden_command(cand)
            .arg("-c")
            .arg("import sys; print(sys.executable)")
            .output()
            .ok()
            .filter(|o| o.status.success());
        if let Some(out) = ok {
            let path = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !path.is_empty() {
                return Some(path);
            }
        }
    }
    None
}

fn python_version_tuple(py: &str) -> Option<(u32, u32)> {
    let out = hidden_command(py)
        .args([
            "-c",
            "import sys; print(sys.version_info.major, sys.version_info.minor)",
        ])
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parts: Vec<&str> = stdout.split_whitespace().collect();
    if parts.len() >= 2 {
        let major = parts[0].parse().ok()?;
        let minor = parts[1].parse().ok()?;
        return Some((major, minor));
    }
    None
}

fn pip_index_for_variant(variant: &str) -> Option<&'static str> {
    match variant {
        "cu124" => Some("https://download.pytorch.org/whl/cu124"),
        "cu121" => Some("https://download.pytorch.org/whl/cu121"),
        "rocm62" | "rocm" => Some("https://download.pytorch.org/whl/rocm6.2"),
        "cpu" | _ => None,
    }
}

fn detect_nvidia_gpu() -> Option<String> {
    if !cfg!(windows) && !cfg!(target_os = "linux") {
        return None;
    }
    let out = hidden_command("nvidia-smi")
        .arg("--query-gpu=name")
        .arg("--format=csv,noheader")
        .output()
        .ok()?;
    if !out.status.success() {
        return None;
    }
    let name = String::from_utf8_lossy(&out.stdout)
        .lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string();
    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn detect_amd_gpu() -> Option<String> {
    if cfg!(windows) {
        if let Ok(out) = hidden_command("wmic")
            .args(["path", "win32_VideoController", "get", "name"])
            .output()
        {
            if out.status.success() {
                for line in String::from_utf8_lossy(&out.stdout).lines() {
                    let l = line.trim();
                    if l.is_empty() || l.eq_ignore_ascii_case("name") {
                        continue;
                    }
                    let lower = l.to_lowercase();
                    if lower.contains("radeon")
                        || lower.contains("amd")
                        || lower.contains("gfx")
                    {
                        return Some(l.to_string());
                    }
                }
            }
        }
    }
    if let Ok(out) = hidden_command("rocm-smi")
        .arg("--showproductname")
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            for line in text.lines() {
                let t = line.trim();
                if !t.is_empty() && !t.starts_with('=') {
                    return Some(t.to_string());
                }
            }
        }
    }
    None
}

fn run_pip(py: &str, args: &[&str]) -> Result<(String, String), String> {
    let output = hidden_command(py)
        .arg("-m")
        .arg("pip")
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .map_err(|e| format!("pip failed: {e}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    if !output.status.success() {
        return Err(format!("{stdout}\n{stderr}"));
    }
    Ok((stdout, stderr))
}

fn install_rocm721_windows(py: &str) -> Result<(String, String), String> {
    let ver = python_version_tuple(py).ok_or("Could not read Python version")?;
    if ver < (3, 12) {
        return Err(format!(
            "AMD ROCm 7.2.1 on Windows requires Python 3.12+. Found {}.{}. See AMD docs.",
            ver.0, ver.1
        ));
    }
    let _ = run_pip(py, &["install", "--upgrade", "pip"]);
    let sdk = [
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/rocm_sdk_core-7.2.1-py3-none-win_amd64.whl",
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/rocm_sdk_devel-7.2.1-py3-none-win_amd64.whl",
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/rocm_sdk_libraries_custom-7.2.1-py3-none-win_amd64.whl",
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/rocm-7.2.1.tar.gz",
    ];
    let mut sdk_args = vec!["install", "--no-cache-dir"];
    sdk_args.extend(sdk.iter().copied());
    run_pip(py, &sdk_args)?;

    let torch = [
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/torch-2.9.1%2Brocm7.2.1-cp312-cp312-win_amd64.whl",
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/torchaudio-2.9.1%2Brocm7.2.1-cp312-cp312-win_amd64.whl",
        "https://repo.radeon.com/rocm/windows/rocm-rel-7.2.1/torchvision-0.24.1%2Brocm7.2.1-cp312-cp312-win_amd64.whl",
    ];
    let mut torch_args = vec!["install", "--no-cache-dir"];
    torch_args.extend(torch.iter().copied());
    run_pip(py, &torch_args)
}

#[tauri::command]
pub async fn pytorch_detect() -> Result<serde_json::Value, String> {
    let python = resolve_python();
    let python_version = python
        .as_ref()
        .and_then(|py| python_version_tuple(py))
        .map(|(a, b)| format!("{a}.{b}"));

    let nvidia_name = detect_nvidia_gpu();
    let amd_name = detect_amd_gpu();

    let gpu_vendor = if nvidia_name.is_some() {
        "nvidia"
    } else if amd_name.is_some() {
        "amd"
    } else {
        "none"
    };

    let mut torch_version: Option<String> = None;
    let mut torch_backend: Option<String> = None;
    if let Some(ref py) = python {
        let script = r#"try:
    import torch
    v = torch.__version__
    if "+rocm" in v or getattr(torch.version, "hip", None):
        backend = "rocm"
    elif torch.version.cuda:
        backend = "cuda"
    else:
        backend = "cpu"
    print(v + "|" + backend)
except Exception:
    pass
"#;
        if let Ok(out) = hidden_command(py).args(["-c", script]).output() {
            if out.status.success() {
                let line = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if let Some((v, b)) = line.split_once('|') {
                    if !v.is_empty() {
                        torch_version = Some(v.to_string());
                        torch_backend = Some(b.to_string());
                    }
                }
            }
        }
    }

    let recommended = if nvidia_name.is_some() {
        "cu124"
    } else if amd_name.is_some() {
        if cfg!(windows) { "rocm721" } else { "rocm62" }
    } else {
        "cpu"
    };

    Ok(json!({
        "python": python,
        "python_version": python_version,
        "torch_version": torch_version,
        "torch_backend": torch_backend,
        "cuda_available": nvidia_name.is_some(),
        "rocm_available": amd_name.is_some(),
        "gpu_vendor": gpu_vendor,
        "gpu_name": nvidia_name.clone().or(amd_name.clone()),
        "nvidia_gpu_name": nvidia_name,
        "amd_gpu_name": amd_name,
        "recommended_variant": recommended,
    }))
}

#[tauri::command]
pub async fn pytorch_install(variant: String) -> Result<serde_json::Value, String> {
    let py = resolve_python().ok_or("Python not found. Install Python 3.10+ and ensure it is on PATH.")?;
    let v = variant.to_lowercase();

    if v == "rocm721" || v == "rocm72" {
        if !cfg!(windows) {
            return Err("Use rocm62 variant on Linux. rocm721 is Windows Radeon ROCm 7.2.1.".to_string());
        }
        let (stdout, stderr) = install_rocm721_windows(&py)?;
        return Ok(json!({ "ok": true, "stdout": stdout, "stderr": stderr, "variant": v }));
    }

    let _ = run_pip(&py, &["install", "--upgrade", "pip"]);

    if v.starts_with("rocm") {
        let index = pip_index_for_variant(&v).ok_or("Unknown ROCm variant")?;
        let (stdout, stderr) = run_pip(
            &py,
            &[
                "install",
                "torch",
                "torchvision",
                "torchaudio",
                "--index-url",
                index,
            ],
        )?;
        return Ok(json!({ "ok": true, "stdout": stdout, "stderr": stderr, "variant": v }));
    }

    let index = pip_index_for_variant(&v);
    let mut args = vec!["install", "torch", "torchvision", "torchaudio"];
    if let Some(url) = index {
        args.push("--index-url");
        args.push(url);
    }
    let (stdout, stderr) = run_pip(&py, &args)?;
    Ok(json!({ "ok": true, "stdout": stdout, "stderr": stderr, "variant": v }))
}

#[tauri::command]
pub async fn pytorch_verify() -> Result<serde_json::Value, String> {
    let py = resolve_python().ok_or("Python not found")?;
    let script = r#"
import json
try:
    import torch
    version = torch.__version__
    hip = getattr(torch.version, "hip", None)
    cuda = torch.version.cuda
    if "+rocm" in version or hip:
        backend = "rocm"
    elif cuda:
        backend = "cuda"
    else:
        backend = "cpu"
    info = {
        "ok": True,
        "version": version,
        "backend": backend,
        "cuda_available": bool(torch.cuda.is_available()),
        "cuda_version": cuda,
        "hip_version": hip,
        "rocm_version": hip,
        "device_count": torch.cuda.device_count() if torch.cuda.is_available() else 0,
        "device_name": torch.cuda.get_device_name(0) if torch.cuda.is_available() else None,
    }
    x = torch.tensor([1.0, 2.0, 3.0])
    info["sample_sum"] = float(x.sum())
    print(json.dumps(info))
except Exception as e:
    print(json.dumps({"ok": False, "error": str(e)}))
"#;
    let output = hidden_command(&py)
        .args(["-c", script])
        .output()
        .map_err(|e| format!("verify failed: {e}"))?;
    let text = String::from_utf8_lossy(&output.stdout);
    serde_json::from_str(text.trim()).map_err(|e| format!("parse verify output: {e}"))
}
