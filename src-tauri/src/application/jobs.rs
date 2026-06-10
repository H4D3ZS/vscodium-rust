use tauri::State;
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
    pub jobs: Arc<Mutex<HashMap<String, BackgroundJob>>>,
}

impl JobManager {
    pub fn new() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn register(&self, id: String, name: String) {
        let mut jobs = self.jobs.lock().await;
        jobs.insert(
            id.clone(),
            BackgroundJob {
                id,
                name,
                progress: 0,
                status: "running".to_string(),
            },
        );
    }

    pub async fn update(&self, id: &str, progress: u8, status: &str) {
        let mut jobs = self.jobs.lock().await;
        if let Some(j) = jobs.get_mut(id) {
            j.progress = progress;
            j.status = status.to_string();
        }
    }

    pub async fn complete(&self, id: &str, ok: bool, status: &str) {
        let mut jobs = self.jobs.lock().await;
        if let Some(j) = jobs.get_mut(id) {
            j.progress = if ok { 100 } else { j.progress };
            j.status = status.to_string();
        }
    }

    pub async fn remove(&self, id: &str) {
        let mut jobs = self.jobs.lock().await;
        jobs.remove(id);
    }
}

#[tauri::command]
pub async fn get_background_jobs(state: State<'_, Arc<JobManager>>) -> Result<Vec<BackgroundJob>, String> {
    let jobs = state.jobs.lock().await;
    Ok(jobs.values().cloned().collect())
}

#[tauri::command]
pub async fn register_background_job(
    state: State<'_, Arc<JobManager>>,
    id: String,
    name: String,
) -> Result<(), String> {
    state.register(id, name).await;
    Ok(())
}

#[tauri::command]
pub async fn update_background_job(
    state: State<'_, Arc<JobManager>>,
    id: String,
    progress: u8,
    status: String,
) -> Result<(), String> {
    state.update(&id, progress, &status).await;
    Ok(())
}

#[tauri::command]
pub async fn remove_background_job(
    state: State<'_, Arc<JobManager>>,
    id: String,
) -> Result<(), String> {
    state.remove(&id).await;
    Ok(())
}
