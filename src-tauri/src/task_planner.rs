use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

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
        // Activate the first step so the live plan UI shows work starting.
        if let Some(first) = s.first_mut() {
            first.status = TaskStatus::InProgress;
        }
        let mut state = self.state.lock().await;
        *state = TaskState::Executing(0);
    }

    /// Mark the current in-progress step Completed and activate the next Pending
    /// step. Returns the new active step index, or `None` when the plan is fully
    /// complete. Drives the live plan-progress UI as the executor makes headway.
    pub async fn advance_step(&self) -> Option<usize> {
        let mut steps = self.steps.lock().await;
        if steps.is_empty() {
            return None;
        }
        // Complete whatever is currently active (in-progress, else first pending).
        let cur = steps.iter().position(|s| matches!(s.status, TaskStatus::InProgress))
            .or_else(|| steps.iter().position(|s| matches!(s.status, TaskStatus::Pending)));
        if let Some(i) = cur {
            steps[i].status = TaskStatus::Completed;
        }
        // Activate the next still-pending step, if any.
        let next = steps.iter().position(|s| matches!(s.status, TaskStatus::Pending));
        let mut state = self.state.lock().await;
        match next {
            Some(n) => {
                steps[n].status = TaskStatus::InProgress;
                *state = TaskState::Executing(n);
                Some(n)
            }
            None => {
                *state = TaskState::Complete;
                None
            }
        }
    }

    pub async fn get_state(&self) -> TaskState {
        self.state.lock().await.clone()
    }

    pub async fn get_steps(&self) -> Vec<TaskStep> {
        self.steps.lock().await.clone()
    }

    pub async fn _update_step_status(&self, step_id: &str, status: TaskStatus) {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn step(id: &str) -> TaskStep {
        TaskStep {
            id: id.into(),
            description: format!("step {}", id),
            status: TaskStatus::Pending,
            dependencies: vec![],
        }
    }

    #[tokio::test]
    async fn set_plan_activates_first_step() {
        let p = TaskPlanner::new();
        p.set_plan(vec![step("1"), step("2")]).await;
        let steps = p.get_steps().await;
        assert!(matches!(steps[0].status, TaskStatus::InProgress));
        assert!(matches!(steps[1].status, TaskStatus::Pending));
        assert!(matches!(p.get_state().await, TaskState::Executing(0)));
    }

    #[tokio::test]
    async fn advance_step_walks_then_completes_plan() {
        let p = TaskPlanner::new();
        p.set_plan(vec![step("1"), step("2")]).await;
        // step 0 -> step 1
        assert_eq!(p.advance_step().await, Some(1));
        let steps = p.get_steps().await;
        assert!(matches!(steps[0].status, TaskStatus::Completed));
        assert!(matches!(steps[1].status, TaskStatus::InProgress));
        // step 1 -> done
        assert_eq!(p.advance_step().await, None);
        let steps = p.get_steps().await;
        assert!(matches!(steps[1].status, TaskStatus::Completed));
        assert!(matches!(p.get_state().await, TaskState::Complete));
    }

    #[tokio::test]
    async fn advance_on_empty_plan_is_safe() {
        let p = TaskPlanner::new();
        assert_eq!(p.advance_step().await, None);
    }
}
