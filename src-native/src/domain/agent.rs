use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct ChatToolCall {
    pub name: String,
    pub args: String,
    pub output: Option<String>,
    pub completed: bool,
}

#[derive(Clone, Debug)]
pub struct ChatMessage {
    pub is_user: bool,
    pub text: String,
    pub thoughts: Option<String>,
    pub tool_calls: Vec<ChatToolCall>,
    pub timestamp: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ComposerMode {
    Chat,
    Edit,
    Agent,
    Plan,
}

pub struct AgentTurnState {
    pub is_streaming: Arc<Mutex<bool>>,
    pub streaming_text: Arc<Mutex<String>>,
    pub finished_turn: Arc<Mutex<Option<ChatMessage>>>,
}

impl Default for AgentTurnState {
    fn default() -> Self {
        Self {
            is_streaming: Arc::new(Mutex::new(false)),
            streaming_text: Arc::new(Mutex::new(String::new())),
            finished_turn: Arc::new(Mutex::new(None)),
        }
    }
}
