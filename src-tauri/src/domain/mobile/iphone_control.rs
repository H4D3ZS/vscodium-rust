//! Interactive iPhone control — the input half of "Cyber-Ifrit Mirror".
//!
//! Lifecycle: ensure the iOS 17+ tunnel is up → install WebDriverAgent if
//! needed — either via `ios ui install` (manual/ASC signing) or via
//! Sideloadly/AltServer (free 7-day Apple-ID signing) → launch it
//! (`ios runwda`) → forward its port (`ios forward 8100 8100`) → talk to it
//! with [`WdaClient`]. The frontend sends tap/swipe/type/home over Tauri IPC
//! (not a WebSocket — IPC is the native, CSP-safe bridge here).

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Mutex, OnceLock};
use std::time::Duration;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::process::{Child, Command};
use tokio::sync::Mutex as AsyncMutex;

use crate::infrastructure::process_ext::TokioCommandExtHidden;
use crate::iphone_device::{device_ios_major_pub, resolve_go_ios, tunnel_is_up_pub};
use crate::domain::mobile::wda_client::WdaClient;

/// WDA's on-device HTTP port; we forward host:8100 → device:8100.
const WDA_PORT: u16 = 8100;

fn wda() -> &'static AsyncMutex<Option<WdaClient>> {
    static C: OnceLock<AsyncMutex<Option<WdaClient>>> = OnceLock::new();
    C.get_or_init(|| AsyncMutex::new(None))
}

/// ioscpy daemon port on device (forwarded via iproxy).
const IOSCPY_PORT: u16 = 27183;

/// Which control engine is active for the current session.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ControlEngine { Wda, Ioscpy }

fn active_engine() -> &'static Mutex<Option<ControlEngine>> {
    static C: OnceLock<Mutex<Option<ControlEngine>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

/// ioscpy session state for jailbroken devices. Stored when the daemon is
/// detected, used for touch/key/text without WDA or signing.
struct IoscpyControlState {
    port: u16, // forwarded local port
}

fn ioscpy_state() -> &'static Mutex<Option<IoscpyControlState>> {
    static C: OnceLock<Mutex<Option<IoscpyControlState>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(None))
}

/// Long-lived helper processes (runwda, forward). Killed on stop.
fn control_children() -> &'static Mutex<Vec<Child>> {
    static C: OnceLock<Mutex<Vec<Child>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(Vec::new()))
}

fn clog(app: &AppHandle, stream: &str, line: impl Into<String>) {
    let _ = app.emit("iphone:mirror-log", json!({ "stream": stream, "line": line.into() }));
}

fn go_ios_cmd() -> Result<Command, String> {
    let bin = resolve_go_ios().ok_or_else(|| "go-ios ('ios') not found.".to_string())?;
    let mut cmd = Command::new(bin);
    cmd.hidden_sidecar();
    Ok(cmd)
}

/// Spawn a long-lived go-ios helper, streaming its output to the mirror log.
fn spawn_helper(app: &AppHandle, args: &[&str]) -> Result<Child, String> {
    let mut cmd = go_ios_cmd()?;
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn 'ios {}': {e}", args.join(" ")))?;
    if let Some(o) = child.stdout.take() {
        let app2 = app.clone();
        tokio::spawn(async move {
            use tokio::io::AsyncBufReadExt;
            let mut lines = tokio::io::BufReader::new(o).lines();
            while let Ok(Some(l)) = lines.next_line().await { clog(&app2, "out", l); }
        });
    }
    if let Some(e) = child.stderr.take() {
        let app2 = app.clone();
        tokio::spawn(async move {
            use tokio::io::AsyncBufReadExt;
            let mut lines = tokio::io::BufReader::new(e).lines();
            while let Ok(Some(l)) = lines.next_line().await { clog(&app2, "err", l); }
        });
    }
    Ok(child)
}

// ─── Signing config (entered once, reused for WDA + deploy) ──────────────────

/// How to obtain the signing assets WDA needs.
/// - `free`: orchestrate Sideloadly / AltServer — the user signs with a free
///   Apple ID (7-day cert, no paid account needed). One manual step: the user
///   enters their Apple ID in the signer GUI. Everything else is automated.
/// - `manual`: you supply a `.p12` + `.mobileprovision` (free or paid account).
/// - `asc`: an App Store Connect API key; go-ios generates the assets itself
///   (`ios sign provision appstoreconnect`) — requires a paid membership, but
///   then WDA install is fully hands-off.
#[derive(Serialize, Deserialize, Default, Clone)]
pub struct SigningConfig {
    pub method: String, // "free" | "manual" | "asc"
    pub p12: Option<String>,
    pub p12_password: Option<String>,
    pub profile: Option<String>,
    pub asc_key_id: Option<String>,
    pub asc_issuer_id: Option<String>,
    pub asc_p8: Option<String>,
    pub bundle_id: Option<String>,
    pub signer_path: Option<String>, // custom path to Sideloadly/AltServer.exe
}

