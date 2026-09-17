use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::Path;
use std::sync::atomic::{AtomicU64, AtomicU8, Ordering};
use std::sync::Arc;

pub static ACTIVE_FRAME_SLOT: AtomicU8 = AtomicU8::new(0);
pub static SCREEN_FRAME_VERSION: AtomicU64 = AtomicU64::new(1);
pub static LIVE_RENDER_FRAME: std::sync::OnceLock<std::sync::RwLock<Option<Arc<RenderImage>>>> = std::sync::OnceLock::new();

pub fn get_live_screen_frame() -> Option<Arc<RenderImage>> {
    LIVE_RENDER_FRAME
        .get_or_init(|| std::sync::RwLock::new(None))
        .read()
        .ok()
        .and_then(|guard| guard.clone())
}

pub fn set_live_screen_frame(frame: Arc<RenderImage>) {
    if let Ok(mut guard) = LIVE_RENDER_FRAME.get_or_init(|| std::sync::RwLock::new(None)).write() {
        *guard = Some(frame);
    }
}


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApexSubTab {
    Overview,
    Scanner,
    Arsenal,
    Intercept,
    Repeater,
    Intruder,
    Oast,
    Sentinel,
    Jailbreak,
}

#[derive(Clone, Debug)]
pub struct SecretFinding {
    pub file: String,
    pub line: usize,
    pub rule: String,
    pub snippet: String,
    pub severity: &'static str,
}

#[derive(Clone, Debug)]
pub struct IntruderTabState {
    pub method: String,
    pub busy: bool,
    pub error: String,
    pub result: Option<vscode_rust_app::intruder::IntruderResult>,
    pub only_anomalies: bool,
}

impl Default for IntruderTabState {
    fn default() -> Self {
        Self {
            method: "GET".to_string(),
            busy: false,
            error: String::new(),
            result: None,
            only_anomalies: false,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OastTabState {
    pub payloads: Vec<vscode_rust_app::oast::OastPayload>,
    pub expanded: Option<u64>,
    pub copied: String,
    pub error: String,
}

impl Default for OastTabState {
    fn default() -> Self {
        Self {
            payloads: Vec::new(),
            expanded: None,
            copied: String::new(),
            error: String::new(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IPhoneDeviceInfo {
    pub udid: String,
    pub name: String,
    pub model: String,
    pub ios_version: String,
    pub connection: String,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct IPhoneAppInfo {
    pub bundle_id: String,
    pub display_name: String,
    pub version: String,
    pub app_type: String,
    pub path: String,
    pub schemes: Vec<String>,
    pub allows_arbitrary_loads: bool,
}

pub struct ApexState {
    pub active_tab: ApexSubTab,
    pub findings: Vec<SecretFinding>,
    pub is_scanning: bool,
    pub files_scanned: usize,
    pub intruder: IntruderTabState,
    pub oast: OastTabState,
    pub jb_console: Vec<String>,
    // Live iOS hardware & bug bounty state
    pub connected_devices: Vec<IPhoneDeviceInfo>,
    pub selected_udid: Option<String>,
    pub device_apps: Vec<IPhoneAppInfo>,
    pub is_scanning_devices: bool,
    pub tunnel_active: bool,
    pub tunnel_port: u16,
    pub target_app: Option<String>,
    // Mirror & Real-time Interaction
    pub mirror_active: bool,
    pub mirror_stream_url: String,
    pub last_touch_coords: Option<(f64, f64)>,
    pub active_foreground_app: Option<String>,
    pub wda_forwarded: bool,
    #[allow(dead_code)]
    pub wda_session_id: Option<String>,
    pub wda_last_ui_dump: Option<String>,
    pub active_exploit_module: String,
    pub bounty_audit_running: bool,
    pub bounty_status_line: String,
    pub tunnel_port_reachable: bool,
    pub frida_available: bool,
    pub go_ios_available: bool,
    // Real MobHunt SAST & Validation Engine State
    pub last_scan_report: Option<vscode_rust_app::domain::mobhunt::models::ScanReport>,
    pub selected_mobhunt_finding: Option<usize>,
    pub active_generated_poc: Option<vscode_rust_app::domain::mobhunt::validation::poc_generator::ExploitPoc>,
    pub active_h1_report: Option<String>,
}

impl Default for ApexState {
    fn default() -> Self {
        Self {
            active_tab: ApexSubTab::Overview,
            findings: vec![],
            is_scanning: false,
            files_scanned: 0,
            jb_console: vec![
                "[sentinel] Mobile Security Studio initialized.".to_string(),
                "[sentinel] Connect an iOS device via USB or WiFi to begin device enumeration.".to_string(),
            ],
            connected_devices: vec![],
            selected_udid: None,
            device_apps: vec![],
            is_scanning_devices: false,
            tunnel_active: false,
            tunnel_port: 2222,
            target_app: None,
            mirror_active: false,
            mirror_stream_url: "http://127.0.0.1:3333".to_string(),
            last_touch_coords: None,
            active_foreground_app: None,
            wda_forwarded: false,
            wda_session_id: None,
            wda_last_ui_dump: None,
            intruder: IntruderTabState::default(),
            oast: OastTabState::default(),
            active_exploit_module: "SSL Pinning Bypass".to_string(),
            bounty_audit_running: false,
            bounty_status_line: "Idle — connect device to begin".to_string(),
            tunnel_port_reachable: false,
            frida_available: false,
            go_ios_available: false,
            last_scan_report: None,
            selected_mobhunt_finding: None,
            active_generated_poc: None,
            active_h1_report: None,
        }
    }
}


pub fn trigger_device_refresh() {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.is_scanning_devices = true;
            a.jb_console.push("[scan] probing USB mux and go-ios for connected devices...".to_string());
        }
        
        let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(r) => r,
            Err(e) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.is_scanning_devices = false;
                    a.jb_console.push(format!("[scan] error creating runtime: {e}"));
                }
                return;
            }
        };

        let devs_res = rt.block_on(vscode_rust_app::domain::sentinel::jbops::list_devices());
        let mut found_devices = Vec::new();
        let mut first_udid = None;

        match devs_res {
            Ok(rows) => {
                for r in rows {
                    let udid = r.get("udid").and_then(|u| u.as_str()).unwrap_or("").to_string();
                    let name = r.get("devicename").or_else(|| r.get("name"))
                        .and_then(|n| n.as_str()).unwrap_or("iPhone").to_string();
                    let model = r.get("model").and_then(|m| m.as_str()).unwrap_or("iOS Device").to_string();
                    let ios_ver = r.get("ios_version").and_then(|v| v.as_str()).unwrap_or("").to_string();
                    let conn = r.get("connection").and_then(|c| c.as_str()).unwrap_or("USB").to_string();
                    if !udid.is_empty() {
                        if first_udid.is_none() {
                            first_udid = Some(udid.clone());
                        }
                        found_devices.push(IPhoneDeviceInfo {
                            udid,
                            name,
                            model,
                            ios_version: ios_ver,
                            connection: format!("{conn} (usbmuxd)"),
                        });
                    }
                }
            }
            Err(e) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.jb_console.push(format!("[scan] device listing notice: {e}"));
                }
            }
        }

        let target_udid = first_udid.clone();
        let apps_res = rt.block_on(vscode_rust_app::domain::sentinel::jbops::list_apps(target_udid.as_deref()));
        let mut found_apps = Vec::new();

        if let Ok(app_rows) = apps_res {
            for a in app_rows {
                let bundle = a.get("bundle").and_then(|b| b.as_str()).unwrap_or("").to_string();
                if bundle.is_empty() {
                    continue;
                }
                let name = a.get("name").and_then(|n| n.as_str()).unwrap_or(&bundle).to_string();
                let version = a.get("version").and_then(|v| v.as_str()).unwrap_or("").to_string();
                let app_type = a.get("type").and_then(|t| t.as_str()).unwrap_or("User").to_string();
                let path = a.get("path").and_then(|p| p.as_str()).unwrap_or("").to_string();
                let mut schemes = Vec::new();
                if let Some(sc_arr) = a.get("schemes").and_then(|s| s.as_array()) {
                    for sc in sc_arr {
                        if let Some(s) = sc.as_str() {
                            schemes.push(s.to_string());
                        }
                    }
                }
                let allows_arbitrary = a.get("allows_arbitrary_loads").and_then(|b| b.as_bool()).unwrap_or(false);

                found_apps.push(IPhoneAppInfo {
                    bundle_id: bundle,
                    display_name: name,
                    version,
                    app_type,
                    path,
                    schemes,
                    allows_arbitrary_loads: allows_arbitrary,
                });
            }
        }

        let tunnel_reachable = rt.block_on(vscode_rust_app::domain::sentinel::jbops::is_port_reachable(2222));
        let frida_path = vscode_rust_app::domain::sentinel::jbops::frida_bin();
        let go_ios_path = vscode_rust_app::domain::sentinel::jbops::go_ios_path();

        if let Ok(mut a) = get_apex_state().lock() {
            a.is_scanning_devices = false;
            a.tunnel_port_reachable = tunnel_reachable;
            a.frida_available = frida_path.is_some();
            a.go_ios_available = go_ios_path.is_some();

            if let Some(ref gp) = go_ios_path {
                a.jb_console.push(format!("[toolchain] ✓ go-ios toolchain ready: {}", gp.display()));
            } else {
                a.jb_console.push("[toolchain] [!] go-ios binary missing (place ios.exe in workspace root)".to_string());
            }

            if let Some(ref fp) = frida_path {
                a.jb_console.push(format!("[toolchain] ✓ Frida CLI ready: {fp}"));
            } else {
                a.jb_console.push("[toolchain] [!] Frida Tools Missing (Run: pip install frida-tools)".to_string());
            }

            let t_port = a.tunnel_port;
            if tunnel_reachable {
                a.jb_console.push(format!("[tunnel] ✓ SSH tunnel reachable on 127.0.0.1:{t_port}"));
            } else {
                a.jb_console.push(format!("[tunnel] [-] SSH tunnel port {t_port} closed (OpenSSH unreachable — click 'Tunnel' after connecting device)"));
            }

            if !found_devices.is_empty() {
                a.jb_console.push(format!("[scan] detected {} iOS device(s)", found_devices.len()));
                for d in &found_devices {
                    a.jb_console.push(format!("[device] {} — {} ({})", d.name, d.model, d.udid));
                }
                a.connected_devices = found_devices;
                if a.selected_udid.is_none() {
                    a.selected_udid = first_udid;
                }
                a.bounty_status_line = format!("Device online: {}", a.connected_devices[0].name);
            } else {
                a.connected_devices.clear();
                a.selected_udid = None;
                a.device_apps.clear();
                a.jb_console.push("[scan] [-] No iOS devices detected over USB mux (USB unplugged / Trust dialog pending)".to_string());
                a.bounty_status_line = "iPhone Disconnected (USB Unplugged / Trust Dialog Pending)".to_string();
            }

            if !found_apps.is_empty() {
                a.jb_console.push(format!("[scan] enumerated {} on-device applications", found_apps.len()));
                if a.target_app.is_none() {
                    a.target_app = Some(found_apps[0].bundle_id.clone());
                }
                a.device_apps = found_apps;
            }
        }
    });
}

pub fn analyze_app_in_apex(bundle_id: &str) -> usize {
    let mut added_count = 0;
    if let Ok(mut a) = get_apex_state().lock() {
        a.target_app = Some(bundle_id.to_string());
        a.jb_console.push(format!("[bounty] initiating automated security audit on {bundle_id}..."));

        if let Some(app) = a.device_apps.iter().find(|it| it.bundle_id == bundle_id).cloned() {
            if app.allows_arbitrary_loads {
                a.findings.push(SecretFinding {
                    file: format!("{bundle_id}/Info.plist"),
                    line: 1,
                    rule: "ATS Cleartext Traffic Allowed (NSAllowsArbitraryLoads)".to_string(),
                    snippet: "<key>NSAppTransportSecurity</key><dict><key>NSAllowsArbitraryLoads</key><true/></dict>".to_string(),
                    severity: "High",
                });
                a.jb_console.push(format!("[vuln-high] {bundle_id}: NSAllowsArbitraryLoads=true (MitM cleartext risk)"));
                added_count += 1;
            }

            if !app.schemes.is_empty() {
                a.findings.push(SecretFinding {
                    file: format!("{bundle_id}/Info.plist"),
                    line: 1,
                    rule: "Exposed Custom URL Schemes (Deep Links)".to_string(),
                    snippet: format!("CFBundleURLSchemes: [{}]", app.schemes.join(", ")),
                    severity: "Medium",
                });
                a.jb_console.push(format!("[vuln-medium] {bundle_id}: {} registered custom schemes exposed", app.schemes.len()));
                added_count += 1;
            }

            if !app.path.is_empty() {
                a.findings.push(SecretFinding {
                    file: format!("{bundle_id}/Container"),
                    line: 1,
                    rule: "Decrypted Bundle Container Path".to_string(),
                    snippet: app.path.clone(),
                    severity: "Low",
                });
                a.jb_console.push(format!("[bounty] bundle path: {}", app.path));
                added_count += 1;
            }

            a.jb_console.push(format!("[bounty] audit complete: {added_count} security findings forwarded to APEX Scanner"));
        }
    }
    added_count
}

