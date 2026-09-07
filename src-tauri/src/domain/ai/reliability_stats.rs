//! Process-global, lock-free counters for every reliability/compute-cost
//! lever, incremented at the real call sites — so "is this actually firing"
//! is a measured fact, not a guess, matching the discipline the spec-decode
//! acceptance readout (`spec: NN% kept · M tok/step`) already established for
//! decode throughput. This is that same pattern for the reliability cluster.
//!
//! Deliberately not per-request or persisted — a simple in-memory tally since
//! the IDE last started, cheap enough (`AtomicU64::fetch_add`) to call from
//! every hot path without a second thought, read by the `kortex_reliability_status`
//! Tauri command for the settings panel.

use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};

macro_rules! counters {
    ($($name:ident),+ $(,)?) => {
        $(static $name: AtomicU64 = AtomicU64::new(0);)+

        /// Snapshot every counter as a plain `{name: count}` map.
        pub fn snapshot() -> std::collections::BTreeMap<&'static str, u64> {
            let mut m = std::collections::BTreeMap::new();
            $(m.insert(stringify!($name), $name.load(Ordering::Relaxed));)+
            m
        }
    };
}

counters!(
    // Semantic cache (semantic_cache.rs)
    SEMCACHE_HITS,
    SEMCACHE_MISSES,
    // Cascade router (cascade.rs)
    CASCADE_STARTED_OPERATOR,
    CASCADE_STARTED_REASONER_PRECLASSIFIED,
    CASCADE_ACCEPTED_OPERATOR,
    CASCADE_ESCALATED,
    // Grounding + abstain (reliability.rs / grounding.rs / abstain.rs)
    GROUNDING_CLAIMS_CHECKED,
    GROUNDING_UNGROUNDED_FLAGGED,
    ABSTAIN_QUALIFIED,
    ABSTAIN_WITHHELD,
    // Authorization (authorization.rs, via dispatch.rs)
    AUTHZ_ALLOWED,
    AUTHZ_CONFIRMED,
    AUTHZ_DENIED,
    // Provenance (provenance.rs, via dispatch.rs)
    PROVENANCE_FENCED,
    PROVENANCE_INJECTION_FLAGGED,
    // Verification (verify.rs, via verify_implementation)
    VERIFY_RUNS,
    VERIFY_PASSED,
    VERIFY_FAILED,
    // Harness compression (kortex_harness)
    HARNESS_TOOL_OUTPUT_COMPACTED,
    HARNESS_STEER_APPLIED,
    HARNESS_SCHEMA_COMPACTED,
    // Search engine (tgrep_search.rs)
    TGREP_SERVER_PROBE_HITS,
    TGREP_IN_PROCESS_HITS,
);

/// Bump one counter by name. A silent no-op for an unknown name (never a
/// panic — this is instrumentation, not a control path) so a typo can't take
/// down whatever called it.
pub fn bump(name: &str) {
    let cell: Option<&AtomicU64> = match name {
        "SEMCACHE_HITS" => Some(&SEMCACHE_HITS),
        "SEMCACHE_MISSES" => Some(&SEMCACHE_MISSES),
        "CASCADE_STARTED_OPERATOR" => Some(&CASCADE_STARTED_OPERATOR),
        "CASCADE_STARTED_REASONER_PRECLASSIFIED" => Some(&CASCADE_STARTED_REASONER_PRECLASSIFIED),
        "CASCADE_ACCEPTED_OPERATOR" => Some(&CASCADE_ACCEPTED_OPERATOR),
        "CASCADE_ESCALATED" => Some(&CASCADE_ESCALATED),
        "GROUNDING_CLAIMS_CHECKED" => Some(&GROUNDING_CLAIMS_CHECKED),
        "GROUNDING_UNGROUNDED_FLAGGED" => Some(&GROUNDING_UNGROUNDED_FLAGGED),
        "ABSTAIN_QUALIFIED" => Some(&ABSTAIN_QUALIFIED),
        "ABSTAIN_WITHHELD" => Some(&ABSTAIN_WITHHELD),
        "AUTHZ_ALLOWED" => Some(&AUTHZ_ALLOWED),
        "AUTHZ_CONFIRMED" => Some(&AUTHZ_CONFIRMED),
        "AUTHZ_DENIED" => Some(&AUTHZ_DENIED),
        "PROVENANCE_FENCED" => Some(&PROVENANCE_FENCED),
        "PROVENANCE_INJECTION_FLAGGED" => Some(&PROVENANCE_INJECTION_FLAGGED),
        "VERIFY_RUNS" => Some(&VERIFY_RUNS),
        "VERIFY_PASSED" => Some(&VERIFY_PASSED),
        "VERIFY_FAILED" => Some(&VERIFY_FAILED),
        "HARNESS_TOOL_OUTPUT_COMPACTED" => Some(&HARNESS_TOOL_OUTPUT_COMPACTED),
        "HARNESS_STEER_APPLIED" => Some(&HARNESS_STEER_APPLIED),
        "HARNESS_SCHEMA_COMPACTED" => Some(&HARNESS_SCHEMA_COMPACTED),
        "TGREP_SERVER_PROBE_HITS" => Some(&TGREP_SERVER_PROBE_HITS),
        "TGREP_IN_PROCESS_HITS" => Some(&TGREP_IN_PROCESS_HITS),
        _ => None,
    };
    if let Some(c) = cell {
        c.fetch_add(1, Ordering::Relaxed);
    }
}

