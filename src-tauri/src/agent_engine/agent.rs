use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentConfig {
    pub name: String,
    pub model: String,
    pub provider: String,
    pub capabilities: Vec<String>,
    pub max_concurrency: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentState {
    pub id: String,
    pub config: AgentConfig,
    pub status: AgentStatus,
    pub subagents: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum AgentStatus {
    Idle,
    Working,
    Paused,
    Error(String),
}

pub struct AgentOrchestrator {
    agents: Arc<Mutex<HashMap<String, AgentState>>>,
}

impl AgentOrchestrator {
    pub fn new() -> Self {
        AgentOrchestrator {
            agents: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn create_agent(&self, config: AgentConfig) -> Result<String, String> {
        let id = uuid::Uuid::new_v4().to_string();
        let agent = AgentState {
            id: id.clone(),
            config,
            status: AgentStatus::Idle,
            subagents: Vec::new(),
        };
        self.agents.lock().await.insert(id.clone(), agent);
        Ok(id)
    }

    pub async fn spawn_subagent(&self, agent_id: &str, task: String) -> Result<String, String> {
        // Implementation for spawning subagents
        Ok("subagent_id".to_string())
    }

    pub async fn list_agents(&self) -> Vec<AgentState> {
        self.agents.lock().await.values().cloned().collect()
    }
}