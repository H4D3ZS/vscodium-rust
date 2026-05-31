# Quick Build: llama.cpp for Windows
# Builds without HADES Bridge (can be added later)

Write-Host "=== Quick Build: llama.cpp ===" -ForegroundColor Cyan
Write-Host ""

$LLAMA_DIR = "C:\Users\HADES\Desktop\vscodium-rust\kortex\llama.cpp"
$BUILD_DIR = "$LLAMA_DIR\build"

# Create build directory
if (!(Test-Path $BUILD_DIR)) {
    mkdir $BUILD_DIR | Out-Null
}

cd $BUILD_DIR

Write-Host "Configuring..." -ForegroundColor Yellow

# Configure without HADES
cmake .. `
    -DCMAKE_BUILD_TYPE=Release `
    -DLLAMA_HADES_BRIDGE=OFF `
    -DGGML_CUDA=OFF `
    -Wno-dev

if ($LASTEXITCODE -ne 0) { exit 1 }

Write-Host "`nBuilding (5-10 minutes)..." -ForegroundColor Yellow
cmake --build . --config Release

if ($LASTEXITCODE -eq 0) {
    Write-Host "`n✓ Build Complete!" -ForegroundColor Green
    Write-Host ""
    Write-Host "Run server with:" -ForegroundColor Cyan
    Write-Host "  .\bin\Release\llama-server.exe -m C:\models\ollama-model.gguf -ngl 99 --port 8080"
} else {
    Write-Host "`n✗ Build Failed" -ForegroundColor Red
}
