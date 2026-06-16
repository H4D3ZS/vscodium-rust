# C++ Virtual-iPhone-Emulator → rax (Rust) Port Map

Single source of truth for the rewrite. For every C++ subsystem: what it does, the
concrete data extracted, the rax status, and the **cleanup verdict** (what C++ can be
deleted/archived once the rax side is confirmed). Target: rax becomes the only engine;
the C++ tree is reduced to reference + the offline patch/discovery scripts.

Kernelcache: `iPhone17,3 / vphone600ap / iOS 26.1 23B85`, sha `7fbcacda36851b1b`,
linked base `0xFFFFFE0007004000`, guest RAM `0x80000000`, load offset `0x1000000`
(kernel at `0x81000000`). These match the rax harness exactly.

Legend — rax status: ✅ ported · 🟡 partial · ⬜ todo · 📄 reference-only (data already
baked into the kernelcache/DTB or offline-only).

---

## 1. Boot state seeds  → `rax/src/vphone_boot.rs::apply_styx_seeds` ✅
Source: `WindowsHypervisor.cpp` (ApplyStyx* fns), `StyxRuntimeGates.generated.hpp`,
`StyxFilesetMetadata.generated.hpp`.
- Two-object cpu_data: `rec @0x80A00000` (= CpuDataEntries[0].cpu_data, +8 in table),
  `cpu_data @0x80B00000` (TPIDR_EL1). rec[0x18]=thr_stk 0x80CA0000, [0x28]=int_stk
  0x80C10000, [0x30]/[0x68]=cpu_data, [0xB8]=platform-init fn 0x81C48834, [0x1C8]=cpuid 0;
  cpu_data[0]=self, [0x198]=rec.
- BSS seeds: `0x81784570`=1 (8B), `0x83837530`=1 (4B).
- FILESET metadata (page `0x81784000`): +0x248/+0x268/+0x290=0x81004000,
  +0x250/+0x270/+0x298=0x839F0000, +0x460=0x81000000, +0x3E0=0x83843C00.
- Dispatch stubs: timer (global 0x838437A0→object 0x83843800→vtable 0x83843900[+0x5B8]→
  method 0x83843A00=`mov w0,#0;ret`), fileset-entry (stub 0x83843C00[+0xC]=0xC, ptrs
  0x817843E0/0x83812E30), scheduler (root 0x837B7000, node 0x837B7100, method 0x83843E00,
  vtable 0x83843F00, root[+0x5F0]=vtable).
- 30 runtime-gate NOPs (`STYX_GATE_NOPS`).
- Event-stream globals: bit_idx `0x81711C98`=4, timer_freq `0x81712048`=24000000.
**Cleanup:** C++ seed code → reference-only; the *generated headers* + discovery scripts
stay (they regenerate seeds for new kernels). Port future regenerations into the manifest.

## 2. MMIO device map  → `rax/src/vphone_boot.rs` TracingMem 🟡 (addresses ✅, behaviors partial)
Source: `io/DeviceEmulator.hpp`. Apple-SoC windows (all above RAM, intercepted):

| Device | Base | Region | rax |
|--------|------|--------|-----|
| SEP Mailbox | `0x210000000` | 0x1000 | ⬜ |
| PMU | `0x231000000` | 0x4000 | ⬜ |
| Display controller | `0x232000000` | — | ⬜ |
| UART (PL011) | `0x235200000` | 0x1000 | ✅ (TX→stdout, FR ready) |
| SMC | `0x23B700000` | 0x100000 | ⬜ |
| Apple timer | `0x23E100000` | 0x1000 | 🟡 (free-running counter) |
| AIC | `0x28E100000` | 0x800000 | 🟡 (returns 0; needs INFO) |
| VirtIO net/blk/rng | `0x400/401/402000000` | 0x200 | ⬜ (likely not needed early) |
| Framebuffer/VRAM | `0x800000000` | fb size | ⬜ (M3) |

### Device register behaviors (extracted — to port into device_read/device_write)
- **AIC** (`io/AICController.hpp`): +0x04 INFO → `0x00000401` (nr_irqs=128, nr_cpus=1);
  +0x2000 EVENT → 0 (none); +0x4000 SW_SET, +0x4080 SW_CLR, +0x4100 MASK_SET,
  +0x4180 MASK_CLR, +0x5000 TARGET_CPU (writes accepted). **INFO=0x401 is the gap** —
  rax currently returns 0 there.
- **SMC** (`io/SMCController.hpp`): +0x00 TX FIFO, +0x10 TX_SEND (write 1 = dispatch),
  +0x20 RX FIFO, +0x24 RX_STATUS (bit0 = data avail), +0x30 status (0 = ready). Command
  keys: `'RVBF'`→0x0200, default→0. Response queued after SEND.
- **SEP Mailbox** (`io/SEPMailbox.hpp`): +0x00 SEND, +0x10 RECV, +0x20 STS (0=hasData,
  1=empty), +0x24 CTRL. write SEND 0x1/0x3/0x6 → RECV returns READY (0x1).
- **Apple timer** (`io/AppleARM64Timer.hpp`): +0x00 VALUE_LOW, +0x04 VALUE_HIGH
  (counter = elapsed), +0x08 CONFIG (bit0 enable), +0x0C STATUS (bit0 irq pending).
- **PMU** (`io/PMUController.hpp`): +0x10 POWER_GATE_STATUS → `0x1` (stable);
  +0x14 POWER_GATE_CONTROL (write).
**Cleanup:** once these are in `TracingMem::device_read/write`, the C++ `io/` device
headers → reference-only.