fn cyber_ifrit_dir() -> Option<PathBuf> {
    std::env::var("USERPROFILE")
        .or_else(|_| std::env::var("HOME"))
        .ok()
        .map(|h| PathBuf::from(h).join(".cyber-ifrit"))
}

fn signing_path() -> Option<PathBuf> {
    cyber_ifrit_dir().map(|d| d.join("signing.json"))
}

fn load_signing() -> Option<SigningConfig> {
    let p = signing_path()?;
    let text = std::fs::read_to_string(p).ok()?;
    serde_json::from_str(&text).ok()
}

/// Persist the signing config so WDA install (and deploy) never prompt again.
#[tauri::command]
pub fn iphone_set_signing(config: SigningConfig) -> Result<String, String> {
    let dir = cyber_ifrit_dir().ok_or("No home directory")?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let path = dir.join("signing.json");
    let json = serde_json::to_string_pretty(&config).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(format!("Signing saved to {}", path.display()))
}

/// Load the saved signing config (for prefilling the UI). None if unset.
#[tauri::command]
pub fn iphone_get_signing() -> Result<Option<SigningConfig>, String> {
    Ok(load_signing())
}

/// Default WDA bundle id go-ios installs under (used for ASC provisioning).
const WDA_BUNDLE_ID: &str = "com.facebook.WebDriverAgentRunner.xctrunner";

/// Resolve concrete (p12, password, profile) from the signing config, generating
/// them via App Store Connect when method = "asc".
async fn resolve_assets(app: &AppHandle, udid: &str, cfg: &SigningConfig)
    -> Result<(String, Option<String>, String), String>
{
    if cfg.method == "asc" {
        let key_id = cfg.asc_key_id.clone().filter(|s| !s.is_empty()).ok_or("ASC key id missing")?;
        let issuer = cfg.asc_issuer_id.clone().filter(|s| !s.is_empty()).ok_or("ASC issuer id missing")?;
        let p8 = cfg.asc_p8.clone().filter(|s| !s.is_empty()).ok_or("ASC .p8 private key path missing")?;
        let bundle = cfg.bundle_id.clone().filter(|s| !s.is_empty()).unwrap_or_else(|| WDA_BUNDLE_ID.into());

        let dir = cyber_ifrit_dir().ok_or("No home directory")?;
        std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        let p12_out = dir.join("wda.p12");
        let prof_out = dir.join("wda.mobileprovision");
        let pw = cfg.p12_password.clone().unwrap_or_default();

        clog(app, "meta", "generating signing assets via App Store Connect…");
        let mut args: Vec<String> = vec![
            "sign".into(), "provision".into(), "appstoreconnect".into(),
            "--bundleid".into(), bundle,
            "--asc-key-id".into(), key_id,
            "--asc-issuer-id".into(), issuer,
            "--asc-private-key".into(), p8,
            "--p12-output".into(), p12_out.to_string_lossy().into(),
            "--profile-output".into(), prof_out.to_string_lossy().into(),
            "--udid".into(), udid.into(),
        ];
        if !pw.is_empty() { args.push("--p12password".into()); args.push(pw.clone()); }
        let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
        let mut cmd = go_ios_cmd()?;
        let out = cmd.args(&refs).output().await.map_err(|e| format!("asc provision: {e}"))?;
        let se = String::from_utf8_lossy(&out.stderr);
        for l in se.lines() { clog(app, "out", l.to_string()); }
        if !out.status.success() {
            return Err(format!("App Store Connect provisioning failed: {}", se.trim()));
        }
        Ok((p12_out.to_string_lossy().into(), if pw.is_empty() { None } else { Some(pw) }, prof_out.to_string_lossy().into()))
    } else {
        let p12 = cfg.p12.clone().filter(|s| !s.is_empty()).ok_or("Signing: .p12 path not set")?;
        let prof = cfg.profile.clone().filter(|s| !s.is_empty()).ok_or("Signing: .mobileprovision path not set")?;
        Ok((p12, cfg.p12_password.clone().filter(|s| !s.is_empty()), prof))
    }
}

