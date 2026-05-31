# Launch llama-server with Kortex GAC scheduling.
#
# Without arguments, this script:
#   1. Locates the model under -ModelPath (defaults to C:\models\ollama-model.gguf).
#   2. Looks for a `<model>.geometry.aim` profile next to it, or generates one
#      via the IDE's Tauri kortex_gac_profile command.
#   3. Plans tiers against the user's VRAM budget (-VramMb, default 8192 for
#      RX 580 8GB) at theta -Theta (default 0.85).
#   4. Spawns llama-server with the planner's --n-gpu-layers and
#      --override-tensor flags applied.
#
# Profiling and planning live in the Tauri app's `kortex_gac` module. This script
# is the CLI shim — it shells out to a minimal helper binary built from the same
# Rust crate (`vscode-rust-app` exposes `kortex-gac-cli` as a side bin), or
# falls back to using the already-running IDE if that binary is not built.
#
# The "geometry of consolidation" theorem is what makes this not a meme:
# spread tensors keep their bandwidth (GPU), tight tensors lose theirs gracefully
# (CPU). Same VRAM as `-ngl N`, but the bytes that land on the fast path are the
# bytes that actually need it.

param(
    [string]$ModelPath = "C:\models\ollama-model.gguf",
    [string]$ServerBinary = "llama-server.exe",
    [int]$VramMb = 8192,
    [double]$Theta = 0.85,
    [ValidateSet("vulkan","cuda","rocm","metal","sycl")]
    [string]$Backend = "vulkan",
    [int]$Port = 8081,
    [int]$CtxSize = 8192,
    [int]$Threads = 0,
    [int]$BatchSize = 512,
    [string]$ExtraArgs = "",
    [switch]$RefreshProfile,
    [switch]$DryRun,
    [string]$KortexCli = "",

    # ─── Kortex Disk KV Cache (KDKVC) ──────────────────────────────────────
    # Boots the prefix-cache proxy in front of llama-server so coding agents
    # don't re-prefill the same 25K-token system prompt on every restart.
    [switch]$NoKvCache,
    [int]$KvCachePort = 8090,
    [string]$KvCacheBaseDir = "$env:USERPROFILE\.kortex\kvcache",
    [int]$KvCacheMaxGb = 16,
    [string]$KvCacheCli = ""
)

$ErrorActionPreference = "Stop"

function Resolve-KortexCli {
    if ($KortexCli -ne "" -and (Test-Path $KortexCli)) { return (Resolve-Path $KortexCli).Path }
    $candidates = @(
        ".\src-tauri\target\release\kortex-gac-cli.exe",
        ".\target\release\kortex-gac-cli.exe",
        "$env:USERPROFILE\.cargo\bin\kortex-gac-cli.exe"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { return (Resolve-Path $c).Path }
    }
    return $null
}

function Resolve-KvCacheCli {
    if ($KvCacheCli -ne "" -and (Test-Path $KvCacheCli)) { return (Resolve-Path $KvCacheCli).Path }
    $candidates = @(
        ".\src-tauri\target\release\kortex-kvcache-cli.exe",
        ".\target\release\kortex-kvcache-cli.exe",
        "$env:USERPROFILE\.cargo\bin\kortex-kvcache-cli.exe"
    )
    foreach ($c in $candidates) {
        if (Test-Path $c) { return (Resolve-Path $c).Path }
    }
    return $null
}

function Wait-LlamaHealth([string]$baseUrl, [int]$timeoutSec = 60) {
    $deadline = (Get-Date).AddSeconds($timeoutSec)
    while ((Get-Date) -lt $deadline) {
        try {
            $resp = Invoke-WebRequest -Uri "$baseUrl/health" -UseBasicParsing -TimeoutSec 2 -ErrorAction SilentlyContinue
            if ($resp.StatusCode -eq 200) { return $true }
        } catch { }
        Start-Sleep -Milliseconds 500
    }
    return $false
}

function Get-ProfilePath([string]$model) {
    return "$model.geometry.aim"
}

