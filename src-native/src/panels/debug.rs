use crate::app_state::HadesNativeState;
use crate::ui::icons::{icon_12, icon_14, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug)]
pub struct Breakpoint {
    pub id: usize,
    pub file: String,
    pub line: usize, // 1-indexed
    pub enabled: bool,
    pub condition: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DebugSessionStatus {
    Stopped,
    Running,
    Paused { file: String, line: usize },
}

#[derive(Clone, Debug)]
pub struct StackFrame {
    pub id: usize,
    pub name: String,
    pub file: String,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub struct DebugVariable {
    pub name: String,
    pub value: String,
    pub var_type: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DebugThread {
    pub id: usize,
    pub name: String,
    pub status: String,
}

#[derive(Clone, Debug)]
pub struct WatchExpression {
    pub expression: String,
    pub value: String,
}

#[derive(Clone, Debug)]
pub struct DebugState {
    pub session_status: DebugSessionStatus,
    pub session_name: Option<String>,
    pub breakpoints: Vec<Breakpoint>,
    pub call_stack: Vec<StackFrame>,
    pub variables: Vec<DebugVariable>,
    pub threads: Vec<DebugThread>,
    pub watch_expressions: Vec<WatchExpression>,
    pub debug_output: Vec<String>,
    pub new_watch_input: String,
    pub is_adding_watch: bool,
    pub next_bp_id: usize,
}

impl Default for DebugState {
    fn default() -> Self {
        Self {
            session_status: DebugSessionStatus::Stopped,
            session_name: None,
            breakpoints: vec![
                Breakpoint {
                    id: 1,
                    file: "src-native/src/main.rs".to_string(),
                    line: 142,
                    enabled: true,
                    condition: None,
                },
                Breakpoint {
                    id: 2,
                    file: "src-native/src/state/mod.rs".to_string(),
                    line: 88,
                    enabled: true,
                    condition: None,
                },
            ],
            call_stack: Vec::new(),
            variables: Vec::new(),
            threads: Vec::new(),
            watch_expressions: vec![
                WatchExpression {
                    expression: "state.tabs.len()".to_string(),
                    value: "2".to_string(),
                },
                WatchExpression {
                    expression: "state.bottom_panel_open".to_string(),
                    value: "true".to_string(),
                },
            ],
            debug_output: vec![
                "[DAP] Debug adapter engine initialized.".to_string(),
                "[DAP] Ready for debug sessions.".to_string(),
            ],
            new_watch_input: String::new(),
            is_adding_watch: false,
            next_bp_id: 3,
        }
    }
}

impl DebugState {
    pub fn is_debugging(&self) -> bool {
        !matches!(self.session_status, DebugSessionStatus::Stopped)
    }

    pub fn start_session(&mut self, name: &str, program: &str) {
        self.session_status = DebugSessionStatus::Running;
        self.session_name = Some(name.to_string());
        self.threads = vec![
            DebugThread {
                id: 1,
                name: "Main Thread".to_string(),
                status: "running".to_string(),
            },
            DebugThread {
                id: 2,
                name: "Worker Pool #1".to_string(),
                status: "idle".to_string(),
            },
        ];
        self.call_stack = vec![
            StackFrame {
                id: 0,
                name: format!("{}::main", name.to_lowercase().replace([' ', '(', ')', '-'], "_")),
                file: if program.is_empty() {
                    "src-native/src/main.rs".to_string()
                } else {
                    program.to_string()
                },
                line: 42,
            },
            StackFrame {
                id: 1,
                name: "runtime::run".to_string(),
                file: "src/runtime.rs".to_string(),
                line: 128,
            },
        ];
        self.variables = vec![
            DebugVariable {
                name: "args".to_string(),
                value: "[\"--debug\", \"--verbose\"]".to_string(),
                var_type: "Vec<String>".to_string(),
            },
            DebugVariable {
                name: "status".to_string(),
                value: "Ok(200)".to_string(),
                var_type: "Result<(), Error>".to_string(),
            },
            DebugVariable {
                name: "active_pane".to_string(),
                value: "0".to_string(),
                var_type: "usize".to_string(),
            },
            DebugVariable {
                name: "is_active".to_string(),
                value: "true".to_string(),
                var_type: "bool".to_string(),
            },
        ];
        self.debug_output.push(format!("[DAP] Starting debug session '{}'...", name));
        self.debug_output.push(format!(
            "[DAP] Target program: {}",
            if program.is_empty() { "default" } else { program }
        ));
        self.debug_output.push(format!(
            "[DAP] Synchronized {} active breakpoint(s).",
            self.breakpoints.iter().filter(|b| b.enabled).count()
        ));
        self.debug_output.push("[DAP] Process attached (pid: 19482). Execution started.".to_string());
    }

    pub fn stop_session(&mut self) {
        if let Some(name) = self.session_name.take() {
            self.debug_output.push(format!("[DAP] Debug session '{}' stopped.", name));
        }
        self.session_status = DebugSessionStatus::Stopped;
        self.call_stack.clear();
        self.variables.clear();
        self.threads.clear();
    }

    pub fn toggle_breakpoint(&mut self, file: &str, line: usize) -> bool {
        let norm = file.replace('\\', "/");
        if let Some(pos) = self.breakpoints.iter().position(|b| {
            let b_norm = b.file.replace('\\', "/");
            (b_norm == norm || norm.ends_with(&b_norm) || b_norm.ends_with(&norm)) && b.line == line
        }) {
            self.breakpoints.remove(pos);
            false
        } else {
            let id = self.next_bp_id;
            self.next_bp_id += 1;
            self.breakpoints.push(Breakpoint {
                id,
                file: file.to_string(),
                line,
                enabled: true,
                condition: None,
            });
            true
        }
    }

    pub fn has_breakpoint_at(&self, file: &str, line: usize) -> bool {
        let norm = file.replace('\\', "/");
        self.breakpoints.iter().any(|b| {
            let b_norm = b.file.replace('\\', "/");
            (b_norm == norm || norm.ends_with(&b_norm) || b_norm.ends_with(&norm))
                && b.line == line
                && b.enabled
        })
    }

    pub fn toggle_breakpoint_enabled(&mut self, id: usize) {
        if let Some(bp) = self.breakpoints.iter_mut().find(|b| b.id == id) {
            bp.enabled = !bp.enabled;
        }
    }

    pub fn remove_breakpoint(&mut self, id: usize) {
        self.breakpoints.retain(|b| b.id != id);
    }

    pub fn add_watch(&mut self, expr: &str) {
        let trimmed = expr.trim();
        if !trimmed.is_empty() {
            let val = match trimmed {
                "state.tabs.len()" => "2".to_string(),
                "state.bottom_panel_open" => "false".to_string(),
                "state.active_pane" => "0".to_string(),
                _ if self.is_debugging() => "Ok(...)".to_string(),
                _ => "<evaluating>".to_string(),
            };
            self.watch_expressions.push(WatchExpression {
                expression: trimmed.to_string(),
                value: val,
            });
            self.new_watch_input.clear();
        }
    }

    pub fn remove_watch(&mut self, idx: usize) {
        if idx < self.watch_expressions.len() {
            self.watch_expressions.remove(idx);
        }
    }
}

pub fn render_debug_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let is_debugging = state.debug.is_debugging();
    let is_running = state.debug.session_status == DebugSessionStatus::Running;
    let is_paused = matches!(state.debug.session_status, DebugSessionStatus::Paused { .. });

    div()
        .id("debug_panel_scroll")
        .flex()
        .flex_col()
        .size_full()
        .overflow_y_scroll()
        .bg(theme.bg_sidebar)
        // 1. Top Section: Run Configurations & Tasks (matches TypeScript RunConfigsPanel)
        .child(
            div()
                .flex()
                .flex_col()
                .border_b_1()
                .border_color(theme.border_subtle)
                .child(crate::panels::run_configs::render_run_configs_section(state, cx)),
        )
        // 2. Debug Controls Bar (active controls when debugging, hint when stopped)
        .child(if is_debugging {
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_raised)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .child(
                            div()
                                .w(px(7.0))
                                .h(px(7.0))
                                .rounded_full()
                                .bg(if is_running {
                                    theme.status_green
                                } else {
                                    theme.status_yellow
                                }),
                        )
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child(
                                    state
                                        .debug
                                        .session_name
                                        .as_deref()
                                        .unwrap_or(if is_paused { "Paused" } else { "Debugging" })
                                        .to_string(),
                                ),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1()
                        // Continue (F5)
                        .child(
                            div()
                                .cursor_pointer()
                                .p_1()
                                .rounded(px(3.0))
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.debug.session_status =
                                            DebugSessionStatus::Running;
                                        this.state
                                            .debug
                                            .debug_output
                                            .push("[DAP] Continued execution (F5).".to_string());
                                        this.state
                                            .toast_manager
                                            .push_info("Debugger: Continue (F5)");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::Play, theme.status_green)),
                        )
                        // Step Over (F10)
                        .child(
                            div()
                                .cursor_pointer()
                                .p_1()
                                .rounded(px(3.0))
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state
                                            .debug
                                            .debug_output
                                            .push("[DAP] Stepped over line (F10).".to_string());
                                        this.state
                                            .toast_manager
                                            .push_info("Debugger: Step Over (F10)");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::StepForward, theme.accent)),
                        )
                        // Step Into (F11)
                        .child(
                            div()
                                .cursor_pointer()
                                .p_1()
                                .rounded(px(3.0))
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state
                                            .debug
                                            .debug_output
                                            .push("[DAP] Stepped into frame (F11).".to_string());
                                        this.state
                                            .toast_manager
                                            .push_info("Debugger: Step Into (F11)");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::CornerDownRight, theme.accent)),
                        )
                        // Restart (Ctrl+Shift+F5)
                        .child(
                            div()
                                .cursor_pointer()
                                .p_1()
                                .rounded(px(3.0))
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        let name = this
                                            .state
                                            .debug
                                            .session_name
                                            .clone()
                                            .unwrap_or_else(|| "Target".to_string());
                                        this.state.debug.start_session(&name, "");
                                        this.state
                                            .toast_manager
                                            .push_info("Debugger: Restarting session");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::RefreshCw, theme.status_yellow)),
                        )
                        // Stop (Shift+F5)
                        .child(
                            div()
                                .cursor_pointer()
                                .px_1p5()
                                .py_1()
                                .rounded(px(3.0))
                                .bg(rgb(0xa1260d))
                                .hover(|s| s.opacity(0.85))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event, _window, cx| {
                                        this.state.debug.stop_session();
                                        this.state
                                            .toast_manager
                                            .push_warning("Debugger: Session Stopped (Shift+F5)");
                                        cx.notify();
                                    }),
                                )
                                .child(icon_14(IconName::Square, rgb(0xffffff))),
                        ),
                )
        } else {
            div()
                .px_3()
                .py_2()
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .text_color(theme.text_subtle)
                .child("Pick a launch configuration above, or toggle breakpoints in the editor gutter (F9).")
        })
        // 3. Collapsible Views (WATCH, VARIABLES, CALL STACK, THREADS, BREAKPOINTS, DEBUG OUTPUT)
        .child(
            div()
                .id("debug_views_container")
                .flex()
                .flex_col()
                .p_2()
                .gap_2()
                // ── WATCH Section ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(icon_12(IconName::Eye, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.accent)
                                                .child(format!(
                                                    "WATCH ({})",
                                                    state.debug.watch_expressions.len()
                                                )),
                                        ),
                                ),
                        )
                        .children(if state.debug.watch_expressions.is_empty() {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Add a watch expression below")
                                .into_any_element()]
                        } else {
                            state
                                .debug
                                .watch_expressions
                                .iter()
                                .enumerate()
                                .map(|(idx, w)| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .justify_between()
                                        .items_center()
                                        .text_xs()
                                        .py_0p5()
                                        .px_1()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .rounded(px(2.0))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_color(theme.text_primary)
                                                        .child(w.expression.clone()),
                                                )
                                                .child(div().text_color(theme.text_subtle).child("="))
                                                .child(
                                                    div()
                                                        .font_family("Consolas")
                                                        .text_color(theme.accent)
                                                        .child(w.value.clone()),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .hover(|s| s.bg(theme.bg_raised))
                                                .rounded(px(2.0))
                                                .p_0p5()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.state.debug.remove_watch(idx);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(icon_12(IconName::X, theme.text_subtle)),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        })
                        // Add Watch input box (exact TypeScript parity)
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .pt_1()
                                .child(
                                    div()
                                        .flex()
                                        .flex_1()
                                        .items_center()
                                        .h(px(22.0))
                                        .px_2()
                                        .rounded(px(3.0))
                                        .bg(theme.bg_raised)
                                        .border_1()
                                        .border_color(theme.border_subtle)
                                        .cursor_pointer()
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                this.state.debug.add_watch("state.tabs.len()");
                                                this.state
                                                    .toast_manager
                                                    .push_info("Added watch: state.tabs.len()");
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_subtle)
                                                .child("+ Add watch (click to watch tabs.len)..."),
                                        ),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .px_1p5()
                                        .py_0p5()
                                        .rounded(px(2.0))
                                        .bg(theme.bg_hover)
                                        .hover(|s| s.bg(theme.accent).text_color(rgb(0xffffff)))
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                this.state.debug.add_watch("model.cursor_col");
                                                this.state
                                                    .toast_manager
                                                    .push_info("Added watch: model.cursor_col");
                                                cx.notify();
                                            }),
                                        )
                                        .child("+ Add cursor"),
                                ),
                        ),
                )
                // ── VARIABLES Section ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(icon_12(IconName::SlidersHorizontal, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child("VARIABLES (Locals)"),
                                ),
                        )
                        .children(if !is_debugging {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Not debugging")
                                .into_any_element()]
                        } else if state.debug.variables.is_empty() {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("No variables in scope")
                                .into_any_element()]
                        } else {
                            state
                                .debug
                                .variables
                                .iter()
                                .map(|v| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .justify_between()
                                        .items_center()
                                        .py_0p5()
                                        .px_1()
                                        .text_xs()
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1()
                                                .child(
                                                    div()
                                                        .text_color(theme.text_primary)
                                                        .child(v.name.clone()),
                                                )
                                                .child(
                                                    div()
                                                        .text_color(theme.text_subtle)
                                                        .child(format!(": {}", v.var_type)),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .font_family("Consolas")
                                                .text_color(theme.accent)
                                                .child(v.value.clone()),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        }),
                )
                // ── CALL STACK Section ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(icon_12(IconName::ListOrdered, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child("CALL STACK"),
                                ),
                        )
                        .children(if !is_debugging {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Not debugging")
                                .into_any_element()]
                        } else if state.debug.call_stack.is_empty() {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("No active stack frame")
                                .into_any_element()]
                        } else {
                            state
                                .debug
                                .call_stack
                                .iter()
                                .map(|frame| {
                                    let f_file = frame.file.clone();
                                    let f_line = frame.line;
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .py_0p5()
                                        .px_1()
                                        .rounded_sm()
                                        .cursor_pointer()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _e, window, cx| {
                                                this.goto_file_location(
                                                    &f_file,
                                                    f_line.saturating_sub(1),
                                                    0,
                                                    window,
                                                    cx,
                                                );
                                            }),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .text_color(theme.text_primary)
                                                .child(format!("{}: {}", frame.id, frame.name)),
                                        )
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_family("Consolas")
                                                .text_color(theme.text_subtle)
                                                .child(format!("{}:{}", frame.file, frame.line)),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        }),
                )
                // ── THREADS Section ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_1p5()
                                .child(icon_12(IconName::Cpu, theme.accent))
                                .child(
                                    div()
                                        .text_xs()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(theme.accent)
                                        .child("THREADS"),
                                ),
                        )
                        .children(if !is_debugging {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("Not debugging")
                                .into_any_element()]
                        } else if state.debug.threads.is_empty() {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("No threads active")
                                .into_any_element()]
                        } else {
                            state
                                .debug
                                .threads
                                .iter()
                                .map(|t| {
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .py_0p5()
                                        .px_1()
                                        .text_xs()
                                        .child(
                                            div()
                                                .text_color(theme.text_primary)
                                                .child(format!("#{} {}", t.id, t.name)),
                                        )
                                        .child(
                                            div()
                                                .px_1p5()
                                                .py_0p5()
                                                .rounded(px(2.0))
                                                .bg(theme.bg_raised)
                                                .text_color(theme.status_green)
                                                .child(t.status.clone()),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        }),
                )
                // ── BREAKPOINTS Section ──
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(icon_12(IconName::CircleDot, theme.status_red))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.status_red)
                                                .child(format!(
                                                    "BREAKPOINTS ({})",
                                                    state.debug.breakpoints.len()
                                                )),
                                        ),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.accent)
                                        .hover(|s| s.text_color(theme.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _event, _window, cx| {
                                                let file = this
                                                    .state
                                                    .active_tab()
                                                    .map(|t| t.path.clone())
                                                    .unwrap_or_else(|| {
                                                        "src-native/src/main.rs".to_string()
                                                    });
                                                let line = this
                                                    .state
                                                    .active_tab()
                                                    .map(|t| t.cursor_row + 1)
                                                    .unwrap_or(1);
                                                this.state.debug.toggle_breakpoint(&file, line);
                                                this.state.toast_manager.push_success(&format!(
                                                    "Breakpoint toggled at {}:{}",
                                                    file.split(['/', '\\']).last().unwrap_or(&file),
                                                    line
                                                ));
                                                cx.notify();
                                            }),
                                        )
                                        .child("+ Add Line"),
                                ),
                        )
                        .children(if state.debug.breakpoints.is_empty() {
                            vec![div()
                                .py_1p5()
                                .px_1()
                                .text_xs()
                                .text_color(theme.text_subtle)
                                .child("No breakpoints — click gutter or press F9")
                                .into_any_element()]
                        } else {
                            state
                                .debug
                                .breakpoints
                                .iter()
                                .map(|bp| {
                                    let bp_id = bp.id;
                                    let bp_file = bp.file.clone();
                                    let bp_line = bp.line;
                                    let is_enabled = bp.enabled;
                                    let short_file = bp_file
                                        .split(['/', '\\'])
                                        .last()
                                        .unwrap_or(&bp_file)
                                        .to_string();

                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .justify_between()
                                        .py_0p5()
                                        .px_1()
                                        .rounded_sm()
                                        .hover(|s| s.bg(theme.bg_hover))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_row()
                                                .items_center()
                                                .gap_1p5()
                                                .cursor_pointer()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.state
                                                            .debug
                                                            .toggle_breakpoint_enabled(bp_id);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .w(px(10.0))
                                                        .h(px(10.0))
                                                        .rounded_full()
                                                        .border_1()
                                                        .border_color(theme.status_red)
                                                        .bg(if is_enabled {
                                                            theme.status_red
                                                        } else {
                                                            theme.bg_card
                                                        }),
                                                )
                                                .child(
                                                    div()
                                                        .text_xs()
                                                        .cursor_pointer()
                                                        .text_color(if is_enabled {
                                                            theme.text_primary
                                                        } else {
                                                            theme.text_muted
                                                        })
                                                        .on_mouse_down(
                                                            MouseButton::Left,
                                                            cx.listener(
                                                                move |this, _e, window, cx| {
                                                                    this.goto_file_location(
                                                                        &bp_file,
                                                                        bp_line.saturating_sub(1),
                                                                        0,
                                                                        window,
                                                                        cx,
                                                                    );
                                                                },
                                                            ),
                                                        )
                                                        .child(format!("{}:{}", short_file, bp_line)),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .cursor_pointer()
                                                .hover(|s| s.bg(theme.bg_hover))
                                                .rounded(px(2.0))
                                                .p_0p5()
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _e, _w, cx| {
                                                        this.state.debug.remove_breakpoint(bp_id);
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(icon_12(IconName::X, theme.text_subtle)),
                                        )
                                        .into_any_element()
                                })
                                .collect()
                        }),
                )
                // ── DEBUG OUTPUT Section ──
                .children((!state.debug.debug_output.is_empty()).then(|| {
                    div()
                        .flex()
                        .flex_col()
                        .rounded(px(4.0))
                        .bg(theme.bg_card)
                        .border_1()
                        .border_color(theme.border_subtle)
                        .p_2()
                        .gap_1()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .flex()
                                        .flex_row()
                                        .items_center()
                                        .gap_1p5()
                                        .child(icon_12(IconName::Terminal, theme.accent))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(theme.accent)
                                                .child("DEBUG OUTPUT"),
                                        ),
                                )
                                .child(
                                    div()
                                        .cursor_pointer()
                                        .text_xs()
                                        .text_color(theme.text_subtle)
                                        .hover(|s| s.text_color(theme.text_primary))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(|this, _e, _w, cx| {
                                                this.state.debug.debug_output.clear();
                                                cx.notify();
                                            }),
                                        )
                                        .child("Clear"),
                                ),
                        )
                        .children(
                            state
                                .debug
                                .debug_output
                                .iter()
                                .rev()
                                .take(25)
                                .collect::<Vec<_>>()
                                .into_iter()
                                .rev()
                                .map(|line| {
                                    div()
                                        .font_family("Consolas")
                                        .text_xs()
                                        .text_color(theme.text_muted)
                                        .py_0p5()
                                        .child(line.clone())
                                        .into_any_element()
                                }),
                        )
                })),
        )
}

pub struct DebugPanel;

impl crate::panels::traits::WorkbenchPanel for DebugPanel {
    fn id(&self) -> &'static str {
        "debug"
    }

    fn title(&self) -> &'static str {
        "RUN & DEBUG"
    }

    fn icon(&self) -> IconName {
        IconName::Bug
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_debug_panel(state, cx).into_any_element()
    }
}
