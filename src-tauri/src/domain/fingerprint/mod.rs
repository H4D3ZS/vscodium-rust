//! Cross-app vulnerability fingerprint network.
//!
//! The compounding layer: a database of component fingerprints, confirmed
//! vulnerability patterns (decoupled from the app they were found in), and
//! payout history. Each new target is matched against the database so a bug
//! confirmed once can be verified in minutes on any later app sharing the same
//! component.
//!
//! A component match is always a *hypothesis*, never an auto-confirmed
//! finding — the backend integration around a shared SDK may differ. See
//! `matcher.rs` for the two-level matching algorithm and `scope.rs` for the
//! authorization gate that all matching is subject to.
//!
//! Authorized testing / bug bounty use only.

pub mod matcher;
pub mod model;
pub mod scope;
pub mod store;

pub use matcher::{component_matches, endpoint_matches, match_against, FingerprintMatch};
pub use model::{
    AppAnalysis, ComponentFingerprint, ComponentKind, ConfidenceTier, DetectionSignature,
    ReproStep, ReproTemplate, RootCause, TriggerCondition, VerificationStatus, VulnClass,
    VulnFingerprint,
};
pub use scope::Scope;
pub use store::FingerprintStore;
