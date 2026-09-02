# IDE Comprehensive Overhaul: Structure → UI → Settings → Perf (+ Extension API)

## Context

The IDE (Tauri v2 + React 19) works but has grown organically:
- **Backend**: 170 flat Rust modules / 60K LOC, 605 tauri::commands, `EditorState` god-object (52 fields), giants like `ai_tools.rs` (8,511 LOC) and `ai_engine.rs` (7,207 LOC). Only 34/170 modules have tests because logic lives inline in `#[tauri::command]` fns.
- **Frontend**: DDD folders (`src/domain`, `src/application`, `src/infrastructure`) exist but components bypass them and call `invoke()` directly. God-components: `RightSidebar.tsx` (2,776 LOC), `AgentSettingsView.tsx` (2,003), `Editor.tsx` (1,488), `SettingsPage.tsx` (1,201 — 43 settings sections). `agentSlice.ts` is 1,267 LOC; 364 Zustand subscriptions with no `useShallow`.
- **Goal**: maintainable-without-AI codebase, VSCode-native look (no heavy animations), minimal settings UI, real extension API, then perf passes.

**User decisions**: VSCode-style minimal settings (~8 sections + Advanced); full DDD restructure both sides; order = Structure → UI → Settings → Perf; Extension API as its own milestone after.

**Handoff requirement**: every phase maintains living docs in `docs/overhaul/` so Cursor (or any agent/human) can resume mid-phase if Claude hits a rate limit.

---

## Phase 0 — Handoff documentation scaffolding (commit 1, ~30 min)

Create in-repo docs so any tool can continue the work:

- `docs/overhaul/MASTER_PLAN.md` — this entire plan, copied verbatim with the full module-mapping tables.
- `docs/overhaul/PROGRESS.md` — living checklist: one line per task, status (`todo / in-progress / done / blocked`), last-touched commit hash, and "next action" note. **Rule: update before every commit.**
- `docs/overhaul/CONVENTIONS.md` — the architecture rules (layering rules, command-wrapper pattern, invoke()-only-in-infrastructure, animation budget, settings registry pattern) written as instructions an agent can follow cold.
- Add a pointer to these in `CLAUDE.md` (also fix the stale "Windows-native" claim — current dev is macOS `mac_dev` branch).

---

## Milestone A — Structure (DDD restructure, both sides)

### A1. Backend taxonomy + migration mechanics (commits 2–10)

Target layout (new directories under `src-tauri/src/`), migrated **batch-by-batch with `cargo check` green after every batch**:

```
src-tauri/src/
├── domain/            # pure logic, no tauri:: imports
│   ├── ai/            # ai_engine.rs split: sentient.rs, streaming.rs, providers.rs, prompt.rs
│   ├── tools/         # ai_tools.rs split: registry.rs, shell.rs, web.rs, security_tools.rs, fs_tools.rs
│   ├── memory/        # memory_store, memory_layer, aim_store, memory_optimizer
│   ├── security/      # apex_*, oast, intruder, pentest_*, intercept_proxy, vega
│   ├── vcs/           # git, git_checkpoints, patch_engine, shadow_workspace
│   ├── indexing/      # context_indexer, vector_indexer, knowledge distiller
│   ├── mobile/        # ios_simulator, iphone_emulator, android, logcat, gradle
│   ├── editor/        # buffers, lsp router logic, lsp_bundle
│   └── extensions/    # extension_host, marketplace, activation
├── application/
│   └── commands/      # ALL #[tauri::command] fns, grouped by domain (ai.rs, files.rs, git.rs, …)
├── infrastructure/
│   ├── ollama.rs, http.rs, sqlite.rs, vfs_bridge.rs, browser.rs
│   └── platform/      # windows.rs, macos.rs, linux.rs (working-set trim, pressure relief)
└── state.rs           # EditorState (decomposed, see below)
```

