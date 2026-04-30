# Fix Android SDK PATH for Windows 10
# Run this ONCE to add avdmanager to your system PATH

$SDK_ROOT = "C:\Users\HADES\AppData\Local\Android\Sdk"

Write-Host "🔧 Fixing Android SDK PATH..." -ForegroundColor Cyan
Write-Host ""

# Get current user PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")

# Paths to add
$pathsToAdd = @(
    "$SDK_ROOT\cmdline-tools\latest\bin",
    "$SDK_ROOT\platform-tools",
    "$SDK_ROOT\emulator",
    "C:\Dev\scrcpy"
)

$added = $false
foreach ($path in $pathsToAdd) {
    if (-not $userPath.Contains($path)) {
        $userPath += ";$path"
        $added = $true
        Write-Host "  ✅ Added: $path" -ForegroundColor Green
    } else {
        Write-Host "  ℹ️  Already exists: $path" -ForegroundColor Gray
    }
}

if ($added) {
    # Save updated PATH
    [Environment]::SetEnvironmentVariable("Path", $userPath, "User")
    
    Write-Host ""
    Write-Host "✅ PATH updated successfully!" -ForegroundColor Green
    Write-Host ""
    Write-Host "⚠️  IMPORTANT: Close ALL terminal windows and reopen for changes to take effect" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "After reopening terminal, verify with:" -ForegroundColor Cyan
    Write-Host "  avdmanager list avd" -ForegroundColor Gray
    Write-Host "  adb version" -ForegroundColor Gray
    Write-Host "  emulator -list-avds" -ForegroundColor Gray
} else {
    Write-Host ""
    Write-Host "ℹ️  PATH already configured correctly" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "SDK Location: $SDK_ROOT" -ForegroundColor Cyan
