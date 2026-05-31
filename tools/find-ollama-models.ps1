# Quick Setup: Use Ollama Model with llama.cpp
# This script helps you configure llama.cpp to use your existing Ollama model

Write-Host "=== Ollama Model Location Finder ===" -ForegroundColor Cyan
Write-Host ""

# Check Ollama models
$OLLAMA_BLOBS = "C:\Users\HADES\.ollama\models\blobs"

if (Test-Path $OLLAMA_BLOBS) {
    Write-Host "Found Ollama models folder" -ForegroundColor Green
    Write-Host ""
    
    # List all blobs with sizes
    $blobs = Get-ChildItem -Path $OLLAMA_BLOBS | Sort-Object LastWriteTime -Descending
    
    Write-Host "Available model blobs (newest first):" -ForegroundColor Cyan
    Write-Host ("{0,-80} {1}" -f "Filename", "Size")
    Write-Host ("-" * 100)
    
    foreach ($blob in $blobs) {
        $sizeGB = [math]::Round($blob.Length / 1GB, 2)
        $name = $blob.Name
        Write-Host ("{0,-80} {1}GB" -f $name, $sizeGB)
    }
    
    Write-Host ""
    Write-Host "To use these models with llama.cpp:" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Option 1: Copy the largest blob (likely your main model)" -ForegroundColor Cyan
    Write-Host "  Copy-Item 'C:\Users\HADES\.ollama\models\blobs\<largest-file>' 'C:\models\model.gguf'"
    Write-Host ""
    Write-Host "Option 2: Export using Ollama CLI (recommended)" -ForegroundColor Cyan
    Write-Host "  ollama show --modelfile <model-name> > Modelfile"
    Write-Host "  Then convert using llama.cpp's convert.py"
    Write-Host ""
    Write-Host "Option 3: Download GGUF directly from HuggingFace" -ForegroundColor Cyan
    Write-Host "  https://huggingface.co/TheBloke"
    Write-Host ""
    
    # Find the largest blob (likely the model)
    $largestBlob = $blobs | Sort-Object Length -Descending | Select-Object -First 1
    if ($largestBlob) {
        $sizeGB = [math]::Round($largestBlob.Length / 1GB, 2)
        Write-Host "`nLargest blob (likely your model):" -ForegroundColor Green
        Write-Host "  Path: $($largestBlob.FullName)"
        Write-Host "  Size: ${sizeGB}GB"
        Write-Host ""
        Write-Host "You can use this file directly with llama.cpp (rename to .gguf):" -ForegroundColor Yellow
        Write-Host "  Copy-Item '$($largestBlob.FullName)' 'C:\models\model.gguf'"
    }
} else {
    Write-Host "Error: Ollama models folder not found at $OLLAMA_BLOBS" -ForegroundColor Red
    Write-Host ""
    Write-Host "Make sure Ollama is installed and you've pulled at least one model:"
    Write-Host "  ollama pull llama3"
}

Write-Host ""
Write-Host "=== Next Steps ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "1. Copy or export your model to GGUF format"
Write-Host "2. Start llama.cpp server:"
Write-Host "   llama-server -m C:\models\model.gguf -ngl 99 --port 8080"
Write-Host ""
Write-Host "3. In VSCodium-Rust:"
Write-Host "   - Open Settings > Inference Backend"
Write-Host "   - Select 'llama.cpp + HADES'"
Write-Host "   - Set Model Path to your GGUF file"
Write-Host "   - Enable HADES Bridge for 8GB optimization"