**Mechanics that keep the build green:**
1. **Move + shim**: when `foo.rs` moves to `domain/x/foo.rs`, old path becomes `pub use crate::domain::x::foo;` (a 1-line `foo.rs` or a re-export in `lib.rs`). Delete shims only in the final cleanup commit once nothing references old paths.
2. **Command extraction pattern** (applied to every commands file as it migrates):
   ```rust
   #[tauri::command]
   pub async fn ai_chat(state: State<'_, EditorState>, req: ChatReq) -> Result<ChatResp, String> {
       crate::domain::ai::chat(&state.ai, req).await.map_err(|e| e.to_string())
   }
   ```
   Inner fn is plain Rust → unit-testable without Tauri.
3. **Batch order** (lowest risk → highest):
   1. Leaf utility modules (no cross-deps) → `infrastructure/` & `domain/` (≈40 small files)
   2. Domain groups: vcs → mobile → security → memory → indexing → editor/lsp → extensions
   3. Split the giants last, when their dependencies are already in place: `ai_tools.rs` (8.5K → ~6 files in `domain/tools/`), `ai_engine.rs` (7.2K → ~4 files in `domain/ai/`), `ai_commands.rs` (1.6K → thin wrappers in `application/commands/ai.rs`)
   4. Cleanup commit: delete shims, run `cargo check` + `cargo test`.
4. **EditorState decomposition** (no touching 605 call sites at once): group the 52 fields into per-domain substructs kept inside the single managed state:
   ```rust
   pub struct EditorState {
       pub ai: AiState,            // ai_engine, ai_tools, models, advisor
       pub editor: EditorCore,     // buffers, active_path, lsp_router, diagnostics
       pub terminal: TerminalState,
       pub memory: MemoryState,
       pub security: SecurityState,
       pub mobile: MobileState,
       pub ext: ExtensionState,
       pub services: ServiceState, // auth, account, browser, mcp, perf_monitor
   }
   ```
   Commands migrate from `state.ai_engine` → `state.ai.engine` per batch. `kortex/daemon` (imported, zero callers) gets dropped from `src-tauri/Cargo.toml`.

### A2. Frontend restructure (commits 11–18)

1. **Consolidate stray folders**: fold `src/agent`, `src/architecture`, `src/services`, `src/utils`, `src/security`, `src/mcp` contents into `domain/` / `application/` / `infrastructure/` / `lib/` as appropriate (re-export from old paths during transition, delete after typecheck passes). `src/kortex`, `src/airi`, `src/hermes`, `src/claurst` stay (distinct subsystems) but get README.md headers explaining their role.
2. **Split god-components** (each becomes a folder of lazy-loaded sub-panels):
   - `RightSidebar.tsx` (2,776) → `components/rightSidebar/{RightSidebar.tsx (router shell ≤200 LOC), AgentPanel.tsx, AiriPanel.tsx, EmulatorPanel.tsx, SecurityPanel.tsx, MlPanel.tsx, …}` — each `React.lazy`, following the existing Workbench lazy pattern.
   - `AgentSettingsView.tsx` (2,003) → per-concern files under `components/settings/agent/`.
   - `Editor.tsx` (1,488) → extract `editorDecorations.ts`, `editorKeybindings.ts`, `monacoSetup.ts` into `application/editor/`.
3. **Split `agentSlice.ts`** (1,267) → `agentMessagesSlice.ts`, `agentToolsSlice.ts`, `agentModesSlice.ts` (same store, three slices — no persistence shape change).
4. **Enforce layering with a script** (no ESLint in repo; cheapest enforceable check): `scripts/check-architecture.mjs` — fails if `@tauri-apps/api` / `invoke(` is imported anywhere under `src/components` or `src/store` (allowlist: `src/infrastructure`, `src/tauri_api_shim`). Wire into `npm test`. Migrate violations batch-by-batch by routing through existing adapters in `src/infrastructure/` (e.g., `TauriFileRepository`).
5. **Hooks layer**: add `src/hooks/` selector hooks wrapping `useStore` with `useShallow` (`useAgentMessages()`, `useActiveTab()`, `useLayout()` …). Components consume hooks, never raw selectors. (Full sweep happens in Perf phase; the hooks land here.)

**Verification A**: `cargo check && cargo test` per batch; `npm run typecheck && npm test` per batch; `npx tauri dev` smoke (open file, chat with agent, terminal, git panel). Update `PROGRESS.md` per commit.

---

