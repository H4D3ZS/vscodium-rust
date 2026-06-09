use std::fs;
use tauri::Manager;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub mod aim_store;
pub mod account;
pub mod enterprise_audit;
pub mod enterprise_governance;
pub mod auth;
mod ai_auth;
mod ai_commands;
mod editor_commands;
mod hunter;
mod state;
mod window_commands;
pub use state::EditorState;
pub mod ai_engine;
pub mod ai_tools;
pub mod tool_aliases;
pub mod ane;
pub mod ane_inference;
mod ane_commands;
pub mod model_manager;
mod model_commands;
pub mod memory_offload;
pub mod optimized_inference;
mod inference_commands;
pub mod auth_commands;
pub mod browser_actuation;
pub mod process_ext;
// use crate::process_ext::CommandExtHidden;

mod ai_agent_commands;
mod ai_patch_commands;
mod antigravity_commands;
mod antigravity_compat;
mod ai_project_commands;
mod airi_bridge;
mod claurst_bridge;
mod remote_commands;
mod ripgrep_search;
mod hermes_gateway;
mod ann_index;
mod streaming_tool_executor;
mod ide_shell;
mod hermes_skills;
mod skill_audit;
mod skill_store;
mod android_commands;
mod android_sdk;
mod gradle_commands;
mod logcat_commands;
mod logcat_service;
mod test_commands;
mod test_runner_service;
mod pytorch_commands;
mod ml_studio;
mod stop_hooks;
mod embeddings;
mod apex_commands;
mod attachment_manager;
mod vision_sidecar;
mod binary_analyzer;
mod context_indexer;
pub mod context_quantizer;
mod debug_commands;
mod image_gen;
mod symbols;
pub mod domain;
mod extensions_commands;
mod file_commands;
mod ghost_runtime;
mod agent_harness;
mod hades_harness;
mod hades_vision;
mod iphone_emulator;
mod ios_stream;
mod ios_sim_native;
mod ios_sim_embed;
mod ios_simulator;
mod kairos;
mod knowledge_distiller;
mod kortex_commands;
pub mod kortex_gac;
pub mod kortex_kvcache;
mod lsp_commands;
mod lsp_manager;
mod lsp_bundle;
mod lsp_catalog;
mod lsp_store;
mod lsp_router;
mod workspace_settings_commands;
mod port_commands;
mod mcp_client;
mod mcp_commands;
mod mcp_registry;
mod mcp_resolver;
mod mobile_toolchain;
mod mcp_server;
mod memory_layer;
pub mod memory_optimizer;
mod memory_store;
mod patch_engine;
mod performance_commands;
mod security_distiller;
mod security_generators;
mod security_generator_commands;
mod system_commands;
mod task_planner;
mod terminal_commands;
mod tool_invoker;
mod vector_commands;
mod vfs_bridge;
mod vision_bridge;
mod visual_commands;
mod visual_lab;
mod voice_commands;
mod web_commands;
mod workflow_engine;
pub mod jobs;

// ═══ APEX Intelligence Framework ═══
mod activation;
mod ai_prompts;
pub mod apex_orchestrator;
pub mod apex_red_team;
pub mod pentest_scope;
pub mod pentest_report;
pub mod pentest_executor;
pub mod sec_distro;
mod browser;
mod context_key;
mod debug_adapter;
mod emulator_stream;
mod extension_host;
mod git;
mod git_checkpoints;
pub mod git_commands;
mod keybindings;
mod lsp;
mod marketplace;
mod architecture;
mod performance;
mod rules_engine;
mod cursor_compat;
mod cursor_commands;
mod workspace_compat;
mod workspace_commands;
mod scrcpy;
mod shadow_workspace;
mod specs_commands;
pub mod specs_db;
mod vector_indexer;
mod vision;
mod workers;