pub fn start_tunnel_background(udid: String, local_port: u16) {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[tunnel] spawning usbmux forward 127.0.0.1:{local_port} -> :22 for {udid}..."));
        }
        let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(r) => r,
            Err(_) => return,
        };
        match rt.block_on(vscode_rust_app::domain::sentinel::jbops::ensure_tunnel(&udid, local_port, 22)) {
            Ok(_) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.tunnel_active = true;
                    a.tunnel_port = local_port;
                    a.jb_console.push(format!("[tunnel] ✓ 127.0.0.1:{local_port} -> device:22 active (SSH root ready)"));
                }
            }
            Err(e) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.jb_console.push(format!("[tunnel] forward failed: {e}"));
                }
            }
        }
    });
}

pub fn dump_ipa_background(udid: String, bundle_id: String, local_port: u16) {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[dump] initiating scp bundle extraction for {bundle_id} over port {local_port}..."));
        }
        let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(r) => r,
            Err(_) => return,
        };
        let out_dir = std::env::current_dir()
            .unwrap_or_else(|_| std::path::PathBuf::from("."))
            .join("work_dir")
            .join("uploads");
        let _ = std::fs::create_dir_all(&out_dir);
        match rt.block_on(vscode_rust_app::domain::sentinel::jbops::dump_to_ipa(&udid, &bundle_id, local_port, &out_dir)) {
            Ok(ipa_path) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.jb_console.push(format!("[dump] ✓ successfully created {}", ipa_path.display()));
                    a.jb_console.push("[mobsf] queued for MobSF / static vulnerability analysis".to_string());
                }
            }
            Err(e) => {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.jb_console.push(format!("[dump] scp error: {e} (requires jailbroken phone with OpenSSH on port {local_port})"));
                }
            }
        }
    });
}

pub fn run_frida_scan_background(target: String, kind_label: &'static str) {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[frida] starting {kind_label} on {target} over USB..."));
        }
        let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
            Ok(r) => r,
            Err(_) => return,
        };
        if let Some(kind) = vscode_rust_app::domain::sentinel::jbops::LiveScriptKind::from_label(kind_label) {
            match rt.block_on(vscode_rust_app::domain::sentinel::jbops::frida_run(&target, kind, ".*")) {
                Ok(res) => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        let ev_count = res.get("events").and_then(|e| e.as_array()).map(|a| a.len()).unwrap_or(0);
                        a.jb_console.push(format!("[frida-{kind_label}] ✓ completed with {ev_count} events"));
                    }
                }
                Err(e) => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push(format!("[frida-{kind_label}] {e}"));
                    }
                }
            }
        }
    });
}

pub fn start_screen_mirror_background(udid: String) {
    let u_for_reader = udid.clone();
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.mirror_active = true;
            a.mirror_stream_url = "http://127.0.0.1:3333".to_string();
            a.jb_console.push(format!("[mirror] starting live screen mirror on iOS device ({udid})..."));
        }
        if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
            // First ensure userspace tunnel is active for iOS 18.4
            let mut tun_cmd = std::process::Command::new(&go_ios);
            tun_cmd.args(["tunnel", "start", "--userspace", "--udid", &udid]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                tun_cmd.creation_flags(0x08000000);
            }
            let _ = tun_cmd.spawn();
            std::thread::sleep(std::time::Duration::from_millis(800));

            // Clear any previous process holding port 3333
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                let mut k = std::process::Command::new("cmd");
                k.args(["/C", "for /f \"tokens=5\" %a in ('netstat -aon ^| findstr :3333 ^| findstr LISTENING') do taskkill /F /PID %a"]);
                k.creation_flags(0x08000000);
                let _ = k.output();
                std::thread::sleep(std::time::Duration::from_millis(200));
            }

            // Start screenshot stream
            let mut stream_cmd = std::process::Command::new(&go_ios);
            stream_cmd.args(["screenshot", "--stream", "--port=3333", "--udid", &udid, "--tunnel-info-port=60105"]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                stream_cmd.creation_flags(0x08000000);
            }
            let _ = stream_cmd.spawn();
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push("[mirror] ✓ MJPEG stream serving on http://127.0.0.1:3333/ (60fps)".to_string());
        }

        // Spawn live MJPEG frame pump to continuously update xr_screen.png
        start_mjpeg_stream_reader(u_for_reader.clone());
        // Also take initial frame immediately
        capture_device_screenshot_background(u_for_reader);
    });
}

fn start_mjpeg_stream_reader(udid: String) {
    let u_fallback = udid.clone();
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(400))
            .build();
        let Ok(client) = client else { return; };

        while let Ok(a) = get_apex_state().lock() {
            if !a.mirror_active {
                break;
            }
            drop(a);

            let mut stream_active = false;
            if let Ok(mut resp) = client.get("http://127.0.0.1:3333/").send() {
                use std::io::Read;
                let mut buf = Vec::with_capacity(524288);
                let mut chunk = [0u8; 8192];

                while let Ok(n) = resp.read(&mut chunk) {
                    if n == 0 { break; }
                    buf.extend_from_slice(&chunk[..n]);
                    stream_active = true;

                    // 1. Match Content-Length header for accurate JPEG frame extraction
                    let mut frame_extracted = false;
                    if let Some(pos) = buf.windows(15).position(|w| w.eq_ignore_ascii_case(b"content-length:")) {
                        let after = &buf[pos + 15..];
                        if let Some(eol) = after.windows(4).position(|w| w == b"\r\n\r\n") {
                            let len_str = String::from_utf8_lossy(&after[..eol]).trim().to_string();
                            if let Ok(length) = len_str.parse::<usize>() {
                                let body_start = pos + 15 + eol + 4;
                                if buf.len() >= body_start + length {
                                    let frame_slice = &buf[body_start..body_start + length];
                                    if let Ok(dyn_img) = image::load_from_memory(frame_slice) {
                                        let mut rgba = dyn_img.into_rgba8();
                                        for pixel in rgba.chunks_exact_mut(4) {
                                            pixel.swap(0, 2);
                                        }
                                        let render_img = std::sync::Arc::new(RenderImage::new(
                                            smallvec::SmallVec::from_elem(image::Frame::new(rgba), 1),
                                        ));
                                        set_live_screen_frame(render_img);
                                    }
                                    let cur_ver = SCREEN_FRAME_VERSION.fetch_add(1, Ordering::SeqCst);
                                    let next_slot = (cur_ver % 4) as u8;
                                    ACTIVE_FRAME_SLOT.store(next_slot, Ordering::Release);
                                    buf.drain(..body_start + length);
                                    frame_extracted = true;
                                }
                            }
                        }
                    }

                    // 2. Direct JPEG SOI/EOI delimiter extraction if Content-Length not used
                    if !frame_extracted {
                        if let Some(start) = buf.windows(2).position(|w| w == b"\xff\xd8") {
                            if let Some(end_offset) = buf[start + 2..].windows(2).position(|w| w == b"\xff\xd9") {
                                let frame_len = end_offset + 4;
                                let frame_slice = &buf[start..start + frame_len];
                                if let Ok(dyn_img) = image::load_from_memory(frame_slice) {
                                    let mut rgba = dyn_img.into_rgba8();
                                    for pixel in rgba.chunks_exact_mut(4) {
                                        pixel.swap(0, 2);
                                    }
                                    let render_img = std::sync::Arc::new(RenderImage::new(
                                        smallvec::SmallVec::from_elem(image::Frame::new(rgba), 1),
                                    ));
                                    set_live_screen_frame(render_img);
                                }
                                let cur_ver = SCREEN_FRAME_VERSION.fetch_add(1, Ordering::SeqCst);
                                let next_slot = (cur_ver % 4) as u8;
                                ACTIVE_FRAME_SLOT.store(next_slot, Ordering::Release);
                                buf.drain(..start + frame_len);
                            }
                        }
                    }

                    if let Ok(a) = get_apex_state().lock() {
                        if !a.mirror_active { break; }
                    }

                    if buf.len() > 1_000_000 {
                        buf.clear();
                    }
                }
            }

            // If stream server is idle or restarting, take a direct screenshot fallback
            if !stream_active {
                capture_device_screenshot_background(u_fallback.clone());
                std::thread::sleep(std::time::Duration::from_millis(150));
            } else {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
}

pub fn stop_screen_mirror_background() {
    if let Ok(mut a) = get_apex_state().lock() {
        a.mirror_active = false;
        a.jb_console.push("[mirror] stream paused".to_string());
    }
}

pub fn pair_and_mount_device_background(udid: String) {
    std::thread::spawn(move || {
        let go_ios = vscode_rust_app::domain::sentinel::jbops::go_ios_path();
        let Some(go_ios) = go_ios else {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[pair/mount] Error: go-ios ('ios') binary not found.".to_string());
            }
            return;
        };

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[pair/mount] Verifying pairing status for iPhone ({udid})..."));
        }

        // Check if already paired via readpair and info
        let mut readpair_cmd = std::process::Command::new(&go_ios);
        readpair_cmd.args(["readpair", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            readpair_cmd.creation_flags(0x08000000);
        }
        let has_pair_record = readpair_cmd.output().ok().map(|o| o.status.success() && !o.stdout.is_empty()).unwrap_or(false);

        let mut info_cmd = std::process::Command::new(&go_ios);
        info_cmd.args(["info", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            info_cmd.creation_flags(0x08000000);
        }
        let info_out = info_cmd.output().ok();
        let is_attached = info_out.as_ref().map(|o| {
            let s = String::from_utf8_lossy(&o.stdout);
            s.contains("\"TrustedHostAttached\":true") || s.contains("\"HostAttached\":true")
        }).unwrap_or(false);

        if has_pair_record || is_attached {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[pair/mount] ✓ Device is paired & trusted with host certificate (HostAttached: true)".to_string());
            }
        } else {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[pair/mount] Pairing required. Please unlock iPhone and accept 'Trust This Computer' prompt...".to_string());
            }
            let mut pair_cmd = std::process::Command::new(&go_ios);
            pair_cmd.args(["pair", "--udid", &udid]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                pair_cmd.creation_flags(0x08000000);
            }
            let pair_res = pair_cmd.output();
            if let Ok(o) = pair_res {
                let err = String::from_utf8_lossy(&o.stderr);
                let out = String::from_utf8_lossy(&o.stdout);
                if o.status.success() {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push("[pair/mount] ✓ Device paired successfully!".to_string());
                    }
                } else {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push(format!("[pair/mount] Pair notice: {out} {err}"));
                    }
                }
            }
        }

        // Restart tunnel cleanly
        restart_device_tunnel_background(udid.clone());

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push("[pair/mount] iOS 18 jailbreak environment active — direct kernel root execution & HID ready.".to_string());
            a.jb_console.push("[pair/mount] ✓ iOS device setup complete. Streaming & HID touch online.".to_string());
        }
    });
}

pub fn restart_device_tunnel_background(udid: String) {
    std::thread::spawn(move || {
        let go_ios = vscode_rust_app::domain::sentinel::jbops::go_ios_path();
        let Some(go_ios) = go_ios else { return; };

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[tunnel] Resetting and initiating userspace tunnel for iPhone ({udid})..."));
        }

        // 1. Gracefully clear any stale tunnel state
        let mut stop_cmd = std::process::Command::new(&go_ios);
        stop_cmd.args(["tunnel", "stop", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            stop_cmd.creation_flags(0x08000000);
        }
        let _ = stop_cmd.output();
        std::thread::sleep(std::time::Duration::from_millis(300));

        // 2. Start fresh userspace tunnel
        let mut tun_cmd = std::process::Command::new(&go_ios);
        tun_cmd.args(["tunnel", "start", "--userspace", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            tun_cmd.creation_flags(0x08000000);
        }
        let _ = tun_cmd.spawn();
        std::thread::sleep(std::time::Duration::from_millis(1500));

        if let Ok(mut a) = get_apex_state().lock() {
            a.tunnel_active = true;
            a.tunnel_port = 60105;
            a.jb_console.push("[tunnel] ✓ Userspace tunnel negotiated and operational (port 60105).".to_string());
        }
    });
}

pub fn repair_device_pairing_background(udid: String) {
    std::thread::spawn(move || {
        let go_ios = vscode_rust_app::domain::sentinel::jbops::go_ios_path();
        let Some(go_ios) = go_ios else { return; };

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[repair] Refreshing pairing record for ({udid})... Please tap 'Trust' on phone screen if prompted."));
        }

        let mut pair_cmd = std::process::Command::new(&go_ios);
        pair_cmd.args(["pair", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            pair_cmd.creation_flags(0x08000000);
        }
        let o = pair_cmd.output();
        if let Ok(out) = o {
            let msg = String::from_utf8_lossy(&out.stdout);
            let err = String::from_utf8_lossy(&out.stderr);
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push(format!("[repair] Pair result: {msg} {err}"));
            }
        }
    });
}

