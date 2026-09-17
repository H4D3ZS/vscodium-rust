#![recursion_limit = "4096"]
#![windows_subsystem = "windows"]

mod app_state;
mod commands;
mod domain;
pub mod editor;
pub mod events;
mod panels;
mod state;
mod terminal_ansi;
mod terminal_pty;
mod theme;
mod ui;

use app_state::HadesNativeState;
use gpui_kit::*;

pub struct HadesAppView {
    pub state: HadesNativeState,
    pub focus_handle: FocusHandle,
    pub editor: Entity<gpui_kit::component::input::EditorState>,
    pub search: panels::SearchState,
    pub scm: panels::ScmState,
    pub diff_viewer: panels::DiffViewerState,
    pub quick_open: ui::QuickOpenState,
    pub input_box: ui::InputBoxState,
    pub registry: panels::PanelRegistry,
    pub commands: commands::CommandRegistry,
    pub _tick_task: Task<()>,
    pub _model_poll_task: Task<()>,
}

impl HadesAppView {
    pub fn new(
        window: &mut Window,
        initial_path: Option<std::path::PathBuf>,
        cx: &mut Context<Self>,
    ) -> Self {
        let state = HadesNativeState::with_initial_path(initial_path);
        let initial_content = state.active_tab().map(|t| t.content()).unwrap_or_default();
        let editor = cx.new(|cx| {
            let mut ed = gpui_kit::component::input::EditorState::new(window, cx).language("rust");
            ed.set_value(initial_content, window, cx);
            ed
        });

        let scm = state.scm.clone();

        let mut registry = panels::PanelRegistry::new();
        registry.register_primary(panels::ExplorerPanel);
        registry.register_primary(panels::AgentChatPanel);
        registry.register_primary(panels::SearchPanel);
        registry.register_primary(panels::ScmPanel);
        registry.register_primary(panels::DebugPanel);
        registry.register_primary(panels::TestExplorerPanel);
        registry.register_primary(panels::ExtensionsPanel);
        registry.register_primary(panels::TerminalPanel);
        registry.register_primary(panels::MobilePanel);
        registry.register_primary(panels::ApexPanel);
        registry.register_primary(panels::PyTorchStudioPanel);
        registry.register_primary(panels::VisionPanel);
        registry.register_primary(panels::SettingsPanel);
        registry.register_primary(panels::McpStorePanel);
        registry.register_primary(panels::OutlinePanel);
        registry.register_primary(panels::KeybindingsPanel);
        registry.register_primary(panels::CanvasesPanel);
        registry.register_primary(panels::BrowserPanel);

        registry.register_bottom(panels::TerminalPanel);
        registry.register_bottom(panels::ProblemsPanel::default());
        registry.register_bottom(panels::OutputPanel::default());
        registry.register_bottom(panels::DebugConsolePanel::default());
        registry.register_bottom(panels::LogcatPanel);
        registry.register_bottom(panels::PortsPanel::default());
        registry.register_bottom(panels::JobsPanel::default());

        registry.register_auxiliary(panels::ComposerTab);
        registry.register_auxiliary(panels::AgentStudioTab);
        registry.register_auxiliary(panels::SpecsPanel);
        registry.register_auxiliary(panels::RulesPanel);
        registry.register_auxiliary(panels::KortexPanel);
        registry.register_auxiliary(panels::McpStorePanel);
        registry.register_auxiliary(panels::OutlinePanel);
        registry.register_auxiliary(panels::KeybindingsPanel);
        registry.register_auxiliary(panels::VectorSearchPanel);

        let commands = commands::CommandRegistry::new();

        let focus_handle = cx.focus_handle();
        window.focus(&focus_handle, cx);

        // UI redraw at ~60fps for snappy cursor blink, streaming, and hover states.
        let _tick_task = cx.spawn_in(window, async move |this, cx| {
            loop {
                cx.background_executor().timer(std::time::Duration::from_millis(16)).await;
                if this.update_in(cx, |_this, _window, cx| {
                    cx.notify();
                }).is_err() {
                    return;
                }
            }
        });

        // Background Lemonade model poll: fires 2s after launch, then every 30s.
        // Keeps the model picker live without blocking startup.
        let _model_poll_task = cx.spawn_in(window, async move |this, cx| {
            // Short delay so the window is fully up before the first HTTP hit.
            cx.background_executor().timer(std::time::Duration::from_secs(2)).await;
            loop {
                let endpoint = this.update_in(cx, |app, _w, _cx| {
                    app.state.inference_health.endpoint.clone()
                }).unwrap_or_else(|_| "http://127.0.0.1:13305".to_string());

                // Mark as probing so the UI shows a spinner.
                let _ = this.update_in(cx, |app, _w, cx| {
                    app.state.inference_health.refresh_probe();
                    cx.notify();
                });

                let result = cx
                    .background_executor()
                    .spawn(async move {
                        crate::ui::InferenceHealthState::fetch_models_blocking(&endpoint)
                    })
                    .await;

                let _ = this.update_in(cx, |app, _w, cx| {
                    match result {
                        Ok((latency_ms, models)) if !models.is_empty() => {
                            app.state.inference_health.apply_fetched_models(latency_ms, models);
                            // Also refresh the model picker list from live Lemonade data.
                            let ids: Vec<String> = app.state.inference_health.models
                                .iter()
                                .map(|m| m.id.clone())
                                .collect();
                            // Prepend live local models, keep cloud models at the bottom.
                            let cloud_models: Vec<String> = app.state.available_ai_models
                                .iter()
                                .filter(|m| {
                                    m.contains("Claude") || m.contains("Gemini")
                                        || m.contains("GPT") || m.contains("DeepSeek")
                                        || m.contains("ModelScope")
                                })
                                .cloned()
                                .collect();
                            let mut merged = ids;
                            merged.extend(cloud_models);
                            merged.dedup();
                            app.state.available_ai_models = merged;
                        }
                        Ok((latency_ms, _empty)) => {
                            // Connected but no models yet — update latency only.
                            app.state.inference_health.latency_ms = Some(latency_ms);
                            app.state.inference_health.probing = false;
                            app.state.inference_health.status =
                                crate::ui::InferenceStatus::Connected;
                        }
                        Err(_) => {
                            app.state.inference_health.status =
                                crate::ui::InferenceStatus::Disconnected;
                            app.state.inference_health.probing = false;
                        }
                    }
                    cx.notify();
                });

                // Wait 30s before the next poll.
                cx.background_executor().timer(std::time::Duration::from_secs(30)).await;
            }
        });

        Self {
            state,
            focus_handle,
            editor,
            search: panels::SearchState::default(),
            scm,
            diff_viewer: panels::DiffViewerState::default(),
            quick_open: ui::QuickOpenState::default(),
            input_box: ui::InputBoxState::default(),
            registry,
            commands,
            _tick_task,
            _model_poll_task,
        }
    }

    pub fn open_file(&mut self, path: &str, window: &mut Window, cx: &mut Context<Self>) {
        self.state.open_file(path);
        if let Some(tab) = self.state.active_tab() {
            let content = tab.content();
            self.editor.update(cx, |ed, cx| {
                ed.set_value(content, window, cx);
            });
        }
    }

    pub fn open_workspace(
        &mut self,
        path: std::path::PathBuf,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.state.open_workspace(path);
        self.editor.update(cx, |ed, cx| {
            ed.set_value("", window, cx);
        });
        cx.notify();
    }

    pub fn prompt_open_folder(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let current_dir = if self.state.has_workspace && self.state.workspace_root.is_dir() {
            Some(self.state.workspace_root.clone())
        } else {
            dirs::home_dir()
        };
        cx.spawn_in(window, async move |this, cx| {
            let folder = cx
                .background_executor()
                .spawn(async move {
                    let mut picker = rfd::FileDialog::new().set_title("Open Workspace Folder");
                    if let Some(ref dir) = current_dir {
                        picker = picker.set_directory(dir);
                    }
                    picker.pick_folder()
                })
                .await;

            if let Some(folder) = folder {
                let _ = this.update_in(cx, |this, window, cx| {
                    this.open_workspace(folder, window, cx);
                });
            }
        })
        .detach();
    }

