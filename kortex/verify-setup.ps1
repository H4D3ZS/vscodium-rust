#!/usr/bin/env pwsh
# Kortex Setup Verification Script

Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "           KORTEX ZERO-TOKEN SETUP VERIFICATION            " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

$KortexRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex"
$ReleaseDir = "$KortexRoot\target\release"

# Check executables
Write-Host "`n[1/4] Checking Built Executables..." -ForegroundColor Yellow
$Executables = @("aim-proxy.exe", "neuraldrive.exe", "hades-tui.exe", "aim-vfs.exe")
$AllExist = $true

foreach ($exe in $Executables) {
    $path = "$ReleaseDir\$exe"
    if (Test-Path $path) {
        $size = (Get-Item $path).Length / 1MB
        Write-Host "  ✓ $exe ($([math]::Round($size, 2)) MB)" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $exe (NOT FOUND)" -ForegroundColor Red
        $AllExist = $false
    }
}

# Check configuration files
Write-Host "`n[2/4] Checking Configuration Files..." -ForegroundColor Yellow
$ConfigFiles = @(
    "$KortexRoot\..\CLAUDE.MD",
    "$KortexRoot\..\kortex\AGENTS.md",
    "$KortexRoot\..\kortex\SETUP.md",
    "$KortexRoot\..\kortex\README.md"
)

foreach ($file in $ConfigFiles) {
    if (Test-Path $file) {
        Write-Host "  ✓ $(Split-Path $file -Leaf)" -ForegroundColor Green
    } else {
        Write-Host "  ✗ $(Split-Path $file -Leaf) (NOT FOUND)" -ForegroundColor Red
    }
}

# Check Ollama status
Write-Host "`n[3/4] Checking Ollama Status..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:11434/api/tags" -TimeoutSec 2 -ErrorAction Stop
    Write-Host "  ✓ Ollama is running on port 11434" -ForegroundColor Green
} catch {
    Write-Host "  ⚠ Ollama not detected on port 11434" -ForegroundColor Yellow
    Write-Host "    Start with: ollama serve" -ForegroundColor Gray
}

# Check AIM Proxy status
Write-Host "`n[4/4] Checking AIM Proxy Status..." -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://127.0.0.1:1536/api/manifest" -TimeoutSec 2 -Method Post -Body '{"objective":"test"}' -ContentType "application/json" -ErrorAction Stop
    Write-Host "  ✓ AIM Proxy is running on port 1536" -ForegroundColor Green
} catch {
    Write-Host "  ⚠ AIM Proxy not running on port 1536" -ForegroundColor Yellow
    Write-Host "    Start with: cd $KortexRoot && .\target\release\aim-proxy.exe" -ForegroundColor Gray
}

# Summary
Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
if ($AllExist) {
    Write-Host "  STATUS: ✓ Kortex is ready for zero-token development" -ForegroundColor Green
} else {
    Write-Host "  STATUS: ⚠ Some components missing - run: cargo build --release" -ForegroundColor Yellow
}
Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

Write-Host "`nQuick Start:" -ForegroundColor Cyan
Write-Host "  1. ollama serve" -ForegroundColor White
Write-Host "  2. cd $KortexRoot" -ForegroundColor White
Write-Host "  3. .\target\release\aim-proxy.exe" -ForegroundColor White
Write-Host "  4. Configure AI client to use http://127.0.0.1:1536" -ForegroundColor White
Write-Host ""
