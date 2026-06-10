# Overhaul Progress

> **Handoff contract**: whoever works on this (Claude Code, Cursor, human) MUST update this file
> before every commit: flip statuses, record the commit hash, and write the "Next action" line so
> the next session can resume cold. Read `MASTER_PLAN.md` for the full plan and `CONVENTIONS.md`
> for the architecture rules before touching code.

**Statuses**: `todo` | `in-progress` | `done` | `blocked(<reason>)`

**Next action**: A1 batch 5 — move memory group (memory_store, memory_layer, aim_store, memory_optimizer, memory_offload, context_quantizer) → `src-tauri/src/domain/memory/`; then batch 6 indexing (context_indexer, vector_indexer, knowledge_distiller, embeddings, ann_index, ripgrep_search, symbols) → `domain/indexing/`. Same git-mv + shim pattern (lib.rs ~line 75). Mind: aim_store/memory_optimizer/memory_offload/context_quantizer are `pub mod` → shim with `pub use`, others `pub(crate) use`. `cargo check && cargo test --lib` per batch.

---

## Phase 0 — Handoff docs
| Task | Status | Commit |
|---|---|---|
| MASTER_PLAN.md / PROGRESS.md / CONVENTIONS.md created | done | (phase-0 commit) |
| CLAUDE.md pointer + macOS dev note | done | (phase-0 commit) |

## Milestone A1 — Backend DDD restructure (src-tauri)
| Task | Status | Commit |
|---|---|---|
| Layer skeleton: domain/ + application/commands/ + infrastructure/platform/ dirs; domain.rs → domain/types.rs | done | (batch-1 commit) |
| Batch 1: leaf utility modules → infrastructure/ + domain/ (~40 small files) | todo | — |
| Batch 2: vcs (git, git_checkpoints, patch_engine, shadow_workspace) → domain/vcs | done | (batch-1 commit) |
| Batch 3: mobile (ios_*, iphone_emulator, emulator_stream, scrcpy, android_sdk, logcat_service, mobile_toolchain) → domain/mobile | done | (batch-3 commit) |
| Batch 4: security (apex_*, pentest_*, oast, intruder, repeater, intercept_proxy, chunk_secrets, security_*, sec_distro, hunter) → domain/security | done | (batch-4 commit) |
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
- 2026-06-10 (later): ANE fixed for real (outside overhaul scope, user-requested). Discovery: the ANE never worked — MIL header had a brace bug (`({` vs `({{`) so every kernel since day one failed with InvalidMILProgram; UI showed hardcoded fake "2.5-3x / ANE Accelerated". Fixes: header fixed; fp16 I/O required (current macOS ANECompiler rejects fp32 I/O); ANE needs seq≥32 (tile granularity). ANE now does batched cosine similarity for ann_index with pre-packed buffers: 1.1ms vs 21ms CPU (1024×768-dim, ~19x). Token gen stays Ollama/Metal (bandwidth-bound ~45 tok/s — ANE cannot raise it; all fake claims removed from UI/commands). Bridge dylib rebuilt with eval error logging (ANE/bridge/ane_bridge.m). Diagnostics: src-tauri/tests/ane_exec_probe.rs.
