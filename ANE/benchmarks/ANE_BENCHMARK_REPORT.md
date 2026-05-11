# Apple Neural Engine — Cross-Generation Benchmark Report

Community-submitted benchmark data from [Issue #3](https://github.com/maderix/ANE/issues/3).

## Model Configuration

All training benchmarks use **Stories110M** — a Llama2-architecture transformer:

```
Parameter       Value
────────────────────────
Architecture    Llama2 (RoPE, SwiGLU, RMSNorm, GQA-ready)
Layers          12
Dimension       768
Hidden (FFN)    2048
Heads           12
Vocab           32000 (Llama 2 BPE)
Sequence        256
Total Params    109.53M (84.95M transformer + 24.58M embedding)
Training Data   TinyStories (~20M tokens, pretokenized)
Optimizer       Adam (lr=1e-4 to 3e-4, b1=0.9, b2=0.999)
Precision       FP16 on ANE, FP32 on CPU
```

Kernels per step (static pipeline): 72 (60 weight-bearing + 12 static sdpaBwd2).
Forward: sdpaFwd + ffnW13 + ffnW2 per layer. Backward: ffnBwdW2t + ffnBwdW13t + wotBwd + sdpaBwd1 + sdpaBwd2 + qkvBwd per layer. Weight gradients (dW) via `cblas_sgemm` on CPU.

## Training Performance (Static Pipeline)

```
Chip            ms/step   ANE ms   Compile/10   ANE TFLOPS   Util%    Contributor
─────────────────────────────────────────────────────────────────────────────────
M1 Pro          148-163   32-35    7.9-8.5s     0.57-0.63    3.6-4.0  @moriwang
M1 Max          143-167   35-45    ~7.1s        0.54-0.65    3.4-4.1  @andyg5000
M3 Ultra*       91        ~10      ~3.7s        0.88         5.6      (repo ref)
M4 Pro          69-73     8.9      ~3.5s        1.28         8.1      @srt54558
M4 Max          64        10.2     ~3.5s        1.45         9.2      @SethBurkart123
M5              101-120   9.1-9.8  3.2-3.4s     0.77-0.91    4.9-5.8  @GitBubble
```

*M3 Ultra = reference platform this project was developed on.

## Peak ANE Throughput (inmem_peak, 128x conv 512ch sp64)

```
Chip            NE Cores  FP16 TFLOPS (measured)    Rated TOPS (Apple spec*)
────────────────────────────────────────────────────────────────────────────
M1 Pro          16        FAIL                      11    (MIL compat issue)
M1 Max          16        FAIL                      11    (MIL compat issue)
M3 Pro          16        9.98                      15.8
M3 Ultra        32        -                         31.6  (ref platform)
M4 Pro          16        12.57                     38
M4 Max          16        10.93                     38
M5              16        12.17                     not disclosed
M5 (other)      16        12.44                     not disclosed
```

*Apple's "Rated TOPS" changed methodology across generations — M1/M3 report FP16,
M4 reports INT8/mixed-precision peak. The numbers are not directly comparable across
generations. Use the measured FP16 TFLOPS column for apples-to-apples comparison.
All chips have 16 NE cores except Ultra variants (32 cores, two dies via UltraFusion).
Max variants share the same 16-core NE as Pro — the M4 Max vs M4 Pro TFLOPS difference
is run-to-run variance, not hardware.*

## Comparative Chart

```
ANE Training Speed (ms/step, lower is better)
══════════════════════════════════════════════════════════════

M1 Pro    ████████████████████████████████████████░░░░  148-163 ms
M1 Max    ██████████████████████████████████████░░░░░░  143-167 ms
M3 Ultra  ██████████████████░░░░░░░░░░░░░░░░░░░░░░░░░   91 ms
M4 Pro    ██████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   69-73 ms
M4 Max    ████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░   64 ms
M5        ████████████████████████░░░░░░░░░░░░░░░░░░░░  101-120 ms

          0        50       100       150       200


Peak ANE Throughput (TFLOPS, higher is better)
══════════════════════════════════════════════════════════════

M1 Pro    FAIL (MIL compat)
M1 Max    FAIL (MIL compat)
M3 Pro    ████████████████████░░░░░░░░░░░░░░░░░░░░░░░░  9.98
M4 Pro    ████████████████████████████████░░░░░░░░░░░░░  12.57
M4 Max    ██████████████████████░░░░░░░░░░░░░░░░░░░░░░  10.93
M5        █████████████████████████░░░░░░░░░░░░░░░░░░░  12.17

          0     3     6     9     12    15    18


ANE Sustained Throughput (TFLOPS, 5s window)
══════════════════════════════════════════════════════════════

M3 Pro    ██████████████████████████████████████████████  15.04 (95.2%)

          0     3     6     9     12    15    18
          (Only M3 Pro submitted sustained benchmark)
```

## Key Findings

### M1/M1 Pro/M1 Max
- **Standalone benchmarks fail** — `ane_mil_gen.h` single-blob weight format rejected
- **Training works** via `stories_mil.h` (separate per-matrix weight blobs)
- ANE compiler handles weight blobs differently from M4+
- Training at 148-167 ms/step, ~0.6 TFLOPS

### M3 Pro
- **Only ch=512 compiles** — 52 channel values tested (1-4096), only 512 accepted
- Fixed 512-wide lane structure in SRAM tiling
- **Peak: 16.77 TFLOPS** (106% of rated 15.8 TOPS) at 128x conv 512ch sp2048
- **Sustained: 15.04 TFLOPS** over 5 seconds (95.2% utilization)
- Spatial dimension is the key to peak throughput (sp64→sp2048 = 2x improvement)

### M4 Pro / M4 Max
- Flexible channel support (256/384/512/768+)
- M4 Pro: peak 12.57 TFLOPS, training at 72.5 ms/step
- M4 Max: peak 10.93 TFLOPS, training at 64 ms/step (fastest overall)
- `sram_probe` and `inmem_bench` fail on M4 Pro (same MIL compat issue)

### M5
- Training works out of the box with existing `program(1.3)` MIL
- Training speed 101-120 ms/step (slower than M4 Max, comparable to M3 Ultra)
- Peak ANE throughput ~12.2-12.4 TFLOPS (similar to M4 Pro)
- ANE appears to be same H16 family as M4
- **M5 Pro/Max not yet benchmarked** — Fusion Architecture may change ANE behavior

### Cross-Generation MIL Compatibility

```
Feature                    M1       M3       M4       M5
─────────────────────────────────────────────────────────
program(1.3) / ios18       PARTIAL  YES      YES      YES
Single-blob weights        FAIL     YES      YES      YES
Per-matrix weight blobs    YES      YES      YES      YES
Channel flexibility        ?        ch=512   FLEX     FLEX
BLOBFILE offset refs       FAIL     YES      YES      YES
```

## macOS Compatibility Issues

- **macOS 26.x** — `[MLModel compileModelAtURL:]` broken for standalone benchmarks
  (fixed in PR #27: switched to in-memory MIL compilation)
- **macOS 15.x** — Works for all M-series with correct MIL format
- M1 generation requires `stories_mil.h` path, not `ane_mil_gen.h`

## How to Contribute

Run on your hardware and post results to [Issue #3](https://github.com/maderix/ANE/issues/3):

```bash
cd training && make train_large
./train_large ane_stories110M_ckpt.bin 256 20 1e-4
```

Include: chip model, macOS version, full output with JSON lines.

---
*Report compiled 2026-03-04 from community submissions.*
*Contributors: @SethBurkart123, @srt54558, @andyg5000, @moriwang, @D-Ogi, @GitBubble, @elijah-pelton*
