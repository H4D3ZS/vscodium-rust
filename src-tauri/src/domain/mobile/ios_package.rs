//! Assemble, sign and install an iOS app — all host-native.
//!
//! Picks up where `ios_crosscompile` leaves off. Compilation is already proven
//! to work on Windows (clang/rustc emit ARM64 Mach-O against the bundled SDK);
//! what remains is the part Xcode normally hides:
//!
//! ```text
//!   Mach-O binary ─► Payload/App.app/  (binary + Info.plist)
//!                 ─► zsign | ldid      (signature + entitlements)
//!                 ─► zip               (.ipa)
//!                 ─► ios install       (device)
//! ```
//!
//! Two signing routes, because they solve different problems:
//! * **zsign** — a real `.p12` + `.mobileprovision`. Installs on a stock device.
//! * **ldid** — ad-hoc "fake" signature. Enough for a jailbroken device or an
//!   emulator, and needs no Apple account at all.
//!
//! An `.ipa` is just a zip with a `Payload/` directory at the root, so no Apple
//! tooling is involved anywhere in this file.

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Deserialize, Clone, Debug)]
pub struct PackageRequest {
    /// Compiled ARM64 Mach-O executable.
    pub binary: String,
    /// App name — becomes `<name>.app` and CFBundleName.
    pub app_name: String,
    /// e.g. `com.example.myapp`.
    pub bundle_id: String,
    /// Defaults to "1.0".
    pub version: Option<String>,
    /// Minimum iOS. Must match what the binary was built with, or it installs
    /// and then refuses to launch.
    pub min_os: Option<String>,
    /// `.p12` for zsign. Without it we ad-hoc sign with ldid.
    pub cert_p12: Option<String>,
    pub cert_password: Option<String>,
    pub mobileprovision: Option<String>,
    /// Where to write the `.ipa`. Defaults beside the binary.
    pub out_dir: Option<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct PackageResult {
    pub ipa: String,
    pub app_dir: String,
    /// "zsign" | "ldid" | "unsigned"
    pub signed_with: String,
    pub notes: Vec<String>,
}

/// Minimal `Info.plist`. Written as XML — the loader accepts it, and unlike a
/// binary plist it needs no `plutil`, which is macOS-only.
fn info_plist(req: &PackageRequest) -> String {
    let version = req.version.clone().unwrap_or_else(|| "1.0".into());
    let min_os = req.min_os.clone().unwrap_or_else(|| "15.0".into());
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleName</key><string>{name}</string>
  <key>CFBundleDisplayName</key><string>{name}</string>
  <key>CFBundleExecutable</key><string>{name}</string>
  <key>CFBundleIdentifier</key><string>{id}</string>
  <key>CFBundleVersion</key><string>{version}</string>
  <key>CFBundleShortVersionString</key><string>{version}</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleSupportedPlatforms</key><array><string>iPhoneOS</string></array>
  <key>MinimumOSVersion</key><string>{min_os}</string>
  <key>UIDeviceFamily</key><array><integer>1</integer><integer>2</integer></array>
  <key>UILaunchScreen</key><dict/>
</dict>
</plist>
"#,
        name = req.app_name,
        id = req.bundle_id,
        version = version,
        min_os = min_os,
    )
}

/// Ad-hoc entitlements — `get-task-allow` is what lets a debugger attach and is
/// required for a development-signed build to launch at all.
const DEV_ENTITLEMENTS: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>get-task-allow</key><true/>
</dict>
</plist>
"#;

fn find_signer(name: &str) -> Option<PathBuf> {
    super::iphone_device::find_bundled_tool(&[name]).or_else(|| which::which(name).ok())
}

