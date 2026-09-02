/**
 * Canvas spec — declarative schema for agent-generated interactive artifacts.
 *
 * Design constraint: must be producible by 2b–4b local models in offline mode,
 * so the schema is flat JSON (no code generation, no runtime compilation).
 * The normalizer below is deliberately tolerant: small models emit slightly
 * malformed specs, and we coerce/salvage instead of rejecting.
 */

export type CanvasTone = 'neutral' | 'success' | 'warning' | 'danger' | 'info' | 'accent';

export interface CanvasStatItem {
    label: string;
    value: string | number;
    tone?: CanvasTone;
    hint?: string;
}

export interface CanvasChartSeries {
    name?: string;
    values: number[];
    tone?: CanvasTone;
}

export interface CanvasProgressItem {
    label: string;
    value: number;
    max?: number;
    tone?: CanvasTone;
}

export interface CanvasTodoItem {
    text: string;
    done?: boolean;
}

export interface CanvasTimelineItem {
    title: string;
    detail?: string;
    status?: 'done' | 'active' | 'pending';
}

export type CanvasBlock =
    | { type: 'stats'; items: CanvasStatItem[] }
    | { type: 'table'; title?: string; columns: string[]; rows: (string | number)[][] }
    | { type: 'chart'; chart: 'bar' | 'line' | 'pie'; title?: string; labels: string[]; series: CanvasChartSeries[] }
    | { type: 'markdown'; content: string }
    | { type: 'callout'; tone?: CanvasTone; title?: string; content: string }
    | { type: 'progress'; title?: string; items: CanvasProgressItem[] }
    | { type: 'todo'; title?: string; items: CanvasTodoItem[] }
    | { type: 'kv'; title?: string; pairs: { key: string; value: string }[] }
    | { type: 'code'; title?: string; language?: string; content: string }
    | { type: 'timeline'; title?: string; items: CanvasTimelineItem[] };

export interface CanvasSpec {
    id: string;
    title: string;
    subtitle?: string;
    updatedAt: number;
    blocks: CanvasBlock[];
}

export const CANVAS_FILE_SUFFIX = '.canvas.json';
export const CANVAS_DIR = '.agent/canvases';

export function canvasSlug(title: string): string {
    return (title || 'canvas')
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/^-+|-+$/g, '')
        .slice(0, 60) || 'canvas';
}

// ── Tolerant normalization ───────────────────────────────────────────────────

const TONES: CanvasTone[] = ['neutral', 'success', 'warning', 'danger', 'info', 'accent'];

function asTone(v: unknown): CanvasTone | undefined {
    const s = String(v ?? '').toLowerCase();
    if ((TONES as string[]).includes(s)) return s as CanvasTone;
    // Common aliases small models produce
    if (s === 'green' || s === 'ok' || s === 'good' || s === 'pass') return 'success';
    if (s === 'yellow' || s === 'orange' || s === 'warn') return 'warning';
    if (s === 'red' || s === 'error' || s === 'critical' || s === 'fail' || s === 'high') return 'danger';
    if (s === 'blue') return 'info';
    if (s === 'purple') return 'accent';
    return undefined;
}

function asString(v: unknown, fallback = ''): string {
    if (typeof v === 'string') return v;
    if (typeof v === 'number' || typeof v === 'boolean') return String(v);
    return fallback;
}

function asNumber(v: unknown): number {
    if (typeof v === 'number' && Number.isFinite(v)) return v;
    const n = parseFloat(String(v ?? '').replace(/[^0-9.+-]/g, ''));
    return Number.isFinite(n) ? n : 0;
}

function asArray(v: unknown): any[] {
    return Array.isArray(v) ? v : [];
}

/** Map block-type aliases small models commonly emit onto canonical types. */
function canonicalBlockType(raw: string): string {
    const t = raw.toLowerCase().replace(/[^a-z]/g, '');
    const aliases: Record<string, string> = {
        stats: 'stats', stat: 'stats', metrics: 'stats', metric: 'stats', cards: 'stats',
        table: 'table', datatable: 'table', grid: 'table',
        chart: 'chart', barchart: 'chart', linechart: 'chart', piechart: 'chart', graph: 'chart',
        markdown: 'markdown', text: 'markdown', md: 'markdown', paragraph: 'markdown', section: 'markdown',
        callout: 'callout', alert: 'callout', note: 'callout', banner: 'callout', warning: 'callout',
        progress: 'progress', progressbar: 'progress', bars: 'progress',
        todo: 'todo', todos: 'todo', tasks: 'todo', checklist: 'todo', list: 'todo',
        kv: 'kv', keyvalue: 'kv', properties: 'kv', details: 'kv', facts: 'kv',
        code: 'code', codeblock: 'code', snippet: 'code', diff: 'code',
        timeline: 'timeline', steps: 'timeline', history: 'timeline',
    };
    return aliases[t] ?? t;
}

