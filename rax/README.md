<h1 align="center">
    <img width="128" height="auto" src="./assets/ida.png" /><br>
    <strong>rax</strong><br>
</p>

<h5 align="center">
rax is a CPU emulator that does not trust itself. It implements four instruction sets in software<br/>
(x86-64, AArch64, Hexagon, and RISC-V) and checks every one of them, instruction by instruction,<br/>
against a reference that cannot be argued with: real silicon (KVM) for x86-64, and QEMU for the rest.<br/>
<br/>
The x86-64 core is a complete machine. It boots a real Linux kernel two ways: through hardware<br/>
virtualization (KVM on Linux, Hypervisor.framework on macOS) at near-native speed, or through a<br/>
from-scratch interpreter that traces, single-steps, snapshots, and profiles every instruction it<br/>
runs, and now boots all the way to a BusyBox shell. It covers the ISA out to AVX-512, AVX10.2, and<br/>
Intel APX, and behind it sits SMIR, a shared multi-architecture IR whose hot-block JIT lifts hot<br/>
x86-64 loops to native code at ~80× the interpreter.<br/>
<br/>
Three more CPUs run alongside: an AArch64 that also boots Linux (near-native on Apple Silicon via<br/>
HVF, or fully emulated at EL0/EL1) with complete SVE, a fully oracle-verified Hexagon (every opcode,<br/>
scalar + HVX), and a correctly-rounded RV64GC. Hexagon and RISC-V boot bare-metal programs too.
</h5>

<div align="center"><code>Rust</code> • <code>x86-64 · AArch64+SVE · Hexagon+HVX · RV64GC</code> • <code>boots Linux</code> • <code>hot-block JIT</code> • <code>122k+ tests</code></div>

---

## The thirty-second version

Build it, then run a Linux kernel, at silicon speed, or one instruction at a time:

```bash
cargo build --release

# 1. Boot a real Linux kernel on hardware virtualization (Linux + KVM).
./target/release/rax --kernel bzImage --initrd initrd.img

# 2. Boot it on the software CPU instead: slower, but every instruction is yours, and it
#    now runs all the way to a BusyBox shell. (Use an ELF vmlinux; bzImage real-mode boot is WIP.)
./target/release/rax --backend emulator --kernel vmlinux --initrd initrd.cpio

# 3. ...and trace every instruction the kernel executes, SDE-compatible.
./target/release/rax --backend emulator --kernel vmlinux --trace boot.trace

# 4. Boot AArch64 Linux: the architecture is sniffed from the kernel image, and a DTB with
#    GICv3, PL011, generic timer, and PSCI is generated on the fly. Works on any host.
./target/release/rax --kernel linux-aarch64/Image --initrd initramfs.cpio

# 5. Boot something that isn't Linux: a bootable ISO, through the real-mode mini-BIOS. rax takes
#    TempleOS V5.03 from real mode to its 64-bit HolyC shell (El-Torito boot + ATAPI CD-ROM).
./target/release/rax --backend emulator --kernel TempleOS.ISO --memory 512M
```

RISC-V and Hexagon are bootable emulator backends too (bare-metal programs, UART + halt):

```bash
./target/release/rax --arch riscv64 --backend emulator --kernel program.elf
```

And all four cores are checked against a reference oracle. Each harness runs an instruction on both rax
and the reference from an identical state, then diffs the result:

```bash
cargo test --release --test differential       # x86-64  vs. KVM (the silicon)
cargo test --release --test arm_diff           # AArch64 vs. qemu-aarch64
cargo test --release --test hexagon_hvx_diff   # Hexagon vs. qemu-hexagon
cargo test --release --test riscv_diff         # RV64GC  vs. qemu-riscv64
```

> **Good to know** Every oracle harness self-skips cleanly if the cross-compiler / QEMU / `/dev/kvm`
> isn't present, so the suite is green on any host. They only fail when rax and the reference genuinely
> disagree.

---

## Why it exists

If you have ever wondered what actually happens between launching a kernel and seeing a shell, most tools
give you one of two unsatisfying answers. A real hypervisor (QEMU/KVM) runs the kernel so fast you cannot
watch it; the CPU is a black box. A pure emulator (Bochs, Unicorn) lets you watch, but its instruction
coverage trails the hardware by years, and you have no easy way to know whether what it *did* matches
what a real chip would have done.

rax is built around the second problem. A software CPU is only as good as your confidence that it is
*right*, and the only honest way to earn that confidence is to compare it, instruction by instruction,
against something authoritative. So that comparison is the project's spine, not an afterthought:

- **x86-64** is checked against **KVM**, the actual silicon in your machine. The same machine code runs
  on the interpreter and on hardware from an identical architectural state, and the final state is
  diffed. When you want to know what an instruction *should* do, you ask the chip.
