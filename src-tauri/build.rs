use std::fs;
use std::path::Path;

fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-ld_classic");
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=dylib=ane_bridge");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources");

        // Dynamically copy libane_bridge.dylib into the target executable directory (e.g. target/debug)
        if let Ok(out_dir) = std::env::var("OUT_DIR") {
            let target_dir = Path::new(&out_dir)
                .parent()
                .and_then(|p| p.parent())
                .and_then(|p| p.parent());
            if let Some(target) = target_dir {
                let src_lib = Path::new("libane_bridge.dylib");
                let dest_lib = target.join("libane_bridge.dylib");
                if src_lib.exists() {
                    let _ = fs::copy(&src_lib, &dest_lib);
                }
            }
        }
    }
    tauri_build::build()
}
