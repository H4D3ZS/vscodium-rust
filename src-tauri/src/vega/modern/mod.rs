//! Modern attack modules and payload packs (2020–2026 era).
//!
//! Vega's 2016 modules miss SSRF cloud metadata, GraphQL, JWT confusion, NoSQL
//! operators, SSTI, IDOR/BOLA, and LLM-gateway probes. This layer adds:
//! - Static payload packs (`payloads.rs`) usable by Rust-native scanners
//! - Optional JS modules under `resources/vega/scripts/modules/modern/`
//! - Local LLM assist for payload mutation + FP triage (`ai_assist.rs`)

pub mod ai_assist;
pub mod payloads;

pub use ai_assist::{AiAssistConfig, AiAssistResult, VegaAiAssist};
pub use payloads::ModernPayloadPack;
