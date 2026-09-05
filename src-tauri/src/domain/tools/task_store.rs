//! In-memory task store with disk persistence. Provides CRUD for agent tasks.
//! Tasks are stored in `tasks.json` at the project root.

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub parent_id: Option<String>,
    pub created_at: u64,
    pub updated_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
}

impl TaskStatus {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "in_progress" | "in-progress" | "active" => TaskStatus::InProgress,
            "done" | "complete" | "completed" => TaskStatus::Done,
            "blocked" | "waiting" => TaskStatus::Blocked,
            _ => TaskStatus::Todo,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskStore {
    tasks: HashMap<String, Task>,
}

impl TaskStore {
    pub fn new() -> Self {
        Self { tasks: HashMap::new() }
    }

    pub fn load(path: &std::path::Path) -> Self {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Ok(store) = serde_json::from_str::<TaskStore>(&content) {
                return store;
            }
        }
        Self::new()
    }

    pub fn save(&self, path: &std::path::Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn create(&mut self, title: &str, description: &str, parent_id: Option<&str>) -> Task {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let id = format!("task_{}", now * 1000 + self.tasks.len() as u64);
        let task = Task {
            id: id.clone(),
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Todo,
            parent_id: parent_id.map(|s| s.to_string()),
            created_at: now,
            updated_at: now,
        };
        self.tasks.insert(id.clone(), task.clone());

        // Cap at 100 tasks — evict done tasks first
        const MAX_TASKS: usize = 100;
        if self.tasks.len() > MAX_TASKS {
            let mut done_ids: Vec<String> = self.tasks.iter()
                .filter(|(_, t)| t.status == TaskStatus::Done)
                .map(|(k, _)| k.clone())
                .collect();
            let to_remove = self.tasks.len() - MAX_TASKS;
            for id in done_ids.drain(..to_remove.min(done_ids.len())) {
                self.tasks.remove(&id);
            }
            // If still over cap, remove oldest tasks
            if self.tasks.len() > MAX_TASKS {
                let mut oldest: Vec<(String, u64)> = self.tasks.iter()
                    .map(|(k, t)| (k.clone(), t.created_at))
                    .collect();
                oldest.sort_by_key(|(_, ts)| *ts);
                for (id, _) in oldest.drain(..self.tasks.len() - MAX_TASKS) {
                    self.tasks.remove(&id);
                }
            }
        }

        task
    }

    pub fn update(&mut self, id: &str, status: Option<&str>, description: Option<&str>) -> Result<Task> {
        let task = self.tasks.get_mut(id).ok_or_else(|| anyhow!("Task not found: {id}"))?;
        if let Some(s) = status {
            task.status = TaskStatus::from_str(s);
        }
        if let Some(d) = description {
            task.description = d.to_string();
        }
        task.updated_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Ok(task.clone())
    }

    pub fn get(&self, id: &str) -> Option<&Task> {
        self.tasks.get(id)
    }

    pub fn list(&self, status_filter: Option<&str>) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| {
                if let Some(s) = status_filter {
                    t.status == TaskStatus::from_str(s)
                } else {
                    true
                }
            })
            .collect()
    }

    pub fn task_to_value(task: &Task) -> Value {
        json!({
            "id": task.id,
            "title": task.title,
            "description": task.description,
            "status": match task.status {
                TaskStatus::Todo => "todo",
                TaskStatus::InProgress => "in_progress",
                TaskStatus::Done => "done",
                TaskStatus::Blocked => "blocked",
            },
            "parent_id": task.parent_id,
            "created_at": task.created_at,
            "updated_at": task.updated_at,
        })
    }
}
