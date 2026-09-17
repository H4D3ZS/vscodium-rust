use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::path::{Path, PathBuf};

/// A VS Code task defined in `.vscode/tasks.json`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VsCodeTask {
    pub label: String,
    #[serde(rename = "type", default)]
    pub task_type: Option<String>,
    #[serde(default)]
    pub command: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(rename = "isDefault", default)]
    pub is_default: bool,
}

/// A VS Code launch configuration defined in `.vscode/launch.json`.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct VsCodeLaunchConfig {
    pub name: String,
    #[serde(rename = "type", default)]
    pub config_type: Option<String>,
    #[serde(default)]
    pub request: Option<String>,
    #[serde(default)]
    pub program: Option<String>,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub cwd: Option<String>,
    #[serde(rename = "preLaunchTask", default)]
    pub pre_launch_task: Option<String>,
    #[serde(rename = "adapterPath", default)]
    pub adapter_path: Option<String>,
}

/// Context used for VS Code variable substitution.
#[derive(Clone, Debug, Default)]
pub struct VarContext {
    pub workspace_folder: Option<String>,
    pub file: Option<String>,
    pub line_number: Option<usize>,
    pub selected_text: Option<String>,
    pub exec_path: Option<String>,
}

/// Strip C-style comments (`//` and `/* ... */`) and trailing commas from JSONC.
pub fn strip_jsonc(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let chars: Vec<char> = raw.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;
    let mut string_escape = false;

    while i < len {
        let ch = chars[i];

        if in_string {
            out.push(ch);
            if string_escape {
                string_escape = false;
            } else if ch == '\\' {
                string_escape = true;
            } else if ch == '"' {
                in_string = false;
            }
            i += 1;
            continue;
        }

        if ch == '"' {
            in_string = true;
            out.push(ch);
            i += 1;
            continue;
        }

        // Check single-line comment `//`
        if ch == '/' && i + 1 < len && chars[i + 1] == '/' {
            i += 2;
            while i < len && chars[i] != '\n' {
                i += 1;
            }
            continue;
        }

        // Check multi-line comment `/* ... */`
        if ch == '/' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            while i + 1 < len && !(chars[i] == '*' && chars[i + 1] == '/') {
                i += 1;
            }
            i += 2; // Skip `*/`
            continue;
        }

        out.push(ch);
        i += 1;
    }

    // Best effort: remove trailing commas before closing braces/brackets
    clean_trailing_commas(&out)
}

fn clean_trailing_commas(s: &str) -> String {
    let mut cleaned = String::with_capacity(s.len());
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    let mut i = 0;
    let mut in_string = false;

    while i < len {
        let ch = chars[i];
        if ch == '"' {
            in_string = !in_string;
            cleaned.push(ch);
            i += 1;
            continue;
        }

        if !in_string && ch == ',' {
            // Peek forward to see if the next non-whitespace char is '}' or ']'
            let mut j = i + 1;
            while j < len && chars[j].is_whitespace() {
                j += 1;
            }
            if j < len && (chars[j] == '}' || chars[j] == ']') {
                i += 1;
                continue; // Skip the trailing comma
            }
        }

        cleaned.push(ch);
        i += 1;
    }

    cleaned
}

