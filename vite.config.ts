import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { resolve } from 'path';

export default defineConfig({
    root: './src',
    plugins: [
      react(),
      nodePolyfills({
        protocolImports: true,
        // 'crypto' intentionally omitted — it drags in `elliptic` which has an
        // open low-severity CVE (GHSA-848j-6mx2-7j84) with no upstream fix yet.
        // No source file imports `crypto`; browser-side code uses WebCrypto.
        include: ['path', 'http', 'https', 'events', 'os', 'buffer', 'stream', 'util'],
      }),
    ],
    resolve: {
        alias: {
            'fs/promises': resolve(__dirname, 'src/airi/fs-stub.ts'),
            'fs': resolve(__dirname, 'src/airi/fs-stub.ts'),
            'child_process': resolve(__dirname, 'src/airi/fs-stub.ts'),
            'ollama': resolve(__dirname, 'node_modules/ollama/dist/browser.mjs'),
        },
    },
    build: {
        outDir: '../dist',
        emptyOutDir: true,
        rollupOptions: {
            onwarn(warning, warn) {
                if (warning.code === 'MODULE_LEVEL_DIRECTIVE') return;
                warn(warning);
            },
        },
    },
    server: {
        port: 5173,
        strictPort: false,
        host: true,
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    base: './',
});
