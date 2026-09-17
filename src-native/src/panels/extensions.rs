use crate::app_state::HadesNativeState;
use crate::editor::engine::vsx::manager::{ExtensionStatus, MarketplaceEntry};
use crate::editor::engine::vsx::manifest::ExtensionManifest;
use crate::theme::Theme;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

fn format_downloads(count: u64) -> String {
    if count >= 1_000_000 {
        format!("{:.1}M", count as f64 / 1_000_000.0)
    } else if count >= 1_000 {
        format!("{:.0}k", count as f64 / 1_000.0)
    } else {
        count.to_string()
    }
}

fn format_rating(rating: f64) -> String {
    if rating <= 0.0 {
        "—".to_string()
    } else {
        format!("{:.1}", rating)
    }
}

pub fn render_extensions_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    let mcp_servers = [
        (
            "filesystem",
            "Local workspace file reader and writer",
            "v0.4.1",
            "modelcontextprotocol",
            true,
        ),
        (
            "git",
            "Direct repository git status, commit, and diff operations",
            "v1.2.0",
            "vscodium-rust",
            true,
        ),
        (
            "fetch",
            "Secure outbound web fetching and markdown conversion",
            "v0.3.0",
            "modelcontextprotocol",
            true,
        ),
        (
            "brave-search",
            "Live web search query and snippet retrieval",
            "v0.1.8",
            "brave",
            true,
        ),
        (
            "code-intelligence",
            "Tree-Sitter syntax AST analyzer and symbol indexer",
            "v2.0.0",
            "vscodium-rust",
            true,
        ),
    ];

    let hermes_skills = [
        (
            "security-auditor",
            "APEX static taint-tracking and zero-trust sandbox scan",
            "v1.0.4",
            "vscodium-rust",
            true,
        ),
        (
            "memory-distiller",
            "Cognitive memory synthesis and vector embedding sync",
            "v1.1.2",
            "vscodium-rust",
            true,
        ),
        (
            "ios-crosscompile",
            "Apple iOS toolchain manager and IPA packaging pipeline",
            "v0.9.5",
            "vscodium-rust",
            true,
        ),
        (
            "git-checkpoints",
            "Automatic rolling shadow-branch recovery points",
            "v1.0.0",
            "vscodium-rust",
            true,
        ),
    ];

    let marketplace_count = state.vsx.marketplace_results.len();
    let installed_count = state.vsx.installed.len();
    let mcp_count = mcp_servers.len();
    let skills_count = hermes_skills.len();

    let categories = [
        format!("Marketplace ({marketplace_count})"),
        format!("Installed ({installed_count})"),
        format!("MCP Servers ({mcp_count})"),
        format!("Hermes Skills ({skills_count})"),
    ];

    let quick_tags = [
        ("Themes", "tag:theme"),
        ("Rust", "rust"),
        ("Python", "python"),
        ("Prettier", "prettier"),
        ("TOML", "toml"),
        ("Git", "git"),
    ];

    let active_cat = state.extensions_category;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Top Header & Search
        .child(
            div()
                .flex()
                .flex_col()
                .p_3()
                .gap_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                // Header Row
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
                                .child(icon_14(IconName::Blocks, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_primary)
                                        .child("EXTENSIONS & OPEN VSX"),
                                ),
                        )
                        .child(if state.vsx.is_searching {
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .text_color(theme.status_yellow)
                                .child(icon_12(IconName::RefreshCw, theme.status_yellow))
                                .child("Searching...")
                        } else {
                            div()
                                .flex()
                                .items_center()
                                .gap_1p5()
                                .text_xs()
                                .text_color(theme.status_green)
                                .child(icon_12(IconName::Activity, theme.status_green))
                                .child("Open VSX Live")
                        }),
                )
                // Interactive Filter / Search Bar
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .h(px(28.0))
                        .px_2()
                        .gap_1p5()
                        .rounded(px(4.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(if state.vsx.search_query.is_empty() {
                            theme.border_subtle
                        } else {
                            theme.accent
                        })
                        .child(icon_12(
                            IconName::Search,
                            if state.vsx.search_query.is_empty() {
                                theme.text_subtle
                            } else {
                                theme.accent
                            },
                        ))
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .text_color(if state.vsx.search_query.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.vsx.search_query.is_empty() {
                                    "Type to search Open VSX marketplace (Enter)...".to_string()
                                } else {
                                    state.vsx.search_query.clone()
                                }),
                        )
                        .children((!state.vsx.search_query.is_empty()).then(|| {
                            div()
                                .cursor_pointer()
                                .p_0p5()
                                .rounded(px(2.0))
                                .hover(|s| s.bg(theme.bg_card))
                                .child(icon_12(IconName::X, theme.text_muted))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.vsx.search_query.clear();
                                        cx.notify();
                                    }),
                                )
                        }))
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.accent)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.85))
                                .child("Search")
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        let q = this.state.vsx.search_query.clone();
                                        this.state.extensions_category = 0;
                                        match this.state.vsx.search_marketplace(&q) {
                                            Ok(()) => {
                                                this.state
                                                    .toast_manager
                                                    .push_success("Open VSX search completed");
                                            }
                                            Err(e) => {
                                                this.state
                                                    .toast_manager
                                                    .push_error(&format!("Search failed: {e}"));
                                            }
                                        }
                                        cx.notify();
                                    }),
                                ),
                        ),
                )
                // Quick Filter Tag Chips
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .flex_wrap()
                        .gap_1()
                        .children(quick_tags.into_iter().map(|(label, tag_query)| {
                            let is_active = state.vsx.search_query == tag_query;
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if is_active {
                                    theme.accent
                                } else {
                                    theme.bg_card
                                })
                                .border_1()
                                .border_color(if is_active {
                                    theme.accent
                                } else {
                                    theme.border_subtle
                                })
                                .text_xs()
                                .text_color(if is_active {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .hover(|s| {
                                    s.text_color(theme.text_primary)
                                        .border_color(theme.border_focus)
                                })
                                .child(label)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener({
                                        let tag_query = tag_query.to_string();
                                        move |this, _e, _w, cx| {
                                            this.state.vsx.search_query = tag_query.clone();
                                            this.state.extensions_category = 0;
                                            let _ = this.state.vsx.search_marketplace(&tag_query);
                                            cx.notify();
                                        }
                                    }),
                                )
                        })),
                ),
        )
        // Category Pills (Marketplace, Installed, MCP, Skills) with dynamic counts
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .flex_wrap()
                .children(categories.into_iter().enumerate().map(|(idx, title)| {
                    let is_sel = idx == active_cat;
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_full()
                        .cursor_pointer()
                        .bg(if is_sel {
                            theme.accent
                        } else {
                            theme.bg_raised
                        })
                        .border_1()
                        .border_color(if is_sel {
                            theme.accent
                        } else {
                            theme.border_subtle
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
                        .hover(|s| s.text_color(theme.text_primary))
                        .child(title)
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _e, _w, cx| {
                                this.state.extensions_category = idx;
                                cx.notify();
                            }),
                        )
                })),
        )
        // Scrollable Content Area based on Category
        .child(
            div()
                .id("extensions_list_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .p_2()
                .gap_2()
                .child(match active_cat {
                    0 => render_marketplace_category(state, theme, cx).into_any_element(),
                    1 => render_installed_category(state, theme, cx).into_any_element(),
                    2 => render_mcp_category(&mcp_servers, theme).into_any_element(),
                    3 => render_hermes_category(&hermes_skills, theme).into_any_element(),
                    _ => div().into_any_element(),
                }),
        )
}