- **AArch64, Hexagon, and RISC-V** are each checked against **QEMU** in user mode, the same way: a tiny
  reference harness loads a state, runs one instruction, and reports back; rax runs it from the identical
  state; any divergence is a bug, reported precisely.
- **Intel APX** is the exception that has no chip to ask: no shipping CPU implements it and QEMU doesn't
  emulate it either, so KVM can't be the oracle. Its encodings instead come from **LLVM**, the only
  assembler that speaks APX: each test pins an instruction to LLVM's exact bytes and asserts rax's
  architectural effect against the documented semantics.

That methodology is what lets rax be both *legible* (you can open `insn/arith/add.rs` and read exactly
what `ADD` does to the flags) and *trusted*: it tracks four instruction sets out to their modern vector
extensions, and well over a hundred thousand cases stand between a change and a regression. It is also
what lets the JIT exist at all: a native code generator is only safe if you can prove its output matches
the interpreter, and the same oracle does that proof.

---

## What a run looks like

A throughput benchmark of the x86-64 interpreter's hot path (`examples/bench_loop.rs`, a tight
register-only guest loop) reports sustained MIPS, the apples-to-apples metric for interpreter work:

```text
$ RUSTFLAGS="-C target-cpu=native" cargo run --release --example bench_loop
[bench] iterations    : 268435456 (0x10000000)
[bench] expected insns: 1342177283
[bench] executed insns: 1342177283
[bench] elapsed       : 9.26 s
[bench] throughput    : ~145 MIPS
[bench] final eax=0x20000000 ecx=0x0
```

The native JIT is on by default: that same loop is detected as hot, lifted to SMIR, lowered to native
x86-64, and run directly: **~11,000 MIPS, roughly 80×**, bit-identical to the interpreter (the JIT-tier
vcpu test asserts register-for-register equality, and `RAX_JIT_VERIFY=1` re-checks every region against
the interpreter at runtime).

Boot a kernel under the emulator with `--trace`, and every retired instruction lands in an
SDE-compatible trace file: instruction, register changes, and (when they happen) memory reads/writes
and XMM updates, which you can diff against Intel's own Software Development Emulator:

```text
$ ./target/release/rax --backend emulator --kernel vmlinux --trace boot.trace
...
$ head -4 boot.trace
INS 0x00000000010000f0   xor eax, eax                                       | eax=0x00000000
INS 0x00000000010000f2   mov ecx, 0x80000000                                | ecx=0x80000000
INS 0x00000000010000f7   mov cr0, eax                                       | cr0=0x80000011
Write *(UINT64*)0x9000 = 0x000000000000a003
```

> **Good to know** The trace, GDB stub, snapshot facility, and per-mnemonic profiler are all wired into
> the *interpreter's* step loop, so they observe the genuine instruction stream, not a re-derived
> approximation. The KVM backend traps only on I/O, so it is fast but opaque by design.

---

## Four CPUs

x86-64 is the complete VM target: it boots Linux, with the full device platform, boot protocol, tracing,
GDB, snapshots, and the JIT. AArch64 boots Linux too, near-native on Apple Silicon via HVF or fully
emulated at EL0/EL1 (details under "How the machines work"). Hexagon and RISC-V are bootable emulator
backends for bare-metal programs. All four also have SMIR lifters.

| Core | Size | Runnable? | Coverage | Oracle |
|------|-----:|-----------|----------|--------|
| **x86-64** | ~54k LOC | **boots Linux** (KVM/HVF/emulator) + JIT | Legacy → SSE/AVX/AVX2 → AVX-512 → AVX10.1/10.2 → APX; x87; AES/SHA/GFNI; XSAVE | KVM (real hardware) |
| **Hexagon** | ~38k LOC | bare-metal (`--arch hexagon`) | V73 scalar + VLIW packets + HVX, **every opcode verified** | qemu-hexagon |
| **RISC-V** | ~11k LOC | bare-metal (`--arch riscv64`) | full RVA23 *scalar* set (RV64GC + Zfh/Zicond/Zfa/Zbk\*/Zcb + scalar crypto + vector-config) | qemu-riscv64 |
| **AArch64 / ARM** | ~64k LOC | **boots Linux** (emulator + HVF on Apple Silicon) | A64 base, **complete SVE + SVE2 + SVE2.1**, NEON/VFP, FP16; AArch32/Thumb; Cortex-M (M0-M85) | qemu-aarch64 + ASL |

### x86-64: the complete machine

The primary target and the one that boots Linux. A full decoder handles the entire encoding zoo (REX and
REX2, every legacy prefix including the `0x67` address-size override, ModR/M + SIB, VEX2/VEX3, EVEX
including APX Map 4, RIP-relative), feeding 88 instruction-implementation files.

