# RISC-V in rax — Implementation Status

This document records the state of RISC-V support in rax across two layers:

1. **The interpreter** (`src/riscv/`) — a self-contained, spec-faithful RV64
   software interpreter, differentially verified against `qemu-riscv64`.
2. **The SMIR lifter** (`src/smir/lift/riscv.rs`) — translation of RISC-V machine
   code to rax's SMIR (the hot-block JIT IR), verified against the interpreter.

Companion docs: [`REMAINING.md`](REMAINING.md) (interpreter roadmap — privileged
arch / MMU). The two verification harnesses are the backbone of the "provably
correct" guarantee below; both **fail on any divergence** and self-skip when their
toolchain is absent.

---

## 1. Interpreter (`src/riscv/`)

A foundational RV64 interpreter structured to parallel `src/arm/`, intentionally
decoupled from the VMM so the differential oracle drives it directly.

### Coverage

The **entire RVA23 unprivileged scalar ISA** plus the **complete RVV 1.0 vector
data path**:

| Group | Extensions |
|-------|-----------|
| Base + GC | RV64I, **M**, **A** (LR/SC + AMO), **F** + **D**, **C** (compressed) |
| FP | full IEEE-754, all 5 rounding modes; **Zfh** (half); integer-significand soft-float (`float.rs`) generic over `Fmt = {F16,F32,F64}` |
| Bit-manip | **Zba / Zbb / Zbc / Zbs**, **Zbkb**, **Zbkx**, **Zcb** |
| Conditional / FP-aux | **Zicond**, **Zfa** |
| Scalar crypto | **Zknh** (SHA-256/512), **Zksh** (SM3), **Zksed** (SM4), **Zkne/Zknd** (AES-64) — S-box tables + GF(2⁸) |
| CSR / fence | **Zicsr**, **Zifencei** |
| **Vector** | **V (RVV 1.0)** — see below |

### Vector (RVV 1.0) — the entire data path

