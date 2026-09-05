use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn target_dir_from_out() -> Option<PathBuf> {
    let out_dir = std::env::var("OUT_DIR").ok()?;
    Path::new(&out_dir)
        .parent()
        .and_then(|p| p.parent())
        .and_then(|p| p.parent())
        .map(|p| p.to_path_buf())
}

fn build_sim_host(target: &Path) {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let native = manifest.join("../tools/ios-simulator/native");
    let swift_src = native.join("sim_host.swift");
    let bridge_src = native.join("ns_window_bridge.m");
    let dylib = target.join("libsim_host.dylib");
    let bridge_o = target.join("ns_window_bridge.o");

    if !swift_src.exists() {
        eprintln!("cargo:warning=sim_host.swift missing — iOS native panel disabled");
        return;
    }

    let needs_build = |out: &Path| -> bool {
        if !out.exists() {
            return true;
        }
        let out_m = fs::metadata(out).and_then(|m| m.modified()).ok();
        let sw_m = fs::metadata(&swift_src).and_then(|m| m.modified()).ok();
        let br_m = fs::metadata(&bridge_src).and_then(|m| m.modified()).ok();
        match (out_m, sw_m, br_m) {
            (Some(o), Some(s), Some(b)) => o < s || o < b,
            _ => true,
        }
    };

    if needs_build(&dylib) {
        let _ = fs::create_dir_all(target);
        let clang = Command::new("clang")
            .args([
                "-fobjc-arc",
                "-c",
                bridge_src.to_str().unwrap_or_default(),
                "-o",
                bridge_o.to_str().unwrap_or_default(),
            ])
            .output();
        match clang {
            Ok(o) if o.status.success() => {}
            Ok(o) => {
                eprintln!("cargo:warning=ns_window_bridge.m failed:");
                eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                return;
            }
            Err(e) => {
                eprintln!("cargo:warning=clang not available: {e}");
                return;
            }
        }
        let dev_dir = Command::new("xcode-select")
            .arg("-p")
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
            .unwrap_or_else(|| "/Applications/Xcode.app/Contents/Developer".to_string());
        let priv_fw = format!("{dev_dir}/Platforms/iPhoneSimulator.platform/Developer/Library/PrivateFrameworks");
        let swift = Command::new("swiftc")
            .args([
                "-O",
                "-emit-library",
                "-module-name",
                "SimHost",
                "-F",
                "/Library/Developer/PrivateFrameworks",
                "-F",
                &priv_fw,
                "-framework",
                "AppKit",
                "-framework",
                "QuartzCore",
                "-framework",
                "IOSurface",
                "-framework",
                "Foundation",
                swift_src.to_str().unwrap_or_default(),
                bridge_o.to_str().unwrap_or_default(),
                "-o",
                dylib.to_str().unwrap_or_default(),
            ])
            .output();
        match swift {
            Ok(o) if o.status.success() && dylib.exists() => {}
            Ok(o) => {
                eprintln!("cargo:warning=libsim_host.dylib build failed:");
                eprintln!("{}", String::from_utf8_lossy(&o.stderr));
                return;
            }
            Err(e) => {
                eprintln!("cargo:warning=swiftc not available: {e}");
                return;
            }
        }
    }

    println!("cargo:rustc-link-search=native={}", target.display());
    println!("cargo:rustc-link-lib=dylib=sim_host");
}

fn main() {
    // Use CARGO_CFG_* (the *target* triple) instead of cfg!() (the *host*) so
    // cross-builds — e.g. building x86_64-apple-darwin from an M-series Mac —
    // resolve link flags correctly.
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    if target_os == "macos" {
        // Scope dylib link flags to the IDE binary only — the lib crate also
        // builds a cdylib; global link-arg would duplicate LC_RPATH on the exe.
        println!("cargo:rustc-link-arg-bin=vscode-rust-app=-Wl,-ld_classic");
        println!("cargo:rustc-link-arg-bin=vscode-rust-app=-Wl,-rpath,@executable_path/");
        println!(
            "cargo:rustc-link-arg-bin=vscode-rust-app=-Wl,-rpath,@executable_path/../Resources"
        );

        // libane_bridge.dylib is a prebuilt arm64-only binary. Linking it on
        // x86_64 (Intel Mac / Hackintosh) breaks the build; ANE hardware does
        // not exist there anyway, so ane.rs compiles its stub and
        // ane_inference falls back to Ollama at runtime.
        if target_arch == "aarch64" && Path::new("libane_bridge.dylib").exists() {
            println!("cargo:rustc-link-search=native=.");
            println!("cargo:rustc-link-lib=dylib=ane_bridge");
        } else {
            println!(
                "cargo:warning=ane_bridge link skipped (target_arch={target_arch}) — ANE is Apple Silicon-only; Ollama fallback active"
            );
        }

        if let Some(target) = target_dir_from_out() {
            build_sim_host(&target);
            if target_arch == "aarch64" {
                let src_lib = Path::new("libane_bridge.dylib");
                let dest_lib = target.join("libane_bridge.dylib");
                if src_lib.exists() {
                    let _ = fs::copy(src_lib, &dest_lib);
                }
            }
        }
    }
    if std::env::var("CARGO_FEATURE_TAURI").is_ok() {
        tauri_build::build();
    }

    if target_os == "windows" {
        if let Some(target) = target_dir_from_out() {
            let dest_dir = target.join("binaries");
            let _ = fs::create_dir_all(&dest_dir);
            let sidecar = Path::new(env!("CARGO_MANIFEST_DIR")).join("binaries").join("browser-agent.exe");
            if sidecar.exists() {
                let _ = fs::copy(&sidecar, dest_dir.join("browser-agent.exe"));
            }
        }
    }
}
