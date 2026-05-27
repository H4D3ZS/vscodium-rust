# Antigravity IDE — Master Plan
**Goal:** Ship a native, AI-powered IDE that uses <150 MB RAM at idle and <600 MB under full AI load. No Electron. No Chrome subprocess bloat. Every feature earns its weight.

---

## STATUS LEGEND
- `[DONE]` — working
- `[PARTIAL]` — scaffold exists, incomplete
- `[STUB]` — registered but non-functional
- `[MISSING]` — does not exist yet
- `[BLOAT]` — present but hurting performance; target for removal/rewrite
- `[KILL]` — delete entirely

---

## PHASE 0 — KILL THE BLOAT (Do First)
*Target: drop idle RAM from ~1 GB to ~150 MB. No new features until this is done.*

### 0-A. Rust Backend Fat Cuts

| # | Change | Expected Saving | Effort |
|---|--------|-----------------|--------|
| 0.1 | **Remove `headless_chrome`** — spawn Chromium subprocess costs 300–500 MB. Replace with `reqwest` for scraping, `tauri-plugin-shell` for `open` calls | 300–500 MB | Medium |
| 0.2 | **Lazy-init `candle-core`** — vector embedding model (`candle`) loaded at startup; gate behind `once_cell::Lazy` and only load when `index_codebase` is first called | 100–150 MB | Low |
| 0.3 | **Persist vector index to SQLite** — current in-memory `HashMap<String, Vec<f32>>` for semantic search; serialize to `rusqlite` on disk, load only queried chunks | 50–100 MB | Medium |
| 0.4 | **Drop `screenshots` + `image` crates** — vision system is a stub; these crates pull ~25 MB and are unused | 25 MB | Low |
| 0.5 | **Drop `warp`** — `axum` already handles the proxy; two HTTP frameworks loaded simultaneously | 5–8 MB | Low |
| 0.6 | **Slim `tokio` features** — `features = ["full"]` includes io-std, test-util, tracing bridges we don't need; switch to `["rt-multi-thread", "macros", "sync", "io-util", "net", "time", "fs"]` | 5–10 MB | Low |
| 0.7 | **Trim tree-sitter language parsers** — only ship Rust, TypeScript, JavaScript, Python; remove Go/C/C++/etc. or load as optional plugins | 5–10 MB | Low |

### 0-B. JavaScript / Frontend Fat Cuts

| # | Change | Expected Saving | Effort |
|---|--------|-----------------|--------|
| 0.8 | **Lazy-import Three.js** — AIRI avatar scene loaded even when panel is closed; destroy `THREE.Scene` on panel close, reconstruct on open | 30–50 MB GPU/heap | Medium |
| 0.9 | **Remove `@mediapipe`** — hand/pose detection; vision system is stub; 19.5 MB dead weight | 19.5 MB | Low |
| 0.10 | **Remove `stats-gl`** — GPU stats overlay (Visual Lab only); move to optional debug flag | 26 MB | Low |
| 0.11 | **Code-split `reactflow`** — Visual Lab graph; lazy `import()` only when pane opens | 10 MB | Low |
| 0.12 | **Virtualize `RightSidebar` message list** — no windowing on chat history; use `@tanstack/virtual` or `react-window` so only ~20 rows are in DOM | 100–200 MB DOM heap | Medium |
| 0.13 | **Compress stored chat sessions** — agent message history is raw JSON strings in Zustand; serialize with `fflate` (zstd-wasm) before persisting | 50–100 MB | Low |
| 0.14 | **Monaco tab eviction** — no upper bound on open editor models; cap at 20 active models, dispose oldest on eviction | 50–100 MB | Low |
| 0.15 | **Xterm scroll-back cap** — each terminal instance has unlimited buffer; set `scrollback: 2000` | 20–40 MB | Low |

**Phase 0 target:** idle ~120 MB, full AI session ~400 MB.

---

## PHASE 1 — CORE IDE COMPLETENESS
*Features that any developer expects. Without these we are a demo, not an IDE.*

