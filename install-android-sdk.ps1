# Android SDK Setup Script for Windows 10
# Run this in PowerShell to set up everything automatically

$ErrorActionPreference = "Stop"

Write-Host "🤖 Android SDK Installer for Windows 10" -ForegroundColor Cyan
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""

# Configuration
$SDK_ROOT = "C:\Users\HADES\AppData\Local\Android\Sdk"
$TOOLS_URL = "https://dl.google.com/android/repository/commandlinetools-win-11076708_latest.zip"
$TEMP_ZIP = "$env:TEMP\android-command-tools.zip"

Write-Host "📥 Step 1/5: Downloading Command-Line Tools..." -ForegroundColor Yellow
Write-Host "   URL: $TOOLS_URL"
Write-Host "   Size: ~100MB"
Write-Host ""

try {
    Invoke-WebRequest -Uri $TOOLS_URL -OutFile $TEMP_ZIP -UseBasicParsing
    Write-Host "   ✅ Download complete" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Download failed: $_" -ForegroundColor Red
    Write-Host ""
    Write-Host "Manual download: $TOOLS_URL" -ForegroundColor Yellow
    Write-Host "Then extract to: $SDK_ROOT\cmdline-tools\latest" -ForegroundColor Yellow
    exit 1
}

Write-Host ""
Write-Host "📦 Step 2/5: Creating SDK directories..." -ForegroundColor Yellow

# Create directory structure
$null = New-Item -ItemType Directory -Force -Path "$SDK_ROOT\cmdline-tools\latest"
$null = New-Item -ItemType Directory -Force -Path "$SDK_ROOT\platform-tools"
$null = New-Item -ItemType Directory -Force -Path "$SDK_ROOT\emulator"
$null = New-Item -ItemType Directory -Force -Path "$SDK_ROOT\platforms"
$null = New-Item -ItemType Directory -Force -Path "$SDK_ROOT\system-images"

Write-Host "   ✅ Directories created" -ForegroundColor Green

Write-Host ""
Write-Host "🔧 Step 3/5: Extracting tools..." -ForegroundColor Yellow

try {
    Expand-Archive -Path $TEMP_ZIP -DestinationPath "$SDK_ROOT\cmdline-tools" -Force
    
    # Rename cmdline-tools\cmdline-tools to cmdline-tools\latest
    if (Test-Path "$SDK_ROOT\cmdline-tools\cmdline-tools") {
        Rename-Item -Path "$SDK_ROOT\cmdline-tools\cmdline-tools" -NewName "latest" -Force
    }
    
    Write-Host "   ✅ Tools extracted" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Extraction failed: $_" -ForegroundColor Red
    exit 1
}

# Clean up temp file
Remove-Item $TEMP_ZIP -Force -ErrorAction SilentlyContinue

Write-Host ""
Write-Host "🛠️  Step 4/5: Adding to PATH..." -ForegroundColor Yellow

# Add to user PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$newPaths = ";$SDK_ROOT\cmdline-tools\latest\bin;$SDK_ROOT\platform-tools;$SDK_ROOT\emulator;C:\Dev\scrcpy"

if (-not $userPath.Contains("$SDK_ROOT\cmdline-tools")) {
    [Environment]::SetEnvironmentVariable("Path", $userPath + $newPaths, "User")
    Write-Host "   ✅ PATH updated" -ForegroundColor Green
    Write-Host "   ⚠️  RESTART YOUR TERMINAL for PATH changes to take effect!" -ForegroundColor Red
} else {
    Write-Host "   ℹ️  PATH already configured" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "✅ Step 5/5: Installation complete!" -ForegroundColor Green
Write-Host ""
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host "📋 Next Steps (run these commands):" -ForegroundColor Cyan
Write-Host ""
Write-Host "1️⃣  Close this terminal and open a NEW PowerShell window" -ForegroundColor White
Write-Host ""
Write-Host "2️⃣  Accept licenses:" -ForegroundColor White
Write-Host '   sdkmanager --licenses' -ForegroundColor Gray
Write-Host ""
Write-Host "3️⃣  Install components (takes ~10 minutes):" -ForegroundColor White
Write-Host '   sdkmanager "platform-tools" "emulator" "platforms;android-34"' -ForegroundColor Gray
Write-Host ""
Write-Host "4️⃣  Install system image (~1.2GB):" -ForegroundColor White
Write-Host '   sdkmanager "system-images;android-34;google_apis;x86_64"' -ForegroundColor Gray
Write-Host ""
Write-Host "5️⃣  Create your first AVD:" -ForegroundColor White
Write-Host '   avdmanager create avd -n "Pixel_4_API_34" -k "system-images;android-34;google_apis;x86_64" -d "pixel_4"' -ForegroundColor Gray
Write-Host ""
Write-Host "6️⃣  Verify installation:" -ForegroundColor White
Write-Host '   avdmanager list avd' -ForegroundColor Gray
Write-Host ""
Write-Host "=======================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "📁 SDK Location: $SDK_ROOT" -ForegroundColor Cyan
Write-Host "📱 scrcpy Location: C:\Dev\scrcpy" -ForegroundColor Cyan
Write-Host ""
Write-Host "After creating an AVD, you can spawn emulators directly from VSCodium-Rust!" -ForegroundColor Green
Write-Host ""
