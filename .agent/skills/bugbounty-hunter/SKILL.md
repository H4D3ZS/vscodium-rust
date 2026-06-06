---
name: bugbounty-hunter
description: Professional bug bounty methodology for authorized web/API/mobile engagements. Use when pentesting a live URL, running /bugbounty, reducing false positives, or validating findings before report. Aligns with signal-first hunter playbooks (e.g. xploiter/bugbounty-ai on Ollama).
metadata:
  author: cyber-ifrit
  version: 1.0.0
---

# Bug Bounty Hunter (Authorized Scope Only)

Reference model: [xploiter/bugbounty-ai](https://ollama.com/xploiter/bugbounty-ai:latest) — professional hunter identity, evidence-first, one vuln per report.

## When to use

- User gives a **live URL** or domain to test
- `/bugbounty`, `/bounty`, `/pentest` on external assets
- User asks to reduce false positives or write triage-ready reports
- Mobile app scope (APK/IPA) with explicit authorization

## Scope lock (non-negotiable)

1. **Parse the exact target** from the user message (e.g. `https://app.camerainstallatie.nl/`).
2. **Do not** probe `localhost`, `127.0.0.1`, or local dev ports when the target is external.
3. **Do not** guess domain spellings when DNS fails — report `TARGET_UNREACHABLE` and ask for confirmation.
4. **Do not** scan the user's LAN (`192.168.x.x`, `10.x`, `172.16–31.x`) unless internal pentest is explicitly scoped.

## Thinking model

| Principle | Meaning |
|-----------|---------|
| Signal first | Concrete behavior before claiming a bug |
| Evidence only | Request/response, status, body snippet — no speculation |
| One vuln per finding | No stacked issues in one report |
| Reproducibility | PoC must work twice; discard flaky hits |
| Triage-ready | Severity justified; fix suggested |

## Web workflow

1. **Scope** — Restate in-scope URL(s) and out-of-scope defaults.
2. **Recon** — `browser_navigate` + `web_security_audit` on the **exact** URL.
3. **Signal** — List behaviors with raw evidence (headers, cookies, API errors).
4. **Validate** — Minimal PoC per candidate; re-run to confirm.
5. **Writeup** — `reports/<target>/<finding>.md` with CVSS, impact, repro, remediation.
6. **Disclosure** — Draft for program triage; redact secrets in files.

## Finding template (mandatory per confirmed issue)

```markdown
### Signal
What concrete behavior exists (quote evidence).

### Vulnerability
One precise issue (CWE if known).

### Validation strategy
Steps to prove it for this severity.

### Commands
Exact curl/browser steps (copy-paste).

### Severity
Level + justification (not scanner default).

### Report guidance
What triage needs to accept the report.
```

## False positive filters

Discard or downgrade unless proven:

| Scanner noise | Why |
|---------------|-----|
| Missing CSP / Permissions-Policy alone | Misconfiguration, often informational unless XSS chain exists |
| SameSite=None on third-party cookies | Expected for analytics/CDN |
| Column name wrong in API probe | Schema error, not IDOR |
| `signup_disabled` | Secure config, not a finding |
| Static analysis `innerHTML` without user input path | Needs DOM XSS PoC |

Confirm before reporting: **working PoC**, **impact on scoped asset**, **not out-of-scope**.

## API / Supabase engagements

- Test **authenticated vs anon** behavior separately.
- Wrong column names → fix query, don't report as vuln.
- Empty arrays `[]` with 200 → RLS working, not exposure.
- Edge functions returning `NOT_FOUND` → enumerate only in-scope names from recon, not blind lists.

## Mobile app (APK/IPA — authorized builds only)

1. **Static** — jadx/apktool; secrets, exported components, deep links, WebView bridges.
2. **Dynamic** — mitmproxy/Burp on device/emulator; map API calls to backend scope.
3. **MASVS-aligned checks** — insecure storage, weak crypto, exported activities, intent hijacking, SSL pinning (bypass only on owned test builds).
4. **PoC on device** — screenshot/log proof; no theoretical OWASP checkbox reports.

## Tool preference order (live URL)

1. `web_security_audit` / `apex_scan_url` on exact URL
2. `browser_navigate` + `browser_read_dom`
3. `run_command` curl against **in-scope host only**
4. `deep_security_audit` on **downloaded artifacts**, not whole workspace blind scan

## On DNS / connection failure

Stop immediately:

```
TARGET_UNREACHABLE: <url> — DNS/connection failed.
Confirm the exact in-scope URL. Do not scan localhost or guess domains.
```

## Deliverables layout

```
reports/<target>/PENTEST-REPORT-<date>.md   ← master comprehensive report
reports/<target>/findings/FIND-NNN-<name>.md
recon/<target>/notes.md
exploits/<target>/poc_<name>.py
payloads/<target>/   (when relevant)
```

## Master report structure (mandatory)

1. Classification banner (CONFIDENTIAL)
2. Executive summary + risk rating table
3. Scope & rules of engagement
4. Methodology (OWASP WSTG / MITRE ATT&CK)
5. Attack narrative (kill-chain table)
6. Findings summary (ID, severity, CVSS, CWE, status)
7. Detailed findings — Signal, Evidence, Impact, PoC, Remediation, Blue-team detection
8. Remediation roadmap (P1/P2/P3)
9. Appendices

Preview in IDE: **Ctrl+Shift+V** (VS Code side-by-side) or click **Preview** on `.md` files.

## Kali / Parrot OS (Debian security distros)

Cyber-Ifrit targets ParrotSec / Kali-class environments. At engagement start:

1. Call **`sec_distro_inventory`** (native tool)
2. Use installed binaries (`nmap`, `nuclei`, `sqlmap`, `ffuf`, `bloodhound`, …) via `run_command`
3. Skill: `.agent/skills/kali-parrot-offensive/SKILL.md`

Slash: `/kali`, `/parrot`, `/bugbounty <url>`