| Category | Coverage |
|----------|----------|
| **Integer / logic / bit** | full ALU, ADCX/ADOX, BT/BTS/BTR/BTC, BSF/BSR, POPCNT/LZCNT/TZCNT; `#DE` on ÷0/overflow |
| **Shifts / strings / BCD** | SHL…RCR, SHLD/SHRD; REP MOVS/STOS/SCAS/CMPS (bulk fast path); DAA…AAD |
| **x87 FPU** | escape codes D8-DF via f64 |
| **SSE → SSE4 / AVX / AVX2** | moves, arithmetic, all compare predicates, shuffle/permute/convert (XMM/YMM) |
| **FMA / BMI1 / BMI2** | VFMADD/SUB/NMADD/NSUB {132,213,231}; ANDN, BZHI, PEXT, PDEP, MULX, … |
| **AVX-512** | F / VL / BW / DQ / CD; masked ops, opmask k0-k7 |
| **AVX10.1 / 10.2** | VNNI, IFMA, VPOPCNTDQ, VBMI, BF16; VMPSADBW, VMINMAX, saturating converts |
| **APX** | REX2, EGPRs R16-R31, NDD (3-operand), NF (no-flags), CCMP/CTEST, SETZUcc, PUSH2, JMPABS, MOVBE, MUL/DIV, EVEX Map 4 (LLVM-verified, not KVM) |
| **Crypto / state / system** | AES, SHA1/256, GFNI (FIPS/SDM known-answer tested); XSAVE/XRSTOR/XCR0; CPUID, MSRs, CR/DR, descriptor-table loads, CPL-checked, faults injected (`#UD`/`#GP`) |

### Hexagon: complete, every opcode

A Qualcomm Hexagon (V73) implementation where **all 2,178 instruction opcodes are decoded, executed, and
differentially verified against qemu-hexagon at zero divergence**: scalar core, the full HVX vector set,
even the awkward corners (CABAC bin decode, scatter/gather, FP reciprocal/sqrt seed tables recovered from
the oracle). It takes the hard parts seriously:

- **VLIW packets** give true parallel-packet semantics: all instructions read the old register file and
  commit atomically at packet end; `.new` value forwarding for scalars *and* HVX vectors; duplex
  encodings; hardware loops (SA0/LC0, SA1/LC1) with circular and bit-reversed addressing; dual stores.
- **HVX** provides 1024-bit vectors V0-V31 and predicates Q0-Q3: ALU, compare, min/max, the full multiply
  family, permute, shift, round/saturate, LUT, histogram, vector-predicate ops, `vmem` loads/stores
  (`.cur`/`.tmp`, scalar-predicated), and V65 scatter/gather.

### RISC-V: small, correct, and bootable

A complete **RV64GC** core (~11k lines) wired into the VMM as a real `--arch riscv64 --backend emulator`
target: it loads an ELF, drives a 16550 UART over MMIO, and halts on `ecall`. Coverage is the **entire
RVA23 scalar set**: RV64I/M/A/F/D, **Zfh** half-precision, C (compressed), Zicsr/Zifencei,
Zba/Zbb/Zbc/Zbs, Zicond, Zfa, Zbkb/Zbkx/Zcb, the full scalar **crypto** suite (Zknh SHA-256/512, Zksh
SM3, Zksed SM4, Zkne/Zknd AES), and the vector *configuration* instructions (vsetvl\*, VLEN=128). The
floating-point core computes the round-to-nearest result, recovers the *exact* residual (2Sum / FMA /
Newton), and uses it to deliver correctly-rounded answers in all five rounding modes with all five IEEE
flags. It's checked against qemu-riscv64 by fuzzers that drive ~300k comparisons across the whole
non-control-flow opcode space.

### AArch64 / ARM: the deepest core

The largest and most thoroughly tested ISA, and now a runnable backend that boots Linux:

- **SVE** is **complete**: no valid SVE encoding is unhandled. The full data-processing set (predicate
  generation/logical, predicated integer & FP ALU, reductions, permutes, CPY/SEL/CMP, shifts), the
  entire memory subsystem (contiguous, gather/scatter, LD2-4/ST2-4, first-fault + FFR), all at VL=128.
