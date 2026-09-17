use crate::app_state::HadesNativeState;
use crate::panels::traits::AuxiliaryTab;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use vscode_rust_app::vector_indexer::{IndexStats, SearchResult, VectorIndexer};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VectorSearchMode {
    Code,
    Symbol,
}

#[derive(Clone)]
pub struct VectorSearchState {
    pub query: String,
    pub query_focused: bool,
    pub mode: VectorSearchMode,
    pub loading: Arc<AtomicBool>,
    pub error: Arc<Mutex<String>>,
    pub results: Arc<Mutex<Vec<SearchResult>>>,
    pub stats: Arc<Mutex<Option<IndexStats>>>,
    pub indexer: Option<Arc<VectorIndexer>>,
}

impl Default for VectorSearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            query_focused: false,
            mode: VectorSearchMode::Code,
            loading: Arc::new(AtomicBool::new(false)),
            error: Arc::new(Mutex::new(String::new())),
            results: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(Mutex::new(None)),
            indexer: None,
        }
    }
}

impl VectorSearchState {
    pub fn is_loading(&self) -> bool {
        self.loading.load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn error_snapshot(&self) -> String {
        self.error.lock().map(|s| s.clone()).unwrap_or_default()
    }

    pub fn results_snapshot(&self) -> Vec<SearchResult> {
        self.results.lock().map(|r| r.clone()).unwrap_or_default()
    }

    pub fn stats_snapshot(&self) -> Option<IndexStats> {
        self.stats.lock().ok().and_then(|s| s.clone())
    }

    fn set_loading(&self, v: bool) {
        self.loading.store(v, std::sync::atomic::Ordering::SeqCst);
    }

    fn set_error(&self, msg: &str) {
        if let Ok(mut e) = self.error.lock() {
            *e = msg.to_string();
        }
    }
}

fn fallback_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("vscodium-rust")
}

fn open_indexer(root: &Path) -> Option<VectorIndexer> {
    let candidates = [root.to_path_buf(), root.join(".kortex").to_path_buf()];
    for c in &candidates {
        if let Ok(ix) = VectorIndexer::new(c.clone(), fallback_dir()) {
            return Some(ix);
        }
    }
    VectorIndexer::new(root.to_path_buf(), fallback_dir()).ok()
}

pub fn execute_search(state: &mut VectorSearchState, root: &Path) {
    let query = state.query.trim().to_string();
    if query.is_empty() || state.is_loading() {
        return;
    }
    state.set_loading(true);
    state.set_error("");
    if let Ok(mut results) = state.results.lock() {
        results.clear();
    }

    let indexer = match state.indexer.clone() {
        Some(ix) => ix,
        None => match open_indexer(root) {
            Some(ix) => {
                let ix = Arc::new(ix);
                state.indexer = Some(ix.clone());
                ix
            }
            None => {
                state.set_loading(false);
                state.set_error("Could not open vector index");
                return;
            }
        },
    };

    let mode = state.mode;
    let loading = state.loading.clone();
    let error = state.error.clone();
    let results = state.results.clone();

    std::thread::Builder::new()
        .name("vector-search".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            if let Ok(rt) = rt {
                let r = rt.block_on(async {
                    match mode {
                        VectorSearchMode::Code => indexer.search_codebase(&query, 20).await,
                        VectorSearchMode::Symbol => indexer.find_symbol(&query).await,
                    }
                });
                match r {
                    Ok(res) => {
                        if let Ok(mut slot) = results.lock() {
                            *slot = res;
                        }
                    }
                    Err(e) => {
                        if let Ok(mut slot) = results.lock() {
                            slot.clear();
                        }
                        if let Ok(mut err) = error.lock() {
                            *err = format!("{e}");
                        }
                    }
                }
                loading.store(false, std::sync::atomic::Ordering::SeqCst);
            }
        })
        .ok();
}

pub fn refresh_stats(state: &mut VectorSearchState, root: &Path) {
    let indexer = match state.indexer.clone() {
        Some(ix) => ix,
        None => match open_indexer(root) {
            Some(ix) => {
                let ix = Arc::new(ix);
                state.indexer = Some(ix.clone());
                ix
            }
            None => return,
        },
    };
    let stats = state.stats.clone();
    std::thread::Builder::new()
        .name("vector-stats".to_string())
        .spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build();
            if let Ok(rt) = rt {
                let r = rt.block_on(async { indexer.get_index_stats().await });
                if let Ok(mut slot) = stats.lock() {
                    *slot = r.ok();
                }
            }
        })
        .ok();
}

pub struct VectorSearchPanel;

impl AuxiliaryTab for VectorSearchPanel {
    fn id(&self) -> &'static str {
        "vector_search"
    }

    fn title(&self) -> &'static str {
        "Vector Search"
    }

    fn icon(&self) -> IconName {
        IconName::Search
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_vector_search_panel(state, cx).into_any_element()
    }
}

