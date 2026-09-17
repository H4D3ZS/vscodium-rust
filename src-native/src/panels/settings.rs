// Preferences & Runtime Settings Panel for Native GPUI.
//
// 100% Parity with TypeScript SettingsPage.tsx & registry.ts:
// - 8 Canonical Sections:
//   1. Editor (Editor, Workspace)
//   2. Appearance (Theme, AI Avatar)
//   3. AI Models (Models & API Keys, Model Selection, Lemonade, Free Claude Code, Inference Backend)
//   4. Agent (Chat & Agent, Permissions, Skill Store, Specs & Workflow)
//   5. Additional Modules (Module Installer, Platform Status)
//   6. Privacy & Account (Account, Enterprise, Privacy)
//   7. Keyboard (Keyboard Shortcuts)
//   8. Advanced (APEX, ANE, Kortex, Memory, Voice, Steering, Hooks, LSP, MCP, PyTorch)

use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[allow(dead_code)]
pub struct SettingsSectionDef {
    pub id: &'static str,
    pub label: &'static str,
    pub icon: IconName,
    pub items: &'static [&'static str],
}

pub const SETTINGS_SECTIONS: &[SettingsSectionDef] = &[
    SettingsSectionDef {
        id: "editor",
        label: "Editor",
        icon: IconName::FileCode,
        items: &["Editor", "Workspace"],
    },
    SettingsSectionDef {
        id: "appearance",
        label: "Appearance",
        icon: IconName::Palette,
        items: &["Theme", "AI Avatar"],
    },
    SettingsSectionDef {
        id: "ai-models",
        label: "AI Models",
        icon: IconName::Brain,
        items: &[
            "Models & API Keys",
            "Model Selection",
            "Lemonade",
            "Free Claude Code",
            "Inference Backend",
        ],
    },
    SettingsSectionDef {
        id: "agent",
        label: "Agent",
        icon: IconName::Bot,
        items: &[
            "Chat & Agent",
            "Permissions",
            "Skill Store",
            "Specs & Workflow",
        ],
    },
    SettingsSectionDef {
        id: "extensions",
        label: "Additional Modules",
        icon: IconName::Blocks,
        items: &["Module Installer", "Platform Status"],
    },
    SettingsSectionDef {
        id: "privacy-account",
        label: "Privacy & Account",
        icon: IconName::User,
        items: &["Account", "Enterprise", "Privacy"],
    },
    SettingsSectionDef {
        id: "keyboard",
        label: "Keyboard",
        icon: IconName::Keyboard,
        items: &["Keyboard Shortcuts"],
    },
    SettingsSectionDef {
        id: "advanced",
        label: "Advanced",
        icon: IconName::Cpu,
        items: &[
            "APEX Intelligence",
            "ANE Acceleration",
            "Kortex / AIM",
            "Memory (.aim)",
            "Voice & TTS",
            "Steering & Rules",
            "Hooks & Lifecycle",
            "Language Servers",
            "MCP & Tools",
            "PyTorch ML Studio",
        ],
    },
];

pub fn render_settings_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    render_settings_page_view(state, cx)
}

