# AArch32 / Thumb Completion Session — 2026-06-05

Drove rax's AArch32 interpreter (`src/arm/execution.rs` + `src/arm/instructions.rs`
+ `src/arm/decoder/{aarch32,thumb}.rs`) to **bit-exact, hardware-verified**
coverage of the entire **A32 + T16 + T32 integer register-data-processing**
instruction set, verified against a new **qemu-arm differential oracle**.

## New oracle infrastructure

`tools/arm-diff/oracle-a32.c` (+ `build-a32.sh`) — a static 32-bit ARM ELF run
under `qemu-arm` (user mode), the 32-bit analog of the AArch64 oracle. Mechanism
mirrors it: a two-phase signal dance loads the full architectural state (R0–R14,
CPSR NZCVQ+GE, FPSCR, D0–D31) via an ARM register-loading prologue, then an
**interworking `ldr pc` branch** (literal bit0 = T) enters the patched test slot
in **ARM or Thumb** state, runs one instruction, and `BKPT`s into a handler that
captures the post-state from the signal frame (GPRs/CPSR from `sigcontext`,
D-regs/FPSCR from the VFP record). A MAP_FIXED scratch window at `0x200000`
backs load/store tests.

`tools/arm-diff/gen_a32.py` enumerates every integer mnemonic across the
operand variations that affect semantics and emits `tests/arm32_gen.rs`
(`A32_SWEEP`: 1666 entries — A32 916, T16 35, T32 715), each tagged with a mode
(0=ARM, 1=T16, 2=T32). The harness `tests/arm_diff32.rs` drives `Armv7Cpu` +
`Aarch32Decoder`/`ThumbDecoder` + `Executor` per instruction and compares
GPRs + CPSR(NZCVQ+GE) + scratch against the oracle at **24 inputs/insn**
(~40k oracle cases), asserting zero divergence. Self-skips if `qemu-arm` / the
cross toolchain is absent.

## A32 fixes (commit 43b60d2)

Started at 7507 mismatches. Root causes / additions:

- **RRX shift**: `decode_dp_operands` mapped `ROR`-with-`imm5==0` to ROR #1
  instead of RRX (rotate-right-through-carry).
- **MOVW/MOVT**: not decoded (S=0 TST/CMP opcode slots) — added, lowered to the
  reused `MOVZ`/`MOVK` exec.
