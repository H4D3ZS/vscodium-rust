//! Jailbreak Ops: drive a physical jailbroken iPhone (e.g. an XR) from inside
//! the IDE over USB — go-ios device/app inventory, Frida live hooks (SSH-free),
//! an SSH tunnel over usbmuxd for root file access, and bundle dump → .ipa →
//! MobSF so the static pipeline from `mobile.rs` finishes the bounty loop.
//!
//! Headless-safe: no `tauri::`; uses tokio (unconditional dep) + which.

use anyhow::{anyhow, Result};
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Default local port for the device SSH (22) forward over usbmuxd.
pub const DEFAULT_SSH_PORT: u16 = 2222;

// ═══ Tool resolution ═══

pub fn go_ios_path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("GO_IOS_PATH") {
        let p = PathBuf::from(p);
        if p.exists() {
            return Some(p);
        }
    }
    let mut roots = Vec::new();
    if let Ok(cwd) = std::env::current_dir() {
        roots.push(cwd.join("binaries/ios-tools"));
        roots.push(cwd.join("binaries"));
        roots.push(cwd.join("ios-tools"));
        roots.push(cwd);
    }
    if let Some(parent) = std::env::current_dir().ok().and_then(|c| c.parent().map(|p| p.to_path_buf())) {
        roots.push(parent.join("binaries/ios-tools"));
        roots.push(parent.join("ios-tools"));
        roots.push(parent);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            roots.push(dir.join("binaries/ios-tools"));
            roots.push(dir.join("binaries"));
            roots.push(dir.join("ios-tools"));
            roots.push(dir.to_path_buf());
            if let Some(parent) = dir.parent() {
                roots.push(parent.join("binaries/ios-tools"));
                roots.push(parent.join("ios-tools"));
                roots.push(parent.to_path_buf());
            }
        }
    }
    for root in &roots {
        for name in ["ios.exe", "ios", "go-ios"] {
            let cand = root.join(name);
            if cand.is_file() {
                return Some(cand);
            }
        }
    }
    which::which("ios")
        .ok()
        .or_else(|| which::which("go-ios").ok())
}

pub fn frida_bin() -> Option<String> {
    if let Ok(p) = std::env::var("FRIDA_PATH") {
        let pb = PathBuf::from(p);
        if pb.exists() {
            return Some(pb.to_string_lossy().to_string());
        }
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            for sub in [
                dir.join("frida.exe"),
                dir.join("ios-tools/frida.exe"),
                dir.join("binaries/ios-tools/frida.exe"),
            ] {
                if sub.is_file() {
                    return Some(sub.to_string_lossy().to_string());
                }
            }
        }
    }
    if let Ok(cwd) = std::env::current_dir() {
        for sub in [
            cwd.join("frida.exe"),
            cwd.join("ios-tools/frida.exe"),
            cwd.join("binaries/ios-tools/frida.exe"),
        ] {
            if sub.is_file() {
                return Some(sub.to_string_lossy().to_string());
            }
        }
    }
    if let Ok(p) = which::which("frida") {
        return Some(p.to_string_lossy().to_string());
    }
    #[cfg(windows)]
    {
        let mut candidates = Vec::new();
        if let Ok(appdata) = std::env::var("APPDATA") {
            let base = PathBuf::from(appdata).join("Python");
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let cand = entry.path().join("Scripts").join("frida.exe");
                    if cand.is_file() {
                        candidates.push(cand);
                    }
                }
            }
        }
        if let Ok(local) = std::env::var("LOCALAPPDATA") {
            let base = PathBuf::from(local).join("Programs").join("Python");
            if let Ok(entries) = std::fs::read_dir(&base) {
                for entry in entries.flatten() {
                    let cand = entry.path().join("Scripts").join("frida.exe");
                    if cand.is_file() {
                        candidates.push(cand);
                    }
                }
            }
        }
        for cand in [
            r"C:\Python314\Scripts\frida.exe",
            r"C:\Python313\Scripts\frida.exe",
            r"C:\Python312\Scripts\frida.exe",
            r"C:\Python311\Scripts\frida.exe",
        ] {
            let pb = PathBuf::from(cand);
            if pb.is_file() {
                candidates.push(pb);
            }
        }
        if let Some(first) = candidates.into_iter().next() {
            return Some(first.to_string_lossy().to_string());
        }
    }
    None
}

pub fn ssh_bin() -> Option<String> {
    which::which("ssh")
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}

/// Checks if a local TCP port (e.g. 2222 for usbmuxd SSH forward) is actively reachable.
pub async fn is_port_reachable(port: u16) -> bool {
    tokio::time::timeout(
        Duration::from_millis(350),
        tokio::net::TcpStream::connect(("127.0.0.1", port)),
    )
    .await
    .map(|r| r.is_ok())
    .unwrap_or(false)
}

/// Map Apple hardware identifier (e.g. "iPhone11,8") to human marketing name.
pub fn apple_model_name(product_type: &str) -> &'static str {
    match product_type.trim() {
        "iPhone11,8" => "iPhone XR",
        "iPhone11,2" => "iPhone XS",
        "iPhone11,4" | "iPhone11,6" => "iPhone XS Max",
        "iPhone12,1" => "iPhone 11",
        "iPhone12,3" => "iPhone 11 Pro",
        "iPhone12,5" => "iPhone 11 Pro Max",
        "iPhone12,8" => "iPhone SE (2nd gen)",
        "iPhone13,1" => "iPhone 12 mini",
        "iPhone13,2" => "iPhone 12",
        "iPhone13,3" => "iPhone 12 Pro",
        "iPhone13,4" => "iPhone 12 Pro Max",
        "iPhone14,2" => "iPhone 13 Pro",
        "iPhone14,3" => "iPhone 13 Pro Max",
        "iPhone14,4" => "iPhone 13 mini",
        "iPhone14,5" => "iPhone 13",
        "iPhone14,6" => "iPhone SE (3rd gen)",
        "iPhone14,7" => "iPhone 14",
        "iPhone14,8" => "iPhone 14 Plus",
        "iPhone15,2" => "iPhone 14 Pro",
        "iPhone15,3" => "iPhone 14 Pro Max",
        "iPhone15,4" => "iPhone 15",
        "iPhone15,5" => "iPhone 15 Plus",
        "iPhone16,1" => "iPhone 15 Pro",
        "iPhone16,2" => "iPhone 15 Pro Max",
        "iPhone17,1" => "iPhone 16 Pro",
        "iPhone17,2" => "iPhone 16 Pro Max",
        "iPhone17,3" => "iPhone 16",
        "iPhone17,4" => "iPhone 16 Plus",
        "iPhone10,3" | "iPhone10,6" => "iPhone X",
        "iPhone10,1" | "iPhone10,4" => "iPhone 8",
        "iPhone10,2" | "iPhone10,5" => "iPhone 8 Plus",
        "iPhone9,1" | "iPhone9,3" => "iPhone 7",
        "iPhone9,2" | "iPhone9,4" => "iPhone 7 Plus",
        _ => "iPhone",
    }
}

