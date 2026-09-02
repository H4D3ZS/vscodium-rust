# Freeze the invisible_playwright browser sidecar into a standalone EXE so the
# installer ships a working browser-automation engine WITHOUT requiring the end
# user to install Python. Run once before `npx tauri build`.
#
#   powershell -ExecutionPolicy Bypass -File scripts\build-sidecar.ps1
#
# Output: src-tauri\binaries\browser-agent.exe  (bundled via bundle.resources)

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$agent = Join-Path $root "src-tauri\sidecars\browser_agent.py"
$ip    = Join-Path $root "invisible_playwright"
$out   = Join-Path $root "src-tauri\binaries"

Write-Host "==> Ensuring PyInstaller + deps" -ForegroundColor Cyan
python -m pip install --quiet --upgrade pyinstaller playwright | Out-Null
# Install invisible_playwright from the in-repo source so it is importable/frozen.
if (Test-Path $ip) { python -m pip install --quiet -e $ip | Out-Null }
# Firefox is downloaded at first run by the sidecar; ensure the browser is present
# in CI by uncommenting the next line (adds ~80MB you may want to bundle separately):
# python -m playwright install firefox | Out-Null

Write-Host "==> Freezing browser_agent.py -> browser-agent.exe" -ForegroundColor Cyan
New-Item -ItemType Directory -Force -Path $out | Out-Null
python -m PyInstaller `
  --onefile `
  --noconsole `
  --name browser-agent `
  --distpath $out `
  --workpath (Join-Path $env:TEMP "cf-sidecar-build") `
  --specpath (Join-Path $env:TEMP "cf-sidecar-spec") `
  --collect-all invisible_playwright `
  --collect-all playwright `
  $agent

if (Test-Path (Join-Path $out "browser-agent.exe")) {
  Write-Host "OK -> src-tauri\binaries\browser-agent.exe" -ForegroundColor Green
} else {
  Write-Error "Freeze failed - browser-agent.exe not produced"
}
