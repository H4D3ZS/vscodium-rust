use crate::app_state::HadesNativeState;
use crate::domain::canvas::*;
use crate::panels::traits::WorkbenchPanel;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub struct CanvasesPanel;

impl WorkbenchPanel for CanvasesPanel {
    fn id(&self) -> &'static str {
        "canvases"
    }

    fn title(&self) -> &'static str {
        "Canvases"
    }

    fn icon(&self) -> IconName {
        IconName::LayoutDashboard
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_canvases_panel(state, cx).into_any_element()
    }
}

pub fn render_canvases_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let count = state.canvases.len();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_2p5()
                .h(px(38.0))
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(IconName::LayoutDashboard, 15.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("CANVASES"),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.text_muted)
                        .child(count.to_string()),
                ),
        )
        .child(
            div()
                .id("canvases_list_scroll")
                .flex_1()
                .flex()
                .flex_col()
                .gap_1p5()
                .p_2()
                .overflow_y_scroll()
                .children(
                    state
                        .canvases
                        .iter()
                        .map(|cv| canvas_list_item(state, cv, cx)),
                ),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .px_2p5()
                .py_2()
                .border_t_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(theme.accent)
                        .hover(|s| s.bg(theme.accent_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.toast_manager.push_info(
                                    "Drop a .canvas.json into .agent/canvases, then hit Refresh",
                                );
                                cx.notify();
                            }),
                        )
                        .child("New"),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .hover(|s| s.bg(theme.bg_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                let before = this.state.canvases.len();
                                this.state.refresh_canvases();
                                let added = this.state.canvases.len() - before;
                                if added > 0 {
                                    this.state
                                        .toast_manager
                                        .push_success(&format!("Loaded {added} new canvas(es)"));
                                } else {
                                    this.state
                                        .toast_manager
                                        .push_info("No new .canvas.json files found");
                                }
                                cx.notify();
                            }),
                        )
                        .child("Refresh"),
                ),
        )
}

fn canvas_list_item(
    state: &HadesNativeState,
    cv: &CanvasSpec,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let id = cv.id.clone();
    let id_for_open = id.clone();
    let title = cv.title.clone();
    let subtitle = cv.subtitle.clone().unwrap_or_default();
    let block_count = cv.blocks.len();
    let is_active = state
        .active_tab()
        .map(|t| t.path == format!("canvas://{id}"))
        .unwrap_or(false);

    div()
        .flex()
        .flex_col()
        .gap_1()
        .p_2p5()
        .rounded(px(6.0))
        .bg(if is_active {
            theme.bg_active
        } else {
            theme.bg_card
        })
        .border_1()
        .border_color(if is_active {
            theme.accent
        } else {
            theme.border_subtle
        })
        .cursor_pointer()
        .hover(|s| s.bg(theme.bg_hover))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _event, _window, cx| {
                this.state.open_canvas(&id_for_open);
                this.state.focused_panel = crate::app_state::FocusedPanel::Editor;
                cx.notify();
            }),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .child(ui_icon(IconName::LayoutTemplate, 14.0, theme.accent))
                .child(
                    div()
                        .flex_1()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(title),
                )
                .child(icon_14(
                    IconName::ChevronRight,
                    if is_active {
                        theme.accent
                    } else {
                        theme.text_subtle
                    },
                )),
        )
        .children((!subtitle.is_empty()).then(|| {
            div()
                .text_xs()
                .text_color(theme.text_subtle)
                .child(subtitle)
        }))
        .child(
            div()
                .text_xs()
                .font_family("Cascadia Code, Consolas, monospace")
                .text_color(theme.text_muted)
                .child(format!("{block_count} blocks Â· canvas://{id}")),
        )
        .into_any_element()
}