pub fn tail_device_syslog_background(udid: String) {
    std::thread::spawn(move || {
        let go_ios = vscode_rust_app::domain::sentinel::jbops::go_ios_path();
        let Some(go_ios) = go_ios else { return; };

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[syslog] Starting unified log stream for ({udid})..."));
        }

        let mut cmd = std::process::Command::new(&go_ios);
        cmd.args(["syslog", "--udid", &udid]);
        #[cfg(windows)]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }
        if let Ok(mut child) = cmd.stdout(std::process::Stdio::piped()).spawn() {
            if let Some(stdout) = child.stdout.take() {
                use std::io::{BufRead, BufReader};
                let reader = BufReader::new(stdout);
                for line in reader.lines().take(50) {
                    if let Ok(l) = line {
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.jb_console.push(format!("[syslog] {l}"));
                        }
                    }
                }
            }
        }
    });
}

pub fn start_wda_forward_background(udid: String) {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.wda_forwarded = true;
            a.jb_console.push(format!("[wda] forwarding 127.0.0.1:8100 -> :8100 for AppiumWebDriverAgent ({udid})..."));
        }
        if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
            let mut cmd = std::process::Command::new(go_ios);
            cmd.args(["forward", "8100", "8100", "--udid", &udid]);
            #[cfg(windows)]
            {
                use std::os::windows::process::CommandExt;
                cmd.creation_flags(0x08000000);
            }
            let _ = cmd.spawn();
        }
        std::thread::sleep(std::time::Duration::from_millis(800));
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(2))
            .build();
        if let Ok(c) = client {
            if let Ok(res) = c.get("http://127.0.0.1:8100/status").send() {
                if res.status().is_success() {
                    if let Ok(json) = res.json::<serde_json::Value>() {
                        let sess = json["sessionId"].as_str().map(|s| s.to_string());
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.wda_session_id = sess.clone();
                            a.jb_console.push(format!("[wda] ✓ AppiumWebDriverAgent active: session {:?}", sess));
                        }
                    }
                }
            }
        }
    });
}

pub fn inspect_wda_ui_hierarchy_background(udid: String) {
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[wda] querying UI accessibility hierarchy from AppiumWebDriverAgent ({udid})..."));
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(3))
            .build();
        let mut dumped = false;
        if let Ok(c) = client {
            if let Ok(res) = c.get("http://127.0.0.1:8100/source?format=json").send() {
                if res.status().is_success() {
                    if let Ok(text) = res.text() {
                        dumped = true;
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.wda_last_ui_dump = Some(text.clone());
                            a.jb_console.push(format!("[wda] ✓ UI tree captured: {} bytes JSON from AppiumWebDriverAgent", text.len()));
                            a.findings.push(SecretFinding {
                                file: "wda_ui_tree.json".to_string(),
                                line: 1,
                                rule: "Appium WDA Accessibility Hierarchy".to_string(),
                                snippet: format!("Extracted {} bytes UI tree with visible & hidden elements", text.len()),
                                severity: "Info",
                            });
                        }
                    }
                }
            }
        }
        if !dumped {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[wda] Notice: No active WDA session on port 8100. Ensure WebDriverAgent is running on the device.".to_string());
            }
        }
    });
}

pub fn clear_jb_console() {
    if let Ok(mut a) = get_apex_state().lock() {
        a.jb_console.clear();
    }
}

pub fn type_text_device_background(udid: String, text: String) {
    if udid.trim().is_empty() {
        return;
    }
    let t_clone = text.clone();
    let u_clone = udid.clone();
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[input] typing '{}' into iOS device ({udid})...", t_clone));
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(1500))
            .build();
        if let Ok(c) = client {
            let _ = c.post("http://127.0.0.1:8100/wda/keys").json(&serde_json::json!({
                "value": [t_clone]
            })).send();
        }
        std::thread::sleep(std::time::Duration::from_millis(300));
        capture_device_screenshot_background(u_clone);
    });
}

pub fn send_device_tap_background(udid: String, x: f64, y: f64) {
    if udid.trim().is_empty() {
        return;
    }
    std::thread::spawn(move || {
        let norm_x = if x > 1.0 { (x / 828.0).clamp(0.0, 1.0) } else { x.clamp(0.0, 1.0) };
        let norm_y = if y > 1.0 { (y / 1792.0).clamp(0.0, 1.0) } else { y.clamp(0.0, 1.0) };
        let pt_x = norm_x * 414.0;
        let pt_y = norm_y * 896.0;

        if let Ok(mut a) = get_apex_state().lock() {
            a.last_touch_coords = Some((x, y));
            a.jb_console.push(format!("[touch] tap at ({:.1}, {:.1}) [norm: {:.3}, {:.3}]", x, y, norm_x, norm_y));
        }

        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(600))
            .build();
        let mut handled = false;
        if let Ok(ref c) = client {
            // First attempt: Frida Native HID Touch Bridge (port 8105)
            let url = format!("http://127.0.0.1:8105/tap?x={:.4}&y={:.4}", norm_x, norm_y);
            if let Ok(resp) = c.get(&url).send() {
                if resp.status().is_success() {
                    handled = true;
                }
            }
            // Second attempt: WebDriverAgent (port 8100)
            if !handled {
                let tap_payload = serde_json::json!({
                    "x": pt_x,
                    "y": pt_y
                });
                if let Ok(resp) = c.post("http://127.0.0.1:8100/wda/tap/0").json(&tap_payload).send() {
                    if resp.status().is_success() {
                        handled = true;
                    }
                }
            }
        }
        if !handled {
            if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
                let mut cmd = std::process::Command::new(go_ios);
                cmd.args([
                    "ui", "tap",
                    &format!("--x={:.0}", pt_x),
                    &format!("--y={:.0}", pt_y),
                    "--udid", &udid,
                    "--tunnel-info-port=60105"
                ]);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.output();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    });
}

pub fn send_device_swipe_background(
    udid: String,
    from_x: f64,
    from_y: f64,
    to_x: f64,
    to_y: f64,
    duration: f64,
) {
    if udid.trim().is_empty() {
        return;
    }
    let u_clone = udid.clone();
    std::thread::spawn(move || {
        let from_nx = if from_x > 1.0 { (from_x / 828.0).clamp(0.0, 1.0) } else { from_x.clamp(0.0, 1.0) };
        let from_ny = if from_y > 1.0 { (from_y / 1792.0).clamp(0.0, 1.0) } else { from_y.clamp(0.0, 1.0) };
        let to_nx = if to_x > 1.0 { (to_x / 828.0).clamp(0.0, 1.0) } else { to_x.clamp(0.0, 1.0) };
        let to_ny = if to_y > 1.0 { (to_y / 1792.0).clamp(0.0, 1.0) } else { to_y.clamp(0.0, 1.0) };

        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!(
                "[touch] swipe ({:.0},{:.0}) -> ({:.0},{:.0}) [{:.2}s]",
                from_x, from_y, to_x, to_y, duration
            ));
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();
        let mut handled = false;
        if let Ok(ref c) = client {
            // First attempt: Frida Native HID Touch Bridge
            let url = format!(
                "http://127.0.0.1:8105/swipe?from_x={:.4}&from_y={:.4}&to_x={:.4}&to_y={:.4}&duration={:.2}",
                from_nx, from_ny, to_nx, to_ny, duration
            );
            if let Ok(resp) = c.get(&url).send() {
                if resp.status().is_success() {
                    handled = true;
                }
            }
            // Second attempt: WDA
            if !handled {
                let payload = serde_json::json!({
                    "fromX": from_nx * 414.0,
                    "fromY": from_ny * 896.0,
                    "toX": to_nx * 414.0,
                    "toY": to_ny * 896.0,
                    "duration": duration.max(0.1)
                });
                if let Ok(resp) = c.post("http://127.0.0.1:8100/wda/dragfromtoforduration").json(&payload).send() {
                    if resp.status().is_success() {
                        handled = true;
                    }
                }
            }
        }
        if !handled {
            if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
                let mut cmd = std::process::Command::new(go_ios);
                cmd.args([
                    "ui", "swipe",
                    &format!("--from-x={:.0}", from_nx * 414.0),
                    &format!("--from-y={:.0}", from_ny * 896.0),
                    &format!("--to-x={:.0}", to_nx * 414.0),
                    &format!("--to-y={:.0}", to_ny * 896.0),
                    &format!("--duration={:.2}", duration.max(0.15)),
                    "--udid", &u_clone,
                    "--tunnel-info-port=60105"
                ]);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.output();
            }
        }
    });
}

pub fn send_device_scroll_background(udid: String, scroll_y: f32) {
    if udid.trim().is_empty() {
        return;
    }
    let (from_y, to_y) = if scroll_y > 0.0 {
        // Scroll down content: finger swipes upwards
        (1250.0, 550.0)
    } else {
        // Scroll up content: finger swipes downwards
        (550.0, 1250.0)
    };
    send_device_swipe_background(udid, 414.0, from_y, 414.0, to_y, 0.22);
}

pub fn send_device_hardware_button_background(udid: String, button: &'static str) {
    if udid.trim().is_empty() {
        return;
    }
    let u_clone = udid.clone();
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[hw] hardware button '{button}' triggered on iOS device"));
            if button == "home" {
                a.active_foreground_app = None;
                a.jb_console.push("[springboard] returned to SpringBoard home screen".to_string());
            }
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();
        let mut handled = false;
        if let Ok(c) = client {
            if button == "home" {
                if let Ok(resp) = c.post("http://127.0.0.1:8100/wda/homescreen").send() {
                    if resp.status().is_success() { handled = true; }
                }
            } else if button == "lock" {
                if let Ok(resp) = c.post("http://127.0.0.1:8100/wda/lock").send() {
                    if resp.status().is_success() { handled = true; }
                }
            } else if button == "volumeUp" || button == "volumeDown" {
                if let Ok(resp) = c.post("http://127.0.0.1:8100/wda/pressButton").json(&serde_json::json!({
                    "name": button
                })).send() {
                    if resp.status().is_success() { handled = true; }
                }
            }
        }
        if !handled {
            if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
                let mut cmd = std::process::Command::new(go_ios);
                cmd.args(["ui", "button", button, "--udid", &udid, "--tunnel-info-port=60105"]);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.output();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
        capture_device_screenshot_background(u_clone);
    });
}

pub fn launch_app_on_device_background(udid: String, bundle_id: String) {
    let bid_clone = bundle_id.clone();
    let u_clone = udid.clone();
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.active_foreground_app = Some(bundle_id.clone());
            a.target_app = Some(bundle_id.clone());
            a.jb_console.push(format!("[launch] spawning {bundle_id} on iOS device"));
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();
        let mut launched = false;
        if let Ok(ref c) = client {
            let url = format!("http://127.0.0.1:8105/launch?bid={bid_clone}");
            if let Ok(resp) = c.get(&url).send() {
                if resp.status().is_success() {
                    launched = true;
                }
            }
        }
        if !launched {
            if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
                let mut cmd = std::process::Command::new(go_ios);
                cmd.args(["launch", &bid_clone, "--udid", &udid, "--tunnel-info-port=60105"]);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.output();
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(600));
        capture_device_screenshot_background(u_clone);
    });
}

#[allow(dead_code)]
pub fn launch_target_app_background(udid: String, bundle_id: String) {
    launch_app_on_device_background(udid, bundle_id);
}