### 1-A. Code Editing — Missing Primitives

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.1 | **Format on save** — `rustfmt`, `prettier`, `black` via `tauri-plugin-shell` subprocess | `[MISSING]` | Wire `onDidSaveTextDocument` → shell command → apply edit |
| 1.2 | **Rename symbol** (LSP `workspace/renameSymbol`) | `[MISSING]` | LSP client exists; missing rename command handler + multi-file apply |
| 1.3 | **Extract function / extract variable** | `[MISSING]` | Could use Monaco's code action API + AI fallback |
| 1.4 | **Organize imports** (LSP code action) | `[MISSING]` | Already have `textDocument/codeAction`; just need UI trigger |
| 1.5 | **Code folding** (region markers + LSP fold ranges) | `[MISSING]` | Monaco has native support; just needs to be enabled |
| 1.6 | **Snippet system** (`.code-snippets` files) | `[MISSING]` | Monaco `CompletionItemKind.Snippet`; load from `.vscode/` dir |
| 1.7 | **Inline type hints** (inlay hints, LSP 3.17) | `[MISSING]` | `textDocument/inlayHint` — Rust rust-analyzer supports this |
| 1.8 | **Bracket pair colorization** | `[MISSING]` | Monaco built-in option: `bracketPairColorization.enabled: true` |
| 1.9 | **Multi-cursor column editing** | `[PARTIAL]` | Monaco has it; keyboard shortcut not exposed in settings yet |
| 1.10 | **Diff editor** (file compare) | `[MISSING]` | Monaco `createDiffEditor`; needs a "Compare with saved" command |

### 1-B. Debugging — Completely Missing

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.11 | **DAP client** (Debug Adapter Protocol) | `[MISSING]` | Integrate `codelldb` (Rust/C++) and `js-debug` (Node/TS); run as sidecar |
| 1.12 | **Breakpoint gutter** — click margin to toggle | `[MISSING]` | Monaco `editor.addDecoration()` for bp markers; IPC to DAP |
| 1.13 | **Step through (into/over/out)** | `[MISSING]` | DAP `next`, `stepIn`, `stepOut` commands |
| 1.14 | **Watch expressions panel** | `[MISSING]` | DAP `evaluate` request on selection |
| 1.15 | **Stack trace + call frames** | `[MISSING]` | DAP `stackTrace` response → sidebar panel |
| 1.16 | **Hover inspect (debug)** — hover variable = show value | `[MISSING]` | Monaco hover provider → DAP `evaluate` |
| 1.17 | **Debug console (REPL)** | `[MISSING]` | DAP `repl` evaluate channel; reuse terminal pane |

### 1-C. Testing — Completely Missing

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.18 | **Test explorer** — list all tests per file/project | `[STUB]` | `test_runner_commands.rs` registered; no logic |
| 1.19 | **Run / debug single test** — inline codelens | `[MISSING]` | LSP codelens + `cargo test --test <name>` subprocess |
| 1.20 | **Inline test results** (pass/fail gutter icons) | `[MISSING]` | Parse cargo test output → Monaco decorations |
| 1.21 | **Coverage gutters** (`cargo-tarpaulin` or `llvm-cov`) | `[MISSING]` | Parse lcov → line coverage decorations in Monaco |

### 1-D. Build & Task Runner

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.22 | **Task runner** — `cargo build`, `npm run`, `make` via config | `[MISSING]` | Parse `.vscode/tasks.json`; run in terminal pane |
| 1.23 | **Problem matcher** — parse compiler errors → inline diagnostics | `[PARTIAL]` | LSP diagnostics work; `cargo` stderr not piped to Monaco yet |
| 1.24 | **Build output panel** — dedicated output view | `[MISSING]` | Separate from terminal; filter by task |

### 1-E. Search & Navigation

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.25 | **Find in files** — ripgrep with regex, include/exclude | `[PARTIAL]` | `ripgrep` crate loaded; frontend panel incomplete |
| 1.26 | **Go to definition** — cross-file, multi-language | `[PARTIAL]` | LSP `textDocument/definition` registered; no jump-to-file handler |
| 1.27 | **Find all references** — sidebar panel | `[PARTIAL]` | LSP `textDocument/references` registered; no results panel |
| 1.28 | **Quick open** (Ctrl+P) — fuzzy file picker | `[PARTIAL]` | Basic file list; no frecency ranking, no symbol mode |
| 1.29 | **Go to symbol** (Ctrl+Shift+O) — in-file + workspace | `[PARTIAL]` | `documentSymbol` LSP; no fuzzy filter UI |