/// Build `Payload/<App>.app`, sign it, zip to `.ipa`.
#[cfg_attr(feature = "tauri", tauri::command)]
pub fn ios_package_app(req: PackageRequest) -> Result<PackageResult, String> {
    let bin = PathBuf::from(req.binary.trim());
    if !bin.is_file() {
        return Err(format!("binary not found: {}", bin.display()));
    }
    let out_dir = req
        .out_dir
        .clone()
        .map(PathBuf::from)
        .unwrap_or_else(|| bin.parent().map(|p| p.to_path_buf()).unwrap_or_default());

    // Rebuild Payload/ from scratch — a stale .app from a previous run silently
    // keeps old resources and produces an .ipa that does not match the source.
    let payload = out_dir.join("Payload");
    if payload.exists() {
        std::fs::remove_dir_all(&payload).map_err(|e| format!("clear Payload: {e}"))?;
    }
    let app_dir = payload.join(format!("{}.app", req.app_name));
    std::fs::create_dir_all(&app_dir).map_err(|e| format!("create {}: {e}", app_dir.display()))?;

    // The executable inside the bundle MUST match CFBundleExecutable.
    std::fs::copy(&bin, app_dir.join(&req.app_name)).map_err(|e| format!("copy binary: {e}"))?;
    std::fs::write(app_dir.join("Info.plist"), info_plist(&req))
        .map_err(|e| format!("write Info.plist: {e}"))?;

    let mut notes = Vec::new();
    let mut signed_with = "unsigned".to_string();

    // Prefer a real signature; fall back to ad-hoc so the pipeline still yields
    // something installable on a jailbroken device or emulator.
    if let (Some(p12), Some(prov)) = (req.cert_p12.as_ref(), req.mobileprovision.as_ref()) {
        if let Some(zsign) = find_signer("zsign") {
            let mut c = crate::process_ext::hidden_command(&zsign);
            c.arg("-k").arg(p12);
            if let Some(pw) = req.cert_password.as_ref() {
                c.arg("-p").arg(pw);
            }
            c.arg("-m").arg(prov).arg(&app_dir);
            match c.output() {
                Ok(o) if o.status.success() => signed_with = "zsign".into(),
                Ok(o) => notes.push(format!(
                    "zsign failed: {}",
                    String::from_utf8_lossy(&o.stderr).trim().chars().take(300).collect::<String>()
                )),
                Err(e) => notes.push(format!("zsign could not run: {e}")),
            }
        } else {
            notes.push("zsign not found — drop zsign.exe in binaries/ios-tools/".into());
        }
    }

    if signed_with == "unsigned" {
        if let Some(ldid) = find_signer("ldid") {
            // ldid writes its scratch file as `.ldid.<arg>`, so a path with
            // directories becomes `.ldid.Payload/App.app/App` — a directory that
            // does not exist, and it aborts with an opaque _assert() in ldid.cpp.
            // Run from inside the bundle and pass bare filenames.
            let ents = app_dir.join("entitlements.plist");
            std::fs::write(&ents, DEV_ENTITLEMENTS).map_err(|e| format!("write entitlements: {e}"))?;
            match crate::process_ext::hidden_command(&ldid)
                .current_dir(&app_dir)
                .arg("-Sentitlements.plist")
                .arg(&req.app_name)
                .output()
            {
                Ok(o) if o.status.success() => {
                    signed_with = "ldid".into();
                    notes.push(
                        "Ad-hoc signed. Installs on a jailbroken device or the emulator; a stock \
                         device needs a real .p12 + .mobileprovision via zsign."
                            .into(),
                    );
                }
                Ok(o) => notes.push(format!(
                    "ldid failed: {}",
                    String::from_utf8_lossy(&o.stderr).trim().chars().take(300).collect::<String>()
                )),
                Err(e) => notes.push(format!("ldid could not run: {e}")),
            }
        } else {
            notes.push(
                "No signer found. Add zsign (real certs) or ldid (ad-hoc) to \
                 binaries/ios-tools/ — the .ipa is built but will not install unsigned."
                    .into(),
            );
        }
    }

    // Drop the scratch entitlements file so it does not ship inside the bundle.
    let _ = std::fs::remove_file(app_dir.join("entitlements.plist"));

    // An .ipa is a plain zip whose root holds Payload/.
    let ipa = out_dir.join(format!("{}.ipa", req.app_name));
    zip_dir(&payload, &ipa, &out_dir).map_err(|e| format!("zip .ipa: {e}"))?;

    Ok(PackageResult {
        ipa: ipa.display().to_string(),
        app_dir: app_dir.display().to_string(),
        signed_with,
        notes,
    })
}

