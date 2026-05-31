# Kortex Context Savepoint - Quick Reference

## ⚡ Quick Commands

```powershell
# Save context NOW (before heavy work or shutdown risk)
qs "Working on BL sign extension fix"

# OR full command
powershell -ExecutionPolicy Bypass -File "C:\Users\HADES\Desktop\vscodium-rust\kortex\save.ps1" "Your message"

# Recover after shutdown
powershell -ExecutionPolicy Bypass -File "C:\Users\HADES\.qwen\sessions\latest\recover.ps1"

# View saved context
notepad "C:\Users\HADES\.qwen\sessions\latest\context.md"
```

## 📁 File Locations

| Type | Location |
|------|----------|
| Save script | `C:\Users\HADES\Desktop\vscodium-rust\kortex\save.ps1` |
| Saved sessions | `C:\Users\HADES\.qwen\sessions\` |
| Latest context | `C:\Users\HADES\.qwen\sessions\latest\context.md` |
| Recovery script | `C:\Users\HADES\.qwen\sessions\latest\recover.ps1` |

## 🔄 Workflow

1. **Before heavy work:** `qs "Starting X"`
2. **PC shuts down unexpectedly**
3. **After restart:** Run `recover.ps1` or read `context.md`
4. **Continue** from where you left off

## 📋 What Gets Saved

- ✅ Active plan
- ✅ Active todos  
- ✅ Last 10 conversation exchanges
- ✅ Session ID
- ✅ Timestamp and context message

## ⚠️ Important

- **Kortex executables do NOT need rebuilding** - they're already built at:
  - `kortex/target/release/{aim-proxy.exe,neuraldrive.exe,hades-tui.exe,aim-vfs.exe}`
- **Only run `cargo build --release` if you modify Rust source code**
- Save context every 10-15 minutes during intensive work

---

**Created:** 2026-04-27  
**For:** HADES-KORTEX project with PC auto-shutdown protection
