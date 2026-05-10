/**
 * Unit tests for the CCET token router and η tracker.
 *
 * Covers:
 *  - splitSegments via routePrompt (code fences stay atomic, paragraphs split on \n\n)
 *  - structural / repetition heuristics produce sensible Route labels
 *  - max_skip_fraction cap prevents over-aggressive dropping
 *  - DEFAULT_POLICY shape
 *  - recordRequest / summarizeEfficiency aggregation
 *  - ccetWrap end-to-end with a fake runner
 *
 * The router is heuristic-only — we test invariants and not exact scores.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import {
    routePrompt,
    DEFAULT_POLICY,
    recordRequest,
    summarizeEfficiency,
    getRecentEfficiency,
    ccetWrap,
    __resetCcetHistoryForTesting,
} from '../ccet';

describe('routePrompt', () => {
    it('returns SKIP-only counts when given an empty prompt', () => {
        const r = routePrompt('');
        expect(r.segments).toHaveLength(0);
        expect(r.counts).toEqual({ full: 0, compress: 0, skip: 0 });
        expect(r.output_text).toBe('');
        expect(r.saved_fraction).toBe(1);
    });

    it('keeps short non-trivial prompts intact (no segments dropped)', () => {
        const r = routePrompt('Refactor the cache LRU into a separate module.');
        // Single segment, score may be FULL or COMPRESS depending on heuristic
        expect(r.segments).toHaveLength(1);
        // It must not be SKIP — short prompts shouldn't be dropped
        expect(r.segments[0].route).not.toBe('SKIP');
    });

    it('keeps code fences as a single atomic segment', () => {
        const text = [
            'Here is some code:',
            '',
            '```rust',
            'fn main() {',
            '    println!("hi");',
            '}',
            '```',
            '',
            'And a follow-up.',
        ].join('\n');
        const r = routePrompt(text);
        // The code-fence block should appear verbatim in exactly one segment.
        const fenceSeg = r.segments.find(s => s.text.startsWith('```rust'));
        expect(fenceSeg).toBeDefined();
        expect(fenceSeg!.text).toContain('println!');
    });

    it('rates code-heavy segments as FULL', () => {
        const code = [
            'fn add(a: i32, b: i32) -> i32 {',
            '    return a + b;',
            '}',
            'pub mod util;',
            'use std::sync::Arc;',
        ].join('\n');
        const r = routePrompt(code);
        // At least one segment should be FULL because of structural anchors.
        expect(r.segments.some(s => s.route === 'FULL')).toBe(true);
    });

    it('promotes SKIPs to COMPRESS to respect max_skip_fraction', () => {
        // Build a prompt where most segments are obvious low-info filler.
        // With max_skip_fraction = 0.0, NO segment is allowed to be SKIP, so
        // they must all be promoted to at least COMPRESS.
        const filler = Array.from({ length: 8 }, (_, i) => `noise ${i}`).join('\n\n');
        const r = routePrompt(filler, { max_skip_fraction: 0.0 });
        expect(r.counts.skip).toBe(0);
    });

    it('keeps the saved_fraction in [0, 1]', () => {
        const text = ('boilerplate '.repeat(100) + '\n\n').repeat(5);
        const r = routePrompt(text);
        expect(r.saved_fraction).toBeGreaterThanOrEqual(0);
        expect(r.saved_fraction).toBeLessThanOrEqual(1);
    });

    it('respects custom thresholds', () => {
        const text = 'plain text without code or paths';
        // With τ_compress = 0.0 everything that isn't a SKIP becomes FULL.
        const r = routePrompt(text, { tau_compress: 0.0, tau_skip: 0.0, max_skip_fraction: 1.0 });
        expect(r.segments.every(s => s.route === 'FULL')).toBe(true);
    });

    it('exposes counts that sum to the segment count', () => {
        const text = [
            'paragraph one with some words',
            '',
            'paragraph one with some words', // repeat to trigger repetition
            '',
            'fn x() {}',
        ].join('\n');
        const r = routePrompt(text);
        const totalCounted = r.counts.full + r.counts.compress + r.counts.skip;
        expect(totalCounted).toBe(r.segments.length);
    });
});

describe('DEFAULT_POLICY', () => {
    it('has the documented threshold values', () => {
        expect(DEFAULT_POLICY).toEqual({
            tau_skip: 0.05,
            tau_compress: 0.30,
            max_skip_fraction: 0.40,
            ngram_size: 5,
        });
    });
});

describe('η tracker', () => {
    beforeEach(() => __resetCcetHistoryForTesting());

    it('returns null when history is empty', () => {
        expect(summarizeEfficiency()).toBeNull();
        expect(getRecentEfficiency()).toEqual([]);
    });

    it('computes η = output / (active * wall_secs)', () => {
        const m = recordRequest({
            request_id: 'r1',
            model: 'test',
            input_chars: 1000,
            active_chars: 100,
            output_chars: 50,
            wall_clock_ms: 1000, // 1 second
            routing_counts: { full: 1, compress: 0, skip: 0 },
            saved_fraction: 0.9,
        });
        // η = 50 / (100 * 1) = 0.5
        expect(m.eta).toBeCloseTo(0.5, 6);
    });

    it('clamps wall_clock_ms below 1ms to avoid divide-by-zero', () => {
        const m = recordRequest({
            request_id: 'r2',
            model: 'test',
            input_chars: 0,
            active_chars: 0,
            output_chars: 10,
            wall_clock_ms: 0,
            routing_counts: { full: 0, compress: 0, skip: 0 },
            saved_fraction: 0,
        });
        // active * wall_secs = 0 → denom clamped to 1 → eta = 10
        expect(m.eta).toBe(10);
        expect(Number.isFinite(m.eta)).toBe(true);
    });

    it('aggregates across the recent window', () => {
        for (let i = 0; i < 5; i++) {
            recordRequest({
                request_id: `r${i}`,
                model: 'test',
                input_chars: 1000,
                active_chars: 100,
                output_chars: 50,
                wall_clock_ms: 1000,
                routing_counts: { full: 1, compress: 0, skip: 2 },
                saved_fraction: 0.5,
            });
        }
        const s = summarizeEfficiency(50);
        expect(s).not.toBeNull();
        expect(s!.sample_size).toBe(5);
        expect(s!.avg_eta).toBeCloseTo(0.5, 6);
        expect(s!.avg_saved_fraction).toBeCloseTo(0.5, 6);
        expect(s!.total_skipped_segments).toBe(10);
    });

    it('orders most-recent first in getRecentEfficiency', () => {
        recordRequest({
            request_id: 'first',
            model: 'test',
            input_chars: 0, active_chars: 1, output_chars: 1, wall_clock_ms: 1000,
            routing_counts: { full: 0, compress: 0, skip: 0 }, saved_fraction: 0,
        });
        recordRequest({
            request_id: 'second',
            model: 'test',
            input_chars: 0, active_chars: 1, output_chars: 1, wall_clock_ms: 1000,
            routing_counts: { full: 0, compress: 0, skip: 0 }, saved_fraction: 0,
        });
        const recent = getRecentEfficiency();
        expect(recent[0].request_id).toBe('second');
        expect(recent[1].request_id).toBe('first');
    });

    it('caps history at 200 entries', () => {
        for (let i = 0; i < 250; i++) {
            recordRequest({
                request_id: `r${i}`, model: 'test',
                input_chars: 0, active_chars: 1, output_chars: 1, wall_clock_ms: 1000,
                routing_counts: { full: 0, compress: 0, skip: 0 }, saved_fraction: 0,
            });
        }
        expect(getRecentEfficiency(1000)).toHaveLength(200);
    });
});

describe('ccetWrap', () => {
    beforeEach(() => __resetCcetHistoryForTesting());

    it('routes the prompt and records η', async () => {
        const prompt = 'fn foo() {}\n\nplain prose without code anchors\n\nfn bar() {}';
        const captured: string[] = [];
        const result = await ccetWrap('test-model', prompt, async (active) => {
            captured.push(active);
            return 'ok';
        });
        expect(captured).toHaveLength(1);
        // The runner sees the routed prompt, not the raw one.
        expect(captured[0]).toBe(result.route.output_text);
        expect(result.output).toBe('ok');
        expect(result.metric.input_chars).toBe(prompt.length);
        expect(result.metric.active_chars).toBe(result.route.output_text.length);
        expect(result.metric.output_chars).toBe(2);
    });

    it('persists metrics to the shared history', async () => {
        await ccetWrap('m', 'short prompt', async () => 'reply');
        const recent = getRecentEfficiency();
        expect(recent).toHaveLength(1);
        expect(recent[0].model).toBe('m');
    });
});
