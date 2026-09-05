//! Cyber-Ifrit deploy bridge: build → sign → install → tunnel → launch a
//! Flutter or React Native app onto a **physical iPhone from Windows/Linux**,
//! with no Mac and no Xcode.
//!
//! This sidesteps the macOS-only assumptions baked into `flutter run`/
//! `react-native run-ios` (they hardcode `xcodebuild`/`xcrun`) by driving the
//! pieces ourselves:
//!
//! 1. **Build** — `flutter build ios --debug --no-codesign` / `npx react-native
//!    build-ios --mode Debug`. These compile & bundle the app *without* signing,
//!    which is the only part that truly needs a Mac.
//! 2. **Sign** — `zsign` (cross-platform) signs the `.app` with the user's own
//!    free Apple dev cert (`.p12`) + provisioning profile → a `.ipa`.
//! 3. **Install** — `go-ios` (`ios app install`) pushes the signed ipa over USB.
//! 4. **Tunnel** — `ideviceiproxy` forwards the Metro/Dart-VM port so USB hot
//!    reload works (the app on the phone talks back to the bundler on this box).
//! 5. **Launch** — `go-ios` (`ios app launch`) starts it.
//!
//! Every step streams stdout/stderr to the frontend as `iphone:deploy-log`
//! events, so the IDE shows a live build log instead of a frozen spinner.
//!
//! **Authorized-use note:** this signs the developer's *own* app with their
//! *own* certificate for on-device debugging — the standard iOS dev loop, just
//! without the Mac. It is not a signature-bypass or DRM-circumvention tool.

use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::sync::{Mutex, OnceLock};

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::{AppHandle, Emitter};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};

use crate::infrastructure::process_ext::TokioCommandExtHidden;
use crate::iphone_device::{bundled_ios_tool_roots, resolve_go_ios};

/// Live tunnel handles (ideviceiproxy). Kept so a redeploy / stop can kill them.
fn proxy_children() -> &'static Mutex<Vec<Child>> {
    static C: OnceLock<Mutex<Vec<Child>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(Vec::new()))
}

// ─── Tool resolution ─────────────────────────────────────────────────────────

/// Resolve an external binary. Order: explicit env override → the Cyber-Ifrit
/// bin dirs (`CYBER_IFRIT_BIN`, `C:\cyber-ifrit\bin`, `~/.cyber-ifrit/bin`) →
/// next to the IDE exe → `PATH`. `env_override` is the name of an env var that,
/// if set to a full path, wins outright (e.g. `ZSIGN_PATH`).
fn resolve_bin(names: &[&str], env_override: Option<&str>) -> Option<PathBuf> {
    if let Some(var) = env_override {
        if let Ok(p) = std::env::var(var) {
            let path = PathBuf::from(p);
            if path.exists() {
                return Some(path);
            }
        }
    }

    let exe_name = |n: &str| if cfg!(windows) { format!("{n}.exe") } else { n.to_string() };

    // Bundled-with-the-IDE locations first (zero user setup), then the optional
    // Cyber-Ifrit bin dirs, then PATH.
    let mut dirs: Vec<PathBuf> = bundled_ios_tool_roots();
    if let Ok(p) = std::env::var("CYBER_IFRIT_BIN") {
        dirs.push(PathBuf::from(p));
    }
    #[cfg(windows)]
    dirs.push(PathBuf::from(r"C:\cyber-ifrit\bin"));
    if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
        dirs.push(PathBuf::from(&home).join(".cyber-ifrit").join("bin"));
    }

    for dir in &dirs {
        for n in names {
            let cand = dir.join(exe_name(n));
            if cand.exists() {
                return Some(cand);
            }
        }
    }
    for n in names {
        if let Ok(p) = which::which(n) {
            return Some(p);
        }
    }
    None
}

fn zsign_path() -> Option<PathBuf> {
    resolve_bin(&["zsign"], Some("ZSIGN_PATH"))
}
fn ideviceiproxy_path() -> Option<PathBuf> {
    resolve_bin(&["ideviceiproxy"], Some("IDEVICEIPROXY_PATH"))
}
fn idevice_id_path() -> Option<PathBuf> {
    resolve_bin(&["idevice_id"], Some("IDEVICE_ID_PATH"))
}

