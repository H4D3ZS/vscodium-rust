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
          // Object-form manualChunks was dropped from Rollup's types in the
          // rollup major that ships with Vite 8; same groupings, function form.
          manualChunks(id) {
            if (id.includes('monaco-editor') || id.includes('@monaco-editor/react')) return 'monaco';
            if (
              id.includes('@xterm/xterm') ||
              id.includes('@xterm/addon-fit') ||
              id.includes('@xterm/addon-canvas') ||
              id.includes('@xterm/addon-webgl') ||
              id.includes('@xterm/addon-search') ||
              id.includes('@xterm/addon-web-links') ||
              id.includes('@xterm/addon-unicode11')
            ) return 'xterm';
            if (id.includes('reactflow')) return 'reactflow';
            if (id.includes('marked') || id.includes('react-markdown') || id.includes('remark-gfm')) return 'markdown';
            if (id.includes('@tauri-apps/api') || id.includes('@tauri-apps/plugin-dialog')) return 'tauri';
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