/// Zip `dir` into `out`, storing paths relative to `base`.
fn zip_dir(dir: &Path, out: &Path, base: &Path) -> std::io::Result<()> {
    let file = std::fs::File::create(out)?;
    let mut zip = zip::ZipWriter::new(file);
    let opts = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    fn walk(
        zip: &mut zip::ZipWriter<std::fs::File>,
        opts: &zip::write::FileOptions,
        dir: &Path,
        base: &Path,
    ) -> std::io::Result<()> {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            let rel = path
                .strip_prefix(base)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            if path.is_dir() {
                zip.add_directory(format!("{rel}/"), *opts)?;
                walk(zip, opts, &path, base)?;
            } else {
                zip.start_file(rel, *opts)?;
                let bytes = std::fs::read(&path)?;
                zip.write_all(&bytes)?;
            }
        }
        Ok(())
    }

    walk(&mut zip, &opts, dir, base)?;
    zip.finish()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn req() -> PackageRequest {
        PackageRequest {
            binary: String::new(),
            app_name: "Demo".into(),
            bundle_id: "com.example.demo".into(),
            version: None,
            min_os: None,
            cert_p12: None,
            cert_password: None,
            mobileprovision: None,
            out_dir: None,
        }
    }

    /// CFBundleExecutable must equal the file name written into the bundle, or
    /// iOS installs the app and then fails to launch it with no useful error.
    #[test]
    fn plist_executable_matches_the_app_name() {
        let p = info_plist(&req());
        assert!(p.contains("<key>CFBundleExecutable</key><string>Demo</string>"));
        assert!(p.contains("<key>CFBundleIdentifier</key><string>com.example.demo</string>"));
    }

    #[test]
    fn plist_defaults_are_sane() {
        let p = info_plist(&req());
        assert!(p.contains("<key>MinimumOSVersion</key><string>15.0</string>"));
        assert!(p.contains("<key>CFBundleVersion</key><string>1.0</string>"));
        // Without APPL the bundle is not treated as an application.
        assert!(p.contains("<key>CFBundlePackageType</key><string>APPL</string>"));
    }

    /// The whole point of ldid here is that it needs no Apple account, but the
    /// signature is only accepted where code-signing is relaxed.
    #[test]
    fn dev_entitlements_allow_debugging() {
        assert!(DEV_ENTITLEMENTS.contains("get-task-allow"));
    }

    /// A .ipa is a zip with Payload/ at the root — anything else is rejected by
    /// the installer without explanation.
    #[test]
    fn ipa_stores_paths_under_payload() {
        let tmp = std::env::temp_dir().join("vscr-ipa-test");
        let _ = std::fs::remove_dir_all(&tmp);
        let app = tmp.join("Payload").join("Demo.app");
        std::fs::create_dir_all(&app).unwrap();
        std::fs::write(app.join("Demo"), b"MACHO").unwrap();
        let out = tmp.join("Demo.ipa");
        zip_dir(&tmp.join("Payload"), &out, &tmp).unwrap();

        let f = std::fs::File::open(&out).unwrap();
        let mut z = zip::ZipArchive::new(f).unwrap();
        let names: Vec<String> = (0..z.len()).map(|i| z.by_index(i).unwrap().name().to_string()).collect();
        assert!(names.iter().any(|n| n.starts_with("Payload/")), "got {names:?}");
        assert!(names.iter().any(|n| n == "Payload/Demo.app/Demo"), "got {names:?}");
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
