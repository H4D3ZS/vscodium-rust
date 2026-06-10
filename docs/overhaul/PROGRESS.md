# Overhaul Progress

> **Handoff contract**: whoever works on this (Claude Code, Cursor, human) MUST update this file
> before every commit: flip statuses, record the commit hash, and write the "Next action" line so
> the next session can resume cold. Read `MASTER_PLAN.md` for the full plan and `CONVENTIONS.md`
> for the architecture rules before touching code.

**Statuses**: `todo` | `in-progress` | `done` | `blocked(<reason>)`

**Next action**: Start A1 batch 1 — move leaf utility modules in `src-tauri/src/` into `infrastructure/` and `domain/` with `pub use` shims (see MASTER_PLAN §A1 batch order). Keep `cargo check` green; commit per batch.

---

## Phase 0 — Handoff docs
| Task | Status | Commit |
|---|---|---|
| MASTER_PLAN.md / PROGRESS.md / CONVENTIONS.md created | done | (phase-0 commit) |
| CLAUDE.md pointer + macOS dev note | done | (phase-0 commit) |

## Milestone A1 — Backend DDD restructure (src-tauri)
| Task | Status | Commit |
|---|---|---|
| Batch 1: leaf utility modules → infrastructure/ + domain/ (~40 small files) | todo | — |
| Batch 2: vcs (git, git_checkpoints, patch_engine, shadow_workspace) → domain/vcs | todo | — |
| Batch 3: mobile (ios_simulator, iphone_emulator, android, logcat, gradle) → domain/mobile | todo | — |
| Batch 4: security (apex_*, oast, intruder, pentest_*, intercept_proxy) → domain/security | todo | — |
| Batch 5: memory (memory_store, memory_layer, aim_store, memory_optimizer) → domain/memory | todo | — |
| Batch 6: indexing (context_indexer, vector_indexer) → domain/indexing | todo | — |
| Batch 7: editor/lsp (buffers, lsp router, lsp_bundle) → domain/editor | todo | — |
| Batch 8: extensions (extension_host, marketplace) → domain/extensions | todo | — |
| Split ai_tools.rs (8,511 LOC) → domain/tools/{registry,shell,web,security_tools,fs_tools}.rs | todo | — |
| Split ai_engine.rs (7,207 LOC) → domain/ai/{sentient,streaming,providers,prompt}.rs | todo | — |
| ai_commands.rs → thin wrappers in application/commands/ai.rs (testable inner fns) | todo | — |
| All *_commands.rs → application/commands/ with command-extraction pattern | todo | — |
| EditorState: group 52 fields into substructs (ai, editor, terminal, memory, security, mobile, ext, services) | todo | — |
| Drop unused kortex/daemon dep from src-tauri/Cargo.toml | todo | — |
| Cleanup: delete pub-use shims; cargo check + cargo test green | todo | — |

## Milestone A2 — Frontend DDD restructure (src/)
| Task | Status | Commit |
|---|---|---|
| Consolidate stray folders: agent, architecture, services, utils, security, mcp → layers | todo | — |
| README headers for kept subsystems: kortex, airi, hermes, claurst | todo | — |
| Split RightSidebar.tsx (2,776) → components/rightSidebar/* lazy sub-panels | todo | — |
| Split AgentSettingsView.tsx (2,003) → components/settings/agent/* | todo | — |
| Editor.tsx: extract monacoSetup/decorations/keybindings → application/editor/ | todo | — |
| Split agentSlice.ts (1,267) → agentMessagesSlice + agentToolsSlice + agentModesSlice | todo | — |
| scripts/check-architecture.mjs (no invoke() outside infrastructure) wired into npm test | todo | — |
| Migrate invoke() violations in components/store → infrastructure adapters | todo | — |
| Hooks layer: src/hooks selector hooks with useShallow | todo | — |

## Milestone B — VSCode-native UI polish
| Task | Status | Commit |
|---|---|---|
| Remove framer-motion (4 usages) + drop from package.json | todo | — |
| Clamp all transitions ≤150ms, opacity/background/transform only | todo | — |
| Move Dark+ palette into tokens.css with VSCode theme-key naming; no raw hex in components | todo | — |
| Density: 13px UI font, 22px rows, 35px tabs, 22px statusbar, 48px activitybar | todo | — |
| Thin overlay scrollbars + :focus-visible rings + codicon sweep | todo | — |
| Light + High Contrast theme audit | todo | — |

## Milestone C — Settings redesign (43 → 8 sections)
| Task | Status | Commit |
|---|---|---|
| src/domain/settings/registry.ts declarative registry | todo | — |
| SettingsPage.tsx rebuilt as ≤300 LOC shell, 8 sections, lazy sub-panels | todo | — |
| VSCode-style settings search over registry | todo | — |
| SettingsRepository adapter + single settings.json persistence | todo | — |
| One-time localStorage → settings.json migration | todo | — |
| Registry unit tests + round-trip test | todo | — |

## Milestone D — Performance
| Task | Status | Commit |
|---|---|---|
| terminal_pending Mutex hot loop → tokio mpsc | todo | — |
| vfs_bridge.rs std::fs → tokio::fs / spawn_blocking (~20 calls) | todo | — |
| Lazy-init VectorIndexer / ContextIndexer / KnowledgeDistiller (OnceCell) | todo | — |
| RwLock / immutable Arc for read-mostly AI data; mpsc activity log | todo | — |
| Try lto="thin" in release profile on macOS; keep only if green | todo | — |
| useShallow sweep (364 subscriptions) + React.memo sub-panels | todo | — |
| Verify vite chunking post-restructure | todo | — |
| Record before/after metrics (boot, EditorState::new, terminal latency, re-renders) | todo | — |

## Milestone E — Extension API
| Task | Status | Commit |
|---|---|---|
| Sidecar JS host + JSON-RPC over existing ext_host IPC | todo | — |
| packages/hades-extension-api typed .d.ts + runtime shim (v1 surface) | todo | — |
| Manifest-validated contribution points in Rust | todo | — |
| Capability permission prompts via tool_permission_senders | todo | — |
| docs/extensions/API.md + examples/hello-extension | todo | — |
| Open VSX gallery + ExtensionsView error states | todo | — |

---

## Metrics log (fill during Milestone D)
| Metric | Before | After |
|---|---|---|
| App start → interactive | — | — |
| EditorState::new duration | — | — |
| Terminal echo latency | — | — |
| Re-renders per agent stream message | — | — |
| Entry chunk size (vite build) | — | — |

## Session notes
- 2026-06-10: Plan approved; Phase 0 scaffolding created. Working tree also contains unrelated uncommitted "potato offload" work (ollama_offload.rs etc.) — keep overhaul commits scoped, do not sweep those files in.
