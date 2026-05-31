# Quick Start for Cloud Ollama via SSH Tunnel
# Use this if you have SSH tunnel running but don't want AIM proxy

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  Quick Start: Cloud Ollama (SSH)     " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$SSH_CLOUD_IP = "your-cloud-ip-here"  # Replace with your actual cloud IP

Write-Host "STEP 1: Verify SSH Tunnel" -ForegroundColor Yellow
$sshProcess = Get-Process | Where-Object { $_.ProcessName -eq "ssh" -and $_.CommandLine -like "*-L*11434*" }
if ($sshProcess) {
    Write-Host "  ✓ SSH tunnel running (PID: $($sshProcess.Id))" -ForegroundColor Green
} else {
    Write-Host "  ✗ SSH tunnel NOT running!" -ForegroundColor Red
    Write-Host ""
    Write-Host "  Start it first:" -ForegroundColor Cyan
    Write-Host "  ssh -L 11434:localhost:11434 root@$SSH_CLOUD_IP" -ForegroundColor White
    Write-Host ""
    exit 1
}

Write-Host ""
Write-Host "STEP 2: Test Ollama Connection" -ForegroundColor Yellow
try {
    $response = Invoke-WebRequest -Uri "http://localhost:11434/api/tags" -TimeoutSec 3 -ErrorAction Stop
    $models = ($response.Content | ConvertFrom-Json).models
    Write-Host "  ✓ Cloud Ollama connected!" -ForegroundColor Green
    if ($models) {
        Write-Host "  Models available:" -ForegroundColor Gray
        foreach ($m in $models) {
            Write-Host "    - $($m.name)" -ForegroundColor Gray
        }
    }
} catch {
    Write-Host "  ✗ Cannot connect to Ollama on port 11434" -ForegroundColor Red
    Write-Host "  Check if SSH tunnel is still active" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "STEP 3: Configure VSCodium-Rust" -ForegroundColor Yellow
Write-Host ""
Write-Host "  Your setup is ready! Just make sure:" -ForegroundColor Green
Write-Host "  1. Keep SSH tunnel running in background" -ForegroundColor Gray
Write-Host "  2. In VSCodium-Rust settings, use:" -ForegroundColor Gray
Write-Host "     Ollama URL: http://localhost:11434" -ForegroundColor White
Write-Host "     Connection Mode: direct (not proxy)" -ForegroundColor Gray
Write-Host ""
Write-Host "  NOTE: Some AIRI modules expect port 1536 (AIM proxy)." -ForegroundColor Yellow
Write-Host "  If you see ':1536 connection refused' errors, either:" -ForegroundColor Yellow
Write-Host "  a) Start AIM proxy: .\kortex\target\release\aim-proxy.exe" -ForegroundColor Cyan
Write-Host "  b) Or update AIRI modules to use :11434 instead" -ForegroundColor Cyan
Write-Host ""
Write-Host "========================================" -ForegroundColor Green
Write-Host "  Ready to code with Cloud GPU!       " -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Green
Write-Host ""
Write-Host "Press any key to continue..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
