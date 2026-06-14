fn main() {
    // Only use custom linker script for bare-metal target
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.contains("none") {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        println!("cargo:rustc-link-arg=-T{}/linker.ld", manifest_dir);
        println!("cargo:rustc-link-arg=--no-pie");
        println!("cargo:rerun-if-changed=linker.ld");
    }
}
