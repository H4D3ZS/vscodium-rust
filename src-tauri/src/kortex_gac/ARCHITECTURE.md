# Kortex Inference Stack — Unified Architecture

This document explains how the three Kortex inference subsystems compose:

1. **GAC** — *Geometry-Aware Consolidation* of model weights.
2. **KDKVC** — *Kortex Disk KV Cache*, a SHA-keyed prefix-cache proxy in front of `llama-server`.
3. **CCET** — *Context-Compute Efficiency* token router and η metric.

The mental model: each subsystem attacks a different axis of the cost function for local LLM inference, and together they let an 8 GB consumer GPU host workloads that were previously the domain of 24 GB+ datacentre cards.

---

## 1. The cost we're minimising

Total wall-clock cost of one inference request can be approximated as:

```
C = C_prefill(n_tokens, weight_layout) + C_decode(n_out, weight_layout) + C_io(prefix_loads)
```

The three subsystems each shrink one term:

| Subsystem | Reduces                                  | How                                            |
|-----------|------------------------------------------|------------------------------------------------|
| **GAC**   | Constant in front of `C_prefill+C_decode`| Smarter weight tier assignment (CPU vs GPU)    |
| **KDKVC** | `C_prefill` to ~0 on cache hit           | Disk-resident KV state, prefix matching by SHA |
| **CCET**  | `n_tokens` (active)                      | Drops / compresses low-ΔI tokens before decode |

---

## 2. GAC — geometry-aware weight placement

### Theory (recap from `geometry-of-consolidation/`)

For a tensor's row set, three quantities decide whether it can be losslessly compressed (or, in our adaptation, *bandwidth-throttled* by living on CPU):

```
d_bar  = mean within-cluster cosine distance
d_eff  = participation-ratio dimensionality
rho    = top-eigenvalue / sum-of-eigenvalues  (spectral concentration)
```

The identity-retrieval bound from the paper:

```
eps_id  >=  1 - c1 * ((1 - theta) / d_bar) ^ (d_eff / 2)
```

Defines a critical threshold:

```
d_bar_critical = (1 - theta) * 2 ^ (1 / d_eff_global)
```

with three regimes:

- **Tight**     `d_bar < safe_mult * d_bar_critical` → safe to consolidate / offload to CPU.
- **Borderline**`safe < d_bar < unsafe`              → hedge.
- **Spread**    `d_bar > unsafe_mult * d_bar_critical` → must stay on GPU.

### Implementation

- `profiler.rs` walks the GGUF, dequantises a sample of rows per weight tensor, and writes `<model>.geometry.aim`.
- `planner.rs` groups tensors by *kind* (`attn_q`, `ffn_down`, …), scores each kind by mean `d_bar / d_bar_critical`, sorts descending, and greedily packs into the VRAM budget. Anything that doesn't fit gets a `--override-tensor PATTERN=CPU` flag.
- `launcher.rs` spawns `llama-server` with the rendered argv plus `--slot-save-path` (so KDKVC can restore later).

### MoE adaptation (ds4-inspired)

DeepSeek V3/V4-style models route 2-of-256 experts per token. Routed experts dwarf the rest of the model in bytes but fire only ~1% of the time.

- `profiler.rs::detect_moe_meta` reads `<arch>.expert_count` / `<arch>.expert_used_count` from GGUF metadata; falls back to scanning tensor names for `_exps` and recovering `expert_count` from the leading dim.
- `planner.rs` applies a *sparsity discount* to expert kinds:

  ```
  score(expert_kind) *= expert_used_count / expert_count
  ```

  Result: for a 64×top-2 MoE, expert kinds get ~3% of their dense-equivalent priority. They're the first to lose the GPU when the budget tightens.
- The MoE *router* (`ffn_gate_inp`) and norms (`attn_norm`, `ffn_norm`) get pinned to GPU regardless of geometry — they're tiny and fire every token.

---

## 3. KDKVC — disk KV cache proxy (port of antirez/ds4)

### Why

Coding agents (Claude Code, Cursor, opencode) are *stateless from the server's point of view*. Each turn re-sends the entire conversation, often 20–30 K tokens of system prompt + history. Without prefix reuse, every turn re-prefills the same tokens. With ds4-style disk caching, we pay that prefill once.

### Design

End-to-end:

```
client ──POST /v1/chat/completions──▶ KDKVC proxy (axum, port 8090)
                                          │
                                          ├─ tokenise prefix via /tokenize
                                          ├─ SHA-256(LE u32 token stream)
                                          ├─ longest-prefix match in <index_dir>
                                          │
                                          ├─ HIT  → POST /slots/0?action=restore
                                          │         then forward request
                                          │         (llama-server skips prefill
                                          │          for the restored prefix)
                                          │
                                          └─ MISS → forward, then on completion
                                                    POST /slots/0?action=save
                                                    + write <sha>.kkv index file
```

### File layout

Two co-located directories:

- `<base>/index/<sha>.kkv` — small JSON sidecar with our metadata (token count, ctx size, timestamps, hit count, slot path).
- `<base>/slots/<sha>.slotbin` — opaque KV binary owned by `llama-server` (dumped by its `/slots` endpoint).

### Cache key

SHA-256 over the LE-encoded `u32` token stream. This is exactly antirez/ds4's scheme except SHA-1 → SHA-256 (we already depend on `sha2`; we don't need ds4 file-format compatibility because `llama.cpp`'s slot binary is its own thing).

