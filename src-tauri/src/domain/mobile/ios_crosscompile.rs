//! Host-native iOS builds — no macOS, no Xcode, no cloud runner.
//!
//! Apple's *compilers* are not the macOS-only part. `clang` and `rustc` both
//! carry an ARM64 Mach-O backend that works identically on Windows and Linux;
//! only Apple's packaging utilities (`xcodebuild`, `codesign`, `actool`) are
//! macOS-only, and each has a portable replacement. So a complete pipeline is:
//!
//! ```text
//!   rustc/clang  --target aarch64-apple-ios  -isysroot $SDKROOT
//!        │                         (Mach-O executable)
//!        ├─► Payload/App.app/      (Info.plist + binary + assets)
//!        └─► zsign / ldid          (entitlements + signature)
//!               │
//!               └─► .ipa ──► go-ios install ──► device
//! ```
//!
//! `iPhoneOS.sdk` — headers and `.tbd` stubs from Apple's Xcode `.xip` — is
//! resolved from, in order: `SDKROOT`, the SDK bundled beside the executable,
//! or `~/iPhoneOS.sdk`. Whether to ship one inside the installer is a packaging
//! decision (Apple's Xcode licence governs redistribution); `ios_import_sdk`
//! covers the case where it is not bundled, by copying the user's own copy into
//! the same location so both paths behave identically.
//!
//! What this module deliberately does NOT claim: `flutter build ios` and
//! `react-native run-ios` still invoke `xcodebuild` internally. Driving those
//! frameworks host-natively means compiling their AOT/JSC artefacts and doing
//! the `.app` assembly ourselves, which is a separate piece of work. This module
//! is the foundation that makes it possible, not the whole of it.

//!
//! ## Flutter on Windows — verified working
//!
//! Flutter's own tool refuses the iOS path off macOS (`Platform.isMacOS` gates
//! device discovery, and `flutter build ios` shells to `xcodebuild`). None of
//! that is needed: the pieces underneath have no such check, and the Windows
//! Flutter cache already ships a `gen_snapshot` that emits Mach-O.
//!
//! Measured end to end on Windows, producing a real `App.framework` payload:
//!
//! ```text
//! 1. AOT kernel
//!    dartaotruntime bin/cache/dart-sdk/bin/snapshots/gen_kernel_aot.dart.snapshot //!      --platform .../vm_platform_strong.dill --aot --output aot.dill main.dart
//!
//! 2. Dart -> iOS Mach-O   (note: the android-arm64 host snapshot; it is the
//!    ARM64 code generator, and --snapshot_kind selects the container format)
//!    bin/cache/artifacts/engine/android-arm64-release/windows-x64/gen_snapshot.exe //!      --snapshot_kind=app-aot-macho-dylib --macho=App.dylib aot.dill
//!
//!    -> Mach-O 64-bit arm64 dynamically linked shared library
//!       1.9MB, carrying kDartVmSnapshot / kDartIsolateSnapshot
//! ```
//!
//! Two things still needed for a runnable Flutter app, neither of them blocked:
//! `Flutter.xcframework` (precompiled by Google, a plain zip on their CDN keyed
//! by engine hash — Flutter simply never downloads iOS artefacts on Windows),
//! and assembling `Runner.app` around both, which `ios_package` already does.
//!
//! Device detection is solved separately and officially: `flutter custom-devices`
//! registers the phone with our own go-ios ping/install/launch/forward hooks, so
//! `flutter devices` lists it on Windows. Its `platform` field has no iOS value,
//! so `flutter run` still builds the wrong format — the artefact swap belongs in
//! a `postBuild` hook.

use serde::Serialize;
use std::path::PathBuf;

/// One prerequisite of the host-native pipeline.
#[derive(Serialize, Clone, Debug)]
pub struct ToolStatus {
    pub name: String,
    pub found: bool,
    /// Resolved path, or empty when missing.
    pub path: String,
    /// What this piece does, and how to get it when absent.
    pub detail: String,
}

/// Everything needed to turn source into a signed `.ipa` on this host.
#[derive(Serialize, Clone, Debug)]
pub struct CrossCompileStatus {
    pub ready: bool,
    pub sdk_root: String,
    pub tools: Vec<ToolStatus>,
    /// Ordered, human-readable next steps — empty when `ready`.
    pub missing_steps: Vec<String>,
}

