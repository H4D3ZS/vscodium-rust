# Release — Installer build, sidecar bundling, anti-RE, distribution

Produces a downloadable Windows installer with the **invisible_playwright**
browser engine bundled (no Python needed on the user's machine), optionally
hardened against reverse-engineering, ready to host on the website.

## 1. Bundle language servers (REQUIRED — zero-config IntelliSense)

```powershell
powershell -ExecutionPolicy Bypass -File scripts\fetch-lsp-binaries.ps1
```

→ `src-tauri\binaries\lsp\` (rust-analyzer, gopls, typescript-language-server + Node, pyright).
Bundled via `binaries/*`. First launch also auto-downloads Rust/Go servers into app data if missing.

Mirror the `lsp/` folder on your DO CDN and set `LSP_BUNDLE_MIRROR` for air-gapped builds.

## 2. Freeze the browser sidecar (REQUIRED for a standalone installer)

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1
```

→ `src-tauri\binaries\browser-agent.exe` (invisible_playwright baked in). It's
auto-bundled via `bundle.resources` (`binaries/*`). At runtime `browser.rs`
prefers this frozen exe and only falls back to system Python for source/dev runs.

> Skip this and the installer still builds, but browser automation will require
> the user to have Python + `pip install playwright invisible_playwright`.

## 2b. Bundle claurst (optional external agent backend)

```powershell
powershell -ExecutionPolicy Bypass -File scripts\build-claurst.ps1
```

→ `src-tauri\binaries\claurst.exe`. Shipped in the same `binaries/*` bundle.
Settings → Agent Engine → **Claurst** uses this process (GPL, kept at arm's length).
Default engine remains **Sentient** (built-in).

Both sidecars are built automatically by `npm run prebuild:sidecar` before `tauri build`.

## 3. Build the installer

```powershell
npm run build:tauri
```

(`prebuild:sidecar` + frontend `build` + `npx tauri build` — same as `scripts\release.ps1`.)

Outputs in `src-tauri\target\release\bundle\`:
- `nsis\VSCodium Rust IDE_0.1.0_x64-setup.exe`  (Windows — default installer)

Windows builds **NSIS only** (WiX `.msi` omitted — per-user `%LOCALAPPDATA%\Programs\`
install fails Tauri's MSI ICE validation). Default path:

`%LOCALAPPDATA%\Programs\VSCodium Rust IDE`

(Antigravity / VS Code style — no admin, no `Program Files`, writable app data.)

If you previously installed via MSI under `Program Files`, uninstall that copy first.

`bundle.targets` is `["nsis", ...]` (Windows NSIS only). To emit only NSIS: `npx tauri build --bundles nsis`.

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

The IDE is **free to download** (works with local Lemonade + your own API keys).
Payment unlocks **Cyber-Ifrit Cloud** managed models (enforced in-app + at the AMD
gateway). So the download link is public:

1. Upload the `-setup.exe` to **GitHub Releases** (tag e.g. `Release-Platforms-Binary`).
2. Set `PUBLIC_DOWNLOAD_URL` in the portfolio `.env` / Netlify to the direct asset URL
   (e.g. `.../releases/download/Release-Platforms-Binary/VSCodium.Rust.IDE_0.1.0_x64-setup.exe`).
   The site's `/download` page uses this; falls back to the same URL baked into `download.astro`.

## 6. macOS (M1) — same flow

`npx tauri build` on the Mac → `.dmg` in `bundle/dmg/`. Freeze the sidecar with a
macOS PyInstaller run (or ship Python-required). Notarize with `xcrun notarytool`.

---

## Quick ship checklist

Run a full Windows release:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\release.ps1
```

Or step-by-step:

```powershell
npm run typecheck
powershell -ExecutionPolicy Bypass -File scripts\fetch-lsp-binaries.ps1   # once
powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1        # once (invisible_playwright)
powershell -ExecutionPolicy Bypass -File scripts\build-claurst.ps1        # once (optional agent backend)
npm run build:tauri
```

Output: `src-tauri\target\release\bundle\nsis\` (`VSCodium Rust IDE_*-setup.exe`).

## Bundled runtime

| Component | Location | Notes |
|-----------|----------|--------|
| LSP servers | `src-tauri/binaries/lsp/` | rust-analyzer, gopls, pyright, tsserver |
| Stealth browser | `src-tauri/binaries/browser-agent.exe` | invisible_playwright via PyInstaller |
| Claurst agent | `src-tauri/binaries/claurst.exe` | optional external agent backend (GPL, separate process) |
| Extension host | `src-tauri/ext-host/` | Open VSX extensions |

Dev mode uses Python + `invisible_playwright/src` on `PYTHONPATH` when `browser-agent.exe` is absent. Claurst falls back to `claurst/src-rust/target/release/` in dev when `claurst.exe` is not prebuilt.

Both sidecars are built automatically by `npm run prebuild:sidecar` (runs before `npx tauri build`).

## Pre-ship smoke test

1. Open folder → chat → agent edits a file → accept diff
2. **+** new chat tab → close tab with **×** → History → **Restore conversation**
3. `/manus` web mission (stealth Firefox launches)
4. Settings → Account → plan card layout
5. Install one Open VSX extension (ESLint or Prettier)
6. F9 breakpoint → debug panel
7. Ctrl+T ghost completion + Tab next-edit toast (if enabled in Settings)

## Optional services (user machine)

- **Lemonade** `:13305` — local models (real llama.cpp)
- **aim-proxy** `:1536` — `.aim` context injection
- Cloud keys in Settings → Providers

## Updater

Tauri updater is **disabled** until signing keys are configured in `tauri.conf.json`.