pub fn render_canvas_view(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let canvas = state.active_tab().and_then(|t| {
        t.path
            .strip_prefix("canvas://")
            .and_then(|id| state.canvases.iter().find(|c| c.id == id))
    });

    let Some(canvas) = canvas else {
        return div()
            .flex_1()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(theme.bg_editor)
            .text_color(theme.text_muted)
            .text_sm()
            .child("Canvas not found")
            .into_any_element();
    };

    let title = canvas.title.clone();
    let subtitle = canvas
        .subtitle
        .clone()
        .unwrap_or_else(|| format!("{} blocks", canvas.blocks.len()));
    let canvas_id = canvas.id.clone();
    let blocks: Vec<AnyElement> = canvas
        .blocks
        .iter()
        .map(|b| canvas_block(theme, b))
        .collect();

    div()
        .flex_1()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        .overflow_hidden()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .px_3()
                .h(px(38.0))
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(ui_icon(IconName::LayoutDashboard, 16.0, theme.accent))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(title),
                )
                .child(div().text_xs().text_color(theme.text_muted).child(subtitle))
                .child(
                    div().flex_1().flex().justify_end().child(
                        div()
                            .px_1p5()
                            .py_0p5()
                            .rounded(px(3.0))
                            .bg(theme.bg_raised)
                            .text_xs()
                            .font_family("Cascadia Code, Consolas, monospace")
                            .text_color(theme.status_green)
                            .child("LIVE"),
                    ),
                ),
        )
        .child(
            div()
                .id("canvas_body_scroll")
                .flex_1()
                .flex()
                .flex_col()
                .gap_2p5()
                .p_3()
                .overflow_y_scroll()
                .children(blocks),
        )
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_3()
                .py_1p5()
                .border_t_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .font_family("Cascadia Code, Consolas, monospace")
                        .text_color(theme.text_muted)
                        .child(format!("canvas://{canvas_id}")),
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(4.0))
                        .cursor_pointer()
                        .bg(theme.bg_raised)
                        .hover(|s| s.bg(theme.bg_hover))
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.accent)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                if let Some(cv) =
                                    this.state.canvases.iter().find(|c| c.id == canvas_id)
                                {
                                    let json = spec_to_json(cv);
                                    let stash =
                                        this.state.workspace_root.join(".agent").join("canvases");
                                    let _ = std::fs::create_dir_all(&stash);
                                    let spec_path = stash.join(format!("{}.spec.json", cv.id));
                                    if std::fs::write(&spec_path, &json).is_ok() {
                                        this.state.open_file(&spec_path.to_string_lossy());
                                    }
                                }
                                this.state.toast_manager.push_info(
                                    "Open the .canvas.json in Explorer to edit the spec",
                                );
                                cx.notify();
                            }),
                        )
                        .child("View Spec JSON"),
                ),
        )
        .into_any_element()
}

