#!/usr/bin/env node
// Cross-platform LSP bundle fetcher → src-tauri/binaries/lsp/
// Works on macOS (x64/arm64), Linux (x64/arm64) and Windows (x64).
// Produces the layout `src-tauri/src/lsp_bundle.rs` resolves at runtime:
//   - Windows: <server>/<server>.exe or <server>.cmd (%~dp0-relative wrappers)
//   - Unix:    <server>/<server> native binary or executable sh wrapper
//
// Usage: node scripts/fetch-lsp-binaries.ts   (Node >=23.6, native TS)
// Mirror override: LSP_BUNDLE_MIRROR for rust-analyzer assets.

import { execFileSync, execSync } from 'node:child_process';
import { createWriteStream, existsSync, mkdirSync, readdirSync, statSync, writeFileSync, chmodSync, rmSync, cpSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { pipeline } from 'node:stream/promises';
import { createGunzip } from 'node:zlib';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT = resolve(__dirname, '..', 'src-tauri', 'binaries', 'lsp');
mkdirSync(ROOT, { recursive: true });

const IS_WIN = process.platform === 'win32';
const IS_MAC = process.platform === 'darwin';
const ARCH = process.arch === 'arm64' ? 'arm64' : 'x64';

const NODE_VER = 'v20.18.0';
const NODE_DIST = IS_WIN
    ? `node-${NODE_VER}-win-x64`
    : `node-${NODE_VER}-${IS_MAC ? 'darwin' : 'linux'}-${ARCH}`;

const ok = (m: string): void => console.log(`  OK: ${m}`);
const warn = (m: string): void => console.warn(`  WARN: ${m}`);
const section = (m: string): void => console.log(`\n── ${m}`);

interface GithubAsset {
    name: string;
    browser_download_url: string;
}

interface GithubRelease {
    tag_name?: string;
    assets?: GithubAsset[];
}

async function fetchBuffer(url: string): Promise<Buffer> {
    const res = await fetch(url, { headers: { 'user-agent': 'vscodium-rust-ide' }, redirect: 'follow' });
    if (!res.ok) throw new Error(`HTTP ${res.status} for ${url}`);
    return Buffer.from(await res.arrayBuffer());
}

async function downloadTo(url: string, dest: string): Promise<void> {
    const res = await fetch(url, { headers: { 'user-agent': 'vscodium-rust-ide' }, redirect: 'follow' });
    if (!res.ok || !res.body) throw new Error(`HTTP ${res.status} for ${url}`);
    await pipeline(res.body, createWriteStream(dest));
}

/** Extract zip / tar.gz / tar.xz using the system tar (bsdtar everywhere except GNU/Linux zips → unzip). */
function extract(archive: string, destDir: string): void {
    mkdirSync(destDir, { recursive: true });
    if (archive.endsWith('.zip')) {
        try {
            execFileSync('tar', ['-xf', archive, '-C', destDir], { stdio: 'pipe' });
            return;
        } catch {
            execFileSync('unzip', ['-oq', archive, '-d', destDir], { stdio: 'pipe' });
            return;
        }
    }
    execFileSync('tar', ['-xf', archive, '-C', destDir], { stdio: 'pipe' });
}

function findFile(dir: string, predicate: (name: string, path: string) => boolean): string | null {
    if (!existsSync(dir)) return null;
    for (const entry of readdirSync(dir)) {
        const p = join(dir, entry);
        const st = statSync(p);
        if (st.isDirectory()) {
            const found = findFile(p, predicate);
            if (found) return found;
        } else if (predicate(entry, p)) {
            return p;
        }
    }
    return null;
}

function makeExecutable(p: string): void {
    if (!IS_WIN) chmodSync(p, 0o755);
}

/** Wrapper that survives relocation: %~dp0 on Windows, $(dirname $0) on Unix. */
function writeWrapper(wrapperPath: string, nodeRelFromWrapper: string, scriptRelFromWrapper: string): void {
    if (IS_WIN) {
        const body = `@echo off\r\n"%~dp0${nodeRelFromWrapper.replaceAll('/', '\\')}" "%~dp0${scriptRelFromWrapper.replaceAll('/', '\\')}" %*\r\n`;
        writeFileSync(wrapperPath, body, 'ascii');
    } else {
        const body = `#!/bin/sh\nDIR="$(cd "$(dirname "$0")" && pwd)"\nexec "$DIR/${nodeRelFromWrapper}" "$DIR/${scriptRelFromWrapper}" "$@"\n`;
        writeFileSync(wrapperPath, body);
        makeExecutable(wrapperPath);
    }
}

const nodeLeaf = IS_WIN ? 'node.exe' : 'node';
const exeLeaf = (name: string): string => (IS_WIN ? `${name}.exe` : name);
const wrapLeaf = (name: string): string => (IS_WIN ? `${name}.cmd` : name);

// ── rust-analyzer ────────────────────────────────────────────────────────────
async function fetchRustAnalyzer(): Promise<void> {
    section('rust-analyzer');
    const dir = join(ROOT, 'rust-analyzer');
    const dest = join(dir, exeLeaf('rust-analyzer'));
    if (existsSync(dest)) return ok(`${dest} (cached)`);
    mkdirSync(dir, { recursive: true });

    const triple = IS_WIN
        ? 'x86_64-pc-windows-msvc'
        : IS_MAC
            ? (ARCH === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin')
            : (ARCH === 'arm64' ? 'aarch64-unknown-linux-gnu' : 'x86_64-unknown-linux-gnu');
    const ext = IS_WIN ? 'zip' : 'gz';

    const release = JSON.parse(
        (await fetchBuffer('https://api.github.com/repos/rust-lang/rust-analyzer/releases/latest')).toString(),
    ) as GithubRelease;
    const mirror = process.env.LSP_BUNDLE_MIRROR || 'https://github.com/rust-lang/rust-analyzer/releases/download';
    const asset = (release.assets || []).find((a) => a.name.includes(triple) && a.name.endsWith(ext));
    const url = asset?.browser_download_url
        || `${mirror}/${release.tag_name || 'nightly'}/rust-analyzer-${triple}.${ext}`;

    if (IS_WIN) {
        const tmp = join(tmpdir(), 'ra.zip');
        await downloadTo(url, tmp);
        extract(tmp, dir);
        rmSync(tmp, { force: true });
        const found = findFile(dir, (n) => n === 'rust-analyzer.exe');
        if (found && found !== dest) cpSync(found, dest);
    } else {
        const res = await fetch(url, { headers: { 'user-agent': 'vscodium-rust-ide' }, redirect: 'follow' });
        if (!res.ok || !res.body) throw new Error(`HTTP ${res.status} for ${url}`);
        await pipeline(res.body, createGunzip(), createWriteStream(dest));
        makeExecutable(dest);
    }
    ok(dest);
}

// ── Portable Node runtime ────────────────────────────────────────────────────
async function fetchNode(): Promise<string> {
    section(`Node ${NODE_VER} (${NODE_DIST})`);
    const dir = join(ROOT, 'typescript-language-server');
    mkdirSync(dir, { recursive: true });
    let nodeBin = findFile(dir, (n) => n === nodeLeaf);
    if (nodeBin) {
        ok(`${nodeBin} (cached)`);
        return nodeBin;
    }
    const archive = IS_WIN ? `${NODE_DIST}.zip` : `${NODE_DIST}.tar.gz`;
    const url = `https://nodejs.org/dist/${NODE_VER}/${archive}`;
    const tmp = join(tmpdir(), archive);
    await downloadTo(url, tmp);
    extract(tmp, dir);
    rmSync(tmp, { force: true });
    nodeBin = findFile(dir, (n) => n === nodeLeaf);
    if (!nodeBin) throw new Error('node binary missing after extract');
    makeExecutable(nodeBin);
    ok(nodeBin);
    return nodeBin;
}

function npmInstall(nodeBin: string, prefixDir: string, pkgs: string[]): void {
    const nodeHome = dirname(nodeBin);
    // npm ships inside the node dist: bin/npm (unix) / npm.cmd (win)
    const npmCli = IS_WIN
        ? join(nodeHome, 'node_modules', 'npm', 'bin', 'npm-cli.js')
        : join(nodeHome, '..', 'lib', 'node_modules', 'npm', 'bin', 'npm-cli.js');
    mkdirSync(prefixDir, { recursive: true });
    execFileSync(nodeBin, [npmCli, 'install', ...pkgs, '--no-save', '--prefix', prefixDir], {
        stdio: 'inherit',
        cwd: prefixDir,
    });
}

// ── Node-based servers: TS, Pyright, HTML/CSS/JSON, Bash ────────────────────
async function fetchNodeServers(nodeBin: string): Promise<void> {
    const tsDir = join(ROOT, 'typescript-language-server');
    const nodeRelTs = nodeBin.substring(tsDir.length + 1).replaceAll('\\', '/');

    section('typescript-language-server');
    const tsWrap = join(tsDir, wrapLeaf('typescript-language-server'));
    if (!existsSync(join(tsDir, 'node_modules', 'typescript-language-server'))) {
        npmInstall(nodeBin, tsDir, ['typescript-language-server@4.3.3', 'typescript@5.7.2']);
    }
    writeWrapper(tsWrap, nodeRelTs, 'node_modules/typescript-language-server/lib/cli.mjs');
    ok(tsWrap);

    section('pyright');
    const pyDir = join(ROOT, 'pyright');
    if (!existsSync(join(pyDir, 'node_modules', 'pyright'))) {
        npmInstall(nodeBin, pyDir, ['pyright@1.1.390']);
    }
    writeWrapper(
        join(pyDir, wrapLeaf('pyright-langserver')),
        `../typescript-language-server/${nodeRelTs}`,
        'node_modules/pyright/langserver.index.js',
    );
    ok(join(pyDir, wrapLeaf('pyright-langserver')));

    section('vscode-langservers (HTML/CSS/JSON) + bash-language-server');
    const markupDir = join(ROOT, 'vscode-langservers');
    if (!existsSync(join(markupDir, 'node_modules', 'bash-language-server'))) {
        npmInstall(nodeBin, markupDir, ['vscode-langservers-extracted@4.0.0', 'bash-language-server@5.4.2']);
    }
    writeWrapper(
        join(markupDir, wrapLeaf('vscode-html-language-server')),
        `../typescript-language-server/${nodeRelTs}`,
        'node_modules/vscode-langservers-extracted/bin/vscode-html-language-server',
    );
    ok(join(markupDir, wrapLeaf('vscode-html-language-server')));

    const bashDir = join(ROOT, 'bash-language-server');
    mkdirSync(bashDir, { recursive: true });
    writeWrapper(
        join(bashDir, wrapLeaf('bash-language-server')),
        `../typescript-language-server/${nodeRelTs}`,
        '../vscode-langservers/node_modules/bash-language-server/out/cli.js',
    );
    ok(join(bashDir, wrapLeaf('bash-language-server')));
}

// ── gopls (go install — no official prebuilt binaries) ──────────────────────
function fetchGopls(): void {
    section('gopls');
    const dir = join(ROOT, 'gopls');
    const dest = join(dir, exeLeaf('gopls'));
    if (existsSync(dest)) return ok(`${dest} (cached)`);
    mkdirSync(dir, { recursive: true });
    try {
        execSync('go version', { stdio: 'pipe' });
    } catch {
        return warn('Go toolchain not found — skipping gopls (runtime falls back to PATH / on-demand install)');
    }
    execSync('go install golang.org/x/tools/gopls@latest', {
        stdio: 'inherit',
        env: { ...process.env, GOBIN: dir },
    });
    existsSync(dest) ? ok(dest) : warn('go install ran but gopls missing');
}

// ── clangd ───────────────────────────────────────────────────────────────────
async function fetchClangd(): Promise<void> {
    section('clangd');
    const ver = '19.1.2';
    const dir = join(ROOT, 'clangd');
    const dest = join(dir, exeLeaf('clangd'));
    if (existsSync(dest)) return ok(`${dest} (cached)`);
    const plat = IS_WIN ? 'windows' : IS_MAC ? 'mac' : 'linux';
    const url = `https://github.com/clangd/clangd/releases/download/${ver}/clangd-${plat}-${ver}.zip`;
    const tmp = join(tmpdir(), 'clangd.zip');
    await downloadTo(url, tmp);
    extract(tmp, dir);
    rmSync(tmp, { force: true });
    const found = findFile(dir, (n) => n === exeLeaf('clangd'));
    if (!found) return warn('clangd binary not found in archive');
    if (found !== dest) cpSync(found, dest);
    makeExecutable(dest);
    ok(dest);
}

// ── lua-language-server ──────────────────────────────────────────────────────
async function fetchLua(): Promise<void> {
    section('lua-language-server');
    const ver = '3.13.6';
    const dir = join(ROOT, 'lua-language-server');
    const dest = join(dir, exeLeaf('lua-language-server'));
    if (existsSync(dest)) return ok(`${dest} (cached)`);
    const asset = IS_WIN
        ? `lua-language-server-${ver}-win32-x64.zip`
        : IS_MAC
            ? `lua-language-server-${ver}-darwin-${ARCH}.tar.gz`
            : `lua-language-server-${ver}-linux-${ARCH}.tar.gz`;
    const url = `https://github.com/LuaLS/lua-language-server/releases/download/${ver}/${asset}`;
    const tmp = join(tmpdir(), asset);
    await downloadTo(url, tmp);
    extract(tmp, dir);
    rmSync(tmp, { force: true });
    const found = findFile(dir, (n) => n === exeLeaf('lua-language-server'));
    if (!found) return warn('lua-language-server binary not found in archive');
    if (found !== dest) cpSync(found, dest);
    makeExecutable(dest);
    ok(dest);
}

// ── zls (Zig) ────────────────────────────────────────────────────────────────
async function fetchZls(): Promise<void> {
    section('zls');
    const ver = '0.14.0';
    const dir = join(ROOT, 'zls');
    const dest = join(dir, exeLeaf('zls'));
    if (existsSync(dest)) return ok(`${dest} (cached)`);
    const cpu = ARCH === 'arm64' ? 'aarch64' : 'x86_64';
    const asset = IS_WIN
        ? `zls-${cpu}-windows.zip`
        : IS_MAC
            ? `zls-${cpu}-macos.tar.xz`
            : `zls-${cpu}-linux.tar.xz`;
    const url = `https://github.com/zigtools/zls/releases/download/${ver}/${asset}`;
    const tmp = join(tmpdir(), asset);
    await downloadTo(url, tmp);
    extract(tmp, dir);
    rmSync(tmp, { force: true });
    const found = findFile(dir, (n) => n === exeLeaf('zls'));
    if (!found) return warn('zls binary not found in archive');
    if (found !== dest) cpSync(found, dest);
    makeExecutable(dest);
    ok(dest);
}

// ── main ─────────────────────────────────────────────────────────────────────
const steps: Array<[string, () => Promise<void> | void]> = [
    ['rust-analyzer', fetchRustAnalyzer],
    ['node servers', async () => fetchNodeServers(await fetchNode())],
    ['gopls', fetchGopls],
    ['clangd', fetchClangd],
    ['lua-language-server', fetchLua],
    ['zls', fetchZls],
];

console.log(`Fetching LSP bundles for ${process.platform}-${ARCH} → ${ROOT}`);
let failures = 0;
for (const [name, fn] of steps) {
    try {
        await fn();
    } catch (e) {
        failures += 1;
        warn(`${name} failed: ${(e as Error).message}`);
    }
}

console.log('\nStacks bundled: Rust, TS/JS, Python, Bash, HTML/CSS/JSON, Go*, C/C++, Lua, Zig');
console.log('(* gopls needs a Go toolchain at fetch time)');
console.log('Java/Kotlin/Dart/Swift/etc. resolve from PATH at runtime (path_fallbacks).');
process.exit(failures > 2 ? 1 : 0);
