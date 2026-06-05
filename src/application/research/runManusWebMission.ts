import { persistManusReport } from '../manus/persistManusReport';
import {
    auditUrlSecurity,
    captureStealthScreenshot,
    ensureStealthBrowser,
    navigateStealth,
    openResearchTerminal,
    readStealthPageSummary,
    scrapeUrl,
    searchWeb,
    searchWebStealth,
} from './runWebResearch';

export type ManusMissionStepStatus = 'pending' | 'running' | 'done' | 'error' | 'skipped';

export interface ManusMissionStep {
    id: string;
    label: string;
    status: ManusMissionStepStatus;
    detail?: string;
}

export interface ManusMissionResult {
    steps: ManusMissionStep[];
    report: string;
}

export interface ManusMissionOptions {
    query: string;
    targetUrl?: string;
    workspaceRoot?: string;
    runCodebaseAudit?: boolean;
    onStep?: (step: ManusMissionStep) => void;
}

const STEP_DEFS: { id: string; label: string }[] = [
    { id: 'plan', label: 'Session files' },
    { id: 'search', label: 'Web search' },
    { id: 'browser', label: 'Stealth browser research' },
    { id: 'scrape', label: 'Page scrape (stealth)' },
    { id: 'audit', label: 'URL security audit' },
    { id: 'screenshot', label: 'Browser capture' },
    { id: 'codebase', label: 'Codebase security' },
    { id: 'terminal', label: 'Research terminal' },
    { id: 'persist', label: 'Save findings' },
];

/**
 * Full autonomous web-research pipeline using invisible_playwright (stealth Firefox)
 * for all page loads — bypasses Cloudflare and chatbot gates that block plain HTTP.
 */
