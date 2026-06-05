# Ship checklist — product-grade release

Run a full Windows release:

```powershell
powershell -ExecutionPolicy Bypass -File scripts\release.ps1
```

Or step-by-step:

```powershell
npm run typecheck
powershell -ExecutionPolicy Bypass -File scripts\fetch-lsp-binaries.ps1   # once
powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1        # once (needs invisible_playwright)
npm run build:tauri
```

Output: `src-tauri\target\release\bundle\` (NSIS/MSI).

## Bundled runtime

| Component | Location | Notes |
|-----------|----------|--------|
| LSP servers | `src-tauri/binaries/lsp/` | rust-analyzer, gopls, pyright, tsserver |
| Stealth browser | `src-tauri/binaries/browser-agent.exe` | invisible_playwright via PyInstaller |
| Extension host | `src-tauri/ext-host/` | Open VSX extensions |

Dev mode uses Python + `invisible_playwright/src` on `PYTHONPATH` when `browser-agent.exe` is absent.

## Pre-ship smoke test

1. Open folder → chat → agent edits a file → accept diff
2. **+** new chat tab → close tab with **×** → History → **Restore conversation**
3. `/manus` web mission (stealth Firefox launches)
4. Settings → Account → plan card layout
5. Install one Open VSX extension (ESLint or Prettier)
6. F9 breakpoint → debug panel
7. Ctrl+T ghost completion + Tab next-edit toast (if enabled in Settings)

## Optional services (user machine)

- **Ollama** `:11434` — local models
- **aim-proxy** `:1536` — `.aim` context injection
- Cloud keys in Settings → Providers

## Updater

Tauri updater is **disabled** until signing keys are configured in `tauri.conf.json`.
