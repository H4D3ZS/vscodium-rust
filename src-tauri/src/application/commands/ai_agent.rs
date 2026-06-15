use tauri::State;
use crate::EditorState;

#[tauri::command]
pub async fn stop_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai.engine.stop();

    Ok(())
}

#[tauri::command]
pub fn pause_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai.engine.pause();
    Ok(())
}

#[tauri::command]
pub fn resume_ai_agent(state: State<'_, EditorState>) -> Result<(), String> {
    state.ai.engine.resume();
    Ok(())
}
