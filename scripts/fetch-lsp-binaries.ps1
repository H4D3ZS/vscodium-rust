# Fetch IDE-bundled language servers into src-tauri/binaries/lsp/
# Run before `npx tauri build` so subscribers get zero-config IntelliSense.
# Mirror these zips on your DO CDN: $env:LSP_BUNDLE_MIRROR

$ErrorActionPreference = "Stop"
$Root = Join-Path $PSScriptRoot "..\src-tauri\binaries\lsp"
New-Item -ItemType Directory -Force -Path $Root | Out-Null

function Write-CmdWrapper {
    param(
        [string]$Path,
        [string]$NodeExe,
        [string]$Target
    )
    $batch = "@echo off`r`n`"$NodeExe`" `"$Target`" %*"
    Set-Content -Path $Path -Encoding ASCII -Value $batch
}

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
$wrapper = Join-Path $nodeDir "typescript-language-server.cmd"
$nodeExe = Get-ChildItem -Path $nodeDir -Recurse -Filter "node.exe" -ErrorAction SilentlyContinue | Select-Object -First 1

if ((Test-Path -LiteralPath $wrapper) -and $nodeExe) {
    Write-Host "  OK: $wrapper (cached)"
} else {
    if (-not $nodeExe) {
        $nodeZipPath = Join-Path $env:TEMP $nodeZip
        Write-Host "Downloading Node $nodeVer ..."
        Invoke-WebRequest -Uri $nodeUrl -OutFile $nodeZipPath -UseBasicParsing
        $extractTmp = Join-Path $env:TEMP "hades-node-$nodeVer"
        if (Test-Path $extractTmp) { Remove-Item -Recurse -Force $extractTmp -ErrorAction SilentlyContinue }
        Expand-Archive -Path $nodeZipPath -DestinationPath $extractTmp -Force
        Remove-Item $nodeZipPath -Force -ErrorAction SilentlyContinue
        $extracted = Get-ChildItem -Path $extractTmp -Directory | Select-Object -First 1
        if ($extracted) {
            Copy-Item -Path (Join-Path $extracted.FullName '*') -Destination $nodeDir -Recurse -Force
        }
        Remove-Item -Recurse -Force $extractTmp -ErrorAction SilentlyContinue
        $nodeExe = Get-ChildItem -Path $nodeDir -Recurse -Filter "node.exe" | Select-Object -First 1
    }

    if (-not $nodeExe) { throw "node.exe not found after extract in $nodeDir" }

    $nodeHome = $nodeExe.Directory.FullName
    $npmCmd = Join-Path $nodeHome "npm.cmd"
    if (-not (Test-Path -LiteralPath $npmCmd)) { $npmCmd = Join-Path $nodeHome "npm.exe" }

    Write-Host "Installing typescript-language-server locally ..."
    Push-Location $nodeDir
    & $npmCmd install typescript-language-server@4.3.3 typescript@5.7.2 --no-save --prefix .
    Pop-Location

    $tsCli = Get-ChildItem -Path $nodeDir -Recurse -Filter "cli.mjs" | Where-Object { $_.FullName -match "typescript-language-server" } | Select-Object -First 1
    if (-not $tsCli) { throw "typescript-language-server cli.mjs not found" }
    Write-CmdWrapper -Path $wrapper -NodeExe $nodeExe.FullName -Target $tsCli.FullName
    Write-Host "  OK: $wrapper"
}

# pyright (Python LSP via Node — no Python install required)
$pyDir = Join-Path $Root "pyright"
New-Item -ItemType Directory -Force -Path $pyDir | Out-Null
$pyWrapper = Join-Path $pyDir "pyright-langserver.cmd"
if ((Test-Path -LiteralPath $pyWrapper) -and $nodeExe) {
    Write-Host "  OK: $pyWrapper (cached)"
} else {
    if (-not $nodeExe) { throw "node.exe required for pyright - fix typescript-language-server step first" }
    $npmCmd = Join-Path $nodeExe.Directory.FullName "npm.cmd"
    if (-not (Test-Path -LiteralPath $npmCmd)) { $npmCmd = Join-Path $nodeExe.Directory.FullName "npm.exe" }
    Push-Location $pyDir
    & $npmCmd install pyright@1.1.390 --no-save --prefix .
    Pop-Location
    $pyrightJs = Get-ChildItem -Path $pyDir -Recurse -Filter "langserver.index.js" | Where-Object { $_.FullName -match "pyright" } | Select-Object -First 1
    if (-not $pyrightJs) { throw "pyright langserver.index.js not found" }
    Write-CmdWrapper -Path $pyWrapper -NodeExe $nodeExe.FullName -Target $pyrightJs.FullName
    Write-Host "  OK: $pyWrapper"
}

