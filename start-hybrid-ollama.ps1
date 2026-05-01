# HADES-KORTEX Startup Script for Cloud Ollama (AMD MI300X via SSH)
# For users with SSH tunnel to cloud GPU

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  HADES-KORTEX Cloud Ollama Startup   " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$AIM_PROXY_PATH = "C:\Users\HADES\Desktop\vscodium-rust\kortex\target\release\aim-proxy.exe"
$SSH_CLOUD_IP = "your-cloud-ip-here"  # Replace with your actual cloud IP

Write-Host "[1/3] Checking SSH Tunnel to Cloud..." -ForegroundColor Yellow
$sshProcess = Get-Process | Where-Object { $_.ProcessName -eq "ssh" -and $_.CommandLine -like "*-L*11434*" }
if ($sshProcess) {
    Write-Host "  ✓ SSH tunnel already running (PID: $($sshProcess.Id))" -ForegroundColor Green
    Write-Host "  Cloud Ollama: root@$SSH_CLOUD_IP → localhost:11434" -ForegroundColor Gray
} else {
    Write-Host "  ! SSH tunnel not found" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  START SSH TUNNEL FIRST:" -ForegroundColor Cyan
    Write-Host "  ssh -L 11434:localhost:11434 root@$SSH_CLOUD_IP" -ForegroundColor White
    Write-Host ""
    Write-Host "  Keep this terminal open while working!" -ForegroundColor Yellow
    Write-Host ""
}

Write-Host ""
Write-Host "[2/3] Starting AIM Proxy..." -ForegroundColor Yellow

# Start AIM Proxy in background
Start-Process -FilePath $AIM_PROXY_PATH -WindowStyle Hidden
Start-Sleep -Seconds 2

# Verify AIM Proxy started
try {
    $response = Invoke-WebRequest -Uri "http://localhost:1536/api/tags" -TimeoutSec 2 -ErrorAction Stop
    Write-Host "  ✓ AIM Proxy started (port 1536)" -ForegroundColor Green
    Write-Host "  .aim context injection: ENABLED" -ForegroundColor Gray
} catch {
    Write-Host "  ✗ AIM Proxy failed to start!" -ForegroundColor Red
    Write-Host "  Rebuild: cd kortex && cargo build --release" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "[3/3] Configuration Complete!" -ForegroundColor Green

Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Your Setup:                         " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "  Cloud GPU:     AMD MI300X (ROCm)" -ForegroundColor Gray
Write-Host "  SSH Tunnel:    localhost:11434 → cloud:11434" -ForegroundColor Gray
Write-Host "  AIM Proxy:     localhost:1536 (.aim context)" -ForegroundColor Gray
Write-Host ""
Write-Host "  VSCodium-Rust will use:" -ForegroundColor Yellow
Write-Host "  Ollama URL: http://localhost:1536" -ForegroundColor White
Write-Host ""
Write-Host "  This gives you:" -ForegroundColor Green
Write-Host "  ✓ MI300X GPU acceleration (fast!)" -ForegroundColor Gray
Write-Host "  ✓ .aim VFS context injection" -ForegroundColor Gray
Write-Host "  ✓ Token-efficient prompts" -ForegroundColor Gray
Write-Host "  ✓ Thermal governor monitoring" -ForegroundColor Gray
Write-Host ""
Write-Host "Press any key to continue..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")

