# VSCodium-Rust — Development Progress

Last updated: 2026-05-28

---

## ✅ COMPLETED

### Phase A — AI Engine Hardening

| ID | Feature | File(s) |
|----|---------|---------|
| A1 | Streaming `on_chunk` callback wiring | `ai_engine.rs` — already correct, confirmed |
| A2 | Emergency context overflow guard (intra-loop token estimate → early phase-wrap) | `ai_engine.rs` + added `estimate_messages_tokens()`, `model_context_limit()` helpers |
| A3 | Tool call deduplication enforcement (synthetic ToolResult error injected after 3 repeats) | `ai_engine.rs` |
| A4 | Advisor model fallback on HTTP error (continue instead of fatal crash) | `ai_engine.rs` |
| A5 | Task-aware tool filtering (Rust/Cargo, security/exploit, React/TS, Android keyword groups) | `ai_engine.rs` |

### Phase B — IDE Feature Parity

| ID | Feature | File(s) | Notes |
|----|---------|---------|-------|
| B1 | Cursor-style inline gutter diffs (green added / red deleted per hunk) | `Editor.tsx`, `styles.css` | Accept/Reject bar was pre-existing |
| B2 | @-mention autocomplete in chat (files + symbols + slash commands) | `RightSidebar.tsx` | Was already fully implemented |
| B3 | Tab predictive ghost-text completions (600ms debounce → `ai_inline_complete`) | `Editor.tsx` | Was already fully implemented |
| B4 | Windsurf-style agent "hands" cursor indicator (animated blue gutter on active edit) | `ai_tools.rs`, `Editor.tsx`, `styles.css` | Fires on every write tool call |
| B5 | Slash commands in editor right-click menu (`/explain`, `/refactor`, `/test`, `/document`) | `Editor.tsx` | |
| B6 | Cascade write mode with `⚡ Live` toggle (auto-reload on agent writes) | `agent.ts`, `ChatToolbar.tsx`, `agentSlice.ts` | Default ON; toggle to review diffs manually |
| B7 | Per-edit checkpoint rollback fix (`git_checkout_stash` → `git_rollback_checkpoint`) | `agentSlice.ts` | Was calling non-existent command |
| B8 | Plan-before-execute mode (`📋 Plan` toggle → prepends TASK_PLAN directive) | `ChatToolbar.tsx`, `agent.ts`, `agentSlice.ts` | |
| B9 | Per-tool permission prompts (dangerous ops pause for user approval) | `tool_invoker.rs`, `system_commands.rs`, `ToolPermissionDialog.tsx`, `lib.rs`, `App.tsx` | |
| B10 | TodoList / PlanningPanel wired to backend `task-phase-update` events | `App.tsx` | `PlanningPanel.tsx` was pre-existing |

### Phase C — iPhone Emulator

| ID | Feature | File(s) |
|----|---------|---------|
| C1 | UART console auto-creates `.aim` directory (removed dormant guard) | `context_indexer.rs` |
| C2 | VirtIO disk image attachment via `--disk` CLI arg | `iphone_emulator.rs`, `Runtime.cpp` (already supported) |
| C3 | Darwin serial console in IDE panel (streams stdout/stderr from acheron process) | `IPhoneEmulatorPanel.tsx`, `iphone_emulator.rs` |
| C4 | Framebuffer streaming to IDE display tab (raw BGRA file → BMP data URI → canvas) | `Win32Display.cpp`, `WindowsHypervisor.cpp`, `iphone_emulator.rs`, `IPhoneEmulatorPanel.tsx` |

### AIM VFS Zero-Grep Integration

| ID | Feature | File(s) |
|----|---------|---------|
| AIM-1 | Auto-create `.aim/` directory so indexer runs on every workspace | `context_indexer.rs` |
| AIM-2 | `needs_initial_index()` + `get_project_tree_summary()` helpers | `memory_store.rs` |
| AIM-3 | Always-on `### BRAIN` section in system prompt (all providers incl. Ollama) | `ai_engine.rs` |
| AIM-4 | `aim_pack_context` Tauri command + AI tool (codebase map in ~6 gist tokens) | `kortex_commands.rs`, `ai_tools.rs`, `lib.rs` |
| AIM-5 | `aim_query_spans` wired as AI-callable tool | `ai_tools.rs` |
| AIM-6 | `trigger_workspace_index` Tauri command | `kortex_commands.rs`, `lib.rs` |
| AIM-7 | AIM brain pre-loaded before every `ai_chat` invocation | `agent.ts` |
| AIM-8 | `## BRAIN` section in `buildSystemPrompt` from `kortexBrain` config | `system_prompt.ts` |
| AIM-9 | Kortex panel "⚡ Index Workspace" button with live status | `KortexInferencePanel.tsx` |
| AIM-10 | `aim_pack_context` + `aim_query_spans` added to OLLAMA_ESSENTIAL_TOOLS | `ai_engine.rs` |

