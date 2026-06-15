# Offline-First IDE Test Checklist

**Goal**: Verify IDE works flawlessly offline with local Ollama (qwen3.5:12b) and NO login required

## Pre-Test Setup
- [ ] Ensure Ollama is running: `ollama serve` (should be on localhost:11434)
- [ ] Have qwen3.5:12b pulled locally (or any 12b-and-below model)
- [ ] Disable internet OR verify no external API calls are made
- [ ] Tauri dev app is launched successfully

## Phase 1: Launch & Initial State (No Login)
- [ ] App window opens without any login modal/screen
- [ ] Main IDE interface is visible (editor, sidebar, status bar)
- [ ] No error messages about authentication missing
- [ ] Welcome screen or onboarding appears (optional, dismissible)
- [ ] Activity bar is responsive

## Phase 2: Ollama Detection & Model Selection
- [ ] Settings → Model Selection panel is accessible
- [ ] "Auto-Detect Best Model" button is visible
- [ ] Clicking it detects qwen3.5:12b or other local models
- [ ] Models list shows without requiring login
- [ ] Model switch is instant (no external API call)

## Phase 3: Local Inference
- [ ] Open chat/agent panel
- [ ] Ask a simple question: "Hello, what's 2+2?"
- [ ] **CRITICAL**: Should NOT prompt for login
- [ ] Response should stream from local Ollama
- [ ] Response appears in IDE (not "(no response)" bug)
- [ ] Token generation speed shows 12-15 tok/sec (CPU) or 30-40 tok/sec (if ANE enabled)

## Phase 4: ANE Status (M1+ Only)
- [ ] Settings → Optimizations shows ANE status
- [ ] Should detect M1/M2/M3/M4 chip
- [ ] ANE should show as available/enabled
- [ ] Enabling ANE should show ~2.5-3x faster inference

## Phase 5: Memory State
- [ ] No errors about out-of-memory
- [ ] Model remains in RAM (no swap lag)
- [ ] Sustained inference doesn't degrade

## Phase 6: Other Features
- [ ] Terminal opens and uses native zsh (not bundled shell)
- [ ] ripgrep search works for codebase indexing
- [ ] File editing is responsive
- [ ] Git integration works (if repo is open)

## Phase 7: Login Gate Verification
- [ ] Attempt to access bug bounty tools (if available in settings)
  - Should prompt for login ONLY here
  - Should NOT block basic IDE use
- [ ] Regular inference should work without login
- [ ] Settings → Account shows optional login

## Post-Test Success Criteria

✅ **All pass** → IDE is ready for production
- App launches instantly, no auth gates
- Local inference works 30+ tok/sec (with ANE)
- Memory stays under 8GB
- No "(no response)" bug
- Can build production DMG

⚠️ **Partial** → Debug issue and retest
- Document exact failure
- Check inference logs
- Verify Ollama connectivity

❌ **Fails** → Blocker, fix before production
- App hangs on auth check
- Login gate blocks IDE startup
- Inference doesn't work offline

## Debug Commands (if needed)

```bash
# Check Ollama is running
curl http://localhost:11434/api/tags

# List local models
ollama list

# Check app logs
tail -f ~/.config/vscodium-rust-ide/logs.txt  # adjust path as needed

# Monitor memory while using IDE
top -o mem

# Check ANE detection in IDE settings
# (should show hardware info in Optimizations panel)
```

---

**Test conducted**: ________
**Tester**: User (H4D3ZS)
**Outcome**: ✅ / ⚠️ / ❌
