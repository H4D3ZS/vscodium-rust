#!/usr/bin/env pwsh
# Start Kortex Zero-Token Development Environment
# Run this before starting Qwen Code development sessions

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "        STARTING KORTEX ZERO-TOKEN ENVIRONMENT             " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

$KortexRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex"
$ProxyPath = "$KortexRoot\target\release\aim-proxy.exe"
$OllamaPath = "ollama"

# Check if executables exist
if (-not (Test-Path $ProxyPath)) {
    Write-Host "[ERROR] AIM Proxy not found at: $ProxyPath" -ForegroundColor Red
    Write-Host "Run: cd $KortexRoot && cargo build --release`n" -ForegroundColor Yellow
    exit 1
}

# Start Ollama (if not already running)
Write-Host "[1/3] Checking Ollama..." -ForegroundColor Yellow
try {
    $null = Invoke-WebRequest -Uri "http://127.0.0.1:11434/api/tags" -TimeoutSec 2 -ErrorAction Stop
    Write-Host "  ✓ Ollama already running on port 11434" -ForegroundColor Green
} catch {
    Write-Host "  → Starting Ollama..." -ForegroundColor Gray
    Start-Process $OllamaPath -ArgumentList "serve" -WindowStyle Normal
    Start-Sleep -Seconds 3
    Write-Host "  ✓ Ollama started" -ForegroundColor Green
}

# Start AIM Proxy (if not already running)
Write-Host "`n[2/3] Checking AIM Proxy..." -ForegroundColor Yellow
try {
    $null = Invoke-WebRequest -Uri "http://127.0.0.1:1536/" -TimeoutSec 2 -ErrorAction Stop
    Write-Host "  ✓ AIM Proxy already running on port 1536" -ForegroundColor Green
} catch {
    Write-Host "  → Starting AIM Proxy..." -ForegroundColor Gray
    Start-Process "powershell.exe" -ArgumentList "-NoExit", "-Command", "cd '$KortexRoot'; & '.\target\release\aim-proxy.exe'" -WindowStyle Normal
    Start-Sleep -Seconds 2
    Write-Host "  ✓ AIM Proxy started" -ForegroundColor Green
}

# Verify setup
Write-Host "`n[3/3] Verifying configuration..." -ForegroundColor Yellow
& "$KortexRoot\verify-setup.ps1"

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  ✓ Zero-token environment ready!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

Write-Host "Configuration for Qwen Code:" -ForegroundColor Cyan
Write-Host "  • Ollama endpoint: http://127.0.0.1:1536 (AIM Proxy)" -ForegroundColor White
Write-Host "  • Context injection: Automatic via .aim files" -ForegroundColor White
Write-Host "  • Token savings: ~99.9% via prefix caching" -ForegroundColor White
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "  1. Open NeuralDrive: $KortexRoot\target\release\neuraldrive.exe" -ForegroundColor White
Write-Host "  2. Click 'Mount Project' and select your codebase" -ForegroundColor White
Write-Host "  3. Start coding with Qwen Code - context is automatic!" -ForegroundColor White
Write-Host ""