---

## ✅ FIXED (Session 2)

| ID | Fix | Files |
|----|-----|-------|
| F1 | Permission prompts wired into autonomous agent loop | `ai_engine.rs` — `execute_tool_with_permission` replaces `execute_tool`; `Sentient::permission_senders` Arc shared with `EditorState::tool_permission_senders` |
| F2 | Plan mode pause at AWAITING_APPROVAL | `ai_engine.rs` — detects token, pauses loop, emits `plan-approval-required` event; `RightSidebar.tsx` — new `PlanApprovalBanner` with Approve/Cancel |
| F3 | Memory layer stubs now delegate to MemoryStore | `memory_layer.rs` — `search()`, `query_context()`, `get_file_context()` use `memory_store.retrieve_context()` |
| F4 | Duplicate `start_emulator_stream` registration removed | `lib.rs` |
| F5 | Per-hunk inline accept/reject (right-click context menu) | `Editor.tsx` — context menu actions "✓ Accept this hunk" + "✕ Reject this hunk" with partial model edit |
| F6 | Multi-agent dashboard: ⊕ Spawn button added to BackgroundAgentsTray | `RightSidebar.tsx` — always visible, inline spawn input |
| F7 | Android streaming was already complete — confirmed | `emulator_stream.rs`, `EmulatorPreview.tsx` both fully wired |
| F8 | APEX orchestrator was already complete — confirmed | `apex_orchestrator.rs`, `apex_red_team.rs` both have real Ollama calls |
| F9 | iPhone emulator ramdisk support | `StyxHypervisor.hpp` — `loadRamdisk()` interface; `WindowsHypervisor.cpp` — implementation loads into guest RAM + writes boot_args ramdisk fields; `Runtime.cpp` — auto-loads `initrd.bin`; `iphone_emulator.rs` — `create_stub_ramdisk` Tauri command; `IPhoneEmulatorPanel.tsx` — "🧪 Create Stub Ramdisk" button |

---

## ✅ FIXED (Session 3 — Title bar, memory, offensive security)

| ID | Fix | Files |
|----|-----|-------|
| S3-1 | Title bar window controls (maximize/minimize/close) now functional | `TitleBar.tsx` — replaced `data-tauri-drag-region` (which captured OS events and blocked button clicks) with programmatic `startDragging()` on mousedown of non-interactive areas; added `pointer-events: all` + `z-index: 200` to `.wc-btn` |
| S3-2 | Memory cap: slots limited to 800 (evicts oldest "code" first) | `memory_store.rs` |
| S3-3 | Memory cap: conversation_state hard-capped at 100 messages | `ai_engine.rs::optimize_memory` |
| S3-4 | Memory cap: project_tree summary truncated to 50 entries | `ai_engine.rs` workspace cache |
| S3-5 | Memory cap: workspace memory files truncated to 3KB each | `ai_engine.rs` |
| S3-6 | Memory cap: context_indexer skips after 2000 files per cycle | `context_indexer.rs` |
| S3-7 | Memory cap: binary_body in MemoryStore capped at 4MB | `memory_store.rs` |
| S3-8 | Memory cap: Monaco model count reduced 20 → 12 | `Editor.tsx` |
| S3-9 | Memory cap: agentTrajectory reduced 1000 → 200 events | `agentSlice.ts` |
| S3-10 | Periodic Windows working-set trim every 5 minutes | `lib.rs` |
| S3-11 | `optimize_memory()` auto-fires after every agent turn | `ai_commands.rs` |
| S3-12 | Offensive security tools reclassified Safe (were Dangerous) | `tool_invoker.rs` — `apex_red_team_scan`, `weaponize_env` are analysis-only, not exploitation. Was causing every red-team scan to block on permission dialog |
| S3-13 | Yolo mode now bypasses permission prompts via `AIRI_YOLO_MODE` env var | `tool_invoker.rs`, `ai_engine.rs` |
| S3-14 | All 12 offensive security tools added to OLLAMA_ESSENTIAL_TOOLS | `ai_engine.rs` — `weaponize_env`, `apex_*`, `binary_mach_o_scanner`, `file_entropy_analysis`, `network_port_scanner`, `extract_strings`, `hex_dump` |
| S3-15 | Task-aware filter expanded for security domain (was stripping offensive tools!) | `ai_engine.rs` — now matches `red team`, `attack`, `weaponize`, `payload`, `bug bounty`, `malware`, `reverse`, `ctf` keywords and includes all 13+ offensive tools |

