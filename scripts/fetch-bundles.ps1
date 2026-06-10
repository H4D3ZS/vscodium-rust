# Fetch PortableGit + Hermes skills bundle for IDE installer (Windows).
# Output:
#   src-tauri/bundles/portable-git/     - Git Bash for agent + terminal
#   src-tauri/bundles/ripgrep/          - bundled rg (agent + terminal PATH)
#   src-tauri/bundles/hermes-skills/    - vendored SKILL.md trees for offline install
#
# Skips download when present unless -Force or FORCE_BUNDLE_FETCH=1

param(
    [switch]$Force
)

$ErrorActionPreference = "Stop"
$Root = Split-Path -Parent $PSScriptRoot
$Bundles = Join-Path $Root "src-tauri\bundles"
$GitDir = Join-Path $Bundles "portable-git"
$SkillsOut = Join-Path $Bundles "hermes-skills"
$HermesSrc = Join-Path $Root "hermes-agent"

function Test-PortableGitReady($Dir) {
    return (Test-Path (Join-Path $Dir "bin\bash.exe"))
}

# PortableGit (pinned release - no GitHub API rate limit)
if ($Force -or -not (Test-PortableGitReady $GitDir)) {
    $gitTag = "v2.54.0.windows.1"
    $gitVer = "2.54.0"
    $arch = [System.Runtime.InteropServices.RuntimeInformation]::OSArchitecture
    if ($arch -eq "Arm64") {
        $assetName = "PortableGit-$gitVer-arm64.7z.exe"
    } else {
        $assetName = "PortableGit-$gitVer-64-bit.7z.exe"
    }
    $downloadUrl = "https://github.com/git-for-windows/git/releases/download/$gitTag/$assetName"
    $tmpFile = Join-Path $env:TEMP $assetName

    Write-Host "[fetch-bundles] Downloading $assetName ..."
    Invoke-WebRequest -Uri $downloadUrl -OutFile $tmpFile -UseBasicParsing

    if (Test-Path $GitDir) { Remove-Item -Recurse -Force $GitDir }
    New-Item -ItemType Directory -Path $GitDir -Force | Out-Null

    Write-Host "[fetch-bundles] Extracting PortableGit to $GitDir ..."
    $outArg = "-o`"$GitDir`""
    $extractProc = Start-Process -FilePath $tmpFile -ArgumentList @($outArg, "-y") -NoNewWindow -Wait -PassThru
    if ($extractProc.ExitCode -ne 0) {
        throw "PortableGit extraction failed (exit $($extractProc.ExitCode))"
    }
    Remove-Item -Force $tmpFile -ErrorAction SilentlyContinue

    if (-not (Test-PortableGitReady $GitDir)) {
        throw "PortableGit extraction did not produce bin\bash.exe"
    }
    Write-Host "[fetch-bundles] OK - PortableGit ready."
} else {
    Write-Host "[fetch-bundles] portable-git present - skip (use -Force to re-fetch)."
}

# Hermes skills snapshot (copy from submodule - no network)
if (-not (Test-Path $HermesSrc)) {
    Write-Warning "[fetch-bundles] hermes-agent/ missing - bundled skills skipped."
    exit 0
}

if ($Force -or -not (Test-Path (Join-Path $SkillsOut "skills"))) {
    if (Test-Path $SkillsOut) { Remove-Item -Recurse -Force $SkillsOut }
    New-Item -ItemType Directory -Path $SkillsOut -Force | Out-Null

    $skillsSrc = Join-Path $HermesSrc "skills"
    $optSrc = Join-Path $HermesSrc "optional-skills"
    if (Test-Path $skillsSrc) {
        Write-Host "[fetch-bundles] Copying hermes-agent/skills ..."
        Copy-Item -Recurse -Force $skillsSrc (Join-Path $SkillsOut "skills")
    }
    if (Test-Path $optSrc) {
        Write-Host "[fetch-bundles] Copying hermes-agent/optional-skills ..."
        Copy-Item -Recurse -Force $optSrc (Join-Path $SkillsOut "optional-skills")
    }
    Write-Host "[fetch-bundles] OK - hermes-skills bundle ready."
} else {
    Write-Host "[fetch-bundles] hermes-skills present - skip."
}

Write-Host "[fetch-bundles] Bundles ready for tauri build."

# Ripgrep (rg) — agent grep tool + Git Bash PATH
Write-Host "[fetch-bundles] Fetching ripgrep bundle …"
$fetchRg = Join-Path $Root "scripts\fetch-ripgrep.ts"
if (Test-Path $fetchRg) {
    node $fetchRg
    if ($LASTEXITCODE -ne 0) { throw "fetch-ripgrep.ts failed" }
} else {
    Write-Warning "[fetch-bundles] fetch-ripgrep.ts missing"
}
