# Frontend Clean Architecture

Layers (dependencies point **inward** only):

| Layer | Folder | Responsibility |
|-------|--------|----------------|
| **Domain** | `src/domain/` | Entities, value objects, repository **ports** (interfaces). No React, no Tauri. |
| **Application** | `src/application/` | **Use-cases** — one file per user goal (`sendAgentTurn`, `restoreWorkspaceOnBoot`). |
| **Infrastructure** | `src/infrastructure/` | **Adapters** — Tauri IPC, lazy legacy engines, event subscribers. |
| **Presentation** | `src/components/` | React UI — calls application layer, never `invoke()` directly. |

## Why this structure?

1. **Navigate without AI** — open `domain/agent/IAgentRepository.ts` to see what chat can do; open `infrastructure/agent/` to see how.
2. **Less RAM** — `agent.ts` (4k lines) loads only when you send a message; boot uses `bootstrapAgentRuntime` (~15 KB).
3. **No debt creep** — new feature = port in domain → adapter in infrastructure → use-case → thin UI.

## Public entry points (use these in new code)

| Goal | Import |
|------|--------|
| Boot agent listeners | `application/agent/bootstrapAgentRuntime` |
| Send chat message | `application/agent/sendAgentTurn` or `agent/index` |
| Stop agent | `application/agent/stopAgent` |
| Restore folder on launch | `application/workspace/restoreWorkspaceOnBoot` |
| Multi-root folders | `application/workspace/multiRootWorkspace` |
| Memory stats | `application/performance/refreshProcessMemory` |
| Run security review | `application/security/runCodebaseSecurityReview` |
| Spawn terminal | `application/terminal/spawnTerminal` or `terminal/index` |
| Split terminal | `application/terminal/splitTerminal` |
| Terminal workflows | `application/terminal/runWorkflow` |
| Terminal theme | `application/terminal/refreshTerminalTheme` |
| Open / save / tree | `application/editor` (`openFile`, `saveActiveFile`, `refreshFileTree`, `toggleDirectory`) |
| Composer hunk nav | `application/editor/navigatePendingChange` |
| Accept diff hunk | `application/editor/acceptHunk` |
| Workspace settings | `application/workspace/workspaceSettings` |
| Debug DAP boot | `application/debug/bootstrapDebugRuntime` |
| Debug DAP send | `application/debug/sendDapRequest` |
| File symbol outline | `application/symbols/fetchFileSymbols` |
| iPhone emulator path | `application/emulator/resolveEmulatorProject` |
| Agent Studio sub-views | `application/agentStudio/agentStudioSubViews` |
| Session plan files | `application/research/loadSessionPlanFiles` |
| Web research (scrape/audit) | `application/research/runWebResearch` |
| Manus web mission | `application/research/runManusWebMission` |
| Sync agent messages | `application/agent/syncAgentMessages` |
| Pure Chat (no tool loop) | `application/agent/runPureChatTurn` + `agent.ts` fast path |
| Heavy feature defaults (vision OFF) | `application/agent/bootstrapHeavyFeaturesDefaults` |
| Panel chrome | `application/layout/togglePanels` |

## Migration status

| Bounded context | Domain | Application | Infrastructure | UI wired |
|-----------------|--------|-------------|----------------|----------|
| Performance / RAM | ✅ | ✅ | ✅ | StatusBar |
| Agent / Chat | ✅ | ✅ | ✅ (lazy legacy) | RightSidebar, App boot |
| Workspace / Root | ✅ | ✅ | ✅ | App.tsx, Sidebar |
| Multi-root workspace | ✅ | ✅ | ✅ (LSP sync) | Sidebar folder list |
| Security / Audit | ✅ | ✅ | ✅ | SecurityReviewPanel, StatusBar |
| **Terminal** | ✅ | ✅ | ✅ | TerminalView, BottomPanel |
| **Editor / Patches** | ✅ | ✅ | ✅ | Editor, editorSlice |
| **Layout / Chrome** | — | 🟡 | — | layoutSlice + `application/layout` |
| **Debug / DAP** | — | ✅ | ✅ | DebugView, Editor gutter, RunConfigsPanel |
| Workspace settings | ✅ | ✅ | ✅ | Settings → Workspace |
| Extensions host | — | — | ✅ | `extensions.ts` + ext-host sync |
| Symbols / Outline | ✅ | ✅ | ✅ | Sidebar SymbolOutlinePane |
| Emulator (acheron) | — | ✅ | ✅ (Rust) | Devices tab, `iphone_emulator.rs` |
| Agent Studio (specs/steering) | ✅ | ✅ | — | RightSidebar → `AgentStudioPanel` |
| Web research & security | — | ✅ | ✅ | Studio → Web Agent |
| Session planning | ✅ | ✅ | ✅ | Studio → Session tab |
| Chat tabs + history restore | — | ✅ | ✅ | `agentThreads`, History tab |
| Release bundle (LSP + browser) | — | — | ✅ | `scripts/release.ps1`, `prebuild-release.mjs` |

## Terminal bounded context

```
src/domain/terminal/          — TerminalProfile, TerminalWorkflow, ITerminalRepository
src/application/terminal/     — spawnTerminal, splitTerminal, navigateCommandBlocks
src/infrastructure/terminal/  — TauriTerminalRepository, terminalThemes, terminalProfiles
src/components/terminal/      — presentation only (xterm via legacy terminalManager)
src/terminal.ts               — xterm runtime (shrinking; use application layer for new code)
```

Features ported from **warp-terminal** (OSC 133 blocks, workflows) and **cmder** (aliases, tasks):
- Command blocks with gutter marks + hover toolbar
- `Ctrl+Shift+R` palette (history + saved workflows)
- cmder aliases in PowerShell + Bash integration
- Horizontal + vertical splits, resizable sashes
- Native vs IDE theme toggle

## Editor bounded context

```
src/domain/editor/           — IFileRepository, IPatchRepository
src/application/editor/      — openFile, saveFile, refreshFileTree, toggleDirectory, navigatePendingChange, acceptHunk
src/infrastructure/editor/   — TauriFileRepository, TauriPatchRepository
```

Composer review: `Alt+J/K` navigates pending changes by `change.id`.
Per-hunk accept uses `DiffService.patchContentAccepted` → disk when all hunks accepted.
Breakpoints: F9 + gutter click → `debugSlice`, glyphs in Monaco glyph margin.

## Debug bounded context

```
src/application/debug/       — bootstrapDebugRuntime, sendDapRequest
src/store/debugSlice.ts      — breakpoints, threads, stack, variables
src-tauri/debug_adapter.rs   — Content-Length DAP framing
```

Boot flow: `initialize` → `initialized` event → `configurationDone` → `launch`.
Stopped events trigger `stackTrace` → `scopes` → `variables`.

## Next migrations (optional)

1. **Agent legacy peel** — provider gating from `agent.ts` → `application/agent/gates/`
2. **setActiveRoot peel** — move invoke chain → `application/workspace/setActiveRoot`
3. **Layout domain** — extract panel state entities from layoutSlice

## Rust backend mirror

See `src-tauri/src/architecture/` — same layering for `ProcessMemorySnapshot` and future domains.

## LSP bundle (production)

Run `scripts/fetch-lsp-binaries.ps1` before `npx tauri build` to populate `src-tauri/binaries/lsp/`.

## Release (production installer)

```powershell
npm run release
# or: powershell -ExecutionPolicy Bypass -File scripts\release.ps1
```

See `docs/SHIP.md` for the full checklist. Browser sidecar (`browser-agent.exe`) is built automatically when `invisible_playwright/` is present.