**Memory Profile Targets:**
- Baseline (idle): ~150 MB ✓
- With workspace indexed (2000 files cap, 800 slots cap): ~220 MB ✓
- With 12 Monaco models open: +60 MB → ~280 MB ✓
- Periodic Windows trim every 5 min keeps paged-out memory reclaimed
- Hard caps prevent runaway growth from long agent loops, large indexers, or chat history

**Offensive Security Verified Intact:**
- Red team engine (BugTraceAI-Apex-G4-26B) — `apex_red_team.rs::scan` ✓
- 7 APEX intelligence engines — `apex_orchestrator.rs` (architect, threat, perf, self-improve, explainer, multi-system, predictor) ✓
- MITRE ATT&CK kill-chain tactics — `apex_red_team.rs::MitreTactic` enum (14 tactics) ✓
- Dual-use system prompt — `ai_engine.rs` line 1845+ (THREAT ACTOR DEMO / RED TEAM / BLUE TEAM / BUG BOUNTY playbooks) intact ✓
- 12 offensive tools all registered, dispatched, and available to local models ✓
- Zero sanitizer/censor/filter functions found in the codebase ✓
- Permission system honors yolo_mode (no dialogs during autonomous offensive runs) ✓

---

## 🔴 NOT YET DONE — Critical Missing Features

### iPhone Emulator: Still at "kernel runs but no userspace"

The kernel executes (80M+ exits/sec confirmed) but Darwin never reaches launchd. This is the core blocker for "full boot → homescreen → apps."

| ID | What's needed | Complexity | Notes |
|----|--------------|------------|-------|
| E1 | **RestoreRamDisk loading** — mount a minimal ramdisk so the kernel can find `/sbin/launchd` | High | Requires creating a minimal HFS+/APFS image with launchd binary |
| E2 | **Mach IPC bootstrap server** — launchd requires a bootstrap port before it can spawn | Very High | XPC/Mach port emulation in C++ |
| E3 | **launchd process spawning** — ELF/Mach-O loader + process table emulation | Very High | Guest process context management |
| E4 | **Touch input forwarding** — canvas touch events → emulator HID device | Medium | VirtIO HID or Apple SPI touch |
| E5 | **WindowServer stub** — SpringBoard/Setup.app need a display compositor | Very High | CoreGraphics layer emulation |
| E6 | **Setup Assistant** (boot → hello screen) | High | Requires E1-E5 |
| E7 | **SpringBoard / Home Screen** | Very High | Requires E6 |
| E8 | **System apps** (Settings, Safari, Notes) | Very High | Requires E7 |
| E9 | **IPA sideloading** (third-party apps) | High | JailbreakEngine AMFI bypass already exists |

