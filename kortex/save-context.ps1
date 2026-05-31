#!/usr/bin/env pwsh
# Kortex Context Savepoint - Creates timestamped gist token snapshots
# Usage: .\save-context.ps1 [-Message "Optional description"]

param(
    [string]$Message = "Auto-savepoint"
)

$Timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$SavepointDir = "C:\Users\HADES\Desktop\vscodium-rust\kortex\.savepoints"
$KortexRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex"

# Create savepoint directory if not exists
if (-not (Test-Path $SavepointDir)) {
    New-Item -ItemType Directory -Path $SavepointDir | Out-Null
}

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "        KORTEX CONTEXT SAVEPOINT                           " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

# Collect context state
$ContextState = @{
    timestamp = (Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ")
    message = $Message
    kortex_version = "0.1.0"
    services = @{
        ollama_port = 11434
        aim_proxy_port = 1536
    }
    paths = @{
        kortex_root = $KortexRoot
        memory_paths = @(
            "$KortexRoot\.aim",
            "C:\Users\HADES\Desktop\kortex\.aim",
            ".\.aim"
        )
    }
}

# Check service status
try {
    $null = Invoke-WebRequest -Uri "http://127.0.0.1:11434/api/tags" -TimeoutSec 2 -ErrorAction Stop
    $ContextState.services.ollama_running = $true
    Write-Host "  ✓ Ollama running on port 11434" -ForegroundColor Green
} catch {
    $ContextState.services.ollama_running = $false
    Write-Host "  ✗ Ollama not running" -ForegroundColor Yellow
}

try {
    $null = Invoke-WebRequest -Uri "http://127.0.0.1:1536/" -TimeoutSec 2 -ErrorAction Stop
    $ContextState.services.aim_proxy_running = $true
    Write-Host "  ✓ AIM Proxy running on port 1536" -ForegroundColor Green
} catch {
    $ContextState.services.aim_proxy_running = $false
    Write-Host "  ✗ AIM Proxy not running" -ForegroundColor Yellow
}

# Check for .aim files
$AimFiles = @()
foreach ($path in $ContextState.paths.memory_paths) {
    if (Test-Path "$path\memory.aim") {
        $AimFiles += $path
        Write-Host "  ✓ Found .aim context: $path\memory.aim" -ForegroundColor Green
    }
}
$ContextState.aim_files = $AimFiles

# Save context snapshot
$SavepointFile = "$SavepointDir\savepoint-$Timestamp.json"
$ContextState | ConvertTo-Json -Depth 10 | Set-Content $SavepointFile -Encoding UTF8

Write-Host "`n  ✓ Savepoint created: $SavepointFile" -ForegroundColor Green

# Create recovery script for this savepoint
$RecoveryScript = @"
# Kortex Context Recovery Script
# Generated: $Timestamp
# Message: $Message

Write-Host "`nRecovering Kortex context from savepoint: $Timestamp`n" -ForegroundColor Cyan

`$KortexRoot = "$KortexRoot"

# Start services if not running
if (-not `$ContextState.services.ollama_running) {
    Write-Host "Starting Ollama..." -ForegroundColor Yellow
    Start-Process ollama -ArgumentList "serve" -WindowStyle Normal
    Start-Sleep -Seconds 3
}

if (-not `$ContextState.services.aim_proxy_running) {
    Write-Host "Starting AIM Proxy..." -ForegroundColor Yellow
    Start-Process powershell.exe -ArgumentList "-NoExit", "-Command", "cd '`$KortexRoot'; & '.\target\release\aim-proxy.exe'" -WindowStyle Normal
    Start-Sleep -Seconds 2
}

# Mount .aim files if they exist
foreach (`$aimPath in `$ContextState.aim_files) {
    if (Test-Path "`$aimPath\memory.aim") {
        Write-Host "Context available: `$aimPath\memory.aim" -ForegroundColor Green
    }
}

Write-Host "`n✓ Context recovery complete!" -ForegroundColor Green
Write-Host "  Qwen Code can now resume with saved context.`n" -ForegroundColor Green
"@

$RecoveryScriptFile = "$SavepointDir\recover-$Timestamp.ps1"
$RecoveryScript | Set-Content $RecoveryScriptFile -Encoding UTF8

Write-Host "  ✓ Recovery script created: $RecoveryScriptFile" -ForegroundColor Green

# Cleanup old savepoints (keep last 10)
$OldSavepoints = Get-ChildItem "$SavepointDir\savepoint-*.json" | Sort-Object LastWriteTime -Descending | Select-Object -Skip 10
foreach ($old in $OldSavepoints) {
    Remove-Item $old.FullName -Force
    Write-Host "  Cleaned up old savepoint: $($old.Name)" -ForegroundColor Gray
}

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  ✓ Savepoint saved successfully!" -ForegroundColor Green
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

Write-Host "To recover this context later:" -ForegroundColor Cyan
Write-Host "  powershell -ExecutionPolicy Bypass -File `"$RecoveryScriptFile`"`n" -ForegroundColor White
