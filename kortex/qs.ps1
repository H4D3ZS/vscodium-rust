#!/usr/bin/env pwsh
# Quick Save Command - Type 'qs "message"' to save context
# Adds itself to PATH for easy access

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProfilePath = $PROFILE.CurrentUserCurrentHost

# Add to PATH if not already
if ($env:PATH -notlike "*$ScriptDir*") {
    $env:PATH = "$ScriptDir;$env:PATH"
}

# Save function for easy access
function global:qs {
    param([string]$msg = "Checkpoint")
    & "$ScriptDir\save.ps1" $msg
}

# Add to profile for future sessions
$ProfileLine = "Set-Alias qs '$ScriptDir\save.ps1'"
if (Test-Path $ProfilePath) {
    $ProfileContent = Get-Content $ProfilePath -Raw
    if ($ProfileContent -notlike "*qs*") {
        "`n# Kortex quick save alias" | Add-Content $ProfilePath
        $ProfileLine | Add-Content $ProfilePath
    }
} else {
    "`n# Kortex quick save alias`n$ProfileLine" | Set-Content $ProfilePath
}

Write-Host "`n✓ Kortex save command ready!" -ForegroundColor Green
Write-Host "  Usage: qs `"Your message`"" -ForegroundColor White
Write-Host "  Or:    .\save.ps1 `"Your message`"`n" -ForegroundColor White
