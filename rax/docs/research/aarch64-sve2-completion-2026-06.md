# AArch64 SVE2 / SVE2.1 Completion Session — 2026-06-04

A differential-testing session that drove rax's AArch64 interpreter
(`src/arm/aarch64/cpu.rs`) to a **bit-exact, hardware-verified** implementation
of the entire *register-data-processing* surface of **SVE2** and **SVE2.1**
(FEAT_SVE2, FEAT_SVE2p1, FEAT_SVE_B16B16, and the SVE-encoded multi-vector and
predicate-as-counter ops).

Every change is verified against the **qemu-aarch64 differential oracle**
(`tools/arm-diff/` + `tests/arm_diff.rs`, pinned to VL=128) — bit-exact vs
hardware semantics, not merely "does not crash."

- **6 commits** (`b07b4ca` … `049606a`), all `feat/fix(aarch64)`.
- A new permanent regression test **`diff_sve2_comprehensive_sweep`** sweeps an
  **879-instruction** `llvm-mc`-generated encoding table (`tests/sve2_gen.rs`)
  through the oracle with random + "interesting" (special-FP-laden) inputs and
  asserts **zero divergences**.
- The full `arm_diff` suite is **197/197 green**.
- Net `cpu.rs`: **+947 / −53** lines.

> The disassembler (`decoder/aarch64.rs::decode_sve`) is SVE1-only and was *not*
> the target. The executing decoder is `exec_sve` in `cpu.rs`, which re-decodes
> the raw instruction word directly; all work below is there.

---

## Methodology: the loop-until-dry coverage probe

This is the same technique the FP/SVE session
(`aarch64-fp-sve-coverage-2026-06.md`) used, refined into a single asserting
sweep test:

1. **Enumerate + assemble.** Hand-build a broad list of SVE2/SVE2.1 mnemonics
   across every element size and key variant; assemble with `llvm-mc` to get the
   **exact** encodings:
   ```
   llvm-mc --arch=aarch64 \
     --mattr=+v9a,+sve2,+sve2p1,+sve-b16b16,+sve2-bitperm,+i8mm,+bf16,\
+sve2-aes,+sve2-sha3,+sve2-sm4,+fullfp16,+f32mm,+f64mm,+sme2 --show-encoding
   ```
   Convert the `[b0,b1,b2,b3]` bytes to a little-endian `u32` and append to
   `tests/sve2_gen.rs` (`pub static SVE2_SWEEP: &[(&str, u32)]`).
2. **Probe.** `diff_sve2_comprehensive_sweep` runs every encoding through the
   oracle with random + `interesting()` inputs (which tile `0`, `-1`, `1`,
   `0x8000…`, `0x…8000_0000`, and random — i.e. `±0`, `±Inf`, qNaN/sNaN, denormal
   patterns into the 128-bit Z lanes and all 16 predicates), and **classifies**
   each mnemonic:
   - *decode-gap* — hw ran the encoding, rax returned `Undefined`/error;
   - *value-mismatch* — rax computed a wrong answer;
   - *fault-disagree* — hw trapped but rax executed.
3. **Fix** against ground truth, **re-probe**, repeat until a round is clean,
   then keep the encodings in the table permanently.

