use crate::app_state::HadesNativeState;
use crate::editor::engine::diff::{
    AlignedDiffRow, DiffHunk, DiffLineKind, DiffModel, HunkKind, UnifiedDiffRow,
};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiffKind {
    Added,
    Removed,
    Context,
    HunkHeader,
}

#[derive(Clone, Debug)]
pub struct DiffLine {
    pub kind: DiffKind,
    pub old_line_no: Option<usize>,
    pub new_line_no: Option<usize>,
    pub content: String,
}

#[derive(Clone, Debug, Default)]
pub struct DiffViewerState {
    pub is_open: bool,
    pub file_path: String,
    pub model: Option<DiffModel>,
    pub lines: Vec<DiffLine>,
    pub additions: usize,
    pub deletions: usize,
}

impl DiffViewerState {
    pub fn open_diff(&mut self, root: &Path, rel_path: &str) {
        self.is_open = true;
        self.file_path = rel_path.to_string();
        self.lines.clear();

        // 1. Fetch git HEAD content (if tracked)
        let output = Command::new("git")
            .current_dir(root)
            .args(["show", &format!("HEAD:{}", rel_path)])
            .output();

        let head_content = match output {
            Ok(out) if out.status.success() => String::from_utf8_lossy(&out.stdout).to_string(),
            _ => String::new(),
        };

        // 2. Read current working tree file content
        let full_path = root.join(rel_path);
        let work_content = std::fs::read_to_string(&full_path).unwrap_or_default();

        // 3. Initialize Monaco Myers DiffModel
        let model = DiffModel::new(
            "HEAD (Original)".to_string(),
            &head_content,
            "Working Tree (Modified)".to_string(),
            &work_content,
            rel_path.to_string(),
        );

        self.additions = model.additions;
        self.deletions = model.deletions;

        // Also populate legacy lines for any backwards-compatible consumers
        for row in &model.unified_rows {
            let (kind, content) = match row.kind {
                DiffLineKind::Added => (DiffKind::Added, row.text.clone()),
                DiffLineKind::Deleted => (DiffKind::Removed, row.text.clone()),
                DiffLineKind::Unchanged => (DiffKind::Context, row.text.clone()),
                DiffLineKind::Empty => (DiffKind::Context, String::new()),
            };
            self.lines.push(DiffLine {
                kind,
                old_line_no: row.old_line_no,
                new_line_no: row.new_line_no,
                content,
            });
        }

        self.model = Some(model);
    }

    pub fn open_buffer_diff(
        &mut self,
        orig_title: &str,
        orig_content: &str,
        mod_title: &str,
        mod_content: &str,
        file_path: &str,
    ) {
        self.is_open = true;
        self.file_path = file_path.to_string();
        self.lines.clear();

        let model = DiffModel::new(
            orig_title.to_string(),
            orig_content,
            mod_title.to_string(),
            mod_content,
            file_path.to_string(),
        );

        self.additions = model.additions;
        self.deletions = model.deletions;
        self.model = Some(model);
    }

    pub fn next_hunk(&mut self) {
        if let Some(m) = &mut self.model {
            m.next_hunk();
        }
    }

    pub fn prev_hunk(&mut self) {
        if let Some(m) = &mut self.model {
            m.prev_hunk();
        }
    }

    pub fn revert_hunk(&mut self, hunk_idx: usize, root: &Path) {
        if let Some(m) = &mut self.model {
            m.revert_hunk(hunk_idx);
            self.additions = m.additions;
            self.deletions = m.deletions;

            // Write reverted content back to disk
            let full_path = root.join(&self.file_path);
            let _ = std::fs::write(&full_path, m.modified_content());
        }
    }

    pub fn revert_all(&mut self, root: &Path) {
        if let Some(m) = &mut self.model {
            let orig_content = m.original_lines.join("\n");
            let full_path = root.join(&self.file_path);
            let _ = std::fs::write(&full_path, &orig_content);
            self.open_diff(root, &self.file_path.clone());
        }
    }