// ─── Preflight ───────────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DeployPreflight {
    pub go_ios: Option<String>,
    pub zsign: Option<String>,
    pub ideviceiproxy: Option<String>,
    pub idevice_id: Option<String>,
    pub flutter: Option<String>,
    pub npx: Option<String>,
    pub ready_flutter: bool,
    pub ready_react_native: bool,
    /// Human-readable list of what's missing per framework.
    pub notes: Vec<String>,
}

/// Report which parts of the toolchain are present. Drives the setup UI so the
/// user learns exactly what to install (go-ios, zsign, libimobiledevice).
#[tauri::command]
pub fn iphone_deploy_preflight() -> DeployPreflight {
    let go_ios = resolve_go_ios();
    let zsign = zsign_path();
    let proxy = ideviceiproxy_path();
    let idid = idevice_id_path();
    let flutter = which::which("flutter").ok();
    let npx = which::which("npx").ok();

    let mut notes = Vec::new();
    if go_ios.is_none() {
        notes.push("go-ios ('ios') missing — needed to install & launch on device.".into());
    }
    if zsign.is_none() {
        notes.push("zsign missing — needed to sign the .app with your Apple cert.".into());
    }
    if proxy.is_none() {
        notes.push("ideviceiproxy missing — needed for USB hot reload (Metro/Dart VM tunnel).".into());
    }

    let core_ready = go_ios.is_some() && zsign.is_some();
    let ready_flutter = core_ready && flutter.is_some();
    let ready_react_native = core_ready && npx.is_some();
    if flutter.is_none() {
        notes.push("flutter not on PATH — Flutter deploy unavailable.".into());
    }
    if npx.is_none() {
        notes.push("npx not on PATH — React Native deploy unavailable.".into());
    }

    let s = |p: Option<PathBuf>| p.map(|x| x.to_string_lossy().to_string());
    DeployPreflight {
        go_ios: s(go_ios),
        zsign: s(zsign),
        ideviceiproxy: s(proxy),
        idevice_id: s(idid),
        flutter: s(flutter),
        npx: s(npx),
        ready_flutter,
        ready_react_native,
        notes,
    }
}

// ─── Deploy config ───────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct DeployConfig {
    /// "flutter" | "react-native"
    pub framework: String,
    /// App project root (where `flutter`/`npx` runs).
    pub project_dir: String,
    /// Target device UDID (from iphone_list_devices).
    pub udid: String,
    /// App bundle id, e.g. com.example.myapp — used to launch after install.
    pub bundle_id: String,
    /// Path to the signing cert (.p12).
    pub cert_p12: Option<String>,
    /// Cert password.
    pub cert_password: Option<String>,
    /// Provisioning profile (.mobileprovision).
    pub mobileprovision: Option<String>,
    /// Explicit path to the built `.app` (skips the default-location guess).
    pub app_path: Option<String>,
    /// Port to tunnel for hot reload (Metro / Dart VM). Default 8081.
    pub hot_reload_port: Option<u16>,
    /// Skip the `flutter build` / `npx build-ios` step (reuse an existing build).
    pub skip_build: Option<bool>,
}

fn emit(app: &AppHandle, phase: &str, stream: &str, line: impl Into<String>) {
    let _ = app.emit(
        "iphone:deploy-log",
        json!({ "phase": phase, "stream": stream, "line": line.into() }),
    );
}

