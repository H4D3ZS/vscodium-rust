/**
 * Adaptive polling + idle trim + heap monitoring — keeps the IDE lean.
 * Monitors JS heap, trims Monaco models, and triggers backend GC.
 */
import { invoke } from '../../tauri_bridge';
import { STATS_POLL_MS, IDLE_STATS_POLL_MS, HIDDEN_PAUSE_POLL_MS } from '../../memory_budget';
import { useStore } from '../../store';

// Aggressive, fast checks: WebView2 fragments/crashes well before the reported
// jsHeapSizeLimit, so we trim on the used/limit ratio every few seconds (a
// streaming agent run can blow the heap between 30s checks). Thresholds are
// ratios of the REAL runtime limit — absolute byte floors were dead code when
// the configured V8 cap (tauri.conf.json --max-old-space-size) was below them.
const HEAP_TRIM_RATIO = 0.55;          // soft trim
const HEAP_HARD_RATIO = 0.75;          // hard trim (sheds transcript to avoid crash)
// During an active agent run the heap climbs fast (streamed code + diffs + render
// churn). Trim earlier and check more often so the spike never reaches the crash
// point — the OOM the user hits is almost always *mid-generation*, not at idle.
const GEN_TRIM_RATIO = 0.45;
const GEN_HARD_RATIO = 0.65;
// 2s (not 5s): on RAM-starved machines the effective heap ceiling is small
// (~200MB), so a streaming spike can crash between slow polls — check often.
const HEAP_CHECK_INTERVAL_MS = 2_000;
const GEN_CHECK_INTERVAL_MS = 750;     // sub-second while the agent is writing

let statsIntervalMs = STATS_POLL_MS;
let hidden = typeof document !== 'undefined' && document.hidden;
let lastTrimAt = 0;
let highMemoryActive = false;
let generationActive = false;
let lastFileOpenAt = 0; // Cooldown: don't trim models right after opening a file
const FILE_OPEN_COOLDOWN_MS = 5000; // 5 seconds after file open

export function isHighMemory(): boolean {
    return highMemoryActive;
}

/** Call this when a file is opened to prevent premature model disposal. */
export function markFileOpened(): void {
    lastFileOpenAt = Date.now();
}

export function currentStatsPollMs(): number {
    if (hidden) return HIDDEN_PAUSE_POLL_MS;
    return statsIntervalMs;
}

function getHeap(): { ratio: number; usedBytes: number } {
    const perf = (globalThis as any).performance;
    if (!perf?.memory) return { ratio: 0, usedBytes: 0 };
    const { usedJSHeapSize, jsHeapSizeLimit } = perf.memory;
    return { ratio: jsHeapSizeLimit ? usedJSHeapSize / jsHeapSizeLimit : 0, usedBytes: usedJSHeapSize || 0 };
}

/** Last-resort trim: shed the chat transcript + tool state to avoid an OOM crash.
 *  Better to lose old (scrolled-away) messages than blank the whole IDE.
 *  NEVER disposes Monaco models — they are the editor's core state. */
function hardTrim(): void {
    // Don't hard-trim right after a file was opened
    const sinceFileOpen = Date.now() - lastFileOpenAt;
    if (sinceFileOpen < FILE_OPEN_COOLDOWN_MS) return;

    try {
        const st = useStore.getState() as any;
        const msgs = st.agentMessages || [];
        if (msgs.length > 12) st.setAgentMessages?.(msgs.slice(-12));
        // Drop heavy buffers on the kept messages.
        const kept = (useStore.getState() as any).agentMessages || [];
        st.setAgentMessages?.(kept.map((m: any) => m.raw_buffer ? { ...m, raw_buffer: undefined } : m));
        st.clearAgentToolBlocks?.();
        st.clearTrajectory?.();
    } catch { /* best effort */ }
    // NOTE: trimMonacoModels() removed — disposing models causes files to disappear.
    // Monaco models are the editor's core state and must never be touched.
    trimTabCache();
    void invoke<string>('optimize_memory').catch(() => {});
}

function trimTabCache(): void {
    useStore.setState((state) => {
        const activeId = state.activeTabId;
        const trimmedTabs = state.tabs.map((t: any) => {
            if (t.id === activeId) return t;
            if (t.isLargePaged && t.loadedRange) {
                return { ...t, loadedRange: undefined };
            }
            return t;
        });
        return { tabs: trimmedTabs };
    });
}

