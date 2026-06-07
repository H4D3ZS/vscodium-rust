#!/usr/bin/env node
/**
 * Pre-build hook: freeze sidecar binaries into src-tauri/binaries/ before
 * `tauri build` so the MSI/NSIS installer ships them without Python or a
 * separate cargo build on the user's machine.
 *
 *   browser-agent.exe  — invisible_playwright (PyInstaller)
 *   claurst.exe        — optional GPL agent backend (cargo release)
 */
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { platform } from 'node:os';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const binariesDir = join(root, 'src-tauri', 'binaries');
const sidecarOut = join(binariesDir, 'browser-agent.exe');
const claurstOut = join(binariesDir, 'claurst.exe');
const ipDir = join(root, 'invisible_playwright');
const claurstDir = join(root, 'claurst', 'src-rust');

function runPs1(script) {
    execSync(`powershell -ExecutionPolicy Bypass -File ${script}`, {
        cwd: root,
        stdio: 'inherit',
    });
}

function fetchBundledRipgrep() {
    const rgBin = join(root, 'src-tauri', 'bundles', 'ripgrep', platform() === 'win32' ? 'rg.exe' : 'rg');
    if (existsSync(rgBin) && !process.env.FORCE_BUNDLE_FETCH) {
        console.log('[prebuild] ripgrep bundle present — skip (FORCE_BUNDLE_FETCH=1 to re-fetch).');
        return;
    }
    console.log('[prebuild] Fetching bundled ripgrep …');
    try {
        execSync('node scripts/fetch-ripgrep.mjs', { cwd: root, stdio: 'inherit' });
    } catch (e) {
        console.warn('[prebuild] fetch-ripgrep.mjs failed (non-fatal for dev):', e?.message ?? e);
    }
    if (!existsSync(rgBin)) {
        console.warn('[prebuild] ripgrep not ready — run node scripts/fetch-ripgrep.mjs');
    } else {
        console.log('[prebuild] OK — ripgrep bundle ready.');
    }
}

fetchBundledRipgrep();

if (platform() !== 'win32') {
    console.log('[prebuild] Skipping Windows sidecar freeze on non-Windows (dev uses Python + cargo paths).');
    process.exit(0);
}

// ── invisible_playwright → browser-agent.exe ────────────────────────────────
if (existsSync(sidecarOut) && !process.env.FORCE_SIDECAR_REBUILD) {
    console.log('[prebuild] browser-agent.exe present — skip (FORCE_SIDECAR_REBUILD=1 to rebuild).');
} else if (!existsSync(ipDir)) {
    console.warn('[prebuild] invisible_playwright/ missing — browser will need Python at runtime.');
} else {
    console.log('[prebuild] Freezing invisible_playwright -> browser-agent.exe …');
    runPs1('scripts/build-sidecar.ps1');
    if (!existsSync(sidecarOut)) {
        console.error('[prebuild] browser-agent.exe not produced. pip install -e invisible_playwright pyinstaller playwright');
        process.exit(1);
    }
    console.log('[prebuild] OK — browser-agent.exe ready.');
}

// ── claurst → claurst.exe ─────────────────────────────────────────────────
if (existsSync(claurstOut) && !process.env.FORCE_CLAURST_REBUILD) {
    console.log('[prebuild] claurst.exe present — skip (FORCE_CLAURST_REBUILD=1 to rebuild).');
} else if (!existsSync(claurstDir)) {
    console.warn('[prebuild] claurst/src-rust missing — optional Claurst backend dev-only.');
} else {
    console.log('[prebuild] Building claurst -> claurst.exe …');
    runPs1('scripts/build-claurst.ps1');
    if (!existsSync(claurstOut)) {
        console.error('[prebuild] claurst.exe not produced.');
        process.exit(1);
    }
    console.log('[prebuild] OK — claurst.exe ready.');
}

// ── PortableGit + Hermes skills bundles for installer ───────────────────────
const gitBash = join(root, 'src-tauri', 'bundles', 'portable-git', 'bin', 'bash.exe');
if (existsSync(gitBash) && !process.env.FORCE_BUNDLE_FETCH) {
    console.log('[prebuild] portable-git bundle present — skip (FORCE_BUNDLE_FETCH=1 to re-fetch).');
} else {
    console.log('[prebuild] Fetching installer bundles (PortableGit + skills) …');
    try {
        runPs1('scripts/fetch-bundles.ps1');
    } catch (e) {
        console.warn('[prebuild] fetch-bundles.ps1 failed (non-fatal for dev):', e?.message ?? e);
    }
    if (!existsSync(gitBash)) {
        console.warn('[prebuild] portable-git not ready — terminal will auto-install on first launch if bundled.');
    } else {
        console.log('[prebuild] OK — portable-git bundle ready.');
    }
}

// ── Language servers (TS/JS, Flutter, Android, Kotlin, Java, …) ─────────────
const tsLsp = join(binariesDir, 'lsp', 'typescript-language-server', 'typescript-language-server.cmd');
if (existsSync(tsLsp) && !process.env.FORCE_LSP_FETCH) {
    console.log('[prebuild] LSP bundle present — skip (FORCE_LSP_FETCH=1 to re-fetch).');
} else {
    console.log('[prebuild] Fetching language server bundles …');
    runPs1('scripts/fetch-lsp-binaries.ps1');
    if (!existsSync(tsLsp)) {
        console.warn('[prebuild] typescript-language-server missing — run scripts/fetch-lsp-binaries.ps1 manually.');
    } else {
        console.log('[prebuild] OK — LSP bundles ready.');
    }
}

console.log('[prebuild] Sidecar binaries ready for bundle.');
