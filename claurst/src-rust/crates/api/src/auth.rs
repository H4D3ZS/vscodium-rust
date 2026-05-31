// auth.rs — Authentication trait for LLM provider adapters.
//
// Each provider that requires credentials implements `AuthProvider`.  The
// trait covers both API-key-based and OAuth-based flows.

use async_trait::async_trait;

use crate::provider_error::ProviderError;
use crate::provider_types::AuthMethod;

// ---------------------------------------------------------------------------
// LoginFlow
// ---------------------------------------------------------------------------

/// Describes an interactive login flow that the UI should present to the user.
#[derive(Debug, Clone)]
pub struct LoginFlow {
    /// URL to open in the browser for OAuth-based flows.  `None` for providers
    /// that use API-key-only authentication.
    pub auth_url: Option<String>,

    /// Human-readable instructions to display to the user.
    pub instructions: String,

    /// Authentication method kind: `"oauth"`, `"api_key"`, or `"none"`.
    pub method: String,
}

// ---------------------------------------------------------------------------
// AuthProvider
// ---------------------------------------------------------------------------

/// Authentication management for a single provider.
///
/// Implementors handle credential storage, token refresh, and interactive
/// login/logout flows.  The trait is intentionally coarse-grained so that
/// both simple API-key stores and full OAuth implementations can satisfy it.
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Retrieve the current credentials for this provider, refreshing OAuth
    /// tokens automatically if they are expired.
    async fn get_credentials(&self) -> Result<AuthMethod, ProviderError>;

    /// Return `true` if this provider currently has valid, usable credentials.
    ///
    /// Implementations should avoid network calls where possible (e.g. check
    /// whether a stored token is present and not obviously expired).
    async fn is_authenticated(&self) -> bool;

    /// Begin an interactive login flow.
    ///
    /// Returns a `LoginFlow` describing the URL to open (for OAuth) and/or
    /// instructions to display to the user.
    async fn login(&self) -> Result<LoginFlow, ProviderError>;

    /// Clear all stored credentials for this provider.
    async fn logout(&self) -> Result<(), ProviderError>;
}
