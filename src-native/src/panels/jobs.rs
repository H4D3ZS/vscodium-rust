use crate::app_state::HadesNativeState;
use crate::panels::traits::BottomPanelTab;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

/// Execution status of a background task or process.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JobStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

impl JobStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            JobStatus::Running => "Running",
            JobStatus::Completed => "Completed",
            JobStatus::Failed => "Failed",
            JobStatus::Cancelled => "Cancelled",
        }
    }
}

/// A monitored background process, compiler pass, or agent action.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BackgroundJob {
    pub id: String,
    pub name: String,
    pub description: String,
    pub progress: u8, // 0..=100
    pub status: JobStatus,
    pub started_at: String,
    pub cancellable: bool,
}

/// State for the Background Jobs monitor panel.
#[derive(Clone, Debug)]
pub struct JobsState {
    pub jobs: Vec<BackgroundJob>,
    pub next_id: usize,
}

impl Default for JobsState {
    fn default() -> Self {
        Self {
            jobs: vec![
                BackgroundJob {
                    id: "job-1".to_string(),
                    name: "Rust Analyzer Indexing".to_string(),
                    description: "Indexing workspace crates & dependencies".to_string(),
                    progress: 100,
                    status: JobStatus::Completed,
                    started_at: "Just now".to_string(),
                    cancellable: false,
                },
                BackgroundJob {
                    id: "job-2".to_string(),
                    name: "Kortex Vector Embeddings".to_string(),
                    description: "Generating semantic code vectors (1,420 chunks)".to_string(),
                    progress: 68,
                    status: JobStatus::Running,
                    started_at: "30s ago".to_string(),
                    cancellable: true,
                },
                BackgroundJob {
                    id: "job-3".to_string(),
                    name: "VSX Extension Cache Sync".to_string(),
                    description: "Synchronizing marketplace manifests".to_string(),
                    progress: 100,
                    status: JobStatus::Completed,
                    started_at: "1m ago".to_string(),
                    cancellable: false,
                },
            ],
            next_id: 4,
        }
    }
}

impl JobsState {
    pub fn running_count(&self) -> usize {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Running)
            .count()
    }

    pub fn register(&mut self, name: &str, description: &str, cancellable: bool) -> String {
        let id = format!("job-{}", self.next_id);
        self.next_id += 1;
        self.jobs.push(BackgroundJob {
            id: id.clone(),
            name: name.to_string(),
            description: description.to_string(),
            progress: 0,
            status: JobStatus::Running,
            started_at: "Just now".to_string(),
            cancellable,
        });
        id
    }

    pub fn update(&mut self, id: &str, progress: u8, status: JobStatus) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == id) {
            j.progress = progress.min(100);
            j.status = status;
        }
    }

    pub fn complete(&mut self, id: &str, ok: bool) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == id) {
            j.progress = 100;
            j.status = if ok {
                JobStatus::Completed
            } else {
                JobStatus::Failed
            };
        }
    }

    pub fn cancel(&mut self, id: &str) {
        if let Some(j) = self.jobs.iter_mut().find(|j| j.id == id) {
            if j.cancellable && j.status == JobStatus::Running {
                j.status = JobStatus::Cancelled;
            }
        }
    }

    pub fn clear_completed(&mut self) {
        self.jobs.retain(|j| j.status == JobStatus::Running);
    }
}

pub struct JobsPanel;

impl Default for JobsPanel {
    fn default() -> Self {
        Self
    }
}

