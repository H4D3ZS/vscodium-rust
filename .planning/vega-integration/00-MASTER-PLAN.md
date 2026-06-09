# Vega → APEX Web Scanner: Master Integration Plan

> **Codename:** APEX Vega (web vulnerability scanner module for the IDE)
> **Goal:** Extract the full concept of Subgraph Vega (an abandoned 2016 Java/Eclipse web vuln scanner) and rebuild it as a modern, native Rust/Tauri pentesting toolkit inside this IDE.
> **Source:** `./Vega/` (last upstream commit 2016-06-29, ~10 yrs stale)
> **Status:** Phase 0 complete. See `PROGRESS.md` for live status.

---

## 🔴 AGENT HANDOFF PROTOCOL (READ FIRST)

This project is built across multiple AI sessions because the human gets rate-limited.
**Every agent working on this MUST:**

1. **Read `PROGRESS.md` first** — it is the single source of truth for "what's done / what's next."
2. **Before starting a task**, update `PROGRESS.md`: mark the task `IN PROGRESS` with your session date.
3. **After finishing a task**, update `PROGRESS.md`: mark it `DONE`, note files touched, commit hash, and any deviation.
4. **Commit after each working increment** with a clear message (the human reviews via git).
5. **If you discover something that changes the plan**, write it in `PROGRESS.md` under "Deviations & Decisions" — never silently diverge.
6. **Keep changes surgical** (per `CLAUDE.md`): SEARCH/REPLACE edits, no full-file rewrites of existing IDE code.
7. **Never break the build.** Run `cd src-tauri && cargo check` after Rust changes.

The three planning docs:
- `00-MASTER-PLAN.md` (this file) — architecture & phases. Rarely changes.
- `01-VEGA-INVENTORY.md` — exact catalog of what Vega contains + the JS API contract. Reference.
- `PROGRESS.md` — **living** task tracker + handoff log. Updated every session.

---

## Why This Matters (the gap it fills)

The IDE already has `apex_red_team.rs`, but that is **LLM-based static code analysis** — it asks Ollama "is this code vulnerable?". It cannot test a *running* web target.

Vega is the opposite and complementary: a **dynamic application security testing (DAST)** engine. It:
- **Crawls** a live target site
- Runs an **intercepting proxy** (Burp-style MITM)
- **Fuzzes** every parameter with attack payloads (XSS, SQLi, command injection, …)
- Uses **response differential fingerprinting** to detect blind vulns
- Emits structured **alerts** with severity + remediation

Integrating it turns the IDE into a genuine offensive-security workstation (fits existing **BugBounty / APEX** modes), 100% local & offline-capable (uses local `reqwest`, no cloud).

---

## Architecture (2026 Modernization)

| Vega (2016, Java/Eclipse) | APEX Vega (2026, Rust/Tauri) |
|---|---|
| Eclipse RCP plugins (OSGi) | Rust modules under `src-tauri/src/vega/` |
| Rhino JS engine (modules) | **`boa_engine`** (pure-Rust JS) — modules run UNCHANGED |
| Apache HttpClient | **`reqwest`** (already a dep) |
| db4o object DB | In-memory + optional `.aim` persistence |
| SWT/JFace UI | **React panel** `VegaScannerPanel.tsx` |
| XML alert defs | Same XML, parsed by `vega/alerts.rs` |
| 46 JS attack modules | **Same 46 JS files, copied verbatim** |

**Key design decision:** Keep Vega's **JS module format**. It is genuinely good — 46 self-contained
attack/detection scripts driven by a ~42-method API (`ctx.*`, `ps.*`). We rebuild only the *engine*
that hosts them. This means:
- We inherit all 46 modules + 85 alert definitions for free.
- New modules can be dropped in as `.js` files (great for a "weaponization toolkit").
- Community Vega modules remain compatible.

