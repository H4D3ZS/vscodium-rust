#!/usr/bin/env node
// Regenerate the architecture baseline from current violations.
import { readFileSync, readdirSync, statSync, writeFileSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = join(fileURLToPath(new URL('..', import.meta.url)));
const CHECKED_DIRS = ['src/components', 'src/store'];
const PATTERNS = [
    /from\s+['"]@tauri-apps\/api/,
    /\binvoke\s*</,
    /\binvoke\s*\(/,
];
const BRIDGE_IMPORT = /from\s+['"][./]*tauri_bridge['"]/;

function* walk(dir) {
    for (const name of readdirSync(dir)) {
        const p = join(dir, name);
        const st = statSync(p);
        if (st.isDirectory()) yield* walk(p);
        else if (/\.(ts|tsx)$/.test(name)) yield p;
    }
}

const violations = [];
for (const dir of CHECKED_DIRS) {
    for (const file of walk(join(ROOT, dir))) {
        const src = readFileSync(file, 'utf8');
        if (PATTERNS.some((re) => re.test(src)) || BRIDGE_IMPORT.test(src)) {
            violations.push(relative(ROOT, file).replace(/\\/g, '/'));
        }
    }
}

writeFileSync(join(ROOT, 'scripts', 'architecture-baseline.json'), JSON.stringify(violations, null, 2));
console.log(`Baseline updated: ${violations.length} files`);