### 1-F. Git — Advanced Missing

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.30 | **Merge conflict resolution UI** — 3-way merge view | `[MISSING]` | Monaco diff + conflict markers; "Accept ours/theirs" actions |
| 1.31 | **Branch create/switch/delete** | `[MISSING]` | Git commands exist; no UI in SCM panel |
| 1.32 | **Stash management** — list, apply, drop stashes | `[MISSING]` | `git stash list` → SCM panel section |
| 1.33 | **Interactive rebase** — reorder/squash commits UI | `[MISSING]` | Complex; defer to Phase 3 |
| 1.34 | **Push / pull / fetch with progress** | `[PARTIAL]` | Commands exist; no progress streaming to UI |

### 1-G. Settings & Keybindings

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 1.35 | **Settings page** — full coverage of all IDE options | `[PARTIAL]` | Page exists; missing ~60% of knobs |
| 1.36 | **Keybinding editor** | `[PARTIAL]` | Registered; not wired to global key handler |
| 1.37 | **Workspace-level settings** (`.vscode/settings.json`) | `[MISSING]` | Load/merge on folder open |
| 1.38 | **Font / theme picker** | `[PARTIAL]` | Theme loads; no picker UI |

---

## PHASE 2 — AI POWER FEATURES
*What makes this IDE distinct from every Electron fork. Finish what's started.*

### 2-A. Agent Intelligence Upgrades

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 2.1 | **Multi-step autonomous task execution** — full plan → code → test → fix loop | `[PARTIAL]` | `autonomous_loop` exists; no test-run feedback cycle |
| 2.2 | **Agent-driven debugger** — AI sets breakpoints, inspects state, proposes fix | `[MISSING]` | Wire DAP + agent; biggest differentiator |
| 2.3 | **Error explanation overlay** — click compiler error → AI explains + suggests fix | `[MISSING]` | Monaco `registerCodeActionProvider` → `ai_chat` inline |
| 2.4 | **Diff review with agent** — paste PR URL or file diff → agent reviews | `[PARTIAL]` | Multi-file diff review works for local; no PR URL input |
| 2.5 | **Spec-to-PR pipeline** — write spec in chat → agent scaffolds, implements, commits | `[PARTIAL]` | Specs manager exists; commit/PR step missing |
| 2.6 | **Context-aware completions** — agent sees open files, cursor, recent edits | `[PARTIAL]` | Vector search wired; context window trimming not optimal |
| 2.7 | **Agent memory across sessions** — remember project patterns, decisions | `[PARTIAL]` | `.aim` memory format exists; read-back in prompts not always active |

### 2-B. AIRI Avatar — Complete or Streamline

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 2.8 | **Unload 3D scene when panel closed** | `[BLOAT]` | Three.js + VRM stays hot; see 0.8 |
| 2.9 | **ElevenLabs voice quality** — confirm streaming TTS path works end-to-end | `[PARTIAL]` | API call exists; audio queue not always flushed |
| 2.10 | **Emotion → animation mapping completeness** | `[PARTIAL]` | 4 emotions wired; `confusion`, `excitement`, `frustration` missing |
| 2.11 | **Optional 2D fallback** — if WebGL unavailable or user prefers lightweight | `[MISSING]` | Lottie/CSS-only avatar as fallback |
| 2.12 | **Voice input** — speech-to-text → agent chat (browser Web Speech API) | `[PARTIAL]` | Wake-word listener exists; full voice-to-text pipeline incomplete |

### 2-C. Vision System — Build or Kill

*Decision point: either ship vision or remove the dead weight (candle, screenshots, image crates).*

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 2.13 | **Screenshot → analysis** — capture screen region, send to VLM (Qwen-VL or Gemini-Vision) | `[STUB]` | Wire `screenshots` crate → base64 → `ai_chat` vision path |
| 2.14 | **UI understanding** — AI describes what it sees in dev tools / browser preview | `[STUB]` | Needs 2.13 first |
| 2.15 | **If vision deferred past Phase 2** — remove `candle-core`, `screenshots`, `image`, `@mediapipe` | `[KILL]` | Free ~200 MB binary + ~150 MB RAM |

### 2-D. Vector Search / RAG — Solidify

