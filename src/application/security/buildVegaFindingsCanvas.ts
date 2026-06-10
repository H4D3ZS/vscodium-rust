/**
 * Builds a findings-dashboard CanvasSpec from a Vega DAST scan result so
 * scan output auto-renders as an interactive canvas tab instead of (only)
 * a list in the side panel.
 */
import type { CanvasBlock, CanvasSpec, CanvasTone } from '../../domain/canvas/CanvasSpec';
import { canvasSlug } from '../../domain/canvas/CanvasSpec';

export interface VegaAlertLike {
    type_key: string;
    title: string;
    severity: string;
    resource: string;
    output: string;
    detection_type?: string;
}

export interface VegaScanResultLike {
    target: string;
    paths_scanned: number;
    modules_run: number;
    alerts: VegaAlertLike[];
    duration_ms: number;
    ai_triage?: string[];
}

const SEVERITIES = ['critical', 'high', 'medium', 'low', 'info'] as const;

const SEV_TONE: Record<string, CanvasTone> = {
    critical: 'danger',
    high: 'danger',
    medium: 'warning',
    low: 'neutral',
    info: 'info',
};

function severityOf(a: VegaAlertLike): string {
    const s = (a.severity || 'info').toLowerCase();
    return (SEVERITIES as readonly string[]).includes(s) ? s : 'info';
}

export function buildVegaFindingsCanvas(result: VegaScanResultLike): CanvasSpec {
    const counts: Record<string, number> = {};
    for (const sev of SEVERITIES) counts[sev] = 0;
    for (const a of result.alerts) counts[severityOf(a)] += 1;

    const blocks: CanvasBlock[] = [];

    blocks.push({
        type: 'stats',
        items: [
            { label: 'Alerts', value: result.alerts.length, tone: result.alerts.length ? 'accent' : 'success' },
            { label: 'Critical', value: counts.critical, tone: counts.critical ? 'danger' : 'neutral' },
            { label: 'High', value: counts.high, tone: counts.high ? 'danger' : 'neutral' },
            { label: 'Paths', value: result.paths_scanned, tone: 'info' },
            { label: 'Duration', value: `${(result.duration_ms / 1000).toFixed(1)}s`, tone: 'neutral' },
        ],
    });

    blocks.push({
        type: 'kv',
        title: 'Scan',
        pairs: [
            { key: 'Target', value: result.target },
            { key: 'Modules run', value: String(result.modules_run) },
        ],
    });

    if (result.alerts.length > 0) {
        blocks.push({
            type: 'chart',
            chart: 'pie',
            title: 'Alerts by severity',
            labels: SEVERITIES.filter((s) => counts[s] > 0).map((s) => s.toUpperCase()),
            series: [{ name: 'Alerts', values: SEVERITIES.filter((s) => counts[s] > 0).map((s) => counts[s]) }],
        });

        const hasTriage = (result.ai_triage?.length ?? 0) > 0;
        const columns = hasTriage
            ? ['Severity', 'Finding', 'Resource', 'Triage']
            : ['Severity', 'Finding', 'Resource'];
        const rows = result.alerts.map((a, i) => {
            const base: (string | number)[] = [
                severityOf(a).toUpperCase(),
                a.title || a.type_key,
                a.resource,
            ];
            if (hasTriage) base.push((result.ai_triage?.[i] ?? '—').replace('_', ' '));
            return base;
        });
        blocks.push({ type: 'table', title: 'Findings', columns, rows });
    } else {
        blocks.push({
            type: 'callout',
            tone: 'success',
            title: 'No alerts',
            content: 'No alerts on crawled parametric paths.',
        });
    }

    return {
        id: `vega-scan-${canvasSlug(result.target)}`,
        title: `Vega Scan · ${result.target}`,
        subtitle: `${result.alerts.length} alerts · ${result.paths_scanned} paths · ${result.modules_run} modules`,
        updatedAt: Date.now(),
        blocks,
    };
}
