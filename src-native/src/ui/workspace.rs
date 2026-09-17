use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::panels::{
    render_canvas_view, render_diff_viewer, render_markdown_preview, render_mobile_panel,
    render_right_sidebar_panel, DiffViewerState, PanelRegistry, ScmState, SearchState,
};
use crate::ui::activity_bar::render_activity_bar;
use crate::ui::context_menu::render_context_menu;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::ui::inline_edit::render_inline_edit_overlay;
use crate::ui::quick_open::{render_quick_open_overlay, QuickOpenState};
use crate::ui::status_bar::render_status_bar;
use crate::ui::titlebar::render_titlebar;
use crate::ui::toast::render_toast_overlay;
use crate::ui::welcome::render_branded_welcome_screen;
use crate::HadesAppView;
use gpui_kit::gpui::*;

pub fn render_workspace(
    state: &HadesNativeState,
    editor: &Entity<gpui_kit::component::input::EditorState>,
    _search: &SearchState,
    _scm: &ScmState,
    diff: &DiffViewerState,
    quick_open: &QuickOpenState,
    input_box: &crate::ui::InputBoxState,
    registry: &PanelRegistry,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .relative()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_app)
        // Top: Custom Native Titlebar
        .child(render_titlebar(state, cx))
        // Workspace Trust Warning Banner (Restricted Mode)
        .children(
            (!state.workspace_trusted && !state.workspace_trust_dismissed)
                .then(|| render_workspace_trust_banner(state, cx)),
        )
        // Center Area: Activity Bar + Left Sidebar + Main Editor Area + Right Agent Sidebar
        .child(
            div()
                .flex()
                .flex_row()
                .flex_1()
                .w_full()
                .overflow_hidden()
                // Activity Bar
                .child(render_activity_bar(state, registry, cx))
                // Collapsible Left Sidebar Panel (Polymorphic Registry Dispatch)
                .children(
                    (state.left_sidebar_open).then(|| render_left_sidebar(state, registry, cx)),
                )
                // Left Sidebar Draggable Splitter
                .children((state.left_sidebar_open).then(|| {
                    let is_resizing = state.is_resizing_sidebar;
                    div()
                        .w(px(5.0))
                        .h_full()
                        .bg(if is_resizing {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .cursor_col_resize()
                        .hover(|s| s.bg(theme.accent))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.is_resizing_sidebar = true;
                                cx.notify();
                            }),
                        )
                }))
                // Center Editor Canvas & Bottom Terminal
                .child(render_center_area(state, editor, diff, registry, cx))
                // Right Col 1 Draggable Splitter (Agent Chat)
                .children((state.right_sidebar_open).then(|| {
                    let is_resizing = state.is_resizing_right_sidebar;
                    div()
                        .w(px(5.0))
                        .h_full()
                        .bg(if is_resizing {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .cursor_col_resize()
                        .hover(|s| s.bg(theme.accent))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.is_resizing_right_sidebar = true;
                                cx.notify();
                            }),
                        )
                }))
                // Right Col 1: Cursor AI Agent Studio (Chat, Studio, Specs, AIM, Rules)
                .children((state.right_sidebar_open).then(|| render_right_sidebar(state, cx)))
                // Right Col 2 Draggable Splitter (iPhone Live Preview)
                .children((state.iphone_preview_open).then(|| {
                    let is_resizing = state.is_resizing_iphone_preview;
                    div()
                        .w(px(5.0))
                        .h_full()
                        .bg(if is_resizing {
                            theme.accent
                        } else {
                            theme.border_subtle
                        })
                        .cursor_col_resize()
                        .hover(|s| s.bg(theme.accent))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.is_resizing_iphone_preview = true;
                                cx.notify();
                            }),
                        )
                }))
                // Right Col 2: Physical iPhone Device Mirror & Simulator Preview
                .children((state.iphone_preview_open).then(|| render_iphone_preview_sidebar(state, cx))),
        )
        // Bottom: Status Bar
        .child(render_status_bar(state, cx))
        // Quick Open Floating Overlay
        .children(render_quick_open_overlay(state, quick_open, cx))
        // Interactive Input Box Overlay
        .children(crate::ui::input_box::render_input_box_overlay(
            state, input_box, cx,
        ))
        // Context Menu Overlay
        .children(render_context_menu(&state.context_menu, theme, cx))
        // Toast Notifications Overlay
        .children(render_toast_overlay(&state.toast_manager, theme))
        // Inline Edit (Ctrl+K) Overlay
        .children(render_inline_edit_overlay(&state.inline_edit, theme, cx))
        // Tool Permission Dialog (Security Overlay)
        .children(render_tool_permission_dialog(state, cx))
        // AI Cognitive Thought Process HUD Overlay
        .children(
            state
                .current_thought
                .as_ref()
                .map(|thought| crate::ui::render_thought_process_hud(state, thought, cx)),
        )
        // Local Inference Health & Performance Dashboard Overlay
        .children(
            state
                .inference_health
                .visible
                .then(|| crate::ui::render_inference_health_dashboard(state, cx)),
        )
}

