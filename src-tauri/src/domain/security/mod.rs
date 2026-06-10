//! Security domain: APEX scanning/orchestration, pentest scope/report/executor,
//! OAST collaborator, intruder/repeater, intercept proxy, secret chunking, and
//! native security pattern analysis. (The Vega DAST engine lives in `crate::vega`.)

pub mod apex_orchestrator;
pub mod apex_red_team;
pub mod chunk_secrets;
pub mod hunter;
pub mod intercept_proxy;
pub mod intruder;
pub mod oast;
pub mod pentest_executor;
pub mod pentest_report;
pub mod pentest_scope;
pub mod repeater;
pub mod sec_distro;
pub mod security_distiller;
pub mod security_generators;
pub mod security_native;
pub mod security_patterns;
