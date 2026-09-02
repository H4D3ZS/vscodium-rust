<#
Build the vendored iOS-over-USB tools from source and stage them into
src-tauri\binaries\ios-tools\ so `npx tauri build` bundles them (Windows).

  ./scripts/build-ios-tools.ps1            # build everything buildable
  ./scripts/build-ios-tools.ps1 go-ios     # build just one

Sources are git submodules under third_party\ios-tools\. Init them first:
  git submodule update --init --recursive third_party/ios-tools/*

libimobiledevice (ideviceiproxy) has no simple Windows source build — use the
prebuilt libimobiledevice-win32 release and drop ideviceiproxy.exe into the
output folder, or set IDEVICEIPROXY_PATH at runtime.
#>
param([string]$Target = "all")

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$Src  = Join-Path $Root "third_party\ios-tools"
$Out  = Join-Path $Root "src-tauri\binaries\ios-tools"
New-Item -ItemType Directory -Force -Path $Out | Out-Null

function Log($m)  { Write-Host "[ios-tools] $m" -ForegroundColor Cyan }
function Warn($m) { Write-Host "[ios-tools] $m" -ForegroundColor Yellow }

function Build-GoIos {
    $dir = Join-Path $Src "go-ios"
    if (-not (Test-Path $dir)) { Warn "go-ios submodule missing - skipping"; return }
    if (-not (Get-Command go -ErrorAction SilentlyContinue)) { Warn "Go toolchain not found - skipping go-ios"; return }
    Log "building go-ios..."
    Push-Location $dir
    try { & go build -trimpath -ldflags "-s -w" -o (Join-Path $Out "ios.exe") . ; Log "-> $Out\ios.exe" }
    finally { Pop-Location }
}

function Build-Zsign {
    $dir = Join-Path $Src "zsign"
    if (-not (Test-Path $dir)) { Warn "zsign submodule missing - skipping"; return }
    if (-not (Get-Command cmake -ErrorAction SilentlyContinue)) { Warn "cmake not found - skipping zsign (needs cmake + OpenSSL)"; return }
    Log "building zsign (cmake)..."
    & cmake -S $dir -B (Join-Path $dir "build") -DCMAKE_BUILD_TYPE=Release | Out-Null
    & cmake --build (Join-Path $dir "build") --config Release
    $bin = Get-ChildItem -Path (Join-Path $dir "build") -Recurse -Filter "zsign.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($bin) { Copy-Item $bin.FullName (Join-Path $Out "zsign.exe") -Force; Log "-> $Out\zsign.exe" }
    else { Warn "zsign.exe not found after build" }
}

function Stage-Ideviceiproxy {
    $sys = Get-Command ideviceiproxy -ErrorAction SilentlyContinue
    if ($sys) { Copy-Item $sys.Source (Join-Path $Out "ideviceiproxy.exe") -Force; Log "-> staged ideviceiproxy.exe from PATH"; return }
    Warn "ideviceiproxy.exe not found. Download libimobiledevice-win32 release and place ideviceiproxy.exe in:"
    Warn "  $Out"
}

switch ($Target) {
    "go-ios"  { Build-GoIos }
    "zsign"   { Build-Zsign }
    "idevice" { Stage-Ideviceiproxy }
    "all"     { Build-GoIos; Build-Zsign; Stage-Ideviceiproxy }
    default   { Write-Host "usage: build-ios-tools.ps1 [go-ios|zsign|idevice|all]"; exit 1 }
}

Log "staged binaries:"
Get-ChildItem $Out | Where-Object { $_.Name -ne "README.md" } | Format-Table Name, Length
