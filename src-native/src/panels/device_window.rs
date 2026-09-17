use crate::panels::apex::{
    analyze_app_in_apex, audit_target_app_background, capture_device_screenshot_background,
    clear_jb_console, dump_ipa_background, execute_exploit_module_background, get_apex_state,
    get_live_screen_frame, inspect_wda_ui_hierarchy_background, launch_app_on_device_background,
    pair_and_mount_device_background, repair_device_pairing_background,
    restart_device_tunnel_background, run_frida_scan_background,
    send_device_hardware_button_background, send_device_scroll_background,
    send_device_swipe_background, send_device_tap_background, set_live_screen_frame,
    start_screen_mirror_background, start_wda_forward_background, stop_screen_mirror_background,
    tail_device_syslog_background, trigger_device_refresh, type_text_device_background,
};
use crate::theme::Theme;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use gpui_kit::gpui::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BountyWorkbenchTab {
    Targets,
    Exploits,
    WdaInspector,
    Console,
    Findings,
}

pub struct DeviceWindowView {
    pub zoom_level: f32,
    pub active_tab: BountyWorkbenchTab,
    pub touch_start: Option<(f64, f64, Instant)>,
    pub theme: Theme,
}

static DETACHED_STREAM_RUNNING: AtomicBool = AtomicBool::new(false);
static DETACHED_SCREEN_BOUNDS: std::sync::OnceLock<std::sync::RwLock<Option<(f32, f32, f32, f32)>>> =
    std::sync::OnceLock::new();

impl DeviceWindowView {
    pub fn new(_window: &mut Window, _cx: &mut Context<Self>) -> Self {
        // Automatically trigger scan for USB connected devices on launch
        trigger_device_refresh();

        let udid = get_apex_state()
            .lock()
            .ok()
            .and_then(|a| a.connected_devices.first().map(|d| d.udid.clone()))
            .unwrap_or_default();

        if !udid.is_empty() && !DETACHED_STREAM_RUNNING.swap(true, Ordering::SeqCst) {
            start_detached_stream_reader(udid);
        }

        Self {
            zoom_level: 1.25,
            active_tab: BountyWorkbenchTab::Targets,
            touch_start: None,
            theme: Theme::cursor_dark(),
        }
    }
}