**Ground truth = qemu source**, not the rendered ARM-ARM:
qemu's `target/arm/tcg/{sve.decode,sve_helper.c,vec_helper.c,
sme_helper.c,vec_internal.h}`. The decode patterns disambiguate the bit fields;
the `HELPER(...)` / `trans_*` functions give the exact arithmetic, NaN ordering,
saturation, and identity values.

**Why "interesting" inputs matter.** Several finds were *silent wrong answers*
on clean-looking ops that already passed every prior test — they only diverged
on a NaN, a `−0.0`, or a saturation edge (see the three fundamental bugs below).

---

## Results by area

### Commit `b07b4ca` — missing SVE2 ops + FMA/NaN fixes (the seed batch)

The 778-encoding probe immediately found **22 decode-gaps + 5 value-mismatches**:

| Instruction(s) | Kind | Fix |
|---|---|---|
| `SADDLBT`, `SSUBLBT`, `SSUBLTB` | gap | New arm at `0x45 / bits[15:12]==1000`. Signed add/sub long where the two narrow halves come from **different** positions — qemu's `sel`: `{2,2,1}` = (Zn-bottom + Zm-top), (Zn-bottom − Zm-top), (Zn-top − Zm-bottom). |
| `SQRDCMLAH` (indexed) | gap | Extended the indexed-CMLA arm (`bits[15:13]==011`, bit12 = sat) with the saturating-rounding-doubling-high path of the (already bit-exact) non-indexed `SQRDCMLAH`. |
| `BFCVTNT` | gap | Added `(opc,opc2)=(10,10)` to the FCVTNT/FCVTLT top-variant handler; narrows f32→bf16 into the odd half via the verified `f32_to_bf16`. |
| `BFMLSLB/T` (vector + indexed) | gap | The handler was rejecting bf16-subtract as "needs SVE2p1"; removed the guard — BFMLSL is just BFMLAL with a negated Zn (XOR `0x8000`, the FPCR.AH=0 form). |
| `BFMLALB/T`, `FMLALB/T` (vector + indexed) | **value** (NaN) | Replaced Rust's `f32::mul_add` with `fp_muladd_bits` (= `float32_muladd`). x86 FMA picks a *different* input NaN than ARM's `FPProcessNaNs3`, which processes the **addend first** — so an accumulator NaN must win. |
| `FCVTXNT` (and, latently, `FCVTX`) | **value** (NaN) | `round_odd_f64_to_f32` was canonicalising every NaN to the default `0x7FC0_0000`; ARM at `FPCR.DN=0` **preserves** sign + top-23 payload and forces the quiet bit (FPConvertNaN). The old FCVTX test only used finite inputs, so it never caught this. |

This commit also added the permanent harness: `diff_sve2_comprehensive_sweep`
+ `tests/sve2_gen.rs`.

### Commit `edb4337` — SVE2.1 quadword / clamp / per-segment permute

The next probe round (with the SVE2.1 set added) found **25 gaps + 3 mismatches**:

- **Integer quadword reductions** `ADDQV / SMAXQV / UMAXQV / SMINQV / UMINQV /
  ANDQV / ORQV / EORQV` (`exec_sve_qv_reduce_int`). Discriminator vs the scalar
  reductions: same `bits[15:13]==001` but `bit18==1`. A quadword reduction
  collapses each element *position* across the 128-bit segments of Zn into one
  quadword in Vd; at VL=128 (one segment) that is `Vd[e] = active ? Zn[e] :
  identity` — identity per op: ADD/OR/EOR/UMAX = 0, SMAX = INT_MIN, SMIN =
  INT_MAX, UMIN/AND = all-ones.
- **FP quadword reductions** `FADDQV / FMAXNMQV / FMINNMQV / FMAXQV / FMINQV`
  (`bits[21:19]==010, bits[15:13]==101` in `exec_sve_fp_pred`). *Mismatch on the
  first try:* I combined the active lane with the identity (`fp_op(ident, x)`),
  which normalised `−0.0`→`+0.0` and quieted input NaNs. **Fix:** at VL=128 a
  single-element column reduction is the **raw** element — `Vd[e] = active ?
  Zn[e] : identity`. (Identities: FADD `+0.0`, FMAX `−Inf`, FMIN `+Inf`,
  FMAXNM/FMINNM default-NaN — `sve_fp_identity`.)
- **Clamp** `SCLAMP / UCLAMP` (`0x44 / bits[15:11]==11000`, bit10 = unsigned):
  `Zd = min(max(Zd, Zn), Zm)`.
- **Per-segment permute** `ZIPQ1/2`, `UZPQ1/2`, `TBLQ` (`0x44 / bits[15:13]==111`)
  — at VL=128 (one 128-bit segment) these coincide with `ZIP/UZP/TBL`.
- **`DUPQ`** (broadcast the indexed element of each 128-bit segment; tsz-encoded
  index), **`EXTQ`** (extract within the segment; 4-bit byte offset), **`REVD`**
  (swap the two 64-bit doublewords of each segment, predicated/merging on the
  low-doubleword bit) — all in the `0x05` permute space; previously *mis-decoded*
  by the regular DUP/EXT handlers (hence the 3 mismatches).

### Commit `608673d` — FEAT_SVE_B16B16 + dot products

Probe found **16 gaps + 1 mismatch**:

- **bf16 arithmetic** `BFADD/BFSUB/BFMUL` (unpred 3-same + predicated),
  `BFMAX/BFMIN/BFMAXNM/BFMINNM` (predicated), `BFMLA/BFMLS` (predicated FMA),
  indexed `BFMUL/BFMLA/BFMLS`, and `BFCLAMP` (the mismatch — FCLAMP had been
  running it as fp16; bf16 needs its own path). All routed through
  `try_exec_sve_bf16`, which intercepts the size==00 encoding slots before the
  f16/f32/f64 dispatch.
  - `bf16_binop`: widen bf16→f32 (exact 16-bit shift); MUL/MAX/MIN/MAXNM/MINNM
    and any NaN/Inf go through the verified ARM `fp_three_same_f32` then a single
    `f32_to_bf16` narrow (a bf16 product is exact in f32; MAX/MIN return an
    operand). Finite ADD/SUB accumulate the **exact sum in f64** and narrow once
    (round-to-odd → RNE) to dodge f32→bf16 double rounding.
  - `bf16_fma`: exact f64 product + f64 add, narrow once; NaN/Inf via
    `fp_muladd_bits`. BFMLS negates the Zn input (FPCR.AH=0).
- **2-way integer dot** `SDOT/UDOT` (`.s ← .h`, vector + indexed) — distinct
  encoding `0x44 / bits[15:11]==11001`; 2 products per `.s` lane.
- **FP dot** `FDOT` (`.s ← .h`, vector + indexed). `f16_dotadd` is a faithful
  port of qemu: NaN follows `FPProcessNaNs4` (first signalling, else first quiet,
  widened+quieted); finite products are **exact in f64**, summed with a Knuth
  2Sum and rounded **once** to f32 (round-to-odd of the exact sum → RNE narrow is
  double-rounding-safe), then a separate (non-fused) f32 accumulate.

### Commit `67115ae` — PMOV + multi-vector extract-narrow

- **`PMOV`** (`0x05 / bits[21:19]==101, bits[15:10]==001110`, bit16 = direction):
  moves a bit-plane between a vector and a predicate. Predicate bit `e*esize` ⇄
  vector bit `elements*idx + e` (`elements = 16/esize`); the `Z→P` form is a pure
  extract, the `P→Z` form zeroes Zd only when `idx==0` (else merges the plane).
- **`SQCVTN / UQCVTN / SQCVTUN`** (`0x45 / bits[20:16]==10001, bits[15:13]==010`,
  op = bits[12:10]): the only multi-vector data-processing ops in the *SVE*
  encoding space. They read the register pair `{Zn, Zn+1}` and interleave the
  saturated `.s→.h` narrowings: `Zd.h[2i]=sat(Zn.s[i])`, `Zd.h[2i+1]=sat(Zn+1.s[i])`.

### Commit `aedd196` — SQDMLALBT/SQDMLSLBT + PSEL

- **`SQDMLALBT / SQDMLSLBT`** (`0x44 / bits[15:11]==00001`, bit10 = sub): the
  interleaved (Zn-bottom × Zm-top, qemu `sel=2`) saturating-doubling
  multiply-add-long — the doubling-mul analogue of `SADDLBT`.
- **`PSEL`** (`0x25 / bit21==1, bits[15:14]==01`, bit9==0, bit4==0): `Pd = Pn` if
  the `Pm` element at `(Wv + imm) mod elements` is active, else `Pd` all-false.
  The element size + imm are tsz-encoded in bits[23:18]; `Wv = W(bits[17:16]+12)`.

### Commit `049606a` — PEXT (predicate-as-counter)

- **`PEXT_1 / PEXT_2`** (`0x25 / bits[20:16]==00000, bits[15:12]==0111`, bit4==1):
  extract a normal predicate (or pair) from a **predicate-as-counter** PNn
  (= `p[8 + bits[7:5]]`). Ports qemu's `decode_counter` (CounterToPredicate):
  `p_esz = ctz(png)`, `count = (png & (vl·8−1)) >> (p_esz+1)`, `invert = png[15]`,
  with the `p_esz≠v_esz` count adjustment + element stride; then fills the
  `part`-th VL-sized chunk via while-lo / while-hi over the
  `pred_esz_masks[v_esz+stride]` mask. Bit-exact across random counter values and
  all sizes/forms.

---

## The three fundamental bugs

Highest-impact finds — operations that *looked* implemented and passed every
existing test, but were wrong because they had never been differentially tested
with the right inputs.

### #1 — FP fused-multiply-add used the host (x86) NaN, not ARM's (`b07b4ca`)

`BFMLAL/BFMLSL` and `FMLAL/FMLSL` widened their f16/bf16 inputs correctly but
performed the accumulate with Rust's `f32::mul_add`, i.e. x86 `vfmadd`. When more
than one operand is NaN, x86 and ARM select **different** NaNs: ARM `FPMulAdd`
runs `FPProcessNaNs3` in `(addend, op1, op2)` order, so an accumulator NaN must
propagate. Switching to `fp_muladd_bits` (the verified `float32_muladd`) fixed
all four families.

> **Lesson:** "uses an FMA" is not enough — the *NaN-selection order* is part of
> the architecture, and the host FMA gets it wrong.

### #2 — FCVTX/FCVTXNT canonicalised NaNs (`b07b4ca`)

`round_odd_f64_to_f32` returned the default NaN `sign|0x7FC0_0000` for any NaN
input. But FCVTX/FCVTXNT are **not** default-NaN ops — at `FPCR.DN=0` they apply
`FPConvertNaN`: preserve the sign and the top 23 fraction bits, forcing the quiet
bit. The existing `diff_sve2_fcvtx` test fed only `finite_fp_bits`, so a NaN lane
never reached the rounder. Random inputs exposed it instantly
(`rax=0xffc00000` vs `hw=0xffffffff`).

### #3 — FP quadword reduction combined the active lane with the identity (`edb4337`)

A natural-looking `Vd[e] = fp_op(identity, Zn[e])` is **wrong** for a
single-element column: `fadd(+0.0, −0.0)` flips `−0.0`→`+0.0`, and
`fmaxnm(qNaN_ident, x)` quiets/replaces a NaN lane. Hardware leaves a sole active
element **untouched**: `Vd[e] = active ? Zn[e] : identity`. Same shape as the
integer QV reductions — but for FP the distinction is observable.

---

## Key techniques (reusable)

- **`llvm-mc` for exact encodings; qemu source for exact semantics.** Sweep the
  index/size/rotation operands to triangulate per-size bit packing
  (e.g. bf16-indexed `index = bit22:bits[20:19]`, fdot `index = bits[20:19]`).
- **Exact-in-f64 + single narrow beats double rounding.** bf16/f16 inputs widen
  exactly; products are exact in f64; a Knuth 2Sum + round-to-odd → RNE narrow
  reproduces hardware's single-rounding for dot products and bf16 add/sub.
- **At VL=128, "quadword"/"per-segment" ops degenerate to whole-register ops.**
  One 128-bit segment ⇒ `ZIPQ=ZIP`, `TBLQ=TBL`, `DUPQ`=broadcast, the QV
  reductions = masked-Zn-with-identity, `REVD`=swap-halves. rax is fixed at
  VL=128 (`self.v[..]` is a single `u128`), so these are *complete*, not
  simplified.
- **Special-value inputs are mandatory.** Clean-input suites declared FCVTX, the
  BF/F-MLAL family, and FP-QV "done"; only `±0`/NaN/denormal tiling caught them.

---

## Deliberately out of scope (and why — not regressions)

These are *not* part of the verified surface, and were excluded honestly rather
than guessed at:

- **Multi-vector memory** `LD1Q / ST1Q` and quadword gather/scatter — memory
  ops; the differential oracle is register-only. Covered (for the non-quadword
  forms) by the existing `diff_sve_{gather,scatter,ld,st}*` tests.
- **FEAT_LUT** `LUTI2 / LUTI4` — qemu-user's default ("max") CPU *traps* them, so
  they cannot be differentially verified here; implementing them would be
  unverifiable speculation.
- **SME streaming** `RDSVL / ADDSVL / ADDSPL` — these read the *Streaming* vector
  length (an SME concept). The oracle's qemu has SME (SVL=256), so it executes
  them; rax has no SME mode and currently mis-runs them as the SVE
  `RDVL/ADDVL/ADDPL` (a latent SME-decode issue). Out of SVE2 scope and
  unverifiable without an SME model.

---

## Multi-agent / workflow hazards (unchanged)

A second agent expands SMIR concurrently in the same tree, so:

- **Build/test in the main tree but stage only the ARM files** when committing:
  `git add src/arm/aarch64/cpu.rs tests/arm_diff.rs tests/sve2_gen.rs` then
  `red -m --staged --run <those files>`. Never `git add -A` / unscoped `red`
  (it would sweep the other agent's SMIR WIP into the commit).
- `git stash` is shared across worktrees — never use it for A/B benchmarking.

---

## How to run

```
cargo test --release --test arm_diff diff_sve2_comprehensive_sweep -- --nocapture
cargo test --release --test arm_diff            # full suite, 197 families
```

Self-skips if `qemu-aarch64` / the cross-toolchain are absent. See
`memory/rax-sve2-completion.md` and `memory/rax-arm-diff-oracle.md` for the
running log and the oracle harness internals.
