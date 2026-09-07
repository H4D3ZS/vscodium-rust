# MTP tuning for Qwen3.8-27B on the kortex inference path

Distilled from **[github.com/sudoingX/qwen38-mtp](https://github.com/sudoingX/qwen38-mtp)**
(Apache-2.0) — a community record of ~50 MTP configs across a decade of GPUs.
`probe.py` and `serve_mtp.sh` are vendored from it.

## The one fact

`Qwen/Qwen3.8-27B` ships a **multi-token-prediction head inside the GGUF**
(`blk.*.nextn.*` tensors, kept by unsloth / AtomicChat / etc.). Upstream
llama.cpp loads it and, without a flag, ignores it. Turn it on:

```
--spec-type draft-mtp --spec-draft-n-max 2 --parallel 1
```

The big model verifies every drafted token, so **output is unchanged** — this
is pure decode speed. No custom build (MTP merged in llama.cpp PR #22673), no
draft model, no conversion.

## Numbers on 16 GB RDNA (the cards kortex targets)

| Card | Model | n-max | Decode | Notes |
|---|---|---|---|---|
| **RX 9060 XT 16 GB** | AtomicChat Qwen3.8-27B **IQ3_XXS** | 2 | **~28.7 tok/s** | ROCm build; `n-max 2` won on complex tasks. n-max 3–4 ≈ 30–31 but lose on prose. |
| RX 7900 GRE 16 GB (packed 96 %) | IQ3_XXS + turbo KV, 90K ctx | 3, p-min 0.75 | **~47.8 tok/s** | live 8-turn agent session, acceptance ~0.93. Spec-off on the same card: ~28.7 → **+54 %**. |
| 2× RX 9070 16 GB (pool) | UD-Q4_K_XL, 262K ctx | 2 | 22.1 → **41.6** | +88 %. n-max 4 + `p-min 0.60` → 42.9. |

IQ4_XS (~14.5 GB) fits a 16 GB card only with a small context; **IQ3_XXS is
the community pick for Qwen3.8-27B + a real agent window on 16 GB.** It's
~3.06 bpw — a real step up from the 2.5-bit ROCmFP2 Escha build, and it's the
*instruct* model, not a base or a custom quant.

## The rules that actually matter here

1. **n-max is card-dependent.** 16–24 GB cards peak at **2** (3 for pure
   code); deeper loses on prose as acceptance decays. Re-sweep after any
   config change.
2. **`--spec-draft-p-min` helps starved cards, hurts fast ones.** On a packed
   16 GB card `~0.75` at n-max 3 keeps rejection cheap; `0.60` just buys
   noise. On a 2×9070 pool `0.60` made n-max 4 free. Sweep it, don't adopt it.
3. **MTP is a single-stream optimization.** Gone by `--parallel 4`; a
   `--parallel 2` *baseline* reads ~20 % low and inflates the delta. Always
   `--parallel 1` for both arms. (kortex's launcher forces `--parallel 1`
   whenever `draft-mtp` is selected.)
4. **KV quant is not optional past ~90K ctx.** `--cache-type-k q4_0
   --cache-type-v q4_0` — without it, context creation fails next to 14–17 GB
   of weights.
5. **Rebuild llama.cpp before tuning.** This arch's kernels are young; a
   current build was +10–15 % on every quant before any flag.
6. **A shared desktop halves everything silently** — weights spill to host
   RAM over PCIe, `/health` stays green. Bench headless or check
   `mem_info_gtt_used`.

## Where kortex applies this

`kortex_gac/launcher.rs` → `build_argv`:
- `spec_type` includes `draft-mtp` → emits `--parallel 1` automatically.
- `draft_max` → `--draft-max` (default **2** for MTP, set in
  `gac-orchestrator.ts::specDecodeExtras`).
- `draft_p_min` → `--spec-draft-p-min` (from `kortex.spec.pMin`; unset = ungated).
- The *Kortex ROCmFPX* panel's Speculative-decoding picker selects the type;
  KV quant is already `q4_0` in the panel's default launch args.

## Probe it

```
python3 tools/mtp/probe.py http://127.0.0.1:8081      # spec-off serve
python3 tools/mtp/probe.py http://127.0.0.1:8081      # again with draft-mtp
```

The paired median delta is the number. The panel also shows live acceptance
(`parseSpecAcceptance` off the server log).
