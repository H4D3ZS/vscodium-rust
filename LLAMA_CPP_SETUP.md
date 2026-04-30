# Using llama.cpp with HADES Bridge in VSCodium-Rust

## Quick Start

### Step 1: Prepare Your Model

Your Ollama models are located at:
```
C:\Users\HADES\.ollama\models\blobs\
```

The largest file (22.29GB) is your main model. Run this script to copy it:

```powershell
powershell -ExecutionPolicy Bypass -File tools\setup-ollama-model-for-llama-cpp.ps1
```

This copies your model to `C:\models\ollama-model.gguf`

### Step 2: Start llama.cpp Server

```bash
# Navigate to llama.cpp directory
cd kortex/llama.cpp/build/bin

# Start server with HADES Bridge optimization
llama-server.exe -m C:\models\ollama-model.gguf \
  -ngl 99 \
  --port 8080 \
  --hades-jit \
  --hades-gist C:\path\to\project.aim
```

**Flags explained:**
- `-m`: Path to your GGUF model
- `-ngl 99`: Offload all layers to GPU (99 = maximum)
- `--port 8080`: Server port (default for llama.cpp)
- `--hades-jit`: Enable HADES JIT decompression (optional, for 8GB VRAM)
- `--hades-gist`: Path to 6KB gist file for semantic navigation

### Step 3: Configure VSCodium-Rust

1. **Open Settings** (`Ctrl+,`)
2. **Navigate to "Inference Backend"**
3. **Select "llama.cpp + HADES"**
4. **Configure:**
   - Server URL: `http://localhost:8080`
   - Model Path: `C:\models\ollama-model.gguf`
   - GPU Layers: `99`
   - Enable HADES Bridge: ✓ (checked)
5. **Click "Save Settings"**
6. **Click "Check Connection"** - should show "running"

### Step 4: Start Using

Switch your AI agent to use llama.cpp backend. The HADES Bridge will:
- Monitor GPU temperature (throttle at 72°C)
- Page layers on-demand for 8GB VRAM
- Use JIT decompression for infinite context
- Maintain 6KB semantic gist in VRAM

---

## Configuration Options

### Inference Backend Selection

| Backend | Best For | VRAM Usage | Setup |
|---------|----------|------------|-------|
| **Ollama** | Easy setup, model management | Automatic | `ollama pull llama3` |
| **llama.cpp + HADES** | 8GB VRAM optimization | Manual config | This guide |
| **OpenAI API** | Cloud inference | N/A | API key required |

### llama.cpp Server Options

```bash
# Basic usage (8GB VRAM with HADES)
llama-server -m model.gguf -ngl 99 --port 8080

# With HADES JIT decompression
llama-server -m model.gguf -ngl 99 --port 8080 \
  --hades-jit --hades-gist project.aim

# With context size
llama-server -m model.gguf -ngl 99 -c 4096 --port 8080

# With batch size
llama-server -m model.gguf -ngl 99 -b 512 --port 8080
```

### HADES Bridge Settings

| Setting | Recommended | Description |
|---------|-------------|-------------|
| GPU Layers | 99 | Offload all layers to GPU |
| HADES Enabled | ✓ | Enable 8GB VRAM optimization |
| Gist Path | (your project) | 6KB semantic map for JIT |
| Thermal Throttle | 72°C | Auto-throttle temperature |

---

## Monitoring

### Check llama.cpp Status

In VSCodium-Rust Settings → Inference Backend, click "Check Connection"

### View GPU Telemetry

```bash
# Run HADES Governor daemon
cd kortex/hades-kernel
./target/release/hades-governor.exe
```

Shows:
- GPU Temperature
- Power Draw
- VRAM Usage
- Throttle Ratio

### HADES Metrics

When enabled, HADES Bridge logs:
```
[HADES] Bridge initialized - Mode: 8GB Local (active paging)
[HADES] FAULT DETECTED: cluster 42 activation ≥ 0.85
[HADES] Inflated 2.3MB from SSD to VRAM
[HADES] LRU evicted block 15: 1.2MB freed
```

---

## Troubleshooting

### "Connection failed" error

1. Make sure llama.cpp server is running
2. Check port 8080 is not in use
3. Verify firewall allows localhost:8080

### "Out of VRAM" error

1. Enable HADES Bridge in settings
2. Reduce batch size: `-b 256`
3. Reduce context size: `-c 2048`
4. Reduce GPU layers: `-ngl 80`

### Model not loading

1. Verify GGUF file path is correct
2. Check file size matches expected (~22GB for your model)
3. Ensure GGUF format (not Ollama blob format)

### Slow inference

1. Check GPU layers: should be `-ngl 99`
2. Verify GPU is being used (check GPU-Z or Task Manager)
3. HADES may be paging - normal for large models on 8GB

---

## Performance Expectations

### With HADES Bridge (8GB VRAM)

| Metric | Expected |
|--------|----------|
| Token Generation | 10-20 tok/s |
| VRAM Usage | 6-6.5GB (capped) |
| GPU Temp | 65-72°C (throttled) |
| Context | Unlimited (JIT) |

### Without HADES (Full VRAM)

| Metric | Expected |
|--------|----------|
| Token Generation | 20-30 tok/s |
| VRAM Usage | 22GB (all layers) |
| GPU Temp | 70-80°C |
| Context | Limited by VRAM |

---

## Advanced: Create 6KB Gist

For JIT decompression with semantic navigation:

```bash
cd kortex/hades-kernel

# Index your project
./target/release/kortex-indexer \
  --path C:/Users/HADES/Desktop/vscodium-rust/ \
  --output ./project.aim
```

This creates a 6KB semantic map that stays resident in VRAM while code blocks are inflated on-demand.

---

## Resources

- **llama.cpp**: https://github.com/ggerganov/llama.cpp
- **HADES-KORTEX**: https://github.com/H4D3ZS/kortex
- **GGUF Models**: https://huggingface.co/TheBloke
- **Ollama Models**: `ollama list`

---

**Created:** 2026-04-30  
**Version:** 1.0  
**Backend:** llama.cpp + HADES Bridge v0.1.0