## Milestone B — VSCode-native UI polish (commits 19–23)

The shell already mirrors VSCode (ActivityBar/Sidebar/Editor/Panel/StatusBar). This phase is refinement, not redesign:

1. **Animation purge**: remove `framer-motion` entirely (4 usages → CSS); audit the ~295 transition usages — anything > `--duration-base` (150ms) or animating layout (width/height/top) gets clamped or removed; keep only opacity/background/transform ≤150ms. Drop `framer-motion` from package.json.
2. **Token consolidation** (`src/styles/tokens.css`): move the hardcoded VSCode Dark+ palette out of `styles.css` into tokens as the canonical semantic set (`--color-sideBar-background`, `--color-list-hoverBackground`, etc., named after VSCode theme keys so extension themes map 1:1). All component CSS references tokens only — no raw hex in components.
3. **Density & metrics to VSCode spec**: 13px UI font, 22px list/tree rows, 35px tab height, 22px status bar, 48px activity bar; consistent 4/8px spacing from existing `--space-*`.
4. **Native details**: thin overlay scrollbars (fade-in on hover, like VSCode), `:focus-visible` outline using `--color-accent` (1px, no glow/shadows), codicons (already in `src/codicon/`) everywhere — replace any emoji/inline-SVG one-offs in panels.
5. **Light theme + High Contrast audit** so the token layer provably supports theming.

**Verification B**: visual smoke in `npx tauri dev` against VSCode side-by-side; `npm run typecheck`; grep CI check: no `framer-motion` import, no `transition.*[3-9][0-9]{2}ms`.

---

## Milestone C — Settings redesign (commits 24–28)

### Information architecture (43 → 8 sections)

| New section | Absorbs (old sections) |
|---|---|
| **Editor** | Editor, Theme→(moved to Appearance), Keyboard→own, Run Configs, Workspace |
| **Appearance** | Theme, density, AIRI avatar visibility toggle |
| **AI Models** | Models & API Keys (15 providers), Ollama, Model Selection, Reasoning |
| **Agent** | Chat & Agent, Permissions, Skill Store, Specs & Workflow |
| **Extensions** | Extension Modules, Skill Store install, Platform Status |
| **Privacy & Account** | Account, Enterprise, Privacy/telemetry |
| **Keyboard** | Keyboard Shortcuts |
| **Advanced** | APEX engines, Sentient Core/AIRI, ANE, Kortex/AIM, Memory, Voice & TTS, Steering, Hooks, Lifecycle Hooks, LSP servers, PyTorch ML Studio, MCP |

### Implementation

1. **Declarative settings registry** — `src/domain/settings/registry.ts`: every setting = `{ id, label, description, section, keywords, type, default, storage }`. This is the single source of truth; sections/search/rendering all derive from it. Most generic rows render from schema; complex panels (provider keys, MCP, hooks) stay as `customRender` components but are *registered* with keywords so search finds them.
2. **Search like VSCode**: top search box filters the registry (label+keywords+section, simple substring + fuzzy ranking via existing utils); matching settings render flat with section breadcrumbs.
3. **Persistence consolidation**: single `settings.json` in the Tauri config dir, read/written via one infrastructure adapter (`src/infrastructure/SettingsRepository.ts` → new/existing backend command). Zustand `settingsSlice` becomes a pure in-memory cache hydrated at boot. One-time migration sweeps known `localStorage` keys (`tab.predictionEnabled`, `void.globalSettings`, `reasoning.budget`, …) into the file, then removes them. **Exception**: API keys stay in backend `api_keys.json` (secrets stay out of the renderer).
4. **Rebuild `SettingsPage.tsx`** as a ≤300 LOC shell: sidebar (8 sections), search, virtualized settings list; sub-panels lazy-loaded. Old per-domain panels under `components/settings/` are reused, slotted into the new sections.

**Verification C**: settings round-trip test (change → restart app → persisted); search finds an Advanced setting (e.g., "thermal"); `npm test` (add registry unit tests); migration test from seeded localStorage.

---

## Milestone D — Performance (commits 29–35)

