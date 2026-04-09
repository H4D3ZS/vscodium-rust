use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStep {
    pub id: String,
    pub description: String,
    pub status: TaskStatus,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TaskState {
    Planning,
    Executing(usize),
    Verifying(usize),
    SelfHealing {
        step: usize,
        attempt: u8,
        error: String,
    },
    Complete,
    Failed(String),
}

pub struct TaskPlanner {
    state: tokio::sync::Mutex<TaskState>,
    steps: tokio::sync::Mutex<Vec<TaskStep>>,
}

impl TaskPlanner {
    pub fn new() -> Self {
        Self {
            state: tokio::sync::Mutex::new(TaskState::Planning),
            steps: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    pub async fn set_plan(&self, steps: Vec<TaskStep>) {
        let mut s = self.steps.lock().await;
        *s = steps;
        let mut state = self.state.lock().await;
        *state = TaskState::Executing(0);
    }

    pub async fn get_state(&self) -> TaskState {
        self.state.lock().await.clone()
    }

    pub async fn get_steps(&self) -> Vec<TaskStep> {
        self.steps.lock().await.clone()
    }

    pub async fn update_step_status(&self, step_id: &str, status: TaskStatus) {
        let mut steps = self.steps.lock().await;
        if let Some(step) = steps.iter_mut().find(|s| s.id == step_id) {
            step.status = status;
        }
    }

    pub async fn transition_to(&self, next_state: TaskState) {
        let mut state = self.state.lock().await;
        *state = next_state;
    }

    pub async fn current_task_metadata(&self) -> Value {
        let state = self.get_state().await;
        let steps = self.get_steps().await;
        json!({
            "state": state,
            "steps": steps,
            "metadata": "Sentient IDE Task Framework"
        })
    }
}