- **SVE2 and SVE2.1** are complete on the register surface, every encoding bit-exact against the
  qemu-aarch64 oracle: long/wide/narrow and saturating integer ops, complex arithmetic, bit-permute,
  pairwise, BF16 (B16B16, dot product), quadword reductions, and PMOV/PSEL/PEXT. A committed
  879-instruction `llvm-mc` sweep (`tests/sve2_gen.rs`) guards it; only multi-vector memory, SME, and
  FEAT_LUT remain (the register-only oracle can't reach them).
- **NEON / VFP**: full Advanced SIMD and scalar FP including FP16, bit-exact against the oracle on a
  3939-instruction sweep (`tests/neon_gen.rs`); full crypto (AES, SHA1/256/512, SHA3, SM3, SM4).
- **Modern A64 extensions**: MTE (memory tagging), PAuth (pointer authentication), FlagM, LRCPC (RCpc
  release-consistency atomics), and FP8 (FP8FMA).
- **AArch32 / Thumb / Cortex-M (M0-M85)**: the A32 + Thumb (T16/T32) integer ISA is bit-exact against a
  new qemu-arm oracle (`tests/arm_diff32.rs`, a 1666-encoding sweep), now with VFP and NEON (Advanced
  SIMD) execution and hardware exception routing; Cortex-M adds NVIC/SysTick/SCB/MPU, ARMv6-M to v8.1-M,
  and there is an ARMv6 core (CP15 + MMU) with two emulated SoC machines: the S3C64xx, and the S5L8900
  (the original iPhone / iPod Touch 1G), which boots Apple's **iBoot** from real device firmware
  (bootrom + LLB + NOR, with the VIC/SYSIC/timer/I2C/PMU/UART platform set; `RAX_MACHINE=s5l8900`).

---

## Correctness: every architecture has an oracle

This is the part that matters. rax's claim is not "it implements a lot of instructions"; it is "the
instructions are *checked against an authority*." Each harness builds an initial architectural state,
runs one instruction (or a short sequence) on both rax and the reference from that identical state, then
diffs the full register file (and, for x86, a scratch memory page). Inputs are enumerated over encoding
fields and driven with many pseudo-random states, so a single `#[test]` function covers a large family.

| Harness | rax core | Oracle | `#[test]` fns | Compares |
|---------|----------|--------|-------------:|----------|
| `tests/differential.rs` | x86-64 | **KVM** (hardware) | 463 | GPRs, RIP, RFLAGS, XMM, memory |
| `tests/arm_diff.rs` | AArch64: NEON + SVE/SVE2/SVE2.1 | `qemu-aarch64` | 198 | X0-X30, SP, NZCV, V0-V31, P0-P15 |
| `tests/arm_diff32.rs` | AArch32: A32 + Thumb T16/T32 | `qemu-arm` | 6 | R0-R14, CPSR, FPSCR, D0-D31, scratch |
| `tests/hexagon_*_diff.rs` | Hexagon (scalar / cf / float / mem / HVX / HVX-mem) | `qemu-hexagon` | 134 | GPRs, P3:0, USR, loop regs, V0-V31, Q0-Q3 |
| `tests/riscv_diff.rs` | RV64GC | `qemu-riscv64` | 29 | x1-x31, f0-f31, fcsr, scratch |
| `tests/diff_fuzz.rs` | SMIR (lift → interp / native) | KVM | 35 | guest state after lift+run |
| `tests/riscv_smir_lift.rs` | RISC-V → SMIR lift | rax RISC-V interp | 9 | x/f/v/fcsr (~180k insns incl. RVV, 0 divergence) |
| `tests/hexagon_smir_lift.rs` | Hexagon → SMIR lift | rax Hexagon interp | 305 | R/P/USR/V/Q (entire ISA: scalar + 100% HVX) |
| `tests/smir_jit_vcpu.rs` | SMIR JIT in the real vcpu | interpreter | 7 | register-for-register + throughput |

> **Good to know** Those `#[test]` counts understate the work by orders of magnitude. Each function
> enumerates many encodings and many random input states internally (the ARM and RISC-V fuzzers drive
> hundreds of thousands of comparisons per run). The reference harnesses are tiny C/asm programs
> (`tools/{arm,riscv,hexagon}-diff/`) that QEMU runs as ground truth; for x86 the ground truth is KVM.

### Tests by the numbers

On top of the oracles, there are exhaustive unit suites:

| Suite | Count | How |
|-------|------:|-----|
| **ARM (ASL-generated)** | 92,131 | generated from ARM's official machine-readable **ASL** spec via `tools/asl-parser/` |
| **x86-64 instruction suite** | 28,554 | `tests/x86_64/` (850 files), behind `--features x86_64-suite` |
| **Everything else** | ~1,600 | oracle + SMIR-lift harnesses, real-mode/ISO boot, Hexagon bare-metal, RISC-V boot, crypto known-answer (FIPS/SDM) |
| **Total** | **122,373** | `#[test]` functions across `tests/` |

The ARM tests are not written by hand: the `asl-parser` downloads and parses ARM's ASL release and emits
exhaustive instruction tests from it, which is how 92,000+ ARM cases exist at all.

---

## SMIR and the hot-block JIT

**SMIR** (Sigma Machine IR, `src/smir/`, ~140k LOC; spec in
[`docs/specifications/smir/`](docs/specifications/smir/)) is the layer that makes "four CPUs" one
project. Each guest architecture has a *lifter* that translates its instructions into a common set of
200+ typed operations; the IR is interpreted directly, optimized, and lowered to native machine code on
both x86-64 and ARM64 hosts.

```text
  ┌─────────────────────────────────────────────────┐
  │   x86-64 · AArch64 · Hexagon · RISC-V · AVX10   │   per-arch lifters
  └────────────────────────┬────────────────────────┘
                           ▼
  ┌─────────────────────────────────────────────────┐
  │                   SMIR Module                   │   SmirFunction → SmirBlock → SmirOp
  └────────────────────────┬────────────────────────┘
         ┌─────────────────┼─────────────────┐
         ▼                 ▼                 ▼
     Interpreter       optimizer        JIT → x86-64 / ARM64
     (lazy flags)      (O0/O1/O2)       (emit · regalloc · W^X)
```

The native JIT is real, integrated, and **on by default**. The `X86_64Vcpu` run loop auto-detects hot
loops (a back-edge counter promotes a region at a threshold of 64), lifts the region to SMIR, runs the
O2 optimizer over it, lowers it to native x86-64, caches the compiled block, and runs it through a W^X
`mmap` trampoline. On the bench loop the lowered body is one native instruction per guest instruction:
**~80× the interpreter (~11,000 MIPS)**, bit-identical to interpreting it, and `RAX_JIT_VERIFY=1`
proves that live by re-running every region on the interpreter and diffing.

What makes that safe to ship is a **fail-safe gate**: a region compiles only from operations proven
equal to KVM, and anything else makes it **bail back to the interpreter**, so native code never runs
unless it is known correct. The gate covers the integer core (ALU, shifts, multiply, mov/extend, LEA,
BSF/BSR, setcc/cmov, branches) and memory, FS/GS segment-relative accesses included: loads and stores
lower to MMU helper calls that bail cleanly on a page fault or a write to a code page. Still
interpreter-only: RSP/RBP-relative frames, locked/RMW and FP/SIMD ops, and the double-width DIV the IR
can't yet model. Self-modifying code evicts compiled blocks via the MMU's dirty-page journal,
persistently-ineligible region heads are memoized so SMC-heavy guests like TempleOS don't thrash the
compiler, and a frontier-less spin loop is refused so native code can't trap the vcpu.

The lowerer is retargetable. Alongside the x86-64 host backend sits a full **AArch64 host backend**
(`lower/aarch64.rs`, ~51k LOC, 800+ lowering tests) that emits native ARM64 for the entire SMIR op set,
x86-64 guest semantics included (APX, atomics, REP MOVS, vector/FP), plus an AArch64-guest-to-x86
lowerer: the groundwork for JIT-compiling a guest across host ISAs (x86 on ARM, ARM on x86). The live
exec runtime is x86-host today.

| Piece | Where | State |
|-------|-------|-------|
| **Lifters** | `lift/` | x86-64, AArch64, RISC-V, AVX10, and a lift-complete Hexagon (every opcode, scalar + 100% HVX, verified) |
| **Interpreter** | `interp.rs` | direct execution; lazy flags, block caching |
| **Optimizer** | `opt.rs` | frontier-aware liveness; dead-flag and dead-code elimination, copy propagation, constant + branch folding (O2) |
| **JIT lowering** | `lower/x86_64.rs`, `lower/aarch64.rs`, `lower/regalloc.rs`, `lower/runtime.rs` | x86-64 emitter **and** a ~51k-LOC AArch64 host emitter (SMIR to native ARM64, x86 guest semantics included), plus an ARM-guest-to-x86 lowerer; 1:1 register map, W^X exec runtime + trampoline |
| **JIT integration** | `backend/.../x86_64/cpu.rs` (on by default) | hot-loop detection, region cache, memory via MMU helpers, safety gate, SMC eviction |

The lifters are verified too, not just the JIT's native output: `tests/riscv_smir_lift.rs` and
`tests/hexagon_smir_lift.rs` lift each instruction to SMIR, interpret it, and diff against that
architecture's own qemu-verified interpreter. RISC-V lifts its entire user-mode RV64GCV (scalar, the
full RVV vector ISA, FP, the Zb*/Zk* bit-manip and crypto extensions, and CSRs) at zero divergence over
~180k instructions; Hexagon lifts its entire ISA across 305 verified families.

