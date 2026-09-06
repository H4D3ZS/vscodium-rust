import React, { useCallback, useEffect, useRef, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

/**
 * Kortex ROCmFPX — the local AMD-GPU backend.
 *
 * One card, one button. "Start" profiles nothing, launches the bundled ROCmFPX
 * llama-server (`-ngl 999`), puts the Kortex KV-slot cache in front, and points
 * the IDE's chat at it. If a compatible server is already running at the URL it
 * just connects. This *is* the `llama-cpp` inference backend — selecting it here
 * is the same as picking it in "Other backends".
 */

type Phase = 'idle' | 'starting' | 'loading' | 'running' | 'connected' | 'error';

/** Pull a human-meaningful progress hint out of a llama-server log line. */
function progressHint(lines: string[]): string {
    for (let i = lines.length - 1; i >= 0; i--) {
        const l = lines[i];
        if (/load_tensors|loading model|model params|tensor|offloAD|offloaded|CUDA|Vulkan|warming up|kv cache|n_ctx/i.test(l)) {
            return l.replace(/^\[(out|err|gac)\]\s*/, '').replace(/^\d[\d.:]*\s+[A-Z]\s+/, '').slice(0, 120);
        }
    }
    return lines.length ? lines[lines.length - 1].replace(/^\[(out|err|gac)\]\s*/, '').slice(0, 120) : '';
}

const BINARY_REL = 'src-tauri/binaries/rocmfpx/llama-server.exe';
/** The port kortex_gac_launch binds llama-server to. The KV-cache proxy fronts it. */
const UPSTREAM_URL = 'http://127.0.0.1:8081';

const inputStyle: React.CSSProperties = {
    fontSize: 12, padding: '4px 8px', flex: 1, minWidth: 0,
    background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)',
    border: '1px solid var(--vscode-input-border)', borderRadius: 3,
};
const btn: React.CSSProperties = {
    fontSize: 12, padding: '5px 14px', borderRadius: 4, cursor: 'pointer',
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-button-secondaryBackground, #2a2d2e)',
    color: 'var(--vscode-button-secondaryForeground, inherit)',
};
const primaryBtn: React.CSSProperties = {
    ...btn, background: 'var(--vscode-button-background, #0e639c)',
    color: 'var(--vscode-button-foreground, #fff)', border: '1px solid transparent', fontWeight: 600,
};

async function pathExists(p: string): Promise<boolean> {
    try { return await invoke<boolean>('path_exists', { path: p }); } catch { return false; }
}
async function serverHealthy(url: string): Promise<boolean> {
    try {
        const r = await fetch(`${url.replace(/\/$/, '')}/health`, { signal: AbortSignal.timeout(2000) });
        return r.ok;
    } catch { return false; }
}
async function pickFile(filters: { name: string; extensions: string[] }[]): Promise<string | null> {
    try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const picked = await open({ multiple: false, directory: false, filters });
        return typeof picked === 'string' ? picked : null;
    } catch { return null; }
}

const dirEntries = (r: any[]): { name: string; path: string }[] =>
    (r || []).map((e: any) => (typeof e === 'string' ? { name: e, path: e } : e)).filter((e: any) => e && e.name);