impl BottomPanelTab for JobsPanel {
    fn id(&self) -> &'static str {
        "jobs"
    }

    fn title(&self) -> &'static str {
        "JOBS"
    }

    fn badge(&self, state: &HadesNativeState) -> Option<String> {
        let count = state.jobs.running_count();
        if count > 0 {
            Some(count.to_string())
        } else {
            None
        }
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        let theme = &state.theme;
        let jobs_state = &state.jobs;
        let running = jobs_state.running_count();

        div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.bg_editor)
            // Toolbar Header
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_between()
                    .h(px(34.0))
                    .px_3()
                    .bg(theme.bg_titlebar)
                    .border_b_1()
                    .border_color(theme.border_subtle)
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_row()
                                    .items_center()
                                    .gap_1p5()
                                    .child(icon_12(IconName::Zap, theme.accent))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child(format!("{running} ACTIVE BACKGROUND JOBS"))
                                    )
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            // Clear Completed button
                            .child(
                                div()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(3.0))
                                    .bg(theme.bg_raised)
                                    .border_1()
                                    .border_color(theme.border_subtle)
                                    .cursor_pointer()
                                    .hover(|s| s.bg(theme.bg_hover))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.state.jobs.clear_completed();
                                        this.state.toast_manager.push_info("Cleared completed background jobs");
                                        cx.notify();
                                    }))
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(theme.text_muted)
                                            .child("Clear Completed")
                                    )
                            )
                            // Spawn Background Demo Job button
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_1()
                                    .px_2()
                                    .py_1()
                                    .rounded(px(3.0))
                                    .bg(theme.accent)
                                    .cursor_pointer()
                                    .hover(|s| s.opacity(0.9))
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event, _window, cx| {
                                        this.state.jobs.register(
                                            "Cargo Build Incremental",
                                            "Compiling debug artifacts in background",
                                            true,
                                        );
                                        this.state.toast_manager.push_success("Spawned background job");
                                        cx.notify();
                                    }))
                                    .child(icon_12(IconName::Plus, theme.text_primary))
                                    .child(
                                        div()
                                            .text_xs()
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(theme.text_primary)
                                            .child("New Job")
                                    )
                            )
                    )
            )
            // Jobs List View
            .child(
                div()
                    .id("jobs_list_scroll")
                    .flex()
                    .flex_col()
                    .size_full()
                    .overflow_y_scroll()
                    .p_3()
                    .gap_2()
                    .children(if jobs_state.jobs.is_empty() {
                        vec![
                            div()
                                .p_4()
                                .text_xs()
                                .text_color(theme.text_muted)
                                .child("No background jobs currently tracked.")
                                .into_any_element()
                        ]
                    } else {
                        jobs_state.jobs.iter().map(|job| {
                            let job_id = job.id.clone();
                            let status = job.status;
                            let is_running = status == JobStatus::Running;
                            let progress = job.progress;

                            let status_color = match status {
                                JobStatus::Running => theme.accent,
                                JobStatus::Completed => theme.status_green,
                                JobStatus::Failed => theme.status_red,
                                JobStatus::Cancelled => theme.text_muted,
                            };

                            div()
                                .flex()
                                .flex_col()
                                .p_2p5()
                                .rounded(px(6.0))
                                .bg(theme.bg_card)
                                .border_1()
                                .border_color(theme.border_subtle)
                                .gap_1p5()
                                // Top row: Title, Status Chip, Cancel button
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
                                                .child(
                                                    div()
                                                        .size(px(7.0))
                                                        .rounded_full()
                                                        .bg(status_color)
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(theme.text_primary)
                                                        .child(job.name.clone())
                                                )
                                                .child(
                                                    div()
                                                        .px_1p5()
                                                        .py_0p5()
                                                        .rounded(px(3.0))
                                                        .bg(status_color.opacity(0.15))
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(status_color)
                                                        .child(status.as_str())
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
                                                        .text_xs()
                                                        .text_color(theme.text_muted)
                                                        .child(job.started_at.clone())
                                                )
                                                .children((is_running && job.cancellable).then(|| {
                                                    div()
                                                        .p_1()
                                                        .rounded(px(3.0))
                                                        .cursor_pointer()
                                                        .hover(|s| s.bg(theme.bg_hover))
                                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event, _window, cx| {
                                                            this.state.jobs.cancel(&job_id);
                                                            this.state.toast_manager.push_info("Cancelled job");
                                                            cx.notify();
                                                        }))
                                                        .child(icon_12(IconName::X, theme.text_muted))
                                                }))
                                        )
                                )
                                // Description row
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child(job.description.clone())
                                )
                                // Progress bar row
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex_1()
                                                .h(px(4.0))
                                                .rounded_full()
                                                .bg(theme.bg_raised)
                                                .overflow_hidden()
                                                .child(
                                                    div()
                                                        .h_full()
                                                        .w(px(progress as f32 * 3.0)) // Visual width scaled
                                                        .bg(status_color)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_muted)
                                                .child(format!("{progress}%"))
                                        )
                                )
                                .into_any_element()
                        }).collect()
                    })
            )
            .into_any_element()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_jobs_state_lifecycle() {
        let mut state = JobsState::default();
        state.jobs.clear();
        assert_eq!(state.running_count(), 0);

        // Register job
        let id = state.register("Build Crates", "Compiling hades", true);
        assert_eq!(state.running_count(), 1);

        // Update progress
        state.update(&id, 50, JobStatus::Running);
        let job = state.jobs.iter().find(|j| j.id == id).unwrap();
        assert_eq!(job.progress, 50);
        assert_eq!(job.status, JobStatus::Running);

        // Complete job
        state.complete(&id, true);
        let job = state.jobs.iter().find(|j| j.id == id).unwrap();
        assert_eq!(job.progress, 100);
        assert_eq!(job.status, JobStatus::Completed);
        assert_eq!(state.running_count(), 0);

        // Clear completed
        state.clear_completed();
        assert!(state.jobs.is_empty());
    }

    #[test]
    fn test_job_cancellation() {
        let mut state = JobsState::default();
        state.jobs.clear();
        let id = state.register("Long Task", "Running...", true);
        assert_eq!(state.running_count(), 1);

        state.cancel(&id);
        let job = state.jobs.iter().find(|j| j.id == id).unwrap();
        assert_eq!(job.status, JobStatus::Cancelled);
        assert_eq!(state.running_count(), 0);
    }
}
