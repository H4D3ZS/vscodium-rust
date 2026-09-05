//! AI domain: the Sentient engine, tool registry/invocation, model management,
//! ANE acceleration, Ollama offloading, agent/MCTS harnesses, and vision.
//!
//! NOTE: ai_engine.rs (7.2K LOC) and ai_tools.rs (8.5K LOC) are slated to be
//! split into focused submodules — see ARCHITECTURE.md

pub mod agent_harness;
pub mod engine;
pub mod ai_prompts;
pub mod ane;
pub mod ane_inference;
pub mod hades_harness;
pub mod hades_vision;
pub mod image_gen;
pub mod memory_budget;
pub mod model_manager;
pub mod thinking_handler;
pub mod gpu_offload;
pub mod optimized_inference;
pub mod rules_engine;
pub mod streaming_tool_executor;
pub mod task_planner;
pub mod tool_aliases;
pub mod tool_invoker;
#[cfg(feature = "tauri")]
pub mod vision;
#[cfg(feature = "tauri")]
pub mod vision_bridge;
pub mod vision_sidecar;
pub mod webui_protocol;
pub mod workflow_engine;
#[cfg(feature = "tauri")]
pub mod ml_studio;
