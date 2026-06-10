# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Is

VSCodium-Rust: a custom IDE built with **Rust/Tauri v2** backend + **React 19/TypeScript/Vite** frontend. Designed for agentic development with local Ollama models, cybersecurity research, and data sovereignty.

> **Active overhaul in progress** (Structure → UI → Settings → Perf → Extension API).
> Before changing code, read `docs/overhaul/MASTER_PLAN.md`, follow the rules in
> `docs/overhaul/CONVENTIONS.md`, and update `docs/overhaul/PROGRESS.md` before every commit.
> This applies to any agent (Claude Code, Cursor) or human picking up the work.

---

## Build & Run Commands

### Full IDE (Tauri app)
```powershell
# Dev mode (hot-reload frontend + Rust backend)
npm run dev:tauri
# or
npx tauri dev

# Frontend only (no Tauri shell)
npm run dev

# Production build
npx tauri build
```

### Rust backend only (faster iteration)
```powershell
# From src-tauri/ — check compilation
cargo check

# Build release
cargo build --release
```

### Kortex workspace (supporting crates)
```powershell
cd kortex
cargo build --release

# Individual binaries
cargo build --release --bin aim-proxy    # Ollama MITM proxy on :1536
cargo build --release --bin neuraldrive  # 3D visualization GUI
```

### Tests & Type-checking
```powershell
npm test               # vitest run (frontend)
npm run test:watch     # vitest watch mode
npm run typecheck      # tsc type-check only

# Rust tests
cargo test             # from src-tauri/ or kortex/
```

---

## Architecture

### Two Build Roots

| Root | Stack | Purpose |
|------|-------|---------|
| `src-tauri/` | Rust (Tauri v2) | Main IDE backend |
| `src/` + `vite.config.mjs` | React/TS/Vite | Main IDE frontend |
| `kortex/` | Rust workspace | Supporting tools (proxy, TUI, VFS) |
| `airi/` | pnpm/Node | 3D VRM avatar (AIRI) |
| `claurst/` | Rust + Bun | Separate agent SDK workspace |

### Backend: `src-tauri/src/`

`EditorState` (`state.rs`) is the central Tauri managed state — all async fields are `Arc<tokio::sync::Mutex<T>>`. Commands access it via `tauri::State<EditorState>`.

**Key modules:**

| Module | Role |
|--------|------|
| `ai_engine.rs` | `Sentient` struct — routes prompts to Ollama/external APIs, streaming |
| `apex_orchestrator.rs` | Coordinates 7 specialist models (architect, threat, perf, etc.) |
| `apex_red_team.rs` | Security scanning engine |
| `patch_engine.rs` | Surgical code patching via `diffy` crate (SEARCH/REPLACE, not full-file) |
| `shadow_workspace.rs` | Virtual branch for safe code mutation before commit |
| `vfs_bridge.rs` | Bridge to kortex VFS daemon |
| `memory_layer.rs` / `aim_store.rs` | `.aim` binary memory format (memmap2) |
| `lsp.rs` / `lsp_commands.rs` | LSP client (tree-sitter diagnostics) |
| `mcp_server.rs` / `mcp_client.rs` / `mcp_registry.rs` | MCP protocol support |
| `terminal_commands.rs` | PTY terminals via `portable-pty` |
| `git.rs` / `git_checkpoints.rs` | Git operations + savepoints |
| `context_indexer.rs` / `vector_indexer.rs` | Codebase semantic indexing |
| `hades_harness.rs` | MCTS verification loop (propose → shadow VFS → `cargo check` → commit) |

### Kortex Workspace: `kortex/`

| Crate | Binary | Role |
|-------|--------|------|
| `aim-proxy` | `aim-proxy.exe` | MITM proxy on port 1536, intercepts Ollama requests |
| `libaim` | (lib) | `.aim` binary format read/write via memmap2 |
| `daemon` | — | Memory management daemon |
| `vfs_layer` | `aim-vfs.exe` | VFS daemon |
| `harness` | — | MCTS + tree-sitter eval harness |
| `tui` | `hades-tui.exe` | ratatui dashboard |
| `neuraldrive/src-tauri` | `neuraldrive.exe` | Tauri 3D code graph visualization |
| `hades-kernel` / `hades-bridge` | — | Kernel/bridge utilities |

### Frontend: `src/`

React 19 + Zustand state + Monaco editor. Notable integrations:
- `@xterm/xterm` for terminal panels
- `reactflow` for Visual Lab diagram builder
- `three` + `@pixiv/three-vrm` for AIRI 3D avatar
- `@monaco-editor/react` for the code editor

### `.aim` Memory System

`.aim` files are binary Neural Weight-Maps (memmap2). Locations checked at runtime:
- `.aim/memory.aim` (project-relative)
- `C:\Users\HADES\Desktop\kortex\.aim\memory.aim`

The `aim-proxy` intercepts Ollama calls on `:1536` and injects compressed `.aim` context before forwarding to Ollama on `:11434`.

---

## Key Constraints

- **Patch discipline:** Use surgical SEARCH/REPLACE via `patch_engine.rs` / `diffy`. No full-file rewrites.
- **Memory budget:** Core footprint < 150MB. Heap-heavy operations belong in `kortex/daemon`, not main process.
- **Cross-platform:** Active development happens on macOS (`mac_dev` branch, Apple Silicon); Windows remains a release target. Don't hardcode platform paths — platform-specific code is gated (`#[cfg(...)]` in Rust, runtime checks in TS).
- **Tauri IPC:** All frontend↔backend calls go through `#[tauri::command]` handlers registered in `lib.rs`. Commands are grouped by domain (`file_commands`, `ai_commands`, `git_commands`, etc.).
- **Model assignments:** APEX engines use hardcoded Ollama model strings in `apex_orchestrator.rs` (e.g. `qwen2.5:32b` for architect, `qwen2.5-coder:7b` for perf). Change there to swap models.

---

## Services to Run Locally

For full AI features:
```powershell
# 1. Ollama
ollama serve

# 2. AIM Proxy (optional, for .aim context injection)
kortex\target\release\aim-proxy.exe

# 3. AIRI avatar (optional)
cd airi && pnpm dev --host
```