#[cfg(target_os = "windows")]
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
            app.manage(std::sync::Arc::new(jobs::JobManager::new()));
            let iphone_manager = iphone_emulator::IPhoneEmulatorManager::new();
            app.manage(iphone_manager);
            let state = app.state::<EditorState>();
            let _app_handle = app.handle().clone();

            // Re-hide console if on windows and not debug
            #[cfg(all(windows, not(debug_assertions)))]
            {
                use windows::Win32::System::Console::FreeConsole;
                unsafe {
                    let _ = FreeConsole();
                }
            }

            // Ensure config dir exists
            if !state.config_dir.exists() {
                fs::create_dir_all(&state.config_dir).ok();
            }

            // Bundle PortableGit + ripgrep into %LOCALAPPDATA%\\HADES\\ on first launch.
            tauri::async_runtime::spawn(async {
                match tauri::async_runtime::spawn_blocking(ide_shell::ensure_portable_git_installed).await {
                    Ok(Ok(true)) => println!("[ide_shell] PortableGit installed to HADES home."),
                    Ok(Ok(false)) => {}
                    Ok(Err(e)) => eprintln!("[ide_shell] PortableGit install: {e}"),
                    Err(e) => eprintln!("[ide_shell] ensure_portable_git task failed: {e}"),
                }
                match tauri::async_runtime::spawn_blocking(ide_shell::ensure_ripgrep_installed).await {
                    Ok(Ok(true)) => println!("[ide_shell] ripgrep installed to HADES home."),
                    Ok(Ok(false)) => {}
                    Ok(Err(e)) => eprintln!("[ide_shell] ripgrep install: {e}"),
                    Err(e) => eprintln!("[ide_shell] ensure_ripgrep task failed: {e}"),
                }
            });

            // ChatGPT bridge: lazy-init on first use — a hidden webview costs ~40–80MB RSS.

            // ANE warms on first inference / deferred offline stack — not at boot (saves RSS).

            // Start background OAuth listener
            let oauth_app_handle = _app_handle.clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = auth_commands::start_oauth_listener(14285, oauth_app_handle).await {
                    eprintln!("Failed to start OAuth listener: {}", e);
                }
            });

            // Initial working set trim on Windows — drops paged-out memory immediately.
            #[cfg(target_os = "windows")]
            unsafe {
                let handle = GetCurrentProcess();
                let _ = SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
            }

            // Periodic working set trim: reclaim unused pages every 5 minutes.
            // After heavy indexing or long agent loops, Rust may hold large amounts
            // of paged-out heap. This forces Windows to reclaim those pages.
            #[cfg(target_os = "windows")]
            tauri::async_runtime::spawn(async {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(300)).await;
                    unsafe {
                        let handle = GetCurrentProcess();
                        let _ = SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
                    }
                }
            });

            // Dev builds: native fullscreen (hides macOS Dock / uses full display).
            // Release builds keep the normal resizable window from tauri.conf.json.
            #[cfg(debug_assertions)]
            if let Some(w) = app.get_webview_window("main") {
                if let Err(e) = w.set_fullscreen(true) {
                    eprintln!("[dev] set_fullscreen failed: {e}");
                }
            }

            // Memory watchdog — soft trim ~90MB, hard trim ~150MB (lean idle target 54MB when panels closed).
            let perf_monitor = state.perf_monitor.clone();
            let engine_for_trim = state.ai_engine.clone();
            let mem_opt = state.memory_optimizer.clone();
            tauri::async_runtime::spawn(async move {
                loop {
                    tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                    if let Some(stats) = perf_monitor.get_stats().await {
                        let mb = stats.memory_mb;
                        if mb > 150 {
                            println!("[Memory Watchdog] RSS={mb}MB — hard trim");
                            let _ = engine_for_trim.optimize_memory().await;
                            let _ = mem_opt.optimize().await;
                        } else if mb > 90 {
                            let _ = mem_opt.optimize().await;
                        }
                        #[cfg(target_os = "windows")]
                        if mb > 90 {
                            unsafe {
                                let handle = GetCurrentProcess();
                                let _ = SetProcessWorkingSetSize(handle, usize::MAX, usize::MAX);
                            }
                        }
                        #[cfg(target_os = "macos")]
                        if mb > 90 {
                            crate::performance_commands::macos_pressure_relief();
                        }
                    }
                }
            });

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
            auth_commands::start_webui_login,
            auth_commands::list_webui_sessions,
            auth_commands::switch_webui_session,
            auth_commands::delete_webui_session,
            auth_commands::check_login_status,
            auth_commands::get_stored_token,
            auth_commands::send_webui_prompt,
            auth_commands::save_webui_response,
            auth_commands::webui_agent_run,
            auth_commands::toggle_webui_window_visibility,
            // ═══ Auth (Supabase) ═══
            auth::auth_sign_up,
            auth::auth_sign_in,
            auth::auth_session,
            auth::auth_sign_out,
            // ═══ Account / Subscription / ToS ═══
            account::account_get,
            account::account_accept_tos,
            account::account_tos_status,
            account::account_has_feature,
            account::account_has_feature_offline,
            account::account_set_tier,
            account::account_acquire_addon,
            account::account_subscribe,
            account::account_sync,
            account::account_check_and_count,
            account::account_usage,
            account::account_add_tokens,
            account::account_start_trial,
            account::account_open_billing,
            // ═══ Enterprise (audit + org policy) ═══
            enterprise_audit::enterprise_get_policy,
            enterprise_audit::enterprise_set_policy,
            enterprise_audit::enterprise_audit_list,
            enterprise_audit::enterprise_audit_export,
            enterprise_audit::enterprise_audit_log,
            enterprise_audit::enterprise_seed_cyber_policy,
            enterprise_audit::enterprise_init_engagement,
            enterprise_audit::enterprise_export_sarif,
            // ═══ Security Arsenal (Obsidian-style generators) ═══
            security_generator_commands::security_reverse_shell,
            security_generator_commands::security_listener,
            security_generator_commands::security_csp_analyze,
            security_generator_commands::security_shellcode_recipe,
            security_generator_commands::security_encode_payload,
            // ═══ AI Commands ═══
            ai_commands::ai_chat,
            ai_commands::aim_inspect,
            ai_commands::ai_chat_fast,
            ai_commands::ai_chat_oneshot,
            ai_commands::ai_debug_code,
            ai_commands::ai_document_code,
            ai_commands::ai_execute_command,
            ai_commands::ai_explain_code,
            ai_commands::ai_generate_code,
            ai_commands::ai_get_context,
            ai_commands::ai_inline_complete,
            ai_commands::predict_next_edit,
            claurst_bridge::claurst_status,
            claurst_bridge::claurst_run,
            ide_shell::ide_shell_status,
            ide_shell::ide_git_bash_path,
            ide_shell::ide_ensure_portable_git,
            ide_shell::ide_ensure_ripgrep,
            hermes_skills::hermes_integration_status,
            hermes_skills::hermes_skills_list,
            hermes_skills::hermes_skills_get,
            hermes_skills::hermes_skills_search,
            skill_store::skill_store_status,
            skill_store::skill_store_list,
            skill_store::skill_store_install,
            skill_store::skill_store_uninstall,
            skill_store::skill_store_audit,
            skill_store::skill_store_refresh,
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
            ai_commands::preview_search_replace,
            ai_commands::set_ollama_url,
            ai_commands::list_workspace_rules,
            ai_commands::reindex_workspace,
            ai_commands::check_ollama_status,
            ai_commands::pull_ollama_model,
            ai_commands::unload_ollama_model,
            ai_commands::get_ollama_ps,
            ai_commands::diagnose_ollama,
            ai_commands::list_provider_models,
            ai_commands::search_ollama_library,
            ai_commands::ollama_native_get,
            ai_commands::ollama_native_post,
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
            ai_commands::agent_activity_drain,
            cursor_commands::cursor_scan_project,
            cursor_commands::cursor_init_project,
            cursor_commands::cursor_append_debug_log,
            cursor_commands::cursor_list_worktrees,
            cursor_commands::cursor_create_worktree,
            cursor_commands::cursor_reload_workspace,
            workspace_commands::workspace_scan,
            workspace_commands::workspace_init,
            workspace_commands::workspace_reload,
            workspace_commands::workspace_get_steering,
            workspace_commands::workspace_steering_prompt,
            workspace_commands::workspace_dispatch_hooks,
            workspace_commands::workspace_list_hooks,
            workspace_commands::workspace_save_hook,
            workspace_commands::workspace_delete_hook,
            workspace_commands::workspace_list_agent_runs,
            workspace_commands::workspace_save_agent_run,
            ai_commands::chat_stream_drain,
            ai_commands::agent_proposals_drain,
            ai_commands::revert_file_content,
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
            // ═══ Antigravity Workflow Engine ═══
            antigravity_commands::ag_list_all_tasks,
            antigravity_commands::ag_get_next_task,
            antigravity_commands::ag_mark_task_done,
            antigravity_commands::ag_create_spec,
            antigravity_commands::ag_phase_wrap,
            antigravity_commands::ag_get_workflows,
            antigravity_commands::ag_get_rules,
            antigravity_commands::ag_init_layout,
            antigravity_commands::ag_brain_list,
            antigravity_commands::ag_brain_list_cascades,
            antigravity_commands::ag_brain_save_artifact,
            antigravity_commands::ag_brain_save_media,
            antigravity_commands::ag_list_trajectories,
            antigravity_commands::ag_get_trajectory,
            antigravity_commands::ag_save_trajectory,
            antigravity_commands::ag_append_trajectory_step,
            antigravity_commands::ag_export_trajectory_jsonl,
            antigravity_commands::ag_upsert_subagent,
            antigravity_commands::ag_load_lifecycle_hooks,
            antigravity_commands::ag_save_lifecycle_hooks,
            antigravity_commands::ag_dispatch_lifecycle_hooks,
            antigravity_commands::ag_get_autonomy_policies,
            antigravity_commands::ag_save_autonomy_policies,
            antigravity_commands::ag_apply_autonomy_preset,
            // ═══ AI Patching ═══
            ai_patch_commands::accept_sentient_patch,
            ai_patch_commands::reject_sentient_patch,
            ai_patch_commands::propose_fast_edit,
            // ═══ Background Jobs ═══
            jobs::get_background_jobs,
            jobs::register_background_job,
            jobs::update_background_job,
            jobs::remove_background_job,
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
            apex_commands::apex_set_local_mode,
            // ═══ ANE Inference Acceleration (M1/M2/M3/M4) ═══
            ane_commands::ane_get_status,
            ane_commands::ane_init_inference,
            ane_commands::ane_can_accelerate,
            ane_commands::ane_update_metrics,
            ane_commands::ane_diagnostics,
            // ═══ Model Management (Dynamic Ollama selection) ═══
            model_commands::list_ollama_models,
            model_commands::get_current_model,
            model_commands::set_current_model,
            model_commands::get_model_info,
            model_commands::detect_best_model,
            model_commands::apply_model_to_all_engines,
            // ═══ Optimized Inference (ANE + Memory + MoE) ═══
            inference_commands::inference_get_status,
            inference_commands::inference_prepare_model,
            inference_commands::inference_get_setup_recommendation,
            // ═══ Android Commands ═══
            android_commands::adb_list_devices,
            android_commands::adb_list_emulators,
            android_commands::adb_install_and_run,
            android_commands::set_active_device,
            android_commands::get_android_config,
            android_commands::set_android_sdk_path,
            android_commands::spawn_emulator,
            gradle_commands::gradle_detect_project,
            gradle_commands::gradle_sync_project,
            gradle_commands::gradle_list_tasks,
            gradle_commands::gradle_run_task,
            logcat_commands::logcat_start,
            logcat_commands::logcat_stop,
            logcat_commands::logcat_status,
            test_commands::test_sniff_framework,
            test_commands::test_discover,
            test_commands::test_run_file,
            test_commands::test_run_all,
            pytorch_commands::pytorch_detect,
            pytorch_commands::pytorch_install,
            pytorch_commands::pytorch_verify,
            ml_studio::ml_studio_init,
            ml_studio::ml_studio_get_config,
            ml_studio::ml_studio_save_config,
            ml_studio::ml_studio_list_data,
            ml_studio::ml_studio_prepare_dataset,
            ml_studio::ml_studio_train,
            ml_studio::ml_studio_list_runs,
            ml_studio::ml_studio_get_run_metrics,
            ml_studio::ml_studio_get_active_run,
            ml_studio::ml_studio_infer,
            ml_studio::ml_studio_install_deps,
            ml_studio::ml_studio_dataset_stats,
            ml_studio::ml_studio_model_summary,
            ml_studio::ml_studio_export_model,
            ml_studio::ml_studio_pretrained_gallery,
            ml_studio::ml_studio_hpo,
            ml_studio::ml_studio_lr_finder,
            ml_studio::ml_studio_grad_check,
            ml_studio::ml_studio_benchmark,
            ml_studio::ml_studio_list_experiments,
            ml_studio::ml_studio_export_report,
            ml_studio::ml_studio_cancel_train,
            ml_studio::ml_studio_model_graph,
            ml_studio::ml_studio_augment_preview,
            ml_studio::ml_studio_load_pretrained,
            ml_studio::ml_studio_compare_runs,
            ml_studio::ml_studio_list_workers,
            ml_studio::ml_studio_save_worker,
            ml_studio::ml_studio_export_onnx,
            ml_studio::ml_studio_netron_url,
            stop_hooks::stop_hooks_get,
            stop_hooks::stop_hooks_save,
            stop_hooks::stop_hooks_run,
            // ═══ Emulator Stream ═══
            emulator_stream::list_available_avds,
            emulator_stream::create_avd,
            emulator_stream::spawn_emulator_by_name,
            emulator_stream::list_running_emulators,
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
            editor_commands::list_keybindings,
            editor_commands::update_keybinding,
            editor_commands::switch_to_buffer,
            editor_commands::get_highlights,
            editor_commands::set_context_key,
            editor_commands::evaluate_when_clause,
            editor_commands::set_active_root,
            editor_commands::get_active_root,
            editor_commands::path_exists,
            extensions_commands::ext_host_init,
            extensions_commands::ext_host_send,
            extensions_commands::ext_host_sync_workspace,
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
            extensions_commands::ext_host_sync_workspace,
            extensions_commands::refresh_popular_extensions,
            extensions_commands::refresh_installed_extensions,
            extensions_commands::check_activation_event,
            // ═══ Git Commands ═══
            git_commands::git_status,
            git_commands::git_diff,
            git_commands::get_git_diff,
            git_commands::git_diff_file,
            git_commands::git_stage,
            git_commands::git_unstage,
            git_commands::git_get_unmerged,
            git_commands::git_commit,
            git_commands::git_revert,
            git_commands::git_stash,
            git_commands::git_stash_pop,
            git_commands::git_clone,
            git_commands::git_push,
            git_commands::git_pull,
            git_commands::git_fetch,
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
            terminal_commands::terminal_take_pending,
            terminal_commands::terminal_terminate,
            terminal_commands::terminal_toggle,
            terminal_commands::get_available_shells,
            // ═══ File Commands ═══
            file_commands::refresh_file_tree,
            // ═══ MCP ═══
            mcp_commands::detect_ghidra_install_dir,
            mcp_commands::detect_ida_install_dir,
            mcp_commands::add_mcp_server,
            mcp_commands::remove_mcp_server,
            mcp_commands::list_mcp_servers,
            mcp_commands::set_mcp_server_enabled,
            mcp_commands::get_mcp_config_path,
            mcp_commands::list_mcp_tools,
            mcp_commands::read_mcp_config,
            mcp_commands::write_mcp_config,
            // ═══ System ═══
            system_commands::backend_ping,
            system_commands::respond_tool_permission,
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
            web_commands::http_probe,
            web_commands::embed_text,
            // ═══ Visual Lab ═══
            visual_commands::get_visual_graph,
            visual_commands::get_neural_omni_graph,
            visual_commands::get_all_memory_slots,
            visual_commands::generate_visual_graph,
            // ═══ Voice ═══
            voice_commands::elevenlabs_get_voices,
            // ═══ Workspace ═══
            attachment_manager::select_and_process_attachment,
            vision_sidecar::discover_vision_models_cmd,
            vision_sidecar::vision_sidecar_process_attachments,
            kortex_commands::save_kortex_memory,
            kortex_commands::load_kortex_memory,
            kortex_commands::load_kortex_metadata,
            kortex_commands::aim_trust_manifest,
            kortex_commands::aim_query_spans,
            kortex_commands::aim_load_telemetry,
            kortex_commands::aim_append_telemetry,
            kortex_commands::aim_telemetry_snapshot,
            kortex_commands::aim_flush_telemetry,
            kortex_commands::aim_clear_telemetry_samples,
            kortex_commands::aim_set_bound_model,
            kortex_commands::kortex_resolve_ollama_gguf,
            kortex_commands::aim_upsert_gist,
            kortex_commands::aim_load_gists,
            kortex_commands::aim_invalidate_stale,
            kortex_commands::vfs_write_atomic,
            kortex_commands::vfs_apply_patch,
            kortex_commands::aim_pack_context,
            kortex_commands::trigger_workspace_index,
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
            browser::browser_set_headless,
            browser::browser_close,
            browser::browser_navigate,
            browser::browser_read_dom,
            browser::browser_type,
            browser::browser_click,
            browser::browser_screenshot,
            browser::browser_get_content_summary,
            browser::browser_status,
            browser::browser_capture_vision_context,
            // ═══ Remote SSH ═══
            remote_commands::remote_ssh_probe,
            remote_commands::remote_ssh_list_dir,
            remote_commands::remote_ssh_mount,
            remote_commands::remote_ssh_sync_pull,
            remote_commands::remote_ssh_sync_push,
            remote_commands::remote_ssh_disconnect,
            remote_commands::remote_ssh_status,
            remote_commands::remote_ssh_exec,
            remote_commands::remote_ssh_read_file,
            remote_commands::remote_ssh_write_file,
            // ═══ Hermes gateway (:8642) ═══
            hermes_gateway::hermes_gateway_start,
            hermes_gateway::hermes_gateway_stop,
            hermes_gateway::hermes_gateway_status,
            // ═══ Vector Search ═══
            vector_commands::vector_index_codebase,
            vector_commands::vector_search_codebase,
            vector_commands::vector_find_symbol,
            vector_commands::vector_get_file_chunks,
            vector_commands::vector_get_index_stats,
            vector_commands::vector_get_indexing_progress,
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
            lsp_commands::lsp_auto_start,
            lsp_commands::lsp_ensure_for_file,
            lsp_commands::lsp_detect_workspace,
            lsp_commands::lsp_start_server,
            lsp_commands::lsp_bundle_status,
            lsp_commands::lsp_ensure_bundle,
            lsp_commands::lsp_send_request,
            lsp_commands::lsp_stop,
            lsp_commands::lsp_initialized,
            lsp_commands::lsp_did_open,
            lsp_commands::lsp_did_change,
            lsp_commands::lsp_did_save,
            lsp_commands::lsp_set_workspace,
            lsp_commands::lsp_change_workspace_folders,
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
            lsp_store::lsp_store_status,
            lsp_store::lsp_store_list,
            lsp_store::lsp_store_catalog,
            lsp_store::lsp_store_scan_path,
            lsp_store::lsp_store_install_preset,
            lsp_store::lsp_store_install_path,
            lsp_store::lsp_store_set_enabled,
            lsp_store::lsp_store_uninstall,
            lsp_store::lsp_store_install_npm,
            workspace_settings_commands::get_workspace_settings,
            workspace_settings_commands::update_workspace_settings,
            port_commands::list_listening_ports,
            port_commands::port_forward_add,
            port_commands::port_forward_remove,
            // ═══ Extra Commands ═══
            file_commands::open_folder,
            iphone_emulator::launch_iphone_emulator,
            iphone_emulator::stop_iphone_emulator,
            iphone_emulator::is_iphone_emulator_running,
            iphone_emulator::send_iphone_touch,
            iphone_emulator::launch_vphone,
            iphone_emulator::prepare_ios_firmware,
            iphone_emulator::create_stub_ramdisk,
            ios_simulator::ios_sim_preflight,
            ios_simulator::ios_sim_list_devices,
            ios_simulator::ios_sim_boot_device,
            ios_simulator::ios_sim_start_mirror,
            ios_simulator::ios_sim_stop_mirror,
            ios_simulator::ios_sim_send_touch,
            ios_simulator::ios_sim_send_home,
            ios_simulator::ios_sim_capture_screenshot,
            ios_simulator::ios_sim_mirror_running,
            ios_simulator::ios_sim_warmup,
            ios_simulator::ios_sim_pause,
            ios_simulator::ios_sim_resume,
            ios_simulator::ios_sim_session_state,
            ios_simulator::ios_sim_stream_url,
            ios_simulator::ios_sim_stream_status,
            ios_simulator::ios_sim_embed_layout,
            mobile_toolchain::resolve_mobile_toolchain_paths,
            mobile_toolchain::run_vphone_doctor,
            mobile_toolchain::install_vphone_toolchain,
            mobile_toolchain::get_mobile_toolchain_env,
            window_commands::win_minimize,
            window_commands::win_toggle_maximize,
            window_commands::win_close,
            window_commands::win_start_drag,
            window_commands::win_state,
            performance_commands::get_inference_history,
            ai_project_commands::search_project,
            ai_project_commands::query_workspace_memory,
            ai_project_commands::get_file_context,
            ai_project_commands::clear_ai_memory,
            ai_project_commands::search_codebase_files,
            airi_bridge::airi_event,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