fn render_left_sidebar(
    state: &HadesNativeState,
    registry: &PanelRegistry,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let panel_id = state.active_activity.id();

    div()
        .flex()
        .flex_col()
        .w(px(state.sidebar_width))
        .h_full()
        .bg(theme.bg_sidebar)
        .border_r_1()
        .border_color(theme.border_subtle)
        .child(if let Some(panel) = registry.find_primary(panel_id) {
            panel.render(state, cx)
        } else {
            div()
                .p_4()
                .text_color(theme.text_muted)
                .child(format!("Panel '{panel_id}' not loaded."))
                .into_any_element()
        })
}

fn render_center_area(
    state: &HadesNativeState,
    _editor: &Entity<gpui_kit::component::input::EditorState>,
    diff: &DiffViewerState,
    registry: &PanelRegistry,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    if state.tabs.is_empty() && !diff.is_open {
        return div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .bg(theme.bg_editor)
            .overflow_hidden()
            .child(render_branded_welcome_screen(state, cx))
            .children((state.bottom_panel_open).then(|| render_bottom_panel(state, registry, cx)));
    }

    div()
        .flex()
        .flex_col()
        .flex_1()
        .h_full()
        .bg(theme.bg_editor)
        .overflow_hidden()
        // Tab Strip (Cursor / VSCode exact parity)
        .child(
            div()
                .id("editor_tab_strip_scroll")
                .flex()
                .flex_row()
                .items_center()
                .h(px(35.0))
                .w_full()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .overflow_x_scroll()
                .px_0()
                .children(state.tabs.iter().enumerate().map(|(idx, tab)| {
                    let is_active = state.active_tab_idx == Some(idx) && !diff.is_open;
                    let is_dirty = tab.dirty;
                    let is_pinned = tab.is_pinned;
                    let tab_title = tab.title.clone();

                    let (file_icon_name, file_icon_color) =
                        if tab.is_settings || tab.path == "hades://settings" {
                            (IconName::Settings, theme.accent)
                        } else if tab.path.starts_with("canvas://") {
                            (IconName::LayoutDashboard, theme.accent)
                        } else if tab.title.ends_with(".rs") {
                            (IconName::FileCode, rgb(0xe57373))
                        } else if tab.title.ends_with(".ts")
                            || tab.title.ends_with(".tsx")
                            || tab.title.ends_with(".js")
                        {
                            (IconName::FileCode, rgb(0x64b5f6))
                        } else if tab.title.ends_with(".toml") || tab.title.ends_with(".json") {
                            (IconName::FileCode, rgb(0xffb74d))
                        } else if tab.title.ends_with(".md") {
                            (IconName::FileText, rgb(0x81c784))
                        } else {
                            (IconName::File, theme.text_subtle)
                        };

                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .h_full()
                        .cursor_pointer()
                        .bg(if is_active {
                            theme.bg_editor
                        } else {
                            theme.bg_titlebar
                        })
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        // Top accent border for active tab
                        .border_t_2()
                        .border_color(if is_active {
                            theme.accent
                        } else {
                            theme.bg_titlebar
                        })
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, window, cx| {
                                this.diff_viewer.close();
                                this.select_tab(idx, window, cx);
                                this.state.focused_panel = FocusedPanel::Editor;
                                cx.notify();
                            }),
                        )
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(move |_this, event: &MouseDownEvent, _window, cx| {
                                let x: f32 = event.position.x.into();
                                let y: f32 = event.position.y.into();
                                _this.state.context_menu.open(
                                    x,
                                    y,
                                    crate::ui::context_menu::tab_context_menu_for(is_pinned),
                                    idx.to_string(),
                                );
                                cx.notify();
                            }),
                        )
                        .child(ui_icon(file_icon_name, 13.0, file_icon_color))
                        .child(
                            div()
                                .text_xs()
                                .text_color(if is_active {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .font_weight(if is_active {
                                    FontWeight::MEDIUM
                                } else {
                                    FontWeight::NORMAL
                                })
                                .child(tab_title),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .w(px(16.0))
                                .h(px(16.0))
                                .rounded(px(3.0))
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, window, cx| {
                                        if is_pinned {
                                            this.state.unpin_tab(idx);
                                            this.state.toast_manager.push_info("Tab unpinned");
                                        } else {
                                            this.close_tab(idx, window, cx);
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(if is_dirty {
                                    div()
                                        .w(px(7.0))
                                        .h(px(7.0))
                                        .rounded_full()
                                        .bg(if is_active {
                                            theme.accent
                                        } else {
                                            theme.text_muted
                                        })
                                        .into_any_element()
                                } else if is_pinned {
                                    icon_12(IconName::Pin, theme.accent).into_any_element()
                                } else {
                                    icon_12(IconName::X, theme.text_subtle).into_any_element()
                                }),
                        )
                }))
                // Diff viewer tab
                .children(diff.is_open.then(|| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .h_full()
                        .bg(theme.bg_editor)
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .border_t_2()
                        .border_color(theme.accent)
                        .child(ui_icon(IconName::GitCompare, 13.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.text_primary)
                                .child(format!("Diff: {}", diff.file_path)),
                        )
                        .child(
                            div()
                                .p_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.diff_viewer.close();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_subtle)),
                        )
                }))
                // Markdown Preview Tab
                .children(state.markdown_preview.is_open.then(|| {
                    let active_title = state
                        .active_tab()
                        .map(|t| t.title.clone())
                        .unwrap_or_else(|| "Markdown".to_string());
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .px_3()
                        .h_full()
                        .bg(theme.bg_editor)
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .border_t_2()
                        .border_color(theme.accent)
                        .child(ui_icon(IconName::FileText, 13.0, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(theme.accent)
                                .child(format!("Preview: {}", active_title)),
                        )
                        .child(
                            div()
                                .p_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.markdown_preview.is_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_subtle)),
                        )
                }))
                // Add New File Tab action
                .child(
                    div()
                        .p_2()
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.input_box.open_new_file(None);
                                cx.notify();
                            }),
                        )
                        .child(icon_14(IconName::Plus, theme.text_subtle)),
                )
                // Split Editor Right action (Ctrl+\)
                .child(
                    div()
                        .p_2()
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.split_editor();
                                cx.notify();
                            }),
                        )
                        .child(icon_14(
                            IconName::PanelRight,
                            if state.is_split_editor {
                                theme.accent
                            } else {
                                theme.text_subtle
                            },
                        )),
                )
                // If split, Toggle Split Direction button
                .children(state.is_split_editor.then(|| {
                    div()
                        .p_2()
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.split_horizontal = !this.state.split_horizontal;
                                cx.notify();
                            }),
                        )
                        .child(icon_14(
                            IconName::PanelBottom,
                            if state.split_horizontal {
                                theme.accent
                            } else {
                                theme.text_subtle
                            },
                        ))
                }))
                // Markdown View Mode Toggle (if active file is .md)
                .children(state.active_tab().and_then(|t| {
                    let is_md = t.path.ends_with(".md") || t.path.ends_with(".markdown");
                    if !is_md {
                        return None;
                    }
                    let reader_mode = t.markdown_reader_mode;
                    let side_preview = state.markdown_preview.is_open && !reader_mode;
                    let source_mode = !reader_mode && !side_preview;
                    Some(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .h(px(24.0))
                            .rounded(px(4.0))
                            .bg(theme.bg_raised)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .p(px(2.0))
                            .gap_1()
                            // Reader Mode Button (Full Preview)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .bg(if reader_mode {
                                        theme.accent
                                    } else {
                                        rgba(0x00000000)
                                    })
                                    .text_color(if reader_mode {
                                        theme.text_on_accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .hover(|s| {
                                        if !reader_mode {
                                            s.bg(theme.bg_hover)
                                        } else {
                                            s
                                        }
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.markdown_reader_mode = true;
                                            }
                                            this.state.markdown_preview.is_open = false;
                                            cx.notify();
                                        }),
                                    )
                                    .child(ui_icon(
                                        IconName::FileText,
                                        12.0,
                                        if reader_mode {
                                            theme.text_on_accent
                                        } else {
                                            theme.text_muted
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("📖 Reader"),
                                    ),
                            )
                            // Side-by-Side Preview Button
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .bg(if side_preview {
                                        theme.accent
                                    } else {
                                        rgba(0x00000000)
                                    })
                                    .text_color(if side_preview {
                                        theme.text_on_accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .hover(|s| {
                                        if !side_preview {
                                            s.bg(theme.bg_hover)
                                        } else {
                                            s
                                        }
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.markdown_reader_mode = false;
                                            }
                                            this.state.markdown_preview.is_open = true;
                                            cx.notify();
                                        }),
                                    )
                                    .child(ui_icon(
                                        IconName::Columns2,
                                        12.0,
                                        if side_preview {
                                            theme.text_on_accent
                                        } else {
                                            theme.text_muted
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("◫ Side Preview"),
                                    ),
                            )
                            // Source Button (Pure Code)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .cursor_pointer()
                                    .bg(if source_mode {
                                        theme.accent
                                    } else {
                                        rgba(0x00000000)
                                    })
                                    .text_color(if source_mode {
                                        theme.text_on_accent
                                    } else {
                                        theme.text_muted
                                    })
                                    .hover(|s| {
                                        if !source_mode {
                                            s.bg(theme.bg_hover)
                                        } else {
                                            s
                                        }
                                    })
                                    .on_mouse_down(
                                        MouseButton::Left,
                                        cx.listener(|this, _e, _w, cx| {
                                            if let Some(tab) = this.state.active_tab_mut() {
                                                tab.markdown_reader_mode = false;
                                            }
                                            this.state.markdown_preview.is_open = false;
                                            cx.notify();
                                        }),
                                    )
                                    .child(ui_icon(
                                        IconName::Code,
                                        12.0,
                                        if source_mode {
                                            theme.text_on_accent
                                        } else {
                                            theme.text_muted
                                        },
                                    ))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::MEDIUM)
                                            .child("📝 Source"),
                                    ),
                            ),
                    )
                })),
        )
        // Center View: Diff Viewer, Markdown Reader/Side-by-Side, Split Editor, or Single Monaco Code Editor
        .child(
            div()
                .relative()
                .flex()
                .flex_1()
                .w_full()
                .h_full()
                .overflow_hidden()
                .child(if diff.is_open {
                    render_diff_viewer(state, diff, cx).into_any_element()
                } else if let Some(tab) = state.active_tab() {
                    let is_md = tab.path.ends_with(".md") || tab.path.ends_with(".markdown");
                    if tab.is_settings || tab.path == "hades://settings" {
                        crate::panels::settings::render_settings_page_view(state, cx)
                            .into_any_element()
                    } else if tab.path.starts_with("canvas://") {
                        render_canvas_view(state, cx).into_any_element()
                    } else if is_md && tab.markdown_reader_mode {
                        render_markdown_preview(state, cx).into_any_element()
                    } else if is_md && state.markdown_preview.is_open {
                        render_markdown_side_by_side(state, cx).into_any_element()
                    } else if state.is_split_editor {
                        render_split_editor_container(state, cx).into_any_element()
                    } else {
                        crate::editor::render_native_editor(state, cx).into_any_element()
                    }
                } else if state.is_split_editor {
                    render_split_editor_container(state, cx).into_any_element()
                } else {
                    crate::editor::render_native_editor(state, cx).into_any_element()
                })
                // Floating In-Editor Find Bar (Ctrl+F)
                .children((state.find_bar_open).then(|| render_in_editor_find_bar(state, cx))),
        )
        // Collapsible Bottom Panel (Interactive Terminal, Diagnostics, Problems, Output)
        .children((state.bottom_panel_open).then(|| render_bottom_panel(state, registry, cx)))
}

