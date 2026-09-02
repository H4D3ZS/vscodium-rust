use std::sync::Arc;

use crate::architecture::domain::test::{TestCase, TestFramework, TestRunResult, TestRunnerRepository};
use crate::architecture::infrastructure::test::WorkspaceTestRunner;

pub struct TestRunnerService {
    repo: Arc<WorkspaceTestRunner>,
}

impl TestRunnerService {
    pub fn new() -> Self {
        Self {
            repo: Arc::new(WorkspaceTestRunner::new()),
        }
    }

    pub fn sniff_framework(&self, root: &str) -> TestFramework {
        self.repo.sniff_framework(root)
    }

    pub fn discover(&self, root: &str) -> Result<Vec<TestCase>, String> {
        let fw = self.repo.sniff_framework(root);
        self.repo.discover_tests(root, fw)
    }

    pub fn run_file(&self, root: &str, path: &str) -> Result<TestRunResult, String> {
        let fw = self.repo.sniff_framework(root);
        self.repo.run_test_file(root, fw, path)
    }

    pub fn run_all(&self, root: &str) -> Result<TestRunResult, String> {
        let fw = self.repo.sniff_framework(root);
        self.repo.run_all_tests(root, fw)
    }
}

impl Default for TestRunnerService {
    fn default() -> Self {
        Self::new()
    }
}
