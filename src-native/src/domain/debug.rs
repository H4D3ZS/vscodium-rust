#[derive(Clone, Debug)]
pub struct Breakpoint {
    pub file: String,
    pub line: usize,
    pub verified: bool,
}

#[derive(Clone, Debug)]
pub struct DebugStore {
    pub active_target: String,
    pub breakpoints: Vec<Breakpoint>,
    pub is_running: bool,
    pub is_paused: bool,
}

impl Default for DebugStore {
    fn default() -> Self {
        Self {
            active_target: "vscodium-rust".to_string(),
            breakpoints: vec![
                Breakpoint {
                    file: "main.rs".to_string(),
                    line: 142,
                    verified: true,
                },
                Breakpoint {
                    file: "app_state.rs".to_string(),
                    line: 88,
                    verified: true,
                },
            ],
            is_running: false,
            is_paused: false,
        }
    }
}