/// Run `ios ui install wda` with concrete signing assets.
async fn install_wda_with(app: &AppHandle, udid: &str, p12: &str, pw: &Option<String>, profile: &str)
    -> Result<(), String>
{
    let mut args: Vec<String> = vec![
        "ui".into(), "install".into(), "wda".into(),
        "--udid".into(), udid.into(),
        "--p12file".into(), p12.into(),
        "--profile".into(), profile.into(),
    ];
    if let Some(p) = pw.as_ref().filter(|s| !s.is_empty()) {
        args.push("--p12password".into());
        args.push(p.clone());
    }
    let refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    clog(app, "meta", "installing WebDriverAgent (ios ui install wda)…");
    let mut cmd = go_ios_cmd()?;
    let out = cmd.args(&refs).output().await.map_err(|e| format!("ui install: {e}"))?;
    let so = String::from_utf8_lossy(&out.stdout);
    let se = String::from_utf8_lossy(&out.stderr);
    for l in so.lines().chain(se.lines()) { clog(app, "out", l.to_string()); }
    if out.status.success() { Ok(()) } else { Err(format!("ui install failed: {}", se.trim())) }
}

/// Is WebDriverAgent already installed on the device?
async fn wda_installed(udid: &str) -> bool {
    let Ok(mut cmd) = go_ios_cmd() else { return false };
    match cmd.args(["apps", "--udid", udid]).output().await {
        Ok(o) => String::from_utf8_lossy(&o.stdout).contains("WebDriverAgent"),
        Err(_) => false,
    }
}

// ─── Free signing: Sideloadly / AltServer orchestration ──────────────────────

/// Find Sideloadly.exe on Windows (common install locations).
fn find_sideloadly() -> Option<PathBuf> {
    let local = std::env::var("LOCALAPPDATA").ok()?;
    let pf = std::env::var("PROGRAMFILES").ok();
    let pf86 = std::env::var("PROGRAMFILES(X86)").ok();
    [
        Some(PathBuf::from(&local).join("Sideloadly").join("Sideloadly.exe")),
        pf.as_ref().map(|p| PathBuf::from(p).join("Sideloadly").join("Sideloadly.exe")),
        pf86.as_ref().map(|p| PathBuf::from(p).join("Sideloadly").join("Sideloadly.exe")),
    ]
    .into_iter()
    .flatten()
    .find(|p| p.exists())
}

/// Find AltServer.exe on Windows (common install locations).
fn find_altserver() -> Option<PathBuf> {
    let local = std::env::var("LOCALAPPDATA").ok()?;
    let pf = std::env::var("PROGRAMFILES").ok();
    [
        Some(PathBuf::from(&local).join("AltServer").join("AltServer.exe")),
        pf.as_ref().map(|p| PathBuf::from(p).join("AltServer").join("AltServer.exe")),
    ]
    .into_iter()
    .flatten()
    .find(|p| p.exists())
}

/// The WebDriverAgent runner bundle id actually installed on the device.
///
/// go-ios defaults to `com.facebook.WebDriverAgentRunner.xctrunner`, but free
/// Apple-ID sideloading (Sideloadly/AltServer) appends the team id to keep the
/// id unique — e.g. `…​.xctrunner.8Y95T56Q2L`. `ios runwda` then cannot find the
/// test app and reports "Did not find test app … Is it installed?" while WDA is
/// sitting right there, installed and trusted. Look up the real id instead of
/// assuming the stock one.
async fn installed_wda_bundle_id(udid: &str) -> Option<String> {
    let mut cmd = go_ios_cmd().ok()?;
    let out = cmd.args(["apps", "--udid", udid]).output().await.ok()?;
    let apps: serde_json::Value = serde_json::from_slice(&out.stdout).ok()?;
    let list = apps.as_array()?;
    let mut best: Option<String> = None;
    for a in list {
        let id = a.get("CFBundleIdentifier").and_then(|v| v.as_str())?.to_string();
        if !id.contains("WebDriverAgentRunner") || !id.contains("xctrunner") {
            continue;
        }
        // Prefer an exact stock id when present; otherwise take the suffixed one.
        if id == "com.facebook.WebDriverAgentRunner.xctrunner" {
            return Some(id);
        }
        best.get_or_insert(id);
    }
    best
}

/// Find a free signing tool: custom path → Sideloadly → AltServer → PATH.
fn find_free_signer(cfg: &SigningConfig) -> Option<PathBuf> {
    // 1. Explicit path from signing config.
    if let Some(p) = cfg.signer_path.as_ref().filter(|s| !s.is_empty()) {
        let path = PathBuf::from(p);
        if path.exists() { return Some(path); }
    }
    // 2. Common install locations.
    if let Some(p) = find_sideloadly().or_else(find_altserver) {
        return Some(p);
    }
    // 3. PATH lookup.
    which::which("Sideloadly").ok()
        .or_else(|| which::which("AltServer").ok())
}

