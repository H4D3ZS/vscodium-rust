# mojo-knn — a Mojo prototype for the kortex retrieval hot path

**Status: prototype / benchmark. Not wired into the build.**

## Why

kortex's per-request compute on consumer hardware is dominated by two things:
model prefill (handled by the KV-slot cache + prompt-prefix stability + the
context budget), and — for the retrieval proxy — **vector search over the
`.aim` catalog**. That search runs on the async blocking pool under a
~100 ms wall-clock budget; when it overruns, the request goes upstream
*without* context, i.e. you paid for the embed + partial search and got
nothing for it. Making the search 2–5× faster means more requests land
inside the budget → smaller prompts → less prefill → lower cost.

libaim does the search with `turbovec` (SIMD-quantized ANN) plus an f32
re-rank pass (`ivf.rs::dot`, `LiveCatalog::page_fault`). The re-rank /
brute-force top-k over f32 vectors is the cleanest thing to hand to Mojo:
a tight, embarrassingly-parallel dot-product + top-k with a stable C ABI.

Mojo is the right tool here (and *only* here — not for inference; ROCmFPX
already owns that) because:

- it vectorizes and parallelizes the reduction with `vectorize` /
  `parallelize` without hand-written intrinsics per target,
- it compiles to a plain shared library callable from Rust over the C ABI
  (`libaim` can `dlopen` it and fall back to the Rust kernel if absent),
- one kernel covers x86 AVX2/AVX-512 and Apple/ARM NEON.

## Layout

| file | what |
|---|---|
| `knn.mojo` | the kernel: `topk_cosine(...)` + a `main()` that runs the bench binary format for standalone timing |
| `format.md` | the little-endian binary interchange format the bench + a future FFI shim use |
| `bench.py` | generates random unit vectors, runs a NumPy reference (correctness = recall@k, plus timing), invokes the compiled Mojo binary, prints the comparison |
| `reference.rs` | the scalar Rust kernel the Mojo one must match and beat (mirrors `libaim/src/ivf.rs::dot` + a top-k heap) |

## Run it

```bash
# 1. Install the Mojo SDK (https://www.modular.com/max/install). Needs `mojo` on PATH.
mojo --version

# 2. Build the kernel binary
cd tools/mojo-knn
mojo build knn.mojo -o knn

# 3. Benchmark against the NumPy reference
python bench.py --n 50000 --dim 1024 --k 8 --queries 200
#  -> writes bench.bin, runs ./knn, compares recall@8 and median latency
```

Target: **recall@k == 1.0** vs the exact reference (it's exact brute force,
not ANN) and **≥ 2× the throughput** of `reference.rs` compiled `--release`
on the same box. If it doesn't clear 2×, the integration cost isn't worth
it and we keep the Rust kernel.

## Integration path (once the bench clears the bar)

1. `mojo build knn.mojo --emit shared-lib -o libkortex_knn.so` (`.dylib` /
   `.dll` per OS).
2. A thin `libaim` feature `mojo-knn`: `extern "C"` decl of `topk_cosine`,
   `libloading::Library::new` at catalog open, used by `page_fault`'s
   re-rank step; `#[cfg(not(feature))]` or a null-handle check falls back
   to the existing Rust `dot` loop.
3. Ship the compiled lib next to the `aim-vfs` / `aim-proxy` binaries the
   IDE already resolves via `kortex_bin::find_kortex_tool`.

No submodule change until step 2 — this directory is self-contained.
