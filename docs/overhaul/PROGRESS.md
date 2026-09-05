# Overhaul Progress

> **Handoff contract**: whoever works on this (Claude Code, Cursor, human) MUST update this file
> before every commit: flip statuses, record the commit hash, and write the "Next action" line so
> the next session can resume cold. Read `MASTER_PLAN.md` for the full plan and `CONVENTIONS.md`
> for the architecture rules before touching code.

**Statuses**: `todo` | `in-progress` | `done` | `blocked(<reason>)`

**Next action**: Lemonade offload advisor + inference-path audit DONE (2026-07-06). (1) ollama_offload.rs: lemonade_offload_advice(model) + lemonade_doctor tauri command (registered lib.rs) — since Lemonade runs an EXTERNAL llama.cpp server whose GPU split is launch-time (-ngl), the IDE can't set num_gpu per-request; advisor computes recommended -ngl from detect_vram_gb + plan_gpu_layers and surfaces launch guidance (mirrors ollama_doctor). 3 tests (13 total in module). (2) AUDIT of local-inference provider gating: most paths already Lemonade-inclusive — `is_ollama` var actually means "Ollama-compatible local" and INCLUDES lemonade+antigravity (so forced_tool_choice, native-tool path already cover Lemonade); preflight prompt-trim (autonomous.rs:1559), timeouts, progress HUD, empty-stream detection, KV-cache Kv tier (Lemonade-llamacpp first-class) all include lemonade. Correct-by-design Ollama-only: ollama_openai_compat (lemonade uses OpenAI path), keep_alive, options block (OpenAI-compat rejects Ollama fields). FIXED one real asymmetry: the 400 "does not support tools" retry was Ollama-only → now covers lemonade (guarded by error text). Remaining minor Ollama-only: context-overflow 400 auto-retry (lemonade overflow is HTTP-200-JSON, handled separately + preflight trim covers it) and reasoning-param injection. Full crate cargo check clean. Commits da223dfd (parent) + 0704f7d (submodule) NOT pushed.

VRAM-tiered offload + turbovec integration DONE (2026-07-06). (1) ollama_offload.rs: detect_vram_gb (HADES_VRAM_GB override → nvidia-smi → Apple unified 70% of RAM → None), model_param_billions, estimate_layers, plan_gpu_layers (Q4 ~0.56GB/B, 1.5GB reserve), recommended_num_gpu — wired into providers.rs ollama_inference_options (replaces env-only HADES_NUM_GPU); 10 tests. So a 27B partially offloads to RAM on 8GB instead of OOMing (still slower — physics). (2) turbovec (TurboQuant quantized vector index) now backs domain/indexing/ann_index.rs — IdMapIndex(bit_width=4), same public API (upsert/search/load/clear/len), stable_id→u64, legacy ann_flat.json migration; 5 tests. DEP FIX (important): workspace [patch.crates-io] redirected rand→git tag 0.8.5 but not rand_core/rand_chacha, so rand_distr linked a DIFFERENT rand_core@0.6.4 instance → trait mismatch; fixed by adding rand_core + rand_chacha to the same [patch] (src-tauri/Cargo.toml). Used add_with_ids_2d (lazy-dim). Full crate cargo check clean. KORTEX BUILT PROPERLY (2026-07-06, user chose submodule approach): local kortex/ was a gitignored 3-crate fragment (libaim/harness/daemon). Backed up to kortex.local-backup/ (gitignored), un-ignored kortex/ in .gitignore, added github.com/H4D3ZS/kortex as a git submodule at kortex/ (full 9-crate workspace + vendored llama.cpp + whitepaper.md). REGRESSION FIX: submodule daemon had `sha3 = "*"` → resolved sha3 0.12 whose API broke `sha3::Shake256` (watermark.rs); pinned to `sha3 = "0.10"` in kortex/daemon/Cargo.toml (SUBMODULE working-tree edit — needs committing upstream to H4D3ZS/kortex). After that, src-tauri (the IDE) cargo check CLEAN against submodule crates. Verified building: aim-proxy, vfs_layer, hades-bridge, hades-tui, hades-kernel (HIP feature-gated off), libaim, harness, daemon. UNTESTED: neuraldrive/src-tauri (Tauri GUI, needs npm frontend). Backup kortex.local-backup/ retained until user confirms.

