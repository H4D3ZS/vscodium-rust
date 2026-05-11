/**
 * Vitest config — scoped to the Kortex inference modules. The full app pulls
 * in Tauri APIs, Three.js, Monaco, etc. that aren't worth the ceremony of
 * mocking out for unit tests. We test the pure functions and let cargo
 * cover everything that ships through Tauri commands.
 */
import { defineConfig } from 'vitest/config';

export default defineConfig({
    // Don't inherit `root: './src'` from vite.config.ts — vitest needs the
    // package root so it can resolve `vitest` itself.
    root: '.',
    test: {
        include: ['src/kortex/__tests__/**/*.test.ts'],
        environment: 'node',
        pool: 'forks',
        isolate: true,
    },
});
