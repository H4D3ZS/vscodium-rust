import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import { nodePolyfills } from 'vite-plugin-node-polyfills';
import { resolve } from 'node:path';


export default defineConfig(() => {

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
    // Pre-bundle everything the (heavily React.lazy'd) app pulls in, so the dev
    // server doesn't discover a new dep per panel, re-optimize, and force a full
    // page reload each time — which cascades into a white-screen reload loop.
    optimizeDeps: {
      include: [
        'react', 'react-dom', 'react-dom/client',
        'zustand', 'zustand/react/shallow',
        'monaco-editor', '@monaco-editor/react',
        '@heroui/react',
        'lucide-react', '@tabler/icons-react',
        'react-window',
        'marked', 'react-markdown', 'remark-gfm',
        'diff', 'dompurify',
        'reactflow',
        'three', '@pixiv/three-vrm', '@pixiv/three-vrm-springbone',
        '@xterm/xterm', '@xterm/addon-fit', '@xterm/addon-canvas', '@xterm/addon-webgl',
        '@xterm/addon-search', '@xterm/addon-web-links', '@xterm/addon-unicode11',
        '@tauri-apps/api', '@tauri-apps/api/core', '@tauri-apps/api/event',
        '@tauri-apps/api/window', '@tauri-apps/api/path',
        '@tauri-apps/plugin-dialog',
      ],
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
    },
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    base: './',
  };
});
