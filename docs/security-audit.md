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

### cargo — `reqwest` 0.11 → 0.12 (workspace-wide), then a full `cargo update`

`reqwest` bumped 0.11 → 0.12 in `src-tauri` and `kortex/{aim-proxy,daemon,harness,libaim}`
(`vfs_layer` was already on 0.12). The API surface we use — `Client`,
`Client::builder`, `redirect::Policy`, `Method::from_bytes`, `header::*`,
`Response::bytes_stream()`, `blocking` — is unchanged between 0.11 and 0.12,
so it compiled with **no code changes**. Dropping the old `hyper 0.14` /
`h2 0.3` freed the rest of the graph, and an unconstrained `cargo update`
then resolved:

| Crate | → | Advisory cleared |
|---|---|---|
| h2 `0.3.27` | removed (only `0.4.19` remains) | RUSTSEC-2026-0258 (DoS) |
| quick-xml `0.37/0.38/0.39` | `0.41.0` / `0.42.0` | RUSTSEC-2026-0194 + 0195 (DoS) |
| rand `0.7.3` | `0.8.5` / `0.9.5` | RUSTSEC-2026-0097 (unsound) |

**`cargo audit`: 9 → 0 vulnerabilities.** Only 6 `unmaintained` *warnings*
remain (`gtk` GTK3 bindings, `unic-*` unicode crates) — deep transitive,
no fix, not vulnerabilities.

Full workspace build (`cargo build --bins --workspace`) and `cargo test
--lib` (437 pass) verified after the bump.

## Accepted (no fix available)

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

### cargo — `unmaintained` warnings (not vulnerabilities)

`gtk` (GTK3 bindings, Linux, via Tauri), `unic-char-*`, `unic-common`,
`unic-ucd-*` — deep transitive, no direct dependency line to change, and
flagged `unmaintained`, not `vulnerability`. They clear when the relevant
upstreams migrate (gtk4-rs, a maintained unicode crate).

## Re-run

```
npm audit
cd src-tauri && cargo audit          # needs `cargo install cargo-audit`
```
