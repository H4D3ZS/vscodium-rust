use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::ui::icons::{icon_12, icon_14, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::Path;
use std::process::Command;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScmViewTab {
    Changes,
    Graph,
}

#[derive(Clone, Debug)]
pub struct GitCommitNode {
    pub graph: String,
    pub hash: String,
    pub author: String,
    pub time: String,
    pub message: String,
    pub decoration: Option<String>,
}

#[derive(Clone, Debug)]
pub struct GitFileChange {
    pub path: String,
    pub status: String,
    pub is_staged: bool,
}

#[derive(Clone, Debug)]
pub struct ScmState {
    pub branch: String,
    pub branches: Vec<String>,
    pub commit_message: String,
    pub changes: Vec<GitFileChange>,
    pub commits: Vec<GitCommitNode>,
    pub git_graph: crate::panels::git_graph::GitGraphState,
    pub active_subtab: ScmViewTab,
    pub expanded_diff_file: Option<String>,
    pub inline_diff_lines: Vec<(String, String)>, // (line_text, "add" | "del" | "hunk" | "plain")
    pub is_refreshing: bool,
    pub is_generating_msg: bool,
    pub status_banner: Option<String>,
}

impl Default for ScmState {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
            branches: vec!["main".to_string()],
            commit_message: String::new(),
            changes: Vec::new(),
            commits: Vec::new(),
            git_graph: crate::panels::git_graph::GitGraphState::default(),
            active_subtab: ScmViewTab::Changes,
            expanded_diff_file: None,
            inline_diff_lines: Vec::new(),
            is_refreshing: false,
            is_generating_msg: false,
            status_banner: None,
        }
    }
}

impl ScmState {
    pub fn refresh(&mut self, root: &Path) {
        self.is_refreshing = true;

        // 1. Current branch
        let branch_out = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(root)
            .output();
        if let Ok(out) = branch_out {
            let b = String::from_utf8_lossy(&out.stdout).trim().to_string();
            self.branch = if b.is_empty() { "main".to_string() } else { b };
        }

        // 2. All branches
        let branches_out = Command::new("git")
            .args(["branch", "--format=%(refname:short)"])
            .current_dir(root)
            .output();
        if let Ok(out) = branches_out {
            let lines: Vec<String> = String::from_utf8_lossy(&out.stdout)
                .lines()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
            if !lines.is_empty() {
                self.branches = lines;
            }
        }

        // 3. Status porcelain
        let status_out = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(root)
            .output();

        let mut items = Vec::new();
        if let Ok(out) = status_out {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                if line.len() >= 3 {
                    let code = &line[..2];
                    let file_path = line[3..].trim().to_string();
                    let is_staged = !code.starts_with(' ') && !code.starts_with('?');
                    items.push(GitFileChange {
                        path: file_path,
                        status: code.trim().to_string(),
                        is_staged,
                    });
                }
            }
        }
        self.changes = items;

        // 4. Git Graph log
        self.refresh_log(root);
        self.git_graph.reload(root);