**JS engine choice:** `boa_engine` (pure Rust) for v1 → no V8/QuickJS C-toolchain pain on M1, fits the
8GB/offline/easy-build constraints. `rquickjs` (QuickJS) is the documented perf upgrade path if module
execution becomes a bottleneck under heavy fuzzing.

### Module layout (target)
```
src-tauri/src/vega/
  mod.rs          — public surface, re-exports, VegaEngine struct
  model.rs        — Request, Response, PathState, FuzzableParam, Alert types
  alerts.rs       — parse ./Vega/xml/alerts/*.xml → AlertDefinition registry
  js_runtime.rs   — boa_engine host; exposes ctx.* / ps.* API to JS modules
  engine.rs       — scan orchestrator: path states, request mutation, fingerprint differential
  crawler.rs      — async site crawler (reqwest)
  proxy.rs        — intercepting HTTP proxy (later: unify w/ kortex aim-proxy)
  fingerprint.rs  — response fingerprinting for differential blind detection
  vega_commands.rs (sibling in src/) — #[tauri::command] handlers
resources/vega/   — copied scan modules + alert XML shipped with the app
  scripts/        — from ./Vega/scripts/scanner/
  alerts/         — from ./Vega/xml/alerts/
src/components/security/VegaScannerPanel.tsx — React UI
```

---

## Phase Index

> Detailed task lists + checkboxes live in `PROGRESS.md`. This is the high-level map.

- **Phase 0 — Recon & Planning** ✅ *(this document + inventory)*
- **Phase 1 — Data Model + Alert Parser** — `model.rs`, `alerts.rs`. Pure, no network, unit-testable. Parse all 85 alert XMLs.
- **Phase 2 — JS Runtime + API Shims** — `js_runtime.rs` with `boa_engine`. Run ONE passive module (`vinfo-headers.js`) against a canned HTTP response. Proves the module-hosting model.
- **Phase 3 — HTTP Scan Engine** — `engine.rs` + `fingerprint.rs`. Real request submission via `reqwest`, altered-request mutation, response-count gating, differential fingerprint matching. Run the injection modules (XSS, SQLi) against a deliberately-vulnerable local target.
- **Phase 4 — Crawler** — `crawler.rs`. Spider a target, discover paths + fuzzable params, feed the engine.
- **Phase 5 — Intercepting Proxy** — `proxy.rs`. MITM that records traffic + replays into the scanner. (Stretch: TLS interception.)
- **Phase 6 — Tauri Commands + Wiring** — register in `lib.rs`, expose to frontend, bundle `resources/vega/` in `tauri.conf.json`.
- **Phase 7 — React UI** — `VegaScannerPanel.tsx`: target input, scan progress tree, live alert feed (severity-colored), request/response inspector. Add to activity bar.
- **Phase 8 — Agentic + APEX wiring** — expose scanner as an agent tool so the AI (in BugBounty/Sentient mode) can launch scans and reason over findings. Gate behind existing Bug-Bounty ToS + Pro entitlement.

---

## Constraints (from CLAUDE.md + this hardware)

- **M1 Mac, 8GB RAM, offline-first.** Scanner must run fully local. No cloud calls.
- **Surgical patches**, no full-file rewrites of existing IDE code.
- **Core footprint < 150MB** — scanner state is bounded; heavy crawl queues go to `kortex/daemon` or `.aim` if needed.
- **macOS-native paths** in this checkout (the CLAUDE.md says Windows but we are on darwin — use cross-platform path handling).
- **Authorization gate:** active scanning is offensive. Reuse the existing Bug-Bounty ToS acceptance (`account::require_security_suite`) before any active scan can run. Passive/proxy can be looser.

---

## Ethical / Legal Framing

This is a **defensive-security / authorized-pentest** toolkit. The IDE already ships APEX red-team +
Bug-Bounty mode behind a Terms-of-Service acceptance that requires the user to confirm authorization
(own assets or in-scope program). APEX Vega inherits that same gate. Active scanning of a target MUST
be blocked until ToS is accepted for the session. This is consistent with the existing security suite.
