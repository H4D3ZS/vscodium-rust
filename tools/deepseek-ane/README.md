# DeepSeek V2 (ds2) on Apple Silicon

Local DeepSeek V2 inference for the IDE, running on M1 / M2 / M3 / M4 Macs.
Speaks the OpenAI-compatible HTTP API the rest of the agent already uses,
so you get the full agent loop, tools, checkpoints, and security playbooks
without an internet connection or API key.

## TL;DR

```bash
# One-time setup
bash tools/deepseek-ane/bootstrap.sh

# Start the server (leave running)
bash tools/deepseek-ane/start-server.sh

# In the IDE: Settings → AI → provider "deepseek-ane" → pick a model
```

That's it.

## Two runtimes — which one runs on the ANE?

This is the question most people ask first, so the honest answer up front:

**No production LLM today runs end-to-end on the Apple Neural Engine.**

The ANE is designed for fixed-shape Conv2D / specific transformer kernels
that Apple has profiled in CoreML. Modern decoder LLMs use dynamic shapes,
KV-caches, and attention patterns the ANE can't execute as a single graph.
Inference frameworks dispatch *parts* of the model to the ANE where they
fit — but the bulk of generation goes to GPU (Metal) on Apple Silicon.

So "Apple ANE" in this directory is shorthand for "tuned for the Apple
Neural Engine *family of accelerators*" — i.e. the unified-memory
Metal/ANE/CPU substrate. The bootstrap gives you two runtimes that lean
on that substrate differently:

| Runtime | Backend | ANE usage | Speed | Maturity |
|---------|---------|-----------|-------|----------|
| `llama` (default) | llama.cpp + Metal | None — pure GPU | Fastest today | Production |
| `mlx` | Apple MLX | Per-kernel via unified memory; some ops route to ANE | Slightly slower than llama.cpp but improving fast | Newer |

Pick `mlx` if you want the closest practically available "ANE-aware"
path. Pick `llama` (default) if you want max tokens/sec right now.

```bash
DS2_RUNTIME=llama  bash tools/deepseek-ane/bootstrap.sh   # default
DS2_RUNTIME=mlx    bash tools/deepseek-ane/bootstrap.sh
```

## Model choice

DeepSeek V2 full (236B) won't fit on consumer M-series. The practical
choice is **DeepSeek V2-Lite (16B, 2.4B active per token via MoE)**.

| `DS2_MODEL` | Use case | Size on disk |
|-------------|----------|--------------|
| `chat` (default) | General coding + reasoning + agentic loop | ~9 GB (Q4_K_M GGUF) |
| `coder` | Code-first tasks; sharper on Python / TS / Rust | ~9 GB (Q4_K_M GGUF) |

```bash
DS2_MODEL=chat   bash tools/deepseek-ane/bootstrap.sh   # default
DS2_MODEL=coder  bash tools/deepseek-ane/bootstrap.sh
```

You can run both — they get downloaded to `~/.cache/deepseek-ane/models/`
and you just swap which one `start-server.sh` points at.

## Hardware requirements

| Mac | Verdict |
|-----|---------|
| M1 / M2 / M3 / M4 — 16 GB RAM | Works. Q4_K_M chat model uses ~9 GB; comfortable. |
| M1 / M2 — 8 GB RAM | Tight. Drop context to 8k via `DS2_CTX=8192`. |
| Intel Mac | Not supported. Use the cloud `deepseek` provider or Ollama. |
| Windows / Linux | Not supported. Use Ollama; bootstrap.sh is Mac-only. |

## How the IDE finds the server

The IDE reads the URL in this order:

1. `DEEPSEEK_ANE_URL` env var (highest priority)
2. The provider setting in the AI panel
3. Default: `http://127.0.0.1:8080`

If the first chat returns a connection error, you didn't start
`start-server.sh` — that's the most common failure mode.

## Updating the model

```bash
# Re-download the latest quantization
rm -rf ~/.cache/deepseek-ane/models
bash tools/deepseek-ane/bootstrap.sh
```

## Troubleshooting

**"Address already in use" on port 8080**

```bash
# Find what's holding it
lsof -i :8080
# Pick a different port
DS2_PORT=8090 bash tools/deepseek-ane/bootstrap.sh
DEEPSEEK_ANE_URL=http://127.0.0.1:8090 ./your-ide
```

**"out of memory" / kernel panic during chat**

Drop the context window — DeepSeek-V2-Lite is MoE so KV-cache balloons fast:

```bash
DS2_CTX=8192 bash tools/deepseek-ane/bootstrap.sh
```

**MLX path: "no matching distribution for mlx-lm"**

You're on an older macOS. MLX requires macOS 13.5+. Either upgrade or
use the `llama` runtime instead.

## Why not Ollama for DeepSeek V2?

You can — Ollama works fine on Mac. The `deepseek-ane` provider exists
so the IDE has a dedicated provider entry that:
- doesn't conflict with your Ollama-hosted models on the same machine
- explicitly points at the M1-tuned quantizations
- can switch between llama.cpp and MLX without changing IDE config

If you're already happy with Ollama, just use it — the cloud
`deepseek` provider and the local `deepseek-ane` provider are both
optional and independent.
