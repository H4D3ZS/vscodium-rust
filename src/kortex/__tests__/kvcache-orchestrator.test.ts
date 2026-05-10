/**
 * Unit tests for the KV cache frontend orchestrator.
 *
 * Only covers pure functions — anything that calls Tauri's `invoke` is
 * skipped (those are integration-tested by the Rust side via cargo test
 * and by manual end-to-end runs).
 */

import { describe, it, expect } from 'vitest';
import {
    DEFAULT_KV_CACHE_OPTS,
    makeKvCacheOptions,
    summarizeKvCache,
    type KvCacheStats,
} from '../kvcache-orchestrator';

describe('makeKvCacheOptions', () => {
    it('expands base into {base}/index and {base}/slots paths', () => {
        const opts = makeKvCacheOptions('/tmp/kortex');
        expect(opts.index_dir).toBe('/tmp/kortex/index');
        expect(opts.slot_dir).toBe('/tmp/kortex/slots');
    });

    it('inherits all DEFAULT_KV_CACHE_OPTS fields', () => {
        const opts = makeKvCacheOptions('/tmp/x');
        expect(opts.max_bytes).toBe(DEFAULT_KV_CACHE_OPTS.max_bytes);
        expect(opts.min_tokens).toBe(DEFAULT_KV_CACHE_OPTS.min_tokens);
        expect(opts.cold_max_tokens).toBe(DEFAULT_KV_CACHE_OPTS.cold_max_tokens);
        expect(opts.boundary_align_tokens).toBe(DEFAULT_KV_CACHE_OPTS.boundary_align_tokens);
        expect(opts.proxy_port).toBe(DEFAULT_KV_CACHE_OPTS.proxy_port);
        expect(opts.upstream_url).toBe(DEFAULT_KV_CACHE_OPTS.upstream_url);
    });

    it('applies overrides on top of defaults', () => {
        const opts = makeKvCacheOptions('/tmp/x', {
            max_bytes: 999,
            proxy_port: 9999,
            upstream_url: 'http://elsewhere:9000',
        });
        expect(opts.max_bytes).toBe(999);
        expect(opts.proxy_port).toBe(9999);
        expect(opts.upstream_url).toBe('http://elsewhere:9000');
        // Untouched overrides stay at defaults
        expect(opts.proxy_host).toBe(DEFAULT_KV_CACHE_OPTS.proxy_host);
    });

    it('lets a caller override the index_dir / slot_dir explicitly', () => {
        const opts = makeKvCacheOptions('/tmp/x', {
            index_dir: '/elsewhere/idx',
            slot_dir: '/elsewhere/slots',
        });
        expect(opts.index_dir).toBe('/elsewhere/idx');
        expect(opts.slot_dir).toBe('/elsewhere/slots');
    });
});

describe('summarizeKvCache', () => {
    const empty: KvCacheStats = {
        entries: 0,
        total_bytes: 0,
        hits: 0,
        misses: 0,
        saves: 0,
        evictions: 0,
        tokens_skipped: 0,
    };

    it('reports 0% hit rate when no requests have happened yet', () => {
        const s = summarizeKvCache(empty);
        expect(s).toContain('0 entries');
        expect(s).toContain('0% hit rate');
    });

    it('computes hit rate from hits + misses', () => {
        const s = summarizeKvCache({ ...empty, hits: 80, misses: 20 });
        expect(s).toContain('80% hit rate');
    });

    it('formats GB to two decimals', () => {
        const s = summarizeKvCache({ ...empty, total_bytes: 3.5 * 1024 ** 3 });
        expect(s).toContain('3.50 GB');
    });

    it('shortens large token-skipped counts to k-units', () => {
        const s = summarizeKvCache({ ...empty, tokens_skipped: 142_315 });
        expect(s).toContain('142.3k tokens skipped');
    });

    it('keeps small token-skipped counts as plain integers', () => {
        const s = summarizeKvCache({ ...empty, tokens_skipped: 42 });
        expect(s).toContain('42 tokens skipped');
        expect(s).not.toContain('k tokens');
    });
});
