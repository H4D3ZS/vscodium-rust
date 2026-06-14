# AArch64 FP & SVE Coverage Session — 2026-06-02

A differential-testing session that extended rax's AArch64 interpreter
(`src/arm/aarch64/cpu.rs`) toward a bit-exact, hardware-verified implementation
of the full practical ISA, and fixed several **fundamental, previously-untested
bugs** in scalar FP and floating-point NaN handling.

All work was verified against the **qemu-aarch64 differential oracle**
(`tools/arm-diff/` + `tests/arm_diff.rs`, pinned to VL=128) — every commit is
bit-exact vs hardware semantics, not merely "does not crash."

- **14 commits** (`bc1eaf4` … `a8a705c`), all `feat/fix(aarch64)`.
- The `arm_diff` suite grew **180 → 196 families**, every one green.
- 16 new differential test functions added (listed at the end).

---

## Methodology: the coverage probe

The session ran a **loop-until-dry coverage probe**, the technique that has
repeatedly found gaps after the ISA was prematurely declared "complete":

1. Assemble a batch of diverse mnemonics with `llvm-mc`
   (`/home/null/local/bin/llvm-mc --mattr=+sve2,+fullfp16,+i8mm,…`), which gives
   the **exact** encodings (sweep index/register values to triangulate per-size
   bit packing).
2. Convert to `u32` and run each through `run_batch` against the oracle with
   **random *and* special-value** input states.
3. Classify each divergence:
   - *"hw executed but rax rejected"* → a **decode gap** (unimplemented or
     mis-routed encoding).
   - *"rax = X, hw = Y"* → a **silent-wrong-answer** (computed but incorrect).
4. Fix, re-probe, repeat until a round finds nothing; then promote the probe to
   a permanent test.

**The crucial refinement this session:** feeding **special FP values**
(`±inf`, `qNaN`, `sNaN`, denormal, `−0`, max-normal — tiled into 128-bit
operands) alongside random data. "No `Undefined` for valid encodings" and
"passes on clean inputs" are *necessary but not sufficient*: the FP→int
conversion group and the whole FP NaN surface looked complete and passed every
existing test, yet were silently wrong (or rejected) the moment a NaN, infinity,
or invalid operation was involved.

---

## Results by area

### New SVE / SVE2 instruction support

