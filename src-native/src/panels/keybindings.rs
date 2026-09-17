use crate::app_state::{FocusedPanel, HadesNativeState};
use crate::panels::traits::{AuxiliaryTab, WorkbenchPanel};
use crate::theme::Theme;
use crate::ui::icons::{icon_12, ui_icon, IconName};
use crate::HadesAppView;
use gpui_kit::gpui::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeybindingSource {
    Default,
    User,
}

impl KeybindingSource {
    pub fn label(&self) -> &'static str {
        match self {
            KeybindingSource::Default => "Default",
            KeybindingSource::User => "User",
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeybindingItem {
    pub id: String,
    pub command: String,
    pub title: String,
    pub category: String,
    pub key: String,
    pub default_key: String,
    pub when: Option<String>,
    pub source: KeybindingSource,
}

impl KeybindingItem {
    pub fn new(
        command: &str,
        title: &str,
        category: &str,
        default_key: &str,
        when: Option<&str>,
    ) -> Self {
        Self {
            id: command.to_string(),
            command: command.to_string(),
            title: title.to_string(),
            category: category.to_string(),
            key: default_key.to_string(),
            default_key: default_key.to_string(),
            when: when.map(|s| s.to_string()),
            source: KeybindingSource::Default,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KeybindingsState {
    pub filter_query: String,
    pub selected_category: Option<String>,
    pub editing_command: Option<String>,
    pub draft_key: String,
    pub custom_overrides: HashMap<String, String>,
    pub items: Vec<KeybindingItem>,
}

impl Default for KeybindingsState {
    fn default() -> Self {
        Self::new()
    }
}

impl KeybindingsState {
    pub fn new() -> Self {
        let items = Self::default_catalog();
        Self {
            filter_query: String::new(),
            selected_category: None,
            editing_command: None,
            draft_key: String::new(),
            custom_overrides: HashMap::new(),
            items,
        }
    }

    pub fn default_catalog() -> Vec<KeybindingItem> {
        vec![
            // ── General & Workbench ──────────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.showCommands",
                "Command Palette",
                "General",
                "Ctrl+Shift+P",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.quickOpen",
                "Go to File (Quick Open)",
                "General",
                "Ctrl+P",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.openSettings",
                "Preferences: Open Settings",
                "General",
                "Ctrl+,",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.openGlobalKeybindings",
                "Preferences: Keyboard Shortcuts",
                "General",
                "Ctrl+K Ctrl+S",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.toggleSidebarVisibility",
                "View: Toggle Primary Side Bar",
                "View",
                "Ctrl+B",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.togglePanel",
                "View: Toggle Bottom Panel (Terminal)",
                "View",
                "Ctrl+J",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.toggleAuxiliaryBar",
                "View: Toggle Auxiliary Bar (Composer)",
                "View",
                "Ctrl+L",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.reloadWindow",
                "Developer: Reload Window",
                "General",
                "Ctrl+R",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.closeWindow",
                "File: Close Window",
                "File",
                "Alt+F4",
                None,
            ),
            // ── Navigation & Views ──────────────────────────────────────────
            KeybindingItem::new(
                "workbench.view.explorer",
                "View: Show Explorer",
                "View",
                "Ctrl+Shift+E",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.search",
                "View: Show Search",
                "View",
                "Ctrl+Shift+F",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.scm",
                "View: Show Source Control (Git)",
                "View",
                "Ctrl+Shift+G",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.debug",
                "View: Show Run and Debug",
                "View",
                "Ctrl+Shift+D",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.extensions",
                "View: Show Extensions & MCP Store",
                "View",
                "Ctrl+Shift+X",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.outline",
                "View: Show Outline",
                "View",
                "Ctrl+Shift+O",
                None,
            ),
            KeybindingItem::new(
                "workbench.view.testing",
                "View: Show Testing",
                "View",
                "Ctrl+Shift+T",
                None,
            ),
            // ── File Management ─────────────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.files.newUntitledFile",
                "File: New Untitled Text File",
                "File",
                "Ctrl+N",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.files.save",
                "File: Save",
                "File",
                "Ctrl+S",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.files.saveAs",
                "File: Save As...",
                "File",
                "Ctrl+Shift+S",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.files.saveAll",
                "File: Save All",
                "File",
                "Ctrl+K S",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.closeActiveEditor",
                "View: Close Editor",
                "File",
                "Ctrl+W",
                Some("editorFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.reopenClosedEditor",
                "View: Reopen Closed Editor",
                "File",
                "Ctrl+Shift+T",
                None,
            ),
            // ── Editor Navigation & Jumping ─────────────────────────────────
            KeybindingItem::new(
                "workbench.action.gotoLine",
                "Go to Line/Column...",
                "Go",
                "Ctrl+G",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.gotoSymbol",
                "Go to Symbol in Editor...",
                "Go",
                "Ctrl+Shift+O",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.revealDefinition",
                "Go to Definition",
                "Go",
                "F12",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.peekDefinition",
                "Peek Definition",
                "Go",
                "Alt+F12",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.referenceSearch.trigger",
                "Find All References",
                "Go",
                "Shift+F12",
                Some("editorTextFocus"),
            ),
            // ── Editor Text Operations ──────────────────────────────────────
            KeybindingItem::new("undo", "Undo", "Edit", "Ctrl+Z", Some("editorTextFocus")),
            KeybindingItem::new("redo", "Redo", "Edit", "Ctrl+Y", Some("editorTextFocus")),
            KeybindingItem::new(
                "editor.action.clipboardCutAction",
                "Cut",
                "Edit",
                "Ctrl+X",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.clipboardCopyAction",
                "Copy",
                "Edit",
                "Ctrl+C",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.clipboardPasteAction",
                "Paste",
                "Edit",
                "Ctrl+V",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "actions.find",
                "Find in Buffer",
                "Edit",
                "Ctrl+F",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.startFindReplaceAction",
                "Replace in Buffer",
                "Edit",
                "Ctrl+H",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.formatDocument",
                "Format Document",
                "Edit",
                "Shift+Alt+F",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.quickFix",
                "Quick Fix & Code Actions",
                "Edit",
                "Ctrl+.",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.rename",
                "Rename Symbol",
                "Edit",
                "F2",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.commentLine",
                "Toggle Line Comment",
                "Edit",
                "Ctrl+/",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.editor.changeLanguageMode",
                "Change Language Mode",
                "Edit",
                "Ctrl+K M",
                Some("editorTextFocus"),
            ),
            // ── Multi-Cursor & Selection ────────────────────────────────────
            KeybindingItem::new(
                "editor.action.selectAll",
                "Select All",
                "Selection",
                "Ctrl+A",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.addSelectionToNextFindMatch",
                "Add Next Occurrence (Multi-Cursor)",
                "Selection",
                "Ctrl+D",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.selectHighlights",
                "Select All Occurrences",
                "Selection",
                "Ctrl+Shift+L",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "cursorUndo",
                "Undo Last Selection Caret",
                "Selection",
                "Ctrl+U",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.smartSelect.expand",
                "Expand Selection",
                "Selection",
                "Shift+Alt+Right",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "editor.action.smartSelect.shrink",
                "Shrink Selection",
                "Selection",
                "Shift+Alt+Left",
                Some("editorTextFocus"),
            ),
            // ── Layout & Split Editor ───────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.splitEditor",
                "View: Split Editor Right / Layout",
                "View",
                "Ctrl+\\",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.focusFirstEditorGroup",
                "View: Focus First Editor Pane",
                "View",
                "Ctrl+1",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.focusSecondEditorGroup",
                "View: Focus Second Editor Pane",
                "View",
                "Ctrl+2",
                None,
            ),
            KeybindingItem::new(
                "editor.action.toggleWordWrap",
                "View: Toggle Soft Word Wrap",
                "View",
                "Alt+Z",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.toggleMinimap",
                "View: Toggle Editor Minimap",
                "View",
                "Alt+M",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.toggleStickyScroll",
                "View: Toggle Sticky Scroll Headers",
                "View",
                "Alt+S",
                None,
            ),
            KeybindingItem::new(
                "editor.action.toggleBracketColorization",
                "View: Toggle Bracket Colorization",
                "View",
                "Alt+B",
                None,
            ),
            KeybindingItem::new(
                "editor.action.toggleIndentGuides",
                "View: Toggle Indent Guides",
                "View",
                "Alt+I",
                None,
            ),
            // ── Terminal & Shell ────────────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.terminal.new",
                "Terminal: Create New Integrated Terminal",
                "Terminal",
                "Ctrl+Shift+`",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.terminal.toggleTerminal",
                "Terminal: Toggle Terminal",
                "Terminal",
                "Ctrl+`",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.terminal.clear",
                "Terminal: Clear Buffer",
                "Terminal",
                "Ctrl+K",
                Some("terminalFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.terminal.kill",
                "Terminal: Kill Active Terminal Session",
                "Terminal",
                "Ctrl+Shift+K",
                Some("terminalFocus"),
            ),
            // ── Debugging & Execution ───────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.debug.start",
                "Debug: Start / Continue",
                "Debug",
                "F5",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.debug.stepOver",
                "Debug: Step Over",
                "Debug",
                "F10",
                Some("inDebugMode"),
            ),
            KeybindingItem::new(
                "workbench.action.debug.stepInto",
                "Debug: Step Into",
                "Debug",
                "F11",
                Some("inDebugMode"),
            ),
            KeybindingItem::new(
                "workbench.action.debug.stepOut",
                "Debug: Step Out",
                "Debug",
                "Shift+F11",
                Some("inDebugMode"),
            ),
            KeybindingItem::new(
                "workbench.action.debug.stop",
                "Debug: Stop",
                "Debug",
                "Shift+F5",
                Some("inDebugMode"),
            ),
            KeybindingItem::new(
                "workbench.action.debug.restart",
                "Debug: Restart",
                "Debug",
                "Ctrl+Shift+F5",
                Some("inDebugMode"),
            ),
            KeybindingItem::new(
                "editor.debug.action.toggleBreakpoint",
                "Debug: Toggle Breakpoint",
                "Debug",
                "F9",
                Some("editorTextFocus"),
            ),
            // ── AI & Cognitive Studio ───────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.ai.inlineEdit",
                "AI: Inline Code Generation & Edit",
                "AI",
                "Ctrl+I",
                Some("editorTextFocus"),
            ),
            KeybindingItem::new(
                "workbench.action.ai.toggleComposer",
                "AI: Toggle Agent Composer",
                "AI",
                "Ctrl+Alt+A",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.ai.toggleThoughtProcess",
                "AI: Toggle Cognitive Thought HUD",
                "AI",
                "Ctrl+Shift+I",
                None,
            ),
            KeybindingItem::new(
                "workbench.action.ai.toggleInferenceHealth",
                "AI: Toggle Local Inference Health",
                "AI",
                "Ctrl+Alt+H",
                None,
            ),
            // ── Markdown & Preview ──────────────────────────────────────────
            KeybindingItem::new(
                "markdown.showPreview",
                "Markdown: Open Live Preview",
                "Markdown",
                "Ctrl+Shift+V",
                Some("editorLangId == markdown"),
            ),
            KeybindingItem::new(
                "markdown.showPreviewToSide",
                "Markdown: Open Preview to Side",
                "Markdown",
                "Ctrl+K V",
                Some("editorLangId == markdown"),
            ),
            // ── Diff & Compare ──────────────────────────────────────────────
            KeybindingItem::new(
                "workbench.action.compareFiles",
                "File: Compare Active File with Saved",
                "Diff",
                "Ctrl+K D",
                Some("editorFocus"),
            ),
            KeybindingItem::new(
                "editor.action.diffReview.next",
                "Diff: Jump to Next Change Hunk",
                "Diff",
                "Alt+F5",
                Some("isInDiffEditor"),
            ),
            KeybindingItem::new(
                "editor.action.diffReview.prev",
                "Diff: Jump to Previous Change Hunk",
                "Diff",
                "Shift+Alt+F5",
                Some("isInDiffEditor"),
            ),
        ]
    }

    pub fn update_binding(&mut self, command: &str, new_key: &str) {
        let trimmed = new_key.trim();
        for item in &mut self.items {
            if item.command == command {
                if trimmed.is_empty() {
                    item.key = String::new();
                    item.source = KeybindingSource::User;
                    self.custom_overrides
                        .insert(command.to_string(), String::new());
                } else if trimmed == item.default_key {
                    item.key = item.default_key.clone();
                    item.source = KeybindingSource::Default;
                    self.custom_overrides.remove(command);
                } else {
                    item.key = trimmed.to_string();
                    item.source = KeybindingSource::User;
                    self.custom_overrides
                        .insert(command.to_string(), trimmed.to_string());
                }
                break;
            }
        }
    }

    pub fn reset_binding(&mut self, command: &str) {
        for item in &mut self.items {
            if item.command == command {
                item.key = item.default_key.clone();
                item.source = KeybindingSource::Default;
                self.custom_overrides.remove(command);
                break;
            }
        }
    }

    pub fn reset_all(&mut self) {
        self.custom_overrides.clear();
        for item in &mut self.items {
            item.key = item.default_key.clone();
            item.source = KeybindingSource::Default;
        }
        self.editing_command = None;
        self.draft_key.clear();
    }

    pub fn categories(&self) -> Vec<String> {
        let mut cats = vec![
            "All".to_string(),
            "General".to_string(),
            "View".to_string(),
            "File".to_string(),
            "Edit".to_string(),
            "Go".to_string(),
            "Selection".to_string(),
            "Terminal".to_string(),
            "Debug".to_string(),
            "AI".to_string(),
            "Markdown".to_string(),
            "Diff".to_string(),
        ];
        cats.dedup();
        cats
    }

    pub fn filtered_items(&self) -> Vec<&KeybindingItem> {
        let query = self.filter_query.trim().to_lowercase();
        self.items
            .iter()
            .filter(|item| {
                if let Some(cat) = &self.selected_category {
                    if cat != "All" && &item.category != cat {
                        return false;
                    }
                }
                if query.is_empty() {
                    return true;
                }
                item.title.to_lowercase().contains(&query)
                    || item.command.to_lowercase().contains(&query)
                    || item.key.to_lowercase().contains(&query)
                    || item.category.to_lowercase().contains(&query)
                    || item
                        .when
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .collect()
    }
}

pub struct KeybindingsPanel;

impl WorkbenchPanel for KeybindingsPanel {
    fn id(&self) -> &'static str {
        "keybindings"
    }

    fn title(&self) -> &'static str {
        "Keyboard Shortcuts"
    }

    fn icon(&self) -> IconName {
        IconName::Sparkles
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_keybindings_panel(state, cx).into_any_element()
    }
}

impl AuxiliaryTab for KeybindingsPanel {
    fn id(&self) -> &'static str {
        "keybindings"
    }

