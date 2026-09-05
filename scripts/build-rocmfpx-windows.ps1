# Build ROCmFPX llama-server natively on Windows for the RX 9060 XT (gfx1200 / RDNA4).
#
# Produces:  src-tauri/binaries/rocmfpx/llama-server.exe  (+ runtime DLLs)
# which the IDE's Kortex Local Inference panel launches as the upstream for the
# KV-slot cache proxy. This is the only binary that can load the Escha
# Q2_0_ROCMFPX GGUF - stock llama.cpp / Lemonade cannot.
#
# Prerequisites (install once, in this order):
#   1. Visual Studio 2022 Build Tools with "Desktop development with C++"
#      + "C++ Clang tools for Windows" + "C++ CMake tools".
#      winget install --id Microsoft.VisualStudio.2022.BuildTools -e --override `
#        "--quiet --wait --add Microsoft.VisualStudio.Workload.VCTools `
#         --add Microsoft.VisualStudio.Component.VC.Llvm.Clang `
#         --add Microsoft.VisualStudio.Component.VC.Llvm.ClangToolset `
#         --add Microsoft.VisualStudio.Component.Windows11SDK.22621 --includeRecommended"
#   2. CMake + Ninja:  winget install Kitware.CMake Ninja-build.Ninja
#   3. AMD HIP SDK for Windows 7.2 (ROCm 10) - natively supports gfx1200 /
#      gfx1201 (RX 9000 / RDNA4). NOT on winget - download from:
#        https://rocm.docs.amd.com/projects/install-on-windows/en/latest/
#      Install (admin, ~5 GB, may reboot), then reopen the shell so HIP_PATH
#      is set. Only needed for the faster HIP build; -VulkanOnly skips it.
#
# Usage (from a normal PowerShell, repo root or anywhere):
#   pwsh -File scripts/build-rocmfpx-windows.ps1
#   pwsh -File scripts/build-rocmfpx-windows.ps1 -Arch gfx1201   # RX 9070 / 9070 XT
#   pwsh -File scripts/build-rocmfpx-windows.ps1 -Jobs 12 -Clean

[CmdletBinding()]
param(
    [string]$Arch = "gfx1200",
    [int]$Jobs = [Environment]::ProcessorCount,
    [switch]$Clean,
    [switch]$VulkanOnly   # skip HIP; build the portable Vulkan path only
)

$ErrorActionPreference = "Stop"
$repoRoot   = Split-Path -Parent $PSScriptRoot
$src        = Join-Path $repoRoot "kortex\ROCmFPX"
$buildDir   = Join-Path $src ("build-{0}-win" -f $Arch)
$stageDir   = Join-Path $repoRoot "src-tauri\binaries\rocmfpx"

function Die($msg) { Write-Host "`n  ERROR: $msg`n" -ForegroundColor Red; exit 1 }
function Info($msg) { Write-Host "  $msg" -ForegroundColor Cyan }

if (-not (Test-Path (Join-Path $src "CMakeLists.txt"))) {
    Die "ROCmFPX source not found at $src. Run:  git -C `"$repoRoot`" submodule update --init --recursive kortex"
}

# --- locate CMake / Ninja -----------------------------------------------------
$cmakeCmd = Get-Command cmake -ErrorAction SilentlyContinue
$cmake = if ($cmakeCmd) { $cmakeCmd.Source } else { $null }
if (-not $cmake) {
    foreach ($c in @("C:\Program Files\CMake\bin\cmake.exe", "$env:ProgramFiles\CMake\bin\cmake.exe")) {
        if (Test-Path $c) { $cmake = $c; break }
    }
    if (-not $cmake) { Die "cmake not found. winget install Kitware.CMake  (then reopen shell)" }
}
$ninjaCmd = Get-Command ninja -ErrorAction SilentlyContinue
$ninja = if ($ninjaCmd) { $ninjaCmd.Source } else { $null }
if (-not $ninja) {
    $cands = @(
        "$env:LOCALAPPDATA\Microsoft\WinGet\Links\ninja.exe",
        "$env:LOCALAPPDATA\Microsoft\WinGet\Packages\Ninja-build.Ninja_Microsoft.Winget.Source_8wekyb3d8bbwe\ninja.exe"
    )
    $cands += (Get-ChildItem "$env:LOCALAPPDATA\Microsoft\WinGet\Packages" -Recurse -Filter ninja.exe -ErrorAction SilentlyContinue | ForEach-Object { $_.FullName })
    foreach ($n in $cands) { if ($n -and (Test-Path $n)) { $ninja = $n; $env:PATH = "$(Split-Path $n);$env:PATH"; break } }
    if (-not $ninja) { Die "ninja not found. winget install Ninja-build.Ninja  (then reopen shell)" }
}
Info "cmake : $cmake"
Info "ninja : $ninja"

# --- locate the MSVC environment (for headers/libs even under clang) --------
$vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
if (-not (Test-Path $vswhere)) { Die "vswhere not found - install VS 2022 Build Tools (see header of this file)" }
$vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.x86.x64 -property installationPath
if (-not $vsPath) { Die "VS 2022 with VC Tools not found - install the 'Desktop development with C++' workload" }
$vcvars = Join-Path $vsPath "VC\Auxiliary\Build\vcvars64.bat"
if (-not (Test-Path $vcvars)) { Die "vcvars64.bat missing under $vsPath" }
Info "vs    : $vsPath"

# Pull the vcvars64 environment into this process.
cmd /c "`"$vcvars`" && set" | ForEach-Object {
    if ($_ -match "^([^=]+)=(.*)$") { Set-Item -Path ("env:{0}" -f $matches[1]) -Value $matches[2] }
}