> **Good to know** The JIT accelerates without changing behaviour: a kernel boots identically with it on
> or off, because every region it can't prove correct degrades gracefully to the interpreter, and
> `RAX_JIT_VERIFY=1` audits that equivalence live. What remains is the double-width DIV model and native
> block-to-block chaining.

---

## How the machines work

### Booting x86-64

The fast path loads a Linux kernel (ELF or bzImage) straight into 64-bit mode: kernel at physical
`0x1000000` (16 MiB), initrd at `0x4000000`, initial page tables (identity-mapped first 8 GiB via 1 GiB
huge pages, kernel space at `0xFFFFFFFF80000000`, direct map at `0xFFFF888000000000`), a minimal 64-bit
GDT, then `CR0.PG=1` / `CR4.PAE=1` / `EFER.LME=1` and a jump to the entry point.

rax also boots the *old* way, from a bootable CD. A real-mode mini-BIOS (INT 10h/13h/15h/16h/1Ah, an
El-Torito catalog parser, and an ATAPI CD-ROM model) drops a boot image at `0x7C00` in 16-bit real mode
and lets the guest walk itself up through protected mode into long mode. That path boots **TempleOS
V5.03** from its ISO: real to protected to long to 64-bit kernel init, mounting its RedSea CD as drive
`T:` and running its own HolyC JIT compiler.

