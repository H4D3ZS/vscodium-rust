// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    // Pure Tauri + Rust startup; no external Zed/GPUI runtime.
    vscode_rust_app_lib::run();
}
