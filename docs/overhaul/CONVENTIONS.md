# Architecture Conventions (Overhaul)

Rules any agent or human must follow when working on this codebase mid-overhaul.
Read `MASTER_PLAN.md` for the why; this file is the how. Update `PROGRESS.md` before every commit.

---

## 1. Layering (both sides)

```
components/UI  →  application  →  domain  →  infrastructure
```

- **domain**: pure logic. Rust: no `tauri::` imports. TS: no `@tauri-apps/api`, no React.
- **application**: use-case orchestration. Rust: the ONLY place `#[tauri::command]` lives. TS: handlers/sessions that call infrastructure adapters.
- **infrastructure**: the ONLY place that touches Tauri IPC, HTTP, SQLite, filesystem.
- **components (TS)**: render + local UI state only. Get data via hooks (`src/hooks/`) and application handlers. Never `invoke()` directly — enforced by `scripts/check-architecture.mjs` (runs in `npm test`).

## 2. Rust command-extraction pattern

Every Tauri command is a thin wrapper around a testable plain function:

```rust
// application/commands/ai.rs
#[tauri::command]
pub async fn ai_chat(state: State<'_, EditorState>, req: ChatReq) -> Result<ChatResp, String> {
    crate::domain::ai::chat(&state.ai, req).await.map_err(|e| e.to_string())
}
```

- Logic goes in `domain/`, gets `#[cfg(test)]` unit tests there.
- Never put business logic inside the `#[tauri::command]` fn body beyond state plumbing + error mapping.

## 3. Rust migration mechanics (keep the build green)

1. Move file with `git mv` into the new layer folder.
2. Leave a 1-line shim at the old path: `pub use crate::domain::x::foo::*;` (or re-export in `lib.rs`).
3. `cargo check` must pass after EVERY batch. One batch = one commit.
4. Shims are deleted only in the final A1 cleanup commit.
5. `EditorState` fields migrate into per-domain substructs (`state.ai.engine`, `state.terminal.masters`, …). Old top-level fields may temporarily coexist; remove them as their last caller migrates.

## 4. Frontend migration mechanics

1. Splitting a god-component: create `components/<name>/` folder; the original file becomes a ≤200 LOC router shell; sub-panels are `React.lazy` (copy the pattern in `src/components/Workbench.tsx`).
2. Splitting a Zustand slice: keep the same store and state shape (no persistence migration); only the slice files change.
3. Store access from components goes through `src/hooks/` selector hooks using `useShallow` from `zustand/react/shallow`.
4. Old import paths get a temporary re-export barrel; delete once `npm run typecheck` shows no users.

## 5. UI / animation budget

- NO animation libraries (framer-motion is being removed; don't reintroduce).
- CSS transitions only: `opacity`, `background-color`, `color`, `transform`. Max duration `var(--duration-base)` (150ms). Never animate width/height/top/left.
- All colors via tokens in `src/styles/tokens.css` (VSCode theme-key naming, e.g. `--color-sideBar-background`). No raw hex in component CSS.
- Icons: codicons (`src/codicon/`) only. No emoji, no one-off inline SVGs.
- Density targets: 13px UI font, 22px list/tree rows, 35px tabs, 22px status bar, 48px activity bar.

## 6. Settings

- Every setting is declared in `src/domain/settings/registry.ts` (`{ id, label, description, section, keywords, type, default, storage }`). UI, search, and persistence all derive from the registry — never add an ad-hoc localStorage key.
- Persistence: single `settings.json` in Tauri config dir via `src/infrastructure/SettingsRepository.ts`. `settingsSlice` is an in-memory cache only.
- Secrets (API keys) stay in backend `api_keys.json`, never in renderer storage.
- Sections (fixed, 8): Editor, Appearance, AI Models, Agent, Extensions, Privacy & Account, Keyboard, Advanced.

## 7. Verification gate (every commit)

```bash
cd src-tauri && cargo check && cargo test
npm run typecheck && npm test        # includes check-architecture.mjs
```

Manual smoke for risky batches: `npx tauri dev` → open file, agent chat, terminal, git panel, settings.

## 8. Commit discipline

- One batch/concern per commit; message prefix by milestone: `A1:`, `A2:`, `B:`, `C:`, `D:`, `E:`.
- Do NOT sweep in the unrelated uncommitted "potato offload" working-tree changes (ollama_offload.rs and friends) — stage files explicitly.
- Update `PROGRESS.md` (status + commit hash + "Next action") in the same commit.
