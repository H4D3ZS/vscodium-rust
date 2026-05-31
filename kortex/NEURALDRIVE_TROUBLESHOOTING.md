# NeuralDrive Troubleshooting Guide

## Issue: NeuralDrive Doesn't Display

### Symptoms
- App launches but shows blank/black screen
- Window opens but no 3D graph appears
- Sidebar shows but graph area is empty

### Quick Fixes

#### 1. Check if Frontend is Built
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex\neuraldrive
npm run build
```

Look for:
- `dist/index.html` ✓
- `dist/assets/*.js` ✓
- `dist/assets/*.css` ✓

#### 2. Rebuild Everything
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
cargo clean
cargo build --release
```

#### 3. Launch with Console Logging
Run from PowerShell to see errors:
```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"
```

#### 4. Check Ollama Connection
If the app tries to connect to Ollama and it's not running, it might hang:
```powershell
# Test Ollama
curl http://127.0.0.1:11434/api/tags

# If not running, start it
ollama serve
```

### Common Issues

#### Blank Graph Area
**Cause:** No files loaded or project not mounted

**Fix:**
1. Click "Mount Project" button (green, bottom left)
2. Select a folder with code files
3. The graph should populate

#### Graph Shows but No Nodes
**Cause:** Selected folder is empty or ignored

**Fix:**
- Make sure folder contains `.rs`, `.ts`, `.tsx`, `.js`, etc.
- Check that files aren't in `node_modules`, `target`, or `.git`

#### App Crashes on Launch
**Cause:** Missing Visual C++ redistributables or Tauri runtime

**Fix:**
```powershell
# Install VC++ redistributables
winget install Microsoft.VCRedist.2015+.x64
```

#### 3D Graph Performance Issues
**Cause:** Large project or GPU acceleration disabled

**Fix:**
1. Update GPU drivers
2. Mount smaller projects first
3. Check Windows Graphics Settings → Add neuraldrive.exe → Set to "High Performance"

### Diagnostic Commands

```powershell
# Check if exe exists
Test-Path "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"

# Check file size (should be ~14MB)
Get-Item "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe" | Select-Object Name, Length

# Run diagnostic script
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\neuraldrive\diagnose.ps1"
```

### Manual Testing

1. **Test Frontend Alone** (dev mode):
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex\neuraldrive
npm run dev
# Opens at http://localhost:1420
```

2. **Test Backend Only**:
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
cargo test
```

3. **Check Logs**:
- Windows Event Viewer → Windows Logs → Application
- Look for errors from "neuraldrive" at launch time

### Known Limitations

- **First Launch:** May take 5-10 seconds to initialize WebGL context
- **Large Projects:** 10,000+ files may cause slow initial render (be patient)
- **Integrated Graphics:** May have performance issues on Intel HD Graphics

### Getting Help

If none of the above works:

1. Run diagnostic and save output:
```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\neuraldrive\diagnose.ps1" > diagnosis.txt
```

2. Check for Rust panics:
```powershell
$env:RUST_BACKTRACE="full"
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"
```

3. Report issue with:
- OS version
- GPU info
- Diagnosis output
- Any error messages

---

## Quick Launch Command

```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"
```

Or use the launcher script:
```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\launch-neuraldrive.ps1"
```