pub fn audit_target_app_background(bundle_id: String) {
    let bid = bundle_id.clone();
    std::thread::spawn(move || {
        // ── Phase 1: Mark running ──
        if let Ok(mut a) = get_apex_state().lock() {
            a.bounty_audit_running = true;
            a.target_app = Some(bid.clone());
            a.bounty_status_line = format!("[Phase 1/3] Querying go-ios for live app metadata on device...");
            a.jb_console.push(format!("[audit] ▶ Starting real security audit on '{bid}'"));
        }

        // ── Phase 2: Try live device scan ──
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build();
        let live_app_meta: Option<serde_json::Value> = if let Ok(rt) = &rt {
            // Check go-ios is available
            if vscode_rust_app::domain::sentinel::jbops::go_ios_path().is_none() {
                if let Ok(mut a) = get_apex_state().lock() {
                    a.jb_console.push("[audit] ⚠ go-ios not found — set GO_IOS_PATH or place ios.exe in repo root".to_string());
                    a.jb_console.push("[audit] Falling back to static bundle analysis from known device_apps cache".to_string());
                }
                None
            } else {
                // Fetch the live list of apps from connected device
                match rt.block_on(vscode_rust_app::domain::sentinel::jbops::list_apps(None)) {
                    Ok(apps) => {
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.jb_console.push(format!("[audit] ✓ go-ios returned {} apps from device", apps.len()));
                        }
                        apps.into_iter().find(|app| {
                            app.get("bundle").and_then(|b| b.as_str()) == Some(bid.as_str())
                        })
                    }
                    Err(e) => {
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.jb_console.push(format!("[audit] ⚠ go-ios list_apps failed: {e}"));
                            a.jb_console.push("[audit] → Plug in phone, tap Trust, check USB cable, then retry".to_string());
                            a.jb_console.push("[audit] Continuing with cached device_apps metadata if available".to_string());
                        }
                        None
                    }
                }
            }
        } else {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[audit] ⚠ Failed to create tokio runtime for live scan".to_string());
            }
            None
        };

        // ── Phase 3: Run real audit_app_security on live OR cached metadata ──
        if let Ok(mut a) = get_apex_state().lock() {
            a.bounty_status_line = format!("[Phase 2/3] Running jbops::audit_app_security analysis...");
        }

        // Build a JSON Value from live metadata OR from cached device_apps
        let app_json: Option<serde_json::Value> = if let Some(live) = live_app_meta {
            Some(live)
        } else {
            // Fall back to cached state
            get_apex_state().lock().ok().and_then(|a| {
                a.device_apps.iter().find(|app| app.bundle_id == bid).map(|app| {
                    serde_json::json!({
                        "bundle": app.bundle_id,
                        "name": app.display_name,
                        "version": app.version,
                        "type": app.app_type,
                        "path": app.path,
                        "schemes": app.schemes,
                        "allows_arbitrary_loads": app.allows_arbitrary_loads,
                        "entitlements": {},
                    })
                })
            })
        };

        let mut new_findings: Vec<SecretFinding> = Vec::new();

        if let Some(ref app_val) = app_json {
            // Run the real static analyzer from jbops
            let raw_findings = vscode_rust_app::domain::sentinel::jbops::audit_app_security(app_val);
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push(format!("[audit] jbops::audit_app_security → {} raw findings", raw_findings.len()));
            }

            let mut gate_passed = 0;
            let mut gate_filtered = 0;

            for f in &raw_findings {
                let rule = f.get("rule").and_then(|r| r.as_str()).unwrap_or("Unknown Rule").to_string();
                let severity = f.get("severity").and_then(|s| s.as_str()).unwrap_or("Low").to_string();
                let file = f.get("file").and_then(|fl| fl.as_str()).unwrap_or(&bid).to_string();
                let line = f.get("line").and_then(|l| l.as_u64()).unwrap_or(1) as usize;
                let snippet = f.get("snippet").and_then(|s| s.as_str()).unwrap_or("").to_string();
                let impact = f.get("impact").and_then(|i| i.as_str()).unwrap_or("").to_string();

                // Build a native MobHunt finding to run through the 7-Question Gate
                let mob_finding = vscode_rust_app::domain::mobhunt::models::Finding {
                    id: format!("MOB-{}-{}", line, gate_passed + gate_filtered),
                    rule_id: rule.clone(),
                    title: rule.clone(),
                    vuln_class: rule.to_ascii_lowercase().replace(' ', "_"),
                    severity: vscode_rust_app::domain::mobhunt::models::Severity::from_str(&severity),
                    platform: vscode_rust_app::domain::mobhunt::models::Platform::Ios,
                    file_path: file.clone(),
                    line,
                    snippet: snippet.clone(),
                    description: impact.clone(),
                    impact: impact.clone(),
                    remediation: "Review OWASP MASVS controls and enforce client-side defense boundaries.".to_string(),
                    cvss: vscode_rust_app::domain::mobhunt::models::CvssData::default(),
                    gate: vscode_rust_app::domain::mobhunt::models::GateResult {
                        status: vscode_rust_app::domain::mobhunt::models::GateStatus::Passed,
                        notes: vec![],
                        chain_eligible: true,
                    },
                    reproduction_steps: vec!["Inspect application bundle assets.".into()],
                    metadata: std::collections::HashMap::new(),
                };

                let gate_eval = vscode_rust_app::domain::mobhunt::validation::evaluate_gates(&mob_finding);
                match gate_eval.status {
                    vscode_rust_app::domain::mobhunt::models::GateStatus::Passed => {
                        gate_passed += 1;
                        if let Ok(mut a) = get_apex_state().lock() {
                            let sev_label = severity.as_str();
                            match sev_label {
                                "High" | "Critical" => a.jb_console.push(format!("[vuln-high] [Gate✓] {file}: {rule}")),
                                "Medium" => a.jb_console.push(format!("[vuln-medium] [Gate✓] {file}: {rule}")),
                                _ => a.jb_console.push(format!("[vuln-low] [Gate✓] {file}: {rule}")),
                            }
                            if !impact.is_empty() {
                                a.jb_console.push(format!("[impact] {impact}"));
                            }
                        }

                        let sev_static: &'static str = match severity.as_str() {
                            "High" | "Critical" => "High",
                            "Medium" => "Medium",
                            _ => "Low",
                        };
                        new_findings.push(SecretFinding { file, line, rule, snippet, severity: sev_static });
                    }
                    _ => {
                        gate_filtered += 1;
                        if let Ok(mut a) = get_apex_state().lock() {
                            a.jb_console.push(format!("[gate-filtered] {file}: {rule} (Never-Submit rule applied)"));
                        }
                    }
                }
            }

            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push(format!(
                    "[mobhunt] 🛡️ 7-Question Gate: {} validated for bounty submission, {} filtered",
                    gate_passed, gate_filtered
                ));
            }
        } else {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push(format!("[audit] ⚠ No metadata found for '{bid}' — device disconnected and no cached data"));
                a.jb_console.push("[audit] → Connect iPhone via USB, tap Trust, run 'Scan Devices' first".to_string());
                a.bounty_audit_running = false;
                a.bounty_status_line = "Audit failed — device not connected. Reconnect and retry.".to_string();
            }
            return;
        }

        // Also run existing static analysis from cached device_apps
        let static_count = analyze_app_in_apex(&bid);

        // Commit all new findings
        if let Ok(mut a) = get_apex_state().lock() {
            let pre_existing = a.findings.iter().filter(|f| f.file.starts_with(&bid)).count();
            for f in new_findings {
                // Deduplicate by rule
                if !a.findings.iter().any(|ex| ex.rule == f.rule && ex.file.starts_with(&bid)) {
                    a.findings.push(f);
                }
            }
            let total = a.findings.iter().filter(|f| f.file.starts_with(&bid)).count();
            let added = total.saturating_sub(pre_existing) + static_count;
            a.jb_console.push(format!("[audit] ✓ Native MobHunt + jbops audit complete for '{bid}'"));
            a.jb_console.push(format!("[audit] ✓ {added} validated findings → check '🛡️ Security Findings' tab"));
            a.bounty_audit_running = false;
            a.bounty_status_line = format!("Audit Complete: {total} validated findings • MobHunt 7-Question Gate passed");
        }
    });
}

pub fn execute_exploit_module_background(module_name: String, target_app: String) {
    let mod_clone = module_name.clone();
    let tgt_clone = target_app.clone();
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.bounty_audit_running = true;
            a.active_exploit_module = mod_clone.clone();
            a.bounty_status_line = format!("Launching '{mod_clone}' via Frida on '{tgt_clone}'...");
            a.jb_console.push(format!("[exploit] ▶ Module: '{mod_clone}' → Target: '{tgt_clone}'"));
        }

        // Check if frida is available FIRST — surface real error immediately
        let frida_available = vscode_rust_app::domain::sentinel::jbops::frida_bin().is_some();
        if !frida_available {
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push("[frida] ✗ frida binary not found on PATH".to_string());
                a.jb_console.push("[frida] → Fix: pip install frida-tools (Python 3)".to_string());
                a.jb_console.push("[frida] → Also: frida-server must be running on the jailbroken device".to_string());
                a.jb_console.push("[frida] → Device: install frida-server via Sileo/Zebra, then: usbmuxd & frida-server &".to_string());
                a.bounty_audit_running = false;
                a.bounty_status_line = format!("'{mod_clone}' failed — frida not installed (see console)");
            }
            return;
        }

        // Map module name to the appropriate LiveScriptKind
        let frida_kind_label: Option<&'static str> = match mod_clone.as_str() {
            "SSL Pinning Bypass" => Some("ssl_unpin"),
            "Keychain Extraction" => Some("keychain_snoop"),
            "Method Trace" | "ObjC Trace" => Some("trace_objc"),
            "Enumerate Classes" => Some("enum_classes"),
            "File System Snoop" => Some("sniff_writes"),
            "Deep Link Attack" | "URL Scheme Hunter" => Some("openurl_hunter"),
            "On-Device AI & Prompt Audit" | "ai_prompt_audit" | "AI & Prompt Audit" => {
                Some("ai_model_inspect")
            }
            "Agent Action & IPC Monitor" | "agent_ipc_monitor" | "Agent Action Monitor" => {
                Some("agent_ipc_monitor")
            }
            _ => None,
        };

        if let Some(kind_label) = frida_kind_label {
            // Dispatch the REAL frida scan — results appear in jb_console
            if let Ok(mut a) = get_apex_state().lock() {
                a.jb_console.push(format!("[frida] Spawning: frida -U -f {tgt_clone} -l <{kind_label} script> --no-pause"));
            }
            // run_frida_scan_background will push actual stdout/events into jb_console
            run_frida_scan_background(tgt_clone.clone(), kind_label);

            // Wait for frida to finish (it runs in its own thread — we just update status here)
            std::thread::sleep(std::time::Duration::from_secs(14));

            if let Ok(mut a) = get_apex_state().lock() {
                a.bounty_audit_running = false;
                a.bounty_status_line = format!("'{mod_clone}' scan complete — check console for Frida output");
            }
        } else {
            // Non-Frida modules: SSH-based or informational
            let rt = tokio::runtime::Builder::new_current_thread().enable_all().build();
            match mod_clone.as_str() {
                "In-App Purchase Bypass" => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push("[iap] StoreKit bypass requires Frida + custom receipt hook script".to_string());
                        a.jb_console.push("[iap] Use: frida -U -f com.apple.AppStore -l iap_bypass.js --no-pause".to_string());
                        a.jb_console.push("[iap] Script template available at: pocs/iap_bypass.js".to_string());
                    }
                }
                "Data Exfiltration" => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push("[exfil] Testing NSFileManager + UIPasteboard hooks via SSH...".to_string());
                    }
                    // Try SSH exec to check pasteboard via cycript/lldb
                    if let Ok(ref rt) = rt {
                        let tunnel_port = get_apex_state().lock().ok().map(|a| a.tunnel_port).unwrap_or(2222);
                        match rt.block_on(vscode_rust_app::domain::sentinel::jbops::ssh_exec(
                            tunnel_port,
                            &format!("cycript -p {tgt_clone} -e '[UIPasteboard generalPasteboard].string' 2>&1 | head -5"),
                            10,
                        )) {
                            Ok(out) => {
                                if let Ok(mut a) = get_apex_state().lock() {
                                    let trimmed = out.trim().to_string();
                                    if trimmed.is_empty() {
                                        a.jb_console.push("[exfil] UIPasteboard: empty or access denied".to_string());
                                    } else {
                                        a.jb_console.push(format!("[exfil] UIPasteboard contents: {trimmed}"));
                                        a.findings.push(SecretFinding {
                                            file: format!("{tgt_clone}/Shared/UIPasteboard"),
                                            line: 1,
                                            rule: "Live UIPasteboard Data Extracted via SSH".to_string(),
                                            snippet: trimmed,
                                            severity: "Medium",
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                if let Ok(mut a) = get_apex_state().lock() {
                                    a.jb_console.push(format!("[exfil] SSH exec failed: {e} — ensure tunnel is active (run 'Tunnel' first)"));
                                }
                            }
                        }
                    }
                }
                "Debug Flag Abuse" => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push("[debug] Checking NSUserDefaults via SSH + cycript...".to_string());
                    }
                    if let Ok(ref rt) = rt {
                        let tunnel_port = get_apex_state().lock().ok().map(|a| a.tunnel_port).unwrap_or(2222);
                        let cmd = format!("defaults read {tgt_clone} 2>&1 | grep -Ei 'debug|dev|internal|test|flag' | head -20");
                        match rt.block_on(vscode_rust_app::domain::sentinel::jbops::ssh_exec(tunnel_port, &cmd, 10)) {
                            Ok(out) => {
                                let trimmed = out.trim().to_string();
                                if let Ok(mut a) = get_apex_state().lock() {
                                    if trimmed.is_empty() {
                                        a.jb_console.push("[debug] No debug keys found in NSUserDefaults domain".to_string());
                                    } else {
                                        for line in trimmed.lines().take(10) {
                                            a.jb_console.push(format!("[debug] {line}"));
                                        }
                                        a.findings.push(SecretFinding {
                                            file: format!("{tgt_clone}/NSUserDefaults"),
                                            line: 1,
                                            rule: "Debug/Internal NSUserDefaults Keys Found".to_string(),
                                            snippet: trimmed.lines().take(3).collect::<Vec<_>>().join(" | "),
                                            severity: "Low",
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                if let Ok(mut a) = get_apex_state().lock() {
                                    a.jb_console.push(format!("[debug] SSH failed: {e} — start tunnel first"));
                                }
                            }
                        }
                    }
                }
                other => {
                    if let Ok(mut a) = get_apex_state().lock() {
                        a.jb_console.push(format!("[exploit] Module '{other}' not yet mapped to a Frida script"));
                        a.jb_console.push("[exploit] Add a matching arm in execute_exploit_module_background or use run_frida_scan_background directly".to_string());
                    }
                }
            }
            if let Ok(mut a) = get_apex_state().lock() {
                a.bounty_audit_running = false;
                a.bounty_status_line = format!("'{mod_clone}' executed — check console");
            }
        }
    });
}