- **UMAAL** + **MLS** opcode fix (`decode_multiply` had MLS at UMAAL's slot);
  S-flag setting for `MLAS`/`UMLALS`/`SMLALS`.
- **GE flags**: added `ge` to `Psr` (bits 19:16) — set by parallel add/sub, read
  by SEL.
- A whole **media/DSP build-out** (`decode_media` + the misc/halfword-multiply
  decode in `decode_dp_misc`, with umbrella mnemonics whose exec re-derives the
  op from the raw word): saturating `QADD/QSUB/QDADD/QDSUB`, `SSAT16/USAT16`,
  half/word multiplies `SMUL/SMLA/SMULW/SMLAW/SMLAL<x><y>`, dual
  `SMUAD/SMUSD/SMLAD/SMLSD`, `SMLALD/SMLSLD`, `SMMUL/SMMLA/SMMLS`, `USAD8/USADA8`,
  `PKHBT/PKHTB`, the extend family `(U|S)XT(A)(B|H|B16)`, `SEL`, the
  parallel add/sub matrix (`S/Q/SH/U/UQ/UH` × `add8/add16/sub8/sub16/asx/sax`,
  with correct GE), and the bit-field/saturate ops that existed in exec but were
  never decoded for A32 (`SSAT/USAT/SBFX/UBFX/BFI/BFC`).

## Thumb fixes (commit 347070e)

The `ThumbDecoder`+`Executor` path had **never been differentially exercised**
(`tests/arm/generated/mod.rs` only enables a64). Both T16 and T32 mis-executed
because the A32 exec re-reads `insn.raw` with A32 field positions.

- **Operand-based Thumb executor**: `decode_dp_operands`, `decode_shift_operands`,
  `decode_mul/mla/mull_operands`, `exec_neg`, the two-register ops
  (`CLZ/REV/REV16/REVSH/RBIT/extends`), `SDIV/UDIV`, the bit-field/saturate ops
  and `MOVT` now read the decoded operands (or T32 raw layout) when
  `insn.state.is_thumb()`, reusing all the shared arithmetic. This made **T16**
  green immediately.
- **T32 decode build-out**: implemented the stubbed `decode_32bit_dp_register`
  (register-controlled shifts, extends, parallel, QADD/CLZ/REV/SEL),
  `decode_32bit_data_processing` (shifted-register DP), `decode_32bit_dp_plain_imm`
  (ADDW/SUBW/MOVW/MOVT + bitfield/saturate, split from the modified-immediate
  path on bit25), and the DSP multiplies in `decode_32bit_multiply` /
  `decode_32bit_long_multiply_divide` (incl. UMAAL/SMLALD/SMLSLD).
- **ThumbExpandImm fix**: the modified-immediate rotate used a 4-bit `imm12>>8`
  control; corrected to value `0x80:imm12[6:0]` rotated by the 5-bit `imm12[11:7]`.
- The A32 media/DSP umbrella exec functions were made **state-aware** (T32 field
  layouts differ entirely from A32 — different register positions, op/prefix
  encodings, and the `tb` bit at `hw2[5]`).

## Result

`diff_a32_integer_sweep`, `diff_t16_integer_sweep`, `diff_t32_integer_sweep` —
all green at 24 inputs/insn, zero divergence vs `qemu-arm`.

## Memory load/store (commit 8dec9c5)

A second sweep (`A32_MEM_SWEEP`, 396 entries; tests `diff_{a32,t16,t32}_
memory_sweep`) drives single/dual/multiple loads and stores through the MAP_FIXED
scratch window (base register pointed at `SCRATCH_BASE`, offset register held
small so every access stays in the exchanged region). All green. Fixes:

- A32 **LDRD/STRD** were undecoded (extra-load/store `L=0` op1=10/11 slots) and
  **LDM/STM IB/DA** modes had no exec — added a unified `exec_ldm_stm` covering
  all four IA/IB/DA/DB modes (lowest register ⇒ lowest address; LDM-base-in-list
  suppresses writeback).
- The Thumb load/store exec was made operand-based (`decode_mem_thumb`):
  `decode_ldst_operands`/`decode_ldst_halfword_operands`/`decode_ldstm_operands`
  read the decoded `MemOperand`/`RegList` when `insn.state.is_thumb()`.
- **T32 single load/store** decoders only handled the T3 positive-imm12 form;
  added `t32_mem_operand` covering T3, T4 (±imm8 offset/pre/post-index +
  writeback), and the register-offset (LSL imm2) form.
- **T32 LDM/STM** decoder ignored the IA/DB mode bit (always LDMIA/STMDB); fixed
  to read bits[24:23]. Implemented the stubbed **T32 LDRD/STRD** dual decoder
  (the `Rn` field was also read from the wrong halfword).

## Remaining frontier (not in this pass)

- **AArch32 VFP/NEON** — `Armv7Cpu` has no integrated `VfpState`; the A32/T32
  decoders don't decode VFP. The oracle already captures D0–D31/FPSCR, so this is
  a wiring + decode + exec effort, not an oracle gap.
- **Cortex-M system features** (NVIC/SCB/SysTick/MPU/exception model/TrustZone/
  MVE) live in the separate `src/arm/cortex_m/` CPU and are system-level, not
  user-mode-differential-testable.

## How to run

```
cargo test --release --test arm_diff32                       # all 6 sweeps
cargo test --release --test arm_diff32 diff_t32_integer_sweep -- --nocapture
cargo test --release --test arm_diff32 diff_a32_memory_sweep  -- --nocapture
A32DIFF_FILTER=qadd cargo test --release --test arm_diff32 diff_a32_integer_sweep -- --nocapture
```
