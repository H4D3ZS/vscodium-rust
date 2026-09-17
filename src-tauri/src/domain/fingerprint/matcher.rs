//! Component identification and endpoint matching.
//!
//! Two-level matching, matching the design goal that a shared component does
//! not guarantee the same bug:
//!
//! 1. **Component identification** — a cheap gate: does the target's extracted
//!    artifacts (symbols, strings, hosts, request shape) match a known
//!    component signature?
//! 2. **Endpoint resolution** — does the templated root-cause endpoint pattern
//!    resolve against one of the target's *concrete* discovered endpoints?
//!
//! A match is a *hypothesis*, surfaced with a confidence score (component
//! confidence weighted by historical payout), never an auto-confirmed finding.
//!
//! The pipeline refuses out-of-scope targets unconditionally.
//!
//! Authorized testing / bug bounty use only.

use std::collections::HashMap;

use crate::domain::fingerprint::model::*;
use crate::domain::fingerprint::scope::Scope;

/// Whether a component's detection signature matches the target's artifacts.
pub fn component_matches(component: &ComponentFingerprint, analysis: &AppAnalysis) -> bool {
    match &component.signature {
        DetectionSignature::BinarySymbols { symbols } => symbols
            .iter()
            .any(|s| analysis.symbols.iter().any(|x| x.contains(s.as_str()))),
        DetectionSignature::StringConstants { constants } => constants.iter().any(|c| {
            analysis
                .string_constants
                .iter()
                .any(|x| x.contains(c.as_str()) || c.contains(x.as_str()))
        }),
        DetectionSignature::NetworkHostPattern { pattern } => {
            let suffix = pattern.trim_start_matches("*.");
            analysis
                .hosts
                .iter()
                .any(|h| h == suffix || h.ends_with(&format!(".{suffix}")))
        }
        DetectionSignature::RequestShape {
            headers,
            body_fields,
        } => {
            headers
                .iter()
                .all(|h| analysis.headers.iter().any(|x| x.eq_ignore_ascii_case(h)))
                && body_fields
                    .iter()
                    .all(|b| analysis.body_fields.iter().any(|x| x == b))
        }
    }
}

/// Match a templated pattern against a concrete endpoint path. A `{name}`
/// segment matches any single non-`/` segment; literal segments must match
/// exactly.
pub fn endpoint_matches(pattern: &str, endpoint: &str) -> bool {
    let p: Vec<&str> = pattern.split('/').collect();
    let e: Vec<&str> = endpoint.split('/').collect();
    if p.len() != e.len() {
        return false;
    }
    p.iter().zip(e.iter()).all(|(ps, es)| {
        if ps.len() >= 2 && ps.starts_with('{') && ps.ends_with('}') {
            true
        } else {
            ps == es
        }
    })
}

/// A resolved hypothesis: a vulnerability fingerprint whose endpoint pattern
/// matched a concrete endpoint on this target.
#[derive(Debug, Clone)]
pub struct FingerprintMatch {
    pub component: ComponentFingerprint,
    pub vuln: VulnFingerprint,
    pub concrete_endpoint: String,
    pub score: f32,
}

fn confidence_value(tier: ConfidenceTier) -> f32 {
    match tier {
        ConfidenceTier::Confirmed => 1.0,
        ConfidenceTier::Probable => 0.7,
        ConfidenceTier::Speculative => 0.4,
    }
}