        self.is_refreshing = false;
    }

    pub fn refresh_log(&mut self, root: &Path) {
        let log_out = Command::new("git")
            .args([
                "log",
                "--graph",
                "--pretty=format:%h|%an|%ar|%s|%d",
                "-n",
                "35",
            ])
            .current_dir(root)
            .output();

        let mut nodes = Vec::new();
        if let Ok(out) = log_out {
            let stdout = String::from_utf8_lossy(&out.stdout);
            for line in stdout.lines() {
                if line.contains('|') {
                    let parts: Vec<&str> = line.split('|').collect();
                    if parts.len() >= 4 {
                        let graph_and_hash = parts[0].trim();
                        let (graph_part, hash_part) =
                            if let Some(last_space) = graph_and_hash.rfind(' ') {
                                (
                                    &graph_and_hash[..last_space],
                                    &graph_and_hash[last_space + 1..],
                                )
                            } else {
                                ("*", graph_and_hash)
                            };

                        let author = parts[1].trim().to_string();
                        let time = parts[2].trim().to_string();
                        let message = parts[3].trim().to_string();
                        let decoration = if parts.len() >= 5 && !parts[4].trim().is_empty() {
                            Some(
                                parts[4]
                                    .trim()
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .to_string(),
                            )
                        } else {
                            None
                        };

                        nodes.push(GitCommitNode {
                            graph: graph_part.to_string(),
                            hash: hash_part.to_string(),
                            author,
                            time,
                            message,
                            decoration,
                        });
                    }
                }
            }
        }
        self.commits = nodes;
    }

    pub fn stage_all(&mut self, root: &Path) {
        let _ = Command::new("git")
            .args(["add", "-A"])
            .current_dir(root)
            .output();
        self.refresh(root);
    }

    pub fn unstage_all(&mut self, root: &Path) {
        let _ = Command::new("git")
            .args(["reset"])
            .current_dir(root)
            .output();
        self.refresh(root);
    }

    pub fn stage_file(&mut self, root: &Path, file: &str) {
        let _ = Command::new("git")
            .args(["add", file])
            .current_dir(root)
            .output();
        self.refresh(root);
    }

    pub fn unstage_file(&mut self, root: &Path, file: &str) {
        let _ = Command::new("git")
            .args(["reset", "HEAD", file])
            .current_dir(root)
            .output();
        self.refresh(root);
    }

    pub fn discard_file(&mut self, root: &Path, file: &str) {
        let _ = Command::new("git")
            .args(["checkout", "--", file])
            .current_dir(root)
            .output();
        self.refresh(root);
    }

    pub fn stash(&mut self, root: &Path) {
        let out = Command::new("git")
            .args(["stash"])
            .current_dir(root)
            .output();
        if let Ok(o) = out {
            self.status_banner = Some(String::from_utf8_lossy(&o.stdout).trim().to_string());
        }
        self.refresh(root);
    }

    pub fn stash_pop(&mut self, root: &Path) {
        let out = Command::new("git")
            .args(["stash", "pop"])
            .current_dir(root)
            .output();
        if let Ok(o) = out {
            self.status_banner = Some(String::from_utf8_lossy(&o.stdout).trim().to_string());
        }
        self.refresh(root);
    }

    pub fn push(&mut self, root: &Path) {
        let out = Command::new("git")
            .args(["push"])
            .current_dir(root)
            .output();
        if let Ok(o) = out {
            let msg = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            self.status_banner = Some(if !msg.trim().is_empty() {
                msg.trim().to_string()
            } else {
                err.trim().to_string()
            });
        }
        self.refresh(root);
    }

    pub fn pull(&mut self, root: &Path) {
        let out = Command::new("git")
            .args(["pull"])
            .current_dir(root)
            .output();
        if let Ok(o) = out {
            let msg = String::from_utf8_lossy(&o.stdout);
            self.status_banner = Some(msg.trim().to_string());
        }
        self.refresh(root);
    }

    pub fn toggle_inline_diff(&mut self, root: &Path, file: &str) {
        if self.expanded_diff_file.as_deref() == Some(file) {
            self.expanded_diff_file = None;
            self.inline_diff_lines.clear();
            return;
        }

        self.expanded_diff_file = Some(file.to_string());
        self.inline_diff_lines.clear();

        let diff_out = Command::new("git")
            .args(["diff", file])
            .current_dir(root)
            .output();

        let mut lines = Vec::new();
        if let Ok(out) = diff_out {
            let text = String::from_utf8_lossy(&out.stdout);
            let text = if text.is_empty() {
                // Try cached diff
                let cached = Command::new("git")
                    .args(["diff", "--cached", file])
                    .current_dir(root)
                    .output();
                cached
                    .map(|c| String::from_utf8_lossy(&c.stdout).to_string())
                    .unwrap_or_default()
            } else {
                text.to_string()
            };

            for line in text.lines().take(60) {
                if line.starts_with('+') && !line.starts_with("+++") {
                    lines.push((line.to_string(), "add".to_string()));
                } else if line.starts_with('-') && !line.starts_with("---") {
                    lines.push((line.to_string(), "del".to_string()));
                } else if line.starts_with("@@") {
                    lines.push((line.to_string(), "hunk".to_string()));
                } else {
                    lines.push((line.to_string(), "plain".to_string()));
                }
            }
        }

        if lines.is_empty() {
            lines.push((
                "(no diff available or untracked binary)".to_string(),
                "plain".to_string(),
            ));
        }
        self.inline_diff_lines = lines;
    }

    pub fn generate_ai_commit_message(&mut self, _root: &Path) {
        if self.changes.is_empty() {
            self.commit_message = "chore: general workspace updates".to_string();
            return;
        }

        // Generate semantic conventional commit based on changed files
        let first_file = &self.changes[0].path;
        let ext = Path::new(first_file)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");

        let scope = if first_file.contains("scm") || first_file.contains("git") {
            "scm"
        } else if first_file.contains("terminal") {
            "terminal"
        } else if first_file.contains("agent") || first_file.contains("chat") {
            "agent"
        } else if first_file.contains("mobile") || first_file.contains("ios") {
            "mobile"
        } else if first_file.contains("apex") || first_file.contains("security") {
            "security"
        } else if ext == "rs" {
            "native"
        } else if ext == "ts" || ext == "tsx" {
            "frontend"
        } else {
            "core"
        };

        let verb = if self
            .changes
            .iter()
            .any(|c| c.status.contains('A') || c.status.contains('?'))
        {
            "feat"
        } else if self.changes.iter().any(|c| c.status.contains('D')) {
            "refactor"
        } else {
            "fix"
        };

        let desc = if self.changes.len() == 1 {
            format!(
                "update {}",
                Path::new(first_file)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
            )
        } else {
            format!(
                "update {} and {} other files",
                Path::new(first_file)
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy(),
                self.changes.len() - 1
            )
        };

        self.commit_message = format!("{verb}({scope}): {desc}");
    }

    pub fn commit(&mut self, root: &Path) -> Result<String, String> {
        let msg = self.commit_message.trim();
        if msg.is_empty() {
            return Err("Commit message cannot be empty".to_string());
        }

        // Stage all tracked changes if nothing staged
        let _ = Command::new("git")
            .args(["add", "-u"])
            .current_dir(root)
            .output();

        let out = Command::new("git")
            .args(["commit", "-m", msg])
            .current_dir(root)
            .output()
            .map_err(|e| e.to_string())?;

        self.commit_message.clear();
        self.refresh(root);

        Ok(String::from_utf8_lossy(&out.stdout).to_string())
    }
}