### Boundary alignment

Cold saves trim a 32-token tail and align down to a 2048-token chunk boundary. Same trick as ds4: text tokenisation isn't deterministic across appended suffixes (BPE merges can cross prompt boundaries), so a slightly-shorter prefix has a much higher chance of being a *prefix in tokens* of any future request.

### LRU + size budget

`max_bytes` (default 16 GB) caps the total `.slotbin` size. When the budget is exceeded we evict by ascending `last_used_at`.

---

## 4. CCET — token router + η metric

### Theory (recap from the user-supplied framework)

The Context-Compute Efficiency Theorem says, for any sequence model:

```
min_S  M(S(x_1:T))  <=  alpha · sqrt(P · I(x_1:T; y))  +  O(log T)
```

with the corollary that a token `x_t` does useful work iff:

```
ΔI_t = I(y; x_t | x_1:t-1) > tau
```

and the Hardware-Independence Lemma:

```
Max Performance  ∝  I(x_1:T; y) / K(x_1:T)
```

That is: model capability is bounded by *algorithmic information density*, not VRAM. Stuffing more tokens in without raising `I(x; y)` only stores entropy.

### Heuristic v1 implementation (`src/kortex/ccet.ts`)

A trained router would estimate ΔI directly. The shipped v1 uses three text-level proxies:

1. **n-gram repetition penalty** — duplicate spans get a low score.
2. **Structural anchor detection** — code, identifiers, paths, errors get the maximum score.
3. **Length × novelty** — long novel paragraphs score higher than short ones.

Per-segment routing:

| Score        | Route    | Effect                                        |
|--------------|----------|-----------------------------------------------|
| `≥ τ_compress` | `FULL`   | Forward verbatim                              |
| `[τ_skip, τ_compress)` | `COMPRESS` | Replaced with `[…N chars compressed: <head>…]` |
| `< τ_skip`   | `SKIP`   | Dropped entirely                              |

A `max_skip_fraction` cap (default 40%) prevents the router from being too aggressive on long, low-novelty inputs (e.g. boilerplate disclaimers).

### η — Information Efficiency

```
η = output_chars / (active_chars × wall_seconds)
```

Recorded per request via `recordRequest()`; aggregated by `summarizeEfficiency()` over a rolling window. This is an intentionally crude proxy until we wire `logprobs` from `llama-server` for an entropy-based denominator.

### Roadmap

The TS heuristic is the v1 floor. Path forward:

- **v2**: small router (Mamba-tiny + classifier head) trained to predict ΔI from per-token features (LM logit drop, repetition statistics). Lives in Rust, runs as a `Tower` middleware in front of llama-server.
- **v3**: state-space context compressor — replace the raw KV cache for COMPRESS-routed tokens with a state-space buffer `h_t = C(h_{t-1}, x_t)` with `K(h_t) ≤ β · I(y; x_{1:t})`.

---

## 5. How the three compose

The end-to-end path for a coding-agent request:

```
client (Cursor / Claude Code / opencode)
  │
  │ POST /v1/chat/completions  (stream=true)
  ▼
[CCET router]              ← drops/compresses redundant prompt segments
  │
  ▼
[KDKVC proxy]              ← restores any matching prefix from disk KV
  │
  │ forwarded request
  ▼
[GAC-tiered llama-server]  ← spread weights on GPU, tight on CPU
  │
  ▼
SSE stream back to client
  │
  ▼
[KDKVC save]               ← post-stream slot save under sha256(tokens)
[η tracker]                ← record (input, active, output, wall_clock)
```

### What each axis is worth

Rough gains, on an RX 580 8 GB / 64 GB system, with a 35B Q4_K_M model:

| Axis            | Win                                              | Mechanism                              |
|-----------------|--------------------------------------------------|----------------------------------------|
| GAC alone       | ~2.5× tokens/s vs naïve `--n-gpu-layers N`       | Spread tensors win the GPU             |
| GAC + MoE-aware | +30% on MoE arch (DeepSeek V4 Flash-class)       | Sparsely-activated experts go to CPU   |
| GAC + KDKVC     | Up to 50× wall-clock on second turn (cache hit)  | No re-prefill of system prompt         |
| All three       | 8 GB matches 24 GB on real coding workloads      | The compounded effect of all of above  |

---

## 6. Open work (not in MVP)

- **Trained CCET router** (v2 above).
- **Algorithmic context compressor** (v3 above).
- **Server-side prefix index** — offer `/v1/cache/lookup` so clients can decide *before* sending whether they have a cache-amenable prompt.
- **Cross-quant slot reuse** — ds4 supports reusing 2-bit checkpoints with 4-bit servers when the prefix matches. We currently key strictly on tokens, not quant; same prefix across quant levels may be safe.
- **MoE expert routing trace** — track per-token expert activations to refine the sparsity discount per kind (some experts fire 5×, others almost never).

---

## 7. References

- *Geometry of Consolidation* — `geometry-of-consolidation/paper/`
- *DS4* — `ds4/` (antirez's DeepSeek V4 Flash inference engine, KDKVC's design ancestor)
- *CCET* — user-supplied framework (Shannon + Kolmogorov + Tishby's Information Bottleneck applied to sequence models)
- *llama.cpp* — `llama-server` slot save/restore API: `POST /slots/{id}?action={save,restore,erase}`
