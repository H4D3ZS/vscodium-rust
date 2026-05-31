// ide.rs — IDE environment detection.
//
// Detects the IDE hosting the current terminal session by inspecting
// well-known environment variables set by each IDE's built-in terminal.

/// Detected IDE environment.
#[derive(Debug, Clone, PartialEq)]
pub enum IdeKind {
    VSCode,
    VSCodeInsiders,
    Cursor,
    Windsurf,
    VSCodium,
    JetBrains,
    Zed,
    Unknown(String),
}

impl IdeKind {
    pub fn display_name(&self) -> &str {
        match self {
            Self::VSCode => "VS Code",
            Self::VSCodeInsiders => "VS Code Insiders",
            Self::Cursor => "Cursor",
            Self::Windsurf => "Windsurf",
            Self::VSCodium => "VSCodium",
            Self::JetBrains => "JetBrains IDE",
            Self::Zed => "Zed",
            Self::Unknown(s) => s,
        }
    }

    /// Install command for the Claurst extension (if known for this IDE).
    pub fn extension_install_command(&self) -> Option<String> {
        match self {
            Self::VSCode => Some("code --install-extension claurst.claurst".to_string()),
            Self::VSCodeInsiders => {
                Some("code-insiders --install-extension claurst.claurst".to_string())
            }
            Self::Cursor => Some("cursor --install-extension claurst.claurst".to_string()),
            Self::Windsurf => Some("windsurf --install-extension claurst.claurst".to_string()),
            Self::VSCodium => Some("codium --install-extension claurst.claurst".to_string()),
            _ => None,
        }
    }
}

/// Detect the currently running IDE from environment variables.
///
/// Returns `Some(IdeKind)` when a known IDE is detected, `None` otherwise.
pub fn detect_ide() -> Option<IdeKind> {
    // TERM_PROGRAM is set by VS Code's integrated terminal (and forks).
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        if term_program.as_str() == "vscode" {
            // Distinguish VS Code forks by checking GIT_ASKPASS for IDE-specific paths.
            if let Ok(askpass) = std::env::var("GIT_ASKPASS") {
                let lower = askpass.to_lowercase();
                if lower.contains("cursor") {
                    return Some(IdeKind::Cursor);
                }
                if lower.contains("windsurf") {
                    return Some(IdeKind::Windsurf);
                }
                if lower.contains("codium") {
                    return Some(IdeKind::VSCodium);
                }
                if lower.contains("code-insiders") {
                    return Some(IdeKind::VSCodeInsiders);
                }
            }
            return Some(IdeKind::VSCode);
        }
    }

    // GIT_ASKPASS alone (without TERM_PROGRAM) can also identify the IDE.
    if let Ok(askpass) = std::env::var("GIT_ASKPASS") {
        let lower = askpass.to_lowercase();
        if lower.contains("cursor") {
            return Some(IdeKind::Cursor);
        }
        if lower.contains("windsurf") {
            return Some(IdeKind::Windsurf);
        }
        if lower.contains("codium") {
            return Some(IdeKind::VSCodium);
        }
        if lower.contains("code-insiders") {
            return Some(IdeKind::VSCodeInsiders);
        }
        if lower.contains("code") {
            return Some(IdeKind::VSCode);
        }
    }

    // VSCODE_INJECTION is set by the VS Code shell integration feature.
    if std::env::var("VSCODE_INJECTION").is_ok() {
        return Some(IdeKind::VSCode);
    }

    // CURSOR_NONCE is a Cursor-specific variable.
    if std::env::var("CURSOR_NONCE").is_ok() {
        return Some(IdeKind::Cursor);
    }

    // ZED_TERM is set by Zed's integrated terminal.
    if std::env::var("ZED_TERM").is_ok() {
        return Some(IdeKind::Zed);
    }

    // JetBrains IDEs set one of these variables in their terminal.
    if std::env::var("IDEA_INITIAL_DIRECTORY").is_ok()
        || std::env::var("__INTELLIJ_COMMAND_HISTFILE__").is_ok()
    {
        return Some(IdeKind::JetBrains);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_names_are_nonempty() {
        let kinds = [
            IdeKind::VSCode,
            IdeKind::VSCodeInsiders,
            IdeKind::Cursor,
            IdeKind::Windsurf,
            IdeKind::VSCodium,
            IdeKind::JetBrains,
            IdeKind::Zed,
            IdeKind::Unknown("TestIDE".to_string()),
        ];
        for k in &kinds {
            assert!(!k.display_name().is_empty());
        }
    }

    #[test]
    fn extension_install_command_for_known_ides() {
        assert!(IdeKind::VSCode.extension_install_command().is_some());
        assert!(IdeKind::Cursor.extension_install_command().is_some());
        assert!(IdeKind::JetBrains.extension_install_command().is_none());
        assert!(IdeKind::Zed.extension_install_command().is_none());
    }
}
