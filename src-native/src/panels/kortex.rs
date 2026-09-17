use crate::app_state::HadesNativeState;
use crate::panels::traits::AuxiliaryTab;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct KortexPanel;

impl AuxiliaryTab for KortexPanel {
    fn id(&self) -> &'static str {
        "kortex"
    }

    fn title(&self) -> &'static str {
        "Kortex"
    }

    fn icon(&self) -> IconName {
        IconName::Layers
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_kortex_panel(state, cx).into_any_element()
    }
}

pub fn render_kortex_panel(
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
                                .child(ui_icon(IconName::Cpu, 14.0, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("KORTEX AIM SERVICES")
                                )
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.status_green)
                                .child("Local Daemon Active")
                        )
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("Semantic memory, vector retrieval, and prompt KV-cache accelerator")
                )
        )
        // Codebase Indexing Catalog Summary Card
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
                                .child("WORKSPACE VECTOR CATALOG")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.accent)
                                .font_weight(FontWeight::MEDIUM)
                                .child("Qwen3-0.6B Embeddings")
                        )
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .p_2()
                                .rounded(px(5.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Indexed Chunks")
                                )
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child(format!("{}", state.kortex_chunks_indexed))
                                )
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .p_2()
                                .rounded(px(5.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Vector Latency")
                                )
                                .child(
                                    div()
                                        .text_base()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.status_green)
                                        .child("0.4 ms")
                                )
                        )
                )
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
                                    this.state.status_message = "Indexing workspace with Qwen3-0.6B embeddings...".to_string();
                                    this.state.kortex_chunks_indexed += 120;
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::RotateCw, theme.text_on_accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_on_accent)
                                        .child("Re-Index Workspace")
                                )
                        )
                )
        )
        // In-Process Port Daemons
        .child(
            div()
                .id("kortex_panel_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(theme.text_muted)
                        .child("ACTIVE SERVICE PROCESSES")
                )
                // Service 1: AIM Retrieval Proxy :1536
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_raised)
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
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w(px(7.0))
                                                .h(px(7.0))
                                                .rounded_full()
                                                .bg(theme.status_green)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("AIM Retrieval Proxy")
                                        )
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_input)
                                        .text_xs()
                                        .font_family("Consolas")
                                        .text_color(theme.text_muted)
                                        .child("127.0.0.1:1536")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Routes inference requests through localized semantic context embedding chunks.")
                        )
                )
                // Service 2: KV-Slot Cache :1537
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_raised)
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
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w(px(7.0))
                                                .h(px(7.0))
                                                .rounded_full()
                                                .bg(theme.status_green)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("KV-Slot Cache Orchestrator")
                                        )
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_input)
                                        .text_xs()
                                        .font_family("Consolas")
                                        .text_color(theme.text_muted)
                                        .child("127.0.0.1:1537")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Zero-overhead prefix caching for multi-turn Agent conversations. 94.2% token reuse.")
                        )
                )
                // Service 3: AIM-VFS :1538
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .p_2p5()
                        .rounded(px(6.0))
                        .bg(theme.bg_raised)
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
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .w(px(7.0))
                                                .h(px(7.0))
                                                .rounded_full()
                                                .bg(theme.status_green)
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("AIM Virtual File System (VFS)")
                                        )
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_input)
                                        .text_xs()
                                        .font_family("Consolas")
                                        .text_color(theme.text_muted)
                                        .child("127.0.0.1:1538")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Instant memory-mapped AST file chunks for deep semantic codebase search.")
                        )
                )
        )
}
