# HADES-KORTEX Cloud Server Setup
# Run this ONCE on your AMD MI300X cloud server

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  HADES-KORTEX Cloud Server Setup     " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "This script helps you set up Ollama on your cloud server." -ForegroundColor Gray
Write-Host ""
Write-Host "STEP 1: SSH to your cloud server" -ForegroundColor Yellow
Write-Host "  ssh root@your-cloud-ip" -ForegroundColor White
Write-Host ""
Write-Host "STEP 2: Install Ollama (if not installed)" -ForegroundColor Yellow
Write-Host "  curl -fsSL https://ollama.com/install.sh | sh" -ForegroundColor White
Write-Host ""
Write-Host "STEP 3: Start Ollama service" -ForegroundColor Yellow
Write-Host "  ollama serve" -ForegroundColor White
Write-Host ""
Write-Host "  (Run in background: nohup ollama serve &)" -ForegroundColor Gray
Write-Host ""
Write-Host "STEP 4: Pull a model for AIRI" -ForegroundColor Yellow
Write-Host "  Recommended models:" -ForegroundColor Gray
Write-Host ""
Write-Host "  # Fast & Good (7B parameters):" -ForegroundColor Cyan
Write-Host "  ollama pull qwen2.5-coder:7b" -ForegroundColor White
Write-Host ""
Write-Host "  # Better Quality (14B parameters):" -ForegroundColor Cyan
Write-Host "  ollama pull qwen2.5-coder:14b" -ForegroundColor White
Write-Host ""
Write-Host "  # Best Quality (32B parameters, needs more VRAM):" -ForegroundColor Cyan
Write-Host "  ollama pull qwen2.5-coder:32b" -ForegroundColor White
Write-Host ""
Write-Host "STEP 5: Verify model is loaded" -ForegroundColor Yellow
Write-Host "  ollama list" -ForegroundColor White
Write-Host ""
Write-Host "  Should show:" -ForegroundColor Gray
Write-Host "  NAME                  ID              SIZE" -ForegroundColor DarkGray
Write-Host "  qwen2.5-coder:7b      ...             4.7 GB" -ForegroundColor DarkGray
Write-Host ""
Write-Host "STEP 6: Setup SSH Tunnel (on your LOCAL PC)" -ForegroundColor Yellow
Write-Host "  ssh -L 11434:localhost:11434 root@your-cloud-ip" -ForegroundColor White
Write-Host ""
Write-Host "STEP 7: Test connection (on your LOCAL PC)" -ForegroundColor Yellow
Write-Host "  curl http://localhost:11434/api/tags" -ForegroundColor White
Write-Host ""
Write-Host "  Should return your models!" -ForegroundColor Green
Write-Host ""
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "  After setup, AIRI will respond!     " -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Press any key to continue..." -ForegroundColor Gray
$null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