    pub fn toggle_side_by_side(&mut self) {
        if let Some(m) = &mut self.model {
            m.toggle_mode();
        }
    }

    pub fn is_side_by_side(&self) -> bool {
        self.model
            .as_ref()
            .map(|m| m.is_side_by_side)
            .unwrap_or(true)
    }

    pub fn hunk_info(&self) -> (usize, usize) {
        if let Some(m) = &self.model {
            let total = m.hunks.len();
            let current = if total > 0 { m.active_hunk_idx + 1 } else { 0 };
            (current, total)
        } else {
            (0, 0)
        }
    }

    pub fn close(&mut self) {
        self.is_open = false;
        self.file_path.clear();
        self.lines.clear();
        self.model = None;
    }
}

pub fn render_diff_viewer(
    state: &HadesNativeState,
    diff: &DiffViewerState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let is_side_by_side = diff.is_side_by_side();
    let (cur_hunk, total_hunks) = diff.hunk_info();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_editor)
        // ── Main Diff Toolbar ────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(36.0))
                .px_3()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                // Left: File Title & Stats Badges
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(crate::ui::icons::icon_14(
                            crate::ui::icons::IconName::GitCompare,
                            theme.accent,
                        ))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(format!("DIFF: {}", diff.file_path)),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgb(0x132b1b))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.status_green)
                                .child(format!("+{}", diff.additions)),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgb(0x351619))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.status_red)
                                .child(format!("-{}", diff.deletions)),
                        ),
                )
                // Center: Hunk Navigation Controls
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        // Prev Hunk Button
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
                                .text_color(theme.text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.diff_viewer.prev_hunk();
                                        cx.notify();
                                    }),
                                )
                                .child(crate::ui::icons::icon_12(
                                    crate::ui::icons::IconName::ChevronUp,
                                    theme.text_muted,
                                ))
                                .child("Prev Hunk"),
                        )
                        // Hunk Counter Pill
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(10.0))
                                .bg(if total_hunks > 0 {
                                    theme.accent
                                } else {
                                    theme.bg_raised
                                })
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(if total_hunks > 0 {
                                    rgb(0xffffff)
                                } else {
                                    theme.text_subtle
                                })
                                .child(if total_hunks > 0 {
                                    format!("Hunk {} of {}", cur_hunk, total_hunks)
                                } else {
                                    "No Changes".to_string()
                                }),
                        )
                        // Next Hunk Button
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
                                .text_color(theme.text_muted)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.diff_viewer.next_hunk();
                                        cx.notify();
                                    }),
                                )
                                .child("Next Hunk")
                                .child(crate::ui::icons::icon_12(
                                    crate::ui::icons::IconName::ChevronDown,
                                    theme.text_muted,
                                )),
                        ),
                )
                // Right: Mode Toggle (Side-by-Side vs Inline) & Actions
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        // Side-by-Side / Inline Mode Toggle Switch
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .p_0p5()
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(2.0))
                                        .text_xs()
                                        .cursor_pointer()
                                        .bg(if is_side_by_side {
                                            theme.accent
                                        } else {
                                            rgba(0x00000000)
                                        })
                                        .text_color(if is_side_by_side {
                                            rgb(0xffffff)
                                        } else {
                                            theme.text_subtle
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                if !this.diff_viewer.is_side_by_side() {
                                                    this.diff_viewer.toggle_side_by_side();
                                                    cx.notify();
                                                }
                                            }),
                                        )
                                        .child("Side-by-Side"),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded(px(2.0))
                                        .text_xs()
                                        .cursor_pointer()
                                        .bg(if !is_side_by_side {
                                            theme.accent
                                        } else {
                                            rgba(0x00000000)
                                        })
                                        .text_color(if !is_side_by_side {
                                            rgb(0xffffff)
                                        } else {
                                            theme.text_subtle
                                        })
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                if this.diff_viewer.is_side_by_side() {
                                                    this.diff_viewer.toggle_side_by_side();
                                                    cx.notify();
                                                }
                                            }),
                                        )
                                        .child("Inline"),
                                ),
                        )
                        // Revert All Changes Button
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
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.diff_viewer.revert_all(&root);
                                        this.scm.refresh(&root);
                                        cx.notify();
                                    }),
                                )
                                .child(crate::ui::icons::icon_12(
                                    crate::ui::icons::IconName::RefreshCw,
                                    theme.status_red,
                                ))
                                .child("Revert All"),
                        )
                        // Stage File Button
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
                                .text_color(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        let path = this.diff_viewer.file_path.clone();
                                        let _ = Command::new("git")
                                            .current_dir(&root)
                                            .args(["add", &path])
                                            .status();
                                        this.scm.refresh(&root);
                                        this.diff_viewer.open_diff(&root, &path);
                                        cx.notify();
                                    }),
                                )
                                .child(crate::ui::icons::icon_12(
                                    crate::ui::icons::IconName::Check,
                                    theme.accent,
                                ))
                                .child("Stage"),
                        )
                        // Close Button
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1()
                                .p_1()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .hover(|s| s.bg(theme.bg_hover).text_color(theme.text_primary))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.diff_viewer.close();
                                        cx.notify();
                                    }),
                                )
                                .child(crate::ui::icons::icon_12(
                                    crate::ui::icons::IconName::X,
                                    theme.text_subtle,
                                )),
                        ),
                ),
        )
        // ── Sub-header Columns (For Side-by-Side) ───────────────────────────
        .children((is_side_by_side).then(|| {
            div()
                .flex()
                .flex_row()
                .h(px(24.0))
                .bg(theme.bg_sidebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_muted)
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_row()
                        .items_center()
                        .px_3()
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .child("Original (HEAD)"),
                )
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_row()
                        .items_center()
                        .px_3()
                        .child("Modified (Working Tree)"),
                )
        }))
        // ── Diff Viewport Canvas ─────────────────────────────────────────────
        .child(if let Some(model) = &diff.model {
            if is_side_by_side {
                render_side_by_side_diff(state, model, cx).into_any_element()
            } else {
                render_unified_diff(state, model, cx).into_any_element()
            }
        } else {
            render_legacy_diff_lines(state, diff, cx).into_any_element()
        })
}

