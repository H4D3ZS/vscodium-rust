//! Shared, compile-once secret detection patterns for chunk scans and workspace audits.

use regex::Regex;
use std::sync::OnceLock;

#[derive(Clone)]
pub struct SecretPattern {
    pub kind: &'static str,
    pub severity: &'static str,
    pub bounty_hint: &'static str,
    pub re: Regex,
}

fn compile(raw: &'static [(&'static str, &'static str, &'static str, &'static str)]) -> Vec<SecretPattern> {
    raw.iter()
        .filter_map(|(kind, sev, hint, pat)| {
            Regex::new(pat).ok().map(|re| SecretPattern {
                kind,
                severity: sev,
                bounty_hint: hint,
                re,
            })
        })
        .collect()
}

fn core_raw() -> &'static [(&'static str, &'static str, &'static str, &'static str)] {
    &[
        (
            "aws_access_key_id",
            "CRITICAL",
            "AWS AKIA in source — credential report; check S3/public bucket chain.",
            r#"\bAKIA[0-9A-Z]{16}\b"#,
        ),
        (
            "aws_secret_access_key",
            "CRITICAL",
            "AWS secret key assignment — immediate rotation required.",
            r#"(?i)aws_secret_access_key\s*[:=]\s*['"]?([A-Za-z0-9/+=]{40})['"]?"#,
        ),
        (
            "github_token",
            "CRITICAL",
            "GitHub PAT — repo/org takeover.",
            r#"\bgh[pousr]_[A-Za-z0-9]{20,}\b"#,
        ),
        (
            "stripe_secret_key",
            "CRITICAL",
            "Stripe sk_live — payment fraud; high-severity bounty.",
            r#"\bsk_(?:live|test)_[A-Za-z0-9]{20,}\b"#,
        ),
        (
            "openai_api_key",
            "CRITICAL",
            "OpenAI key exposed — billing abuse / GPT access.",
            r#"\bsk-(?:proj-)?[A-Za-z0-9_\-]{20,}\b"#,
        ),
        (
            "anthropic_api_key",
            "CRITICAL",
            "Anthropic Claude key — rotate and audit usage.",
            r#"\bsk-ant-[A-Za-z0-9\-_]{20,}\b"#,
        ),
        (
            "private_key_block",
            "CRITICAL",
            "Private key material — full compromise.",
            r#"-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----"#,
        ),
        (
            "postgres_url",
            "HIGH",
            "Database URL with credentials — pivot to data exfil.",
            r#"postgres(?:ql)?://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#,
        ),
        (
            "mongodb_url",
            "HIGH",
            "MongoDB connection string with credentials.",
            r#"mongodb(?:\+srv)?://[^\s'"@]+:[^\s'"@]+@[A-Za-z0-9\.\-]+"#,
        ),
        (
            "jwt_in_bundle",
            "HIGH",
            "Hardcoded JWT — decode for privilege escalation.",
            r#"\beyJ[A-Za-z0-9_\-]{10,}\.eyJ[A-Za-z0-9_\-]{10,}\.[A-Za-z0-9_\-]{10,}\b"#,
        ),
        (
            "generic_api_key",
            "MEDIUM",
            "Generic apiKey/accessToken assignment.",
            r#"(?i)(?:api[_-]?key|apikey|access[_-]?token|auth[_-]?token)\s*[:=]\s*['"]([A-Za-z0-9_\-/+=]{20,})['"]"#,
        ),
    ]
}

