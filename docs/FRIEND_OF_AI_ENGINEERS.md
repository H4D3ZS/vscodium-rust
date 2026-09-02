# Friend of AI Engineers

VSCodium-Rust is built for people who work at the intersection of **machine learning**, **agentic systems**, and **systems programming** — not as three separate tools, but as one workflow.

## Principles

1. **Your machine, your models** — Ollama and local PyTorch first. Cloud routing is optional, never mandatory.
2. **Experiments stay in-repo** — `.hades/ml/` runs, metrics, checkpoints, and reports live beside your code.
3. **Agents that respect engineering** — Shadow VFS, surgical patches, verify-before-done; not blind full-file rewrites.
4. **Open stacks, open weights** — torchvision, timm, HuggingFace loaders; export to ONNX and TorchScript.
5. **No performance theater** — Rust backend, mmap `.aim` memory, sub-150MB core footprint target.

## What ships today

| Surface | Path |
|---------|------|
| PyTorch ML Studio | Activity bar → beaker · `src/components/pytorch/` |
| Agent loop + tools | AIRI chat · `src-tauri/src/ai_engine.rs` |
| Local Ollama | Settings → Ollama · `:11434` default |
| Skills hub | Hermes-compatible SKILL.md · `hermes_skills.rs` |
| Semantic index | Vector + `.aim` · `vector_indexer.rs` |

## PyTorch ML Studio flow

```
Setup (CUDA/ROCm) → Data (analyze + augment preview) → Train (resume/cancel)
→ Dashboard (live charts) → Model (graph + gallery) → Experiments (Optuna HPO)
→ Export/Debug (ONNX, grad check, LR finder) → Inference
```

Inspired by [TorchStudio](https://www.torchstudio.ai/) module architecture; implemented natively in this IDE.

## Who this is for

- ML engineers fine-tuning locally before cloud deploy
- Agent builders who need IDE + terminal + browser + model training in one shell
- Security researchers who refuse to send proprietary binaries to SaaS
- Indie hackers running Ollama on a single GPU and shipping anyway

If that sounds like you — welcome. We're building this *with* AI engineers, not *for* a generic "developer" persona that never trains a model.