fn render_markdown_side_by_side(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let preview_pct = state.markdown_preview.width_pct.clamp(20.0, 75.0);
    let editor_pct = 100.0 - preview_pct;
    let is_resizing = state.markdown_preview.is_resizing;

    div()
        .flex()
        .flex_row()
        .size_full()
        .overflow_hidden()
        // Left pane: Monaco code editor
        .child(
            div()
                .w(relative(editor_pct / 100.0))
                .h_full()
                .overflow_hidden()
                .child(crate::editor::render_native_editor(state, cx)),
        )
        // Middle: Draggable Vertical Resizer
        .child(
            div()
                .w(px(4.0))
                .h_full()
                .cursor_col_resize()
                .bg(if is_resizing {
                    theme.accent
                } else {
                    theme.border_subtle
                })
                .hover(|s| s.bg(theme.accent))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.markdown_preview.is_resizing = true;
                        cx.notify();
                    }),
                ),
        )
        // Right pane: Live Markdown Preview
        .child(
            div()
                .w(relative(preview_pct / 100.0))
                .h_full()
                .overflow_hidden()
                .child(render_markdown_preview(state, cx)),
        )
}

fn render_split_editor_container(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let is_horiz = state.split_horizontal;
    let active_pane = state.active_pane;

    let base = div().relative().size_full().flex().bg(theme.bg_editor);

    let base = if is_horiz {
        base.flex_col()
    } else {
        base.flex_row()
    };

    base
        // Primary / Left Pane (Pane 0)
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .h_full()
                .overflow_hidden()
                .border_1()
                .border_color(if active_pane == 0 { theme.accent.opacity(0.3) } else { theme.border_subtle })
                .child(crate::editor::render_editor_pane(state, 0, cx))
        )
        // Splitter Divider
        .child({
            let d = div()
                .bg(theme.border_subtle)
                .hover(|s| s.bg(theme.accent))
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                    this.state.split_horizontal = !this.state.split_horizontal;
                    cx.notify();
                }));
            if is_horiz {
                d.h(px(4.0)).w_full()
            } else {
                d.w(px(4.0)).h_full()
            }
        })
        // Secondary / Right Pane (Pane 1)
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .h_full()
                .overflow_hidden()
                .border_1()
                .border_color(if active_pane == 1 { theme.accent.opacity(0.3) } else { theme.border_subtle })
                // Secondary Pane Tab Bar
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .h(px(32.0))
                        .w_full()
                        .bg(theme.bg_titlebar)
                        .border_b_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .h_full()
                                .overflow_hidden()
                                .children(state.secondary_tabs.iter().enumerate().map(|(idx, tab)| {
                                    let is_active = state.active_secondary_tab_idx == Some(idx);
                                    let is_dirty = tab.dirty;
                                    let tab_title = tab.title.clone();

                                    let (file_icon_name, file_icon_color) = if tab.title.ends_with(".rs") {
                                        (IconName::FileCode, rgb(0xe57373))
                                    } else if tab.title.ends_with(".ts") || tab.title.ends_with(".tsx") || tab.title.ends_with(".js") {
                                        (IconName::FileCode, rgb(0x64b5f6))
                                    } else if tab.title.ends_with(".toml") || tab.title.ends_with(".json") {
                                        (IconName::FileCode, rgb(0xffb74d))
                                    } else if tab.title.ends_with(".md") {
                                        (IconName::FileText, rgb(0x81c784))
                                    } else {
                                        (IconName::File, theme.text_subtle)
                                    };

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .px_3()
                                        .h_full()
                                        .cursor_pointer()
                                        .bg(if is_active { theme.bg_editor } else { theme.bg_titlebar })
                                        .border_r_1()
                                        .border_color(theme.border_subtle)
                                        .border_t_2()
                                        .border_color(if is_active { theme.accent } else { theme.bg_titlebar })
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, window, cx| {
                                            this.select_secondary_tab(idx, window, cx);
                                        }))
                                        .on_mouse_down(MouseButton::Right, cx.listener(move |_this, event: &MouseDownEvent, _window, cx| {
                                            let x: f32 = event.position.x.into();
                                            let y: f32 = event.position.y.into();
                                            _this.state.context_menu.open(x, y, crate::ui::context_menu::tab_context_menu(), idx.to_string());
                                            cx.notify();
                                        }))
                                        .child(ui_icon(file_icon_name, 13.0, file_icon_color))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if is_active { theme.text_primary } else { theme.text_muted })
                                                .font_weight(if is_active { FontWeight::MEDIUM } else { FontWeight::NORMAL })
                                                .child(tab_title)
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .w(px(16.0))
                                                .h(px(16.0))
                                                .rounded(px(3.0))
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _e, window, cx| {
                                                    this.close_secondary_tab(idx, window, cx);
                                                }))
                                                .child(
                                                    if is_dirty {
                                                        div()
                                                            .w(px(7.0))
                                                            .h(px(7.0))
                                                            .rounded_full()
                                                            .bg(if is_active { theme.accent } else { theme.text_muted })
                                                            .into_any_element()
                                                    } else {
                                                        icon_12(IconName::X, theme.text_subtle).into_any_element()
                                                    }
                                                )
                                        )
                                        .into_any_element()
                                }))
                        )
                        .child(
                            // Close Split Pane Button
                            div()
                                .px_2()
                                .py_1()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.close_split_editor();
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::X, theme.text_subtle))
                        )
                )
                // Secondary Editor Content
                .child(crate::editor::render_editor_pane(state, 1, cx))
        )
}

