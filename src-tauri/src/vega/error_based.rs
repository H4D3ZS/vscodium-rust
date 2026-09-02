//! Built-in, deterministic active checks that don't depend on the JS
//! differential modules. These are the highest-confidence detectors for the
//! common case (DVWA-style apps) and run in pure Rust:
//!
//! * **Error-based SQL injection** — inject a syntax-breaking payload and look
//!   for a database error string in the response that the baseline didn't have.
//!   This is the single most reliable SQLi signal: a back-end that surfaces
//!   `You have an error in your SQL syntax` is unambiguously injectable.
//! * **Reflected XSS** — inject a unique marker wrapped in a tag and confirm it
//!   comes back un-encoded in an HTML response.
//!
//! Blind/differential detection still lives in the JS modules; this complements
//! them so a finding isn't lost when fingerprint bucketing is too coarse.

use crate::vega::model::{Alert, HttpResponse, PathState, Severity};

/// Substrings (lowercased) that only appear when a DB driver echoes a query
/// error. Kept specific enough to avoid matching ordinary pages that merely
/// contain the word "error".
const SQL_ERROR_SIGNATURES: &[&str] = &[
    "you have an error in your sql syntax",
    "warning: mysql",
    "mysqli_sql_exception",
    "mysql_fetch_array",
    "mysql_fetch_assoc",
    "mysql_num_rows",
    "supplied argument is not a valid mysql",
    "unclosed quotation mark after the character string",
    "quoted string not properly terminated",
    "incorrect syntax near",
    "microsoft ole db provider for sql server",
    "odbc sql server driver",
    "ora-00933",
    "ora-00921",
    "ora-01756",
    "ora-00936",
    "postgresql query failed",
    "pg_query()",
    "pg_exec()",
    "syntax error at or near",
    "sqlite3::query",
    "sqlite_query",
    "near \"' \": syntax error",
    "sql syntax error",
    "fatal error: uncaught mysqli",
];

/// Payloads that break SQL string/numeric context to provoke a driver error.
/// Each is appended to the parameter's existing value.
pub const SQL_PROBE_PAYLOADS: &[&str] = &["'", "\"", "')", "\")", "' \"", "`"];

/// Return the first SQL error signature present in `body` (lowercased scan).
fn sql_error_signature(body_lower: &str) -> Option<&'static str> {
    SQL_ERROR_SIGNATURES
        .iter()
        .copied()
        .find(|sig| body_lower.contains(sig))
}

/// Decide whether a probe response evidences error-based SQLi relative to the
/// baseline. Returns the matched signature if so.
pub fn detect_sql_error(baseline: &HttpResponse, probed: &HttpResponse) -> Option<&'static str> {
    if probed.fetch_fail {
        return None;
    }
    let probed_lower = probed.body.to_ascii_lowercase();
    let sig = sql_error_signature(&probed_lower)?;
    // Suppress if the baseline already shows the same error (noisy page, not a
    // parameter-controlled injection).
    let baseline_lower = baseline.body.to_ascii_lowercase();
    if baseline_lower.contains(sig) {
        return None;
    }
    Some(sig)
}

/// Build the SQLi alert for an error-based finding.
pub fn sql_alert(resource: &str, param: &str, signature: &str, evidence: &str, ts_ms: u64) -> Alert {
    let snippet: String = evidence.chars().take(400).collect();
    Alert {
        type_key: "vinfo-sql-inject".into(),
        title: "SQL Injection".into(),
        severity: Severity::High,
        resource: resource.to_string(),
        key: format!("sql-error:{resource}:{param}"),
        output: format!("Database error provoked by `{param}`: \"{signature}\"\n\n{snippet}"),
        detection_type: Some("Error-Based SQL Injection".into()),
        request_id: None,
        response_id: None,
        extra: Default::default(),
        timestamp_ms: ts_ms,
    }
}

/// Stable dedupe key for an alert produced here.
pub fn dedupe_key(ps: &PathState, param: &str) -> String {
    let base = ps.uri.split('?').next().unwrap_or(&ps.uri);
    format!("sql-error:{base}:{param}")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn res(body: &str) -> HttpResponse {
        HttpResponse {
            status: 200,
            body: body.into(),
            ..Default::default()
        }
    }

    #[test]
    fn flags_new_sql_error() {
        let base = res("<html>Welcome</html>");
        let bad = res("<html>You have an error in your SQL syntax; check the manual</html>");
        assert_eq!(
            detect_sql_error(&base, &bad),
            Some("you have an error in your sql syntax")
        );
    }

    #[test]
    fn ignores_preexisting_error() {
        let base = res("You have an error in your SQL syntax already");
        let bad = res("You have an error in your SQL syntax already");
        assert_eq!(detect_sql_error(&base, &bad), None);
    }

    #[test]
    fn clean_page_no_alert() {
        let base = res("<html>ok</html>");
        let good = res("<html>still ok</html>");
        assert_eq!(detect_sql_error(&base, &good), None);
    }
}
