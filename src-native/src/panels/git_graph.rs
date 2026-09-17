use crate::state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::Path;
use std::process::Command;

pub fn get_lane_color(idx: usize) -> Rgba {
    match idx % 6 {
        0 => rgb(0x38bdf8), // Sky Blue
        1 => rgb(0xf472b6), // Cherry Blossom Pink
        2 => rgb(0x4ade80), // Spring Green
        3 => rgb(0xfbbf24), // Amber
        4 => rgb(0xa78bfa), // Lavender Purple
        _ => rgb(0xf87171), // Coral
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GitCommit {
    pub hash: String,
    pub short_hash: String,
    pub author: String,
    pub author_initials: String,
    pub date: String,
    pub relative_time: String,
    pub message: String,
    pub parents: Vec<String>,
    pub lane: usize,
    pub color_idx: usize,
}

#[derive(Clone, Debug)]
pub struct GitGraphState {
    pub commits: Vec<GitCommit>,
    pub selected_commit: Option<String>,
    pub selected_commit_info: Option<GitCommit>,
    pub commit_diff: Option<String>,
    pub diff_lines: Vec<(String, String)>, // (line_content, kind: "add" | "del" | "hunk" | "header" | "plain")
    pub filter_query: String,
    pub is_loading: bool,
    pub error_message: Option<String>,
}

impl Default for GitGraphState {
    fn default() -> Self {
        Self {
            commits: Vec::new(),
            selected_commit: None,
            selected_commit_info: None,
            commit_diff: None,
            diff_lines: Vec::new(),
            filter_query: String::new(),
            is_loading: false,
            error_message: None,
        }
    }
}

impl GitGraphState {
    pub fn reload(&mut self, root: &Path) {
        self.is_loading = true;
        self.error_message = None;

        let output = Command::new("git")
            .args(["log", "--format=%H|%an|%ar|%s|%P", "-n", "60"])
            .current_dir(root)
            .output();

        match output {
            Ok(out) if out.status.success() => {
                let stdout = String::from_utf8_lossy(&out.stdout);
                let mut parsed = Self::parse_git_log(&stdout);
                Self::assign_lanes(&mut parsed);
                self.commits = parsed;
            }
            Ok(out) => {
                let err = String::from_utf8_lossy(&out.stderr);
                self.error_message = Some(if err.trim().is_empty() {
                    "No commits found or not a git repository".to_string()
                } else {
                    err.trim().to_string()
                });
            }
            Err(e) => {
                self.error_message = Some(format!("Failed to execute git: {e}"));
            }
        }

        self.is_loading = false;
    }

    pub fn select_commit(&mut self, root: &Path, hash: &str) {
        if self.selected_commit.as_deref() == Some(hash) {
            // Toggle off if already selected
            self.close_diff();
            return;
        }

        self.selected_commit = Some(hash.to_string());
        self.selected_commit_info = self.commits.iter().find(|c| c.hash == hash).cloned();

        let out = Command::new("git")
            .args(["show", "--stat", "--patch", "--pretty=format:%b", hash])
            .current_dir(root)
            .output();

        if let Ok(res) = out {
            let text = String::from_utf8_lossy(&res.stdout).to_string();
            self.diff_lines = Self::parse_diff(&text);
            self.commit_diff = Some(text);
        } else {
            self.diff_lines.clear();
            self.commit_diff = None;
        }
    }

    pub fn close_diff(&mut self) {
        self.selected_commit = None;
        self.selected_commit_info = None;
        self.commit_diff = None;
        self.diff_lines.clear();
    }

    pub fn filtered_commits(&self) -> Vec<&GitCommit> {
        let q = self.filter_query.trim().to_lowercase();
        if q.is_empty() {
            self.commits.iter().collect()
        } else {
            self.commits
                .iter()
                .filter(|c| {
                    c.message.to_lowercase().contains(&q)
                        || c.author.to_lowercase().contains(&q)
                        || c.short_hash.to_lowercase().contains(&q)
                        || c.hash.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn parse_git_log(output: &str) -> Vec<GitCommit> {
        let mut result = Vec::new();

        for line in output.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 {
                let full_hash = parts[0].trim().to_string();
                let short_hash = if full_hash.len() >= 7 {
                    full_hash[..7].to_string()
                } else {
                    full_hash.clone()
                };
                let author = parts[1].trim().to_string();
                let relative_time = parts[2].trim().to_string();
                let message = parts[3].trim().to_string();
                let parents: Vec<String> = parts
                    .get(4)
                    .unwrap_or(&"")
                    .split_whitespace()
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                let initials = author_initials(&author);
                let color_idx = str_hash(&author) % 6;

                result.push(GitCommit {
                    hash: full_hash,
                    short_hash,
                    author,
                    author_initials: initials,
                    date: String::new(),
                    relative_time,
                    message,
                    parents,
                    lane: 0,
                    color_idx,
                });
            }
        }

        result
    }

    pub fn assign_lanes(commits: &mut [GitCommit]) {
        let mut active_lanes: Vec<Option<String>> = Vec::new();

        for commit in commits.iter_mut() {
            // Check if this commit's hash is expected by any active lane
            let assigned_lane = if let Some(idx) = active_lanes.iter().position(|lane| {
                lane.as_ref()
                    .map_or(false, |h| h == &commit.hash || commit.hash.starts_with(h))
            }) {
                active_lanes[idx] = None;
                idx
            } else if let Some(free_idx) = active_lanes.iter().position(|lane| lane.is_none()) {
                free_idx
            } else {
                let new_idx = active_lanes.len();
                active_lanes.push(None);
                new_idx
            };

            commit.lane = assigned_lane.min(4);
            commit.color_idx = commit.lane % 6;

            // Assign parents to lanes
            for (p_idx, parent_hash) in commit.parents.iter().enumerate() {
                if p_idx == 0 {
                    // Primary parent continues this lane
                    if assigned_lane < active_lanes.len() {
                        active_lanes[assigned_lane] = Some(parent_hash.clone());
                    }
                } else {
                    // Secondary parent (branch/merge) takes another lane
                    if let Some(free_slot) = active_lanes.iter().position(|l| l.is_none()) {
                        active_lanes[free_slot] = Some(parent_hash.clone());
                    } else if active_lanes.len() < 5 {
                        active_lanes.push(Some(parent_hash.clone()));
                    }
                }
            }
        }
    }

    pub fn parse_diff(diff: &str) -> Vec<(String, String)> {
        let mut lines = Vec::new();
        for line in diff.lines() {
            let kind = if line.starts_with("+++") || line.starts_with("---") {
                "header"
            } else if line.starts_with('+') {
                "add"
            } else if line.starts_with('-') {
                "del"
            } else if line.starts_with("@@") {
                "hunk"
            } else if line.starts_with("diff --git") || line.starts_with("index ") {
                "header"
            } else {
                "plain"
            };
            lines.push((line.to_string(), kind.to_string()));
        }
        lines
    }
}

fn author_initials(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.is_empty() {
        return "U".to_string();
    }
    if parts.len() == 1 {
        parts[0]
            .chars()
            .next()
            .unwrap_or('U')
            .to_uppercase()
            .to_string()
    } else {
        let c1 = parts[0].chars().next().unwrap_or(' ');
        let c2 = parts[1].chars().next().unwrap_or(' ');
        format!("{}{}", c1.to_uppercase(), c2.to_uppercase())
    }
}

fn str_hash(s: &str) -> usize {
    let mut hash: usize = 5381;
    for b in s.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(b as usize);
    }
    hash
}

pub fn render_git_graph(
    state: &HadesNativeState,
    graph: &GitGraphState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let filtered = graph.filtered_commits();
    let commit_count = filtered.len();

    div()
        .id("git_graph_container")
        .flex()
        .flex_col()
        .flex_1()
        .w_full()
        .overflow_hidden()
        .bg(theme.bg_sidebar)
        // ── Filter & Action Toolbar ──────────────────────────────────────────
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
                .child(
                    div()
                        .flex()
                        .flex_1()
                        .items_center()
                        .gap_1p5()
                        .px_2()
                        .py_0p5()
                        .rounded(px(3.0))
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .child(icon_12(IconName::Search, theme.text_muted))
                        .child(
                            div()
                                .text_xs()
                                .font_family("Consolas")
                                .text_color(if graph.filter_query.is_empty() {
                                    theme.text_muted
                                } else {
                                    theme.text_primary
                                })
                                .child(if graph.filter_query.is_empty() {
                                    "Filter commits by message, author, or hash...".to_string()
                                } else {
                                    graph.filter_query.clone()
                                }),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_2()
                        .pl_2()
                        .child(
                            div()
                                .px_1p5()
                                .py_0p5()
                                .rounded_full()
                                .bg(rgba(0x38bdf818))
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x38bdf8))
                                .child(format!("{commit_count} commits")),
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
                                        this.scm.git_graph.reload(&root);
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::RefreshCw, theme.text_subtle)),
                        ),
                ),
        )
        // ── Main Graph & Commits View ────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_row()
                .flex_1()
                .w_full()
                .overflow_hidden()
                // Left: Commit DAG List
                .child(
                    div()
                        .id("git_commits_dag_list")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .overflow_y_scroll()
                        .children(if filtered.is_empty() {
                            vec![div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .justify_center()
                                .p_8()
                                .gap_2()
                                .child(icon_14(IconName::GitBranch, theme.text_muted))
                                .child(
                                    div().text_xs().text_color(theme.text_muted).child(
                                        graph.error_message.clone().unwrap_or_else(|| {
                                            "No matching commits found".to_string()
                                        }),
                                    ),
                                )
                                .into_any_element()]
                        } else {
                            filtered
                                .iter()
                                .map(|commit| {
                                    let hash = commit.hash.clone();
                                    let hash_clone = hash.clone();
                                    let short = commit.short_hash.clone();
                                    let msg = commit.message.clone();
                                    let author = commit.author.clone();
                                    let initials = commit.author_initials.clone();
                                    let time = commit.relative_time.clone();
                                    let lane = commit.lane;
                                    let is_selected =
                                        graph.selected_commit.as_deref() == Some(&hash);
                                    let lane_color = get_lane_color(commit.color_idx);

                                    div()
                                        .flex()
                                        .flex_col()
                                        .px_2()
                                        .py_1p5()
                                        .border_b_1()
                                        .border_color(theme.border_subtle)
                                        .bg(if is_selected {
                                            rgba(0x38bdf812)
                                        } else {
                                            rgba(0x00000000)
                                        })
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .cursor_pointer()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                let root = this.state.workspace_root.clone();
                                                this.scm
                                                    .git_graph
                                                    .select_commit(&root, &hash_clone);
                                                cx.notify();
                                            }),
                                        )
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
                                                        // Lane Track Visualization
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .flex_row()
                                                                .items_center()
                                                                .gap_1()
                                                                .w(px(32.0))
                                                                .children((0..3).map(|i| {
                                                                    if i == lane {
                                                                        div()
                                                                            .w(px(9.0))
                                                                            .h(px(9.0))
                                                                            .rounded_full()
                                                                            .bg(lane_color)
                                                                            .border_1()
                                                                            .border_color(
                                                                                theme.text_primary,
                                                                            )
                                                                    } else {
                                                                        div()
                                                                            .w(px(2.0))
                                                                            .h(px(14.0))
                                                                            .bg(rgba(0xffffff15))
                                                                    }
                                                                })),
                                                        )
                                                        // Commit Short Hash
                                                        .child(
                                                            div()
                                                                .px_1p5()
                                                                .py_0p5()
                                                                .rounded(px(3.0))
                                                                .bg(rgba(0xffffff08))
                                                                .text_xs()
                                                                .font_family("Consolas")
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(lane_color)
                                                                .child(short),
                                                        )
                                                        // Author Initials Avatar
                                                        .child(
                                                            div()
                                                                .flex()
                                                                .items_center()
                                                                .justify_center()
                                                                .w(px(18.0))
                                                                .h(px(18.0))
                                                                .rounded_full()
                                                                .bg(lane_color.opacity(0.2))
                                                                .text_xs()
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(lane_color)
                                                                .child(initials),
                                                        )
                                                        // Commit Message
                                                        .child(
                                                            div()
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
                                                                .child(msg),
                                                        ),
                                                )
                                                // Relative Time Pill
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.text_subtle)
                                                        .child(time),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_2()
                                                .pl(px(40.0))
                                                .pt_0p5()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .text_color(theme.text_subtle)
                                                        .child(format!("by {author}")),
                                                ),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        }),
                )
                // Right: Commit Diff Inspector (When a commit is selected)
                .children(graph.selected_commit_info.as_ref().map(|info| {
                    let full_h = info.hash.clone();
                    let author = info.author.clone();
                    let msg = info.message.clone();
                    let time = info.relative_time.clone();
                    let parents = info.parents.join(", ");

                    div()
                        .flex()
                        .flex_col()
                        .w(px(320.0))
                        .border_l_1()
                        .border_color(theme.border_subtle)
                        .bg(theme.bg_raised)
                        // Commit Header
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .p_2p5()
                                .gap_1p5()
                                .border_b_1()
                                .border_color(theme.border_subtle)
                                .bg(theme.bg_titlebar)
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.accent)
                                                .child("COMMIT DETAILS"),
                                        )
                                        .child(
                                            div()
                                                .p_1()
                                                .cursor_pointer()
                                                .rounded(px(3.0))
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event, _window, cx| {
                                                        this.scm.git_graph.close_diff();
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(icon_12(IconName::X, theme.text_subtle)),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_family("Consolas")
                                        .text_color(theme.text_muted)
                                        .child(full_h),
                                )
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::MEDIUM)
                                        .text_color(theme.text_primary)
                                        .child(msg),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_between()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(format!("Author: {author}"))
                                        .child(time),
                                )
                                .children((!parents.is_empty()).then(|| {
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .child(format!("Parents: {parents}"))
                                })),
                        )
                        // Commit Diff Lines
                        .child(
                            div()
                                .id("git_commit_diff_scroll")
                                .flex()
                                .flex_col()
                                .flex_1()
                                .overflow_y_scroll()
                                .p_1()
                                .children(if graph.diff_lines.is_empty() {
                                    vec![div()
                                        .p_4()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("No diff changes in this commit.")
                                        .into_any_element()]
                                } else {
                                    graph
                                        .diff_lines
                                        .iter()
                                        .map(|(line, kind)| {
                                            let (bg, fg) = match kind.as_str() {
                                                "add" => (rgba(0x10b98114), rgb(0x4ade80)),
                                                "del" => (rgba(0xef444414), rgb(0xf87171)),
                                                "hunk" => (rgba(0x38bdf814), rgb(0x38bdf8)),
                                                "header" => (rgba(0xffffff08), theme.text_muted),
                                                _ => (rgba(0x00000000), theme.text_subtle),
                                            };

                                            div()
                                                .px_2()
                                                .py_0p5()
                                                .bg(bg)
                                                .text_xs()
                                                .font_family("Consolas")
                                                .text_color(fg)
                                                .child(line.clone())
                                                .into_any_element()
                                        })
                                        .collect()
                                }),
                        )
                })),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_parse_git_log_and_lanes() {
        let sample = "e2b01234567890abcdef|Alice Developer|2 hours ago|feat: implement git graph parser|d1a09876543210 fedcba0987654321\n\
                      d1a09876543210fedcba|Bob Architect|5 hours ago|refactor: layout store|c09876543210fedc\n\
                      fedcba09876543210123|Charlie Tester|1 day ago|test: add unit tests|c09876543210fedc\n\
                      c09876543210fedcba98|Alice Developer|2 days ago|initial commit|";

        let mut commits = GitGraphState::parse_git_log(sample);
        assert_eq!(commits.len(), 4);

        // Check first commit
        assert_eq!(commits[0].short_hash, "e2b0123");
        assert_eq!(commits[0].author, "Alice Developer");
        assert_eq!(commits[0].author_initials, "AD");
        assert_eq!(commits[0].parents.len(), 2);
        assert_eq!(commits[0].parents[0], "d1a09876543210");
        assert_eq!(commits[0].parents[1], "fedcba0987654321");

        // Assign lanes
        GitGraphState::assign_lanes(&mut commits);
        assert_eq!(commits[0].lane, 0);
    }

    #[test]
    fn test_parse_diff_lines() {
        let diff_sample = "diff --git a/src/main.rs b/src/main.rs\n\
                           index 1234567..abcdef0 100644\n\
                           --- a/src/main.rs\n\
                           +++ b/src/main.rs\n\
                           @@ -10,4 +10,5 @@\n\
                            fn main() {\n\
                           -    println!(\"old\");\n\
                           +    println!(\"new\");\n\
                           +    println!(\"second\");\n\
                            }";

        let lines = GitGraphState::parse_diff(diff_sample);
        assert_eq!(lines.len(), 10);
        assert_eq!(lines[0].1, "header");
        assert_eq!(lines[4].1, "hunk");
        assert_eq!(lines[5].1, "plain");
        assert_eq!(lines[6].1, "del");
        assert_eq!(lines[7].1, "add");
        assert_eq!(lines[8].1, "add");
        assert_eq!(lines[9].1, "plain");
    }

    #[test]
    fn test_git_graph_filtering_and_selection() {
        let mut state = GitGraphState::default();
        let sample = "11111111111111111111|Alice|1m ago|feature one|\n\
                      22222222222222222222|Bob|2m ago|fix bug in parsing|\n\
                      33333333333333333333|Alice|3m ago|docs update|";

        state.commits = GitGraphState::parse_git_log(sample);
        assert_eq!(state.filtered_commits().len(), 3);

        // Filter by author
        state.filter_query = "bob".to_string();
        let filtered = state.filtered_commits();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].author, "Bob");

        // Filter by message
        state.filter_query = "docs".to_string();
        assert_eq!(state.filtered_commits().len(), 1);

        // Close diff
        state.selected_commit = Some("11111111111111111111".to_string());
        state.close_diff();
        assert!(state.selected_commit.is_none());
    }
}
