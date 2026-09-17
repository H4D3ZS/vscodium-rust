//! Testing authorization scope.
//!
//! A [`Scope`] records the targets an operator is explicitly authorized to test
//! (a bug bounty program's listed app bundle-ids and backend hosts). The
//! matching/submission pipeline consults this *before* any live work, so the
//! tool refuses out-of-scope targets instead of relying on the operator to
//! remember to stay in bounds. A scope with nothing in it is treated as "no
//! authorization" and yields no matches.
//!
//! Authorized testing / bug bounty use only.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Scope {
    /// Authorized app bundle identifiers / display names.
    pub app_ids: Vec<String>,
    /// Authorized API / backend hosts (exact or suffix match).
    pub hosts: Vec<String>,
    /// Free-form provenance: program name, invite URL, listing reference.
    pub note: String,
}

impl Scope {
    pub fn new(note: impl Into<String>) -> Self {
        Self {
            app_ids: Vec::new(),
            hosts: Vec::new(),
            note: note.into(),
        }
    }

    pub fn allow_app(&mut self, id: impl Into<String>) {
        self.app_ids.push(id.into());
    }

    pub fn allow_host(&mut self, host: impl Into<String>) {
        self.hosts.push(host.into());
    }

    /// Whether `app_id` is explicitly in scope.
    pub fn app_in_scope(&self, app_id: &str) -> bool {
        self.app_ids.iter().any(|a| a == app_id)
    }

    /// Whether `host` is in scope: an exact match of an allowed host, or a
    /// subdomain of one (e.g. allowed "stripe.com" matches "api.stripe.com").
    pub fn host_in_scope(&self, host: &str) -> bool {
        self.hosts.iter().any(|allowed| {
            host == allowed
                || host
                    .strip_suffix(allowed)
                    .map(|prefix| prefix.ends_with('.') || prefix.is_empty())
                    .unwrap_or(false)
        })
    }

    /// Whether `analysis` is authorized: any matching app id or host.
    pub fn covers(&self, app_id: &str, hosts: &[String]) -> bool {
        self.app_in_scope(app_id) || hosts.iter().any(|h| self.host_in_scope(h))
    }

    /// True when the scope authorizes nothing at all.
    pub fn is_empty(&self) -> bool {
        self.app_ids.is_empty() && self.hosts.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_suffix_matching() {
        let mut s = Scope::new("test");
        s.allow_host("stripe.com");
        assert!(s.host_in_scope("api.stripe.com"));
        assert!(s.host_in_scope("stripe.com"));
        assert!(!s.host_in_scope("evilstripe.com"));
        assert!(!s.host_in_scope("notstripe.com"));
    }

    #[test]
    fn empty_scope_covers_nothing() {
        let s = Scope::new("empty");
        assert!(s.is_empty());
        assert!(!s.covers("com.example", &["api.example.com".into()]));
    }
}