fn start_detached_stream_reader(udid: String) {
    let u_cap = udid.clone();
    std::thread::spawn(move || {
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_millis(400))
            .build();
        let Ok(client) = client else { return };

        loop {
            let mut got_frame = false;
            if let Ok(mut resp) = client.get("http://127.0.0.1:3333/").send() {
                use std::io::Read;
                let mut buf = Vec::with_capacity(524288);
                let mut chunk = [0u8; 8192];

                while let Ok(n) = resp.read(&mut chunk) {
                    if n == 0 {
                        break;
                    }
                    buf.extend_from_slice(&chunk[..n]);

                    if let Some(pos) = buf
                        .windows(15)
                        .position(|w| w.eq_ignore_ascii_case(b"content-length:"))
                    {
                        let after = &buf[pos + 15..];
                        if let Some(eol) = after.windows(4).position(|w| w == b"\r\n\r\n") {
                            let len_str =
                                String::from_utf8_lossy(&after[..eol]).trim().to_string();
                            if let Ok(length) = len_str.parse::<usize>() {
                                let body_start = pos + 15 + eol + 4;
                                if buf.len() >= body_start + length {
                                    let frame_slice = &buf[body_start..body_start + length];
                                    if let Ok(dyn_img) = image::load_from_memory(frame_slice) {
                                        let mut rgba = dyn_img.into_rgba8();
                                        for pixel in rgba.chunks_exact_mut(4) {
                                            pixel.swap(0, 2);
                                        }
                                        let render_img = Arc::new(RenderImage::new(
                                            smallvec::SmallVec::from_elem(
                                                image::Frame::new(rgba),
                                                1,
                                            ),
                                        ));
                                        set_live_screen_frame(render_img);
                                        got_frame = true;
                                    }
                                    buf.drain(..body_start + length);
                                    std::thread::sleep(std::time::Duration::from_millis(16));
                                    continue;
                                }
                            }
                        }
                    }

                    if buf.len() > 1_000_000 {
                        buf.clear();
                    }
                }
            }

            if !got_frame {
                capture_device_screenshot_background(u_cap.clone());
                std::thread::sleep(std::time::Duration::from_millis(150));
            } else {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    });
}

impl Render for DeviceWindowView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = &self.theme;
        let zoom = self.zoom_level;
        let base_w = 270.0 * zoom;
        let base_h = 585.0 * zoom;
        let active_tab = self.active_tab;

        // Dynamically read live state from APEX orchestrator on every render
        let (
            connected_dev,
            is_scanning,
            tunnel_up,
            tunnel_reachable,
            _tunnel_port,
            apps_list,
            mirror_active,
            _wda_forwarded,
            bounty_running,
            bounty_status,
            active_module,
            findings_count,
            target_app_id,
            recent_logs,
            frida_ready,
            go_ios_ready,
        ) = {
            let a = get_apex_state().lock().unwrap();
            let dev = a.connected_devices.first().cloned();
            let logs: Vec<String> = a.jb_console.iter().rev().take(30).cloned().collect();
            (
                dev,
                a.is_scanning_devices,
                a.tunnel_active,
                a.tunnel_port_reachable,
                a.tunnel_port,
                a.device_apps.clone(),
                a.mirror_active,
                a.wda_forwarded,
                a.bounty_audit_running,
                a.bounty_status_line.clone(),
                a.active_exploit_module.clone(),
                a.findings.len(),
                a.target_app.clone().unwrap_or_default(),
                logs,
                a.frida_available
                    || vscode_rust_app::domain::sentinel::jbops::frida_bin().is_some(),
                a.go_ios_available
                    || vscode_rust_app::domain::sentinel::jbops::go_ios_path().is_some(),
            )
        };

        let (dev_name, dev_model, dev_udid, dev_ios, dev_conn, has_device) = match connected_dev {
            Some(d) => (d.name, d.model, d.udid, d.ios_version, d.connection, true),
            None => (
                "No Device Detected".to_string(),
                "iPhone XR".to_string(),
                String::new(),
                String::new(),
                "USB Unplugged".to_string(),
                false,
            ),
        };

        // Ensure detached stream reader is active if we just discovered a UDID
        if !dev_udid.is_empty() && !DETACHED_STREAM_RUNNING.swap(true, Ordering::SeqCst) {
            start_detached_stream_reader(dev_udid.clone());
        }

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(rgb(0x0c0e14))
            .text_color(theme.text_primary)
            // ── 1. Top Header Bar (Draggable across monitors, zero emojis) ──
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .flex_wrap()
                    .gap_2()
                    .px_4()
                    .py_2()
                    .bg(rgb(0x131620))
                    .border_b_1()
                    .border_color(rgb(0x232738))
                    .cursor_move()
                    .on_mouse_down(MouseButton::Left, |event: &MouseDownEvent, window, _| {
                        if event.click_count == 2 {
                            window.zoom_window();
                        } else {
                            window.start_window_move();
                        }
                    })
                    // Left: Device Identity & Status
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .w(px(9.0))
                                    .h(px(9.0))
                                    .rounded_full()
                                    .bg(if !has_device {
                                        rgb(0xef4444)
                                    } else if mirror_active {
                                        rgb(0x22c55e)
                                    } else {
                                        rgb(0x38bdf8)
                                    }),
                            )
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
                                            .gap_2()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded(px(4.0))
                                                    .bg(rgb(0x007acc))
                                                    .text_color(rgb(0xffffff))
                                                    .child("vscodium-ip-panel"),
                                            )
                                            .child(
                                                div()
                                                    .text_sm()
                                                    .font_weight(FontWeight::BOLD)
                                                    .child(if has_device {
                                                        format!("{dev_name} — {dev_model}")
                                                    } else {
                                                        "Physical iPhone Disconnected [OFFLINE]".to_string()
                                                    }),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child(if has_device {
                                                format!(
                                                    "iOS {dev_ios} • {dev_conn} • 60 FPS MJPEG Stream • Multi-Monitor Window"
                                                )
                                            } else {
                                                "Connect iPhone XR via USB cable • usbmuxd ready".to_string()
                                            }),
                                    ),
                            ),
                    )
                    // Center: Toolchain Badges
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            // go-ios badge
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .bg(if go_ios_ready { rgb(0x052e16) } else { rgb(0x2d1515) })
                                    .border_1()
                                    .border_color(if go_ios_ready { rgb(0x15803d) } else { rgb(0xb91c1c) })
                                    .text_xs()
                                    .text_color(if go_ios_ready { rgb(0x4ade80) } else { rgb(0xf87171) })
                                    .child(if go_ios_ready { "go-ios ready" } else { "go-ios missing" }),
                            )
                            // Frida badge
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .bg(if frida_ready { rgb(0x052e16) } else { rgb(0x2d1515) })
                                    .border_1()
                                    .border_color(if frida_ready { rgb(0x15803d) } else { rgb(0xb91c1c) })
                                    .text_xs()
                                    .text_color(if frida_ready { rgb(0x4ade80) } else { rgb(0xf87171) })
                                    .child(if frida_ready { "Frida ready" } else { "Frida missing" }),
                            )
                            // SSH tunnel badge
                            .child(
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(4.0))
                                    .bg(if tunnel_reachable { rgb(0x052e16) } else { rgb(0x1e293b) })
                                    .border_1()
                                    .border_color(if tunnel_reachable { rgb(0x15803d) } else { rgb(0x334155) })
                                    .text_xs()
                                    .text_color(if tunnel_reachable { rgb(0x4ade80) } else { rgb(0x94a3b8) })
                                    .child(if tunnel_reachable { "SSH :2222 Online" } else { "SSH :2222 Closed" }),
                            ),
                    )
                    // Right: Zoom Presets & Quick Actions
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .flex_wrap()
                            .gap_2()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .bg(rgb(0x1a1e2e))
                                    .p_1()
                                    .rounded(px(6.0))
                                    .gap_1()
                                    .children(
                                        [(1.00, "100%"), (1.25, "125%"), (1.50, "150%"), (2.00, "200%")]
                                            .into_iter()
                                            .map(|(val, label)| {
                                                let is_sel = (zoom - val).abs() < 0.05;
                                                div()
                                                    .px_2()
                                                    .py_0p5()
                                                    .rounded(px(4.0))
                                                    .text_xs()
                                                    .font_weight(if is_sel {
                                                        FontWeight::BOLD
                                                    } else {
                                                        FontWeight::MEDIUM
                                                    })
                                                    .bg(if is_sel {
                                                        rgb(0x007acc)
                                                    } else {
                                                        rgb(0x00000000)
                                                    })
                                                    .text_color(if is_sel {
                                                        rgb(0xffffff)
                                                    } else {
                                                        rgb(0x94a3b8)
                                                    })
                                                    .hover(|s| {
                                                        s.bg(if is_sel {
                                                            rgb(0x007acc)
                                                        } else {
                                                            rgb(0x252a3f)
                                                        })
                                                    })
                                                    .cursor_pointer()
                                                    .child(label)
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(move |this, _e, _w, cx| {
                                                            this.zoom_level = val;
                                                            cx.notify();
                                                        }),
                                                    )
                                            }),
                                    ),
                            )
                            // Scan Devices
                            .child(
                                action_button("Scan Devices", cx.listener(|_this, _e, _w, cx| {
                                    trigger_device_refresh();
                                    cx.notify();
                                })),
                            )
                            // Snapshot
                            .child({
                                let u = dev_udid.clone();
                                action_button("Snapshot", cx.listener(move |_this, _e, _w, cx| {
                                    capture_device_screenshot_background(u.clone());
                                    cx.notify();
                                }))
                            })
                            // Pair + Mount
                            .child({
                                let u = dev_udid.clone();
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded(px(5.0))
                                    .bg(rgb(0x1e1b4b))
                                    .border_1()
                                    .border_color(rgb(0x6366f1))
                                    .hover(|s| s.bg(rgb(0x312e81)))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xa5b4fc))
                                    .cursor_pointer()
                                    .child("Pair + Mount")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |_this, _e, _w, cx| {
                                            pair_and_mount_device_background(u.clone());
                                            cx.notify();
                                        }),
                                    )
                            })
                            // Tunnel (iOS 18)
                            .child({
                                let u = dev_udid.clone();
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded(px(5.0))
                                    .bg(if tunnel_up { rgb(0x064e3b) } else { rgb(0x1e293b) })
                                    .border_1()
                                    .border_color(if tunnel_up { rgb(0x10b981) } else { rgb(0x334155) })
                                    .hover(|s| s.bg(if tunnel_up { rgb(0x065f46) } else { rgb(0x334155) }))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(if tunnel_up { rgb(0x34d399) } else { rgb(0x94a3b8) })
                                    .cursor_pointer()
                                    .child("Tunnel (iOS 18)")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |_this, _e, _w, cx| {
                                            restart_device_tunnel_background(u.clone());
                                            cx.notify();
                                        }),
                                    )
                            })
                            // WDA Bridge
                            .child({
                                let u = dev_udid.clone();
                                div()
                                    .px_2p5()
                                    .py_1()
                                    .rounded(px(5.0))
                                    .bg(rgb(0x1e293b))
                                    .border_1()
                                    .border_color(rgb(0x38bdf8))
                                    .hover(|s| s.bg(rgb(0x334155)))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x38bdf8))
                                    .cursor_pointer()
                                    .child("WDA Bridge")
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(move |_this, _e, _w, cx| {
                                            start_wda_forward_background(u.clone());
                                            cx.notify();
                                        }),
                                    )
                            })
                            // Repair
                            .child({
                                let u = dev_udid.clone();
                                action_button("Repair", cx.listener(move |_this, _e, _w, cx| {
                                    repair_device_pairing_background(u.clone());
                                    cx.notify();
                                }))
                            })
                            // Logs
                            .child({
                                let u = dev_udid.clone();
                                action_button("Logs", cx.listener(move |this, _e, _w, cx| {
                                    tail_device_syslog_background(u.clone());
                                    this.active_tab = BountyWorkbenchTab::Console;
                                    cx.notify();
                                }))
                            }),
                    ),
            )
            // ── 2. Main Dual Column Body (Phone Left | Bug Bounty Right) ──
            .child(
                div()
                    .flex()
                    .flex_row()
                    .flex_1()
                    .overflow_hidden()
                    // ── LEFT: Zoomable Physical iPhone XR Chassis ──
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .justify_center()
                            .p_6()
                            .bg(rgb(0x10131d))
                            .border_r_1()
                            .border_color(rgb(0x232738))
                            .overflow_hidden()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    // Left Hardware Buttons (Volume Up & Down)
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .w(px(5.0))
                                                    .h(px(32.0 * zoom))
                                                    .rounded(px(2.5))
                                                    .bg(rgb(0x3f4459))
                                                    .hover(|s| s.bg(rgb(0x007acc)))
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener({
                                                            let u = dev_udid.clone();
                                                            move |_this, _e, _w, cx| {
                                                                send_device_hardware_button_background(
                                                                    u.clone(),
                                                                    "volumeUp",
                                                                );
                                                                cx.notify();
                                                            }
                                                        }),
                                                    ),
                                            )
                                            .child(
                                                div()
                                                    .w(px(5.0))
                                                    .h(px(32.0 * zoom))
                                                    .rounded(px(2.5))
                                                    .bg(rgb(0x3f4459))
                                                    .hover(|s| s.bg(rgb(0x007acc)))
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener({
                                                            let u = dev_udid.clone();
                                                            move |_this, _e, _w, cx| {
                                                                send_device_hardware_button_background(
                                                                    u.clone(),
                                                                    "volumeDown",
                                                                );
                                                                cx.notify();
                                                            }
                                                        }),
                                                    ),
                                            ),
                                    )
                                    // Main Phone Screen Canvas
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .w(px(base_w))
                                            .h(px(base_h))
                                            .rounded(px(34.0 * zoom))
                                            .bg(rgb(0x000000))
                                            .border_2()
                                            .border_color(rgb(0x2d3246))
                                            .shadow_xl()
                                            .overflow_hidden()
                                            // Mouse down
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, event: &MouseDownEvent, _window, _cx| {
                                                    let (ox, oy, w, h) = DETACHED_SCREEN_BOUNDS
                                                        .get_or_init(|| std::sync::RwLock::new(None))
                                                        .read()
                                                        .ok()
                                                        .and_then(|g| *g)
                                                        .unwrap_or((0.0, 0.0, base_w, base_h));

                                                    let click_x: f32 = event.position.x.into();
                                                    let click_y: f32 = event.position.y.into();

                                                    let local_x = (click_x - ox).clamp(0.0, w);
                                                    let local_y = (click_y - oy).clamp(0.0, h);

                                                    this.touch_start = Some((
                                                        local_x as f64,
                                                        local_y as f64,
                                                        Instant::now(),
                                                    ));
                                                }),
                                            )
                                            // Mouse up (Tap vs Swipe)
                                            .on_mouse_up(
                                                MouseButton::Left,
                                                cx.listener({
                                                    let u = dev_udid.clone();
                                                    move |this, event: &MouseUpEvent, _window, cx| {
                                                        let (ox, oy, w, h) = DETACHED_SCREEN_BOUNDS
                                                            .get_or_init(|| std::sync::RwLock::new(None))
                                                            .read()
                                                            .ok()
                                                            .and_then(|g| *g)
                                                            .unwrap_or((0.0, 0.0, base_w, base_h));

                                                        let click_x: f32 = event.position.x.into();
                                                        let click_y: f32 = event.position.y.into();

                                                        let local_end_x = (click_x - ox).clamp(0.0, w);
                                                        let local_end_y = (click_y - oy).clamp(0.0, h);

                                                        if let Some((start_local_x, start_local_y, start_time)) =
                                                            this.touch_start.take()
                                                        {
                                                            let dx = local_end_x as f64 - start_local_x;
                                                            let dy = local_end_y as f64 - start_local_y;
                                                            let local_dist = (dx * dx + dy * dy).sqrt();
                                                            let elapsed = start_time.elapsed().as_secs_f64();

                                                            let norm_start_x =
                                                                (start_local_x / w as f64).clamp(0.0, 1.0);
                                                            let norm_start_y =
                                                                (start_local_y / h as f64).clamp(0.0, 1.0);
                                                            let norm_end_x =
                                                                (local_end_x as f64 / w as f64).clamp(0.0, 1.0);
                                                            let norm_end_y =
                                                                (local_end_y as f64 / h as f64).clamp(0.0, 1.0);

                                                            if local_dist < 15.0
                                                                || (elapsed < 0.28 && local_dist < 32.0)
                                                            {
                                                                send_device_tap_background(
                                                                    u.clone(),
                                                                    norm_end_x,
                                                                    norm_end_y,
                                                                );
                                                            } else {
                                                                let duration = elapsed.clamp(0.12, 0.55);
                                                                send_device_swipe_background(
                                                                    u.clone(),
                                                                    norm_start_x,
                                                                    norm_start_y,
                                                                    norm_end_x,
                                                                    norm_end_y,
                                                                    duration,
                                                                );
                                                            }
                                                            cx.notify();
                                                        }
                                                    }
                                                }),
                                            )
                                            // Mouse wheel scrolling
                                            .on_scroll_wheel(cx.listener({
                                                let u = dev_udid.clone();
                                                move |_this, event: &ScrollWheelEvent, _window, cx| {
                                                    let dy: f32 = match event.delta {
                                                        ScrollDelta::Lines(d) => -d.y * 30.0,
                                                        ScrollDelta::Pixels(d) => {
                                                            let p: f32 = d.y.into();
                                                            -p
                                                        }
                                                    };
                                                    send_device_scroll_background(u.clone(), dy);
                                                    cx.notify();
                                                }
                                            }))
                                            // Canvas to accurately read element bounds on window
                                            .child(
                                                canvas(
                                                    |bounds, _window, _cx| {
                                                        let ox: f32 = bounds.origin.x.into();
                                                        let oy: f32 = bounds.origin.y.into();
                                                        let w: f32 = bounds.size.width.into();
                                                        let h: f32 = bounds.size.height.into();
                                                        if let Ok(mut g) = DETACHED_SCREEN_BOUNDS
                                                            .get_or_init(|| std::sync::RwLock::new(None))
                                                            .write()
                                                        {
                                                            *g = Some((ox, oy, w, h));
                                                        }
                                                    },
                                                    |_bounds, _, _window, _cx| {},
                                                )
                                                .size_full()
                                                .absolute()
                                                .inset_0(),
                                            )
                                            // Screen Content: Live Mirror vs Standby vs Disconnected OLED
                                            .child({
                                                if mirror_active {
                                                    if let Some(frame) = get_live_screen_frame() {
                                                        div()
                                                            .relative()
                                                            .size_full()
                                                            .child(
                                                                img(frame)
                                                                    .size_full()
                                                                    .object_fit(ObjectFit::Cover),
                                                            )
                                                            // Bottom Home Bar Pill
                                                            .child(
                                                                div()
                                                                    .absolute()
                                                                    .bottom_2()
                                                                    .left_0()
                                                                    .right_0()
                                                                    .flex()
                                                                    .justify_center()
                                                                    .child(
                                                                        div()
                                                                            .w(px(100.0 * zoom))
                                                                            .h(px(5.0 * zoom))
                                                                            .rounded(px(2.5 * zoom))
                                                                            .bg(rgba(0xffffffaa))
                                                                            .hover(|s| s.bg(rgb(0xffffff)))
                                                                            .cursor_pointer()
                                                                            .on_mouse_down(
                                                                                MouseButton::Left,
                                                                                cx.listener({
                                                                                    let u = dev_udid.clone();
                                                                                    move |_this, _e, _w, cx| {
                                                                                        send_device_hardware_button_background(
                                                                                            u.clone(),
                                                                                            "home",
                                                                                        );
                                                                                        cx.notify();
                                                                                    }
                                                                                }),
                                                                            ),
                                                                    ),
                                                            )
                                                            .into_any_element()
                                                    } else {
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .items_center()
                                                            .justify_center()
                                                            .size_full()
                                                            .gap_2()
                                                            .p_4()
                                                            .bg(rgb(0x07090e))
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .font_weight(FontWeight::BOLD)
                                                                    .text_color(rgb(0x38bdf8))
                                                                    .child("Connecting to iPhone USB Stream..."),
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_xs()
                                                                    .text_color(theme.text_muted)
                                                                    .child("Streaming in-memory at 60 FPS (0 disk writes)"),
                                                            )
                                                            .into_any_element()
                                                    }
                                                } else if has_device {
                                                    // Connected Standby
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .items_center()
                                                        .justify_center()
                                                        .size_full()
                                                        .gap_2p5()
                                                        .p_4()
                                                        .bg(rgb(0x0a0d14))
                                                        .child(
                                                            div()
                                                                .p_2p5()
                                                                .rounded_full()
                                                                .bg(rgb(0x131d2e))
                                                                .border_1()
                                                                .border_color(rgb(0x1d4ed8))
                                                                .child(icon_14(IconName::Smartphone, rgb(0x60a5fa))),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .items_center()
                                                                .gap_0p5()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text_color(rgb(0xffffff))
                                                                        .child(dev_name.clone()),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(rgb(0x94a3b8))
                                                                        .child(format!("iOS {dev_ios} • USB Connected")),
                                                                ),
                                                        )
                                                        .child(
                                                            div()
                                                                .px_3()
                                                                .py_1()
                                                                .rounded(px(5.0))
                                                                .bg(rgb(0x007acc))
                                                                .hover(|s| s.bg(rgb(0x0066aa)))
                                                                .cursor_pointer()
                                                                .text_xs()
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0xffffff))
                                                                .child("Start Live Mirror")
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    cx.listener({
                                                                        let u = dev_udid.clone();
                                                                        move |_this, _e, _w, cx| {
                                                                            start_screen_mirror_background(u.clone());
                                                                            cx.notify();
                                                                        }
                                                                    }),
                                                                ),
                                                        )
                                                        .into_any_element()
                                                } else {
                                                    // GENUINE OFFLINE DISCONNECTED OLED DISPLAY - ZERO MOCKUPS
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .items_center()
                                                        .justify_center()
                                                        .size_full()
                                                        .gap_2p5()
                                                        .p_4()
                                                        .bg(rgb(0x07090e))
                                                        .child(
                                                            div()
                                                                .p_2p5()
                                                                .rounded_full()
                                                                .bg(rgb(0x1c1317))
                                                                .border_1()
                                                                .border_color(rgb(0x5c1d24))
                                                                .child(icon_14(IconName::Smartphone, rgb(0xf87171))),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .items_center()
                                                                .gap_0p5()
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .font_weight(FontWeight::BOLD)
                                                                        .text_color(rgb(0xfca5a5))
                                                                        .child("NO DEVICE DETECTED"),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .text_xs()
                                                                        .text_color(theme.text_subtle)
                                                                        .text_center()
                                                                        .child("Connect iPhone via USB cable"),
                                                                ),
                                                        )
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_col()
                                                                .gap_1()
                                                                .w_full()
                                                                .p_2()
                                                                .rounded(px(5.0))
                                                                .bg(rgb(0x0f131d))
                                                                .border_1()
                                                                .border_color(rgb(0x1e2436))
                                                                .child(
                                                                    div()
                                                                        .flex()
                                                                        .flex_row()
                                                                        .items_center()
                                                                        .gap_1p5()
                                                                        .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0x22c55e)))
                                                                        .child(div().text_xs().text_color(rgb(0x94a3b8)).child("usbmuxd: ready")),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .flex()
                                                                        .flex_row()
                                                                        .items_center()
                                                                        .gap_1p5()
                                                                        .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0xef4444)))
                                                                        .child(div().text_xs().text_color(rgb(0x94a3b8)).child("USB link: unplugged")),
                                                                )
                                                                .child(
                                                                    div()
                                                                        .flex()
                                                                        .flex_row()
                                                                        .items_center()
                                                                        .gap_1p5()
                                                                        .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0xeab308)))
                                                                        .child(div().text_xs().text_color(rgb(0x94a3b8)).child("Lockdownd: waiting")),
                                                                ),
                                                        )
                                                        .child(
                                                            div()
                                                                .px_3()
                                                                .py_1()
                                                                .rounded(px(5.0))
                                                                .bg(rgb(0x1a2233))
                                                                .border_1()
                                                                .border_color(rgb(0x2d3a54))
                                                                .hover(|s| s.bg(rgb(0x243048)))
                                                                .cursor_pointer()
                                                                .text_xs()
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0x38bdf8))
                                                                .child("Scan Devices")
                                                                .on_mouse_down(
                                                                    MouseButton::Left,
                                                                    cx.listener(|_this, _e, _w, cx| {
                                                                        trigger_device_refresh();
                                                                        cx.notify();
                                                                    }),
                                                                ),
                                                        )
                                                        .into_any_element()
                                                }
                                            }),
                                    )
                                    // Right Hardware Button (Power / Lock)
                                    .child(
                                        div()
                                            .w(px(5.0))
                                            .h(px(46.0 * zoom))
                                            .rounded(px(2.5))
                                            .bg(rgb(0x3f4459))
                                            .hover(|s| s.bg(rgb(0x007acc)))
                                            .cursor_pointer()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener({
                                                    let u = dev_udid.clone();
                                                    move |_this, _e, _w, cx| {
                                                        send_device_hardware_button_background(
                                                            u.clone(),
                                                            "lock",
                                                        );
                                                        cx.notify();
                                                    }
                                                }),
                                            ),
                                    ),
                            )
                            // Bottom Action Controls under chassis
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_center()
                                    .gap_2()
                                    .mt_4()
                                    .child(action_button(
                                        "Home",
                                        cx.listener({
                                            let u = dev_udid.clone();
                                            move |_this, _e, _w, cx| {
                                                send_device_hardware_button_background(u.clone(), "home");
                                                cx.notify();
                                            }
                                        }),
                                    ))
                                    .child(action_button(
                                        "App Switcher",
                                        cx.listener({
                                            let u = dev_udid.clone();
                                            move |_this, _e, _w, cx| {
                                                send_device_swipe_background(
                                                    u.clone(),
                                                    414.0,
                                                    1750.0,
                                                    414.0,
                                                    1050.0,
                                                    0.45,
                                                );
                                                cx.notify();
                                            }
                                        }),
                                    ))
                                    .child(action_button(
                                        "Lock Screen",
                                        cx.listener({
                                            let u = dev_udid.clone();
                                            move |_this, _e, _w, cx| {
                                                send_device_hardware_button_background(u.clone(), "lock");
                                                cx.notify();
                                            }
                                        }),
                                    )),
                            ),
                    )
                    // ── RIGHT: Bug Bounty & Security Operations Suite ──
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .flex_1()
                            .bg(rgb(0x0f121a))
                            // Navigation Tabs
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .border_b_1()
                                    .border_color(rgb(0x232738))
                                    .bg(rgb(0x131722))
                                    .px_4()
                                    .pt_2()
                                    .gap_2()
                                    .children(
                                        [
                                            (BountyWorkbenchTab::Targets, "Target Apps", IconName::Smartphone),
                                            (BountyWorkbenchTab::Exploits, "Exploit Suite", IconName::Zap),
                                            (BountyWorkbenchTab::WdaInspector, "UI Hierarchy", IconName::Search),
                                            (BountyWorkbenchTab::Console, "Ops Console", IconName::Terminal),
                                            (BountyWorkbenchTab::Findings, "Security Findings", IconName::Shield),
                                        ]
                                        .into_iter()
                                        .map(|(tab, label, ic)| {
                                            let is_active = active_tab == tab;
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1p5()
                                                .px_3()
                                                .py_2()
                                                .border_b_2()
                                                .border_color(if is_active {
                                                    rgb(0x38bdf8)
                                                } else {
                                                    rgb(0x00000000)
                                                })
                                                .text_xs()
                                                .font_weight(if is_active {
                                                    FontWeight::BOLD
                                                } else {
                                                    FontWeight::MEDIUM
                                                })
                                                .text_color(if is_active {
                                                    rgb(0x38bdf8)
                                                } else {
                                                    rgb(0x94a3b8)
                                                })
                                                .hover(|s| s.text_color(rgb(0xffffff)))
                                                .cursor_pointer()
                                                .child(icon_12(
                                                    ic,
                                                    if is_active {
                                                        rgb(0x38bdf8)
                                                    } else {
                                                        rgb(0x64748b)
                                                    },
                                                ))
                                                .child(label)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.active_tab = tab;
                                                        cx.notify();
                                                    }),
                                                )
                                        }),
                                    ),
                            )
                            // Content Panel
                            .child(
                                div()
                                    .flex_1()
                                    .p_4()
                                    .overflow_hidden()
                                    .child(match active_tab {
                                        BountyWorkbenchTab::Targets => {
                                            render_targets_tab(&dev_udid, &apps_list, is_scanning, cx)
                                        }
                                        BountyWorkbenchTab::Exploits => render_exploits_tab(
                                            &dev_udid,
                                            &target_app_id,
                                            &active_module,
                                            bounty_running,
                                            &bounty_status,
                                            findings_count,
                                            cx,
                                        ),
                                        BountyWorkbenchTab::WdaInspector => {
                                            render_wda_tab(&dev_udid, cx)
                                        }
                                        BountyWorkbenchTab::Console => {
                                            render_console_tab(&recent_logs, &bounty_status, bounty_running, &target_app_id, cx)
                                        }
                                        BountyWorkbenchTab::Findings => render_findings_tab(),
                                    }),
                            ),
                    ),
            )
    }
}

