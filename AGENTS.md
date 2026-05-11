# Repository Agent Guide

This repository contains four interconnected projects. Each uses a different package manager and toolchain. Work in the subdirectory for the target system; do not mix commands across boundaries.

## Monorepo Layout

| Directory | Stack | Package Manager | Primary Entry |
|-----------|-------|----------------|---------------|
| `claurst/kilocode/` | TypeScript/Bun + Tauri | **Bun** | Kilo CLI & VS Code extension |
| `airi/` | TypeScript/Vue/Electron | **pnpm** | AIRI digital entity apps |
| `kortex/` | Rust (Tauri/axum) | **cargo** | Neural memory & AIM proxy |
| `src-tauri/` | Rust (Tauri) | **cargo** | VSCodium-Rust desktop shell |

## Quick Start by Target

### Kilo CLI / OpenCode (`claurst/kilocode/`)
```bash
cd claurst/kilocode
bun run dev                  # TUI dev server
bun turbo typecheck          # type check all packages
bun test                    # run tests from packages/opencode/ only
bun run extension           # build + launch VS Code with extension
```

Important:
- Tests MUST run from `packages/opencode/`, not root
- Typecheck uses `tsgo` (`bun turbo typecheck`), not `tsc`
- SDK is auto-generated: after changing `packages/opencode/src/server/`, run `./script/generate.ts` from the Kilo root to regenerate `packages/sdk/js/`

### AIRI (`airi/`)
```bash
cd airi
pnpm dev                    # start web app (stage-web)
pnpm -F @proj-airi/stage-tamagotchi dev   # Electron desktop
pnpm -F @proj-airi/stage-pocket dev:ios   # iOS mobile
pnpm test:run               # run vitest across all projects
pnpm lint && pnpm typecheck # lint + typecheck
```

Important:
- Uses pnpm workspaces and Turbo
- `packages/stage-ui/` is the core business component library
- `apps/stage-tamagotchi/` is the main Electron desktop app
- `apps/stage-pocket/` is the Capacitor mobile app (iOS/Android)

### Kortex (`kortex/`)
```bash
cd kortex
cargo build --release        # builds all workspace members
.\target\release\aim-proxy.exe   # start Ollama MITM proxy (port 1536)
.\target\release\neuraldrive.exe # launch 3D code visualization GUI
.\target\release\hades-tui.exe   # terminal dashboard
```

Important:
- AIM proxy must run alongside `ollama serve`
- `.aim/memory.aim` files provide zero-token context injection
- Check `CLAUDE.MD` section 0 for Kortex integration details

### VSCodium-Rust Tauri Shell (`src-tauri/`)
```bash
cd src-tauri
cargo build                 # builds the native Tauri frontend
```

Important:
- This is the main application shell that bundles Kortex components
- Links to `kortex/libaim`, `kortex/daemon`, `kortex/harness` via path dependencies

## Critical Conventions

### Kilo Change Markers (`claurst/kilocode/` only)
When modifying files that exist in upstream OpenCode, mark Kilo-specific changes with `kilocode_change` comments to simplify merges.

Single line: `const x = 1 // kilocode_change`  
Block:
```
// kilocode_change start
... Kilo code ...
// kilocode_change end
```

**Exempt paths** (no markers needed — these are pure Kilo additions):
- `packages/opencode/src/kilocode/`
- `packages/opencode/test/kilocode/`
- Any path containing `kilocode` in the name
- `packages/kilo-vscode/`
- `packages/kilo-ui/`

Run `bun run check-kilocode-change` from `packages/kilo-vscode/` before pushing to verify no stray markers exist.

### Naming Style (Kilo packages)
Single-word names preferred. Avoid camelCase compounds when a clear single-word alternative exists (`pid` over `inputPID`, `err` over `errorMsg`). Review identifiers before finishing edits.

### Prefer Early Returns
Avoid `else` blocks; use early returns or IIFE.

### No Empty `catch` Blocks
At minimum log errors: `catch (err) { log.error("msg", { err }) }`.

### Style by Project
- **Kilo/OpenCode**: Bun + SolidJS; see `claurst/kilocode/AGENTS.md` for full style guide
- **AIRI**: pnpm + Vue 3 + UnoCSS; see `airi/AGENTS.md`
- **Kortex**: Rust 2021 edition; follow existing patterns in `kortex/`
- **No mocks** in tests for either project — test real implementation

## Cross-Project Dependencies

`src-tauri/Cargo.toml` depends on Kortex crates via path:
```toml
libaim = { path = "../kortex/libaim" }
daemon = { path = "../kortex/daemon" }
hades-harness = { path = "../kortex/harness" }
```

Changes in `kortex/` that affect these crates require rebuilding `src-tauri/`.

## CI / Pre-commit

- **Kilo**: `simple-git-hooks` runs `moeru-lint --fix` on pre-commit (via `nano-staged`)
- **AIRI**: `simple-git-hooks` runs `pnpm exec simple-git-hooks` postinstall
- All repos enforce lint + typecheck in CI

## Troubleshooting

### AIM proxy not injecting context
1. Verify Ollama is running: `ollama serve`
2. Start AIM proxy: `cd kortex && .\target\release\aim-proxy.exe`
3. Configure client to use `http://127.0.0.1:1536` (not 11434)
4. Check for `.aim/memory.aim` in project root

### Kilo tests failing from wrong directory
Always `cd packages/opencode/` before `bun test`. Root-level test invocation is blocked.

### Merge conflicts with upstream OpenCode
Keep Kilo changes in `packages/opencode/src/kilocode/` and use `kilocode_change` markers elsewhere. Do not refactor upstream code structure.

## Workspace Awareness

You may be in a git worktree. All changes must be made in the current working directory — never modify files in the main repo checkout.

Default branch is `main`.