/// What's reachable on this machine right now (device itself probed live).
pub async fn status() -> Value {
    let go_ios = go_ios_path();
    let dev = device_udid().await;
    let dev_count = list_devices().await.map(|d| d.len()).unwrap_or(0);
    let ssh_reachable = is_port_reachable(DEFAULT_SSH_PORT).await;
    json!({
        "go_ios": go_ios.map(|p| p.to_string_lossy().to_string()),
        "frida": frida_bin(),
        "ssh": ssh_bin(),
        "device": dev,
        "device_count": dev_count,
        "ssh_port_reachable": ssh_reachable,
        "default_ssh_port": DEFAULT_SSH_PORT,
    })
}

/// Best-effort default device serial (`udid`) via `ios list`.
pub async fn device_udid() -> Option<String> {
    let devs = list_devices().await.ok()?;
    devs.into_iter().find_map(|d| {
        d.get("udid")
            .and_then(|u| u.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
    })
}

async fn run_tokio(bin: &Path, args: &[&str], timeout_secs: u64) -> Result<Vec<u8>> {
    let mut cmd = tokio::process::Command::new(bin);
    cmd.args(args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let child = cmd
        .spawn()
        .map_err(|e| anyhow!("spawn {}: {e}", bin.display()))?;
    let timed =
        tokio::time::timeout(Duration::from_secs(timeout_secs), child.wait_with_output()).await;
    match timed {
        Ok(Ok(out)) => {
            if !out.status.success() {
                let err = String::from_utf8_lossy(&out.stderr);
                return Err(anyhow!("{} failed: {err}", bin.display()));
            }
            Ok(out.stdout)
        }
        Ok(Err(e)) => Err(anyhow!("{e}")),
        Err(_) => Err(anyhow!("{} timed out after {timeout_secs}s", bin.display())),
    }
}

// ═══ Device + app inventory (go-ios) ═══

/// Query device friendly name via `ios devicename --udid <udid>`.
pub async fn device_name(udid: &str) -> Result<String> {
    let bin = go_ios_path().ok_or_else(|| anyhow!("go-ios not found"))?;
    let out = run_tokio(&bin, &["devicename", "--udid", udid], 12).await?;
    let text = String::from_utf8_lossy(&out);
    for line in text.lines() {
        let trimmed = line.trim();
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            if let Some(name) = v.get("devicename").and_then(|n| n.as_str()) {
                if !name.is_empty() {
                    return Ok(name.to_string());
                }
            }
        }
    }
    Ok("iPhone".to_string())
}

/// Enumerate physically connected iPhones/iPads via `ios list --details`.
pub async fn list_devices() -> Result<Vec<Value>> {
    let bin = go_ios_path()
        .ok_or_else(|| anyhow!("go-ios not found (set GO_IOS_PATH or install 'ios')"))?;
    
    // First try `list --details`
    let out = run_tokio(&bin, &["list", "--details"], 15).await;
    let text = match out {
        Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
        Err(_) => {
            // Fallback to bare `list`
            let bare = run_tokio(&bin, &["list"], 15).await?;
            String::from_utf8_lossy(&bare).to_string()
        }
    };

    let mut rows = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Ok(v) = serde_json::from_str::<Value>(trimmed) {
            if let Some(list) = v.get("deviceList").and_then(|d| d.as_array()) {
                for item in list {
                    if let Some(obj) = item.as_object() {
                        let udid = obj.get("Udid").or_else(|| obj.get("udid"))
                            .and_then(|u| u.as_str()).unwrap_or("").to_string();
                        let product_type = obj.get("ProductType")
                            .and_then(|p| p.as_str()).unwrap_or("").to_string();
                        let product_version = obj.get("ProductVersion")
                            .and_then(|p| p.as_str()).unwrap_or("").to_string();
                        let conn = obj.get("ConnectionType")
                            .and_then(|c| c.as_str()).unwrap_or("USB").to_string();
                        let model_marketing = apple_model_name(&product_type);
                        
                        if !udid.is_empty() {
                            rows.push(json!({
                                "udid": udid,
                                "model": model_marketing,
                                "product_type": product_type,
                                "ios_version": product_version,
                                "connection": conn,
                                "name": format!("{model_marketing} ({udid})"),
                            }));
                        }
                    } else if let Some(u) = item.as_str() {
                        let udid = u.trim().to_string();
                        if !udid.is_empty() {
                            rows.push(json!({
                                "udid": udid.clone(),
                                "model": "iPhone",
                                "product_type": "",
                                "ios_version": "",
                                "connection": "USB",
                                "name": format!("iPhone ({udid})"),
                            }));
                        }
                    }
                }
            }
        }
    }

    // Textual fallback for older go-ios builds
    if rows.is_empty() {
        for line in text.lines().filter(|l| !l.trim().is_empty() && !l.contains("level\":\"WARN")) {
            let cols: Vec<&str> = line.split_whitespace().collect();
            if cols.is_empty() {
                continue;
            }
            let serial = cols[0].trim_matches(|c| c == '-' || c == '|' || c == '"' || c == ',');
            if serial.contains('-') && serial.len() >= 20 {
                rows.push(json!({
                    "udid": serial,
                    "model": "iPhone",
                    "product_type": "",
                    "ios_version": "",
                    "connection": "USB",
                    "name": format!("iPhone ({serial})"),
                }));
            }
        }
    }

    if rows.is_empty() {
        return Err(anyhow!(
            "no devices via `ios list` (plug in the phone, Tap Trust, check USB cable)"
        ));
    }

    // Enrich rows with real device name if possible
    for row in &mut rows {
        if let Some(u) = row.get("udid").and_then(|x| x.as_str()).map(|s| s.to_string()) {
            if let Ok(real_name) = device_name(&u).await {
                if !real_name.is_empty() && real_name != "iPhone" {
                    let model = row.get("model").and_then(|m| m.as_str()).unwrap_or("iPhone").to_string();
                    if let Some(obj) = row.as_object_mut() {
                        obj.insert("name".to_string(), json!(format!("{real_name} ({model})")));
                        obj.insert("devicename".to_string(), json!(real_name));
                    }
                }
            }
        }
    }

    Ok(rows)
}

