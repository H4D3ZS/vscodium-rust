use tauri::State;
use crate::EditorState;

#[tauri::command]
pub async fn accept_sentient_patch(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut engine = state.patch_engine.lock().await;
    engine.commit_shadow(std::path::Path::new(&path)).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reject_sentient_patch(state: State<'_, EditorState>, path: String) -> Result<(), String> {
    let mut engine = state.patch_engine.lock().await;
    engine.discard_shadow(std::path::Path::new(&path));
    Ok(())
}

