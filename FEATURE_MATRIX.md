# FEATURE MATRIX — VSCodium-Rust vs reference IDEs

Comparison of VSCodium-Rust against the reference codebases (Cursor, Void, Kiro,
Kilo Code, Antigravity, Manus, claurst). Status is grounded in **verified
vscodium-rust code** + each reference IDE's documented headline features. Rows
marked 🟡/⬜ become integration tasks (see bottom).

Legend: ✅ have · 🟡 partial · ⬜ missing

---

## Core AI coding (Cursor / Void / Kilo)

| Feature | From | Status | Where in vscodium-rust |
|---|---|---|---|
| Agent loop (tool-calling, autonomous) | all | ✅ | `ai_engine.rs` (Sentient), `agent.ts` |
| Inline edit (Cmd-K) | Cursor | ✅ | `Editor.tsx` CTRL_K + `InlineEditOverlay` |
| Tab / ghost completion | Cursor/Kilo | ✅ | `ai_inline_complete`, `MonacoProviders.ts`, `Editor.tsx` |
| **Predictive multi-location Tab ("jump to next edit")** | Cursor | 🟡 | single-spot ghost text only; no cross-line/next-edit prediction |
| Multi-file composer/agent edits | Cursor | ✅ | `MultiFileReview`, cascade write mode |
| @-mentions (files/symbols/commands) | Cursor/Kilo | ✅ | `RightSidebar.tsx` mention dropdown |
| Inline gutter diffs + per-hunk accept/reject | Cursor | ✅ | `Editor.tsx`, `styles.css` |
| Checkpoints (per-turn restore) | Void/Cursor | ✅ | `git_checkpoints.rs`, restore-to-message |
| **Checkpoint timeline visualization** | Void | 🟡 | restore works; no visual diff-timeline UI |
| Chat history + sessions | all | ✅ | `store_message`, HISTORY tab, `TaskManager` |
| Progressive multi-turn chat | all | ✅ | fixed Session 8 (`visibleMessages`) |
| Plan mode (approve before execute) | Kilo/Cursor | ✅ | `task_planner.rs`, plan-approval banner |

## Modes & customization (Kilo)

| Feature | From | Status | Where |
|---|---|---|---|
| Fixed modes (Agent / Chat / Plan / Harness) | Kilo | ✅ | mode toggle in chat toolbar |
| **User-defined custom modes** (own persona/tools/model per mode) | Kilo | ⬜ | only built-in modes exist |
| **Pure Chat mode that skips the agentic loop** | Cursor/Kilo | 🟡 | Agent mode runs full loop even for chit-chat (slow) |
| Per-feature model routing | Void/Kilo | ✅ | `modelSelectionOfFeature` |
| Rules / steering files | Cursor/Kiro | ✅ | `RulesManager.tsx`, `rules_engine.rs` |

## MCP & extensions

| Feature | From | Status | Where |
|---|---|---|---|
| MCP client/server/registry | Kilo/Cursor | ✅ | `mcp_client.rs`, `mcp_server.rs`, `mcp_registry.rs` |
| Add/list/toggle MCP servers | Kilo | ✅ | `McpManager.tsx` |
| **MCP server marketplace (browse/discover/install)** | Kilo | ⬜ | only manual add; no discovery catalog |
| VS Code extensions (Open VSX) | VSCodium | ✅ | `marketplace.rs` (open-vsx.org) + Node ext host |
| Extension activation / `vscode` API coverage | VSCodium | 🟡 | host scaffolding exists; API surface unverified at runtime |

## Spec-driven & automation (Kiro)

| Feature | From | Status | Where |
|---|---|---|---|
| Specs manager | Kiro | ✅ | `SpecsManager.tsx`, `specs_db.rs`, `specs_commands.rs` |
| **Spec-driven flow (requirements → design → tasks → impl)** | Kiro | 🟡 | `SpecsToCodeWizard.tsx` exists; full guided pipeline partial |
| Agent hooks (event-triggered automation) | Kiro | ✅ | `HooksPanel.tsx`, `agentHooks` store |
| **On-save / on-event hook triggers (not just manual)** | Kiro | 🟡 | hooks fire manually; auto file-event triggers unclear |

## Agents & autonomy (Antigravity / Manus / claurst)

| Feature | From | Status | Where |
|---|---|---|---|
| Background / parallel agents | Antigravity/Manus | ✅ | `/bg`, `spawn_subagent`, BackgroundAgentsTray |
| **Agent manager UI (track/steer many parallel agents)** | Antigravity | 🟡 | tray exists; no full manager (logs/steer/cancel per agent) |
| Browser control / actuation | Antigravity/Manus | ✅ | `browser_actuation/` (claude/gemini/chatgpt bridges) |
| Computer/screen use (real capture) | Manus | ✅ | `vision_bridge.rs` real GDI capture (Session 8) |
| Autonomous 24/7 loop | Manus | ✅ | opt-in (`airi.autonomous24x7`) |
| Rust agent SDK | claurst | 🟡 | `claurst/` separate workspace; not integrated into IDE |

## Distinctive to VSCodium-Rust (NOT in the reference IDEs)

| Feature | Status | Where |
|---|---|---|
| Kortex AIM VFS (zero-grep semantic map, .aim binary memory) | ✅ | `aim_store.rs`, `memory_store.rs`, `kortex_commands.rs` |
| APEX offensive-security engines + red-team (real HTTP probing) | ✅ | `apex_*.rs`, `offensive-security.ts` (Session 8) |
| iPhone emulator (acheron) + Android emulator | ✅ | `iphone_emulator.rs`, `emulator_stream.rs` |
| AIRI VRM avatar + voice/TTS | ✅ | `AiriPanel`, `voice.ts` |
| Local-first / data sovereignty (Ollama) | ✅ | provider manager |

---

## Integration backlog (the genuine gaps → tasks)

Ordered by value. Each becomes a tracked task, integrated + verified one-by-one.

1. **Pure Chat mode** that skips the agentic loop (instant conversational replies) — Cursor/Kilo. *(highest value: fixes the "ran full loop + timed out" UX)*
2. **User-defined custom modes** (name + system prompt + allowed tools + model) — Kilo.
3. **MCP server marketplace** (browse/discover/install from a catalog) — Kilo.
4. **Predictive multi-location Tab** ("jump to next edit") — Cursor.
5. **Checkpoint timeline visualization** (visual diff history) — Void.
6. **Agent manager UI** (per-agent logs / steer / cancel for parallel agents) — Antigravity.
7. **Spec-driven pipeline** (guided requirements→design→tasks→impl) — Kiro.
8. **Auto hook triggers** (on-save / on-file-event, not just manual) — Kiro.
9. **claurst Rust agent SDK** integration into the IDE agent layer — claurst.
10. **Extension `vscode` API coverage** audit + fill (runtime activation) — VSCodium.

> Note: this matrix is grounded in verified vscodium-rust code + documented
> reference-IDE features. Deeper per-repo source audits (esp. Cursor/Antigravity,
> which had no headline README here) may surface more rows — refine as we go.
