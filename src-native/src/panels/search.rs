use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::collections::BTreeMap;
use std::path::Path;

#[derive(Clone, Debug)]
pub struct SearchMatch {
    pub file_path: String,
    pub relative_path: String,
    pub line_number: usize,
    pub line_content: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum SearchFocusedField {
    #[default]
    Query,
    Replace,
    Include,
    Exclude,
}

#[derive(Clone, Debug, Default)]
pub struct SearchState {
    pub query: String,
    pub replace: String,
    pub is_searching: bool,
    pub case_sensitive: bool,
    pub whole_word: bool,
    pub is_regex: bool,
    pub show_replace: bool,
    pub show_details: bool,
    pub include_pattern: String,
    pub exclude_pattern: String,
    pub regex_error: Option<String>,
    pub matches: Vec<SearchMatch>,
    pub collapsed_files: Vec<String>,
    pub focused_field: SearchFocusedField,
}

pub fn matches_pattern(path: &str, pattern: &str) -> bool {
    let pat = pattern.trim();
    if pat.is_empty() {
        return true;
    }
    let norm_path = path.replace('\\', "/");
    for sub_pat in pat.split(',') {
        let sub = sub_pat.trim().replace('\\', "/");
        if sub.is_empty() {
            continue;
        }
        if sub.starts_with('*') {
            let ext = &sub[1..];
            if norm_path.ends_with(ext) {
                return true;
            }
        } else if sub.ends_with('*') {
            let prefix = &sub[..sub.len() - 1];
            if norm_path.starts_with(prefix) {
                return true;
            }
        } else if norm_path.contains(&sub) {
            return true;
        }
    }
    false
}

impl SearchState {
    pub fn execute_search(&mut self, root: &Path) {
        let q = self.query.trim();
        if q.is_empty() {
            self.matches.clear();
            self.regex_error = None;
            return;
        }

        self.is_searching = true;
        let mut results = Vec::new();

        let compiled_re = if self.is_regex {
            let mut b = regex::RegexBuilder::new(q);
            b.case_insensitive(!self.case_sensitive);
            match b.build() {
                Ok(re) => {
                    self.regex_error = None;
                    Some(re)
                }
                Err(e) => {
                    self.regex_error = Some(format!("Regex error: {e}"));
                    self.matches.clear();
                    self.is_searching = false;
                    return;
                }
            }
        } else {
            self.regex_error = None;
            None
        };

        let target_query = if self.case_sensitive {
            q.to_string()
        } else {
            q.to_lowercase()
        };

        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_entry(|e| {
                let name = e.file_name().to_string_lossy();
                name != ".git" && name != "target" && name != "node_modules"
            })
            .flatten()
        {
            if entry.file_type().is_file() {
                let p = entry.path();
                let rel = p
                    .strip_prefix(root)
                    .unwrap_or(p)
                    .to_string_lossy()
                    .to_string();

                if !self.include_pattern.trim().is_empty()
                    && !matches_pattern(&rel, &self.include_pattern)
                {
                    continue;
                }
                if !self.exclude_pattern.trim().is_empty()
                    && matches_pattern(&rel, &self.exclude_pattern)
                {
                    continue;
                }

                if let Ok(content) = std::fs::read_to_string(p) {
                    for (line_idx, line) in content.lines().enumerate() {
                        let is_match = if let Some(ref re) = compiled_re {
                            re.is_match(line)
                        } else {
                            let check = if self.case_sensitive {
                                line.to_string()
                            } else {
                                line.to_lowercase()
                            };
                            if self.whole_word {
                                check
                                    .split(|c: char| !c.is_alphanumeric() && c != '_')
                                    .any(|w| w == target_query)
                            } else {
                                check.contains(&target_query)
                            }
                        };

                        if is_match {
                            results.push(SearchMatch {
                                file_path: p.to_string_lossy().to_string(),
                                relative_path: rel.clone(),
                                line_number: line_idx + 1,
                                line_content: line.trim().to_string(),
                            });
                            if results.len() >= 500 {
                                break;
                            }
                        }
                    }
                }
            }
            if results.len() >= 500 {
                break;
            }
        }

        self.matches = results;
        self.is_searching = false;
    }

    pub fn replace_all(&mut self, root: &Path) -> usize {
        if self.query.is_empty() || self.matches.is_empty() {
            return 0;
        }

        let mut replaced_files = 0;
        let mut files_to_update: Vec<String> =
            self.matches.iter().map(|m| m.file_path.clone()).collect();
        files_to_update.sort();
        files_to_update.dedup();

        for file_path in files_to_update {
            if self.replace_file_content(&file_path) {
                replaced_files += 1;
            }
        }

        self.execute_search(root);
        replaced_files
    }

