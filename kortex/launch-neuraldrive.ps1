#!/usr/bin/env pwsh
# Launch NeuralDrive

$ExePath = "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\neuraldrive.exe"

if (Test-Path $ExePath) {
    Write-Host "`n🚀 Launching NeuralDrive...`n" -ForegroundColor Green
    Start-Process $ExePath
} else {
    Write-Host "`n❌ neuraldrive.exe not found!`n" -ForegroundColor Red
    Write-Host "Building..." -ForegroundColor Yellow
    cd "C:\Users\HADES\Desktop\vscodium-rust\kortex"
    cargo build --release
    if ($LASTEXITCODE -eq 0) {
        Write-Host "`n🚀 Launching NeuralDrive...`n" -ForegroundColor Green
        Start-Process $ExePath
    }
}
