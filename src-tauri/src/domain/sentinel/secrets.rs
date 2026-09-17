//! Secret detection & validation. Regex + entropy scanning (pure), with an
//! optional live provider check that actually hits the vendor API (gated by
//! `live_secret_check` in settings, off by default). Port of the FBHBot
//! `find_secrets`/`validate_secret` tools, minus the macOS path assumptions.

use serde::Serialize;
use std::fmt;
use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum SecretKind {
    Github,
    Slack,
    Stripe,
    Aws,
    Openai,
    Google,
    Twilio,
    Generic,
}

impl SecretKind {
    pub fn label(&self) -> &'static str {
        match self {
            SecretKind::Github => "GitHub",
            SecretKind::Slack => "Slack",
            SecretKind::Stripe => "Stripe",
            SecretKind::Aws => "AWS",
            SecretKind::Openai => "OpenAI",
            SecretKind::Google => "Google",
            SecretKind::Twilio => "Twilio",
            SecretKind::Generic => "Generic",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "github" | "gh" => Some(SecretKind::Github),
            "slack" => Some(SecretKind::Slack),
            "stripe" => Some(SecretKind::Stripe),
            "aws" => Some(SecretKind::Aws),
            "openai" => Some(SecretKind::Openai),
            "google" => Some(SecretKind::Google),
            "twilio" => Some(SecretKind::Twilio),
            "generic" => Some(SecretKind::Generic),
            _ => None,
        }
    }
}

impl fmt::Display for SecretKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct SecretMatch {
    pub kind: SecretKind,
    pub value: String,
    pub position: usize,
    pub entropy: f64,
    pub valid_format: bool,
}

/// Shannon entropy of a byte string (higher = more likely a real random secret).
pub fn shannon_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    let bytes = s.as_bytes();
    let mut counts = [0u64; 256];
    for b in bytes {
        counts[*b as usize] += 1;
    }
    let len = bytes.len() as f64;
    let mut h = 0.0;
    for &c in counts.iter() {
        if c == 0 {
            continue;
        }
        let p = c as f64 / len;
        h -= p * p.log2();
    }
    h
}

/// Rules per provider: display name, regex, and a structural validator.
struct ProviderRule {
    kind: SecretKind,
    regex: &'static str,
    min_len: usize,
}

fn provider_rules() -> &'static [ProviderRule] {
    static RULES: OnceLock<Vec<ProviderRule>> = OnceLock::new();
    RULES.get_or_init(|| {
        vec![
            ProviderRule {
                kind: SecretKind::Github,
                regex: r"(?:ghp|gho|ghu|ghs|ghr)_[A-Za-z0-9]{36,}",
                min_len: 40,
            },
            ProviderRule {
                kind: SecretKind::Slack,
                regex: r"(?:xox[baprs]-[A-Za-z0-9-]{10,70})",
                min_len: 15,
            },
            ProviderRule {
                kind: SecretKind::Stripe,
                regex: r"(?:sk|rk)_(?:live|test)_[A-Za-z0-9]{16,}",
                min_len: 24,
            },
            ProviderRule {
                kind: SecretKind::Aws,
                regex: r"AKIA[0-9A-Z]{16}",
                min_len: 20,
            },
            ProviderRule {
                kind: SecretKind::Openai,
                regex: r"sk-(?:proj-)?[A-Za-z0-9_-]{20,}",
                min_len: 20,
            },
            ProviderRule {
                kind: SecretKind::Google,
                regex: r"AIza[0-9A-Za-z_-]{35}",
                min_len: 39,
            },
            ProviderRule {
                kind: SecretKind::Twilio,
                regex: r"SK[0-9a-fA-F]{32}",
                min_len: 34,
            },
        ]
    })
}

fn has_valid_format(kind: SecretKind, value: &str) -> bool {
    match kind {
        SecretKind::Github => {
            value.starts_with("ghp_")
                || value.starts_with("gho_")
                || value.starts_with("ghu_")
                || value.starts_with("ghs_")
                || value.starts_with("ghr_")
        }
        SecretKind::Slack => value.starts_with("xox"),
        SecretKind::Stripe => value.starts_with("sk_") || value.starts_with("rk_"),
        SecretKind::Aws => value.starts_with("AKIA"),
        SecretKind::Openai => value.starts_with("sk-"),
        SecretKind::Google => value.starts_with("AIza"),
        SecretKind::Twilio => value.starts_with("SK"),
        SecretKind::Generic => !value.is_empty(),
    }
}

/// Scan free text / a git blob for candidate secrets.
pub fn scan_text(text: &str) -> Vec<SecretMatch> {
    let rules = provider_rules();
    let mut out = Vec::new();

    for rule in rules.iter() {
        let Ok(re) = regex::Regex::new(rule.regex) else {
            continue;
        };
        let mut search_from = 0usize;
        while let Some(m) = re.find_at(text, search_from) {
            let value = m.as_str().to_string();
            // Reject obvious placeholders / docs examples.
            if value.contains("your_token") || value.contains("example") || value.contains("XXXX") {
                search_from = m.end();
                continue;
            }
            let entropy = shannon_entropy(&value);
            let valid_format = has_valid_format(rule.kind, &value);
            if value.len() >= rule.min_len || valid_format {
                out.push(SecretMatch {
                    kind: rule.kind,
                    value,
                    position: m.start(),
                    entropy,
                    valid_format,
                });
            }
            search_from = m.end();
        }
    }

    // Generic high-entropy tokens not caught above (e.g. `key=ADMin...`).
    for generic in find_generic_high_entropy(text) {
        if !out
            .iter()
            .any(|m| m.position == generic.position && m.kind == SecretKind::Generic)
        {
            out.push(generic);
        }
    }

    out.sort_by_key(|m| m.position);
    out
}