fn render_side_by_side_diff(
    state: &HadesNativeState,
    model: &DiffModel,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .id("diff_side_by_side_scroll")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .font_family("Cascadia Code, Consolas, monospace")
        .text_xs()
        .children(model.aligned_rows.iter().enumerate().map(|(row_idx, row)| {
            let is_hunk_start = row.hunk_idx.is_some()
                && (row_idx == 0 || model.aligned_rows[row_idx - 1].hunk_idx != row.hunk_idx);
            let hunk_idx = row.hunk_idx;

            div()
                .flex()
                .flex_row()
                .w_full()
                .min_h(px(19.0))
                // Left Pane: Original Line
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_row()
                        .items_center()
                        .border_r_1()
                        .border_color(theme.border_subtle)
                        .bg(match row.original_kind {
                            DiffLineKind::Deleted => rgb(0x351619),
                            DiffLineKind::Empty => rgb(0x13151b),
                            _ => rgba(0x00000000),
                        })
                        .children(
                            (row.original_kind == DiffLineKind::Deleted)
                                .then(|| div().w(px(3.0)).h_full().bg(theme.status_red)),
                        )
                        // Original Line Gutter
                        .child(
                            div()
                                .w(px(44.0))
                                .px_1()
                                .text_align(TextAlign::Right)
                                .text_color(match row.original_kind {
                                    DiffLineKind::Deleted => rgb(0xf87171),
                                    _ => theme.text_subtle,
                                })
                                .child(
                                    row.original_line_no
                                        .map(|n| n.to_string())
                                        .unwrap_or_default(),
                                ),
                        )
                        // Deleted Sign Indicator
                        .child(
                            div()
                                .w(px(16.0))
                                .text_align(TextAlign::Center)
                                .text_color(rgb(0xf87171))
                                .child(if row.original_kind == DiffLineKind::Deleted {
                                    "-"
                                } else {
                                    " "
                                }),
                        )
                        // Original Code Text
                        .child(
                            div()
                                .flex_1()
                                .px_1()
                                .text_color(match row.original_kind {
                                    DiffLineKind::Deleted => rgb(0xfee2e2),
                                    DiffLineKind::Empty => theme.text_subtle,
                                    _ => theme.text_primary,
                                })
                                .child(row.original_text.clone().unwrap_or_default()),
                        ),
                )
                // Center Revert Hunk Action (visible at hunk start)
                .children((is_hunk_start && hunk_idx.is_some()).then(|| {
                    let h_idx = hunk_idx.unwrap();
                    div()
                        .absolute()
                        .left(px(3.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.status_red)
                        .text_color(theme.status_red)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                let root = this.state.workspace_root.clone();
                                this.diff_viewer.revert_hunk(h_idx, &root);
                                this.scm.refresh(&root);
                                cx.notify();
                            }),
                        )
                        .child("↺")
                }))
                // Right Pane: Modified Line
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_row()
                        .items_center()
                        .bg(match row.modified_kind {
                            DiffLineKind::Added => rgb(0x132b1b),
                            DiffLineKind::Empty => rgb(0x13151b),
                            _ => rgba(0x00000000),
                        })
                        .children(
                            (row.modified_kind == DiffLineKind::Added)
                                .then(|| div().w(px(3.0)).h_full().bg(theme.status_green)),
                        )
                        // Modified Line Gutter
                        .child(
                            div()
                                .w(px(44.0))
                                .px_1()
                                .text_align(TextAlign::Right)
                                .text_color(match row.modified_kind {
                                    DiffLineKind::Added => rgb(0x4ade80),
                                    _ => theme.text_subtle,
                                })
                                .child(
                                    row.modified_line_no
                                        .map(|n| n.to_string())
                                        .unwrap_or_default(),
                                ),
                        )
                        // Added Sign Indicator
                        .child(
                            div()
                                .w(px(16.0))
                                .text_align(TextAlign::Center)
                                .text_color(rgb(0x4ade80))
                                .child(if row.modified_kind == DiffLineKind::Added {
                                    "+"
                                } else {
                                    " "
                                }),
                        )
                        // Modified Code Text
                        .child(
                            div()
                                .flex_1()
                                .px_1()
                                .text_color(match row.modified_kind {
                                    DiffLineKind::Added => rgb(0xdcfce7),
                                    DiffLineKind::Empty => theme.text_subtle,
                                    _ => theme.text_primary,
                                })
                                .child(row.modified_text.clone().unwrap_or_default()),
                        ),
                )
        }))
}