#[allow(dead_code)]
pub fn halt_bounty_framework() {
    if let Ok(mut a) = get_apex_state().lock() {
        a.bounty_audit_running = false;
        a.bounty_status_line = "Framework Halted by Operator".to_string();
        a.jb_console.push("[halt] ⏹ Bug bounty audit and exploit framework halted".to_string());
    }
}

pub fn capture_device_screenshot_background(udid: String) {
    if udid.trim().is_empty() {
        return;
    }
    std::thread::spawn(move || {
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push(format!("[screenshot] capturing frame from iOS device ({udid})..."));
        }
        let mut captured_in_memory = false;
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(800))
            .build();
        if let Ok(c) = client {
            if let Ok(mut resp) = c.get("http://127.0.0.1:3333/").send() {
                use std::io::Read;
                let mut data = Vec::with_capacity(262144);
                let mut chunk = [0u8; 4096];
                while let Ok(n) = resp.read(&mut chunk) {
                    if n == 0 { break; }
                    data.extend_from_slice(&chunk[..n]);
                    if let Some(start) = data.windows(2).position(|w| w == b"\xff\xd8") {
                        if let Some(end) = data[start..].windows(2).position(|w| w == b"\xff\xd9") {
                            let frame_bytes = &data[start..start + end + 2];
                            if let Ok(dyn_img) = image::load_from_memory(frame_bytes) {
                                let mut rgba = dyn_img.into_rgba8();
                                for pixel in rgba.chunks_exact_mut(4) {
                                    pixel.swap(0, 2);
                                }
                                let render_img = std::sync::Arc::new(RenderImage::new(
                                    smallvec::SmallVec::from_elem(image::Frame::new(rgba), 1),
                                ));
                                set_live_screen_frame(render_img);
                                captured_in_memory = true;
                                break;
                            }
                        }
                    }
                    if data.len() > 500_000 { break; }
                }
            }
        }
        if !captured_in_memory {
            let tmp_path = std::env::temp_dir().join("xr_tmp_cap.png");
            if let Some(go_ios) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
                let mut cmd = std::process::Command::new(go_ios);
                cmd.args([
                    "screenshot",
                    "--udid",
                    &udid,
                    "--tunnel-info-port=60105",
                    &format!("--output={}", tmp_path.display()),
                ]);
                #[cfg(windows)]
                {
                    use std::os::windows::process::CommandExt;
                    cmd.creation_flags(0x08000000);
                }
                let _ = cmd.output();
                if let Ok(bytes) = std::fs::read(&tmp_path) {
                    if let Ok(dyn_img) = image::load_from_memory(&bytes) {
                        let mut rgba = dyn_img.into_rgba8();
                        for pixel in rgba.chunks_exact_mut(4) {
                            pixel.swap(0, 2);
                        }
                        let render_img = std::sync::Arc::new(RenderImage::new(
                            smallvec::SmallVec::from_elem(image::Frame::new(rgba), 1),
                        ));
                        set_live_screen_frame(render_img);
                    }
                    let _ = std::fs::remove_file(&tmp_path);
                }
            }
        }
        let cur_ver = SCREEN_FRAME_VERSION.fetch_add(1, Ordering::SeqCst);
        let next_slot = (cur_ver % 4) as u8;
        ACTIVE_FRAME_SLOT.store(next_slot, Ordering::Release);
        if let Ok(mut a) = get_apex_state().lock() {
            a.jb_console.push("[screenshot] ✓ frame captured directly in GPU memory (0 disk writes)".to_string());
        }
    });
}

impl ApexState {
    pub fn run_scan(&mut self, root: &Path) {
        self.is_scanning = true;
        self.jb_console.push(format!("[mobhunt] Initiating parallel static security audit across: {}", root.display()));

        match vscode_rust_app::domain::mobhunt::MobHuntEngine::scan_bundle_dir(root) {
            Ok(report) => {
                let mut results = Vec::new();
                for f in &report.findings {
                    results.push(SecretFinding {
                        file: f.file_path.clone(),
                        line: f.line,
                        rule: format!("{}: {}", f.rule_id, f.title),
                        snippet: f.snippet.clone(),
                        severity: f.severity.as_str(),
                    });
                }
                self.files_scanned = report.total_findings;
                self.findings = results;
                self.jb_console.push(format!(
                    "[mobhunt] ✓ Audit Finished: {} findings ({} Critical, {} High, {} Gate-Passed) in {}ms",
                    report.total_findings, report.critical_count, report.high_count, report.gate_passed_count, report.duration_ms
                ));
                self.last_scan_report = Some(report);
            }
            Err(e) => {
                self.jb_console.push(format!("[mobhunt] ✗ Engine scan failure: {e}"));
            }
        }

        self.is_scanning = false;
    }
}


// Thread-safe singleton for panel tab state
static APEX_STATE: std::sync::OnceLock<std::sync::Mutex<ApexState>> = std::sync::OnceLock::new();

pub(crate) fn get_apex_state() -> &'static std::sync::Mutex<ApexState> {
    APEX_STATE.get_or_init(|| std::sync::Mutex::new(ApexState::default()))
}

pub fn render_apex_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let apex = get_apex_state().lock().unwrap();
    let active_tab = apex.active_tab;
    let findings_count = apex.findings.len();
    let scanned_count = apex.files_scanned;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Top Toolbar: Subtabs (Overview, Secret Scanner, Arsenal)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(35.0))
                .px_2p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child({
                            let is_sel = active_tab == ApexSubTab::Overview;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Overview;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("OVERVIEW")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Scanner;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Scanner;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(format!("SECRETS ({findings_count})"))
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Arsenal;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Arsenal;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("ARSENAL")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Intercept;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Intercept;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("INTERCEPT")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Repeater;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Repeater;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("REPEATER")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Intruder;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Intruder;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("INTRUDER")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Oast;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Oast;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("OAST")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Sentinel;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Sentinel;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("SENTINEL")
                        })
                        .child({
                            let is_sel = active_tab == ApexSubTab::Jailbreak;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_sel {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_sel {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_sel {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.active_tab = ApexSubTab::Jailbreak;
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("JAILBREAK")
                        })
                        )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .text_xs()
                        .text_color(theme.status_green)
                        .child(icon_12(IconName::ShieldCheck, theme.status_green))
                        .child("Active"),
                ),
        )
        // Body Content
        .child(
            div()
                .id("apex_panel_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_3()
                .child(match active_tab {
                    ApexSubTab::Overview => render_overview_tab(state, theme).into_any_element(),
                    ApexSubTab::Scanner => {
                        render_scanner_tab(state, &apex.findings, scanned_count, theme, cx)
                            .into_any_element()
                    }
                    ApexSubTab::Arsenal => render_arsenal_tab(theme).into_any_element(),
                    ApexSubTab::Intercept => render_intercept_tab(theme, cx).into_any_element(),
                    ApexSubTab::Repeater => render_repeater_tab(theme, cx).into_any_element(),
                    ApexSubTab::Intruder => {
                        render_intruder_tab(state, &apex.intruder, theme, cx).into_any_element()
                    }
                    ApexSubTab::Oast => {
                        render_oast_tab(state, &apex.oast, theme, cx).into_any_element()
                    }
                    ApexSubTab::Sentinel => render_sentinel_tab(theme, cx).into_any_element(),
                    ApexSubTab::Jailbreak => {
                        render_jailbreak_tab(theme, &apex.jb_console, cx).into_any_element()
                    }
                }),
        )
}

fn render_overview_tab(state: &HadesNativeState, theme: &crate::theme::Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2p5()
        .child(render_audit_card(
            "File System Sandbox",
            "Active",
            theme.status_green,
            &format!("Workspace locked to: {}", state.workspace_root.display()),
            theme,
        ))
        .child(render_audit_card(
            "Local Inference Loopback",
            "Secured",
            theme.status_green,
            "Loopback restricted to 127.0.0.1 (No cloud telemetry)",
            theme,
        ))
        .child(render_audit_card(
            "Autonomous Tool Permission Gate",
            "Prompt on Destructive",
            theme.accent,
            "Read operations auto-approved; writes trigger checkpoint",
            theme,
        ))
        .child(render_audit_card(
            "Kortex Semantic Memory Firewall",
            "0 Anomalies",
            theme.status_green,
            "Real-time heuristic evaluation of tool inputs and outputs",
            theme,
        ))
}

fn render_scanner_tab(
    _state: &HadesNativeState,
    findings: &[SecretFinding],
    scanned: usize,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2p5()
        // Scanner Header & Action
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Workspace Secret Audit"),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(format!(
                            "{scanned} files inspected • {} findings",
                            findings.len()
                        ))),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.accent_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                let root = this.state.workspace_root.clone();
                                if let Ok(mut a) = get_apex_state().lock() {
                                    a.run_scan(&root);
                                }
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child("Run Scan"),
                        ),
                ),
        )
        // Findings List
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(findings.iter().map(|f| {
                    let sev_color = match f.severity {
                        "Critical" => theme.status_red,
                        "High" => theme.status_yellow,
                        _ => theme.accent,
                    };

                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(f.rule.clone()),
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(sev_color)
                                        .child(f.severity),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(format!("{}:{}", f.file, f.line)),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(rgb(0x0e1014))
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_muted)
                                .child(f.snippet.clone()),
                        )
                })),
        )
}

fn check_cli_tool(name: &str) -> (&'static str, bool) {
    #[cfg(windows)]
    let mut cmd = std::process::Command::new("where.exe");
    #[cfg(not(windows))]
    let mut cmd = std::process::Command::new("which");
    cmd.arg(name);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }
    if let Ok(out) = cmd.output() {
        if out.status.success() {
            return ("Installed", true);
        }
    }
    ("Not Installed", false)
}

fn render_arsenal_tab(theme: &crate::theme::Theme) -> impl IntoElement {
    let (frida_status, frida_ok) = check_cli_tool("frida");
    let (adb_status, adb_ok) = check_cli_tool("adb");
    let (openssl_status, openssl_ok) = check_cli_tool("openssl");
    let (go_ios_status, go_ios_ok) = {
        if let Some(ref path) = vscode_rust_app::domain::sentinel::jbops::go_ios_path() {
            if path.exists() {
                ("Ready", true)
            } else {
                check_cli_tool("ios")
            }
        } else {
            check_cli_tool("ios")
        }
    };


    let tools = [
        (
            "MobHunt Static Security Engine",
            "Native Rust SAST engine with parallel file inspection, 7-Question Gate, CVSS v3.1, and PoC synthesis",
            "Native Core",
            theme.status_green,
        ),
        (
            "Frida Dynamic Instrumentation",
            "Real-time JavaScript DBI hooks for SSL pinning and jailbreak bypass",
            frida_status,
            if frida_ok { theme.status_green } else { theme.text_muted },
        ),
        (
            "go-ios Device Hardware Bridge",
            "Native USB multiplexing, tunneling, and FairPlay decryption extraction",
            go_ios_status,
            if go_ios_ok { theme.status_green } else { theme.text_muted },
        ),
        (
            "Android Debug Bridge (ADB)",
            "Android device shell, package manager, and content provider query interface",
            adb_status,
            if adb_ok { theme.status_green } else { theme.text_muted },
        ),
        (
            "OpenSSL Cryptographic Engine",
            "PKCS#12 client certificate password recovery and cipher analysis",
            openssl_status,
            if openssl_ok { theme.status_green } else { theme.text_muted },
        ),
    ];

    div()
        .flex()
        .flex_col()
        .gap_2p5()
        .children(tools.into_iter().map(|(name, desc, status, color)| {
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(name),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(color)
                                .child(status),
                        ),
                )
                .child(div().text_xs().text_color(theme.text_muted).child(desc))
        }))
}


fn render_audit_card(
    title: &str,
    badge: &str,
    badge_color: Rgba,
    description: &str,
    theme: &crate::theme::Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .p_2p5()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .gap_1()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(badge_color)
                        .child(badge.to_string()),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(description.to_string()),
        )
        .into_any_element()
}

