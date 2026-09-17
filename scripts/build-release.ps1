# build-release.ps1 — Windows release build & Inno Setup packaging
# Usage: powershell -ExecutionPolicy Bypass -File scripts\build-release.ps1
#        powershell -ExecutionPolicy Bypass -File scripts\build-release.ps1 -SkipBuild
#        powershell -ExecutionPolicy Bypass -File scripts\build-release.ps1 -PackageOnly

param(
    [switch]$SkipBuild,
    [switch]$PackageOnly
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
Set-Location $Root

$Version = if ($env:VERSION) { $env:VERSION } else { "1.0.0" }

Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  VSCodium Rust — Windows Release Build" -ForegroundColor Cyan
Write-Host "  Version: $Version" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan

# ── Build ─────────────────────────────────────────────────────────────────────
if (-not $SkipBuild -and -not $PackageOnly) {
    Write-Host ""
    Write-Host "▶ Building native binary (release)..." -ForegroundColor Yellow
    $env:RUST_MIN_STACK = "67108864"
    cargo build --release --manifest-path src-native/Cargo.toml
    if ($LASTEXITCODE -ne 0) { throw "Cargo build failed" }
    Write-Host "✅ Build complete" -ForegroundColor Green
}

$Binary = Join-Path $Root "src-native\target\release\vscodium-rust.exe"
if (-not (Test-Path $Binary)) {
    $Binary = Join-Path $Root "target\release\vscodium-rust.exe"
}
if (-not (Test-Path $Binary)) {
    throw "Binary not found at $Binary. Run without -SkipBuild first."
}

# ── Assemble release bundle ───────────────────────────────────────────────────
$BundleDir = Join-Path $Root "release-bundle"
Write-Host ""
Write-Host "▶ Assembling release bundle → $BundleDir" -ForegroundColor Yellow

New-Item -ItemType Directory -Force -Path $BundleDir | Out-Null

# Core binaries
Copy-Item -Force $Binary (Join-Path $BundleDir "vscodium-rust.exe")
Copy-Item -Force $Binary (Join-Path $BundleDir "vscodium-ip-panel.exe")

# Supplementary binaries
foreach ($item in @("ios.exe", "frida.exe")) {
    $src = Join-Path $Root $item
    if (Test-Path $src) { Copy-Item -Force $src (Join-Path $BundleDir $item) }
}

# Supplementary directories
foreach ($dir in @("binaries", "bundles", "ext-host", "devimages")) {
    $src = Join-Path $Root $dir
    if (Test-Path $src) { Copy-Item -Force -Recurse $src (Join-Path $BundleDir $dir) }
}

# Config files
foreach ($cfg in @("mcp_config.json")) {
    $src = Join-Path $Root $cfg
    if (Test-Path $src) { Copy-Item -Force $src (Join-Path $BundleDir $cfg) }
}

Write-Host "✅ Release bundle assembled" -ForegroundColor Green

# ── Package with Inno Setup ──────────────────────────────────────────────────
Write-Host ""
Write-Host "▶ Building Inno Setup installer..." -ForegroundColor Yellow

$DistDir = Join-Path $Root "dist"
New-Item -ItemType Directory -Force -Path $DistDir | Out-Null

# Find ISCC.exe
$IsccPaths = @(
    "C:\Program Files (x86)\Inno Setup 6\ISCC.exe",
    "C:\Program Files\Inno Setup 6\ISCC.exe",
    "$env:LOCALAPPDATA\Programs\Inno Setup 6\ISCC.exe"
)
$Iscc = $null
foreach ($p in $IsccPaths) {
    if (Test-Path $p) { $Iscc = $p; break }
}
if (-not $Iscc) {
    # Try PATH
    $Iscc = (Get-Command iscc -ErrorAction SilentlyContinue).Source
}
if (-not $Iscc) {
    throw "Inno Setup not found. Install from https://jrsoftware.org/isinfo.php"
}

Write-Host "  Using: $Iscc" -ForegroundColor DarkGray

# Update version in .iss dynamically
$IssFile = Join-Path $Root "installer.iss"
$OutputName = "VSCodium-Rust-Setup-v${Version}-windows-x64"

& $Iscc /DMyAppVersion="$Version" /O"$DistDir" /F"$OutputName" "$IssFile"
if ($LASTEXITCODE -ne 0) { throw "Inno Setup compilation failed" }

Write-Host "✅ Installer → $DistDir\$OutputName.exe" -ForegroundColor Green

# ── Checksums ─────────────────────────────────────────────────────────────────
Write-Host ""
Write-Host "▶ Generating checksums..." -ForegroundColor Yellow

$artifacts = Get-ChildItem (Join-Path $DistDir "*") -File -Include *.exe, *.zip | Where-Object { $_.Length -gt 0 }
if ($artifacts.Count -gt 0) {
    $checksums = foreach ($f in $artifacts) {
        $hash = (Get-FileHash $f.FullName -Algorithm SHA256).Hash.ToLower()
        "$hash  $($f.Name)"
    }
    $checksums | Out-File -Encoding utf8 (Join-Path $DistDir "SHA256SUMS.txt")
    Write-Host "✅ SHA256SUMS.txt → $DistDir\" -ForegroundColor Green
}

Write-Host ""
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  ✅ Windows release build complete!" -ForegroundColor Green
Write-Host "  Artifacts: $DistDir\" -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════════" -ForegroundColor Cyan
