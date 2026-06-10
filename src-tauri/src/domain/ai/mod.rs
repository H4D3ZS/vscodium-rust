//! AI domain: the Sentient engine, tool registry/invocation, model management,
//! ANE acceleration, Ollama offloading, agent/MCTS harnesses, and vision.
//!
//! NOTE: ai_engine.rs (7.2K LOC) and ai_tools.rs (8.5K LOC) are slated to be
//! split into focused submodules — see docs/overhaul/MASTER_PLAN.md §A1.

pub mod agent_harness;
pub mod ai_engine;
pub mod ai_prompts;
pub mod ai_tools;
pub mod ane;
pub mod ane_inference;
pub mod hades_harness;
pub mod hades_vision;
pub mod image_gen;
pub mod model_manager;
pub mod ollama_offload;
pub mod optimized_inference;
pub mod rules_engine;
pub mod streaming_tool_executor;
pub mod task_planner;
pub mod tool_aliases;
pub mod tool_invoker;
pub mod vision;
pub mod vision_bridge;
pub mod vision_sidecar;
pub mod workflow_engine;
pub mod ml_studio;