    fn title(&self) -> &'static str {
        "Shortcuts"
    }

    fn icon(&self) -> IconName {
        IconName::Sparkles
    }

    fn render(&self, state: &HadesNativeState, cx: &mut Context<HadesAppView>) -> AnyElement {
        render_keybindings_panel(state, cx).into_any_element()
    }
}

pub fn render_keybindings_panel(
    state: &HadesNativeState,
    cx: &mut Context<HadesAppView>,
) -> impl IntoElement {
    let theme = &state.theme;
    let filtered_items = state.keybindings.filtered_items();
    let total_count = state.keybindings.items.len();
    let custom_count = state.keybindings.custom_overrides.len();
    let editing_cmd = state.keybindings.editing_command.clone();

    div()
        .flex()
        .flex_col()
        .size_full()
        .bg(theme.bg_sidebar)
        // ── Top Header Bar ───────────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .h(px(40.0))
                .px_4()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_panel)
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .child(ui_icon(IconName::Sparkles, 14.0, theme.accent))
                        .child(
                            div()
                                .text_sm()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.text_primary)
                                .child("KEYBOARD SHORTCUTS"),
                        )
                        .child(
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(theme.bg_raised)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.accent)
                                .child(format!("{total_count} Chords")),
                        )
                        .children((custom_count > 0).then(|| {
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .bg(rgba(0x38bdf822))
                                .border_1()
                                .border_color(theme.accent)
                                .text_xs()
                                .font_weight(FontWeight::BOLD)
                                .text_color(theme.accent)
                                .child(format!("{custom_count} Modified"))
                        })),
                )
                // Action Buttons
                .child(div().flex().flex_row().items_center().gap_2().children(
                    (custom_count > 0).then(|| {
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_1()
                            .px_2()
                            .py_1()
                            .rounded(px(4.0))
                            .bg(theme.bg_raised)
                            .hover(|s| s.bg(theme.bg_hover))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _e, _w, cx| {
                                    this.state.keybindings.reset_all();
                                    this.state.status_message =
                                        "All keyboard shortcuts reset to defaults".to_string();
                                    cx.notify();
                                }),
                            )
                            .child(icon_12(IconName::Undo, theme.text_muted))
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.text_muted)
                                    .child("Reset Defaults"),
                            )
                    }),
                )),
        )
        // ── Search Filter & Category Chips ───────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .p_3()
                .border_b_1()
                .border_color(theme.border_subtle)
                .bg(theme.bg_app)
                // Search Input Box
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_2()
                        .h(px(28.0))
                        .px_3()
                        .bg(theme.bg_input)
                        .border_1()
                        .border_color(theme.border_glass)
                        .rounded(px(4.0))
                        .child(ui_icon(IconName::Search, 12.0, theme.text_subtle))
                        .child(
                            div()
                                .flex_1()
                                .text_xs()
                                .text_color(if state.keybindings.filter_query.is_empty() {
                                    theme.text_subtle
                                } else {
                                    theme.text_primary
                                })
                                .child(if state.keybindings.filter_query.is_empty() {
                                    "Type to search shortcuts (e.g. format, save, ctrl+k)..."
                                        .to_string()
                                } else {
                                    state.keybindings.filter_query.clone()
                                }),
                        )
                        .children((!state.keybindings.filter_query.is_empty()).then(|| {
                            div()
                                .p_0p5()
                                .rounded(px(2.0))
                                .cursor_pointer()
                                .hover(|s| s.bg(theme.bg_hover))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _e, _w, cx| {
                                        this.state.keybindings.filter_query.clear();
                                        cx.notify();
                                    }),
                                )
                                .child(icon_12(IconName::X, theme.text_subtle))
                        })),
                )
                // Category Pills
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .gap_1p5()
                        .overflow_x_hidden()
                        .children(state.keybindings.categories().into_iter().map(|cat| {
                            let is_active = match &state.keybindings.selected_category {
                                Some(c) => c == &cat,
                                None => cat == "All",
                            };
                            let cat_clone = cat.clone();
                            div()
                                .px_2()
                                .py_0p5()
                                .rounded(px(3.0))
                                .cursor_pointer()
                                .bg(if is_active {
                                    theme.accent
                                } else {
                                    theme.bg_raised
                                })
                                .text_xs()
                                .font_weight(if is_active {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::NORMAL
                                })
                                .text_color(if is_active {
                                    rgb(0x0e1017)
                                } else {
                                    theme.text_muted
                                })
                                .hover(|s| if !is_active { s.bg(theme.bg_hover) } else { s })
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _e, _w, cx| {
                                        if cat_clone == "All" {
                                            this.state.keybindings.selected_category = None;
                                        } else {
                                            this.state.keybindings.selected_category =
                                                Some(cat_clone.clone());
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child(cat)
                        })),
                ),
        )
        // ── Table Column Headers ─────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .h(px(26.0))
                .px_4()
                .bg(theme.bg_raised)
                .border_b_1()
                .border_color(theme.border_subtle)
                .text_xs()
                .font_weight(FontWeight::BOLD)
                .text_color(theme.text_subtle)
                .child(div().w(px(240.0)).child("COMMAND"))
                .child(div().w(px(200.0)).child("KEYBINDING"))
                .child(div().w(px(100.0)).child("CATEGORY"))
                .child(div().w(px(90.0)).child("SOURCE"))
                .child(div().flex_1().child("WHEN"))
                .child(div().w(px(80.0)).text_right().child("ACTIONS")),
        )
        // ── Scrollable Shortcut List ─────────────────────────────────────────
        .child(
            div()
                .id("keybindings_table_scroll")
                .flex()
                .flex_col()
                .flex_1()
                .overflow_y_scroll()
                .children(if filtered_items.is_empty() {
                    vec![div()
                        .p_8()
                        .flex()
                        .flex_col()
                        .items_center()
                        .justify_center()
                        .gap_2()
                        .child(ui_icon(IconName::Search, 24.0, theme.text_subtle))
                        .child(
                            div()
                                .text_sm()
                                .text_color(theme.text_muted)
                                .child("No keybindings match the search query"),
                        )
                        .into_any_element()]
                } else {
                    filtered_items
                        .into_iter()
                        .map(|item| {
                            let is_editing = editing_cmd.as_deref() == Some(&item.command);
                            render_keybinding_row(item, is_editing, theme, cx)
                        })
                        .collect()
                }),
        )
}