Write-Host "=== Kortex GAC Launcher ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "Model:   $ModelPath" -ForegroundColor Yellow
Write-Host "VRAM:    $VramMb MB" -ForegroundColor Yellow
Write-Host "Theta:   $Theta" -ForegroundColor Yellow
Write-Host "Backend: $Backend" -ForegroundColor Yellow
Write-Host ""

if (-not (Test-Path $ModelPath)) {
    Write-Host "ERROR: model not found at $ModelPath" -ForegroundColor Red
    exit 1
}

$cli = Resolve-KortexCli
$profilePath = Get-ProfilePath $ModelPath

if ($null -eq $cli) {
    Write-Host "kortex-gac-cli.exe was not found. Two options:" -ForegroundColor Yellow
    Write-Host "  1. Build it:  cargo build --release --bin kortex-gac-cli  (from src-tauri/)" -ForegroundColor Yellow
    Write-Host "  2. Profile via the IDE (Settings -> Inference Backend -> 'Profile model')" -ForegroundColor Yellow
    Write-Host "     then re-run this script with -RefreshProfile:`$false; the IDE writes the profile next to the GGUF." -ForegroundColor Yellow
    Write-Host ""
    if (-not (Test-Path $profilePath)) {
        Write-Host "No profile found at $profilePath. Aborting." -ForegroundColor Red
        exit 1
    }
    Write-Host "Found existing profile, will plan from it without the CLI helper." -ForegroundColor Green
} else {
    Write-Host "Using kortex-gac-cli at $cli" -ForegroundColor Green
    if ($RefreshProfile -or -not (Test-Path $profilePath)) {
        Write-Host "Profiling model (this can take ~30s for 35B, ~90s for 70B on a desktop CPU)..." -ForegroundColor Yellow
        & $cli profile --model $ModelPath
        if ($LASTEXITCODE -ne 0) {
            Write-Host "Profiler failed with code $LASTEXITCODE" -ForegroundColor Red
            exit $LASTEXITCODE
        }
    } else {
        Write-Host "Profile cached at $profilePath" -ForegroundColor Green
    }
}

# Plan: ask the CLI to produce a JSON tier plan and the rendered argv list.
$planJsonPath = "$ModelPath.plan.json"
if ($null -ne $cli) {
    & $cli plan --profile $profilePath --vram-mb $VramMb --theta $Theta --backend $Backend --output $planJsonPath
    if ($LASTEXITCODE -ne 0) {
        Write-Host "Planner failed with code $LASTEXITCODE" -ForegroundColor Red
        exit $LASTEXITCODE
    }
} else {
    Write-Host "Skipping plan step (no CLI). Caller must invoke kortex_gac_plan via the IDE first." -ForegroundColor Yellow
    if (-not (Test-Path $planJsonPath)) {
        Write-Host "No plan file at $planJsonPath. Aborting." -ForegroundColor Red
        exit 1
    }
}

# Read the rendered argv from the plan file.
$plan = Get-Content $planJsonPath -Raw | ConvertFrom-Json
$nGpuLayers = $plan.n_gpu_layers
$overrides  = @($plan.overrides)

$bufferName = switch ($Backend) {
    "cuda"   { "CUDA0" }
    "rocm"   { "ROCm0" }
    "vulkan" { "Vulkan0" }
    "metal"  { "Metal" }
    "sycl"   { "SYCL0" }
    default  { "CPU" }
}

$serverArgs = @(
    "-m", $ModelPath,
    "--host", "127.0.0.1",
    "--port", $Port,
    "-c", $CtxSize,
    "-b", $BatchSize,
    "--n-gpu-layers", $nGpuLayers
)

if ($Threads -gt 0) {
    $serverArgs += @("-t", $Threads)
}

foreach ($ov in $overrides) {
    $tgt = if ($ov.buffer -eq "Cpu") { "CPU" } else { $bufferName }
    $serverArgs += @("--override-tensor", "$($ov.pattern)=$tgt")
}

# When KDKVC is enabled, llama-server needs --slot-save-path so the proxy can
# round-trip slot binaries via /slots/{id}?action=save|restore.
if (-not $NoKvCache) {
    $slotDir = Join-Path $KvCacheBaseDir "slots"
    if (-not (Test-Path $slotDir)) { New-Item -Path $slotDir -ItemType Directory -Force | Out-Null }
    $serverArgs += @("--slot-save-path", $slotDir)
}

