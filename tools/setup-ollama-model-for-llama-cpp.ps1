# Copy Ollama Model for llama.cpp Usage
# This copies your largest Ollama model blob to a GGUF file

$SOURCE = "C:\Users\HADES\.ollama\models\blobs\sha256-f5ee307a2982106a6eb82b62b2c00b575c9072145a759ae4660378acda8dcf2d"
$DEST_DIR = "C:\models"
$DEST_FILE = "C:\models\ollama-model.gguf"

Write-Host "=== Copying Ollama Model for llama.cpp ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Source: $SOURCE" -ForegroundColor Yellow
Write-Host "Destination: $DEST_FILE" -ForegroundColor Yellow
Write-Host ""

# Create destination directory
if (!(Test-Path $DEST_DIR)) {
    Write-Host "Creating directory: $DEST_DIR" -ForegroundColor Green
    New-Item -ItemType Directory -Path $DEST_DIR | Out-Null
}

# Copy the file
Write-Host "Copying model (this may take a while)..." -ForegroundColor Yellow
Copy-Item -Path $SOURCE -Destination $DEST_FILE -Verbose

Write-Host ""
Write-Host "✓ Model copied successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "Model size: $((Get-Item $DEST_FILE).Length / 1GB) GB" -ForegroundColor Cyan
Write-Host ""
Write-Host "=== Next Steps ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Start llama.cpp server with HADES Bridge:" -ForegroundColor Yellow
Write-Host "   llama-server -m $DEST_FILE -ngl 99 --port 8080 --hades-jit"
Write-Host ""
Write-Host "2. In VSCodium-Rust IDE:" -ForegroundColor Yellow
Write-Host "   - Open Settings (Ctrl+,)"
Write-Host "   - Go to 'Inference Backend'"
Write-Host "   - Select 'llama.cpp + HADES'"
Write-Host "   - Set Model Path to: $DEST_FILE"
Write-Host "   - Set GPU Layers to: 99"
Write-Host "   - Enable 'HADES Bridge' checkbox"
Write-Host "   - Click 'Save Settings'"
Write-Host ""
Write-Host "3. Check connection status - should show 'running'" -ForegroundColor Yellow
Write-Host ""
Write-Host "Note: This is a 22GB model. With HADES Bridge enabled," -ForegroundColor Cyan
Write-Host "the JIT decompression engine will handle the VRAM constraints" -ForegroundColor Cyan
Write-Host "of your 8GB RX 580 by paging layers on-demand." -ForegroundColor Cyan
