fn main() {
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-arg=-Wl,-ld_classic");
        println!("cargo:rustc-link-search=native=.");
        println!("cargo:rustc-link-lib=dylib=ane_bridge");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/");
        println!("cargo:rustc-link-arg=-Wl,-rpath,@executable_path/../Resources");
    }
    tauri_build::build()
}
