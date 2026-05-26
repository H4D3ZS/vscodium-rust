use tauri::{State, AppHandle, Emitter};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct BackgroundJob {
    pub id: String,
    pub name: String,
    pub progress: u8,
    pub status: String,
}

pub struct JobManager {
    jobs: Arc<Mutex<HashMap<String, BackgroundJob>>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

#[tauri::command]
pub async fn get_background_jobs(state: State<'_, Arc<JobManager>>) -> Result<Vec<BackgroundJob>, String> {
    let jobs = state.jobs.lock().await;
    Ok(jobs.values().cloned().collect())
}