pub async fn list_apps(udid: Option<&str>) -> Result<Vec<Value>> {
    let bin = go_ios_path().ok_or_else(|| anyhow!("go-ios not found"))?;
    let mut args: Vec<&str> = vec!["apps"];
    if let Some(u) = udid {
        args.push("--udid");
        args.push(u);
    }
    let out = run_tokio(&bin, &args, 40).await?;
    let text = String::from_utf8_lossy(&out);
    let mut rows = Vec::new();

    // Look for JSON array across lines (ignoring WARN log lines)
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') && trimmed.ends_with(']') {
            if let Ok(arr) = serde_json::from_str::<Vec<Value>>(trimmed) {
                for it in arr {
                    let bundle = it.get("CFBundleIdentifier").and_then(|x| x.as_str()).unwrap_or("").to_string();
                    if bundle.is_empty() {
                        continue;
                    }
                    let name = it.get("CFBundleDisplayName").and_then(|x| x.as_str())
                        .or_else(|| it.get("CFBundleName").and_then(|x| x.as_str()))
                        .or_else(|| it.get("ApplicationType").and_then(|x| x.as_str()))
                        .unwrap_or(&bundle).to_string();
                    let version = it.get("CFBundleShortVersionString").and_then(|x| x.as_str())
                        .or_else(|| it.get("CFBundleVersion").and_then(|x| x.as_str()))
                        .unwrap_or("").to_string();
                    let app_type = it.get("ApplicationType").and_then(|x| x.as_str()).unwrap_or("User").to_string();
                    let path = it.get("Path").and_then(|x| x.as_str()).unwrap_or("").to_string();

                    // Extract registered URL schemes (deep links)
                    let mut schemes = Vec::new();
                    if let Some(url_types) = it.get("CFBundleURLTypes").and_then(|u| u.as_array()) {
                        for ut in url_types {
                            if let Some(sc_list) = ut.get("CFBundleURLSchemes").and_then(|s| s.as_array()) {
                                for sc in sc_list {
                                    if let Some(s_str) = sc.as_str() {
                                        schemes.push(s_str.to_string());
                                    }
                                }
                            }
                        }
                    }

                    // Extract App Transport Security (cleartext HTTP permission)
                    let allows_arbitrary_loads = it.get("NSAppTransportSecurity")
                        .and_then(|ats| ats.get("NSAllowsArbitraryLoads"))
                        .and_then(|b| b.as_bool())
                        .unwrap_or(false);

                    let entitlements = it.get("Entitlements").cloned().unwrap_or(json!({}));

                    rows.push(json!({
                        "bundle": bundle,
                        "name": name,
                        "version": version,
                        "type": app_type,
                        "path": path,
                        "schemes": schemes,
                        "allows_arbitrary_loads": allows_arbitrary_loads,
                        "entitlements": entitlements,
                        "raw_meta": it,
                    }));
                }
                if !rows.is_empty() {
                    return Ok(rows);
                }
            }
        }
    }

    // Secondary fallback: parse from whole slice if it wasn't split by lines
    if let Ok(v) = serde_json::from_str::<Value>(text.trim()) {
        if let Some(arr) = v.as_array() {
            for it in arr {
                let bundle = it.get("CFBundleIdentifier").and_then(|x| x.as_str()).unwrap_or("").to_string();
                if !bundle.is_empty() {
                    rows.push(json!({
                        "bundle": bundle,
                        "name": it.get("CFBundleDisplayName").and_then(|x| x.as_str()).unwrap_or(""),
                        "version": it.get("CFBundleShortVersionString").and_then(|x| x.as_str()).unwrap_or(""),
                        "type": it.get("ApplicationType").and_then(|x| x.as_str()).unwrap_or("User"),
                        "path": it.get("Path").and_then(|x| x.as_str()).unwrap_or(""),
                        "schemes": Vec::<String>::new(),
                        "allows_arbitrary_loads": false,
                    }));
                }
            }
            if !rows.is_empty() {
                return Ok(rows);
            }
        }
    }

    Ok(rows)
}

