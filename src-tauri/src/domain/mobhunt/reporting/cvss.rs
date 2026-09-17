use crate::domain::mobhunt::models::{CvssData, Severity};
use std::collections::HashMap;

pub fn calculate_cvss(vector_string: &str) -> Result<CvssData, String> {
    let clean = vector_string.trim().replace("CVSS:3.1/", "");
    let mut parts = HashMap::new();
    for component in clean.split('/') {
        if let Some((k, v)) = component.split_once(':') {
            parts.insert(k.trim().to_uppercase(), v.trim().to_uppercase());
        }
    }

    let required = ["AV", "AC", "PR", "UI", "S", "C", "I", "A"];
    for req in &required {
        if !parts.contains_key(*req) {
            return Err(format!("Missing CVSS metric: {req}"));
        }
    }

    let av: f64 = match parts.get("AV").map(|s| s.as_str()) {
        Some("N") => 0.85,
        Some("A") => 0.62,
        Some("L") => 0.55,
        Some("P") => 0.20,
        _ => return Err("Invalid AV metric".into()),
    };

    let ac: f64 = match parts.get("AC").map(|s| s.as_str()) {
        Some("L") => 0.77,
        Some("H") => 0.44,
        _ => return Err("Invalid AC metric".into()),
    };

    let scope_changed = parts.get("S").map(|s| s.as_str()) == Some("C");

    let pr: f64 = match (scope_changed, parts.get("PR").map(|s| s.as_str())) {
        (false, Some("N")) => 0.85,
        (false, Some("L")) => 0.62,
        (false, Some("H")) => 0.27,
        (true, Some("N")) => 0.85,
        (true, Some("L")) => 0.68,
        (true, Some("H")) => 0.50,
        _ => return Err("Invalid PR metric".into()),
    };

    let ui: f64 = match parts.get("UI").map(|s| s.as_str()) {
        Some("N") => 0.85,
        Some("R") => 0.62,
        _ => return Err("Invalid UI metric".into()),
    };

    let c: f64 = match parts.get("C").map(|s| s.as_str()) {
        Some("H") => 0.56,
        Some("L") => 0.22,
        Some("N") => 0.0,
        _ => return Err("Invalid C metric".into()),
    };

    let i: f64 = match parts.get("I").map(|s| s.as_str()) {
        Some("H") => 0.56,
        Some("L") => 0.22,
        Some("N") => 0.0,
        _ => return Err("Invalid I metric".into()),
    };

    let a: f64 = match parts.get("A").map(|s| s.as_str()) {
        Some("H") => 0.56,
        Some("L") => 0.22,
        Some("N") => 0.0,
        _ => return Err("Invalid A metric".into()),
    };

    let exploitability: f64 = 8.22 * av * ac * pr * ui;
    let iss: f64 = 1.0 - ((1.0 - c) * (1.0 - i) * (1.0 - a));

    let impact: f64 = if scope_changed {
        7.52 * (iss - 0.029) - 3.25 * (iss - 0.02).powf(15.0)
    } else {
        6.42 * iss
    };

    let base_score: f64 = if impact <= 0.0 {
        0.0
    } else if scope_changed {
        let raw: f64 = 1.08 * (impact + exploitability);
        let capped: f64 = raw.min(10.0);
        (capped * 10.0).ceil() / 10.0
    } else {
        let raw: f64 = impact + exploitability;
        let capped: f64 = raw.min(10.0);
        (capped * 10.0).ceil() / 10.0
    };

    let severity = if base_score == 0.0 {
        Severity::Info
    } else if base_score < 4.0 {
        Severity::Low
    } else if base_score < 7.0 {
        Severity::Medium
    } else if base_score < 9.0 {
        Severity::High
    } else {
        Severity::Critical
    };

    Ok(CvssData {
        score: base_score,
        severity,
        vector: format!("CVSS:3.1/{}", clean),
        exploitability: (exploitability * 100.0).round() / 100.0,
        impact: (impact * 100.0).round() / 100.0,
    })
}

/// Helper to get a CVSS profile for common mobile vulnerability classes
pub fn mobile_profile_cvss(profile_name: &str) -> CvssData {
    let vector = match profile_name {
        "exported_component" => "AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "exported_component_write" => "AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
        "content_provider_sqli" => "AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "url_scheme_hijack" => "AV:N/AC:L/PR:N/UI:R/S:U/C:H/I:H/A:N",
        "deep_link_hijack" => "AV:N/AC:L/PR:N/UI:R/S:U/C:L/I:L/A:N",
        "webview_js_bridge_rce" => "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:H",
        "webview_file_read" => "AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "hardcoded_aws_key" => "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:H/A:N",
        "hardcoded_api_key" => "AV:N/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "ats_disabled_mitm" => "AV:N/AC:H/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "backup_extraction" => "AV:P/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        "insecure_local_db" => "AV:L/AC:L/PR:N/UI:N/S:U/C:H/I:N/A:N",
        _ => "AV:L/AC:L/PR:N/UI:N/S:U/C:L/I:N/A:N",
    };
    calculate_cvss(vector).unwrap_or_default()
}
