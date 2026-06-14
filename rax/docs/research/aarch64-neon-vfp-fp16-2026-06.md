# AArch64 NEON / VFP / FP16 Completion Session — 2026-06-05

Drove rax's AArch64 interpreter (`src/arm/aarch64/cpu.rs`) to **bit-exact,
hardware-verified** coverage of the entire AdvSIMD (NEON) + scalar-VFP + FP16
*data-processing* instruction set, verified against the **qemu-aarch64
differential oracle** (`tools/arm-diff/` + `tests/arm_diff.rs`).

- A new permanent regression test **`diff_neon_comprehensive_sweep`** sweeps an
  `llvm-mc`-generated **3939-instruction** encoding table (`tests/neon_gen.rs`),
  covering every NEON/VFP/FP16 mnemonic across all element arrangements, Q
  variants, element indices, shift amounts, immediates and sign variants, at
  **16 random + special-value inputs each** (~63k oracle cases), and asserts
  zero divergence.
- The full `arm_diff` suite is **198/198 green**.
- Starting point: the sweep found **422 decode-gaps + 121 value-mismatches**;
  all closed.

---

## Methodology

The same loop-until-dry coverage probe as the SVE2/SVE2.1 work
(`aarch64-sve2-completion-2026-06.md`):

1. **Enumerate.** A fan-out of 16 agents (one per encoding group: three-same
   int/FP, three-different, two-reg-misc int/FP, across-lanes, copy, modified-
   immediate, shift-by-immediate, by-element, permute, scalar-AdvSIMD, crypto,
   VFP scalar DP, VFP convert/compare, FML/BF16/I8MM) each exhaustively listed +
   `llvm-mc`-validated its group's encodings. Deduped to 3939.
2. **Probe.** `diff_neon_comprehensive_sweep` runs each through the oracle with
   random + `interesting()` (special-FP-laden) inputs and classifies every
   mnemonic as decode-gap / value-mismatch / fault-disagree / OK.
3. **Fix** against qemu source ground truth (`target/arm/tcg/{a64.decode,
   translate-a64.c, vfp_helper.c, crypto_helper.c, vec_helper.c}` and the ARM
   ASL), re-probe, repeat.
4. After green at 6 inputs, **hardened to 16 inputs**, which surfaced one more
   rare NaN-reduction-order bug (fixed); then re-verified.

A final **cross-check against qemu's full `a64.decode` mnemonic list** confirmed
every FP/SIMD data-processing mnemonic is covered by the sweep or the focused
tests (the only non-matches were non-NEON: `CLREX`, `SMC`, and the internal
alias names `FMOVI`/`TBL_TBX`).

---

## Fixes by theme

**Systematic root causes** (each cleared a whole cluster):

- **Scalar AdvSIMD ops not zeroing the upper V bits.** `exec_simd_two_reg`,
  the XTN narrowing, scalar shift-by-immediate, scalar by-element widening and
  scalar three-different all treated the scalar (top-byte 0x5E/0x5F/0x7E/0x7F)
  forms as 2-lane vectors. Added scalar detection so they write one element to
  lane 0 and zero the rest (`abs/neg/cmXX/sqabs/sqneg/suqadd/usqadd/sqxtn/uqxtn/
  sqxtun/sqdmlal/sqdmull/...` scalar).
- **Scalar shift-by-immediate never dispatched.** The shift-imm gate matched
  only the vector top-byte (`bits[28:23]==011110`); scalar is `111110`. Widened
  the gate; the handler's scalar flag was also reading the wrong marker
  (`11110` vs the shift-imm `11111`).
- **FP reductions used the default NaN / a sequential fold.** `fp_max_f32`/
  `fp_min_f32` returned the default NaN; the across-lanes reduction folded
  left-to-right. ARM `Reduce()` is a recursive split-in-half **tree** whose
  order is observable when a NaN is present — switched FMAXV/FMINV/FMAXNMV/
  FMINNMV (f32 and FP16) to `sve_fp_tree_reduce`.