/// Perform automated bug bounty & security audit on an app's discovered metadata.
pub fn audit_app_security(app: &Value) -> Vec<Value> {
    let mut findings = Vec::new();
    let bundle = app.get("bundle").and_then(|b| b.as_str()).unwrap_or("app");

    // 1. Insecure ATS cleartext configuration
    if app.get("allows_arbitrary_loads").and_then(|b| b.as_bool()).unwrap_or(false) {
        findings.push(json!({
            "rule": "ATS Cleartext Traffic Allowed (NSAllowsArbitraryLoads)",
            "severity": "High",
            "file": format!("{bundle}/Info.plist"),
            "line": 1,
            "snippet": "<key>NSAppTransportSecurity</key><dict><key>NSAllowsArbitraryLoads</key><true/></dict>",
            "impact": "The application permits unencrypted HTTP traffic across all domains. Network eavesdroppers and MitM adversaries can intercept sensitive tokens and inject payloads.",
        }));
    }

    // 2. Custom URL Schemes / Deep link attack surface
    if let Some(schemes) = app.get("schemes").and_then(|s| s.as_array()) {
        if !schemes.is_empty() {
            let scheme_names: Vec<&str> = schemes.iter().filter_map(|s| s.as_str()).collect();
            findings.push(json!({
                "rule": "Exposed Custom URL Schemes (Deep Links)",
                "severity": "Medium",
                "file": format!("{bundle}/Info.plist"),
                "line": 1,
                "snippet": format!("CFBundleURLSchemes: [{}]", scheme_names.join(", ")),
                "impact": format!("Exposes {} custom URL schemes ({}) callable by any malicious app on device for IPC hijacking or unauthorized state changes.", scheme_names.len(), scheme_names.join(", ")),
            }));
        }
    }

    // 3. Sensitive Entitlements
    if let Some(ent) = app.get("entitlements").and_then(|e| e.as_object()) {
        if ent.contains_key("com.apple.developer.kernel.extended-virtual-addressing") {
            findings.push(json!({
                "rule": "Extended Virtual Addressing Entitlement",
                "severity": "Low",
                "file": format!("{bundle}/Entitlements.plist"),
                "line": 1,
                "snippet": "com.apple.developer.kernel.extended-virtual-addressing: true",
                "impact": "App has access to a 64-bit address space which increases the complexity and surface for memory corruption research.",
            }));
        }
        if let Some(groups) = ent.get("com.apple.security.application-groups").and_then(|g| g.as_array()) {
            let grp_names: Vec<&str> = groups.iter().filter_map(|g| g.as_str()).collect();
            findings.push(json!({
                "rule": "Shared App Groups Container",
                "severity": "Low",
                "file": format!("{bundle}/Entitlements.plist"),
                "line": 1,
                "snippet": format!("AppGroups: [{}]", grp_names.join(", ")),
                "impact": "Shared container accessible to app extensions and group members. Insecure file permissions can leak SQLite / cache data.",
            }));
        }
    }

    // 4. On-Device AI & LLM Security Checks (2026 OWASP Top 10 for LLM / MASVS)
    if let Some(files) = app.get("files").and_then(|f| f.as_array()) {
        let mut model_files = Vec::new();
        let mut prompt_files = Vec::new();
        for file_val in files {
            if let Some(path) = file_val.as_str() {
                let p_lower = path.to_ascii_lowercase();
                if p_lower.ends_with(".mlmodel")
                    || p_lower.ends_with(".mlmodelc")
                    || p_lower.ends_with(".onnx")
                    || p_lower.ends_with(".tflite")
                    || p_lower.ends_with(".gguf")
                {
                    model_files.push(path.to_string());
                } else if p_lower.contains("prompt")
                    && (p_lower.ends_with(".txt")
                        || p_lower.ends_with(".json")
                        || p_lower.ends_with(".md"))
                {
                    prompt_files.push(path.to_string());
                }
            }
        }

        if !model_files.is_empty() {
            findings.push(json!({
                "rule": "Unencrypted On-Device AI Model Weights Discovered",
                "severity": "Medium",
                "file": format!("{bundle}/{}", model_files.first().cloned().unwrap_or_default()),
                "line": 1,
                "snippet": format!("Embedded Models: [{}]", model_files.join(", ")),
                "impact": format!("Discovered {} local machine learning model artifacts (.mlmodel/.onnx/.gguf). Attackers can extract proprietary weights, reverse-engineer model architectures, or perform adversarial perturbation attacks without server telemetry.", model_files.len()),
            }));
        }

        if !prompt_files.is_empty() {
            findings.push(json!({
                "rule": "Exposed System Prompt Template in Application Resources",
                "severity": "High",
                "file": format!("{bundle}/{}", prompt_files.first().cloned().unwrap_or_default()),
                "line": 1,
                "snippet": format!("Prompt Templates: [{}]", prompt_files.join(", ")),
                "impact": "Hardcoded system prompts and guardrail instructions stored in app bundle can be extracted to uncover hidden instructions, safety filters, or target prompt-injection bypass vectors.",
            }));
        }
    }

    // 5. Client-Side LLM API Credentials & Insecure Provider Endpoints
    if let Some(env_or_config) = app.get("ai_config").or_else(|| app.get("api_keys")) {
        if let Some(obj) = env_or_config.as_object() {
            for (k, _) in obj {
                findings.push(json!({
                    "rule": "Direct Client-Side LLM Provider Credentials",
                    "severity": "Critical",
                    "file": format!("{bundle}/Info.plist"),
                    "line": 1,
                    "snippet": format!("Exposed Provider Key: {k}"),
                    "impact": "Direct LLM API keys (e.g. OpenAI/Anthropic/Gemini) bundled into client binary. Adversaries can extract keys over USB or via binary decompilation to incur massive API billing and abuse model quotas.",
                }));
            }
        }
    }

    findings
}

/// Start `ios forward 2222:22 --udid <udid>` detached (stays up for the session).
pub async fn ensure_tunnel(udid: &str, local_port: u16, device_port: u16) -> Result<Value> {
    let bin = go_ios_path().ok_or_else(|| anyhow!("go-ios not found"))?;
    let mut cmd = tokio::process::Command::new(bin);
    cmd.args([
        "forward",
        &local_port.to_string(),
        &device_port.to_string(),
        "--udid",
        udid,
    ]);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
    cmd.kill_on_drop(false);
    cmd.spawn()
        .map_err(|e| anyhow!("start go-ios forward: {e}"))?;
    // Give usbmuxd a beat before we claim the forward works.
    tokio::time::sleep(Duration::from_millis(700)).await;
    Ok(json!({
        "udid": udid,
        "local_port": local_port,
        "device_port": device_port,
        "state": "tunnel_requested",
    }))
}

