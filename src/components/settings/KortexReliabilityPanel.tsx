import React, { useEffect, useRef, useState } from 'react';
import { invoke } from '../../tauri_bridge';

/** Mirrors `reliability_stats::LeverStatus` (Rust). */
interface LeverStatus {
    name: string;
    enabled: boolean;
    counters: Record<string, number>;
}

interface StatusResponse {
    levers: LeverStatus[];
}

/** Display label + one-line counter summary per lever, in a fixed, sensible order. */
const LEVER_META: Record<string, { label: string; summarize: (c: Record<string, number>) => string }> = {
    semantic_cache: {
        label: 'Semantic cache',
        summarize: (c) => {
            const hits = c.SEMCACHE_HITS ?? 0;
            const total = hits + (c.SEMCACHE_MISSES ?? 0);
            return total === 0 ? 'no lookups yet' : `${hits}/${total} hits (${Math.round((hits / total) * 100)}%) — zero-inference`;
        },
    },
    cascade: {
        label: 'Cascade router',
        summarize: (c) => {
            const accepted = c.CASCADE_ACCEPTED_OPERATOR ?? 0;
            const escalated = c.CASCADE_ESCALATED ?? 0;
            const total = accepted + escalated;
            return total === 0 ? 'no turns routed yet' : `${accepted} on Operator, ${escalated} escalated`;
        },
    },
    grounding: {
        label: 'Reference grounding',
        summarize: (c) => {
            const checked = c.GROUNDING_CLAIMS_CHECKED ?? 0;
            const flagged = c.GROUNDING_UNGROUNDED_FLAGGED ?? 0;
            return checked === 0 ? 'no answers checked yet' : `${flagged} fabricated ref(s) caught / ${checked} checked`;
        },
    },
    abstain: {
        label: 'Calibrated abstention',
        summarize: (c) => {
            const q = c.ABSTAIN_QUALIFIED ?? 0;
            const w = c.ABSTAIN_WITHHELD ?? 0;
            return q + w === 0 ? 'always confident so far' : `${w} withheld, ${q} qualified`;
        },
    },
    authorization: {
        label: 'Tool authorization',
        summarize: (c) => {
            const a = c.AUTHZ_ALLOWED ?? 0, cf = c.AUTHZ_CONFIRMED ?? 0, d = c.AUTHZ_DENIED ?? 0;
            return a + cf + d === 0 ? 'no calls gated yet' : `${a} allowed, ${cf} confirmed, ${d} denied`;
        },
    },
    provenance: {
        label: 'Content provenance',
        summarize: (c) => {
            const f = c.PROVENANCE_FENCED ?? 0, i = c.PROVENANCE_INJECTION_FLAGGED ?? 0;
            return f === 0 ? 'no external content fenced yet' : `${f} fenced, ${i} injection-flagged`;
        },
    },
    verify: {
        label: 'Programmatic verification',
        summarize: (c) => {
            const r = c.VERIFY_RUNS ?? 0, p = c.VERIFY_PASSED ?? 0;
            return r === 0 ? 'not run yet' : `${p}/${r} passed`;
        },
    },
    harness: {
        label: 'Context compression',
        summarize: (c) => {
            const s = c.HARNESS_SCHEMA_COMPACTED ?? 0, t = c.HARNESS_TOOL_OUTPUT_COMPACTED ?? 0, v = c.HARNESS_STEER_APPLIED ?? 0;
            return s + t + v === 0 ? 'nothing compacted yet' : `${s} schema, ${t} tool-output, ${v} steered`;
        },
    },
    tgrep: {
        label: 'Search engine',
        summarize: (c) => {
            const srv = c.TGREP_SERVER_PROBE_HITS ?? 0, ip = c.TGREP_IN_PROCESS_HITS ?? 0;
            return srv + ip === 0 ? 'ripgrep only so far' : `${srv} via server, ${ip} in-process`;
        },
    },
};

const LEVER_ORDER = [
    'cascade', 'semantic_cache', 'grounding', 'abstain',
    'verify', 'authorization', 'provenance', 'harness', 'tgrep',
];