pub fn canvas_block(theme: &crate::theme::Theme, block: &CanvasBlock) -> AnyElement {
    match block {
        CanvasBlock::Stats { items } => div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .children(items.iter().map(|item| {
                let (value_color, dot_color) = tone_colors(theme, item.tone.as_ref());
                div()
                    .flex_1()
                    .min_w(px(130.0))
                    .flex()
                    .flex_col()
                    .gap_1()
                    .p_2p5()
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_1p5()
                            .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(dot_color))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child(item.label.to_string()),
                            ),
                    )
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::BOLD)
                            .text_color(value_color)
                            .child(item.value.to_string()),
                    )
                    .children(item.hint.as_ref().map(|h| {
                        div()
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .child(h.to_string())
                    }))
            }))
            .into_any_element(),
        CanvasBlock::Table {
            title,
            columns,
            rows,
        } => {
            let title = title.clone();
            let columns = columns.clone();
            let row_vals: Vec<Vec<String>> = rows
                .iter()
                .map(|r| {
                    columns
                        .iter()
                        .enumerate()
                        .map(|(i, _)| r.get(i).cloned().unwrap_or_default())
                        .collect()
                })
                .collect();
            block_shell(theme, title.as_deref(), None, move |theme| {
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(
                        columns
                            .iter()
                            .enumerate()
                            .map(|(ci, col)| {
                                let col = col.to_string();
                                let cells = row_vals
                                    .iter()
                                    .map(|row| row.get(ci).cloned().unwrap_or_default())
                                    .collect::<Vec<_>>();
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_0p5()
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(col),
                                    )
                                    .child(div().flex().flex_col().children(cells.into_iter().map(
                                        |c| {
                                            div()
                                                .text_xs()
                                                .py_0p5()
                                                .text_color(theme.text_muted)
                                                .child(c)
                                                .border_b_1()
                                                .border_color(theme.border_subtle)
                                                .into_any_element()
                                        },
                                    )))
                                    .into_any_element()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .into_any_element()
            })
            .into_any_element()
        }
        CanvasBlock::Chart {
            chart,
            title,
            labels,
            series,
        } => {
            let kind = *chart;
            let title = title.clone();
            let labels = labels.clone();
            let series = series.clone();
            block_shell(theme, title.as_deref(), None, move |theme| match kind {
                CanvasChartKind::Bar | CanvasChartKind::Line => {
                    render_bar_chart(theme, &labels, &series).into_any_element()
                }
                CanvasChartKind::Pie => {
                    render_pie_chart(theme, &labels, &series).into_any_element()
                }
            })
            .into_any_element()
        }
        CanvasBlock::Markdown { content } => {
            let content = content.clone();
            block_shell(theme, Some("Markdown"), None, move |theme| {
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .children(content.lines().map(|l| {
                        div()
                            .text_sm()
                            .text_color(theme.text_primary)
                            .child(l.to_string())
                    }))
                    .into_any_element()
            })
            .into_any_element()
        }
        CanvasBlock::Callout {
            tone,
            title,
            content,
        } => {
            let title = title.clone();
            let content = content.clone();
            let tone = *tone;
            let (accent_color, icon) = callout_tone(theme, &tone);
            div()
                .flex()
                .flex_col()
                .gap_1p5()
                .p_2p5()
                .rounded(px(6.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(accent_color)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(icon, 15.0, accent_color))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(title.unwrap_or_else(|| "Note".to_string())),
                        ),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(theme.text_primary)
                        .child(content),
                )
                .into_any_element()
        }
        CanvasBlock::Progress { title, items } => {
            let title = title.clone();
            let items = items.clone();
            block_shell(theme, title.as_deref(), None, move |theme| {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(items.into_iter().map(|item| {
                        let max = item.max.unwrap_or(100.0).max(0.001);
                        let pct = (item.value / max).clamp(0.0, 1.0);
                        let (fill, _) = tone_colors(theme, item.tone.as_ref());
                        div()
                            .flex()
                            .flex_col()
                            .gap_0p5()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .justify_between()
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_primary)
                                            .child(item.label.to_string()),
                                    )
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .text_color(theme.text_muted)
                                            .child(format!("{:.0}%", pct * 100.0)),
                                    ),
                            )
                            .child(
                                div().h(px(6.0)).rounded_full().bg(theme.bg_raised).child(
                                    div()
                                        .h_full()
                                        .rounded_full()
                                        .bg(fill)
                                        .w(relative(pct as f32)),
                                ),
                            )
                            .into_any_element()
                    }))
                    .into_any_element()
            })
            .into_any_element()
        }
        CanvasBlock::Todo { title, items } => {
            let title = title.clone();
            let items = items.clone();
            block_shell(theme, title.as_deref(), None, move |theme| {
                div()
                    .flex()
                    .flex_col()
                    .gap_1p5()
                    .children(items.into_iter().map(|item| {
                        let done = item.done;
                        let text = item.text.to_string();
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .w(px(16.0))
                                    .h(px(16.0))
                                    .rounded(px(3.0))
                                    .bg(if done {
                                        theme.status_green
                                    } else {
                                        theme.bg_raised
                                    })
                                    .border_1()
                                    .border_color(if done {
                                        theme.status_green
                                    } else {
                                        theme.border_subtle
                                    })
                                    .children(
                                        done.then(|| icon_12(IconName::Check, rgb(0xffffff))),
                                    ),
                            )
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(if done {
                                        theme.text_muted
                                    } else {
                                        theme.text_primary
                                    })
                                    .child(text),
                            )
                            .into_any_element()
                    }))
                    .into_any_element()
            })
            .into_any_element()
        }
        CanvasBlock::Kv { title, pairs } => {
            let title = title.clone();
            let pairs = pairs.clone();
            block_shell(
                theme,
                title.as_deref(),
                Some(IconName::Info),
                move |theme| {
                    let pair_count = pairs.len();
                    div()
                        .flex()
                        .flex_col()
                        .children(
                            pairs
                                .into_iter()
                                .enumerate()
                                .map(|(i, p)| {
                                    let is_last = i + 1 == pair_count;
                                    div()
                                        .flex()
                                        .flex_row()
                                        .justify_between()
                                        .py_1()
                                        .border_b_1()
                                        .border_color(if is_last {
                                            rgba(0x00000000)
                                        } else {
                                            theme.border_subtle
                                        })
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_subtle)
                                                .child(p.key.to_string()),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child(p.value.to_string()),
                                        )
                                        .into_any_element()
                                })
                                .collect::<Vec<_>>(),
                        )
                        .into_any_element()
                },
            )
            .into_any_element()
        }
        CanvasBlock::Code {
            title,
            language,
            content,
        } => {
            let title = title.clone();
            let language = language.clone().unwrap_or_else(|| "text".to_string());
            let content = content.clone();
            block_shell(
                theme,
                title.as_deref(),
                Some(IconName::Brackets),
                move |theme| {
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_subtle)
                                .child(language),
                        )
                        .child(
                            div()
                                .id("canvas_code_block_scroll")
                                .px_2p5()
                                .py_1()
                                .overflow_x_scroll()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .children(content.lines().map(|l| {
                                    div()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .whitespace_nowrap()
                                        .child(l.to_string())
                                })),
                        )
                        .into_any_element()
                },
            )
            .into_any_element()
        }
        CanvasBlock::Timeline { title, items } => {
            let title = title.clone();
            let items = items.clone();
            block_shell(theme, title.as_deref(), None, move |theme| {
                div()
                    .flex()
                    .flex_col()
                    .children(
                        items
                            .iter()
                            .enumerate()
                            .map(|(i, item)| {
                                let is_last = i + 1 == items.len();
                                let (dot_color, status_label) =
                                    timeline_tone(theme, item.status.as_deref().unwrap_or(""));
                                let show_detail = item.detail.clone().unwrap_or_default();
                                div()
                                    .flex()
                                    .flex_row()
                                    .gap_2()
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .child(
                                                div()
                                                    .w(px(10.0))
                                                    .h(px(10.0))
                                                    .rounded_full()
                                                    .bg(dot_color),
                                            )
                                            .children((!is_last).then(|| {
                                                div().w_px().flex_1().bg(theme.border_subtle)
                                            })),
                                    )
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .gap_0p5()
                                            .pb_2()
                                            .child(
                                                div()
                                                    .flex()
                                                    .items_center()
                                                    .gap_2()
                                                    .child(
                                                        div()
                                                            .text_sm()
                                                            .font_weight(FontWeight::MEDIUM)
                                                            .text_color(theme.text_primary)
                                                            .child(item.title.to_string()),
                                                    )
                                                    .children((!status_label.is_empty()).then(
                                                        || {
                                                            div()
                                                                .px_1p5()
                                                                .py_0p5()
                                                                .rounded_full()
                                                                .bg(theme.bg_raised)
                                                                .text_xs()
                                                                .text_color(dot_color)
                                                                .child(status_label)
                                                        },
                                                    )),
                                            )
                                            .children((!show_detail.is_empty()).then(|| {
                                                div()
                                                    .text_xs()
                                                    .text_color(theme.text_subtle)
                                                    .child(show_detail)
                                            })),
                                    )
                                    .into_any_element()
                            })
                            .collect::<Vec<_>>(),
                    )
                    .into_any_element()
            })
            .into_any_element()
        }
    }
}