pub fn render_scm_panel(
    state: &HadesNativeState,
    scm: &ScmState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let change_count = scm.changes.len();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // Top Header: Title, Subtab switcher (Changes / Git Graph), Refresh
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(35.0))
                .px_2p5()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        // Subtab pills
                        .child({
                            let is_changes = scm.active_subtab == ScmViewTab::Changes;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_changes {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_changes {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_changes {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.scm.active_subtab = ScmViewTab::Changes;
                                        cx.notify();
                                    }),
                                )
                                .child(format!("CHANGES ({change_count})"))
                        })
                        .child({
                            let is_graph = scm.active_subtab == ScmViewTab::Graph;
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(4.0))
                                .cursor_pointer()
                                .bg(if is_graph {
                                    theme.bg_raised
                                } else {
                                    theme.bg_sidebar
                                })
                                .text_color(if is_graph {
                                    theme.text_primary
                                } else {
                                    theme.text_muted
                                })
                                .text_xs()
                                .font_weight(if is_graph {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.scm.active_subtab = ScmViewTab::Graph;
                                        let root = this.state.workspace_root.clone();
                                        this.scm.git_graph.reload(&root);
                                        cx.notify();
                                    }),
                                )
                                .child("GIT GRAPH")
                        }),
                )
                .child(
                    div().flex().flex_row().items_center().gap_1().child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event, _window, cx| {
                                    let root = this.state.workspace_root.clone();
                                    this.scm.refresh(&root);
                                    cx.notify();
                                }),
                            )
                            .child(icon_14(IconName::RefreshCw, theme.text_subtle)),
                    ),
                ),
        )
        // Toolbar: Branch Selector, Pull, Push, Stash
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(30.0))
                .px_2p5()
                .bg(theme.bg_titlebar)
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_1()
                        .font_weight(FontWeight::MEDIUM)
                        .text_xs()
                        .text_color(theme.accent)
                        .child(icon_12(IconName::GitBranch, theme.accent))
                        .child(scm.branch.clone()),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.scm.pull(&root);
                                        cx.notify();
                                    }),
                                )
                                .child("Pull"),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.scm.push(&root);
                                        cx.notify();
                                    }),
                                )
                                .child("Push"),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.scm.stash(&root);
                                        cx.notify();
                                    }),
                                )
                                .child("Stash"),
                        )
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .text_xs()
                                .text_color(theme.text_muted)
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.scm.stash_pop(&root);
                                        cx.notify();
                                    }),
                                )
                                .child("Pop"),
                        ),
                ),
        )
        // Optional Status Banner
        .children(scm.status_banner.as_ref().map(|banner| {
            div()
                .px_3()
                .py_1()
                .bg(theme.bg_card)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.accent)
                .child(banner.clone())
        }))
        // Main Body: Changes Tab or Git Graph Tab
        .child(match scm.active_subtab {
            ScmViewTab::Changes => render_changes_tab(state, scm, cx).into_any_element(),
            ScmViewTab::Graph => render_graph_tab(state, scm, cx).into_any_element(),
        })
}