| # | Feature | Status | Notes |
|---|---------|--------|-------|
| 2.16 | **Incremental re-indexing** — only re-embed changed files (watch via `notify` crate) | `[MISSING]` | Currently full re-index on every call |
| 2.17 | **SQLite-backed vector store** — swap in-memory index for disk (see 0.3) | `[MISSING]` | Use `rusqlite` blob storage + simple HNSW |
| 2.18 | **Search results panel** — show semantic matches with file + line + snippet | `[PARTIAL]` | Results returned; UI panel sparse |
| 2.19 | **Context injection quality** — rank + trim by token budget before inserting to prompt | `[PARTIAL]` | Basic top-K; no token-budget awareness |

---

## PHASE 3 — POLISH & PRODUCTION
*Ship-quality reliability, performance, and UX.*

### 3-A. Performance Targets

| Metric | Current | Target |
|--------|---------|--------|
| Cold start time | ~4–6 s | <1.5 s |
| Idle RAM | 1–4 GB | <150 MB |
| AI session RAM (peak) | 4 GB | <600 MB |
| Binary size (release) | ~150 MB | <40 MB |
| First keystroke latency | ~80 ms | <20 ms |
| Agent first-token latency | ~3–5 s | <1.5 s (local) |

### 3-B. Reliability

| # | Task | Notes |
|---|------|-------|
| 3.1 | **Error boundaries on all panels** — crash in AIRI panel must not kill editor | React `ErrorBoundary` per panel |
| 3.2 | **LSP server restart on crash** — auto-respawn dead language servers | `notify` on process exit → respawn |
| 3.3 | **File watcher resilience** — handle network drives, NTFS junctions | Debounce + retry on `notify` errors |
| 3.4 | **Agent timeout + kill** — runaway `autonomous_loop` must be cancellable | `tokio::select!` + frontend cancel button |
| 3.5 | **Checkpoint integrity** — verify `.aim` checkpoint on load, skip corrupt | CRC32 check on `.aim` file open |
| 3.6 | **Undo history across sessions** — Monaco undo stack currently lost on file close | Serialize undo history to SQLite |

### 3-C. UX Polish

| # | Task | Notes |
|---|------|-------|
| 3.7 | **Welcome screen** — on first launch: open folder, clone repo, or start from spec | Replace empty editor on cold start |
| 3.8 | **Onboarding tour** — first-time use highlights AIRI, agent, diff review | Coachmark overlay |
| 3.9 | **Command palette completeness** — Ctrl+Shift+P surfaces all registered commands | Wire all `[STUB]` commands |
| 3.10 | **Status bar actions** — click on error count → opens problems panel; click model → AI settings | All status bar segments should be interactive |
| 3.11 | **Drag-to-resize all panels** — some panels (AIRI, bottom) have dead resize handles | Pointer-based drag resize everywhere |
| 3.12 | **Window title** — show `filename — project — Antigravity IDE` | Currently static |
| 3.13 | **Accessibility (a11y)** — keyboard nav for all panels, ARIA labels | No current ARIA; needed for screen readers |

### 3-D. Distribution

| # | Task | Notes |
|---|------|-------|
| 3.14 | **Release build pipeline** — GitHub Actions: `cargo build --release` + `npm run build` + `tauri build` | Currently manual |
| 3.15 | **Auto-update** — `tauri-plugin-updater` with GitHub Releases as endpoint | No auto-update yet |
| 3.16 | **Installer** (NSIS for Windows, `.dmg` for Mac, `.deb`/`.AppImage` for Linux) | Tauri bundler handles this; needs config |
| 3.17 | **Code signing** (Windows Authenticode, macOS notarization) | Unsigned binary triggers SmartScreen |
| 3.18 | **Telemetry opt-in** — anonymous crash reports + performance metrics | Privacy-first: opt-in only |

---

## DEPENDENCY VERDICT TABLE
*Every Cargo.toml dependency and its fate.*

