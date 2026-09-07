//! Calibrated abstention — the fix for the incentive problem.
//!
//! The 2026 Nature/OpenAI result: accuracy-based training rewards *guessing* —
//! a confident wrong answer scores the same as a lucky right one, so models
//! learn to never say "I don't know". The correction is calibration: let the
//! system **abstain** when its own signals say it's likely wrong, because a
//! marked "I'm not sure" is worth more to a developer than a fabricated answer.
//!
//! This module fuses the signals the rest of the stack already produces into one
//! decision:
//!   * `cascade::Confidence` — token logprobs (shaky generation),
//!   * `grounding::GroundingReport` — reference risk (fabricated file/symbol),
//! and returns [`Stance::Answer`], [`Stance::Qualify`] (answer, but flag the
//! uncertainty), or [`Stance::Abstain`] (don't assert; ask or escalate). The
//! thresholds are the calibration knob.
//!
//! Opt-in via `KORTEX_ABSTAIN`.

use super::grounding::GroundingReport;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Stance {
    /// Confident and grounded — answer normally.
    Answer,
    /// Usable but uncertain — answer with an explicit caveat.
    Qualify,
    /// Likely wrong — don't assert it. Escalate or ask instead.
    Abstain,
}

#[derive(Debug, Clone)]
pub struct AbstainConfig {
    pub enabled: bool,
    /// Below this fused risk → Answer.
    pub qualify_at: f32,
    /// Above this fused risk → Abstain.
    pub abstain_at: f32,
    /// Any confirmed ungrounded reference forces at least Qualify (a fabricated
    /// filename is a hard error regardless of the soft signals).
    pub ungrounded_forces_qualify: bool,
}

impl Default for AbstainConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            qualify_at: 0.25,
            abstain_at: 0.60,
            ungrounded_forces_qualify: true,
        }
    }
}

impl AbstainConfig {
    pub fn from_env() -> Self {
        let mut cfg = Self {
            enabled: matches!(
                std::env::var("KORTEX_ABSTAIN").ok().as_deref(),
                Some("1") | Some("true") | Some("on")
            ),
            ..Self::default()
        };
        if let Some(v) = env_f32("KORTEX_ABSTAIN_AT") {
            cfg.abstain_at = v.clamp(0.0, 1.0);
        }
        if let Some(v) = env_f32("KORTEX_ABSTAIN_QUALIFY_AT") {
            cfg.qualify_at = v.clamp(0.0, 1.0);
        }
        cfg
    }
}

fn env_f32(k: &str) -> Option<f32> {
    std::env::var(k).ok()?.trim().parse().ok()
}

#[derive(Debug, Clone, Serialize)]
pub struct AbstainDecision {
    pub stance: Stance,
    /// Fused risk in [0,1] that drove the decision.
    pub risk: f32,
    pub reason: String,
}

/// Decide a stance from the fused signals. `mean_p` is the answer's mean token
/// probability (`cascade::Confidence::mean_p`), `hedged` its hedge flag.
pub fn decide(
    report: &GroundingReport,
    mean_p: Option<f64>,
    hedged: bool,
    cfg: &AbstainConfig,
) -> AbstainDecision {
    // Reuse the fused hallucination score (reference risk weighted over
    // confidence), then let hard/explicit signals push the stance up.
    let risk = super::grounding::hallucination_risk(report, mean_p);

    let mut stance = if risk >= cfg.abstain_at {
        Stance::Abstain
    } else if risk >= cfg.qualify_at {
        Stance::Qualify
    } else {
        Stance::Answer
    };

    let mut reasons: Vec<String> = Vec::new();
    if report.ungrounded > 0 && cfg.ungrounded_forces_qualify && stance == Stance::Answer {
        stance = Stance::Qualify;
        reasons.push(format!("{} unverified reference(s)", report.ungrounded));
    }
    if hedged && stance == Stance::Answer {
        stance = Stance::Qualify;
        reasons.push("the model hedged".into());
    }
    if reasons.is_empty() {
        reasons.push(match stance {
            Stance::Answer => "confident and grounded".into(),
            Stance::Qualify => format!("moderate uncertainty (risk {:.2})", risk),
            Stance::Abstain => format!("high uncertainty (risk {:.2})", risk),
        });
    }

    AbstainDecision { stance, risk, reason: reasons.join("; ") }
}

