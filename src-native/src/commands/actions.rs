use crate::app_state::ActivityTab;
use crate::commands::{Command, CommandRegistry};
use crate::ui::icons::IconName;
use std::sync::Arc;

pub fn register_all_commands(registry: &mut CommandRegistry) {
    // ── Layout & View Commands ──────────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.toggleSidebarVisibility",
        title: "View: Toggle Primary Side Bar",
        category: "View",
        shortcut: Some("Ctrl+B"),
        icon: IconName::PanelLeft,
        handler: Arc::new(|view, _window, cx| {
            view.state.left_sidebar_open = !view.state.left_sidebar_open;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.togglePanel",
        title: "View: Toggle Bottom Panel (Terminal)",
        category: "View",
        shortcut: Some("Ctrl+J"),
        icon: IconName::PanelBottom,
        handler: Arc::new(|view, _window, cx| {
            view.state.bottom_panel_open = !view.state.bottom_panel_open;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.toggleAuxiliaryBar",
        title: "View: Toggle Auxiliary Bar (Composer)",
        category: "View",
        shortcut: Some("Ctrl+L"),
        icon: IconName::PanelRight,
        handler: Arc::new(|view, _window, cx| {
            view.state.right_sidebar_open = !view.state.right_sidebar_open;
            cx.notify();
        }),
    });

    // ── Navigation & Activity Commands ──────────────────────────────────────
    registry.register(Command {
        id: "workbench.view.explorer",
        title: "View: Show Explorer",
        category: "View",
        shortcut: Some("Ctrl+Shift+E"),
        icon: IconName::Files,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Explorer);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.view.search",
        title: "View: Show Search",
        category: "View",
        shortcut: Some("Ctrl+Shift+F"),
        icon: IconName::Search,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Search);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.view.scm",
        title: "View: Show Source Control (Git)",
        category: "View",
        shortcut: Some("Ctrl+Shift+G"),
        icon: IconName::GitBranch,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::SourceControl);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.view.debug",
        title: "View: Show Run and Debug",
        category: "View",
        shortcut: Some("Ctrl+Shift+D"),
        icon: IconName::Bug,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Debug);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.view.extensions",
        title: "View: Show Extensions and MCP Store",
        category: "View",
        shortcut: Some("Ctrl+Shift+X"),
        icon: IconName::Blocks,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Extensions);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.openSettings",
        title: "Preferences: Open Settings",
        category: "Preferences",
        shortcut: Some("Ctrl+,"),
        icon: IconName::Settings,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Settings);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.openGlobalKeybindings",
        title: "Preferences: Open Keyboard Shortcuts",
        category: "Preferences",
        shortcut: Some("Ctrl+K Ctrl+S"),
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Settings);
            view.state.settings_category = 11; // Shortcuts tab
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.view.outline",
        title: "View: Show Outline",
        category: "View",
        shortcut: Some("Ctrl+Shift+O"),
        icon: IconName::ListTree,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Explorer);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    // ── File & Buffer Commands ──────────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.files.save",
        title: "File: Save Current Buffer",
        category: "File",
        shortcut: Some("Ctrl+S"),
        icon: IconName::Save,
        handler: Arc::new(|view, window, cx| {
            view.save_current_file(window, cx);
            view.state.status_message = "File saved successfully".to_string();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.files.newUntitledFile",
        title: "File: New Untitled File",
        category: "File",
        shortcut: Some("Ctrl+N"),
        icon: IconName::FilePlus,
        handler: Arc::new(|view, _window, cx| {
            let id = chrono::Local::now().timestamp_subsec_millis();
            view.state.create_new_file(&format!("untitled_{id}.rs"));
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.closeActiveEditor",
        title: "View: Close Active Tab",
        category: "View",
        shortcut: Some("Ctrl+W"),
        icon: IconName::X,
        handler: Arc::new(|view, window, cx| {
            if let Some(idx) = view.state.active_tab_idx {
                view.close_tab(idx, window, cx);
                cx.notify();
            }
        }),
    });

    // ── Advanced Language & LSP Commands ──────────────────────────────────
    registry.register(Command {
        id: "workbench.action.formatDocument",
        title: "Format Document",
        category: "Editor",
        shortcut: Some("Shift+Alt+F"),
        icon: IconName::Check,
        handler: Arc::new(|view, _window, cx| {
            if view.state.format_active_tab() {
                view.state.toast_manager.push_success("Document formatted");
            } else {
                view.state
                    .toast_manager
                    .push_info("Formatting completed (no changes)");
            }
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.quickFix",
        title: "Quick Fix / Code Actions...",
        category: "Editor",
        shortcut: Some("Ctrl+."),
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.trigger_code_actions_for_active_tab();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.rename",
        title: "Rename Symbol",
        category: "Editor",
        shortcut: Some("F2"),
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            view.state.trigger_rename_for_active_tab();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.referenceSearch.trigger",
        title: "Find All References",
        category: "Editor",
        shortcut: Some("Shift+F12"),
        icon: IconName::Search,
        handler: Arc::new(|view, _window, cx| {
            view.state.find_references_for_active_tab();
            cx.notify();
        }),
    });

    // ── Split Editor Commands ────────────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.splitEditor",
        title: "View: Split Editor Right",
        category: "View",
        shortcut: Some("Ctrl+\\"),
        icon: IconName::PanelRight,
        handler: Arc::new(|view, _window, cx| {
            view.state.split_editor();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.focusFirstEditorGroup",
        title: "View: Focus First Editor Group",
        category: "View",
        shortcut: Some("Ctrl+1"),
        icon: IconName::PanelLeft,
        handler: Arc::new(|view, _window, cx| {
            view.state.focus_pane(0);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.focusSecondEditorGroup",
        title: "View: Focus Second Editor Group",
        category: "View",
        shortcut: Some("Ctrl+2"),
        icon: IconName::PanelRight,
        handler: Arc::new(|view, _window, cx| {
            if view.state.is_split_editor {
                view.state.focus_pane(1);
                cx.notify();
            }
        }),
    });

    registry.register(Command {
        id: "workbench.action.closeSplitEditor",
        title: "View: Close Split Editor",
        category: "View",
        shortcut: None,
        icon: IconName::X,
        handler: Arc::new(|view, _window, cx| {
            view.state.close_split_editor();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.toggleStickyScroll",
        title: "View: Toggle Sticky Scroll",
        category: "View",
        shortcut: None,
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            view.state.toggle_sticky_scroll();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.toggleMinimap",
        title: "View: Toggle Minimap",
        category: "View",
        shortcut: None,
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            view.state.toggle_minimap();
            cx.notify();
        }),
    });

    // ── Monaco Diff Editor Commands ──────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.compareFiles",
        title: "File: Compare Active File with HEAD (Git Diff)",
        category: "File",
        shortcut: Some("Ctrl+K D"),
        icon: IconName::GitCompare,
        handler: Arc::new(|view, _window, cx| {
            if let Some(tab) = view.state.active_tab() {
                let path = tab.path.clone();
                let root = view.state.workspace_root.clone();
                let rel_path = if let Ok(rel) = std::path::Path::new(&path).strip_prefix(&root) {
                    rel.to_string_lossy().to_string()
                } else {
                    path.clone()
                };
                view.diff_viewer.open_diff(&root, &rel_path);
                cx.notify();
            }
        }),
    });

    registry.register(Command {
        id: "editor.action.diffReview.next",
        title: "Diff: Next Change",
        category: "Diff",
        shortcut: Some("Alt+F5"),
        icon: IconName::ChevronDown,
        handler: Arc::new(|view, _window, cx| {
            if view.diff_viewer.is_open {
                view.diff_viewer.next_hunk();
                cx.notify();
            }
        }),
    });

    registry.register(Command {
        id: "editor.action.diffReview.prev",
        title: "Diff: Previous Change",
        category: "Diff",
        shortcut: Some("Shift+Alt+F5"),
        icon: IconName::ChevronUp,
        handler: Arc::new(|view, _window, cx| {
            if view.diff_viewer.is_open {
                view.diff_viewer.prev_hunk();
                cx.notify();
            }
        }),
    });

    registry.register(Command {
        id: "workbench.action.toggleDiffSideBySide",
        title: "Diff: Toggle Side-by-Side / Inline",
        category: "Diff",
        shortcut: None,
        icon: IconName::GitCompare,
        handler: Arc::new(|view, _window, cx| {
            if view.diff_viewer.is_open {
                view.diff_viewer.toggle_side_by_side();
                cx.notify();
            }
        }),
    });

    // ── Quick Open & Command Palette ────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.quickOpen",
        title: "Go to File...",
        category: "Navigation",
        shortcut: Some("Ctrl+P"),
        icon: IconName::Search,
        handler: Arc::new(|view, _window, cx| {
            let cmds = view.commands.clone();
            view.quick_open.open(&view.state, &cmds);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.gotoSymbol",
        title: "Go to Symbol in File...",
        category: "Navigation",
        shortcut: Some("Ctrl+Shift+O"),
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            let cmds = view.commands.clone();
            view.quick_open.open_symbols(&view.state, &cmds);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.gotoLine",
        title: "Go to Line/Column...",
        category: "Navigation",
        shortcut: Some("Ctrl+G"),
        icon: IconName::Code,
        handler: Arc::new(|view, _window, cx| {
            let cmds = view.commands.clone();
            view.quick_open.open_line(&view.state, &cmds);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.showCommands",
        title: "View: Command Palette...",
        category: "Navigation",
        shortcut: Some("Ctrl+Shift+P"),
        icon: IconName::Terminal,
        handler: Arc::new(|view, _window, cx| {
            let cmds = view.commands.clone();
            view.quick_open.open_commands(&view.state, &cmds);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.editor.changeLanguageMode",
        title: "Change Language Mode",
        category: "Editor",
        shortcut: Some("Ctrl+K M"),
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            let cmds = view.commands.clone();
            view.quick_open.open_commands(&view.state, &cmds);
            view.state
                .toast_manager
                .push_info("Select Language Mode (Rust, TypeScript, Python, JSON, Markdown, etc.)");
            cx.notify();
        }),
    });

    // ── Terminal Commands ───────────────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.terminal.new",
        title: "Terminal: Create New Integrated Terminal",
        category: "Terminal",
        shortcut: Some("Ctrl+Shift+`"),
        icon: IconName::SquareTerminal,
        handler: Arc::new(|view, _window, cx| {
            view.state.bottom_panel_open = true;
            view.state.status_message = "New integrated terminal session created".to_string();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.terminal.clear",
        title: "Terminal: Clear Terminal Output",
        category: "Terminal",
        shortcut: None,
        icon: IconName::Trash,
        handler: Arc::new(|view, _window, cx| {
            view.state.clear_terminal();
            cx.notify();
        }),
    });

    // ── Git / SCM Commands ──────────────────────────────────────────────────
    registry.register(Command {
        id: "git.refresh",
        title: "Git: Refresh Repository Status",
        category: "Git",
        shortcut: None,
        icon: IconName::RefreshCw,
        handler: Arc::new(|view, _window, cx| {
            view.scm.refresh(&view.state.workspace_root);
            view.state.status_message = "Git status refreshed".to_string();
            cx.notify();
        }),
    });

    // ── Agent & AI Composer Commands ────────────────────────────────────────
    registry.register(Command {
        id: "agent.action.newChat",
        title: "Agent: New Composer Chat Session",
        category: "Agent",
        shortcut: Some("Ctrl+Alt+N"),
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.clear_ai_chat();
            view.state.right_sidebar_open = true;
            view.state.status_message = "New AI Agent session ready".to_string();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "agent.action.openComposer",
        title: "Agent: Open AI Composer",
        category: "Agent",
        shortcut: Some("Ctrl+I"),
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.right_sidebar_open = true;
            view.state.focused_panel = crate::app_state::FocusedPanel::Composer;
            cx.notify();
        }),
    });

    // ── Bracket Pair Colorization & Indent Guides ───────────────────────────
    registry.register(Command {
        id: "editor.action.toggleBracketColorization",
        title: "Editor: Toggle Bracket Pair Colorization",
        category: "Editor",
        shortcut: None,
        icon: IconName::Code,
        handler: Arc::new(|view, _window, cx| {
            view.state.bracket_pairs.toggle_colorization();
            let enabled = view.state.bracket_pairs.colorization_enabled;
            view.state.status_message = format!(
                "Bracket Pair Colorization: {}",
                if enabled { "On" } else { "Off" }
            );
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.toggleIndentGuides",
        title: "Editor: Toggle Indent Guides",
        category: "Editor",
        shortcut: None,
        icon: IconName::Code,
        handler: Arc::new(|view, _window, cx| {
            view.state.bracket_pairs.toggle_indent_guides();
            let enabled = view.state.bracket_pairs.indent_guides.enabled;
            view.state.status_message =
                format!("Indent Guides: {}", if enabled { "On" } else { "Off" });
            cx.notify();
        }),
    });

    // ── Multi-Cursor & Occurrence Highlighting ──────────────────────────────
    registry.register(Command {
        id: "editor.action.addSelectionToNextFindMatch",
        title: "Selection: Add Next Occurrence (Multi-Cursor)",
        category: "Selection",
        shortcut: Some("Ctrl+D"),
        icon: IconName::Search,
        handler: Arc::new(|view, _window, cx| {
            view.state.add_next_occurrence();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.selectHighlights",
        title: "Selection: Select All Occurrences",
        category: "Selection",
        shortcut: Some("Ctrl+Shift+L"),
        icon: IconName::Search,
        handler: Arc::new(|view, _window, cx| {
            view.state.select_all_occurrences();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.insertCursorAbove",
        title: "Selection: Add Cursor Above (Multi-Cursor)",
        category: "Selection",
        shortcut: Some("Ctrl+Alt+Up"),
        icon: IconName::ChevronUp,
        handler: Arc::new(|view, _window, cx| {
            view.state.add_cursor_above();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.insertCursorBelow",
        title: "Selection: Add Cursor Below (Multi-Cursor)",
        category: "Selection",
        shortcut: Some("Ctrl+Alt+Down"),
        icon: IconName::ChevronDown,
        handler: Arc::new(|view, _window, cx| {
            view.state.add_cursor_below();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "cursorUndo",
        title: "Cursor Undo",
        category: "Selection",
        shortcut: Some("Ctrl+U"),
        icon: IconName::Undo,
        handler: Arc::new(|view, _window, cx| {
            view.state.cursor_undo();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "editor.action.toggleWordHighlight",
        title: "Editor: Toggle Word Occurrence Highlighting",
        category: "Editor",
        shortcut: None,
        icon: IconName::FileCode,
        handler: Arc::new(|view, _window, cx| {
            view.state.toggle_word_highlight();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "markdown.showPreview",
        title: "Markdown: Open Live Preview",
        category: "Markdown",
        shortcut: Some("Ctrl+Shift+V"),
        icon: IconName::FileText,
        handler: Arc::new(|view, _window, cx| {
            view.state.toggle_markdown_preview();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.trust.manage",
        title: "Security: Manage Workspace Trust",
        category: "Security",
        shortcut: None,
        icon: IconName::Shield,
        handler: Arc::new(|view, _window, cx| {
            view.state.settings_category = 10; // Workspace & Trust tab
            view.state.set_activity(ActivityTab::Settings);
            view.state.left_sidebar_open = true;
            cx.notify();
        }),
    });

    // ── Tasks & Run Configurations ──────────────────────────────────────────
    registry.register(Command {
        id: "workbench.action.tasks.runTask",
        title: "Tasks: Run Task",
        category: "Tasks",
        shortcut: Some("Ctrl+Shift+B"),
        icon: IconName::Play,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Debug);
            view.state.left_sidebar_open = true;
            if let Some(task) = view.state.run_configs.tasks.first().cloned() {
                let ctx = crate::panels::run_configs::VarContext {
                    workspace_folder: Some(view.state.workspace_root.to_string_lossy().to_string()),
                    file: view.state.active_tab().map(|t| t.path.clone()),
                    line_number: view.state.active_tab().map(|t| t.cursor_row + 1),
                    selected_text: None,
                    exec_path: std::env::current_exe()
                        .ok()
                        .map(|p| p.to_string_lossy().to_string()),
                };
                let cmd = crate::panels::run_configs::build_task_command(&task, &ctx);
                view.execute_terminal_command(&cmd);
                view.state.bottom_panel_open = true;
                view.state
                    .toast_manager
                    .push_info(&format!("Running task: {}", task.label));
            } else {
                view.state
                    .toast_manager
                    .push_info("Open Run & Debug to configure tasks");
            }
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.tasks.reload",
        title: "Tasks: Reload tasks.json & launch.json",
        category: "Tasks",
        shortcut: None,
        icon: IconName::RefreshCw,
        handler: Arc::new(|view, _window, cx| {
            let root = view.state.workspace_root.clone();
            view.state.run_configs.reload(&root);
            view.state
                .toast_manager
                .push_success("Reloaded tasks.json & launch.json");
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.debug.selectAndStart",
        title: "Debug: Select and Start Debugging",
        category: "Debug",
        shortcut: Some("F5"),
        icon: IconName::Bug,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::Debug);
            view.state.left_sidebar_open = true;
            if view.state.debug.is_debugging() {
                view.state.debug.session_status = crate::panels::debug::DebugSessionStatus::Running;
                view.state.debug.debug_output.push("[DAP] Continued execution (F5).".to_string());
                view.state.toast_manager.push_info("Debugger: Continue (F5)");
            } else {
                let cfg = view.state.run_configs.selected_launch_config().cloned();
                let (name, prog) = if let Some(c) = cfg {
                    (c.name, c.program.unwrap_or_default())
                } else {
                    ("Default Debug Target".to_string(), "src-native/src/main.rs".to_string())
                };
                view.state.debug.start_session(&name, &prog);
                view.state.run_configs.last_status =
                    Some((name.clone(), true, "Debug session started.".to_string()));
                view.state
                    .toast_manager
                    .push_success(&format!("Started debug session: {}", name));
            }
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.debug.console",
        title: "Debug: Focus Debug Console",
        category: "Debug",
        shortcut: Some("Ctrl+Shift+Y"),
        icon: IconName::Terminal,
        handler: Arc::new(|view, _window, cx| {
            view.state
                .set_bottom_tab(crate::domain::layout::BottomPanelTab::DebugConsole);
            view.state.bottom_panel_open = true;
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.scm.viewGraph",
        title: "Git: View Commit Graph",
        category: "Git",
        shortcut: None,
        icon: IconName::GitBranch,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_activity(ActivityTab::SourceControl);
            view.state.left_sidebar_open = true;
            view.scm.active_subtab = crate::panels::scm::ScmViewTab::Graph;
            let root = view.state.workspace_root.clone();
            view.scm.git_graph.reload(&root);
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.ai.toggleThoughtHud",
        title: "AI: Toggle Cognitive Process HUD",
        category: "AI",
        shortcut: None,
        icon: IconName::Bot,
        handler: Arc::new(|view, _window, cx| {
            if view.state.current_thought.is_some() {
                view.state.clear_thought();
            } else {
                view.state.set_thought(
                    "Evaluating AST symbol dependencies and active workspace context...",
                    "Inspect module exports and resolve references",
                    Some(0.96),
                );
            }
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.ai.simulateThought",
        title: "AI: Preview Cognitive Reasoning Trace",
        category: "AI",
        shortcut: None,
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.set_thought(
                "Detecting unhandled edge cases in piecewise table gap buffer allocations...",
                "Optimize contiguous span slices into monotonic vector",
                Some(0.99),
            );
            view.state
                .toast_manager
                .push_info("Cognitive trace active in workspace HUD");
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.ai.toggleInferenceHealth",
        title: "AI: Toggle Inference Health & Performance Dashboard",
        category: "AI",
        shortcut: None,
        icon: IconName::Sparkles,
        handler: Arc::new(|view, _window, cx| {
            view.state.toggle_inference_health();
            cx.notify();
        }),
    });

    registry.register(Command {
        id: "workbench.action.openMcpStore",
        title: "MCP: Open Model Context Protocol Store & Manager",
        category: "MCP",
        shortcut: None,
        icon: IconName::Bot,
        handler: Arc::new(|view, _window, cx| {
            view.state.mcp_store.view = crate::panels::McpStoreView::Store;
            view.state.status_message =
                "Opened Model Context Protocol Store & Manager.".to_string();
            cx.notify();
        }),
    });
}