| Crate | Verdict | Reason |
|-------|---------|--------|
| `headless_chrome` | **KILL** (Phase 0.1) | 300–500 MB Chromium. Replace with `reqwest` |
| `candle-core` | **LAZY** (Phase 0.2) | Load only when vector index first requested |
| `screenshots` | **KILL** (Phase 0.4 or 2.15) | Stub; dead weight until vision ships |
| `image` | **KILL** (same as above) | Only used by vision stub |
| `warp` | **KILL** (Phase 0.5) | Duplicate of `axum` |
| `tokio` | **SLIM** (Phase 0.6) | Remove `full`, explicit features only |
| `tree-sitter-*` | **TRIM** (Phase 0.7) | Keep Rust/TS/JS/Python only |
| `tauri` | Keep | Core runtime |
| `serde` + `serde_json` | Keep | Required everywhere |
| `reqwest` | Keep | AI provider HTTP |
| `rusqlite` | Keep | Disk storage |
| `axum` | Keep (replace warp) | Proxy / local server |
| `tokio-tungstenite` | Keep | WebSocket (Satori/AIRI) |
| `ripgrep` | Keep | Find in files |
| `tree-sitter` (core) | Keep | AST parsing |
| `notify` | Keep | File watcher |
| `diffy` | Keep | Patch application |
| `ropey` | Keep | Rope text buffer |
| `lsp-types` | Keep | LSP protocol |
| `tracing` | Keep | Logging |
| `anyhow` | Keep | Error handling |
| `walkdir` | Keep | Directory traversal |
| `uuid` | Keep | Session IDs |
| `chrono` | Keep | Timestamps |
| `rand` | Keep | Sampling |
| `sha2` | Keep | Checksum |
| `tempfile` | Keep | Safe temp writes |
| `rayon` | Keep | Parallel indexing |
| `futures` + `futures-util` | Keep | Async combinators |
| `base64` | Keep | API payloads |
| `lazy_static` | Keep | Static init |
| `lz4_flex` | Keep | `.aim` compression |
| `zip` | Keep | Extension bundling |
| `semver` | Keep | Extension version checks |
| `glob` | Keep | Pattern matching |
| `regex` | Keep | Search |
| `ignore` | Keep | .gitignore-aware walk |
| `portable-pty` | Keep | Terminal PTY |
| `tokio-util` | Keep | Stream codecs |
| `async-trait` | Keep | Trait bounds |
| `urlencoding` | Keep | URL construction |
| `tracing-chrome` | Dev only | Drop from release profile |
| `criterion` | Dev only | Bench only |
| `rmp-serde` | Dev only | Bench only |
| `candle-core` | **LAZY** | See above |
| `half` | Keep (candle dep) | Float16 math |
| `memmap2` | Keep | `.aim` mmap reads |
| `sysinfo` | Keep | System health panel |
| `which` | Keep | Binary detection |
| `streaming-iterator` | Keep | tree-sitter |
| `bytes` | Keep | HTTP bodies |
| `windows` | Keep | Win32 API |
| `hades-harness` | Keep | Core harness |
| `libaim` | Keep | Memory layer |
| `daemon` | Keep | Background daemon |

---

## EXECUTION ORDER

```
Phase 0  →  Phase 1-B (Debug)  →  Phase 1-A (Editor)
                ↓
         Phase 1-C (Testing)
                ↓
         Phase 1-D/E/F/G (Build, Search, Git, Settings)
                ↓
         Phase 2-A (Agent upgrades)
                ↓
         Phase 2-B/C (AIRI, Vision decision)
                ↓
         Phase 2-D (Vector RAG)
                ↓
         Phase 3 (Polish, distribution)
```

**Rule:** No new feature work until Phase 0 bloat cuts are done. Memory budget is a hard constraint, not a nice-to-have.

---

## MEMORY BUDGET (After All Phases)

```
Component                    Budget
─────────────────────────────────────────
Tauri runtime + WebView2     ~40 MB
Vite app (JS heap)           ~30 MB
Monaco editor (1 open file)  ~20 MB
LSP server (rust-analyzer)   ~80 MB  (external process, not counted)
AI agent (idle)              ~5 MB
Vector index (SQLite mmap)   ~10 MB
AIRI avatar (panel open)     ~30 MB  (Three.js scene)
AIRI avatar (panel closed)   ~0 MB   (scene destroyed)
Candle model (lazy, unloaded)~0 MB
Candle model (indexing active)~150 MB (temporary spike, evicted after)
Terminal (1 instance)        ~10 MB
─────────────────────────────────────────
IDLE TOTAL:                  ~135 MB  ✓
ACTIVE AI SESSION (peak):    ~400 MB  ✓
```

---

*Last updated: 2026-05-27*
*Owner: Hades (Rolando)*
