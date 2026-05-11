# Security Policy

## Supported Versions

We currently maintain only the `main` branch. Releases are tagged in GitHub.

## Reporting a Vulnerability

If you discover a security issue in **our** code (root `package.json`,
`src-tauri/Cargo.toml`, or any source file we authored), please open a private
security advisory via GitHub:
<https://github.com/H4D3ZS/vscodium-rust/security/advisories/new>

Please do **not** open a public issue for security findings.

## Dependency Scanning Policy

This monorepo vendors several upstream open-source projects as regular files
(rather than git submodules):

| Path                  | Upstream                                | Maintained by   |
|-----------------------|-----------------------------------------|-----------------|
| `airi/`               | <https://github.com/moeru-ai/airi>      | upstream        |
| `kortex/llama.cpp/`   | <https://github.com/ggerganov/llama.cpp>| upstream        |
| `claurst/kilocode/`   | (Kilo / OpenCode internal)              | upstream        |
| `kortex/`             | mixed (libaim, daemon, neuraldrive)     | us + upstream   |

GitHub's Dependabot scans every `package.json`, `Cargo.toml`, and similar
manifest in the repository — including these vendored trees. That generates
hundreds of alerts that **we cannot fix in this repository**, because the
fixes have to land upstream first.

### What we patch

- **Root `package.json`** — every advisory triaged each release. Use
  `npm audit` to verify.
- **`src-tauri/Cargo.toml`** — every advisory triaged. Use `cargo audit`
  (with `src-tauri/.cargo/audit.toml` as the suppression config) to verify.
- Any vendored code we have actually modified.

### What we do NOT patch

- Pristine vendored upstream code (we expect upstream to ship patches and we
  re-vendor).
- Advisories suppressed in `src-tauri/.cargo/audit.toml` — those entries
  carry a written rationale (e.g. unmaintained Linux-only GTK3 deps that
  never compile on our Windows/macOS targets).

### Dismissing vendored alerts

Existing dependabot alerts that point to vendored upstream code can be
bulk-dismissed using `tools/dismiss-vendored-alerts.mjs` once a personal
access token with `security_events:write` is exported as
`GITHUB_TOKEN`. The script dismisses with reason
`tolerable_risk` and comment `Vendored upstream code; tracked upstream.`

## Build-time vs Runtime Risk

Several advisories cover **build-time-only** deps (e.g. `vite`,
`vite-plugin-node-polyfills`, the `elliptic` chain). These do not run in the
production application binary — they only execute on developer/CI machines
during `npm run build`. We still patch them when fixes exist; when no
upstream fix exists we document and accept the build-time risk.
