import { describe, it, expect } from 'vitest';
import { buildVegaFindingsCanvas, type VegaScanResultLike } from '../buildVegaFindingsCanvas';

const baseResult: VegaScanResultLike = {
    target: 'https://target.app/',
    paths_scanned: 12,
    modules_run: 8,
    duration_ms: 4200,
    alerts: [
        { type_key: 'sqli', title: 'SQL Injection', severity: 'critical', resource: '/search?q=', output: 'diff' },
        { type_key: 'xss', title: 'Reflected XSS', severity: 'high', resource: '/profile', output: 'echo' },
        { type_key: 'hdr', title: 'Missing CSP', severity: 'INFO', resource: '/', output: '' },
    ],
};

describe('buildVegaFindingsCanvas', () => {
    it('produces stable id keyed on target', () => {
        const spec = buildVegaFindingsCanvas(baseResult);
        expect(spec.id).toBe(buildVegaFindingsCanvas(baseResult).id);
        expect(spec.id.startsWith('vega-scan-')).toBe(true);
    });

    it('summarizes severities in stats and pie chart', () => {
        const spec = buildVegaFindingsCanvas(baseResult);
        const stats = spec.blocks.find((b) => b.type === 'stats');
        expect(stats).toBeDefined();
        if (stats?.type === 'stats') {
            const critical = stats.items.find((i) => i.label === 'Critical');
            expect(critical?.value).toBe(1);
            expect(critical?.tone).toBe('danger');
        }
        const chart = spec.blocks.find((b) => b.type === 'chart');
        expect(chart).toBeDefined();
        if (chart?.type === 'chart') {
            expect(chart.labels).toEqual(['CRITICAL', 'HIGH', 'INFO']);
            expect(chart.series[0].values).toEqual([1, 1, 1]);
        }
    });

    it('builds a findings table, adding triage column when present', () => {
        const spec = buildVegaFindingsCanvas({ ...baseResult, ai_triage: ['CONFIRMED', 'LIKELY', 'FALSE_POSITIVE'] });
        const table = spec.blocks.find((b) => b.type === 'table');
        expect(table).toBeDefined();
        if (table?.type === 'table') {
            expect(table.columns).toEqual(['Severity', 'Finding', 'Resource', 'Triage']);
            expect(table.rows[0]).toEqual(['CRITICAL', 'SQL Injection', '/search?q=', 'CONFIRMED']);
            expect(table.rows[2][3]).toBe('FALSE POSITIVE');
        }
    });

    it('renders a success callout instead of table when clean', () => {
        const spec = buildVegaFindingsCanvas({ ...baseResult, alerts: [] });
        expect(spec.blocks.some((b) => b.type === 'table')).toBe(false);
        const callout = spec.blocks.find((b) => b.type === 'callout');
        expect(callout).toBeDefined();
        if (callout?.type === 'callout') expect(callout.tone).toBe('success');
    });

    it('coerces unknown severities to info', () => {
        const spec = buildVegaFindingsCanvas({
            ...baseResult,
            alerts: [{ type_key: 'x', title: 'Weird', severity: 'banana', resource: '/', output: '' }],
        });
        const table = spec.blocks.find((b) => b.type === 'table');
        if (table?.type === 'table') expect(table.rows[0][0]).toBe('INFO');
    });
});
