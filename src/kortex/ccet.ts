/**
 * Context-Compute Efficiency Theorem (CCET) — heuristic token router + η tracker.
 *
 * The user-supplied CCET framework establishes that, for any sequence model:
 *
 *     min_S M(S(x_1:T))  ≤  α · sqrt(P · I(x_1:T; y))  +  O(log T)
 *
 * with the corollary that a token x_t contributes useful computation iff
 *
 *     ΔI_t = I(y; x_t | x_1:t-1) > τ.
 *
 * A *trained* router would estimate ΔI directly. This module ships a v1
 * **heuristic** router that uses three text-level proxies:
 *
 *   1. n-gram repetition penalty (already-seen content has near-zero ΔI),
 *   2. structural anchor detection (code, identifiers, paths, errors → high ΔI),
 *   3. length × novelty (longer novel paragraphs carry more information).
 *
 * Output is one of three labels per segment:
 *
 *   - FULL     ΔI > τ_compress  → forward verbatim.
 *   - COMPRESS τ_skip < ΔI      → replaced with a stub "[…N chars compressed]".
 *   - SKIP     ΔI ≤ τ_skip      → dropped entirely.
 *
 * A `max_skip_fraction` cap prevents the router from being too aggressive on
 * low-info-but-large prompts (e.g. boilerplate disclaimers).
 *
 * The η tracker computes Information Efficiency per request:
 *
 *     η = output_chars / (active_chars × wall_seconds)
 *
 * This is a deliberately blunt approximation. Hooking it up to logit entropy
 * once we have llama-server `logprobs` enabled is on the roadmap.
 */

export type Route = 'FULL' | 'COMPRESS' | 'SKIP';

export interface RoutedSegment {
    text: string;
    route: Route;
    /** Estimated ΔI in [0, 1], higher = more informative. */
    score: number;
    reason: string;
}

export interface RoutingPolicy {
    /** Below this score, segment is SKIP. Default 0.05. */
    tau_skip: number;
    /** Above this score, segment is FULL. Between τ_skip and τ_compress, COMPRESS. Default 0.30. */
    tau_compress: number;
    /** Cap on total fraction-by-length the router is allowed to drop. Default 0.40. */
    max_skip_fraction: number;
    /** n-gram size for repetition detection. Default 5. */
    ngram_size: number;
}

export interface RoutingResult {
    segments: RoutedSegment[];
    /** The reconstructed prompt with COMPRESS/SKIP applied. */
    output_text: string;
    counts: { full: number; compress: number; skip: number };
    /** Sum of scores for segments that survived (FULL+COMPRESS). */
    total_score: number;
    /** 1 - len(output) / len(input). */
    saved_fraction: number;
}

export const DEFAULT_POLICY: RoutingPolicy = {
    tau_skip: 0.05,
    tau_compress: 0.30,
    max_skip_fraction: 0.40,
    ngram_size: 5,
};

// ─── routing primitives ─────────────────────────────────────────────────────

const CODE_LINE_PATTERN =
    /^\s*(import|export|fn |def |class |const |let |var |if |else |for |while |return |\}|\{|<\/?\w+|@\w+|use |pub |mod |trait |impl )/;
const PATH_PATTERN = /[\\/][\w.\-]+\.\w{1,8}(:\d+)?/;
const ERROR_PATTERN = /\b(Error|Exception|Traceback|panic|undefined|null reference|stack ?trace|TypeError|RangeError|SyntaxError)\b/i;
const CAMEL_OR_SNAKE = /\b([a-z][a-zA-Z0-9]*[A-Z][a-zA-Z0-9]+|[A-Z][A-Z0-9_]{2,})\b/;

/** Score a segment by structural information density. */
function structuralScore(text: string): number {
    const lines = text.split('\n');
    let codey = 0;
    for (const line of lines) {
        if (CODE_LINE_PATTERN.test(line)) codey++;
    }
    if (codey >= 3) return 1.0;
    if (PATH_PATTERN.test(text)) return 0.85;
    if (ERROR_PATTERN.test(text)) return 0.75;
    if (CAMEL_OR_SNAKE.test(text)) return 0.55;
    return 0.0;
}