/// `SDKROOT`, else a sensible per-platform default location.
///
/// Checked rather than assumed: an unset or stale `SDKROOT` is the single most
/// common reason a cross-build dies with thousands of "header not found".
pub fn resolve_sdk_root() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SDKROOT") {
        let path = PathBuf::from(p);
        if path.join("usr").join("include").is_dir() {
            return Some(path);
        }
    }
    // Conventional drop-in spots, so a user can just unpack and go.
    let mut candidates: Vec<PathBuf> = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("sdk").join("iPhoneOS.sdk"));
            candidates.push(dir.join("resources").join("sdk").join("iPhoneOS.sdk"));
        }
    }
    candidates.push(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("sdk").join("iPhoneOS.sdk"));
    if let Ok(home) = std::env::var("USERPROFILE").or_else(|_| std::env::var("HOME")) {
        candidates.push(PathBuf::from(home).join("iPhoneOS.sdk"));
    }
    candidates.into_iter().find(|c| c.join("usr").join("include").is_dir())
}

/// Is the Rust `aarch64-apple-ios` target installed?
fn rust_ios_target_installed() -> bool {
    crate::process_ext::hidden_command("rustup")
        .args(["target", "list", "--installed"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).contains("aarch64-apple-ios"))
        .unwrap_or(false)
}

fn which(bin: &str) -> Option<PathBuf> {
    which::which(bin).ok()
}

/// Report every prerequisite and exactly what to do about the missing ones.
#[cfg_attr(feature = "tauri", tauri::command)]
pub fn ios_crosscompile_status() -> CrossCompileStatus {
    let sdk = resolve_sdk_root();
    let clang = which("clang");
    let lld = which("ld64.lld").or_else(|| which("lld"));
    // zsign signs with a real cert/profile; ldid fake-signs for jailbroken or
    // emulator targets. Either is enough to produce a loadable binary.
    let signer = super::iphone_device::find_bundled_tool(&["zsign", "ldid"])
        .or_else(|| which("zsign"))
        .or_else(|| which("ldid"));
    let rust_target = rust_ios_target_installed();

    let tools = vec![
        ToolStatus {
            name: "iPhoneOS.sdk".into(),
            found: sdk.is_some(),
            path: sdk.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            detail: "Headers and .tbd framework stubs. Apple's licence prevents us shipping it — \
                     unpack an Xcode .xip (7-Zip handles .xip on Windows) and point SDKROOT at \
                     Contents/Developer/Platforms/iPhoneOS.platform/Developer/SDKs/iPhoneOS.sdk."
                .into(),
        },
        ToolStatus {
            name: "clang".into(),
            found: clang.is_some(),
            path: clang.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            detail: "Compiles C/C++/ObjC to ARM64 Mach-O with -target aarch64-apple-ios. Any \
                     stock LLVM works; the Mach-O backend is not macOS-specific."
                .into(),
        },
        ToolStatus {
            name: "ld64.lld".into(),
            found: lld.is_some(),
            path: lld.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            detail: "LLVM's Mach-O linker, replacing Apple's ld64. Ships with LLVM.".into(),
        },
        ToolStatus {
            name: "rust aarch64-apple-ios".into(),
            found: rust_target,
            path: if rust_target { "installed".into() } else { String::new() },
            detail: "rustup target add aarch64-apple-ios".into(),
        },
        ToolStatus {
            name: "zsign / ldid".into(),
            found: signer.is_some(),
            path: signer.as_ref().map(|p| p.display().to_string()).unwrap_or_default(),
            detail: "Replaces Apple's codesign. zsign signs with a .p12 + .mobileprovision; \
                     ldid fake-signs with entitlements for jailbroken/emulator targets."
                .into(),
        },
    ];

    let missing_steps: Vec<String> = tools
        .iter()
        .filter(|t| !t.found)
        .map(|t| format!("{}: {}", t.name, t.detail))
        .collect();

    CrossCompileStatus {
        ready: missing_steps.is_empty(),
        sdk_root: sdk.map(|p| p.display().to_string()).unwrap_or_default(),
        tools,
        missing_steps,
    }
}

/// Where a user-imported SDK is stored so it resolves like a bundled one.
///
/// Kept beside the executable under `resources/sdk`, which is exactly where the
/// installer would place a shipped SDK — so an imported SDK and a bundled SDK
/// are indistinguishable to `resolve_sdk_root`, and no `SDKROOT` env var or
/// shell restart is needed.
fn managed_sdk_dir() -> Option<PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let dir = exe.parent()?;
    Some(dir.join("resources").join("sdk"))
}