/// Substitute VS Code variables (`${workspaceFolder}`, `${file}`, etc.).
pub fn substitute_vars(template: &str, ctx: &VarContext) -> String {
    if template.is_empty() {
        return String::new();
    }

    let ws = ctx.workspace_folder.as_deref().unwrap_or("");
    let file = ctx.file.as_deref().unwrap_or("");
    let sep = if file.contains('\\') || ws.contains('\\') {
        "\\"
    } else {
        "/"
    };

    let file_path = Path::new(file);
    let ws_path = Path::new(ws);

    let basename = file_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let stem = file_path
        .file_stem()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    let ext = file_path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let dirname = file_path
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();
    let dirname_basename = Path::new(&dirname)
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let ws_basename = ws_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let relative_file = if !file.is_empty() && !ws.is_empty() && file.starts_with(ws) {
        let trimmed = &file[ws.len()..];
        trimmed.trim_start_matches(['/', '\\']).to_string()
    } else {
        file.to_string()
    };
    let relative_dirname = Path::new(&relative_file)
        .parent()
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_default();

    let line_str = ctx.line_number.map(|l| l.to_string()).unwrap_or_default();
    let selected = ctx.selected_text.as_deref().unwrap_or("");
    let exec = ctx.exec_path.as_deref().unwrap_or("");

    let mut result = String::with_capacity(template.len());
    let mut rest = template;

    while let Some(start_pos) = rest.find("${") {
        result.push_str(&rest[..start_pos]);
        let after_start = &rest[start_pos + 2..];
        if let Some(end_pos) = after_start.find('}') {
            let var_expr = &after_start[..end_pos];
            let replacement = if let Some(env_name) = var_expr.strip_prefix("env:") {
                std::env::var(env_name).unwrap_or_default()
            } else {
                match var_expr {
                    "workspaceFolder" | "cwd" => ws.to_string(),
                    "workspaceFolderBasename" => ws_basename.clone(),
                    "file" => file.to_string(),
                    "fileWorkspaceFolder" => ws.to_string(),
                    "fileBasename" => basename.clone(),
                    "fileBasenameNoExtension" => stem.clone(),
                    "fileExtname" => ext.clone(),
                    "fileDirname" => dirname.clone(),
                    "fileDirnameBasename" => dirname_basename.clone(),
                    "relativeFile" => relative_file.clone(),
                    "relativeFileDirname" => relative_dirname.clone(),
                    "lineNumber" => line_str.clone(),
                    "selectedText" => selected.to_string(),
                    "execPath" => exec.to_string(),
                    "pathSeparator" | "/" => sep.to_string(),
                    other => format!("${{{other}}}"),
                }
            };
            result.push_str(&replacement);
            rest = &after_start[end_pos + 1..];
        } else {
            result.push_str(&rest[start_pos..]);
            break;
        }
    }
    result.push_str(rest);
    result
}

/// Construct full command string for a task with argument quotation.
pub fn build_task_command(task: &VsCodeTask, ctx: &VarContext) -> String {
    let cmd = substitute_vars(task.command.as_deref().unwrap_or(""), ctx);
    if task.args.is_empty() {
        return cmd;
    }

    let mut parts = vec![cmd];
    for arg in &task.args {
        let resolved = substitute_vars(arg, ctx);
        if resolved.contains(' ') && !resolved.starts_with('"') {
            parts.push(format!("\"{resolved}\""));
        } else {
            parts.push(resolved);
        }
    }
    parts.join(" ")
}

/// Parse `.vscode/tasks.json` string into a list of tasks.
pub fn parse_tasks_json(raw: &str) -> Vec<VsCodeTask> {
    let sanitized = strip_jsonc(raw);
    #[derive(serde::Deserialize)]
    struct TasksFile {
        #[serde(default)]
        tasks: Vec<VsCodeTask>,
    }

    serde_json::from_str::<TasksFile>(&sanitized)
        .map(|f| f.tasks)
        .unwrap_or_default()
}

/// Parse `.vscode/launch.json` string into a list of launch configs.
pub fn parse_launch_json(raw: &str) -> Vec<VsCodeLaunchConfig> {
    let sanitized = strip_jsonc(raw);
    #[derive(serde::Deserialize)]
    struct LaunchFile {
        #[serde(default)]
        configurations: Vec<VsCodeLaunchConfig>,
    }

    serde_json::from_str::<LaunchFile>(&sanitized)
        .map(|f| f.configurations)
        .unwrap_or_default()
}

/// State of the Run & Debug configuration manager.
#[derive(Clone, Debug)]
pub struct RunConfigsState {
    pub tasks: Vec<VsCodeTask>,
    pub launch_configs: Vec<VsCodeLaunchConfig>,
    pub selected_launch_index: usize,
    pub is_tasks_collapsed: bool,
    pub is_launch_collapsed: bool,
    pub last_status: Option<(String, bool, String)>, // (label, ok, message)
}

impl Default for RunConfigsState {
    fn default() -> Self {
        Self {
            tasks: Vec::new(),
            launch_configs: Vec::new(),
            selected_launch_index: 0,
            is_tasks_collapsed: false,
            is_launch_collapsed: false,
            last_status: None,
        }
    }
}

impl RunConfigsState {
    pub fn new(workspace_root: &Path) -> Self {
        let mut state = Self::default();
        state.reload(workspace_root);
        state
    }

