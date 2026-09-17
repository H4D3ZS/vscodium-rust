use crate::state::*;
use crate::ui::context_menu::ContextMenuItem;
use crate::ui::toast::ToastSeverity;
use crate::HadesAppView;
use gpui_kit::gpui::*;

#[derive(Clone, Debug)]
pub enum AppEvent {
    // Editor & File actions
    OpenFile(String),
    SelectTab(usize),
    CloseTab(usize),
    SaveActiveFile,
    CreateNewFile(String),
    ToggleFolder(String),
    RefreshFileTree,

    // Layout toggles
    ToggleLeftSidebar,
    ToggleRightSidebar,
    ToggleBottomPanel,
    SetActivity(ActivityTab),
    SetBottomTab(BottomPanelTab),
    SetRightTab(RightSidebarTab),
    FocusPanel(FocusedPanel),

    // AI & Composer actions
    SendAiPrompt(String),
    ClearAiChat,
    CreateAgentThread,
    SelectAgentThread(usize),
    SetAgentMode(String),
    SetAgentModel(String),

    // Terminal actions
    ExecuteTerminalCommand(String),
    ClearTerminal,
    SpawnTerminal(Option<String>),
    KillTerminal,
    SelectTerminal(usize),

    // SCM & Git actions
    RefreshGit,
    StageAll,
    UnstageAll,
    CommitChanges(Option<String>),

    // Editor find
    ToggleFindBar,
    SetFindQuery(String),

    // Overlays & Feedback
    ShowToast {
        message: String,
        severity: ToastSeverity,
    },
    ShowContextMenu {
        x: f32,
        y: f32,
        items: Vec<ContextMenuItem>,
    },
    CloseContextMenu,
    OpenInlineEdit {
        top: f32,
        left: f32,
        target_file: String,
        line: usize,
        selected_text: String,
    },
    OpenInlineEditAtCursor,
    CloseInlineEdit,
    SubmitInlineEdit(String),
}

impl HadesAppView {
    pub fn handle_event(&mut self, event: AppEvent, window: &mut Window, cx: &mut Context<Self>) {
        match event {
            AppEvent::OpenFile(path) => {
                self.open_file(&path, window, cx);
            }
            AppEvent::SelectTab(idx) => {
                self.select_tab(idx, window, cx);
            }
            AppEvent::CloseTab(idx) => {
                self.close_tab(idx, window, cx);
            }
            AppEvent::SaveActiveFile => {
                self.save_active_file(cx);
            }
            AppEvent::CreateNewFile(rel_path) => {
                self.state.create_new_file(&rel_path);
            }
            AppEvent::ToggleFolder(path) => {
                self.state.toggle_folder(&path);
            }
            AppEvent::RefreshFileTree => {
                self.state.refresh_file_tree();
            }
            AppEvent::ToggleLeftSidebar => {
                self.state.toggle_left_sidebar();
            }
            AppEvent::ToggleRightSidebar => {
                self.state.toggle_right_sidebar();
            }
            AppEvent::ToggleBottomPanel => {
                self.state.toggle_bottom_panel();
            }
            AppEvent::SetActivity(tab) => {
                self.state.set_activity(tab);
            }
            AppEvent::SetBottomTab(tab) => {
                self.state.set_bottom_tab(tab);
            }
            AppEvent::SetRightTab(tab) => {
                self.state.active_right_tab = tab;
            }
            AppEvent::FocusPanel(panel) => {
                self.state.focused_panel = panel;
            }
            AppEvent::SendAiPrompt(prompt) => {
                self.state.send_ai_prompt(&prompt);
            }
            AppEvent::ClearAiChat => {
                self.state.clear_ai_chat();
            }
            AppEvent::CreateAgentThread => {
                self.state.create_agent_thread();
            }
            AppEvent::SelectAgentThread(idx) => {
                self.state.select_agent_thread(idx);
            }
            AppEvent::SetAgentMode(mode) => {
                self.state.agent_mode = mode;
            }
            AppEvent::SetAgentModel(model) => {
                self.state.agent_model = model;
            }
            AppEvent::ExecuteTerminalCommand(cmd) => {
                self.execute_terminal_command(&cmd);
            }
            AppEvent::ClearTerminal => {
                self.state.clear_terminal();
            }
            AppEvent::SpawnTerminal(shell) => {
                self.state.spawn_terminal(shell.as_deref());
            }
            AppEvent::KillTerminal => {
                self.state.kill_active_terminal();
            }
            AppEvent::SelectTerminal(idx) => {
                self.state.select_terminal(idx);
            }
            AppEvent::RefreshGit => {
                let root = self.state.workspace_root.clone();
                self.scm.refresh(&root);
                self.state.toast_manager.push_info("Git status refreshed");
            }
            AppEvent::StageAll => {
                let root = self.state.workspace_root.clone();
                self.scm.stage_all(&root);
            }
            AppEvent::UnstageAll => {
                let root = self.state.workspace_root.clone();
                self.scm.unstage_all(&root);
            }
            AppEvent::CommitChanges(custom_msg) => {
                if let Some(msg) = custom_msg {
                    self.scm.commit_message = msg;
                }
                let root = self.state.workspace_root.clone();
                match self.scm.commit(&root) {
                    Ok(_) => self
                        .state
                        .toast_manager
                        .push_success("Git commit succeeded"),
                    Err(e) => self
                        .state
                        .toast_manager
                        .push_error(&format!("Git commit failed: {e}")),
                }
            }
            AppEvent::ToggleFindBar => {
                self.state.toggle_find_bar();
            }
            AppEvent::SetFindQuery(query) => {
                self.state.find_query = query;
                self.state.update_find_matches();
            }
            AppEvent::ShowToast { message, severity } => match severity {
                ToastSeverity::Info => self.state.toast_manager.push_info(&message),
                ToastSeverity::Success => self.state.toast_manager.push_success(&message),
                ToastSeverity::Warning => self.state.toast_manager.push_warning(&message),
                ToastSeverity::Error => self.state.toast_manager.push_error(&message),
            },
            AppEvent::ShowContextMenu { x, y, items } => {
                self.state.context_menu.show(x, y, items);
            }
            AppEvent::CloseContextMenu => {
                self.state.context_menu.close();
            }
            AppEvent::OpenInlineEdit {
                top,
                left,
                target_file,
                line,
                selected_text,
            } => {
                self.state
                    .inline_edit
                    .open(top, left, target_file, line, selected_text);
            }
            AppEvent::OpenInlineEditAtCursor => {
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
            }
            AppEvent::CloseInlineEdit => {
                self.state.inline_edit.close();
            }
            AppEvent::SubmitInlineEdit(instruction) => {
                let prompt = format!("Inline edit instruction: {}", instruction);
                self.state.send_ai_prompt(&prompt);
                self.state.inline_edit.close();
                self.state
                    .toast_manager
                    .push_info("Dispatched AI inline edit instruction");
            }
        }
        cx.notify();
    }
}