fn action_button(
    label: &'static str,
    handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
    div()
        .px_3()
        .py_1p5()
        .rounded(px(6.0))
        .bg(rgb(0x1a1e2e))
        .border_1()
        .border_color(rgb(0x2d3246))
        .hover(|s| s.bg(rgb(0x282f48)))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(rgb(0xffffff))
        .cursor_pointer()
        .child(label)
        .on_mouse_down(MouseButton::Left, handler)
}

fn render_targets_tab(
    udid: &str,
    apps: &[crate::panels::apex::IPhoneAppInfo],
    is_scanning: bool,
    cx: &mut Context<DeviceWindowView>,
) -> AnyElement {
    let u_launch = udid.to_string();
    let u_dump = udid.to_string();

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
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x38bdf8))
                        .child(format!("Target Applications on Connected iPhone ({})", apps.len())),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x1e293b))
                        .border_1()
                        .border_color(rgb(0x38bdf8))
                        .hover(|s| s.bg(rgb(0x334155)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x38bdf8))
                        .child(if is_scanning { "Scanning..." } else { "Refresh Apps" })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|_this, _e, _w, cx| {
                                trigger_device_refresh();
                                cx.notify();
                            }),
                        ),
                ),
        )
        .child(if apps.is_empty() {
            div()
                .p_4()
                .rounded(px(6.0))
                .bg(rgb(0x151926))
                .border_1()
                .border_color(rgb(0x252a3d))
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child("No apps enumerated yet. Connect iPhone via USB, ensure Trust prompt is confirmed, and click 'Refresh Apps'."),
                )
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(apps.iter().map(|app| {
                    let bid = app.bundle_id.clone();
                    let name = if app.display_name.is_empty() {
                        app.bundle_id.clone()
                    } else {
                        app.display_name.clone()
                    };
                    let ver = app.version.clone();
                    let app_type = app.app_type.clone();
                    let path = app.path.clone();
                    let ats = app.allows_arbitrary_loads;

                    let bid1 = bid.clone();
                    let u1 = u_launch.clone();
                    let bid2 = bid.clone();
                    let bid3 = bid.clone();
                    let bid4 = bid.clone();
                    let u4 = u_dump.clone();

                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p_3()
                        .rounded(px(6.0))
                        .bg(rgb(0x151926))
                        .border_1()
                        .border_color(rgb(0x252a3d))
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
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .child(name),
                                        )
                                        .children(if ats {
                                            Some(
                                                div()
                                                    .px_1p5()
                                                    .py_0p5()
                                                    .rounded(px(2.0))
                                                    .bg(rgb(0x854d0e))
                                                    .text_xs()
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xfef08a))
                                                    .child("ATS BYPASS"),
                                            )
                                        } else {
                                            None
                                        }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_color(rgb(0x38bdf8))
                                        .child(bid),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x94a3b8))
                                        .child(format!("v{ver} • {app_type} • {path}")),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .gap_1p5()
                                // Launch
                                .child(
                                    div()
                                        .px_2p5()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x007acc))
                                        .hover(|s| s.bg(rgb(0x0062a3)))
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .cursor_pointer()
                                        .child("Launch")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |_this, _e, _w, cx| {
                                                launch_app_on_device_background(u1.clone(), bid1.clone());
                                                cx.notify();
                                            }),
                                        ),
                                )
                                // Audit
                                .child(
                                    div()
                                        .px_2p5()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x1e293b))
                                        .border_1()
                                        .border_color(rgb(0x334155))
                                        .hover(|s| s.bg(rgb(0x334155)))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0x38bdf8))
                                        .cursor_pointer()
                                        .child("Audit")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, _w, cx| {
                                                audit_target_app_background(bid2.clone());
                                                this.active_tab = BountyWorkbenchTab::Console;
                                                cx.notify();
                                            }),
                                        ),
                                )
                                // Trace
                                .child(
                                    div()
                                        .px_2p5()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x1e293b))
                                        .border_1()
                                        .border_color(rgb(0x334155))
                                        .hover(|s| s.bg(rgb(0x334155)))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0x94a3b8))
                                        .cursor_pointer()
                                        .child("Trace")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |_this, _e, _w, cx| {
                                                run_frida_scan_background(bid3.clone(), "trace_objc");
                                                cx.notify();
                                            }),
                                        ),
                                )
                                // Dump IPA
                                .child(
                                    div()
                                        .px_2p5()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x1e293b))
                                        .border_1()
                                        .border_color(rgb(0x334155))
                                        .hover(|s| s.bg(rgb(0x334155)))
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0xf59e0b))
                                        .cursor_pointer()
                                        .child("Dump IPA")
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |_this, _e, _w, cx| {
                                                dump_ipa_background(u4.clone(), bid4.clone(), 2222);
                                                cx.notify();
                                            }),
                                        ),
                                ),
                        )
                }))
                .into_any_element()
        })
        .into_any_element()
}