    pub fn reload(&mut self, workspace_root: &Path) {
        let vscode_dir = workspace_root.join(".vscode");
        let tasks_path = vscode_dir.join("tasks.json");
        let launch_path = vscode_dir.join("launch.json");

        if tasks_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&tasks_path) {
                self.tasks = parse_tasks_json(&content);
            }
        } else {
            // Default built-in discovered tasks for Cargo/Rust workspace
            self.tasks = Self::discovered_default_tasks(workspace_root);
        }

        if launch_path.exists() {
            if let Ok(content) = std::fs::read_to_string(&launch_path) {
                self.launch_configs = parse_launch_json(&content);
            }
        } else {
            // Default built-in launch configs
            self.launch_configs = Self::discovered_default_launch_configs(workspace_root);
        }

        if self.selected_launch_index >= self.launch_configs.len() {
            self.selected_launch_index = 0;
        }
    }

    pub fn selected_launch_config(&self) -> Option<&VsCodeLaunchConfig> {
        self.launch_configs.get(self.selected_launch_index)
    }

    pub fn discovered_default_tasks(root: &Path) -> Vec<VsCodeTask> {
        let is_rust =
            root.join("Cargo.toml").exists() || root.join("src-native").join("Cargo.toml").exists();
        if is_rust {
            vec![
                VsCodeTask {
                    label: "cargo check (vscodium-rust)".to_string(),
                    task_type: Some("shell".to_string()),
                    command: Some("cargo check --bin vscodium-rust".to_string()),
                    args: Vec::new(),
                    cwd: Some("${workspaceFolder}/src-native".to_string()),
                    group: Some("build".to_string()),
                    is_default: true,
                },
                VsCodeTask {
                    label: "cargo test (vscodium-rust)".to_string(),
                    task_type: Some("shell".to_string()),
                    command: Some("cargo test --bin vscodium-rust".to_string()),
                    args: Vec::new(),
                    cwd: Some("${workspaceFolder}/src-native".to_string()),
                    group: Some("test".to_string()),
                    is_default: false,
                },
                VsCodeTask {
                    label: "cargo build --release".to_string(),
                    task_type: Some("shell".to_string()),
                    command: Some("cargo build --release --bin vscodium-rust".to_string()),
                    args: Vec::new(),
                    cwd: Some("${workspaceFolder}/src-native".to_string()),
                    group: Some("build".to_string()),
                    is_default: false,
                },
            ]
        } else {
            Vec::new()
        }
    }

    pub fn discovered_default_launch_configs(root: &Path) -> Vec<VsCodeLaunchConfig> {
        let is_rust =
            root.join("Cargo.toml").exists() || root.join("src-native").join("Cargo.toml").exists();
        if is_rust {
            vec![
                VsCodeLaunchConfig {
                    name: "Cargo Run (vscodium-rust)".to_string(),
                    config_type: Some("lldb".to_string()),
                    request: Some("launch".to_string()),
                    program: Some(
                        "${workspaceFolder}/src-native/target/debug/vscodium-rust".to_string(),
                    ),
                    args: Vec::new(),
                    cwd: Some("${workspaceFolder}/src-native".to_string()),
                    pre_launch_task: Some("cargo check (vscodium-rust)".to_string()),
                    adapter_path: None,
                },
                VsCodeLaunchConfig {
                    name: "Debug Current Test".to_string(),
                    config_type: Some("lldb".to_string()),
                    request: Some("launch".to_string()),
                    program: Some(
                        "${workspaceFolder}/src-native/target/debug/deps/vscodium_rust".to_string(),
                    ),
                    args: vec!["--nocapture".to_string()],
                    cwd: Some("${workspaceFolder}/src-native".to_string()),
                    pre_launch_task: None,
                    adapter_path: None,
                },
            ]
        } else {
            Vec::new()
        }
    }

    pub fn write_sample_tasks_json(workspace_root: &Path) -> Result<PathBuf, String> {
        let vscode_dir = workspace_root.join(".vscode");
        std::fs::create_dir_all(&vscode_dir).map_err(|e| e.to_string())?;
        let tasks_file = vscode_dir.join("tasks.json");
        let content = r#"{
    // See https://go.microsoft.com/fwlink/?LinkId=733558
    // for the documentation about the tasks.json format
    "version": "2.0.0",
    "tasks": [
        {
            "label": "cargo check (vscodium-rust)",
            "type": "shell",
            "command": "cargo check --bin vscodium-rust",
            "cwd": "${workspaceFolder}/src-native",
            "group": "build",
            "problemMatcher": ["$rustc"]
        },
        {
            "label": "cargo test (vscodium-rust)",
            "type": "shell",
            "command": "cargo test --bin vscodium-rust",
            "cwd": "${workspaceFolder}/src-native",
            "group": "test"
        }
    ]
}
"#;
        std::fs::write(&tasks_file, content).map_err(|e| e.to_string())?;
        Ok(tasks_file)
    }

    pub fn write_sample_launch_json(workspace_root: &Path) -> Result<PathBuf, String> {
        let vscode_dir = workspace_root.join(".vscode");
        std::fs::create_dir_all(&vscode_dir).map_err(|e| e.to_string())?;
        let launch_file = vscode_dir.join("launch.json");
        let content = r#"{
    // Use IntelliSense to learn about possible attributes.
    // Hover to view descriptions of existing attributes.
    // For more information, visit: https://go.microsoft.com/fwlink/?linkid=830387
    "version": "0.2.0",
    "configurations": [
        {
            "name": "Cargo Run (vscodium-rust)",
            "type": "lldb",
            "request": "launch",
            "program": "${workspaceFolder}/src-native/target/debug/vscodium-rust",
            "args": [],
            "cwd": "${workspaceFolder}/src-native",
            "preLaunchTask": "cargo check (vscodium-rust)"
        }
    ]
}
"#;
        std::fs::write(&launch_file, content).map_err(|e| e.to_string())?;
        Ok(launch_file)
    }
}

