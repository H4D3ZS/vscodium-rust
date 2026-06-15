# Overhaul Progress

> **Handoff contract**: whoever works on this (Claude Code, Cursor, human) MUST update this file
> before every commit: flip statuses, record the commit hash, and write the "Next action" line so
> the next session can resume cold. Read `MASTER_PLAN.md` for the full plan and `CONVENTIONS.md`
> for the architecture rules before touching code.

**Statuses**: `todo` | `in-progress` | `done` | `blocked(<reason>)`

**Next action**: ALL MILESTONES (0/A/B/C/D/E) functionally complete. Remaining verification + polish backlog (in priority order): (1) `npx tauri dev` runtime smoke — new Settings shell, hello-extension folder install (Verification E: command + status bar + restart survival), record EditorState::new elapsed_ms metric; (2) deferred decompositions: RightSidebar main fn (2.3K), AgentSettingsView (2.0K), Editor.tsx (1.5K), autonomous_loop (3.5K method); (3) lib.rs shim deletion sweep; (4) B leftovers: 179 raw hex → tokens, Light/HC audit, codicon sweep; (5) terminal_pending mpsc (needs runtime contention profile first); (6) baseline shrink: migrate the 97 grandfathered invoke() files through infrastructure adapters.

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
| Batch 1: infrastructure leaf modules (vfs_bridge, browser, browser_actuation, mcp_*, performance, process_ext) → infrastructure/ | done | (infra batch commit) |
| Batch 2: vcs (git, git_checkpoints, patch_engine, shadow_workspace) → domain/vcs | done | (batch-1 commit) |
| Batch 3: mobile (ios_*, iphone_emulator, emulator_stream, scrcpy, android_sdk, logcat_service, mobile_toolchain) → domain/mobile | done | (batch-3 commit) |
| Batch 4: security (apex_*, pentest_*, oast, intruder, repeater, intercept_proxy, chunk_secrets, security_*, sec_distro, hunter) → domain/security | done | (batch-4 commit) |
| Batch 5: memory (aim_store, memory_store/layer/optimizer/offload, context_quantizer) → domain/memory | done | 1bb83c4f |
| Batch 6: indexing (context/vector indexers, embeddings, ann_index, ripgrep_search, symbols, distiller) → domain/indexing | done | 8ea762da |
| Batch 7: editor/lsp (lsp*, debug_adapter) → domain/editor | done | e57b9388 |
| Batch 8: extensions (extension_host, marketplace, activation, keybindings, context_key) → domain/extensions | done | e57b9388 |
| Batch 9: ai domain moved whole (engine, tools, ANE, harnesses, vision, workflow) — split still todo | done | 21b41649 |
| Split ai_tools.rs (8,511 LOC) → domain/tools/{registry,schemas,dispatch,shell,fs_tools,security_tools,web_edit} | done | (tools-split commit) |
| Split ai_engine.rs (7,207 LOC) → domain/ai/engine/{types,sentient,streaming,providers,autonomous,prompt} | done | (engine-split commit) |
| ai_commands.rs → thin wrappers in application/commands/ai.rs (testable inner fns) | todo | — |
| All 44 *_commands.rs → application/commands/<domain>.rs (move done; thin-wrapper extraction per-file still todo) | done | (commands batch) |
| Dead provider_commands.rs (never declared in lib.rs) parked as provider.rs.dead | done | (commands batch) |
| EditorState: 52 fields → 8 substructs (editor, terminal, ai, mobile, ext, memory, services + flat config_dir) | done | (state-decomp commit) |
| Batch 10: services/workspace/compat domains + remaining infra (bridges, gateways, profile) + application/jobs | done | (batch-10 commit) |
| Drop kortex/daemon dep — WRONG premise: daemon IS used by attachment_manager (GistInjector, VECTOR_DIM). Dep stays. | done | n/a |
| Delete dead never-declared files: repository, editor_service, zed_test, openwebui_client, provider_manager, provider.rs.dead | done | (dead-files commit) |
| Cleanup: delete pub-use shims; cargo check + cargo test green | todo | — |

