# Build tgrep (our fork, microsoft/tgrep MIT-licensed) from source into
# src-tauri/bundles/tgrep/tgrep.exe -- the same destination
# domain::workspace::ide_shell::ensure_tgrep_installed already expects, so no
# Rust-side change is needed when the build source changes.
#
# We build from the vendored submodule instead of downloading Microsoft's
# release binary: end users get a binary built by us from source we can
# audit and modify, not a redistributed third-party artifact. It also means
# `tgrep/` (the fork) is the place to land the "optimize tgrep to work hand
# in hand with kortex" work -- trigram-index sharing with kortex's own
# workspace walker, an in-process search API in tgrep-core instead of
# CLI-only, etc. -- as real commits on our fork, buildable the same way.
#
# ASCII-only on purpose: Windows PowerShell 5.1 (the default on a plain
# Windows box, no pwsh required) misparses a UTF-8-without-BOM script that
# contains non-ASCII characters like em-dashes -- this script has to run
# correctly there, not just under PowerShell 7.
#
# Usage (from a normal PowerShell, repo root or anywhere):
#   powershell -File scripts/build-tgrep.ps1
#   powershell -File scripts/build-tgrep.ps1 -Clean

[CmdletBinding()]
param(
    [switch]$Clean
)

$ErrorActionPreference = "Stop"
$repoRoot = Split-Path -Parent $PSScriptRoot
$src      = Join-Path $repoRoot "tgrep"
$stageDir = Join-Path $repoRoot "src-tauri\bundles\tgrep"

function Die($msg) { Write-Host "`n  ERROR: $msg`n" -ForegroundColor Red; exit 1 }
function Info($msg) { Write-Host "  $msg" -ForegroundColor Cyan }

if (-not (Test-Path (Join-Path $src "Cargo.toml"))) {
    Die "tgrep submodule not found at $src. Run:  git -C `"$repoRoot`" submodule update --init tgrep"
}

$cargoCmd = Get-Command cargo -ErrorAction SilentlyContinue
if (-not $cargoCmd) {
    Die "cargo not found on PATH. Install Rust: https://rustup.rs"
}

Push-Location $src
try {
    if ($Clean) {
        Info "cargo clean ..."
        cargo clean
        if ($LASTEXITCODE -ne 0) { Die "cargo clean failed" }
    }

    Info "Building tgrep-cli --release from source ($src) ..."
    cargo build --release -p tgrep-cli
    if ($LASTEXITCODE -ne 0) { Die "cargo build failed" }
}
finally {
    Pop-Location
}

$builtExe = Join-Path $src "target\release\tgrep.exe"
if (-not (Test-Path $builtExe)) {
    Die "Build succeeded but $builtExe is missing -- unexpected output layout."
}

New-Item -ItemType Directory -Force -Path $stageDir | Out-Null
$destExe = Join-Path $stageDir "tgrep.exe"
Copy-Item -Path $builtExe -Destination $destExe -Force

Info "OK -- $destExe"
Info "Version: $(& $destExe --version)"