const row: React.CSSProperties = { display: 'flex', alignItems: 'center', gap: 8, padding: '3px 0' };
const dot = (on: boolean): React.CSSProperties => ({
    width: 7, height: 7, borderRadius: '50%', flex: 'none',
    background: on ? 'var(--vscode-charts-green, #3fb950)' : '#555',
});
const label: React.CSSProperties = { fontSize: 11, flex: '0 0 150px', opacity: 0.85 };
const summary: React.CSSProperties = {
    fontSize: 10.5, opacity: 0.6, flex: 1, minWidth: 0,
    fontFamily: 'var(--vscode-editor-font-family, monospace)',
    whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis',
};

/**
 * Live status for every kortex compute-cost / AI-reliability lever — the
 * cascade router, semantic cache, grounding, abstention, verification,
 * authorization, provenance, context compression, and the search engine.
 * Each is code-level tested and wired (see docs/kortex-compute-cost.md); this
 * is the one place a user can actually SEE whether any of it fired, matching
 * the "measured, not assumed" readout the spec-decode acceptance line already
 * set the precedent for.
 *
 * Polls `kortex_reliability_status` while expanded. A poll failure (command
 * unavailable, e.g. non-Tauri dev preview) leaves the last good status up
 * rather than clearing it — a transient hiccup shouldn't blank the panel.
 */
export function KortexReliabilityPanel(): React.ReactElement {
    const [expanded, setExpanded] = useState(false);
    const [levers, setLevers] = useState<LeverStatus[] | null>(null);
    const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

    useEffect(() => {
        if (!expanded) {
            if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; }
            return;
        }
        let cancelled = false;
        const tick = async () => {
            try {
                const res = await invoke<StatusResponse>('kortex_reliability_status');
                if (!cancelled && res?.levers) setLevers(res.levers);
            } catch { /* leave last-known status up */ }
        };
        void tick();
        pollRef.current = setInterval(tick, 5000);
        return () => {
            cancelled = true;
            if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; }
        };
    }, [expanded]);

    const byName = new Map((levers ?? []).map((l) => [l.name, l]));
    const anyOn = (levers ?? []).some((l) => l.enabled);

    return (
        <div style={{ marginTop: 10 }}>
            <button
                type="button"
                onClick={() => setExpanded((v) => !v)}
                style={{
                    display: 'flex', alignItems: 'center', gap: 6, width: '100%',
                    background: 'transparent', border: 'none', cursor: 'pointer', padding: '4px 0',
                    color: 'var(--vscode-foreground)', fontSize: 11, fontWeight: 600,
                }}
                title="What's actually firing: cascade, cache, grounding, abstain, verify, authorization, provenance, compression, search"
            >
                <span style={{ opacity: 0.6 }}>{expanded ? '▾' : '▸'}</span>
                <span>Reliability &amp; compute levers</span>
                {levers && (
                    <span style={{ fontSize: 10, opacity: 0.5, fontWeight: 400 }}>
                        ({levers.filter((l) => l.enabled).length}/{levers.length} active)
                    </span>
                )}
            </button>

            {expanded && (
                <div style={{ marginLeft: 14, marginTop: 4 }}>
                    {!levers && (
                        <div style={{ fontSize: 10.5, opacity: 0.5 }}>loading…</div>
                    )}
                    {levers && !anyOn && (
                        <div style={{ fontSize: 10.5, opacity: 0.5, marginBottom: 4 }}>
                            All off. Most default on for local models — see docs/kortex-compute-cost.md.
                        </div>
                    )}
                    {LEVER_ORDER.map((name) => {
                        const l = byName.get(name);
                        const meta = LEVER_META[name];
                        if (!l || !meta) return null;
                        return (
                            <div key={name} style={row}>
                                <span style={dot(l.enabled)} title={l.enabled ? 'on' : 'off'} />
                                <span style={label}>{meta.label}</span>
                                <span style={summary}>{meta.summarize(l.counters)}</span>
                            </div>
                        );
                    })}
                </div>
            )}
        </div>
    );
}
