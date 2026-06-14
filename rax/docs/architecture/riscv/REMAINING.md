# RISC-V (`rax::riscv`) — Remaining Work

Status snapshot for the self-contained RISC-V interpreter at `src/riscv/`. The
**user-mode ISA is complete and differentially verified** against `qemu-riscv64`;
what remains is privileged/system architecture, a few illegal-encoding fidelity
gaps, and additional (mostly optional) extensions.

## Done (for context)

- **RV64GC**: I, M, A (LR/SC + AMO), F + D, C (compressed).
- **FP**: full IEEE-754 incl. **Zfh** (half), all 5 rounding modes, integer-significand
  soft-float in `float.rs` generic over `Fmt = {F16, F32, F64}`.
- **Bit-manip / crypto**: Zba/Zbb/Zbc/Zbs, Zicond, Zfa, Zbkb, Zbkx, Zcb, and full
  **scalar crypto** (Zknh SHA-256/512, Zksh SM3, Zksed SM4, Zkne/Zknd AES-64).
- **Zicsr + Zifencei**, user-visible counters (cycle/time/instret read paths).
- **V (RVV 1.0) — the entire data path**: config (`vsetvl*`), all integer/FP
  arithmetic, multiply/divide, FMA, reductions (incl. widening), fixed-point
  (sat/avg/scaling/clip with `vxsat`/`vxrm`), mask ops, permutes (slide/gather/
  compress), widening/narrowing, all conversions (`vfcvt`/`vfwcvt`/`vfncvt` incl.
  rtz + round-to-odd), `vfrsqrt7`/`vfrec7`, and **all load/store addressing modes**
  (unit/strided/indexed/mask/whole-register/segment + `vleff`).
- **Verification**: 29 scalar + 32 vector differential suites (`tests/riscv_diff.rs`,
  `tests/riscv_vector.rs`) + ~45 lib unit tests, all green. Oracles:
  `tools/riscv-diff/{oracle,voracle}.c` (static RV64 ELF run under qemu-user;
  SIGTRAP handler captures the register/vector frame).

---

## Tier 1 — Privileged architecture + MMU (needed to boot Linux)

This is the single largest remaining frontier. The current privileged support is a
**minimal M-mode trap model only**: `mstatus/mtvec/mepc/mcause/mie/mip/medeleg/
mscratch` exist, synchronous trap entry into M-mode works, and `mret` restores
state. `sret` currently **aliases** `mret` (single-mode model — see
`cpu.rs` "single-mode model: same restore path"). There is **no S-mode and no
address translation**.

Concretely missing:

- **S-mode CSRs**: `sstatus, stvec, sepc, scause, stval, sscratch, sie, sip,
  satp, scounteren` (none are in `csr.rs`). Proper `sret` distinct from `mret`.
- **Trap delegation**: `mideleg` wiring, interrupt vs. exception routing to S-mode,
  `mstatus` SUM/MXR/SPP/SPIE semantics, `mstatus.TVM/TW/TSR`.
- **Sv39 (and Sv48/Sv57) page-table walk + TLB**: `satp` MODE/ASID/PPN, multi-level
  walk, A/D bit updates, permission checks (R/W/X/U), page-fault causes
  (12/13/15), `sfence.vma`. Currently every access is a flat physical access.
- **Interrupt controllers**: CLINT (mtime/mtimecmp/msip) and PLIC (external
  interrupt claim/complete). Timer interrupts (`mip.MTIP/STIP`).
- **SBI** (Supervisor Binary Interface): `ecall`-from-S handling for console,
  timer, IPI, HSM, system-reset (enough for OpenSBI → Linux).
- **WFI** beyond a nop; **counters** as real M-mode counters (`mcycle`/`minstret`
  writable, `mcounteren`/`scounteren` gating).

**Verification problem (the hard part):** the qemu-*user* signal-frame trick that
makes the current methodology "provable" does not exist for system mode. A
qemu-*system* differential oracle is a separate infrastructure project — options:
(a) gdbstub register/memory compare against `qemu-system-riscv64 -s -S` stepping a
known ROM; (b) a custom bare-metal test program whose final state is compared; or
(c) golden traces. None reuse the existing harness. Until one exists, privileged
work can only be unit-tested, not oracle-verified.