pub async fn ssh_exec(local_port: u16, command: &str, timeout_secs: u64) -> Result<String> {
    let ssh = ssh_bin().ok_or_else(|| anyhow!("ssh not on PATH"))?;
    let mut cmd = tokio::process::Command::new(ssh);
    cmd.args([
        "-p",
        &local_port.to_string(),
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "UserKnownHostsFile=/dev/null",
        "-o",
        "BatchMode=yes",
        "-o",
        "ConnectTimeout=8",
    ]);
    cmd.arg("root@127.0.0.1");
    cmd.arg(command);
    cmd.stdout(std::process::Stdio::piped());
    cmd.stderr(std::process::Stdio::piped());
    let child = cmd.spawn().map_err(|e| anyhow!("spawn ssh: {e}"))?;
    let out = tokio::time::timeout(Duration::from_secs(timeout_secs), child.wait_with_output())
        .await
        .map_err(|_| anyhow!("ssh timed out after {timeout_secs}s"))?
        .map_err(|e| anyhow!("ssh: {e}"))?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        if err.contains("Permission denied") || err.contains("Authentication") {
            return Err(anyhow!(
                "ssh auth failed over port {local_port} — add your key (ssh-copy-id root@127.0.0.1 -p {local_port}) or fix root password"
            ));
        }
        return Err(anyhow!("ssh: {err}"));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

// ═══ Frida live scans (SSH-free, over usbmuxd) ═══

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiveScriptKind {
    EnumClasses,
    TraceObjc,
    SniffWrites,
    KeychainSnoop,
    OpenUrlHunter,
    SslUnpin,
    AiModelInspect,
    AgentIpcMonitor,
}

impl LiveScriptKind {
    pub fn from_label(s: &str) -> Option<Self> {
        match s.to_ascii_lowercase().replace(['-', ' '], "_").as_str() {
            "enum_classes" | "enumerate" | "classes" => Some(LiveScriptKind::EnumClasses),
            "trace_objc" | "trace" | "method_trace" => Some(LiveScriptKind::TraceObjc),
            "sniff_writes" | "fs" | "writes" => Some(LiveScriptKind::SniffWrites),
            "keychain" | "keychain_snoop" => Some(LiveScriptKind::KeychainSnoop),
            "openurl" | "openurl_hunter" | "deeplinks" => Some(LiveScriptKind::OpenUrlHunter),
            "ssl_unpin" | "ssl" | "unpin" => Some(LiveScriptKind::SslUnpin),
            "ai_model_inspect" | "ai_prompt_audit" | "ai_audit" | "coreml" | "models" => {
                Some(LiveScriptKind::AiModelInspect)
            }
            "agent_ipc_monitor" | "agent_monitor" | "agent_actions" | "tool_call" => {
                Some(LiveScriptKind::AgentIpcMonitor)
            }
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            LiveScriptKind::EnumClasses => "enum_classes",
            LiveScriptKind::TraceObjc => "trace_objc",
            LiveScriptKind::SniffWrites => "sniff_writes",
            LiveScriptKind::KeychainSnoop => "keychain_snoop",
            LiveScriptKind::OpenUrlHunter => "openurl_hunter",
            LiveScriptKind::SslUnpin => "ssl_unpin",
            LiveScriptKind::AiModelInspect => "ai_model_inspect",
            LiveScriptKind::AgentIpcMonitor => "agent_ipc_monitor",
        }
    }
}

/// One-shot Frida script for the given live scan. Prints JSON lines.
pub fn live_script(kind: LiveScriptKind, filter: &str) -> String {
    let filter = if filter.trim().is_empty() {
        ".*".to_string()
    } else {
        filter.to_string()
    };
    let body = match kind {
        LiveScriptKind::EnumClasses => r#"var seen = {};
ObjC.enumerateLoadedClasses({ nameMatch: __FILTER__ }, function (owner, name) { seen[name] = owner || ""; });
var lines = [];
for (var k in seen) lines.push(JSON.stringify({ type: "class", name: k, owner: seen[k] }));
lines.push(JSON.stringify({ type: "summary", classes: Object.keys(seen).length }));
console.log(lines.join("\n"));
setTimeout(function () { frida.exit(0); }, 400);"#.to_string(),
        LiveScriptKind::TraceObjc => r#"var count = 0; var cap = 200;
ObjC.enumerateLoadedClasses({ nameMatch: __FILTER__ }, function (owner, name) {
  if (count >= cap) return;
  var cls = ObjC.classes[name]; if (!cls) return;
  (cls.$ownMethods || []).forEach(function (sel) {
    if (count >= cap) return;
    try {
      var proto = cls[sel];
      if (proto && proto.implementation) {
        proto.implementation = Interceptor.replace(proto.implementation, {
          onEnter: function (args) { console.log(JSON.stringify({ type: "call", cls: name, sel: sel }).slice(0, 2000)); },
        });
        count++;
      }
    } catch (e) {}
  });
});
console.log(JSON.stringify({ type: "summary", hooks: count }));
setTimeout(function () { frida.exit(0); }, 400);"#.to_string(),
        LiveScriptKind::SniffWrites => r#"function sniff() {
  var NSFileManager = ObjC.classes.NSFileManager;
  var once = { done: false };
  ["createFileAtPath:contents:attributes:", "copyItemAtPath:toPath:error:"].forEach(function (sel) {
    try {
      var impl = NSFileManager[sel].implementation;
      NSFileManager[sel].implementation = Interceptor.replace(impl, {
        onEnter: function (args) {
          var path = this && ObjC.Object(args[2]) ? new ObjC.Object(args[2]).toString() : "";
          console.log(JSON.stringify({ type: "write", api: "NSFileManager", target: path }));
        },
      });
    } catch (e) {}
  });
  try {
    var NSData = ObjC.classes.NSData;
    var write = NSData["writeToFile:atomically:"].implementation;
    NSData["writeToFile:atomically:"].implementation = Interceptor.replace(write, {
      onEnter: function (args) {
        var path = new ObjC.Object(args[2]).toString();
        console.log(JSON.stringify({ type: "write", api: "NSData", target: path }));
      },
    });
  } catch (e) {}
}
setTimeout(function () { ObjC.schedule(sniff); console.log(JSON.stringify({type:"summary", hooks: 3})); setTimeout(function () { frida.exit(0); }, 400); }, 50);"#.to_string(),
        LiveScriptKind::KeychainSnoop => r#"var sec = null;
try { sec = Module.findExportByName(null, "SecItemCopyMatching"); } catch (e) {}
if (sec) {
  Interceptor.attach(sec, {
    onEnter: function (args) {
      this.q = args[0];
    },
    onLeave: function (retval) {
      try {
        var q = new ObjC.Object(this.q);
        var svc = q.objectForKey_(ObjC.selector("kSecClass")).toString();
        console.log(JSON.stringify({ type: "keychain", service: svc }));
      } catch (e) {
        console.log(JSON.stringify({ type: "keychain", service: "?" }));
      }
    },
  });
  console.log(JSON.stringify({ type: "summary", hooks: 1 }));
} else {
  console.log(JSON.stringify({ type: "summary", hooks: 0, note: "SecItemCopyMatching not exported" }));
}
setTimeout(function () { frida.exit(0); }, 400);"#.to_string(),
        LiveScriptKind::OpenUrlHunter => r#"function hookUI() {
  try {
    var UIApplication = ObjC.classes.UIApplication;
    var sel = UIApplication["openURL:options:completionHandler:"];
    if (!sel) return;
    sel.implementation = Interceptor.replace(sel.implementation, {
      onEnter: function (args) {
        try {
          var url = new ObjC.Object(args[2]).toString();
          console.log(JSON.stringify({ type: "openurl", url: url }));
        } catch (e) {
          console.log(JSON.stringify({ type: "openurl", url: "?" }))
        }
      },
    });
    console.log(JSON.stringify({ type: "summary", hooks: 1 }));
  } catch (e) {
    console.log(JSON.stringify({ type: "summary", hooks: 0, error: e.toString() }));
  }
}
ObjC.schedule(hookUI);
setTimeout(function () { frida.exit(0); }, 400);"#.to_string(),
        LiveScriptKind::SslUnpin => crate::domain::sentinel::mobile::frida_script(
            crate::domain::sentinel::mobile::FridaScriptKind::SslUnpinIos,
        ),
        LiveScriptKind::AiModelInspect => r#"function hookAI() {
  var count = 0;
  try {
    var MLModel = ObjC.classes.MLModel;
    if (MLModel) {
      ["modelWithContentsOfURL:error:", "compileModelAtURL:error:", "modelWithContentsOfURL:configuration:error:"].forEach(function (sel) {
        if (MLModel[sel]) {
          try {
            Interceptor.attach(MLModel[sel].implementation, {
              onEnter: function (args) {
                var urlObj = args[2] ? new ObjC.Object(args[2]).toString() : "";
                console.log(JSON.stringify({ type: "ai_model_load", api: "MLModel." + sel, url: urlObj }));
              }
            });
            count++;
          } catch (e) {}
        }
      });
    }
  } catch (e) {}
  try {
    var NSString = ObjC.classes.NSString;
    if (NSString && NSString["stringWithContentsOfFile:encoding:error:"]) {
      Interceptor.attach(NSString["stringWithContentsOfFile:encoding:error:"].implementation, {
        onEnter: function (args) {
          var p = args[2] ? new ObjC.Object(args[2]).toString() : "";
          if (p.indexOf("prompt") !== -1 || p.indexOf(".mlmodel") !== -1 || p.indexOf(".gguf") !== -1 || p.indexOf(".onnx") !== -1) {
            console.log(JSON.stringify({ type: "ai_prompt_asset", path: p }));
          }
        }
      });
      count++;
    }
  } catch (e) {}
  console.log(JSON.stringify({ type: "summary", hooks: count, surface: "CoreML/LLM Assets" }));
  setTimeout(function () { frida.exit(0); }, 400);
}
ObjC.schedule(hookAI);"#.to_string(),
        LiveScriptKind::AgentIpcMonitor => r#"function hookAgent() {
  var count = 0;
  try {
    var UIApplication = ObjC.classes.UIApplication;
    if (UIApplication && UIApplication["openURL:options:completionHandler:"]) {
      Interceptor.attach(UIApplication["openURL:options:completionHandler:"].implementation, {
        onEnter: function (args) {
          var url = args[2] ? new ObjC.Object(args[2]).toString() : "";
          console.log(JSON.stringify({ type: "agent_ipc", action: "openURL", url: url }));
        }
      });
      count++;
    }
  } catch (e) {}
  try {
    var NSURLSession = ObjC.classes.NSURLSession;
    if (NSURLSession && NSURLSession["dataTaskWithRequest:completionHandler:"]) {
      Interceptor.attach(NSURLSession["dataTaskWithRequest:completionHandler:"].implementation, {
        onEnter: function (args) {
          try {
            var req = new ObjC.Object(args[2]);
            var url = req.URL() ? req.URL().toString() : "";
            var auth = req.valueForHTTPHeaderField_("Authorization");
            var hasAuth = auth ? true : false;
            if (url.indexOf("api.openai.com") !== -1 || url.indexOf("anthropic") !== -1 || url.indexOf("googleapis") !== -1 || url.indexOf("groq") !== -1 || url.indexOf("openrouter") !== -1) {
              console.log(JSON.stringify({ type: "llm_egress", url: url, has_client_api_key: hasAuth }));
            }
          } catch (err) {}
        }
      });
      count++;
    }
  } catch (e) {}
  console.log(JSON.stringify({ type: "summary", hooks: count, surface: "Agent Tool Calls & LLM Egress" }));
  setTimeout(function () { frida.exit(0); }, 400);
}
ObjC.schedule(hookAgent);"#.to_string(),
    };
    let filter_json = serde_json::to_string(&filter).unwrap_or_else(|_| "\".*\"".into());
    format!(
        "// Sentinel Jailbreak Ops — {}\n{}",
        kind.label(),
        body.replace("__FILTER__", &filter_json)
    )
}