### The AArch64 machine

An AArch64 guest needs no flags: rax sniffs the architecture from the kernel image, generates a DTB
(RAM, GICv3, PL011, armv8 generic timer, PSCI) on the fly, and boots it. On the software emulator,
that means full EL0/EL1 system emulation: the stage-1 MMU, exception delivery, a GICv3 distributor +
redistributor + ICC system registers, the generic timer, and a PL011 console. On Apple Silicon the same
guest runs near-native through Hypervisor.framework with the in-kernel GICv3. Both paths share the
generated DTB, PSCI, and PL011 wiring.

### The x86-64 interpreter loop

Fetch / decode / execute, with two twists that make it both fast and honest:

```text
loop {
    entry = decode_cache[rip & 0xFFF];          // 4096-entry, RIP-indexed
    insn  = if entry.matches(rip, mode) {
        entry.bytes                              // HIT: skip the memory fetch entirely
    } else {
        decode(fetch(rip))                       // MISS: prefixes, ModR/M, SIB, VEX/EVEX, immediates
    };
    execute(insn);                               // update regs / memory / lazy flags
    if (++insn_count & 1023) == 0 { poll_lapic_and_yield(); }
}
```

| Mechanism | What it does |
|-----------|--------------|
| **Decode cache** | 4096 entries indexed by RIP, keyed on a mode tag (`CR3 \| CS.L \| CS.D`). A hit reuses the cached bytes and **skips the guest-memory fetch entirely**. Kept coherent by SMC detection on guest writes. |
| **Lazy flags** | Arithmetic records its operands and defers RFLAGS materialization until a consumer (a `Jcc`, a `PUSHF`) reads them. Most computed flags are never needed. |
| **Fast paths** | A direct host-pointer path for physical RAM, a fast path for common ModR/M memory operands, and page-at-a-time `REP MOVS`/`STOS`. |
| **Hot-block JIT** | On by default: hot loops (and their memory ops) promote to native code (see above); `RAX_NO_JIT=1` disables it. |
| **TLB** | 256-entry direct-mapped cache over the 4-level page walk (4 KiB / 2 MiB / 1 GiB pages). |

The interpreter sustains around **145 MIPS** on the register loop; the JIT tier lifts that to ~11,000 on
hot regions.

---

## Devices

The machine boots a full legacy PC platform, always on: a 16550 serial console (interrupt-driven, so
guest console input works), 8254 PIT, 8259 PIC, LAPIC + IOAPIC, RTC/CMOS, the 8237 DMA controller,
i8042 PS/2 keyboard, primary and secondary IDE, a floppy controller, system-control ports, QEMU
`fw_cfg`, and the Bochs-style debug port.

On top of that sits a functional **PCI host bridge** with BAR-mapped routing and an MMU MMIO aperture.
Pass `--pci-devices` and rax attaches real controllers that Linux enumerates and drives:

| Class | Device | Status |
|-------|--------|--------|
| **Network** | Intel e1000 (82540EM) | `eth0` comes up; Microwire EEPROM bit-banged via EECD |
| **Storage** | AHCI, NVMe, IDE | enumerate and bind; AHCI reports SATA link state correctly |
| **Audio / USB** | AC97, UHCI | enumerate as PCI endpoints |

> **Good to know** `--pci-devices` is opt-in, so the default boot stays byte-identical and the
> interpreter hot path is untouched (the MMIO aperture collapses to a no-op when no bridge is set). VGA
> is the one model still on the shelf: its legacy `0xA0000` window shadows guest RAM in the flat-memory
> model, and it is display-only, so it is pointless on a serial VM.

---

## Observability & tooling

Because the interpreter owns the step loop, the introspection tools see the real instruction stream:

| Tool | Flag / feature | What you get |
|------|----------------|--------------|
| **Interactive console** | on by default on a TTY | raw-mode serial console with a qemu-style `Ctrl-A` mux (`Ctrl-A s` checkpoints, `x` quits, `h` helps); termios restored even on panic or signal |
| **Machine checkpoints** | `Ctrl-A s` / `SIGUSR1` / `--snapshot-interval N`; resume with `--checkpoint <f.rxc>` | a self-contained `.rxc` image: embedded config + CPU + zstd RAM + every device's serialized state + a timing anchor. `rax --checkpoint m.rxc` brings a live machine back with no `--kernel` or `--config` |
| **Instruction trace** | `--trace <file>` (`--features trace`) | SDE-compatible per-instruction trace, diffable against Intel SDE |
| **GDB stub** | `--gdb <port> --wait-gdb` (`--features debug`) | Remote Serial Protocol server: registers, memory, stepping |
| **Profiler** | `--profile` (`--features profiling`) | per-mnemonic execution counts and a hot-instruction report, optional JSON export |