fn render_changes_tab(
    state: &HadesNativeState,
    scm: &ScmState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;

    div()
        .flex()
        .flex_col()
        .flex_1()
        .overflow_hidden()
        // Commit Box + AI Generate Button
        .child(
            div()
                .flex()
                .flex_col()
                .p_2p5()
                .gap_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .h(px(28.0))
                        .px_2()
                        .rounded(px(4.0))
                        .bg(theme.bg_editor)
                        .border_1()
                        .border_color(theme.border_composer)
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .text_color(if scm.commit_message.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if scm.commit_message.is_empty() {
                                    "Message (Ctrl+Enter to commit)".to_string()
                                } else {
                                    scm.commit_message.clone()
                                }),
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
                                        this.scm.generate_ai_commit_message(&root);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Sparkles, theme.accent)),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_1()
                                .flex_row()
                                .items_center()
                                .justify_center()
                                .gap_1p5()
                                .h(px(26.0))
                                .rounded(px(4.0))
                                .bg(theme.accent)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.accent_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        match this.scm.commit(&root) {
                                            Ok(hash) => {
                                                this.state
                                                    .toast_manager
                                                    .push_success(&format!("Committed: {}", hash));
                                            }
                                            Err(e) => {
                                                this.state
                                                    .toast_manager
                                                    .push_error(&format!("Commit failed: {}", e));
                                            }
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::Check, rgb(0xffffff)))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(rgb(0xffffff))
                                        .child("Commit"),
                                ),
                        )
                        .child(
                            div()
                                .px_2()
                                .h(px(26.0))
                                .flex()
                                .items_center()
                                .rounded(px(4.0))
                                .bg(theme.bg_raised)
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let root = this.state.workspace_root.clone();
                                        this.scm.stage_all(&root);
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("Stage All"),
                                ),
                        ),
                ),
        )
        // Changes File List
        .child(
            div()
                .id("scm_changes_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .py_1()
                .children(scm.changes.iter().map(|change| {
                    let path = change.path.clone();
                    let p_str = path.clone();
                    let p_diff = path.clone();
                    let p_stage = path.clone();
                    let p_discard = path.clone();
                    let code = change.status.clone();
                    let is_staged = change.is_staged;
                    let is_expanded = scm.expanded_diff_file.as_deref() == Some(&path);

                    let status_color = if code.contains('M') {
                        theme.status_yellow
                    } else if code.contains('?') || code.contains('A') {
                        theme.status_green
                    } else {
                        theme.status_red
                    };

                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .justify_between()
                                .h(px(24.0))
                                .px_3()
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                let root = this.state.workspace_root.clone();
                                                this.diff_viewer.open_diff(&root, &p_str);
                                                this.state.focused_panel = FocusedPanel::Editor;
                                                cx.notify();
                                            }),
                                        )
                                        .child(ui_icon(
                                            if is_expanded {
                                                IconName::ChevronDown
                                            } else {
                                                IconName::ChevronRight
                                            },
                                            11.0,
                                            theme.text_subtle,
                                        ))
                                        .child(ui_icon(IconName::FileCode, 13.0, theme.text_subtle))
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child(path),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        // Inline diff toggle
                                        .child(
                                            div()
                                                .p_0p5()
                                                .rounded(px(2.0))
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(
                                                        move |this, _event, _window, cx| {
                                                            let root =
                                                                this.state.workspace_root.clone();
                                                            this.scm
                                                                .toggle_inline_diff(&root, &p_diff);
                                                            cx.notify();
                                                        },
                                                    ),
                                                )
                                                .child(icon_12(
                                                    IconName::GitCompare,
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Stage/Unstage button
                                        .child(
                                            div()
                                                .p_0p5()
                                                .rounded(px(2.0))
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(
                                                        move |this, _event, _window, cx| {
                                                            let root =
                                                                this.state.workspace_root.clone();
                                                            if is_staged {
                                                                this.scm
                                                                    .unstage_file(&root, &p_stage);
                                                            } else {
                                                                this.scm
                                                                    .stage_file(&root, &p_stage);
                                                            }
                                                            cx.notify();
                                                        },
                                                    ),
                                                )
                                                .child(icon_12(
                                                    if is_staged {
                                                        IconName::Minus
                                                    } else {
                                                        IconName::Plus
                                                    },
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Discard button
                                        .child(
                                            div()
                                                .p_0p5()
                                                .rounded(px(2.0))
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(
                                                        move |this, _event, _window, cx| {
                                                            let root =
                                                                this.state.workspace_root.clone();
                                                            this.scm
                                                                .discard_file(&root, &p_discard);
                                                            cx.notify();
                                                        },
                                                    ),
                                                )
                                                .child(icon_12(
                                                    IconName::RotateCcw,
                                                    theme.text_subtle,
                                                )),
                                        )
                                        // Status badge
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(status_color)
                                                .child(code),
                                        ),
                                ),
                        )
                        // Expandable inline diff view
                        .children(is_expanded.then(|| {
                            div()
                                .flex()
                                .flex_col()
                                .bg(rgb(0x0e1014))
                                .border_y_1()
                                .border_color(theme.border_subtle)
                                .py_1()
                                .children(scm.inline_diff_lines.iter().map(|(line_text, kind)| {
                                    let (bg_color, text_color) = match kind.as_str() {
                                        "add" => (rgba(0x10b98118), rgb(0x4ade80)),
                                        "del" => (rgba(0xef444418), rgb(0xf87171)),
                                        "hunk" => (rgba(0x3b82f615), rgb(0x60a5fa)),
                                        _ => (rgba(0x00000000), theme.text_muted),
                                    };

                                    div()
                                        .px_3()
                                        .h(px(16.0))
                                        .bg(bg_color)
                                        .text_xs()
                                        .font_family(".SystemUIFont")
                                        .text_color(text_color)
                                        .child(line_text.clone())
                                }))
                        }))
                })),
        )
}

fn render_graph_tab(
    state: &HadesNativeState,
    scm: &ScmState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    crate::panels::git_graph::render_git_graph(state, &scm.git_graph, cx)
}

pub struct ScmPanel;

impl crate::panels::traits::WorkbenchPanel for ScmPanel {
    fn id(&self) -> &'static str {
        "scm"
    }

    fn title(&self) -> &'static str {
        "SOURCE CONTROL"
    }

    fn icon(&self) -> IconName {
        IconName::GitBranch
    }

    fn badge(&self, state: &HadesNativeState) -> Option<String> {
        let count = state.scm.changes.len();
        if count > 0 {
            Some(format!("{count}"))
        } else {
            None
        }
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_scm_panel(state, &state.scm, cx).into_any_element()
    }
}
