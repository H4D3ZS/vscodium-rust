#!/usr/bin/env node
/**
 * Download official ripgrep release into src-tauri/bundles/ripgrep/
 * (rg.exe on Windows, rg on macOS/Linux). Pinned version — no GitHub API.
 */
import { execSync } from 'node:child_process';
import { existsSync, mkdirSync, cpSync, rmSync, readdirSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { platform, arch } from 'node:os';

const VERSION = '14.1.1';
const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const outDir = join(root, 'src-tauri', 'bundles', 'ripgrep');
const binName = platform() === 'win32' ? 'rg.exe' : 'rg';
const outBin = join(outDir, binName);

function triple() {
    const p = platform();
    const a = arch();
    if (p === 'win32') return a === 'arm64' ? 'aarch64-pc-windows-msvc' : 'x86_64-pc-windows-msvc';
    if (p === 'darwin') return a === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    if (p === 'linux') return a === 'arm64' ? 'aarch64-unknown-linux-gnu' : 'x86_64-unknown-linux-gnu';
    throw new Error(`Unsupported platform: ${p}/${a}`);
}

function findRgInDir(dir, depth = 0) {
    if (depth > 4) return null;
    const candidate = join(dir, binName);
    if (existsSync(candidate)) return candidate;
    for (const name of readdirSync(dir)) {
        const p = join(dir, name);
        if (statSync(p).isDirectory()) {
            const found = findRgInDir(p, depth + 1);
            if (found) return found;
        }
    }
    return null;
}

if (existsSync(outBin) && !process.env.FORCE_BUNDLE_FETCH) {
    console.log(`[fetch-ripgrep] ${outBin} present — skip (FORCE_BUNDLE_FETCH=1 to re-fetch).`);
    process.exit(0);
}

const t = triple();
const zipName = `ripgrep-${VERSION}-${t}.zip`;
const url = `https://github.com/BurntSushi/ripgrep/releases/download/${VERSION}/${zipName}`;
const tmpZip = join(outDir, zipName);
const tmpExtract = join(outDir, '_extract');

mkdirSync(outDir, { recursive: true });
if (existsSync(tmpExtract)) rmSync(tmpExtract, { recursive: true, force: true });

console.log(`[fetch-ripgrep] Downloading ${zipName} …`);
execSync(`curl -fsSL "${url}" -o "${tmpZip}"`, { stdio: 'inherit' });

console.log('[fetch-ripgrep] Extracting …');
mkdirSync(tmpExtract, { recursive: true });
if (platform() === 'win32') {
    execSync(
        `powershell -NoProfile -Command "Expand-Archive -LiteralPath '${tmpZip.replace(/'/g, "''")}' -DestinationPath '${tmpExtract.replace(/'/g, "''")}' -Force"`,
        { stdio: 'inherit' },
    );
} else {
    execSync(`unzip -o -q "${tmpZip}" -d "${tmpExtract}"`, { stdio: 'inherit' });
}

const found = findRgInDir(tmpExtract);
if (!found) {
    console.error('[fetch-ripgrep] rg binary not found in archive');
    process.exit(1);
}

cpSync(found, outBin);
if (platform() !== 'win32') {
    execSync(`chmod +x "${outBin}"`, { stdio: 'inherit' });
}

rmSync(tmpExtract, { recursive: true, force: true });
rmSync(tmpZip, { force: true });

console.log(`[fetch-ripgrep] OK — ${outBin}`);
