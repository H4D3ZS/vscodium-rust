//! Build script for hades-bridge
//! 
//! Compiles the C++ integration layer if building with C++ support

fn main() {
    // Only build C++ layer if explicitly enabled
    if std::env::var("HADES_BUILD_CPP").unwrap_or_default() == "1" {
        println!("cargo:rerun-if-changed=cpp/ggml-hades.cpp");
        println!("cargo:rerun-if-changed=include/hades-bridge.h");
        
        // Note: C++ compilation requires ggml headers
        // This is typically done in the llama.cpp build system
        println!("cargo:warning=C++ integration requires llama.cpp build system");
        println!("cargo:warning=Set up integration in llama.cpp CMakeLists.txt instead");
    }
    
    // Export include directory for downstream crates
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:include={}/include", manifest_dir);
}
