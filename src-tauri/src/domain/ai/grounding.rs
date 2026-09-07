//! Hallucination grounding — verify what the model *claims* against ground truth.
//!
//! You can't eliminate hallucination, but for a coding agent its dominant form
//! is a **reference that doesn't exist**: a file, a `path:line`, or a function
//! the model never actually saw. That is deterministically checkable against the
//! index and the filesystem — no second model, no probability guesswork.
//!
//! This module extracts factual references from model output and checks each
//! against a [`GroundTruth`] oracle (the real one wraps the workspace index +
//! filesystem + LSP; tests use a mock). Ungrounded references are flagged, and
//! the answer can be annotated so the user *sees* which claims are unverified.
//!
//! It composes with the cascade's confidence signal (`cascade::Confidence`):
//! grounding catches *reference* hallucination (a wrong name), logprobs catch
//! *low-confidence* generation (a shaky span). [`hallucination_risk`] fuses them
//! into one score. Together they cover far more than either alone — though
//! neither catches a fluent-but-wrong *explanation* of code that does exist;
//! that's the honest limit of a deterministic checker.
//!
//! Opt-in via `KORTEX_GROUNDING`.

use serde::Serialize;
use std::collections::BTreeSet;

/// Oracle for "does this thing actually exist?". Implemented over the real
/// workspace; mocked in tests. Every method may answer "I can't tell"
/// (`None`) — an unverifiable claim is flagged as such, never as a hallucination.
pub trait GroundTruth {
    /// Does a file exist at this workspace-relative (or absolute) path?
    /// `None` when the path can't be resolved to a decision.
    fn file_exists(&self, path: &str) -> Option<bool>;
    /// Number of lines in the file, if known — used to catch `path:line`
    /// citations that point past the end of a real file.
    fn line_count(&self, path: &str) -> Option<usize> {
        let _ = path;
        None
    }
    /// Is this a known symbol (function/type/etc.) in the index?
    /// `None` when there's no symbol index to check against.
    fn symbol_exists(&self, name: &str) -> Option<bool> {
        let _ = name;
        None
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimKind {
    /// A file path reference, e.g. `src/main.rs`.
    FilePath,
    /// A `path:line` citation, e.g. `src/main.rs:42`.
    FileLine,
    /// A function/symbol reference, e.g. `parse_config()`.
    Symbol,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Claim {
    pub kind: ClaimKind,
    /// The path or symbol name being asserted.
    pub target: String,
    /// Line number for a `FileLine` claim.
    pub line: Option<usize>,
    /// The raw token as it appeared, for annotation.
    pub raw: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "verdict", rename_all = "snake_case")]
pub enum Verdict {
    /// Verified to exist.
    Grounded,
    /// Verified to NOT exist — a reference hallucination.
    Ungrounded { reason: String },
    /// No oracle could decide (e.g. no symbol index) — reported, not counted
    /// against the model.
    Unverifiable,
}

impl Verdict {
    pub fn is_ungrounded(&self) -> bool {
        matches!(self, Verdict::Ungrounded { .. })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct GroundingReport {
    pub claims: Vec<(Claim, Verdict)>,
    pub grounded: usize,
    pub ungrounded: usize,
    pub unverifiable: usize,
}

impl GroundingReport {
    /// Fraction of *checkable* claims that were ungrounded, in [0,1]. 0 when
    /// nothing was checkable (no evidence either way).
    pub fn reference_risk(&self) -> f32 {
        let checkable = self.grounded + self.ungrounded;
        if checkable == 0 {
            0.0
        } else {
            self.ungrounded as f32 / checkable as f32
        }
    }

    pub fn has_hallucinations(&self) -> bool {
        self.ungrounded > 0
    }
}

/// Common source-file extensions — a bare `name.ext` only counts as a file
/// claim when the extension is one of these (keeps prose like "e.g." out).
const CODE_EXTS: &[&str] = &[
    "rs", "ts", "tsx", "js", "jsx", "py", "go", "java", "c", "h", "cpp", "hpp",
    "cc", "cs", "rb", "php", "swift", "kt", "scala", "sh", "ps1", "toml", "yaml",
    "yml", "json", "md", "sql", "html", "css", "vue", "svelte", "lua", "dart",
    "ex", "exs", "clj", "hs", "ml", "zig", "proto", "gradle", "cmake",
];

/// Rust/TS keywords and common noise that look like `word(` but aren't symbols.
const NOT_SYMBOLS: &[&str] = &[
    "if", "for", "while", "match", "switch", "return", "fn", "function", "def",
    "let", "const", "var", "class", "struct", "enum", "impl", "async", "await",
    "print", "println", "console", "log", "e.g", "i.e", "etc",
];

fn ext_of(tok: &str) -> Option<&str> {
    let name = tok.rsplit(['/', '\\']).next().unwrap_or(tok);
    let ext = name.rsplit_once('.')?.1;
    if CODE_EXTS.contains(&ext.to_ascii_lowercase().as_str()) {
        Some(ext)
    } else {
        None
    }
}

fn is_path_like(tok: &str) -> bool {
    !tok.is_empty()
        && !tok.starts_with("http")
        && (tok.contains('/') || tok.contains('\\'))
        && ext_of(tok).is_some()
}

/// Split a token into a `path:line` pair if it matches (`src/x.rs:42`).
fn as_file_line(tok: &str) -> Option<(String, usize)> {
    let (path, line) = tok.rsplit_once(':')?;
    let line: usize = line.parse().ok()?;
    if ext_of(path).is_some() && (path.contains('/') || path.contains('\\') || ext_of(path).is_some())
    {
        Some((path.to_string(), line))
    } else {
        None
    }
}

/// Extract inline-code spans delimited by single backticks.
fn code_spans(text: &str) -> Vec<String> {
    let mut spans = Vec::new();
    let mut in_span = false;
    let mut cur = String::new();
    for ch in text.chars() {
        if ch == '`' {
            if in_span {
                if !cur.is_empty() {
                    spans.push(std::mem::take(&mut cur));
                }
                in_span = false;
            } else {
                in_span = true;
                cur.clear();
            }
        } else if in_span {
            cur.push(ch);
        }
    }
    spans
}

/// Extract checkable factual claims (file paths, `path:line`, `func()`) from a
/// model answer. Conservative by design: a bare `foo.rs` in prose is only a
/// claim when backtick-wrapped or path-qualified, so we don't flag "Node.js".
pub fn extract_claims(text: &str) -> Vec<Claim> {
    let mut seen: BTreeSet<(ClaimKind, String, Option<usize>)> = BTreeSet::new();
    let mut out = Vec::new();
    let mut push = |kind: ClaimKind, target: String, line: Option<usize>, raw: &str| {
        if seen.insert((kind.clone(), target.clone(), line)) {
            out.push(Claim { kind, target, line, raw: raw.to_string() });
        }
    };

    // 1. Backtick spans: a path, a path:line, or a function call.
    for span in code_spans(text) {
        let s = span.trim();
        if let Some((p, l)) = as_file_line(s) {
            push(ClaimKind::FileLine, p, Some(l), s);
        } else if ext_of(s).is_some() && !s.contains(' ') {
            push(ClaimKind::FilePath, s.to_string(), None, s);
        } else if let Some(name) = as_symbol_call(s) {
            push(ClaimKind::Symbol, name, None, s);
        }
    }

    // 2. Whitespace tokens: only slash-qualified paths / citations (high
    //    precision — avoids flagging ordinary words).
    for word in text.split_whitespace() {
        // Strip wrapping quotes/parens, then trailing sentence punctuation, but
        // never the interior `:` that separates a `path:line` citation.
        let cleaned = word
            .trim_matches(|c: char| matches!(c, '`' | '"' | '\'' | '(' | ')' | '[' | ']'))
            .trim_end_matches(|c: char| matches!(c, ',' | ';' | '!' | '?' | '.'));
        if let Some((p, l)) = as_file_line(cleaned) {
            push(ClaimKind::FileLine, p, Some(l), word);
        } else if is_path_like(cleaned) {
            push(ClaimKind::FilePath, cleaned.to_string(), None, word);
        }
    }

    out
}

/// `name(` → `name` when `name` is a plausible identifier and not a keyword.
fn as_symbol_call(s: &str) -> Option<String> {
    let open = s.find('(')?;
    let name = &s[..open];
    let name = name.rsplit(['.', ':', ' ']).next().unwrap_or(name);
    if name.len() < 3 || NOT_SYMBOLS.contains(&name.to_ascii_lowercase().as_str()) {
        return None;
    }
    let ok = name.chars().next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
    ok.then(|| name.to_string())
}

/// Check every extracted claim against the oracle.
pub fn verify(claims: Vec<Claim>, gt: &dyn GroundTruth) -> GroundingReport {
    let mut report = GroundingReport {
        claims: Vec::with_capacity(claims.len()),
        grounded: 0,
        ungrounded: 0,
        unverifiable: 0,
    };
    for claim in claims {
        let verdict = match claim.kind {
            ClaimKind::FilePath => match gt.file_exists(&claim.target) {
                Some(true) => Verdict::Grounded,
                Some(false) => Verdict::Ungrounded {
                    reason: format!("no such file: {}", claim.target),
                },
                None => Verdict::Unverifiable,
            },
            ClaimKind::FileLine => match gt.file_exists(&claim.target) {
                Some(false) => Verdict::Ungrounded {
                    reason: format!("no such file: {}", claim.target),
                },
                Some(true) => match (claim.line, gt.line_count(&claim.target)) {
                    (Some(l), Some(n)) if l > n => Verdict::Ungrounded {
                        reason: format!("{}:{} is past end of file ({} lines)", claim.target, l, n),
                    },
                    _ => Verdict::Grounded,
                },
                None => Verdict::Unverifiable,
            },
            ClaimKind::Symbol => match gt.symbol_exists(&claim.target) {
                Some(true) => Verdict::Grounded,
                Some(false) => Verdict::Ungrounded {
                    reason: format!("unknown symbol: {}", claim.target),
                },
                None => Verdict::Unverifiable,
            },
        };
        match verdict {
            Verdict::Grounded => report.grounded += 1,
            Verdict::Ungrounded { .. } => report.ungrounded += 1,
            Verdict::Unverifiable => report.unverifiable += 1,
        }
        report.claims.push((claim, verdict));
    }
    report
}

/// Convenience: extract + verify in one call.
pub fn ground(text: &str, gt: &dyn GroundTruth) -> GroundingReport {
    verify(extract_claims(text), gt)
}

/// Real oracle over a workspace: filesystem for file/line checks, plus an
/// optional in-memory symbol set (fed from the workspace index). When no symbol
/// set is supplied, symbol claims come back `Unverifiable` rather than being
/// (wrongly) called hallucinations.
pub struct WorkspaceGroundTruth {
    root: std::path::PathBuf,
    symbols: Option<std::collections::HashSet<String>>,
}

impl WorkspaceGroundTruth {
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into(), symbols: None }
    }

    /// Attach a known-symbol set (e.g. from the code index) so `func()` claims
    /// become checkable.
    pub fn with_symbols(mut self, symbols: std::collections::HashSet<String>) -> Self {
        self.symbols = Some(symbols);
        self
    }

    fn resolve(&self, path: &str) -> std::path::PathBuf {
        let p = std::path::Path::new(path);
        if p.is_absolute() {
            p.to_path_buf()
        } else {
            self.root.join(p)
        }
    }
}

impl GroundTruth for WorkspaceGroundTruth {
    fn file_exists(&self, path: &str) -> Option<bool> {
        Some(self.resolve(path).is_file())
    }

    fn line_count(&self, path: &str) -> Option<usize> {
        let full = self.resolve(path);
        // Skip pathologically large files — a citation check isn't worth reading
        // a 100 MB generated blob.
        let meta = std::fs::metadata(&full).ok()?;
        if meta.len() > 8 * 1024 * 1024 {
            return None;
        }
        let content = std::fs::read_to_string(&full).ok()?;
        Some(content.lines().count().max(1))
    }

    fn symbol_exists(&self, name: &str) -> Option<bool> {
        self.symbols.as_ref().map(|set| set.contains(name))
    }
}

/// Append a compact warning block listing the ungrounded references, so the user
/// sees exactly which claims are unverified. Returns the text unchanged when
/// there's nothing to flag.
pub fn annotate(text: &str, report: &GroundingReport) -> String {
    if !report.has_hallucinations() {
        return text.to_string();
    }
    let mut note = String::from("\n\n> ⚠ **Unverified references** (not found in this workspace):\n");
    for (claim, verdict) in &report.claims {
        if let Verdict::Ungrounded { reason } = verdict {
            note.push_str(&format!("> - `{}` — {}\n", claim.raw.trim(), reason));
        }
    }
    format!("{text}{note}")
}

/// Fuse reference-grounding with the cascade's token confidence into one
/// hallucination-risk score in [0,1]. `mean_p` is the answer's mean token
/// probability (from `cascade::Confidence`), if available.
///
/// Reference risk dominates (a wrong filename is a hard error); low confidence
/// adds a softer contribution. Either alone can raise the score.
pub fn hallucination_risk(report: &GroundingReport, mean_p: Option<f64>) -> f32 {
    let ref_risk = report.reference_risk(); // hard signal
    let conf_risk = match mean_p {
        Some(p) => (1.0 - p as f32).clamp(0.0, 1.0),
        None => 0.0,
    };
    // Hard errors weighted 0.7, soft confidence 0.3; a single ungrounded ref
    // already yields a meaningful score.
    (0.7 * ref_risk + 0.3 * conf_risk).clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    /// Mock oracle: known files (with line counts) + known symbols.
    struct Mock {
        files: HashMap<&'static str, usize>,
        symbols: HashSet<&'static str>,
        symbol_index: bool,
    }
    impl GroundTruth for Mock {
        fn file_exists(&self, path: &str) -> Option<bool> {
            Some(self.files.contains_key(path))
        }
        fn line_count(&self, path: &str) -> Option<usize> {
            self.files.get(path).copied()
        }
        fn symbol_exists(&self, name: &str) -> Option<bool> {
            if self.symbol_index {
                Some(self.symbols.contains(name))
            } else {
                None
            }
        }
    }

    fn mock() -> Mock {
        Mock {
            files: HashMap::from([("src/main.rs", 100), ("lib/util.ts", 50)]),
            symbols: HashSet::from(["parse_config", "run_cascade"]),
            symbol_index: true,
        }
    }

    #[test]
    fn extracts_paths_and_citations() {
        let text = "See `src/main.rs` and also foo/bar/baz.py in the loop, plus `src/main.rs:42`.";
        let claims = extract_claims(text);
        assert!(claims.iter().any(|c| c.kind == ClaimKind::FilePath && c.target == "src/main.rs"));
        assert!(claims.iter().any(|c| c.kind == ClaimKind::FilePath && c.target == "foo/bar/baz.py"));
        assert!(claims.iter().any(|c| c.kind == ClaimKind::FileLine && c.target == "src/main.rs" && c.line == Some(42)));
    }

    #[test]
    fn does_not_flag_prose_or_urls() {
        let text = "Node.js is great and see https://example.com/foo.rs for details.";
        let claims = extract_claims(text);
        assert!(claims.is_empty(), "should not flag Node.js or a URL, got {claims:?}");
    }

    #[test]
    fn extracts_function_calls_from_backticks() {
        let text = "Call `parse_config()` then `nonexistent_fn()`; avoid `if()`.";
        let claims = extract_claims(text);
        let syms: Vec<_> = claims.iter().filter(|c| c.kind == ClaimKind::Symbol).map(|c| c.target.as_str()).collect();
        assert!(syms.contains(&"parse_config"));
        assert!(syms.contains(&"nonexistent_fn"));
        assert!(!syms.contains(&"if"), "keyword must be skipped");
    }

    #[test]
    fn grounds_real_references() {
        let r = ground("`src/main.rs` defines `parse_config()`", &mock());
        assert_eq!(r.ungrounded, 0);
        assert!(r.grounded >= 2);
        assert!(!r.has_hallucinations());
        assert_eq!(r.reference_risk(), 0.0);
    }

    #[test]
    fn catches_missing_file() {
        let r = ground("The fix is in `src/does_not_exist.rs`", &mock());
        assert_eq!(r.ungrounded, 1);
        assert!(r.has_hallucinations());
        assert!(r.reference_risk() > 0.99);
    }

    #[test]
    fn catches_citation_past_end_of_file() {
        // main.rs has 100 lines; :500 is a fabricated citation.
        let r = ground("See `src/main.rs:500`", &mock());
        assert_eq!(r.ungrounded, 1);
        match &r.claims[0].1 {
            Verdict::Ungrounded { reason } => assert!(reason.contains("past end")),
            v => panic!("expected ungrounded, got {v:?}"),
        }
    }

    #[test]
    fn catches_unknown_symbol() {
        let r = ground("Just call `frobnicate_widget()`", &mock());
        assert_eq!(r.ungrounded, 1);
    }

    #[test]
    fn unverifiable_when_no_symbol_index() {
        let mut m = mock();
        m.symbol_index = false;
        let r = ground("Call `frobnicate_widget()`", &m);
        assert_eq!(r.ungrounded, 0, "no index → can't call it a hallucination");
        assert_eq!(r.unverifiable, 1);
    }

    #[test]
    fn annotate_lists_ungrounded_only() {
        let r = ground("Use `src/main.rs` and `src/ghost.rs`", &mock());
        let out = annotate("Use `src/main.rs` and `src/ghost.rs`", &r);
        assert!(out.contains("Unverified references"));
        assert!(out.contains("src/ghost.rs"));
        assert!(!out.contains("> - `src/main.rs`"), "grounded file must not be flagged");
    }

    #[test]
    fn annotate_noop_when_clean() {
        let text = "All good, see `src/main.rs`.";
        let r = ground(text, &mock());
        assert_eq!(annotate(text, &r), text);
    }

    #[test]
    fn fused_risk_combines_signals() {
        // clean refs, high confidence → ~0
        let clean = ground("`src/main.rs`", &mock());
        assert!(hallucination_risk(&clean, Some(0.95)) < 0.05);
        // one bad ref → dominated by the hard signal
        let bad = ground("`src/ghost.rs`", &mock());
        assert!(hallucination_risk(&bad, Some(0.95)) > 0.65);
        // clean refs but shaky confidence → moderate
        let shaky = hallucination_risk(&clean, Some(0.4));
        assert!(shaky > 0.1 && shaky < 0.3);
    }
}