# ── Kotlin Language Server (Android / Kotlin Gradle) ────────────────────────
try {
    $klsDir = Join-Path $Root "kotlin-language-server"
    $klsExe = Join-Path $klsDir "kotlin-language-server.exe"
    if (-not (Test-Path -LiteralPath $klsExe)) {
        $klsVer = "1.3.7"
        $klsZip = "kotlin-language-server.zip"
        $klsUrl = "https://github.com/fwcd/kotlin-language-server/releases/download/v$klsVer/$klsZip"
        Write-Host "Downloading kotlin-language-server $klsVer ..."
        New-Item -ItemType Directory -Force -Path $klsDir | Out-Null
        $klsZipPath = Join-Path $env:TEMP $klsZip
        Invoke-WebRequest -Uri $klsUrl -OutFile $klsZipPath -UseBasicParsing
        Expand-Archive -Path $klsZipPath -DestinationPath $klsDir -Force
        Remove-Item $klsZipPath -Force
        $found = Get-ChildItem -Path $klsDir -Recurse -Filter "kotlin-language-server.exe" | Select-Object -First 1
        if ($found -and -not (Test-Path -LiteralPath $klsExe)) {
            Copy-Item $found.FullName $klsExe -Force
        }
        Write-Host "  OK: $klsExe"
    } else {
        Write-Host "  OK: $klsExe (cached)"
    }
} catch {
    Write-Warning "kotlin-language-server fetch failed: $_"
}

# ── JDTLS + JRE 17 (Java / Android) ─────────────────────────────────────────
try {
    $jdtlsDir = Join-Path $Root "jdtls"
    $jreDir = Join-Path $Root "jre"
    $launcher = Get-ChildItem -Path (Join-Path $jdtlsDir "plugins") -Filter "org.eclipse.equinox.launcher_*.jar" -ErrorAction SilentlyContinue | Select-Object -First 1
    if (-not $launcher) {
        $jdtlsUrl = "https://download.eclipse.org/jdtls/milestones/1.42.0/jdtls.tar.gz"
        Write-Host "Downloading Eclipse JDT Language Server ..."
        New-Item -ItemType Directory -Force -Path $jdtlsDir | Out-Null
        $tar = Join-Path $env:TEMP "jdtls.tar.gz"
        Invoke-WebRequest -Uri $jdtlsUrl -OutFile $tar -UseBasicParsing
        tar -xzf $tar -C $jdtlsDir
        Remove-Item $tar -Force
        Write-Host "  OK: jdtls extracted to $jdtlsDir"
    } else {
        Write-Host "  OK: jdtls (cached)"
    }

    $javaExe = Join-Path $jreDir "bin\java.exe"
    if (-not (Test-Path -LiteralPath $javaExe)) {
        $jreUrl = "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.14%2B7/OpenJDK17U-jre_x64_windows_hotspot_17.0.14_7.zip"
        Write-Host "Downloading Temurin JRE 17 ..."
        New-Item -ItemType Directory -Force -Path $jreDir | Out-Null
        $jreZip = Join-Path $env:TEMP "temurin-jre17.zip"
        Invoke-WebRequest -Uri $jreUrl -OutFile $jreZip -UseBasicParsing
        Expand-Archive -Path $jreZip -DestinationPath $jreDir -Force
        Remove-Item $jreZip -Force
        $foundJava = Get-ChildItem -Path $jreDir -Recurse -Filter "java.exe" | Select-Object -First 1
        if ($foundJava) { Write-Host "  OK: $($foundJava.FullName)" }
    } else {
        Write-Host "  OK: $javaExe (cached)"
    }
} catch {
    Write-Warning "jdtls/jre fetch failed: $_"
}

# ── Dart SDK (Flutter) ────────────────────────────────────────────────────────
try {
    $dartRoot = Join-Path $Root "dart-sdk"
    $dartExe = Join-Path $dartRoot "bin\dart.exe"
    if (-not (Test-Path -LiteralPath $dartExe)) {
        $dartVer = "3.7.3"
        $dartZip = "dartsdk-windows-x64.zip"
        $dartUrl = "https://storage.googleapis.com/dart-archive/channels/stable/release/$dartVer/sdk/$dartZip"
        Write-Host "Downloading Dart SDK $dartVer (Flutter) ..."
        New-Item -ItemType Directory -Force -Path $dartRoot | Out-Null
        $dartZipPath = Join-Path $env:TEMP $dartZip
        Invoke-WebRequest -Uri $dartUrl -OutFile $dartZipPath -UseBasicParsing
        Expand-Archive -Path $dartZipPath -DestinationPath $dartRoot -Force
        Remove-Item $dartZipPath -Force
        $foundDart = Get-ChildItem -Path $dartRoot -Recurse -Filter "dart.exe" | Select-Object -First 1
        if ($foundDart) { Write-Host "  OK: $($foundDart.FullName)" }
    } else {
        Write-Host "  OK: $dartExe (cached)"
    }
} catch {
    Write-Warning "Dart SDK fetch failed: $_"
}

