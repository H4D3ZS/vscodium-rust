# Kortex Context Recovery Script
# Generated: 20260427-025442
# Message: Initial context recovery setup after unexpected shutdown

Write-Host "
Recovering Kortex context from savepoint: 20260427-025442
" -ForegroundColor Cyan

$KortexRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex"

# Start services if not running
if (-not $ContextState.services.ollama_running) {
    Write-Host "Starting Ollama..." -ForegroundColor Yellow
    Start-Process ollama -ArgumentList "serve" -WindowStyle Normal
    Start-Sleep -Seconds 3
}

if (-not $ContextState.services.aim_proxy_running) {
    Write-Host "Starting AIM Proxy..." -ForegroundColor Yellow
    Start-Process powershell.exe -ArgumentList "-NoExit", "-Command", "cd '$KortexRoot'; & '.\target\release\aim-proxy.exe'" -WindowStyle Normal
    Start-Sleep -Seconds 2
}

# Mount .aim files if they exist
foreach ($aimPath in $ContextState.aim_files) {
    if (Test-Path "$aimPath\memory.aim") {
        Write-Host "Context available: $aimPath\memory.aim" -ForegroundColor Green
    }
}

Write-Host "
✓ Context recovery complete!" -ForegroundColor Green
Write-Host "  Qwen Code can now resume with saved context.
" -ForegroundColor Green