/// Match a target against the fingerprint database.
///
/// `weights` maps a [`VulnClass`] to an expected-value multiplier derived from
/// payout history (see [`crate::domain::fingerprint::store::FingerprintStore`]);
/// absent classes default to no multiplier.
///
/// Refuses to match when the scope is empty or the target is out of scope.
pub fn match_against(
    scope: &Scope,
    analysis: &AppAnalysis,
    components: &[ComponentFingerprint],
    vulns: &[VulnFingerprint],
    weights: &HashMap<VulnClass, f32>,
) -> Vec<FingerprintMatch> {
    if scope.is_empty() || !scope.covers(&analysis.app_id, &analysis.hosts) {
        return Vec::new();
    }

    let mut out = Vec::new();
    for component in components.iter().filter(|c| component_matches(c, analysis)) {
        for vuln in vulns.iter().filter(|v| v.component_id == component.id) {
            if let Some(ep) = analysis
                .endpoints
                .iter()
                .find(|e| endpoint_matches(&vuln.root_cause.endpoint_pattern, e))
            {
                let weight = weights.get(&vuln.class).copied().unwrap_or(0.0).max(0.0);
                let score = confidence_value(vuln.confidence) * (1.0 + weight);
                out.push(FingerprintMatch {
                    component: component.clone(),
                    vuln: vuln.clone(),
                    concrete_endpoint: ep.clone(),
                    score,
                });
            }
        }
    }
    out.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stripe_component() -> ComponentFingerprint {
        ComponentFingerprint {
            id: "comp-stripe".into(),
            kind: ComponentKind::PaymentProcessor,
            name: "Stripe iOS SDK".into(),
            version_range: None,
            signature: DetectionSignature::StringConstants {
                constants: vec!["stripe.com".into()],
            },
        }
    }

    fn bola_vuln(component_id: &str) -> VulnFingerprint {
        VulnFingerprint {
            id: "v-bola".into(),
            component_id: component_id.into(),
            class: VulnClass::Bola,
            root_cause: RootCause {
                endpoint_pattern: "/api/v1/users/{id}/profile".into(),
                trigger: TriggerCondition::MissingOwnershipCheck {
                    id_field: "id".into(),
                },
                methods: vec!["GET".into()],
            },
            confidence: ConfidenceTier::Confirmed,
            first_seen_app: "com.example".into(),
            first_seen_at: 0,
            repro: ReproTemplate {
                steps: vec![ReproStep {
                    op: "ReplayRequest".into(),
                    note: "replay {base_url}/users/{id}/profile with account B token".into(),
                }],
                placeholders: vec!["{base_url}".into(), "{id}".into()],
                canary: None,
            },
        }
    }

    fn analysis_in_scope() -> AppAnalysis {
        AppAnalysis {
            app_id: "com.example.demo".into(),
            string_constants: vec!["https://api.stripe.com/v1".into()],
            hosts: vec!["api.example.com".into()],
            endpoints: vec!["/api/v1/users/42/profile".into(), "/health".into()],
            ..Default::default()
        }
    }

    #[test]
    fn component_matches_by_string_constant() {
        assert!(component_matches(&stripe_component(), &analysis_in_scope()));
    }

    #[test]
    fn endpoint_pattern_matches_placeholder() {
        assert!(endpoint_matches(
            "/api/v1/users/{id}/profile",
            "/api/v1/users/42/profile"
        ));
        assert!(!endpoint_matches(
            "/api/v1/users/{id}/profile",
            "/api/v1/orders/42/profile"
        ));
        assert!(!endpoint_matches(
            "/api/v1/users/{id}",
            "/api/v1/users/42/profile"
        ));
    }

    #[test]
    fn empty_scope_yields_no_match() {
        let scope = Scope::new("empty");
        let m = match_against(
            &scope,
            &analysis_in_scope(),
            &[stripe_component()],
            &[bola_vuln("comp-stripe")],
            &HashMap::new(),
        );
        assert!(m.is_empty());
    }

    #[test]
    fn out_of_scope_yields_no_match() {
        let mut scope = Scope::new("different program");
        scope.allow_host("other.com");
        let m = match_against(
            &scope,
            &analysis_in_scope(),
            &[stripe_component()],
            &[bola_vuln("comp-stripe")],
            &HashMap::new(),
        );
        assert!(m.is_empty());
    }

    #[test]
    fn in_scope_match_ranks_confirmed_over_speculative() {
        let mut scope = Scope::new("program");
        scope.allow_app("com.example.demo");

        let mut speculative = bola_vuln("comp-stripe");
        speculative.id = "v-bola-spec".into();
        speculative.confidence = ConfidenceTier::Speculative;

        let m = match_against(
            &scope,
            &analysis_in_scope(),
            &[stripe_component()],
            &[bola_vuln("comp-stripe"), speculative],
            &HashMap::new(),
        );
        assert_eq!(m.len(), 2);
        assert!(m[0].score > m[1].score);
        assert_eq!(m[0].concrete_endpoint, "/api/v1/users/42/profile");
    }

    #[test]
    fn payout_weight_raises_priority() {
        let mut scope = Scope::new("program");
        scope.allow_host("api.example.com");

        let base = match_against(
            &scope,
            &analysis_in_scope(),
            &[stripe_component()],
            &[bola_vuln("comp-stripe")],
            &HashMap::new(),
        );
        let mut weights = HashMap::new();
        weights.insert(VulnClass::Bola, 1.5);
        let weighted = match_against(
            &scope,
            &analysis_in_scope(),
            &[stripe_component()],
            &[bola_vuln("comp-stripe")],
            &weights,
        );
        assert!(weighted[0].score > base[0].score);
    }
}
