# Build the local `claude-bug-bounty` Ollama model used by the IDE's Security mode.
# Usage:  pwsh scripts/ollama/build-claude-bug-bounty.ps1
$ErrorActionPreference = 'Stop'
$dir = Split-Path -Parent $MyInvocation.MyCommand.Path
$modelfile = Join-Path $dir 'claude-bug-bounty.Modelfile'

# Pull the abliterated base first if it's missing (uncensored Gemma-4 12B).
$base = 'huihui_ai/gemma-4-abliterated:12b-q4_K'
$have = (ollama list) -match [regex]::Escape($base)
if (-not $have) {
    Write-Host "Pulling base model $base ..." -ForegroundColor Cyan
    ollama pull $base
}

Write-Host "Building claude-bug-bounty from $modelfile ..." -ForegroundColor Cyan
ollama create claude-bug-bounty -f $modelfile
Write-Host "Done. Select 'Security' mode in the IDE — it auto-uses claude-bug-bounty." -ForegroundColor Green