    pub fn replace_file(&mut self, file_path: &str, root: &Path) -> bool {
        let ok = self.replace_file_content(file_path);
        if ok {
            self.execute_search(root);
        }
        ok
    }

    fn replace_file_content(&self, file_path: &str) -> bool {
        if let Ok(content) = std::fs::read_to_string(file_path) {
            let new_content = if self.is_regex {
                let mut b = regex::RegexBuilder::new(&self.query);
                b.case_insensitive(!self.case_sensitive);
                if let Ok(re) = b.build() {
                    re.replace_all(&content, self.replace.as_str()).to_string()
                } else {
                    return false;
                }
            } else if self.case_sensitive {
                content.replace(&self.query, &self.replace)
            } else {
                let mut result = String::new();
                let mut last = 0;
                let lower_content = content.to_lowercase();
                let lower_query = self.query.to_lowercase();
                while let Some(pos) = lower_content[last..].find(&lower_query) {
                    let actual_pos = last + pos;
                    result.push_str(&content[last..actual_pos]);
                    result.push_str(&self.replace);
                    last = actual_pos + self.query.len();
                }
                result.push_str(&content[last..]);
                result
            };

            std::fs::write(file_path, new_content).is_ok()
        } else {
            false
        }
    }
}

pub fn render_search_panel(
    state: &HadesNativeState,
    search: &SearchState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let match_count = search.matches.len();

    // Group matches by file
    let mut grouped_matches: BTreeMap<String, Vec<&SearchMatch>> = BTreeMap::new();
    for m in &search.matches {
        grouped_matches
            .entry(m.relative_path.clone())
            .or_default()
            .push(m);
    }
    let file_count = grouped_matches.len();
    let all_file_keys: Vec<String> = grouped_matches.keys().cloned().collect();
    let is_all_collapsed =
        !all_file_keys.is_empty() && search.collapsed_files.len() >= all_file_keys.len();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Title Bar
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(35.0))
                .px_3()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.text_muted)
                        .child("SEARCH"),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(div().text_xs().text_color(theme.text_subtle).child(
                            if match_count > 0 {
                                format!("{match_count} in {file_count} files")
                            } else {
                                "No results".to_string()
                            },
                        ))
                        // Collapse All / Expand All button
                        .child(
                            div()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        if is_all_collapsed {
                                            this.search.collapsed_files.clear();
                                        } else {
                                            this.search.collapsed_files = all_file_keys.clone();
                                        }
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(
                                    if is_all_collapsed {
                                        IconName::ChevronRight
                                    } else {
                                        IconName::ChevronDown
                                    },
                                    theme.text_muted,
                                )),
                        )
                        // Refresh button
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
                                        this.search.execute_search(&root);
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::RefreshCw, theme.text_muted)),
                        ),
                ),
        )
        // Input Controls (Search & Replace & Pattern Details)
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .gap_1p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                // Search Input Row with embedded filters (Aa, \b, .*)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(
                            // Toggle replace chevron
                            div()
                                .p_1()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.search.show_replace = !this.search.show_replace;
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(if search.show_replace {
                                    icon_12(IconName::ChevronDown, theme.text_muted)
                                } else {
                                    icon_12(IconName::ChevronRight, theme.text_muted)
                                }),
                        )
                        .child({
                            let is_query_focused =
                                search.focused_field == SearchFocusedField::Query;
                            div()
                                .flex()
                                .flex_1()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(px(26.0))
                                .px_2()
                                .rounded(px(4.0))
                                .cursor_text()
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(if is_query_focused {
                                    theme.accent
                                } else {
                                    theme.border_composer
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.search.focused_field = SearchFocusedField::Query;
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(ui_icon(
                                            IconName::Search,
                                            12.0,
                                            if is_query_focused {
                                                theme.accent
                                            } else {
                                                theme.text_subtle
                                            },
                                        ))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if search.query.is_empty() {
                                                    theme.text_subtle
                                                } else {
                                                    theme.text_primary
                                                })
                                                .child(if search.query.is_empty() {
                                                    if is_query_focused {
                                                        "Type to search in files...".to_string()
                                                    } else {
                                                        "Search (Ctrl+Shift+F)...".to_string()
                                                    }
                                                } else {
                                                    format!(
                                                        "{}{}",
                                                        search.query,
                                                        if is_query_focused { "▎" } else { "" }
                                                    )
                                                }),
                                        ),
                                )
                                // Filter action buttons
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1()
                                        // Case Sensitive (Aa)
                                        .child(
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .cursor_pointer()
                                                .bg(if search.case_sensitive {
                                                    theme.accent
                                                } else {
                                                    theme.bg_input
                                                })
                                                .text_color(if search.case_sensitive {
                                                    theme.text_on_accent
                                                } else {
                                                    theme.text_subtle
                                                })
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.search.case_sensitive =
                                                            !this.search.case_sensitive;
                                                        let root =
                                                            this.state.workspace_root.clone();
                                                        this.search.execute_search(&root);
                                                        this.state.search = this.search.clone();
                                                        cx.notify();
                                                    }),
                                                )
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .child("Aa"),
                                        )
                                        // Whole word (\b)
                                        .child(
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .cursor_pointer()
                                                .bg(if search.whole_word {
                                                    theme.accent
                                                } else {
                                                    theme.bg_input
                                                })
                                                .text_color(if search.whole_word {
                                                    theme.text_on_accent
                                                } else {
                                                    theme.text_subtle
                                                })
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.search.whole_word =
                                                            !this.search.whole_word;
                                                        let root =
                                                            this.state.workspace_root.clone();
                                                        this.search.execute_search(&root);
                                                        this.state.search = this.search.clone();
                                                        cx.notify();
                                                    }),
                                                )
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .child(r"\b"),
                                        )
                                        // Regex (.*)
                                        .child(
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .cursor_pointer()
                                                .bg(if search.is_regex {
                                                    theme.accent
                                                } else {
                                                    theme.bg_input
                                                })
                                                .text_color(if search.is_regex {
                                                    theme.text_on_accent
                                                } else {
                                                    theme.text_subtle
                                                })
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.search.is_regex =
                                                            !this.search.is_regex;
                                                        let root =
                                                            this.state.workspace_root.clone();
                                                        this.search.execute_search(&root);
                                                        this.state.search = this.search.clone();
                                                        cx.notify();
                                                    }),
                                                )
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .child(".*"),
                                        ),
                                )
                        })
                        // Toggle Details button (...)
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if search.show_details {
                                    theme.accent.opacity(0.18)
                                } else {
                                    theme.bg_input
                                })
                                .text_color(if search.show_details {
                                    theme.accent
                                } else {
                                    theme.text_subtle
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.search.show_details = !this.search.show_details;
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .child("..."),
                        ),
                )
                // Regex Error Display
                .children(search.regex_error.as_ref().map(|err| {
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.status_red.opacity(0.12))
                        .text_xs()
                        .text_color(theme.status_red)
                        .child(err.clone())
                }))
                // Replace Row (Shown when toggled)
                .children(search.show_replace.then(|| {
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .pl(px(20.0))
                        .child({
                            let is_replace_focused =
                                search.focused_field == SearchFocusedField::Replace;
                            div()
                                .flex()
                                .flex_1()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(px(26.0))
                                .px_2()
                                .rounded(px(4.0))
                                .cursor_text()
                                .bg(theme.bg_input)
                                .border_1()
                                .border_color(if is_replace_focused {
                                    theme.accent
                                } else {
                                    theme.border_composer
                                })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.search.focused_field = SearchFocusedField::Replace;
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(if search.replace.is_empty() {
                                            theme.text_subtle
                                        } else {
                                            theme.text_primary
                                        })
                                        .child(if search.replace.is_empty() {
                                            if is_replace_focused {
                                                "Type replace term...".to_string()
                                            } else {
                                                "Replace...".to_string()
                                            }
                                        } else {
                                            format!(
                                                "{}{}",
                                                search.replace,
                                                if is_replace_focused { "▎" } else { "" }
                                            )
                                        }),
                                )
                        })
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .justify_center()
                                .px_2()
                                .h(px(26.0))
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .text_xs()
                                .text_color(theme.text_primary)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        let count = this.search.replace_all(&root);
                                        this.state.search = this.search.clone();
                                        this.state.toast_manager.push_success(&format!(
                                            "Replaced matches across {count} files"
                                        ));
                                        cx.notify();
                                    }),
                                )
                                .child("Replace All"),
                        )
                }))
                // Details Row: files to include / exclude (Shown when toggled)
                .children(search.show_details.then(|| {
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .pt_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .w(px(70.0))
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("files to include"),
                                )
                                .child({
                                    let is_include_focused =
                                        search.focused_field == SearchFocusedField::Include;
                                    div()
                                        .flex_1()
                                        .h(px(22.0))
                                        .px_2()
                                        .rounded(px(3.0))
                                        .cursor_text()
                                        .bg(theme.bg_input)
                                        .border_1()
                                        .border_color(if is_include_focused {
                                            theme.accent
                                        } else {
                                            theme.border_composer
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.search.focused_field =
                                                    SearchFocusedField::Include;
                                                this.state.search = this.search.clone();
                                                cx.notify();
                                            }),
                                        )
                                        .flex()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if search.include_pattern.is_empty() {
                                                    theme.text_subtle
                                                } else {
                                                    theme.text_primary
                                                })
                                                .child(if search.include_pattern.is_empty() {
                                                    "e.g. *.rs, src/*".to_string()
                                                } else {
                                                    format!(
                                                        "{}{}",
                                                        search.include_pattern,
                                                        if is_include_focused { "▎" } else { "" }
                                                    )
                                                }),
                                        )
                                }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(
                                    div()
                                        .w(px(70.0))
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child("files to exclude"),
                                )
                                .child({
                                    let is_exclude_focused =
                                        search.focused_field == SearchFocusedField::Exclude;
                                    div()
                                        .flex_1()
                                        .h(px(22.0))
                                        .px_2()
                                        .rounded(px(3.0))
                                        .cursor_text()
                                        .bg(theme.bg_input)
                                        .border_1()
                                        .border_color(if is_exclude_focused {
                                            theme.accent
                                        } else {
                                            theme.border_composer
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                this.search.focused_field =
                                                    SearchFocusedField::Exclude;
                                                this.state.search = this.search.clone();
                                                cx.notify();
                                            }),
                                        )
                                        .flex()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(if search.exclude_pattern.is_empty() {
                                                    theme.text_subtle
                                                } else {
                                                    theme.text_primary
                                                })
                                                .child(if search.exclude_pattern.is_empty() {
                                                    "e.g. target/*, *.min.js".to_string()
                                                } else {
                                                    format!(
                                                        "{}{}",
                                                        search.exclude_pattern,
                                                        if is_exclude_focused { "▎" } else { "" }
                                                    )
                                                }),
                                        )
                                }),
                        )
                })),
        )
        // Grouped Results List
        .child(
            div()
                .id("search_results_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .py_1()
                .children(grouped_matches.into_iter().map(|(rel_path, matches)| {
                    let is_collapsed = search.collapsed_files.contains(&rel_path);
                    let match_cnt = matches.len();
                    let rel_click = rel_path.clone();
                    let first_file_path = matches
                        .first()
                        .map(|m| m.file_path.clone())
                        .unwrap_or_default();

                    let file_icon_name = if rel_path.ends_with(".rs") {
                        IconName::FileCode
                    } else if rel_path.ends_with(".md") {
                        IconName::FileText
                    } else {
                        IconName::File
                    };

                    div()
                        .flex()
                        .flex_col()
                        .w_full()
                        // File Header row
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(px(24.0))
                                .px_2()
                                .cursor_pointer()
                                .bg(theme.bg_sidebar)
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event, _window, cx| {
                                        if let Some(pos) = this
                                            .search
                                            .collapsed_files
                                            .iter()
                                            .position(|f| f == &rel_click)
                                        {
                                            this.search.collapsed_files.remove(pos);
                                        } else {
                                            this.search.collapsed_files.push(rel_click.clone());
                                        }
                                        this.state.search = this.search.clone();
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(if is_collapsed {
                                            icon_12(IconName::ChevronRight, theme.text_subtle)
                                        } else {
                                            icon_12(IconName::ChevronDown, theme.text_subtle)
                                        })
                                        .child(ui_icon(file_icon_name, 12.0, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::MEDIUM)
                                                .text_color(theme.text_primary)
                                                .child(rel_path),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        // Replace in this file button (when replace is open)
                                        .children(search.show_replace.then(|| {
                                            let fp = first_file_path.clone();
                                            div()
                                                .px_1()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .bg(theme.bg_raised)
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        let root =
                                                            this.state.workspace_root.clone();
                                                        this.search.replace_file(&fp, &root);
                                                        this.state.search = this.search.clone();
                                                        this.state
                                                            .toast_manager
                                                            .push_info("Replaced matches in file");
                                                        cx.notify();
                                                    }),
                                                )
                                                .child("Replace")
                                        }))
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(8.0))
                                                .bg(theme.bg_raised)
                                                .text_xs()
                                                .text_color(theme.text_muted)
                                                .child(match_cnt.to_string()),
                                        ),
                                ),
                        )
                        // Matching lines
                        .children((!is_collapsed).then(|| {
                            div()
                                .flex()
                                .flex_col()
                                .w_full()
                                .children(matches.into_iter().map(|m| {
                                    let path = m.file_path.clone();
                                    let line_num = m.line_number;
                                    let snippet = m.line_content.clone();

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .h(px(20.0))
                                        .pl(px(24.0))
                                        .pr_2()
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, window, cx| {
                                                this.goto_file_location(
                                                    &path,
                                                    line_num.saturating_sub(1),
                                                    0,
                                                    window,
                                                    cx,
                                                );
                                            }),
                                        )
                                        .child(
                                            div()
                                                .w(px(24.0))
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child(line_num.to_string()),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_family("Cascadia Code, Consolas, monospace")
                                                .text_color(theme.text_muted)
                                                .child(snippet),
                                        )
                                }))
                        }))
                })),
        )
}