function normalizeBlock(raw: any): CanvasBlock | null {
    if (!raw || typeof raw !== 'object') return null;
    const type = canonicalBlockType(asString(raw.type));

    switch (type) {
        case 'stats': {
            const items = asArray(raw.items ?? raw.stats ?? raw.cards).map((it) => ({
                label: asString(it?.label ?? it?.name ?? it?.title, 'stat'),
                value: typeof it?.value === 'number' ? it.value : asString(it?.value, '—'),
                tone: asTone(it?.tone ?? it?.color),
                hint: it?.hint != null ? asString(it.hint) : undefined,
            }));
            return items.length ? { type: 'stats', items } : null;
        }
        case 'table': {
            const columns = asArray(raw.columns ?? raw.headers ?? raw.header).map((c) => asString(c, ''));
            let rows = asArray(raw.rows ?? raw.data).map((r) => {
                if (Array.isArray(r)) return r.map((c) => (typeof c === 'number' ? c : asString(c, '')));
                if (r && typeof r === 'object') {
                    // Row given as object — project onto columns (or object values)
                    return columns.length
                        ? columns.map((c) => {
                            const v = r[c] ?? r[c.toLowerCase()];
                            return typeof v === 'number' ? v : asString(v, '');
                        })
                        : Object.values(r).map((v) => (typeof v === 'number' ? v as number : asString(v, '')));
                }
                return [asString(r, '')];
            });
            if (!columns.length && rows.length) {
                // Derive generic headers
                const width = Math.max(...rows.map((r) => r.length));
                for (let i = 0; i < width; i++) columns.push(`Col ${i + 1}`);
            }
            rows = rows.filter((r) => r.length > 0);
            return columns.length && rows.length ? { type: 'table', title: raw.title ? asString(raw.title) : undefined, columns, rows } : null;
        }
        case 'chart': {
            const chartKindRaw = asString(raw.chart ?? raw.kind ?? raw.variant, 'bar').toLowerCase();
            const chart = chartKindRaw.includes('pie') || chartKindRaw.includes('donut') ? 'pie'
                : chartKindRaw.includes('line') || chartKindRaw.includes('area') ? 'line' : 'bar';
            const labels = asArray(raw.labels ?? raw.categories ?? raw.x).map((l) => asString(l, ''));
            let series: CanvasChartSeries[] = asArray(raw.series).map((s) => ({
                name: s?.name != null ? asString(s.name) : undefined,
                values: asArray(s?.values ?? s?.data ?? s?.y).map(asNumber),
                tone: asTone(s?.tone ?? s?.color),
            })).filter((s) => s.values.length > 0);
            // Salvage: flat values array at top level
            if (!series.length && Array.isArray(raw.values)) {
                series = [{ values: asArray(raw.values).map(asNumber) }];
            }
            // Salvage: data as [{label, value}]
            if (!series.length && Array.isArray(raw.data) && raw.data.length && typeof raw.data[0] === 'object') {
                const pts = asArray(raw.data);
                if (!labels.length) labels.push(...pts.map((p) => asString(p?.label ?? p?.name, '')));
                series = [{ values: pts.map((p) => asNumber(p?.value ?? p?.count ?? p?.y)) }];
            }
            if (!series.length) return null;
            if (!labels.length) series[0].values.forEach((_, i) => labels.push(String(i + 1)));
            return { type: 'chart', chart, title: raw.title ? asString(raw.title) : undefined, labels, series };
        }
        case 'markdown': {
            const content = asString(raw.content ?? raw.text ?? raw.markdown ?? raw.body);
            return content ? { type: 'markdown', content } : null;
        }
        case 'callout': {
            const content = asString(raw.content ?? raw.text ?? raw.message ?? raw.body);
            if (!content) return null;
            return {
                type: 'callout',
                tone: asTone(raw.tone ?? raw.color ?? raw.severity ?? raw.level) ?? 'info',
                title: raw.title ? asString(raw.title) : undefined,
                content,
            };
        }
        case 'progress': {
            const items = asArray(raw.items ?? raw.bars).map((it) => ({
                label: asString(it?.label ?? it?.name, ''),
                value: asNumber(it?.value ?? it?.percent ?? it?.progress),
                max: it?.max != null ? asNumber(it.max) : undefined,
                tone: asTone(it?.tone ?? it?.color),
            })).filter((it) => it.label);
            return items.length ? { type: 'progress', title: raw.title ? asString(raw.title) : undefined, items } : null;
        }
        case 'todo': {
            const items = asArray(raw.items ?? raw.todos ?? raw.tasks).map((it) => {
                if (typeof it === 'string') return { text: it, done: false };
                return {
                    text: asString(it?.text ?? it?.title ?? it?.label ?? it?.task, ''),
                    done: Boolean(it?.done ?? it?.completed ?? it?.checked),
                };
            }).filter((it) => it.text);
            return items.length ? { type: 'todo', title: raw.title ? asString(raw.title) : undefined, items } : null;
        }
        case 'kv': {
            let pairs: { key: string; value: string }[] = [];
            const rawPairs = raw.pairs ?? raw.items ?? raw.entries;
            if (Array.isArray(rawPairs)) {
                pairs = rawPairs.map((p) => ({
                    key: asString(p?.key ?? p?.label ?? p?.name, ''),
                    value: asString(p?.value ?? p?.val, ''),
                })).filter((p) => p.key);
            } else if (rawPairs && typeof rawPairs === 'object') {
                pairs = Object.entries(rawPairs).map(([key, value]) => ({ key, value: asString(value, '') }));
            }
            return pairs.length ? { type: 'kv', title: raw.title ? asString(raw.title) : undefined, pairs } : null;
        }
        case 'code': {
            const content = asString(raw.content ?? raw.code ?? raw.text);
            if (!content) return null;
            return {
                type: 'code',
                title: raw.title ? asString(raw.title) : undefined,
                language: raw.language ? asString(raw.language) : undefined,
                content,
            };
        }
        case 'timeline': {
            const items = asArray(raw.items ?? raw.steps ?? raw.events).map((it) => {
                if (typeof it === 'string') return { title: it } as CanvasTimelineItem;
                const statusRaw = asString(it?.status, '').toLowerCase();
                const status = statusRaw.includes('done') || statusRaw.includes('complete') ? 'done'
                    : statusRaw.includes('active') || statusRaw.includes('progress') || statusRaw.includes('running') ? 'active'
                        : statusRaw ? 'pending' : undefined;
                return {
                    title: asString(it?.title ?? it?.name ?? it?.text, ''),
                    detail: it?.detail != null ? asString(it.detail) : (it?.description != null ? asString(it.description) : undefined),
                    status,
                } as CanvasTimelineItem;
            }).filter((it) => it.title);
            return items.length ? { type: 'timeline', title: raw.title ? asString(raw.title) : undefined, items } : null;
        }
        default:
            // Unknown block with usable text — salvage as markdown
            {
                const content = asString(raw.content ?? raw.text);
                return content ? { type: 'markdown', content } : null;
            }
    }
}

