/**
 * Pure-function unit tests for the GAC orchestrator.
 *
 * Anything that calls `invoke` (profile / plan / launch / stop) needs the
 * Tauri runtime and is integration-tested by cargo. We only cover summarizePlan
 * here since it's a pure formatting function and is what the user sees in the
 * settings panel and the launch script.
 */

import { describe, it, expect } from 'vitest';
import { summarizePlan, type TierPlan } from '../gac-orchestrator';

const samplePlan: TierPlan = {
    n_gpu_layers: 80,
    overrides: [
        { pattern: '\\.ffn_down_exps', buffer: 'Cpu' },
        { pattern: '\\.ffn_up_exps', buffer: 'Cpu' },
    ],
    total_gpu_bytes: 5_368_709_120, // 5.0 GiB
    total_cpu_bytes: 15_032_385_536, // 14.0 GiB
    vram_budget_mb: 8192,
    theta: 0.85,
    d_bar_critical: 0.151,
    routing_counts: {
        spread_to_gpu: 24,
        spread_to_cpu: 0,
        borderline_to_gpu: 12,
        borderline_to_cpu: 4,
        tight_to_gpu: 0,
        tight_to_cpu: 28,
    },
    backend: 'vulkan',
};

describe('summarizePlan', () => {
    it('formats counts, theta, d_bar_critical, and GB sizes', () => {
        const s = summarizePlan(samplePlan);
        expect(s).toContain('spread→GPU 24');
        expect(s).toContain('borderline→GPU 12');
        expect(s).toContain('tight→CPU 28');
        expect(s).toContain('θ=0.85');
        expect(s).toContain('d̄_crit=0.151');
        expect(s).toContain('GPU 5.00G');
        expect(s).toContain('CPU 14.00G');
    });

    it('returns a comma-separated, single-line string', () => {
        const s = summarizePlan(samplePlan);
        expect(s.split(',').length).toBeGreaterThanOrEqual(5);
        expect(s).not.toContain('\n');
    });

    it('rounds to two decimals for GB sizes', () => {
        const plan: TierPlan = {
            ...samplePlan,
            total_gpu_bytes: 7.123 * 1024 * 1024 * 1024,
            total_cpu_bytes: 0,
        };
        const s = summarizePlan(plan);
        expect(s).toContain('GPU 7.12G');
        expect(s).toContain('CPU 0.00G');
    });

    it('handles zero counts without crashing', () => {
        const plan: TierPlan = {
            ...samplePlan,
            routing_counts: {
                spread_to_gpu: 0,
                spread_to_cpu: 0,
                borderline_to_gpu: 0,
                borderline_to_cpu: 0,
                tight_to_gpu: 0,
                tight_to_cpu: 0,
            },
        };
        const s = summarizePlan(plan);
        expect(s).toContain('spread→GPU 0');
        expect(s).toContain('tight→CPU 0');
    });
});
