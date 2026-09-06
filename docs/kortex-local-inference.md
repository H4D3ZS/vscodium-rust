# Kortex Local Inference — ROCmFPX + Escha on the RX 9060 XT

The demo path: run the Escha W2 35B-A3B model (2.5 bpw `Q2_0_ROCMFPX` quant) on a
16 GB RX 9060 XT through a ROCmFPX-built `llama-server`, with Kortex's
geometry-aware tier planner and KV-slot prefix cache in front, driven from the
IDE.

```
IDE chat ──▶ kortex_kvcache proxy (:1537) ──▶ llama-server (:8081, ROCmFPX)
                 │  SHA-keyed prefix match → /slots restore → skip prefill
                 └─ geometry plan from kortex_gac decides GPU/CPU tensor tiers
```

## 1. Build the ROCmFPX `llama-server`

Stock llama.cpp / Lemonade **cannot** load the `Q2_0_ROCMFPX` quant. You need a
`llama-server` from the ROCmFPX tree (`kortex/ROCmFPX`, submodule).

### Prerequisites (installed once via winget)

| Tool | Package |
|---|---|
| CMake | `Kitware.CMake` |
| Ninja | `Ninja-build.Ninja` |
| MSVC + Clang | `Microsoft.VisualStudio.2022.BuildTools` (workload `VCTools` + `VC.Llvm.Clang` + `Windows11SDK`) |
| Vulkan SDK | `KhronosGroup.VulkanSDK` |
| **AMD HIP SDK** (HIP build only) | **not on winget** — <https://rocm.docs.amd.com/projects/install-on-windows/en/latest/> — must include RDNA4 / `gfx1200` device libraries |

### Vulkan build (works today, no HIP SDK)

```powershell
powershell -File scripts/build-rocmfpx-windows.ps1 -VulkanOnly -Arch gfx1200
```

Runs the `q2_0_rocmfpx` Vulkan decode kernels on the RX 9060 XT via `-dev Vulkan0`.
Slower than HIP but fully functional. Output staged to
`src-tauri/binaries/rocmfpx/llama-server.exe`.

### HIP build (faster, needs the AMD HIP SDK)

```powershell
powershell -File scripts/build-rocmfpx-windows.ps1 -Arch gfx1200
# RX 9070 / 9070 XT / R9700 -> -Arch gfx1201  (NOT interchangeable with gfx1200)
```

`HSA_OVERRIDE_GFX_VERSION` does **not** work on Windows — the binary's `gfx`
target must match the card.

## 2. Get the model

```
lemonade pull cafonez/Escha-W2-35B-A3B-ROCmFP2
# -> %USERPROFILE%\.cache\huggingface\hub\models--cafonez--Escha-W2-35B-A3B-ROCmFP2
#    \snapshots\<id>\Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf   (~13 GB)
```

Add a Defender exclusion for `%USERPROFILE%\.cache\huggingface` first — it locks
the file mid-rename on large pulls.

To make your own from the EschaLabs source model (needs the ROCmFPX
`llama-quantize`, also produced by the build):

```
llama-quantize model-BF16.gguf model-Q2_0_ROCMFPX.gguf Q2_0_ROCMFPX
```

## 3. Launch from the IDE

**Settings → Kortex / AIM Layer → Local Inference (Kortex + ROCmFPX)**

1. **Model (GGUF)** → the Escha `.gguf`.
2. **Server binary** → `src-tauri/binaries/rocmfpx/llama-server.exe` (blank =
   resolve `llama-server` from PATH).
3. **VRAM budget** → `16384`.
4. **Launch**. The panel then:
   - profiles the GGUF geometry (`kortex_gac_profile`), caches
     `<model>.geometry.aim`;
   - builds a tier plan for the budget and spawns `llama-server` on `:8081` with
     `--slot-save-path`;
   - starts the KV-slot cache proxy on `:1537` in front of it;
   - repoints the IDE inference backend to `http://127.0.0.1:1537`.
5. The status line shows the plan (`spread→GPU …, θ=0.85, GPU x.xG / CPU y.yG`)
   and, once running, live cache stats
   (`N entries, X GB, H% hit rate, K tokens skipped`).

**Stop** tears down the proxy and the server.

## 4. Demo script

1. Open a large repo in the IDE.
2. Ask the agent a question that pulls a big context (system prompt + tools +
   files ≈ 15–30 K tokens). Note time-to-first-token.
3. Ask a follow-up in the same thread. The prefix is a cache hit → `/slots
   restore` skips re-prefill → TTFT drops sharply and the panel's
   "tokens skipped" counter jumps.
4. Restart `llama-server` (Stop → Launch). The cache is on disk, so the next
   turn is *still* a hit — the point of KDKVC.

## Notes

- `-dev Vulkan0` for the Vulkan build, `-dev ROCm0` for the HIP build. The GAC
  launcher passes device flags via the tier plan's `backend`
  (`vulkan` / `rocm`).
- The proxy binds model identity from `/props` at start — a cache populated by
  one model is never served to another (`ModelIdentity::accepts`).
- Tier 2 (response cache) is not wired yet; this path is Tier 1 (KV-slot reuse)
  only. See `docs/kortex-cache.md`.