if ($ExtraArgs -ne "") {
    $serverArgs += ($ExtraArgs -split " ")
}

Write-Host ""
Write-Host "=== GAC Tier Plan ===" -ForegroundColor Cyan
Write-Host ("  GPU bytes : {0:N2} GB" -f ($plan.total_gpu_bytes / 1GB))
Write-Host ("  CPU bytes : {0:N2} GB" -f ($plan.total_cpu_bytes / 1GB))
Write-Host ("  d_bar_crit: {0:N3}" -f $plan.d_bar_critical)
Write-Host ("  spread→GPU: {0}" -f $plan.routing_counts.spread_to_gpu)
Write-Host ("  tight→CPU : {0}" -f $plan.routing_counts.tight_to_cpu)
Write-Host ""
Write-Host "=== llama-server invocation ===" -ForegroundColor Cyan
Write-Host "$ServerBinary $($serverArgs -join ' ')" -ForegroundColor Gray
Write-Host ""

if ($DryRun) {
    Write-Host "(dry run — server not started)" -ForegroundColor Yellow
    exit 0
}

# Resolve the server binary on PATH if it was passed without an absolute path.
if (-not (Test-Path $ServerBinary)) {
    $resolved = Get-Command $ServerBinary -ErrorAction SilentlyContinue
    if ($null -ne $resolved) { $ServerBinary = $resolved.Source }
}

if (-not (Test-Path $ServerBinary)) {
    Write-Host "ERROR: cannot find $ServerBinary on PATH. Install llama.cpp or pass -ServerBinary <path>." -ForegroundColor Red
    exit 1
}

Write-Host "Starting llama-server..." -ForegroundColor Green

if ($NoKvCache) {
    # Old behaviour: this script blocks on llama-server in the foreground.
    & $ServerBinary @serverArgs
    exit $LASTEXITCODE
}

# KDKVC path: spawn llama-server in the background, wait for /health, then
# run the cache proxy in the foreground. Killing the proxy (Ctrl-C) tears
# llama-server down too so the user gets one-key shutdown.
$kvCli = Resolve-KvCacheCli
if ($null -eq $kvCli) {
    Write-Host "Warning: kortex-kvcache-cli.exe not found — falling back to direct llama-server." -ForegroundColor Yellow
    Write-Host "  Build it with:  cargo build --release --bin kortex-kvcache-cli  (from src-tauri/)" -ForegroundColor Yellow
    & $ServerBinary @serverArgs
    exit $LASTEXITCODE
}

if (-not (Test-Path $KvCacheBaseDir)) {
    New-Item -Path $KvCacheBaseDir -ItemType Directory -Force | Out-Null
}

Write-Host "  llama-server  → http://127.0.0.1:$Port (background)" -ForegroundColor Gray
Write-Host "  kvcache proxy → http://127.0.0.1:$KvCachePort (foreground, Ctrl-C to stop)" -ForegroundColor Gray
Write-Host "  cache base    → $KvCacheBaseDir (LRU cap $KvCacheMaxGb GB)" -ForegroundColor Gray
Write-Host ""

$llamaProc = Start-Process -FilePath $ServerBinary -ArgumentList $serverArgs -PassThru -NoNewWindow
try {
    $upstream = "http://127.0.0.1:$Port"
    if (-not (Wait-LlamaHealth $upstream 90)) {
        Write-Host "ERROR: llama-server never became healthy at $upstream/health" -ForegroundColor Red
        if (-not $llamaProc.HasExited) { $llamaProc.Kill() }
        exit 1
    }
    Write-Host "llama-server healthy. Booting Kortex KV cache proxy..." -ForegroundColor Green

    & $kvCli serve `
        --upstream $upstream `
        --host "127.0.0.1" `
        --port $KvCachePort `
        --base $KvCacheBaseDir `
        --max-gb $KvCacheMaxGb
}
finally {
    if ($null -ne $llamaProc -and -not $llamaProc.HasExited) {
        Write-Host "Stopping llama-server (pid $($llamaProc.Id))..." -ForegroundColor Yellow
        try { $llamaProc.Kill() } catch { }
    }
}