Tiered codebase map for cheap AI context DONE (2026-07-06). Goal: give the AI whole-repo awareness without dumping raw files (token cost / small-ctx breakage). NEW domain/indexing/codebase_map.rs — generate_repo_map(root, max_chars): deterministic one-line-per-directory map with per-module summary from entry-file (mod.rs/lib.rs/index) doc comment; char-budgeted + truncation note; skips node_modules/target/etc. Reuses existing structural_blueprints.rs (already built, was never fed to AI) for per-file detail. Two tools wired (dispatch + schemas + fs_tools impl): `codebase_map` (compact orientation) and `get_file_signatures(path)` (stripped-body signatures on demand). ALWAYS-ON injection: map added to the run-start stable system prompt (autonomous.rs ~line 906) so it rides Anthropic's cache_control block (0.1x on repeats) and forms a stable local KV prefix; budget 2K chars small models / 8K else. 7 codebase_map tests + full crate cargo check clean; 57 kvcache tests still green. PERSISTENCE/CACHING DONE (2026-07-06): generate_repo_map_cached(root, max_chars) persists to .kortex/codebase_map.json (+ .md sidecar, gitignored) keyed on tree_signature = SHA-256 over sorted (rel_path, mtime, size) of source files — invalidates on add/remove/rename/edit; unchanged tree returns byte-identical cached map (keeps downstream Anthropic/KV prefix stable); budget mismatch busts cache; atomic tmp+rename write. Both call sites (autonomous.rs injection + codebase_map tool) use the cached variant. 11 codebase_map tests pass. NOTE: the tree walk still runs each call to compute the signature (cheap, no content reads) — only entry-file reads + render are skipped on cache hit. Prior KV-cache work below.

KV-cache save/restore correctness hardened (2026-07-06, kortex_kvcache): extracted the duplicated save-gate logic from proxy.rs (spawn_save_after + flush_shutdown_checkpoint, which differed only in cold_max enforcement) into `store::plan_save_count(opts, n, enforce_cold_max)` — single source of truth so the save side and `longest_prefix` match side can't drift. Added 5 tests incl. the core save→restore round-trip invariant (planned save length == length longest_prefix finds) and the "request shorter than saved prefix must not match" guard. 54 kvcache tests pass. OPEN DIRECTION (user ask): extend kortex KV reuse to Ollama + Lemonade — BLOCKED by API surface: KDKVC needs llama.cpp slot-save/restore (`--slot-save-path`), which Ollama does NOT expose and Lemonade only exposes on its llamacpp recipe (not ryzenai/NPU). Realistic path = provider-agnostic proxy: true disk-KV where slot-save exists (llama.cpp / Lemonade-llamacpp), exact-prefix completion caching + keep_alive tuning everywhere else (Ollama, Lemonade-NPU). DESIGN DOC: docs/overhaul/KORTEX_CACHE_DESIGN.md (three-tier arch: Tier0 residency/keep_alive, Tier1 KV-slot KDKVC, Tier2 provider-agnostic response cache; capability detection; determinism gating; 7 milestones). MILESTONE 1 DONE (2026-07-06): new kortex_kvcache/capability.rs — probe_tier (tokenize + non-destructive slot save probe) + resolve_tier; CacheTier enum {Auto,Kv,Response,Off} on KvCacheOptions (serde default Auto, back-compat); ProxyState.resolved_tier set at start via probe; handle_intercepted gates KDKVC to tier==Kv else transparent passthrough (safe in front of Ollama/Lemonade-NPU now); tier surfaced in RunningCacheInfo/status; CLI bin updated. 57 kvcache tests pass (+3 capability), full crate+bins cargo check clean. NEXT: milestone 3 (Tier 2 response cache: key incl. all sampling params+tools, non-stream hit/miss + tests) — milestone 2 (wire Tier1 in front of Lemonade-llamacpp) is detection-only now that routing exists.

Long-running-tool interruptibility FIXED (2026-07-06): trace showed the 180s stream watchdog is safe (only wraps model SSE, tool exec is a separate later phase; run_command has no timeout; prefetch is read-only allowlist gated) BUT a foreground scan already running could NOT be killed by Stop (is_stopped only checked between tools; child.wait on spawn_blocking is uncancellable). Fix: (a) new infrastructure/process_registry.rs — global PID set; run_command registers each foreground child (spawned into its own process group on unix via process_group(0)), Sentient::stop() calls kill_all() → taskkill /F /T on Windows, kill -KILL -<pgid> on unix; wired reachable via stop_ai_agent command. (b) auto-background known long-running scanners (nmap/ffuf/sqlmap/nuclei/… curated list in is_long_running_command) when caller didn't set background, so they stream to a pollable terminal instead of blocking the turn. Tests: long_running_command_tests (2) + model_classification_tests (3) all pass; cargo check clean. CAVEAT: auto-background means the model must poll terminal_get_status/read_output for scan results — weak local models may need prompt reinforcement. Prior fixes below still pending runtime smoke.