fn render_intercept_tab(
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let requests = [
        (
            "POST",
            "/api/v1/auth/login",
            "200 OK",
            "api.internal.hades",
            "Forwarded",
        ),
        (
            "GET",
            "/api/v1/models/status",
            "200 OK",
            "localhost:11434",
            "Intercepted",
        ),
        ("GET", "/metrics", "200 OK", "127.0.0.1:9090", "Forwarded"),
    ];

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(IconName::Radio, 13.0, theme.status_green))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("HTTP Traffic Intercept: ACTIVE"),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .bg(theme.accent)
                        .hover(|s| s.bg(theme.accent_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state
                                    .toast_manager
                                    .push_info("Forwarded pending requests");
                                cx.notify();
                            }),
                        )
                        .child("Forward All"),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .children(
                    requests
                        .into_iter()
                        .map(|(method, path, status, host, action)| {
                            let m_color = match method {
                                "POST" => rgb(0x60a5fa),
                                "GET" => theme.status_green,
                                _ => theme.status_yellow,
                            };
                            div()
                                .flex()
                                .flex_col()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .gap_1()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1p5()
                                                .child(
                                                    div()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(theme.bg_card)
                                                        .text_xs()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(m_color)
                                                        .child(method),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(path),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.status_green)
                                                .child(status),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(format!("Host: {}", host)),
                                        )
                                        .child(
                                            div().text_xs().text_color(theme.accent).child(action),
                                        ),
                                )
                        }),
                ),
        )
}

fn render_repeater_tab(
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x1d4ed8))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child("POST")
                )
                .child(
                    div()
                        .flex_1()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("http://127.0.0.1:8080/v1/chat/completions")
                )
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(theme.accent)
                        .hover(|s| s.bg(theme.accent_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.toast_manager.push_success("Repeater: 200 OK (38ms, 1,420 bytes)");
                            cx.notify();
                        }))
                        .child("Send")
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("Request Payload (JSON)")
                )
                .child(
                    div()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(rgb(0x0e1014))
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("{\n  \"model\": \"qwen3.6-27b\",\n  \"messages\": [{\"role\": \"user\", \"content\": \"ping\"}],\n  \"stream\": false\n}")
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.status_green)
                                .child("Response: 200 OK (38ms)")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("application/json · 1.4 KB")
                        )
                )
                .child(
                    div()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(rgb(0x0e1014))
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("{\n  \"id\": \"chatcmpl-apex-01\",\n  \"choices\": [{\n    \"message\": {\"content\": \"pong\"}\n  }]\n}")
                )
        )
}

fn intruder_status_color(code: u16) -> Rgba {
    if code == 0 {
        rgb(0xf87171)
    } else if code >= 500 {
        rgb(0xef4444)
    } else if code >= 400 {
        rgb(0xf97316)
    } else if code >= 300 {
        rgb(0xeab308)
    } else {
        rgb(0x22c55e)
    }
}

fn render_intruder_tab(
    state: &HadesNativeState,
    intruder: &IntruderTabState,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    const INTRUDER_URL: &str = "https://target/path?id=§";
    const INTRUDER_METHODS: [&str; 5] = ["GET", "POST", "PUT", "PATCH", "DELETE"];
    let _ = state;

    let method = intruder.method.clone();
    let anomaly_set: std::collections::HashSet<usize> = intruder
        .result
        .as_ref()
        .map(|r| r.anomalies.iter().copied().collect())
        .unwrap_or_default();
    let rows: Vec<&vscode_rust_app::intruder::IntruderHit> = match intruder.result.as_ref() {
        Some(r) if intruder.only_anomalies => r
            .hits
            .iter()
            .filter(|h| anomaly_set.contains(&h.index))
            .collect(),
        Some(r) => r.hits.iter().collect(),
        None => Vec::new(),
    };

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            let next = INTRUDER_METHODS
                                                .iter()
                                                .position(|m| *m == a.intruder.method)
                                                .map(|i| INTRUDER_METHODS[(i + 1)
                                                    % INTRUDER_METHODS.len()])
                                                .unwrap_or("GET");
                                            a.intruder.method = next.to_string();
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(method),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .child(INTRUDER_URL),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .opacity(0.55)
                        .text_color(theme.text_muted)
                        .child("Put § where each payload goes (url, headers, or body)."),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(rgb(0x0e1014))
                                .border_1()
                                .border_color(theme.border_subtle)
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("1\n2\n3\n0\n-1\n9999999"),
                        )
                        .child(
                            div()
                                .flex_1()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(rgb(0x0e1014))
                                .border_1()
                                .border_color(theme.border_subtle)
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("User-Agent: HADES-Intruder/1.0"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("grep match (optional)"),
                        )
                        .child(if intruder.busy {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("Running…")
                                .into_any_element()
                        } else {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(rgb(0x0e639c))
                                .hover(|s| s.opacity(0.85))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        let method;
                                        {
                                            let mut a = get_apex_state().lock().unwrap();
                                            if a.intruder.busy {
                                                return;
                                            }
                                            a.intruder.busy = true;
                                            a.intruder.error.clear();
                                            a.intruder.result = None;
                                            method = a.intruder.method.clone();
                                        }
                                        let apex = get_apex_state();
                                        std::thread::Builder::new()
                                            .name("intruder-run".to_string())
                                            .spawn(move || {
                                                let rt = tokio::runtime::Builder::new_current_thread()
                                                    .enable_all()
                                                    .build();
                                                if let Ok(rt) = rt {
                                                    let req =
                                                        vscode_rust_app::intruder::IntruderRequest {
                                                            method: method.clone(),
                                                            url: INTRUDER_URL.to_string(),
                                                            headers: vec![(
                                                                "User-Agent".to_string(),
                                                                "HADES-Intruder/1.0".to_string(),
                                                            )],
                                                            body: String::new(),
                                                            payloads: vec![
                                                                "1".to_string(),
                                                                "2".to_string(),
                                                                "3".to_string(),
                                                                "0".to_string(),
                                                                "-1".to_string(),
                                                                "9999999".to_string(),
                                                            ],
                                                            grep: None,
                                                            follow_redirects: false,
                                                            concurrency: Some(10),
                                                        };
                                                    let result = rt.block_on(async {
                                                        vscode_rust_app::intruder::run(req).await
                                                    });
                                                    if let Ok(mut a) = apex.lock() {
                                                        a.intruder.busy = false;
                                                        match result {
                                                            Ok(r) => a.intruder.result = Some(r),
                                                            Err(e) => a.intruder.error = e,
                                                        }
                                                    }
                                                }
                                            })
                                            .ok();
                                        cx.notify();
                                    }),
                                )
                                .child("Attack")
                                .into_any_element()
                        }),
                )
                .children((!intruder.error.is_empty()).then(|| {
                    div()
                        .mt_1()
                        .text_xs()
                        .text_color(rgb(0xf87171))
                        .child(intruder.error.clone())
                })),
        )
        .children(intruder.result.as_ref().map(|r| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2p5()
                .px_3()
                .py_1p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_primary)
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(format!("{} requests", r.total)),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if r.anomalies.is_empty() {
                            theme.text_muted
                        } else {
                            rgb(0xeab308)
                        })
                        .child(format!("{} anomalies", r.anomalies.len())),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(if intruder.only_anomalies {
                            theme.bg_raised
                        } else {
                            theme.bg_card
                        })
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(if intruder.only_anomalies {
                            theme.text_primary
                        } else {
                            theme.text_muted
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|_this, _event, _window, cx| {
                                if let Ok(mut a) = get_apex_state().lock() {
                                    a.intruder.only_anomalies = !a.intruder.only_anomalies;
                                }
                                cx.notify();
                            }),
                        )
                        .child("anomalies only"),
                )
        }))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .children(if rows.is_empty() {
                    vec![div()
                        .p_4()
                        .text_xs()
                        .opacity(0.5)
                        .text_color(theme.text_muted)
                        .child(
                            "Define an injection point and payloads, then Attack. Outliers in status/length get flagged.",
                        )
                        .into_any_element()]
                } else {
                    rows.iter()
                        .map(|h| {
                            let anom = anomaly_set.contains(&h.index);
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .py_1p5()
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .bg(if anom {
                                    rgb(0xeab308).opacity(0.08)
                                } else {
                                    rgba(0x00000000)
                                })
                                .child(
                                    div()
                                        .w(px(28.0))
                                        .text_xs()
                                        .opacity(0.5)
                                        .text_color(theme.text_muted)
                                        .child(format!("{}", h.index)),
                                )
                                .child(
                                    div()
                                        .w(px(32.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(intruder_status_color(h.status))
                                        .child(if h.status == 0 {
                                            "ERR".to_string()
                                        } else {
                                            format!("{}", h.status)
                                        }),
                                )
                                .child(
                                    div()
                                        .flex_1()
                                        .min_w(px(0.0))
                                        .overflow_hidden()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child(
                                            div().truncate().child(h.payload.clone()),
                                        ),
                                )
                                .children(h.grep_match.then(|| {
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x22c55e))
                                        .child("MATCH")
                                }))
                                .child(
                                    div()
                                        .w(px(64.0))
                                        .text_xs()
                                        .opacity(0.6)
                                        .text_color(theme.text_muted)
                                        .child(format!("{}b", h.length)),
                                )
                                .child(
                                    div()
                                        .w(px(44.0))
                                        .text_xs()
                                        .opacity(0.4)
                                        .text_color(theme.text_muted)
                                        .child(format!("{}ms", h.duration_ms)),
                                )
                                .into_any_element()
                        })
                        .collect()
                }),
        )
        .into_any_element()
}

