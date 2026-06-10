import { describe, it, expect } from 'vitest';
import {
    classifyToolKind,
    createToolBlock,
    enrichCanvasBlockFromResult,
    isQuietReconBlock,
} from '../agentToolBlocks';

describe('canvas tool blocks', () => {
    it('classifies create_canvas as canvas kind', () => {
        expect(classifyToolKind('create_canvas')).toBe('canvas');
    });

    it('creates a canvas block carrying id/title from args', () => {
        const b = createToolBlock('create_canvas', { id: 'scan-1', title: 'Scan Findings', blocks: [] });
        expect(b.kind).toBe('canvas');
        expect(b.canvasId).toBe('scan-1');
        expect(b.canvasTitle).toBe('Scan Findings');
        expect(b.title).toBe('Canvas · Scan Findings');
    });

    it('canvas blocks never collapse into recon summaries', () => {
        const b = createToolBlock('create_canvas', { title: 'X', blocks: [] });
        b.status = 'done';
        expect(isQuietReconBlock(b)).toBe(false);
    });

    it('enriches canvas id from a wrapped tool result', () => {
        const b = createToolBlock('create_canvas', { title: 'No id given', blocks: [] });
        expect(b.canvasId).toBeUndefined();
        const enriched = enrichCanvasBlockFromResult(
            b,
            JSON.stringify({ success: true, data: { status: 'rendered', canvas_id: 'no-id-given' } }),
        );
        expect(enriched.canvasId).toBe('no-id-given');
    });

    it('enriches canvas id from a bare result object', () => {
        const b = createToolBlock('create_canvas', { title: 'Bare', blocks: [] });
        const enriched = enrichCanvasBlockFromResult(b, JSON.stringify({ canvas_id: 'bare' }));
        expect(enriched.canvasId).toBe('bare');
    });

    it('leaves non-canvas blocks and bad json untouched', () => {
        const term = createToolBlock('run_command', { command: 'ls' });
        expect(enrichCanvasBlockFromResult(term, '{"canvas_id":"x"}')).toBe(term);
        const b = createToolBlock('create_canvas', { title: 'X', blocks: [] });
        expect(enrichCanvasBlockFromResult(b, 'not json').canvasId).toBeUndefined();
    });
});