Local-model classification bug FIXED (2026-07-06, providers.rs `is_small_model_name`): bare "4b"/"3b"/"2b" substrings matched INSIDE "14b"/"24b"/"34b"/"12b", so qwen2.5-coder:14b, mistral-small:24b, yi:34b, gemma-4-12b were all misclassified as SMALL → forced onto text-JSON fallback and the ≥14B forced-tool-choice path (autonomous.rs:2011) became dead code for every mid-size model. Fixed by running the numeric param parser FIRST, substring tags only as fallback; e2b/e4b now matched as exact tokens. Added `model_classification_tests` (3 tests, all pass) — first real tests on the classification path. THEN the agent-loop resilience pass (autonomous.rs): (1) two-phase stream watchdog — 600s cold-start, 180s inter-token once flowing; a stall or transport error mid-turn now SALVAGES partial content instead of hanging 10 min or killing the run; (2) adaptive tool-protocol downgrade — a content-but-zero-tool-calls turn (observed with gemma-family fine-tunes that accept `tools` but never emit tool_calls) flips the run to the explicit text-JSON tool protocol and sends a targeted corrective (cut-off / code-as-prose / no-op variants); (3) 3 consecutive no-tool turns end the run gracefully with best answer instead of YOLO ping-pong to iteration 200. Recovery gated to turns with tool intent (stall, code fence, or yolo) so plain prose answers still return immediately. cargo check clean. Next: runtime smoke — re-run "create a scientific calculator" on gemma-4-12B-coder; plus the pending Lemonade live-server smoke (chat streaming on /api/v1/chat/completions, pull, offline→reconnect banner) and FCC start via uv.

Previous: Free Claude Code (FCC) integration complete. Changes: (15) Added FCC as git submodule at third_party/free-claude-code/; (16) Created fcc_sidecar.rs (~270 LOC) — Python sidecar manager with process spawn/monitor/stop, health check polling, global singleton; (17) Created fcc.rs Tauri commands (start/stop/status/health/get_url/open_admin); (18) Added 'fcc' to inferenceBackend type + fccUrl/fccStatus/fccEnabled state in inferenceSlice; (19) Created FccSettingsPanel.tsx — start/stop, status indicator, Admin UI button, backend switch; (20) Registered FCC in settings registry + SettingsPage; (21) Auto-start FCC on boot if enabled. TypeScript clean (only pre-existing RulesManager error). Remaining: runtime smoke test with FCC running, RightSidebar full decomposition, Voice/TTS offline fallback.

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

- 2026-07-03: Offline + Agent Resilience pass. Made IDE 100% offline-ready with
  Lemonade as primary backend. Backend changes: 7 hardcoded Ollama URLs in
  embeddings.rs / attachment_manager.rs / fs_tools.rs / vision_sidecar.rs /
  image_gen.rs now route through configured backend; VectorIndexer + AttachmentManager
  receive URL updates on backend switch. Frontend: agentResilience.ts provides
  auto-reconnect with exponential backoff (5s→60s) + message queue (max 20);
  OfflineBanner shows persistent status in chat; ChatErrorBlock provides styled
  error display with retry. StatusBar shows active backend status dot (clickable
  for LemonadeHealthDashboard). ModelSelectorPanel probes active backend first.
  Extension icons fall back to local codicon instead of external URLs. Added
  semantic tokens for status colors and component backgrounds. TypeScript clean
  (only pre-existing RulesManager error).

- 2026-07-03 (continued): Cursor 2.5 Composer parity + session persistence.
  Composer gains rich @mentions (MentionPopup: @codebase/@web/@git/@docs/
  @symbol/@folder/@problems/@terminal + slash commands + file search with
  keyboard navigation), markdown rendering (marked+sanitizeHtml for assistant
  messages), Background Composer button (run-in-background via runBackgroundAgent).
  Session auto-save every 30s via syncAgentMessages.startAutoSave(), crash
  detection via localStorage timestamp, CrashRecoveryBanner with restore/dismiss,
  beforeunload final save. Todo/checklist was already implemented (ComposerTodoList).
  TypeScript clean (only pre-existing RulesManager error).

