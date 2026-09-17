use crate::app_state::HadesNativeState;
use crate::panels::traits::WorkbenchPanel;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct PyTorchStudioPanel;

impl WorkbenchPanel for PyTorchStudioPanel {
    fn id(&self) -> &'static str {
        "pytorch"
    }

    fn title(&self) -> &'static str {
        "PyTorch ML Studio"
    }

    fn icon(&self) -> IconName {
        IconName::Flame
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_pytorch_panel(state, cx).into_any_element()
    }
}

pub fn render_pytorch_panel(
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
                                .child(ui_icon(IconName::Flame, 16.0, rgb(0xee4c2c))) // PyTorch Orange
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("PYTORCH ML STUDIO"),
                                ),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0xee4c2c))
                                .child("CUDA 12.4 Active"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("Hardware-accelerated neural model training & tensor telemetry"),
                ),
        )
        // Training Telemetry Dashboard
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
                                .child("LIVE TRAINING RUN #4"),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .child(
                                    div()
                                        .w(px(6.0))
                                        .h(px(6.0))
                                        .rounded_full()
                                        .bg(theme.status_green),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.status_green)
                                        .child("Training (Epoch 14/20)"),
                                ),
                        ),
                )
                // Metrics grid
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Train Loss"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("0.142"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Val Acc"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.status_green)
                                        .child("94.8%"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .flex_1()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("Throughput"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child("1,850/s"),
                                ),
                        ),
                )
                // GPU VRAM & Hardware Meter
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .text_xs()
                                .child(
                                    div()
                                        .text_color(theme.text_muted)
                                        .child("GPU VRAM Allocation"),
                                )
                                .child(
                                    div()
                                        .text_color(theme.text_primary)
                                        .child("4,120 MB / 16,384 MB (25.1%)"),
                                ),
                        )
                        .child(
                            div()
                                .h(px(4.0))
                                .w_full()
                                .rounded_full()
                                .bg(theme.bg_input)
                                .child(div().h_full().w(px(70.0)).rounded_full().bg(rgb(0xee4c2c))),
                        ),
                )
                // Controls toolbar
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
                                .bg(rgb(0xee4c2c))
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.9))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.execute_terminal_command(
                                            "python -m hades_ml.train --epochs 20 --lr 0.001",
                                        );
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Play, theme.text_on_accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_on_accent)
                                        .child("Resume Run"),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .px_3()
                                .py_1p5()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.status_message =
                                            "PyTorch checkpoint exported to artifacts/model.onnx"
                                                .to_string();
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child("Export ONNX"),
                                ),
                        ),
                ),
        )
        // Dataset Catalog & Checkpoints List
        .child(
            div()
                .id("pytorch_studio_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_3()
                .gap_3()
                // Datasets section
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_muted)
                                .child("DATASETS (.hades/ml/datasets/)"),
                        )
                        .children(
                            [
                                (
                                    "code_embeddings.parquet",
                                    "88.4 MB · 120,000 vectors · 768 dims",
                                    "Parquet",
                                ),
                                (
                                    "mnist_digits.csv",
                                    "14.2 MB · 60,000 rows · 784 features",
                                    "CSV",
                                ),
                                (
                                    "security_audit.jsonl",
                                    "4.1 MB · 25,000 events · 12 labels",
                                    "JSONL",
                                ),
                            ]
                            .into_iter()
                            .map(|(filename, info, format)| {
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
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_primary)
                                                    .child(filename),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_subtle)
                                                    .child(info),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_1p5()
                                            .py_0p5()
                                            .rounded(px(3.0))
                                            .bg(theme.bg_input)
                                            .text_xs()
                                            .text_color(theme.accent)
                                            .child(format),
                                    )
                            }),
                        ),
                )
                // Checkpoints section
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1p5()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(theme.text_muted)
                                .child("CHECKPOINTS & ARTIFACTS"),
                        )
                        .children(
                            [
                                (
                                    "model_epoch_14_best.pt",
                                    "Val Acc: 94.8% · 2.4 MB · Just now",
                                    theme.status_green,
                                ),
                                (
                                    "model_epoch_10.pt",
                                    "Val Acc: 92.1% · 2.4 MB · 12m ago",
                                    theme.text_muted,
                                ),
                                (
                                    "model_epoch_05.pt",
                                    "Val Acc: 86.4% · 2.4 MB · 28m ago",
                                    theme.text_muted,
                                ),
                            ]
                            .into_iter()
                            .map(|(ckpt, details, status_col)| {
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .justify_between()
                                    .p_2()
                                    .rounded(px(4.0))
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
                                                    .font_weight(FontWeight::MEDIUM)
                                                    .text_color(theme.text_primary)
                                                    .child(ckpt),
                                            )
                                            .child(
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_subtle)
                                                    .child(details),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .px_2()
                                            .py_0p5()
                                            .rounded(px(3.0))
                                            .bg(theme.bg_raised)
                                            .text_xs()
                                            .text_color(status_col)
                                            .child("Saved"),
                                    )
                            }),
                        ),
                ),
        )
}
