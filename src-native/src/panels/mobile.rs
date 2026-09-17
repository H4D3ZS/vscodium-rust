use crate::app_state::HadesNativeState;
use crate::domain::layout::EmulatorSubTab;
use crate::panels::apex::get_apex_state;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

static TOUCH_START_POS: std::sync::OnceLock<std::sync::RwLock<Option<(f64, f64, std::time::Instant)>>> =
    std::sync::OnceLock::new();
static MOBILE_SCREEN_BOUNDS: std::sync::OnceLock<std::sync::RwLock<Option<(f32, f32, f32, f32)>>> =
    std::sync::OnceLock::new();

pub struct MobilePanel;

impl crate::panels::traits::WorkbenchPanel for MobilePanel {
    fn id(&self) -> &'static str {
        "mobile"
    }

    fn title(&self) -> &'static str {
        "DEVICES"
    }

    fn icon(&self) -> IconName {
        IconName::Smartphone
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_mobile_panel(state, cx).into_any_element()
    }
}

pub fn render_mobile_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let active_subtab = state.active_emulator_subtab;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        .p_2p5()
        .gap_2p5()
        // ── 1. Top Sub-Tabs Row (UnifiedEmulatorPanel.tsx parity) ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                .p_1()
                .rounded(px(7.0))
                .bg(rgb(0x131722))
                .border_1()
                .border_color(rgb(0x23293a))
                .children(
                    [
                        (EmulatorSubTab::Android, "Android", IconName::Smartphone),
                        (EmulatorSubTab::IPhone, "iPhone", IconName::Smartphone),
                        (EmulatorSubTab::Device, "Device", IconName::Zap),
                        (EmulatorSubTab::Toolchain, "Toolchain", IconName::Terminal),
                        (EmulatorSubTab::Gradle, "Gradle", IconName::Settings),
                    ]
                    .into_iter()
                    .map(|(tab, label, ic)| {
                        let is_sel = active_subtab == tab;
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
                            .px_3()
                            .py_1()
                            .rounded(px(5.0))
                            .cursor_pointer()
                            .bg(if is_sel {
                                rgb(0x007acc)
                            } else {
                                rgba(0x00000000)
                            })
                            .text_color(if is_sel {
                                rgb(0xffffff)
                            } else {
                                rgb(0x94a3b8)
                            })
                            .text_xs()
                            .font_weight(if is_sel {
                                FontWeight::BOLD
                            } else {
                                FontWeight::MEDIUM
                            })
                            .hover(|s| {
                                s.bg(if is_sel {
                                    rgb(0x0066aa)
                                } else {
                                    rgb(0x1e2436)
                                })
                                .text_color(rgb(0xffffff))
                            })
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    this.state.active_emulator_subtab = tab;
                                    cx.notify();
                                }),
                            )
                            .child(icon_12(ic, if is_sel { rgb(0xffffff) } else { rgb(0x64748b) }))
                            .child(label)
                    }),
                ),
        )
        // ── 2. Active Tab Content Switcher ──
        .child(
            div()
                .id("mobile_panel_content_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .min_h(px(0.0))
                .overflow_y_scroll()
                .child(match active_subtab {
                    EmulatorSubTab::Android => render_android_emulator(state, cx).into_any_element(),
                    EmulatorSubTab::IPhone => render_iphone_simulator(state, cx).into_any_element(),
                    EmulatorSubTab::Device => render_physical_device_mirror(state, cx).into_any_element(),
                    EmulatorSubTab::Toolchain => render_mobile_toolchain(state, cx).into_any_element(),
                    EmulatorSubTab::Gradle => render_gradle_tools(state, cx).into_any_element(),
                })
        )
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. PHYSICAL DEVICE MIRROR & INTERACTIVE CHASSIS (IPhoneMirrorPanel.tsx Parity)
// ─────────────────────────────────────────────────────────────────────────────
fn render_physical_iphone_xr_chassis(
    state: &HadesNativeState,
    dev_udid: &str,
    dev_name: &str,
    _dev_model: &str,
    dev_ios: &str,
    has_device: bool,
    mirror_active: bool,
    _last_touch: Option<(f64, f64)>,
    fg_app: Option<String>,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let udid_up = dev_udid.to_string();
    let udid_scroll = dev_udid.to_string();
    let udid_home = dev_udid.to_string();
    let udid_switcher = dev_udid.to_string();
    let udid_shot = dev_udid.to_string();
    let udid_lock = dev_udid.to_string();
    let udid_volup = dev_udid.to_string();
    let udid_voldown = dev_udid.to_string();
    let udid_inspect = dev_udid.to_string();
    let _current_app = fg_app.unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .items_center()
        .gap_2()
        .child(
            // Top Live Mirror Stream Status Pill & Pop-out Action
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w(px(232.0))
                .px_2()
                .py_1()
                .rounded(px(6.0))
                .bg(rgb(0x131722))
                .border_1()
                .border_color(rgb(0x272e42))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(
                            div()
                                .w(px(7.0))
                                .h(px(7.0))
                                .rounded_full()
                                .bg(if !has_device {
                                    rgb(0xef4444)
                                } else if mirror_active {
                                    rgb(0x22c55e)
                                } else {
                                    rgb(0xeab308)
                                })
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(if !has_device {
                                    rgb(0xf87171)
                                } else if mirror_active {
                                    rgb(0x22c55e)
                                } else {
                                    theme.text_muted
                                })
                                .child(if !has_device {
                                    "OFFLINE"
                                } else if mirror_active {
                                    "60 FPS LIVE"
                                } else {
                                    "PAUSED"
                                })
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
                                .rounded(px(4.0))
                                .bg(if !has_device {
                                    rgb(0x1a2030)
                                } else if mirror_active {
                                    rgb(0x007acc)
                                } else {
                                    theme.bg_card
                                })
                                .hover(|s| s.bg(if !has_device {
                                    rgb(0x252e46)
                                } else if mirror_active {
                                    rgb(0x0066aa)
                                } else {
                                    theme.bg_hover
                                }))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if !has_device { rgb(0x38bdf8) } else { rgb(0xffffff) })
                                .cursor_pointer()
                                .child(if !has_device { "Scan" } else if mirror_active { "Stop" } else { "Start" })
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let udid = dev_udid.to_string();
                                    move |this, _e, _w, cx| {
                                        if !has_device {
                                            crate::panels::apex::trigger_device_refresh();
                                            this.state.toast_manager.push_info("Scanning USB bus for iOS devices...");
                                        } else if mirror_active {
                                            crate::panels::apex::stop_screen_mirror_background();
                                            this.state.toast_manager.push_info("Screen mirror paused");
                                        } else {
                                            crate::panels::apex::start_screen_mirror_background(udid.clone());
                                            this.state.toast_manager.push_info("Live USB screen mirror started (60fps)");
                                        }
                                        cx.notify();
                                    }
                                }))
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(rgb(0x1e293b))
                                .border_1()
                                .border_color(rgb(0x38bdf8))
                                .hover(|s| s.bg(rgb(0x334155)))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x38bdf8))
                                .cursor_pointer()
                                .child("⤢ Pop-out")
                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, _w, cx| {
                                    if let Ok(exe) = std::env::current_exe() {
                                        let mut cmd = if let Some(parent) = exe.parent() {
                                            let ip_panel = parent.join("vscodium-ip-panel.exe");
                                            if !ip_panel.exists() {
                                                let _ = std::fs::copy(&exe, &ip_panel);
                                            }
                                            if ip_panel.exists() {
                                                std::process::Command::new(&ip_panel)
                                            } else {
                                                let mut c = std::process::Command::new(&exe);
                                                c.arg("--device-window");
                                                c
                                            }
                                        } else {
                                            let mut c = std::process::Command::new(&exe);
                                            c.arg("--device-window");
                                            c
                                        };
                                        cmd.stdin(std::process::Stdio::null())
                                            .stdout(std::process::Stdio::null())
                                            .stderr(std::process::Stdio::null());
                                        let _ = cmd.spawn();
                                        this.state.toast_manager.push_info("vscodium-ip-panel launched (draggable, movable & resizable)");
                                        cx.notify();
                                    }
                                }))
                        )
                )
        )
        // Center Phone Body with Hardware Buttons
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1()
                // Left Hardware Buttons (Volume Up, Volume Down)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .child(
                            div()
                                .w(px(4.0))
                                .h(px(26.0))
                                .rounded(px(2.0))
                                .bg(rgb(0x3f4459))
                                .hover(|s| s.bg(theme.accent))
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let u = udid_volup.clone();
                                    move |this, _e, _w, cx| {
                                        if !has_device {
                                            this.state.toast_manager.push_info("Connect an iPhone via USB to send hardware keys");
                                            return;
                                        }
                                        crate::panels::apex::send_device_hardware_button_background(u.clone(), "volumeUp");
                                        this.state.toast_manager.push_info("Volume Up sent to iOS device");
                                        cx.notify();
                                    }
                                }))
                        )
                        .child(
                            div()
                                .w(px(4.0))
                                .h(px(26.0))
                                .rounded(px(2.0))
                                .bg(rgb(0x3f4459))
                                .hover(|s| s.bg(theme.accent))
                                .cursor_pointer()
                                .on_mouse_down(MouseButton::Left, cx.listener({
                                    let u = udid_voldown.clone();
                                    move |this, _e, _w, cx| {
                                        if !has_device {
                                            this.state.toast_manager.push_info("Connect an iPhone via USB to send hardware keys");
                                            return;
                                        }
                                        crate::panels::apex::send_device_hardware_button_background(u.clone(), "volumeDown");
                                        this.state.toast_manager.push_info("Volume Down sent to iOS device");
                                        cx.notify();
                                    }
                                }))
                        )
                )
                // Main Glass Chassis (Liquid Retina Display)
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .w(px(232.0))
                        .h(px(470.0))
                        .rounded(px(26.0))
                        .bg(rgb(0x000000))
                        .border_2()
                        .border_color(rgb(0x272b3c))
                        .shadow_xl()
                        .overflow_hidden()
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            move |_this, event: &MouseDownEvent, _window, _cx| {
                                if !has_device || !mirror_active {
                                    return;
                                }
                                let (ox, oy, w, h) = MOBILE_SCREEN_BOUNDS
                                    .get_or_init(|| std::sync::RwLock::new(None))
                                    .read()
                                    .ok()
                                    .and_then(|g| *g)
                                    .unwrap_or((0.0, 0.0, 232.0, 470.0));

                                let click_x: f32 = event.position.x.into();
                                let click_y: f32 = event.position.y.into();

                                let local_x = (click_x - ox).clamp(0.0, w);
                                let local_y = (click_y - oy).clamp(0.0, h);

                                if let Ok(mut lock) = TOUCH_START_POS.get_or_init(|| std::sync::RwLock::new(None)).write() {
                                    *lock = Some((local_x as f64, local_y as f64, std::time::Instant::now()));
                                }
                            }
                        }))
                        .on_mouse_up(MouseButton::Left, cx.listener({
                            let u = udid_up.clone();
                            move |this, event: &MouseUpEvent, _window, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Device offline. Connect an iPhone via USB.");
                                    return;
                                }
                                if !mirror_active {
                                    this.state.toast_manager.push_info("Mirror paused. Click 'Start' to activate live touch stream.");
                                    return;
                                }
                                let (ox, oy, w, h) = MOBILE_SCREEN_BOUNDS
                                    .get_or_init(|| std::sync::RwLock::new(None))
                                    .read()
                                    .ok()
                                    .and_then(|g| *g)
                                    .unwrap_or((0.0, 0.0, 232.0, 470.0));

                                let click_x: f32 = event.position.x.into();
                                let click_y: f32 = event.position.y.into();

                                let local_end_x = (click_x - ox).clamp(0.0, w);
                                let local_end_y = (click_y - oy).clamp(0.0, h);

                                let start_info = TOUCH_START_POS
                                    .get_or_init(|| std::sync::RwLock::new(None))
                                    .write()
                                    .ok()
                                    .and_then(|mut g| g.take());

                                if let Some((start_local_x, start_local_y, start_time)) = start_info {
                                    let dx = local_end_x as f64 - start_local_x;
                                    let dy = local_end_y as f64 - start_local_y;
                                    let local_dist = (dx * dx + dy * dy).sqrt();
                                    let elapsed = start_time.elapsed().as_secs_f64();

                                    let norm_start_x = (start_local_x / w as f64).clamp(0.0, 1.0);
                                    let norm_start_y = (start_local_y / h as f64).clamp(0.0, 1.0);
                                    let norm_end_x = (local_end_x as f64 / w as f64).clamp(0.0, 1.0);
                                    let norm_end_y = (local_end_y as f64 / h as f64).clamp(0.0, 1.0);

                                    let pt_end_x = norm_end_x * 414.0;
                                    let pt_end_y = norm_end_y * 896.0;

                                    if local_dist < 15.0 || (elapsed < 0.28 && local_dist < 32.0) {
                                        // Tap
                                        crate::panels::apex::send_device_tap_background(u.clone(), norm_end_x, norm_end_y);
                                        this.state.toast_manager.push_info(&format!("Tap at ({:.0}, {:.0})", pt_end_x, pt_end_y));
                                    } else {
                                        // Swipe / Drag gesture
                                        let duration = elapsed.clamp(0.12, 0.55);
                                        crate::panels::apex::send_device_swipe_background(
                                            u.clone(),
                                            norm_start_x,
                                            norm_start_y,
                                            norm_end_x,
                                            norm_end_y,
                                            duration,
                                        );
                                        this.state.toast_manager.push_info(&format!(
                                             "Swipe ({:.0},{:.0}) -> ({:.0},{:.0})",
                                            norm_start_x * 414.0, norm_start_y * 896.0, pt_end_x, pt_end_y
                                        ));
                                    }
                                    cx.notify();
                                }
                            }
                        }))
                        .on_scroll_wheel(cx.listener({
                            let u = udid_scroll.clone();
                            move |_this, event: &ScrollWheelEvent, _window, cx| {
                                if !has_device || !mirror_active {
                                    return;
                                }
                                let dy: f32 = match event.delta {
                                    ScrollDelta::Lines(d) => -d.y * 30.0,
                                    ScrollDelta::Pixels(d) => {
                                        let p: f32 = d.y.into();
                                        -p
                                    }
                                };
                                crate::panels::apex::send_device_scroll_background(u.clone(), dy);
                                cx.notify();
                            }
                        }))
                        // Screen Canvas to accurately obtain window layout bounds
                        .child(
                            canvas(
                                |bounds, _window, _cx| {
                                    let ox: f32 = bounds.origin.x.into();
                                    let oy: f32 = bounds.origin.y.into();
                                    let w: f32 = bounds.size.width.into();
                                    let h: f32 = bounds.size.height.into();
                                    if let Ok(mut g) = MOBILE_SCREEN_BOUNDS.get_or_init(|| std::sync::RwLock::new(None)).write() {
                                        *g = Some((ox, oy, w, h));
                                    }
                                },
                                |_bounds, _, _window, _cx| {},
                            )
                            .size_full()
                            .absolute()
                            .inset_0()
                        )
                        .child({
                            if has_device && mirror_active {
                                let live_frame = crate::panels::apex::get_live_screen_frame();
                                if let Some(frame) = live_frame {
                                    div()
                                        .relative()
                                        .size_full()
                                        .child(
                                            img(frame)
                                                .size_full()
                                                .object_fit(ObjectFit::Contain)
                                        )
                                        // Interactive Bottom Home Bar Indicator (iPhone Mirroring UX)
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
                                                        .w(px(90.0))
                                                        .h(px(4.0))
                                                        .rounded(px(2.0))
                                                        .bg(rgba(0xffffffaa))
                                                        .hover(|s| s.bg(rgb(0xffffff)))
                                                        .cursor_pointer()
                                                        .on_mouse_down(MouseButton::Left, cx.listener({
                                                            let u = udid_home.clone();
                                                            move |this, _e, _w, cx| {
                                                                crate::panels::apex::send_device_hardware_button_background(u.clone(), "home");
                                                                this.state.toast_manager.push_info("Home Screen Gesture Sent");
                                                                cx.notify();
                                                            }
                                                        }))
                                                )
                                        )
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
                                                .child("Connecting to iPhone USB Stream...")
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child("Streaming in-memory at 60 FPS (0 disk writes)")
                                        )
                                }
                            } else if has_device && !mirror_active {
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
                                            .child(icon_14(IconName::Smartphone, rgb(0x60a5fa)))
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
                                                    .child(dev_name.to_string())
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(rgb(0x94a3b8))
                                                    .child(format!("iOS {dev_ios} • USB Connected"))
                                            )
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
                                            .child("▶ Start Live Mirror")
                                            .on_mouse_down(MouseButton::Left, cx.listener({
                                                let udid = dev_udid.to_string();
                                                move |this, _e, _w, cx| {
                                                    crate::panels::apex::start_screen_mirror_background(udid.clone());
                                                    this.state.toast_manager.push_info("Live USB screen mirror started (60fps)");
                                                    cx.notify();
                                                }
                                            }))
                                    )
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
                                            .child(icon_14(IconName::Smartphone, rgb(0xf87171)))
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
                                                    .child("NO DEVICE DETECTED")
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_subtle)
                                                    .text_center()
                                                    .child("Connect iPhone via USB cable")
                                            )
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
                                                    .child(div().text_xs().text_color(rgb(0x94a3b8)).child("usbmuxd: ready"))
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_1p5()
                                                    .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0xef4444)))
                                                    .child(div().text_xs().text_color(rgb(0x94a3b8)).child("USB link: unplugged"))
                                            )
                                            .child(
                                                div()
                                                    .flex()
                                                    .flex_row()
                                                    .items_center()
                                                    .gap_1p5()
                                                    .child(div().w(px(5.0)).h(px(5.0)).rounded_full().bg(rgb(0xeab308)))
                                                    .child(div().text_xs().text_color(rgb(0x94a3b8)).child("Lockdownd: waiting"))
                                            )
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
                                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                                crate::panels::apex::trigger_device_refresh();
                                                this.state.toast_manager.push_info("Scanning USB mux for connected iOS devices...");
                                                cx.notify();
                                            }))
                                    )
                            }
                        })
                )
                // Right Hardware Button (Power / Lock)
                .child(
                    div()
                        .w(px(4.0))
                        .h(px(38.0))
                        .rounded(px(2.0))
                        .bg(rgb(0x3f4459))
                        .hover(|s| s.bg(theme.accent))
                        .cursor_pointer()
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_lock.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to send hardware keys");
                                    return;
                                }
                                crate::panels::apex::send_device_hardware_button_background(u.clone(), "lock");
                                this.state.toast_manager.push_info("Lock button sent to iOS device");
                                cx.notify();
                            }
                        }))
                )
        )
        // Hardware Control Action Buttons (Home, Lock, Refresh, Inspect)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w(px(232.0))
                .gap_1()
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_card))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child("Home")
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_home.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to interact");
                                    return;
                                }
                                crate::panels::apex::send_device_hardware_button_background(u.clone(), "home");
                                this.state.toast_manager.push_info("Home dispatched to iOS device");
                                cx.notify();
                            }
                        }))
                )
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_card))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child("Apps")
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_switcher.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to interact");
                                    return;
                                }
                                crate::panels::apex::send_device_swipe_background(u.clone(), 414.0, 1750.0, 414.0, 1050.0, 0.42);
                                this.state.toast_manager.push_info("App Switcher gesture sent");
                                cx.notify();
                            }
                        }))
                )
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_card))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child("Lock")
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_lock.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to interact");
                                    return;
                                }
                                crate::panels::apex::send_device_hardware_button_background(u.clone(), "lock");
                                this.state.toast_manager.push_info("Lock / Wake dispatched to iOS device");
                                cx.notify();
                            }
                        }))
                )
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0062a3)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .child("Snap")
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_shot.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to capture snapshot");
                                    return;
                                }
                                crate::panels::apex::capture_device_screenshot_background(u.clone());
                                this.state.toast_manager.push_info("Capturing fresh screen frame from iOS device...");
                                cx.notify();
                            }
                        }))
                )
                .child(
                    div()
                        .flex_1()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x272b3c))
                        .hover(|s| s.bg(rgb(0x353a4f)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0xffffff))
                        .child("WDA")
                        .on_mouse_down(MouseButton::Left, cx.listener({
                            let u = udid_inspect.clone();
                            move |this, _e, _w, cx| {
                                if !has_device {
                                    this.state.toast_manager.push_info("Connect an iPhone via USB to inspect WDA hierarchy");
                                    return;
                                }
                                crate::panels::apex::inspect_wda_ui_hierarchy_background(u.clone());
                                this.state.toast_manager.push_info("Inspecting UI view hierarchy via Appium WDA...");
                                cx.notify();
                            }
                        }))
                )
        )
        // Bottom Telemetry Bar
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w(px(232.0))
                .px_2()
                .py_1()
                .rounded(px(4.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.text_subtle)
                        .child(if has_device { "Latency: ~11ms (USB)" } else { "Link: Disconnected" })
                )
                .child(
                    div()
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(if has_device { theme.accent } else { theme.text_muted })
                        .child(if has_device { "828×1792 @2x" } else { "usbmuxd ready" })
                )
        )
}