fn render_keybinding_row(
    item: &KeybindingItem,
    is_editing: bool,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let command_id = item.command.clone();
    let command_title = item.title.clone();
    let category = item.category.clone();
    let key = item.key.clone();
    let is_modified = item.source == KeybindingSource::User;

    let (cat_bg, cat_fg) = match category.as_str() {
        "Edit" => (rgba(0x38bdf820), rgb(0x38bdf8)),
        "File" => (rgba(0xa78bfa20), rgb(0xa78bfa)),
        "View" => (rgba(0x34d39920), rgb(0x34d399)),
        "Go" => (rgba(0xfbbf2420), rgb(0xfbbf24)),
        "Selection" => (rgba(0x60a5fa20), rgb(0x60a5fa)),
        "Terminal" => (rgba(0xf43f5e20), rgb(0xf43f5e)),
        "Debug" => (rgba(0xf8717120), rgb(0xf87171)),
        "AI" => (rgba(0xc084fc20), rgb(0xc084fc)),
        "Markdown" => (rgba(0x4ade8020), rgb(0x4ade80)),
        "Diff" => (rgba(0xfb923c20), rgb(0xfb923c)),
        _ => (rgba(0x94a3b820), rgb(0x94a3b8)),
    };

    if is_editing {
        // ── Inline Editor Row ────────────────────────────────────────────────
        let cmd_for_save = command_id.clone();
        let cmd_for_unbind = command_id.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(36.0))
            .px_4()
            .bg(rgba(0x38bdf812))
            .border_1()
            .border_color(theme.accent)
            .child(
                div()
                    .w(px(240.0))
                    .flex()
                    .flex_col()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(theme.accent)
                            .child(command_title),
                    )
                    .child(
                        div()
                            .text_xs()
                            .font_family("Consolas, monospace")
                            .text_color(theme.text_subtle)
                            .child(command_id),
                    ),
            )
            // Interactive Chord Inputs / Modifiers
            .child(
                div()
                    .w(px(200.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .child(render_modifier_toggle("Ctrl", theme, cx))
                    .child(render_modifier_toggle("Alt", theme, cx))
                    .child(render_modifier_toggle("Shift", theme, cx))
                    .child(
                        div()
                            .px_2()
                            .py_0p5()
                            .rounded(px(3.0))
                            .bg(theme.bg_input)
                            .border_1()
                            .border_color(theme.border_glass)
                            .text_xs()
                            .font_family("Consolas, monospace")
                            .text_color(theme.text_primary)
                            .child(if key.is_empty() {
                                "Press key...".to_string()
                            } else {
                                key.clone()
                            }),
                    ),
            )
            .child(
                div()
                    .w(px(100.0))
                    .child(div().text_xs().text_color(cat_fg).child(category)),
            )
            .child(
                div().w(px(90.0)).child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(2.0))
                        .bg(rgba(0x38bdf820))
                        .text_xs()
                        .text_color(theme.accent)
                        .font_weight(FontWeight::BOLD)
                        .child("Editing..."),
                ),
            )
            .child(
                div().flex_1().child(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child(item.when.clone().unwrap_or_else(|| "—".to_string())),
                ),
            )
            // Action buttons: Save (check), Cancel (cross), Unbind (trash)
            .child(
                div()
                    .w(px(80.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap_1()
                    // Save Button
                    .child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .bg(rgba(0x22c55e20))
                            .hover(|s| s.bg(rgba(0x22c55e40)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    let draft = this.state.keybindings.draft_key.clone();
                                    this.state.keybindings.update_binding(&cmd_for_save, &draft);
                                    this.state.keybindings.editing_command = None;
                                    this.state.status_message =
                                        format!("Keybinding updated for {}", cmd_for_save);
                                    cx.notify();
                                }),
                            )
                            .child(ui_icon(IconName::Check, 12.0, rgb(0x22c55e))),
                    )
                    // Cancel Button
                    .child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.state.keybindings.editing_command = None;
                                    this.state.keybindings.draft_key.clear();
                                    cx.notify();
                                }),
                            )
                            .child(ui_icon(IconName::X, 12.0, theme.text_muted)),
                    )
                    // Unbind Button
                    .child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgba(0xf8717120)))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.state.keybindings.update_binding(&cmd_for_unbind, "");
                                    this.state.keybindings.editing_command = None;
                                    this.state.status_message =
                                        format!("Keybinding unbound for {}", cmd_for_unbind);
                                    cx.notify();
                                }),
                            )
                            .child(ui_icon(IconName::Trash, 12.0, rgb(0xf87171))),
                    ),
            )
            .into_any_element()
    } else {
        // ── Normal Display Row ───────────────────────────────────────────────
        let cmd_for_edit = command_id.clone();
        let cmd_for_reset = command_id.clone();
        let key_for_edit = key.clone();

        div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(28.0))
            .px_4()
            .border_b_1()
            .border_color(rgba(0xffffff08))
            .hover(|s| s.bg(theme.bg_hover))
            // Command Name + ID
            .child(
                div()
                    .w(px(240.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .overflow_hidden()
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme.text_primary)
                            .font_weight(FontWeight::MEDIUM)
                            .overflow_hidden()
                            .child(command_title),
                    ),
            )
            // Keybinding Chords
            .child(
                div()
                    .w(px(200.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_1()
                    .children(if key.is_empty() {
                        vec![div()
                            .text_xs()
                            .text_color(theme.text_subtle)
                            .child("Unbound")
                            .into_any_element()]
                    } else {
                        render_key_chord_badges(&key, theme)
                    }),
            )
            // Category Badge
            .child(
                div().w(px(100.0)).child(
                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded(px(2.0))
                        .bg(cat_bg)
                        .text_xs()
                        .text_color(cat_fg)
                        .font_weight(FontWeight::BOLD)
                        .child(category),
                ),
            )
            // Source Badge (Default vs User)
            .child(
                div().w(px(90.0)).child(
                    div()
                        .text_xs()
                        .text_color(if is_modified {
                            theme.accent
                        } else {
                            theme.text_subtle
                        })
                        .font_weight(if is_modified {
                            FontWeight::BOLD
                        } else {
                            FontWeight::NORMAL
                        })
                        .child(item.source.label()),
                ),
            )
            // When Clause
            .child(
                div()
                    .flex_1()
                    .text_xs()
                    .font_family("Consolas, monospace")
                    .text_color(theme.text_subtle)
                    .overflow_hidden()
                    .child(item.when.clone().unwrap_or_else(|| "—".to_string())),
            )
            // Actions: Edit (Pencil), Reset (Undo if modified)
            .child(
                div()
                    .w(px(80.0))
                    .flex()
                    .flex_row()
                    .items_center()
                    .justify_end()
                    .gap_1()
                    // Edit Button
                    .child(
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.state.keybindings.editing_command =
                                        Some(cmd_for_edit.clone());
                                    this.state.keybindings.draft_key = key_for_edit.clone();
                                    cx.notify();
                                }),
                            )
                            .child(ui_icon(IconName::Pencil, 12.0, theme.text_muted)),
                    )
                    // Reset Button (visible only if modified)
                    .children(is_modified.then(|| {
                        div()
                            .p_1()
                            .rounded(px(3.0))
                            .cursor_pointer()
                            .hover(|s| s.bg(theme.bg_hover))
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(move |this, _e, _w, cx| {
                                    this.state.keybindings.reset_binding(&cmd_for_reset);
                                    this.state.status_message =
                                        format!("Reset {} to default keybinding", cmd_for_reset);
                                    cx.notify();
                                }),
                            )
                            .child(ui_icon(IconName::Undo, 12.0, theme.accent))
                    })),
            )
            .into_any_element()
    }
}