    pub fn prompt_open_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let current_dir = if self.state.has_workspace && self.state.workspace_root.is_dir() {
            Some(self.state.workspace_root.clone())
        } else {
            dirs::home_dir()
        };
        cx.spawn_in(window, async move |this, cx| {
            let file = cx
                .background_executor()
                .spawn(async move {
                    let mut picker = rfd::FileDialog::new().set_title("Open File");
                    if let Some(ref dir) = current_dir {
                        picker = picker.set_directory(dir);
                    }
                    picker.pick_file()
                })
                .await;

            if let Some(file) = file {
                let _ = this.update_in(cx, |this, window, cx| {
                    let p_str = file.to_string_lossy().to_string();
                    this.open_file(&p_str, window, cx);
                    cx.notify();
                });
            }
        })
        .detach();
    }

    pub fn prompt_save_file_as(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let (content, current_dir, file_name) = if let Some(tab) = self.state.active_tab() {
            (
                tab.content().to_string(),
                std::path::Path::new(&tab.path).parent().map(|p| p.to_path_buf()),
                tab.title.clone(),
            )
        } else {
            return;
        };

        cx.spawn_in(window, async move |this, cx| {
            let save_path = cx
                .background_executor()
                .spawn(async move {
                    let mut picker = rfd::FileDialog::new()
                        .set_title("Save File As")
                        .set_file_name(&file_name);
                    if let Some(ref dir) = current_dir {
                        picker = picker.set_directory(dir);
                    }
                    picker.save_file()
                })
                .await;

            if let Some(save_path) = save_path {
                if std::fs::write(&save_path, &content).is_ok() {
                    let _ = this.update_in(cx, |this, window, cx| {
                        let p_str = save_path.to_string_lossy().to_string();
                        this.open_file(&p_str, window, cx);
                        this.state
                            .toast_manager
                            .push_info(&format!("Saved as: {}", save_path.display()));
                        cx.notify();
                    });
                }
            }
        })
        .detach();
    }

    pub fn goto_file_location(
        &mut self,
        path: &str,
        row: usize,
        col: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.open_file(path, window, cx);
        if let Some(tab) = self.state.active_tab_mut() {
            tab.cursor_row = row;
            tab.cursor_col = col;
            tab.model.set_cursor(row, col);
            tab.scroll_row = row.saturating_sub(10);
            self.state.status_message = format!("Jumped to {}:{}:{}", path, row + 1, col + 1);
        }
        self.state.focused_panel = crate::app_state::FocusedPanel::Editor;
        cx.notify();
    }

    pub fn select_tab(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.state.select_tab(idx);
        if let Some(tab) = self.state.active_tab() {
            let content = tab.content();
            self.editor.update(cx, |ed, cx| {
                ed.set_value(content, window, cx);
            });
        }
    }

    pub fn close_tab(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.state.close_tab(idx);
        if let Some(tab) = self.state.active_tab() {
            let content = tab.content();
            self.editor.update(cx, |ed, cx| {
                ed.set_value(content, window, cx);
            });
        } else {
            self.editor.update(cx, |ed, cx| {
                ed.set_value("", window, cx);
            });
        }
    }

    pub fn execute_input_box(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let trimmed = self.input_box.value.trim().to_string();
        if trimmed.is_empty() {
            self.input_box.close();
            return;
        }

        match self.input_box.mode {
            ui::InputBoxMode::NewFile => {
                let base = self
                    .input_box
                    .target_dir
                    .clone()
                    .unwrap_or_else(|| self.state.workspace_root.clone());
                let resolved = base.join(&trimmed);

                if resolved.exists() {
                    self.input_box.error = Some("File or folder already exists".to_string());
                    return;
                }

                if let Some(parent) = resolved.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }

                if std::fs::write(&resolved, "").is_ok() {
                    self.state.refresh_file_tree();
                    let path_str = resolved.to_string_lossy().to_string();
                    self.open_file(&path_str, window, cx);
                    self.state
                        .toast_manager
                        .push_info(&format!("Created file: {trimmed}"));
                } else {
                    self.state
                        .toast_manager
                        .push_error(&format!("Failed to create file: {trimmed}"));
                }
                self.input_box.close();
            }
            ui::InputBoxMode::NewFolder => {
                let base = self
                    .input_box
                    .target_dir
                    .clone()
                    .unwrap_or_else(|| self.state.workspace_root.clone());
                let resolved = base.join(&trimmed);

                if resolved.exists() {
                    self.input_box.error = Some("Folder already exists".to_string());
                    return;
                }

                if std::fs::create_dir_all(&resolved).is_ok() {
                    self.state.refresh_file_tree();
                    self.state
                        .toast_manager
                        .push_info(&format!("Created folder: {trimmed}"));
                } else {
                    self.state
                        .toast_manager
                        .push_error(&format!("Failed to create folder: {trimmed}"));
                }
                self.input_box.close();
            }
            ui::InputBoxMode::Rename => {
                if let Some(target) = self.input_box.target_path.clone() {
                    let parent = target.parent().unwrap_or(&self.state.workspace_root);
                    let new_path = parent.join(&trimmed);

                    if new_path != target && new_path.exists() {
                        self.input_box.error = Some("Target already exists".to_string());
                        return;
                    }

                    if std::fs::rename(&target, &new_path).is_ok() {
                        let old_str = target.to_string_lossy().to_string();
                        let new_str = new_path.to_string_lossy().to_string();

                        for tab in &mut self.state.tabs {
                            if tab.path == old_str {
                                tab.path = new_str.clone();
                                tab.title = trimmed.to_string();
                            }
                        }
                        for tab in &mut self.state.secondary_tabs {
                            if tab.path == old_str {
                                tab.path = new_str.clone();
                                tab.title = trimmed.to_string();
                            }
                        }
                        self.state.refresh_file_tree();
                        self.state.save_session();
                        self.state
                            .toast_manager
                            .push_info(&format!("Renamed to: {trimmed}"));
                    } else {
                        self.state
                            .toast_manager
                            .push_error(&format!("Failed to rename `{trimmed}`"));
                    }
                }
                self.input_box.close();
            }
            ui::InputBoxMode::Custom => {
                self.input_box.close();
            }
        }
    }

    pub fn select_secondary_tab(
        &mut self,
        idx: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.state.select_secondary_tab(idx);
        if let Some(tab) = self.state.active_tab() {
            let content = tab.content();
            self.editor.update(cx, |ed, cx| {
                ed.set_value(content, window, cx);
            });
        }
        cx.notify();
    }

    pub fn close_secondary_tab(&mut self, idx: usize, window: &mut Window, cx: &mut Context<Self>) {
        self.state.close_secondary_tab(idx);
        if let Some(tab) = self.state.active_tab() {
            let content = tab.content();
            self.editor.update(cx, |ed, cx| {
                ed.set_value(content, window, cx);
            });
        } else {
            self.editor.update(cx, |ed, cx| {
                ed.set_value("", window, cx);
            });
        }
        cx.notify();
    }

    pub fn save_active_file(&mut self, cx: &mut Context<Self>) {
        if self.state.format_on_save {
            self.state.format_active_tab();
        }
        if let Some(tab) = self.state.active_tab_mut() {
            let title = tab.title.clone();
            match tab.save_to_disk() {
                Ok(_) => {
                    self.state
                        .toast_manager
                        .push_success(&format!("Saved {}", title));
                }
                Err(e) => {
                    self.state
                        .toast_manager
                        .push_error(&format!("Failed to save {}: {}", title, e));
                }
            }
            cx.notify();
        }
    }

    pub fn save_current_file(&mut self, _window: &mut Window, cx: &mut Context<Self>) {
        self.save_active_file(cx);
    }

    pub fn execute_terminal_command(&mut self, cmd: &str) {
        self.state.execute_terminal_command(cmd);
    }
}

