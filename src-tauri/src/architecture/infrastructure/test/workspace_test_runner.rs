use std::fs;
use std::path::{Path, PathBuf};

use crate::architecture::domain::test::{TestCase, TestFramework, TestRunnerRepository, TestRunResult};
#[cfg(feature = "tauri")]
use crate::architecture::infrastructure::gradle::gradle_util::resolve_gradlew;
use crate::architecture::infrastructure::test::test_runner_logic::{
    run_all_command, run_file_command,
};
use crate::process_ext::hidden_command;

pub struct WorkspaceTestRunner;

impl WorkspaceTestRunner {
    pub fn new() -> Self {
        Self
    }

    fn read_text(path: &Path) -> Option<String> {
        fs::read_to_string(path).ok()
    }

    fn execute(
        root_path: &Path,
        program: &str,
        args: Vec<String>,
    ) -> Result<TestRunResult, String> {
        let (program, args) = if program == "__gradlew__" {
            #[cfg(feature = "tauri")]
            {
                let gradlew = resolve_gradlew(root_path)
                    .ok_or("Gradle wrapper not found (gradlew / gradlew.bat)")?;
                (
                    gradlew.to_string_lossy().to_string(),
                    args,
                )
            }
            #[cfg(not(feature = "tauri"))]
            {
                return Err("Gradle support requires the Tauri shell".to_string());
            }
        } else {
            (program.to_string(), args)
        };
        let output = hidden_command(program)
            .args(args)
            .current_dir(root_path)
            .output()
            .map_err(|e| format!("Test run failed: {e}"))?;
        Ok(TestRunResult {
            ok: output.status.success(),
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

impl Default for WorkspaceTestRunner {
    fn default() -> Self {
        Self::new()
    }
}

impl TestRunnerRepository for WorkspaceTestRunner {
    fn sniff_framework(&self, root: &str) -> TestFramework {
        let root_path = PathBuf::from(root);
        if Self::read_text(&root_path.join("Cargo.toml")).is_some() {
            return TestFramework::Cargo;
        }
        if let Some(pkg) = Self::read_text(&root_path.join("package.json")) {
            if pkg.contains("vitest") {
                return TestFramework::Vitest;
            }
            if pkg.contains("jest") {
                return TestFramework::Jest;
            }
            if pkg.contains("bun") {
                return TestFramework::Bun;
            }
        }
        if Self::read_text(&root_path.join("pyproject.toml")).is_some()
            || root_path.join("pytest.ini").is_file()
        {
            return TestFramework::Pytest;
        }
        if Self::read_text(&root_path.join("go.mod")).is_some() {
            return TestFramework::Go;
        }
        if root_path.join("settings.gradle").is_file()
            || root_path.join("settings.gradle.kts").is_file()
        {
            return TestFramework::Gradle;
        }
        TestFramework::Unknown
    }

    fn discover_tests(&self, root: &str, framework: TestFramework) -> Result<Vec<TestCase>, String> {
        let root_path = PathBuf::from(root);
        let mut cases = Vec::new();
        fn walk(dir: &Path, framework: TestFramework, out: &mut Vec<TestCase>) {
            let Ok(read) = fs::read_dir(dir) else { return };
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if name == "node_modules" || name == "target" || name == ".git" {
                        continue;
                    }
                    walk(&path, framework, out);
                    continue;
                }
                let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                let matches = match framework {
                    TestFramework::Vitest | TestFramework::Jest | TestFramework::Bun => {
                        file_name.ends_with(".test.ts")
                            || file_name.ends_with(".test.tsx")
                            || file_name.ends_with(".spec.ts")
                            || file_name.ends_with(".spec.tsx")
                    }
                    TestFramework::Pytest => {
                        file_name.starts_with("test_") || file_name.ends_with("_test.py")
                    }
                    TestFramework::Go => file_name.ends_with("_test.go"),
                    TestFramework::Cargo => {
                        file_name.ends_with(".rs") && path.to_string_lossy().contains("test")
                    }
                    TestFramework::Gradle => {
                        file_name.ends_with("Test.java") || file_name.ends_with("Test.kt")
                    }
                    TestFramework::Unknown => false,
                };
                if matches {
                    out.push(TestCase {
                        path: path.to_string_lossy().to_string(),
                        name: file_name.to_string(),
                        framework,
                    });
                }
            }
        }
        walk(&root_path, framework, &mut cases);
        Ok(cases)
    }

    fn run_test_file(&self, root: &str, framework: TestFramework, path: &str) -> Result<TestRunResult, String> {
        let root_path = PathBuf::from(root);
        let rel = path
            .strip_prefix(root)
            .unwrap_or(path)
            .trim_start_matches(['/', '\\']);
        let (program, args) = run_file_command(framework, rel)?;
        Self::execute(&root_path, &program, args)
    }

    fn run_all_tests(&self, root: &str, framework: TestFramework) -> Result<TestRunResult, String> {
        let root_path = PathBuf::from(root);
        let (program, args) = run_all_command(framework);
        Self::execute(&root_path, program, args)
    }
}

#[cfg(test)]
mod tests {
    use crate::architecture::infrastructure::test::test_runner_logic::gradle_test_task_for_file;

    #[test]
    fn gradle_wrapper_marker_resolves() {
        assert_eq!(
            gradle_test_task_for_file("feature/src/test/kotlin/FooTest.kt"),
            ":feature:testDebugUnitTest"
        );
    }
}
