# Fetch IDE-bundled language servers into src-tauri/binaries/lsp/
# Run before `npx tauri build` so subscribers get zero-config IntelliSense.
# Mirror these zips on your DO CDN: $env:LSP_BUNDLE_MIRROR

$ErrorActionPreference = "Stop"
$Root = Join-Path $PSScriptRoot "..\src-tauri\binaries\lsp"
New-Item -ItemType Directory -Force -Path $Root | Out-Null

function Expand-ZipExe($ZipUrl, $DestDir, $ExeName) {
    Write-Host "Downloading $ExeName from $ZipUrl ..."
    $zip = Join-Path $env:TEMP "lsp-$ExeName.zip"
    Invoke-WebRequest -Uri $ZipUrl -OutFile $zip -UseBasicParsing
    Expand-Archive -Path $zip -DestinationPath $DestDir -Force
    Remove-Item $zip -Force
    $found = Get-ChildItem -Path $DestDir -Recurse -Filter $ExeName | Select-Object -First 1
    if (-not $found) { throw "$ExeName not found in archive" }
    $dest = Join-Path $DestDir $ExeName
    if (-not (Test-Path -LiteralPath $dest)) {
        Copy-Item $found.FullName $dest -Force
    }
    Write-Host "  OK: $(Join-Path $DestDir $ExeName)"
}

# rust-analyzer (Windows x64)
$raRelease = Invoke-RestMethod -Uri "https://api.github.com/repos/rust-lang/rust-analyzer/releases/latest" -Headers @{ "User-Agent" = "vscodium-rust" }
$raAsset = $raRelease.assets | Where-Object { $_.name -match "x86_64-pc-windows-msvc\.zip$" } | Select-Object -First 1
Expand-ZipExe $raAsset.browser_download_url (Join-Path $Root "rust-analyzer") "rust-analyzer.exe"

# gopls (no official zip - build via go install when toolchain is present)
$goplsDir = Join-Path $Root "gopls"
New-Item -ItemType Directory -Force -Path $goplsDir | Out-Null
$goplsDest = Join-Path $goplsDir "gopls.exe"
if (-not (Test-Path -LiteralPath $goplsDest)) {
    $go = Get-Command go -ErrorAction SilentlyContinue
    if ($go) {
        Write-Host "Building gopls via go install ..."
        $env:GOBIN = $goplsDir
        & go install 'golang.org/x/tools/gopls@latest'
        if (Test-Path -LiteralPath $goplsDest) {
            Write-Host "  OK: $goplsDest"
        } else {
            $built = Get-ChildItem -Path $goplsDir -Filter "gopls.exe" -Recurse | Select-Object -First 1
            if ($built) { Write-Host "  OK: $($built.FullName)" }
            else { Write-Warning "gopls build failed - install Go and re-run, or add gopls.exe manually to $goplsDir" }
        }
    } else {
        Write-Warning "Go not found - skip gopls (install Go or copy gopls.exe to $goplsDir)"
    }
} else {
    Write-Host "  OK: $goplsDest (cached)"
}

# Node.js portable + typescript-language-server (no global npm for users)
$nodeVer = "v20.18.0"
$nodeZip = "node-$nodeVer-win-x64.zip"
$nodeUrl = "https://nodejs.org/dist/$nodeVer/$nodeZip"
$nodeDir = Join-Path $Root "typescript-language-server"
New-Item -ItemType Directory -Force -Path $nodeDir | Out-Null
$nodeZipPath = Join-Path $env:TEMP $nodeZip
Write-Host "Downloading Node $nodeVer ..."
Invoke-WebRequest -Uri $nodeUrl -OutFile $nodeZipPath -UseBasicParsing
Expand-Archive -Path $nodeZipPath -DestinationPath $nodeDir -Force
Remove-Item $nodeZipPath -Force
$nodeExe = Get-ChildItem -Path $nodeDir -Recurse -Filter "node.exe" | Select-Object -First 1
$nodeHome = $nodeExe.Directory.FullName
$npmCmd = Join-Path $nodeHome "npm.cmd"
if (-not (Test-Path -LiteralPath $npmCmd)) { $npmCmd = Join-Path $nodeHome "npm.exe" }

Write-Host "Installing typescript-language-server locally ..."
Push-Location $nodeDir
& $npmCmd install typescript-language-server@4.3.3 typescript@5.7.2 --no-save --prefix .
Pop-Location

$tsCli = Get-ChildItem -Path $nodeDir -Recurse -Filter "cli.mjs" | Where-Object { $_.FullName -match "typescript-language-server" } | Select-Object -First 1
$wrapper = Join-Path $nodeDir "typescript-language-server.cmd"
@"
@echo off
"$($nodeExe.FullName)" "$($tsCli.FullName)" %*
"@ | Set-Content -Path $wrapper -Encoding ASCII
Write-Host "  OK: $wrapper"

# pyright (Python LSP via Node — no Python install required)
$pyDir = Join-Path $Root "pyright"
New-Item -ItemType Directory -Force -Path $pyDir | Out-Null
Push-Location $pyDir
& $npmCmd install pyright@1.1.390 --no-save --prefix .
Pop-Location
$pyrightJs = Get-ChildItem -Path $pyDir -Recurse -Filter "langserver.index.js" | Where-Object { $_.FullName -match "pyright" } | Select-Object -First 1
$pyWrapper = Join-Path $pyDir "pyright-langserver.cmd"
@"
@echo off
"$($nodeExe.FullName)" "$($pyrightJs.FullName)" %*
"@ | Set-Content -Path $pyWrapper -Encoding ASCII
Write-Host "  OK: $pyWrapper"

Write-Host ""
Write-Host "Done. Bundled LSP binaries in: $Root"
Write-Host "Ship with installer via tauri.conf.json bundle.resources binaries/*"