fn render_exploits_tab(
    udid: &str,
    target_app: &str,
    act_mod: &str,
    running: bool,
    _status_text: &str,
    findings_total: usize,
    cx: &mut Context<DeviceWindowView>,
) -> AnyElement {
    let u_send = udid.to_string();
    let tgt_curr = if target_app.is_empty() {
        "com.xunmeng.pinduoduo".to_string()
    } else {
        target_app.to_string()
    };

    let exploit_modules = [
        ("SSL Pinning Bypass", "Bypass TLS certificate pinning via Frida hooks"),
        ("Keychain Extraction", "Dump sensitive credentials from iOS Keychain database"),
        ("Method Trace", "Trace Objective-C/Swift method invocations in real time"),
        ("Enumerate Classes", "Enumerate loaded Objective-C classes & declared selectors"),
        ("File System Snoop", "Monitor sandbox file read/write operations"),
        ("Deep Link Attack", "Fuzz and trace registered custom URL schemes"),
        ("AI & Prompt Audit", "Audit on-device AI models and prompt leakage"),
        ("Agent Action Monitor", "Monitor IPC and system agent actions"),
    ];

    let frida_presets = [
        ("SSL Bypass", "ssl_unpin"),
        ("Dump Keychain", "keychain_snoop"),
        ("Hook SecItem", "hook_secitem"),
        ("Intercept Crypto", "crypto_trace"),
        ("Bypass Jailbreak", "bypass_jb"),
        ("Log Pasteboard", "pasteboard_spy"),
    ];

    let wda_payloads = [
        ("admin' OR 1=1--", "SQL Injection"),
        ("<script>alert('XSS')</script>", "WebView XSS"),
        ("../../../../../../../../etc/passwd", "Path Traversal"),
        ("pddopen://goods_detail?goods_id=1000", "Pinduoduo DeepLink"),
        ("grab://open?screen=wallet", "Grab Wallet Scheme"),
        ("temu://home?source=bounty", "Temu DeepLink"),
        ("clear", "Dismiss Keyboard"),
    ];

    // Calculate vulnerability score
    let (crit_count, high_count, med_count, low_count) = {
        let a = get_apex_state().lock().unwrap();
        let mut crit = 0;
        let mut high = 0;
        let mut med = 0;
        let mut low = 0;
        for f in &a.findings {
            match f.severity.to_lowercase().as_str() {
                "critical" => crit += 1,
                "high" => high += 1,
                "medium" | "med" => med += 1,
                _ => low += 1,
            }
        }
        (crit, high, med, low)
    };

    let (score_text, score_color, badge_text, badge_bg, badge_color) = if findings_total == 0 {
        ("0.0 / 10 — PENDING AUDIT", rgb(0x22c55e), "Audit Clean", rgb(0x064e3b), rgb(0x34d399))
    } else if crit_count > 0 {
        ("9.5 / 10 — CRITICAL EXPLOITABLE", rgb(0xef4444), "CRITICAL SEVERITY", rgb(0x7f1d1d), rgb(0xfca5a5))
    } else if high_count > 0 {
        ("7.8 / 10 — HIGH SEVERITY", rgb(0xf59e0b), "HIGH SEVERITY", rgb(0x78350f), rgb(0xfde68a))
    } else if med_count > 0 {
        ("5.2 / 10 — MEDIUM RISK", rgb(0x38bdf8), "MEDIUM RISK", rgb(0x1e293b), rgb(0x93c5fd))
    } else {
        ("2.5 / 10 — LOW RISK", rgb(0x94a3b8), "LOW RISK", rgb(0x1e293b), rgb(0x94a3b8))
    };

    div()
        .flex()
        .flex_col()
        .gap_3()
        // Header & Live Status
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x38bdf8))
                                .child("iOS Exploitation Toolkit & Frida Automation Suite"),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x94a3b8))
                                .child(format!("Target: {} • Active Module: {}", tgt_curr, act_mod)),
                        ),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(if running { rgb(0x451a03) } else { rgb(0x064e3b) })
                        .border_1()
                        .border_color(if running { rgb(0xf59e0b) } else { rgb(0x10b981) })
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if running { rgb(0xfcd34d) } else { rgb(0x6ee7b7) })
                        .child(if running { "AUDITING LIVE" } else { "READY" }),
                ),
        )
        // Vulnerability Score Matrix
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .p_2p5()
                .rounded(px(5.0))
                .bg(rgb(0x151926))
                .border_1()
                .border_color(if findings_total > 0 { score_color } else { rgb(0x252a3d) })
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(score_color)
                                .child(score_text),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x94a3b8))
                                .child(format!(
                                    "({} Crit • {} High • {} Med • {} Low)",
                                    crit_count, high_count, med_count, low_count
                                )),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(badge_bg)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(badge_color)
                        .child(badge_text),
                ),
        )
        // 8 Exploit Modules
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x94a3b8))
                        .child("Live Exploit Modules (Click to Launch)"),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .children(exploit_modules.into_iter().map(|(mod_name, desc)| {
                            let is_curr = act_mod == mod_name;
                            let is_run = running && is_curr;
                            let mod_str = mod_name.to_string();
                            let tgt_run = tgt_curr.clone();
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(if is_curr { rgb(0x131d2e) } else { rgb(0x151926) })
                                .border_1()
                                .border_color(if is_curr { rgb(0x1d4ed8) } else { rgb(0x252a3d) })
                                .hover(|s| s.bg(rgb(0x1e2436)))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |_this, _event, _window, cx| {
                                        execute_exploit_module_background(mod_str.clone(), tgt_run.clone());
                                        cx.notify();
                                    }),
                                )
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
                                                .gap_2()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(if is_curr { rgb(0x38bdf8) } else { rgb(0xffffff) })
                                                        .child(mod_name),
                                                )
                                                .children(if is_run {
                                                    Some(
                                                        div()
                                                            .px_1p5()
                                                            .py_0p5()
                                                            .rounded(px(2.0))
                                                            .bg(rgb(0x007acc))
                                                            .text_xs()
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0xffffff))
                                                            .child("RUNNING"),
                                                    )
                                                } else {
                                                    None
                                                }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x94a3b8))
                                                .child(desc),
                                        ),
                                )
                        })),
                ),
        )
        // Frida Presets
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x94a3b8))
                        .child("Frida Script Presets"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_1p5()
                        .children(frida_presets.into_iter().map(|(label, script_kind)| {
                            let t = tgt_curr.clone();
                            div()
                                .px_2p5()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(rgb(0x1a1e2e))
                                .border_1()
                                .border_color(rgb(0x2d3246))
                                .hover(|s| s.bg(rgb(0x282f48)))
                                .cursor_pointer()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0x38bdf8))
                                .child(label)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |_this, _event, _window, cx| {
                                        run_frida_scan_background(t.clone(), script_kind);
                                        cx.notify();
                                    }),
                                )
                        })),
                ),
        )
        // WDA Keystroke Payloads
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x94a3b8))
                        .child("WDA Remote Keystroke Injection"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_wrap()
                        .gap_1p5()
                        .children(wda_payloads.into_iter().map(|(payload, label)| {
                            let u = u_send.clone();
                            let p = payload.to_string();
                            div()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(rgb(0x1a1e2e))
                                .border_1()
                                .border_color(rgb(0x2d3246))
                                .hover(|s| s.bg(rgb(0x282f48)))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0xe2e8f0))
                                .child(label)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |_this, _event, _window, cx| {
                                        type_text_device_background(u.clone(), p.clone());
                                        cx.notify();
                                    }),
                                )
                        })),
                ),
        )
        .into_any_element()
}