/// Category 0: Marketplace / Open VSX Search Results & Featured Extensions
fn render_marketplace_category(
    state: &HadesNativeState,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let results = &state.vsx.marketplace_results;

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_1()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(icon_12(IconName::Package, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.accent)
                                .child(if state.vsx.search_query.is_empty() {
                                    "POPULAR OPEN VSX PACKAGES"
                                } else {
                                    "SEARCH RESULTS"
                                }),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(format!("{} found", results.len())),
                ),
        )
        // Error banner if search had an issue
        .children(state.vsx.error_message.as_ref().map(|err| {
            div()
                .p_2()
                .rounded(px(4.0))
                .bg(theme.bg_raised)
                .border_1()
                .border_color(theme.status_red)
                .flex()
                .flex_row()
                .items_center()
                .gap_2()
                .child(icon_12(IconName::CircleAlert, theme.status_red))
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.status_red)
                        .child(err.clone()),
                )
        }))
        // Results cards
        .children(
            results
                .iter()
                .map(|entry| render_marketplace_card(entry, theme, cx)),
        )
}

fn render_marketplace_card(
    entry: &MarketplaceEntry,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let is_installed = matches!(entry.status, ExtensionStatus::Installed);
    let downloads_str = format_downloads(entry.download_count);
    let rating_str = format_rating(entry.average_rating);

    div()
        .flex()
        .flex_col()
        .p_2p5()
        .rounded(px(5.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .gap_1p5()
        .hover(|s| s.border_color(theme.border_focus))
        // Top row: Name, namespace badge, version
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
                        .child(icon_14(IconName::Package, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(entry.display_name.clone()),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(entry.namespace.clone()),
                        ),
                )
                .child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("v{}", entry.version)),
                ),
        )
        // Description
        .child(
            div()
                .text_xs()
                .text_color(theme.text_muted)
                .line_clamp(2)
                .child(entry.description.clone()),
        )
        // Bottom row: stats (downloads, rating) + action button
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .pt_1()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2p5()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(icon_12(IconName::Download, theme.text_subtle))
                                .child(downloads_str),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .child(icon_12(IconName::Star, theme.status_yellow))
                                .child(rating_str),
                        ),
                )
                // Action Button: Install vs Installed
                .child(if is_installed {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.status_green)
                        .text_xs()
                        .text_color(theme.status_green)
                        .child(icon_12(IconName::Check, theme.status_green))
                        .child("Installed")
                } else {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.accent)
                        .text_xs()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .child(icon_12(IconName::Download, theme.text_primary))
                        .child("Install")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let publisher = entry.namespace.clone();
                                let name = entry.name.clone();
                                let version = entry.version.clone();
                                move |this, _e, _w, cx| {
                                    match this
                                        .state
                                        .vsx
                                        .install_extension(&publisher, &name, &version)
                                    {
                                        Ok(id) => {
                                            this.state
                                                .toast_manager
                                                .push_success(&format!("Installed {}", id));
                                        }
                                        Err(e) => {
                                            this.state
                                                .toast_manager
                                                .push_error(&format!("Install failed: {e}"));
                                        }
                                    }
                                    cx.notify();
                                }
                            }),
                        )
                }),
        )
}

