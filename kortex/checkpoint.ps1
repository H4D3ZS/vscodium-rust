#!/usr/bin/env pwsh
# Kortex Context Savepoint - Full conversation state capture
# Usage: .\checkpoint.ps1 [-Message "Work description"] [-Project "project-path"]

param(
    [string]$Message = "Checkpoint",
    [string]$Project = ""
)

$Timestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$CheckpointDir = "C:\Users\HADES\.qwen\checkpoints"
$QwenProjects = "C:\Users\HADES\.qwen\projects"

# Create checkpoint directory
if (-not (Test-Path $CheckpointDir)) {
    New-Item -ItemType Directory -Path $CheckpointDir | Out-Null
}

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "        KORTEX CONTEXT CHECKPOINT                          " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

# Find active chat (most recently modified)
$ActiveChat = Get-ChildItem "$QwenProjects\*\chats\*.jsonl" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$SessionId = [System.IO.Path]::GetFileNameWithoutExtension($ActiveChat.FullName)

Write-Host "  Session ID: $SessionId" -ForegroundColor Gray
Write-Host "  Chat file: $($ActiveChat.FullName)" -ForegroundColor Gray

# Extract conversation summary
$ChatContent = Get-Content $ActiveChat.FullName -Raw | ConvertFrom-Json
$LastUserMessage = ($ChatContent | Where-Object { $_.type -eq "user" -and $_.message.role -eq "user" } | Select-Object -Last 1).message.parts.text
$LastAssistantMessage = ($ChatContent | Where-Object { $_.type -eq "assistant' } | Select-Object -Last 1).message.parts.text

# Collect active plan
$PlanFile = Get-ChildItem "C:\Users\HADES\.qwen\plans\*.md" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$PlanContent = ""
if ($PlanFile) {
    $PlanContent = Get-Content $PlanFile.FullName -Raw
    Write-Host "  Plan: $($PlanFile.Name)" -ForegroundColor Gray
}

# Collect active todos
$TodoFile = Get-ChildItem "C:\Users\HADES\.qwen\todos\*.json" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$TodoContent = ""
if ($TodoFile) {
    $TodoContent = Get-Content $TodoFile.FullName -Raw
    Write-Host "  Todos: $($TodoFile.Name)" -ForegroundColor Gray
}

# Collect modified files in last hour (work in progress)
$RecentFiles = @()
$OneHourAgo = (Get-Date).AddHours(-1)
Get-ChildItem -Path "C:\Users\HADES\Desktop" -Recurse -File -ErrorAction SilentlyContinue | Where-Object { $_.LastWriteTime -gt $OneHourAgo -and $_.Extension -match '\.(rs|ts|tsx|js|jsx|py|md|json|toml|css|html)$' } | ForEach-Object {
    $RecentFiles += @{
        path = $_.FullName
        content = (Get-Content $_.FullName -Raw -ErrorAction SilentlyContinue)
        modified = $_.LastWriteTime.ToString("o")
    }
}

# Build checkpoint state
$CheckpointState = @{
    timestamp = (Get-Date -Format "yyyy-MM-ddTHH:mm:ssZ")
    session_id = $SessionId
    message = $Message
    project = $Project
    conversation = @{
        last_user_message = $LastUserMessage
        last_assistant_message = $LastAssistantMessage
        chat_file = $ActiveChat.FullName
    }
    plan = @{
        file = $PlanFile.FullName
        content = $PlanContent
    }
    todos = @{
        file = $TodoFile.FullName
        content = ($TodoContent | ConvertFrom-Json -ErrorAction SilentlyContinue)
    }
    recent_files = $RecentFiles
    services = @{
        ollama_port = 11434
        aim_proxy_port = 1536
    }
}

# Save checkpoint
$CheckpointFile = "$CheckpointDir\checkpoint-$Timestamp.json"
$CheckpointState | ConvertTo-Json -Depth 20 | Set-Content $CheckpointFile -Encoding UTF8

Write-Host "`n  ✓ Checkpoint created: $CheckpointFile" -ForegroundColor Green
Write-Host "  Message: $Message" -ForegroundColor Gray
Write-Host "  Files captured: $($RecentFiles.Count)" -ForegroundColor Gray

# Create simple recovery script
$RecoveryScript = @"
# Kortex Context Recovery
# Checkpoint: $Timestamp
# Message: $Message

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "        RECOVERING CONTEXT: $Timestamp                     " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

Write-Host "Session: $SessionId" -ForegroundColor White
Write-Host "Message: $Message`n" -ForegroundColor White

# Show last messages
Write-Host "Last User Message:" -ForegroundColor Yellow
Write-Host @"
$LastUserMessage
"@ -ForegroundColor White

if ($PlanContent) {
    Write-Host "`nActive Plan:" -ForegroundColor Yellow
    Write-Host @"
$PlanContent
"@ -ForegroundColor White
}

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  ✓ Context recovered. Continue from above.`n" -ForegroundColor Green
"@

$RecoveryFile = "$CheckpointDir\recover-$Timestamp.ps1"
$RecoveryScript | Set-Content $RecoveryFile -Encoding UTF8
Write-Host "  ✓ Recovery script: $RecoveryFile`n" -ForegroundColor Green

# Cleanup old checkpoints (keep last 20)
$OldCheckpoints = Get-ChildItem "$CheckpointDir\checkpoint-*.json" | Sort-Object LastWriteTime -Descending | Select-Object -Skip 20
foreach ($old in $OldCheckpoints) {
    Remove-Item $old.FullName -Force
    $RecoveryOld = "$CheckpointDir\recover-$($old.BaseName.Substring(12)).ps1"
    if (Test-Path $RecoveryOld) { Remove-Item $RecoveryOld -Force }
}

Write-Host "═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "  ✓ Checkpoint saved (keeping last 20)`n" -ForegroundColor Green