/// Spawn a command in `cwd`, streaming stdout+stderr as deploy-log events, and
/// await completion. Returns Err on non-zero exit.
async fn run_streamed(
    app: &AppHandle,
    phase: &str,
    program: &Path,
    args: &[String],
    cwd: Option<&Path>,
) -> Result<(), String> {
    emit(app, phase, "meta", format!("$ {} {}", program.display(), args.join(" ")));

    let mut cmd = Command::new(program);
    cmd.args(args).stdout(Stdio::piped()).stderr(Stdio::piped());
    cmd.hidden_sidecar();
    if let Some(dir) = cwd {
        cmd.current_dir(dir);
    }

    let mut child = cmd.spawn().map_err(|e| format!("{phase}: spawn failed: {e}"))?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let app_o = app.clone();
    let phase_o = phase.to_string();
    let out_task = tokio::spawn(async move {
        if let Some(o) = stdout {
            let mut lines = BufReader::new(o).lines();
            while let Ok(Some(l)) = lines.next_line().await {
                emit(&app_o, &phase_o, "stdout", l);
            }
        }
    });
    let app_e = app.clone();
    let phase_e = phase.to_string();
    let err_task = tokio::spawn(async move {
        if let Some(e) = stderr {
            let mut lines = BufReader::new(e).lines();
            while let Ok(Some(l)) = lines.next_line().await {
                emit(&app_e, &phase_e, "stderr", l);
            }
        }
    });

    let status = child.wait().await.map_err(|e| format!("{phase}: {e}"))?;
    let _ = out_task.await;
    let _ = err_task.await;

    if status.success() {
        Ok(())
    } else {
        Err(format!("{phase} failed (exit {})", status.code().unwrap_or(-1)))
    }
}

// ─── Locate the built .app ───────────────────────────────────────────────────

fn default_app_path(framework: &str, project_dir: &Path) -> Option<PathBuf> {
    match framework {
        "flutter" => {
            let p = project_dir.join("build/ios/iphoneos/Runner.app");
            p.exists().then_some(p)
        }
        "react-native" => {
            // RN's Xcode build products land under ios/build/Build/Products/
            // Debug-iphoneos/<App>.app — the app name varies, so scan for a .app.
            let dir = project_dir.join("ios/build/Build/Products/Debug-iphoneos");
            std::fs::read_dir(&dir).ok().and_then(|rd| {
                rd.flatten()
                    .map(|e| e.path())
                    .find(|p| p.extension().map(|x| x == "app").unwrap_or(false))
            })
        }
        _ => None,
    }
}

// ─── Deploy pipeline ─────────────────────────────────────────────────────────

#[derive(Serialize)]
pub struct DeployResult {
    pub ok: bool,
    pub bundle_id: String,
    pub udid: String,
    pub signed_ipa: Option<String>,
    pub hot_reload_port: u16,
    pub tunneled: bool,
}