- 2026-07-06 (health): Lemonade health-polling consolidation. inferenceSlice.
  checkLemonadeStatus is now the single source of truth: uses the
  check_lemonade_status command (empty model list no longer counts as
  "running"), 5s throttle guard dedupes mount-effect storms, and it records
  lemonadeLatencyMs in the store. LemonadeHealthDashboard dropped its private
  15s list_provider_models poll — it reads status/latency from the store and
  its Refresh button triggers the store check; "Loaded Models" relabeled
  "Available Models" (it lists the global model list, not Lemonade-loaded
  ones). typecheck + vitest (117) green.

- 2026-07-06 (perf): OOM/smoothness pass. (a) tauri.conf.json: renderer V8 heap
  cap raised 128MB → 1024MB (--max-old-space-size) and --in-process-gpu removed
  — the 128MB cap was THE recurring mid-generation OOM (Monaco + React + xterm
  + streaming transcript in one 128MB old-space). (b) memoryGovernor: dropped
  the dead absolute-byte thresholds (700MB/1200MB could never fire under the
  old cap); trims now key purely off used/limit ratio. (c) openFile.ts:
  MAX_OPEN_EDITOR_TABS 50 → 15, eviction loops past the cap, never evicts
  dirty tabs, model disposal extracted to disposeMonacoModelForPath().
  (d) terminal.rs: opencode PTY pending buffer now has the same 1MB drain cap
  as spawn_terminal (was unbounded); MAX_PENDING hoisted to module scope;
  removed the per-read `terminal-data` emit (frontend listener was a
  documented no-op — output renders via terminal_take_pending polling) and the
  matching dead listener in terminal.ts. (e) agentMessagesSlice
  appendLastAgentMessage: <think>-parse phase memoized per message; scans only
  the delta + carry tail instead of regex over the whole raw buffer per token
  (was O(n²) per turn). (f) AIRI activity drain (120ms invoke poll) pauses
  while document.hidden. cargo check + typecheck + vitest (117) green.

- 2026-07-06: Lemonade correctness pass. (a) Added `check_lemonade_status` +
  `pull_lemonade_model` Tauri commands (providers.rs, ai.rs, lib.rs) — the
  agentResilience probe previously invoked a non-existent command, so the
  default backend was permanently marked offline; the probe now also checks the
  returned boolean. (b) Lemonade chat now targets its real OpenAI-compatible
  endpoint `/api/v1/chat/completions` (prompt.rs) with a one-shot `/v1` retry on
  404 (streaming.rs single-shot + autonomous.rs); payloads use OpenAI shape
  (ollama_use_openai_compat_endpoint returns true for lemonade; Ollama-only
  `options`/`keep_alive` dropped); stream parser accepts OpenAI SSE
  `choices[0].delta.content`/`.reasoning_content` alongside Ollama NDJSON.
  Model listing tries `/api/v1/models` then `/v1/models`. (c) fcc_sidecar uv
  launch fixed (`uv run python server.py`, was invalid `uv server.py`).
  (d) detect_provider (commands/ai.rs) accepts a preferred provider so
  qwen/llama-named Lemonade models aren't name-sniffed to Ollama;
  PredictiveEditOverlay passes the active backend. (e) LemonadeSettingsPanel
  pull routes through Rust (JWT/CORS) instead of raw fetch. (f) removed
  duplicate CLOUD_PROVIDERS entries in agent.ts. Correction to the 2026-07-03
  FCC entry: sidecar spawns `python server.py` (not uvicorn); boot auto-start
  lives in App.tsx (fcc.enabled localStorage). cargo check + typecheck green.

- 2026-07-03 (FCC): Free Claude Code integration. Added FCC as git submodule
  at third_party/free-claude-code/. Created fcc_sidecar.rs — Python sidecar
  manager that spawns `uv run uvicorn server:app` on port 8082, monitors health
  via /v1/models polling, manages process lifecycle. Created fcc.rs Tauri commands
  (start/stop/status/health/get_url/open_admin). Added 'fcc' inference backend
  to inferenceSlice with fccUrl/fccStatus/fccEnabled state. Created
  FccSettingsPanel.tsx — start/stop controls, status indicator, Admin UI button,
  "set as backend" action. Auto-starts on boot if fcc.enabled=true in localStorage.
  FCC routes Anthropic Messages API traffic through 19+ providers (Ollama,
  Lemonade, OpenRouter, NVIDIA NIM, etc.). TypeScript clean.
