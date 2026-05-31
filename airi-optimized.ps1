# AIRI Optimized Runner Script
# Use this instead of direct ollama run for optimized performance

param(
    [string]$Prompt = "",
    [switch]$Interactive
)

# Set optimized environment variables for this session
$env:OLLAMA_GPU_LAYER = "35"
$env:OLLAMA_NUM_THREAD = "12"
$env:OLLAMA_CONTEXT_LENGTH = "8192"
$env:OLLAMA_KEEP_ALIVE = "-1"
$env:OLLAMA_NUM_BATCH = "512"

Write-Host "🚀 AIRI Optimized Runner" -ForegroundColor Cyan
Write-Host "GPU Layers: 35 | Context: 8192 | Threads: 12" -ForegroundColor Gray
Write-Host ""

if ($Interactive) {
    Write-Host "💬 Interactive mode (type 'exit' to quit)..." -ForegroundColor Yellow
    Write-Host ""
    
    while ($true) {
        $userInput = Read-Host "You"
        
        if ($userInput -eq "exit" -or $userInput -eq "quit") {
            break
        }
        
        if ([string]::IsNullOrWhiteSpace($userInput)) {
            continue
        }
        
        Write-Host ""
        Write-Host "AIRI: " -NoNewline -ForegroundColor Green
        
        # Stream the response
        ollama run hades:latest $userInput 2>&1
        
        Write-Host ""
        Write-Host ""
    }
} else {
    if ([string]::IsNullOrWhiteSpace($Prompt)) {
        Write-Host "Usage: .\airi-optimized.ps1 -Prompt `'your message`'" -ForegroundColor Yellow
        Write-Host "       .\airi-optimized.ps1 -Interactive" -ForegroundColor Yellow
        exit 1
    }
    
    Write-Host "💬 You: $Prompt" -ForegroundColor Blue
    Write-Host ""
    Write-Host "AIRI: " -NoNewline -ForegroundColor Green
    
    ollama run hades:latest $Prompt
}
