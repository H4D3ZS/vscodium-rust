//! # APEX Vega — native web vulnerability scanner
//!
//! A Rust/Tauri reimplementation of Subgraph Vega's dynamic application
//! security testing (DAST) engine. Vega's 46 JavaScript attack/detection
//! modules and 85 alert definitions are reused verbatim (shipped under
//! `resources/vega/`); only the engine that hosts them is rewritten here.
//!
//! Build progresses in phases — see `.planning/vega-integration/PROGRESS.md`.
//! Phases 0–3 complete: model, alerts, JS host, HTTP scan engine + modern layer.

pub mod alerts;
pub mod campaign;
pub mod crawler;
pub mod engine;
pub mod error_based;
pub mod fingerprint;
pub mod injection_host;
pub mod js_runtime;
pub mod modern;
pub mod model;
pub mod report;

pub use alerts::AlertRegistry;
pub use campaign::{list_modules, run_campaign, VegaModuleInfo, VegaScanOptions, VegaScanResult};
pub use crawler::{crawl, CrawlConfig};
pub use engine::ScanEngine;
pub use fingerprint::ResponseFingerprint;
pub use injection_host::{build_altered_request, InjectionModuleHost, PlanStep};
pub use js_runtime::{JsModuleHost, ModuleKind, ModuleMeta, ModuleRunResult};
pub use modern::{AiAssistConfig, ModernPayloadPack, VegaAiAssist};
pub use model::{
    Alert, AlertDefinition, FuzzableParam, HttpRequest, HttpResponse, ParamLocation, PathState,
    Severity,
};