| Commit | Area |
|---|---|
| `bc1eaf4` | SVE FP compares→predicate (register `FCMGE/GT/EQ/NE/UO`, `FACGE/GT`; compare-with-zero `FCMGE/GT/LT/LE/EQ/NE`); integer CMP-immediate (signed/unsigned); BFCVT. **FP compares write only the predicate — they do *not* set NZCV** (unlike integer compares); `FCMNE#0` is `sub=11,bit4=0`, and `(sub=10/11,bit4=1)` are unallocated. |
| `4985486` | Saturating `SQINCP/UQINCP/SQDECP/UQDECP` (GPR 32/64-bit + per-element vector) — ports of qemu `do_sat_addsub_{32,64}` and the `sve_{s,u}q{add,sub}i` element helpers. |
| `4f995f2` | `WHILEGT/WHILEGE/WHILEHI/WHILEHS`. Unlike the `lt`-family, these anchor the active run at the **top** of the predicate (high-numbered elements) per qemu `do_whileg`, and the equality sense is inverted (`eq` bit 0 ⇒ GE/HS, 1 ⇒ GT/HI). |
| `a250a67` | `SABA/UABA` (same-width absolute-difference-accumulate; widened-precision abs so `INT_MIN/INT_MAX` don't overflow). |
| `fe354e0` | Predicated shift-by-immediate `ASRD/SQSHL/UQSHL/SRSHR/URSHR/SQSHLU`. The handler only decoded `bits[18:16]`, which can't distinguish `ASRD (000_100)` from `SRSHR (001_100)`; now decodes the full `bits[21:16]` and widens the dispatch to `bits[21:20]==00`. |
| `cee0cc5` | **The entire SVE element-count / stack-allocation family** — `RDVL/ADDVL/ADDPL`, `CNTB/H/W/D`, `INCB/DECB…` + `SQINCB/UQINCB…` (GPR 32/64 + vector) by pattern — plus `PTEST`. (See *Fundamental bug #2* below.) |
| `43f6133` | Predicate `SEL/MOV`, the flag-setting predicate logicals (`ANDS/BICS/EORS/ORRS/ORNS/NORS/NANDS`, `MOVS`), and `RDFFRS`. (See *Fundamental bug #3* below.) |
| `83e7470` | SVE2 `CDOT` by indexed element (`CDOT_zzxw`). |
| `10702c1` | SVE predicated `FMULX` (`opc5 == 01010`, was `Undefined`). |

### Floating-point correctness (the heart of the session)

| Commit | Area |
|---|---|
| `3114f82` | `BFDOT/BFMMLA` made bit-exact (see *FP topic A*). |
| `10702c1` | ARM-correct NaN handling across **all** SVE/NEON f32/f64 FP ops (see *FP topic B*). |
| `8509c19` | f16 fused-multiply-add NaN ordering. |
| `ab825b3` | NEON `FRECPS/FRSQRTS/FMLS` NaN, the full `FPRecipEstimate` ASL, and scalar-only `FRECPX` (see *FP topic C*). |
| `db6f090` | Scalar FP NaN, `FCVT` precision change, and the **FP↔integer conversion group** (see *Fundamental bug #1*). |
| `a8a705c` | Scalar `FCMPE`-with-`#0.0`, half-precision scalar `FCMP/FCMPE`, and one-source `FSQRT/FRINT/FABS/FNEG`. |

---

## The three fundamental bugs

These were the highest-impact finds: basic operations that *looked* implemented
and passed every existing test, but were broken because they had never been
differentially tested with the right inputs.

### #1 — The FP↔integer conversion group was mis-decoded (`db6f090`)

`FCVTZS/FCVTZU/FCVTNS/…/SCVTF/UCVTF` between FP registers and **general-purpose
registers** — used by *every* float↔int cast in compiled code — were broken.

The encoding is `sf 0 0 11110 ptype 1 rmode opcode 000000 Rn Rd`, where
`rmode = bits[20:19]` and `opcode = bits[18:16]`. Both existing handlers gated on
`bits[18:17]` (which is part of the **opcode**, not the rmode), so:

- `FCVTZS` (`rmode=11, opcode=000`) had `bits[18:17]==00`, so it was caught by the
  "SCVTF handler" and **executed as int→float**;
- `SCVTF` (`opcode=010`) had `bits[18:17]==01`, matching **neither** handler;
- and the `FMOV`-general handler shared the same outer gate and `_ =>`-rejected
  every non-FMOV opcode, so the whole group fell through to `Undefined`.

Fixed by rewriting it as one handler keyed on `rmode`+`opcode`, with the correct
per-mode rounding (`N`=ties-even, `P`=ceil, `M`=floor, `Z`=trunc, plus ties-away
`FCVTA*`), saturation, and `NaN → 0`; and tightening the `FMOV` gate to opcode
`11x`.

> **Lesson:** even "obviously implemented" fundamental ops can be silently broken
> if never differentially tested — the handlers looked complete but decoded the
> wrong bitfield.

### #2 — The SVE element-count / stack-allocation family was missing/misrouted (`cee0cc5`)

`RDVL/ADDVL/ADDPL` share `bits[15:13]==010` with `INDEX` (differing only at
`bit12`) and were being **executed as `INDEX`** — writing a Z register instead of
the GPR. The element-count forms (`CNTB/H/W/D`, `INCB/DECB…`, `SQINCB/UQINCB…`)
had only a stub that ignored the pattern. A coverage probe of the count/scale
category flagged **21 of 22** instructions.

Fixed with a dedicated `exec_sve_elem_count`, routed **before** `INDEX`:
stack forms (`bits[15:11]==01010`, reg 31 = SP), and count forms
(`bits[15:14]==11`; `bits[13:12]` = `00` vector / `10` GPR-plain / `11` GPR-sat),
reusing `sve_pattern_count` and the new `sat_addsub_*` helpers.

### #3 — Flag-setting predicate logicals never set NZCV; SEL missing (`43f6133`)

The predicate-on-predicate logical handler was missing `SEL`
(`(bit23,o2,o3)=(0,1,1)` fell through to `Undefined`) and ignored the S-bit, so
the flag-setting forms (`ANDS/BICS/EORS/ORRS/ORNS/NORS/NANDS` and the `MOVS`
alias) computed the right predicate but **never set NZCV**. Added `SEL`
(`Pd = Pg ? Pn : Pm` per bit, non-zeroing, no flags) and
`NZCV = PredTest(Pg, Pd)` for the S-forms; `RDFFRS` is `RDFFR` predicated with
the S-bit.

---

## Floating-point topics

ARM's IEEE-754 behavior differs from native Rust/x86 arithmetic in ways that only
surface with special operands — which is precisely why the existing FP suites
(clean inputs only) never caught them.

### A — BFDOT / BFMMLA (`3114f82`)

The BF16 2-way dot product (`FPCR.EBF==0`, the qemu-user default) was computing
products exactly in f64 and rounding only the final sums with plain round-to-odd.
That diverged from hardware in four ways:

- overflow rounded to the max finite value instead of `±∞`
  (**round-to-odd vs round-to-odd-*inf***);
- generated NaNs kept a sign instead of the default NaN `0x7FC00000`
  (default-NaN mode is forced for these ops);
- denormal inputs/results were not flushed to zero;
- each bf16 product was not individually rounded to f32, so an overflowing
  product never became `∞` before the sums, missing the `∞ + −∞ → NaN` cases.

Reimplemented as a faithful port of qemu `bfdotadd`: two f32 multiplies and two
f32 adds, each under round-to-odd-inf / FTZ / default-NaN. Products are exact in
f64 (widened bf16 has ≤8-bit significands); the adds use a **Knuth 2Sum plus an
f64 round-to-odd step** so the sticky bit survives even when operands differ by
more than f64 precision, making the final f32 round-to-odd exact.

### B — Default-NaN / FPProcessNaNs across all f32/f64 ops (`10702c1`)

Native Rust arithmetic on x86 produces the x86 "indefinite" NaN `0xFFC00000` for
invalid operations; ARM (`FPCR.DN=0`) produces the **default NaN `0x7FC00000`**
and propagates a NaN input **quieted, with its sign and payload preserved**.

Fixed `fp_three_same_f32/f64` (and therefore the predicated, indexed and
fused-multiply-add paths) with `FPProcessNaNs2/3` + default-NaN-on-invalid, plus
the per-op subtleties:

- `FABD` clears the propagated NaN sign (it is an absolute value);
- `FMAXNM/FMINNM` propagate an sNaN but let a number beat a *lone* qNaN;
- `FRECPS/FRSQRTS` negate `op1` **before** `FPProcessNaNs` (`FPRecipStepFused`),
  flipping the propagated NaN sign — applied to `sve_recps/rsqrts` and
  `fp16_recps/rsqrts`;
- `FMULX` `inf×0 → ±2.0` uses `sign(x) ⊕ sign(y)`, not a `copysign` chain;
- `FMSUB/FNMADD/FNMSUB/FNMUL` flip the product/addend (and hence its NaN) sign.

The new helpers are `is_nan32/64`, `is_snan32/64`, `fp32/64_nan2/nan3`, and
`fp_convert_nan` (FPConvertNaN). `FCVTX` deliberately keeps the existing
`round_odd_f64_to_f32` (max-finite-on-overflow + signed NaN, governed by
`FPCR.DN` rather than the forced default-NaN here).

### C — Reciprocal estimate / exponent (`ab825b3`)

- NEON `FRECPS/FRSQRTS` now delegate to `sve_recps/rsqrts`, gaining the **fused**
  single-rounding formula (the old non-fused `2.0 − x*y` was up to 1 ULP off)
  and the FPNeg-first NaN sign.
- `FPRecipEstimate` (`FRECPE`) rewritten to the full ASL (`arm_defs.asl ~6082`):
  tiny inputs (`|x| < 2^-128 / 2^-1024`) overflow to `±∞`, denormal inputs are
  normalized, and large inputs produce a **denormal output** (`result_exp ≤ 0`)
  instead of wrongly wrapping the exponent to infinity.
- `FRECPX` is **scalar-only** in AArch64 — it was falling through to a vector path
  that wrote every lane; now writes lane 0 only and zeroes the rest.

### D — f16 and scalar FP details (`8509c19`, `a8a705c`)

- `fp16_mla/mls` processed NaNs in `(op1, op2, addend)` order; ARM `FPMulAdd` uses
  `(addend, op1, op2)`.
- The scalar FP-compare gate excluded `bits[4:3]==11`, wrongly rejecting `FCMPE`
  with the `#0.0` form; half-precision scalar `FCMP/FCMPE` and one-source
  `FSQRT/FRINT*/FABS/FNEG/FMOV` were `Undefined` — all added.
- Scalar `FCVT` (precision change between h/s/d) was `Undefined`; added with RNE
  and `FPConvertNaN`.

---

## Key techniques (reusable)

- **`llvm-mc` for exact encodings.** Hand-decoding the SVE bit packing is
  error-prone; assemble the mnemonic and read the bytes. Sweep index/register
  operands to triangulate per-element-size fields.
- **qemu source is the ground-truth oracle.** Read the `trans_*` / `HELPER(*)`
  functions in `/models/dev/qemu/target/arm/tcg/{translate-sve.c,sve_helper.c,
  vec_helper.c}` and `vec_internal.h` for exact semantics — qemu *is* the oracle,
  so its source beats guessing or the rendered ARM-ARM pages. The ARM ASL
  (`docs/architecture/arm/asl/`) is authoritative for the estimate/convert
  helpers.
- **Special-value FP probing.** Tile `±inf / qNaN / sNaN / denormal / −0 /
  max-normal` into 128-bit operands. This is what exposed the BFDOT round-to-odd
  bug, the cross-cutting default-NaN bug, and the FP→int decode bug.
- **`FPSR.QC` is not compared** by the harness, so saturating ops needn't track it.
- **Round-to-odd is double-rounding-safe**: rounding to f64 with round-to-odd and
  then to f32 with round-to-odd equals direct round-to-odd, which is what makes
  the 2Sum-based BFDOT add exact.

---

## Process / workflow hazards (multi-agent repo)

A second agent expands x86/hexagon concurrently in the same tree, so:

- **Commit with a pathspec from the main tree only:**
  `git -C /models/dev/rax commit -F /tmp/msg.txt -- src/arm/aarch64/cpu.rs tests/arm_diff.rs`.
  Never `red --run`/`git add -A` (it sweeps the other agent's files into your
  commit).
- **Build/test in an isolated worktree** (`/tmp/rax-arm-iso`) so the other agent's
  uncommitted WIP doesn't break the lib — but **never** combine
  `cd /tmp/rax-arm-iso && cargo test` with a `git commit` in one shell command
  (the `cd` persists and the commit lands in the iso worktree's detached HEAD).
- `git stash` is shared across worktrees — never use it for A/B benchmarking.

---

## Remaining work

One niche edge case is documented and deferred:

- **FCMLA by indexed element (.h) with NaN/inf inputs** (~1/19 special cases):
  the complex multiply's invalid-op NaN canonicalization through the
  `fp_muladd_bits` / `fp_neg_bits` intermediates produces a wrong intermediate
  NaN (`0x7f80` vs the f16 default `0x7e00`). All other FCMLA/FCADD/CMLA forms
  and non-NaN FCMLA-indexed are bit-exact. Low priority (FCMLA with NaN operands
  is exotic).

Everything else in the probed surface — SVE/SVE2 data-processing, predicate,
FP-convert and FP↔int, the memory subsystem, NEON (three-same, two-reg,
estimates, crypto, dot, complex, bf16), and scalar FP in all precisions — is
bit-exact vs the qemu-aarch64 oracle.

---

## Differential tests added this session

`tests/arm_diff.rs` (16 functions):

```
diff_sve_fp_cmp            diff_sve_cmp_imm           diff_sve_bfcvt
diff_sve_sincdecp          diff_sve_while_gt          diff_sve2_saba
diff_sve2_shift_imm_sat    diff_bf16_dot              diff_sve_elem_count
diff_sve_pred_logical      diff_sve_fp_specials       diff_sve2_cdot_indexed
diff_sve_fp16_fma_specials diff_neon_fp_specials      diff_scalar_fp
diff_scalar_fp_cmp
```

Run with `cargo test --release --test arm_diff` (self-skips if qemu/cross-gcc
are absent). See `memory/rax-arm-diff-oracle.md` for the full running log of the
oracle harness and all prior milestones.
