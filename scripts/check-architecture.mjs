#!/usr/bin/env node
// Architecture layering check (docs/overhaul/CONVENTIONS.md §1).
//
// Rule: UI layers never talk to Tauri directly. `invoke(` / `@tauri-apps/api`
// belong in src/infrastructure (adapters) and src/tauri_bridge / tauri_api_shim.
// Components and the store consume adapters or hooks instead.
//
// Existing violations are grandfathered in BASELINE below — the check fails
// only when a NEW file violates the rule or a clean file regresses. Shrink
// the baseline as A2 migrations land; never grow it.

import { readFileSync, readdirSync, statSync } from 'node:fs';
import { join, relative } from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = join(fileURLToPath(new URL('..', import.meta.url)));
const CHECKED_DIRS = ['src/components', 'src/store'];
const PATTERNS = [
    /from\s+['"]@tauri-apps\/api/,
    /\binvoke\s*</, // typed invoke<...>(
    /\binvoke\s*\(/,
];
// Imports of the in-repo bridge are allowed only via adapters; direct use in
// components is part of the same debt. Tracked in the same baseline.
const BRIDGE_IMPORT = /from\s+['"][./]*tauri_bridge['"]/;

const baselinePath = join(ROOT, 'scripts', 'architecture-baseline.json');
let baseline = [];
try {
    baseline = JSON.parse(readFileSync(baselinePath, 'utf8'));
} catch {
    /* no baseline yet — strict mode */
}

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
        const hit = PATTERNS.some((re) => re.test(src)) || BRIDGE_IMPORT.test(src);
        if (hit) violations.push(relative(ROOT, file).replace(/\\/g, '/'));
    }
}

const baselineSet = new Set(baseline);
const newViolations = violations.filter((v) => !baselineSet.has(v));
const fixed = baseline.filter((b) => !violations.includes(b));

if (fixed.length > 0) {
    console.log(
        `check-architecture: ${fixed.length} baseline file(s) now clean — remove from scripts/architecture-baseline.json:\n  ${fixed.join('\n  ')}`,
    );
}

if (newViolations.length > 0) {
    console.error(
        `check-architecture: FAIL — direct Tauri access in UI layers (use src/infrastructure adapters):\n  ${newViolations.join('\n  ')}`,
    );
    process.exit(1);
}

console.log(
    `check-architecture: OK (${violations.length} grandfathered, 0 new violations)`,
);
