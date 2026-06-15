//! Response fingerprinting for Vega differential (blind) detection.
//!
//! Vega compares fingerprints of saved responses via `ctx.isFingerprintMatch(i, j)`.
//! Two responses with the same fingerprint are considered equivalent (no differential).

use crate::vega::model::HttpResponse;

/// Normalized fingerprint of an HTTP response.
/// Matches Vega's intent: status + coarse body shape, not byte-for-byte equality.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResponseFingerprint(u64);

impl ResponseFingerprint {
    pub fn compute(response: &HttpResponse) -> Self {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();

        response.status.hash(&mut hasher);
        if response.fetch_fail {
            hasher.write(b"fetch_fail");
            return ResponseFingerprint(hasher.finish());
        }

        // Coarse length bucket (512-byte) — blind SQLi often shifts page size.
        (response.body.len() / 512).hash(&mut hasher);

        // HTML/XML tag density as a cheap structural signal.
        response.body.matches('<').count().hash(&mut hasher);

        // Common error / success markers (case-insensitive scan of first 4 KiB).
        let head = response.body.to_ascii_lowercase();
        let head = if head.len() > 4096 { &head[..4096] } else { &head };
        for marker in [
            "sql syntax",
            "mysql",
            "ora-",
            "odbc",
            "error",
            "exception",
            "warning",
            "not found",
        ] {
            if head.contains(marker) {
                marker.hash(&mut hasher);
            }
        }

        ResponseFingerprint(hasher.finish())
    }

    pub fn raw(&self) -> u64 {
        self.0
    }
}

pub fn fingerprints_match(a: &HttpResponse, b: &HttpResponse) -> bool {
    ResponseFingerprint::compute(a) == ResponseFingerprint::compute(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_body_same_fingerprint() {
        let a = HttpResponse {
            status: 200,
            body: "<html>hello</html>".into(),
            ..Default::default()
        };
        let b = a.clone();
        assert!(fingerprints_match(&a, &b));
    }

    #[test]
    fn different_status_different_fingerprint() {
        let a = HttpResponse {
            status: 200,
            body: "ok".into(),
            ..Default::default()
        };
        let b = HttpResponse {
            status: 500,
            body: "ok".into(),
            ..Default::default()
        };
        assert!(!fingerprints_match(&a, &b));
    }

    #[test]
    fn sqli_differential_pages_differ() {
        let ok = HttpResponse {
            status: 200,
            body: "users: alice,bob,charlie".repeat(20),
            ..Default::default()
        };
        let err = HttpResponse {
            status: 200,
            body: "error: no results found".into(),
            ..Default::default()
        };
        assert!(!fingerprints_match(&ok, &err));
    }
}
