use std::path::PathBuf;

use crate::architecture::domain::gradle::{
    GradleBuildRepository, GradleProject, GradleTask,
};
use crate::architecture::infrastructure::gradle::gradle_util::{
    parse_gradle_tasks, parse_modules, resolve_gradlew, sniff_android_kotlin,
};
use crate::process_ext::hidden_command;

pub struct CliGradleRepository;

impl CliGradleRepository {
    pub fn new() -> Self {
        Self
    }
}

impl Default for CliGradleRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl GradleBuildRepository for CliGradleRepository {
    fn detect_project(&self, root: &str) -> Result<GradleProject, String> {
        let root_path = PathBuf::from(root);
        if !root_path.is_dir() {
            return Err(format!("Not a directory: {root}"));
        }
        let has_gradle = root_path.join("settings.gradle").is_file()
            || root_path.join("settings.gradle.kts").is_file()
            || root_path.join("build.gradle").is_file()
            || root_path.join("build.gradle.kts").is_file();
        if !has_gradle {
            return Err("No Gradle project found".into());
        }
        let (is_android, uses_kotlin) = sniff_android_kotlin(&root_path);
        Ok(GradleProject {
            root: root.to_string(),
            is_android,
            uses_kotlin,
            wrapper_present: resolve_gradlew(&root_path).is_some(),
            modules: parse_modules(&root_path),
            tasks: vec![],
        })
    }

    fn sync_project(&self, root: &str) -> Result<GradleProject, String> {
        let mut project = self.detect_project(root)?;
        project.tasks = self.list_tasks(root)?;
        Ok(project)
    }

    fn list_tasks(&self, root: &str) -> Result<Vec<GradleTask>, String> {
        let root_path = PathBuf::from(root);
        let gradlew = resolve_gradlew(&root_path).ok_or("Gradle wrapper not found (gradlew / gradlew.bat)")?;
        let output = hidden_command(gradlew.to_string_lossy().to_string())
            .current_dir(&root_path)
            .args(["tasks", "--all", "--no-daemon", "--console=plain"])
            .output()
            .map_err(|e| format!("gradlew tasks failed: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(parse_gradle_tasks(&stdout))
    }

    fn run_task(&self, root: &str, task: &str) -> Result<String, String> {
        let root_path = PathBuf::from(root);
        let gradlew = resolve_gradlew(&root_path).ok_or("Gradle wrapper not found")?;
        let output = hidden_command(gradlew.to_string_lossy().to_string())
            .current_dir(&root_path)
            .args([task, "--no-daemon", "--console=plain"])
            .output()
            .map_err(|e| format!("gradlew {task} failed: {e}"))?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        if output.status.success() {
            Ok(format!("{stdout}\n{stderr}"))
        } else {
            Err(format!("Task failed:\n{stdout}\n{stderr}"))
        }
    }
}