fn render_unified_diff(
    state: &HadesNativeState,
    model: &DiffModel,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .id("diff_unified_scroll")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .font_family("Cascadia Code, Consolas, monospace")
        .text_xs()
        .children(model.unified_rows.iter().enumerate().map(|(idx, row)| {
            let is_hunk_start = row.hunk_idx.is_some()
                && (idx == 0 || model.unified_rows[idx - 1].hunk_idx != row.hunk_idx);
            let hunk_idx = row.hunk_idx;

            div()
                .flex()
                .flex_row()
                .w_full()
                .min_h(px(19.0))
                .bg(match row.kind {
                    DiffLineKind::Added => rgb(0x132b1b),
                    DiffLineKind::Deleted => rgb(0x351619),
                    _ => rgba(0x00000000),
                })
                .children(
                    (row.kind == DiffLineKind::Added)
                        .then(|| div().w(px(3.0)).h_full().bg(theme.status_green)),
                )
                .children(
                    (row.kind == DiffLineKind::Deleted)
                        .then(|| div().w(px(3.0)).h_full().bg(theme.status_red)),
                )
                // Old line number
                .child(
                    div()
                        .w(px(40.0))
                        .px_1()
                        .text_align(TextAlign::Right)
                        .text_color(if row.kind == DiffLineKind::Deleted {
                            rgb(0xf87171)
                        } else {
                            theme.text_subtle
                        })
                        .child(row.old_line_no.map(|n| n.to_string()).unwrap_or_default()),
                )
                // New line number
                .child(
                    div()
                        .w(px(40.0))
                        .px_1()
                        .text_align(TextAlign::Right)
                        .text_color(if row.kind == DiffLineKind::Added {
                            rgb(0x4ade80)
                        } else {
                            theme.text_subtle
                        })
                        .child(row.new_line_no.map(|n| n.to_string()).unwrap_or_default()),
                )
                // Diff symbol
                .child(
                    div()
                        .w(px(20.0))
                        .text_align(TextAlign::Center)
                        .text_color(match row.kind {
                            DiffLineKind::Added => rgb(0x4ade80),
                            DiffLineKind::Deleted => rgb(0xf87171),
                            _ => theme.text_subtle,
                        })
                        .child(match row.kind {
                            DiffLineKind::Added => "+",
                            DiffLineKind::Deleted => "-",
                            _ => " ",
                        }),
                )
                // Text
                .child(
                    div()
                        .flex_1()
                        .px_1()
                        .text_color(match row.kind {
                            DiffLineKind::Added => rgb(0xdcfce7),
                            DiffLineKind::Deleted => rgb(0xfee2e2),
                            _ => theme.text_primary,
                        })
                        .child(row.text.clone()),
                )
                // Revert Hunk button on header of hunk
                .children((is_hunk_start && hunk_idx.is_some()).then(|| {
                    let h_idx = hunk_idx.unwrap();
                    div()
                        .flex()
                        .items_center()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_raised)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .text_xs()
                        .text_color(theme.status_red)
                        .cursor_pointer()
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event, _window, cx| {
                                let root = this.state.workspace_root.clone();
                                this.diff_viewer.revert_hunk(h_idx, &root);
                                this.scm.refresh(&root);
                                cx.notify();
                            }),
                        )
                        .child("Revert Hunk")
                }))
        }))
}

