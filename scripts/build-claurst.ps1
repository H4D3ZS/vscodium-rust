# Build the optional claurst agent CLI and copy it into src-tauri/binaries/
# so the MSI/NSIS installer ships it next to browser-agent.exe.
#
#   powershell -ExecutionPolicy Bypass -File scripts\build-claurst.ps1
#
# Output: src-tauri\binaries\claurst.exe

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent $PSScriptRoot
$workspace = Join-Path $root "claurst\src-rust"
$out = Join-Path $root "src-tauri\binaries"

if (-not (Test-Path $workspace)) {
    Write-Warning "claurst/src-rust not found - skipping claurst bundle."
    exit 0
}

Write-Host "==> Building claurst (release) ..." -ForegroundColor Cyan
Push-Location $workspace
try {
    & cargo build --release --bin claurst
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed with exit code $LASTEXITCODE"
    }
} finally {
    Pop-Location
}

$built = Join-Path $workspace "target\release\claurst.exe"
if (-not (Test-Path -LiteralPath $built)) {
    Write-Error "claurst build failed - $built not found"
}

New-Item -ItemType Directory -Force -Path $out | Out-Null
Copy-Item -LiteralPath $built -Destination (Join-Path $out "claurst.exe") -Force
Write-Host "OK -> src-tauri\binaries\claurst.exe" -ForegroundColor Green