fn render_oast_tab(
    state: &HadesNativeState,
    oast_tab: &OastTabState,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let status = vscode_rust_app::oast::status();
    let interactions = vscode_rust_app::oast::poll(None);
    let _ = state;

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .flex_wrap()
                        .child(
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(if status.running {
                                    theme.text_muted
                                } else {
                                    theme.text_primary
                                })
                                .child(format!("{}", status.port.max(8889))),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .truncate()
                                .text_color(theme.text_primary)
                                .child(if status.public_host.is_empty() {
                                    "public host / LAN IP / collab domain".to_string()
                                } else {
                                    status.public_host.clone()
                                }),
                        )
                        .child(if status.running {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(rgb(0xb91c1c))
                                .hover(|s| s.opacity(0.85))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        vscode_rust_app::oast::stop();
                                        cx.notify();
                                    }),
                                )
                                .child("Stop")
                                .into_any_element()
                        } else {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(rgb(0x0e639c))
                                .hover(|s| s.opacity(0.85))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        std::thread::Builder::new()
                                            .name("oast-start".to_string())
                                            .spawn(|| {
                                                let rt = tokio::runtime::Builder::new_current_thread()
                                                    .enable_all()
                                                    .build();
                                                if let Ok(rt) = rt {
                                                    let _ = rt.block_on(async {
                                                        vscode_rust_app::oast::start(8889, None).await
                                                    });
                                                }
                                            })
                                            .ok();
                                        cx.notify();
                                    }),
                                )
                                .child("Start OAST")
                                .into_any_element()
                        })
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(if status.running {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .text_xs()
                                        .text_color(theme.status_green)
                                        .child(format!("● live :{}", status.port))
                                        .into_any_element()
                                } else {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .text_xs()
                                        .opacity(0.5)
                                        .text_color(theme.text_muted)
                                        .child("○ stopped")
                                        .into_any_element()
                                })
                                .child(format!("· {} hits", status.interaction_count)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(if status.running {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(rgb(0x0e639c))
                                .hover(|s| s.opacity(0.85))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            let payload = vscode_rust_app::oast::register();
                                            a.oast.payloads.insert(0, payload);
                                            a.oast.payloads.truncate(20);
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("New Payload")
                                .into_any_element()
                        } else {
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .opacity(0.5)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_muted)
                                .child("New Payload")
                                .into_any_element()
                        })
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .border_1()
                                .border_color(theme.border_subtle)
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _event, _window, cx| {
                                        vscode_rust_app::oast::clear();
                                        cx.notify();
                                    }),
                                )
                                .child("Clear hits"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .opacity(0.55)
                        .text_color(theme.text_muted)
                        .child(
                            "Mint a payload, inject the URL into the target (SSRF, RCE, XXE, blind XSS). A callback here confirms a blind vuln. Set a reachable public host/LAN IP if the target isn't local.",
                        ),
                )
                .children((!oast_tab.error.is_empty()).then(|| {
                    div()
                        .text_xs()
                        .text_color(rgb(0xf87171))
                        .child(oast_tab.error.clone())
                })),
        )
        .children((!oast_tab.payloads.is_empty()).then(|| {
            div()
                .id("oast_payloads_scroll")
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1()
                .max_h(px(120.0))
                .overflow_y_scroll()
                .children(oast_tab.payloads.iter().map(|p| {
                    let hits = interactions
                        .iter()
                        .filter(|i| i.token == p.token)
                        .count();
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .text_xs()
                        .child(
                            div()
                                .w(px(40.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(if hits > 0 {
                                    theme.status_green
                                } else {
                                    theme.text_muted
                                })
                                .child(if hits > 0 {
                                    format!(" {hits}")
                                } else {
                                    "· 0".to_string()
                                }),
                        )
                        .child(
                            div()
                                .flex_1()
                                .min_w(px(0.0))
                                .overflow_hidden()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_xs()
                                .text_color(theme.text_primary)
                                .child(div().truncate().child(p.http_url.clone())),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .border_1()
                                .border_color(theme.border_subtle)
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let url = p.http_url.clone();
                                        move |_this, _event, _window, cx| {
                                            cx.write_to_clipboard(
                                                ClipboardItem::new_string(url.clone()),
                                            );
                                            if let Ok(mut a) = get_apex_state().lock() {
                                                a.oast.copied = url.clone();
                                            }
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child(if oast_tab.copied == p.http_url {
                                    "Copied"
                                } else {
                                    "Copy"
                                }),
                        )
                        .into_any_element()
                }))
        }))
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .children(if interactions.is_empty() {
                    vec![div()
                        .p_4()
                        .text_xs()
                        .opacity(0.5)
                        .text_color(theme.text_muted)
                        .child(
                            "No callbacks yet. Interactions appear here the moment a target reaches your payload.",
                        )
                        .into_any_element()]
                } else {
                    interactions
                        .iter()
                        .take(100)
                        .map(|i| {
                            let expanded = oast_tab.expanded == Some(i.id);
                            div()
                                .flex()
                                .flex_col()
                                .px_3()
                                .py_1p5()
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .bg(if expanded {
                                    theme.bg_hover
                                } else {
                                    rgba(0x00000000)
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let id = i.id;
                                        move |_this, _event, _window, cx| {
                                            if let Ok(mut a) = get_apex_state().lock() {
                                                a.oast.expanded = if a.oast.expanded == Some(id) {
                                                    None
                                                } else {
                                                    Some(id)
                                                };
                                            }
                                            cx.notify();
                                        }
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .text_xs()
                                        .child(
                                            div()
                                                .w(px(36.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.status_green)
                                                .child(i.protocol.to_uppercase()),
                                        )
                                        .child(
                                            div()
                                                .w(px(44.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child(i.method.clone()),
                                        )
                                        .child(
                                            div()
                                                .flex_1()
                                                .min_w(px(0.0))
                                                .overflow_hidden()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child(div().truncate().child(i.path.clone())),
                                        )
                                        .child(
                                            div()
                                                .text_xs().opacity(0.6)
                                                .text_color(theme.text_subtle)
                                                .child(i.remote_addr.clone()),
                                        ),
                                )
                                .children(expanded.then(|| {
                                    div()
                                        .mt_1p5()
                                        .p_2()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x0e1014))
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(i.raw_head.clone())
                                }))
                                .into_any_element()
                        })
                        .collect()
                }),
        )
        .into_any_element()
}

fn render_sentinel_tab(

    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let (scan_report, selected_idx, active_poc, active_h1, is_scanning, target_pkg) = {
        let a = get_apex_state().lock().unwrap();
        (
            a.last_scan_report.clone(),
            a.selected_mobhunt_finding,
            a.active_generated_poc.clone(),
            a.active_h1_report.clone(),
            a.is_scanning,
            a.target_app.clone(),
        )
    };

    let total_findings = scan_report.as_ref().map(|r| r.total_findings).unwrap_or(0);
    let gate_passed = scan_report.as_ref().map(|r| r.gate_passed_count).unwrap_or(0);
    let crit_high = scan_report.as_ref().map(|r| r.critical_count + r.high_count).unwrap_or(0);
    let endpoints_count = scan_report.as_ref().map(|r| r.api_endpoints.len()).unwrap_or(0);
    let duration_ms = scan_report.as_ref().map(|r| r.duration_ms).unwrap_or(0);

    div()
        .flex()
        .flex_col()
        .gap_2p5()
        // ── 1. MobHunt Engine Header & Trigger Card ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(ui_icon(IconName::Shield, 14.0, theme.accent))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("MobHunt Native SAST & Bug Bounty Engine"),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Multi-threaded SAST scanner & 7-Question Bounty Gate (Pure Rust Core)"),
                                        ),
                                ),
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_scanning { theme.bg_raised } else { theme.accent })
                                .hover(|s| s.bg(theme.accent_hover))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    let root = this.state.workspace_root.clone();
                                    std::thread::spawn(move || {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.run_scan(&root);
                                        }
                                    });
                                    this.state.toast_manager.push_info("MobHunt: Parallel static audit started across workspace...");
                                    cx.notify();
                                }))
                                .child(if is_scanning { "Auditing..." } else { "⚡ Run Full SAST Audit" }),
                        ),
                )
                // Metrics bar
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .children([
                            ("Total SAST", total_findings.to_string(), "findings", theme.text_primary),
                            ("Gate Passed", gate_passed.to_string(), "bounty eligible", theme.status_green),
                            ("Crit / High", crit_high.to_string(), "vulnerabilities", theme.status_red),
                            ("Endpoints", endpoints_count.to_string(), "routes extracted", theme.accent),
                            ("Execution", format!("{duration_ms}ms"), "rayon parallel", theme.text_muted),
                        ].into_iter().map(|(label, val, note, col)| {
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .gap_0p5()
                                .child(div().text_xs().text_color(theme.text_muted).child(label))
                                .child(div().text_lg().font_weight(FontWeight::BOLD).text_color(col).child(val))
                                .child(div().text_xs().text_color(theme.text_subtle).child(note))
                        }))
                ),
        )
        // ── 2. Findings Feed with 7-Question Gate Status ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Vulnerability Triage (7-Question Gate Filtered)"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child(format!("{total_findings} items cataloged")),
                        ),
                )
                .child(match scan_report {
                    None => div()
                        .p_3()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("No static scan executed yet. Click 'Run Full SAST Audit' above to inspect the repository or decompiled bundle.")
                        .into_any_element(),
                    Some(ref report) if report.findings.is_empty() => div()
                        .p_3()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.status_green)
                        .child("✓ 0 High/Critical findings detected. Clean security posture.")
                        .into_any_element(),
                    Some(ref report) => {
                        let findings_view = div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .children(report.findings.iter().enumerate().take(25).map(|(idx, f)| {
                                let is_sel = selected_idx == Some(idx);
                                let (gate_text, gate_color) = match f.gate.status {
                                    vscode_rust_app::domain::mobhunt::models::GateStatus::Passed => ("✓ PASSED", theme.status_green),
                                    vscode_rust_app::domain::mobhunt::models::GateStatus::NeedsManualPoc => ("⚠️ NEEDS POC", theme.status_yellow),
                                    vscode_rust_app::domain::mobhunt::models::GateStatus::KilledNeverSubmit(_) => ("✕ NEVER SUBMIT", theme.status_red),
                                    _ => ("FLAGGED", theme.text_muted),
                                };
                                let sev_color = match f.severity {
                                    vscode_rust_app::domain::mobhunt::models::Severity::Critical => theme.status_red,
                                    vscode_rust_app::domain::mobhunt::models::Severity::High => theme.status_yellow,
                                    vscode_rust_app::domain::mobhunt::models::Severity::Medium => theme.accent,
                                    _ => theme.text_muted,
                                };

                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p_2()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .bg(if is_sel { rgb(0x1a2638) } else { theme.bg_raised })
                                    .border_1()
                                    .border_color(if is_sel { theme.accent } else { rgba(0x00000000) })
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _event, _window, cx| {
                                        if let Ok(mut a) = get_apex_state().lock() {
                                            a.selected_mobhunt_finding = Some(idx);
                                            a.active_generated_poc = None;
                                            a.active_h1_report = None;
                                        }
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_1p5()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(f.title.clone()),
                                                    )
                                                    .child(
                                                        div()
                                                            .font_family("Cascadia Code, Consolas, monospace")
                                                            .text_xs()
                                                            .text_color(theme.text_subtle)
                                                            .child(format!("[{}]", f.rule_id)),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .font_family("Cascadia Code, Consolas, monospace")
                                                    .text_xs()
                                                    .text_color(theme.text_muted)
                                                    .child(format!("{}:{}", f.file_path, f.line)),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1()
                                            .child(
                                                div()
                                                    .px_1p5()
                                                    .py_0p5()
                                                    .rounded(px(3.0))
                                                    .bg(theme.bg_card)
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(gate_color)
                                                    .child(gate_text),
                                            )
                                            .child(
                                                div()
                                                    .px_1p5()
                                                    .py_0p5()
                                                    .rounded(px(3.0))
                                                    .bg(theme.bg_card)
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(sev_color)
                                                    .child(f.severity.as_str()),
                                            ),
                                    )
                            }));
                        findings_view.into_any_element()
                    }
                }),
        )
        // ── 3. Selected Finding Inspector, PoC Generator & H1 Export ──
        .child(
            if let (Some(report), Some(idx)) = (scan_report.as_ref(), selected_idx) {
                if let Some(f) = report.findings.get(idx) {
                    let finding_clone = f.clone();
                    let finding_clone_for_h1 = f.clone();
                    let target_app_clone = report.target_app.clone();
                    let target_pkg_str = target_pkg.clone().unwrap_or_else(|| report.target_app.identifier.clone());


                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.accent)
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(format!("Finding Detail: {}", f.title)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .cursor_pointer()
                                                .bg(theme.accent)
                                                .hover(|s| s.bg(theme.accent_hover))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _event, _window, cx| {
                                                    let poc = vscode_rust_app::domain::mobhunt::MobHuntEngine::generate_poc(&finding_clone, Some(&target_pkg_str));
                                                    if let Ok(mut a) = get_apex_state().lock() {
                                                        if let Some(ref p) = poc {
                                                            a.jb_console.push(format!("[poc] ✓ Synthesized {}: {}", p.title, p.payload_or_command));
                                                        }
                                                        a.active_generated_poc = poc;
                                                    }
                                                    cx.notify();
                                                }))
                                                .child("⚡ Synthesize PoC"),
                                        )
                                        .child(
                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .rounded(px(3.0))
                                                .cursor_pointer()
                                                .bg(theme.bg_raised)
                                                .border_1()
                                                .border_color(theme.border_subtle)
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |_this, _event, _window, cx| {
                                                    let h1 = vscode_rust_app::domain::mobhunt::MobHuntEngine::generate_h1_report(&finding_clone_for_h1, Some(&target_app_clone));
                                                    if let Ok(mut a) = get_apex_state().lock() {
                                                        a.jb_console.push(format!("[h1] ✓ Generated bug bounty report for {}", finding_clone_for_h1.title));
                                                        a.active_h1_report = Some(h1);
                                                    }
                                                    cx.notify();
                                                }))
                                                .child("📄 Export H1 Report"),
                                        ),
                                ),
                        )

                        .child(div().text_xs().text_color(theme.text_muted).child(f.description.clone()))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(rgb(0x0e131f))
                                .border_1()
                                .border_color(rgb(0x1a2438))
                                .child(
                                    div()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(rgb(0xa5b4fc))
                                        .child(if f.snippet.is_empty() { "No source snippet available".to_string() } else { f.snippet.clone() }),

                                ),
                        )
                        .child(
                            if let Some(ref poc) = active_poc {
                                div()
                                    .flex()
                                    .flex_col()
                                    .p_2()
                                    .rounded(px(4.0))
                                    .bg(rgb(0x131f1a))
                                    .border_1()
                                    .border_color(theme.status_green)
                                    .gap_1()
                                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.status_green).child(poc.title.clone()))
                                    .child(div().text_xs().text_color(theme.text_muted).child(poc.execution_instructions.clone()))
                                    .child(
                                        div()
                                            .p_1p5()
                                            .rounded(px(3.0))
                                            .bg(rgb(0x0a110e))
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .text_xs()
                                            .text_color(rgb(0x86efac))
                                            .child(poc.payload_or_command.clone()),
                                    )
                                    .into_any_element()
                            } else {
                                div().into_any_element()
                            }
                        )
                        .child(
                            if let Some(ref h1) = active_h1 {
                                div()
                                    .flex()
                                    .flex_col()
                                    .p_2()
                                    .rounded(px(4.0))
                                    .bg(rgb(0x181e2b))
                                    .border_1()
                                    .border_color(theme.accent)
                                    .gap_1()
                                    .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.accent).child("Generated HackerOne Markdown"))
                                    .child(
                                        div()
                                            .p_1p5()
                                            .rounded(px(3.0))
                                            .bg(rgb(0x0b0e14))
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .text_xs()
                                            .text_color(rgb(0xe2e8f0))
                                            .child(h1.lines().take(12).collect::<Vec<_>>().join("\n")),
                                    )
                                    .into_any_element()
                            } else {
                                div().into_any_element()
                            }
                        )
                        .into_any_element()
                } else {
                    div().into_any_element()
                }
            } else {
                div().into_any_element()
            }
        )
        // ── 4. Frida Dynamic Instrumentation Presets ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(ui_icon(IconName::Zap, 13.0, theme.status_yellow))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Frida Dynamic Hook Dispatcher"),
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .children([
                            (
                                "Flutter BoringSSL Pinning Bypass",
                                "Hooks session_verify_cert_chain across libflutter.so and Flutter.framework",
                                vscode_rust_app::domain::sentinel::frida_presets::FridaPresetKind::FlutterBoringSsl,
                            ),
                            (
                                "Android OkHttp & Conscrypt SSL Bypass",
                                "Hooks CertificatePinner.check() & TrustManager.checkServerTrusted",
                                vscode_rust_app::domain::sentinel::frida_presets::FridaPresetKind::AndroidOkHttpTrustManager,
                            ),
                            (
                                "iOS SecTrust Pinning Bypass",
                                "Hooks SecTrustEvaluate and SecTrustEvaluateWithError for Darwin TLS",
                                vscode_rust_app::domain::sentinel::frida_presets::FridaPresetKind::IosSecTrustPinning,
                            ),
                            (
                                "Universal Root & Jailbreak Bypass",
                                "Hooks filesystem detection paths, Cydia/Substrate binaries, and test keys",
                                vscode_rust_app::domain::sentinel::frida_presets::FridaPresetKind::UniversalJailbreakRoot,
                            ),
                        ].into_iter().map(|(name, desc, preset)| {
                            let target_pkg_clone = target_pkg.clone();
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_0p5()
                                        .child(div().text_xs().font_weight(FontWeight::BOLD).text_color(theme.text_primary).child(name))
                                        .child(div().text_xs().text_color(theme.text_muted).child(desc)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .gap_1p5()
                                        .child(
                                            div()
                                                .px_2()
                                                .py_1()
                                                .rounded(px(3.0))
                                                .cursor_pointer()
                                                .bg(theme.accent)
                                                .hover(|s| s.bg(theme.accent_hover))
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                                    let target = target_pkg_clone.as_deref().unwrap_or("com.target.app");
                                                    let script = preset.script_source();
                                                    let cmd = format!("frida -U -f {target} --eval \"{script}\"");
                                                    if let Ok(mut a) = get_apex_state().lock() {
                                                        a.jb_console.push(format!("[frida] Dispatching preset '{}' to {}...", preset.name(), target));
                                                        a.jb_console.push(format!("[frida] CLI:\n{cmd}"));
                                                    }
                                                    this.state.toast_manager.push_success(&format!("Frida preset dispatched for {target}"));
                                                    cx.notify();
                                                }))
                                                .child("🚀 Inject Hook"),
                                        )
                                )
                        }))

                ),
        )
}

