# Product release build — typecheck, bundle LSP + browser-agent, Tauri installer.
# Usage: powershell -ExecutionPolicy Bypass -File scripts\release.ps1
#
# Prerequisites (first run only):
#   npm ci
#   pip install -e invisible_playwright pyinstaller playwright
#   playwright install firefox

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

Write-Host "==> TypeScript typecheck" -ForegroundColor Cyan
npm run typecheck

$LspRa = Join-Path $Root "src-tauri\binaries\lsp\rust-analyzer\rust-analyzer.exe"
if (-not (Test-Path $LspRa)) {
    Write-Host "==> Fetching LSP binaries (first run)" -ForegroundColor Cyan
    powershell -ExecutionPolicy Bypass -File scripts\fetch-lsp-binaries.ps1
}

$Sidecar = Join-Path $Root "src-tauri\binaries\browser-agent.exe"
if (-not (Test-Path $Sidecar)) {
    if (Test-Path (Join-Path $Root "invisible_playwright")) {
        Write-Host "==> Freezing browser-agent.exe (invisible_playwright)" -ForegroundColor Cyan
        powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1
    } else {
        Write-Warning "invisible_playwright/ missing — installer will need Python for browser features."
    }
}

Write-Host "==> Frontend + Tauri production build" -ForegroundColor Cyan
npm run build
npx tauri build

Write-Host ""
Write-Host "OK — installer under src-tauri\target\release\bundle\" -ForegroundColor Green
