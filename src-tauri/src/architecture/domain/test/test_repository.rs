use super::{TestCase, TestFramework, TestRunResult};

/// Port: discover and run tests for a workspace.
pub trait TestRunnerRepository: Send + Sync {
    fn sniff_framework(&self, root: &str) -> TestFramework;

    fn discover_tests(&self, root: &str, framework: TestFramework) -> Result<Vec<TestCase>, String>;

    fn run_test_file(&self, root: &str, framework: TestFramework, path: &str) -> Result<TestRunResult, String>;

    fn run_all_tests(&self, root: &str, framework: TestFramework) -> Result<TestRunResult, String>;
}
