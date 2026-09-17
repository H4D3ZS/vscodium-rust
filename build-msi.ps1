# build-msi.ps1 — Build VSCodium-Rust-v1.0.0.msi
# Requires: dotnet tool install -g wix
# Usage: .\build-msi.ps1
# Output: VSCodium-Rust-v1.0.0-x64.msi (current directory)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$root = $PSScriptRoot

Write-Host "[1/4] Checking WiX toolset..." -ForegroundColor Cyan
if (-not (Get-Command wix -ErrorAction SilentlyContinue)) {
    Write-Host "  Installing WiX..."
    dotnet tool install --global wix
}
$wixVer = (wix --version 2>&1)
Write-Host "  WiX: $wixVer" -ForegroundColor Green

# Add the UI extension needed for WixUI_InstallDir
Write-Host "[2/4] Adding WiX UI extension..." -ForegroundColor Cyan
wix extension add WixToolset.UI.wixext/4.0.5 2>&1 | Out-Null
Write-Host "  UI extension ready." -ForegroundColor Green

# Minimal license RTF if missing (WixUI_InstallDir requires one)
$licenseRtf = Join-Path $root "LICENSE.rtf"
if (-not (Test-Path $licenseRtf)) {
    Write-Host "  Creating placeholder LICENSE.rtf..."
    $rtf = "{\rtf1\ansi{\fonttbl\f0\fswiss Helvetica;}\f0\pard " +
           "VSCodium Rust is provided under the MIT License.\par}"
    [System.IO.File]::WriteAllText($licenseRtf, $rtf)
}

Write-Host "[3/4] Building MSI..." -ForegroundColor Cyan
Push-Location $root
try {
    wix build installer.wxs `
        -ext WixToolset.UI.wixext/4.0.5 `
        -o "VSCodium-Rust-v1.0.0-x64.msi" `
        -arch x64
    if ($LASTEXITCODE -ne 0) { throw "wix build failed (exit $LASTEXITCODE)" }
} finally {
    Pop-Location
}

$msiPath = Join-Path $root "VSCodium-Rust-v1.0.0-x64.msi"
$sizeMb   = [math]::Round((Get-Item $msiPath).Length / 1MB, 1)
Write-Host "[4/4] Done! $msiPath ($sizeMb MB)" -ForegroundColor Green
Write-Host ""
Write-Host "Install with:   msiexec /i VSCodium-Rust-v1.0.0-x64.msi" -ForegroundColor Yellow
Write-Host "Silent install: msiexec /i VSCodium-Rust-v1.0.0-x64.msi /quiet" -ForegroundColor Yellow
Write-Host "Uninstall:      msiexec /x VSCodium-Rust-v1.0.0-x64.msi" -ForegroundColor Yellow
