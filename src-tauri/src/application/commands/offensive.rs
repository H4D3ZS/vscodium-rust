//! Tauri IPC for the manual offensive tools (Repeater + Intruder).

use crate::intruder::{run, IntruderRequest, IntruderResult};
use crate::repeater::{send, ManualRequest, ManualResponse};

#[tauri::command]
pub async fn repeater_send(request: ManualRequest) -> Result<ManualResponse, String> {
    send(request).await
}

#[tauri::command]
pub async fn intruder_run(request: IntruderRequest) -> Result<IntruderResult, String> {
    run(request).await
}
