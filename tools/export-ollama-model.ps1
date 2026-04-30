# Export Ollama Model to GGUF for llama.cpp
# 
# This script exports your Ollama model to GGUF format
# so it can be used with direct llama.cpp inference

$OLLAMA_MODEL_NAME = "llama3"  # Change this to your model name
$OUTPUT_PATH = "C:\Users\HADES\Desktop\models"

Write-Host "=== Ollama to GGUF Export Tool ===" -ForegroundColor Cyan
Write-Host ""

# Step 1: Find the model blob
Write-Host "Step 1: Finding model in Ollama registry..." -ForegroundColor Yellow
$OLLAMA_BLOBS = "C:\Users\HADES\.ollama\models\blobs"

if (Test-Path $OLLAMA_BLOBS) {
    $blobs = Get-ChildItem -Path $OLLAMA_BLOBS | Sort-Object LastWriteTime -Descending
    Write-Host "Found $($blobs.Count) model blobs" -ForegroundColor Green
    
    # Show recent blobs
    Write-Host "`nRecent model blobs:" -ForegroundColor Cyan
    $blobs | Select-Object -First 5 | ForEach-Object {
        $size = [math]::Round($_.Length / 1GB, 2)
        Write-Host "  $($_.Name) - ${size}GB"
    }
} else {
    Write-Host "Error: Ollama blobs folder not found at $OLLAMA_BLOBS" -ForegroundColor Red
    exit 1
}

Write-Host ""
Write-Host "Step 2: Export model using Ollama show command..." -ForegroundColor Yellow

# Step 2: Export the model
# Ollama doesn't directly export to GGUF, so we need to use ollama show
$manifestPath = "C:\Users\HADES\.ollama\models\manifests\registry.ollama.ai\library\$OLLAMA_MODEL_NAME\latest"

if (Test-Path $manifestPath) {
    $manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json
    Write-Host "Model manifest found for: $OLLAMA_MODEL_NAME" -ForegroundColor Green
    
    # Get the model digest
    if ($manifest.layers) {
        $modelLayer = $manifest.layers | Where-Object { $_.mediaType -eq "application/vnd.ollama.image.model" }
        if ($modelLayer) {
            $digest = $modelLayer.digest
            Write-Host "Model digest: $digest" -ForegroundColor Cyan
            
            # Find the blob file
            $blobFile = Get-ChildItem -Path $OLLAMA_BLOBS -Filter "*$digest*" | Select-Object -First 1
            
            if ($blobFile) {
                Write-Host "Found model blob: $($blobFile.Name)" -ForegroundColor Green
                $size = [math]::Round($blobFile.Length / 1GB, 2)
                Write-Host "Size: ${size}GB" -ForegroundColor Cyan
                
                # Create output directory
                if (!(Test-Path $OUTPUT_PATH)) {
                    New-Item -ItemType Directory -Path $OUTPUT_PATH | Out-Null
                    Write-Host "Created output directory: $OUTPUT_PATH" -ForegroundColor Green
                }
                
                # Copy the blob (it's already in GGUF-like format)
                $outputFile = Join-Path $OUTPUT_PATH "$OLLAMA_MODEL_NAME.gguf"
                Write-Host "`nCopying model to: $outputFile" -ForegroundColor Yellow
                Copy-Item -Path $blobFile.FullName -Destination $outputFile
                
                Write-Host "`n✓ Model exported successfully!" -ForegroundColor Green
                Write-Host ""
                Write-Host "To use with llama.cpp:" -ForegroundColor Cyan
                Write-Host "  llama-server -m $outputFile -ngl 99 --port 8080"
                Write-Host ""
                Write-Host "Then in VSCodium-Rust:" -ForegroundColor Cyan
                Write-Host "  1. Open Settings > Inference Backend"
                Write-Host "  2. Select 'llama.cpp + HADES'"
                Write-Host "  3. Set Model Path to: $outputFile"
                Write-Host "  4. Enable HADES Bridge"
                Write-Host "  5. Click Save Settings"
            } else {
                Write-Host "Error: Could not find blob file for digest $digest" -ForegroundColor Red
            }
        }
    }
} else {
    Write-Host "Error: Model manifest not found at $manifestPath" -ForegroundColor Red
    Write-Host ""
    Write-Host "Available models in Ollama:" -ForegroundColor Cyan
    ollama list
}

Write-Host ""
Write-Host "=== Export Complete ===" -ForegroundColor Cyan