fn render_modifier_toggle(
    name: &'static str,
    theme: &Theme,
    cx: &mut Context<HadesAppView>,
) -> AnyElement {
    let name_str = name.to_string();
    div()
        .px_1p5()
        .py_0p5()
        .rounded(px(3.0))
        .bg(theme.bg_raised)
        .hover(|s| s.bg(theme.bg_hover))
        .border_1()
        .border_color(theme.border_glass)
        .cursor_pointer()
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _e, _w, cx| {
                let cur = this.state.keybindings.draft_key.clone();
                if cur.contains(&name_str) {
                    this.state.keybindings.draft_key = cur
                        .replace(&format!("{}+", name_str), "")
                        .replace(&name_str, "");
                } else if cur.is_empty() {
                    this.state.keybindings.draft_key = name_str.clone();
                } else {
                    this.state.keybindings.draft_key = format!("{}+{}", name_str, cur);
                }
                cx.notify();
            }),
        )
        .text_xs()
        .font_weight(FontWeight::BOLD)
        .text_color(theme.text_primary)
        .child(name)
        .into_any_element()
}

fn render_key_chord_badges(chord: &str, theme: &Theme) -> Vec<AnyElement> {
    // Splits by spaces (for chord sequences like "Ctrl+K Ctrl+S") or shows single keys
    let sequences: Vec<&str> = chord.split(' ').collect();
    let mut elements = Vec::new();

    for (seq_idx, seq) in sequences.iter().enumerate() {
        if seq_idx > 0 {
            elements.push(
                div()
                    .text_xs()
                    .text_color(theme.text_subtle)
                    .child("then")
                    .into_any_element(),
            );
        }

        let parts: Vec<&str> = seq.split('+').collect();
        for (part_idx, part) in parts.iter().enumerate() {
            if part_idx > 0 {
                elements.push(
                    div()
                        .text_xs()
                        .text_color(theme.text_subtle)
                        .child("+")
                        .into_any_element(),
                );
            }

            elements.push(
                div()
                    .px_1p5()
                    .py_0p5()
                    .rounded(px(3.0))
                    .bg(theme.bg_raised)
                    .border_1()
                    .border_color(theme.border_glass)
                    .text_xs()
                    .font_family("Consolas, monospace")
                    .font_weight(FontWeight::BOLD)
                    .text_color(theme.text_primary)
                    .child(part.to_string())
                    .into_any_element(),
            );
        }
    }

    elements
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::prelude::v1::test;

    #[test]
    fn test_keybindings_catalog_length_and_categories() {
        let state = KeybindingsState::new();
        assert!(
            state.items.len() >= 60,
            "Expected at least 60 keybindings in catalog, got {}",
            state.items.len()
        );

        let categories = state.categories();
        assert!(categories.contains(&"General".to_string()));
        assert!(categories.contains(&"Edit".to_string()));
        assert!(categories.contains(&"Debug".to_string()));
        assert!(categories.contains(&"AI".to_string()));
    }

    #[test]
    fn test_keybinding_filter_by_query_and_category() {
        let mut state = KeybindingsState::new();

        state.filter_query = "format".to_string();
        let filtered = state.filtered_items();
        assert!(!filtered.is_empty());
        assert!(filtered
            .iter()
            .any(|k| k.command == "editor.action.formatDocument"));

        state.filter_query = "Ctrl+Shift+P".to_string();
        let filtered_chord = state.filtered_items();
        assert!(!filtered_chord.is_empty());
        assert!(filtered_chord
            .iter()
            .any(|k| k.command == "workbench.action.showCommands"));

        state.filter_query.clear();
        state.selected_category = Some("Debug".to_string());
        let debug_items = state.filtered_items();
        assert!(debug_items.iter().all(|k| k.category == "Debug"));
        assert!(debug_items
            .iter()
            .any(|k| k.command == "workbench.action.debug.start"));
    }

    #[test]
    fn test_keybinding_override_and_reset() {
        let mut state = KeybindingsState::new();

        state.update_binding("workbench.action.files.save", "Ctrl+Shift+Alt+S");
        let item = state
            .items
            .iter()
            .find(|k| k.command == "workbench.action.files.save")
            .unwrap();
        assert_eq!(item.key, "Ctrl+Shift+Alt+S");
        assert_eq!(item.source, KeybindingSource::User);
        assert_eq!(
            state.custom_overrides.get("workbench.action.files.save"),
            Some(&"Ctrl+Shift+Alt+S".to_string())
        );

        // Reset single binding
        state.reset_binding("workbench.action.files.save");
        let item_reset = state
            .items
            .iter()
            .find(|k| k.command == "workbench.action.files.save")
            .unwrap();
        assert_eq!(item_reset.key, "Ctrl+S");
        assert_eq!(item_reset.source, KeybindingSource::Default);
        assert!(!state
            .custom_overrides
            .contains_key("workbench.action.files.save"));

        // Test Unbind
        state.update_binding("workbench.action.files.save", "");
        let item_unbound = state
            .items
            .iter()
            .find(|k| k.command == "workbench.action.files.save")
            .unwrap();
        assert_eq!(item_unbound.key, "");
        assert_eq!(item_unbound.source, KeybindingSource::User);

        // Reset All
        state.reset_all();
        let item_all = state
            .items
            .iter()
            .find(|k| k.command == "workbench.action.files.save")
            .unwrap();
        assert_eq!(item_all.key, "Ctrl+S");
        assert_eq!(item_all.source, KeybindingSource::Default);
        assert!(state.custom_overrides.is_empty());
    }
}
