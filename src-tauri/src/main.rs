#[cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
fn main() {
    // Ensure loopback connections are never sent through proxy settings.
    // Some VPNs/proxies set HTTP_PROXY/HTTPS_PROXY/ALL_PROXY without excluding localhost.
    const LOOPBACK: [&str; 3] = ["127.0.0.1", "localhost", "::1"];

    let upsert = |key: &str| {
        let mut items = std::env::var(key)
            .unwrap_or_default()
            .split(',')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string())
            .collect::<Vec<_>>();

        for host in LOOPBACK {
            if items.iter().any(|v| v.eq_ignore_ascii_case(host)) {
                continue;
            }
            items.push(host.to_string());
        }

        // Safety: called during startup before any threads are spawned.
        unsafe { std::env::set_var(key, items.join(",")) };
    };

    upsert("NO_PROXY");
    upsert("no_proxy");

    #[cfg(target_os = "linux")]
    {
        if let Some(backend_note) = configure_display_backend() {
            eprintln!("{backend_note}");
        }
    }

    // Call the lib runner which initializes everything properly
    #[cfg(feature = "tauri")]
    vscode_rust_app_lib::run();
    #[cfg(not(feature = "tauri"))]
    eprintln!("This binary was built without the `tauri` feature. Use the gpui-ide native shell for the headless build.");
}

/// Linux/WebKitGTK display hardening. Newer WebKitGTK (2.42+) with several GPU
/// drivers (Nvidia proprietary, some Intel/Mesa combos) renders a blank/black
/// window unless DMA-BUF and accelerated compositing are disabled. We set these
/// only when the user hasn't already chosen, so power users keep control.
/// Returns a human-readable note describing what was applied (or `None`).
#[cfg(target_os = "linux")]
fn configure_display_backend() -> Option<String> {
    fn set_if_unset(key: &str, value: &str) -> bool {
        if std::env::var_os(key).is_some() {
            return false;
        }
        // Safety: called during startup before any threads are spawned.
        unsafe { std::env::set_var(key, value) };
        true
    }

    let mut applied: Vec<&str> = Vec::new();
    if set_if_unset("WEBKIT_DISABLE_DMABUF_RENDERER", "1") {
        applied.push("WEBKIT_DISABLE_DMABUF_RENDERER=1");
    }
    if set_if_unset("WEBKIT_DISABLE_COMPOSITING_MODE", "1") {
        applied.push("WEBKIT_DISABLE_COMPOSITING_MODE=1");
    }

    let session = std::env::var("XDG_SESSION_TYPE").unwrap_or_default();
    if applied.is_empty() {
        None
    } else {
        Some(format!(
            "[display] {} session — applied {} (override by exporting these yourself)",
            if session.is_empty() { "unknown" } else { &session },
            applied.join(", ")
        ))
    }
}