fn render_legacy_diff_lines(
    state: &HadesNativeState,
    diff: &DiffViewerState,
    _cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .id("diff_viewer_lines_scroll")
        .flex()
        .flex_col()
        .flex_1()
        .overflow_y_scroll()
        .font_family("Cascadia Code, Consolas, monospace")
        .text_xs()
        .children(diff.lines.iter().map(|line| {
            match line.kind {
                DiffKind::HunkHeader => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_3()
                    .py_0p5()
                    .bg(rgb(0x182234))
                    .text_color(rgb(0x7dd3fc))
                    .child(line.content.clone()),
                DiffKind::Added => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_2()
                    .py_0p5()
                    .bg(rgb(0x132b1b))
                    .border_l_2()
                    .border_color(theme.status_green)
                    .child(
                        div()
                            .w(px(40.0))
                            .text_color(rgb(0x4ade80))
                            .child(line.new_line_no.map(|n| n.to_string()).unwrap_or_default()),
                    )
                    .child(div().w(px(20.0)).text_color(rgb(0x4ade80)).child("+"))
                    .child(div().text_color(rgb(0xdcfce7)).child(line.content.clone())),
                DiffKind::Removed => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_2()
                    .py_0p5()
                    .bg(rgb(0x351619))
                    .border_l_2()
                    .border_color(theme.status_red)
                    .child(
                        div()
                            .w(px(40.0))
                            .text_color(rgb(0xf87171))
                            .child(line.old_line_no.map(|n| n.to_string()).unwrap_or_default()),
                    )
                    .child(div().w(px(20.0)).text_color(rgb(0xf87171)).child("-"))
                    .child(div().text_color(rgb(0xfee2e2)).child(line.content.clone())),
                DiffKind::Context => div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .px_2()
                    .py_0p5()
                    .child(
                        div()
                            .w(px(40.0))
                            .text_color(theme.text_subtle)
                            .child(line.new_line_no.map(|n| n.to_string()).unwrap_or_default()),
                    )
                    .child(div().w(px(20.0)).text_color(theme.text_subtle).child(" "))
                    .child(
                        div()
                            .text_color(theme.text_primary)
                            .child(line.content.clone()),
                    ),
            }
        }))
}
