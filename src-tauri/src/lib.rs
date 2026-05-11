use std::fs;
use tauri::Manager;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

mod hunter;
mod ai_auth;
mod ai_commands;
mod editor_commands;
mod state;
pub use state::EditorState;
pub mod ai_engine;
pub mod ai_tools;
pub mod ane;
pub mod process_ext;
// use crate::process_ext::CommandExtHidden;

pub mod context_quantizer;
pub mod domain;
mod mcp_client;
mod mcp_registry;
pub mod memory_optimizer;
mod memory_store;
mod task_planner;
mod ghost_runtime;
mod context_indexer;
mod patch_engine;
mod tool_invoker;
mod visual_lab;
mod visual_commands;
mod attachment_manager;
mod knowledge_distiller;
mod vision_bridge;
mod hades_vision;
mod workflow_engine;
mod kairos;
mod lsp_commands;
mod terminal_commands;
mod android_commands;
mod vector_commands;
mod file_commands;
mod iphone_emulator;
mod vfs_bridge;
mod mcp_server;
mod memory_layer;
mod hades_harness;
mod airi_bridge;
mod security_distiller;
mod binary_analyzer;
mod kortex_commands;
pub mod kortex_gac;
pub mod kortex_kvcache;
mod extensions_commands;
mod mcp_commands;
mod apex_commands;
mod web_commands;
mod performance_commands;
mod debug_commands;
mod system_commands;
mod ai_project_commands;
mod ai_patch_commands;
mod ai_agent_commands;
mod voice_commands;

// ═══ APEX Intelligence Framework ═══
pub mod apex_red_team;
pub mod apex_orchestrator;
mod lsp;
mod context_key;
mod extension_host;
mod git;
mod performance;
mod keybindings;
mod debug_adapter;
mod emulator_stream;
mod scrcpy;
mod vision;
mod activation;
mod marketplace;
mod browser;
pub mod git_commands;
pub mod specs_db;
mod workers;
mod rules_engine;
mod shadow_workspace;
mod specs_commands;
mod ai_prompts;
mod vector_indexer;
mod git_checkpoints;