## Milestone A2 — Frontend DDD restructure (src/)
| Task | Status | Commit |
|---|---|---|
| Consolidate stray folders: agent, architecture, services, utils, security, mcp → layers | todo | — |
| README headers for kept subsystems: kortex, airi, hermes, claurst | todo | — |
| Split RightSidebar.tsx: standalone components extracted (2,776 → 2,323); main 2.3K component fn still needs data-flow decomposition | in-progress | (rightsidebar commit) |
| Split AgentSettingsView.tsx (2,003) → components/settings/agent/* | todo | — |
| Editor.tsx: extract monacoSetup/decorations/keybindings → application/editor/ | todo | — |
| Split agentSlice.ts (1,267) → agentMessagesSlice + agentToolsSlice + agentModesSlice | todo | — |
| scripts/check-architecture.mjs (no invoke() outside infrastructure) wired into npm test | todo | — |
| Migrate invoke() violations in components/store → infrastructure adapters | todo | — |
| Hooks layer: src/hooks selector hooks with useShallow | todo | — |

## Milestone B — VSCode-native UI polish
| Task | Status | Commit |
|---|---|---|
| Remove framer-motion (4 usages) + drop from package.json — CSS shim src/lib/motionShim.tsx | done | a9b29382 |
| Clamp transitions ≤150ms; killed 45 perpetual GitGraph petal animations + SMIL pulse | done | (gitgraph commit) |
| Move Dark+ palette into tokens.css with VSCode theme-key naming; no raw hex in components | todo | — |
| Density: was already VSCode-spec except status bar 24→22px (fixed) | done | c17a78be |
| Scrollbars already VSCode-style; :focus-visible accent rings added; codicon sweep todo | in-progress | c17a78be |
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
| terminal_pending Mutex → mpsc: DEFERRED — current std Mutex is a deliberate design (PRIMARY transport, blocking PTY reader must never drop bytes; see state.rs doc). Changing it without runtime soak-testing risks the most-used feature. Revisit with a measured contention profile. | blocked(needs runtime profiling) | — |
| vfs commands → spawn_blocking (vfs_write_atomic, vfs_apply_patch). Premise overstated: only ~6 std::fs calls existed, 2 in async paths. | done | 5ba3f3c2 |
| Background indexing deferred 10s post-boot (UI paints first). Full OnceCell lazy-init deferred: Arc fields have many call sites, low residual win since vector indexing already skips startup indexing. | done | 5ba3f3c2 |
| RwLock / immutable Arc for read-mostly AI data; mpsc activity log | todo | — |
| Try lto="thin" in release profile on macOS; keep only if green | todo | — |
| useShallow sweep: NOT NEEDED — audit showed all 364 are single-field selectors (already referentially stable). React.memo added to rightSidebar sub-panels. Premise was wrong. | done | 5ba3f3c2 |
| vite manualChunks already isolates three/vrm/monaco/xterm/reactflow/markdown/tauri | done | n/a |
| EditorState::new tracing metric added; record numbers during next `npx tauri dev` smoke | in-progress | 5ba3f3c2 |

## Milestone E — Extension API
| Task | Status | Commit |
|---|---|---|
| Sidecar JS host + JSON-RPC: require('hades') runtime in ext-host/index.js, capability-gated per manifest | done | 9dee9b26 |
| packages/hades-extension-api typed .d.ts + sidecar runtime implementing it | done | 9dee9b26 |
| Manifest validation (validate_hades_manifest): capabilities + contribution points, 4 tests | done | 9dee9b26 |
| Capability denial → IDE toast (permissionDenied message). Full interactive grant-flow via tool_permissions: future iteration — v1 is declare-or-deny, no runtime grants. | done(v1 scope) | 9dee9b26 |
| docs/extensions/API.md + examples/hello-extension | done | d839e488 |
| Open VSX hardening: timeout/offline/429 actionable errors; ExtensionsView surfaces them | done | 9dee9b26 |

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

- 2026-06-13: Headless web-chat-as-model + Cursor-style agent UI.
  Backend: web_chat_driver.rs (dedicated stealth-Firefox sidecar drives logged-in
  claude.ai/deepseek), webchat_openai_shim.rs (OpenAI /v1/chat/completions on :1539),
  sidecar add_cookies for one-time-login persistence. Providers webchat-claude /
  webchat-deepseek wired into get_endpoint (→:1539), keyless lists, list_models, and
  FORCED onto the text JSON tool protocol (autonomous.rs is_webchat) so the web model
  emits {"name","arguments"} blocks the loop's try_parse_markdown_tool_calls extracts.
  Commands webchat_login/webchat_sessions; frontend model-menu entries + one-time login.
  Ollama untouched. cargo check + typecheck green.
  UI (Cursor parity): flattened assistant messages (borderless), edit cards show
  filename chip + +N/−N badges in header, richer diff coloring (left gutter), composer
  rounded (12px) + focus accent, mode/model selectors as Cursor pills with carets,
  MultiFileReviewBanner neutralised to "{n} Files · Review" row.
