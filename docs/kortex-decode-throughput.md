# Kortex decode throughput — a software path around the bandwidth wall

The goal: push local decode from ~30 tok/s toward 150–300 tok/s on a 16 GB
consumer AMD card (RX 9060 XT, ~322 GB/s), **without new hardware and without a
smaller model**, keeping output bit-identical.

## Why this is a software problem

Decode is memory-bandwidth bound. Each token streams the active weights from
VRAM once. Escha 35B-A3B is MoE with ~3B active params; at ~2.5 bpw
(`Q2_0_ROCMFPX`) that's ~1 GB read per token, so the bandwidth ceiling is
~320 tok/s and real llama.cpp today is ~30. Two independent gaps:

1. **Kernel efficiency** (~30 → ~150): the forward pass isn't saturating the
   bus — launch latency, unfused MoE gather/scatter, no graph replay.
2. **Tokens per weight-read** (>1): speculative decoding makes the big model
   emit N verified tokens per forward pass. Pure software, zero accuracy loss.

Neither needs a bigger GPU. Frontier vendors face the same arithmetic; if a
consumer stack demonstrates the win, the pressure to follow is real.

## Shipped (branch `fix/kortex-wiring`)

### Speculative decoding, end to end

The ROCmFPX llama.cpp fork already implements 10 speculator types
(`common/speculative.cpp`). Now exposed through the launcher and UI:

| `--spec-type` | model? | VRAM? | notes |
|---|---|---|---|
| `ngram-simple` / `ngram-map-k` / `ngram-map-k4v` / `ngram-mod` | none | none | guess continuation from prompt/context — strong on code |
| `ngram-cache` | none | none | + persistent n-gram file (`--lookup-cache-dynamic`), learns the codebase |
| `draft-mtp` | own MTP head (auto) | ~0 | the model's multi-token head |
| `draft-eagle3` / `draft-simple` / `draft-dflash` / `draft-dspark` | separate GGUF | small | needs a draft/EAGLE model |

- `LaunchOpts` (`kortex_gac/launcher.rs`): `spec_type` (comma list, filtered
  against `SPEC_TYPES`), `draft_model_path`, `draft_ngl`, `draft_max`,
  `lookup_cache`. `build_argv` drops unknown names and `draft-*` types with no
  model; stacks the survivors in priority order.
- `kortex_gac_launch` forwards them; `gac-orchestrator.ts` mirrors the shape,
  `specDecodeExtras()` reads persisted `kortex.spec.*`.
- `KortexLocalInferencePanel` → "Model & engine": a Speculative decoding
  picker (Off / lookup / lookup+cache / lookup+MTP / MTP). Takes effect on
  next Start.
- **Acceptance readout**: `parseSpecAcceptance()` reads the fork's per-request
  `draft acceptance = 0.NNN (A accepted / G generated), mean acceptance
  length = M` log line; the panel shows `spec: NN% kept · M tok/step` live, so
  the toggle is measured, not assumed. `mean acceptance length` is the real
  decode multiplier.

Realistic: on IDE traffic (repetitive) `ngram-simple` alone is ~1.5–2.5x free;
`ngram-simple,draft-mtp` more. Adversarial novel text falls back to ~1x — and
that's fine, the interactive path isn't adversarial.

### Compute-cost receipt

`tools/compute-bench/model.py` predicts prefill savings; now there's a
measured counterpart:

- `kortex_kvcache/trace.rs` — `TraceRecord` + best-effort `append()`. The
  proxy's `handle_intercepted` writes one JSON line per request to
  `KORTEX_COMPUTE_TRACE` (no-op when unset): `tokens_in`, `prefix_hit_tokens`,
  `suffix_tokens_processed`, `cache_hit`.
- `tools/compute-bench/reduce_trace.py` — folds a trace into the same
  before/after table and `--json` shape as `model.py`, so measured vs modelled
  can be diffed in CI. Baseline `sum(tokens_in)`, actual
  `sum(tokens_in - prefix_hit_tokens)`. See `real_run.md` for the A/B.

### HIP graph capture

`scripts/build-rocmfpx-windows.ps1` now configures the HIP build with
`-DGGML_HIP_GRAPHS=ON`. The fork's `ggml-cuda.cu` `use_cuda_graph` path
(HIP-mapped in `vendors/hip.h`) replays a decode step as one graph launch
instead of hundreds of kernel launches — launch-latency is a real cost at
low batch on RDNA4. No code change, identical output. Requires the HIP
backend (HIP SDK ≥ 7.2, gfx1200/gfx1201 device bitcode); the Vulkan build
is unaffected.

## Next — belongs on its own branch (`perf/decode-kernels`)

### 1. Flip the default launch to HIP

`KortexLocalInferencePanel` starts with `backend: 'vulkan'`. The
graphs + MMQ path only exists on the HIP backend. Once a HIP `llama-server`
is staged, add a backend toggle (auto-detect: HIP if `amdhip64_6.dll` +
device bitcode present, else Vulkan) and pass `backend: 'hip'` through the
GAC plan. Measure with `llama-bench -m escha… -p 0 -n 128` both ways.

### 2. Fused MoE decode kernel

`mmid.cu` / `mmq.cu` handle `MUL_MAT_ID` (the expert GEMM). At decode
(batch 1, ~8 active experts of many) the gather → per-expert GEMM → scatter
is several launches over strided, partially-resident tensors. Targets:

- One persistent kernel per MoE layer: gather rows for the routed experts,
  do the quantised GEMV, scatter — no host round-trip, no intermediate
  global writes.
- Route-ahead: read layer *N*'s router logits to prefetch layer *N+1*'s
  expert weights while *N* computes.
- Hot-expert residency: keep the top-k most-activated experts (per session /
  per file domain — this is the GAC geometry signal from `kortex_gac`) at
  higher precision and pinned; stream the cold tail. Reduces effective
  bytes/token, stacks with everything above.

Bar: ≥2x decode tok/s on the 35B-A3B at `n_ctx=32k` vs the current Vulkan
path, output unchanged (verify with a fixed-seed diff of 512 tokens).

### 3. KV-cache quantisation on the decode read

For long contexts the KV read rivals the weight read. The panel already
launches with `--cache-type-k q4_0 --cache-type-v q4_0`; evaluate `q8_0` for
K (quality) vs `q4_0`/`iq4_nl` for V, and SnapKV/H2O-style eviction for
history beyond the budget. Measured via the same `reduce_trace.py` plumbing
extended with a decode-bytes column.

## The ceiling, stated honestly

- **100–200 tok/s**: reachable with graphs + fused MoE + `ngram`/`mtp`
  speculation. Engineering, not physics.
- **1000 tok/s** for a 35B: not on 16 GB / 322 GB/s — that's ~1 TB/s of
  weight reads. Needs a sub-1B model (the draft path *can* hit it, and the
  interactive-completion path can run there) or HBM-class hardware.
