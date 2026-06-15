use std::sync::Arc;

use crate::architecture::domain::gradle::{GradleBuildRepository, GradleProject, GradleTask};
use crate::architecture::infrastructure::gradle::CliGradleRepository;

pub struct GradleService {
    repo: Arc<CliGradleRepository>,
}

impl GradleService {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(CliGradleRepository::new()),
        }
    }

    pub fn detect(&self, root: &str) -> Result<GradleProject, String> {
        self.repo.detect_project(root)
    }

    pub fn sync(&self, root: &str) -> Result<GradleProject, String> {
        self.repo.sync_project(root)
    }

    pub fn list_tasks(&self, root: &str) -> Result<Vec<GradleTask>, String> {
        self.repo.list_tasks(root)
    }

    pub fn run_task(&self, root: &str, task: &str) -> Result<String, String> {
        self.repo.run_task(root, task)
    }
}

impl Default for GradleService {
    fn default() -> Self {
        Self::new()
    }
}
