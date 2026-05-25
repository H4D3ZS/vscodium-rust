# Security Policy

## Supported versions

Only the latest release on `main` receives security fixes.

| Version | Supported |
|---------|-----------|
| latest  | ✅        |
| older   | ❌        |

## Reporting a vulnerability

**Please do not report security issues via public GitHub issues, discussions, or pull requests.**

Use one of the following private channels:

1. **GitHub Private Vulnerability Reporting** (preferred): open an advisory at https://github.com/feder-cr/invisible_playwright/security/advisories/new
2. **Email**: `federico.elia.majo@gmail.com` with subject prefix `[security][invisible_playwright]`

Please include:

- A clear description of the issue and impact
- Steps to reproduce (minimal repro preferred)
- The version of `invisible_playwright` and OS where it was observed
- Whether you have a suggested fix

## What to expect

- Acknowledgement of your report within **7 days**
- An initial assessment and tracking issue (private) within **14 days**
- Coordinated disclosure: a fix and public advisory are released together; reporters are credited unless they prefer to remain anonymous

## Scope

In scope:

- The Python wrapper `invisible_playwright` (this repo)
- The binary download/verification flow (SHA256 pinning, fetch endpoints)
- The CLI

Out of scope here (report to the relevant project):

- Vulnerabilities in the patched Firefox C++ source — open a private report at [feder-cr/invisible_firefox](https://github.com/feder-cr/invisible_firefox/security/advisories/new)
- Vulnerabilities in upstream Firefox / mozilla-central — report to Mozilla per https://www.mozilla.org/security/
- Vulnerabilities in third-party dependencies (`playwright`, `requests`, etc.) — report to those projects directly

## Out of scope

- Reports that the browser is detected by a specific anti-bot service — open a regular GitHub issue, this is a product-quality concern, not a security one
- Social engineering of maintainers
- Denial of service requiring physical access or local privileged access

Thank you for helping keep the project and its users safe.
