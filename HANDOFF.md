# HANDOFF — Session Continuity Doc

**Purpose:** Everything needed to resume this work on another machine (e.g. the
M1) with zero context loss. Read this + `PROGRESS.md` + `ARCHITECTURE.md` and you
are caught up. Written 2026-05-29.

> Raw verbatim transcript of the originating session lives at:
> `C:\Users\HADES\.claude\projects\C--Users-HADES-Desktop-vscodium-rust\*.jsonl`
> (copy that folder too if you want the literal message log).

---

## 0. TL;DR — where we are

- **VSCodium-Rust IDE**: implementation-complete and **all build gates are green**
  (`cargo check`, `cargo test` 96 pass, `npm run typecheck`, `npm run build`).
  Integration audited (commands + events); the real wiring bugs are fixed.
- **iPhone emulator**: two separate engines exist. The from-scratch C++ `acheron`
  hypervisor is **stuck spinning at kernel entry**. The **`vphone-cli` Swift tool
  (Apple Virtualization.framework + PCC) is the WORKING path and runs only on
  Apple Silicon (the M1).** That is the route to a functional iPhone.
- **Big realization:** the active `acheron` backend is host-dependent
  (x86 → JIT, ARM macOS → HVF, ARM Windows → WHP). All the SpringBoard/boot/touch
  stub work from "Session 4" is in the **WHP** backend only — it does **not** run on
  the x86 Windows box, nor on the M1's HVF path. See §3.

---

## 1. The machine / backend matrix (critical — read first)

| Host | `acheron` backend selected | Runs ARM64 iOS kernel how | Notes |
|------|---------------------------|---------------------------|-------|
| **Windows x86_64** (this dev box, Radeon RX 580) | `JITHypervisor` (`#if __x86_64__`) | Binary translation ARM→x86 | Slow; kernel spins at entry. keystone/lzfse NOT installed → **can't even build here**. |
| **macOS Apple Silicon (M1)** | `MacOSHypervisor.mm` (HVF) | Native via Hypervisor.framework | Best host for the C++ path. |
| Windows ARM64 | `WindowsHypervisor.cpp` (WHP) | Native via WinHvPlatform | Where the Session-4 stubs live. |

**`HypervisorFactory::create()`** is defined per-backend, guarded by host `#if`s:
- `jit/JITHypervisor.cpp:1330` → `#if __x86_64__ || _M_X64`
- `macos/MacOSHypervisor.mm:774`
- `win32/WindowsHypervisor.cpp` (WHP)

**Consequence:** code added to one backend's run loop only runs on that host. The
domain models (`domain/userspace/*`) are backend-agnostic and reusable; the
*renderers/run-loop wiring* are per-backend.

---

## 2. The TWO iPhone-emulator engines

### 2a. `acheron` (C++ core) — from-scratch hypervisor
- Entry: `acheron run --prd iPhone13,2 [--disk <ramdisk>] [--diagnostics <dir>]`
- Prepare: `acheron prepare --ipsw <path> --out <dir>` → produces `<out>/raw/initrd.bin`
  (ramdisk), kernelcache, devicetree. `acheron run` auto-loads `raw/initrd.bin` as `rd=md0`.
- **Current ceiling:** kernel reaches entry (`0x82270000` in stale logs) then **spins
  forever at/near entry**, reading boot_args-shaped near-zero offsets, never reaching
  Darwin console / launchd. Logs in `vm/*.log` are **stale** vs current JIT layout
  (kernel @ GPA 0x0, boot_args @ 0x4000, X0=0x4000) — need a FRESH run to diagnose.
- **Userspace stubs (Session 4, WHP-only)** in case the WHP/ARM path is used:
  Mach IPC stub, launchd BSD-syscall stub, touch ring, SpringBoard surface, Apple
  boot animation. See §3.

### 2b. `vphone-cli` (Swift) — **the working path, M1 only**
- Boots real iOS 26.x via **Apple Virtualization.framework (PV=3) + PCC research VM**.
- Tested matrix (its README) boots real iPhone17,3 iOS 26.x incl. **jailbreak**
  (Sileo/apt/TrollStore auto-installed on first boot).