## 3. Interrupt delivery  → `cpu.rs` / harness ⬜ (Part B.2)
Apple timer fires as **FIQ**; AIC delivers device IRQs. rax must wake WFI + vector. Source
behavior is the C++ AIC/timer + the WHP interrupt-injection path. Needed for scheduler tick
and any "wait for IRQ" loop.

## 4. boot_args + memory map  → `vphone_trace.rs::build_boot_args` / `BootLayout` ✅ (map 🟡)
Source: `WindowsHypervisor.cpp` (boot_args build, identical offsets). physBase 0x80000000,
memSize, topOfKernelData, framebuffer 1290x2796 BGRA, deviceTreeP, cmdline. **Open:**
pmap_startup page-count math (Part B.4) — confirm memSize/DT memory node give a packable
vm_page count.

## 5. Kernelcache + DTB patches  → 📄 baked into the artifacts (no runtime port needed)
Source: `_scripts/windows_patcher/patch_kernelcache.py` (file patches 0x1FED0A4
`MOV W0,#1;RET`, 0x1FE9860 `RET`; +14 sandbox / 5 MACF stubs), `patchers/KernelPatcher.cpp`,
`KernelJBPatcher.cpp` (AMFI/CS/sandbox), `patchers/DeviceTreePatcher.cpp` (framebuffer,
backlight, UART, NVRAM nodes). These produce `vm/kernelcache.raw.bin` + `vm/DeviceTree.dtb`,
which rax loads already-patched. **Cleanup:** keep the *scripts* (offline pipeline); the C++
patcher classes → reference-only. Any future patch can also go in `boot_manifest.toml`.

## 6. Image4 / Mach-O / firmware pipeline  → 📄 / `vphone_trace.rs` parser ✅
`Im4p.cpp`, `formats/MachOParser.cpp`, `firmware/FirmwarePipeline.cpp`,
`patchers/IBoot*`, `AVPBooterPatcher`, `TXM*Patcher`. rax has its own Mach-O loader
(`vphone_trace.rs::parse_kernel_metadata` + `arch/arm.rs::load_macho`) and skips iBoot
(direct kernel boot). **Cleanup:** offline firmware-prep only → keep scripts, archive the
C++ iBoot/AVPBooter/TXM patchers (not used by rax direct boot).

## 7. CPU / decoder / hypervisor backends  → rax core ✅ (superior)
`cpu/ARM64SystemRegisters.cpp`, `cpu/ARM64InstructionDecoder.cpp`,
`hypervisors/jit/ARM64Decoder.cpp` + `X86Emitter`, `hypervisors/{win32,macos,linux}`.
rax's oracle-verified interpreter replaces ALL of these (the JIT's silent-NOP was the
original blocker). Apple IMPL-DEF sysregs handled via `set_lenient_sysregs` + specific
seeds. **Cleanup:** the entire C++ hypervisor/JIT/decoder layer → obsolete for rax; archive.

## 8. Higher-level subsystems (M2–M3, mostly todo)  → ⬜
- `sep/SEPEmulator.cpp` (589), `security/KeychainManager.cpp` (681),
  `biometric/FaceIDEmulator.cpp` (448) — SEP/crypto/biometric (M2+ userspace).
- `nvram/NVRAMBridge.cpp` (454) — NVRAM variables.
- `launchd/LaunchdSyscallStub.cpp` (160) — launchd syscall stubs (M2d).
- `springboard/SpringBoardSurface.cpp` (573), `boot/AppleBootAnimation.cpp`,
  `display/*` (Framebuffer, FramebufferPresenter, Win32Display, Metal/SoftwareTextureBridge)
  — UI/display (M3; rax will present via `wgpu`).
- `usb/*` (USBIP, HID, descriptor spoof), `devices/UserModeNAT`, `input/TouchInputBridge`,
  `identity/IdentityManager`, `registry/DeviceRegistry`, `mach/MachIPCStub`,
  `video/VideoToolboxEncoder` — peripheral/host-integration (port as milestones reach them).
**Cleanup:** keep as reference until the corresponding rax milestone ports each.

## 9. Offline tooling (KEEP — not part of the engine)  → 📄
`_scripts/windows_patcher/{patcher.py, styx_discover_runtime_gates.py,
fileset_metadata_seed.py, fairplay_iokit_stubs.py}` — the patch + seed-discovery pipeline.
These regenerate the manifest data for new kernel versions. **Keep**; wire their outputs into
`boot_manifest.toml` going forward.

---

## Cleanup plan (what to delete/archive after rax parity is confirmed per item)
1. Archive `hypervisors/` (jit/win32/macos/linux), `cpu/`, JIT decoder — rax replaces them.
2. Archive C++ device `io/` + `devices/` once §2 behaviors are in rax.
3. Archive iBoot/AVPBooter/TXM patchers + firmware pipeline (rax direct-boots).
4. Keep: `_scripts/windows_patcher/*` (offline), generated headers, `vm/*` artifacts, this map.
5. Higher-level subsystems (§8): archive lazily as each rax milestone ports them.

## rax-side layout (the rewrite home)
- `rax/src/vphone_boot.rs` — boot harness: loader, seeds, devices, manifest, diagnostics.
- `rax/src/arm/aarch64/` — CPU core (interpreter, MMU, sysregs, PAC, exceptions).
- `rax/vphone/boot_manifest.toml` — data-driven seeds/nops/patches/stubs.
- `rax/vphone/CPP_PORT_MAP.md` — this file.
- `rax/vphone/BOOT_MAP.md` — (todo) phase→blocker→fix spine map.
