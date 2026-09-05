# Frontend — public entry points

Architecture and conventions: [`../ARCHITECTURE.md`](../ARCHITECTURE.md).
This file is the index of stable entry points — **import these in new code**, not
the legacy engine modules they wrap.

| Layer | Folder | Responsibility |
|-------|--------|----------------|
| **Domain** | `domain/` | entities, value objects, repository **ports**. No React, no Tauri. |
| **Application** | `application/` | use-cases — one file per user goal. |
| **Infrastructure** | `infrastructure/` | adapters — Tauri IPC, lazy legacy engines, event subscribers. |
| **Presentation** | `components/` | React UI — calls the application layer, never `invoke()`. |

Why: navigate by reading `domain/<ctx>/I*Repository.ts` to see *what* a context
can do and `infrastructure/<ctx>/` to see *how*; heavy engines (e.g. `agent.ts`,
~4k lines) load lazily so boot stays small.

## Entry points

| Goal | Import |
|------|--------|
| Boot agent listeners | `application/agent/bootstrapAgentRuntime` |
| Send chat message | `application/agent/sendAgentTurn` |
| Stop agent | `application/agent/stopAgent` |
| Pure chat (no tool loop) | `application/agent/runPureChatTurn` |
| Sync agent messages | `application/agent/syncAgentMessages` |
| Restore folder on launch | `application/workspace/restoreWorkspaceOnBoot` |
| Multi-root folders | `application/workspace/multiRootWorkspace` |
| Workspace settings | `application/workspace/workspaceSettings` |
| Open / save / tree | `application/editor` (`openFile`, `saveActiveFile`, `refreshFileTree`, `toggleDirectory`) |
| Composer hunk nav / accept | `application/editor/navigatePendingChange`, `application/editor/acceptHunk` |
| File symbol outline | `application/symbols/fetchFileSymbols` |
| Spawn / split terminal | `application/terminal/spawnTerminal`, `application/terminal/splitTerminal` |
| Terminal workflows / theme | `application/terminal/runWorkflow`, `application/terminal/refreshTerminalTheme` |
| Debug (DAP) boot / send | `application/debug/bootstrapDebugRuntime`, `application/debug/sendDapRequest` |
| Launch debug config | `application/debug/startLaunchConfig` |
| Memory stats | `application/performance/refreshProcessMemory` |
| Run security review | `application/security/runCodebaseSecurityReview` |
| Web research (scrape/audit) | `application/research/runWebResearch` |
| LSP bootstrap | `application/lsp/bootstrapLanguageServer` |
| Gradle sync | `application/gradle/syncGradleProject`, `bootstrapGradleProject` |
| Android devices / logcat | `application/android/refreshAndroidDevices`, `logcatSession` |
| Test discovery / run | `application/test/discoverTests` |
| iPhone emulator path | `application/emulator/resolveEmulatorProject` |
| Panel chrome | `application/layout/togglePanels` |

## Tests

| Suite | Command | Covers |
|-------|---------|--------|
| Frontend logic + architecture | `npm test` | pure use-case logic, `check-architecture.mjs` |
| Rust IDE adapters | `npm run test:rust` | Gradle parsers, test-runner logic, DAP framing, logcat parse |
| Both | `npm run test:all` | — |

Hardware-integration paths (adb / gradlew / device firmware) are exercised
manually with the SDK present.