Backend (priority order):
1. `terminal_pending: std::sync::Mutex<HashMap<…>>` hot loop (`state.rs`) → `tokio::sync::mpsc` ring per terminal; frontend drains via non-blocking take.
2. `vfs_bridge.rs` ~20 `std::fs::` calls in async paths → `tokio::fs` / `spawn_blocking`.
3. Lazy-init `VectorIndexer`, `ContextIndexer`, `KnowledgeDistiller` (currently eager in `EditorState::new`, ~200ms+ cold boot, worse on 8GB M1) → `OnceCell` initialized on first use; consolidate vector-indexer double-init fallback to one path.
4. Lock diet in `domain/ai/`: read-mostly data (tool schemas, model catalogs, extension manifests) → `RwLock` or immutable `Arc`; activity logging → mpsc channel instead of locking in the chat loop.
5. macOS release profile experiment: try `lto = "thin"` on the `mac_dev` branch (the `lto=false`/`codegen-units=16` settings were Windows-linker workarounds); measure build success + binary size, keep only if green.

Frontend:
6. `useShallow` sweep over the 364 store subscriptions, using the hooks layer from A2.5; `React.memo` the new RightSidebar sub-panels.
7. Verify chunking after restructure (`npx vite build` → inspect; three/vrm/xterm/reactflow should not land in the entry chunk).

**Verification D**: before/after measurements recorded in `PROGRESS.md` — app start to interactive, `EditorState::new` duration (add a `tracing` span), terminal echo latency, React DevTools re-render counts on agent message stream.

---

## Milestone E — Extension API (commits 36+, separate milestone)

Build on the existing process-based host (`extension_host.rs`, `ExtHostBridge.ts`, VSIX install, contributions registry):

1. **Host model**: keep the **sidecar process** approach (already how `ExtensionHostManager` works) rather than a hidden webview — extensions run in a Node/Deno sidecar with a JSON-RPC bridge over the existing `ext_host_send/receive` IPC. Sandboxing = capability-scoped API surface + per-extension permission prompts reusing the existing `tool_permission_senders` flow.
2. **Typed API package**: `packages/hades-extension-api/` — published `.d.ts` + runtime shim. v1 surface (deliberately small): `commands.register`, `window.showMessage/StatusBarItem`, `workspace.fs (scoped)`, `workspace.onDidChangeTextDocument`, `languages.registerCompletionProvider` (bridges to existing `extHostProviders.ts`), `themes` (already works via VSIX), `settings.get/onChange` (reads the Milestone C registry).
3. **Contribution points** (declared in manifest, validated in Rust): commands, keybindings, themes, languages/grammars, views (sidebar panels), settings.
4. **Docs**: `docs/extensions/API.md` + a sample extension in `examples/hello-extension/`.
5. Harden marketplace path: Open VSX gallery for `search_extensions`, install error states in `ExtensionsView`.

**Verification E**: sample extension installs from folder + VSIX, contributes a command + status bar item + theme, survives app restart, denied capability shows permission prompt.

---

## Verification (global, every commit)

```bash
cd src-tauri && cargo check && cargo test     # backend green
npm run typecheck && npm test                  # frontend green (includes check-architecture.mjs)
npx tauri dev                                  # smoke: open file, agent chat, terminal, git, settings
```

Plus: update `docs/overhaul/PROGRESS.md` before every commit (status + next action) — this is the Cursor-handoff contract.

## Critical files

Backend: `src-tauri/src/lib.rs`, `state.rs`, `ai_tools.rs`, `ai_engine.rs`, `ai_commands.rs`, `vfs_bridge.rs`, `src-tauri/Cargo.toml`
Frontend: `src/components/RightSidebar.tsx`, `SettingsPage.tsx`, `AgentSettingsView.tsx`, `Editor.tsx`, `src/store/agentSlice.ts`, `src/store/settingsSlice.ts`, `src/styles/tokens.css`, `src/styles.css`, `vite.config.mjs`, `package.json`
New: `docs/overhaul/{MASTER_PLAN,PROGRESS,CONVENTIONS}.md`, `scripts/check-architecture.mjs`, `src/domain/settings/registry.ts`, `src/infrastructure/SettingsRepository.ts`, `packages/hades-extension-api/`