fn render_in_editor_find_bar(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .absolute()
        .top(px(8.0))
        .right(px(16.0))
        .w(px(380.0))
        .p_2()
        .rounded(px(6.0))
        .bg(theme.bg_raised)
        .border_1()
        .border_color(theme.border_focus)
        .flex()
        .flex_col()
        .gap_1p5()
        // Row 1: Find input, match counter, navigation & toggles
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2()
                // Replace Toggle Chevron
                .child(
                    div()
                        .p_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _e, _w, cx| {
                                this.state.find_replace_mode = !this.state.find_replace_mode;
                                cx.notify();
                            }),
                        )
                        .child(if state.find_replace_mode {
                            icon_12(IconName::ChevronDown, theme.text_primary)
                        } else {
                            icon_12(IconName::ChevronRight, theme.text_muted)
                        }),
                )
                // Search Input Box
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .h(px(26.0))
                        .px_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .text_xs()
                                .text_color(if state.find_query.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.find_query.is_empty() {
                                    "Find in document (Ctrl+F)...".to_string()
                                } else {
                                    state.find_query.clone()
                                }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(if state.find_match_count > 0 {
                                    theme.status_green
                                } else {
                                    theme.text_subtle
                                })
                                .child(if state.find_query.is_empty() {
                                    "".to_string()
                                } else if state.find_match_count == 0 {
                                    "No results".to_string()
                                } else {
                                    format!("{} matches", state.find_match_count)
                                }),
                        ),
                )
                // Navigation & Toggles
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        // Prev match
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.find_prev();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::ChevronUp, theme.text_muted)),
                        )
                        // Next match
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.find_next();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::ChevronDown, theme.text_muted)),
                        )
                        // Match case toggle
                        .child(
                            div()
                                .px_1()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.find_case_sensitive {
                                    theme.accent
                                } else {
                                    theme.bg_input
                                })
                                .text_xs()
                                .text_color(if state.find_case_sensitive {
                                    theme.text_on_accent
                                } else {
                                    theme.text_muted
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.find_case_sensitive =
                                            !this.state.find_case_sensitive;
                                        this.state.update_find_matches();
                                        cx.notify();
                                    }),
                                )
                                .child("Aa"),
                        )
                        // Whole word toggle
                        .child(
                            div()
                                .px_1()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.find_whole_word {
                                    theme.accent
                                } else {
                                    theme.bg_input
                                })
                                .text_xs()
                                .text_color(if state.find_whole_word {
                                    theme.text_on_accent
                                } else {
                                    theme.text_muted
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.find_whole_word = !this.state.find_whole_word;
                                        this.state.update_find_matches();
                                        cx.notify();
                                    }),
                                )
                                .child(r"\b"),
                        )
                        // Regex toggle
                        .child(
                            div()
                                .px_1()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if state.find_regex {
                                    theme.accent
                                } else {
                                    theme.bg_input
                                })
                                .text_xs()
                                .text_color(if state.find_regex {
                                    theme.text_on_accent
                                } else {
                                    theme.text_muted
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.find_regex = !this.state.find_regex;
                                        this.state.update_find_matches();
                                        cx.notify();
                                    }),
                                )
                                .child(".*"),
                        )
                        // Close find bar
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.find_bar_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_muted)),
                        ),
                ),
        )
        // Row 2: Replace Input & Action Buttons (when replace mode is active)
        .children(state.find_replace_mode.then(|| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .h(px(26.0))
                        .px_2()
                        .items_center()
                        .rounded(px(4.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .text_xs()
                                .text_color(if state.replace_query.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.replace_query.is_empty() {
                                    "Replace with ($1)...".to_string()
                                } else {
                                    state.replace_query.clone()
                                }),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        // Replace Next
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(theme.bg_hover)
                                .hover(|s| s.bg(theme.accent).text_color(theme.text_on_accent))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.replace_next();
                                        cx.notify();
                                    }),
                                )
                                .child("Replace"),
                        )
                        // Replace All
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(theme.bg_hover)
                                .hover(|s| s.bg(theme.accent).text_color(theme.text_on_accent))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.replace_all();
                                        cx.notify();
                                    }),
                                )
                                .child("Replace All"),
                        ),
                )
        }))
}

