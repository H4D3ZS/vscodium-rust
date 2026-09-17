//! Live correlation engine — cross-references static findings, live traffic,
//! and live storage writes into scored, evidence-backed findings.
//!
//! This is the piece that converts "collection" into "triage": the difference
//! between flagging a hardcoded secret and *confirming* that secret was used
//! in a live outbound request. See `engine.rs` for the scoring and dedup logic
//! and `model.rs` for the event/finding types.
//!
//! Authorized testing / bug bounty use only.

pub mod engine;
pub mod model;

pub use engine::CorrelationEngine;
pub use model::{
    CorrelationKind, Finding, NetworkEvent, Severity, StaticFinding, StaticKind, StorageEvent,
    StorageKind,
};
