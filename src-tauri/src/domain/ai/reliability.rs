//! Reliability facade — one call that runs the truth-side chain on a finished
//! answer: **ground → annotate → abstain**.
//!
//! The individual levers (`grounding`, `abstain`) are composable primitives;
//! this is the single entry point the response path calls so the wiring lives in
//! one place. Each stage is independently env-gated:
//!
//!   * `KORTEX_GROUNDING` — verify file/symbol references, append a "⚠ Unverified
//!     references" block for anything fabricated,
//!   * `KORTEX_ABSTAIN` — if the fused risk is high, qualify or withhold the
//!     answer instead of asserting a likely-wrong one.
//!
//! **Default:** on when the answer came from a **local** model, off for a
//! cloud one (`is_local`, threaded from the caller's own provider check).
//! Fact-checking a local 4–27B model against the workspace is exactly the
//! failure mode this cluster targets; a frontier cloud model's own answer isn't
//! second-guessed by a heuristic here unless you explicitly ask for it with
//! `KORTEX_GROUNDING=1`/`KORTEX_ABSTAIN=1`. Either flag can be forced off with
//! `=0` even for a local model.
//!
//! `mean_p` is the answer's mean token probability when the caller has logprobs
//! (else `None` → grounding's hard reference signal carries the decision).

use super::abstain::{self, AbstainConfig, Stance};
use super::env_flag;
use super::grounding::{self, GroundTruth, WorkspaceGroundTruth};
use std::path::Path;

/// Run the chain against the real workspace (files + optional index symbols).
/// `is_local` sets the on/off default (see module docs); an explicit env value
/// always wins over it.
pub fn finalize(text: &str, root: &Path, mean_p: Option<f64>, hedged: bool, is_local: bool) -> String {
    finalize_with(text, &WorkspaceGroundTruth::new(root), mean_p, hedged, is_local)
}

/// Testable core: same logic against any [`GroundTruth`].
pub fn finalize_with(
    text: &str,
    gt: &dyn GroundTruth,
    mean_p: Option<f64>,
    hedged: bool,
    is_local: bool,
) -> String {
    let grounding_on = env_flag::on("KORTEX_GROUNDING", is_local);
    let abstain_on = env_flag::on("KORTEX_ABSTAIN", is_local);
    if !grounding_on && !abstain_on {
        return text.to_string();
    }

    let report = grounding::ground(text, gt);

    // Grounding annotation first (flags fabricated refs inline).
    let mut out = if grounding_on {
        grounding::annotate(text, &report)
    } else {
        text.to_string()
    };

    // Then abstention: qualify prepends a caveat to the (annotated) answer;
    // abstain withholds the likely-wrong assertion entirely.
    if abstain_on {
        let cfg = AbstainConfig { enabled: true, ..AbstainConfig::from_env() };
        let decision = abstain::decide(&report, mean_p, hedged, &cfg);
        out = match decision.stance {
            Stance::Answer => out,
            _ => abstain::apply(&out, &decision),
        };
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::sync::MutexGuard;

    struct Mock {
        files: HashMap<&'static str, usize>,
    }
    impl GroundTruth for Mock {
        fn file_exists(&self, p: &str) -> Option<bool> {
            Some(self.files.contains_key(p))
        }
        fn line_count(&self, p: &str) -> Option<usize> {
            self.files.get(p).copied()
        }
    }
    fn gt() -> Mock {
        Mock { files: HashMap::from([("src/main.rs", 100)]) }
    }

    // Env is process-global; serialize the env-mutating tests.
    fn env_lock() -> MutexGuard<'static, ()> {
        static L: std::sync::Mutex<()> = std::sync::Mutex::new(());
        L.lock().unwrap_or_else(|p| p.into_inner())
    }

    fn with_env(vars: &[(&str, Option<&str>)], f: impl FnOnce()) {
        let _g = env_lock();
        for (k, v) in vars {
            match v {
                Some(val) => std::env::set_var(k, val),
                None => std::env::remove_var(k),
            }
        }
        f();
        for (k, _) in vars {
            std::env::remove_var(k);
        }
    }

    #[test]
    fn passthrough_when_disabled_and_not_local() {
        with_env(&[("KORTEX_GROUNDING", None), ("KORTEX_ABSTAIN", None)], || {
            let text = "the fix is in `src/ghost.rs`";
            assert_eq!(finalize_with(text, &gt(), None, false, false), text);
        });
    }

    #[test]
    fn defaults_on_for_a_local_model_with_no_env_set() {
        // is_local=true with nothing set defaults BOTH grounding and abstain
        // on, so a fabricated reference (high fused risk) correctly triggers
        // full abstention, not just an annotation — the abstain stage
        // supersedes the grounding annotation on the same answer.
        with_env(&[("KORTEX_GROUNDING", None), ("KORTEX_ABSTAIN", None)], || {
            let out = finalize_with("see `src/ghost.rs`", &gt(), None, false, true);
            assert!(out.contains("not confident enough"), "local model → both levers on by default → abstains");
        });
    }

    #[test]
    fn grounding_alone_defaults_on_for_a_local_model() {
        // Isolate the grounding-only default by explicitly disabling abstain.
        with_env(&[("KORTEX_GROUNDING", None), ("KORTEX_ABSTAIN", Some("0"))], || {
            let out = finalize_with("see `src/ghost.rs`", &gt(), None, false, true);
            assert!(out.contains("Unverified references"), "grounding alone → on by default for a local model");
        });
    }

    #[test]
    fn explicit_off_beats_local_default() {
        with_env(&[("KORTEX_GROUNDING", Some("0")), ("KORTEX_ABSTAIN", Some("0"))], || {
            let text = "see `src/ghost.rs`";
            assert_eq!(
                finalize_with(text, &gt(), None, false, true),
                text,
                "explicit =0 must win even for a local model"
            );
        });
    }

    #[test]
    fn explicit_on_beats_cloud_default() {
        with_env(&[("KORTEX_GROUNDING", Some("1")), ("KORTEX_ABSTAIN", None)], || {
            let out = finalize_with("see `src/ghost.rs`", &gt(), None, false, false);
            assert!(out.contains("Unverified references"), "explicit =1 must win even for a cloud model");
        });
    }

    #[test]
    fn grounding_only_annotates_bad_ref() {
        with_env(&[("KORTEX_GROUNDING", Some("1")), ("KORTEX_ABSTAIN", Some("0"))], || {
            let out = finalize_with("see `src/ghost.rs`", &gt(), None, false, false);
            assert!(out.contains("Unverified references"));
            assert!(out.contains("src/ghost.rs"));
        });
    }

    #[test]
    fn abstain_withholds_fabricated_answer() {
        with_env(&[("KORTEX_GROUNDING", Some("1")), ("KORTEX_ABSTAIN", Some("1"))], || {
            let out = finalize_with("The bug is in `src/ghost.rs`.", &gt(), Some(0.9), false, false);
            assert!(out.contains("not confident enough"));
            assert!(!out.contains("The bug is in"), "likely-wrong answer withheld");
        });
    }

    #[test]
    fn clean_grounded_answer_passes_clean() {
        with_env(&[("KORTEX_GROUNDING", Some("1")), ("KORTEX_ABSTAIN", Some("1"))], || {
            let text = "See `src/main.rs` for the parser.";
            assert_eq!(finalize_with(text, &gt(), Some(0.95), false, false), text);
        });
    }
}
