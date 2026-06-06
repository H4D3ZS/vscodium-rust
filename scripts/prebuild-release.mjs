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

console.log('[prebuild] Sidecar binaries ready for bundle.');
