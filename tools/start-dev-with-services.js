#!/usr/bin/env node
/**
 * Start Vite dev server + background services (Qwen3-TTS + AIRI 3D)
 */

import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);
const rootDir = join(__dirname, '..');

console.log('╔══════════════════════════════════════════════════════════╗');
console.log('║      Starting AIRI Development Environment               ║');
console.log('╚══════════════════════════════════════════════════════════╝');
console.log('');

// Start Qwen3-TTS Python server
console.log('[1/3] Starting Qwen3-TTS Server on port 8081...');
const qwenProcess = spawn('python', ['qwen-tts-server.py'], {
  cwd: rootDir,
  stdio: 'ignore',
  shell: true
});

qwenProcess.on('error', (err) => {
  console.warn('⚠️  Failed to start Qwen3-TTS:', err.message);
});

setTimeout(() => {
  // Start AIRI 3D app
  console.log('[2/3] Starting AIRI 3D App on port 5174...');
  const airiProcess = spawn('npm', ['run', 'dev'], {
    cwd: join(rootDir, 'airi/apps/stage-web'),
    stdio: 'ignore',
    shell: true
  });

  airiProcess.on('error', (err) => {
    console.warn('⚠️  Failed to start AIRI 3D:', err.message);
  });

  setTimeout(() => {
    // Start main Vite dev server
    console.log('[3/3] Starting Vite Dev Server on port 5173...');
    console.log('');
    console.log('✅ All services starting...');
    console.log('');
    console.log('Services:');
    console.log('  🌐 Main IDE:   http://localhost:5173');
    console.log('  🎭 AIRI 3D:    http://localhost:5174');
    console.log('  🎤 Qwen3-TTS:  http://localhost:8081');
    console.log('');

    const viteProcess = spawn('npx', ['vite'], {
      cwd: rootDir,
      stdio: 'inherit',
      shell: true
    });

    viteProcess.on('close', (code) => {
      // Cleanup
      qwenProcess.kill();
      airiProcess.kill();
      process.exit(code);
    });
  }, 2000);
}, 2000);