impl Render for HadesAppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Poll live background PTY stdout, AI streaming turns and logcat
        self.state.sync_terminal_lines();
        self.state.poll_ai_turn();
        self.state.logcat.sync();
        // Garbage-collect expired toast notifications
        self.state.toast_manager.gc();

        div()
            .size_full()
            .track_focus(&self.focus_handle)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, _event, window, cx| {
                    if !this.focus_handle.is_focused(window) {
                        window.focus(&this.focus_handle, cx);
                    }
                }),
            )
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, window, cx| {
                this.handle_key_event(event, window, cx);
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, window, cx| {
                let mut changed = false;
                let pos_x: f32 = event.position.x.into();
                let pos_y: f32 = event.position.y.into();
                let window_w: f32 = window.viewport_size().width.into();
                let window_h: f32 = window.viewport_size().height.into();

                if this.state.is_resizing_sidebar {
                    // Activity bar width is 48.0
                    let new_w = (pos_x - 48.0).clamp(160.0, 700.0);
                    if (this.state.sidebar_width - new_w).abs() > 0.5 {
                        this.state.sidebar_width = new_w;
                        changed = true;
                    }
                }
                if this.state.is_resizing_right_sidebar {
                    let preview_extra = if this.state.iphone_preview_open {
                        this.state.iphone_preview_width + 5.0
                    } else {
                        0.0
                    };
                    let new_w = (window_w - pos_x - preview_extra).clamp(240.0, 800.0);
                    if (this.state.right_sidebar_width - new_w).abs() > 0.5 {
                        this.state.right_sidebar_width = new_w;
                        changed = true;
                    }
                }
                if this.state.is_resizing_iphone_preview {
                    let new_w = (window_w - pos_x).clamp(280.0, 700.0);
                    if (this.state.iphone_preview_width - new_w).abs() > 0.5 {
                        this.state.iphone_preview_width = new_w;
                        changed = true;
                    }
                }
                if this.state.is_resizing_bottom_panel {
                    // Status bar is 24.0 at the bottom
                    let new_h =
                        (window_h - 24.0 - pos_y).clamp(100.0, (window_h * 0.75).max(300.0));
                    if (this.state.bottom_panel_height - new_h).abs() > 0.5 {
                        this.state.bottom_panel_height = new_h;
                        changed = true;
                    }
                }
                if this.state.markdown_preview.is_resizing {
                    let editor_start_x = 48.0
                        + if this.state.left_sidebar_open {
                            this.state.sidebar_width
                        } else {
                            0.0
                        };
                    let editor_end_x = window_w
                        - (if this.state.right_sidebar_open {
                            this.state.right_sidebar_width
                        } else {
                            0.0
                        })
                        - (if this.state.iphone_preview_open {
                            this.state.iphone_preview_width
                        } else {
                            0.0
                        });
                    let editor_w = (editor_end_x - editor_start_x).max(200.0);
                    let preview_w = (editor_end_x - pos_x).clamp(160.0, editor_w - 160.0);
                    let new_pct = (preview_w / editor_w) * 100.0;
                    if (this.state.markdown_preview.width_pct - new_pct).abs() > 0.5 {
                        this.state.markdown_preview.width_pct = new_pct.clamp(20.0, 75.0);
                        changed = true;
                    }
                }
                if changed {
                    cx.notify();
                }
            }))
            .on_mouse_up(
                MouseButton::Left,
                cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                    if this.state.is_resizing_sidebar
                        || this.state.is_resizing_right_sidebar
                        || this.state.is_resizing_iphone_preview
                        || this.state.is_resizing_bottom_panel
                        || this.state.markdown_preview.is_resizing
                    {
                        this.state.is_resizing_sidebar = false;
                        this.state.is_resizing_right_sidebar = false;
                        this.state.is_resizing_iphone_preview = false;
                        this.state.is_resizing_bottom_panel = false;
                        this.state.markdown_preview.is_resizing = false;
                        cx.notify();
                    }
                }),
            )
            .child(ui::render_workspace(
                &self.state,
                &self.editor,
                &self.search,
                &self.scm,
                &self.diff_viewer,
                &self.quick_open,
                &self.input_box,
                &self.registry,
                cx,
            ))
    }
}