async function autodetectModel(): Promise<string | null> {
    try {
        const { homeDir, join } = await import('@tauri-apps/api/path');
        const hub = await join(await homeDir(), '.cache', 'huggingface', 'hub');
        const repos = dirEntries(await invoke<any[]>('list_directory', { path: hub }).catch(() => []));
        const repo = repos.find(e => /escha/i.test(e.name) && /rocmfp/i.test(e.name))
            ?? repos.find(e => /^models--/i.test(e.name) && /gguf/i.test(e.name) && !/embedding/i.test(e.name));
        if (!repo) return null;
        const snaps = dirEntries(await invoke<any[]>('list_directory', { path: await join(repo.path, 'snapshots') }).catch(() => []));
        for (const snap of snaps) {
            const files = dirEntries(await invoke<any[]>('list_directory', { path: snap.path }).catch(() => []));
            const gguf = files.find(f => f.name.toLowerCase().endsWith('.gguf') && !/embedding/i.test(f.name));
            if (gguf) return gguf.path;
        }
        return null;
    } catch { return null; }
}
async function autodetectBinary(repoRoot: string | null): Promise<string | null> {
    const cands: string[] = [];
    if (repoRoot) cands.push(`${repoRoot.replace(/[\\/]$/, '')}/${BINARY_REL}`);
    cands.push(`../../${BINARY_REL.replace(/^src-tauri\//, '')}`, `../../${BINARY_REL}`);
    for (const c of cands) if (await pathExists(c)) return c;
    try {
        const { resourceDir, join } = await import('@tauri-apps/api/path');
        const p = await join(await resourceDir(), 'binaries', 'rocmfpx', 'llama-server.exe');
        if (await pathExists(p)) return p;
    } catch { /* ignore */ }
    return null;
}