---

## Usage

```
--kernel <path>            Kernel image: ELF, bzImage, or bootable ISO (required)
--initrd <path>            Initial ramdisk
--arch <x86_64|riscv64|hexagon|…>   Target architecture (default x86_64)
--backend <kvm|emulator>   Virtualization backend (hvf on macOS)
--memory <size>            Guest memory, e.g. "512M", "2G"
--cmdline <string>         Kernel command line
--config <file>            Load a TOML config
--trace <file>             Write an SDE-compatible instruction trace   (--features trace)
--gdb <port> [--wait-gdb]  Start a GDB stub, optionally wait for attach (--features debug)
--pci-devices              Attach the optional PCI devices (e1000, AHCI, NVMe, UHCI, AC97)
--snapshot-interval <N>    Checkpoint every N instructions (0 = off)
--snapshot-out <file>      Where Ctrl-A s / SIGUSR1 checkpoints land (default checkpoint.rxc)
--checkpoint <file.rxc>    Resume a whole machine from a checkpoint (no --kernel/--config needed)
--profile [--profile-output <json>]   Instruction profiling             (--features profiling)
```

```toml
# config.toml
backend = "emulator"
memory  = "512M"
kernel  = "/path/to/vmlinux"
initrd  = "/path/to/initrd.cpio"
cmdline = "console=ttyS0 earlyprintk=serial"
```

---

## Building

```bash
# Default (Linux): KVM backend enabled.
cargo build --release

# Cross-platform: software emulator only, no KVM.
cargo build --release --no-default-features

# The native JIT ships on by default (disable at runtime with RAX_NO_JIT=1).
# Build without it: cargo build --release --no-default-features --features kvm

# Fastest local interpreter (uses your host's full ISA).
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Apple Silicon (AArch64 guests near-native on Hypervisor.framework, macOS 15+):
# the binary must carry the hypervisor entitlement after every build.
cargo build --release --features hvf
codesign -s - -f --entitlements rax.entitlements target/release/rax
./target/release/rax --backend hvf --kernel linux-aarch64/Image --initrd initramfs.cpio
```

> **Good to know** `.cargo/config.toml` ships `target-cpu=x86-64-v3` as a portable default. It still
> lets LLVM emit AVX2/BMI2/FMA and autovectorize the scalar SIMD/flag loops while staying runnable on any
> 2013-or-later x86-64 host. The release profile is fat-LTO, one codegen unit, `panic=abort`, stripped.

| Feature | Default | Enables |
|---------|---------|---------|
| `kvm` | ✓ (Linux) | KVM backend (`kvm-bindings` / `kvm-ioctls`) |
| `hvf` | | Hypervisor.framework backend (macOS): x86-64 guests on Intel, AArch64 guests on Apple Silicon |
| `smir-jit` | ✓ | SMIR native hot-block JIT (x86-64 host; on by default, `RAX_NO_JIT=1` disables at runtime) |
| `trace` | | SDE-compatible instruction tracing |
| `debug` | | GDB Remote Serial Protocol server |
| `profiling` | | per-mnemonic profiler + JSON export |
| `x86_64-suite` | | the 28,554-case x86-64 instruction test suite |

---

## The microkernel test harness

`microkernel/` is a freestanding bare-metal x86-64 kernel used to exercise the interpreter end to end
without a full Linux image: an N-body physics simulation, a bump allocator, and broad instruction
coverage, the same binary runnable on rax and on Intel SDE for cross-checking.

```bash
cd microkernel
make baremetal     # build the bare-metal ELF
make test-rax      # boot it in the rax software emulator
make test-sde      # run it under Intel SDE for a reference trace
```

---

## Repository map