/// One lever's live status: whether it's currently enabled (given the
/// process's env + the `is_local` hint where relevant) and its tally.
#[derive(Debug, Clone, Serialize)]
pub struct LeverStatus {
    pub name: &'static str,
    pub enabled: bool,
    pub counters: std::collections::BTreeMap<&'static str, u64>,
}

/// Full status snapshot for the settings panel: every lever's on/off state
/// (computed the same way the request path itself would — local-scoped
/// defaults included) plus its live counters.
pub fn full_status() -> Vec<LeverStatus> {
    let snap = snapshot();
    let pick = |keys: &[&'static str]| -> std::collections::BTreeMap<&'static str, u64> {
        keys.iter().map(|k| (*k, snap.get(k).copied().unwrap_or(0))).collect()
    };

    vec![
        LeverStatus {
            name: "semantic_cache",
            enabled: super::semantic_cache::CacheConfig::from_env().enabled,
            counters: pick(&["SEMCACHE_HITS", "SEMCACHE_MISSES"]),
        },
        LeverStatus {
            name: "cascade",
            enabled: super::cascade::CascadeConfig::from_env().enabled,
            counters: pick(&[
                "CASCADE_STARTED_OPERATOR",
                "CASCADE_STARTED_REASONER_PRECLASSIFIED",
                "CASCADE_ACCEPTED_OPERATOR",
                "CASCADE_ESCALATED",
            ]),
        },
        LeverStatus {
            name: "grounding",
            // Local-scoped default (see reliability.rs docs) — reported as
            // "on for a local model" since that's the common case; the exact
            // per-request value also depends on is_local at call time.
            enabled: env_on("KORTEX_GROUNDING", true),
            counters: pick(&["GROUNDING_CLAIMS_CHECKED", "GROUNDING_UNGROUNDED_FLAGGED"]),
        },
        LeverStatus {
            name: "abstain",
            enabled: env_on("KORTEX_ABSTAIN", true),
            counters: pick(&["ABSTAIN_QUALIFIED", "ABSTAIN_WITHHELD"]),
        },
        LeverStatus {
            name: "authorization",
            enabled: super::authorization::AuthzConfig::from_env().enabled,
            counters: pick(&["AUTHZ_ALLOWED", "AUTHZ_CONFIRMED", "AUTHZ_DENIED"]),
        },
        LeverStatus {
            name: "provenance",
            enabled: super::provenance::ProvenanceConfig::from_env().enabled,
            counters: pick(&["PROVENANCE_FENCED", "PROVENANCE_INJECTION_FLAGGED"]),
        },
        LeverStatus {
            name: "verify",
            enabled: super::verify::VerifyConfig::from_env().enabled,
            counters: pick(&["VERIFY_RUNS", "VERIFY_PASSED", "VERIFY_FAILED"]),
        },
        LeverStatus {
            name: "harness",
            enabled: crate::kortex_harness::HarnessConfig::from_env().enabled,
            counters: pick(&[
                "HARNESS_TOOL_OUTPUT_COMPACTED",
                "HARNESS_STEER_APPLIED",
                "HARNESS_SCHEMA_COMPACTED",
            ]),
        },
        LeverStatus {
            name: "tgrep",
            enabled: crate::ide_shell::resolve_tgrep_exe().is_some(),
            counters: pick(&["TGREP_SERVER_PROBE_HITS", "TGREP_IN_PROCESS_HITS"]),
        },
    ]
}

fn env_on(key: &str, default_when_unset: bool) -> bool {
    super::env_flag::on(key, default_when_unset)
}

#[cfg(feature = "tauri")]
#[tauri::command]
pub fn kortex_reliability_status() -> serde_json::Value {
    serde_json::json!({ "levers": full_status() })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bump_unknown_name_is_a_silent_noop() {
        bump("THIS_DOES_NOT_EXIST"); // must not panic
    }

    #[test]
    fn bump_increments_the_named_counter() {
        let before = snapshot()["SEMCACHE_HITS"];
        bump("SEMCACHE_HITS");
        let after = snapshot()["SEMCACHE_HITS"];
        assert_eq!(after, before + 1);
    }

    #[test]
    fn full_status_reports_every_lever_by_name() {
        let statuses = full_status();
        let names: Vec<&str> = statuses.iter().map(|s| s.name).collect();
        for expect in [
            "semantic_cache", "cascade", "grounding", "abstain",
            "authorization", "provenance", "verify", "harness", "tgrep",
        ] {
            assert!(names.contains(&expect), "missing lever status: {expect}");
        }
    }

    #[test]
    fn full_status_counters_reflect_bumps() {
        bump("CASCADE_ESCALATED");
        let statuses = full_status();
        let cascade = statuses.iter().find(|s| s.name == "cascade").unwrap();
        assert!(cascade.counters["CASCADE_ESCALATED"] >= 1);
    }
}