/// Download WDA via go-ios and repackage into a Sideloadly-ready .ipa.
///
/// go-ios's `ui download wda` produces a .zip with the .app at root.
/// Sideloadly needs a standard .ipa: `Payload/<name>.app/...`. This function
/// re-zips the artifact into that structure.
async fn prepare_wda_ipa(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = cyber_ifrit_dir().ok_or("No home directory")?;
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let ipa_path = dir.join("WebDriverAgent.ipa");

    // Reuse a previously prepared .ipa (avoids re-downloading).
    if ipa_path.exists() {
        clog(app, "meta", "WDA .ipa already prepared");
        return Ok(ipa_path);
    }

    // Download WDA artifact via go-ios.
    clog(app, "meta", "downloading WDA artifact…");
    let dl_dir = dir.join("wda-dl");
    std::fs::create_dir_all(&dl_dir).map_err(|e| e.to_string())?;
    let mut cmd = go_ios_cmd()?;
    let out = cmd
        .args(["ui", "download", "wda", "--output", &dl_dir.to_string_lossy()])
        .output()
        .await
        .map_err(|e| format!("download wda: {e}"))?;
    if !out.status.success() {
        let se = String::from_utf8_lossy(&out.stderr);
        return Err(format!("download WDA failed: {}", se.trim()));
    }

    // Find the downloaded .zip.
    let zip_path = std::fs::read_dir(&dl_dir)
        .map_err(|e| e.to_string())?
        .filter_map(Result::ok)
        .find(|e| e.path().extension().map_or(false, |x| x == "zip"))
        .map(|e| e.path())
        .ok_or("No .zip found in WDA download")?;

    // Repackage: prefix every entry with Payload/ to make a valid .ipa.
    clog(app, "meta", "repackaging WDA into .ipa format…");
    let src = std::fs::File::open(&zip_path).map_err(|e| e.to_string())?;
    let mut src_zip = zip::ZipArchive::new(src).map_err(|e| format!("read zip: {e}"))?;
    let dst = std::fs::File::create(&ipa_path).map_err(|e| e.to_string())?;
    let mut ipa = zip::ZipWriter::new(dst);
    let opts = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);

    for i in 0..src_zip.len() {
        let mut entry = src_zip.by_index(i).map_err(|e| format!("zip entry {i}: {e}"))?;
        let name = format!("Payload/{}", entry.name());
        if entry.is_dir() {
            ipa.add_directory(name, opts).map_err(|e| e.to_string())?;
        } else {
            ipa.start_file(&name, opts).map_err(|e| e.to_string())?;
            std::io::copy(&mut entry, &mut ipa).map_err(|e| e.to_string())?;
        }
    }
    ipa.finish().map_err(|e| e.to_string())?;

    clog(app, "meta", format!("WDA .ipa ready -> {}", ipa_path.display()));
    Ok(ipa_path)
}

/// Open the free signer (Sideloadly / AltServer) with the WDA .ipa, elevated,
/// and with the two environment problems that break Anisette pre-cleared.
///
/// 1. **A system Python on PATH hijacks Sideloadly's bundled interpreter.**
///    Sideloadly ships its own Python for the Anisette (anisette-v3) step; when
///    a newer system Python — 3.14 here — comes first on PATH, Anisette fails to
///    download and signing dies with an opaque error. Launch through a minimal
///    PATH containing only the Windows system directories.
/// 2. **The local Anisette cache corrupts.** `%LOCALAPPDATA%\Sideloadlyn`
///    holds downloaded Anisette assets; a half-written cache makes every
///    subsequent sign-in fail until it is removed. Delete it so Sideloadly
///    re-fetches cleanly.
///
/// Elevation uses the `runas` verb, so Windows shows its normal UAC consent
/// dialog — nothing here bypasses it. Because an elevated process does not
/// inherit our environment, the scrubbed PATH has to be applied by a small
/// generated .cmd that we then elevate.
fn launch_free_signer(signer: &Path, ipa: &Path) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        // Clear the Anisette cache next to the signer (…\Sideloadlyn).
        if let Some(dir) = signer.parent() {
            let anisette = dir.join("an");
            if anisette.is_dir() {
                match std::fs::remove_dir_all(&anisette) {
                    Ok(()) => println!("[ios] cleared Anisette cache {}", anisette.display()),
                    Err(e) => eprintln!("[ios] could not clear {}: {e}", anisette.display()),
                }
            }
        }

        // Minimal PATH: the system dirs only, so no third-party Python wins.
        let sys = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".into());
        let mut script = String::from("@echo off\r\n");
        script.push_str(&format!(
            "set PATH={s}\\system32;{s};{s}\\System32\\Wbem;{s}\\System32\\WindowsPowerShell\\v1.0\r\n",
            s = sys,
        ));
        script.push_str(&format!(
            "start \"\" \"{}\" \"{}\"\r\n",
            signer.display(),
            ipa.display(),
        ));
        let cmd_path = std::env::temp_dir().join("vscodium-rust-sideloadly.cmd");
        std::fs::write(&cmd_path, script)
            .map_err(|e| format!("write launcher {}: {e}", cmd_path.display()))?;

        let ps = format!(
            "Start-Process -FilePath '{}' -Verb RunAs -WindowStyle Hidden",
            cmd_path.display().to_string().replace("'", "''"),
        );
        if let Ok(st) = crate::process_ext::hidden_command("powershell")
            .args(["-NoProfile", "-NonInteractive", "-Command", &ps])
            .status()
        {
            if st.success() {
                return Ok(());
            }
        }
        // UAC declined (or PowerShell unavailable) — fall back to a plain launch
        // so the user still gets the signer window.
    }

    crate::process_ext::hidden_command(signer)
        .arg(ipa)
        .spawn()
        .map_err(|e| format!("launch {}: {e}", signer.display()))?;
    Ok(())
}

