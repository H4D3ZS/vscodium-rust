//! Auto-approve mode state and opt-in tracking.
//! Mirrors src/utils/autoApprove.ts

use serde::{Deserialize, Serialize};

/// Current auto-approve mode for tool execution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AutoApproveMode {
    /// No auto-approve — all tool calls require confirmation.
    #[default]
    None,
    /// Auto-approve edits to existing files, but not new files or commands.
    AcceptEdits,
    /// Bypass all permissions — approve everything including bash commands.
    BypassPermissions,
    /// Auto-approve with plan mode — shows plan before execution.
    Plan,
}

impl AutoApproveMode {
    /// True if this mode auto-approves bash/shell command execution.
    pub fn auto_approves_bash(&self) -> bool {
        matches!(self, Self::BypassPermissions)
    }

    /// True if this mode auto-approves file edits.
    pub fn auto_approves_edits(&self) -> bool {
        matches!(self, Self::AcceptEdits | Self::BypassPermissions)
    }

    /// True if this mode shows a plan before tool execution.
    pub fn is_plan_mode(&self) -> bool {
        matches!(self, Self::Plan)
    }

    /// Short display label for status line.
    pub fn label(&self) -> &'static str {
        match self {
            Self::None => "",
            Self::AcceptEdits => "auto-edit",
            Self::BypassPermissions => "bypass",
            Self::Plan => "plan-mode",
        }
    }
}

/// Opt-in state: tracks whether the user has explicitly enabled auto-approve
/// and which dialog/warning was shown.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutoModeState {
    pub mode: AutoApproveMode,
    /// True if the user saw and accepted the risk warning dialog.
    pub warning_accepted: bool,
    /// Session ID when bypass mode was activated.
    pub activated_session: Option<String>,
    /// Turn number when bypass mode was activated.
    pub activated_turn: Option<u32>,
}

impl AutoModeState {
    pub fn new(mode: AutoApproveMode) -> Self {
        Self {
            mode,
            warning_accepted: false,
            activated_session: None,
            activated_turn: None,
        }
    }

    /// Activate bypass mode; requires warning to have been accepted.
    pub fn activate_bypass(&mut self, session_id: &str, turn: u32) {
        self.mode = AutoApproveMode::BypassPermissions;
        self.warning_accepted = true;
        self.activated_session = Some(session_id.to_string());
        self.activated_turn = Some(turn);
    }

    /// Reset to no auto-approve.
    pub fn reset(&mut self) {
        self.mode = AutoApproveMode::None;
        self.warning_accepted = false;
    }
}
