//! # APEX Vega — native web vulnerability scanner
//!
//! A Rust/Tauri reimplementation of Subgraph Vega's dynamic application
//! security testing (DAST) engine. Vega's 46 JavaScript attack/detection
//! modules and 85 alert definitions are reused verbatim (shipped under
//! `resources/vega/`); only the engine that hosts them is rewritten here.
//!
//! Build progresses in phases — see `.planning/vega-integration/PROGRESS.md`.
//! Current: Phase 1 (model + alert registry). JS host (Phase 2), HTTP engine
//! (Phase 3), crawler (Phase 4), proxy (Phase 5) land in later modules.

pub mod alerts;
pub mod model;

pub use alerts::AlertRegistry;
pub use model::{
    Alert, AlertDefinition, FuzzableParam, HttpRequest, HttpResponse, ParamLocation, PathState,
    Severity,
};
