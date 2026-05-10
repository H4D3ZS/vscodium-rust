/**
 * Vitest config — scoped to the Kortex inference modules. The full app pulls
 * in Tauri APIs, Three.js, Monaco, etc. that aren't worth the ceremony of
 * mocking out for unit tests. We test the pure functions and let cargo
 * cover everything that ships through Tauri commands.
 */
import { defineConfig } from 'vitest/config';

export default defineConfig({
    test: {
        // Scope: only run the Kortex __tests__ suite for now. Add more globs
        // here as more modules grow proper unit tests.
        include: ['src/kortex/__tests__/**/*.test.ts'],
        environment: 'node',
        // Each .test.ts file gets a fresh module graph, so module-level state
        // (e.g. CCET HISTORY) is naturally isolated across files.
        isolate: true,
        // Fail on console errors so flaky logging in product code surfaces.
        printConsoleTrace: true,
    },
});
