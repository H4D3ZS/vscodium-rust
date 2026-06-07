use std::path::Path;

use crate::architecture::domain::test::TestFramework;

/// Infer Gradle module name from a test file path relative to project root.
pub fn gradle_module_for_test_path(rel: &str) -> String {
    let parts: Vec<&str> = rel.split(['/', '\\']).collect();
    if let Some(idx) = parts.iter().position(|&p| p == "src") {
        if idx > 0 {
            return parts[idx - 1].to_string();
        }
    }
    "app".to_string()
}

/// Standard Android unit-test Gradle task for a module.
pub fn gradle_unit_test_task(module: &str) -> String {
    format!(":{module}:testDebugUnitTest")
}

/// Single-file Gradle test task (runs the module's unit test task).
pub fn gradle_test_task_for_file(rel: &str) -> String {
    gradle_unit_test_task(&gradle_module_for_test_path(rel))
}

pub fn run_all_command(framework: TestFramework) -> (&'static str, Vec<String>) {
    match framework {
        TestFramework::Vitest => ("npx", vec!["vitest".into(), "run".into()]),
        TestFramework::Jest => ("npx", vec!["jest".into()]),
        TestFramework::Bun => ("bun", vec!["test".into()]),
        TestFramework::Cargo => ("cargo", vec!["test".into()]),
        TestFramework::Pytest => ("pytest", vec![]),
        TestFramework::Go => ("go", vec!["test".into(), "./...".into()]),
        TestFramework::Gradle => ("__gradlew__", vec!["test".into()]),
        TestFramework::Unknown => ("npm", vec!["test".into()]),
    }
}

pub fn run_file_command(
    framework: TestFramework,
    rel: &str,
) -> Result<(String, Vec<String>), String> {
    let rel = rel.trim_start_matches(['/', '\\']);
    match framework {
        TestFramework::Vitest => Ok((
            "npx".into(),
            vec!["vitest".into(), "run".into(), rel.to_string()],
        )),
        TestFramework::Jest => Ok(("npx".into(), vec!["jest".into(), rel.to_string()])),
        TestFramework::Bun => Ok(("bun".into(), vec!["test".into(), rel.to_string()])),
        TestFramework::Cargo => Ok((
            "cargo".into(),
            vec!["test".into(), "--".into(), rel.to_string()],
        )),
        TestFramework::Pytest => Ok(("pytest".into(), vec![rel.to_string()])),
        TestFramework::Go => {
            let package = Path::new(rel)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| ".".into());
            Ok(("go".into(), vec!["test".into(), package]))
        }
        TestFramework::Gradle => Ok(("__gradlew__".into(), vec![gradle_test_task_for_file(rel)])),
        TestFramework::Unknown => Err("Unknown test framework".into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gradle_module_from_path() {
        assert_eq!(
            gradle_module_for_test_path("app/src/test/java/com/foo/BarTest.kt"),
            "app"
        );
        assert_eq!(
            gradle_module_for_test_path("core/utils/src/test/kotlin/UtilTest.kt"),
            "utils"
        );
    }

    #[test]
    fn gradle_task_for_file() {
        assert_eq!(
            gradle_test_task_for_file("app/src/test/java/com/foo/BarTest.kt"),
            ":app:testDebugUnitTest"
        );
    }

    #[test]
    fn run_all_commands() {
        let (p, a) = run_all_command(TestFramework::Vitest);
        assert_eq!(p, "npx");
        assert_eq!(a[0], "vitest");
        let (_, ga) = run_all_command(TestFramework::Gradle);
        assert_eq!(ga[0], "test");
    }
}