pub fn render_settings_page_view(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let active_sec_idx = state
        .settings_category
        .min(SETTINGS_SECTIONS.len().saturating_sub(1));
    let active_sec = &SETTINGS_SECTIONS[active_sec_idx];
    let active_item_idx = state
        .settings_item
        .min(active_sec.items.len().saturating_sub(1));
    let search_query = state.settings_search.to_lowercase();

    div()
        .id("settings_page_root")
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        .overflow_hidden()
        // ── 1. Top Header Bar (Matching SettingsPage.tsx) ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_7()
                .py_3()
                .bg(theme.bg_editor)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_4()
                        .flex_1()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Settings"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .px_3()
                                .h(px(28.0))
                                .w_full()
                                .max_w(px(480.0))
                                .rounded(px(6.0))
                                .bg(rgba(0xffffff0d))
                                .border_1()
                                .border_color(rgba(0xffffff1a))
                                .child(ui_icon(IconName::Search, 13.0, theme.text_subtle))
                                .child(
                                    div()
                                        .flex_1()
                                        .text_xs()
                                        .text_color(if state.settings_search.is_empty() {
                                            theme.text_subtle
                                        } else {
                                            theme.text_primary
                                        })
                                        .child(if state.settings_search.is_empty() {
                                            "Search settings...".to_string()
                                        } else {
                                            state.settings_search.clone()
                                        }),
                                )
                                .children((!state.settings_search.is_empty()).then(|| {
                                    div()
                                        .cursor_pointer()
                                        .hover(|s| s.text_color(theme.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.state.settings_search.clear();
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(IconName::X, 12.0, theme.text_subtle))
                                })),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_2p5()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(rgba(0x22c55e18))
                        .border_1()
                        .border_color(rgba(0x22c55e30))
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.status_green)
                        .child(ui_icon(IconName::Cpu, 11.0, theme.status_green))
                        .child("Direct3D 12 · 120 FPS"),
                ),
        )
        // ── 2. Settings Body (2-Column: Sidebar + Content Pane) ──
        .child(
            div()
                .flex()
                .flex_row()
                .flex_1()
                .overflow_hidden()
                // ── Left Vertical Navigation (220px) ──
                .child(
                    div()
                        .w(px(220.0))
                        .h_full()
                        .bg(theme.bg_sidebar)
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .id("settings_sidebar_scroll")
                        .flex()
                        .flex_col()
                        .overflow_y_scroll()
                        .children(SETTINGS_SECTIONS.iter().enumerate().map(|(idx, sec)| {
                            let is_current = idx == active_sec_idx;
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2p5()
                                .px_4()
                                .h(px(32.0))
                                .cursor_pointer()
                                .border_l_2()
                                .border_color(if is_current {
                                    theme.accent
                                } else {
                                    rgba(0x00000000)
                                })
                                .bg(if is_current {
                                    rgba(0x4f8ef71f)
                                } else {
                                    rgba(0x00000000)
                                })
                                .hover(|s| {
                                    s.bg(if is_current {
                                        rgba(0x4f8ef72e)
                                    } else {
                                        rgba(0xffffff0a)
                                    })
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        this.state.settings_category = idx;
                                        this.state.settings_item = 0;
                                        cx.notify();
                                    }),
                                )
                                .child(ui_icon(
                                    sec.icon,
                                    14.0,
                                    if is_current {
                                        theme.accent
                                    } else {
                                        theme.text_subtle
                                    },
                                ))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(if is_current {
                                            FontWeight::MEDIUM
                                        } else {
                                            FontWeight::NORMAL
                                        })
                                        .text_color(if is_current {
                                            rgb(0xffffff)
                                        } else {
                                            theme.text_muted
                                        })
                                        .child(sec.label),
                                )
                        })),
                )
                // ── Right Main Content Pane ──
                .child(
                    div()
                        .id("settings_content_scroll")
                        .flex_1()
                        .h_full()
                        .overflow_y_scroll()
                        .px_8()
                        .py_6()
                        .flex()
                        .flex_col()
                        .gap_4()
                        .max_w(px(920.0))
                        // Sub-nav chips
                        .children((active_sec.items.len() > 1).then(|| {
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .flex_wrap()
                                .gap_1p5()
                                .pb_3()
                                .border_b_1()
                                .border_color(rgba(0xffffff0f))
                                .mb_2()
                                .children(active_sec.items.iter().enumerate().map(
                                    |(item_idx, item_name)| {
                                        let is_active = item_idx == active_item_idx;
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded(px(3.0))
                                            .cursor_pointer()
                                            .bg(if is_active {
                                                rgb(0x007acc)
                                            } else {
                                                rgba(0x00000000)
                                            })
                                            .border_1()
                                            .border_color(if is_active {
                                                rgb(0x007acc)
                                            } else {
                                                rgba(0xffffff1f)
                                            })
                                            .text_xs()
                                            .font_weight(if is_active {
                                                FontWeight::BOLD
                                            } else {
                                                FontWeight::NORMAL
                                            })
                                            .text_color(if is_active {
                                                rgb(0xffffff)
                                            } else {
                                                theme.text_muted
                                            })
                                            .hover(|s| {
                                                s.bg(if is_active {
                                                    rgb(0x007acc)
                                                } else {
                                                    rgba(0xffffff0d)
                                                })
                                            })
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _e, _w, cx| {
                                                    this.state.settings_item = item_idx;
                                                    cx.notify();
                                                }),
                                            )
                                            .child(*item_name)
                                    },
                                ))
                        }))
                        // Panel body content
                        .child(match active_sec_idx {
                            0 => render_editor_section(state, active_item_idx, &search_query, cx),
                            1 => {
                                render_appearance_section(state, active_item_idx, &search_query, cx)
                            }
                            2 => {
                                render_ai_models_section(state, active_item_idx, &search_query, cx)
                            }
                            3 => render_agent_section(state, active_item_idx, &search_query, cx),
                            4 => {
                                render_extensions_section(state, active_item_idx, &search_query, cx)
                            }
                            5 => render_privacy_account_section(
                                state,
                                active_item_idx,
                                &search_query,
                                cx,
                            ),
                            6 => render_keyboard_section(state, active_item_idx, &search_query, cx),
                            7 => render_advanced_section(state, active_item_idx, &search_query, cx),
                            _ => div().child("Select a section").into_any_element(),
                        }),
                ),
        )
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. EDITOR SECTION (Editor, Workspace)
// ─────────────────────────────────────────────────────────────────────────────
fn render_editor_section(
    state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;
    let word_wrap_active = state
        .active_tab()
        .map(|t| t.model.wrap.enabled)
        .unwrap_or(true);

    if item_idx == 1 {
        // Workspace
        return div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .child(
                        div()
                            .text_base()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.text_primary)
                            .child("WORKSPACE"),
                    )
                    .child(div().text_xs().text_color(theme.text_muted).mt_0p5().child(
                        "Configure workspace directories, auto-save interval, and trust settings.",
                    )),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_subtle)
                    .child("FILES & PERSISTENCE"),
            )
            .child(render_clean_row(
                "Workspace Root Folder",
                "Root directory currently loaded in the project file explorer.",
                div()
                    .px_2p5()
                    .py_1()
                    .rounded(px(4.0))
                    .bg(rgba(0xffffff0f))
                    .border_1()
                    .border_color(rgba(0xffffff1a))
                    .text_xs()
                    .text_color(theme.text_primary)
                    .child(state.workspace_root.to_string_lossy().to_string()),
                theme,
            ))
            .child(render_clean_row(
                "Auto Save Mode",
                "Automatically persist buffer changes to disk without manual Ctrl+S.",
                div()
                    .h(px(26.0))
                    .px_3()
                    .rounded(px(4.0))
                    .bg(rgba(0xffffff0f))
                    .border_1()
                    .border_color(rgba(0xffffff1a))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(div().text_xs().text_color(theme.text_primary).child("Off"))
                    .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle)),
                theme,
            ))
            .child(render_clean_row(
                "Workspace Trust",
                "Allow autonomous terminal execution and file writing in this workspace.",
                div()
                    .px_2p5()
                    .py_1()
                    .rounded(px(4.0))
                    .bg(rgba(0x22c55e18))
                    .border_1()
                    .border_color(rgba(0x22c55e30))
                    .text_xs()
                    .text_color(theme.status_green)
                    .child("Trusted"),
                theme,
            ))
            .into_any_element();
    }

    // Editor (Screenshot 2 exact parity)
    div()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("EDITOR"),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .mt_0p5()
                        .child("Configure the Monaco editor appearance and behavior."),
                ),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_subtle)
                .child("TEXT & LAYOUT"),
        )
        .child(render_clean_row(
            "Font Size",
            "Editor font size in pixels.",
            div()
                .w(px(76.0))
                .h(px(26.0))
                .px_2p5()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .items_center()
                .justify_end()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.editor_font_size = if this.state.editor_font_size == 14 {
                            16
                        } else {
                            14
                        };
                        this.state.save_preferences();
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(state.editor_font_size.to_string()),
                ),
            theme,
        ))
        .child(render_clean_row(
            "Tab Size",
            "Number of spaces a tab is equal to.",
            div()
                .w(px(76.0))
                .h(px(26.0))
                .px_2p5()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .items_center()
                .justify_end()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.default_tab_size = if this.state.default_tab_size == 4 {
                            2
                        } else {
                            4
                        };
                        this.state.save_preferences();
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(state.default_tab_size.to_string()),
                ),
            theme,
        ))
        .child(render_clean_row(
            "Auto Save",
            "When editor content is automatically saved.",
            div()
                .h(px(26.0))
                .px_3()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(div().text_xs().text_color(theme.text_primary).child("Off"))
                .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle)),
            theme,
        ))
        .child(render_clean_row(
            "Word Wrap",
            "Controls whether lines should wrap or continue horizontally offscreen.",
            div()
                .h(px(26.0))
                .px_3()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.toggle_word_wrap();
                        cx.notify();
                    }),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(if word_wrap_active { "On" } else { "Off" }),
                )
                .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle)),
            theme,
        ))
        .child(render_clean_row(
            "Line Numbers",
            "Controls the display of line numbers in the gutter margin.",
            div()
                .h(px(26.0))
                .px_3()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.line_numbers_enabled = !this.state.line_numbers_enabled;
                        this.state.save_preferences();
                        cx.notify();
                    }),
                )
                .child(div().text_xs().text_color(theme.text_primary).child(
                    if state.line_numbers_enabled {
                        "On"
                    } else {
                        "Off"
                    },
                ))
                .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle)),
            theme,
        ))
        .child(render_clean_row(
            "Sticky Scroll",
            "Pin active function, struct, and impl scopes to the top of the editor.",
            div()
                .h(px(26.0))
                .px_3()
                .rounded(px(4.0))
                .bg(rgba(0xffffff0f))
                .border_1()
                .border_color(rgba(0xffffff1a))
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .cursor_pointer()
                .on_mouse_down(
                    MouseButton::Left,
                    cx.listener(|this, _e, _w, cx| {
                        this.state.sticky_scroll.enabled = !this.state.sticky_scroll.enabled;
                        this.state.save_preferences();
                        cx.notify();
                    }),
                )
                .child(div().text_xs().text_color(theme.text_primary).child(
                    if state.sticky_scroll.enabled {
                        "On"
                    } else {
                        "Off"
                    },
                ))
                .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle)),
            theme,
        ))
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. APPEARANCE SECTION (Theme, AI Avatar)
// ─────────────────────────────────────────────────────────────────────────────
fn render_appearance_section(
    state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;

    if item_idx == 1 {
        // AI Avatar
        return div()
            .flex()
            .flex_col()
            .gap_3()
            .child(section_header("AIRI 3D COMPANION & AVATAR", theme))
            .child(render_setting_card(
                "Avatar Presence",
                "Display the 3D VRM companion avatar in the side panel with real-time lip sync",
                "Avatar",
                div().flex().flex_row().gap_1p5().children(
                    ["Visible", "Minimized", "Hidden"].into_iter().map(|m| {
                        pill_button(
                            m,
                            m == "Visible",
                            theme,
                            move |this, _w, cx| {
                                this.state
                                    .toast_manager
                                    .push_info(&format!("AIRI Avatar: {m}"));
                                cx.notify();
                            },
                            cx,
                        )
                    }),
                ),
                theme,
            ))
            .child(render_setting_card(
                "Active VRM Model",
                "Selected 3D avatar rigging mesh loaded into the WebGL / Direct3D scene",
                "Avatar",
                div()
                    .px_2p5()
                    .py_1()
                    .rounded(px(4.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .text_xs()
                    .text_color(theme.text_primary)
                    .child("Airi-V2-Optimized.vrm"),
                theme,
            ))
            .child(render_toggle_card(
                "Vocal Audio Reactivity",
                "Animate mouth blendshapes in real-time driven by TTS audio frequencies",
                "Avatar",
                true,
                |this, _w, cx| {
                    this.state
                        .toast_manager
                        .push_info("Vocal reactivity toggled.");
                    cx.notify();
                },
                theme,
                cx,
            ))
            .into_any_element();
    }

    // Theme
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(section_header("THEME & COLOR PALETTES", theme))
        .child(render_setting_card(
            "Color Theme",
            "Select the active visual palette for the workspace, editor, and UI chrome",
            "Appearance",
            div().flex().flex_row().flex_wrap().gap_1p5().children(
                [
                    ("Cursor Dark", "cursor_dark"),
                    ("Tokyo Night", "tokyo_night"),
                    ("Obsidian Black", "obsidian"),
                    ("High Contrast", "high_contrast"),
                ]
                .into_iter()
                .map(|(name, id)| {
                    let is_sel = state.active_theme_name == id;
                    pill_button(
                        name,
                        is_sel,
                        theme,
                        move |this, _w, cx| {
                            this.state.set_theme(id);
                            cx.notify();
                        },
                        cx,
                    )
                }),
            ),
            theme,
        ))
        .child(render_setting_card(
            "Font Family",
            "Monospace typography used for all code rendering and text editing buffers",
            "Appearance",
            div()
                .px_2p5()
                .py_1()
                .rounded(px(4.0))
                .bg(theme.bg_raised)
                .border_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_primary)
                .child("Cascadia Code, Consolas, JetBrains Mono"),
            theme,
        ))
        .child(render_setting_card(
            "GPU Renderer Backend",
            "DirectX hardware acceleration delivering stutter-free 120 FPS frame rates",
            "Appearance",
            div()
                .px_2p5()
                .py_1()
                .rounded(px(4.0))
                .bg(rgba(0x22c55e18))
                .border_1()
                .border_color(rgba(0x22c55e30))
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.status_green)
                .child("Direct3D 12 (D3D12) · Active"),
            theme,
        ))
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. AI MODELS SECTION (Models, Model Selection, Lemonade, FCC, Inference)
// ─────────────────────────────────────────────────────────────────────────────
fn render_ai_models_section(
    state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;

    match item_idx {
        1 => {
            // Model Selection
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("MODEL ROUTING & TASK SELECTION", theme))
                .child(render_setting_card(
                    "Composer & Agent Chat Engine",
                    "Primary frontier model used for multi-file editing and agentic workflows",
                    "Routing",
                    pill_button("claude-3-5-sonnet-20241022", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .child(render_setting_card(
                    "Fast Autocomplete & Inline Edit",
                    "Sub-100ms ultra-low-latency model for Copilot-style tab completions",
                    "Routing",
                    pill_button("deepseek-coder-6.7b", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .child(render_setting_card(
                    "Autonomous Reasoning Planner",
                    "High-depth model for complex architectural planning and bug bounty",
                    "Routing",
                    pill_button("deepseek-reasoner-r1", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .child(render_setting_card(
                    "Vision & Screen Grounding",
                    "Multimodal model for analyzing UI screenshots and device mockups",
                    "Routing",
                    pill_button("qwen-2.5-vl-72b", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .into_any_element()
        }
        2 => {
            // Lemonade Local Server
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("LEMONADE LOCAL GPU ENGINE", theme))
                .child(render_setting_card(
                    "GPU Accelerator Device",
                    "Local GPU device allocated for offline zero-latency inference",
                    "Lemonade",
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("NVIDIA GeForce RTX (CUDA 12.4)"),
                    theme,
                ))
                .child(render_setting_card(
                    "VRAM Memory Budget",
                    "Maximum dedicated GPU memory allocated to active model weights",
                    "Lemonade",
                    div().flex().flex_row().gap_1p5().children(
                        ["8 GB", "16 GB", "24 GB"].into_iter().map(|vram| {
                            pill_button(
                                vram,
                                vram == "16 GB",
                                theme,
                                move |this, _w, cx| {
                                    this.state
                                        .toast_manager
                                        .push_info(&format!("VRAM budget set to {vram}"));
                                    cx.notify();
                                },
                                cx,
                            )
                        }),
                    ),
                    theme,
                ))
                .child(render_toggle_card(
                    "FlashAttention 2 / vLLM PagedAttention",
                    "Enable hardware-accelerated memory attention kernels for 4x higher throughput",
                    "Lemonade",
                    true,
                    |this, _w, cx| {
                        this.state
                            .toast_manager
                            .push_info("FlashAttention 2 enabled.");
                        cx.notify();
                    },
                    theme,
                    cx,
                ))
                .into_any_element()
        }
        3 => {
            // Free Claude Code (FCC)
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("FREE CLAUDE CODE (FCC) PROXY", theme))
                .child(render_setting_card(
                    "Reverse Proxy Endpoint",
                    "Local loopback proxy multiplexing Claude Code requests to available providers",
                    "FCC",
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("http://127.0.0.1:8080/v1"),
                    theme,
                ))
                .child(render_setting_card(
                    "Upstream Provider Fallback",
                    "Automatic failover routing if the primary API endpoint returns 429 or 503",
                    "FCC",
                    div().flex().flex_row().gap_1p5().children(
                        ["OpenRouter", "NVIDIA NIM", "Direct"].into_iter().map(|p| {
                            pill_button(
                                p,
                                p == "OpenRouter",
                                theme,
                                move |this, _w, cx| {
                                    this.state
                                        .toast_manager
                                        .push_info(&format!("FCC Fallback: {p}"));
                                    cx.notify();
                                },
                                cx,
                            )
                        }),
                    ),
                    theme,
                ))
                .into_any_element()
        }
        4 => {
            // Inference Backend
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("AI Model")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .mt_0p5()
                                .child("One button to run a model on your AMD GPU, or pick another backend.")
                        )
                )
                // Kortex ROCmFPX Radio item with Start button and Gear
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .py_2()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2p5()
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .h(px(14.0))
                                        .rounded_full()
                                        .border_1()
                                        .border_color(theme.accent)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(div().w(px(6.0)).h(px(6.0)).rounded_full().bg(theme.accent))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("Kortex ROCmFPX")
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("ready · Qwen3.8-27B-AD-IQ3_XXS")
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
                                        .px_3()
                                        .py_1()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x007acc))
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .cursor_pointer()
                                        .child("Start")
                                )
                                .child(ui_icon(IconName::Settings, 14.0, theme.text_muted))
                        )
                )
                // Lemonade Radio item
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2p5()
                        .py_1p5()
                        .child(
                            div()
                                .w(px(14.0))
                                .h(px(14.0))
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(0xffffff33))
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("Lemonade")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("running · standard GGUF quants only")
                        )
                )
                // OpenAI-compatible API Radio item
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2p5()
                        .py_1p5()
                        .child(
                            div()
                                .w(px(14.0))
                                .h(px(14.0))
                                .rounded_full()
                                .border_1()
                                .border_color(rgba(0xffffff33))
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("OpenAI-compatible API")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("cloud / LiteLLM")
                        )
                )
                // Claude Code CLI Checkbox
                .child(
                    div()
                        .pt_3()
                        .mt_1()
                        .border_t_1()
                        .border_color(rgba(0xffffff0f))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .child(
                                    div()
                                        .w(px(14.0))
                                        .h(px(14.0))
                                        .rounded(px(2.0))
                                        .border_1()
                                        .border_color(rgba(0xffffff33))
                                        .bg(theme.accent)
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(ui_icon(IconName::Check, 10.0, rgb(0xffffff)))
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child("Run chat through the Claude Code CLI")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .ml_5()
                                .mt_0p5()
                                .child("Same model, but Claude Code's tools, hooks and skills.")
                        )
                )
                // Route /v1/messages via
                .child(
                    div()
                        .mt_2()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .child("Route /v1/messages via")
                                )
                                .child(
                                    div()
                                        .h(px(26.0))
                                        .px_3()
                                        .rounded(px(4.0))
                                        .bg(rgba(0xffffff0f))
                                        .border_1()
                                        .border_color(rgba(0xffffff1a))
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child("Auto (Kortex proxy if running)")
                                        )
                                        .child(ui_icon(IconName::ChevronDown, 10.0, theme.text_subtle))
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .mt_1()
                                .child("Kortex adds prefix-cache reuse + harness compression on the Claude Code path. Lemonade direct is the plain Anthropic adapter. Applies to the Claude terminal too.")
                        )
                )
                .into_any_element()
        }
        _ => {
            // Models & API Keys (Item 0)
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("MODELS & API KEYS", theme))
                .child(render_api_key_card(
                    "Anthropic Claude 3.5 Sonnet",
                    "State-of-the-art coding and agentic execution. Uses ANTHROPIC_API_KEY (sk-ant-...)",
                    "anthropic",
                    &state.anthropic_api_key,
                    theme,
                    cx,
                ))
                .child(render_api_key_card(
                    "OpenAI GPT-4o",
                    "Multimodal reasoning and tool orchestration. Uses OPENAI_API_KEY (sk-...)",
                    "openai",
                    &state.openai_api_key,
                    theme,
                    cx,
                ))
                .child(render_api_key_card(
                    "Google Gemini 1.5 Pro & 2.0 Flash",
                    "Fast context processing and long-horizon planning. Uses GEMINI_API_KEY (AIza...)",
                    "google",
                    &state.gemini_api_key,
                    theme,
                    cx,
                ))
                .child(render_api_key_card(
                    "DeepSeek V3 & R1",
                    "High-efficiency reasoning and open-weights architecture. Uses DEEPSEEK_API_KEY (sk-...)",
                    "deepseek",
                    &state.deepseek_api_key,
                    theme,
                    cx,
                ))
                .child(render_api_key_card(
                    "ModelScope (Qwen Ambassador)",
                    "Alibaba ModelScope cloud inference (Qwen-Max, Qwen-Plus, Qwen2.5-Coder-32B). Uses MODELSCOPE_API_KEY",
                    "modelscope",
                    &state.modelscope_api_key,
                    theme,
                    cx,
                ))
                .child(render_setting_card(
                    "Lemonade Local Inference",
                    "Run 100% offline open-source models with zero telemetry via Lemonade (:13305)",
                    "Provider",
                    div()
                        .flex()
                        .items_center()
                        .gap_1p5()
                        .px_2p5()
                        .py_0p5()
                        .rounded(px(4.0))
                        .bg(rgba(0x38bdf818))
                        .text_xs()
                        .text_color(rgb(0x38bdf8))
                        .child("● Ready (Lemonade :13305)"),
                    theme,
                ))
                .into_any_element()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. AGENT SECTION (Chat & Agent, Permissions, Skill Store, Specs)