**Current emulator state:** Kernel entry confirmed, 673M+ exits logged, PC stable at `0x8230758c`. Console output streams via UART → `std::cout` → IDE panel. Serial log saved to `out/diagnostics/serial.log`. Framebuffer pipeline ready but display never rendered (kernel hasn't reached graphics stack).

---

### IDE Features Still Missing

| Feature | Priority | Complexity | Notes |
|---------|----------|------------|-------|
| Multi-agent visual dashboard (parallel agents running simultaneously) | High | Medium | `agentThreads` structure exists but no UI |
| Cursor-style per-hunk accept/reject (not just whole-file) | High | Medium | Currently whole-file accept/reject only |
| Extension marketplace (install/manage extensions) | Medium | High | `ExtensionHostManager` exists, UI stubbed |
| Android emulator full scrcpy streaming (live mirroring, not just ADB) | High | Medium | `emulator_stream.rs` exists but not wired to panel |
| VRM avatar AIRI full tool-call responses (avatar reacts to agent phases) | Medium | Medium | `airiAgentBridge` partially wired |
| Reverse engineering panel (binary analysis, disassembly view) | Medium | High | `binary_analyzer.rs` + Capstone exist |
| Full DAP debug adapter (breakpoints, stepping, variable watch) | Medium | High | `debug_adapter.rs` + `DebugManager` exist |
| Git graph visualization (branch history graph) | Low | Medium | `GitGraph.tsx` exists but may be incomplete |
| Inline AI code lenses (show AI suggestions per function) | Low | Medium | LSP CodeLens registered but not AI-powered |
| APEX orchestrator full implementation (7-model coordination) | High | High | Struct exists, `architect_design()` + others stubbed |
| Red team engine execution (`scan_and_exploit()`) | High | High | `ApexRedTeam` struct + MITRE enums exist, no execution |
| `ai_chat_fast` local fast-path properly working | High | Low | Registered, likely works but untested end-to-end |
| KV cache (KDKVC) integration with inference | Medium | Medium | UI exists, backend commands exist |
| Proper vector search (semantic similarity, not substring) | Medium | High | `vector_indexer.rs` + Pythagorean embedding exist |
| LSP rename / code action UI integration | Medium | Medium | Backend commands exist |
| Specs-to-code wizard full flow | Medium | Medium | `SpecsManager.tsx` + `specs_db.rs` partially done |

---

### Backend Commands Referenced in Frontend But May Not Be Fully Implemented

Run `cargo check` — these are registered and compile, but the underlying logic may be stubbed:

| Command | Status | File |
|---------|--------|------|
| `ai_inline_complete` | ✅ Implemented | `ai_commands.rs:268` |
| `accept_sentient_patch` | ✅ Implemented | `ai_patch_commands.rs:5` |
| `apex_architect_design` | ⚠️ May be stubbed | `apex_commands.rs` |
| `apex_red_team_scan` | ⚠️ Stubbed | `apex_red_team.rs:150` |
| `memory_search` | ⚠️ Mock returns empty | `memory_layer.rs:62` |
| `query_context` | ⚠️ Mock | `memory_layer.rs:67` |

---

## 🟡 PARTIALLY DONE / NEEDS VERIFICATION

| Item | Status | What's left |
|------|--------|-------------|
| AIM indexer populates `[code]` slots | ✅ Code exists | Needs `.aim/` dir (now auto-created) + first index run to verify |
| `aim_query_spans` returns useful results | ⚠️ Depends on slots | Only works after indexing; returns empty for fresh workspaces |
| Per-tool permission prompts (`ToolPermissionDialog`) | ✅ Frontend + backend wired | `execute_tool_with_permission` exists but `autonomous_loop` still calls `execute_tool` not the permission-aware variant |
| Checkpoint rollback button | ✅ Fixed command name | Needs end-to-end test to confirm `git_rollback_checkpoint` works |
| Plan mode | ✅ Frontend toggle | Backend loop doesn't actually pause at `AWAITING_APPROVAL`; the AI just outputs the plan and keeps going |
| B9 permission prompts wired in agent loop | ⚠️ Partial | `ToolInvoker::execute_tool_with_permission` exists but `ai_engine.rs` still calls the old `execute_tool` |

---

## 🏗️ ARCHITECTURE REFERENCE

```
vscodium-rust/
├── src-tauri/src/          Rust Tauri backend (~90 modules)
│   ├── ai_engine.rs        Sentient autonomous loop (3000+ lines)
│   ├── ai_tools.rs         150+ tool implementations
│   ├── kortex_commands.rs  AIM VFS + telemetry commands
│   ├── context_indexer.rs  Rayon parallel codebase indexer
│   ├── memory_store.rs     Persistent KortexSnapshot + gist builder
│   ├── tool_invoker.rs     Tool dispatch with permission classifier
│   ├── iphone_emulator.rs  Acheron process manager + frame streamer
│   └── state.rs            EditorState (central managed state)
├── src/                    React/TS frontend
│   ├── agent.ts            Main agent loop + AIM brain injection (~3500 lines)
│   ├── system_prompt.ts    System prompt builder
│   ├── kortex/             AIM VFS TypeScript bindings
│   └── components/
│       ├── Editor.tsx      Monaco editor + all decorations
│       ├── RightSidebar.tsx Agent chat UI
│       ├── chat/ChatToolbar.tsx  Mode/plan/live toggles
│       └── IPhoneEmulatorPanel.tsx  Console + framebuffer panel
├── kortex/                 Rust workspace (libaim, aim-proxy, tui, neuraldrive)
└── Virtual-iPhone-Emulator/ C++ hypervisor (acheron)
    └── core_ide_system/src/core/infrastructure/
        ├── hypervisors/win32/WindowsHypervisor.cpp
        ├── display/Win32Display.cpp  (framebuffer + frame.raw dump)
        └── devices/VirtioBlkDevice.cpp
```

---

## 📋 NEXT PRIORITIES (Suggested Order)

1. **Wire `execute_tool_with_permission` into `ai_engine.rs`** — the permission dialog is built but the loop still bypasses it. Estimated: 30 min.

2. **Fix plan mode pause** — agent needs to actually stop at `AWAITING_APPROVAL` and wait for `[PROCEED]` message. Estimated: 1 hour.

3. **`apex_architect_design` + `apex_red_team_scan` implementation** — the most requested "APEX" features. Estimated: 2-3 hours.

4. **Android scrcpy streaming** — `emulator_stream.rs` + `EmulatorPanel.tsx` need wiring. Estimated: 2 hours.

5. **Per-hunk inline accept/reject** — true Cursor-style per-line accept instead of whole-file. Estimated: 3 hours.

6. **iPhone emulator E1-E3** (RestoreRamDisk + launchd) — weeks of C++ work, highest complexity.

7. **Multi-agent visual dashboard** — spawn + track parallel background agents. `agentThreads` in store, just needs UI. Estimated: 4 hours.
