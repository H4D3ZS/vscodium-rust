# Booting Linux to busybox in the rax x86-64 emulator

**Status:** ✅ **Achieved (2026-06-02).** rax boots a Linux kernel through to an
interactive **busybox `/ #` shell prompt** under the `emulator` backend.

```
[   16.27] Run /init as init process
INIT: execve /bin/sh

BusyBox v1.37.0 (Ubuntu 1:1.37.0-4ubuntu1) built-in shell (ash)
Enter 'help' for a list of built-in commands.

/bin/sh: can't access tty; job control turned off
/ #
```

This document records how it works, the root-cause analysis that got us here
(which **overturned a previous wrong diagnosis**), the fixes, how to reproduce,
and the known remaining issues.

---

## 1. TL;DR

Three independent problems blocked the boot. Two were genuine emulator bugs; one
was a kernel-build wall:

| # | Problem | Fix | Status |
|---|---------|-----|--------|
| 1 | **IRET did not invalidate lazy flags** → a timer interrupt landing inside a flag-dependent loop (`memcpy_orig`, `vsnprintf`) made the resumed code read the *handler's* CF/ZF → wrong copy lengths / digit counts → scattered guest-memory corruption. | `clear_lazy_flags()` in `apply_iret_flags` | **Committed `30badc3`** |
| 2 | **VMOVD/VMOVQ unimplemented** + **VMOVDQA aborted the VM** on an unaligned operand. busybox/glibc startup hit both. | Implement `VMOVD/VMOVQ` (VEX 6E/7E/D6); make VMOVDQA perform the access instead of aborting. | **Committed `3f87705`** |
| 3 | Kernel built with **FineIBT + retpoline/rethunk/ITS/SRSO/call-depth/IBT** self-patching that rax mis-emulates. | Rebuild the kernel with those mitigations **disabled at config**. | Build-side (see §6) |

Plus a timing model change (see §5): the guest TSC now tracks **real wall-clock
time** instead of instruction count, so the kernel boots on a normal clocksource
**without crutch cmdline parameters**.

---

## 2. The root cause — and the correction

A prior investigation had concluded the boot failure was a **latent
"aliasing/codegen UB"** in rax's Rust (because different rax builds crashed at
different points). **That was wrong.** The real situation:

### 2a. It was *nondeterminism*, not codegen-UB
The boot was nondeterministic because several guest-observable values came from
the **host wall clock**, not a deterministic source:
- `RDRAND` / `RDSEED` returned `SystemTime::now()`.
- `RDMSR(0x10)` (TSC MSR) returned `SystemTime::now()`.
- The PIT (`timing::elapsed_nanos`) and the `run()` VMM-yield were wall-clock.

So the **timer IRQ landed at a different guest instruction on every run**,
scattering a latent corruption onto a different victim each boot. The
"different builds crash differently" observation was *per-run* nondeterminism,
not LTO/UB. Making RNG/TSC/timer instruction-count-driven made the boot
reproducible — which is what allowed the bug to be bisected.

### 2b. The actual corruption: IRET lazy-flags staleness
rax evaluates condition codes lazily (from a pending `(op, src, dst, result)`
tuple) and materializes them only when needed. `apply_iret_flags` restored
`RFLAGS` straight into `regs.rflags` **but never invalidated the lazy-flag
state**. So when a timer interrupt landed in the middle of a flag-dependent
loop:

```
memcpy_orig:  ... sub rdx, 0x20 ; ... ; jae <loop>     ; CF/ZF drive the loop
vsnprintf:    ... dec <digits>  ; ... ; jnz <loop>     ; ZF drives digit count
```

…the handler ran (clobbering the lazy-flag state), and on `IRET` the resumed
code evaluated `jae`/`jnz` from the **handler's** last ALU op instead of the
restored flags → wrong copy length / wrong digit count → memory corruption that
surfaced later as a garbage pointer dereference anywhere in the kernel.

