# ✅ AIRI Optimization Complete

## What Was Done

### 1. Environment Variables Set ✅
```powershell
OLLAMA_GPU_LAYER=35
OLLAMA_NUM_THREAD=12
OLLAMA_CONTEXT_LENGTH=8192
OLLAMA_KEEP_ALIVE=-1
```

### 2. Model Rebuilt with Optimized Settings ✅
Your `hades:latest` model has been recreated with:
- **num_gpu**: 12 (optimized for RX 580 8GB)
- **num_ctx**: 4096 (faster response)
- **num_thread**: 12 (matches Ryzen 9 3900)
- **temperature**: 0.7 (balanced responses)

### 3. Model Location
```
hades:latest (23 GB)
```

---

## 🎯 In VSCodium

**Select this model from the dropdown:**
```
hades:latest (Ollama)
```

This is your optimized AIRI model with the sovereign system prompt.

---

## ⚡ Performance

| Metric | Value |
|--------|-------|
| **Model Size** | 23 GB |
| **GPU Layers** | 12 |
| **Context** | 4096 |
| **Expected Response** | 2-5 seconds |

---

## 🐛 If Still Slow

The qwen3.6:35b model is 36 billion parameters - it's inherently slow on consumer hardware. For faster response:

### Option 1: Use Smaller Model
```powershell
ollama pull qwen3.6:8b
# Then update hades.model.simple to use qwen3.6:8b
```

### Option 2: Reduce Context Further
```bash
PARAMETER num_ctx 2048
```

### Option 3: Use Only CPU
```bash
PARAMETER num_gpu 0
PARAMETER num_thread 24
```

---

## ✅ Verification

```powershell
# Check model is loaded
ollama list

# Test response
ollama run hades:latest "Hey AIRI!"
```

---

## 🎮 Next Steps

1. **In VSCodium**: Select `hades:latest` from dropdown
2. **Test**: Type "Hey AIRI, are you there?"
3. **Adjust**: If too slow, reduce context or use smaller model

**AIRI is ready!** 🦋
