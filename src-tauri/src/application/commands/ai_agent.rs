use tauri::State;

#[tauri::command]
pub async fn stop_ai_agent(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    state.ai.engine.stop();

    Ok(())
}

#[tauri::command]
pub fn pause_ai_agent(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    state.ai.engine.pause();
    Ok(())
}

#[tauri::command]
pub fn resume_ai_agent(state: State<'_, std::sync::Arc<crate::EditorState>>) -> Result<(), String> {
    state.ai.engine.resume();
    Ok(())
}
