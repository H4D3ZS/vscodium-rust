use std::fs;
use std::path::{Path, PathBuf};

use crate::architecture::domain::gradle::{GradleModule, GradleTask};

/// Resolve Gradle wrapper script (Windows `.bat` or Unix shell script).
pub fn resolve_gradlew(root: &Path) -> Option<PathBuf> {
    let win = root.join("gradlew.bat");
    if win.is_file() {
        return Some(win);
    }
    let unix = root.join("gradlew");
    if unix.is_file() {
        return Some(unix);
    }
    None
}

pub fn read_text(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

pub fn parse_modules(root: &Path) -> Vec<GradleModule> {
    let mut modules = vec![GradleModule {
        path: root.to_string_lossy().to_string(),
        name: "root".into(),
    }];
    for file in ["settings.gradle", "settings.gradle.kts"] {
        let settings = root.join(file);
        if !settings.is_file() {
            continue;
        }
        let text = read_text(&settings);
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("include(") || trimmed.starts_with("include ") {
                for name in extract_gradle_module_names(trimmed) {
                    modules.push(GradleModule {
                        path: name.replace(':', "/"),
                        name: name.clone(),
                    });
                }
            }
        }
    }
    modules.sort_by(|a, b| a.name.cmp(&b.name));
    modules.dedup_by(|a, b| a.name == b.name);
    modules
}

/// Extract `:app`, `:core:utils`, etc. from Gradle `include(...)` lines.
pub fn extract_gradle_module_names(line: &str) -> Vec<String> {
    let mut names = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b':' && i + 1 < bytes.len() && bytes[i + 1].is_ascii_alphanumeric() {
            let start = i + 1;
            let mut j = start;
            while j < bytes.len()
                && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b':' || bytes[j] == b'_')
            {
                j += 1;
            }
            names.push(line[start..j].to_string());
            i = j;
        } else {
            i += 1;
        }
    }
    names
}

pub fn sniff_android_kotlin(root: &Path) -> (bool, bool) {
    let mut is_android = false;
    let mut uses_kotlin = false;
    for entry in fs::read_dir(root).into_iter().flatten().flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name == "build.gradle" || name == "build.gradle.kts" {
            let text = read_text(&path).to_lowercase();
            if text.contains("com.android.application") || text.contains("com.android.library") {
                is_android = true;
            }
            if text.contains("kotlin-android") || text.contains("org.jetbrains.kotlin") {
                uses_kotlin = true;
            }
        }
    }
    if root.join("AndroidManifest.xml").is_file() {
        is_android = true;
    }
    (is_android, uses_kotlin)
}

/// Parse `gradlew tasks --all` plain console output.
pub fn parse_gradle_tasks(stdout: &str) -> Vec<GradleTask> {
    let mut tasks = Vec::new();
    let mut current_group: Option<String> = None;
    for line in stdout.lines() {
        if line.ends_with(" tasks") && line.starts_with('>') {
            current_group = Some(line.trim_start_matches('>').trim().to_string());
            continue;
        }
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("To see") || trimmed.starts_with("---") {
            continue;
        }
        if trimmed.contains(' ') {
            let name = trimmed.split_whitespace().next().unwrap_or("").to_string();
            if name.contains(':')
                || name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == ':' || c == '-')
            {
                tasks.push(GradleTask {
                    path: name.clone(),
                    name,
                    group: current_group.clone(),
                    description: Some(trimmed.to_string()),
                });
            }
        }
    }
    tasks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_module_names_from_include_call() {
        let names = extract_gradle_module_names(r#"include(":app", ":core:utils")"#);
        assert!(names.contains(&"app".to_string()));
        assert!(names.contains(&"core:utils".to_string()));
    }

    #[test]
    fn parse_modules_from_settings_kts() {
        let dir = std::env::temp_dir().join("vscodium_gradle_test_modules");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("settings.gradle.kts"),
            r#"include(":app", ":core:utils")"#,
        )
        .unwrap();
        let modules = parse_modules(&dir);
        assert!(modules.iter().any(|m| m.name == "app"));
        assert!(modules.iter().any(|m| m.name == "core:utils"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_gradle_tasks_groups() {
        let stdout = r#"
> Task tasks

> Build tasks
assembleDebug - Assembles Debug APK
assembleRelease - Assembles Release APK

> Verification tasks
testDebugUnitTest - Run unit tests
"#;
        let tasks = parse_gradle_tasks(stdout);
        assert!(tasks.iter().any(|t| t.name == "assembleDebug"));
        assert!(tasks.iter().any(|t| t.name == "testDebugUnitTest"));
        assert!(tasks
            .iter()
            .find(|t| t.name == "assembleDebug")
            .and_then(|t| t.group.as_deref())
            .unwrap_or("")
            .contains("Build"));
    }

    #[test]
    fn resolve_gradlew_prefers_bat_on_windows() {
        let dir = std::env::temp_dir().join("vscodium_gradle_test_wrapper");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        fs::write(dir.join("gradlew.bat"), "@echo off").unwrap();
        fs::write(dir.join("gradlew"), "#!/bin/sh").unwrap();
        let p = resolve_gradlew(&dir).unwrap();
        assert!(p.ends_with("gradlew.bat") || p.ends_with("gradlew"));
        let _ = fs::remove_dir_all(&dir);
    }
}