/// Fully automated WDA install: resolves (or ASC-generates) signing assets from
/// the saved config, then downloads + signs + installs WDA. For the "free"
/// method, orchestrates Sideloadly/AltServer (one manual sign step in the GUI).
#[tauri::command]
pub async fn iphone_ensure_wda(app: AppHandle, udid: String) -> Result<String, String> {
    let cfg = load_signing().ok_or(
        "No signing configured. Click \u{2699}\u{fe0f} Signing once to set your method, then it\u{2019}s automatic.",
    )?;

    // Free method: orchestrate Sideloadly / AltServer (user signs in the GUI).
    if cfg.method == "free" {
        if wda_installed(&udid).await {
            return Ok("WebDriverAgent already installed".into());
        }
        let ipa = prepare_wda_ipa(&app).await?;
        let signer = find_free_signer(&cfg).ok_or_else(|| {
            "Sideloadly or AltServer not found. Download Sideloadly from sideloadly.io, \
             install it, then retry. Or set a custom path in Signing config (signer_path)."
        })?;
        clog(&app, "meta", format!("opening {}\u{2026}", signer.display()));
        launch_free_signer(&signer, &ipa)?;
        return Ok(format!(
            "WDA prepared \u{2192} {}. Sign in with your Apple ID and click Start. \
             After it installs, trust the cert: Settings \u{2192} General \u{2192} VPN & Device Management. \
             Then click Control.",
            signer.file_name().unwrap_or_default().to_string_lossy()
        ));
    }

    // Manual / ASC: resolve assets and install via go-ios (unchanged).
    let (p12, pw, profile) = resolve_assets(&app, &udid, &cfg).await?;
    install_wda_with(&app, &udid, &p12, &pw, &profile).await?;
    Ok("WebDriverAgent installed".into())
}

#[derive(Serialize)]
pub struct ControlStartResult {
    pub ready: bool,
    pub width: f64,
    pub height: f64,
    pub wda_url: String,
}

