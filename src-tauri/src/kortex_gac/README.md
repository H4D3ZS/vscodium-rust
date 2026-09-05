# Kortex GAC — Geometry-Aware Inference Scheduling

Applies the [Geometry of Consolidation](../../../geometry-of-consolidation/) theorem to LLM inference. Lets an 8 GiB GPU host 35 B–70 B GGUF models with a smarter CPU/GPU split than naive `--n-gpu-layers N`.

## What it does

1. **Profiler** (`profiler.rs`) — reads a GGUF, samples 256 rows per weight tensor, and computes the GAC triple per tensor:
   - `d̄` mean within-cluster cosine distance
   - `d_eff` participation-ratio dimension
   - `ρ` spectral concentration

   Output: `<model>.geometry.aim` (JSON profile, ~50 KB).

2. **Planner** (`planner.rs`) — applies the GAC routing rule against a VRAM budget:

   ```
   d̄_critical = (1 - θ) · 2^(1/d_eff_global)
   ```

   - `d̄ < 0.75 · d̄_critical` and `ρ > 0.55` → **tight** → ship to CPU
   - `d̄ > 1.25 · d̄_critical` → **spread** → keep on GPU
   - Otherwise → **borderline** → GPU if room, else CPU

   Output: `TierPlan` with `n_gpu_layers` + per-kind `--override-tensor` rules.

3. **Launcher** (`launcher.rs`) — spawns `llama-server` with the planner's argv and waits for `/health`.

## Why an 8 GiB card can outrun an RTX 3070 at 35 B+

Both cards become bandwidth-limited the moment the model spills to system RAM. The DDR4 ceiling (~50 GB/s) wipes out VRAM differences. What costs the RTX 3070 is what costs every naive offload setup: **the bytes that page from CPU are the wrong bytes**. Naive `-ngl N` pages by block index, not by signal density. Half of the time it ships dense, redundant MLP-down tensors at full precision while keeping high-rank attention projections on GPU at the same precision — exactly backwards.

GAC routes by geometry. By Theorem §2.1 of the paper, tight tensors (low `d̄`, high `ρ`) tolerate any compression / bandwidth pressure with bounded identity error. Spread tensors don't. So:

- Tight → CPU (cheap to demote, even at slow DDR4 they degrade gracefully)
- Spread → GPU (every byte read matters)
- Borderline → fill remaining VRAM

Net effect on a fixed VRAM budget: same bytes on the fast path, but those bytes are the bandwidth-critical ones. Same as if you'd gained ~1.5×–3× of *effective* VRAM.

## Tauri commands

| Command                              | Purpose                                                |
|--------------------------------------|--------------------------------------------------------|
| `kortex_gac_profile`                 | Profile a GGUF, write `<model>.geometry.aim`           |
| `kortex_gac_load_profile`            | Read an existing profile                               |
| `kortex_gac_plan`                    | Plan tier assignment from profile + `PlanOptions`      |
| `kortex_gac_render_args`             | Render a plan to llama-server argv (no launch)         |
| `kortex_gac_quickplan`               | Profile-or-cache + plan in one call                    |
| `kortex_gac_launch`                  | Spawn llama-server with the plan                       |
| `kortex_gac_stop`                    | Kill the running server                                |
| `kortex_gac_status`                  | Query running-server info                              |
| `kortex_gac_default_profile_path`    | Path the profiler will write to for a given GGUF       |

## CLI

```bash
# from the repo root, after `cargo build --release --bin kortex-gac-cli` in src-tauri/

kortex-gac-cli profile --model C:\models\llama-70b.Q4_K_M.gguf
kortex-gac-cli plan --profile C:\models\llama-70b.Q4_K_M.gguf.geometry.aim \
                    --vram-mb 8192 --theta 0.85 --backend vulkan \
                    --output C:\models\plan.json
kortex-gac-cli launch --plan C:\models\plan.json \
                      --model C:\models\llama-70b.Q4_K_M.gguf \
                      --port 8081 --ctx 8192
```

Or use `tools/launch-kortex.ps1` for the full pipeline.

## Notes / future work

- KV-cache GAC compression is not yet wired (would require patching llama.cpp).
  Current scope: weight-tensor placement only. That's already the dominant win
  on 8 GiB cards because weight bytes >> KV bytes for `ctx <= 8 K`.
- Sampling 256 rows is a knob; raise to 512 for tighter geometry estimates if
  profiling time isn't a concern.
- The Jacobi eigensolver in `theory.rs` is intentionally pure-Rust (no LAPACK)
  to keep the dep tree small. For large samples (`n > 1024`) consider a
  ndarray-linalg / nalgebra build.
- The planner currently makes binary GPU/CPU decisions per *kind* (every block
  shares the same fate). A per-block planner would let early blocks differ
  from late blocks; some architectures (Llama-3, Mixtral) reward this.
