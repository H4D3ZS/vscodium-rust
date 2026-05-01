# Use Ollama Backend (GPU-Accelerated on RX 580)

## Why Ollama?

Ollama already has AMD GPU support for Windows via DirectML. Your RX 580 is **already being used** when you run Ollama models!

## Quick Setup

### Step 1: Pull a Compatible Model

```powershell
# Qwen2.5 Coder 7B - Great for coding, fully supported
ollama pull qwen2.5-coder:7b

# Or Llama 3.2 3B - Fast and reliable
ollama pull llama3.2

# Or your existing models
ollama pull sec-eng-neuraldevil
```

### Step 2: Configure VSCodium-Rust

1. **Open Settings** (`Ctrl+,`)
2. **Go to "Inference Backend"**
3. **Select "Ollama"** (NOT llama.cpp)
4. **Configure:**
   - URL: `http://localhost:11434`
   - Model: `qwen2.5-coder:7b` (or your choice)
5. **Click "Save Settings"**
6. **Click "Check Connection"**

### Step 3: Start Using!

Your RX 580 will be automatically used for inference. No compilation needed!

---

## Why This Works

| Backend | RX 580 Support | Status |
|---------|---------------|--------|
| **Ollama** | ✅ Yes (DirectML) | Works NOW |
| llama.cpp (CPU) | ⚠️ CPU only | Slow but works |
| llama.cpp (HIP) | ❌ No (needs RX 6000+) | Won't work |
| woodrex83 ROCm | ⚠️ Linux only | Requires Ubuntu |

---

## Performance Expectations (RX 580 8GB)

| Model | Tokens/Second | VRAM Usage |
|-------|--------------|------------|
| llama3.2:3b | ~15-20 tok/s | ~4GB |
| qwen2.5-coder:7b | ~8-12 tok/s | ~6GB |
| gemma3:12b | ~5-8 tok/s | ~8GB |

---

## If You Really Want llama.cpp + GPU

Your only option is **dual-boot Ubuntu** with the woodrex83 patches:

1. Install Ubuntu 22.04 (dual-boot)
2. Follow woodrex83 instructions: https://github.com/woodrex83/ROCm-For-RX580
3. Build llama.cpp with HIP on Linux
4. Run from Ubuntu

**Not recommended** unless you're comfortable with Linux kernel patches.

---

**Bottom line:** Use Ollama backend. It already works with your RX 580 on Windows!
