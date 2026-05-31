# Start ALL services including Vite dev server (Tauri connects to it)
Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║      Starting AIRI Development Environment               ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

# Get script directory
$scriptDir = $PSScriptRoot

# Start main Vite dev server (port 5173) - Tauri needs this!
Write-Host "[1/3] Starting Vite Dev Server on port 5173..." -ForegroundColor Green
Write-Host "       (Tauri will connect to this)" -ForegroundColor Yellow
Start-Process npm -ArgumentList "run", "dev" -WindowStyle Normal
Start-Sleep -Seconds 3

# Start Qwen3-TTS Python server (port 8081)
Write-Host "[2/3] Starting Qwen3-TTS Server on port 8081..." -ForegroundColor Green
Start-Process python -ArgumentList "qwen-tts-server.py" -WorkingDirectory $scriptDir -WindowStyle Hidden
Start-Sleep -Seconds 2

# Start AIRI 3D app (port 5174)
Write-Host "[3/3] Starting AIRI 3D App on port 5174..." -ForegroundColor Green
$airiPath = Join-Path $scriptDir "airi/apps/stage-web"
Start-Process npm -ArgumentList "run", "dev" -WorkingDirectory $airiPath -WindowStyle Hidden

Write-Host ""
Write-Host "✅ All services starting..." -ForegroundColor Green
Write-Host ""
Write-Host "Services:" -ForegroundColor Cyan
Write-Host "  🌐 Main IDE:      http://localhost:5173 (Tauri will connect)" -ForegroundColor White
Write-Host "  🎭 AIRI 3D:       http://localhost:5174" -ForegroundColor White
Write-Host "  🎤 Qwen3-TTS:     http://localhost:8081" -ForegroundColor White
Write-Host ""
Write-Host "Tauri IDE window will open automatically..." -ForegroundColor Yellow
Write-Host ""

# Exit immediately - don't wait! Tauri will continue
exit 0
