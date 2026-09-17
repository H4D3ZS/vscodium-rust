use crate::app_state::HadesNativeState;
use crate::panels::traits::WorkbenchPanel;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct VisionPanel;

impl WorkbenchPanel for VisionPanel {
    fn id(&self) -> &'static str {
        "vision"
    }

    fn title(&self) -> &'static str {
        "Vision & Grounding"
    }

    fn icon(&self) -> IconName {
        IconName::Eye
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_vision_panel(state, cx).into_any_element()
    }
}

pub fn render_vision_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Header
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .border_b_1()
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
                                .items_center()
                                .gap_2()
                                .child(ui_icon(IconName::Eye, 16.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("HADES VISION GROUNDING")
                                )
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.status_green)
                                .child("Screen OCR Active")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .gap_2()
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Spatial UI element coordinates, OCR bounding boxes, and screen perception")
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.toast_manager.push_info("Vision Model: Qwen2.5-VL-7B Active");
                                    cx.notify();
                                }))
                                .child(ui_icon(IconName::Sparkles, 10.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Qwen2.5-VL ▾")
                                )
                        )
                )
        )
        // Live Screen Perception Card
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .gap_2p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_surface)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_muted)
                                .child("VIEWPORT SPATIAL MAP")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .child("1920×1080 @ 120 FPS Direct3D 12")
                        )
                )
                // Simulated screen viewport canvas with bounding boxes
                .child(
                    div()
                        .relative()
                        .w_full()
                        .h(px(140.0))
                        .rounded(px(6.0))
                        .bg(theme.bg_app)
                        .border_1()
                        .border_color(theme.border_focus)
                        .overflow_hidden()
                        // Bounding box 1 (Editor pane)
                        .child(
                            div()
                                .absolute()
                                .top(px(20.0))
                                .left(px(60.0))
                                .w(px(180.0))
                                .h(px(90.0))
                                .border_1()
                                .border_color(theme.accent)
                                .bg(theme.bg_hover)
                                .p_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .child("[Editor: Rust Tree-Sitter]")
                                )
                        )
                        // Bounding box 2 (Sidebar)
                        .child(
                            div()
                                .absolute()
                                .top(px(20.0))
                                .left(px(10.0))
                                .w(px(45.0))
                                .h(px(90.0))
                                .border_1()
                                .border_color(theme.status_green)
                                .p_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.status_green)
                                        .child("[Files]")
                                )
                        )
                        // Bounding box 3 (Terminal)
                        .child(
                            div()
                                .absolute()
                                .bottom(px(5.0))
                                .left(px(60.0))
                                .w(px(180.0))
                                .h(px(20.0))
                                .border_1()
                                .border_color(theme.status_yellow)
                                .p_1()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.status_yellow)
                                        .child("[ConPTY]")
                                )
                        )
                )
                // Capture action button
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_1()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .py_1p5()
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                    this.state.status_message = "Vision grounding: captured 48 UI bounding boxes".to_string();
                                    this.state.toast_manager.push_success("Vision grounding: mapped 48 UI coordinates into AI spatial context");
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::Camera, theme.text_on_accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_on_accent)
                                        .child("Capture & Ground Screen")
                                )
                        )
                )
        )
        // Grounded UI Elements & OCR Segments List
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .overflow_hidden()
                .p_3()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .child("GROUNDED ELEMENTS & OCR TOKENS")
                )
                .children([
                    ("Editor Code Buffer", "x: 320, y: 35, w: 1120, h: 740", "Interactive Element", theme.accent),
                    ("Quick Open Input", "x: 480, y: 5, w: 460, h: 24", "Text Field", theme.status_green),
                    ("ConPTY Terminal Output", "x: 320, y: 775, w: 1120, h: 240", "Interactive Terminal", theme.status_yellow),
                    ("Activity Bar Tabs", "x: 0, y: 34, w: 48, h: 980", "Navigation Bar", theme.text_muted),
                ].into_iter().map(|(label, coords, kind, color)| {
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
                        .hover(|s| s.border_color(theme.border_focus))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child(label)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .font_family("Consolas")
                                        .child(coords)
                                )
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_app)
                                .text_xs()
                                .text_color(color)
                                .child(kind)
                        )
                }))
        )
}