# --- Vulkan SDK (needed for GGML_VULKAN=ON: headers, vulkan-1.lib, glslc) ---
if (-not $env:VULKAN_SDK -or -not (Test-Path $env:VULKAN_SDK)) {
    $vk = Get-ChildItem "C:\VulkanSDK" -Directory -ErrorAction SilentlyContinue |
          Sort-Object Name -Descending | Select-Object -First 1
    if ($vk) { $env:VULKAN_SDK = $vk.FullName }
}
if ($env:VULKAN_SDK -and (Test-Path $env:VULKAN_SDK)) {
    $env:PATH = "$env:VULKAN_SDK\Bin;$env:PATH"
    Info "vulkan: $env:VULKAN_SDK"
} else {
    Die "Vulkan SDK not found. winget install KhronosGroup.VulkanSDK  (then reopen shell)"
}

# --- HIP toolchain ----------------------------------------------------------
$useHip = -not $VulkanOnly
$hipPath = $env:HIP_PATH
if ($useHip) {
    if (-not $hipPath) {
        $guess = Get-ChildItem "C:\Program Files\AMD\ROCm" -Directory -ErrorAction SilentlyContinue |
                 Sort-Object Name -Descending | Select-Object -First 1
        if ($guess) { $hipPath = $guess.FullName }
    }
    if (-not $hipPath -or -not (Test-Path (Join-Path $hipPath "bin\clang.exe"))) {
        Write-Host ""
        Write-Host "  AMD HIP SDK for Windows not found (no HIP_PATH, no clang.exe under it)." -ForegroundColor Yellow
        Write-Host "  Install it from:" -ForegroundColor Yellow
        Write-Host "    https://rocm.docs.amd.com/projects/install-on-windows/en/latest/" -ForegroundColor Yellow
        Write-Host "  Make sure the package includes RDNA4 / $Arch device libraries." -ForegroundColor Yellow
        Write-Host "  Then reopen PowerShell and re-run this script." -ForegroundColor Yellow
        Write-Host "  (Or pass -VulkanOnly to build the slower portable Vulkan path now.)" -ForegroundColor Yellow
        Die "HIP SDK required for the native ROCm build."
    }
    # gfx1200 device-lib sanity check
    $devlib = Get-ChildItem $hipPath -Recurse -Filter "oclc_isa_version_*.bc" -ErrorAction SilentlyContinue |
              Where-Object { $_.Name -match ($Arch -replace '^gfx','') }
    if (-not $devlib) {
        Write-Host "  WARNING: no device bitcode matching $Arch under $hipPath." -ForegroundColor Yellow
        Write-Host "           Your HIP SDK may predate RDNA4. Build will likely fail at link." -ForegroundColor Yellow
    }
    $env:HIP_PATH = $hipPath
    $env:PATH = "$hipPath\bin;$env:PATH"
    Info "hip   : $hipPath"
}

# --- configure ------------------------------------------------------------
if ($Clean -and (Test-Path $buildDir)) { Info "cleaning $buildDir"; Remove-Item -Recurse -Force $buildDir }