fn block_shell(
    theme: &crate::theme::Theme,
    title: Option<&str>,
    icon: Option<IconName>,
    body: impl FnOnce(&crate::theme::Theme) -> AnyElement,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .p_2p5()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .children(title.map(|t| {
            (div())
                .flex()
                .items_center()
                .gap_2()
                .children(icon.map(|i| ui_icon(i, 14.0, theme.accent)))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child(t.to_string()),
                )
        }))
        .child(body(theme))
}

fn render_bar_chart(
    theme: &crate::theme::Theme,
    labels: &[String],
    series: &[CanvasChartSeries],
) -> impl IntoElement {
    let max_val = series
        .iter()
        .flat_map(|s| s.values.iter().copied())
        .fold(0.0, f64::max)
        .max(1.0);

    div()
        .flex()
        .flex_col()
        .gap_2()
        .children(series.iter().enumerate().map(|(si, s)| {
            let (bar_color, _) = tone_colors(theme, s.tone.as_ref());
            let series_name = s
                .name
                .clone()
                .unwrap_or_else(|| format!("series {}", si + 1));
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .child(series_name),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_end()
                        .gap_2()
                        .h(px(90.0))
                        .children(s.values.iter().enumerate().map(|(i, v)| {
                            let pct = (v / max_val) as f32;
                            let label = labels.get(i).cloned().unwrap_or_default();
                            div()
                                .flex_1()
                                .flex()
                                .flex_col()
                                .items_center()
                                .gap_1()
                                .h_full()
                                .child(
                                    div().flex_1().flex().items_end().child(
                                        div()
                                            .w_full()
                                            .rounded(px(2.0))
                                            .bg(bar_color)
                                            .h(relative(pct.max(0.03))),
                                    ),
                                )
                                .child(div().text_xs().text_color(theme.text_muted).child(label))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_color(theme.text_primary)
                                        .child(format!("{v:.1}")),
                                )
                                .into_any_element()
                        })),
                )
                .into_any_element()
        }))
        .into_any_element()
}