- **FP fused-multiply-add used host (x86) NaN selection / non-fused arithmetic.**
  `exec_simd_bfmlal` and `exec_fmlal` used Rust `f32::mul_add`/`a±prod`; switched
  to `fp_muladd_bits` (ARM `float32_muladd`, addend-NaN-first). `fp16_mls`
  didn't negate the multiplicand before NaN processing; `fp16_maxnum_minnum`
  diverged from the verified f32 NaN rules — both fixed.

**New instruction support implemented + verified:**

- Scalar FP: immediate (`FMOV Sd/Dd/Hd,#imm`), GPR/scalar fixed-point converts
  (`SCVTF/UCVTF/FCVTZS/FCVTZU #fbits`), `FJCVTZS` (incl. the JS `-0.0`-is-inexact
  Z-flag), `FMOV` FP16↔GPR, `FRINT32X/Z`+`FRINT64X/Z` (FEAT_FRINTTS, scalar).
- AdvSIMD: scalar `DUP`-to-scalar (`MOV <V>d,<Vn>.T[i]`), scalar pairwise
  (`ADDP/FADDP/FMAXP/FMINP/FMAXNMP/FMINNMP`), FP16 across-lanes, FP16 vector
  `FMOV` immediate, FP convert long/narrow (`FCVTL/FCVTL2/FCVTN/FCVTN2/FCVTXN/
  FCVTXN2`, NaN-correct), `FRINT32/64` vector.
- Extensions: FEAT_RDM (`SQRDMLAH/SQRDMLSH`, three-same-extra + scalar),
  FEAT_I8MM (`SMMLA/UMMLA/USMMLA`), FEAT_FHM (`FMLAL/FMLSL/FMLAL2/FMLSL2`
  by-element), FEAT_SHA512 (`SHA512H/H2/SU0/SU1`) and FEAT_SHA3 (`EOR3/BCAX/
  RAX1/XAR`).

---

## Notable subtleties (qemu ground truth)

- **FRINT32/64**: NaN/Inf/out-of-range yield `INT{32,64}_MIN` as a float
  (`(0x100+126+intsize)<<23` for f32). The exact `INT_MIN` value is in range
  (sign set, fraction 0); everything past it overflows.
- **FJCVTZS**: result is JS `ToInt32` (round-toward-zero, modulo 2^32); Z=1 iff
  exact, where **`-0.0` counts as inexact** (qemu `helper_fjcvtzs`).
- **FCVTXN**: f64→f32 round-to-odd, NaN payload preserved (FPCR.DN-governed),
  not the forced default NaN.
- **TBX** with Q=0 zeroes the upper 64 bits (it had kept Vd's upper half).

---

## Scope / remaining frontier (deliberately not in this pass)

- **NEON load/store** (`LD1/ST1/LDP/STP/LD1R/...`) are memory ops, covered by
  the existing `diff_mem_*` / `diff_simd_*` differential tests via the oracle's
  scratch window — not part of the register-only sweep.
- **FPCR control-bit behaviour** — `FPCR.RMode` (alternate rounding), `FZ`/`FZ16`
  (flush-to-zero), `DN` (default-NaN), `AH` (alternate handling). The whole
  sweep runs at the architectural default `FPCR=0` (round-to-nearest, no flush),
  which is what essentially all software uses, and every instruction is
  bit-exact there. The FP core uses native Rust (round-to-nearest) arithmetic
  and does not read `FPCR.RMode`/`FZ`, so honouring those control bits is a
  **distinct, larger feature** (a softfloat-style rounding rewrite across all FP
  ops), not an instruction-coverage gap. Documented here as the next frontier.

---

## How to run

```
cargo test --release --test arm_diff diff_neon_comprehensive_sweep -- --nocapture
cargo test --release --test arm_diff            # full suite (198 families)
```

Self-skips if `qemu-aarch64` / the cross-toolchain are absent. See
`memory/rax-neon-vfp-completion.md` and `memory/rax-arm-diff-oracle.md`.
