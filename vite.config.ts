import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { resolve } from 'path';

// Default Ollama target for the Vite dev proxy. Override by exporting
// VITE_OLLAMA_DEV_PROXY=<https://your-vps.example.com> (or
// http://localhost:11434 for a fully-local dev box).
// In production the Tauri app uses Rust IPC (no proxy required).
const DEV_OLLAMA_DEFAULT = 'https://ai.cyberifrit.xyz';

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');
  const proxyTarget = env.VITE_OLLAMA_DEV_PROXY || DEV_OLLAMA_DEFAULT;
  return {
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
        // Dev-only Ollama proxy so the renderer can talk to the remote VPS proxy
        // without nginx CORS rejecting `http://localhost:5173`. Routes:
        //   /__ollama/api/tags        -> ${proxyTarget}/api/tags
        //   /__ollama/api/generate    -> ${proxyTarget}/api/generate
        //   /__ollama/v1/...          -> ${proxyTarget}/v1/...
        // The Tauri build uses Rust IPC and never hits this path.
        proxy: {
            '/__ollama': {
                target: proxyTarget,
                changeOrigin: true,
                secure: false,
                rewrite: (path: string) => path.replace(/^\/__ollama/, ''),
            },
        },
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    base: './',
  };
});
