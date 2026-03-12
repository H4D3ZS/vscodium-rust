use serde_json::{json, Value};

pub struct TaskPlanner {
    // Internal task state
}

impl TaskPlanner {
    pub fn new() -> Self {
        Self {}
    }

    pub fn current_task_metadata(&self) -> Value {
        json!({"current_task": "analyzing project and fulfilling user request"})
    }
}