function setStatus_barNote(msg: string | null): void {
    try {
        const el = document.getElementById('memory-governor-status');
        if (el) {
            el.textContent = msg;
            el.style.display = msg ? 'block' : 'none';
        }
    } catch { /* noop */ }
}

export function scheduleMemoryGovernor(): () => void {
    if (typeof document === 'undefined') return () => {};

    const onVisibility = () => {
        hidden = document.hidden;
        if (hidden) {
            statsIntervalMs = HIDDEN_PAUSE_POLL_MS;
            void trimIfStale(true);
        } else {
            statsIntervalMs = STATS_POLL_MS;
        }
    };

    const onIdle = () => {
        statsIntervalMs = IDLE_STATS_POLL_MS;
        void trimIfStale(false);
    };

    document.addEventListener('visibilitychange', onVisibility);

    let idleTimer: ReturnType<typeof setTimeout> | undefined;
    const resetIdle = () => {
        statsIntervalMs = STATS_POLL_MS;
        if (idleTimer) clearTimeout(idleTimer);
        idleTimer = setTimeout(onIdle, 90_000);
    };
    const idleEvents = ['keydown', 'mousedown', 'touchstart'] as const;
    idleEvents.forEach((ev) => {
        document.addEventListener(ev, resetIdle, { passive: true });
    });
    resetIdle();

    // Self-rescheduling check so cadence can tighten mid-run (setInterval can't).
    let heapTimer: ReturnType<typeof setTimeout> | undefined;
    let stopped = false;
    const checkHeap = () => {
        if (stopped) return;
        const { ratio, usedBytes } = getHeap();
        const hardRatio = generationActive ? GEN_HARD_RATIO : HEAP_HARD_RATIO;
        const softRatio = generationActive ? GEN_TRIM_RATIO : HEAP_TRIM_RATIO;
        const hard = ratio > hardRatio;
        const soft = ratio > softRatio;
        const mb = Math.round(usedBytes / (1024 * 1024));
        if (hard) {
            // Near the crash point — shed transcript + tool state, not just models.
            setStatus_barNote(`Memory critical (${mb}MB) — trimming history`);
            hardTrim();
            highMemoryActive = true;
        } else if (soft && !highMemoryActive) {
            // Don't trim models right after a file was opened — the model
            // may not be fully mounted yet, and trimming it causes the
            // "file opens then disappears" bug.
            const sinceFileOpen = Date.now() - lastFileOpenAt;
            if (sinceFileOpen < FILE_OPEN_COOLDOWN_MS) {
                // Skip this trim cycle — file was just opened
            } else {
                highMemoryActive = true;
                setStatus_barNote(`High memory (${mb}MB) — trimming caches`);
                // NOTE: trimMonacoModels() removed — never dispose editor models
                trimTabCache();
                void invoke<string>('optimize_memory').catch(() => {});
            }
        } else if (!soft && highMemoryActive) {
            highMemoryActive = false;
            setStatus_barNote(null);
        }
        heapTimer = setTimeout(checkHeap, generationActive ? GEN_CHECK_INTERVAL_MS : HEAP_CHECK_INTERVAL_MS);
    };
    checkHeap();

    // Track agent-run activity so the loop above pre-empts the mid-generation spike.
    const unsubGen = useStore.subscribe((s: any) => {
        const active = !!s.isAgentThinking;
        if (active === generationActive) return;
        generationActive = active;
        // A run just started — check immediately rather than waiting out the slow tick.
        if (active && heapTimer) { clearTimeout(heapTimer); checkHeap(); }
    });

    return () => {
        stopped = true;
        document.removeEventListener('visibilitychange', onVisibility);
        idleEvents.forEach((ev) => document.removeEventListener(ev, resetIdle));
        if (idleTimer) clearTimeout(idleTimer);
        if (heapTimer) clearTimeout(heapTimer);
        unsubGen();
    };
}

async function trimIfStale(force: boolean): Promise<void> {
    const now = Date.now();
    if (!force && now - lastTrimAt < 120_000) return;
    lastTrimAt = now;
    try {
        await invoke<string>('optimize_memory');
    } catch { /* offline */ }
}
