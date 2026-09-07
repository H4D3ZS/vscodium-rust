#!/usr/bin/env node
/**
 * Download official tgrep (microsoft/tgrep, MIT) release into
 * src-tauri/bundles/tgrep/ (tgrep.exe on Windows, tgrep on macOS/Linux).
 * Pinned version — no GitHub API. Mirrors fetch-ripgrep.ts exactly; tgrep
 * ships side by side with rg (see domain/indexing/tgrep_search.rs), not in
 * place of it, so this bundle is additive.
 *
 * Runs directly under Node >=23.6 (native TypeScript type stripping).
 */
import { execSync } from 'node:child_process';
import { existsSync, mkdirSync, cpSync, rmSync, readdirSync, statSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { platform, arch } from 'node:os';

const VERSION = 'v1.0.4';
const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const outDir = join(root, 'src-tauri', 'bundles', 'tgrep');
const binName = platform() === 'win32' ? 'tgrep.exe' : 'tgrep';
const outBin = join(outDir, binName);

function triple(): string {
    const p = platform();
    const a = arch();
    if (p === 'win32') return a === 'arm64' ? 'aarch64-pc-windows-msvc' : 'x86_64-pc-windows-msvc';
    if (p === 'darwin') return a === 'arm64' ? 'aarch64-apple-darwin' : 'x86_64-apple-darwin';
    // tgrep releases ship musl for linux on both x64 and arm64.
    if (p === 'linux') return a === 'arm64' ? 'aarch64-unknown-linux-musl' : 'x86_64-unknown-linux-musl';
    throw new Error(`Unsupported platform: ${p}/${a}`);
}

function findBinInDir(dir: string, depth = 0): string | null {
    if (depth > 4) return null;
    const candidate = join(dir, binName);
    if (existsSync(candidate)) return candidate;
    for (const name of readdirSync(dir)) {
        const p = join(dir, name);
        if (statSync(p).isDirectory()) {
            const found = findBinInDir(p, depth + 1);
            if (found) return found;
        }
    }
    return null;
}

if (existsSync(outBin) && !process.env.FORCE_BUNDLE_FETCH) {
    console.log(`[fetch-tgrep] ${outBin} present — skip (FORCE_BUNDLE_FETCH=1 to re-fetch).`);
    process.exit(0);
}

const t = triple();
// tgrep ships .zip for Windows and .tar.gz for macOS/Linux; asset name is
// `tgrep-${VERSION}-${triple}.${ext}` (VERSION includes the leading "v").
const ext = platform() === 'win32' ? 'zip' : 'tar.gz';
const archiveName = `tgrep-${VERSION}-${t}.${ext}`;
const url = `https://github.com/microsoft/tgrep/releases/download/${VERSION}/${archiveName}`;
const tmpArchive = join(outDir, archiveName);
const tmpExtract = join(outDir, '_extract');

mkdirSync(outDir, { recursive: true });
if (existsSync(tmpExtract)) rmSync(tmpExtract, { recursive: true, force: true });

console.log(`[fetch-tgrep] Downloading ${archiveName} …`);
execSync(`curl -fsSL "${url}" -o "${tmpArchive}"`, { stdio: 'inherit' });

console.log('[fetch-tgrep] Extracting …');
mkdirSync(tmpExtract, { recursive: true });
if (platform() === 'win32') {
    execSync(
        `powershell -NoProfile -Command "Expand-Archive -LiteralPath '${tmpArchive.replace(/'/g, "''")}' -DestinationPath '${tmpExtract.replace(/'/g, "''")}' -Force"`,
        { stdio: 'inherit' },
    );
} else {
    execSync(`tar -xzf "${tmpArchive}" -C "${tmpExtract}"`, { stdio: 'inherit' });
}

const found = findBinInDir(tmpExtract);
if (!found) {
    console.error('[fetch-tgrep] tgrep binary not found in archive');
    process.exit(1);
}

cpSync(found, outBin);
if (platform() !== 'win32') {
    execSync(`chmod +x "${outBin}"`, { stdio: 'inherit' });
}

rmSync(tmpExtract, { recursive: true, force: true });
rmSync(tmpArchive, { force: true });

console.log(`[fetch-tgrep] OK — ${outBin}`);
console.log('[fetch-tgrep] tgrep is MIT-licensed (Microsoft) — vendoring the release binary is permitted.');
