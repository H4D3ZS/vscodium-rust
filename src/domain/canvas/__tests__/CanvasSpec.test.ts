import { describe, it, expect } from 'vitest';
import { normalizeCanvasSpec, canvasSlug } from '../CanvasSpec';

describe('canvasSlug', () => {
    it('slugifies titles and clamps length', () => {
        expect(canvasSlug('Scan Findings!!')).toBe('scan-findings');
        expect(canvasSlug('')).toBe('canvas');
    });
});

describe('normalizeCanvasSpec — happy path', () => {
    it('keeps a well-formed spec', () => {
        const spec = normalizeCanvasSpec({
            title: 'Findings',
            blocks: [
                { type: 'stats', items: [{ label: 'Critical', value: 3, tone: 'danger' }] },
                { type: 'table', columns: ['Sev', 'Issue'], rows: [['High', 'XSS']] },
            ],
        });
        expect(spec).not.toBeNull();
        expect(spec!.title).toBe('Findings');
        expect(spec!.blocks).toHaveLength(2);
        expect(spec!.id).toBe('findings');
    });
});

describe('normalizeCanvasSpec — small-model tolerance', () => {
    it('parses a JSON string', () => {
        const spec = normalizeCanvasSpec('{"title":"X","blocks":[{"type":"markdown","content":"hi"}]}');
        expect(spec?.blocks[0]).toEqual({ type: 'markdown', content: 'hi' });
    });

    it('unwraps markdown-fenced JSON', () => {
        const spec = normalizeCanvasSpec('```json\n{"title":"X","blocks":[{"type":"text","text":"yo"}]}\n```');
        expect(spec?.blocks[0]).toEqual({ type: 'markdown', content: 'yo' });
    });

    it('maps block-type aliases (metrics→stats, graph→chart, checklist→todo)', () => {
        const spec = normalizeCanvasSpec({
            title: 'Aliased',
            blocks: [
                { type: 'metrics', items: [{ name: 'Hosts', value: 12 }] },
                { type: 'graph', kind: 'bar', labels: ['a', 'b'], series: [{ data: [1, 2] }] },
                { type: 'checklist', items: ['do thing', { text: 'done thing', completed: true }] },
            ],
        });
        expect(spec?.blocks.map((b) => b.type)).toEqual(['stats', 'chart', 'todo']);
        expect((spec?.blocks[2] as any).items[1]).toEqual({ text: 'done thing', done: true });
    });

    it('coerces color aliases to tones', () => {
        const spec = normalizeCanvasSpec({
            title: 'T',
            blocks: [{ type: 'stats', items: [{ label: 'X', value: 1, color: 'red' }] }],
        });
        expect((spec?.blocks[0] as any).items[0].tone).toBe('danger');
    });

    it('salvages a chart from data:[{label,value}]', () => {
        const spec = normalizeCanvasSpec({
            title: 'C',
            blocks: [{ type: 'chart', chart: 'pie', data: [{ label: 'A', value: 3 }, { label: 'B', value: 7 }] }],
        });
        const block = spec?.blocks[0] as any;
        expect(block.type).toBe('chart');
        expect(block.labels).toEqual(['A', 'B']);
        expect(block.series[0].values).toEqual([3, 7]);
    });

    it('projects object-rows onto declared columns', () => {
        const spec = normalizeCanvasSpec({
            title: 'T',
            blocks: [{ type: 'table', columns: ['Name', 'Port'], rows: [{ Name: 'ssh', Port: 22 }] }],
        });
        expect((spec?.blocks[0] as any).rows[0]).toEqual(['ssh', 22]);
    });

    it('converts kv object maps to pairs', () => {
        const spec = normalizeCanvasSpec({
            title: 'T',
            blocks: [{ type: 'kv', pairs: { Target: 'example.com', Scope: 'web' } }],
        });
        expect((spec?.blocks[0] as any).pairs).toEqual([
            { key: 'Target', value: 'example.com' },
            { key: 'Scope', value: 'web' },
        ]);
    });

    it('unwraps a spec nested under canvas/spec keys', () => {
        const spec = normalizeCanvasSpec({ canvas: { title: 'Nested', blocks: [{ type: 'markdown', content: 'x' }] } });
        expect(spec?.title).toBe('Nested');
    });

    it('falls back to a markdown block when content exists but no blocks parse', () => {
        const spec = normalizeCanvasSpec({ title: 'T', content: 'just some prose' });
        expect(spec?.blocks).toEqual([{ type: 'markdown', content: 'just some prose' }]);
    });

    it('drops empty/invalid blocks but keeps the valid ones', () => {
        const spec = normalizeCanvasSpec({
            title: 'T',
            blocks: [
                { type: 'stats', items: [] },
                { type: 'markdown', content: 'kept' },
                null,
                { type: 'table', columns: [], rows: [] },
            ],
        });
        expect(spec?.blocks).toEqual([{ type: 'markdown', content: 'kept' }]);
    });

    it('returns null only when nothing is salvageable', () => {
        expect(normalizeCanvasSpec('not json at all')).toBeNull();
        expect(normalizeCanvasSpec({ title: 'Empty', blocks: [] })).toBeNull();
        expect(normalizeCanvasSpec(42)).toBeNull();
    });
});