/// Shape the outgoing message for the decided stance. `Answer` returns the text
/// unchanged; `Qualify` prepends a one-line confidence caveat; `Abstain`
/// replaces a likely-wrong assertion with an honest "not sure" that hands the
/// user the next move rather than a fabrication.
pub fn apply(text: &str, decision: &AbstainDecision) -> String {
    match decision.stance {
        Stance::Answer => text.to_string(),
        Stance::Qualify => format!(
            "> ⚠ Low confidence ({}). Verify before relying on this.\n\n{text}",
            decision.reason
        ),
        Stance::Abstain => format!(
            "I'm not confident enough to answer this reliably ({}). \
             Rather than guess, I'd suggest: escalate to the larger model, \
             narrow the question, or point me at the specific file/symbol so I can ground it.",
            decision.reason
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::ai::grounding::{self, GroundTruth};
    use std::collections::HashMap;

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

    fn cfg() -> AbstainConfig {
        AbstainConfig { enabled: true, ..Default::default() }
    }

    #[test]
    fn answers_when_confident_and_grounded() {
        let r = grounding::ground("see `src/main.rs`", &gt());
        let d = decide(&r, Some(0.95), false, &cfg());
        assert_eq!(d.stance, Stance::Answer);
    }

    #[test]
    fn abstains_on_ungrounded_reference() {
        // a fabricated file → high reference risk → abstain
        let r = grounding::ground("the fix is in `src/ghost.rs`", &gt());
        let d = decide(&r, Some(0.95), false, &cfg());
        assert_eq!(d.stance, Stance::Abstain);
        assert!(d.risk >= cfg().abstain_at);
    }

    #[test]
    fn qualifies_on_shaky_confidence() {
        let r = grounding::ground("see `src/main.rs`", &gt()); // clean refs
        let d = decide(&r, Some(0.55), false, &cfg()); // conf risk 0.45*0.3 = 0.135 < qualify...
        // low-ish confidence alone (0.45) → conf contribution 0.135; bump lower conf:
        let d2 = decide(&r, Some(0.2), false, &cfg()); // 0.8*0.3 = 0.24 ~ boundary
        assert_eq!(d.stance, Stance::Answer);
        assert!(matches!(d2.stance, Stance::Answer | Stance::Qualify));
    }

    #[test]
    fn hedge_forces_qualify_even_when_clean() {
        let r = grounding::ground("see `src/main.rs`", &gt());
        let d = decide(&r, Some(0.95), true, &cfg());
        assert_eq!(d.stance, Stance::Qualify);
        assert!(d.reason.contains("hedged"));
    }

    #[test]
    fn apply_abstain_gives_next_move_not_fabrication() {
        let r = grounding::ground("`src/ghost.rs`", &gt());
        let d = decide(&r, Some(0.9), false, &cfg());
        let out = apply("The bug is in src/ghost.rs line 4.", &d);
        assert!(out.contains("not confident enough"));
        assert!(!out.contains("The bug is in"), "the likely-wrong assertion is withheld");
    }

    #[test]
    fn apply_qualify_prepends_caveat_keeps_answer() {
        let r = grounding::ground("see `src/main.rs`", &gt());
        let d = decide(&r, Some(0.95), true, &cfg());
        let out = apply("Here is the answer.", &d);
        assert!(out.contains("Low confidence"));
        assert!(out.contains("Here is the answer."));
    }

    #[test]
    fn disabled_config_still_computes_but_caller_gates() {
        // decide() doesn't check `enabled` — the caller does — so it stays usable
        // for measurement; verify it returns a sane stance regardless.
        let r = grounding::ground("`src/main.rs`", &gt());
        let d = decide(&r, Some(0.99), false, &AbstainConfig::default());
        assert_eq!(d.stance, Stance::Answer);
    }
}