/** Fraction of n-grams already seen in `seen`. Mutates `seen`. */
function repetitionPenalty(text: string, seen: Set<string>, n: number): number {
    const tokens = text.split(/\s+/).filter(Boolean);
    if (tokens.length < n) return 0;
    let dup = 0;
    let total = 0;
    for (let i = 0; i + n <= tokens.length; i++) {
        const ngram = tokens.slice(i, i + n).join(' ').toLowerCase();
        if (seen.has(ngram)) dup++;
        seen.add(ngram);
        total++;
    }
    return total > 0 ? dup / total : 0;
}

/** Combined ΔI proxy in [0, 1]. */
function infoScore(text: string, seen: Set<string>, n: number): number {
    const struct = structuralScore(text);
    const rep = repetitionPenalty(text, seen, n);
    const lengthBonus = Math.min(text.length / 200, 1.0);
    return Math.max(struct, lengthBonus * (1 - rep));
}

/** Split text into reasoning-unit segments. Code fences stay atomic. */
function splitSegments(text: string): string[] {
    const segs: string[] = [];
    let i = 0;
    while (i < text.length) {
        if (text.startsWith('```', i)) {
            const close = text.indexOf('```', i + 3);
            if (close === -1) {
                segs.push(text.slice(i));
                break;
            }
            segs.push(text.slice(i, close + 3));
            i = close + 3;
            continue;
        }
        const next = text.indexOf('\n\n', i);
        if (next === -1) {
            segs.push(text.slice(i));
            break;
        }
        segs.push(text.slice(i, next));
        i = next + 2;
    }
    return segs.filter(s => s.trim().length > 0);
}

/** Classify each segment of `text` and return the routed prompt. */
export function routePrompt(text: string, policy: Partial<RoutingPolicy> = {}): RoutingResult {
    const p: RoutingPolicy = { ...DEFAULT_POLICY, ...policy };
    const seen = new Set<string>();

    const segments = splitSegments(text).map((seg): RoutedSegment => {
        const score = infoScore(seg, seen, p.ngram_size);
        let route: Route;
        let reason: string;
        if (score >= p.tau_compress) {
            route = 'FULL';
            reason = 'structural / high info';
        } else if (score >= p.tau_skip) {
            route = 'COMPRESS';
            reason = 'medium info';
        } else {
            route = 'SKIP';
            reason = 'low info / repetitive';
        }
        return { text: seg, route, score, reason };
    });

    // Cap total skip fraction. Promote highest-score SKIPs to COMPRESS until
    // we're under the cap. This protects against pathological "drop the whole
    // system prompt" cases on low-novelty inputs.
    const totalLen = text.length || 1;
    let skipLen = segments
        .filter(s => s.route === 'SKIP')
        .reduce((a, s) => a + s.text.length, 0);
    if (skipLen / totalLen > p.max_skip_fraction) {
        const skips = segments
            .filter(s => s.route === 'SKIP')
            .sort((a, b) => b.score - a.score);
        while (skipLen / totalLen > p.max_skip_fraction && skips.length > 0) {
            const s = skips.shift()!;
            s.route = 'COMPRESS';
            s.reason = 'promoted (skip-fraction cap)';
            skipLen -= s.text.length;
        }
    }

    const output_text = segments
        .map(s => {
            if (s.route === 'FULL') return s.text;
            if (s.route === 'COMPRESS') {
                const head = s.text.replace(/\s+/g, ' ').slice(0, 80);
                return `[…${s.text.length} chars compressed: ${head}…]`;
            }
            return '';
        })
        .filter(t => t.length > 0)
        .join('\n\n');

    const counts = { full: 0, compress: 0, skip: 0 };
    let total_score = 0;
    for (const s of segments) {
        if (s.route === 'FULL') counts.full++;
        else if (s.route === 'COMPRESS') counts.compress++;
        else counts.skip++;
        if (s.route !== 'SKIP') total_score += s.score;
    }

    return {
        segments,
        output_text,
        counts,
        total_score,
        saved_fraction: 1 - output_text.length / totalLen,
    };
}

