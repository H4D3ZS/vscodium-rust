# Release — Installer build, sidecar bundling, anti-RE, distribution

Produces a downloadable Windows installer with the **invisible_playwright**
browser engine bundled (no Python needed on the user's machine), optionally
hardened against reverse-engineering, ready to host on the website.

## 1. Freeze the browser sidecar (REQUIRED for a standalone installer)

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1
```

→ `src-tauri\binaries\browser-agent.exe` (invisible_playwright baked in). It's
auto-bundled via `bundle.resources` (`binaries/*`). At runtime `browser.rs`
prefers this frozen exe and only falls back to system Python for source/dev runs.

> Skip this and the installer still builds, but browser automation will require
> the user to have Python + `pip install playwright invisible_playwright`.

## 2. Build the installer

```powershell
npx tauri build
```

Outputs in `src-tauri\target\release\bundle\`:
- `msi\VSCodium Rust IDE_0.1.0_x64_en-US.msi`  (WiX)
- `nsis\VSCodium Rust IDE_0.1.0_x64-setup.exe`  (NSIS — friendlier installer)

`bundle.targets` is `"all"`; to emit only NSIS: `npx tauri build --bundles nsis`.

## 3. (Optional) Harden against reverse-engineering — Themida

Themida is a commercial Windows protector (not redistributable here). Apply it to
the **main exe inside the bundle** after step 2, then re-pack/re-sign:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\pack-themida.ps1
```

Edit that script to point at your Themida CLI + `.tmd` project. Notes:
- Protect the Tauri **main exe** (`vscode-rust-app.exe`), not the WebView2 runtime.
- Themida + code-signing order: **protect first, then sign** (signing a protected
  binary is fine; protecting a signed binary breaks the signature).
- Test the protected build — anti-debug can trip AV; whitelist / submit to vendors.

## 4. Code signing (avoids SmartScreen warnings)

```powershell
signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /a `
  "src-tauri\target\release\bundle\nsis\VSCodium Rust IDE_0.1.0_x64-setup.exe"
```

Or set Tauri's `bundle.windows.certificateThumbprint` to sign during `tauri build`.

## 5. Host + link on the website

The IDE is **free to download** (works with local Ollama + your own API keys).
Payment unlocks **Cyber-Ifrit Cloud** managed models (enforced in-app + at the AMD
gateway). So the download link is public:

1. Upload the `-setup.exe` to **GitHub Releases** (or Netlify/S3).
2. The site's `/download` page + nav button point at that release asset
   (`PUBLIC_DOWNLOAD_URL` in the portfolio `.env`, falls back to the releases page).

## 6. macOS (M1) — same flow

`npx tauri build` on the Mac → `.dmg` in `bundle/dmg/`. Freeze the sidecar with a
macOS PyInstaller run (or ship Python-required). Notarize with `xcrun notarytool`.
