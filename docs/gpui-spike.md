# gpui-react Spike & Lightweight-Renderer Strategy

> Decision doc. Goal: run on a **10-year-old / 4 GB / 2-core / HDD "potato" laptop**
> and shrink the footprint, without throwing away the React + Rust-backend investment.

## TL;DR
- The recurring **"Out of Memory"** was **runaway growth in the Chromium (WebView2) renderer**, not the engine itself. The leak/growth fixes (bounded collections, decoupled `threadMessages`, debounced+capped `ai-content`, large-file guard, memory governor, bounded Rust buffers) make it **plateau** instead of climbing — that's the real fix.
- Tauri already wins on **binary size + backend RAM** (~28 MB Rust shell). On **Windows the renderer IS Chromium**, so the renderer's memory/CPU profile is Electron-class. That's the honest limitation.
- **VS Code/Cursor are also Chromium-renderer and run on potatoes** → potato support is achievable on the current stack via a **"Potato Mode"** (no rewrite). True sub-100 MB needs a **native renderer (gpui)**.
- **gpui-react** (React → gpui via Bun FFI) is the most attractive native path because it **keeps React + hooks + Zustand**. But it's early and still implies a **view-layer rewrite + a from-scratch code editor**. Treat as a **spike**, not a migration commitment.

## Why the renderer shows ~400–500 MB but Task Manager shows ~28 MB
Different processes. `performance.memory` = the **WebView2 renderer's JS heap** (`msedgewebview2.exe`, a separate process). The ~28 MB is the **Rust app shell** (`vscode-rust-app.exe`). Real total = app exe + all `msedgewebview2.exe` processes. The OOM ceiling is `jsHeapSizeLimit` (~2–4 GB), so ~400–500 MB plateauing is **healthy**.

## Track A — Potato Mode (current stack, no rewrite) — RECOMMENDED FIRST
Biggest potato killers, in order:
1. **GPU/CSS effects** (worst on old Intel HD): `backdrop-filter: blur()`, radial gradients, multi-layer `box-shadow`, CSS animations (shimmer/pulse/fade). → A `data-low-end` CSS layer that flattens all of these.
2. **CPU churn**: React re-renders + Monaco tokenization + `marked`. → `.length`-only store subscriptions (pattern already used in StatusBar), heavier throttling, fewer live indicators.
3. **Heavy modules**: `three`+VRM avatar, `reactflow`, `mermaid`+ELK. → lazy-load on demand; avatar off by default.
4. **Monaco**: minimap/bracket-colorization/sticky-scroll/codeLens off globally in low-end.
5. **RAM**: leak fixes already prevent the climb that caused HDD swapping.

Detector: `navigator.hardwareConcurrency <= 4 && (navigator.deviceMemory ?? 4) <= 4` → set `body[data-low-end]`, plus a manual Settings toggle. Realistic result: idle ~250–300 MB, smooth — VS Code territory.

**Caveat:** potato laptop = editing + **cloud** agentic AI. Local 35B models (GLM/Qwen) need a real GPU/RAM/AVX2 — route potato users to the Lemonade/Ollama **cloud** gate.

## Track B — gpui-react (native renderer, sub-100 MB) — SPIKE
`gpui-react/` (cloned): React `react-reconciler` → gpui `0.2.2` via Bun FFI.
- **Primitives:** `div` (flexbox), `text`/`span`, `input` (incl. multiline), `img`, `canvas`, `scroll`; mouse/keyboard/scroll/focus events; inline-style subset (no CSS classes).
- **Keeps:** React, hooks, Zustand, reconciler logic.
- **Loses (must rewrite):** all DOM UI — HeroUI, Tailwind/CSS, codicon/Tabler fonts, **Monaco**, **mermaid (SVG)**, **reactflow**, **three/VRM**.
- **Maturity risk:** v0.1.10, basic demos, 1 test, gpui Windows support younger than macOS.
- **Backend reuse:** the Rust command layer is reused via a **process-IPC bridge** (local socket/stdio) or FFI lib — not rewritten.

### Make-or-break unknowns
1. **Code editor** — no native editor primitive; build on `canvas` (huge) or embed a **webview island** just for the editor (hybrid).
2. **gpui 0.2.2 stability/perf on the Windows potato.**

### Spike phases (validate cheaply before committing)
- **Phase 0:** `bun run build:rust && bun run event-demo` on the oldest laptop. Measure RAM/CPU; confirm no Windows crash. Gate.
- **Phase 1:** port the **agent chat panel** (flex + text + input, no Monaco/mermaid) to gpui-react, wired to the backend over a local socket. Compare RAM/latency vs WebView on the potato.
- **Phase 2 (decision point):** prototype a **canvas code editor** (or webview-island editor). If unusable → stop; fall back to Track A.
- **Phase 3:** migrate panel-by-panel; diagrams via `canvas` or a webview island.

## Decision rule
- Need it working on potatoes **now** → **Track A (Potato Mode)**, zero rewrite.
- Want genuinely **sub-100 MB / "not Electron"** as the product identity → run the **gpui-react spike**; migrate only if Phase 1 footprint + Phase 2 editor both pass on target hardware. Keep Tauri until then.

## Status (current focus)
- ✅ **Memory leaks/growth** — comprehensively fixed (see commit `26f4a50e`); needs `dev:tauri` rebuild to take effect.
- 🔧 **Agentic tooling** — ripgrep tool + arg-alias normalization fixed (needs rebuild); continue hardening tool reliability for weaker local models.
- ⏳ **Potato Mode** — not built yet (Track A).
- ⏳ **gpui-react spike** — not started (Track B).