This is a **regression**: the lazy-flag rework that introduced it landed after
the last known-good busybox boot (commit `d9cc726`). Two external reviews
independently fingered "flag-sensitive corruption in the interrupt path", and
the `4016lu`-style mangled printk timestamps (a `%06lu` field rendering 4 digits
+ 2 bytes of the format string) were the digit-count corruption in the act.

### 2c. The diagnostic that nailed it
Disabling IRQ0 delivery (a temporary `RAX_NO_TIMER` gate) let the boot sail
**past** the corruption into late device init. That localized the bug to the
**timer-interrupt path**; the first oops was always in the timer handler
(`tick_periodic → queued_spin_lock_slowpath`) reading already-corrupted per-CPU
data → the corruption happened *earlier*, in a loop an interrupt had landed in.

---

## 3. The fixes (emulator)

### 3a. IRET lazy-flag invalidation — `src/.../insn/control/int.rs` (committed `30badc3`)
```rust
fn apply_iret_flags(vcpu, size, value) -> Result<()> {
    match size { 2 => {...}, 4|8 => {...}, _ => return Err(...) }
    // IRET restores RFLAGS wholesale; any pending lazy-flag op left by the
    // returning handler is now stale and MUST be invalidated.
    vcpu.clear_lazy_flags();
    Ok(())
}
```
(Any code path that writes `regs.rflags` wholesale — POPF, sysret — should follow
the same discipline; IRET was the one the timer exercised constantly.)

### 3b. VMOVD/VMOVQ + VMOVDQA — `src/.../insn/simd/avx.rs`, `dispatch/vex/mod.rs` (committed `3f87705`)
- Added `vmovd_load`/`vmovd_store` (VEX.66.0F 6E/7E, `VEX.W` = D vs Q),
  `vmovq_load` (VEX.F3.0F 7E), `vmovq_store` (VEX.66.0F D6); all clear the upper
  YMM lane.