/// Full build→sign→install→tunnel→launch. Long-running; progress arrives via
/// `iphone:deploy-log` events. Returns a summary (or the failing step's error).
#[tauri::command]
pub async fn iphone_deploy(app: AppHandle, config: DeployConfig) -> Result<DeployResult, String> {
    let project_dir = PathBuf::from(&config.project_dir);
    if !project_dir.is_dir() {
        return Err(format!("Project dir not found: {}", config.project_dir));
    }
    let port = config.hot_reload_port.unwrap_or(8081);
    let skip_build = config.skip_build.unwrap_or(false);

    let go_ios = resolve_go_ios()
        .ok_or_else(|| "go-ios ('ios') not found — install it and set GO_IOS_PATH.".to_string())?;
    let zsign = zsign_path().ok_or_else(|| {
        "zsign not found — put it in C:\\cyber-ifrit\\bin, on PATH, or set ZSIGN_PATH.".to_string()
    })?;

    emit(&app, "start", "meta", format!("Deploying {} → {}", config.framework, config.udid));

    // 1) Build (unsigned).
    if !skip_build {
        match config.framework.as_str() {
            "flutter" => {
                let flutter = which::which("flutter")
                    .map_err(|_| "flutter not on PATH".to_string())?;
                run_streamed(
                    &app, "build", &flutter,
                    &["build".into(), "ios".into(), "--debug".into(), "--no-codesign".into()],
                    Some(&project_dir),
                ).await?;
            }
            "react-native" => {
                let npx = which::which("npx").map_err(|_| "npx not on PATH".to_string())?;
                run_streamed(
                    &app, "build", &npx,
                    &["react-native".into(), "build-ios".into(), "--mode".into(), "Debug".into()],
                    Some(&project_dir),
                ).await?;
            }
            other => return Err(format!("Unknown framework '{other}' (use flutter|react-native)")),
        }
    }

    // 2) Locate the .app.
    let app_bundle = match &config.app_path {
        Some(p) => PathBuf::from(p),
        None => default_app_path(&config.framework, &project_dir)
            .ok_or_else(|| format!(
                "Built .app not found. Expected the default {} output — pass app_path explicitly.",
                config.framework
            ))?,
    };
    if !app_bundle.exists() {
        return Err(format!(".app not found at {}", app_bundle.display()));
    }
    emit(&app, "locate", "meta", format!("App bundle: {}", app_bundle.display()));

    // 3) Sign → ipa.
    let signed_ipa = project_dir.join("build").join("cyber-ifrit-signed.ipa");
    if let Some(parent) = signed_ipa.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let mut zargs: Vec<String> = Vec::new();
    if let Some(cert) = config.cert_p12.as_deref().filter(|s| !s.is_empty()) {
        zargs.push("-k".into());
        zargs.push(cert.into());
    } else {
        return Err("Signing cert (.p12) required. Provide cert_p12 (free Apple dev cert).".into());
    }
    if let Some(pw) = config.cert_password.as_deref().filter(|s| !s.is_empty()) {
        zargs.push("-p".into());
        zargs.push(pw.into());
    }
    if let Some(mp) = config.mobileprovision.as_deref().filter(|s| !s.is_empty()) {
        zargs.push("-m".into());
        zargs.push(mp.into());
    }
    zargs.push("-o".into());
    zargs.push(signed_ipa.to_string_lossy().to_string());
    zargs.push(app_bundle.to_string_lossy().to_string());
    run_streamed(&app, "sign", &zsign, &zargs, None).await?;

    // 4) Install over USB.
    run_streamed(
        &app, "install", &go_ios,
        &["app".into(), "install".into(),
          "--path".into(), signed_ipa.to_string_lossy().to_string(),
          "--udid".into(), config.udid.clone()],
        None,
    ).await?;

    // 5) Hot-reload tunnel (Metro / Dart VM). Long-lived — spawned detached and
    //    tracked so a redeploy/stop can tear it down.
    let mut tunneled = false;
    if let Some(proxy) = ideviceiproxy_path() {
        stop_proxies_inner();
        let mut cmd = Command::new(proxy);
        cmd.args([port.to_string(), port.to_string()])
            .stdout(Stdio::null())
            .stderr(Stdio::null());
        cmd.hidden_sidecar();
        match cmd.spawn() {
            Ok(child) => {
                proxy_children().lock().unwrap().push(child);
                tunneled = true;
                emit(&app, "tunnel", "meta", format!("USB hot-reload tunnel on :{port}"));
            }
            Err(e) => emit(&app, "tunnel", "stderr", format!("ideviceiproxy failed: {e}")),
        }
    } else {
        emit(&app, "tunnel", "stderr", "ideviceiproxy not found — hot reload disabled.");
    }

    // 6) Launch.
    run_streamed(
        &app, "launch", &go_ios,
        &["app".into(), "launch".into(),
          "--bundle-id".into(), config.bundle_id.clone(),
          "--udid".into(), config.udid.clone()],
        None,
    ).await?;

    emit(&app, "done", "meta", "Deploy complete — app running on device.");
    Ok(DeployResult {
        ok: true,
        bundle_id: config.bundle_id,
        udid: config.udid,
        signed_ipa: Some(signed_ipa.to_string_lossy().to_string()),
        hot_reload_port: port,
        tunneled,
    })
}

fn stop_proxies_inner() {
    let mut guard = proxy_children().lock().unwrap();
    for mut child in guard.drain(..) {
        let _ = child.start_kill();
    }
}

/// Tear down any active hot-reload tunnels.
#[tauri::command]
pub fn iphone_stop_tunnel() -> Result<String, String> {
    stop_proxies_inner();
    Ok("Hot-reload tunnel stopped".into())
}
