use super::{GradleProject, GradleTask};

/// Port: Gradle project sync and task execution.
pub trait GradleBuildRepository: Send + Sync {
    fn detect_project(&self, root: &str) -> Result<GradleProject, String>;

    fn sync_project(&self, root: &str) -> Result<GradleProject, String>;

    fn list_tasks(&self, root: &str) -> Result<Vec<GradleTask>, String>;

    fn run_task(&self, root: &str, task: &str) -> Result<String, String>;
}