#[allow(dead_code)]
extern "system" {
    fn GetCurrentProcess() -> isize;
    fn SetProcessWorkingSetSize(
        hProcess: isize,
        dwMinimumWorkingSetSize: usize,
        dwMaximumWorkingSetSize: usize,
    ) -> i32;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let filter = EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into());

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            app.manage(EditorState::new(app.handle()));
            let iphone_manager = iphone_emulator::IPhoneEmulatorManager::new();
            app.manage(iphone_manager);
            hades_vision::init_hades_vision(
                5,
                "http://localhost:11434",
                "llava-v1.6-mistral-7b",
                "qwen2.5-vl:72b",
                false,
            );
            let state = app.state::<EditorState>();
            let _app_handle = app.handle().clone();
            
            // Re-hide console if on windows and not debug
            #[cfg(all(windows, not(debug_assertions)))]
            {
                use windows::Win32::System::Console::FreeConsole;
                unsafe { let _ = FreeConsole(); }
            }

            // Ensure config dir exists
            if !state.config_dir.exists() {
                fs::create_dir_all(&state.config_dir).ok();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // ═══ AI Auth & API Keys ═══
            ai_auth::get_api_keys,
            ai_auth::save_api_keys,
            ai_auth::save_api_key,
            ai_auth::hunt_api_keys,

            ai_auth::save_ai_session,
            ai_auth::capture_ai_session,
            ai_auth::capture_ai_session_now,
            ai_auth::provider_login_capabilities,

            // ═══ AI Commands ═══
            ai_commands::ai_chat,
            ai_commands::ai_debug_code,
            ai_commands::ai_document_code,
            ai_commands::ai_execute_command,
            ai_commands::ai_explain_code,
            ai_commands::ai_generate_code,
            ai_commands::ai_get_context,
            ai_commands::ai_inline_complete,
            ai_commands::ai_modify_file,
            ai_commands::ai_multi_cursor_edit,
            ai_commands::ai_pr_review,
            ai_commands::ai_refactor_code,
            ai_commands::set_ai_status,
            ai_commands::ai_tool_result,
            ai_commands::airi_broadcast,
            ai_commands::call_tool,
            ai_commands::grep_files,
            ai_commands::propose_file_change,
            ai_commands::set_ollama_url,
            ai_commands::check_ollama_status,
            ai_commands::pull_ollama_model,
            ai_commands::unload_ollama_model,
            ai_commands::get_ollama_ps,
            ai_commands::list_provider_models,
            ai_commands::set_ai_model,
            ai_commands::set_advisor_model,
            ai_commands::get_agent_messages,
            ai_commands::get_brain_telemetry,
            ai_commands::store_message,
            ai_commands::sync_agent_messages,
            ai_commands::list_chat_sessions,
            ai_commands::load_chat_session,
            ai_commands::archive_chat_session,
            ai_commands::create_new_session,
            ai_commands::compress_session_data,


            // ═══ AI Project & Memory ═══
            ai_project_commands::mount_project,
            ai_project_commands::unmount_project,
            ai_project_commands::get_project_memory,
            ai_project_commands::clear_project_memory,

            // ═══ AI Agents ═══
            ai_agent_commands::stop_ai_agent,
            ai_agent_commands::pause_ai_agent,
            ai_agent_commands::resume_ai_agent,

            // ═══ AI Patching ═══
            ai_patch_commands::accept_sentient_patch,
            ai_patch_commands::reject_sentient_patch,

            // ═══ APEX Intelligence ═══
            apex_commands::apex_architect_design,
            apex_commands::apex_architect_scaffold,
            apex_commands::apex_full_sweep,
            apex_commands::apex_get_findings_history,
            apex_commands::apex_get_results_feed,
            apex_commands::apex_multi_system_scan,
            apex_commands::apex_pentest_report,
            apex_commands::apex_perf_optimize,
            apex_commands::apex_perf_profile,
            apex_commands::apex_predict_failures,
            apex_commands::apex_predict_from_logs,
            apex_commands::apex_quick_check,
            apex_commands::apex_red_team_scan,
            apex_commands::apex_security_audit,
            apex_commands::apex_security_explain,
            apex_commands::apex_self_improve,
            apex_commands::apex_set_engine_model,
            apex_commands::apex_simulate_attack,
            apex_commands::apex_threat_anticipate,
            apex_commands::apex_threat_simulate,

            // ═══ Android Commands ═══
            android_commands::adb_list_devices,
            android_commands::adb_list_emulators,
            android_commands::adb_install_and_run,
            android_commands::set_active_device,
            android_commands::get_android_config,
            android_commands::set_android_sdk_path,
            android_commands::spawn_emulator,

            // ═══ Emulator Stream ═══
            emulator_stream::list_available_avds,
            emulator_stream::create_avd,
            emulator_stream::spawn_emulator_by_name,
            emulator_stream::list_running_emulators,
            emulator_stream::start_emulator_stream,
            emulator_stream::stop_emulator_stream,
            emulator_stream::start_emulator_stream,
            emulator_stream::stop_emulator_stream,
            emulator_stream::get_stream_status,

            // ═══ Scrcpy Integration ═══
            scrcpy::spawn_emulator_headless,
            scrcpy::start_scrcpy_stream,
            scrcpy::stop_scrcpy_stream,
            scrcpy::capture_emulator_frame,
            scrcpy::send_emulator_tap,
            scrcpy::send_emulator_swipe,
            scrcpy::send_emulator_text,
            scrcpy::send_emulator_key,
            scrcpy::get_scrcpy_status,

            // ═══ Vision System ═══
            vision::airi_vision_analyze_screen,
            vision::airi_vision_capture_screen,
            hades_vision::hades_vision_get_current_view,
            hades_vision::hades_vision_get_temporal_analysis,
            hades_vision::hades_vision_switch_to_cloud,
            hades_vision::hades_vision_switch_to_local,
            vision_bridge::capture_preview_screenshot,

            // ═══ Editor Commands ═══
            editor_commands::get_settings,
            editor_commands::update_settings,
            editor_commands::resolve_keybinding,
            editor_commands::switch_to_buffer,
            editor_commands::get_highlights,
            editor_commands::set_context_key,
            editor_commands::evaluate_when_clause,
            editor_commands::set_active_root,
            editor_commands::get_active_root,

            extensions_commands::ext_host_init,
            extensions_commands::ext_host_send,

            // ═══ File Commands ═══
            file_commands::open_file,
            file_commands::save_file,
            file_commands::read_file,
            file_commands::write_file,
            file_commands::write_file_content,
            file_commands::create_file,
            file_commands::create_dir,
            file_commands::create_directory,
            file_commands::delete_path,
            file_commands::rename_path,
            file_commands::list_directory,
            file_commands::list_dir_flat,
            file_commands::list_project_files,
            file_commands::get_directory_contents,
            file_commands::get_directory_tree,
            file_commands::editor_get_active_file,
            file_commands::replace_in_files,
            file_commands::glob_files,
            ai_project_commands::update_project_memory,
            file_commands::get_file_tree,
            file_commands::validate_path,

            // ═══ Extensions ═══
            extensions_commands::install_extension,
            extensions_commands::uninstall_extension,
            extensions_commands::get_installed_extensions,
            extensions_commands::get_running_extensions,
            extensions_commands::get_popular_extensions,
            extensions_commands::get_extension_details,
            extensions_commands::install_vsix,
            extensions_commands::get_installed_themes,
            extensions_commands::get_icon_theme_mapping,
            extensions_commands::get_extension_contributions,
            extensions_commands::load_extension_theme,
            extensions_commands::search_extensions,
            extensions_commands::install_extension,
            extensions_commands::uninstall_extension,
            extensions_commands::get_installed_extensions,
            extensions_commands::get_running_extensions,
            extensions_commands::ext_host_init,
            extensions_commands::ext_host_send,
            extensions_commands::refresh_popular_extensions,
            extensions_commands::refresh_installed_extensions,


            // ═══ Git Commands ═══
            git_commands::git_status,
            git_commands::git_diff,
            git_commands::git_diff_file,
            git_commands::git_stage,
            git_commands::git_unstage,
            git_commands::git_commit,
            git_commands::git_revert,
            git_commands::git_stash,
            git_commands::git_stash_pop,
            git_commands::git_clone,
            git_commands::git_blame,
            git_commands::get_git_branch,
            git_commands::get_git_file_hunks,
            git_commands::git_get_history,
            git_commands::git_auto_checkpoint,
            git_commands::git_create_checkpoint,
            git_commands::git_delete_checkpoint,
            git_commands::git_list_checkpoints,
            git_commands::git_get_checkpoint_diff,
            git_commands::git_rollback_checkpoint,

            // ═══ Terminal ═══
            terminal_commands::spawn_terminal,
            terminal_commands::close_terminal,
            terminal_commands::terminal_send_data,
            terminal_commands::resize_terminal,
            terminal_commands::terminal_get_status,
            terminal_commands::terminal_read_output,
            terminal_commands::terminal_terminate,
            terminal_commands::terminal_toggle,
            terminal_commands::get_available_shells,

            // ═══ AI Model Commands ═══
            ai_commands::set_ai_model,
            ai_commands::set_advisor_model,

            // ═══ File Commands ═══
            file_commands::refresh_file_tree,

            // ═══ MCP ═══
            mcp_commands::add_mcp_server,
            mcp_commands::remove_mcp_server,
            mcp_commands::list_mcp_servers,

            // ═══ System ═══
            system_commands::backend_ping,
            system_commands::get_config_path,
            system_commands::open_ai_login,
            system_commands::get_yolo_mode,
            system_commands::set_yolo_mode,
            system_commands::start_mitm_server,
            system_commands::stop_mitm_server,
            system_commands::get_mitm_status,

            // ═══ Performance ═══
            performance_commands::get_system_health,
            performance_commands::get_process_stats,
            performance_commands::benchmark_ane,
            performance_commands::query_performance_history,
            performance_commands::optimize_memory,
            performance_commands::get_memory_savings,

            // ═══ Debug ═══
            debug_commands::debug_start,
            debug_commands::debug_stop,
            debug_commands::debug_send,
            debug_commands::analyze_file_symbols,

            // ═══ Web ═══
            web_commands::web_fetch,
            web_commands::web_search,

            // ═══ Visual Lab ═══
            visual_commands::get_visual_graph,
            visual_commands::get_neural_omni_graph,
            visual_commands::get_all_memory_slots,
            visual_commands::generate_visual_graph,

            // ═══ Voice ═══
            voice_commands::elevenlabs_get_voices,

            // ═══ Workspace ═══
            attachment_manager::select_and_process_attachment,
            kortex_commands::save_kortex_memory,
            kortex_commands::load_kortex_memory,
            kortex_commands::load_kortex_metadata,
            kortex_commands::aim_load_telemetry,
            kortex_commands::aim_append_telemetry,
            kortex_commands::aim_telemetry_snapshot,
            kortex_commands::aim_flush_telemetry,
            kortex_commands::aim_clear_telemetry_samples,
            kortex_commands::aim_set_bound_model,
            kortex_commands::kortex_resolve_ollama_gguf,

            // ═══ Kortex GAC: geometry-aware inference scheduling ═══
            kortex_gac::kortex_gac_profile,
            kortex_gac::kortex_gac_load_profile,
            kortex_gac::kortex_gac_plan,
            kortex_gac::kortex_gac_render_args,
            kortex_gac::kortex_gac_quickplan,
            kortex_gac::kortex_gac_launch,
            kortex_gac::kortex_gac_stop,
            kortex_gac::kortex_gac_status,
            kortex_gac::kortex_gac_default_profile_path,

            // ═══ Kortex KV Cache: ds4-style disk-persistent prefix reuse ═══
            kortex_kvcache::kortex_kvcache_start,
            kortex_kvcache::kortex_kvcache_stop,
            kortex_kvcache::kortex_kvcache_stats,
            kortex_kvcache::kortex_kvcache_status,
            kortex_kvcache::kortex_kvcache_clear,

            
            // ═══ Browser ═══
            browser::browser_open,
            browser::browser_close,
            browser::browser_navigate,
            browser::browser_read_dom,
            browser::browser_type,
            browser::browser_click,
            browser::browser_screenshot,
            browser::browser_get_content_summary,
            browser::browser_capture_vision_context,

            // ═══ Vector Search ═══
            vector_commands::vector_index_codebase,
            vector_commands::vector_search_codebase,
            vector_commands::vector_find_symbol,
            vector_commands::vector_get_file_chunks,
            vector_commands::vector_get_index_stats,

            // ═══ Specs ═══
            specs_commands::cmd_specs_get_projects,
            specs_commands::cmd_specs_create_project,
            specs_commands::cmd_specs_delete_project,
            specs_commands::cmd_specs_get_project_by_name,
            specs_commands::cmd_specs_get_project_files,
            specs_commands::cmd_specs_get_project_tasks,
            specs_commands::cmd_specs_delete_task,
            specs_commands::cmd_specs_retry_task,
            specs_commands::cmd_specs_set_project_provider,
            specs_commands::cmd_specs_generate_layout,
            specs_commands::cmd_specs_get_extended_project_layout,
            specs_commands::cmd_specs_clear_history,

            // ═══ LSP Commands ═══
            lsp_commands::lsp_start,
            lsp_commands::lsp_send_request,
            lsp_commands::lsp_stop,
            lsp_commands::lsp_initialized,
            lsp_commands::lsp_did_open,
            lsp_commands::lsp_did_change,
            lsp_commands::lsp_did_save,
            lsp_commands::lsp_set_workspace,
            lsp_commands::lsp_get_diagnostics,
            lsp_commands::lsp_is_running,
            lsp_commands::lsp_completion,
            lsp_commands::lsp_hover,
            lsp_commands::lsp_goto_definition,
            lsp_commands::lsp_find_references,
            lsp_commands::lsp_rename_symbol,
            lsp_commands::lsp_format_document,
            lsp_commands::lsp_workspace_symbols,
            lsp_commands::lsp_code_lens,
            lsp_commands::lsp_document_symbols,

            // ═══ Extra Commands ═══
            file_commands::open_folder,
            iphone_emulator::launch_iphone_emulator,
            iphone_emulator::stop_iphone_emulator,
            iphone_emulator::is_iphone_emulator_running,
            performance_commands::get_inference_history,
            ai_project_commands::search_project,
            ai_project_commands::query_workspace_memory,
            ai_project_commands::get_file_context,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