# ── clangd (C/C++) ───────────────────────────────────────────────────────────
try {
    $clangDir = Join-Path $Root "clangd"
    $clangExe = Join-Path $clangDir "clangd.exe"
    if (-not (Test-Path -LiteralPath $clangExe)) {
        $clangVer = "19.1.2"
        $clangUrl = "https://github.com/clangd/clangd/releases/download/$clangVer/clangd-windows-$clangVer.zip"
        Write-Host "Downloading clangd $clangVer ..."
        New-Item -ItemType Directory -Force -Path $clangDir | Out-Null
        Expand-ZipExe $clangUrl $clangDir "clangd.exe"
    } else {
        Write-Host "  OK: $clangExe (cached)"
    }
} catch {
    Write-Warning "clangd fetch failed: $_"
}

# ── lua-language-server ───────────────────────────────────────────────────────
try {
    $luaDir = Join-Path $Root "lua-language-server"
    $luaExe = Join-Path $luaDir "lua-language-server.exe"
    if (-not (Test-Path -LiteralPath $luaExe)) {
        $luaVer = "3.13.6"
        $luaUrl = "https://github.com/LuaLS/lua-language-server/releases/download/$luaVer/lua-language-server-$luaVer-win32-x64.zip"
        Write-Host "Downloading lua-language-server $luaVer ..."
        New-Item -ItemType Directory -Force -Path $luaDir | Out-Null
        $luaZip = Join-Path $env:TEMP "lua-language-server.zip"
        Invoke-WebRequest -Uri $luaUrl -OutFile $luaZip -UseBasicParsing
        Expand-Archive -Path $luaZip -DestinationPath $luaDir -Force
        Remove-Item $luaZip -Force
        $found = Get-ChildItem -Path $luaDir -Recurse -Filter "lua-language-server.exe" | Select-Object -First 1
        if ($found -and -not (Test-Path -LiteralPath $luaExe)) {
            Copy-Item $found.FullName $luaExe -Force
        }
        Write-Host "  OK: $luaExe"
    } else {
        Write-Host "  OK: $luaExe (cached)"
    }
} catch {
    Write-Warning "lua-language-server fetch failed: $_"
}

# ── HTML/CSS/JSON (vscode-langservers-extracted) + bash-language-server ───────
try {
    $markupDir = Join-Path $Root "vscode-langservers"
    New-Item -ItemType Directory -Force -Path $markupDir | Out-Null
    $nodeExe = Get-ChildItem -Path (Join-Path $Root "typescript-language-server") -Recurse -Filter "node.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($nodeExe) {
        $npmCmd = Join-Path $nodeExe.Directory.FullName "npm.cmd"
        Push-Location $markupDir
        & $npmCmd install vscode-langservers-extracted@4.0.0 bash-language-server@5.4.2 --no-save --prefix . 2>$null
        Pop-Location
        $htmlCli = Get-ChildItem -Path $markupDir -Recurse -Filter "cli.js" | Where-Object { $_.FullName -match "vscode-html-language-server" } | Select-Object -First 1
        if ($htmlCli) {
            $htmlWrap = Join-Path $markupDir "vscode-html-language-server.cmd"
            Write-CmdWrapper -Path $htmlWrap -NodeExe $nodeExe.FullName -Target $htmlCli.FullName
            Write-Host "  OK: $htmlWrap"
        }
        $bashCli = Get-ChildItem -Path $markupDir -Recurse -Filter "cli.js" | Where-Object { $_.FullName -match "bash-language-server" } | Select-Object -First 1
        if ($bashCli) {
            $bashWrap = Join-Path (Join-Path $Root "bash-language-server") "bash-language-server.cmd"
            New-Item -ItemType Directory -Force -Path (Split-Path $bashWrap) | Out-Null
            Write-CmdWrapper -Path $bashWrap -NodeExe $nodeExe.FullName -Target $bashCli.FullName
            Write-Host "  OK: $bashWrap"
        }
    } else {
        Write-Warning "Node from typescript-language-server bundle missing - skip markup/bash LSP npm install"
    }
} catch {
    Write-Warning "vscode-langservers fetch failed: $_"
}

