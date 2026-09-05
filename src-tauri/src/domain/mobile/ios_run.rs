//! One-button iOS run — the ▶ equivalent of Android Studio, on Windows.
//!
//! Chains the four stages that already work individually into a single command
//! the UI can bind to a play button:
//!
//! ```text
//!   compile   clang/cargo -> ARM64 Mach-O   (bundled iPhoneOS.sdk)
//!   package   Payload/App.app + Info.plist -> .ipa
//!   sign      zsign (real cert) | ldid (ad-hoc)
//!   install   ios install -> launch on device
//! ```
//!
//! Every stage is reported as it happens, because a run that dies at "sign" is a
//! completely different problem from one that dies at "install", and a single
//! opaque failure string sends the user hunting in the wrong place.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::AppHandle;

use super::ios_package::{ios_package_app, PackageRequest};

#[derive(Deserialize, Clone, Debug)]
pub struct RunRequest {
    /// Project root. A Cargo.toml here selects the Rust path; otherwise the
    /// entry source file is compiled with clang.
    pub project_dir: String,
    pub app_name: String,
    pub bundle_id: String,
    pub udid: String,
    /// C/C++ entry point, when not a Cargo project.
    pub entry: Option<String>,
    pub min_os: Option<String>,
    pub cert_p12: Option<String>,
    pub cert_password: Option<String>,
    pub mobileprovision: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct RunStage {
    pub stage: String,
    pub ok: bool,
    pub detail: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct RunResult {
    pub ok: bool,
    pub ipa: String,
    pub signed_with: String,
    pub stages: Vec<RunStage>,
}

fn stage(stages: &mut Vec<RunStage>, name: &str, ok: bool, detail: impl Into<String>) {
    stages.push(RunStage { stage: name.into(), ok, detail: detail.into() });
}

/// Build, sign, install and launch on a physical device — one call.
#[tauri::command]
pub async fn ios_run(app: AppHandle, req: RunRequest) -> Result<RunResult, String> {
    let mut stages: Vec<RunStage> = Vec::new();
    let root = PathBuf::from(req.project_dir.trim());
    if !root.is_dir() {
        return Err(format!("project directory not found: {}", root.display()));
    }

    // ── 1. SDK ───────────────────────────────────────────────────────────────
    let sdk = super::ios_crosscompile::resolve_sdk_root().ok_or_else(|| {
        "No iPhoneOS SDK. It ships with the IDE; if this is a source build, use \
         “Import SDK” or set SDKROOT."
            .to_string()
    })?;
    stage(&mut stages, "sdk", true, sdk.display().to_string());

    // ── 2. Compile ───────────────────────────────────────────────────────────
    let is_cargo = root.join("Cargo.toml").is_file();
    let out_dir = root.join("target").join("ios-run");
    std::fs::create_dir_all(&out_dir).map_err(|e| format!("create out dir: {e}"))?;
    let binary = out_dir.join(&req.app_name);
    let min_os = req.min_os.clone().unwrap_or_else(|| "15.0".into());
    let target = format!("arm64-apple-ios{min_os}");

    let compile = if is_cargo {
        // The linker still has to be clang with the SDK — rustc only emits the
        // object code; ld64.lld does the Mach-O link.
        let link_args = format!(
            "-Clink-arg=-target -Clink-arg={target} -Clink-arg=-isysroot \
             -Clink-arg={sdk} -Clink-arg=-fuse-ld=lld",
            sdk = sdk.display()
        );
        let out = tokio::process::Command::new("cargo")
            .current_dir(&root)
            .args(["build", "--release", "--target", "aarch64-apple-ios"])
            .env("SDKROOT", &sdk)
            .env("CARGO_TARGET_AARCH64_APPLE_IOS_LINKER", "clang")
            .env("CARGO_TARGET_AARCH64_APPLE_IOS_RUSTFLAGS", link_args)
            .output()
            .await
            .map_err(|e| format!("cargo: {e}"))?;
        if out.status.success() {
            // Cargo names the artefact after the crate, which need not match the
            // app name — copy it to the name the bundle expects.
            let built = root
                .join("target").join("aarch64-apple-ios").join("release")
                .join(&req.app_name);
            let src = if built.is_file() { built } else { find_cargo_bin(&root)? };
            std::fs::copy(&src, &binary).map_err(|e| format!("copy artefact: {e}"))?;
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().chars().take(600).collect::<String>())
        }
    } else {
        let entry = req.entry.clone().unwrap_or_else(|| "main.c".into());
        let src = root.join(&entry);
        if !src.is_file() {
            return Err(format!("entry source not found: {}", src.display()));
        }
        let out = tokio::process::Command::new("clang")
            .args(["-target", &target, "-isysroot"])
            .arg(&sdk)
            .args(["-fuse-ld=lld", "-o"])
            .arg(&binary)
            .arg(&src)
            .output()
            .await
            .map_err(|e| format!("clang: {e} — install LLVM (winget install LLVM.LLVM)"))?;
        if out.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&out.stderr).trim().chars().take(600).collect::<String>())
        }
    };

    match compile {
        Ok(()) => stage(&mut stages, "compile", true, binary.display().to_string()),
        Err(e) => {
            stage(&mut stages, "compile", false, e.clone());
            return Ok(RunResult { ok: false, ipa: String::new(), signed_with: String::new(), stages });
        }
    }

    // ── 3. Package + sign ────────────────────────────────────────────────────
    let pkg = ios_package_app(PackageRequest {
        binary: binary.display().to_string(),
        app_name: req.app_name.clone(),
        bundle_id: req.bundle_id.clone(),
        version: None,
        min_os: Some(min_os),
        cert_p12: req.cert_p12.clone(),
        cert_password: req.cert_password.clone(),
        mobileprovision: req.mobileprovision.clone(),
        out_dir: Some(out_dir.display().to_string()),
    })?;
    stage(&mut stages, "package", true, pkg.ipa.clone());
    let signed = pkg.signed_with != "unsigned";
    stage(&mut stages, "sign", signed, if signed {
        pkg.signed_with.clone()
    } else {
        pkg.notes.join(" | ")
    });
    if !signed {
        // Installing an unsigned .ipa always fails; stop with the reason rather
        // than letting go-ios report a confusing verification error.
        return Ok(RunResult { ok: false, ipa: pkg.ipa, signed_with: pkg.signed_with, stages });
    }

    // ── 4. Install + launch ──────────────────────────────────────────────────
    let go_ios = super::iphone_device::resolve_go_ios()
        .ok_or("go-ios not found — it ships with the IDE under binaries/ios-tools")?;
    let install = tokio::process::Command::new(&go_ios)
        .args(["install", "--path"])
        .arg(&pkg.ipa)
        .args(["--udid", &req.udid])
        .env("ENABLE_GO_IOS_AGENT", "user")
        .output()
        .await
        .map_err(|e| format!("ios install: {e}"))?;
    if !install.status.success() {
        stage(&mut stages, "install", false,
            String::from_utf8_lossy(&install.stderr).trim().chars().take(400).collect::<String>());
        return Ok(RunResult { ok: false, ipa: pkg.ipa, signed_with: pkg.signed_with, stages });
    }
    stage(&mut stages, "install", true, req.udid.clone());

    let launch = tokio::process::Command::new(&go_ios)
        .args(["launch", &req.bundle_id, "--udid", &req.udid])
        .env("ENABLE_GO_IOS_AGENT", "user")
        .output()
        .await;
    match launch {
        Ok(o) if o.status.success() => stage(&mut stages, "launch", true, req.bundle_id.clone()),
        // Installed but not launched is still a useful outcome — the app is on
        // the phone and can be tapped, so do not fail the whole run.
        _ => stage(&mut stages, "launch", false,
            "installed, but could not auto-launch — open it on the device"),
    }

    let _ = &app;
    Ok(RunResult { ok: true, ipa: pkg.ipa, signed_with: pkg.signed_with, stages })
}

/// Fall back to whatever executable cargo produced when the crate name differs
/// from the app name.
fn find_cargo_bin(root: &std::path::Path) -> Result<PathBuf, String> {
    let dir = root.join("target").join("aarch64-apple-ios").join("release");
    let entries = std::fs::read_dir(&dir).map_err(|e| format!("read {}: {e}", dir.display()))?;
    for e in entries.flatten() {
        let p = e.path();
        if p.is_file() && p.extension().is_none() {
            return Ok(p);
        }
    }
    Err(format!("no executable in {}", dir.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Stage names are the contract the UI renders, and the order is how the
    /// user reads the failure. Pin both.
    #[test]
    fn stages_record_in_order_with_status() {
        let mut s = Vec::new();
        stage(&mut s, "sdk", true, "ok");
        stage(&mut s, "compile", false, "boom");
        assert_eq!(s.iter().map(|x| x.stage.as_str()).collect::<Vec<_>>(), ["sdk", "compile"]);
        assert!(s[0].ok && !s[1].ok);
        assert_eq!(s[1].detail, "boom");
    }
}
