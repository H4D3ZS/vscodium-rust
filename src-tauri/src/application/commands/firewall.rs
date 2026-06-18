//! Tauri commands for the Semantic Firewall.
//!
//! Exposes firewall validation, bloat analysis, and structural blueprint
//! generation to the frontend IPC layer.

use crate::domain::vcs::patch_engine::PatchBlock;
use crate::semantic_firewall::{FirewallResult, SemanticFirewall};
use crate::code_bloat_enforcer::{CodeBloatEnforcer, BloatResult};
use crate::structural_blueprints::{BlueprintEntry, StructuralBlueprints};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[tauri::command]
pub async fn firewall_validate_proposal(
    file_path: String,
    original_content: String,
    proposed_content: String,
    search_replace_blocks: Vec<PatchBlock>,
    session_id: String,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<FirewallResult, String> {
    let mut firewall = state.services.firewall.lock().await;
    let router = state.editor.lsp_router.clone();

    let result = firewall
        .validate_proposal(
            PathBuf::from(&file_path).as_path(),
            &original_content,
            &proposed_content,
            &search_replace_blocks,
            &session_id,
            &router,
        )
        .await;

    Ok(result)
}

#[tauri::command]
pub async fn firewall_reset_session(
    session_id: String,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<(), String> {
    let mut firewall = state.services.firewall.lock().await;
    firewall.reset_session(&session_id);
    Ok(())
}

#[tauri::command]
pub async fn firewall_iteration_count(
    file_path: String,
    session_id: String,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<u32, String> {
    let firewall = state.services.firewall.lock().await;
    Ok(firewall.iteration_count(Path::new(&file_path), &session_id))
}

#[tauri::command]
pub async fn bloat_analyze_proposal(
    proposed_content: String,
    original_content: String,
    file_extension: String,
    file_path: String,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<BloatResult, String> {
    let enforcer = state.memory.bloat_enforcer.lock().await;
    Ok(enforcer.analyze_proposal(
        &proposed_content,
        &original_content,
        &file_extension,
        &file_path,
    ))
}

#[tauri::command]
pub async fn bloat_load_symbols(
    symbols: Vec<crate::memory_store::SymbolDefinition>,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<(), String> {
    let mut enforcer = state.memory.bloat_enforcer.lock().await;
    enforcer.load_symbols(&symbols);
    Ok(())
}

#[tauri::command]
pub async fn blueprints_generate_project(
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<Vec<BlueprintEntry>, String> {
    Ok(state.memory.blueprints.blueprint_project())
}

#[tauri::command]
pub async fn blueprints_generate_file(
    file_path: String,
    content: String,
    state: tauri::State<'_, Arc<crate::EditorState>>,
) -> Result<Option<BlueprintEntry>, String> {
    Ok(state.memory.blueprints.blueprint_file(
        Path::new(&file_path),
        &content,
    ))
}

#[tauri::command]
pub async fn blueprints_serialize(
    entries: Vec<BlueprintEntry>,
) -> Result<String, String> {
    Ok(StructuralBlueprints::serialize_blueprints(&entries))
}