export async function runManusWebMission(opts: ManusMissionOptions): Promise<ManusMissionResult> {
    const { query, targetUrl, workspaceRoot, runCodebaseAudit = true, onStep } = opts;
    const steps: ManusMissionStep[] = STEP_DEFS.map(s => ({ ...s, status: 'pending' as const }));

    const emit = (id: string, patch: Partial<ManusMissionStep>) => {
        const idx = steps.findIndex(s => s.id === id);
        if (idx >= 0) {
            steps[idx] = { ...steps[idx], ...patch };
            onStep?.(steps[idx]);
        }
    };

    const chunks: string[] = [
        `# Web Mission`,
        `**Query:** ${query}`,
        `**Engine:** invisible_playwright (stealth Firefox)`,
        `**Started:** ${new Date().toISOString()}`,
        '',
    ];
    const url = targetUrl?.trim() || extractFirstUrl(query);

    emit('plan', { status: 'running' });
    if (workspaceRoot) {
        try {
            await persistManusReport(workspaceRoot, query, '(mission started)');
            emit('plan', { status: 'done', detail: 'task_plan / findings / progress' });
        } catch (e: unknown) {
            emit('plan', { status: 'error', detail: String(e) });
        }
    } else {
        emit('plan', { status: 'skipped', detail: 'no workspace' });
    }

    // Boot stealth browser once — shared across search, scrape, audit, screenshot.
    emit('browser', { status: 'running' });
    try {
        await ensureStealthBrowser();
        const navUrl = url || `https://www.google.com/search?q=${encodeURIComponent(query)}`;
        await navigateStealth(navUrl);
        const summary = await readStealthPageSummary();
        const linkLines = (summary.links || [])
            .slice(0, 15)
            .map(l => `- ${l.text || '(link)'}: ${l.href}`)
            .join('\n');
        chunks.push(
            '## Stealth Browser Research',
            `**URL:** ${navUrl}`,
            '',
            summary.text?.slice(0, 8000) || '(no body text)',
            linkLines ? `\n### Links\n${linkLines}` : '',
            '',
        );
        emit('browser', { status: 'done', detail: 'invisible_playwright' });
    } catch (e: unknown) {
        emit('browser', { status: 'error', detail: String(e) });
    }

    emit('search', { status: 'running' });
    try {
        let searchText = await searchWeb(query).catch(() => '');
        if (!searchText || searchText.length < 80) {
            searchText = await searchWebStealth(query);
        }
        chunks.push('## Web Search', searchText.slice(0, 8000), '');
        emit('search', { status: 'done', detail: `${searchText.length} chars` });
    } catch (e: unknown) {
        emit('search', { status: 'error', detail: String(e) });
    }

    if (url) {
        emit('scrape', { status: 'running' });
        try {
            const page = await scrapeUrl(url);
            chunks.push(`## Stealth Scrape: ${url}`, page.slice(0, 10000), '');
            emit('scrape', { status: 'done', detail: `${page.length} chars` });
        } catch (e: unknown) {
            emit('scrape', { status: 'error', detail: String(e) });
        }

        emit('audit', { status: 'running' });
        try {
            await navigateStealth(url);
            const audit = await auditUrlSecurity(url);
            chunks.push('## URL Security Audit', audit.slice(0, 12000), '');
            emit('audit', { status: 'done', detail: 'APEX scan' });
        } catch (e: unknown) {
            emit('audit', { status: 'error', detail: String(e) });
        }

        emit('screenshot', { status: 'running' });
        try {
            const shot = await captureStealthScreenshot();
            if (shot) {
                chunks.push('## Browser Screenshot', `Captured (${shot.length} bytes base64)`, '');
                emit('screenshot', { status: 'done', detail: 'captured' });
            } else {
                emit('screenshot', { status: 'skipped', detail: 'empty' });
            }
        } catch (e: unknown) {
            emit('screenshot', { status: 'error', detail: String(e) });
        }
    } else {
        emit('scrape', { status: 'skipped', detail: 'no URL' });
        emit('audit', { status: 'skipped', detail: 'no URL' });
        emit('screenshot', { status: 'skipped', detail: 'no URL' });
    }

    if (runCodebaseAudit && workspaceRoot) {
        emit('codebase', { status: 'running' });
        try {
            const { runCodebaseSecurityReview } = await import('../security/runCodebaseSecurityReview');
            await runCodebaseSecurityReview({ depth: 'standard' });
            const report = (await import('../../store')).useStore.getState().securityReviewReport;
            if (report) {
                const critical = report.bySeverity?.critical ?? report.bySeverity?.CRITICAL ?? 0;
                chunks.push(
                    '## Codebase Security',
                    `Findings: ${report.totalFindings} · Critical: ${critical}`,
                    report.summary?.slice(0, 4000) || '',
                    '',
                );
            }
            emit('codebase', { status: 'done', detail: report ? `${report.totalFindings} findings` : 'complete' });
        } catch (e: unknown) {
            emit('codebase', { status: 'error', detail: String(e) });
        }
    } else {
        emit('codebase', { status: 'skipped', detail: 'disabled' });
    }

    emit('terminal', { status: 'running' });
    try {
        await openResearchTerminal();
        chunks.push('## Terminal', 'Research terminal opened — run follow-up commands there.', '');
        emit('terminal', { status: 'done', detail: 'bottom panel' });
    } catch (e: unknown) {
        emit('terminal', { status: 'error', detail: String(e) });
    }

    const report = chunks.join('\n');

    emit('persist', { status: 'running' });
    if (workspaceRoot) {
        try {
            await persistManusReport(workspaceRoot, query, report);
            emit('persist', { status: 'done', detail: 'findings.md updated' });
        } catch (e: unknown) {
            emit('persist', { status: 'error', detail: String(e) });
        }
    } else {
        emit('persist', { status: 'skipped', detail: 'no workspace' });
    }

    return { steps, report };
}

function extractFirstUrl(text: string): string | undefined {
    const m = text.match(/\bhttps?:\/\/[^\s)]+/i);
    return m?.[0];
}

/** Back-compat wrapper */
export async function runManusWebMissionLegacy(
    query: string,
    targetUrl?: string,
    onStep?: (step: ManusMissionStep) => void,
): Promise<ManusMissionResult> {
    return runManusWebMission({ query, targetUrl, onStep });
}