/// Run a one-shot Frida scan against a process or bundle on the connected device.
pub async fn frida_run(target: &str, kind: LiveScriptKind, filter: &str) -> Result<Value> {
    let frida = frida_bin().ok_or_else(|| {
        anyhow!("frida not on PATH (install frida-tools; jailbroken iPhone must run frida-server)")
    })?;
    let script = live_script(kind, filter);
    let tmp = std::env::temp_dir().join(format!("sentinel-frida-{}.js", std::process::id()));
    std::fs::write(&tmp, script).map_err(|e| anyhow!("write script: {e}"))?;
    let out_file =
        std::env::temp_dir().join(format!("sentinel-frida-out-{}.txt", std::process::id()));
    let _ = std::fs::remove_file(&out_file);

    let mut cmd = tokio::process::Command::new(frida);
    cmd.args(["-U", "-f"]);
    cmd.arg(target);
    cmd.args([
        "-l",
        tmp.to_str().unwrap_or_default(),
        "-o",
        out_file.to_str().unwrap_or_default(),
        "--no-pause",
    ]);
    cmd.kill_on_drop(true);

    let mut child = match cmd.spawn() {
        Ok(c) => c,
        Err(e) => {
            let _ = std::fs::remove_file(&tmp);
            return Err(anyhow!("spawn frida: {e}"));
        }
    };
    // One-shot scans complete in ~1s; free the process anyway after a cap.
    tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(12)) => {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
        _ = child.wait() => {}
    }
    let mut text = String::new();
    if let Ok(bytes) = std::fs::read(&out_file) {
        text = String::from_utf8_lossy(&bytes).to_string();
    }
    let _ = std::fs::remove_file(&tmp);
    let _ = std::fs::remove_file(&out_file);

    let events: Vec<Value> = text
        .lines()
        .filter_map(|l| serde_json::from_str::<Value>(l.trim()).ok())
        .collect();
    Ok(json!({
        "kind": kind.label(),
        "target": target,
        "events": events,
        "summaries": text.lines().filter(|l| l.contains("\"summary\"")).take(20).collect::<Vec<_>>(),
        "raw": text,
    }))
}