fn render_bottom_panel(
    state: &HadesNativeState,
    registry: &PanelRegistry,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .w_full()
        .h(px(state.bottom_panel_height))
        .bg(theme.bg_sidebar)
        .border_t_1()
        .border_color(theme.border_subtle)
        // Horizontal Resize Splitter
        .child({
            let is_resizing = state.is_resizing_bottom_panel;
            div()
                .w_full()
                .h(px(5.0))
                .bg(if is_resizing {
                    theme.accent
                } else {
                    theme.border_subtle
                })
                .cursor_row_resize()
                .hover(|s| s.bg(theme.accent))
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _event, _window, cx| {
                        this.state.is_resizing_bottom_panel = true;
                        cx.notify();
                    }),
                )
        })
        // Bottom Panel Header / Tabs
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .px_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_titlebar)
                .child(div().flex().flex_row().items_center().gap_3().children(
                    registry.bottom_tabs.iter().map(|tab| {
                        let is_active = state.active_bottom_tab.id() == tab.id();
                        let tab_id = tab.id();
                        let tab_title = tab.title();

                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1p5()
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
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _event, _window, cx| {
                                    match tab_id {
                                        "terminal" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::Terminal,
                                        ),
                                        "problems" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::Problems,
                                        ),
                                        "output" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::Output,
                                        ),
                                        "debug_console" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::DebugConsole,
                                        ),
                                        "logcat" => {
                                            this.state.set_bottom_tab(
                                                crate::domain::layout::BottomPanelTab::Logcat,
                                            );
                                            this.state.logcat.refresh_devices();
                                        }
                                        "ports" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::Ports,
                                        ),
                                        "jobs" => this.state.set_bottom_tab(
                                            crate::domain::layout::BottomPanelTab::Jobs,
                                        ),
                                        _ => {}
                                    }
                                    cx.notify();
                                }),
                            )
                            .child(tab_title)
                            .children(tab.badge(state).map(|badge| {
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded_full()
                                    .bg(theme.accent.opacity(0.18))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.accent)
                                    .child(badge)
                            }))
                    }),
                ))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        // Decrease Height (-)
                        .child(
                            div()
                                .p_1()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .rounded(px(3.0))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.resize_bottom_panel(-50.0);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Minus, theme.text_subtle)),
                        )
                        // Increase Height (+)
                        .child(
                            div()
                                .p_1()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .rounded(px(3.0))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.resize_bottom_panel(50.0);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Plus, theme.text_subtle)),
                        )
                        // Close
                        .child(
                            div()
                                .p_1()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .rounded(px(3.0))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.bottom_panel_open = false;
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_subtle)),
                        ),
                ),
        )
        // Bottom Panel Content
        .child(div().flex().flex_1().w_full().overflow_hidden().child(
            if let Some(tab) = registry.find_bottom(state.active_bottom_tab.id()) {
                tab.render(state, cx)
            } else {
                crate::panels::render_terminal_panel(state, cx).into_any_element()
            },
        ))
}

