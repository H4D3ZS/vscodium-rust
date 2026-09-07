# Ternary dot — ancient arithmetic, measured

> "find a way, find a math... there is a way."

This is the measured answer. The way is real; here is exactly where it works
and where it doesn't, on a Ryzen 9 3900 (Zen 2, no AVX-512).

## The ancient principle

Every deep compute-efficiency idea is one move: **replace an expensive
operation over a large representation with a cheap operation over a compressed
one.** Ternary weights are the arithmetic edge of it:

- **Rhind papyrus** (~1650 BCE): multiply = repeated doubling + adding.
- **Napier** (1614): logarithms turn multiply into add.
- **BitNet b1.58** (2024): a 1.58-bit LLM whose matmul is add/subtract only.

When vector values are `{-1, 0, +1}`, a dot product has **no multiplies**. Pack
the signs into two bitmasks (`pos`, `neg`) and the dot is pure bit ops:

```
dot = popcount(qpos & vpos) + popcount(qneg & vneg)     // agreements
    - popcount(qpos & vneg) - popcount(qneg & vpos)     // disagreements
```

## What the benchmark measures (`rustc -O bench.rs -o bench && ./bench`)

Against the AVX2+FMA f32 kernel libaim actually ships, on the retrieval shape
(dim-768 unit vectors, 20 000 corpus, exact f32 top-10 as ground truth):

### 1. The arithmetic win is real and data-independent

| | ns/dot | corpus |
|---|---|---|
| f32 AVX2+FMA | ~119 | 58 MB |
| **ternary popcount** | **~44** | **3 MB** |

**~2.7× faster, 16× smaller.** Most of the speed is *bandwidth*: the 3 MB
ternary corpus lives in L3; the 58 MB f32 corpus streams from DRAM. This is
the memory wall being sidestepped, not more FLOPs.

### 2. Whether it holds recall depends entirely on the data

Funnel recall — does ternary's top-N contain the exact top-10?

| data | top-50 | top-100 |
|---|---|---|
| isotropic random (worst case for sign methods) | 0.03 | 0.05 |
| **anisotropic / power-law spectrum (real embeddings)** | **0.998** | **1.000** |

Real text embeddings are anisotropic — a few dominant "rogue" dimensions carry
most of the signal, which is exactly the regime where sign quantization
survives. Isotropic random vectors have per-coordinate signal *below* the
quantization noise (~1/√dim), so signs are noise — a known pathological case,
not a verdict on the method.

## The way, stated plainly

**Coarse-to-fine.** Ternary popcount scan of everything (cache-resident, no
multiplies) → keep the top ~50 → exact f32 AVX2 rerank of those. On realistic
data that recovers the exact top-10 at 0.998 recall, for ~2.7× less work and
16× less memory. This is the same shape as turbovec's quantized-scan-then-rerank
— ternary is a viable, extremely cheap coarse tier.

## Honest limits

- **Synthetic vectors.** The anisotropic model is realistic but not real. Final
  validation needs actual `.aim` embeddings from a running catalog.
- **Naive, not learned.** This is post-hoc ternarization of pretrained f32
  vectors. It wins *because* real embeddings are anisotropic, not because the
  quantizer is smart. The robust production versions either (a) *learn* the
  quantization — turbovec's rotation + codebook PQ, already in the stack, which
  is why the committed catalog doesn't need this — or (b) *train the model*
  ternary-aware (BitNet), where quality is recovered by training, not arithmetic.

## Wired into libaim (opt-in, off by default)

`kortex/libaim/src/ternary.rs` is the production `TernaryVec` (encode + popcount
dot). `DeltaLayer::search` uses it as a coarse-to-fine pre-filter: a ternary
popcount scan narrows to a `k*8` candidate net, then the **exact f32 cosine
reranks** it — so the returned scores are always exact and only the candidate
*set* is approximated.

It is **off by default** (`ternary_min()` returns `usize::MAX`). Enable it with
`KORTEX_TERNARY_MIN_CHUNKS=2048` (the delta size at which it engages), and
`KORTEX_TERNARY_ALPHA` tunes the sign threshold. It ships off because the
default delta embedder is the *sparse feature-hash* one, for which the funnel
is not yet validated — the ~0.998 recall was measured on *dense anisotropic*
embeddings. A test (`ternary_prefilter_matches_exact_top_k_when_engaged`)
proves that, when engaged on suitable data, the coarse path returns the same
top-k as the exhaustive scan.

## Where this belongs

- **Committed catalog:** already covered by turbovec (learned PQ). Don't
  naive-ternarize it.
- **Delta layer** (uncommitted edits, brute-force f32 today): a ternary
  pre-filter would help *if* the delta grows to thousands of chunks. It usually
  doesn't, so f32 is fine for now — wire this only if profiling shows a large
  delta stalling retrieval.
- **Inference:** the real home for ternary is BitNet-style model weights, which
  is a training bet, not a retrieval one.

The arithmetic lever is real (measured here). It must be paired with learning —
it is not a free bolt-on. That is the way, and its price.
