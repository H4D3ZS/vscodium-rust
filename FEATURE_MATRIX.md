# FEATURE MATRIX — VSCodium-Rust vs reference IDEs

Legend: ✅ have · 🟡 partial · ⬜ missing

Last verified: enterprise pass (agent onboarding, audit log, billing auto-sync).

---

## Enterprise & billing

| Feature | Status | Where |
|---|---|---|
| Agent setup onboarding (mode vs model) | ✅ | `AgentSetupWizard.tsx` |
| Billing auto-sync on focus / post-checkout | ✅ | `billingSync.ts`, `AccountSettingsPanel` |
| Usage meters (tokens + requests) | ✅ | `AccountSettingsPanel`, `account_usage` |
| Enterprise org policy | ✅ | `enterprise_audit.rs`, `EnterprisePanel` |
| Compliance audit trail (JSONL) | ✅ | `audit.jsonl`, export command |
| Enterprise secure defaults | ✅ | `applyEnterprisePolicy.ts` |
| SSO / team admin portal | ⬜ | website + backend (not IDE-local) |
| Signed auto-updater | ⬜ | disabled until signing keys configured |

## Core AI coding (Cursor / Void / Kilo)

| Feature | Status | Where |
|---|---|---|
| Agent loop (tool-calling) | ✅ | `ai_engine.rs`, `agent.ts` |
| Inline edit (Cmd-K) | ✅ | `Editor.tsx`, `InlineEditOverlay` |
| Tab / ghost completion | ✅ | `MonacoProviders.ts`, `ai_inline_complete` |
| Predictive multi-location Tab | ✅ | `PredictiveEditOverlay.tsx` (next-edit + propagate) |
| Multi-file composer edits | ✅ | `MultiFileReview`, cascade write |
| @-mentions | ✅ | `RightSidebar.tsx` |
| Inline gutter diffs | ✅ | `Editor.tsx` |
| Checkpoints (per-turn restore) | ✅ | `git_checkpoints.rs` |
| Checkpoint timeline UI | ✅ | `CheckpointTimeline.tsx` (History → collapsed git section) |
| Chat history + restore | ✅ | `list_chat_sessions`, History tab, thread tabs |
| Multi chat tabs (Ctrl+T) | ✅ | `agentThreads`, close × button |
| Plan mode | ✅ | `task_planner.rs` |
| Pure Chat (no tool loop) | ✅ | `agent.ts` fast path + Chat mode |

## Modes & customization (Kilo)

| Feature | Status | Where |
|---|---|---|
| Built-in modes | ✅ | mode picker in chat |
| User-defined custom modes | ✅ | Settings → Custom Agent Modes, mode picker |
| Per-feature model routing | ✅ | `modelSelectionOfFeature` |
| Rules / steering | ✅ | `RulesManager`, `rules_engine.rs` |

## MCP & extensions

| Feature | Status | Where |
|---|---|---|
| MCP client/server/registry | ✅ | `mcp_*.rs`, `McpManager.tsx` |
| MCP catalog (install) | ✅ | `MCP_CATALOG` in `McpManager.tsx` |
| Open VSX extensions | ✅ | `marketplace.rs`, ext-host |
| Extension API coverage | 🟡 | host scaffolding; smoke-test per release |

## Spec-driven & automation (Kiro)

| Feature | Status | Where |
|---|---|---|
| Specs manager | ✅ | `SpecsManager.tsx`, `specs_db.rs` |
| Spec-to-code wizard | ✅ | `SpecsToCodeWizard.tsx`, mode "Develop from Specs" |
| Agent hooks | ✅ | `HooksPanel.tsx` |
| On-save hook triggers | ✅ | `saveFile.ts` → `runBackgroundAgent` |

## Agents & autonomy

| Feature | Status | Where |
|---|---|---|
| Background / parallel agents | ✅ | `spawn_subagent`, `/bg` |
| Agent manager UI | ✅ | Studio → Agents (`AgentManagerPanel`) |
| Browser / Manus web research | ✅ | `runManusWebMission`, invisible_playwright sidecar |
| Vision capture | ✅ | `vision_bridge.rs` (off by default) |
| claurst SDK in IDE | 🟡 | optional backend toggle; separate workspace |

## Distinctive (VSCodium-Rust)

| Feature | Status | Where |
|---|---|---|
| Kortex AIM / semantic map | ✅ | `memory_store.rs`, `kortex_commands.rs` |
| APEX security / red-team | ✅ | `apex_*.rs` |
| iPhone / Android emulator | ✅ | `iphone_emulator.rs` (user-provided firmware) |
| AIRI avatar / voice | ✅ | `AiriPanel`, `voice.ts` |
| Local-first Ollama | ✅ | provider manager |
| Gemma 4 12B local SWE agent | ✅ | `ai_engine.rs` (sampling, ctx, tools, thinking) |

## Release / ship

| Item | Status | Where |
|---|---|---|
| LSP bundle script | ✅ | `scripts/fetch-lsp-binaries.ps1` |
| Browser sidecar bundle | ✅ | `scripts/build-sidecar.ps1`, `prebuild-release.mjs` |
| One-shot release | ✅ | `scripts/release.ps1`, `npm run release` |
| Ship checklist | ✅ | `docs/SHIP.md` |
| CI typecheck | ✅ | `.github/workflows/agent-core-ci.yml` |
| Auto-updater | ⬜ | disabled until signing keys configured |

## Remaining post-1.0 (optional)

1. Extension `vscode` API audit + fill gaps
2. claurst deep integration (default agent backend)
3. Tauri auto-updater + code signing
4. macOS browser-agent freeze (PyInstaller on CI)