/// Render the Tasks and Run Configurations UI section (integrated into Run & Debug).
pub fn render_run_configs_section(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let run_configs = &state.run_configs;

    div()
        .flex()
        .flex_col()
        .w_full()
        // ── Top Title Bar: RUN & DEBUG + Reload ──
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(icon_14(IconName::Bug, theme.accent))
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("RUN & DEBUG"),
                        ),
                )
                .child(
                    div()
                        .cursor_pointer()
                        .p_1()
                        .rounded(px(3.0))
                        .hover(|s| s.bg(theme.bg_hover))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                let root = this.state.workspace_root.clone();
                                this.state.run_configs.reload(&root);
                                this.state
                                    .toast_manager
                                    .push_info("Reloaded tasks.json & launch.json");
                                cx.notify();
                            }),
                        )
                        .child(icon_12(IconName::RefreshCw, theme.text_muted)),
                ),
        )
        // ── Last Status Banner (if an action was executed) ──
        .children(run_configs.last_status.as_ref().map(|(name, ok, msg)| {
            let ok_val = *ok;
            div()
                .mx_2()
                .my_1p5()
                .p_1p5()
                .rounded(px(4.0))
                .bg(if ok_val {
                    theme.status_green.opacity(0.12)
                } else {
                    theme.status_red.opacity(0.12)
                })
                .border_1()
                .border_color(if ok_val {
                    theme.status_green.opacity(0.35)
                } else {
                    theme.status_red.opacity(0.35)
                })
                .flex()
                .flex_col()
                .gap_0p5()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if ok_val {
                            theme.status_green
                        } else {
                            theme.status_red
                        })
                        .child(name.clone()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(theme.text_muted)
                        .child(msg.clone()),
                )
        }))
        // ── Section 1: Tasks ──
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_3()
                        .py_1()
                        .bg(theme.bg_raised)
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.run_configs.is_tasks_collapsed =
                                    !this.state.run_configs.is_tasks_collapsed;
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(icon_12(
                                    if run_configs.is_tasks_collapsed {
                                        IconName::ChevronRight
                                    } else {
                                        IconName::ChevronDown
                                    },
                                    theme.text_muted,
                                ))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_muted)
                                        .child("TASKS"),
                                ),
                        ),
                )
                .children((!run_configs.is_tasks_collapsed).then(|| {
                    div().flex().flex_col().gap_1().px_2().py_1().children(
                        if run_configs.tasks.is_empty() {
                            vec![div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_card)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("No .vscode/tasks.json in this workspace. Add one with a tasks array of label/command entries."),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .py_1()
                                        .px_2()
                                        .rounded(px(3.0))
                                        .bg(theme.accent)
                                        .cursor_pointer()
                                        .hover(|s| s.opacity(0.9))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                let root = this.state.workspace_root.clone();
                                                match RunConfigsState::write_sample_tasks_json(&root) {
                                                    Ok(_) => {
                                                        this.state.run_configs.reload(&root);
                                                        this.state.toast_manager.push_success(
                                                            "Created .vscode/tasks.json",
                                                        );
                                                    }
                                                    Err(e) => this
                                                        .state
                                                        .toast_manager
                                                        .push_error(&format!("Failed: {e}")),
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("Add tasks.json"),
                                        ),
                                )
                                .into_any_element()]
                        } else {
                            run_configs
                                .tasks
                                .iter()
                                .map(|task| {
                                    let task_clone = task.clone();
                                    let task_label = task.label.clone();
                                    let task_label_click = task.label.clone();
                                    let cmd_preview = format!(
                                        "{} {}",
                                        task.command.as_deref().unwrap_or(""),
                                        task.args.join(" ")
                                    );

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .p_1p5()
                                        .rounded(px(4.0))
                                        .bg(theme.bg_card)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                let ctx = VarContext {
                                                    workspace_folder: Some(
                                                        this.state
                                                            .workspace_root
                                                            .to_string_lossy()
                                                            .to_string(),
                                                    ),
                                                    file: this
                                                        .state
                                                        .active_tab()
                                                        .map(|t| t.path.clone()),
                                                    line_number: this
                                                        .state
                                                        .active_tab()
                                                        .map(|t| t.cursor_row + 1),
                                                    selected_text: None,
                                                    exec_path: std::env::current_exe()
                                                        .ok()
                                                        .map(|p| p.to_string_lossy().to_string()),
                                                };
                                                let full_cmd = build_task_command(&task_clone, &ctx);
                                                this.execute_terminal_command(&full_cmd);
                                                this.state.bottom_panel_open = true;
                                                this.state.run_configs.last_status = Some((
                                                    task_label_click.clone(),
                                                    true,
                                                    format!("Running in terminal: {}", full_cmd),
                                                ));
                                                this.state
                                                    .toast_manager
                                                    .push_info(&format!("Running task: {}", task_label_click));
                                                cx.notify();
                                            }),
                                        )
                                        .child(icon_12(IconName::Play, theme.status_green))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .flex_1()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(task_label),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_family("Consolas")
                                                        .text_color(theme.text_muted)
                                                        .child(cmd_preview),
                                                ),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        },
                    )
                })),
        )
        // ── Section 2: Launch Configurations ──
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .px_3()
                        .py_1()
                        .bg(theme.bg_raised)
                        .cursor_pointer()
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event, _window, cx| {
                                this.state.run_configs.is_launch_collapsed =
                                    !this.state.run_configs.is_launch_collapsed;
                                cx.notify();
                            }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(icon_12(
                                    if run_configs.is_launch_collapsed {
                                        IconName::ChevronRight
                                    } else {
                                        IconName::ChevronDown
                                    },
                                    theme.text_muted,
                                ))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.text_muted)
                                        .child("LAUNCH CONFIGURATIONS"),
                                ),
                        ),
                )
                .children((!run_configs.is_launch_collapsed).then(|| {
                    div().flex().flex_col().gap_1().px_2().py_1().children(
                        if run_configs.launch_configs.is_empty() {
                            vec![div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .p_2()
                                .rounded(px(4.0))
                                .bg(theme.bg_card)
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .child("No .vscode/launch.json in this workspace. Add one with a configurations array of debugger entries."),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .py_1()
                                        .px_2()
                                        .rounded(px(3.0))
                                        .bg(theme.accent)
                                        .cursor_pointer()
                                        .hover(|s| s.opacity(0.9))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                let root = this.state.workspace_root.clone();
                                                match RunConfigsState::write_sample_launch_json(&root) {
                                                    Ok(_) => {
                                                        this.state.run_configs.reload(&root);
                                                        this.state.toast_manager.push_success(
                                                            "Created .vscode/launch.json",
                                                        );
                                                    }
                                                    Err(e) => this
                                                        .state
                                                        .toast_manager
                                                        .push_error(&format!("Failed: {e}")),
                                                }
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.text_primary)
                                                .child("Add launch.json"),
                                        ),
                                )
                                .into_any_element()]
                        } else {
                            run_configs
                                .launch_configs
                                .iter()
                                .enumerate()
                                .map(|(idx, cfg)| {
                                    let cfg_clone = cfg.clone();
                                    let desc = format!(
                                        "{} · {} · {}",
                                        cfg.config_type.as_deref().unwrap_or("lldb"),
                                        cfg.request.as_deref().unwrap_or("launch"),
                                        cfg.program.as_deref().unwrap_or("")
                                    );

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_2()
                                        .p_1p5()
                                        .rounded(px(4.0))
                                        .bg(theme.bg_card)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event, _window, cx| {
                                                this.state.run_configs.selected_launch_index = idx;
                                                let prog = cfg_clone
                                                    .program
                                                    .clone()
                                                    .unwrap_or_default();
                                                this.state.debug.start_session(&cfg_clone.name, &prog);
                                                this.state.run_configs.last_status = Some((
                                                    cfg_clone.name.clone(),
                                                    true,
                                                    "Debug session started.".to_string(),
                                                ));
                                                this.state
                                                    .toast_manager
                                                    .push_success(&format!(
                                                        "Debug session started: {}",
                                                        cfg_clone.name
                                                    ));
                                                cx.notify();
                                            }),
                                        )
                                        .child(icon_12(IconName::Bug, theme.status_red))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .flex_1()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_weight(FontWeight::MEDIUM)
                                                        .text_color(theme.text_primary)
                                                        .child(cfg.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .font_family("Consolas")
                                                        .text_color(theme.text_muted)
                                                        .child(desc),
                                                ),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        },
                    )
                })),
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_strip_jsonc_basic() {
        let raw = r#"{
            // Single line comment
            "label": "build",
            /* Multi-line
               comment */
            "type": "shell",
        }"#;

        let stripped = strip_jsonc(raw);
        assert!(!stripped.contains("// Single line comment"));
        assert!(!stripped.contains("Multi-line"));
        assert!(stripped.contains("\"label\": \"build\""));

        let parsed: serde_json::Value = serde_json::from_str(&stripped).expect("valid json");
        assert_eq!(parsed["label"], "build");
        assert_eq!(parsed["type"], "shell");
    }

    #[test]
    fn test_substitute_vars_workspace_and_file() {
        let ctx = VarContext {
            workspace_folder: Some("C:/projects/myapp".to_string()),
            file: Some("C:/projects/myapp/src/main.rs".to_string()),
            line_number: Some(42),
            selected_text: Some("foo_bar".to_string()),
            exec_path: Some("C:/bin/hades.exe".to_string()),
        };

        assert_eq!(
            substitute_vars("${workspaceFolder}/target", &ctx),
            "C:/projects/myapp/target"
        );
        assert_eq!(substitute_vars("${fileBasename}", &ctx), "main.rs");
        assert_eq!(substitute_vars("${fileBasenameNoExtension}", &ctx), "main");
        assert_eq!(substitute_vars("${fileExtname}", &ctx), ".rs");
        assert_eq!(substitute_vars("${relativeFile}", &ctx), "src/main.rs");
        assert_eq!(substitute_vars("${lineNumber}", &ctx), "42");
        assert_eq!(substitute_vars("${selectedText}", &ctx), "foo_bar");
    }

    #[test]
    fn test_build_task_command_with_spaces() {
        let task = VsCodeTask {
            label: "test".to_string(),
            task_type: Some("shell".to_string()),
            command: Some("cargo".to_string()),
            args: vec![
                "test".to_string(),
                "--test".to_string(),
                "my test suite".to_string(),
            ],
            cwd: None,
            group: None,
            is_default: false,
        };

        let ctx = VarContext::default();
        let cmd = build_task_command(&task, &ctx);
        assert_eq!(cmd, "cargo test --test \"my test suite\"");
    }

    #[test]
    fn test_parse_tasks_and_launch_json() {
        let tasks_raw = r#"{
            "version": "2.0.0",
            "tasks": [
                {
                    "label": "cargo build",
                    "type": "shell",
                    "command": "cargo",
                    "args": ["build", "--release"],
                    "isDefault": true,
                }
            ]
        }"#;

        let tasks = parse_tasks_json(tasks_raw);
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].label, "cargo build");
        assert_eq!(tasks[0].args, vec!["build", "--release"]);
        assert!(tasks[0].is_default);

        let launch_raw = r#"{
            "version": "0.2.0",
            "configurations": [
                {
                    "name": "Debug App",
                    "type": "lldb",
                    "request": "launch",
                    "program": "${workspaceFolder}/target/debug/app",
                    "preLaunchTask": "cargo build"
                }
            ]
        }"#;

        let configs = parse_launch_json(launch_raw);
        assert_eq!(configs.len(), 1);
        assert_eq!(configs[0].name, "Debug App");
        assert_eq!(configs[0].pre_launch_task.as_deref(), Some("cargo build"));
    }
}