See [[rax-kernel-boot]] for the analogous x86 boot blocker, and the existing
end-to-end wiring in `tests/riscv_boot.rs` (UART @ 0x10000000, `ecall` → shutdown).

---

## Tier 2 — ISA fidelity gaps (user-mode, illegal/edge encodings only)

These don't affect any real (compiler-emitted) program; they're correctness only
on malformed encodings or fault paths.

- **Vector register-group illegal-instruction traps** — `rax` executes some
  encodings qemu rejects (widening/narrowing/gather/extension dest-vs-source
  overlap, EMUL>1 group misalignment). **Investigated and intentionally not
  shipped:** a `PROBE=1` sweep showed qemu enforces group *alignment* and even
  *source-source different-EEW* overlap rules that go beyond the written spec, so a
  spec-faithful checker both over- and under-traps. Matching qemu here is
  implementation reverse-engineering; a half-correct checker was reverted. To do it
  properly: build the checker against the qemu probe data, not the spec text.
- **`vleff` fault-trim path** — the non-fault path is verified (identical to `vle`);
  the "trim `vl` on a fault past element 0, suppress the trap" path is best-effort
  and **not** differentially tested (the scratch window never faults). Needs a fault
  injection point in the harness.
- **`vstart` resumption** — mid-instruction restart (`vstart > 0`) is handled for
  the simple loop forms but not exhaustively swept; non-zero `vstart` corner cases
  (esp. for slides/gather/segment) are unverified.

---

## Tier 3 — Additional extensions (optional; not in the current `Isa`)

The `Isa` struct (`mod.rs`) currently has no flags for these. Each is additive and
oracle-verifiable with the existing qemu-user harness (qemu supports them):

- **Zacas** — atomic compare-and-swap (`amocas.w/d/q`).
- **Zawrs** — wait-on-reservation (`wrs.nto`/`wrs.sto`).
- **Zicboz / Zicbom / Zicbop** — cache-block zero/management/prefetch.
- **Zihintpause** (`pause`), **Zihintntl**, **Zimop/Zcmop** (may-be-ops).
- **Vector crypto** (Zvbb, Zvbc, Zvkg, Zvkned, Zvknha/Zvknhb, Zvksed, Zvksh) —
  large family, mirrors the scalar crypto already in `crypto.rs`.
- **BF16** (Zfbfmin scalar, Zvfbfmin/Zvfbfwma vector) — bfloat16 convert + dot.
- **Hypervisor (H)** extension — VS/VU modes, two-stage translation (only if
  nested virtualization is ever a goal; large).
- **Sstc** (`stimecmp`), **Svnapot/Svpbmt/Svinval**, **Sscofpmf** — S-mode
  add-ons that pair with Tier 1.

The `v` flag's doc comment in `mod.rs` still reads *"configuration instructions
only"* — **stale**; the full data path is implemented. (Worth fixing.)

---

## Tier 4 — Infrastructure / testing

- **RV32 oracle** — RV32 dual-width is unit-tested only; there is no
  `riscv32-linux-gnu` multilib in the environment, so `oracle32` can't build. The
  decode/exec already thread `Xlen::Rv32`; a 32-bit oracle would close it.
- **qemu-system differential harness** — prerequisite for Tier 1 verification
  (see above).
- **Control-flow oracle coverage** — branch/jal/jalr/lui/auipc are unit-tested
  (excluded from the PC-relative oracle); a PC-aware oracle variant could fold them
  into the differential sweep.
- **Performance** — the interpreter is correctness-first; no block/JIT path for
  RISC-V (cf. the SMIR JIT used for x86). Not needed for the foundational goal.

---

## Suggested order

1. **qemu-system oracle harness** (unblocks everything in Tier 1).
2. **S-mode CSRs + Sv39 walk + sret** → **CLINT/PLIC** → **SBI** → boot OpenSBI/Linux.
3. Tier-3 extensions opportunistically (each is a self-contained, verifiable unit).
4. Tier-2 fidelity gaps last (lowest real-world impact).