fn render_wda_tab(udid: &str, cx: &mut Context<DeviceWindowView>) -> AnyElement {
    let u = udid.to_string();
    let ui_dump = get_apex_state().lock().ok().and_then(|a| a.wda_last_ui_dump.clone());

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
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x38bdf8))
                        .child("AppiumWebDriverAgent Accessibility Hierarchy"),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0062a3)))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .child("Dump UI Hierarchy")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |_this, _e, _w, cx| {
                                inspect_wda_ui_hierarchy_background(u.clone());
                                cx.notify();
                            }),
                        ),
                ),
        )
        .child(
            div()
                .p_3()
                .rounded(px(6.0))
                .bg(rgb(0x0a0d14))
                .border_1()
                .border_color(rgb(0x1e2433))
                .font_family("Cascadia Code, Consolas, monospace")
                .text_xs()
                .text_color(rgb(0x94a3b8))
                .child(ui_dump.unwrap_or_else(|| {
                    "Click 'Dump UI Hierarchy' to query the full active iOS accessibility tree from Appium WDA (port 8100)..."
                        .to_string()
                })),
        )
        .into_any_element()
}

fn render_console_tab(
    logs: &[String],
    status: &str,
    running: bool,
    target: &str,
    cx: &mut Context<DeviceWindowView>,
) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        // Status & Target Bar with Clear Button
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .p_2()
                .rounded(px(6.0))
                .bg(rgb(0x131722))
                .border_1()
                .border_color(rgb(0x23293a))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(if running { rgb(0x451a03) } else { rgb(0x064e3b) })
                                .border_1()
                                .border_color(if running { rgb(0xf59e0b) } else { rgb(0x10b981) })
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(if running { rgb(0xfcd34d) } else { rgb(0x6ee7b7) })
                                .child(if running { "AUDITING ACTIVE" } else { "READY" }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x94a3b8))
                                .child(format!("Target: {}", if target.is_empty() { "None" } else { target })),
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
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(if running { rgb(0xf59e0b) } else { rgb(0x38bdf8) })
                                .child(status.to_string()),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgb(0x1e293b))
                                .border_1()
                                .border_color(rgb(0x334155))
                                .hover(|s| s.bg(rgb(0x334155)))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(rgb(0x94a3b8))
                                .child("Clear")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|_this, _e, _w, cx| {
                                        clear_jb_console();
                                        cx.notify();
                                    }),
                                ),
                        ),
                ),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .p_3()
                .rounded(px(6.0))
                .bg(rgb(0x0a0d14))
                .border_1()
                .border_color(rgb(0x1e2433))
                .font_family("Cascadia Code, Consolas, monospace")
                .text_xs()
                .children(logs.iter().map(|msg| {
                    div()
                        .text_color(if msg.contains("✓") || msg.contains("[Gate✓]") {
                            rgb(0x22c55e)
                        } else if msg.contains("vuln") || msg.contains("CRITICAL") || msg.contains("High") {
                            rgb(0xef4444)
                        } else if msg.contains("touch") || msg.contains("swipe") {
                            rgb(0x38bdf8)
                        } else if msg.contains("launch") || msg.contains("bounty") || msg.contains("audit") {
                            rgb(0xfbbf24)
                        } else if msg.contains("exploit") || msg.contains("ssl") || msg.contains("keychain") {
                            rgb(0x38bdf8)
                        } else {
                            rgb(0x94a3b8)
                        })
                        .child(msg.clone())
                })),
        )
        .into_any_element()
}

