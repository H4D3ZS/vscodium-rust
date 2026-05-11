# NeuralDrive Display Fix

## Problem
NeuralDrive was launching but showing a **blank/black window** with no content.

## Root Cause
The Vite build configuration was missing the `base: './'` setting, causing the built `index.html` to reference assets with absolute paths (`/assets/`) instead of relative paths (`./assets/`).

Tauri requires relative paths because it loads files from the local filesystem, not from a web server root.

## Fix Applied

### 1. Updated `vite.config.ts`
Added the `base: './'` configuration:

```typescript
export default defineConfig(async () => ({
  plugins: [react()],
  
  base: './',  // ← Added this line
  
  // ... rest of config
}));
```

### 2. Rebuilt Frontend
```bash
cd kortex/neuraldrive
npm run build
```

### 3. Rebuilt Tauri App
```bash
cd kortex
cargo build --release --package neuraldrive
```

## Verification

The built `dist/index.html` now correctly uses relative paths:

**Before (broken):**
```html
<script src="/assets/index-DypNOTxz.js"></script>
<link href="/assets/index-h9BSdB70.css">
```

**After (fixed):**
```html
<script src="./assets/index-DypNOTxz.js"></script>
<link href="./assets/index-h9BSdB70.css">
```

## Launch Commands

### Quick Launch
```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"
```

### Or use the launcher script
```powershell
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\launch-neuraldrive.ps1"
```

## Using NeuralDrive

1. **Launch** the application
2. Click **"Mount Project"** (green button, bottom left)
3. **Select a folder** containing code files
4. The 3D neural graph will populate with nodes representing your files
5. **Click nodes** to inspect them and view contents in the Explorer tab

## If Still Blank

Try these additional steps:

### Check WebGL Support
```powershell
# Run with GPU debugging
$env:RUST_BACKTRACE="full"
& "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"
```

### Clear and Rebuild
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex
cargo clean
cd neuraldrive
rm -rf dist node_modules
npm install
npm run build
cd ..\
cargo build --release
```

### Check Console for Errors
Run from PowerShell to see any runtime errors:
```powershell
cd C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release
.\neuraldrive.exe
```

## Files Modified

- `kortex/neuraldrive/vite.config.ts` - Added `base: './'`
- `kortex/neuraldrive/dist/*` - Rebuilt with correct paths
- `kortex/target/release/neuraldrive.exe` - Rebuilt with fixed frontend

---

**Status:** ✅ Fixed - NeuralDrive should now display correctly