// ─── η (information efficiency) tracker ────────────────────────────────────

export interface EfficiencyMetric {
    request_id: string;
    model: string;
    /** Original prompt length in chars. */
    input_chars: number;
    /** Length after CCET routing. */
    active_chars: number;
    /** Output length in chars. */
    output_chars: number;
    wall_clock_ms: number;
    routing_counts: { full: number; compress: number; skip: number };
    saved_fraction: number;
    /** η = output / (active × wall_secs). Higher is better. Arbitrary units. */
    eta: number;
}

const HISTORY: EfficiencyMetric[] = [];
const MAX_HISTORY = 200;

export function recordRequest(m: Omit<EfficiencyMetric, 'eta'>): EfficiencyMetric {
    const wallSecs = Math.max(m.wall_clock_ms / 1000, 1e-3);
    const denom = Math.max(m.active_chars * wallSecs, 1);
    const eta = m.output_chars / denom;
    const full: EfficiencyMetric = { ...m, eta };
    HISTORY.unshift(full);
    if (HISTORY.length > MAX_HISTORY) HISTORY.pop();
    return full;
}

export function getRecentEfficiency(n = 50): EfficiencyMetric[] {
    return HISTORY.slice(0, Math.min(n, HISTORY.length));
}

/**
 * Drop the in-memory η history. Test-only; do not call from product code.
 * Lets unit tests start each scenario with a fresh window.
 */
export function __resetCcetHistoryForTesting(): void {
    HISTORY.length = 0;
}

/** Aggregate stats across the most recent `window` requests, or null if empty. */
export function summarizeEfficiency(window = 50): {
    sample_size: number;
    avg_eta: number;
    avg_saved_fraction: number;
    total_skipped_segments: number;
} | null {
    const recent = HISTORY.slice(0, Math.min(window, HISTORY.length));
    if (recent.length === 0) return null;
    const avg = recent.reduce((a, m) => a + m.eta, 0) / recent.length;
    const savedAvg = recent.reduce((a, m) => a + m.saved_fraction, 0) / recent.length;
    const skipped = recent.reduce((a, m) => a + m.routing_counts.skip, 0);
    return {
        sample_size: recent.length,
        avg_eta: avg,
        avg_saved_fraction: savedAvg,
        total_skipped_segments: skipped,
    };
}

// ─── high-level convenience ─────────────────────────────────────────────────

/**
 * Wrap an inference call with CCET routing + η tracking. Use as:
 *
 *   const reply = await ccetWrap('llama-3-8b', prompt, async (active) => {
 *       return llamaCppService.generate(active);
 *   });
 *
 * Trades a ~ms of preprocessing for any compute saved by the dropped
 * segments — a near-strict win once contexts grow past ~2000 chars.
 */
export async function ccetWrap<T extends string>(
    model: string,
    prompt: string,
    runner: (activePrompt: string) => Promise<T>,
    policy: Partial<RoutingPolicy> = {},
): Promise<{ output: T; route: RoutingResult; metric: EfficiencyMetric }> {
    const route = routePrompt(prompt, policy);
    const t0 = performance.now();
    const output = await runner(route.output_text);
    const t1 = performance.now();
    const metric = recordRequest({
        request_id: `${Date.now()}-${Math.floor(Math.random() * 1e6)}`,
        model,
        input_chars: prompt.length,
        active_chars: route.output_text.length,
        output_chars: output.length,
        wall_clock_ms: Math.round(t1 - t0),
        routing_counts: route.counts,
        saved_fraction: route.saved_fraction,
    });
    return { output, route, metric };
}
