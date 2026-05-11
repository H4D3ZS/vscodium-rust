# Start ONLY background services (Qwen3-TTS + AIRI 3D)
# Tauri will handle the main Vite dev server automatically

Write-Host "╔══════════════════════════════════════════════════════════╗" -ForegroundColor Cyan
Write-Host "║      Starting AIRI Background Services                   ║" -ForegroundColor Cyan
Write-Host "╚══════════════════════════════════════════════════════════╝" -ForegroundColor Cyan
Write-Host ""

$scriptDir = $PSScriptRoot

# Start Qwen3-TTS Python server (port 8081)
Write-Host "[1/2] Starting Qwen3-TTS Server on port 8081..." -ForegroundColor Green
Start-Process python -ArgumentList "qwen-tts-server.py" -WorkingDirectory $scriptDir -WindowStyle Hidden
Start-Sleep -Seconds 2

# Start AIRI 3D app (port 5174)
Write-Host "[2/2] Starting AIRI 3D App on port 5174..." -ForegroundColor Green
$airiPath = Join-Path $scriptDir "airi/apps/stage-web"
Start-Process npm -ArgumentList "run", "dev" -WorkingDirectory $airiPath -WindowStyle Hidden

Write-Host ""
Write-Host "✅ Background services started" -ForegroundColor Green
Write-Host ""
Write-Host "Services:" -ForegroundColor Cyan
Write-Host "  🎭 AIRI 3D:   http://localhost:5174" -ForegroundColor White
Write-Host "  🎤 Qwen3-TTS: http://localhost:8081" -ForegroundColor White
Write-Host ""

# Exit immediately so Tauri can continue
exit 0