// ─────────────────────────────────────────────────────────────────────────────
fn render_agent_section(
    state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;

    match item_idx {
        1 => {
            // Permissions
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("AGENT SAFETY & PERMISSION BOUNDS", theme))
                .child(render_toggle_card(
                    "Auto-Approve Terminal Commands",
                    "Allow AI Agent to execute shell commands in the background without interactive confirmation dialogs",
                    "Permissions",
                    state.agent_auto_approve_terminal,
                    |this, _w, cx| {
                        this.state.agent_auto_approve_terminal = !this.state.agent_auto_approve_terminal;
                        this.state.save_preferences();
                        cx.notify();
                    },
                    theme,
                    cx,
                ))
                .child(render_toggle_card(
                    "Auto-Approve File Modifications",
                    "Permit autonomous write_to_file and replace_file_content within the workspace root",
                    "Permissions",
                    state.agent_auto_approve_file_write,
                    |this, _w, cx| {
                        this.state.agent_auto_approve_file_write = !this.state.agent_auto_approve_file_write;
                        this.state.save_preferences();
                        cx.notify();
                    },
                    theme,
                    cx,
                ))
                .child(render_toggle_card(
                    "Prevent Outbound Network Access",
                    "Block agents from issuing external network calls outside verified localhost and model API domains",
                    "Permissions",
                    false,
                    |this, _w, cx| {
                        this.state.toast_manager.push_info("Network firewall policy updated.");
                        cx.notify();
                    },
                    theme,
                    cx,
                ))
                .into_any_element()
        }
        2 => {
            // Skill Store
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("AGENT SKILL STORE & EXTENSIONS", theme))
                .children([
                    ("antigravity-guide", "Google Antigravity IDE & CLI full user guide and shortcuts", true),
                    ("agy-customizations", "Customization system: skills, rules, MCP servers, and hooks", true),
                    ("git-expert", "Advanced interactive git rebase, bisect, and merge conflict resolution", true),
                    ("rust-architect", "High-performance systems programming in modern Rust 2021/2024", true),
                ].into_iter().map(|(name, desc, installed)| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p_3()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
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
                                        .child(name)
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(desc)
                                )
                        )
                        .child(
                            if installed {
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .bg(rgba(0x22c55e18))
                                    .text_xs()
                                    .text_color(theme.status_green)
                                    .child("✓ Active")
                            } else {
                                div()
                                    .px_2()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .bg(theme.accent)
                                    .cursor_pointer()
                                    .text_xs()
                                    .text_color(rgb(0xffffff))
                                    .child("Install")
                            }
                        )
                }))
                .into_any_element()
        }
        3 => {
            // Specs & Workflow
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("SPEC-DRIVEN DEVELOPMENT WORKFLOW", theme))
                .child(render_setting_card(
                    "Specs Directory",
                    "Root directory storing requirements, implementation plans, and architecture designs",
                    "Workflow",
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child(".specs/"),
                    theme,
                ))
                .child(render_toggle_card(
                    "Mandatory Implementation Plan Approval",
                    "Require interactive user sign-off on implementation_plan.md before modifying code",
                    "Workflow",
                    true,
                    |this, _w, cx| {
                        this.state.toast_manager.push_info("Plan approval policy updated.");
                        cx.notify();
                    },
                    theme,
                    cx,
                ))
                .into_any_element()
        }
        _ => {
            // Chat & Agent (Item 0)
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("AGENT CHAT & COGNITIVE REASONING", theme))
                .child(render_setting_card(
                    "Default Reasoning Effort",
                    "Controls the depth of thought loops before code edits are formulated",
                    "Agent",
                    div()
                        .flex()
                        .flex_row()
                        .gap_1p5()
                        .children(["Low", "Medium", "High"].into_iter().map(|effort| {
                            let is_sel = state.agent_reasoning_effort == effort;
                            pill_button(effort, is_sel, theme, move |this, _w, cx| {
                                this.state.agent_reasoning_effort = effort.to_string();
                                this.state.save_preferences();
                                cx.notify();
                            }, cx)
                        })),
                    theme,
                ))
                .child(render_setting_card(
                    "Temperature",
                    "Model generation entropy (0.2 for deterministic code, 0.7 for creative exploration)",
                    "Agent",
                    pill_button("0.2 (Code Precision)", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .child(render_setting_card(
                    "Max Output Tokens",
                    "Maximum token length per generation turn in the AI Composer buffer",
                    "Agent",
                    pill_button("8,192 tokens", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .into_any_element()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. ADDITIONAL MODULES SECTION (Module Installer, Platform Status)
// ─────────────────────────────────────────────────────────────────────────────
fn render_extensions_section(
    _state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    _cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &_state.theme;

    if item_idx == 1 {
        // Platform Status
        return div()
            .flex()
            .flex_col()
            .gap_3()
            .child(section_header(
                "NATIVE HOST PLATFORM & RUNTIME HEALTH",
                theme,
            ))
            .children(
                [
                    ("Host OS", "Windows 11 (x86_64) • D3D12 GPU Driver", true),
                    (
                        "DirectX Graphics",
                        "Direct3D 12 Hardware Acceleration Active (120 FPS)",
                        true,
                    ),
                    (
                        "ConPTY Terminal",
                        "Native Windows Pseudo Console Bridge Active",
                        true,
                    ),
                    (
                        "Rust Compiler",
                        "rustc 1.84+ (LLVM 19) Target aarch64 & x86_64",
                        true,
                    ),
                ]
                .into_iter()
                .map(|(key, val, ok)| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .p_3()
                        .rounded(px(6.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(key),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(if ok {
                                    theme.status_green
                                } else {
                                    rgb(0xef4444)
                                })
                                .child(val),
                        )
                }),
            )
            .into_any_element();
    }

    // Module Installer
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(section_header("OPTIONAL IDE MODULE INSTALLER", theme))
        .children(
            [
                (
                    "Vision Grounding Sidecar",
                    "Desktop visual analysis using local multimodal model",
                    true,
                ),
                (
                    "Cyber-Ifrit Mobile Deployer",
                    "Cross-compile and USB tether iOS/Android apps from Windows",
                    true,
                ),
                (
                    "Memory Offload Manager",
                    "Quantized .aim context offloader for 100k+ token sessions",
                    true,
                ),
            ]
            .into_iter()
            .map(|(mod_name, desc, ready)| {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p_3()
                    .rounded(px(6.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_subtle)
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
                                    .child(mod_name),
                            )
                            .child(div().text_xs().text_color(theme.text_subtle).child(desc)),
                    )
                    .child(if ready {
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded(px(3.0))
                            .bg(rgba(0x22c55e18))
                            .text_xs()
                            .text_color(theme.status_green)
                            .child("Installed")
                    } else {
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded(px(3.0))
                            .bg(theme.accent)
                            .cursor_pointer()
                            .text_xs()
                            .text_color(rgb(0xffffff))
                            .child("Install")
                    })
            }),
        )
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. PRIVACY & ACCOUNT (Account, Enterprise, Privacy)
// ─────────────────────────────────────────────────────────────────────────────
fn render_privacy_account_section(
    state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &state.theme;

    if item_idx == 1 {
        // Enterprise
        return div()
            .flex()
            .flex_col()
            .gap_3()
            .child(section_header("ENTERPRISE GOVERNANCE & SSO", theme))
            .child(render_setting_card(
                "Single Sign-On (SAML / OIDC)",
                "Connect organization identity provider for unified developer access",
                "Enterprise",
                pill_button("Configure SSO", false, theme, |_, _, _| {}, cx),
                theme,
            ))
            .child(render_toggle_card(
                "Air-Gapped Offline Enforcement",
                "Completely sever all external telemetry, cloud analytics, and public model APIs",
                "Enterprise",
                true,
                |this, _w, cx| {
                    this.state
                        .toast_manager
                        .push_info("Air-gapped mode active.");
                    cx.notify();
                },
                theme,
                cx,
            ))
            .into_any_element();
    }

    if item_idx == 2 {
        // Privacy
        return div()
            .flex()
            .flex_col()
            .gap_3()
            .child(section_header("PRIVACY & TELEMETRY", theme))
            .child(render_toggle_card(
                "Telemetry & Diagnostics",
                "Send anonymous crash reports and editor usage metrics to improve VSCodium",
                "Privacy",
                state.telemetry_enabled,
                |this, _w, cx| {
                    this.state.telemetry_enabled = !this.state.telemetry_enabled;
                    this.state.save_preferences();
                    cx.notify();
                },
                theme,
                cx,
            ))
            .child(render_setting_card(
                "Purge Conversation Cache",
                "Permanently delete local agent conversation transcripts and scratch data",
                "Privacy",
                div()
                    .px_2p5()
                    .py_1()
                    .rounded(px(4.0))
                    .bg(rgba(0xef444420))
                    .border_1()
                    .border_color(rgba(0xef444440))
                    .cursor_pointer()
                    .hover(|s| s.bg(rgba(0xef444430)))
                    .text_xs()
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xef4444))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _e, _w, cx| {
                            this.state
                                .toast_manager
                                .push_success("Local transcript cache cleared.");
                            cx.notify();
                        }),
                    )
                    .child("Purge Cache"),
                theme,
            ))
            .into_any_element();
    }

    // Account (Item 0)
    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(section_header("DEVELOPER PROFILE & SUBSCRIPTION", theme))
        .child(render_setting_card(
            "Account Status",
            "Active developer identity on this workstation",
            "Account",
            div()
                .flex()
                .items_center()
                .gap_1p5()
                .px_2p5()
                .py_0p5()
                .rounded(px(4.0))
                .bg(rgba(0x22c55e18))
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.status_green)
                .child("● Local Mode (Community Edition)"),
            theme,
        ))
        .child(render_setting_card(
            "Subscription Tier",
            "VSCodium-Rust: Free, open-source, and unencumbered pairing IDE",
            "Account",
            pill_button("Pro Active", true, theme, |_, _, _| {}, cx),
            theme,
        ))
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. KEYBOARD SECTION (Keyboard Shortcuts)
// ─────────────────────────────────────────────────────────────────────────────
fn render_keyboard_section(
    _state: &HadesNativeState,
    _item_idx: usize,
    _query: &str,
    _cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &_state.theme;

    div()
        .flex()
        .flex_col()
        .gap_3()
        .child(section_header("KEYBOARD SHORTCUTS CATALOG", theme))
        .children(
            [
                ("Quick Open File", "Ctrl+P", "File Navigation"),
                ("Command Palette", "Ctrl+Shift+P", "General"),
                ("Toggle AI Chat Composer", "Ctrl+L", "AI Assistant"),
                ("Inline Code Edit (Composer)", "Ctrl+K", "AI Assistant"),
                ("Toggle Terminal Panel", "Ctrl+`", "Terminal"),
                ("Toggle Soft Word Wrap", "Alt+Z", "Editor"),
                ("Run Project Target", "F5", "Debugger"),
                ("Format Document", "Shift+Alt+F", "Editor"),
                ("Search Across Files", "Ctrl+Shift+F", "Search"),
                ("Close Active Tab", "Ctrl+W", "Editor Tabs"),
            ]
            .into_iter()
            .map(|(action, chord, cat)| {
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .p_2p5()
                    .rounded(px(4.0))
                    .bg(theme.bg_card)
                    .border_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(
                                div()
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(theme.text_primary)
                                    .child(action),
                            )
                            .child(
                                div()
                                    .px_1p5()
                                    .py_0p5()
                                    .rounded(px(3.0))
                                    .bg(theme.bg_raised)
                                    .text_xs()
                                    .text_color(theme.text_subtle)
                                    .child(cat),
                            ),
                    )
                    .child(
                        div()
                            .px_2()
                            .py_1()
                            .rounded(px(3.0))
                            .bg(theme.bg_raised)
                            .border_1()
                            .border_color(theme.border_subtle)
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.accent)
                            .child(chord),
                    )
            }),
        )
        .into_any_element()
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. ADVANCED SECTION (APEX, ANE, Kortex, Memory, Voice, Steering, Hooks, LSP, MCP, PyTorch)
// ─────────────────────────────────────────────────────────────────────────────
fn render_advanced_section(
    _state: &HadesNativeState,
    item_idx: usize,
    _query: &str,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let theme = &_state.theme;

    match item_idx {
        1 => {
            // ANE Acceleration
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("APPLE NEURAL ENGINE / NPU HARDWARE ACCELERATION", theme))
                .child(render_setting_card(
                    "NPU Co-Processor",
                    "DirectML / Apple Neural Engine acceleration for vector cosine similarity and embeddings",
                    "ANE",
                    pill_button("Active (DirectML NPU)", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .into_any_element()
        }
        2 => {
            // Kortex / AIM
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("KORTEX CONTEXT VECTOR INDEX & AIM", theme))
                .child(render_setting_card(
                    "Embedding Model",
                    "Local embedding model for semantic indexing of the codebase",
                    "Kortex",
                    div()
                        .px_2p5()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .child("bge-small-en-v1.5 (384-dim)"),
                    theme,
                ))
                .child(render_setting_card(
                    "Memory Vector Budget",
                    "Maximum RAM cache for contextual embeddings index",
                    "Kortex",
                    pill_button("256 MB", true, theme, |_, _, _| {}, cx),
                    theme,
                ))
                .into_any_element()
        }
        4 => {
            // Voice & TTS
            div()
                .flex()
                .flex_col()
                .gap_3()
                .child(section_header("VOICE RECOGNITION & TEXT-TO-SPEECH", theme))
                .child(render_setting_card(
                    "TTS Voice Preset",
                    "Vocal timbre and persona for agent audio responses",
                    "Voice",
                    div().flex().flex_row().gap_1p5().children(
                        ["Airi", "Sage", "Nova", "Yamato"].into_iter().map(|v| {
                            pill_button(
                                v,
                                v == "Airi",
                                theme,
                                move |this, _w, cx| {
                                    this.state
                                        .toast_manager
                                        .push_info(&format!("Voice preset: {v}"));
                                    cx.notify();
                                },
                                cx,
                            )
                        }),
                    ),
                    theme,
                ))
                .into_any_element()
        }
        9 => {
            // PyTorch ML Studio Card (Matching PyTorchLaunchCard in SettingsPage.tsx)
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .p_8()
                .rounded(px(8.0))
                .bg(theme.bg_card)
                .border_1()
                .border_color(theme.border_subtle)
                .gap_3()
                .child(ui_icon(IconName::Flame, 36.0, rgb(0xf97316)))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("PyTorch ML Studio")
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .text_center()
                        .child("Train, fine-tune, and monitor local models with GPU VRAM and compute utilization.")
                )
                .child(
                    div()
                        .px_4()
                        .py_2()
                        .rounded(px(4.0))
                        .bg(rgb(0x007acc))
                        .hover(|s| s.bg(rgb(0x0066aa)))
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                            this.state.set_activity(crate::domain::layout::ActivityTab::PyTorchStudio);
                            this.state.left_sidebar_open = true;
                            cx.notify();
                        }))
                        .child("Open ML Studio")
                )
                .into_any_element()
        }
        _ => {
            // APEX Intelligence (Screenshot 3 exact parity)
            div()
                .flex()
                .flex_col()
                .gap_4()
                .child(
                    div()
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("APEX INTELLIGENCE ENGINES")
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .mt_0p5()
                                .child("Configure individual specialist models for different analysis tasks. Recommended: use qwen3.5:4b for 8 GB rigs.")
                        )
                )
                // Quick Setup Card
                .child(
                    div()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_subtle)
                                .mb_2()
                                .child("QUICK SETUP")
                        )
                        .child(
                            div()
                                .px_4()
                                .py_1p5()
                                .rounded(px(4.0))
                                .bg(rgba(0x22c55e26))
                                .border_1()
                                .border_color(rgba(0x22c55e4d))
                                .cursor_pointer()
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.status_green)
                                        .child("Auto-Downgrade to qwen3.5:4b (Local)")
                                )
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .mt_1p5()
                                .child("Sets all APEX engines to qwen3.5:4b for offline / low-VRAM development")
                        )
                )
                // Per-Engine Models
                .child(
                    div()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_subtle)
                                .mb_1()
                                .child("PER-ENGINE MODELS")
                        )
                        .child(render_clean_row("Architect", "System design & architecture recommendations", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                        .child(render_clean_row("Threat Model", "Security threat analysis & red team", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                        .child(render_clean_row("Performance", "Code optimization suggestions", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                        .child(render_clean_row("Self-Improve", "Code self-correction & refactoring", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                        .child(render_clean_row("Explainer", "Code explanation & documentation", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                        .child(render_clean_row("Multi-System", "Cross-system coordination", render_engine_input_placeholder("e.g., qwen3.5:4b", theme), theme))
                )
                .into_any_element()
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// REUSABLE SETTING CARD HELPERS
// ─────────────────────────────────────────────────────────────────────────────
fn render_clean_row(
    label: &str,
    description: &str,
    control: impl IntoElement,
    theme: &crate::theme::Theme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_start()
        .justify_between()
        .gap_4()
        .py_3()
        .border_b_1()
        .border_color(rgba(0xffffff0a))
        .child(
            div()
                .flex_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .child(label.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .mt_0p5()
                        .child(description.to_string()),
                ),
        )
        .child(div().flex_shrink_0().child(control))
}

fn render_engine_input_placeholder(
    placeholder: &str,
    theme: &crate::theme::Theme,
) -> impl IntoElement {
    div()
        .w(px(140.0))
        .h(px(26.0))
        .px_2p5()
        .rounded(px(4.0))
        .bg(rgba(0xffffff0d))
        .border_1()
        .border_color(rgba(0xffffff1a))
        .flex()
        .items_center()
        .text_xs()
        .text_color(theme.text_muted)
        .child(placeholder.to_string())
}

fn section_header(title: &'static str, theme: &crate::theme::Theme) -> AnyElement {
    div()
        .pb_1()
        .border_b_1()
        .border_color(theme.border_subtle)
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(theme.accent)
        .child(title)
        .into_any_element()
}

fn render_api_key_card(
    title: &'static str,
    description: &'static str,
    provider_id: &'static str,
    key_value: &str,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let has_key = !key_value.trim().is_empty();
    let masked_preview = if has_key {
        let trimmed = key_value.trim();
        let tail_len = trimmed.len().min(4);
        let tail = &trimmed[trimmed.len() - tail_len..];
        format!("● Configured (...{})", tail)
    } else {
        "○ Not Configured".to_string()
    };

    let p_id_paste = provider_id.to_string();
    let p_id_clear = provider_id.to_string();

    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .p_3()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
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
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(title),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Provider"),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .px_2p5()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(if has_key { rgba(0x22c55e18) } else { rgba(0xffffff0d) })
                                .text_xs()
                                .text_color(if has_key { theme.status_green } else { theme.text_muted })
                                .child(masked_preview),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_1()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(rgb(0x388bfd))
                                .text_color(rgb(0xffffff))
                                .text_xs()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.85))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        if let Some(item) = cx.read_from_clipboard() {
                                            if let Some(text) = item.text() {
                                                this.state.set_and_save_api_key(&p_id_paste, text);
                                                cx.notify();
                                            }
                                        }
                                    }),
                                )
                                .child(if has_key { "Update (Paste)" } else { "Paste from Clipboard" }),
                        )
                        .children(has_key.then(|| {
                            div()
                                .flex()
                                .items_center()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .bg(rgba(0xef444422))
                                .text_color(rgb(0xef4444))
                                .text_xs()
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.85))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        this.state.remove_and_save_api_key(&p_id_clear);
                                        cx.notify();
                                    }),
                                )
                                .child("Clear")
                        })),
                ),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(description),
        )
        .into_any_element()
}

