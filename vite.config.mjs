import { defineConfig, loadEnv } from 'vite';
import react from '@vitejs/plugin-react';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { resolve } from 'node:path';

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
        include: ['path', 'buffer', 'stream', 'util'],
      }),
    ],
    resolve: {
      alias: {
        'fs/promises': resolve(import.meta.dirname, 'src/airi/fs-stub.ts'),
        fs: resolve(import.meta.dirname, 'src/airi/fs-stub.ts'),
        child_process: resolve(import.meta.dirname, 'src/airi/fs-stub.ts'),
        ollama: resolve(import.meta.dirname, 'node_modules/ollama/dist/browser.mjs'),
      },
    },
    build: {
      outDir: '../dist',
      emptyOutDir: true,
      target: 'es2022',
      cssCodeSplit: true,
      sourcemap: false,
      reportCompressedSize: false,
      chunkSizeWarningLimit: 1500,
      rollupOptions: {
        output: {
          manualChunks: {
            'three': ['three'],
            'vrm': ['@pixiv/three-vrm', '@pixiv/three-vrm-springbone'],
            'monaco': ['monaco-editor', '@monaco-editor/react'],
            'xterm': [
              '@xterm/xterm',
              '@xterm/addon-fit',
              '@xterm/addon-canvas',
              '@xterm/addon-webgl',
              '@xterm/addon-search',
              '@xterm/addon-web-links',
              '@xterm/addon-unicode11',
            ],
            'reactflow': ['reactflow'],
            'markdown': ['marked', 'react-markdown', 'remark-gfm'],
            'tauri': ['@tauri-apps/api', '@tauri-apps/plugin-dialog'],
          },
        },
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
      proxy: {
        '/__ollama': {
          target: proxyTarget,
          changeOrigin: true,
          secure: false,
          rewrite: (path) => path.replace(/^\/__ollama/, ''),
        },
      },
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    base: './',
  };
});