/// Category 1: Installed (.vsix extensions)
fn render_installed_category(
    state: &HadesNativeState,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let installed = &state.vsx.installed;

    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_1()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(icon_12(IconName::Check, theme.status_green))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.status_green)
                                .child("INSTALLED .VSIX EXTENSIONS")
                        )
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(format!("{} active", installed.len()))
                )
        )
        // If no extensions installed yet, show friendly empty state
        .child(if installed.is_empty() {
            div()
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .p_6()
                .gap_2()
                .bg(theme.bg_card)
                .rounded(px(6.0))
                .border_1()
                .border_color(theme.border_subtle)
                .child(icon_14(IconName::Package, theme.text_subtle))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_primary)
                        .child("No Extensions Installed Yet")
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .text_center()
                        .child("Browse the Open VSX marketplace to install themes, snippets, and language extensions.")
                )
                .child(
                    div()
                        .px_3()
                        .py_1()
                        .rounded(px(4.0))
                        .bg(theme.accent)
                        .text_xs()
                        .text_color(theme.text_primary)
                        .cursor_pointer()
                        .hover(|s| s.opacity(0.85))
                        .child("Browse Marketplace")
                        .on_mouse_down(MouseButton::Left, cx.listener(|this, _e, _w, cx| {
                            this.state.extensions_category = 0;
                            cx.notify();
                        }))
                )
                .into_any_element()
        } else {
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(installed.iter().map(|manifest| {
                    render_installed_card(manifest, theme, cx)
                }))
                .into_any_element()
        })
}

