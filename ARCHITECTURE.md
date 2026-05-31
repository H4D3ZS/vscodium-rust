# ARCHITECTURE — VSCodium-Rust

A map of the codebase organised by **bounded context** (DDD). The `src-tauri`
backend is physically flat (~90 `.rs` files in one directory), so this doc is
the logical grouping that makes it navigable. Each context lists its modules
split into the DDD layers:

- **Application** — Tauri `#[command]` adapters (the IPC boundary). Thin; they
  validate input, call a domain service, and shape the response. Usually named
  `*_commands.rs`.
- **Domain / Engine** — the actual logic and stateful services.
- **Infrastructure** — external-system adapters (process spawning, HTTP, FFI,
  mmap, git2, tree-sitter).

> Rule of thumb: a `*_commands.rs` file should contain almost no logic — it
> delegates to an engine. When adding a feature, put logic in the engine and
> only expose a command. This keeps the IPC surface auditable.

---

## Central state

| Module | Layer | Role |
|--------|-------|------|
| `state.rs` | Domain | `EditorState` — the single Tauri managed state. All async fields are `Arc<Mutex<…>>`. Every command receives `tauri::State<EditorState>`. |
| `lib.rs` | Application | App builder + the **one** `invoke_handler![…]` registering every command. Background tasks (memory watchdog, working-set trim). |
| `main.rs` | Application | Binary entry → `lib::run()`. |
| `domain.rs` | Domain | Shared value objects (`FileEntry`, …). |
| `repository.rs` | Infrastructure | Project/FS abstraction (gpui/zed `Project`, `RealFs`). |

---

## 1. AI / Agent (the Sentient autonomous loop)

The flagship context: prompt → tool-calling loop → patches.

| Module | Layer | Role |
|--------|-------|------|
| `ai_engine.rs` | Domain | `Sentient` — autonomous loop, streaming, phase-wrap, context budgeting, BRAIN injection. |
| `ai_tools.rs` | Domain | 150+ tool implementations + dispatch. |
| `tool_invoker.rs` | Domain | Tool permission classifier (Safe/Dangerous) + execution gate. |
| `task_planner.rs` | Domain | Plan-mode state, `task-phase-update` events. |
| `workflow_engine.rs` | Domain | Multi-step workflow orchestration. |
| `attachment_manager.rs` | Domain | Context attachments + neural gist injector. |
| `ghost_runtime.rs` | Domain | Sandboxed `ghost_test` execution. |
| `hades_harness.rs` | Domain | MCTS verify loop (propose → shadow VFS → `cargo check` → commit). |
| `kairos.rs` | Domain | Time/scheduling-aware background indexing helper. |
| `jobs.rs`, `workers.rs` | Domain | Background job queue + worker pool. |
| `ai_prompts.rs` | Domain | Prompt templates (incl. dual-use offensive playbooks). |
| `ai_commands.rs`, `ai_agent_commands.rs`, `ai_project_commands.rs` | Application | Chat / agent / project command adapters. |
| `ai_patch_commands.rs` | Application | Accept/reject Sentient patches. |
| `ai_auth.rs` | Infrastructure | AI provider auth tokens. |

## 2. APEX / Offensive security (user-owned, unrestricted)

| Module | Layer | Role |
|--------|-------|------|
| `apex_orchestrator.rs` | Domain | 7 specialist engines (architect, threat, perf, …) over Ollama. |
| `apex_red_team.rs` | Domain | Red-team scan engine + MITRE ATT&CK tactics (BugTraceAI). |
| `security_distiller.rs` | Domain | Distils security findings into knowledge. |
| `hunter.rs` | Domain | Bug-bounty recon (URL/asset scanning). |
| `binary_analyzer.rs` | Domain | Mach-O / binary analysis (Capstone). |
| `apex_commands.rs` | Application | APEX command adapters. |

> These are **analysis-only** and classified `Safe` in `tool_invoker.rs`.
> Execution (`run_command`) remains `Dangerous`. Do not add censorship/filters.

## 3. Kortex AIM — memory, indexing, zero-grep VFS

| Module | Layer | Role |
|--------|-------|------|
| `aim_store.rs` | Infrastructure | `.aim` binary format read/write (memmap2). |
| `memory_store.rs` | Domain | `KortexSnapshot` + `SemanticSlot`; `build_compact_gist()`, tree summary. |
| `memory_layer.rs` | Domain | Retrieval facade over `memory_store`. |
| `memory_optimizer.rs` | Domain | RSS-aware trimming. |
| `context_indexer.rs` | Domain | Rayon + tree-sitter parallel codebase indexer. |
| `vector_indexer.rs` | Domain | Embedding index (lazy-started). |
| `context_key.rs`, `context_quantizer.rs` | Domain | Context keying + quantisation. |
| `knowledge_distiller.rs` | Domain | Knowledge-brief distillation. |
| `vfs_bridge.rs` | Infrastructure | Bridge to kortex VFS daemon. |
| `kortex_commands.rs`, `vector_commands.rs` | Application | AIM / vector command adapters (`aim_pack_context`, `aim_query_spans`, `trigger_workspace_index`). |

## 4. Editor / Files / LSP / Patching