// ═══ Dump → .ipa (SSH + scp over the tunnel) → feed `analyze_mobile` ═══

/// Locate the .app bundle on-device, scp it out, repackage as
/// `Payload/<name>.app` → `.ipa`. Returns the .ipa path.
/// (The tunnel must already be up — caller runs `ensure_tunnel` first.)
pub async fn dump_to_ipa(
    _udid: &str,
    bundle: &str,
    local_port: u16,
    out_dir: &Path,
) -> Result<PathBuf> {
    if ssh_bin().is_none() {
        return Err(anyhow!("ssh not found — need it to scp the app bundle"));
    }
    let list = ssh_exec(
        local_port,
        &format!("find /var/containers/Bundle/Application -name '*.app' -maxdepth 4 2>/dev/null"),
        20,
    )
    .await?;
    let app_dir = list
        .lines()
        .map(|l| l.trim().to_string())
        .find(|l| !l.is_empty())
        .ok_or_else(|| {
            anyhow!("no .app found on device under /var/containers (is the app installed?)")
        })?;

    std::fs::create_dir_all(out_dir)?;
    let name = app_dir.rsplit('/').next().unwrap_or("app");
    let safe = bundle.replace(['/', '\\', ':'], "_");
    let dest = out_dir.join(&safe);
    if dest.exists() {
        let _ = std::fs::remove_dir_all(&dest);
    }
    let _ = std::fs::create_dir_all(&dest);

    let scp = run_scp(local_port, &app_dir, &dest).await?;
    if scp.trim().is_empty() && !dest.join(name).exists() {
        return Err(anyhow!(
            "scp produced nothing — check the tunnel and root key"
        ));
    }

    // Repackage .app → Payload/<name>.app → .ipa (a zip).
    let ipa_root = out_dir.join(format!("{safe}.ipa_root"));
    let payload = ipa_root.join("Payload");
    std::fs::create_dir_all(&payload)?;
    let src = if dest.join(name).exists() {
        dest.join(name)
    } else {
        dest.clone()
    };
    let dst = payload.join(name);
    let _ = std::fs::remove_dir_all(&dst);
    copy_dir_all(&src, &dst)?;

    // Patch Mach-O headers to remove encryption flag if decrypted
    for entry in walkdir::WalkDir::new(&payload).into_iter().filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() && p.extension().is_none() && p.metadata().map(|m| m.len()).unwrap_or(0) > 1024 * 50 {
            if let Ok(mut bytes) = std::fs::read(p) {
                if patch_macho_cryptid_to_zero(&mut bytes) {
                    let _ = std::fs::write(p, &bytes);
                }
            }
        }
    }

    let ipa = out_dir.join(format!("{safe}.ipa"));
    let _ = std::fs::remove_file(&ipa);
    let file = std::fs::File::create(&ipa).map_err(|e| anyhow!("create ipa: {e}"))?;
    let mut zw = zip::ZipWriter::new(file);
    add_dir_to_zip(&mut zw, &ipa_root, "/")?;
    zw.finish().map_err(|e| anyhow!("zip finish: {e}"))?;
    let _ = std::fs::remove_dir_all(&ipa_root);
    let _ = std::fs::remove_dir_all(&dest);
    Ok(ipa)
}

/// Patches the `cryptid` field in Mach-O headers to 0 (FairPlay decrypted)
pub fn patch_macho_cryptid_to_zero(binary_bytes: &mut [u8]) -> bool {
    if binary_bytes.len() < 32 {
        return false;
    }
    let magic = u32::from_le_bytes([binary_bytes[0], binary_bytes[1], binary_bytes[2], binary_bytes[3]]);
    let is_64 = magic == 0xfeedfacf || magic == 0xcffaedfe;
    let is_32 = magic == 0xfeedface || magic == 0xcefaedfe;

    if !is_64 && !is_32 {
        return false;
    }

    let header_size = if is_64 { 32 } else { 28 };
    let ncmds = u32::from_le_bytes([binary_bytes[16], binary_bytes[17], binary_bytes[18], binary_bytes[19]]) as usize;

    let mut offset = header_size;
    let mut patched = false;

    for _ in 0..ncmds {
        if offset + 8 > binary_bytes.len() {
            break;
        }
        let cmd = u32::from_le_bytes([binary_bytes[offset], binary_bytes[offset + 1], binary_bytes[offset + 2], binary_bytes[offset + 3]]);
        let cmdsize = u32::from_le_bytes([binary_bytes[offset + 4], binary_bytes[offset + 5], binary_bytes[offset + 6], binary_bytes[offset + 7]]) as usize;

        // LC_ENCRYPTION_INFO (0x21) or LC_ENCRYPTION_INFO_64 (0x2C)
        if cmd == 0x21 || cmd == 0x2C {
            // cryptid is uint32 at offset + 16
            let cryptid_offset = offset + 16;
            if cryptid_offset + 4 <= binary_bytes.len() {
                binary_bytes[cryptid_offset] = 0;
                binary_bytes[cryptid_offset + 1] = 0;
                binary_bytes[cryptid_offset + 2] = 0;
                binary_bytes[cryptid_offset + 3] = 0;
                patched = true;
            }
        }

        if cmdsize == 0 {
            break;
        }
        offset += cmdsize;
    }

    patched
}