fn render_pie_chart(
    theme: &crate::theme::Theme,
    labels: &[String],
    series: &[CanvasChartSeries],
) -> impl IntoElement {
    let total: f64 = series.iter().flat_map(|s| s.values.iter()).sum();
    let total = if total > 0.0 { total } else { 1.0 };
    let n = labels.len().max(1);
    let palette = tone_palette();

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div().flex().flex_row().child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(render_donut(theme, &palette, &series, total))
                    .child(div().flex().flex_col().gap_1().children(
                        series.iter().enumerate().take(n).map(|(i, s)| {
                            let (dot, _) = palette_tone(&palette, i);
                            let pct = s.values.iter().sum::<f64>() / total * 100.0;
                            let name = s.name.clone().unwrap_or_else(|| format!("slice {}", i + 1));
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(div().w(px(8.0)).h(px(8.0)).rounded(px(2.0)).bg(dot))
                                .child(div().text_xs().text_color(theme.text_primary).child(name))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_color(theme.text_muted)
                                        .child(format!("{pct:.1}%")),
                                )
                                .into_any_element()
                        }),
                    )),
            ),
        )
        .into_any_element()
}

fn render_donut(
    theme: &crate::theme::Theme,
    palette: &[Rgba],
    series: &[CanvasChartSeries],
    total: f64,
) -> AnyElement {
    let mut radius = 36.0_f32;

    let rings: Vec<AnyElement> = series
        .iter()
        .enumerate()
        .filter(|(_, s)| !s.values.is_empty())
        .enumerate()
        .map(|(ring_i, (i, _))| {
            let (color, _) = palette_tone(palette, i);
            let r = radius - ring_i as f32 * 8.0;
            radius = r;
            div()
                .absolute()
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .rounded_full()
                        .size(px(r * 2.0))
                        .border_4()
                        .border_color(color)
                        .child(
                            div()
                                .absolute()
                                .inset_0()
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(0x00000000))
                                .child(div().absolute().inset_0()),
                        ),
                )
                .into_any_element()
        })
        .collect();

    div()
        .flex()
        .flex_col()
        .gap_3()
        .items_center()
        .child(
            div()
                .relative()
                .size(px((radius + 8.0) * 2.0))
                .child(
                    div()
                        .absolute()
                        .inset_0()
                        .rounded_full()
                        .bg(theme.bg_raised),
                )
                .children(rings)
                .child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_xs()
                                .font_family("Cascadia Code, Consolas, monospace")
                                .text_color(theme.text_primary)
                                .child(format!("{:.0}%", (total * 100.0).round())),
                        ),
                ),
        )
        .into_any_element()
}

fn tone_colors(theme: &crate::theme::Theme, tone: Option<&CanvasTone>) -> (Rgba, Rgba) {
    match tone {
        Some(CanvasTone::Success) => (theme.status_green, theme.status_green),
        Some(CanvasTone::Warning) => (theme.status_yellow, theme.status_yellow),
        Some(CanvasTone::Danger) => (theme.status_red, theme.status_red),
        Some(CanvasTone::Info) => (rgb(0x64b5f6), rgb(0x64b5f6)),
        Some(CanvasTone::Accent) => (theme.accent, theme.accent),
        _ => (theme.text_primary, theme.accent),
    }
}

fn callout_tone(theme: &crate::theme::Theme, tone: &CanvasTone) -> (Rgba, IconName) {
    match tone {
        CanvasTone::Success => (theme.status_green, IconName::CircleCheck),
        CanvasTone::Warning => (theme.status_yellow, IconName::TriangleAlert),
        CanvasTone::Danger => (theme.status_red, IconName::CircleX),
        CanvasTone::Info => (rgb(0x64b5f6), IconName::Info),
        CanvasTone::Accent => (theme.accent, IconName::Sparkles),
        CanvasTone::Neutral => (theme.text_muted, IconName::Activity),
    }
}

fn timeline_tone(theme: &crate::theme::Theme, status: &str) -> (Rgba, String) {
    match status {
        "done" | "complete" => (theme.status_green, "done".to_string()),
        "active" | "running" | "progress" => (theme.accent, "active".to_string()),
        "warning" | "failed" | "error" => (theme.status_red, "warning".to_string()),
        _ => (theme.text_muted, "pending".to_string()),
    }
}

fn tone_palette() -> Vec<Rgba> {
    vec![
        rgb(0x007acc),
        rgb(0x64b5f6),
        rgb(0x81c784),
        rgb(0xffb74d),
        rgb(0xe57373),
        rgb(0xce93d8),
        rgb(0x4dd0e1),
        rgb(0xaed581),
    ]
}

fn palette_tone(palette: &[Rgba], i: usize) -> (Rgba, Rgba) {
    let c = palette[i % palette.len()];
    (c, c)
}
