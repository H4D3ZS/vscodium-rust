// Reusable GPUI UI component primitives.
// Provides factory functions for common UI elements used across all panels:
// badge_pill, section_header, icon_button, toggle_button, separator, card_container.
// This eliminates hundreds of lines of duplicated styling across panel modules.
#![allow(dead_code, unused_variables)]

use crate::theme::Theme;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use gpui_kit::gpui::*;

// ─── Glow & Shadows ─────────────────────────────────────────────────────────
// Single soft box-shadow for hover/active glows. Feed via `.shadow(glow_shadow(..))`.

pub fn glow_shadow(color: impl Into<Hsla>, offset_y: f32, blur: f32) -> Vec<BoxShadow> {
    vec![BoxShadow::new(px(0.), px(offset_y), color.into()).blur_radius(px(blur))]
}

pub fn drop_shadow(shadow_soft: impl Into<Hsla>) -> Vec<BoxShadow> {
    vec![BoxShadow::new(px(0.), px(4.), shadow_soft.into()).blur_radius(px(12.))]
}

// ─── Badge Pill ─────────────────────────────────────────────────────────────
// A small colored badge (e.g., "3" for git changes, "Auto" for mode indicators).

pub fn badge_pill(label: &str, bg: Rgba, fg: Rgba) -> impl IntoElement {
    div()
        .px_1p5()
        .py_0p5()
        .rounded(px(3.5))
        .bg(bg)
        .text_color(fg)
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .child(label.to_string())
}

pub fn badge_pill_sm(label: &str, bg: Rgba, fg: Rgba) -> impl IntoElement {
    div()
        .px_1()
        .rounded(px(3.0))
        .bg(bg)
        .text_color(fg)
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .child(label.to_string())
}

// ─── Status Dot ─────────────────────────────────────────────────────────────
// A small colored circle indicator (green = active, yellow = warning, etc.)

pub fn status_dot(color: Rgba) -> impl IntoElement {
    div().w(px(8.0)).h(px(8.0)).rounded_full().bg(color)
}

// ─── Pulsing Status Dot ─────────────────────────────────────────────────────
// "Alive" indicator: a filled dot with a faint outer ring + soft glow halo,
// mirroring the TS `.activity-dot--running` + hubPulse treatment.

pub fn pulse_dot(color: Rgba) -> impl IntoElement {
    div()
        .relative()
        .flex()
        .items_center()
        .justify_center()
        .w(px(12.0))
        .h(px(12.0))
        .child(
            div()
                .absolute()
                .size_full()
                .rounded_full()
                .border_1()
                .border_color(color)
                .opacity(0.45),
        )
        .child(
            div()
                .absolute()
                .size_full()
                .rounded_full()
                .bg(color),
        )
}

// ─── Section Header ─────────────────────────────────────────────────────────
// Collapsible section header matching VSCode/Cursor sidebar sections.

pub fn section_header(title: &str, is_expanded: bool, theme: &Theme) -> impl IntoElement {
    let chevron = if is_expanded { "▾" } else { "▸" };

    div()
        .flex()
        .flex_row()
        .items_center()
        .h(px(24.0))
        .px_2()
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .mr_1()
                .child(chevron.to_string()),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_muted)
                .child(title.to_string()),
        )
}

pub fn section_header_with_actions(
    title: &str,
    is_expanded: bool,
    theme: &Theme,
    actions: Vec<(IconName, Rgba)>,
) -> impl IntoElement {
    let chevron = if is_expanded { "▾" } else { "▸" };

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .h(px(24.0))
        .px_2()
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .mr_1()
                        .child(chevron.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child(title.to_string()),
                ),
        )
        .child(
            div().flex().flex_row().items_center().gap_1().children(
                actions
                    .into_iter()
                    .map(|(icon, color)| icon_button_sm(icon, color, theme)),
            ),
        )
}

// ─── Icon Button ────────────────────────────────────────────────────────────
// Clickable icon with hover background. Used for toolbar actions.

pub fn icon_button(icon: IconName, color: Rgba, theme: &Theme) -> impl IntoElement {
    div()
        .p_1()
        .rounded(px(3.0))
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .child(icon_14(icon, color))
}

pub fn icon_button_sm(icon: IconName, color: Rgba, theme: &Theme) -> impl IntoElement {
    div()
        .p_0p5()
        .rounded(px(2.0))
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .child(icon_12(icon, color))
}

pub fn icon_button_lg(icon: IconName, size: f32, color: Rgba, theme: &Theme) -> impl IntoElement {
    div()
        .p_1p5()
        .rounded(px(4.0))
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .child(ui_icon(icon, size, color))
}

// ─── Toggle Button / Pill ───────────────────────────────────────────────────
// A toggle pill that shows active/inactive state (e.g., "YOLO", "Think", "AUTO").