# ── jdtls launcher wrapper ────────────────────────────────────────────────────
try {
    $jdtlsDir = Join-Path $Root "jdtls"
    $jreDir = Join-Path $Root "jre"
    $javaExe = Get-ChildItem -Path $jreDir -Recurse -Filter "java.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    $launcher = Get-ChildItem -Path (Join-Path $jdtlsDir "plugins") -Filter "org.eclipse.equinox.launcher_*.jar" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($javaExe -and $launcher) {
        $jdtlsWrap = Join-Path $jdtlsDir "jdtls.cmd"
        $configPath = Join-Path $jdtlsDir "config_win"
        $batch = "@echo off`r`nsetlocal`r`nset DATA=%~1`r`nif `"$DATA`"==`"`" set DATA=%TEMP%\jdtls-data`r`n`"$($javaExe.FullName)`" -Declipse.application=org.eclipse.jdt.ls.core.id1 -Dosgi.bundles.defaultStartLevel=4 -Declipse.product=org.eclipse.jdt.ls.core.product -Xmx1G --add-modules=ALL-SYSTEM --add-opens java.base/java.util=ALL-UNNAMED --add-opens java.base/java.lang=ALL-UNNAMED -jar `"$($launcher.FullName)`" -configuration `"$configPath`" -data `"%DATA%`" %*"
        Set-Content -Path $jdtlsWrap -Encoding ASCII -Value $batch
        Write-Host "  OK: $jdtlsWrap"
    }
} catch {
    Write-Warning "jdtls wrapper failed: $_"
}

# ── C# (csharp-ls via dotnet tool) ──────────────────────────────────────────
try {
    $csharpDir = Join-Path $Root "csharp-ls"
    New-Item -ItemType Directory -Force -Path $csharpDir | Out-Null
    $csharpExe = Join-Path $csharpDir "csharp-ls.exe"
    if (-not (Test-Path -LiteralPath $csharpExe)) {
        $dotnet = Get-Command dotnet -ErrorAction SilentlyContinue
        if ($dotnet) {
            Write-Host "Installing csharp-ls via dotnet tool ..."
            & dotnet tool install csharp-ls --tool-path $csharpDir 2>$null
        }
        if (-not (Test-Path -LiteralPath $csharpExe)) {
            $found = Get-ChildItem -Path $csharpDir -Recurse -Filter "csharp-ls.exe" | Select-Object -First 1
            if ($found) { Copy-Item $found.FullName $csharpExe -Force }
        }
        if (Test-Path -LiteralPath $csharpExe) { Write-Host "  OK: $csharpExe" }
        else { Write-Warning "csharp-ls not found - install .NET SDK and re-run" }
    } else {
        Write-Host "  OK: $csharpExe (cached)"
    }
} catch {
    Write-Warning "csharp-ls fetch failed: $_"
}

# ── Ruby (ruby-lsp via npm) ───────────────────────────────────────────────────
try {
    $rubyDir = Join-Path $Root "ruby-lsp"
    New-Item -ItemType Directory -Force -Path $rubyDir | Out-Null
    $rubyWrap = Join-Path $rubyDir "ruby-lsp.cmd"
    $nodeExe = Get-ChildItem -Path (Join-Path $Root "typescript-language-server") -Recurse -Filter "node.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($nodeExe -and -not (Test-Path -LiteralPath $rubyWrap)) {
        $npmCmd = Join-Path $nodeExe.Directory.FullName "npm.cmd"
        Push-Location $rubyDir
        & $npmCmd install ruby-lsp@0.23.8 --no-save --prefix . 2>$null
        Pop-Location
        $rubyCli = Get-ChildItem -Path $rubyDir -Recurse -Filter "cli.js" | Where-Object { $_.FullName -match "ruby-lsp" } | Select-Object -First 1
        if ($rubyCli) {
            Write-CmdWrapper -Path $rubyWrap -NodeExe $nodeExe.FullName -Target $rubyCli.FullName
            Write-Host "  OK: $rubyWrap"
        }
    } elseif (Test-Path -LiteralPath $rubyWrap) {
        Write-Host "  OK: $rubyWrap (cached)"
    }
} catch {
    Write-Warning "ruby-lsp fetch failed: $_"
}

