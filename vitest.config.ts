/**
 * Vitest — Kortex inference + IDE bounded-context pure logic.
 * Full app UI (Tauri/Monaco/Three) stays out of scope; Rust `cargo test` covers adapters.
 */
import { defineConfig } from 'vitest/config';

export default defineConfig({
    root: '.',
    test: {
        include: [
            'src/kortex/__tests__/**/*.test.ts',
            'src/application/**/__tests__/**/*.test.ts',
            'src/domain/**/__tests__/**/*.test.ts',
        ],
        environment: 'node',
        pool: 'forks',
        isolate: true,
    },
});
