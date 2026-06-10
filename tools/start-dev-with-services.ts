#!/usr/bin/env node
/**
 * Start Vite dev server + background services (Qwen3-TTS + AIRI 3D)
 *
 * Each optional service is gated on prerequisites being present so the
 * launcher never half-dies when (e.g.) the AIRI pnpm workspace hasn't been
 * installed yet. Missing prereqs degrade gracefully with a clear log line.
 *
 * Runs directly under Node >=23.6 (native TypeScript type stripping).
 */

import { spawn, type ChildProcess, type SpawnOptions } from 'node:child_process';
import { existsSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import { dirname, join } from 'node:path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

console.log('╔══════════════════════════════════════════════════════════╗');
console.log('║      Starting AIRI Development Environment               ║');
console.log('╚══════════════════════════════════════════════════════════╝');
console.log('');

const children: ChildProcess[] = [];

function spawnChild(label: string, cmd: string, args: string[], opts: SpawnOptions): ChildProcess {
  const child = spawn(cmd, args, { stdio: 'ignore', shell: true, ...opts });
  child.on('error', (err) => {
    console.warn(`⚠️  ${label} failed to start:`, err.message);
  });
  child.on('exit', (code) => {
    if (code != null && code !== 0) {
      console.warn(`⚠️  ${label} exited with code ${code}`);
    }
  });
  children.push(child);
  return child;
}

const qwenScript = join(rootDir, 'qwen-tts-server.py');
if (existsSync(qwenScript)) {
  console.log('[1/3] Starting Qwen3-TTS Server on port 8081...');
  spawnChild('Qwen3-TTS', 'python', ['qwen-tts-server.py'], { cwd: rootDir });
} else {
  console.log('[1/3] Skipping Qwen3-TTS (qwen-tts-server.py not found).');
}

const airiDir = join(rootDir, 'airi/apps/stage-web');
const airiInstalled = existsSync(join(airiDir, 'node_modules'));
if (airiInstalled) {
  console.log('[2/3] Starting AIRI 3D App on port 5174...');
  spawnChild('AIRI 3D', 'npm', ['run', 'dev'], { cwd: airiDir });
} else {
  console.log('[2/3] Skipping AIRI 3D — `airi/` workspace has no node_modules.');
  console.log('       Run `cd airi && pnpm install` once to enable the avatar iframe.');
}

setTimeout(() => {
  console.log('[3/3] Starting Vite Dev Server on port 5173...');
  console.log('');
  console.log('✅ Services starting...');
  console.log('');
  console.log('  🌐 Main IDE:   http://localhost:5173');
  if (airiInstalled) console.log('  🎭 AIRI 3D:    http://localhost:5174');
  if (existsSync(qwenScript)) console.log('  🎤 Qwen3-TTS:  http://localhost:8081');
  console.log('');

  const viteProcess = spawn('npx', ['vite'], {
    cwd: rootDir,
    stdio: 'inherit',
    shell: true,
  });

  viteProcess.on('close', (code) => {
    for (const c of children) {
      try { c.kill(); } catch { /* ignore */ }
    }
    process.exit(code ?? 0);
  });
}, 1500);