/**
 * Normalize an untrusted/sloppy spec object (or JSON string) into a valid
 * CanvasSpec. Returns null only when nothing at all can be salvaged.
 */
export function normalizeCanvasSpec(raw: unknown): CanvasSpec | null {
    let obj: any = raw;
    if (typeof raw === 'string') {
        try {
            obj = JSON.parse(raw);
        } catch {
            // Small models sometimes wrap JSON in markdown fences
            const m = raw.match(/```(?:json)?\s*([\s\S]*?)```/);
            if (m) {
                try { obj = JSON.parse(m[1]); } catch { return null; }
            } else {
                return null;
            }
        }
    }
    if (!obj || typeof obj !== 'object') return null;
    // Some models nest the spec under a key
    if (!obj.blocks && obj.canvas && typeof obj.canvas === 'object') obj = obj.canvas;
    if (!obj.blocks && obj.spec && typeof obj.spec === 'object') obj = obj.spec;

    const title = asString(obj.title ?? obj.name, 'Untitled Canvas');
    const blocks = asArray(obj.blocks ?? obj.sections ?? obj.components)
        .map(normalizeBlock)
        .filter((b): b is CanvasBlock => b !== null);

    // A markdown-only fallback when no blocks parsed but content exists
    if (!blocks.length) {
        const content = asString(obj.content ?? obj.markdown ?? obj.text);
        if (content) blocks.push({ type: 'markdown', content });
    }
    if (!blocks.length) return null;

    return {
        id: asString(obj.id) || canvasSlug(title),
        title,
        subtitle: obj.subtitle ? asString(obj.subtitle) : undefined,
        updatedAt: Date.now(),
        blocks,
    };
}