```
src/
├── main.rs · lib.rs · config.rs · vmm.rs   # CLI, VM monitor, run loop
├── memory.rs · timing.rs · trace.rs · snapshot.rs
├── cpu/            # VCpu trait, register/system state (x86 · hexagon · riscv), exit reasons
├── arch/           # boot protocols + loaders: x86_64/ · riscv.rs · hexagon · arm
├── backend/
│   ├── kvm/        # Linux hardware virtualization (HVF for macOS)
│   └── emulator/
│       ├── x86_64/ # ~54k LOC: decoder, mmu, flags, dispatch/{legacy,twobyte,vex,evex}, insn/ (88 files), JIT integration
│       ├── hexagon/# ~38k LOC: scalar core, VLIW packets, full HVX, every opcode verified
│       └── riscv/  # RiscVVcpu bridges the rax::riscv interpreter into the VMM
├── arm/            # ~64k LOC: aarch64 (complete SVE) · cortex_m · decoder · vfp · sysreg · cp15
├── riscv/          # ~11k LOC: RV64GC + RVA23 scalar · cpu · decode · rvc · float · csr · crypto · disasm
├── smir/           # ~140k LOC: ir · ops · types · interp · opt · lift/ · lower/ (x86_64 · aarch64 · regalloc · runtime)
├── devices/        # serial·pit·pic·lapic·ioapic·rtc·hpet·pci·fw_cfg·pl011·s3c64xx·s5l8900  +  ahci·nvme·ide·virtio·e1000·vga·ac97·uhci·fdc·dma
├── gdb/            # Remote Serial Protocol server      (--features debug)
└── profiling/      # per-mnemonic profiler              (--features profiling)

tests/              # differential (x86↔KVM) · arm_diff/arm_diff32/riscv_diff/hexagon_*_diff (↔QEMU) · smir_jit_vcpu · realmode_boot · riscv_boot
                    # x86_64/ (28,554) · arm/generated (92,131, from ASL) · diff_fuzz · smir_avx10_roundtrip
tools/              # asl-parser (ARM ASL → tests) · arm-diff · riscv-diff · hexagon-diff (QEMU oracles)
microkernel/        # bare-metal x86-64 test kernel
docs/specifications/# smir/ (the IR spec) · riscv/ (vendored RISC-V specs) · arm/
```

---

## Status

| Path | State |
|------|-------|
| **x86-64 (KVM/HVF)** | Boots Linux to an interactive shell |
| **x86-64 (software)** | Boots Linux to a BusyBox shell; full modern ISA; 463 differential cases vs. KVM; native JIT (`smir-jit`) at ~80× on hot loops |
| **Hexagon** | **Every opcode** (scalar + HVX) verified vs. qemu-hexagon; bootable bare-metal backend |
| **RISC-V** | Full RVA23 scalar set + crypto; bootable `--arch riscv64` backend; verified vs. qemu-riscv64 |
| **AArch64 / ARM** | **Boots Linux** (HVF near-native on Apple Silicon, or full EL0/EL1 software emulation); AArch64 + AArch32 bit-exact vs qemu; ~92k ASL tests; the ARMv6/S5L8900 machine boots Apple's iBoot |
| **SMIR** | JIT on by default, auto-triggered, fail-safe (integer + memory hot regions native, bit-exact vs. KVM); lowers to x86-64 **and** AArch64 hosts; RISC-V (incl. RVV) and Hexagon lifts complete |
| **Platform** | Legacy PC devices wired; PCI host bridge + `--pci-devices` (e1000 `eth0`, AHCI/NVMe/UHCI/AC97); interactive console + full `.rxc` machine checkpoint/resume |
| **Legacy boot** | Real-mode mini-BIOS + El-Torito CD boot; **TempleOS V5.03** boots real to long mode, mounts its CD, runs its HolyC compiler |

### What's missing

A production hypervisor this is not, by design. x86-64 and AArch64 both boot a general-purpose OS;
Hexagon and RISC-V run bare-metal only, and the 32-bit AArch32 core is validated through its oracle but
not yet wired as a backend. There is no SMP (one vCPU executes), VGA isn't wired (serial console only),
and PCI interrupts run in polled mode. The JIT compiles integer and memory hot regions; the double-width
DIV model and native block-to-block chaining are still future work. RISC-V lacks the RVV vector data
path and a privileged/Sv39 MMU. The **software** Linux boot reaches a BusyBox shell on a mitigations-off
ELF kernel; wider configurations (CFI/FineIBT, bzImage real-mode entry) are still being worked through,
and the KVM path boots cleanly throughout.

---

## A note on the name

`rax` is the x86-64 accumulator register, the first register the manuals introduce. The project started
x86-64-centric and the name stuck even as it grew three more instruction sets; it is also just the crate
name, so `cargo run` and you are off.

---

## See also

- [kvm-ioctls](https://github.com/rust-vmm/kvm-ioctls) / [kvm-bindings](https://github.com/rust-vmm/kvm-bindings): KVM access
- [linux-loader](https://github.com/rust-vmm/linux-loader) / [vm-memory](https://github.com/rust-vmm/vm-memory): boot protocol & guest memory
- [QEMU](https://www.qemu.org/): the user-mode reference oracle for AArch64, Hexagon, and RISC-V
- [Intel SDM](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html) · [Arm ASL](https://developer.arm.com/Architectures/A-Profile%20Architecture) · [RISC-V specs](https://riscv.org/technical/specifications/)
- [`docs/specifications/smir/`](docs/specifications/smir/): the SMIR IR specification

## License

MIT