export function KortexLocalInferencePanel() {
    const serverBinary = useStore(s => s.kortexServerBinary);
    const setServerBinary = useStore(s => (s as any).setKortexServerBinary);
    const vramMb = useStore(s => s.kortexVramTotalMb);
    const setVramMb = useStore(s => (s as any).setKortexVramTotalMb);
    const baseDir = useStore(s => s.kvCacheBaseDir);
    const setBaseDir = useStore(s => (s as any).setKvCacheBaseDir);
    const proxyPort = useStore(s => s.kvCacheProxyPort);
    const llamaCppUrl = useStore(s => s.llamaCppUrl);
    const setLlamaCppUrl = useStore(s => (s as any).setLlamaCppUrl);
    const setLlamaCppModelPath = useStore(s => (s as any).setLlamaCppModelPath);
    const inferenceBackend = useStore(s => s.inferenceBackend);
    const setInferenceBackend = useStore(s => (s as any).setInferenceBackend);
    const activeRoot = useStore(s => (s as any).activeRoot as string | undefined);

    const [modelPath, setModelPath] = useState<string>(() => {
        try { return localStorage.getItem('kortex.localModelPath') || ''; } catch { return ''; }
    });
    const [phase, setPhase] = useState<Phase>('idle');
    const [msg, setMsg] = useState('');
    const [modelLabel, setModelLabel] = useState('');
    type LocalGguf = { repo: string; file: string; quant: string; path: string; size_mb: number; aux: boolean };
    const [localGgufs, setLocalGgufs] = useState<LocalGguf[]>([]);
    const [ggufsLoading, setGgufsLoading] = useState(false);
    const [statsLine, setStatsLine] = useState('');
    const [specLine, setSpecLine] = useState('');
    const [specType, setSpecType] = useState<string>(() => {
        try { return localStorage.getItem('kortex.spec.type') || ''; } catch { return ''; }
    });
    // Operator = the small fast model on Lemonade that runs sub-agents + APEX
    // while this (the reasoner) keeps the main loop. Env-backed on the Rust side.
    const [opModel, setOpModel] = useState<string>(() => {
        try { return localStorage.getItem('kortex.operator.model') || ''; } catch { return ''; }
    });
    const [opUrl, setOpUrl] = useState<string>(() => {
        try { return localStorage.getItem('kortex.operator.url') || ''; } catch { return ''; }
    });
    const [nCpuMoe, setNCpuMoe] = useState<number>(() => {
        try { return parseInt(localStorage.getItem('kortex.nCpuMoe') || '0') || 0; } catch { return 0; }
    });
    const [showDetails, setShowDetails] = useState(false);
    const [showLog, setShowLog] = useState(false);
    const startedByUs = useRef(false);
    const abortRef = useRef(false);
    const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

    const isActiveBackend = inferenceBackend === 'llama-cpp';

    useEffect(() => {
        let cancelled = false;
        (async () => {
            if (!serverBinary) {
                const b = await autodetectBinary(activeRoot ?? null);
                if (b && !cancelled) setServerBinary?.(b);
            }
            if (!modelPath) {
                const m = await autodetectModel();
                if (m && !cancelled) setModelPath(m);
            }
            // Already wired through the Kortex proxy?
            try {
                const { getKvCacheStatus } = await import('../../kortex/kvcache-orchestrator');
                const info = await getKvCacheStatus();
                if (!cancelled && info) {
                    setPhase('running');
                    setModelLabel(info.model_id || '');
                    setMsg(`Proxy :${proxyPort} → ${info.upstream_url}`);
                    return;
                }
            } catch { /* not running */ }
            // A bare llama-server already up (ours on :8081, or a manual one)?
            if (cancelled) return;
            const up = (await serverHealthy(UPSTREAM_URL)) ? UPSTREAM_URL
                : (await serverHealthy(llamaCppUrl)) ? llamaCppUrl : null;
            if (up) {
                setPhase('connected');
                setMsg(`Server reachable at ${up} — press Connect to route chat through it.`);
            }
        })();
        return () => { cancelled = true; };
        // eslint-disable-next-line react-hooks/exhaustive-deps
    }, []);

    // Push the Operator override to the Rust process (env-backed) whenever it
    // changes; blank = clear the override / use the default.
    useEffect(() => {
        const t = setTimeout(() => {
            try { localStorage.setItem('kortex.operator.model', opModel); } catch { /* */ }
            try { localStorage.setItem('kortex.operator.url', opUrl); } catch { /* */ }
            invoke('kortex_set_operator', { model: opModel || null, url: opUrl || null }).catch(() => { /* engine offline */ });
        }, 400);
        return () => clearTimeout(t);
    }, [opModel, opUrl]);

    const scanGgufs = useCallback(async () => {
        setGgufsLoading(true);
        try {
            const rows = await invoke<LocalGguf[]>('kortex_gac_list_local_ggufs', { extraDir: null });
            setLocalGgufs(rows || []);
        } catch { /* leave list empty */ }
        finally { setGgufsLoading(false); }
    }, []);

    // Scan the local GGUF caches the first time the details panel is opened.
    useEffect(() => {
        if (showDetails && localGgufs.length === 0 && !ggufsLoading) void scanGgufs();
    }, [showDetails, localGgufs.length, ggufsLoading, scanGgufs]);

    useEffect(() => {
        if (phase !== 'running') {
            if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; }
            return;
        }
        const tick = async () => {
            try {
                const { getKvCacheStats, summarizeKvCache } = await import('../../kortex/kvcache-orchestrator');
                const s = await getKvCacheStats();
                if (s) setStatsLine(summarizeKvCache(s));
            } catch { /* ignore */ }
            try {
                const gac = await import('../../kortex/gac-orchestrator');
                const acc = gac.parseSpecAcceptance(await gac.getServerLog(120));
                setSpecLine(gac.formatSpecAcceptance(acc));
            } catch { /* ignore */ }
        };
        void tick();
        pollRef.current = setInterval(tick, 3000);
        return () => { if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; } };
    }, [phase]);

    const resolveBase = useCallback(async () => {
        const isAbs = (p: string) => /^([a-zA-Z]:[\\/]|\/)/.test(p);
        if (baseDir && isAbs(baseDir)) return baseDir;
        try {
            const { appLocalDataDir, join } = await import('@tauri-apps/api/path');
            const b = await join(await appLocalDataDir(), 'kortex-kv');
            setBaseDir?.(b);
            return b;
        } catch { return baseDir || ''; }
    }, [baseDir, setBaseDir]);

    const useBackend = useCallback((url: string) => {
        setLlamaCppUrl?.(url);
        // The chat router derives the model name from this when the backend is
        // llama-cpp; without it, a stale model tag gets sent to the wrong server.
        if (modelPath) setLlamaCppModelPath?.(modelPath);
        setInferenceBackend?.('llama-cpp');
    }, [setLlamaCppUrl, setLlamaCppModelPath, setInferenceBackend, modelPath]);

    const start = useCallback(async () => {
        abortRef.current = false;
        setPhase('starting'); setStatsLine(''); setSpecLine(''); setMsg('');
        try {
            const kv = await import('../../kortex/kvcache-orchestrator');
            const gac = await import('../../kortex/gac-orchestrator');
            const base = await resolveBase();
            if (!base) throw new Error('Could not resolve a cache directory.');

            // The ngram-cache speculator wants a file to persist learned
            // n-grams between restarts; park it next to the KV slots.
            // startLocalInference reads kortex.spec.* from localStorage.
            try {
                if (specType.split(',').includes('ngram-cache')) {
                    localStorage.setItem('kortex.spec.lookupCache', `${base}/ngram-cache.bin`);
                }
            } catch { /* ignore */ }

            // Where the actual llama-server lives (the KV-cache proxy fronts it).
            let upstream = UPSTREAM_URL;

            if (await serverHealthy(UPSTREAM_URL) || await serverHealthy(llamaCppUrl)) {
                upstream = (await serverHealthy(UPSTREAM_URL)) ? UPSTREAM_URL : llamaCppUrl;
                setMsg('Connecting the prefix cache to the running server…');
            } else {
                if (!modelPath) throw new Error('No model found. Open "Model & engine" and pick a .gguf.');
                try { localStorage.setItem('kortex.localModelPath', modelPath); } catch { /* ignore */ }

                // Spawn and return immediately — do NOT block on health here, or
                // the whole panel freezes for minutes with no feedback.
                setPhase('loading'); setMsg('Starting llama-server…');
                await gac.startLocalInference({
                    model_path: modelPath,
                    backend: 'vulkan',
                    vram_total_mb: vramMb || 16384,
                    launch: {
                        server_binary: serverBinary || undefined,
                        port: 8081,
                        // Claude Code's harness alone is ~28k tokens (system
                        // prompt + tool schemas), so the window has to clear
                        // that with room for a few conversation turns. q4_0 KV
                        // keeps 32k in VRAM next to the 2.5-bpw weights on 16 GB.
                        ctx_size: 32768,
                        slot_save_path: `${base}/slots`,
                        wait_healthy_secs: 0,
                        // MoE expert offload — set for a Q4 35B-A3B on 16 GB
                        // (dense weights on GPU, expert FFNs to RAM). 0 = off.
                        n_cpu_moe: nCpuMoe > 0 ? nCpuMoe : undefined,
                        extra_args: ['--jinja', '--cache-type-k', 'q4_0', '--cache-type-v', 'q4_0'],
                    },
                });
                startedByUs.current = true;

                // Poll the *upstream* (:8081) — NOT llamaCppUrl, which may still
                // point at the not-yet-started proxy.
                const t0 = Date.now();
                const DEADLINE_MS = 8 * 60_000;
                for (;;) {
                    await new Promise(r => setTimeout(r, 1500));
                    if (abortRef.current) return;
                    const secs = Math.round((Date.now() - t0) / 1000);
                    const log = await gac.getServerLog(60);
                    const hint = progressHint(log);
                    setMsg(`Loading model — ${secs}s${hint ? `  ·  ${hint}` : ''}`);

                    const status = await gac.getRunningServer().catch(() => null);
                    if (status && status.alive === false) {
                        const tail = (await gac.getServerLog(12)).join('\n');
                        throw new Error(`llama-server exited (code ${status.exit_code ?? '?'}).\n${tail}`);
                    }
                    if (await serverHealthy(UPSTREAM_URL)) break;
                    if (Date.now() - t0 > DEADLINE_MS) {
                        throw new Error(`Model didn't finish loading in ${DEADLINE_MS / 60000} min. Last log:\n${(await gac.getServerLog(12)).join('\n')}`);
                    }
                }
            }

            setMsg('Starting the prefix cache…');
            const opts = kv.makeKvCacheOptions(base, {
                upstream_url: upstream,
                proxy_port: proxyPort || 1537,
                max_bytes: (vramMb && vramMb > 0 ? vramMb : 16384) * 1024 * 1024,
            });
            const boundPort = await kv.startKvCache(opts).catch((e) => {
                if (String(e).includes('already running')) return proxyPort || 1537;
                throw e;
            });

            const proxyUrl = `http://127.0.0.1:${boundPort}`;
            useBackend(proxyUrl);
            setModelLabel((modelPath.split(/[\\/]/).pop() || '').replace(/\.gguf$/i, ''));
            setPhase('running');
            setMsg(`Live — chat runs on this model (proxy :${boundPort} → ${upstream}).`);
        } catch (e) {
            setPhase('error');
            setMsg(String((e as any)?.message ?? e));
        }
    }, [llamaCppUrl, modelPath, vramMb, serverBinary, proxyPort, specType, resolveBase, useBackend]);

    const stop = useCallback(async () => {
        abortRef.current = true;
        setMsg('Stopping…');
        try {
            const kv = await import('../../kortex/kvcache-orchestrator');
            const gac = await import('../../kortex/gac-orchestrator');
            await kv.stopKvCache().catch(() => {});
            if (startedByUs.current) await gac.stopServer().catch(() => {});
        } finally {
            startedByUs.current = false;
            setPhase('idle'); setMsg(''); setStatsLine(''); setModelLabel('');
        }
    }, []);

    const running = phase === 'running';
    const busy = phase === 'starting' || phase === 'loading';
    const dotColor = running || phase === 'connected' ? '#4ec9b0'
        : phase === 'error' ? '#f7768e'
        : busy ? '#e5a00d' : '#808080';
    const shortModel = (modelLabel || modelPath.split(/[\\/]/).pop() || '').replace(/\.gguf$/i, '');
    const statusText = running ? (isActiveBackend ? 'active' : 'running')
        : phase === 'connected' ? 'reachable'
        : phase === 'loading' ? 'loading…'
        : phase === 'starting' ? 'starting…'
        : phase === 'error' ? 'error'
        : shortModel ? `ready · ${shortModel}` : 'local · AMD GPU';
    const msgLines = msg ? msg.split('\n') : [];
    const headline = msgLines[0] || '';
    const rest = msgLines.slice(1).join('\n');

    const label = { fontSize: 11, opacity: 0.8, marginTop: 4 } as React.CSSProperties;
    const row = { display: 'flex', gap: 6 } as React.CSSProperties;

    const selectThisBackend = () => {
        setInferenceBackend?.('llama-cpp');
        setLlamaCppUrl?.(running ? `http://127.0.0.1:${proxyPort || 1537}` : UPSTREAM_URL);
        if (modelPath) setLlamaCppModelPath?.(modelPath);
    };

    return (
        <div style={{ borderBottom: '1px solid var(--vscode-panel-border)', paddingBottom: 12 }}>
            {/* main row — a real backend option in the same radio group as the rest */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <label style={{ display: 'flex', alignItems: 'center', gap: 8, cursor: 'pointer', flex: 1, minWidth: 0 }}>
                    <input type="radio" name="inference-backend"
                        checked={isActiveBackend}
                        onChange={selectThisBackend} />
                    <span style={{ fontSize: 12, fontWeight: 600 }}>Kortex ROCmFPX</span>
                    <span style={{
                        width: 8, height: 8, borderRadius: '50%', flex: 'none',
                        background: running || phase === 'connected' || busy || phase === 'error' ? dotColor : '#555',
                    }} />
                    <span style={{ fontSize: 11, opacity: 0.55, flex: 1, minWidth: 0, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                        {statusText}
                    </span>
                </label>
                {running || busy
                    ? <button type="button" style={{ ...btn, padding: '3px 10px' }} onClick={stop}>{busy ? 'Cancel' : 'Stop'}</button>
                    : <button type="button" style={{ ...primaryBtn, padding: '3px 12px' }} onClick={start}>
                        {phase === 'connected' ? 'Connect' : 'Start'}
                      </button>}
                <button type="button" title="Model & engine"
                    onClick={() => setShowDetails(v => !v)}
                    style={{ ...btn, padding: '3px 6px', background: 'transparent', opacity: 0.7 }}>
                    {showDetails ? '▾' : '⚙'}
                </button>
            </div>

            {/* one status line; full log behind a toggle */}
            {headline && (
                <div style={{ fontSize: 10.5, marginTop: 6, marginLeft: 18, display: 'flex', gap: 8, alignItems: 'baseline',
                    color: phase === 'error' ? 'var(--vscode-errorForeground)' : 'var(--vscode-descriptionForeground)' }}>
                    <span style={{ flex: 1, minWidth: 0, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{headline}</span>
                    {rest && (
                        <button type="button" onClick={() => setShowLog(v => !v)}
                            style={{ ...btn, padding: '0 6px', fontSize: 10, background: 'transparent' }}>
                            {showLog ? 'hide log' : 'log'}
                        </button>
                    )}
                </div>
            )}
            {running && statsLine && (
                <div style={{ fontSize: 10, marginTop: 3, marginLeft: 18, opacity: 0.6, fontFamily: 'var(--vscode-editor-font-family, monospace)' }}>{statsLine}</div>
            )}
            {running && specLine && (
                <div style={{ fontSize: 10, marginTop: 2, marginLeft: 18, opacity: 0.6, fontFamily: 'var(--vscode-editor-font-family, monospace)' }}>{specLine}</div>
            )}
            {showLog && rest && (
                <pre style={{
                    fontSize: 10, lineHeight: 1.35, margin: '6px 0 0 18px', padding: 8, maxHeight: 110, overflow: 'auto',
                    background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.25))', borderRadius: 3,
                    color: 'var(--vscode-descriptionForeground)', whiteSpace: 'pre-wrap',
                }}>{rest}</pre>
            )}

            {/* details: compact, only on demand */}
            {showDetails && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginTop: 10, marginLeft: 18 }}>
                    <div style={{ ...row, alignItems: 'center' }}>
                        <label style={{ ...label, flex: 1 }}>
                            Downloaded models {ggufsLoading ? '(scanning…)' : localGgufs.length ? `(${localGgufs.filter(g => !g.aux).length})` : ''}
                        </label>
                        <button type="button" style={{ ...btn, padding: '2px 8px' }} disabled={running || ggufsLoading}
                            onClick={scanGgufs} title="Re-scan the HF / Lemonade caches">↻</button>
                    </div>
                    <select style={{ ...inputStyle, flex: 'none', width: '100%' }} disabled={running}
                        value={localGgufs.some(g => g.path === modelPath) ? modelPath : ''}
                        onChange={e => { if (e.target.value) { setModelPath(e.target.value); try { localStorage.setItem('kortex.localModelPath', e.target.value); } catch { /* */ } } }}>
                        <option value="">{localGgufs.length ? '— pick a downloaded GGUF —' : '(none found — pull one via Lemonade, or use the path below)'}</option>
                        {localGgufs.filter(g => !g.aux).map(g => (
                            <option key={g.path} value={g.path}>
                                {g.repo} · {g.quant || 'gguf'} · {(g.size_mb / 1024).toFixed(1)} GB
                            </option>
                        ))}
                    </select>

                    <label style={label}>…or a model file path (.gguf)</label>
                    <div style={row}>
                        <input style={inputStyle} value={modelPath} placeholder="auto-detected"
                            onChange={e => setModelPath(e.target.value)} disabled={running} />
                        <button type="button" style={btn} disabled={running} onClick={async () => {
                            const p = await pickFile([{ name: 'GGUF', extensions: ['gguf'] }]); if (p) setModelPath(p);
                        }}>…</button>
                    </div>
                    {/(^|[^a-z0-9])(i?q2|i?q3|q2_k|q3_k)([^a-z0-9]|$)/i.test(modelPath) && (
                        <div style={{ ...label, color: 'var(--vscode-editorWarning-foreground, #e5a00d)', marginTop: 0 }}>
                            ⚠ Sub-4-bit quant — tool-call JSON and reasoning degrade fast at this size.
                            Use Q4_K_M or better for the reasoner.
                        </div>
                    )}
                    <label style={label}>Server engine</label>
                    <div style={row}>
                        <input style={inputStyle} value={serverBinary} placeholder="auto (bundled ROCmFPX)"
                            onChange={e => setServerBinary?.(e.target.value)} disabled={running} />
                        <button type="button" style={btn} disabled={running} onClick={async () => {
                            const p = await pickFile([{ name: 'Executable', extensions: ['exe', ''] }]); if (p) setServerBinary?.(p);
                        }}>…</button>
                    </div>
                    <label style={label}>Server URL (connect to an existing one)</label>
                    <input style={inputStyle} value={llamaCppUrl} onChange={e => setLlamaCppUrl?.(e.target.value)} />
                    <label style={label}>GPU memory budget (MB)</label>
                    <input type="number" min={2048} step={512} style={{ ...inputStyle, flex: 'none', width: 120 }}
                        value={vramMb || 16384} disabled={running}
                        onChange={e => setVramMb?.(parseInt(e.target.value) || 16384)} />

                    <label style={label}>MoE expert offload (`--n-cpu-moe`, 0 = off)</label>
                    <input type="number" min={0} max={64} step={1} style={{ ...inputStyle, flex: 'none', width: 120 }}
                        value={nCpuMoe} disabled={running}
                        onChange={e => {
                            const v = Math.max(0, Math.min(64, parseInt(e.target.value) || 0));
                            setNCpuMoe(v);
                            try { localStorage.setItem('kortex.nCpuMoe', String(v)); } catch { /* */ }
                        }} />
                    <div style={{ ...label, opacity: 0.55, marginTop: 0 }}>
                        For a Q4 <b>MoE</b> (35B-A3B) on 16&nbsp;GB: try 20–24. Spills that
                        many layers' experts to RAM so the model fits. No effect on a dense model.
                    </div>

                    <label style={label}>Speculative decoding (decode-speed, output unchanged)</label>
                    <select style={{ ...inputStyle, flex: 'none', width: '100%' }} value={specType} disabled={running}
                        onChange={e => {
                            setSpecType(e.target.value);
                            try { localStorage.setItem('kortex.spec.type', e.target.value); } catch { /* ignore */ }
                        }}>
                        <option value="">Off</option>
                        <option value="ngram-simple">Prompt lookup — code (no model, no VRAM)</option>
                        <option value="ngram-cache">Prompt lookup + persistent cache</option>
                        <option value="ngram-simple,draft-mtp">Lookup + MTP head</option>
                        <option value="draft-mtp">MTP head only (auto-detected)</option>
                    </select>
                    <div style={{ ...label, opacity: 0.55, marginTop: 0 }}>
                        The full model verifies every drafted token — same output, more tokens per pass.
                        Applies on next Start.
                    </div>

                    <label style={{ ...label, marginTop: 8 }}>Operator — small model on Lemonade (sub-agents, APEX)</label>
                    <div style={row}>
                        <input style={inputStyle} value={opModel} placeholder="qwen3.5:4b"
                            onChange={e => setOpModel(e.target.value)} />
                    </div>
                    <div style={row}>
                        <input style={inputStyle} value={opUrl} placeholder="http://localhost:13305"
                            onChange={e => setOpUrl(e.target.value)} />
                    </div>
                    <div style={{ ...label, opacity: 0.55, marginTop: 0 }}>
                        This box is the reasoner (big model). The Operator runs the tool-call grunt
                        work on a separate Lemonade server. Blank = defaults.
                    </div>
                </div>
            )}
        </div>
    );
}

export default KortexLocalInferencePanel;