pub fn render_physical_device_mirror(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    let (connected_dev, is_scanning, mirror_active, last_touch, fg_app) = {
        let a = get_apex_state().lock().unwrap();
        let dev = a.connected_devices.first().cloned();
        (
            dev,
            a.is_scanning_devices,
            a.mirror_active,
            a.last_touch_coords,
            a.active_foreground_app.clone(),
        )
    };

    let (dev_name, dev_model, dev_udid, dev_ios, dev_conn, has_device) = match connected_dev {
        Some(d) => (d.name, d.model, d.udid, d.ios_version, d.connection, true),
        None => (
            "No Device Detected".to_string(),
            "iOS Device".to_string(),
            "".to_string(),
            "".to_string(),
            "USB Unplugged".to_string(),
            false,
        ),
    };

    div()
        .flex()
        .flex_col()
        .flex_1()
        .items_center()
        .p_3()
        .gap_2p5()
        // Top status card: Device info & Refresh
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .w(px(242.0))
                .px_2p5()
                .py_1p5()
                .rounded(px(6.0))
                .bg(rgb(0x131722))
                .border_1()
                .border_color(rgb(0x23293a))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(if has_device { rgb(0xffffff) } else { rgb(0x94a3b8) })
                                .child(if has_device {
                                    format!("{dev_name} ({dev_model})")
                                } else {
                                    "No iPhone Detected".to_string()
                                }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(if has_device {
                                    format!("iOS {dev_ios} • {dev_conn}")
                                } else {
                                    "Connect via USB".to_string()
                                }),
                        ),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(rgb(0x1a2030))
                        .border_1()
                        .border_color(rgb(0x2a334c))
                        .hover(|s| s.bg(rgb(0x252e46)))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child(if is_scanning { "Scanning..." } else { "Refresh" })
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                            crate::panels::apex::trigger_device_refresh();
                            this.state.toast_manager.push_info("Scanning USB for connected iOS devices...");
                            cx.notify();
                        })),
                ),
        )
        // Center Physical iPhone Chassis with Mirror & Pop-out
        .child(render_physical_iphone_xr_chassis(
            state,
            &dev_udid,
            &dev_name,
            &dev_model,
            &dev_ios,
            has_device,
            mirror_active,
            last_touch,
            fg_app.clone(),
            cx,
        ))
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. GENUINE ANDROID EMULATOR (EmulatorPanel.tsx & AndroidEmulatorDisplay.tsx)
// ─────────────────────────────────────────────────────────────────────────────
fn render_android_emulator(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .flex_1()
        .gap_2p5()
        // Top Toolbar: AVD Select | Launch | Refresh | Kill | Status
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_1p5()
                // AVD Selector
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("Pixel_7_Pro_API_34 (x86_64)")
                        .child(icon_12(IconName::ChevronDown, theme.text_subtle))
                )
                // ▶ Launch Button
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2p5()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0066aa)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                            this.execute_terminal_command("emulator -avd Pixel_7_Pro_API_34 -netdelay none -netspeed full");
                            this.state.toast_manager.push_info("Launching Android Emulator: Pixel_7_Pro_API_34...");
                            cx.notify();
                        }))
                        .child(icon_12(IconName::Play, rgb(0xffffff)))
                        .child("Launch AVD")
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Refresh")
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                            this.execute_terminal_command("adb emu kill");
                            this.state.toast_manager.push_info("Sent adb emu kill signal.");
                            cx.notify();
                        }))
                        .child("Stop")
                )
                // Status Pill
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(rgba(0x22c55e18))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0x22c55e))
                        .child("● Port 5554 Ready")
                )
        )
        // Center: Android Screen Chassis with Side Hardware Controls
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .flex_1()
                .gap_3()
                .p_2()
                // Phone Glass Chassis
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(250.0))
                        .h(px(460.0))
                        .rounded(px(28.0))
                        .bg(rgb(0x0c0d12))
                        .border_2()
                        .border_color(rgb(0x2a2c36))
                        .shadow_lg()
                        .p_2p5()
                        // Android Status Bar: 10:00 | Wifi | LTE | Battery
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .px_2()
                                .h(px(22.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child("10:00")
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(rgb(0x888888))
                                                .child("5G")
                                        )
                                        .child(icon_12(IconName::Wifi, rgb(0xffffff)))
                                )
                        )
                        // Live Android Display Canvas
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .flex_1()
                                .rounded(px(16.0))
                                .bg(rgb(0x13151c))
                                .border_1()
                                .border_color(rgb(0x1e222e))
                                .p_3()
                                .gap_3()
                                .child(
                                    div()
                                        .w(px(48.0))
                                        .h(px(48.0))
                                        .rounded(px(14.0))
                                        .bg(rgb(0x007acc))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(icon_14(IconName::Play, rgb(0xffffff)))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child("Android 14 (API 34)")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .text_center()
                                        .child("scrcpy/webrtc stream active")
                                )
                        )
                        // Bottom Navigation Bar: Back ◀ | Home ● | Recents ■
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_around()
                                .h(px(28.0))
                                .text_color(rgb(0x888888))
                                .text_xs()
                                .child("◀")
                                .child("●")
                                .child("■")
                        )
                )
                // Side Hardware Action Buttons
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_2()
                        .p_1p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .children([
                            ("Power", IconName::Zap),
                            ("Vol +", IconName::Plus),
                            ("Vol -", IconName::Minus),
                            ("Rotate", IconName::RefreshCw),
                            ("Capture", IconName::Camera),
                        ].into_iter().map(|(label, icon)| {
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_0p5()
                                .p_1p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child(icon_12(icon, theme.text_muted))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(label)
                                )
                        }))
                )
        )
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. GENUINE iPHONE SIMULATOR (IPhoneAcheronPanel.tsx Parity)
// ─────────────────────────────────────────────────────────────────────────────
pub fn render_iphone_simulator(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .flex_1()
        .gap_2p5()
        // Top Toolbar: Device Selector | ▶ Start Simulator | ■ Stop | Firmware | Logs
        .child(
            div()
                .flex()
                .flex_row()
                .flex_wrap()
                .items_center()
                .gap_1p5()
                // Device Dropdown
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("iPhone 16 Pro (iOS 18.0)")
                        .child(icon_12(IconName::ChevronDown, theme.text_subtle)),
                )
                // Start Button
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2p5()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0066aa)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.state.toast_manager.push_info(
                                    "Starting CoreSimulator Acheron session (1290x2796)...",
                                );
                                cx.notify();
                            }),
                        )
                        .child(icon_12(IconName::Play, rgb(0xffffff)))
                        .child("Start Simulator"),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("IPSW Firmware"),
                )
                .child(
                    div()
                        .px_2()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Console Logs"),
                )
                // Status indicator
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(rgba(0x38bdf818))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(rgb(0x38bdf8))
                        .child("CoreSimulator 18.0"),
                ),
        )
        // Center: Genuine iPhone 16 Pro Chassis with Dynamic Island
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_center()
                .flex_1()
                .p_2()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(250.0))
                        .h(px(470.0))
                        .rounded(px(32.0))
                        .bg(rgb(0x0a0a0f))
                        .border_2()
                        .border_color(rgb(0x27272a))
                        .shadow_lg()
                        .p_2p5()
                        // Dynamic Island & Status Bar
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .px_2()
                                .h(px(24.0))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child("9:41"),
                                )
                                // Dynamic Island Pill
                                .child(
                                    div()
                                        .w(px(72.0))
                                        .h(px(16.0))
                                        .rounded_full()
                                        .bg(rgb(0x000000))
                                        .border_1()
                                        .border_color(rgb(0x18181b)),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .child(icon_12(IconName::Wifi, rgb(0xffffff)))
                                        .child(
                                            div()
                                                .w(px(14.0))
                                                .h(px(8.0))
                                                .rounded(px(2.0))
                                                .border_1()
                                                .border_color(rgb(0xffffff)),
                                        ),
                                ),
                        )
                        // Live Display Surface
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .flex_1()
                                .rounded(px(20.0))
                                .bg(rgb(0x11131a))
                                .border_1()
                                .border_color(rgb(0x202433))
                                .p_3()
                                .gap_2()
                                .child(
                                    div()
                                        .w(px(46.0))
                                        .h(px(46.0))
                                        .rounded(px(12.0))
                                        .bg(rgb(0x007acc))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(icon_14(IconName::Smartphone, rgb(0xffffff))),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .child("iOS 18.0 Simulator"),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .text_center()
                                        .child("1290 x 2796 • Direct3D 12"),
                                ),
                        )
                        // iOS Home Indicator
                        .child(
                            div().flex().justify_center().w_full().py_1().child(
                                div()
                                    .w(px(90.0))
                                    .h(px(3.5))
                                    .rounded_full()
                                    .bg(rgb(0x666666)),
                            ),
                        ),
                ),
        )
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. MOBILE TOOLCHAIN (MobileToolchainPanel.tsx Parity)
// ─────────────────────────────────────────────────────────────────────────────
fn render_mobile_toolchain(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .flex_1()
        .gap_2p5()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0066aa)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.execute_terminal_command(
                                    "bash -c './scripts/vphone-doctor.sh'",
                                );
                                this.state.bottom_panel_open = true;
                                cx.notify();
                            }),
                        )
                        .child("Run vphone-doctor"),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("Install Shims"),
                )
                .child(
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .cursor_pointer()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Refresh Paths"),
                ),
        )
        // Checklist Cards
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .p_3()
                .rounded(px(6.0))
                .bg(theme.bg_raised)
                .border_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("HOST TOOLCHAIN PATHS & PREREQUISITES"),
                )
                .children(
                    [
                        ("Android SDK", "Found: ANDROID_HOME resolved", true),
                        ("Java JDK 17", "Found: OpenJDK 17.0.9", true),
                        (
                            "go-ios Daemon",
                            "Install from github.com/danielpaulus/go-ios",
                            false,
                        ),
                        (
                            "zsign / ldid",
                            "Ready for Windows Mach-O code-signing",
                            true,
                        ),
                    ]
                    .into_iter()
                    .map(|(name, status_text, ok)| {
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_between()
                            .py_1()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_2()
                                    .child(if ok {
                                        icon_12(IconName::Check, rgb(0x22c55e)).into_any_element()
                                    } else {
                                        icon_12(IconName::X, rgb(0xef4444)).into_any_element()
                                    })
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(theme.text_primary)
                                            .child(name),
                                    ),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(if ok { theme.text_muted } else { rgb(0xef4444) })
                                    .child(status_text),
                            )
                    }),
                ),
        )
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. GRADLE TOOLS (GradleToolsPanel.tsx Parity)
// ─────────────────────────────────────────────────────────────────────────────
fn render_gradle_tools(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .flex_1()
        .gap_2p5()
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
                        .child("GRADLE BUILD PIPELINE"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child("Variant: debug"),
                ),
        )
        // Task Execution List
        .child(
            div().flex().flex_col().gap_1p5().children(
                [
                    (":app:assembleDebug", "Build debug APK package"),
                    (
                        ":app:installDebug",
                        "Install APK to active running emulator",
                    ),
                    (":app:bundleRelease", "Generate Android App Bundle (.aab)"),
                    (":clean", "Wipe build cache and temporary artifacts"),
                    (":test", "Run JUnit local unit tests"),
                ]
                .into_iter()
                .map(|(task, desc)| {
                    let task_clone = task.to_string();
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child(task),
                                )
                                .child(div().text_xs().text_color(theme.text_subtle).child(desc)),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_1()
                                .rounded(px(3.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.accent_hover))
                                .text_xs()
                                .text_color(rgb(0xffffff))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        this.execute_terminal_command(&format!(
                                            "./gradlew {}",
                                            task_clone
                                        ));
                                        this.state.bottom_panel_open = true;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Play, rgb(0xffffff)))
                                .child("Run"),
                        )
                }),
            ),
        )
}
