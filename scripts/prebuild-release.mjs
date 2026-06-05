#!/usr/bin/env node
/**
 * Pre-build hook: bundle browser-agent.exe (invisible_playwright sidecar) on Windows
 * before `tauri build`. Dev runs use Python + repo invisible_playwright via PYTHONPATH.
 */
import { execSync } from 'node:child_process';
import { existsSync } from 'node:fs';
import { join, dirname } from 'node:path';
import { fileURLToPath } from 'node:url';
import { platform } from 'node:os';

const root = join(dirname(fileURLToPath(import.meta.url)), '..');
const sidecarOut = join(root, 'src-tauri', 'binaries', 'browser-agent.exe');
const ipDir = join(root, 'invisible_playwright');

if (platform() !== 'win32') {
    console.log('[prebuild] Skipping browser-agent freeze on non-Windows (use Python sidecar in dev).');
    process.exit(0);
}

if (existsSync(sidecarOut) && !process.env.FORCE_SIDECAR_REBUILD) {
    console.log('[prebuild] browser-agent.exe already present — skipping PyInstaller (delete or set FORCE_SIDECAR_REBUILD=1 to rebuild).');
    process.exit(0);
}

if (!existsSync(ipDir)) {
    console.warn('[prebuild] invisible_playwright/ not found — browser will require Python at runtime.');
    process.exit(0);
}

console.log('[prebuild] Freezing invisible_playwright -> browser-agent.exe …');
execSync(
    'powershell -ExecutionPolicy Bypass -File scripts/build-sidecar.ps1',
    { cwd: root, stdio: 'inherit' },
);

if (!existsSync(sidecarOut)) {
    console.error('[prebuild] browser-agent.exe was not produced. Install: pip install -e invisible_playwright pyinstaller playwright');
    process.exit(1);
}

console.log('[prebuild] OK — browser-agent.exe ready for bundle.');
