#!/usr/bin/env pwsh
# NeuralDrive Diagnostic Script

Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan
Write-Host "           NEURALDRIVE DIAGNOSTIC TOOL                     " -ForegroundColor Cyan
Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

$NeuralDriveRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex\neuraldrive"
$WorkspaceRoot = "C:\Users\HADES\Desktop\vscodium-rust\kortex"
$ExePath = "$WorkspaceRoot\target\release\neuraldrive.exe"
$DistPath = "$NeuralDriveRoot\dist"

# Check 1: Frontend Build
Write-Host "[1/5] Checking Frontend Build..." -ForegroundColor Yellow
if (Test-Path $DistPath) {
    $files = Get-ChildItem $DistPath -Recurse -File
    Write-Host "  ✓ dist/ folder exists with $($files.Count) files" -ForegroundColor Green
    
    # Check for critical files
    $indexHtml = Get-ChildItem $DistPath -Filter "index.html" -File
    $jsFiles = Get-ChildItem $DistPath -Filter "*.js" -Recurse -File
    $cssFiles = Get-ChildItem $DistPath -Filter "*.css" -Recurse -File
    
    if ($indexHtml) { Write-Host "    ✓ index.html found" -ForegroundColor Green }
    else { Write-Host "    ✗ index.html MISSING" -ForegroundColor Red }
    
    if ($jsFiles) { Write-Host "    ✓ JS bundle found ($($jsFiles[0].Name))" -ForegroundColor Green }
    else { Write-Host "    ✗ JS bundle MISSING" -ForegroundColor Red }
    
    if ($cssFiles) { Write-Host "    ✓ CSS bundle found ($($cssFiles[0].Name))" -ForegroundColor Green }
    else { Write-Host "    ✗ CSS bundle MISSING" -ForegroundColor Red }
} else {
    Write-Host "  ✗ dist/ folder NOT FOUND" -ForegroundColor Red
    Write-Host "    Run: cd $NeuralDriveRoot && npm run build" -ForegroundColor Gray
}

# Check 2: Tauri Backend
Write-Host "`n[2/5] Checking Tauri Backend..." -ForegroundColor Yellow
if (Test-Path $ExePath) {
    $size = (Get-Item $ExePath).Length / 1MB
    Write-Host "  ✓ neuraldrive.exe exists ($([math]::Round($size, 2)) MB)" -ForegroundColor Green
} else {
    Write-Host "  ✗ neuraldrive.exe NOT FOUND" -ForegroundColor Red
    Write-Host "    Run: cd $WorkspaceRoot && cargo build --release" -ForegroundColor Gray
}

# Check 3: Node Modules
Write-Host "`n[3/5] Checking Node Modules..." -ForegroundColor Yellow
if (Test-Path "$NeuralDriveRoot\node_modules") {
    Write-Host "  ✓ node_modules exists" -ForegroundColor Green
    
    # Check critical packages
    $criticalPackages = @("react", "react-dom", "three", "react-force-graph-3d", "@tauri-apps/api")
    foreach ($pkg in $criticalPackages) {
        $pkgPath = "$NeuralDriveRoot\node_modules\$pkg"
        if (Test-Path $pkgPath) {
            Write-Host "    ✓ $pkg installed" -ForegroundColor Green
        } else {
            Write-Host "    ✗ $pkg MISSING" -ForegroundColor Red
        }
    }
} else {
    Write-Host "  ✗ node_modules NOT FOUND" -ForegroundColor Red
    Write-Host "    Run: cd $NeuralDriveRoot && npm install" -ForegroundColor Gray
}

# Check 4: Tauri Config
Write-Host "`n[4/5] Checking Tauri Configuration..." -ForegroundColor Yellow
$tauriConfig = "$NeuralDriveRoot\src-tauri\tauri.conf.json"
if (Test-Path $tauriConfig) {
    Write-Host "  ✓ tauri.conf.json found" -ForegroundColor Green
    
    # Try to parse and show window config
    try {
        $config = Get-Content $tauriConfig -Raw | ConvertFrom-Json
        $window = $config.app.windows[0]
        Write-Host "    Window: $($window.width)x$($window.height)" -ForegroundColor Cyan
        Write-Host "    Title: $($window.title)" -ForegroundColor Cyan
    } catch {
        Write-Host "    ⚠ Could not parse config: $_" -ForegroundColor Yellow
    }
} else {
    Write-Host "  ✗ tauri.conf.json NOT FOUND" -ForegroundColor Red
}

# Check 5: Rust Dependencies
Write-Host "`n[5/5] Checking Rust Dependencies..." -ForegroundColor Yellow
$cargoToml = "$NeuralDriveRoot\src-tauri\Cargo.toml"
if (Test-Path $cargoToml) {
    Write-Host "  ✓ Cargo.toml found" -ForegroundColor Green
    
    # Check for critical dependencies
    $content = Get-Content $cargoToml -Raw
    $criticalCrates = @("tauri", "tauri-plugin-dialog", "tauri-plugin-opener", "walkdir", "notify")
    foreach ($crate in $criticalCrates) {
        if ($content -match "$crate") {
            Write-Host "    ✓ $crate in dependencies" -ForegroundColor Green
        } else {
            Write-Host "    ⚠ $crate not found" -ForegroundColor Yellow
        }
    }
} else {
    Write-Host "  ✗ Cargo.toml NOT FOUND" -ForegroundColor Red
}

# Summary
Write-Host "`n═══════════════════════════════════════════════════════════" -ForegroundColor Cyan

$allGood = (Test-Path $DistPath) -and (Test-Path $ExePath) -and (Test-Path "$NeuralDriveRoot\node_modules")

if ($allGood) {
    Write-Host "  STATUS: ✓ NeuralDrive is ready to run!" -ForegroundColor Green
    Write-Host "`n  To launch NeuralDrive:" -ForegroundColor Cyan
    Write-Host "  1. Run: & '$ExePath'" -ForegroundColor White
    Write-Host "  2. Or double-click neuraldrive.exe in Explorer" -ForegroundColor White
} else {
    Write-Host "  STATUS: ⚠ Some components are missing" -ForegroundColor Yellow
    Write-Host "`n  Quick Fix:" -ForegroundColor Cyan
    Write-Host "  cd $WorkspaceRoot" -ForegroundColor White
    Write-Host "  cargo build --release" -ForegroundColor White
}

Write-Host "═══════════════════════════════════════════════════════════`n" -ForegroundColor Cyan

# Bonus: Test Launch
Write-Host "BONUS: Want to test launch NeuralDrive now?" -ForegroundColor Cyan
Write-Host "  Press 'Y' to launch, any other key to exit..." -ForegroundColor Gray
$key = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
if ($key.VirtualKeyCode -eq 89 -or $key.VirtualKeyCode -eq 121) {  # Y or y
    Write-Host "`nLaunching NeuralDrive..." -ForegroundColor Green
    Start-Process $ExePath
}
