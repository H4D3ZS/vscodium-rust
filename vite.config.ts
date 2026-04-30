import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
    root: './src',
    plugins: [react()],
    build: {
        outDir: '../dist',
        emptyOutDir: true,
    },
    server: {
        port: 5173,
        strictPort: false,  // Don't fail if port is in use, find another
        host: true,
    },
    // Vite settings for Tauri
    clearScreen: false,
    envPrefix: ['VITE_', 'TAURI_'],
    // Tauri needs relative asset paths for production builds
    base: './',
});