fn render_right_sidebar(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .w(px(state.right_sidebar_width))
        .h_full()
        .bg(theme.bg_sidebar)
        .border_l_1()
        .border_color(theme.border_subtle)
        .child(render_right_sidebar_panel(state, cx))
}

fn render_iphone_preview_sidebar(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .w(px(state.iphone_preview_width))
        .h_full()
        .bg(theme.bg_sidebar)
        .border_l_1()
        .border_color(theme.border_subtle)
        .child(
            // Top Header: iPhone Titlebar with Close button & Refresh action
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .px_2p5()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(icon_14(IconName::Zap, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("iPhone — USB Mirror"),
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
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    crate::panels::apex::trigger_device_refresh();
                                    this.state.toast_manager.push_info("Refreshing iPhone device mirror...");
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::RefreshCw, theme.text_subtle))
                        )
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                                    this.state.iphone_preview_open = false;
                                    cx.notify();
                                }))
                                .child(icon_12(IconName::X, theme.text_subtle))
                        )
                )
        )
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .overflow_hidden()
                .child(crate::panels::mobile::render_physical_device_mirror(state, cx))
        )
}

fn render_workspace_trust_banner(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let root_name = state
        .workspace_root
        .file_name()
        .map(|f| f.to_string_lossy().to_string())
        .unwrap_or_else(|| "workspace".to_string());

    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .w_full()
        .px_3()
        .py_1p5()
        .bg(rgba(0xeab30818))
        .border_b_1()
        .border_color(rgba(0xeab30840))
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(ui_icon(IconName::Shield, 14.0, theme.status_yellow))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(format!("Restricted Mode — folder \"{}\" is not trusted. Automated scripts, build tasks, and untrusted extension features are restricted.", root_name))
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
                        .px_2p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .bg(theme.accent)
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .hover(|s| s.opacity(0.88))
                        .child("Trust Folder")
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.trust_workspace();
                            cx.notify();
                        }))
                )
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .hover(|s| s.bg(theme.bg_hover))
                        .child("Dismiss")
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                            this.state.dismiss_workspace_trust();
                            cx.notify();
                        }))
                )
        )
}

