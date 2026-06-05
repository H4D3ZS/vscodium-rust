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
| Memory stats | `application/performance/refreshProcessMemory` |

## Migration status

| Bounded context | Domain | Application | Infrastructure | UI wired |
|-----------------|--------|-------------|----------------|----------|
| Performance / RAM | ✅ | ✅ | ✅ | StatusBar |
| Agent / Chat | ✅ | ✅ | ✅ (lazy legacy) | RightSidebar, App boot |
| Workspace / Root | ✅ | ✅ | ✅ | App.tsx |
| Editor / Files | 🔲 | 🔲 | 🔲 | `store/editorSlice` |
| Layout / Chrome | 🔲 | 🔲 | 🔲 | `store/layoutSlice` |

## Next migrations (recommended order)

1. **Editor** — `domain/editor/IFileRepository`, `application/editor/openFile.ts`
2. **Agent legacy peel** — move provider gating from `agent.ts` → `application/agent/gates/`
3. **Layout** — `application/layout/toggleChat.ts` wrapping layout slice

## Rust backend mirror

See `src-tauri/src/architecture/` — same layering for `ProcessMemorySnapshot` and future domains.
