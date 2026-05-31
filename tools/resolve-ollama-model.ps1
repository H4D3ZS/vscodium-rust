# Resolve an Ollama model name (e.g. "hades:latest", "qwen2.5-coder-abliterate:7b",
# "HammerAI/neuraldaredevil-abliterated:latest") to the absolute path of its GGUF
# blob on disk. Optionally hardlinks the blob to a friendly path so llama-server
# logs read sanely.
#
# Ollama stores models as content-addressed blobs under:
#   %USERPROFILE%\.ollama\models\blobs\sha256-<digest>
#
# The mapping from name → blob digest lives in a Docker-style manifest JSON at:
#   %USERPROFILE%\.ollama\models\manifests\<registry>\<namespace>\<name>\<tag>
#
# The manifest has a `layers` array. The GGUF binary is the layer with
# mediaType "application/vnd.ollama.image.model".
#
# Usage:
#   .\tools\resolve-ollama-model.ps1 -Model hades:latest
#   .\tools\resolve-ollama-model.ps1 -Model qwen2.5-coder-abliterate:7b -Namespace huihui_ai
#   .\tools\resolve-ollama-model.ps1 -Model hades:latest -LinkTo C:\models\hades.gguf
#
# Emits the resolved blob path to stdout so it pipes cleanly into
# launch-kortex.ps1:
#
#   $modelPath = .\tools\resolve-ollama-model.ps1 -Model hades:latest -Quiet
#   .\tools\launch-kortex.ps1 -ModelPath $modelPath

param(
    [Parameter(Mandatory = $true)]
    [string]$Model,
    [string]$Namespace = "",
    [string]$Registry = "registry.ollama.ai",
    [string]$OllamaRoot = "$env:USERPROFILE\.ollama\models",
    [string]$LinkTo = "",
    [switch]$Quiet,
    [switch]$Hardlink
)

$ErrorActionPreference = "Stop"

function Write-Status([string]$msg, [ConsoleColor]$color = 'Yellow') {
    if (-not $Quiet) { Write-Host $msg -ForegroundColor $color }
}

# ── parse name:tag ──────────────────────────────────────────────────────────
$name = $Model
$tag = "latest"
if ($Model -match '^(.*):([^:/]+)$') {
    $name = $Matches[1]
    $tag = $Matches[2]
}

# ── namespace detection ─────────────────────────────────────────────────────
# Names like "HammerAI/foo" embed the namespace; standalone names use "library"
# unless the user passes -Namespace.
if ($name.Contains('/')) {
    $parts = $name.Split('/', 2)
    $resolvedNamespace = $parts[0]
    $resolvedName = $parts[1]
} elseif ($Namespace -ne "") {
    $resolvedNamespace = $Namespace
    $resolvedName = $name
} else {
    $resolvedNamespace = "library"
    $resolvedName = $name
}

$manifestPath = Join-Path $OllamaRoot "manifests\$Registry\$resolvedNamespace\$resolvedName\$tag"
Write-Status "manifest: $manifestPath"

if (-not (Test-Path $manifestPath)) {
    # Fallback: some installs put custom models directly under <name>\<tag>
    # without the namespace folder, mirroring how `ollama create` writes them.
    $altManifest = Join-Path $OllamaRoot "manifests\$Registry\$resolvedName\$tag"
    if (Test-Path $altManifest) {
        $manifestPath = $altManifest
        Write-Status "manifest (fallback): $manifestPath"
    } else {
        # Walk the manifests tree as a last resort. Useful when the namespace
        # convention has drifted across Ollama versions.
        $hit = Get-ChildItem -Path (Join-Path $OllamaRoot "manifests") -Recurse -File `
            -ErrorAction SilentlyContinue `
            | Where-Object { $_.Name -eq $tag -and $_.Directory.Name -eq $resolvedName } `
            | Select-Object -First 1
        if ($null -ne $hit) {
            $manifestPath = $hit.FullName
            Write-Status "manifest (walked): $manifestPath"
        } else {
            Write-Host "ERROR: cannot find manifest for $Model (looked for $manifestPath)." -ForegroundColor Red
            exit 1
        }
    }
}

# ── parse manifest, find the model layer ────────────────────────────────────
try {
    $manifest = Get-Content $manifestPath -Raw | ConvertFrom-Json
} catch {
    Write-Host "ERROR: cannot parse manifest as JSON: $_" -ForegroundColor Red
    exit 1
}

$modelLayer = $manifest.layers | Where-Object {
    $_.mediaType -eq "application/vnd.ollama.image.model"
} | Select-Object -First 1

if ($null -eq $modelLayer) {
    Write-Host "ERROR: no model layer in $manifestPath (mediaType application/vnd.ollama.image.model missing)." -ForegroundColor Red
    exit 1
}

# digest looks like "sha256:<hex>"; the on-disk blob is "sha256-<hex>".
$digest = $modelLayer.digest -replace '^sha256:', ''
$blobName = "sha256-$digest"
$blobPath = Join-Path $OllamaRoot "blobs\$blobName"

if (-not (Test-Path $blobPath)) {
    Write-Host "ERROR: blob $blobPath not found." -ForegroundColor Red
    Write-Host "       Manifest references it but it's missing on disk." -ForegroundColor Red
    exit 1
}

$sizeGb = [math]::Round((Get-Item $blobPath).Length / 1GB, 2)
Write-Status "blob:     $blobPath" 'Green'
Write-Status "size:     $sizeGb GB" 'Cyan'

# ── optional friendly path ──────────────────────────────────────────────────
if ($LinkTo -ne "") {
    $linkDir = Split-Path $LinkTo -Parent
    if (-not (Test-Path $linkDir)) {
        New-Item -ItemType Directory -Path $linkDir -Force | Out-Null
    }
    if (Test-Path $LinkTo) {
        Remove-Item $LinkTo -Force
    }
    if ($Hardlink) {
        # NTFS hardlink → no extra disk space, opens like the original.
        New-Item -ItemType HardLink -Path $LinkTo -Target $blobPath | Out-Null
        Write-Status "hardlinked → $LinkTo" 'Green'
    } else {
        # Symlink (needs Developer Mode or admin on Windows).
        try {
            New-Item -ItemType SymbolicLink -Path $LinkTo -Target $blobPath | Out-Null
            Write-Status "symlinked → $LinkTo" 'Green'
        } catch {
            Write-Status "symlink failed ($($_.Exception.Message)); falling back to hardlink." 'Yellow'
            New-Item -ItemType HardLink -Path $LinkTo -Target $blobPath | Out-Null
            Write-Status "hardlinked → $LinkTo" 'Green'
        }
    }
    Write-Output $LinkTo
} else {
    Write-Output $blobPath
}