- **Host prereqs:** macOS 15+ (Sequoia), SIP/AMFI configured, an IPSW + a CloudOS image.
- Make-driven flow (`Virtual-iPhone-Emulator/vphone-cli/`):
  ```
  make help            # shows required vars/paths (parameterized — read this first)
  make setup_machine   # one-time host config
  make setup_tools
  make fw_prepare      # stage firmware from IPSW
  make fw_patch_jb     # Jailbreak variant (112 patches); or fw_patch_less (patchless)
  make cfw_install_jb
  make boot            # boot (boot_less for patchless)
  ```
- CLI subcommands: `vphone-cli boot`, `vphone-cli patch-firmware`, `vphone-cli patch-component`.
- **OPEN QUESTION for user:** M1's exact macOS version (must be 15+), and the output
  of `make help` (lists the exact required vars — not visible from the Makefile alone).

---

## 3. What was changed this session (file-by-file)

### IDE — memory + build
- `vite.config.mjs` — `manualChunks` splits three/vrm/monaco/xterm/reactflow/markdown/
  tauri into on-demand chunks. **FIXED build-breaker**: had listed uninstalled
  `@tauri-apps/plugin-fs`/`plugin-shell` → trimmed to `@tauri-apps/api` + `plugin-dialog`.
- `src-tauri/src/ai_engine.rs` — `OLLAMA_ESSENTIAL_TOOLS` lifted to module-level
  `pub(crate) const` (single source); added `Sentient::clear_conversation()`; added
  `#[cfg(test)] mod tests` (offensive + AIM tool guards).
- `src-tauri/src/tool_invoker.rs` — added `#[cfg(test)] mod tests` locking the
  Safe/Caution/Dangerous classification (offensive tools must stay Safe).

### IDE — integration fixes (frontend invoke ↔ backend command audit)
- `src-tauri/src/ai_project_commands.rs` — implemented `clear_ai_memory`,
  `search_codebase_files`.