- VMOVDQA previously did `return Err(Emulator("unaligned…"))` which **aborts the
  whole VM**. Real hardware raises `#GP(0)`; aborting is the worst option. It now
  performs the access (rax's `read_mem`/`write_mem` handle any alignment). A
  proper `#GP(0)` would be more compliant — noted as a refinement.

---

## 4. The kernel build (clearing the CFI/mitigation wall) — §6 detail

A stock FineIBT/retpoline kernel makes rax spin/BUG in `apply_retpolines` /
`apply_returns` / ITS thunk allocation — complex runtime self-patching that rax
mis-emulates. The workable kernel is built with these **disabled at config**:

```
MITIGATION_RETPOLINE, MITIGATION_RETHUNK, MITIGATION_ITS, MITIGATION_SRSO,
MITIGATION_GDS, MITIGATION_SPECTRE_V2, MITIGATION_SPECTRE_BHI,
MITIGATION_CALL_DEPTH_TRACKING, CALL_THUNKS, CALL_PADDING, X86_KERNEL_IBT  → all off
```
Built with `make CC=clang LD=ld.lld vmlinux`. (An earlier partial workaround —
advertising IBT in CPUID, commit `3d4424f` — let a FineIBT kernel resolve
`cfi_mode=CFI_FINEIBT`, but disabling the self-patching wholesale is cleaner.)

The ELF `vmlinux` boot path works; the **bzImage** (real-mode/setup entry
`0x1000200`) path does **not** — that is a separate, open bug.

---

## 5. Real-time timing (the clock model)

### The problem
`vcpu.tsc()` returned `insn_count * 3000`. At ~100 MIPS that advances the guest
clock **~100× faster than real time**, so the kernel's TSC clocksource diverged
wildly from the PIT and only worked if forced via cmdline crutches
(`clocksource=tsc nohz=off tsc=reliable`) — and any custom `--cmdline` that
dropped them hung the boot. The `DEFAULT_CMDLINE` (`src/config.rs:18`) was a
stack of exactly these crutches.

### The fix
`vcpu.tsc()` now returns `crate::timing::elapsed_nanos() * 3` — **host wall-clock
elapsed since start, scaled to the advertised 3 GHz** (3 cycles/ns). With the PIT
also wall-clock, both clocks track real time. The kernel then:
- **calibrates its own delay loop** (no `lpj=` needed),
- **selects the real `tsc` clocksource itself** (`Switched to clocksource tsc`),
- runs RCU/tickless normally (no `nohz=off` / `rcu_cpu_stall_suppress`).

`linux/config-clean.toml` demonstrates a boot to busybox with a **minimal
cmdline** carrying none of those crutches.

### Known caveat — clocksource skew
Because rax's instruction throughput is non-uniform, a wall-clock TSC read per
`RDTSC` skews ~15% against the jiffies/PIT over a watchdog interval. Strict
kernels mark `tsc-early` unstable and **fall back to `refined-jiffies`**
(non-fatal — boot continues). A future improvement is to derive the timer tick
and the TSC from the **same** sampled clock so they stay consistent.

---

## 6. How to reproduce

### Kernel (one-time build)
Vendored source at `linux/kernel/linux` (gitignored `vmlinux`). Disable the
mitigations listed in §4, then:
```bash
cd linux/kernel/linux
for o in MITIGATION_ITS MITIGATION_RETHUNK MITIGATION_RETPOLINE \
         MITIGATION_CALL_DEPTH_TRACKING MITIGATION_SRSO MITIGATION_GDS \
         MITIGATION_SPECTRE_V2 MITIGATION_SPECTRE_BHI X86_KERNEL_IBT \
         CALL_THUNKS CALL_PADDING; do ./scripts/config --disable "$o"; done
make CC=clang LD=ld.lld olddefconfig
make -j"$(nproc)" CC=clang LD=ld.lld vmlinux
```

### initrd
`linux/initrd.cpio` (uncompressed; skips the gzip-inflate, much faster) contains
a static busybox + a tiny ELF `/init` that `execve("/bin/sh")`.

### Boot
```bash
cargo build --release --bin rax
./target/release/rax --config linux/config-clean.toml   # minimal cmdline → busybox /#
```
`linux/config-clean.toml` cmdline (no crutches):
```
console=ttyS0 earlycon=uart,io,0x3f8 root=/dev/ram0 noapic nolapic nosmp mitigations=off
```
Reaches the busybox prompt in ~15–20 s of guest time (≈ wall time, real-time
paced). A known-good binary is preserved at `/tmp/busybox_proof/rax-busybox`.

---

## 7. State of the changes

**Committed + pushed (`origin/master`):**
- `30badc3 fix(x86_64): clear lazy flags on IRET to prevent state corruption`
- `3f87705 feat(x86_64): implement VMOVD/VMOVQ and relax VMOVDQA alignment`

**Working-tree only (not committed):**
- `vcpu.tsc()` → real-time (`elapsed_nanos()*3`). Held back because `cpu.rs`
  currently also carries a concurrent agent's whole-file linter rustfmt; commit
  the one-liner cleanly once that settles.
- `linux/config-clean.toml`, `linux/config-test.toml` — boot configs (untracked,
  like the gitignored `vmlinux`).

All session debug instrumentation (corruption-trace probe, `RAX_NO_TIMER` /
`RAX_NO_STRFAST` env gates, the instruction-quantum timing experiment) was
**reverted** — they were diagnostic scaffolding, not part of the fix.

---

## 8. Open items / next steps
- **bzImage boot** hangs at the real-mode/setup entry — separate from the ELF path.
- **Clocksource skew** (§5) — make the timer tick and TSC share one sampled clock
  so strict kernels keep the `tsc` clocksource instead of falling back.
- **VMOVDQA** should raise `#GP(0)` on a genuinely-unaligned operand rather than
  silently performing the access (compliance).
- **SMP / APIC**: boots are single-CPU PIC-mode (`nosmp noapic nolapic`); full
  LAPIC/IOAPIC/SMP bring-up is future work.
- Audit other wholesale `regs.rflags` writers (POPF, sysret) for the same
  lazy-flag-invalidation discipline as the IRET fix.