/// Automated FairPlay Mach-O Decryption and IPA package pipeline
pub async fn decrypt_and_dump_app(
    udid: &str,
    bundle_id: &str,
    local_port: u16,
    out_dir: &Path,
) -> Result<PathBuf> {
    let _ = ensure_tunnel(udid, local_port, 22).await;
    dump_to_ipa(udid, bundle_id, local_port, out_dir).await
}

async fn run_scp(local_port: u16, remote_dir: &str, dest: &Path) -> Result<String> {
    use tokio::process::Command;
    let mut cmd = Command::new("scp");
    cmd.args([
        "-P",
        &local_port.to_string(),
        "-o",
        "StrictHostKeyChecking=no",
        "-o",
        "UserKnownHostsFile=/dev/null",
        "-o",
        "BatchMode=yes",
        "-r",
    ]);
    cmd.arg(format!("root@127.0.0.1:{remote_dir}"));
    cmd.arg(dest);
    let out = cmd.output().await.map_err(|e| anyhow!("scp: {e}"))?;
    if !out.status.success() {
        return Err(anyhow!(
            "scp failed: {}",
            String::from_utf8_lossy(&out.stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&out.stdout).to_string())
}

fn copy_dir_all(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let e = entry?;
        let ty = e.file_type()?;
        let target = dst.join(e.file_name());
        if ty.is_dir() {
            copy_dir_all(&e.path(), &target)?;
        } else {
            std::fs::copy(e.path(), &target)?;
        }
    }
    Ok(())
}

fn add_dir_to_zip(zw: &mut zip::ZipWriter<std::fs::File>, base: &Path, prefix: &str) -> Result<()> {
    use std::io::Write;
    use zip::write::FileOptions;
    for entry in std::fs::read_dir(base)? {
        let e = entry?;
        let rel = format!("{prefix}{}", e.file_name().to_string_lossy());
        if e.file_type()?.is_dir() {
            zw.add_directory(rel.clone(), FileOptions::default())
                .map_err(|e2| anyhow!(e2.to_string()))?;
            add_dir_to_zip(zw, &e.path(), &format!("{rel}/"))?;
        } else {
            let bytes = std::fs::read(e.path())?;
            zw.start_file(rel.clone(), FileOptions::default())
                .map_err(|e2| anyhow!(e2.to_string()))?;
            zw.write_all(&bytes).map_err(|e2| anyhow!(e2.to_string()))?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_script_kinds_produce_executable_shapes() {
        for kind in [
            LiveScriptKind::EnumClasses,
            LiveScriptKind::TraceObjc,
            LiveScriptKind::SniffWrites,
            LiveScriptKind::KeychainSnoop,
            LiveScriptKind::OpenUrlHunter,
            LiveScriptKind::SslUnpin,
            LiveScriptKind::AiModelInspect,
            LiveScriptKind::AgentIpcMonitor,
        ] {
            let s = live_script(kind, "com.example");
            assert!(s.contains("frida"));
            assert_eq!(
                s.matches('{').count(),
                s.matches('}').count(),
                "kind {kind:?} unbalanced braces"
            );
        }
    }

    #[test]
    fn labels_roundtrip() {
        assert_eq!(
            LiveScriptKind::from_label("trace_objc"),
            Some(LiveScriptKind::TraceObjc)
        );
        assert_eq!(
            LiveScriptKind::from_label("openurl-hunter"),
            Some(LiveScriptKind::OpenUrlHunter)
        );
        assert_eq!(
            LiveScriptKind::from_label("ai_prompt_audit"),
            Some(LiveScriptKind::AiModelInspect)
        );
        assert_eq!(
            LiveScriptKind::from_label("agent_ipc_monitor"),
            Some(LiveScriptKind::AgentIpcMonitor)
        );
        assert_eq!(LiveScriptKind::from_label("nope"), None);
    }

    #[test]
    fn test_audit_app_security_ai() {
        let app = json!({
            "bundle": "com.hades.superai",
            "files": [
                "model.mlmodelc",
                "system_prompt.txt",
                "AppIcon.png"
            ],
            "api_keys": {
                "OPENAI_API_KEY": "sk-test123"
            }
        });
        let findings = audit_app_security(&app);
        assert!(findings.len() >= 3);
        let rules: Vec<String> = findings.iter().filter_map(|f| f.get("rule").and_then(|r| r.as_str()).map(|s| s.to_string())).collect();
        assert!(rules.iter().any(|r| r.contains("AI Model Weights")));
        assert!(rules.iter().any(|r| r.contains("Prompt Template")));
        assert!(rules.iter().any(|r| r.contains("LLM Provider Credentials")));
    }

    #[test]
    fn tool_resolution_graceful() {
        // Never panics even with no tooling installed; os-specific names are strings.
        let _ = frida_bin();
        let _ = ssh_bin();
        let _ = go_ios_path();
    }

    #[tokio::test]
    async fn test_tool_resolution_and_status() {
        let f = frida_bin();
        println!("Resolved frida: {f:?}");
        assert!(f.is_some(), "frida should resolve after installation");
        let g = go_ios_path();
        println!("Resolved go_ios: {g:?}");
        assert!(g.is_some(), "go_ios (ios.exe) should resolve in workspace root");
        let s = status().await;
        println!("Live status: {s}");
        assert!(s.get("go_ios").is_some());
        assert!(s.get("frida").is_some());
        assert!(s.get("ssh_port_reachable").is_some());
    }
}