# HIP requires clang (amdclang++). The Vulkan-only path builds fine with MSVC cl,
# which sidesteps clang/MSVC-STL version-pairing errors (STL1000).
$cc  = if ($useHip) { "clang" }   else { "cl" }
$cxx = if ($useHip) { "clang++" } else { "cl" }

$cfg = @(
    "-S", $src, "-B", $buildDir, "-G", "Ninja",
    "-DCMAKE_BUILD_TYPE=Release",
    "-DCMAKE_C_COMPILER=$cc",
    "-DCMAKE_CXX_COMPILER=$cxx",
    "-DGGML_NATIVE=OFF",
    "-DGGML_CUDA=OFF",
    "-DLLAMA_CURL=OFF",
    "-DLLAMA_BUILD_SERVER=ON",
    "-DLLAMA_BUILD_TESTS=OFF",
    "-DLLAMA_BUILD_WEBUI=OFF",
    "-DLLAMA_USE_PREBUILT_WEBUI=OFF",
    "-DGGML_VULKAN=ON"
)
if ($useHip) {
    $cfg += @(
        "-DGGML_HIP=ON",
        "-DGGML_HIP_FORCE_MMQ=ON",
        "-DGGML_HIP_ROCWMMA_FATTN=OFF",
        "-DCMAKE_HIP_ARCHITECTURES=$Arch",
        "-DGPU_TARGETS=$Arch",
        "-DAMDGPU_TARGETS=$Arch"
    )
}

Info "configuring ($Arch, HIP=$useHip, Vulkan=ON)..."
& $cmake @cfg
if ($LASTEXITCODE -ne 0) { Die "cmake configure failed" }

# --- build --------------------------------------------------------------
Info "building with $Jobs jobs (this takes 20-45 min the first time)..."
& $cmake --build $buildDir -j $Jobs --target llama-server llama-cli llama-quantize llama-bench
if ($LASTEXITCODE -ne 0) { Die "build failed" }

# --- stage into the IDE bundle ----------------------------------------
$bin = Join-Path $buildDir "bin"
if (-not (Test-Path (Join-Path $bin "llama-server.exe"))) { Die "llama-server.exe not produced under $bin" }

New-Item -ItemType Directory -Force -Path $stageDir | Out-Null
Get-ChildItem $bin -Filter *.exe | Copy-Item -Destination $stageDir -Force
Get-ChildItem $bin -Filter *.dll | Copy-Item -Destination $stageDir -Force
# HIP runtime DLLs live next to the SDK, not the build tree.
if ($useHip) {
    foreach ($d in "amdhip64_6.dll","amdhip64.dll","rocblas.dll","hipblas.dll","amd_comgr_3.dll","amd_comgr.dll") {
        $p = Join-Path $hipPath "bin\$d"
        if (Test-Path $p) { Copy-Item $p -Destination $stageDir -Force }
    }
    $rocblasLib = Join-Path $hipPath "bin\rocblas\library"
    if (Test-Path $rocblasLib) {
        New-Item -ItemType Directory -Force -Path (Join-Path $stageDir "rocblas\library") | Out-Null
        Copy-Item "$rocblasLib\*" -Destination (Join-Path $stageDir "rocblas\library") -Recurse -Force
    }
}

Write-Host ""
Write-Host "  DONE." -ForegroundColor Green
Write-Host "  Staged: $stageDir\llama-server.exe" -ForegroundColor Green
Write-Host ""
Write-Host "  In the IDE:  Settings -> Kortex / AIM Layer -> Local Inference" -ForegroundColor Green
Write-Host "    Server binary : $stageDir\llama-server.exe" -ForegroundColor Green
Write-Host "    Model (GGUF)  : $env:USERPROFILE\.cache\huggingface\hub\models--cafonez--Escha-W2-35B-A3B-ROCmFP2\snapshots\*\Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf" -ForegroundColor Green
Write-Host "    click Launch." -ForegroundColor Green
Write-Host ""
Write-Host "  Quick CLI smoke test:" -ForegroundColor DarkGray
Write-Host "    $stageDir\llama-server.exe -m <escha.gguf> -dev ROCm0 -ngl 999 -c 8192 --port 8081 --slot-save-path $env:TEMP\kortex-slots" -ForegroundColor DarkGray
