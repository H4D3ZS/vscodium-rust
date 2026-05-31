/**
 * Unit tests for the Kortex throughput tracker.
 *
 * Covers:
 *  - computeTps math (incl. divide-by-zero guard).
 *  - approximateTokensFromChars rule-of-thumb.
 *  - Rolling window keeps only the most recent N samples.
 *  - summarizeThroughput's "current" tps tracks the latest sample.
 *  - avg_tps is an unweighted mean over the window.
 *  - avg_prefill_tps is NaN-safe when no sample carries prefill_ms.
 *  - cache_hit_rate is the fraction of cache_hit=true samples.
 *  - total_tokens_skipped sums across the window.
 *  - formatTps renders --, "X.Y tok/s", "NNN tok/s" tiers correctly.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import {
    recordCompletion,
    summarizeThroughput,
    computeTps,
    approximateTokensFromChars,
    formatTps,
    __resetThroughputHistoryForTesting,
} from '../throughput';

beforeEach(() => {
    __resetThroughputHistoryForTesting();
});

describe('computeTps', () => {
    it('returns tokens-per-second from a positive duration', () => {
        // 50 tokens in 500 ms ⇒ 100 tok/s.
        expect(computeTps(50, 500)).toBeCloseTo(100, 5);
    });

    it('clamps the denominator so zero-duration completions do not divide by zero', () => {
        // Treated as 1 ms ⇒ 1000 * tokens.
        expect(computeTps(5, 0)).toBe(5000);
    });

    it('handles negative durations the same as zero (clamp)', () => {
        expect(computeTps(10, -50)).toBe(10000);
    });
});

describe('approximateTokensFromChars', () => {
    it('returns 0 for non-positive input', () => {
        expect(approximateTokensFromChars(0)).toBe(0);
        expect(approximateTokensFromChars(-10)).toBe(0);
    });

    it('rounds chars / 4 to the nearest token, with a floor of 1', () => {
        expect(approximateTokensFromChars(1)).toBe(1);    // chars/4 = 0.25 → round = 0, floored to 1
        expect(approximateTokensFromChars(4)).toBe(1);
        expect(approximateTokensFromChars(10)).toBe(3);   // 2.5 → 3
        expect(approximateTokensFromChars(400)).toBe(100);
    });
});

describe('summarizeThroughput empty state', () => {
    it('returns zeroed summary with no samples', () => {
        const s = summarizeThroughput();
        expect(s.sample_size).toBe(0);
        expect(s.current_tps).toBe(0);
        expect(s.avg_tps).toBe(0);
        expect(s.cache_hit_rate).toBe(0);
        expect(s.total_tokens_skipped).toBe(0);
        expect(Number.isNaN(s.avg_prefill_tps)).toBe(true);
        expect(s.last).toBeUndefined();
    });
});

describe('summarizeThroughput rolling window', () => {
    it('tracks current_tps as the most recent sample', () => {
        recordCompletion({
            wall_clock_ms: 1000, output_tokens: 10, input_tokens: 20,
            backend: 'ollama', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 1,
        });
        recordCompletion({
            wall_clock_ms: 1000, output_tokens: 50, input_tokens: 20,
            backend: 'ollama', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 2,
        });
        const s = summarizeThroughput();
        // Last sample was 50 tokens / 1000 ms = 50 tok/s.
        expect(s.current_tps).toBeCloseTo(50, 5);
        expect(s.sample_size).toBe(2);
        // Average is unweighted mean of (10, 50) = 30 tok/s.
        expect(s.avg_tps).toBeCloseTo(30, 5);
    });

    it('caps history at 16 samples (FIFO)', () => {
        for (let i = 0; i < 25; i++) {
            recordCompletion({
                wall_clock_ms: 1000, output_tokens: i + 1, input_tokens: 0,
                backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: i,
            });
        }
        const s = summarizeThroughput();
        expect(s.sample_size).toBe(16);
        // Window now contains samples 10..25 (indices 9..24, tokens 10..25). Most recent
        // sample is i=24 ⇒ 25 tokens in 1000 ms ⇒ 25 tok/s.
        expect(s.current_tps).toBeCloseTo(25, 5);
    });

    it('drops samples with non-positive wall_clock_ms', () => {
        recordCompletion({
            wall_clock_ms: 0, output_tokens: 999, input_tokens: 0,
            backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 1,
        });
        recordCompletion({
            wall_clock_ms: -50, output_tokens: 999, input_tokens: 0,
            backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 2,
        });
        const s = summarizeThroughput();
        expect(s.sample_size).toBe(0);
    });
});

describe('summarizeThroughput cache + prefill aggregation', () => {
    it('computes hit rate as hits / total samples', () => {
        const hit = { wall_clock_ms: 100, output_tokens: 5, input_tokens: 0, backend: 'llama.cpp', cache_hit: true, tokens_skipped: 200, ts_unix_ms: 1 };
        const miss = { wall_clock_ms: 100, output_tokens: 5, input_tokens: 0, backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 2 };
        recordCompletion(hit);
        recordCompletion(miss);
        recordCompletion(hit);
        recordCompletion(miss);
        const s = summarizeThroughput();
        expect(s.cache_hit_rate).toBeCloseTo(0.5, 5);
        // Two hits × 200 skipped each = 400.
        expect(s.total_tokens_skipped).toBe(400);
    });

    it('returns NaN avg_prefill_tps when no sample has prefill_ms', () => {
        recordCompletion({
            wall_clock_ms: 1000, output_tokens: 10, input_tokens: 100,
            backend: 'ollama', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 1,
        });
        const s = summarizeThroughput();
        expect(Number.isNaN(s.avg_prefill_tps)).toBe(true);
    });

    it('computes avg_prefill_tps as average of input_tokens / prefill_seconds', () => {
        // Sample A: 100 tokens in 100 ms prefill → 1000 tok/s.
        // Sample B: 200 tokens in 1000 ms prefill → 200 tok/s.
        // Mean → 600 tok/s.
        recordCompletion({
            wall_clock_ms: 1000, prefill_ms: 100,
            output_tokens: 5, input_tokens: 100,
            backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 1,
        });
        recordCompletion({
            wall_clock_ms: 1000, prefill_ms: 1000,
            output_tokens: 5, input_tokens: 200,
            backend: 'llama.cpp', cache_hit: false, tokens_skipped: 0, ts_unix_ms: 2,
        });
        const s = summarizeThroughput();
        expect(s.avg_prefill_tps).toBeCloseTo(600, 1);
    });

    it('exposes the last sample on summary.last', () => {
        recordCompletion({
            wall_clock_ms: 500, output_tokens: 5, input_tokens: 0,
            backend: 'ollama', cache_hit: true, tokens_skipped: 123, ts_unix_ms: 99,
            model_id: 'qwen3-coder.gguf',
        });
        const s = summarizeThroughput();
        expect(s.last).toBeDefined();
        expect(s.last!.model_id).toBe('qwen3-coder.gguf');
        expect(s.last!.cache_hit).toBe(true);
        expect(s.last!.tokens_skipped).toBe(123);
    });
});

describe('formatTps', () => {
    it('renders -- for zero, NaN, or negative', () => {
        expect(formatTps(0)).toBe('--');
        expect(formatTps(Number.NaN)).toBe('--');
        expect(formatTps(-5)).toBe('--');
    });

    it('renders one decimal place under 100 tok/s', () => {
        expect(formatTps(12.3456)).toBe('12.3 tok/s');
        expect(formatTps(0.5)).toBe('0.5 tok/s');
    });

    it('renders rounded integer at or above 100 tok/s', () => {
        expect(formatTps(99.6)).toBe('99.6 tok/s');
        expect(formatTps(100)).toBe('100 tok/s');
        expect(formatTps(523.7)).toBe('524 tok/s');
    });
});