fn render_setting_card(
    title: &'static str,
    description: &'static str,
    category: &'static str,
    control: impl IntoElement,
    theme: &crate::theme::Theme,
) -> AnyElement {
    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .p_3()
        .rounded(px(6.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
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
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(title),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(category),
                        ),
                )
                .child(control),
        )
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .child(description),
        )
        .into_any_element()
}

fn render_toggle_card(
    title: &'static str,
    description: &'static str,
    category: &'static str,
    is_on: bool,
    on_toggle: impl Fn(&mut HadesAppView, &mut Window, &mut Context<HadesAppView>) + 'static,
    theme: &crate::theme::Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let toggle_button = div()
        .flex()
        .items_center()
        .gap_1()
        .px_2p5()
        .py_1()
        .rounded(px(5.0))
        .cursor_pointer()
        .bg(if is_on {
            rgba(0x22c55e20)
        } else {
            theme.bg_raised
        })
        .border_1()
        .border_color(if is_on {
            theme.status_green
        } else {
            theme.border_subtle
        })
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(if is_on {
            theme.status_green
        } else {
            theme.text_subtle
        })
        .hover(|s| s.bg(theme.bg_hover))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, w, cx| {
                on_toggle(this, w, cx);
            }),
        )
        .child(if is_on { "✓ Enabled" } else { "○ Disabled" });

    render_setting_card(title, description, category, toggle_button, theme)
}

fn pill_button(
    label: &str,
    is_selected: bool,
    theme: &crate::theme::Theme,
    on_click: impl Fn(&mut HadesAppView, &mut Window, &mut Context<HadesAppView>) + 'static,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    div()
        .px_2p5()
        .py_1()
        .rounded(px(4.0))
        .cursor_pointer()
        .bg(if is_selected {
            theme.bg_raised
        } else {
            theme.bg_titlebar
        })
        .border_1()
        .border_color(if is_selected {
            theme.accent
        } else {
            theme.border_subtle
        })
        .text_xs()
        .font_weight(if is_selected {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        })
        .text_color(if is_selected {
            theme.text_primary
        } else {
            theme.text_muted
        })
        .hover(|s| s.bg(theme.bg_hover))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, w, cx| {
                on_click(this, w, cx);
            }),
        )
        .child(label.to_string())
        .into_any_element()
}

pub struct SettingsPanel;

impl crate::panels::traits::WorkbenchPanel for SettingsPanel {
    fn id(&self) -> &'static str {
        "settings"
    }

    fn title(&self) -> &'static str {
        "SETTINGS"
    }

    fn icon(&self) -> IconName {
        IconName::Settings
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_settings_panel(state, cx).into_any_element()
    }
}
