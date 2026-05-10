#!/usr/bin/env pwsh
# Quick Context Save - Creates a single recoverable context file
# Usage: .\save.ps1 "What we're working on"

param(
    [string]$Context = "Work checkpoint"
)

$SaveDir = "C:\Users\HADES\.qwen\sessions"
$Timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

if (-not (Test-Path $SaveDir)) { New-Item -ItemType Directory -Path $SaveDir | Out-Null }

# Find latest chat
$LatestChat = Get-ChildItem "C:\Users\HADES\.qwen\projects\*\chats\*.jsonl" | Sort-Object LastWriteTime -Descending | Select-Object -First 1
$SessionId = [System.IO.Path]::GetFileNameWithoutExtension($LatestChat.Name)

# Find latest plan
$LatestPlan = Get-ChildItem "C:\Users\HADES\.qwen\plans\*.md" | Sort-Object LastWriteTime -Descending | Select-Object -First 1

# Find latest todos  
$LatestTodo = Get-ChildItem "C:\Users\HADES\.qwen\todos\*.json" | Sort-Object LastWriteTime -Descending | Select-Object -First 1

# Read chat history line by line (JSONL format)
$RecentExchanges = @()
$lines = Get-Content $LatestChat.FullName
$count = 0
for ($i = $lines.Count - 1; $i -ge 0 -and $count -lt 10; $i--) {
    try {
        $line = $lines[$i]
        if ([string]::IsNullOrWhiteSpace($line)) { continue }
        $entry = $line | ConvertFrom-Json
        if ($entry.type -eq "user" -and $entry.message) {
            $text = $entry.message.parts.text
            if ($text) {
                $RecentExchanges += @{ role = "user"; text = $text }
                $count++
            }
        }
        elseif ($entry.type -eq "assistant" -and $entry.message) {
            $textParts = $entry.message.parts | Where-Object { $_.text }
            $text = ($textParts | ForEach-Object { $_.text }) -join ""
            if ($text) {
                $RecentExchanges += @{ role = "assistant"; text = $text }
                $count++
            }
        }
    }
    catch {
        # Skip malformed lines
        continue
    }
}
# Reverse to get chronological order
$RecentExchanges = $RecentExchanges | Sort-Object -Property { [array]::IndexOf($RecentExchanges, $_) } -Descending

# Build context file
$PlanText = if ($LatestPlan) { Get-Content $LatestPlan.FullName -Raw } else { "No active plan" }
$TodoText = if ($LatestTodo) { Get-Content $LatestTodo.FullName -Raw } else { "No todos" }

$ConversationText = ($RecentExchanges | ForEach-Object { "$($_.role.ToUpper()): $($_.text)" }) -join "`n`n"

$Content = @"
# QWEN CODE SESSION CHECKPOINT
# Created: $Timestamp
# Session: $SessionId
# Context: $Context

## ACTIVE PLAN
$PlanText

## TODOS
$TodoText

## RECENT CONVERSATION
$ConversationText

---
# TO RESUME: Run powershell -ExecutionPolicy Bypass -File "C:\Users\HADES\.qwen\sessions\latest\recover.ps1"
"@

# Save to timestamped file
$SessionTimestamp = Get-Date -Format "yyyyMMdd-HHmmss"
$SaveFile = "$SaveDir\session-$SessionTimestamp.md"
$Content | Set-Content $SaveFile -Encoding UTF8

# Also save as 'latest' for quick access
$LatestDir = "$SaveDir\latest"
if (-not (Test-Path $LatestDir)) { New-Item -ItemType Directory -Path $LatestDir | Out-Null }
$Content | Set-Content "$LatestDir\context.md" -Encoding UTF8

# Create simple recovery script
$RecoverScript = @"
Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "        SESSION RECOVERY                                   " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan
Get-Content "$LatestDir\context.md" -Raw
Write-Host "`n═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan
"@

$RecoverScript | Set-Content "$LatestDir\recover.ps1" -Encoding UTF8

Write-Host "`n✓ Saved: $SaveFile" -ForegroundColor Green
Write-Host "✓ Quick recover: powershell -ExecutionPolicy Bypass -File '$LatestDir\recover.ps1'`n" -ForegroundColor Green
