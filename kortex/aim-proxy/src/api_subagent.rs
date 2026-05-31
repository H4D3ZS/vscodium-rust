use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::AppState;

#[derive(Deserialize)]
pub struct SubagentRequest {
    pub task: String,
}

#[derive(Serialize)]
pub struct SubagentResponse {
    pub status: String,
    pub task_id: String,
    pub message: String,
}

pub async fn handle_subagent(
    State(state): State<AppState>,
    Json(payload): Json<SubagentRequest>,
) -> impl IntoResponse {
    let task = payload.task.clone();
    let target_ollama = state.target_ollama.clone();
    let http_client = state.http_client.clone();
    let task_id = format!("subagent-{}", uuid::Uuid::new_v4().to_string().chars().take(8).collect::<String>());

    let task_id_clone = task_id.clone();
    let active_subagents = state.active_subagents.clone();

    // Spawn async Tokio task for the subagent
    tokio::spawn(async move {
        {
            let mut count = active_subagents.write().await;
            *count += 1;
        }
        println!("🚀 [AIM-PROXY] Spawning subagent task {}: {}", task_id_clone, task);

        let request_body = json!({
            "model": "neuraldaredevil-8b-ablitared", // The default underlying model
            "messages": [
                {
                    "role": "system",
                    "content": "You are an autonomous subagent. Complete the assigned task efficiently and concisely."
                },
                {
                    "role": "user",
                    "content": task
                }
            ],
            "stream": false
        });

        let target_url = format!("{}/api/chat", target_ollama);
        let resp = http_client.post(&target_url)
            .json(&request_body)
            .send()
            .await;

        match resp {
            Ok(response) => {
                if let Ok(json_response) = response.json::<serde_json::Value>().await {
                    if let Some(msg) = json_response.get("message").and_then(|m| m.get("content")).and_then(|c| c.as_str()) {
                        println!("✅ [AIM-PROXY] Subagent {} completed. Result:\n{}", task_id_clone, msg);
                        // In a full implementation, this could save to an artifact or notify the main agent.
                        // For now, we print it to the proxy logs where the IDE/agent could read it via file,
                        // or we save it to the workspace!
                        let result_path = format!("subagent_{}_result.md", task_id_clone);
                        let _ = std::fs::write(&result_path, msg);
                        println!("💾 [AIM-PROXY] Saved subagent output to {}", result_path);
                    }
                }
            },
            Err(e) => {
                eprintln!("🔴 [AIM-PROXY] Subagent {} failed: {}", task_id_clone, e);
            }
        }
        
        {
            let mut count = active_subagents.write().await;
            if *count > 0 {
                *count -= 1;
            }
        }
    });

    let resp = SubagentResponse {
        status: "accepted".to_string(),
        task_id: task_id.clone(),
        message: format!("Subagent spawned in the background. Output will be saved to subagent_{}_result.md", task_id),
    };

    (StatusCode::ACCEPTED, Json(resp))
}

pub async fn handle_subagent_status(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let count = *state.active_subagents.read().await;
    let resp = json!({
        "active_subagents": count
    });
    (StatusCode::OK, Json(resp))
}