/// Copy a user-supplied `iPhoneOS.sdk` into the IDE so it is always available.
///
/// Apple's licence restricts redistributing the SDK, so the IDE ships without
/// one and imports the copy the user already has rights to. After this, the SDK
/// resolves automatically — the cross-compile pipeline stops depending on an
/// environment variable the user has to remember to set.
///
/// Rejects anything without `usr/include`, so a half-extracted `.xip` or a
/// symlink-flattened copy is caught here rather than surfacing later as a wall
/// of missing headers.
#[cfg_attr(feature = "tauri", tauri::command)]
pub fn ios_import_sdk(source: String) -> Result<String, String> {
    let src = PathBuf::from(source.trim());
    if !src.join("usr").join("include").is_dir() {
        return Err(format!(
            "{} does not look like an iPhoneOS SDK — no usr/include inside. Point at the              iPhoneOS.sdk directory itself (…/Developer/SDKs/iPhoneOS.sdk).",
            src.display()
        ));
    }
    // A framework stub proves the .tbd files survived the transfer; a copy that
    // flattened symlinks keeps the folders but loses these, and then nothing links.
    let stub = src
        .join("System").join("Library").join("Frameworks")
        .join("Foundation.framework").join("Foundation.tbd");
    if !stub.is_file() {
        return Err(
            "The SDK is missing Foundation.tbd — the link stubs did not survive the copy.              Re-transfer it as a tar archive (tar -czf on macOS, extract with WSL) so the              symlinks are preserved."
                .into(),
        );
    }

    let dest_dir = managed_sdk_dir().ok_or("cannot resolve the install directory")?;
    let dest = dest_dir.join("iPhoneOS.sdk");
    if dest.exists() {
        std::fs::remove_dir_all(&dest).map_err(|e| format!("clear {}: {e}", dest.display()))?;
    }
    std::fs::create_dir_all(&dest_dir).map_err(|e| format!("create {}: {e}", dest_dir.display()))?;
    copy_tree(&src, &dest).map_err(|e| format!("copy SDK: {e}"))?;
    Ok(dest.display().to_string())
}

/// Recursive copy. `std::fs` has no directory copy, and the SDK is ~9k mostly
/// tiny text files, so a plain walk is fine.
fn copy_tree(from: &std::path::Path, to: &std::path::Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dst = to.join(entry.file_name());
        if ty.is_dir() {
            copy_tree(&entry.path(), &dst)?;
        } else {
            // Symlinks are followed deliberately: Windows needs elevation or
            // Developer Mode to create them, and a resolved copy links fine.
            std::fs::copy(entry.path(), &dst)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A directory only counts as an SDK if it actually holds headers — an
    /// empty or half-extracted folder must not pass, or the build fails much
    /// later with a wall of "header not found".
    #[test]
    fn sdk_root_requires_usr_include() {
        let tmp = std::env::temp_dir().join("vscr-sdk-probe");
        let _ = std::fs::remove_dir_all(&tmp);
        std::fs::create_dir_all(&tmp).unwrap();
        std::env::set_var("SDKROOT", &tmp);
        assert!(
            resolve_sdk_root().map(|p| p != tmp).unwrap_or(true),
            "an empty dir must not be accepted as an SDK"
        );
        std::fs::create_dir_all(tmp.join("usr").join("include")).unwrap();
        assert_eq!(resolve_sdk_root().as_deref(), Some(tmp.as_path()));
        std::env::remove_var("SDKROOT");
        let _ = std::fs::remove_dir_all(&tmp);
    }

    /// The report must name every prerequisite even when none are present, so
    /// the panel can show the full checklist rather than an empty list.
    #[test]
    fn status_lists_all_prerequisites() {
        let s = ios_crosscompile_status();
        let names: Vec<&str> = s.tools.iter().map(|t| t.name.as_str()).collect();
        for expected in ["iPhoneOS.sdk", "clang", "ld64.lld", "rust aarch64-apple-ios", "zsign / ldid"] {
            assert!(names.contains(&expected), "missing {expected} from the checklist");
        }
        assert_eq!(s.ready, s.missing_steps.is_empty());
    }
}