fn bundle_raw() -> &'static [(&'static str, &'static str, &'static str, &'static str)] {
    &[
        (
            "google_ai_key",
            "HIGH",
            "Google AI / Gemini key in frontend bundle.",
            r#"\bAIza[0-9A-Za-z\-_]{30,}\b"#,
        ),
        (
            "firebase_config",
            "HIGH",
            "Firebase web config — test Firestore/Storage rules.",
            r#"apiKey\s*:\s*['"][A-Za-z0-9_\-]{20,}['"]"#,
        ),
        (
            "supabase_anon",
            "HIGH",
            "Supabase key in bundle — verify RLS policies.",
            r#"supabase(?:Url|Key|AnonKey)\s*[:=]\s*['"][^'"]{10,}['"]"#,
        ),
        (
            "vite_env_leak",
            "HIGH",
            "VITE_* secret inlined at build — must not ship to browser.",
            r#"VITE_(?:API|SECRET|KEY|TOKEN|OPENAI|ANTHROPIC)[A-Z0-9_]*['"]?\s*[:=]\s*['"][^'"]{8,}['"]"#,
        ),
        (
            "next_public_secret",
            "HIGH",
            "NEXT_PUBLIC_* with sensitive value in Next.js bundle.",
            r#"NEXT_PUBLIC_[A-Z0-9_]+['"]?\s*[:=]\s*['"][^'"]{12,}['"]"#,
        ),
        (
            "react_app_secret",
            "HIGH",
            "REACT_APP_* credential in CRA bundle.",
            r#"REACT_APP_[A-Z0-9_]+['"]?\s*[:=]\s*['"][^'"]{12,}['"]"#,
        ),
        (
            "process_env_literal",
            "MEDIUM",
            "process.env.* reference left in client bundle.",
            r#"process\.env\.[A-Z0-9_]+\s*[,}\)]"#,
        ),
        (
            "dotenv_in_bundle",
            "CRITICAL",
            "Literal .env key=value inside JS chunk.",
            r#"(?i)(?:DB_PASSWORD|DATABASE_URL|JWT_SECRET|ADMIN_PASSWORD|AWS_SECRET)[^=\n]{0,20}=\s*['"]?[^'"\s]{8,}"#,
        ),
        (
            "sentry_dsn",
            "MEDIUM",
            "Sentry DSN — event injection if misconfigured.",
            r#"https?://[a-f0-9]{32}@[A-Za-z0-9\.\-]+/\d+"#,
        ),
        (
            "source_map_url",
            "INFO",
            "sourceMappingURL — fetch .map for original sources and more secrets.",
            r#"//# sourceMappingURL=([^\s'"]+\.map)"#,
        ),
    ]
}

static CORE: OnceLock<Vec<SecretPattern>> = OnceLock::new();
static BUNDLE: OnceLock<Vec<SecretPattern>> = OnceLock::new();
static SOURCE_MAP_RE: OnceLock<Regex> = OnceLock::new();
static SCRIPT_SRC_RE: OnceLock<Regex> = OnceLock::new();

pub fn core_patterns() -> &'static [SecretPattern] {
    CORE.get_or_init(|| compile(core_raw()))
}

pub fn bundle_patterns() -> &'static [SecretPattern] {
    BUNDLE.get_or_init(|| {
        let mut all = compile(core_raw());
        all.extend(compile(bundle_raw()));
        all
    })
}

pub fn source_map_re() -> &'static Regex {
    SOURCE_MAP_RE.get_or_init(|| {
        Regex::new(r#"//# sourceMappingURL=([^\s'"]+\.map)"#).expect("source map re")
    })
}

pub fn script_src_re() -> &'static Regex {
    SCRIPT_SRC_RE.get_or_init(|| {
        Regex::new(r#"(?i)<script[^>]+src=['"]([^'"]+\.js[^'"]*)['"]"#).expect("script src re")
    })
}

pub fn severity_rank(s: &str) -> u8 {
    match s {
        "CRITICAL" => 0,
        "HIGH" => 1,
        "MEDIUM" => 2,
        "LOW" => 3,
        _ => 4,
    }
}

pub fn resolve_url(base: &str, rel: &str) -> String {
    if rel.starts_with("http://") || rel.starts_with("https://") {
        return rel.to_string();
    }
    if rel.starts_with("//") {
        return format!("https:{rel}");
    }
    let base = base.trim_end_matches('/');
    if rel.starts_with('/') {
        let origin = base.split('/').take(3).collect::<Vec<_>>().join("/");
        if !origin.is_empty() {
            return format!("{origin}{rel}");
        }
    }
    format!("{base}/{rel}")
}
