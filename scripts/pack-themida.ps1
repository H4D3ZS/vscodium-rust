# Post-build: protect the IDE exe with Themida (anti reverse-engineering).
# Themida is COMMERCIAL — install it + create a .tmd project, then point the two
# vars below at them. Run AFTER `npx tauri build`, BEFORE code-signing.
#
#   powershell -ExecutionPolicy Bypass -File scripts\pack-themida.ps1

$ErrorActionPreference = "Stop"

# >>> EDIT THESE <<<
$ThemidaCli  = "C:\Program Files\Themida\Themida64.exe"          # or Themida.exe (x86)
$ThemidaProj = "C:\Users\HADES\Desktop\vscodium-rust\protect.tmd" # your saved project

$root   = Split-Path -Parent $PSScriptRoot
$exe    = Join-Path $root "src-tauri\target\release\bundle\nsis\VSCodium Rust IDE_0.1.0_x64-setup.exe"
# To protect the inner app exe instead of the installer, target:
#   src-tauri\target\release\vscode-rust-app.exe  (then re-run `tauri build`)

if (-not (Test-Path $ThemidaCli))  { Write-Error "Themida CLI not found at $ThemidaCli - install it / edit this script." }
if (-not (Test-Path $ThemidaProj)) { Write-Error "Themida project not found at $ThemidaProj - create one in the Themida GUI first." }
if (-not (Test-Path $exe))         { Write-Error "Build artifact not found: $exe - run `npx tauri build` first." }

Write-Host "==> Protecting $exe with Themida" -ForegroundColor Cyan
# Themida CLI: /protect <project.tmd> <input> <output>  (flags vary by version — check Themida docs)
& $ThemidaCli /protect $ThemidaProj $exe $exe
Write-Host "OK -> protected in place. Now code-sign (see docs/RELEASE.md)." -ForegroundColor Green