pub fn toggle_pill(
    label: &str,
    is_active: bool,
    active_bg: Rgba,
    active_fg: Rgba,
    theme: &Theme,
) -> impl IntoElement {
    div()
        .px_2()
        .py_0p5()
        .rounded(px(4.0))
        .cursor_pointer()
        .bg(if is_active { active_bg } else { theme.pill_bg })
        .border_1()
        .border_color(if is_active {
            theme.ring_focus
        } else {
            theme.pill_border
        })
        .text_color(if is_active {
            active_fg
        } else {
            theme.text_muted
        })
        .text_xs()
        .font_weight(if is_active {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .hover(|s| {
            s.bg(if is_active {
                active_bg
            } else {
                theme.pill_hover_bg
            })
            .border_color(if is_active {
                theme.ring_focus
            } else {
                theme.pill_hover_border
            })
        })
        .child(label.to_string())
}

// ─── Tab Item ───────────────────────────────────────────────────────────────
// Styled tab for tab strips (bottom panel, right sidebar, etc.)

pub fn tab_item(label: &str, is_active: bool, theme: &Theme) -> impl IntoElement {
    div()
        .px_2()
        .py_1()
        .cursor_pointer()
        .border_b_2()
        .border_color(if is_active {
            theme.accent
        } else {
            theme.bg_titlebar
        })
        .text_xs()
        .font_weight(if is_active {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .text_color(if is_active {
            theme.text_primary
        } else {
            theme.text_muted
        })
        .hover(|s| s.text_color(theme.text_primary))
        .child(label.to_string())
}

// ─── Separator / Divider ────────────────────────────────────────────────────

pub fn horizontal_separator(theme: &Theme) -> impl IntoElement {
    div().w_full().h(px(1.0)).bg(theme.border_subtle)
}

// ─── Card Container ─────────────────────────────────────────────────────────
// Rounded card with border, used for mission buttons, agent cards, etc.

pub fn card(theme: &Theme) -> Div {
    div()
        .p_3()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
}

pub fn card_hover(theme: &Theme) -> Div {
    div()
        .p_3()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .cursor_pointer()
        .hover(|s| {
            s.bg(theme.bg_hover)
                .border_color(theme.ring_focus)
        })
}

// ─── Glass Card ─────────────────────────────────────────────────────────────
// Translucent raised card with a bordered surface and soft drop shadow —
// TS `.activity-card` / overlay parity.

pub fn glass_card(theme: &Theme) -> Div {
    div()
        .p_3()
        .rounded(px(8.0))
        .bg(theme.bg_glass_card)
        .border_1()
        .border_color(theme.border_glass)
        .shadow(drop_shadow(theme.shadow_soft))
}

// ─── Accent Gradient Pill ───────────────────────────────────────────────────
// Primary/hero action pill using the accent gradient + glow. Callers attach
// their own click handler and content is appended via a child call.

pub fn accent_pill(label: &str, theme: &Theme) -> impl IntoElement {
    let bg = linear_gradient(
        180.0,
        linear_color_stop(theme.accent_gradient_from, 0.0),
        linear_color_stop(theme.accent_gradient_to, 1.0),
    );
    div()
        .px_2p5()
        .py_0p5()
        .rounded(px(4.0))
        .bg(bg)
        .border_1()
        .border_color(theme.accent_hover)
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_on_accent)
        .child(label.to_string())
}

// ─── Input Box ──────────────────────────────────────────────────────────────
// Styled text input container matching VSCode/Cursor input fields.

pub fn input_box(placeholder: &str, value: &str, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .h(px(28.0))
        .px_2()
        .rounded(px(4.0))
        .bg(theme.bg_input)
        .border_1()
        .border_color(theme.border_subtle)
        .text_xs()
        .text_color(if value.is_empty() {
            theme.text_subtle
        } else {
            theme.text_primary
        })
        .child(if value.is_empty() {
            placeholder.to_string()
        } else {
            value.to_string()
        })
}

// ─── Progress Bar ───────────────────────────────────────────────────────────

pub fn progress_bar(fraction: f32, color: Rgba, theme: &Theme) -> impl IntoElement {
    let pct = (fraction.clamp(0.0, 1.0) * 100.0) as i32;
    div()
        .w_full()
        .h(px(4.0))
        .rounded_full()
        .bg(theme.bg_input)
        .child(
            div()
                .h_full()
                .rounded_full()
                .bg(color)
                .w(relative(fraction)),
        )
}

// ─── Empty State ────────────────────────────────────────────────────────────
// Centered placeholder text for empty panels.

pub fn empty_state(message: &str, theme: &Theme) -> impl IntoElement {
    div()
        .flex()
        .items_center()
        .justify_center()
        .size_full()
        .text_xs()
        .text_color(theme.text_subtle)
        .child(message.to_string())
}

// ─── Key Binding Label ──────────────────────────────────────────────────────
// Styled keyboard shortcut label (e.g., "Ctrl+S")

pub fn keybind_label(keys: &str, theme: &Theme) -> impl IntoElement {
    div()
        .px_1p5()
        .py_0p5()
        .rounded_sm()
        .bg(theme.bg_editor)
        .border_1()
        .border_color(theme.border_subtle)
        .text_xs()
        .font_family("Cascadia Code, Consolas, monospace")
        .text_color(theme.text_subtle)
        .child(keys.to_string())
}
