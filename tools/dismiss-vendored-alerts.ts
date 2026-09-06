#!/usr/bin/env node
/**
 * Bulk-dismiss Dependabot alerts that target vendored upstream code paths.
 *
 * Why: this monorepo vendors entire upstream projects (airi/, kortex/llama.cpp/,
 * claurst/kilocode/) which generate 200+ alerts that we cannot fix in this
 * repo — fixes have to come from upstream. The owned roots ('/', '/src-tauri/')
 * are kept open and triaged via SECURITY.md.
 *
 * Usage:
 *   export GITHUB_TOKEN=ghp_xxx   # must have security_events:write
 *   node tools/dismiss-vendored-alerts.ts                 # dry-run
 *   node tools/dismiss-vendored-alerts.ts --apply         # actually dismiss
 *
 * The script never touches alerts whose manifest_path begins with:
 *   - package.json                  (root npm)
 *   - package-lock.json
 *   - src-tauri/Cargo.toml
 *   - src-tauri/Cargo.lock
 *
 * Everything else is treated as vendored and dismissed as `tolerable_risk`.
 *
 * Runs directly under Node >=23.6 (native TypeScript type stripping).
 */

export {}; // top-level await requires module context

interface DependabotAlert {
    number: number;
    dependency?: { manifest_path?: string };
}

const REPO = process.env.GITHUB_REPO || 'H4D3ZS/vscodium-rust';
const TOKEN = process.env.GITHUB_TOKEN;
const APPLY = process.argv.includes('--apply');

if (!TOKEN) {
    console.error('Set GITHUB_TOKEN (PAT with security_events:write).');
    process.exit(2);
}

const OWNED = new Set([
    'package.json',
    'package-lock.json',
    'src-tauri/Cargo.toml',
    'src-tauri/Cargo.lock',
]);

const headers: Record<string, string> = {
    Authorization: `Bearer ${TOKEN}`,
    Accept: 'application/vnd.github+json',
    'X-GitHub-Api-Version': '2022-11-28',
};

async function* listAlerts(): AsyncGenerator<DependabotAlert> {
    let url: string | null = `https://api.github.com/repos/${REPO}/dependabot/alerts?state=open&per_page=100`;
    while (url) {
        const res: Response = await fetch(url, { headers });
        if (!res.ok) {
            const body = await res.text();
            throw new Error(`GET ${url} -> ${res.status}\n${body}`);
        }
        const items = (await res.json()) as DependabotAlert[];
        for (const a of items) yield a;

        const link = res.headers.get('link') || '';
        const nextMatch = link.match(/<([^>]+)>;\s*rel="next"/);
        url = nextMatch ? nextMatch[1] : null;
    }
}

async function dismissAlert(number: number, reason: string, comment: string): Promise<void> {
    const res = await fetch(
        `https://api.github.com/repos/${REPO}/dependabot/alerts/${number}`,
        {
            method: 'PATCH',
            headers: { ...headers, 'Content-Type': 'application/json' },
            body: JSON.stringify({
                state: 'dismissed',
                dismissed_reason: reason,
                dismissed_comment: comment,
            }),
        }
    );
    if (!res.ok) {
        const body = await res.text();
        throw new Error(`PATCH alert #${number} -> ${res.status}\n${body}`);
    }
}

let total = 0;
let owned = 0;
let vendored = 0;
const vendoredByPath = new Map<string, number>();

for await (const a of listAlerts()) {
    total++;
    const path = a.dependency?.manifest_path || '<unknown>';
    if (OWNED.has(path)) {
        owned++;
        continue;
    }
    vendored++;
    vendoredByPath.set(path, (vendoredByPath.get(path) || 0) + 1);
    if (APPLY) {
        try {
            await dismissAlert(
                a.number,
                'tolerable_risk',
                'Vendored upstream code; tracked upstream. See SECURITY.md.'
            );
            process.stdout.write('.');
        } catch (err) {
            console.error(`\n#${a.number}: ${(err as Error).message}`);
        }
    }
}

console.log('\n');
console.log(`Total open alerts: ${total}`);
console.log(`In owned roots (kept open): ${owned}`);
console.log(`In vendored upstream (${APPLY ? 'dismissed' : 'would dismiss with --apply'}): ${vendored}`);
console.log('\nPer-path breakdown:');
for (const [p, n] of [...vendoredByPath.entries()].sort((a, b) => b[1] - a[1])) {
    console.log(`  ${n.toString().padStart(4)}  ${p}`);
}

if (!APPLY) {
    console.log('\nDry-run complete. Re-run with --apply to dismiss.');
}