fn render_installed_card(
    manifest: &ExtensionManifest,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let id = manifest.id();
    let display_title = manifest.display_title().to_string();
    let desc = manifest
        .description
        .clone()
        .unwrap_or_else(|| "No description provided.".to_string());

    let (theme_count, snippet_count, lang_count, cmd_count) =
        if let Some(contribs) = &manifest.contributes {
            (
                contribs.themes.len(),
                contribs.snippets.len(),
                contribs.languages.len(),
                contribs.commands.len(),
            )
        } else {
            (0, 0, 0, 0)
        };

    div()
        .flex()
        .flex_col()
        .p_2p5()
        .rounded(px(5.0))
        .bg(theme.bg_card)
        .border_1()
        .border_color(theme.border_subtle)
        .gap_1p5()
        .hover(|s| s.border_color(theme.border_focus))
        // Top row
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
                        .child(icon_14(IconName::Package, theme.status_green))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(display_title),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(manifest.publisher.clone()),
                        ),
                )
                .child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("v{}", manifest.version)),
                ),
        )
        // Description
        .child(div().text_xs().text_color(theme.text_muted).child(desc))
        // Contributes badges row
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .flex_wrap()
                .gap_1p5()
                .children((theme_count > 0).then(|| {
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.accent)
                        .child(format!("{theme_count} theme(s)"))
                }))
                .children((snippet_count > 0).then(|| {
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.status_yellow)
                        .child(format!("{snippet_count} snippet(s)"))
                }))
                .children((lang_count > 0).then(|| {
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.status_green)
                        .child(format!("{lang_count} lang(s)"))
                }))
                .children((cmd_count > 0).then(|| {
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(format!("{cmd_count} cmd(s)"))
                })),
        )
        // Bottom row: Path preview & Uninstall button
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .pt_1()
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(id.clone()),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.status_red)
                        .cursor_pointer()
                        .hover(|s| {
                            s.bg(theme.status_red)
                                .text_color(theme.text_primary)
                                .border_color(theme.status_red)
                        })
                        .child(icon_12(IconName::Trash, theme.status_red))
                        .child("Uninstall")
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener({
                                let ext_id = id.clone();
                                move |this, _e, _w, cx| {
                                    match this.state.vsx.uninstall_extension(&ext_id) {
                                        Ok(()) => {
                                            this.state
                                                .toast_manager
                                                .push_success(&format!("Uninstalled {}", ext_id));
                                        }
                                        Err(e) => {
                                            this.state
                                                .toast_manager
                                                .push_error(&format!("Uninstall failed: {e}"));
                                        }
                                    }
                                    cx.notify();
                                }
                            }),
                        ),
                ),
        )
}

/// Category 2: MCP Protocol Servers
fn render_mcp_category(
    mcp_servers: &[(&'static str, &'static str, &'static str, &'static str, bool)],
    theme: &Theme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .px_1()
                .child(icon_12(IconName::Server, theme.accent))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.accent)
                        .child("ACTIVE MCP PROTOCOL SERVERS"),
                ),
        )
        .children(
            mcp_servers
                .iter()
                .map(|(name, desc, ver, author, enabled)| {
                    div()
                        .flex()
                        .flex_col()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1()
                        .hover(|s| s.border_color(theme.border_focus))
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
                                        .gap_1p5()
                                        .child(icon_12(IconName::Package, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child(*name),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .text_xs()
                                        .text_color(if *enabled {
                                            theme.status_green
                                        } else {
                                            theme.text_subtle
                                        })
                                        .child(if *enabled { "Running" } else { "Disabled" }),
                                ),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(*desc))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(format!("{author} • {ver}"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .cursor_pointer()
                                        .text_color(theme.accent)
                                        .hover(|s| s.text_color(theme.text_primary))
                                        .child(icon_12(IconName::Settings, theme.accent))
                                        .child("Configure"),
                                ),
                        )
                }),
        )
}

/// Category 3: Hermes Agent Skills
fn render_hermes_category(
    hermes_skills: &[(&'static str, &'static str, &'static str, &'static str, bool)],
    theme: &Theme,
) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_1p5()
                .px_1()
                .child(icon_12(IconName::Brain, theme.status_yellow))
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.status_yellow)
                        .child("HERMES AGENT SKILLS"),
                ),
        )
        .children(
            hermes_skills
                .iter()
                .map(|(name, desc, ver, author, _enabled)| {
                    div()
                        .flex()
                        .flex_col()
                        .p_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .gap_1()
                        .hover(|s| s.border_color(theme.border_focus))
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
                                        .child(icon_12(IconName::Sparkles, theme.status_yellow))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child(*name),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .text_xs()
                                        .text_color(theme.status_green)
                                        .child(icon_12(IconName::Check, theme.status_green))
                                        .child("Verified"),
                                ),
                        )
                        .child(div().text_xs().text_color(theme.text_muted).child(*desc))
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child(format!("{author} • {ver}"))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .cursor_pointer()
                                        .text_color(theme.accent)
                                        .hover(|s| s.text_color(theme.text_primary))
                                        .child(icon_12(IconName::ExternalLink, theme.accent))
                                        .child("Inspect"),
                                ),
                        )
                }),
        )
}

pub struct ExtensionsPanel;

impl crate::panels::traits::WorkbenchPanel for ExtensionsPanel {
    fn id(&self) -> &'static str {
        "extensions"
    }

    fn title(&self) -> &'static str {
        "EXTENSIONS & MCP"
    }

    fn icon(&self) -> IconName {
        IconName::Blocks
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_extensions_panel(state, cx).into_any_element()
    }
}