/// Try to connect to the ioscpy daemon on a jailbroken device.
/// Forwards port 27183 via iproxy, probes the daemon, and returns the
/// forwarded port + HelloAck on success.
fn try_ioscpy_control(udid: &str) -> Result<(u16, crate::domain::mobile::ioscpy_bridge::HelloAck), String> {
    use crate::domain::mobile::ioscpy_bridge;

    // Find the go-ios binary for forwarding.
    let bin = resolve_go_ios().ok_or("go-ios not found")?;

    // Forward the ioscpy daemon port via iproxy (same approach ioscpy uses).
    // Port 0 lets the OS pick a free port.
    let out = crate::process_ext::hidden_command(&bin)
        .args(["forward", "0", &IOSCPY_PORT.to_string(), "--udid", udid])
        .output()
        .map_err(|e| format!("spawn ios forward: {e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr);

    // go-ios forward prints the mapped local port. Parse it.
    let forwarded_port = parse_forward_port(&String::from_utf8_lossy(&out.stdout), &stderr)
        .ok_or("could not determine forwarded port for ioscpy")?;

    // Give iproxy a moment to bind.
    std::thread::sleep(Duration::from_millis(300));

    // Probe the daemon.
    ioscpy_bridge::probe_daemon(forwarded_port).map(|ack| (forwarded_port, ack))
}

/// Parse the local port from `ios forward` output.
fn parse_forward_port(stdout: &str, stderr: &str) -> Option<u16> {
    // Try JSON first: {"localPort": XXXXX, ...}
    for text in [stdout, stderr] {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            if let Some(p) = v.get("localPort").and_then(|v| v.as_u64()) {
                return Some(p as u16);
            }
        }
    }
    // Try text: "forwarding 127.0.0.1:54321 to ..."
    for text in [stdout, stderr] {
        for word in text.split_whitespace() {
            if let Some(port_str) = word.rsplit(':').next() {
                if let Ok(p) = port_str.parse::<u16>() {
                    if p > 1024 { return Some(p); }
                }
            }
        }
    }
    None
}

/// Start interactive control. Auto-detects jailbroken devices (ioscpy daemon)
/// and falls back to WDA for non-jailbroken. For non-jailbroken, it
/// auto-starts the tunnel, mounts the developer image, and auto-installs WDA
/// when signing is configured — one click does everything.
#[tauri::command]
pub async fn iphone_control_start(app: AppHandle, udid: String) -> Result<ControlStartResult, String> {
    // Fresh helpers each start.
    iphone_control_stop().await.ok();

    // ── Try ioscpy first (jailbroken devices — no signing needed) ──────────
    match try_ioscpy_control(&udid) {
        Ok((port, ack)) => {
            clog(&app, "meta", format!(
                "ioscpy daemon detected ({} {}, iOS {}) \u{2014} native control active",
                ack.capabilities.device_model, ack.capabilities.jailbreak_layout, ack.capabilities.ios_version,
            ));
            *ioscpy_state().lock().unwrap() = Some(IoscpyControlState { port });
            *active_engine().lock().unwrap() = Some(ControlEngine::Ioscpy);
            return Ok(ControlStartResult {
                ready: true,
                width: 1.0,
                height: 1.0,
                wda_url: format!("ioscpy:{port}"),
            });
        }
        Err(e) => {
            clog(&app, "meta", format!("ioscpy not available ({e}), setting up WDA path\u{2026}"));
        }
    }

    // ── Non-jailbroken: full auto-setup ────────────────────────────────────
    let major = device_ios_major_pub(&udid).await.unwrap_or(0);

    // 1. Auto-start tunnel for iOS 17+ (no admin needed, userspace mode).
    if major >= 17 && !tunnel_is_up_pub().await {
        clog(&app, "meta", "iOS 17+ detected \u{2014} starting tunnel\u{2026}");
        match start_tunnel_auto(&app).await {
            Ok(msg) => clog(&app, "meta", msg),
            Err(e) => {
                return Err(format!(
                    "Could not start the tunnel automatically: {e}\n\n\
                     Manual fix:\n  1. Click \u{201c}Start Tunnel\u{201d}\n  \
                     2. Unlock your iPhone and tap Trust if asked\n  \
                     3. Click Control again"
                ));
            }
        }
    }

    // 2. Mount Developer Image for iOS 16+ (needed for screenshot + WDA).
    if major >= 16 {
        clog(&app, "meta", "mounting Developer Image\u{2026}");
        mount_developer_image(&app, &udid).await;
    }

    // 3. Auto-install WDA if not on device (uses saved signing config).
    if !wda_installed(&udid).await {
        let cfg = load_signing();
        if cfg.is_some() {
            clog(&app, "meta", "WebDriverAgent not installed - installing automatically...");
            match iphone_ensure_wda(app.clone(), udid.clone()).await {
                Ok(msg) => clog(&app, "meta", msg),
                Err(e) => {
                    return Err(format!("WDA install failed: {e}\n\nFix: Click Signing and configure your signing method."));
                }
            }
        } else {
            return Err(
                "WebDriverAgent is not installed and no signing is configured.\n\n\
                 One-time setup (click once, never again):\n  \
                 1. Click Signing\n  \
                 2. Type free (Sideloadly/AltServer), manual (.p12), or asc (App Store Connect)\n  \
                 3. Follow the prompt\n  \
                 4. Click Control again - WDA installs automatically".into()
            );
        }
    }

    // The free path only OPENS Sideloadly — the user still has to sign in and
    // press Start there, which takes minutes. Continuing straight to "launching
    // WebDriverAgent" guaranteed "Did not find test app ... Is it installed?"
    // and left a port forwarder retrying every 600ms forever. Stop here instead.
    if !wda_installed(&udid).await {
        return Err(
            "WebDriverAgent is not on the phone yet.

             Sideloadly is open — finish it there:
               1. Sign in with your Apple ID and click Start
               2. Wait for it to report success
               3. On the phone: Settings > General > VPN & Device Management > trust your developer cert
               4. Come back and click Control again

             A free Apple ID signature expires after 7 days, so this repeats weekly."
                .into(),
        );
    }

    // 4. Launch WDA and forward its port.
    clog(&app, "meta", "launching WebDriverAgent\u{2026}");
    let wda_id = installed_wda_bundle_id(&udid).await;
    let runwda = match wda_id.as_deref() {
        Some(id) if id != "com.facebook.WebDriverAgentRunner.xctrunner" => {
            clog(&app, "meta", format!("using sideloaded WDA bundle id {id}"));
            spawn_helper(
                &app,
                &[
                    "runwda",
                    "--bundleid", id,
                    "--testrunnerbundleid", id,
                    "--xctestconfig", "WebDriverAgentRunner.xctest",
                    "--udid", &udid,
                ],
            )?
        }
        _ => spawn_helper(&app, &["runwda", "--udid", &udid])?,
    };
    control_children().lock().unwrap().push(runwda);
    let fwd = spawn_helper(&app, &["forward", &WDA_PORT.to_string(), &WDA_PORT.to_string(), "--udid", &udid])?;
    control_children().lock().unwrap().push(fwd);

    // 5. Wait for WDA to answer /status.
    let base = format!("http://127.0.0.1:{WDA_PORT}");
    let mut client = WdaClient::new(&base);
    let mut ready = false;
    for _ in 0..20 {
        if client.is_ready().await { ready = true; break; }
        tokio::time::sleep(Duration::from_millis(600)).await;
    }
    if !ready {
        return Err(
            "WebDriverAgent did not respond on port 8100.\n\n\
             Try:\n  \
             1. Make sure the iPhone is unlocked\n  \
             2. If WDA just installed, trust it: Settings \u{2192} General \u{2192} VPN & Device Management\n  \
             3. Click Control again"
            .into()
        );
    }

    let (width, height) = client.window_size().await.unwrap_or((0.0, 0.0));
    *wda().lock().await = Some(client);
    *active_engine().lock().unwrap() = Some(ControlEngine::Wda);
    clog(&app, "meta", format!("control ready ({width}\u{00d7}{height} pt)"));
    Ok(ControlStartResult { ready, width, height, wda_url: base })
}

/// Spawn a task that reads a child pipe line-by-line and forwards each to the
/// mirror log.
fn pipe_lines<R>(app: AppHandle, tag: &'static str, reader: R)
where
    R: tokio::io::AsyncRead + Unpin + Send + 'static,
{
    tokio::spawn(async move {
        use tokio::io::AsyncBufReadExt;
        let mut lines = tokio::io::BufReader::new(reader).lines();
        while let Ok(Some(l)) = lines.next_line().await {
            clog(&app, tag, l);
        }
    });
}

/// Auto-start the go-ios tunnel (userspace, no admin needed).
async fn start_tunnel_auto(app: &AppHandle) -> Result<String, String> {
    let mut cmd = go_ios_cmd()?;
    cmd.args(["tunnel", "start", "--userspace"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| format!("spawn tunnel: {e}"))?;

    // Pipe output to diagnostics log.
    if let Some(o) = child.stdout.take() { pipe_lines(app.clone(), "out", o); }
    if let Some(e) = child.stderr.take() { pipe_lines(app.clone(), "err", e); }

    // Wait for tunnel to come up (up to 8 seconds).
    for _ in 0..16 {
        tokio::time::sleep(Duration::from_millis(500)).await;
        if tunnel_is_up_pub().await {
            return Ok("Tunnel started (userspace)".into());
        }
    }
    Err("Tunnel did not start within 8 seconds. Check the diagnostics log.".into())
}

/// Mount the Developer Image for iOS 16+ (needed for screenshot + WDA).
async fn mount_developer_image(app: &AppHandle, udid: &str) {
    if let Ok(mut cmd) = go_ios_cmd() {
        match cmd.args(["image", "auto", "--udid", udid]).output().await {
            Ok(o) if !o.status.success() => {
                let se = String::from_utf8_lossy(&o.stderr);
                clog(app, "err", format!("image auto: {}", se.trim()));
            }
            Err(e) => clog(app, "err", format!("image auto: {e}")),
            _ => {}
        }
    }
}

/// Stop interactive control (kills runwda + forward, drops the session).
#[tauri::command]
pub async fn iphone_control_stop() -> Result<String, String> {
    let mut kids: Vec<Child> = { std::mem::take(&mut *control_children().lock().unwrap()) };
    for child in kids.iter_mut() {
        let _ = child.start_kill();
    }
    *wda().lock().await = None;
    *ioscpy_state().lock().unwrap() = None;
    *active_engine().lock().unwrap() = None;
    Ok("Control stopped".into())
}

/// A WDA error worth retrying once with a fresh session (WDA restarted).
fn stale(e: &str) -> bool {
    e.contains("session") || e.contains("404") || e.contains("500")
}

#[tauri::command]
pub async fn iphone_wda_tap(x: f64, y: f64) -> Result<(), String> {
    if *active_engine().lock().unwrap() == Some(ControlEngine::Ioscpy) {
        return ioscpy_tap(x, y);
    }
    let mut guard = wda().lock().await;
    let c = guard.as_mut().ok_or("Control not started. Click Control first.")?;
    match c.tap(x, y).await {
        Err(e) if stale(&e) => { c.reset_session(); c.tap(x, y).await }
        r => r,
    }
}

#[tauri::command]
pub async fn iphone_wda_swipe(from_x: f64, from_y: f64, to_x: f64, to_y: f64, duration: f64) -> Result<(), String> {
    if *active_engine().lock().unwrap() == Some(ControlEngine::Ioscpy) {
        return ioscpy_swipe(from_x, from_y, to_x, to_y, duration);
    }
    let mut guard = wda().lock().await;
    let c = guard.as_mut().ok_or("Control not started.")?;
    match c.swipe(from_x, from_y, to_x, to_y, duration).await {
        Err(e) if stale(&e) => { c.reset_session(); c.swipe(from_x, from_y, to_x, to_y, duration).await }
        r => r,
    }
}

#[tauri::command]
pub async fn iphone_wda_type(text: String) -> Result<(), String> {
    if *active_engine().lock().unwrap() == Some(ControlEngine::Ioscpy) {
        return ioscpy_type(&text);
    }
    let mut guard = wda().lock().await;
    let client = guard.as_mut().ok_or("Control not started.")?;
    client.type_text(&text).await
}

#[tauri::command]
pub async fn iphone_wda_home() -> Result<String, String> {
    if *active_engine().lock().unwrap() == Some(ControlEngine::Ioscpy) {
        return ioscpy_home();
    }
    let guard = wda().lock().await;
    let client = guard.as_ref().ok_or("Control not started.")?;
    client.home().await.map(|_| "Home pressed".into())
}

// ─── ioscpy input helpers ─────────────────────────────────────────────────────

/// Connect to the ioscpy daemon, authenticate, send input, disconnect.
fn ioscpy_connect() -> Result<crate::domain::mobile::ioscpy_bridge::IoscpySession, String> {
    use crate::domain::mobile::ioscpy_bridge;
    let state = ioscpy_state().lock().unwrap();
    let s = state.as_ref().ok_or("ioscpy session not active")?;
    ioscpy_bridge::IoscpySession::connect(s.port).map(|(session, _)| session)
}

fn ioscpy_tap(x: f64, y: f64) -> Result<(), String> {
    use crate::domain::mobile::ioscpy_bridge::TouchPhase;
    let mut session = ioscpy_connect()?;
    session.send_touch(TouchPhase::Down, x as f32, y as f32)?;
    session.send_touch(TouchPhase::Up, x as f32, y as f32)?;
    Ok(())
}

fn ioscpy_swipe(from_x: f64, from_y: f64, to_x: f64, to_y: f64, _duration: f64) -> Result<(), String> {
    use crate::domain::mobile::ioscpy_bridge::TouchPhase;
    let mut session = ioscpy_connect()?;
    session.send_touch(TouchPhase::Down, from_x as f32, from_y as f32)?;
    // Intermediate move points for smooth swipe.
    let steps = 8;
    for i in 1..=steps {
        let t = i as f32 / steps as f32;
        let mx = from_x as f32 + (to_x as f32 - from_x as f32) * t;
        let my = from_y as f32 + (to_y as f32 - from_y as f32) * t;
        session.send_touch(TouchPhase::Move, mx, my)?;
    }
    session.send_touch(TouchPhase::Up, to_x as f32, to_y as f32)?;
    Ok(())
}

fn ioscpy_type(text: &str) -> Result<(), String> {
    let mut session = ioscpy_connect()?;
    session.send_text(text)
}

fn ioscpy_home() -> Result<String, String> {
    use crate::domain::mobile::ioscpy_bridge::SystemAction;
    let mut session = ioscpy_connect()?;
    session.send_action(SystemAction::Home)?;
    Ok("Home pressed (ioscpy)".into())
}