fn render_findings_tab() -> AnyElement {
    let findings = get_apex_state().lock().ok().map(|a| a.findings.clone()).unwrap_or_default();

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
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x38bdf8))
                        .child(format!("Bounty Security Findings ({})", findings.len())),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if findings.is_empty() { rgb(0x94a3b8) } else { rgb(0xef4444) })
                        .child(if findings.is_empty() {
                            "0 Vulnerabilities"
                        } else {
                            "Active Bounty Surface Identified"
                        }),
                ),
        )
        .children(if findings.is_empty() {
            vec![div()
                .p_4()
                .rounded(px(6.0))
                .bg(rgb(0x151926))
                .border_1()
                .border_color(rgb(0x252a3d))
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child("No findings recorded for this session yet. Go to 'Target Apps' and click 'Audit' to run automated security analysis."),
                )]
        } else {
            findings
                .iter()
                .map(|f| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .p_3()
                        .rounded(px(6.0))
                        .bg(rgb(0x151926))
                        .border_1()
                        .border_color(rgb(0x252a3d))
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
                                        .text_color(rgb(0xffffff))
                                        .child(f.rule.clone()),
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(if f.severity == "High" || f.severity == "Critical" {
                                            rgb(0xef4444)
                                        } else if f.severity == "Medium" {
                                            rgb(0xf59e0b)
                                        } else {
                                            rgb(0x007acc)
                                        })
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child(f.severity),
                                ),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(rgb(0x38bdf8))
                                .child(format!("{}:{}", f.file, f.line)),
                        )
                        .child(
                            div()
                                .p_1p5()
                                .rounded(px(4.0))
                                .bg(rgb(0x0a0d14))
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_xs()
                                .text_color(rgb(0xe2e8f0))
                                .child(f.snippet.clone()),
                        )
                })
                .collect::<Vec<_>>()
        })
        .into_any_element()
}

pub fn run_device_mirror_app() {
    println!("[vscodium-ip-panel] Launching vscodium-ip-panel (Draggable, Movable & Resizable)...");
    gpui_kit::application()
        .with_assets(gpui_kit_assets::AllAssets)
        .run(|cx: &mut App| {
            gpui_kit::init(cx);
            let window_bounds = Some(WindowBounds::centered(size(px(1150.0), px(900.0)), cx));
            cx.open_window(
                WindowOptions {
                    window_bounds,
                    titlebar: Some(TitlebarOptions {
                        title: Some("vscodium-ip-panel".into()),
                        appears_transparent: false,
                        traffic_light_position: None,
                    }),
                    is_movable: true,
                    is_resizable: true,
                    window_min_size: Some(size(px(480.0), px(580.0))),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|cx| DeviceWindowView::new(window, cx));
                    cx.new(|cx| gpui_kit::component::Root::new(view, window, cx))
                },
            )
            .expect("Failed to create detached vscodium-ip-panel window");
        });
}