pub struct SearchPanel;

impl crate::panels::traits::WorkbenchPanel for SearchPanel {
    fn id(&self) -> &'static str {
        "search"
    }

    fn title(&self) -> &'static str {
        "SEARCH"
    }

    fn icon(&self) -> IconName {
        IconName::Search
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_search_panel(state, &state.search, cx).into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;
    use std::fs;

    #[test]
    fn test_matches_pattern_globs() {
        assert!(matches_pattern("src/main.rs", "*.rs"));
        assert!(matches_pattern("src/lib.rs", "*.rs, *.toml"));
        assert!(!matches_pattern("src/main.js", "*.rs"));
        assert!(matches_pattern("src/main.rs", "src/*"));
        assert!(!matches_pattern("tests/test.rs", "src/*"));
        assert!(matches_pattern("any/path/file.txt", ""));
    }

    #[test]
    fn test_search_and_replace_lifecycle() {
        let temp_dir = std::env::temp_dir().join("vscodium_search_test");
        let _ = fs::create_dir_all(&temp_dir);
        let test_file = temp_dir.join("sample.rs");
        fs::write(
            &test_file,
            "fn alpha_test() {\n    let foo = 42;\n    let foo_bar = 100;\n}\n",
        )
        .unwrap();

        let mut search = SearchState::default();
        search.query = "foo".to_string();
        search.execute_search(&temp_dir);
        // Literal substring matches "foo" and "foo_bar"
        assert_eq!(search.matches.len(), 2);

        // Whole word search
        search.whole_word = true;
        search.execute_search(&temp_dir);
        assert_eq!(search.matches.len(), 1);
        assert_eq!(search.matches[0].line_number, 2);

        // Regex search
        search.whole_word = false;
        search.is_regex = true;
        search.query = r"let (foo) = (\d+)".to_string();
        search.execute_search(&temp_dir);
        assert_eq!(search.matches.len(), 1);

        // Replace all with regex capture group substitution
        search.replace = "let bar = $2".to_string();
        let replaced = search.replace_all(&temp_dir);
        assert_eq!(replaced, 1);

        let modified = fs::read_to_string(&test_file).unwrap();
        assert!(modified.contains("let bar = 42;"));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_search_include_exclude_filters() {
        let temp_dir = std::env::temp_dir().join("vscodium_filter_test");
        let _ = fs::create_dir_all(&temp_dir);
        let rs_file = temp_dir.join("hello.rs");
        let txt_file = temp_dir.join("notes.txt");
        fs::write(&rs_file, "TARGET_WORD in rust\n").unwrap();
        fs::write(&txt_file, "TARGET_WORD in txt\n").unwrap();

        let mut search = SearchState::default();
        search.query = "TARGET_WORD".to_string();
        search.include_pattern = "*.rs".to_string();
        search.execute_search(&temp_dir);
        assert_eq!(search.matches.len(), 1);
        assert!(search.matches[0].file_path.ends_with("hello.rs"));

        // Exclude filter
        search.include_pattern.clear();
        search.exclude_pattern = "*.txt".to_string();
        search.execute_search(&temp_dir);
        assert_eq!(search.matches.len(), 1);
        assert!(search.matches[0].file_path.ends_with("hello.rs"));

        // Clean up
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn test_search_focused_fields() {
        let mut search = SearchState::default();
        assert_eq!(search.focused_field, SearchFocusedField::Query);

        search.focused_field = SearchFocusedField::Replace;
        assert_eq!(search.focused_field, SearchFocusedField::Replace);

        search.focused_field = SearchFocusedField::Include;
        assert_eq!(search.focused_field, SearchFocusedField::Include);

        search.focused_field = SearchFocusedField::Exclude;
        assert_eq!(search.focused_field, SearchFocusedField::Exclude);
    }
}
