#[derive(Clone, Debug)]
pub struct TerminalSession {
    pub id: String,
    pub name: String,
    pub cwd: String,
    pub lines: Vec<String>,
}

impl TerminalSession {
    pub fn new(id: String, name: String, cwd: String) -> Self {
        Self {
            id,
            name,
            cwd,
            lines: vec![
                "VSCodium-Rust Integrated Terminal (Direct3D 12 Accelerated)".to_string(),
                "PowerShell 7 / Windows ConPTY attached.".to_string(),
                String::new(),
            ],
        }
    }
}
