use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ChatToolCall {
    pub name: String,
    pub args: String,
    pub output: Option<String>,
    pub completed: bool,
    pub duration_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentStep {
    pub label: String,
    pub status: String, // "running", "completed", "failed"
    pub duration_ms: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentArtifact {
    pub title: String,
    pub path: String,
    pub kind: String, // "diff", "file", "walkthrough", "plan"
    pub diff_content: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachedContext {
    pub kind: String, // "codebase", "docs", "web", "file"
    pub id: String,
    pub name: String,
    pub path: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AgentThread {
    pub id: String,
    pub title: String,
    pub created_at: String,
    pub messages: Vec<ChatMessage>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TokenMetrics {
    pub predicted_per_second: Option<f64>,
    pub predicted_tokens: Option<usize>,
    pub predicted_ms: Option<f64>,
    pub prompt_per_second: Option<f64>,
    pub prompt_ms: Option<f64>,
    pub cache_n: Option<usize>,
    pub draft_tokens: Option<usize>,
    pub draft_accepted: Option<usize>,
    pub mtp_acceptance_rate: Option<f64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub is_user: bool,
    pub text: String,
    pub thoughts: Option<String>,
    pub thought_duration_ms: Option<u64>,
    pub tool_calls: Vec<ChatToolCall>,
    pub steps: Vec<AgentStep>,
    pub artifacts: Vec<AgentArtifact>,
    pub context: Vec<AttachedContext>,
    pub checkpoint_id: Option<String>,
    pub checkpoint_description: Option<String>,
    pub is_subagent_response: bool,
    pub timestamp: String,
    pub token_metrics: Option<TokenMetrics>,
}

impl ChatMessage {
    pub fn user(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::User,
            is_user: true,
            text: text.into(),
            thoughts: None,
            thought_duration_ms: None,
            tool_calls: Vec::new(),
            steps: Vec::new(),
            artifacts: Vec::new(),
            context: Vec::new(),
            checkpoint_id: None,
            checkpoint_description: None,
            is_subagent_response: false,
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            token_metrics: None,
        }
    }

    pub fn assistant(text: impl Into<String>) -> Self {
        Self {
            role: MessageRole::Assistant,
            is_user: false,
            text: text.into(),
            thoughts: None,
            thought_duration_ms: None,
            tool_calls: Vec::new(),
            steps: Vec::new(),
            artifacts: Vec::new(),
            context: Vec::new(),
            checkpoint_id: None,
            checkpoint_description: None,
            is_subagent_response: false,
            timestamp: chrono::Local::now().format("%H:%M:%S").to_string(),
            token_metrics: None,
        }
    }

    pub fn with_thoughts(mut self, thoughts: impl Into<String>, duration_ms: u64) -> Self {
        self.thoughts = Some(thoughts.into());
        self.thought_duration_ms = Some(duration_ms);
        self
    }

    pub fn with_context(mut self, ctx: AttachedContext) -> Self {
        self.context.push(ctx);
        self
    }

    pub fn with_checkpoint(mut self, cp: impl Into<String>, desc: impl Into<String>) -> Self {
        self.checkpoint_id = Some(cp.into());
        self.checkpoint_description = Some(desc.into());
        self
    }

    pub fn with_artifact(mut self, artifact: AgentArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }
}

#[derive(Clone, Default)]
pub struct AiTurnState {
    pub is_running: bool,
    pub streamed_content: Arc<Mutex<String>>,
    pub streamed_thoughts: Arc<Mutex<String>>,
    pub tool_calls: Arc<Mutex<Vec<ChatToolCall>>>,
    pub finished_turn: Arc<Mutex<Option<ChatMessage>>>,
    pub live_tokens_count: Arc<Mutex<usize>>,
    pub live_tokens_per_sec: Arc<Mutex<f64>>,
    pub live_elapsed_secs: Arc<Mutex<f64>>,
    pub live_mtp_rate: Arc<Mutex<Option<f64>>>,
    pub final_metrics: Arc<Mutex<Option<TokenMetrics>>>,
}

impl AiTurnState {
    pub fn reset(&mut self) {
        self.is_running = true;
        if let Ok(mut sc) = self.streamed_content.lock() {
            sc.clear();
        }
        if let Ok(mut st) = self.streamed_thoughts.lock() {
            st.clear();
        }
        if let Ok(mut tc) = self.tool_calls.lock() {
            tc.clear();
        }
        if let Ok(mut fin) = self.finished_turn.lock() {
            *fin = None;
        }
        if let Ok(mut ltc) = self.live_tokens_count.lock() {
            *ltc = 0;
        }
        if let Ok(mut tps) = self.live_tokens_per_sec.lock() {
            *tps = 0.0;
        }
        if let Ok(mut el) = self.live_elapsed_secs.lock() {
            *el = 0.0;
        }
        if let Ok(mut mtp) = self.live_mtp_rate.lock() {
            *mtp = None;
        }
        if let Ok(mut fm) = self.final_metrics.lock() {
            *fm = None;
        }
    }
}
