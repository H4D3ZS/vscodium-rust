# Architecture

Clean/DDD layering on both sides. Dependencies point **inward only**:

```
components / presentation  →  application  →  domain  →  infrastructure
```

| Layer | Rust (`src-tauri/src/`) | Frontend (`src/`) |
|-------|-------------------------|-------------------|
| **domain** | pure logic + stateful services. No `tauri::` imports. | entities, value objects, repository **ports** (interfaces). No React, no `@tauri-apps/api`. |
| **application** | the **only** place `#[tauri::command]` lives — thin adapters that validate, call a domain service, map errors. | **use-cases**, one file per user goal (`sendAgentTurn`, `restoreWorkspaceOnBoot`). |
| **infrastructure** | the only place that spawns processes / does HTTP / FFI / mmap / git2 / tree-sitter. | adapters — Tauri IPC, lazy legacy engines, event subscribers. |
| **presentation** | — | React UI. Reads via `src/hooks/` selectors, calls the application layer. **Never `invoke()` directly** (enforced by `scripts/check-architecture.mjs`). |

`scripts/check-architecture.mjs` runs in `npm test` and fails the build on layer violations.

---

## Composition root

| Module | Role |
|--------|------|
| `src-tauri/src/state.rs` | `EditorState` — the single Tauri managed state; every command receives `State<'_, EditorState>`. Async fields are `Arc<Mutex<…>>`, grouped into per-domain substructs (`state.ai`, `state.terminal`, …). |
| `src-tauri/src/lib.rs` | app builder + the **one** `invoke_handler![…]` registering every command; background tasks (memory watchdog, working-set trim). |
| `src-tauri/src/main.rs` | binary entry → `lib::run()`. |

---

## Backend bounded contexts (`src-tauri/src/`)

| Context | Path | What lives here |
|---------|------|-----------------|
| **AI / agent** | `domain/ai/` | `Sentient` autonomous loop, streaming, context budgeting, tool dispatch, task planner, workflow engine, verify (MCTS) harness, prompt templates |
| **Tools** | `domain/tools/` | tool implementations + the permission classifier (Safe / Dangerous) gate |
| **APEX / offensive security** | `domain/security/` | 7-specialist orchestrator, red-team scan engine (MITRE ATT&CK), finding distiller, recon, binary analysis. Analysis is `Safe`; execution stays `Dangerous`. |
| **Kortex memory / indexing** | `domain/memory/`, `domain/indexing/`, `kortex_kvcache/`, `kortex_gac/`, `kortex_retrieval/`, `kortex_vfs.rs` | `.aim` snapshot format, semantic slots + compact gist, rayon/tree-sitter parallel indexer, embedding index, KV-slot cache proxy |
| **Editor / files / LSP / patching** | `domain/editor/` | editor model, surgical SEARCH/REPLACE patch engine (`diffy`), shadow workspace, LSP client + tree-sitter diagnostics, keybinding registry |
| **Version control** | `domain/vcs/` | git2 operations, savepoints / rollback |
| **Workspace** | `domain/workspace/` | project / multi-root FS abstraction, workspace settings |
| **Mobile** | `domain/mobile/` | iPhone emulator driver (acheron / Xcode-simulator mirror), Android frame streaming, scrcpy, ADB, Gradle |
| **Extensions / marketplace / specs** | `domain/extensions/` | extension host manager, Open VSX marketplace client, specs DB, project rules engine |
| **Providers / auth / web** | `domain/services/` | model-provider resolution, OpenWebUI client, browser actuation, license activation |
| **Web scanner** | `vega/` | crawler, injection host, fingerprint, alert model (Vega-derived, EPL) |
| **Compat shims** | `domain/compat/` | temporary re-exports during layer migration — deleted as their last caller moves |
| **IDE-parity mirror** | `architecture/{domain,application,infrastructure}/` | android / gradle / test contexts modelled with explicit ports + adapters (the reference pattern for new contexts) |