# ── PHP (intelephense via npm) ────────────────────────────────────────────────
try {
    $phpDir = Join-Path $Root "intelephense"
    New-Item -ItemType Directory -Force -Path $phpDir | Out-Null
    $phpWrap = Join-Path $phpDir "intelephense.cmd"
    $nodeExe = Get-ChildItem -Path (Join-Path $Root "typescript-language-server") -Recurse -Filter "node.exe" -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($nodeExe -and -not (Test-Path -LiteralPath $phpWrap)) {
        $npmCmd = Join-Path $nodeExe.Directory.FullName "npm.cmd"
        Push-Location $phpDir
        & $npmCmd install intelephense@1.21.0 --no-save --prefix . 2>$null
        Pop-Location
        $intCli = Get-ChildItem -Path $phpDir -Recurse -Filter "intelephense.js" | Select-Object -First 1
        if ($intCli) {
            Write-CmdWrapper -Path $phpWrap -NodeExe $nodeExe.FullName -Target $intCli.FullName
            Write-Host "  OK: $phpWrap"
        }
    } elseif (Test-Path -LiteralPath $phpWrap) {
        Write-Host "  OK: $phpWrap (cached)"
    }
} catch {
    Write-Warning "intelephense fetch failed: $_"
}

# ── Zig (zls) ─────────────────────────────────────────────────────────────────
try {
    $zlsDir = Join-Path $Root "zls"
    $zlsExe = Join-Path $zlsDir "zls.exe"
    if (-not (Test-Path -LiteralPath $zlsExe)) {
        $zlsVer = "0.14.0"
        $zlsUrl = "https://github.com/zigtools/zls/releases/download/$zlsVer/zls-x86_64-windows.zip"
        Write-Host "Downloading zls $zlsVer ..."
        New-Item -ItemType Directory -Force -Path $zlsDir | Out-Null
        Expand-ZipExe $zlsUrl $zlsDir "zls.exe"
    } else {
        Write-Host "  OK: $zlsExe (cached)"
    }
} catch {
    Write-Warning "zls fetch failed: $_"
}

# ── Elixir (elixir-ls release) ────────────────────────────────────────────────
try {
    $elixirDir = Join-Path $Root "elixir-ls"
    $elixirWrap = Join-Path $elixirDir "elixir-ls.cmd"
    if (-not (Test-Path -LiteralPath $elixirWrap)) {
        $elsVer = "0.28.0"
        $elsZip = "elixir-ls-release.zip"
        $elsUrl = "https://github.com/elixir-lsp/elixir-ls/releases/download/v$elsVer/$elsZip"
        Write-Host "Downloading elixir-ls $elsVer ..."
        New-Item -ItemType Directory -Force -Path $elixirDir | Out-Null
        $elsZipPath = Join-Path $env:TEMP $elsZip
        Invoke-WebRequest -Uri $elsUrl -OutFile $elsZipPath -UseBasicParsing
        Expand-Archive -Path $elsZipPath -DestinationPath $elixirDir -Force
        Remove-Item $elsZipPath -Force
        $launchScript = Get-ChildItem -Path $elixirDir -Recurse -Filter "language_server.bat" | Select-Object -First 1
        if ($launchScript) {
            Copy-Item $launchScript.FullName $elixirWrap -Force
            Write-Host "  OK: $elixirWrap"
        } else {
            Write-Warning "elixir-ls language_server.bat not found in archive"
        }
    } else {
        Write-Host "  OK: $elixirWrap (cached)"
    }
} catch {
    Write-Warning "elixir-ls fetch failed: $_"
}

# ── R (languageserver wrapper — requires R on PATH) ───────────────────────────
try {
    $rDir = Join-Path $Root "languageserver"
    New-Item -ItemType Directory -Force -Path $rDir | Out-Null
    $rWrap = Join-Path $rDir "languageserver.cmd"
    if (-not (Test-Path -LiteralPath $rWrap)) {
        $batch = "@echo off`r`nwhere Rscript >nul 2>&1 || (echo Install R from https://cran.r-project.org/ && exit /b 1)`r`nRscript -e `"if (!requireNamespace('languageserver', quietly=TRUE)) stop('install.packages(\\\\\\\"languageserver\\\\\\\")'); languageserver::run()`""
        Set-Content -Path $rWrap -Encoding ASCII -Value $batch
        Write-Host "  OK: $rWrap (requires R + languageserver package)"
    } else {
        Write-Host "  OK: $rWrap (cached)"
    }
} catch {
    Write-Warning "R languageserver wrapper failed: $_"
}

Write-Host ""
Write-Host "Stacks covered:"
Write-Host "  Rust, TS/JS, Python, Go, C/C++, C#, Ruby, PHP, Lua, Bash"
Write-Host "  HTML/CSS/JSON, Kotlin, Java/Android, Flutter/Dart, Elixir, R, Swift"
Write-Host ""
Write-Host "Done. Bundled LSP binaries in: $Root"
Write-Host 'Ship with installer via tauri.conf.json bundle.resources binaries/*'
