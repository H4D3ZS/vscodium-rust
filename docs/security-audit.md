# Dependency security audit

Snapshot of `npm audit` + `cargo audit` for the two manifests we own
(`/package.json`, `/src-tauri/Cargo.toml`). Vendored trees
(`kortex/llama.cpp`, `airi/`, `claurst/`, …) are out of scope — see the
header comment in `.github/dependabot.yml`.

Last run: this branch.

## Fixed

### npm (`npm audit fix` + one explicit bump)

| Was | Now | Advisory |
|---|---|---|
| vitest (critical — UI server arbitrary file read) | patched via lockfile | — |
| vite, postcss, esbuild, browserslist, nanoid, @babel/core, qs | patched via lockfile | multiple |
| dompurify `3.4.2` (moderate — cross-realm sanitization) inside monaco/mermaid | `overrides.dompurify = ^3.4.15`; also promoted `dompurify` to a real `dependency` (it was an undeclared direct import in `src/lib/markdown.ts` that only worked while hoisted) | GHSA cross-realm |

npm audit: **16 → 6** (all remaining are the item below).

### cargo (`cargo update -p …`, patch/minor only)

| Crate | Was → Now | Advisory |
|---|---|---|
| crossbeam-epoch | 0.9.18 → 0.9.21 | RUSTSEC-2026-0204 (invalid ptr deref) |
| h2 (0.4 line, `--precise 0.4.19`) | 0.4.15 → 0.4.19 | RUSTSEC-2026-0258 (unbounded empty DATA frames) |
| anyhow | 1.0.100 → 1.0.104 | RUSTSEC-2026-0190 (unsound downcast_mut) |
| memmap2 | 0.9.10 → 0.9.11 | RUSTSEC-2026-0186 (unchecked ptr offset) |
| event-listener | 5.4.1 → 5.4.2 | RUSTSEC-2026-0221 (!Send across threads) |
| crossbeam-deque | 0.8.6 → 0.8.8 | (transitive of crossbeam-epoch) |

cargo audit: **9 → 7** vulnerability lines (`h2 0.3.27` ×1 + `quick-xml` 0.37/0.38/0.39 ×2 advisories). All remaining need an unsafe major bump — below.

## Accepted / deferred (no safe fix available)

### npm — `elliptic` chain (6 × low, dev-only)

`elliptic` → `browserify-sign` / `create-ecdh` → `crypto-browserify` →
`node-stdlib-browser` → `vite-plugin-node-polyfills`.

- `elliptic@6.6.1` is the latest published; the "risky cryptographic
  primitive" advisory has **no patched version**.
- `vite-plugin-node-polyfills@0.28.0` and `node-stdlib-browser@1.3.1` are
  both latest and still depend on this chain.
- **Not shipped**: `vite.config.ts` only polyfills `path, buffer, stream,
  util` — `crypto` is never injected into the bundle, so `elliptic` is
  build-time dead weight, not runtime code.

`npm audit fix --force` "resolves" these by *downgrading*
`vite-plugin-node-polyfills`, which breaks the build. Left as-is; ignored
in `dependabot.yml`.

### cargo — 4 vulnerabilities behind a major bump

| Crate | Advisory | Why deferred |
|---|---|---|
| h2 `0.3.27` | RUSTSEC-2026-0258 (DoS) | Only reachable via `reqwest 0.11` → `hyper 0.14`. The 0.3 line has no patch (`>=0.4.16` required). Fix = bump `reqwest` 0.11 → 0.12 across 5 workspace crates (`src-tauri`, `kortex/{aim-proxy,daemon,harness,libaim}`) — an API-breaking change, deferred to a dedicated branch. |
| quick-xml `0.37.5` | RUSTSEC-2026-0194/0195 (DoS) | `tauri-plugin-notification` → `notify-rust` → `tauri-winrt-notification`. Upstream Tauri. |
| quick-xml `0.38.4` | RUSTSEC-2026-0194/0195 | `tauri 2.10.3` core + `plist`. Upstream Tauri. |
| quick-xml `0.39.4` | RUSTSEC-2026-0194/0195 | Linux clipboard: `arboard` → `wl-clipboard-rs` → `wayland-scanner`. Upstream. |

`quick-xml >=0.41` has API changes `plist` / `notify-rust` / `wayland-scanner`
depend on; a `[patch]` override does not compile. These clear when Tauri
and its plugins release.

### cargo — `unmaintained` / `unsound` warnings (not vulnerabilities)

`rustls-pemfile`, `unic-char-*`, `unic-common`, `unic-ucd-*`, `paste`,
`proc-macro-error`, `rand 0.7.3` — all deep transitive, no direct
dependency line to change. `rand 0.7.3` in particular is pulled by an old
transitive and would need the same kind of upstream release.

## Re-run

```
npm audit
cd src-tauri && cargo audit          # needs `cargo install cargo-audit`
```