| Module | Layer | Role |
|--------|-------|------|
| `editor_service.rs` | Domain | Editor model/state. |
| `patch_engine.rs` | Domain | Surgical SEARCH/REPLACE via `diffy` (no full-file rewrites). |
| `shadow_workspace.rs` | Domain | Virtual branch for safe mutation before commit. |
| `lsp.rs` | Domain | LSP client + tree-sitter diagnostics. |
| `keybindings.rs` | Domain | Keybinding registry. |
| `editor_commands.rs`, `file_commands.rs`, `lsp_commands.rs` | Application | Editor/file/LSP command adapters. |

## 5. Git / version control

| Module | Layer | Role |
|--------|-------|------|
| `git.rs` | Domain | git2 operations. |
| `git_checkpoints.rs` | Domain | Savepoints / rollback. |
| `git_commands.rs` | Application | Git command adapters. |

## 6. Terminal / process / system

| Module | Layer | Role |
|--------|-------|------|
| `terminal_commands.rs` | Application+Infra | PTY terminals (`portable-pty`). |
| `process_ext.rs` | Infrastructure | Process spawning helpers. |
| `system_commands.rs` | Application | System/window/permission-response commands. |

## 7. Emulators (iPhone / Android)

| Module | Layer | Role |
|--------|-------|------|
| `iphone_emulator.rs` | Application+Infra | Spawns acheron, streams console + frames, `send_iphone_touch`, `create_stub_ramdisk`. |
| `emulator_stream.rs` | Infrastructure | Android frame streaming. |
| `scrcpy.rs` | Infrastructure | scrcpy mirroring. |
| `android_commands.rs` | Application | Android command adapters. |

> The iPhone emulator's own C++ core (`Virtual-iPhone-Emulator/`) is DDD-layered
> separately — see that repo's `PROGRESS.md` / `CLAUDE.md`.

## 8. MCP (Model Context Protocol)

| Module | Layer | Role |
|--------|-------|------|
| `mcp_server.rs` | Infrastructure | MCP server. |
| `mcp_client.rs` | Infrastructure | MCP client. |
| `mcp_registry.rs` | Domain | Server registry. |
| `mcp_commands.rs` | Application | MCP command adapters. |

## 9. Providers / auth / web

| Module | Layer | Role |
|--------|-------|------|
| `provider_manager.rs` | Domain | Model-provider resolution (Ollama/cloud). |
| `openwebui_client.rs` | Infrastructure | OpenWebUI HTTP client. |
| `browser.rs`, `web_commands.rs` | Infra+App | Web fetch / browser actuation. |
| `auth_commands.rs`, `activation.rs` | Application | Auth + license activation. |
| `provider_commands.rs` | Application | Provider command adapters. |

## 10. Extensions / marketplace / specs / rules

| Module | Layer | Role |
|--------|-------|------|
| `extension_host.rs` | Domain | Extension host manager. |
| `marketplace.rs` | Infrastructure | Marketplace client. |
| `specs_db.rs` | Infrastructure | Specs persistence. |
| `rules_engine.rs` | Domain | Project rules. |
| `extensions_commands.rs`, `specs_commands.rs` | Application | Adapters. |

## 11. Debug (DAP)

| Module | Layer | Role |
|--------|-------|------|
| `debug_adapter.rs` | Infrastructure | DAP adapter. |
| `debug_commands.rs` | Application | Debug command adapters. |

## 12. Multimodal — vision / voice / visual / AIRI / ANE

| Module | Layer | Role |
|--------|-------|------|
| `vision.rs`, `hades_vision.rs`, `vision_bridge.rs` | Domain+Infra | Screen/image understanding. |
| `voice_commands.rs` | Application | Voice command adapters. |
| `visual_lab.rs`, `visual_commands.rs` | Domain+App | reactflow Visual Lab diagram builder. |
| `airi_bridge.rs` | Infrastructure | AIRI 3D avatar bridge. |
| `ane.rs` | Infrastructure | Apple Neural Engine FFI (macOS only). |

## 13. Performance + IDE-parity test modules

| Module | Layer | Role |
|--------|-------|------|
| `performance.rs`, `performance_commands.rs` | Domain+App | RSS/inference telemetry. |
| `antigravity_commands.rs` | Application | Antigravity-IDE-parity commands. |
| `zed_test.rs` | Test | Zed-integration probe. |

---

## Dependency direction (target)

```
Application (*_commands.rs, lib.rs)
        │  calls
        ▼
Domain / Engine (ai_engine, memory_store, patch_engine, apex_*, …)
        │  uses
        ▼
Infrastructure (aim_store, vfs_bridge, process_ext, git2, reqwest, FFI)
```

Commands must not contain business logic; engines must not call Tauri command
APIs. `EditorState` is the composition root that wires engines together.

---

## When adding a feature (checklist)

1. Put logic in the right context's **engine**, not in a command.
2. Expose it with a thin `#[tauri::command]` in that context's `*_commands.rs`.
3. Register the command in `lib.rs` (the single `invoke_handler!`).
4. If it touches shared state, add the field to `EditorState` in `state.rs`.
5. Keep patches surgical (`patch_engine` / `diffy`) — no full-file rewrites.
6. Run `cargo check` (backend) and `npm run typecheck` (frontend).
7. Update `PROGRESS.md`.