impl HadesAppView {
    fn handle_key_event(
        &mut self,
        event: &KeyDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = event.keystroke.key.as_str();
        let ctrl = event.keystroke.modifiers.control;
        let shift = event.keystroke.modifiers.shift;
        let alt = event.keystroke.modifiers.alt;

        // F1: Universal Command Palette (VS Code / Cursor parity)
        if key == "f1" {
            self.quick_open.open_commands(&self.state, &self.commands);
            cx.notify();
            return;
        }

        // Shift+F5: Stop Debugging
        if key == "f5" && shift && !alt {
            if self.state.debug.is_debugging() {
                self.state.debug.stop_session();
                self.state
                    .toast_manager
                    .push_warning("Debugger: Session Stopped (Shift+F5)");
                cx.notify();
                return;
            }
        }

        // F5: Start Debugging / Continue
        if key == "f5" && !alt && !shift {
            self.state.set_activity(app_state::ActivityTab::Debug);
            self.state.left_sidebar_open = true;
            if self.state.debug.is_debugging() {
                self.state.debug.session_status = crate::panels::debug::DebugSessionStatus::Running;
                self.state.debug.debug_output.push("[DAP] Continued execution (F5).".to_string());
                self.state
                    .toast_manager
                    .push_info("Debugger: Continue (F5)");
            } else {
                let cfg = self.state.run_configs.selected_launch_config().cloned();
                let (name, prog) = if let Some(c) = cfg {
                    (c.name, c.program.unwrap_or_default())
                } else {
                    ("Default Debug Target".to_string(), "src-native/src/main.rs".to_string())
                };
                self.state.debug.start_session(&name, &prog);
                self.state.run_configs.last_status =
                    Some((name.clone(), true, "Debug session started.".to_string()));
                self.state
                    .toast_manager
                    .push_success(&format!("Started debug session: {}", name));
            }
            cx.notify();
            return;
        }

        // F9: Toggle Breakpoint on active line
        if key == "f9" {
            if let Some(tab) = self.state.active_tab() {
                let file = tab.path.clone();
                let line = tab.cursor_row + 1;
                let added = self.state.debug.toggle_breakpoint(&file, line);
                let short_file = file.split(['/', '\\']).last().unwrap_or(&file);
                let msg = if added {
                    format!("Breakpoint added at {}:{}", short_file, line)
                } else {
                    format!("Breakpoint removed at {}:{}", short_file, line)
                };
                self.state.toast_manager.push_info(&msg);
                cx.notify();
                return;
            }
        }

        // F10: Step Over
        if key == "f10" {
            if self.state.debug.is_debugging() {
                self.state.debug.debug_output.push("[DAP] Stepped over line (F10).".to_string());
                self.state.toast_manager.push_info("Debugger: Step Over (F10)");
                cx.notify();
                return;
            }
        }

        // F11: Step Into (when not shift)
        if key == "f11" && !shift {
            if self.state.debug.is_debugging() {
                self.state.debug.debug_output.push("[DAP] Stepped into frame (F11).".to_string());
                self.state.toast_manager.push_info("Debugger: Step Into (F11)");
                cx.notify();
                return;
            }
        }

        // Alt+Z: Toggle Word Wrap
        if alt && key == "z" {
            if let Some(tab) = self.state.active_tab_mut() {
                let enabled = tab.model.toggle_word_wrap();
                let msg = if enabled {
                    "Word Wrap: On (100 cols)"
                } else {
                    "Word Wrap: Off"
                };
                self.state.toast_manager.push_info(msg);
                cx.notify();
                return;
            }
        }

        // Monaco Diff Viewer Hunk Navigation: Alt+F5 (Next Hunk), Shift+Alt+F5 (Prev Hunk)
        if self.diff_viewer.is_open && alt && key == "f5" {
            if shift {
                self.diff_viewer.prev_hunk();
            } else {
                self.diff_viewer.next_hunk();
            }
            cx.notify();
            return;
        }

        // Diff Viewer escape handling
        if self.diff_viewer.is_open && key == "escape" {
            self.diff_viewer.close();
            cx.notify();
            return;
        }

        // Context menu escape handling
        if self.state.context_menu.is_open && key == "escape" {
            self.state.context_menu.close();
            cx.notify();
            return;
        }

        // Rename Symbol modal handling when open
        if self.state.rename.is_open {
            match key {
                "escape" => {
                    self.state.rename.close();
                    cx.notify();
                    return;
                }
                "enter" => {
                    self.state.commit_rename();
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.state.rename.backspace();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl && !alt => {
                    if let Some(ch) = c.chars().next() {
                        self.state.rename.insert_char(ch);
                        cx.notify();
                    }
                    return;
                }
                _ => return,
            }
        }

        // Code Action menu handling when open
        if self.state.code_actions.is_open {
            match key {
                "escape" => {
                    self.state.code_actions.close();
                    cx.notify();
                    return;
                }
                "up" => {
                    self.state.code_actions.select_prev();
                    cx.notify();
                    return;
                }
                "down" => {
                    self.state.code_actions.select_next();
                    cx.notify();
                    return;
                }
                "enter" => {
                    self.state.apply_selected_code_action();
                    cx.notify();
                    return;
                }
                _ => return,
            }
        }

        // References Peek overlay handling when open
        if self.state.references.is_open {
            match key {
                "escape" => {
                    self.state.references.close();
                    cx.notify();
                    return;
                }
                "up" => {
                    self.state.references.select_prev();
                    cx.notify();
                    return;
                }
                "down" => {
                    self.state.references.select_next();
                    cx.notify();
                    return;
                }
                "enter" => {
                    if let Some(loc) = self.state.references.selected_reference().cloned() {
                        let path = loc.file_path.to_string_lossy().to_string();
                        let row = loc.row;
                        let col = loc.col;
                        self.open_file(&path, window, cx);
                        if let Some(tab) = self.state.active_tab_mut() {
                            tab.cursor_row = row;
                            tab.cursor_col = col;
                            tab.scroll_row = row.saturating_sub(5);
                        }
                        self.state.references.close();
                        cx.notify();
                    }
                    return;
                }
                _ => return,
            }
        }

        // Inline Edit overlay handling when open
        if self.state.inline_edit.is_open {
            match key {
                "escape" => {
                    self.state.inline_edit.close();
                    cx.notify();
                    return;
                }
                "enter" => {
                    if !self.state.inline_edit.prompt.trim().is_empty() {
                        let prompt = self.state.inline_edit.prompt.clone();
                        let target = self.state.inline_edit.target_file.clone();
                        let line = self.state.inline_edit.line_number;
                        let msg = format!("In file {}, at line {}: {}", target, line, prompt);
                        self.state.send_ai_prompt(&msg);
                        self.state.toast_manager.push_info("AI Edit dispatched");
                        self.state.inline_edit.close();
                        cx.notify();
                    }
                    return;
                }
                "backspace" => {
                    self.state.inline_edit.prompt.pop();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.state.inline_edit.prompt.push_str(c);
                    cx.notify();
                    return;
                }
                _ => return,
            }
        }

        // VS Code-Style Inline File/Folder Creation in Explorer
        if let Some(mut create) = self.state.explorer_create.clone() {
            match key {
                "escape" => {
                    self.state.explorer_create = None;
                    cx.notify();
                    return;
                }
                "enter" => {
                    let trimmed = create.name.trim().to_string();
                    if !trimmed.is_empty() {
                        let base = create
                            .parent_dir
                            .clone()
                            .unwrap_or_else(|| self.state.workspace_root.clone());
                        let target = base.join(&trimmed);
                        if create.is_dir {
                            if let Err(e) = std::fs::create_dir_all(&target) {
                                self.state.toast_manager.push_error(&format!("Failed to create folder: {e}"));
                            } else {
                                self.state.refresh_file_tree();
                                self.state.toast_manager.push_success(&format!("Created folder: {trimmed}"));
                            }
                        } else {
                            if let Some(parent) = target.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            if let Err(e) = std::fs::write(&target, "") {
                                self.state.toast_manager.push_error(&format!("Failed to create file: {e}"));
                            } else {
                                self.state.refresh_file_tree();
                                let target_str = target.to_string_lossy().to_string();
                                self.open_file(&target_str, window, cx);
                                self.state.focused_panel = app_state::FocusedPanel::Editor;
                                self.state.toast_manager.push_success(&format!("Created file: {trimmed}"));
                            }
                        }
                    }
                    self.state.explorer_create = None;
                    cx.notify();
                    return;
                }
                "backspace" => {
                    create.name.pop();
                    self.state.explorer_create = Some(create);
                    cx.notify();
                    return;
                }
                "space" => {
                    create.name.push(' ');
                    self.state.explorer_create = Some(create);
                    cx.notify();
                    return;
                }
                "v" if ctrl => {
                    if let Some(item) = cx.read_from_clipboard() {
                        if let Some(text) = item.text() {
                            let single = text.lines().next().unwrap_or("").trim();
                            create.name.push_str(single);
                            self.state.explorer_create = Some(create);
                            cx.notify();
                        }
                    }
                    return;
                }
                c if c.chars().count() == 1 && !ctrl && !alt => {
                    create.name.push_str(c);
                    self.state.explorer_create = Some(create);
                    cx.notify();
                    return;
                }
                _ => return,
            }
        }

        // Interactive Input Box handling when open
        if self.input_box.is_open {
            match key {
                "escape" => {
                    self.input_box.close();
                    cx.notify();
                    return;
                }
                "enter" => {
                    self.execute_input_box(window, cx);
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.input_box.value.pop();
                    self.input_box.validate();
                    cx.notify();
                    return;
                }
                "space" => {
                    self.input_box.value.push(' ');
                    self.input_box.validate();
                    cx.notify();
                    return;
                }
                "v" if ctrl => {
                    if let Some(item) = cx.read_from_clipboard() {
                        if let Some(text) = item.text() {
                            let single = text.lines().next().unwrap_or("").trim();
                            self.input_box.value.push_str(single);
                            self.input_box.validate();
                            cx.notify();
                        }
                    }
                    return;
                }
                c if c.chars().count() == 1 && !ctrl && !alt => {
                    self.input_box.value.push_str(c);
                    self.input_box.validate();
                    cx.notify();
                    return;
                }
                _ => return,
            }
        }

        // Quick Open Modal handling when open
        if self.quick_open.is_open {
            match key {
                "escape" => {
                    self.quick_open.close();
                    cx.notify();
                    return;
                }
                "up" => {
                    if self.quick_open.selected_idx > 0 {
                        self.quick_open.selected_idx -= 1;
                        cx.notify();
                    }
                    return;
                }
                "down" => {
                    let max_len = if self.quick_open.is_command_mode() {
                        self.quick_open.filtered_commands.len()
                    } else if self.quick_open.is_symbol_mode() {
                        self.quick_open.filtered_symbols.len()
                    } else if self.quick_open.is_workspace_symbol_mode() {
                        self.quick_open.filtered_workspace_symbols.len()
                    } else {
                        self.quick_open.filtered_files.len()
                    };
                    if max_len > 0 && self.quick_open.selected_idx + 1 < max_len {
                        self.quick_open.selected_idx += 1;
                        cx.notify();
                    }
                    return;
                }
                "enter" => {
                    if self.quick_open.is_command_mode() {
                        if let Some(cmd) = self
                            .quick_open
                            .filtered_commands
                            .get(self.quick_open.selected_idx)
                        {
                            let cmd_id = cmd.id;
                            self.quick_open.close();
                            ui::quick_open::execute_command(self, cmd_id, window, cx);
                            cx.notify();
                            return;
                        }
                    } else if self.quick_open.is_symbol_mode() {
                        if let Some(sym) = self
                            .quick_open
                            .filtered_symbols
                            .get(self.quick_open.selected_idx)
                        {
                            let line_num = sym.range.start.line;
                            if let Some(tab) = self.state.active_tab_mut() {
                                tab.cursor_row = line_num;
                                tab.cursor_col = 0;
                                tab.scroll_row = line_num.saturating_sub(5);
                            }
                            self.quick_open.close();
                            cx.notify();
                            return;
                        }
                    } else if self.quick_open.is_workspace_symbol_mode() {
                        if let Some(sym) = self
                            .quick_open
                            .filtered_workspace_symbols
                            .get(self.quick_open.selected_idx)
                            .cloned()
                        {
                            let line_num = sym.line;
                            let file_path = sym.file_path.clone();
                            self.open_file(&file_path, window, cx);
                            if let Some(tab) = self.state.active_tab_mut() {
                                tab.cursor_row = line_num;
                                tab.cursor_col = 0;
                                tab.scroll_row = line_num.saturating_sub(5);
                            }
                            self.quick_open.close();
                            cx.notify();
                            return;
                        }
                    } else if self.quick_open.is_line_mode() {
                        let line_query = self.quick_open.query.trim_start_matches(':').trim();
                        let parts: Vec<&str> = line_query.split(':').collect();
                        if let Some(target_line) =
                            parts.first().and_then(|s| s.parse::<usize>().ok())
                        {
                            let target_col = parts
                                .get(1)
                                .and_then(|s| s.parse::<usize>().ok())
                                .unwrap_or(1);
                            if let Some(tab) = self.state.active_tab_mut() {
                                let max_line = tab.lines.len().max(1);
                                let row = target_line.clamp(1, max_line).saturating_sub(1);
                                let col = target_col.saturating_sub(1);
                                tab.cursor_row = row;
                                tab.cursor_col = col;
                                tab.model.set_cursor(row, col);
                                tab.scroll_row = row.saturating_sub(10);
                                self.state.status_message =
                                    format!("Jumped to line {}, column {}", row + 1, col + 1);
                            }
                        }
                        self.quick_open.close();
                        cx.notify();
                        return;
                    } else if let Some(path) = self
                        .quick_open
                        .filtered_files
                        .get(self.quick_open.selected_idx)
                        .cloned()
                    {
                        self.open_file(&path, window, cx);
                        self.quick_open.close();
                        cx.notify();
                        return;
                    }
                    return;
                }
                "backspace" => {
                    self.quick_open.query.pop();
                    self.quick_open.refresh_list(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.quick_open.query.push_str(c);
                    self.quick_open.refresh_list(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                _ => return,
            }
        }

        // Multi-Cursor Column Selection / Caret Above & Below: Ctrl+Alt+Up / Ctrl+Alt+Down
        if ctrl && alt {
            match key {
                "up" => {
                    self.state.add_cursor_above();
                    cx.notify();
                    return;
                }
                "down" => {
                    self.state.add_cursor_below();
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // Global shortcuts
        if ctrl {
            match key {
                "=" | "+" => {
                    self.state.editor_font_size = (self.state.editor_font_size + 1).min(36);
                    self.state
                        .toast_manager
                        .push_info(&format!("Zoom In (Font: {}px)", self.state.editor_font_size));
                    cx.notify();
                    return;
                }
                "-" | "_" => {
                    self.state.editor_font_size = (self.state.editor_font_size.saturating_sub(1)).max(9);
                    self.state
                        .toast_manager
                        .push_info(&format!("Zoom Out (Font: {}px)", self.state.editor_font_size));
                    cx.notify();
                    return;
                }
                "0" => {
                    self.state.editor_font_size = 14;
                    self.state
                        .toast_manager
                        .push_info("Reset Zoom (Font: 14px)");
                    cx.notify();
                    return;
                }
                "b" if shift => {
                    self.state.set_activity(app_state::ActivityTab::Debug);
                    self.state.left_sidebar_open = true;
                    if let Some(task) = self.state.run_configs.tasks.first().cloned() {
                        let ctx = crate::panels::run_configs::VarContext {
                            workspace_folder: Some(
                                self.state.workspace_root.to_string_lossy().to_string(),
                            ),
                            file: self.state.active_tab().map(|t| t.path.clone()),
                            line_number: self.state.active_tab().map(|t| t.cursor_row + 1),
                            selected_text: None,
                            exec_path: std::env::current_exe()
                                .ok()
                                .map(|p| p.to_string_lossy().to_string()),
                        };
                        let cmd = crate::panels::run_configs::build_task_command(&task, &ctx);
                        self.execute_terminal_command(&cmd);
                        self.state.bottom_panel_open = true;
                        self.state
                            .toast_manager
                            .push_info(&format!("Running task: {}", task.label));
                    } else {
                        self.state
                            .toast_manager
                            .push_info("Open Run & Debug to configure tasks");
                    }
                    cx.notify();
                    return;
                }
                "m" if shift => {
                    self.state
                        .set_bottom_tab(crate::domain::layout::BottomPanelTab::Problems);
                    self.state.bottom_panel_open = true;
                    cx.notify();
                    return;
                }
                "u" if shift => {
                    self.state.active_activity =
                        crate::domain::layout::ActivityTab::ExternalBrowser;
                    self.state.left_sidebar_open = true;
                    cx.notify();
                    return;
                }
                "y" if shift => {
                    self.state
                        .set_bottom_tab(crate::domain::layout::BottomPanelTab::DebugConsole);
                    self.state.bottom_panel_open = true;
                    cx.notify();
                    return;
                }
                "," => {
                    self.state.set_activity(app_state::ActivityTab::Settings);
                    self.state.left_sidebar_open = true;
                    cx.notify();
                    return;
                }
                "i" => {
                    self.state.focused_panel = app_state::FocusedPanel::Composer;
                    self.state.right_sidebar_open = true;
                    cx.notify();
                    return;
                }
                "p" if shift => {
                    self.quick_open.open_commands(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                "p" => {
                    self.quick_open.open(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                "o" if shift => {
                    self.quick_open.open_symbols(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                "e" if shift => {
                    self.state.set_activity(app_state::ActivityTab::Explorer);
                    cx.notify();
                    return;
                }
                "f" if shift => {
                    self.state.set_activity(app_state::ActivityTab::Search);
                    self.search.focused_field = panels::search::SearchFocusedField::Query;
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                "f" => {
                    self.state.toggle_find_bar();
                    cx.notify();
                    return;
                }
                "g" if shift => {
                    self.state
                        .set_activity(app_state::ActivityTab::SourceControl);
                    cx.notify();
                    return;
                }
                "g" if !shift => {
                    let cmds = self.commands.clone();
                    self.quick_open.open_line(&self.state, &cmds);
                    cx.notify();
                    return;
                }
                "d" if shift => {
                    self.state.set_activity(app_state::ActivityTab::Debug);
                    cx.notify();
                    return;
                }
                "x" if shift => {
                    self.state.set_activity(app_state::ActivityTab::Extensions);
                    cx.notify();
                    return;
                }
                "v" if shift => {
                    self.state.toggle_markdown_preview();
                    cx.notify();
                    return;
                }
                "w" => {
                    if self.state.is_split_editor && self.state.active_pane == 1 {
                        if let Some(idx) = self.state.active_secondary_tab_idx {
                            self.close_secondary_tab(idx, window, cx);
                            return;
                        }
                    }
                    if let Some(idx) = self.state.active_tab_idx {
                        self.close_tab(idx, window, cx);
                    }
                    cx.notify();
                    return;
                }
                "\\" => {
                    self.state.split_editor();
                    cx.notify();
                    return;
                }
                "1" => {
                    self.state.focus_pane(0);
                    cx.notify();
                    return;
                }
                "2" => {
                    if self.state.is_split_editor {
                        self.state.focus_pane(1);
                        cx.notify();
                        return;
                    }
                }
                "n" => {
                    self.state.set_activity(app_state::ActivityTab::Explorer);
                    self.state.left_sidebar_open = true;
                    self.state.explorer_create = Some(crate::app_state::ExplorerCreateState {
                        is_dir: false,
                        name: String::new(),
                        parent_dir: None,
                    });
                    self.state.focused_panel = app_state::FocusedPanel::Explorer;
                    window.focus(&self.focus_handle, cx);
                    cx.notify();
                    return;
                }
                "t" => {
                    self.quick_open
                        .open_workspace_symbols(&self.state, &self.commands);
                    cx.notify();
                    return;
                }
                "o" => {
                    if shift {
                        self.prompt_open_folder(window, cx);
                    } else {
                        self.prompt_open_file(window, cx);
                    }
                    cx.notify();
                    return;
                }
                "`" | "~" => {
                    self.state.toggle_bottom_panel();
                    cx.notify();
                    return;
                }
                "b" => {
                    self.state.toggle_left_sidebar();
                    cx.notify();
                    return;
                }
                "j" => {
                    self.state.toggle_bottom_panel();
                    cx.notify();
                    return;
                }
                "l" if !shift => {
                    self.state.focused_panel = app_state::FocusedPanel::Composer;
                    self.state.right_sidebar_open = true;
                    cx.notify();
                    return;
                }
                "s" => {
                    self.save_current_file(window, cx);
                    cx.notify();
                    return;
                }
                "k" => {
                    let active_file = self
                        .state
                        .active_tab()
                        .map(|t| t.path.clone())
                        .unwrap_or_default();
                    let line = self
                        .state
                        .active_tab()
                        .map(|t| t.cursor_row + 1)
                        .unwrap_or(1);
                    self.state
                        .inline_edit
                        .open(100.0, 180.0, active_file, line, String::new());
                    cx.notify();
                    return;
                }
                "z" if !shift => {
                    if let Some(tab) = self.state.active_tab_mut() {
                        tab.undo();
                        cx.notify();
                    }
                    return;
                }
                "y" | "z" if shift => {
                    if let Some(tab) = self.state.active_tab_mut() {
                        tab.redo();
                        cx.notify();
                    }
                    return;
                }
                "/" => {
                    if let Some(tab) = self.state.active_tab_mut() {
                        tab.toggle_comment();
                        cx.notify();
                    }
                    return;
                }
                "a" => {
                    if let Some(tab) = self.state.active_tab_mut() {
                        tab.select_all();
                        cx.notify();
                    }
                    return;
                }
                "d" => {
                    self.state.add_next_occurrence();
                    cx.notify();
                    return;
                }
                "l" if shift => {
                    self.state.select_all_occurrences();
                    cx.notify();
                    return;
                }
                "u" => {
                    self.state.cursor_undo();
                    cx.notify();
                    return;
                }
                "h" => {
                    // Ctrl+H: Show hover tooltip at cursor
                    let hover_target = self.state.active_tab().map(|t| {
                        (
                            t.lines.clone(),
                            t.model.language.clone(),
                            t.cursor_row,
                            t.cursor_col,
                            std::path::PathBuf::from(&t.path),
                        )
                    });
                    if let Some((lines, lang, row, col, path)) = hover_target {
                        let info = self.state.vsx.language_services.provide_hover(
                            &lines,
                            &lang,
                            row,
                            col,
                            Some(&path),
                        );
                        self.state.hover_info = info;
                        self.state.hover_row = row;
                        cx.notify();
                    }
                    return;
                }
                _ => {}
            }
        }

        // In-Editor Find Bar typing
        if self.state.find_bar_open {
            match key {
                "escape" => {
                    self.state.find_bar_open = false;
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.state.find_query.pop();
                    self.state.update_find_matches();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.state.find_query.push_str(c);
                    self.state.update_find_matches();
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // Search panel typing
        if self.state.left_sidebar_open
            && self.state.active_activity == app_state::ActivityTab::Search
        {
            match key {
                "tab" => {
                    self.search.focused_field = match self.search.focused_field {
                        panels::search::SearchFocusedField::Query => {
                            if self.search.show_replace {
                                panels::search::SearchFocusedField::Replace
                            } else if self.search.show_details {
                                panels::search::SearchFocusedField::Include
                            } else {
                                panels::search::SearchFocusedField::Query
                            }
                        }
                        panels::search::SearchFocusedField::Replace => {
                            if self.search.show_details {
                                panels::search::SearchFocusedField::Include
                            } else {
                                panels::search::SearchFocusedField::Query
                            }
                        }
                        panels::search::SearchFocusedField::Include => {
                            panels::search::SearchFocusedField::Exclude
                        }
                        panels::search::SearchFocusedField::Exclude => {
                            panels::search::SearchFocusedField::Query
                        }
                    };
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                "enter" => {
                    let root = self.state.workspace_root.clone();
                    match self.search.focused_field {
                        panels::search::SearchFocusedField::Query => {
                            self.search.execute_search(&root);
                        }
                        panels::search::SearchFocusedField::Replace => {
                            let count = self.search.replace_all(&root);
                            self.state
                                .toast_manager
                                .push_success(&format!("Replaced matches across {count} files"));
                        }
                        panels::search::SearchFocusedField::Include
                        | panels::search::SearchFocusedField::Exclude => {
                            self.search.execute_search(&root);
                        }
                    }
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                "escape" => {
                    if !self.search.query.is_empty() {
                        self.search.query.clear();
                        self.search.matches.clear();
                    } else {
                        self.state.focused_panel = app_state::FocusedPanel::Editor;
                    }
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                "backspace" => {
                    match self.search.focused_field {
                        panels::search::SearchFocusedField::Query => {
                            self.search.query.pop();
                            self.search.execute_search(&self.state.workspace_root);
                        }
                        panels::search::SearchFocusedField::Replace => {
                            self.search.replace.pop();
                        }
                        panels::search::SearchFocusedField::Include => {
                            self.search.include_pattern.pop();
                            self.search.execute_search(&self.state.workspace_root);
                        }
                        panels::search::SearchFocusedField::Exclude => {
                            self.search.exclude_pattern.pop();
                            self.search.execute_search(&self.state.workspace_root);
                        }
                    }
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl && !alt => {
                    match self.search.focused_field {
                        panels::search::SearchFocusedField::Query => {
                            self.search.query.push_str(c);
                            self.search.execute_search(&self.state.workspace_root);
                        }
                        panels::search::SearchFocusedField::Replace => {
                            self.search.replace.push_str(c);
                        }
                        panels::search::SearchFocusedField::Include => {
                            self.search.include_pattern.push_str(c);
                            self.search.execute_search(&self.state.workspace_root);
                        }
                        panels::search::SearchFocusedField::Exclude => {
                            self.search.exclude_pattern.push_str(c);
                            self.search.execute_search(&self.state.workspace_root);
                        }
                    }
                    self.state.search = self.search.clone();
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // Vector search query typing (right sidebar)
        if self.state.right_sidebar_open
            && self.state.active_right_tab == crate::domain::layout::RightSidebarTab::VectorSearch
        {
            match key {
                "enter" => {
                    let root = self.state.workspace_root.clone();
                    if !self.state.vector_search.query.trim().is_empty() {
                        crate::panels::vector_search::execute_search(
                            &mut self.state.vector_search,
                            &root,
                        );
                    }
                    self.state.vector_search.query_focused = false;
                    cx.notify();
                    return;
                }
                "escape" => {
                    self.state.vector_search.query_focused = false;
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.state.vector_search.query.pop();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl && !alt => {
                    self.state.vector_search.query.push_str(c);
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // SCM panel commit message typing
        if self.state.left_sidebar_open
            && self.state.active_activity == app_state::ActivityTab::SourceControl
        {
            match key {
                "enter" if ctrl => {
                    let root = self.state.workspace_root.clone();
                    match self.scm.commit(&root) {
                        Ok(hash) => {
                            self.state
                                .toast_manager
                                .push_success(&format!("Committed: {}", hash));
                        }
                        Err(e) => {
                            self.state
                                .toast_manager
                                .push_error(&format!("Commit failed: {}", e));
                        }
                    }
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.scm.commit_message.pop();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.scm.commit_message.push_str(c);
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // Extensions panel marketplace search typing
        if self.state.left_sidebar_open
            && self.state.active_activity == app_state::ActivityTab::Extensions
        {
            match key {
                "enter" => {
                    let query = self.state.vsx.search_query.clone();
                    match self.state.vsx.search_marketplace(&query) {
                        Ok(()) => {
                            self.state
                                .toast_manager
                                .push_success("Marketplace results updated");
                        }
                        Err(e) => {
                            self.state
                                .toast_manager
                                .push_error(&format!("Search failed: {e}"));
                        }
                    }
                    cx.notify();
                    return;
                }
                "backspace" => {
                    self.state.vsx.search_query.pop();
                    cx.notify();
                    return;
                }
                "escape" => {
                    self.state.vsx.search_query.clear();
                    cx.notify();
                    return;
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.state.vsx.search_query.push_str(c);
                    cx.notify();
                    return;
                }
                _ => {}
            }
        }

        // Panel-specific dispatch
        match self.state.focused_panel {
            app_state::FocusedPanel::Editor => {
                // F2 Rename Symbol
                if key == "f2" {
                    self.state.trigger_rename_for_active_tab();
                    cx.notify();
                    return;
                }

                // Shift+Alt+F: Format Document
                if shift && alt && key.eq_ignore_ascii_case("f") {
                    if self.state.format_active_tab() {
                        self.state.toast_manager.push_success("Document formatted");
                    } else {
                        self.state
                            .toast_manager
                            .push_info("Formatting completed (no changes)");
                    }
                    cx.notify();
                    return;
                }

                // Ctrl+.: Quick Fix / Code Actions
                if ctrl && (key == "." || key == ">") {
                    self.state.trigger_code_actions_for_active_tab();
                    cx.notify();
                    return;
                }

                // Shift+F12: Find All References
                if shift && key == "f12" {
                    self.state.find_references_for_active_tab();
                    cx.notify();
                    return;
                }

                // F12 Go-to-Definition (handled first without holding active_tab_mut)
                if key == "f12" && !shift {
                    let target = self.state.active_tab().map(|t| {
                        (
                            t.lines.clone(),
                            std::path::PathBuf::from(&t.path),
                            t.path.clone(),
                            t.cursor_row,
                            t.cursor_col,
                        )
                    });
                    if let Some((lines, path_buf, current_path, row, col)) = target {
                        if let Some(def) = self
                            .state
                            .vsx
                            .language_services
                            .find_definition(&lines, &path_buf, row, col)
                        {
                            let def_path = def.file_path.to_string_lossy().to_string();
                            if def_path == current_path {
                                if let Some(tab) = self.state.active_tab_mut() {
                                    tab.cursor_row = def.row;
                                    tab.cursor_col = def.col;
                                    tab.model.set_cursor(def.row, def.col);
                                    if def.row < tab.scroll_row || def.row >= tab.scroll_row + 45 {
                                        tab.scroll_row = def.row.saturating_sub(10);
                                    }
                                    self.state
                                        .toast_manager
                                        .push_success(&format!("Definition: line {}", def.row + 1));
                                }
                            } else {
                                self.open_file(&def_path, window, cx);
                                if let Some(new_tab) = self.state.active_tab_mut() {
                                    new_tab.cursor_row = def.row;
                                    new_tab.cursor_col = def.col;
                                    new_tab.model.set_cursor(def.row, def.col);
                                    new_tab.scroll_row = def.row.saturating_sub(10);
                                    self.state.toast_manager.push_success(&format!(
                                        "Navigated to {}:{}",
                                        def_path,
                                        def.row + 1
                                    ));
                                }
                            }
                        } else {
                            self.state.toast_manager.push_error("No definition found");
                        }
                        cx.notify();
                        return;
                    }
                }

                if let Some(tab) = self.state.active_tab_mut() {
                    // IntelliSense popup keyboard navigation
                    if tab.model.completions.is_open {
                        match key {
                            "up" => {
                                tab.model.completions.select_prev();
                                cx.notify();
                                return;
                            }
                            "down" => {
                                tab.model.completions.select_next();
                                cx.notify();
                                return;
                            }
                            "tab" | "enter" => {
                                if let Some(item) = tab.model.completions.current_item().cloned() {
                                    if item.insert_text.contains('$') {
                                        tab.model.insert_snippet(&item.insert_text);
                                    } else {
                                        tab.model.insert_text(&item.insert_text);
                                    }
                                    tab.model.completions.close();
                                    tab.cursor_row = tab.model.cursor_row();
                                    tab.cursor_col = tab.model.cursor_col();
                                    tab.lines = tab.model.buffer.lines().to_vec();
                                    tab.dirty = true;
                                    cx.notify();
                                    return;
                                }
                            }
                            "escape" => {
                                tab.model.completions.close();
                                cx.notify();
                                return;
                            }
                            _ => {}
                        }
                    }

                    // AI Inline Ghost Text acceptance
                    if tab.model.ghost_text.is_active {
                        match key {
                            "tab" => {
                                if let Some(ghost) = tab.model.ghost_text.take() {
                                    tab.model.insert_text(&ghost);
                                    tab.cursor_row = tab.model.cursor_row();
                                    tab.cursor_col = tab.model.cursor_col();
                                    tab.lines = tab.model.buffer.lines().to_vec();
                                    tab.dirty = true;
                                    cx.notify();
                                    return;
                                }
                            }
                            "escape" => {
                                tab.model.ghost_text.clear();
                                cx.notify();
                                return;
                            }
                            _ => {}
                        }
                    }

                    match key {
                        "enter" => {
                            tab.insert_newline();
                            cx.notify();
                        }
                        "backspace" => {
                            tab.backspace();
                            cx.notify();
                        }
                        "delete" => {
                            tab.delete();
                            cx.notify();
                        }
                        "left" => {
                            tab.move_left();
                            cx.notify();
                        }
                        "right" => {
                            tab.move_right();
                            cx.notify();
                        }
                        "up" => {
                            tab.move_up();
                            if tab.cursor_row < tab.scroll_row {
                                tab.scroll_row = tab.cursor_row;
                            }
                            cx.notify();
                        }
                        "down" => {
                            tab.move_down();
                            if tab.cursor_row >= tab.scroll_row + 45 {
                                tab.scroll_row = tab.cursor_row.saturating_sub(44);
                            }
                            cx.notify();
                        }
                        "pageup" => {
                            tab.scroll_row = tab.scroll_row.saturating_sub(35);
                            tab.cursor_row = tab.cursor_row.saturating_sub(35);
                            cx.notify();
                        }
                        "pagedown" => {
                            let max = tab.lines.len().saturating_sub(1);
                            tab.scroll_row = (tab.scroll_row + 35).min(max);
                            tab.cursor_row = (tab.cursor_row + 35).min(max);
                            cx.notify();
                        }
                        "tab" => {
                            if tab.model.snippet_session.is_some() {
                                tab.model.step_snippet_tab_stop(shift);
                                tab.cursor_row = tab.model.cursor_row();
                                tab.cursor_col = tab.model.cursor_col();
                                cx.notify();
                                return;
                            }
                            tab.insert_char(' ');
                            tab.insert_char(' ');
                            tab.insert_char(' ');
                            tab.insert_char(' ');
                            cx.notify();
                        }
                        "escape" => {
                            tab.model.completions.close();
                            tab.model.ghost_text.clear();
                            tab.model.snippet_session = None;
                            tab.clear_secondary_cursors();
                            self.state.hover_info = None;
                            cx.notify();
                        }
                        c if c.chars().count() == 1 && !ctrl => {
                            if let Some(ch) = c.chars().next() {
                                tab.insert_char(ch);
                                cx.notify();
                            }
                        }
                        _ => {}
                    }
                }
            }
            app_state::FocusedPanel::Terminal => match key {
                "enter" => {
                    let cmd = self.state.terminal_input.clone();
                    self.state.execute_terminal_command(&cmd);
                    cx.notify();
                }
                "backspace" => {
                    self.state.terminal_input.pop();
                    if let Some(pty) = self.state.active_pty() {
                        let _ = pty.write_input("\x08 \x08");
                    }
                    cx.notify();
                }
                "c" if ctrl => {
                    if let Some(pty) = self.state.active_pty() {
                        let _ = pty.send_interrupt();
                    }
                    self.state.terminal_input.clear();
                    cx.notify();
                }
                "v" if ctrl => {
                    if let Some(item) = cx.read_from_clipboard() {
                        if let Some(text) = item.text() {
                            self.state.terminal_input.push_str(&text);
                            if let Some(pty) = self.state.active_pty() {
                                let _ = pty.write_input(&text);
                            }
                            cx.notify();
                        }
                    }
                }
                c if c.chars().count() == 1 && !ctrl => {
                    self.state.terminal_input.push_str(c);
                    if let Some(pty) = self.state.active_pty() {
                        let _ = pty.write_input(c);
                    }
                    cx.notify();
                }
                _ => {}
            },
            app_state::FocusedPanel::Composer => {
                if let Some(edit_idx) = self.state.editing_msg_idx {
                    match key {
                        "v" if ctrl => {
                            if let Some(item) = cx.read_from_clipboard() {
                                if let Some(text) = item.text() {
                                    self.state.edit_msg_input.push_str(&text);
                                    cx.notify();
                                }
                            }
                        }
                        "u" if ctrl => {
                            self.state.edit_msg_input.clear();
                            cx.notify();
                        }
                        "enter" if shift => {
                            self.state.edit_msg_input.push('\n');
                            cx.notify();
                        }
                        "enter" => {
                            self.state.save_and_resend_message(edit_idx);
                            cx.notify();
                        }
                        "space" => {
                            self.state.edit_msg_input.push(' ');
                            cx.notify();
                        }
                        "tab" => {
                            self.state.edit_msg_input.push_str("  ");
                            cx.notify();
                        }
                        "backspace" if ctrl => {
                            if let Some(idx) = self.state.edit_msg_input.rfind(|c: char| c.is_whitespace()) {
                                self.state.edit_msg_input.truncate(idx);
                            } else {
                                self.state.edit_msg_input.clear();
                            }
                            cx.notify();
                        }
                        "backspace" => {
                            self.state.edit_msg_input.pop();
                            cx.notify();
                        }
                        "escape" => {
                            self.state.cancel_edit_message();
                            cx.notify();
                        }
                        c if c.chars().count() == 1 && !ctrl && !alt => {
                            self.state.edit_msg_input.push_str(c);
                            cx.notify();
                        }
                        _ => {}
                    }
                } else {
                    match key {
                        "v" if ctrl => {
                            if let Some(item) = cx.read_from_clipboard() {
                                if let Some(text) = item.text() {
                                    self.state.composer_input.push_str(&text);
                                    cx.notify();
                                }
                            }
                        }
                        "a" if ctrl => {
                            // Quick clear or highlight
                            cx.notify();
                        }
                        "u" if ctrl => {
                            self.state.composer_input.clear();
                            cx.notify();
                        }
                        "enter" if shift => {
                            self.state.composer_input.push('\n');
                            cx.notify();
                        }
                        "enter" => {
                            if !self.state.composer_input.trim().is_empty() {
                                let prompt = self.state.composer_input.clone();
                                self.state.send_ai_prompt(&prompt);
                                cx.notify();
                            }
                        }
                        "space" => {
                            self.state.composer_input.push(' ');
                            cx.notify();
                        }
                        "tab" => {
                            self.state.composer_input.push_str("  ");
                            cx.notify();
                        }
                        "backspace" if ctrl => {
                            // Pop last word
                            if let Some(idx) = self.state.composer_input.rfind(|c: char| c.is_whitespace()) {
                                self.state.composer_input.truncate(idx);
                            } else {
                                self.state.composer_input.clear();
                            }
                            cx.notify();
                        }
                        "backspace" => {
                            self.state.composer_input.pop();
                            cx.notify();
                        }
                        "escape" => {
                            if self.state.is_agent_thinking {
                                self.state.cancel_ai_turn();
                            } else {
                                self.state.focused_panel = app_state::FocusedPanel::Editor;
                            }
                            cx.notify();
                        }
                        c if c.chars().count() == 1 && !ctrl && !alt => {
                            self.state.composer_input.push_str(c);
                            cx.notify();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}

fn main() {
    std::panic::set_hook(Box::new(|info| {
        let backtrace = std::backtrace::Backtrace::capture();
        let payload = if let Some(s) = info.payload().downcast_ref::<&str>() {
            *s
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.as_str()
        } else {
            "unknown panic payload"
        };
        let location = info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown location".to_string());
        let log_msg = format!(
            "VSCodium Rust PANIC at {}:\n{}\n\nBacktrace:\n{:?}\n",
            location, payload, backtrace
        );

        if let Some(home) = dirs::home_dir() {
            let dir = home.join(".vscodium-rust");
            let _ = std::fs::create_dir_all(&dir);
            let _ = std::fs::write(dir.join("crash.log"), &log_msg);
        }
        let temp_crash = std::env::temp_dir().join("vscodium_rust_crash.log");
        let _ = std::fs::write(temp_crash, &log_msg);
        eprintln!("{log_msg}");
    }));

    let args: Vec<String> = std::env::args().collect();
    let initial_path = args.iter().skip(1).find_map(|arg| {
        if arg.starts_with("--") || arg.starts_with("-") {
            None
        } else {
            let p = std::path::PathBuf::from(arg);
            if p.exists() {
                Some(p)
            } else {
                None
            }
        }
    });

    let exe_path = std::env::current_exe().unwrap_or_default();
    let exe_name = exe_path.file_name().and_then(|s| s.to_str()).unwrap_or("").to_lowercase();

    // Automatically ensure background daemons for physical iPhone are running (touch HID :8105, tunnel :60105, TSS :816)
    let udid = std::env::var("IPHONE_UDID").unwrap_or_else(|_| "auto".to_string());
    panels::daemon_supervisor::start_auto_daemon_supervisor(udid);

    if exe_name.contains("vscodium-ip-panel") || exe_name.contains("ip-panel") || args.iter().any(|a| a == "--device-window" || a == "--zoom" || a == "--bounty-window") {
        panels::device_window::run_device_mirror_app();
        return;
    }
    gpui_kit::application()
        .with_assets(gpui_kit_assets::AllAssets)
        .run(move |cx: &mut App| {
            gpui_kit::init(cx);
            let window_bounds = Some(WindowBounds::centered(size(px(1300.0), px(860.0)), cx));
            let init_path_capture = initial_path.clone();
            cx.open_window(
                WindowOptions {
                    window_bounds,
                    titlebar: None,
                    is_movable: true,
                    is_resizable: true,
                    window_min_size: Some(size(px(800.0), px(600.0))),
                    ..Default::default()
                },
                move |window, cx| {
                    let view = cx.new(|cx| HadesAppView::new(window, init_path_capture.clone(), cx));
                    cx.new(|cx| gpui_kit::component::Root::new(view, window, cx))
                },
            )
            .expect("Failed to create native GPUI window");
        });
}