- `src-tauri/src/airi_bridge.rs` — implemented `airi_event` → emits namespaced
  `airi:{event}` (lights up AiriOverlay's dead avatar listeners).
- `src-tauri/src/iphone_emulator.rs` — `find_acheron` made `pub`; added `launch_vphone`
  (legacy menu → real manager); added `prepare_ios_firmware` (runs `acheron prepare`,
  streams console, emits `ios-firmware-prepared`); added BMP-converter tests.
- `src-tauri/src/lib.rs` — registered: `git_get_unmerged`, `clear_ai_memory`,
  `search_codebase_files`, `airi_event`, `launch_vphone`, `prepare_ios_firmware`.
- `src/components/IPhoneEmulatorPanel.tsx` — IPSW field + "📦 Prepare Firmware" button;
  iPhone bezel + Dynamic Island around the display; touch-event forwarding
  (`send_iphone_touch`) with device-pixel mapping; listens for `ios-firmware-prepared`.

### Emulator (acheron) — userspace stubs + DDD (WHP backend only)
- Domain (`core_ide_system/src/core/domain/userspace/`): `AppCatalog`, `BootSequence`,
  `HomeScreenModel`, `LockScreenModel` — pure, resolution-independent, **unit-tested**
  (`tests/userspace_domain_test.cpp`, 53 checks, builds via CMake `userspace_domain_test`).
- Infrastructure (`core_ide_system/src/core/infrastructure/`):
  `mach/MachIPCStub`, `launchd/LaunchdSyscallStub`, `input/TouchInputBridge(+C)`,
  `springboard/SpringBoardSurface` (renders HomeScreenModel + per-app content + lock
  screen + paging + glyphs), `boot/AppleBootAnimation` (renders BootSequence).
- `hypervisors/win32/WindowsHypervisor.cpp` — SVC dispatch (X16<0→Mach, ≥0→launchd),
  boot-anim→SpringBoard handoff, touch poll (`touch_in.csv`), guest framebuffer dump
  (`guest_frame.raw`), disk-dump throttle (~250 ms).
- `display/Win32Display.cpp` — added `win32_display_save_raw` wrapper (fixed a latent
  missing-declaration bug; that Windows path never compiled before).
- `CMakeLists.txt` — registered all new sources + the `userspace_domain_test` target.

### Docs
- `PROGRESS.md` — Sessions 4/5/6 logged (this is the running status).
- `ARCHITECTURE.md` — IDE bounded contexts + app/domain/infra layering.
- `HANDOFF.md` — this file.

---

## 4. Verification status (all green on Windows x86 as of handoff)
```
src-tauri> cargo check                 # ✓ clean
src-tauri> cargo test --lib            # ✓ 96 passed, 0 failed, 1 ignored
(root)>    npm run typecheck           # ✓ type-clean (uses --stack-size=10000)
(root)>    npm run build               # ✓ production bundle, on-demand chunks
```
Emulator C++ verified per-file with MSVC `/std:c++20 /Zs` (EXIT 0) — **cannot full-build
on this box** (keystone/lzfse missing). Domain tests compiled+run via `cl` directly (53/53).

---

## 5. Open decisions / what to do next

### On the M1 (the real path)
1. **Confirm macOS ≥ 15 (Sequoia).** Hard gate for `vphone-cli` PV=3.
2. `cd Virtual-iPhone-Emulator/vphone-cli && make help` → capture required vars/paths.
3. Run `make fw_prepare → fw_patch_jb → cfw_install_jb → boot` with your IPSW + CloudOS.
4. If you run the **IDE on the M1**, ask the agent to build the
   **IDE → `vphone-cli` integration** (Tauri command running the make targets / `vphone-cli boot`,
   streaming to the iPhone panel — same pattern as the existing acheron streaming).

### On the acheron C++ path (only if pursuing the from-scratch hypervisor)
1. Install build deps (macOS: `brew install capstone keystone lzfse libimobiledevice libzip`).
2. Build, run `acheron run`, capture a **fresh** serial trace (current JIT/HVF layout).
3. Diagnose the entry spin with real data (the `vm/*.log` files are stale).
4. To show the Session-4 SpringBoard UI on x86/HVF, port the render into that backend's
   run loop AND add a guest-framebuffer GPA to its boot_args (top-of-RAM, like WHP) —
   **do not paint blindly**: JIT/HVF load the kernel at GPA 0x0, wrong FB GPA = clobbered kernel.

### IDE remainders (low priority, intentionally deferred)
- `system-access.ts` commands (`execute_command`/`delete_file`/`start_process`/`stop_process`)
  are intentionally **unregistered** (unsandboxed exec capability — do not wire without explicit ask).
- `import_editor_settings` return value is discarded by the frontend → low payoff; skipped.
- Dead modules: `src/services/git.ts`, `src/lsp.ts` (imported nowhere) — safe to delete.

---

## 6. Hard constraints (carry forward — from CLAUDE.md + user)
- **`Virtual-iPhone-Emulator/` is proprietary** — gitignored, **never pushed**.
- **Offensive security tools stay unrestricted** (`apex_red_team_scan`, `weaponize_env`,
  etc.) — analysis-only, classified `Safe`. Guarded by tests now. Do not add filters.
- **IDE memory budget 200–400 MB.** Patches must be surgical (no full-file rewrites).
- Windows-native paths, PowerShell for scripts. Co-author commits as instructed.
- User is a top bug-bounty hunter; the kernel-boot RE is their domain — don't mutate
  fragile boot/register internals blind.

---

## 7. One-paragraph resume prompt (paste to a fresh agent)
> Read HANDOFF.md, PROGRESS.md, ARCHITECTURE.md in the repo root. The IDE
> (VSCodium-Rust) is build-green; integration audited. The iPhone emulator has two
> engines: the C++ `acheron` (stuck at kernel entry; host-dependent backend — x86=JIT,
> M1=HVF, ARM-Win=WHP) and the Swift `vphone-cli` (Apple Virtualization.framework, the
> WORKING path, M1-only). We are moving to the M1 to drive `vphone-cli`. Next: confirm
> macOS≥15, run `make help` in vphone-cli, then wire the IDE iPhone panel to drive
> `vphone-cli boot`. Do not edit acheron boot internals from the stale `vm/*.log`.