pub fn render_vector_search_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let vs = &state.vector_search;
    let results = vs.results_snapshot();
    let stats = vs.stats_snapshot();
    let loading = vs.is_loading();
    let error = vs.error_snapshot();
    let is_symbol = vs.mode == VectorSearchMode::Symbol;

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(32.0))
                .px_3()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(icon_12(IconName::Search, theme.text_muted))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("CODEBASE SEARCH"),
                        ),
                )
                .child(
                    div()
                        .p_1()
                        .rounded(px(3.0))
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                let root = this.state.workspace_root.clone();
                                refresh_stats(&mut this.state.vector_search, &root);
                                cx.notify();
                            }),
                        )
                        .child(icon_12(IconName::RefreshCw, theme.text_muted)),
                ),
        )
        .children(stats.map(|s| {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap_3()
                .px_3()
                .py_1p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .opacity(0.7)
                .text_color(theme.text_muted)
                .child(format!("{} files", s.total_files))
                .child(format!("{} chunks", s.total_chunks))
                .child(format!("{} symbols", s.total_symbols))
                .children((!s.languages.is_empty()).then(|| {
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(s.languages.keys().cloned().collect::<Vec<_>>().join(", "))
                }))
        }))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .p_3()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_1()
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .cursor_pointer()
                                .bg(if !is_symbol {
                                    theme.accent
                                } else {
                                    theme.bg_raised
                                })
                                .text_color(if !is_symbol {
                                    theme.text_on_accent
                                } else {
                                    theme.text_muted
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.vector_search.mode = VectorSearchMode::Code;
                                        cx.notify();
                                    }),
                                )
                                .child("Code Search"),
                        )
                        .child(
                            div()
                                .flex_1()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .cursor_pointer()
                                .bg(if is_symbol {
                                    theme.accent
                                } else {
                                    theme.bg_raised
                                })
                                .text_color(if is_symbol {
                                    theme.text_on_accent
                                } else {
                                    theme.text_muted
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.vector_search.mode = VectorSearchMode::Symbol;
                                        cx.notify();
                                    }),
                                )
                                .child("Symbol Search"),
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
                                .flex_1()
                                .flex()
                                .items_center()
                                .px_2()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(if vs.query_focused {
                                    theme.bg_input
                                } else {
                                    theme.bg_raised
                                })
                                .border_1()
                                .border_color(if vs.query_focused {
                                    theme.accent
                                } else {
                                    theme.border_subtle
                                })
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_input))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.vector_search.query_focused = true;
                                        cx.notify();
                                    }),
                                )
                                .child(if vs.query.is_empty() {
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(if is_symbol {
                                            "Search symbols..."
                                        } else {
                                            "Search code..."
                                        })
                                        .into_any_element()
                                } else {
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_primary)
                                        .truncate()
                                        .child(format!(
                                            "{}{}",
                                            vs.query,
                                            if vs.query_focused { "▎" } else { "" }
                                        ))
                                        .into_any_element()
                                }),
                        )
                        .child(
                            div()
                                .px_3()
                                .py_1()
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.opacity(0.85))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        execute_search(&mut this.state.vector_search, &root);
                                        this.state.vector_search.query_focused = false;
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        .child(ui_icon(
                                            IconName::Search,
                                            12.0,
                                            theme.text_on_accent,
                                        ))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_on_accent)
                                                .font_weight(FontWeight::BOLD)
                                                .child(if loading {
                                                    "Searching…"
                                                } else {
                                                    "Search"
                                                }),
                                        ),
                                ),
                        ),
                )
                .children(
                    (!error.is_empty())
                        .then(|| div().text_xs().text_color(theme.status_red).child(error)),
                ),
        )
        .child(
            div()
                .id("vector_results_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .border_t_1()
                .border_color(theme.border_subtle)
                .children(if results.is_empty() && !loading {
                    vec![div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .gap_2()
                        .p_6()
                        .opacity(0.5)
                        .text_color(theme.text_muted)
                        .text_center()
                        .child(icon_12(IconName::Search, theme.text_muted))
                        .child(div().text_xs().child("Search your codebase semantically."))
                        .child(
                            div()
                                .text_xs()
                                .child("Find code by meaning, context, and symbols."),
                        )
                        .into_any_element()]
                } else {
                    results
                        .iter()
                        .map(|r| {
                            div()
                                .flex()
                                .flex_col()
                                .gap_1()
                                .px_3()
                                .py_2()
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1()
                                                .child(icon_12(IconName::File, theme.text_muted))
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .truncate()
                                                        .text_color(theme.text_primary)
                                                        .child(r.file_path.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded_full()
                                                .bg(theme.bg_raised)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(format!(
                                                    "L{}-L{}",
                                                    r.start_line, r.end_line
                                                )),
                                        ),
                                )
                                .children((!r.context.is_empty()).then(|| {
                                    div()
                                        .p_2()
                                        .rounded(px(4.0))
                                        .bg(rgb(0x0e1014))
                                        .font_family("Cascadia Code, Consolas, monospace")
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(
                                            div()
                                                .max_h(px(100.0))
                                                .overflow_hidden()
                                                .child(r.context.clone()),
                                        )
                                }))
                                .children((r.relevance_score > 0.0).then(|| {
                                    div()
                                        .text_xs()
                                        .opacity(0.6)
                                        .text_color(theme.text_muted)
                                        .child(format!(
                                            "Relevance: {:.1}%",
                                            r.relevance_score.min(10.0) * 10.0
                                        ))
                                }))
                                .into_any_element()
                        })
                        .collect()
                }),
        )
        .into_any_element()
}
