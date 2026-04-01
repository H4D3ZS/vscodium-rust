fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-ld_classic");
        println!("cargo:rustc-link-search=native=/Users/hades/Desktop/vscodium-rust/ANE/bridge");
        println!("cargo:rustc-link-lib=dylib=ane_bridge");
    }
    tauri_build::build()
}