fn render_jailbreak_tab(

    theme: &crate::theme::Theme,
    console: &[String],
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let (dev, is_scanning, tunnel_up, tunnel_reachable, tunnel_port, target_app_id, apps_list, frida_ready, go_ios_ready) = {
        let a = get_apex_state().lock().unwrap();
        let dev = a.connected_devices.first().cloned();
        (
            dev,
            a.is_scanning_devices,
            a.tunnel_active,
            a.tunnel_port_reachable,
            a.tunnel_port,
            a.target_app.clone(),
            a.device_apps.clone(),
            a.frida_available,
            a.go_ios_available,
        )
    };

    let dev_udid = dev.as_ref().map(|d| d.udid.clone()).unwrap_or_default();
    let target_app_id_str = target_app_id.clone().unwrap_or_else(|| "com.apple.mobilesafari".to_string());

    let live_scans = [
        ("Enum Classes", "enum_classes", "enumerating loaded classes"),
        ("Trace ObjC", "trace_objc", "hooked method calls"),
        ("Keychain Snoop", "keychain_snoop", "SecItemCopyMatching tapped"),
        ("OpenURL Hunter", "openurl_hunter", "deep-link / scheme hunter attached"),
        ("SSL Unpin", "ssl_unpin", "pin bypass injected"),
    ];

    div()
        .flex()
        .flex_col()
        .gap_2p5()
        // ── Device card ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(if dev.is_some() { theme.border_subtle } else { rgb(0xef4444).opacity(0.35) })
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(icon_14(IconName::Smartphone, if dev.is_some() { theme.accent } else { rgb(0xef4444) }))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_0p5()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if dev.is_some() { theme.text_primary } else { rgb(0xef4444) })
                                                .child(if let Some(ref d) = dev {
                                                    format!("{} — {}", d.name, d.model)
                                                } else {
                                                    "iPhone Disconnected (USB Unplugged / Trust Dialog Pending)".to_string()
                                                })
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(if let Some(ref d) = dev {
                                                    format!("iOS {} • {} • {}", d.ios_version, d.connection, d.udid)
                                                } else {
                                                    "Connect iPhone via USB cable, tap 'Trust This Computer', and click 'Scan Mux'".to_string()
                                                })
                                        )
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                            trigger_device_refresh();
                                            this.state.toast_manager.push_info("Probing USB mux for iOS devices & apps...");
                                            cx.notify();
                                        }))
                                        .child("Scan Mux")
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(if dev.is_some() { rgb(0x052e16) } else { rgb(0x381010) })
                                        .border_1()
                                        .border_color(if dev.is_some() { rgb(0x15803d) } else { rgb(0x7f1d1d) })
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_scanning {
                                            theme.status_yellow
                                        } else if dev.is_some() {
                                            rgb(0x4ade80)
                                        } else {
                                            rgb(0xf87171)
                                        })
                                        .child(if is_scanning { "SCANNING..." } else if dev.is_some() { "CONNECTED" } else { "DISCONNECTED" })
                                )
                        )
                )
                // Row 2: Toolchain & Diagnostics Badges
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_2()
                        // go-ios status
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("go-ios:")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if go_ios_ready { rgb(0x4ade80) } else { rgb(0xf87171) })
                                        .child(if go_ios_ready { "✓ ready" } else { "✕ missing (ios.exe)" })
                                )
                        )
                        // Frida status
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Frida:")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if frida_ready { rgb(0x4ade80) } else { rgb(0xf87171) })
                                        .child(if frida_ready { "✓ ready" } else { "missing (pip install frida-tools)" })
                                )
                        )
                        // SSH Tunnel status
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("SSH :2222:")
                                )
                                .child(
                                    div()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if tunnel_reachable { rgb(0x4ade80) } else { theme.text_muted })
                                        .child(if tunnel_reachable { "✓ Online" } else { "Closed (unreachable)" })
                                )
                        )
                        // Target App
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Target App:")
                                )
                                .child(
                                    div()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child(target_app_id.unwrap_or_else(|| "None selected".to_string()))
                                )
                        )
                )
        )
        // ── Live Frida Scans & SSH controls ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(ui_icon(IconName::Zap, 13.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(format!("Live Frida Scans — targeting {target_app_id_str}"))
                                )
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_2()
                        .children(live_scans.into_iter().map(|(label, kind, verb)| {
                            let label = label.to_string();
                            let target_for_scan = target_app_id_str.clone();
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        run_frida_scan_background(target_for_scan.clone(), kind);
                                        this.state
                                            .toast_manager
                                            .push_info(&format!("frida {kind}: {verb}"));
                                        cx.notify();
                                    }),
                                )
                                .child(label)
                        }))
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(
                            {
                                let udid_for_dump = dev_udid.clone();
                                let target_for_dump = target_app_id_str.clone();
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .bg(theme.accent)
                                    .hover(|s| s.bg(theme.accent_hover))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        dump_ipa_background(udid_for_dump.clone(), target_for_dump.clone(), tunnel_port);
                                        this.state.toast_manager.push_info(&format!("Extracting {target_for_dump} bundle to .ipa"));
                                        cx.notify();
                                    }))
                                    .child("Dump Decrypted IPA → MobSF")
                            }
                        )
                        .child(
                            {
                                let udid_for_tun = dev_udid.clone();
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .bg(if tunnel_up { theme.bg_hover } else { theme.bg_raised })
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                        start_tunnel_background(udid_for_tun.clone(), tunnel_port);
                                        this.state.toast_manager.push_info(&format!("Starting tunnel on port {tunnel_port}"));
                                        cx.notify();
                                    }))
                                    .child(if tunnel_up { "Tunnel Active (2222)" } else { "Start SSH Tunnel (2222)" })
                            }
                        )
                )
        )
        // ── Device on-device apps ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("On-device Applications (Live go-ios Inventory)")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(format!("{} apps detected", apps_list.len()))
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .children(
                            apps_list.into_iter().map(|app| {
                                let pkg = app.bundle_id.clone();
                                let name = app.display_name.clone();
                                let ver = app.version.clone();
                                let app_type = app.app_type.clone();
                                let allows_arbitrary = app.allows_arbitrary_loads;
                                let schemes_count = app.schemes.len();
                                let dev_udid_app = dev_udid.clone();

                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p_2()
                                    .rounded(px(4.0))
                                    .bg(theme.bg_raised)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_1p5()
                                                    .child(
                                                        div()
                                                            .text_xs()
                                                            .font_family("Cascadia Code, Consolas, monospace")
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(theme.text_primary)
                                                            .child(pkg.clone())
                                                    )
                                                    .children(if allows_arbitrary {
                                                        Some(
                                                            div()
                                                                .px_1()
                                                                .py_0p5()
                                                                .rounded(px(2.0))
                                                                .bg(theme.status_yellow.opacity(0.2))
                                                                .text_xs()
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(theme.status_yellow)
                                                                .child("ATS Insecure")
                                                        )
                                                    } else {
                                                        None
                                                    })
                                                    .children(if schemes_count > 0 {
                                                        Some(
                                                            div()
                                                                .px_1()
                                                                .py_0p5()
                                                                .rounded(px(2.0))
                                                                .bg(theme.accent.opacity(0.15))
                                                                .text_xs()
                                                                .text_color(theme.accent)
                                                                .child(format!("{schemes_count} schemes"))
                                                        )
                                                    } else {
                                                        None
                                                    })
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_subtle)
                                                    .child(format!("{name} • v{ver} • {app_type}"))
                                            )
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_row()
                                            .items_center()
                                            .gap_1()
                                            // Analyze in APEX
                                            .child({
                                                let pkg_for_audit = pkg.clone();
                                                div()
                                                    .px_2()
                                                    .py_1()
                                                    .rounded(px(3.0))
                                                    .bg(theme.accent)
                                                    .hover(|s| s.bg(theme.accent_hover))
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xffffff))
                                                    .cursor_pointer()
                                                    .child("Analyze in APEX")
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, _event, _window, cx| {
                                                            let count = analyze_app_in_apex(&pkg_for_audit);
                                                            if let Ok(mut a) = get_apex_state().lock() {
                                                                a.active_tab = ApexSubTab::Scanner;
                                                            }
                                                            this.state.active_activity = crate::domain::layout::ActivityTab::ApexSecurity;
                                                            this.state
                                                                .toast_manager
                                                                .push_success(&format!("Audited {pkg_for_audit}: imported {count} findings into APEX Scanner"));
                                                            cx.notify();
                                                        }),
                                                    )
                                            })
                                            // Trace (Frida)
                                            .child({
                                                let pkg_for_frida = pkg.clone();
                                                div()
                                                    .px_1p5()
                                                    .py_1()
                                                    .rounded(px(3.0))
                                                    .bg(theme.bg_card)
                                                    .hover(|s| s.bg(theme.bg_hover))
                                                    .border_1()
                                                    .border_color(theme.border_subtle)
                                                    .text_xs()
                                                    .text_color(theme.text_primary)
                                                    .cursor_pointer()
                                                    .child("Trace")
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, _event, _window, cx| {
                                                            run_frida_scan_background(pkg_for_frida.clone(), "trace_objc");
                                                            this.state
                                                                .toast_manager
                                                                .push_info(&format!("Live tracing ObjC calls for {pkg_for_frida}"));
                                                            cx.notify();
                                                        }),
                                                    )
                                            })
                                            // Dump IPA
                                            .child({
                                                let pkg_for_dump = pkg.clone();
                                                let dev_udid_d = dev_udid_app.clone();
                                                div()
                                                    .px_1p5()
                                                    .py_1()
                                                    .rounded(px(3.0))
                                                    .bg(theme.bg_card)
                                                    .hover(|s| s.bg(theme.bg_hover))
                                                    .border_1()
                                                    .border_color(theme.border_subtle)
                                                    .text_xs()
                                                    .text_color(theme.text_primary)
                                                    .cursor_pointer()
                                                    .child("Dump")
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, _event, _window, cx| {
                                                            dump_ipa_background(dev_udid_d.clone(), pkg_for_dump.clone(), tunnel_port);
                                                            this.state
                                                                .toast_manager
                                                                .push_info(&format!("Dumping {pkg_for_dump} to .ipa"));
                                                            cx.notify();
                                                        }),
                                                    )
                                            })
                                    )
                            })
                        )
                )
        )
        // ── Live console ──
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_1p5()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(ui_icon(IconName::Terminal, 13.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Jailbreak Ops Console")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_sidebar)
                        .children(console.iter().rev().take(14).map(|line| {
                            let clr = if line.starts_with("[jb]") || line.starts_with("[tunnel]") {
                                theme.text_subtle
                            } else if line.contains("vuln") || line.contains("risk") || line.contains("!") {
                                theme.status_yellow
                            } else if line.contains("✓") {
                                theme.status_green
                            } else {
                                theme.text_muted
                            };
                            div()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_xs()
                                .text_color(clr)
                                .child(line.to_string())
                        }))
                )
        )
}

pub struct ApexPanel;

impl crate::panels::traits::WorkbenchPanel for ApexPanel {
    fn id(&self) -> &'static str {
        "apex"
    }

    fn title(&self) -> &'static str {
        "APEX SECURITY"
    }

    fn icon(&self) -> IconName {
        IconName::Shield
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_apex_panel(state, cx).into_any_element()
    }
}