fn render_tool_permission_dialog(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> Option<AnyElement> {
    let req = state.pending_tool_permission.as_ref()?;
    let theme = &state.theme;
    let req_id = req.id.clone();
    let tool_name = req.tool.clone();
    let args_str = req.args.clone();
    let level = req.level.clone();

    let is_dangerous = level == "dangerous";
    let level_color = if is_dangerous {
        theme.status_red
    } else {
        theme.status_yellow
    };

    Some(
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(rgba(0x000000a0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .w(px(460.0))
                    .p_4()
                    .rounded(px(8.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(level_color)
                    .shadow_lg()
                    .gap_3()
                    // Header
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(ui_icon(IconName::TriangleAlert, 18.0, level_color))
                            .child(
                                div()
                                    .text_sm()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(level_color)
                                    .child("Tool Permission Required"),
                            ),
                    )
                    // Description
                    .child(div().text_xs().text_color(theme.text_muted).child(format!(
                        "The AI Agent requested permission to execute operation [{}]:",
                        level
                    )))
                    // Operation details box
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1p5()
                            .p_2p5()
                            .rounded(px(6.0))
                            .bg(theme.bg_raised)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .child(ui_icon(IconName::Terminal, 13.0, theme.accent))
                                    .child(
                                        div()
                                            .font_family("Cascadia Code, Consolas, monospace")
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(tool_name),
                                    ),
                            )
                            .children((!args_str.is_empty()).then(|| {
                                div()
                                    .id("tool_permission_args_scroll")
                                    .font_family("Cascadia Code, Consolas, monospace")
                                    .text_xs()
                                    .text_color(theme.text_subtle)
                                    .max_h(px(100.0))
                                    .overflow_y_scroll()
                                    .child(args_str)
                            })),
                    )
                    // Action Buttons
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .justify_end()
                            .gap_2()
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .bg(theme.status_red.opacity(0.15))
                                    .border_1()
                                    .border_color(theme.status_red.opacity(0.4))
                                    .text_xs()
                                    .font_weight(FontWeight::MEDIUM)
                                    .text_color(theme.status_red)
                                    .hover(|s| s.bg(theme.status_red.opacity(0.25)))
                                    .child("Deny")
                                    .on_mouse_down(MouseButton::Left, {
                                        let req_id = req_id.clone();
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.state.respond_tool_permission(&req_id, false);
                                            cx.notify();
                                        })
                                    }),
                            )
                            .child(
                                div()
                                    .px_3()
                                    .py_1()
                                    .rounded(px(4.0))
                                    .cursor_pointer()
                                    .bg(theme.status_green.opacity(0.2))
                                    .border_1()
                                    .border_color(theme.status_green.opacity(0.5))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.status_green)
                                    .hover(|s| s.bg(theme.status_green.opacity(0.3)))
                                    .child("Allow")
                                    .on_mouse_down(MouseButton::Left, {
                                        let req_id = req_id.clone();
                                        cx.listener(move |this, _event, _window, cx| {
                                            this.state.respond_tool_permission(&req_id, true);
                                            cx.notify();
                                        })
                                    }),
                            ),
                    ),
            )
            .into_any_element(),
    )
}