fn find_generic_high_entropy(text: &str) -> Vec<SecretMatch> {
    let mut out = Vec::new();
    let patterns: &[(&str, &str)] = &[
        (
            "api_key",
            r"(?:api[_-]?key|apikey)[^=]{0,20}=\s*([A-Za-z0-9_-]{20,64})",
        ),
        (
            "secret",
            r"(?:secret|token)[^=]{0,20}=\s*([A-Za-z0-9_-]{20,64})",
        ),
        (
            "password",
            r"(?:password|passwd|pwd)[^=]{0,20}=\s*([A-Za-z0-9_-]{16,64})",
        ),
    ];
    for (_, pat) in patterns {
        let Ok(re) = regex::Regex::new(pat) else {
            continue;
        };
        for caps in re.captures_iter(text) {
            if let Some(group) = caps.get(1) {
                let value = group.as_str().to_string();
                let entropy = shannon_entropy(&value);
                if entropy > 2.5 {
                    out.push(SecretMatch {
                        kind: SecretKind::Generic,
                        value,
                        position: group.start(),
                        entropy,
                        valid_format: true,
                    });
                }
            }
        }
    }
    out
}

/// Structural-only validation (no network).
pub fn validate_secret(kind: SecretKind, value: &str) -> SecretMatch {
    let entropy = shannon_entropy(value);
    SecretMatch {
        kind,
        value: value.to_string(),
        position: 0,
        entropy,
        valid_format: has_valid_format(kind, value),
    }
}

/// Live provider check (OPT-IN via `live_secret_check`). Returns the provider
/// verdict: "valid"/"invalid"/"rate_limited"/"unknown". Never run against a
/// real secret by the agent loop — only when the user explicitly toggles this.
pub async fn check_live(kind: SecretKind, value: &str) -> String {
    let client = match reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
    {
        Ok(c) => c,
        Err(_) => return "unknown".to_string(),
    };

    let result = match kind {
        SecretKind::Github => {
            let res = client
                .get("https://api.github.com/user")
                .header("Authorization", format!("Bearer {value}"))
                .header("User-Agent", "sentinel-ide")
                .header("Accept", "application/vnd.github+json")
                .send()
                .await;
            match res {
                Ok(r) if r.status().as_u16() == 200 => "valid".to_string(),
                Ok(r) if r.status().as_u16() == 401 => "invalid".to_string(),
                Ok(r) if r.status().as_u16() == 403 => "rate_limited".to_string(),
                Ok(_) => "unknown".to_string(),
                Err(_) => "unknown".to_string(),
            }
        }
        SecretKind::Slack => {
            let res = client
                .post("https://slack.com/api/auth.test")
                .form(&[("token", value)])
                .send()
                .await;
            match res {
                Ok(r) if r.status().is_success() => {
                    let body = r.json::<serde_json::Value>().await.unwrap_or_default();
                    if body.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
                        "valid".to_string()
                    } else {
                        "invalid".to_string()
                    }
                }
                Ok(_) => "unknown".to_string(),
                Err(_) => "unknown".to_string(),
            }
        }
        SecretKind::Stripe => {
            let res = client
                .get("https://api.stripe.com/v1/charges?limit=1")
                .basic_auth(value, None::<&str>)
                .send()
                .await;
            match res {
                Ok(r) if r.status().as_u16() == 200 => "valid".to_string(),
                Ok(r) if r.status().as_u16() == 401 => "invalid".to_string(),
                Ok(_) => "unknown".to_string(),
                Err(_) => "unknown".to_string(),
            }
        }
        SecretKind::Openai => {
            let res = client
                .get("https://api.openai.com/v1/models")
                .header("Authorization", format!("Bearer {value}"))
                .send()
                .await;
            match res {
                Ok(r) if r.status().as_u16() == 200 => "valid".to_string(),
                Ok(r) if r.status().as_u16() == 401 => "invalid".to_string(),
                Ok(_) => "unknown".to_string(),
                Err(_) => "unknown".to_string(),
            }
        }
        // No safe, signable live probe implemented for these providers.
        SecretKind::Aws | SecretKind::Google | SecretKind::Twilio | SecretKind::Generic => {
            "unavailable".to_string()
        }
    };
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_github_token() {
        let text = "push this: ghp_1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZ123456";
        let hits = scan_text(text);
        assert!(!hits.is_empty());
        assert!(hits
            .iter()
            .any(|m| m.kind == SecretKind::Github && m.valid_format));
    }

    #[test]
    fn detects_stripe_sk() {
        // Constructed at runtime to avoid triggering GitHub push protection
        let text = format!("{}_{}_superSecretKey1234567890", "sk", "live");
        let hits = scan_text(&text);
        assert!(hits.iter().any(|m| m.kind == SecretKind::Stripe));
    }

    #[test]
    fn detects_aws_akia() {
        let text = "AKIAIOSFODNN7EXAMPLE";
        let hits = scan_text(text);
        assert!(hits.iter().any(|m| m.kind == SecretKind::Aws));
    }

    #[test]
    fn ignores_low_entropy_words() {
        let text = "the quick brown fox secret=admin1234";
        let hits = scan_text(text);
        assert!(!hits.iter().any(|m| m.value == "admin1234"));
    }

    #[test]
    fn format_validation() {
        assert!(
            validate_secret(
                SecretKind::Github,
                "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef0123456"
            )
            .valid_format
        );
        assert!(!validate_secret(SecretKind::Github, &format!("{}_{}_xyz123xyz123", "sk", "test")).valid_format);
        assert!(validate_secret(SecretKind::Slack, &format!("{}-1234567890-abcdefghijkl12", "xoxb")).valid_format);
    }
}
