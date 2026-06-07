use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TestFramework {
    Vitest,
    Jest,
    Bun,
    Cargo,
    Pytest,
    Go,
    Gradle,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub path: String,
    pub name: String,
    pub framework: TestFramework,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestRunResult {
    pub ok: bool,
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}