Command adapters for every context are in `application/commands/*.rs`.

---

## Frontend bounded contexts (`src/`)

`domain/<ctx>/` (ports) · `application/<ctx>/` (use-cases) · `infrastructure/<ctx>/` (Tauri adapters) · `components/<ctx>/` (UI).

Contexts: `agent` · `editor` · `terminal` · `workspace` · `security` · `debug`
(DAP) · `symbols` · `performance` · `android` · `gradle` · `test` · `canvas` ·
`settings` · `research`.

Public entry points to use in new code are listed in [`src/README.md`](src/README.md).

Bridge dirs — `src/kortex/`, `src/hermes/`, `src/claurst/`, `src/airi/` — are thin
frontends over their backend/subprocess counterparts; treat them as infrastructure.

---

## Conventions (every change follows these)

### 1. Command-extraction pattern (Rust)

Every Tauri command is a thin wrapper around a testable plain function:

```rust
// application/commands/ai.rs
#[tauri::command]
pub async fn ai_chat(state: State<'_, EditorState>, req: ChatReq) -> Result<ChatResp, String> {
    crate::domain::ai::chat(&state.ai, req).await.map_err(|e| e.to_string())
}
```

Logic + `#[cfg(test)]` unit tests go in `domain/`. Nothing beyond state plumbing
and error mapping goes in a `#[tauri::command]` body.

### 2. Adding a feature

1. Put logic in the right context's **domain** service, with tests there.
2. Expose it via a thin adapter in `application/commands/<ctx>.rs`.
3. Register it in the single `invoke_handler!` in `lib.rs`.
4. Shared state → a field on the right `EditorState` substruct in `state.rs`.
5. Frontend: port in `domain/` → adapter in `infrastructure/` → use-case in
   `application/` → thin UI. Components never call `invoke()`.
6. Keep patches surgical (`patch_engine` / `diffy`) — no full-file rewrites.

### 3. Migration mechanics (keep the build green)

- Move a file with `git mv`; leave a one-line re-export shim at the old path
  (`domain/compat/`), delete shims in the final cleanup commit.
- `cargo check` + `npm run typecheck` pass after **every** commit; one concern per commit.

### 4. UI budget

- No animation libraries. CSS transitions only, on `opacity` / `color` /
  `background-color` / `transform`, max `var(--duration-base)` (150 ms). Never
  animate width/height/top/left.
- All colours via tokens in `src/styles/tokens.css` (VS Code theme-key names). No raw hex in component CSS.
- Icons: codicons (`src/codicon/`) only. No emoji, no one-off inline SVGs.
- Density: 13 px UI font, 22 px list/tree rows, 35 px tabs, 22 px status bar, 48 px activity bar.

### 5. Settings

- Every setting is declared once in `src/domain/settings/registry.ts`
  (`{ id, label, description, section, keywords, type, default, storage }`); UI,
  search, and persistence derive from it. Never add an ad-hoc `localStorage` key.
- Persistence: one `settings.json` in the Tauri config dir via
  `src/infrastructure/SettingsRepository.ts`. Secrets (API keys) stay backend-side
  in `api_keys.json`, never in renderer storage.
- Sections are fixed (8): Editor, Appearance, AI Models, Agent, Extensions,
  Privacy & Account, Keyboard, Advanced.

### 6. Verification gate (every commit)

```bash
cd src-tauri && cargo check && cargo test
npm run typecheck && npm test        # includes check-architecture.mjs
```

Risky changes also get a manual smoke: `npx tauri dev` → open a file, agent chat,
terminal, git panel, settings.

---

## Dependency direction (target)

```
application (commands, use-cases, lib.rs)
      │ calls
      ▼
domain (engines, services, ports)
      │ uses
      ▼
infrastructure (process spawn, HTTP, FFI, git2, tree-sitter, Tauri IPC)
```

Commands hold no business logic; engines never call Tauri command APIs.
`EditorState` is the only place engines are wired together.
