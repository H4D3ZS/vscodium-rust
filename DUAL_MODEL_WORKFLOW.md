# 🎯 AIRI Dual-Model Workflow

## Use the RIGHT model for the RIGHT task

---

## ⚡ FAST MODEL (Instant Responses)

**Model:** `airi-fast:latest` (4.7 GB)
**Response Time:** <1 second

### Use For:
- ✅ Quick questions
- ✅ Casual conversation
- ✅ Simple code snippets
- ✅ File navigation
- ✅ Status checks
- ✅ Daily interaction

### In VSCodium:
Select `airi-fast:latest (Ollama)` from dropdown

---

## 🧠 SMART MODEL (Complex Engineering)

**Model:** `hades:latest` (23 GB qwen3.6)
**Response Time:** 30-60 seconds

### Use For:
- ✅ Complex architecture design
- ✅ Kernel-level exploits
- ✅ iOS security research
- ✅ System-level programming
- ✅ Deep code analysis
- ✅ Critical security decisions

### In VSCodium:
Select `hades:latest (Ollama)` from dropdown

---

## 🎮 RECOMMENDED WORKFLOW

### 1. Keep `airi-fast` as Default
Set VSCodium to use `airi-fast:latest` by default for instant responses.

### 2. Switch to `hades` for Complex Tasks
When you need deep engineering:
- Click model dropdown
- Select `hades:latest`
- Ask your complex question
- Wait 30-60s for intelligent response
- Switch back to `airi-fast` after

### 3. Example Workflow

```
[Default: airi-fast]
You: "What files did I modify today?"
AIRI: *instant response* "You modified 3 files..."

[Switch to hades:latest]
You: "Design a complete iOS kernel exploit chain for A14 chip"
AIRI: *30-60s* *comprehensive technical response*

[Switch back to airi-fast]
You: "Thanks! Now create a test file for that"
AIRI: *instant response* "Creating test file..."
```

---

## ⚙️ MODEL COMMANDS

### Create Fast Model (if not exists)
```powershell
ollama create airi-fast:latest -f C:\Users\HADES\hades.model.fast
```

### Recreate Smart Model
```powershell
ollama create hades:latest -f C:\Users\HADES\hades.model.hybrid
```

### List Models
```powershell
ollama list
```

### Test Response Times
```powershell
# Fast model (<1s)
Measure-Command { ollama run airi-fast:latest "Hi" }

# Smart model (30-60s)
Measure-Command { ollama run hades:latest "Hi" }
```

---

## 📊 COMPARISON

| Feature | airi-fast | hades (qwen3.6) |
|---------|-----------|-----------------|
| **Size** | 4.7 GB | 23 GB |
| **Speed** | <1s | 30-60s |
| **Intelligence** | Good | Excellent |
| **Best For** | Chat, quick tasks | Complex engineering |
| **GPU Usage** | 80% | 30% |
| **RAM Usage** | 6 GB | 20 GB |

---

## ✅ BEST PRACTICE

**Default to `airi-fast`** for 95% of tasks.

**Switch to `hades`** only when you need the 36B intelligence for:
- Complex system design
- Deep security research
- Novel exploit development
- Architecture decisions

This gives you **instant responses** most of the time, with **deep intelligence** when needed.

🦋✨