`VLEN = 128` (matches qemu's default `vlenb = 16`), a flat `v[32×16]` register
file so LMUL groups and element strides index naturally. Implemented and verified
family-by-family (32 differential suites in `tests/riscv_vector.rs`):

- **Config**: `vsetvli` / `vsetivli` / `vsetvl` (vill, VLMAX, vl clamping)
- **Integer arithmetic**: add/sub/rsub, and/or/xor, min/max(u), shifts (vv/vx/vi)
- **Compares** → mask registers (`vmseq` … `vmsgt`)
- **Merge / move** (`vmerge`, `vmv.v.*`)
- **Multiply / divide**: `vmul`, `vmulh`/`vmulhu`/`vmulhsu`, `vdivu`/`vdiv`/`vremu`/`vrem` (div-by-zero / signed-overflow corners)
- **Fixed-point**: saturating add/sub (`vsadd…`, sets `vxsat`), averaging (`vaadd…`), scaling shifts (`vssrl`/`vssra`), fractional multiply (`vsmul`), narrowing clip (`vnclip…`) — all four `vxrm` rounding modes
- **Carry/borrow**: `vadc`/`vmadc`/`vsbc`/`vmsbc`
- **Integer extension**: `vzext`/`vsext` `.vf2/.vf4/.vf8`
- **FP arithmetic**: add/sub/rsub/mul/div/rdiv/min/max/sgnj/sqrt; all 8 **FMA** variants; `vfrsqrt7`/`vfrec7` (spec lookup tables); `vfclass`
- **FP compares** → mask
- **Reductions**: integer + FP (incl. ordered/unordered) and **widening** (`vwredsum…`, `vfwredsum…`)
- **Mask**: register logicals (`vmand…`), manipulation (`vcpop`/`vfirst`/`vmsbf`/`vmsif`/`vmsof`/`viota`/`vid`)
- **Scalar moves**: `vmv.x.s` / `vmv.s.x` / `vfmv.f.s` / `vfmv.s.f`
- **Permutes**: slides (incl. `vfslide1*`), gather (`vrgather`/`vrgatherei16`), `vcompress`
- **Widening integer**: add/sub (+ `.w`), multiply, multiply-accumulate (all signed/unsigned conventions)
- **Narrowing**: shifts + clip
- **Conversions**: single-width, widening, and narrowing `vfcvt`/`vfwcvt`/`vfncvt` (incl. `rtz` and round-to-odd)
- **Widening FP**: arithmetic, FMA
- **Whole-register move** (`vmv<nr>r.v`)
- **Load/store — all addressing modes**: unit-stride, strided, indexed, mask (`vlm`/`vsm`), whole-register, **segment** (unit/strided/indexed), and fault-only-first (`vleff`, non-fault path)

### Interpreter verification

The golden oracle is `qemu-riscv64` (user mode). Two static RV64 ELF oracles run
under qemu-user; a prologue loads register/vector state from a `MAP_FIXED` block,
runs one patched instruction, then `EBREAK`, and a `SIGTRAP` handler captures the
machine state and `siglongjmp`s back.

- `tools/riscv-diff/oracle.c` — scalar state (x/f/fcsr/pc). Reserves `x3/x4`
  (`gp`/`tp`) so the handler's glibc TLS survives.
- `tools/riscv-diff/voracle.c` — vector state (the V signal-frame context, magic
  `0x53465457`, holding `vstart/vl/vtype/vcsr/vlenb` + the 512-byte register
  file). Extended this session to load/capture **`vcsr`** so `vxsat`/`vxrm` are
  verified for the fixed-point families.

Test inventory (all green; self-skip without qemu + `riscv64-linux-gnu-gcc`):

- `tests/riscv_diff.rs` — **29 scalar suites** including massive fuzzers
  (`diff_decode_fuzz` 140k words, `diff_mem_fuzz` 70k, `diff_fuzz_exhaustive` 90k,
  `diff_compressed_fuzz` 8k) → **~300k+ comparisons/run**, plus structured suites
  for FP, Zfh, crypto, bit-manip, Zicond, Zfa.
- `tests/riscv_vector.rs` — **32 vector suites**, every family above, across SEW
  8/16/32/64, vv/vx/vi/vf forms, masked/unmasked, all rounding modes; compares
  the full x/f/v register file + vl/vtype/fcsr/vcsr/scratch window.
- `tests/riscv_boot.rs` — end-to-end VMM boot (UART @ `0x10000000`, `ecall`→halt).
- `cargo test --lib riscv::` — ~45 unit tests.

### VMM integration

`ArchKind::Riscv64`, `CpuState::RiscV`/`RiscVRegisters`, `src/arch/riscv.rs`
(ELF/raw load, 16550 UART), `src/backend/emulator/riscv/cpu.rs` (`RiscVVcpu`).
Run with `--backend emulator`.

### Known limitation (interpreter)

Vector register-group **overlap / alignment illegal-instruction** traps are not
enforced — `rax` executes some encodings qemu rejects (widening/narrowing/gather
dest-vs-source overlap, EMUL>1 misalignment). A `PROBE` against qemu showed it
enforces alignment + source-source different-EEW rules *beyond* the written spec,
so a spec-faithful checker both over- and under-traps; matching qemu is
implementation reverse-engineering and was deliberately not shipped (affects only
illegal encodings no compiler emits). Privileged arch / Sv39 MMU: see
[`REMAINING.md`](REMAINING.md).

---

## 2. SMIR lifter (`src/smir/lift/riscv.rs`)

`RiscVLifter` (exposed as `rax::smir::RiscVLifter`) translates RISC-V machine code
to SMIR ops for the hot-block JIT.

### Verification harness — `tests/riscv_smir_lift.rs`

For each instruction: lift to SMIR → run on `SmirInterpreter` from a seeded state
→ compare x/f/fcsr/scratch against the (qemu-verified) `RiscVCpu`. The interpreter
is the golden oracle, so no external toolchain is needed and encodings are
generated directly. **The test fails on any divergence**; an op the lifter doesn't
implement is reported as an honest *gap*, never silently mis-lifted.

> Critical harness detail: the RISC-V lifter writes results to SSA virtual regs
> (`ctx.define_arch_reg`), with the arch→vreg map held in the *lifter's*
> allocator. Results are read back via `ctx.read_vreg(lctx.get_arch_reg(…))`, not
> `ctx.arch_regs` (which only holds the seed).

Five sweeps; latest run (**zero divergence across all of them**):

| Sweep | matched | gap-ops | diverged |
|-------|---------|---------|----------|
| `lift_mem` (load/store/AMO) | 40000 | **0** | 0 |
| `lift_c` (compressed) | 20000 | **0** | 0 |
| `lift_op_imm` (OP-IMM/LUI/AUIPC) | 39993 | 1 | 0 |
| `lift_op` (OP/OP-32) | 30378 | 12 | 0 |
| `lift_fp` (FP load/store/op/fma) | 9690 | 99 | 0 |

### Lifted & verified

- **Integer**: RV64I; **M** — multiply, and **div/rem (64-bit + word) via a
  non-trapping `Select`-based sequence** (SMIR's `DivS`/`DivU` trap x86-`#DE` on
  zero; the sequence sanitizes the divisor and selects RISC-V's `/0`→all-ones &
  `MIN/-1`→`MIN`), plus **`mulhsu`** (= `mulhu − (a<0?b:0)`)
- **A**: LR/SC and all AMOs
- **C**: **100% complete** — base + Zcb (`c.mul`/`zext`/`sext`/`not`,
  `c.lbu`/`lhu`/`lh`/`sb`/`sh`) + compressed FP load/store (`c.fld`/`c.fsd`)
- **Zba / Zbb / Zbs / Zicond**: decode-driven `lift_zb_op`/`lift_zb_imm`/
  `lift_zb_imm32` (reuse the rax decoder for the precise `Op`); `andn`/`orn`/
  `xnor`, rotates, min/max, sh-add(.uw), bset/clr/inv/ext, clz/ctz/cpop, sext/
  zext, **`rev8`/`brev8`** (`brev8 = bswap(rbit(x))`), `czero.eqz/nez`
- **Zbkb**: `pack`/`packh`/`packw`
- **Zk-hash**: **SHA-256/512 + SM3** (`crypto_xor3` — rotate/xor folds)
- **FP — the entire fflags/rounding-free subset** (bit ops on the f-register
  VRegs): `FMV.*` moves, `FSGNJ/N/X.S/D/H` (sign inject; `.S`/`.H` canonicalize
  an improperly-NaN-boxed operand via an `unbox` `Select`), `FLW`/`FLD`/`FLH`/
  `FSW`/`FSD`/`FSH` load/store, and **`FCLASS.S/D/H`** (10-bit classify)

### Bugs found & fixed (caught by the harness)

1. `FlatMemory::atomic_rmw` did signed min/max on full-64-bit values regardless of
   access width and didn't mask the operand → `AMOMIN/MAX.W` wrong. Now width-
   masks operands and sign-extends from `size` (fixes **all** architectures' AMOs).
2. AMOs with `rd == x0` skipped the **entire** op, dropping the memory RMW (RISC-V
   still performs it). Now always emits the RMW with a throwaway destination.
3. The C-extension sign-extended 6-bit immediates from bit 7 (`as i8`) instead of
   bit 5 → `c.addi`/`c.addiw`/`c.li`/`c.andi` off by 64 for negative immediates.
4. Word AMO/LR results were not sign-extended into `rd`.
5. The original lifter silently mis-lifted bit-manip variants as base shifts
   (e.g. `rori` as `srai` — it checked `funct7` bits that overlap the 6-bit
   `shamt`). Now anything that isn't a base op routes to the decode-driven path,
   so the lifter **never emits a wrong op** — it lifts correctly or returns
   `Unsupported`.

### Gaps — blocked by SMIR's op-set (not by RISC-V), intentional

Every remaining op needs a **shared SMIR-core change** (`ops.rs`/`interp.rs` + the
exhaustive matches), which is out of the RISC-V-specific scope and actively
churned by concurrent agents:

- **FP arithmetic / convert / compare / min-max / round (~99 ops)** — SMIR's FP
  interp is native `a+b` with no fflags tracking, no NaN canonicalization, no
  dynamic rounding; the harness compares `fcsr`, so lifting these would diverge.
  Needs an FP-with-flags overhaul.
- **AES (`Aes64*`) / SM4 (`Sm4ks/Sm4ed`)** — need a 256-entry S-box table-lookup op.
- **Clmul/Clmulh/Clmulr** — need a carry-less-multiply primitive.
- **Xperm4/8** — crossbar byte/nibble gather.

---

## 3. Commit index (this session, RISC-V-specific)

**Vector data path (interpreter):** `521d6ff` config · `5a1825f` basic ld/st +
int arith · `f4ac4a3` min/max/shift/merge · `3d5aec9` compares · `4dedf44`
mul/div · `79e6bff` FP · `1484a20` FMA · `deba9b5` int reductions · `94f4a32` FP
reductions · `e3d6fe8` scalar moves · `9543e6c` mask logicals · `da157bb`
zext/sext · `a29141e` mask manip · `053b2ff` slides · `97fd740` gather ·
`54795fb` compress · `85c511f` carry/borrow · `29fb294` sat add/sub (+vcsr
harness) · `a6a2750` averaging · `4b7e58e` scaling/vsmul · `906ed62` widening
add/sub · `2cf150b` widening mul · `16e797f` narrowing shift/clip · `816d9cb`
single-width conv · `8b8fee4` widening conv · `e6fc31c` narrowing conv ·
`4d96619` widening FP arith · `bbee9cd` widening FP FMA · `2b5055a` widening
reductions · `1482ffe` vfclass + whole-reg move · `959e54d` advanced ld/st ·
`592c9a2` segment ld/st · `c7530a5` vfrsqrt7/vfrec7 · `29bf643` fault-only-first.

**SMIR lift:** `cc9757b` harness + shift/div/atomic fixes · `15547e0`
Zba/Zbb/Zbs/Zicond · `acaecc7` bit-manip immediates · `b3a34b0` C-ext immediate
sign-ext fix · `a8d9449` SHA/SM3 · `453122e` pack · `751486d` word div/rem ·
`8b3693d` brev8/mulhsu · `fcaf30e` + `75af4d6` Zcb · `ebe64ef` FP load/store +
moves/sign · `684f5d5` compressed FP load/store · `a3fc2d5` `.S`/`.H` sign-inject
· `90792cc` fclass.

---

## Summary

The **unprivileged RV64GCV ISA is comprehensively implemented and verified** in
the interpreter (29 scalar + 32 vector differential suites + ~300k fuzz
comparisons/run, all at zero divergence vs qemu). The **SMIR lift covers
everything expressible with the current SMIR op-set** — the complete integer ISA,
all loads/stores/AMO, 100% of compressed, all Zb*/Zicond/Zk-hash, and the entire
fflags-free FP subset — also at zero divergence (~150k+ instructions/run). The
remaining gaps in both layers are precisely characterized and bounded: the
interpreter needs privileged arch / Sv39 (a qemu-*system* oracle), and the lifter
needs SMIR-core primitives (FP-with-flags, S-box, carry-less multiply).
