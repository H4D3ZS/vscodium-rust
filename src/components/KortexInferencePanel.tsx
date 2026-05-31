/**
 * Kortex Inference Panel
 *
 * Single React panel exposing the three Kortex inference subsystems:
 *   - GAC (geometry-aware tier placement → llama-server launch).
 *   - KDKVC (disk KV cache proxy in front of llama-server).
 *   - CCET (heuristic token router + η information-efficiency metric).
 *
 * Lives inside `AgentSettingsView.tsx`. All state is in the Zustand store so
 * settings persist across sessions via localStorage.
 */

import React, { useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import {
    profileModel,
    quickPlan,
    launchServer,
    stopServer as stopGacServer,
    summarizePlan,
    defaultProfilePath,
    type Backend,
    type TierPlan,
} from '../kortex/gac-orchestrator';
import {
    startKvCache,
    stopKvCache,
    clearKvCache,
    makeKvCacheOptions,
    summarizeKvCache,
    getKvCacheStatus,
    type RunningCacheInfo,
} from '../kortex/kvcache-orchestrator';
import {
    loadAimTelemetry,
    getAimTelemetrySnapshot,
    flushAimTelemetry,
    clearAimTelemetrySamples,
    summarizeAimTelemetry,
    type AimTelemetrySnapshot,
} from '../kortex/aim-vfs';
import { resolveOllamaModelToGguf } from '../kortex/ollama-bridge';

// ─── presentational helpers ────────────────────────────────────────────────

const sectionStyle: React.CSSProperties = {
    marginTop: 16,
    padding: 16,
    background: 'var(--vscode-editor-background)',
    border: '1px solid var(--vscode-panel-border)',
    borderRadius: 6,
};

const sectionTitleStyle: React.CSSProperties = {
    fontSize: 13,
    fontWeight: 700,
    marginBottom: 12,
    letterSpacing: 0.3,
    color: 'var(--vscode-foreground)',
};

const labelStyle: React.CSSProperties = {
    fontSize: 11,
    opacity: 0.8,
    marginTop: 8,
    marginBottom: 4,
    display: 'block',
};

const inputStyle: React.CSSProperties = {
    width: '100%',
    padding: '6px 8px',
    fontSize: 12,
    background: 'var(--vscode-input-background)',
    color: 'var(--vscode-input-foreground)',
    border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
    borderRadius: 4,
    outline: 'none',
};

const btnStyle = (variant: 'primary' | 'secondary' | 'danger' = 'secondary'): React.CSSProperties => ({
    padding: '6px 12px',
    fontSize: 12,
    fontWeight: 600,
    cursor: 'pointer',
    borderRadius: 4,
    border: '1px solid var(--vscode-button-border, transparent)',
    background:
        variant === 'primary'
            ? 'var(--vscode-button-background)'
            : variant === 'danger'
                ? 'rgba(220, 80, 80, 0.15)'
                : 'var(--vscode-button-secondaryBackground)',
    color:
        variant === 'primary'
            ? 'var(--vscode-button-foreground)'
            : variant === 'danger'
                ? '#ff7a7a'
                : 'var(--vscode-button-secondaryForeground)',
    marginRight: 6,
    marginTop: 8,
});

const pillStyle = (color: string): React.CSSProperties => ({
    display: 'inline-block',
    padding: '2px 8px',
    fontSize: 10,
    fontWeight: 700,
    letterSpacing: 0.5,
    borderRadius: 999,
    background: `${color}22`,
    color,
    marginRight: 6,
});

function fmtBytes(n: number): string {
    if (n >= 1024 ** 3) return `${(n / 1024 ** 3).toFixed(2)} GB`;
    if (n >= 1024 ** 2) return `${(n / 1024 ** 2).toFixed(1)} MB`;
    if (n >= 1024) return `${(n / 1024).toFixed(1)} KB`;
    return `${n} B`;
}

/** Strip `Ollama|<tag>` prefix used by the agent model picker. */
function parseOllamaModelTag(agentModel: string): string {
    const t = agentModel.trim();
    const pipe = t.indexOf('|');
    if (pipe > 0 && t.slice(0, pipe).toLowerCase() === 'ollama') {
        return t.slice(pipe + 1).trim();
    }
    return t;
}

// ─── component ──────────────────────────────────────────────────────────────

const KortexInferencePanel: React.FC = () => {
    // GAC state
    const kortexGacEnabled = useStore(s => s.kortexGacEnabled);
    const setKortexGacEnabled = useStore(s => s.setKortexGacEnabled);
    const kortexVramTotalMb = useStore(s => s.kortexVramTotalMb);
    const setKortexVramTotalMb = useStore(s => s.setKortexVramTotalMb);
    const kortexTheta = useStore(s => s.kortexTheta);
    const setKortexTheta = useStore(s => s.setKortexTheta);
    const kortexBackend = useStore(s => s.kortexBackend);
    const setKortexBackend = useStore(s => s.setKortexBackend);
    const kortexServerBinary = useStore(s => s.kortexServerBinary);
    const setKortexServerBinary = useStore(s => s.setKortexServerBinary);
    const llamaCppModelPath = useStore(s => s.llamaCppModelPath);
    const setLlamaCppModelPath = useStore(s => s.setLlamaCppModelPath);
    const llamaCppUrl = useStore(s => s.llamaCppUrl);
    const agentModel = useStore(s => s.agentModel);

    // KV cache state
    const kvCacheEnabled = useStore(s => s.kvCacheEnabled);
    const setKvCacheEnabled = useStore(s => s.setKvCacheEnabled);
    const kvCacheBaseDir = useStore(s => s.kvCacheBaseDir);
    const setKvCacheBaseDir = useStore(s => s.setKvCacheBaseDir);
    const kvCacheMaxBytes = useStore(s => s.kvCacheMaxBytes);
    const setKvCacheMaxBytes = useStore(s => s.setKvCacheMaxBytes);
    const kvCacheProxyPort = useStore(s => s.kvCacheProxyPort);
    const setKvCacheProxyPort = useStore(s => s.setKvCacheProxyPort);
    const kvCacheStats = useStore(s => s.kvCacheStats);
    const refreshKvCacheStats = useStore(s => s.refreshKvCacheStats);

    // CCET state
    const ccetEnabled = useStore(s => s.ccetEnabled);
    const setCcetEnabled = useStore(s => s.setCcetEnabled);
    const ccetTauSkip = useStore(s => s.ccetTauSkip);
    const setCcetTauSkip = useStore(s => s.setCcetTauSkip);
    const ccetTauCompress = useStore(s => s.ccetTauCompress);
    const setCcetTauCompress = useStore(s => s.setCcetTauCompress);
    const ccetMaxSkipFraction = useStore(s => s.ccetMaxSkipFraction);
    const setCcetMaxSkipFraction = useStore(s => s.setCcetMaxSkipFraction);
    const ccetEfficiency = useStore(s => s.ccetEfficiency);
    const refreshCcetEfficiency = useStore(s => s.refreshCcetEfficiency);

    // Live telemetry (populated by inference services after each completion).
    const telemetry = useStore(s => s.kortexTelemetry);

    // Local component state — async results that don't belong in the global store.
    const [profilePath, setProfilePath] = useState<string>('');
    const [plan, setPlan] = useState<TierPlan | null>(null);
    const [busy, setBusy] = useState<string>(''); // human-readable "what's running"
    const [err, setErr] = useState<string>('');
    const [gacRunning, setGacRunning] = useState<boolean>(false);
    const [kvRunning, setKvRunning] = useState<boolean>(false);
    const [cacheStatus, setCacheStatus] = useState<RunningCacheInfo | null>(null);
    const [aim, setAim] = useState<AimTelemetrySnapshot | null>(null);
    const [ollamaTagForGguf, setOllamaTagForGguf] = useState('');

    // Resolve the default profile path whenever the model changes.
    useEffect(() => {
        if (!llamaCppModelPath) { setProfilePath(''); return; }
        defaultProfilePath(llamaCppModelPath).then(setProfilePath).catch(() => setProfilePath(''));
    }, [llamaCppModelPath]);

    // Poll KV cache stats every 4 s while the proxy is running.
    useEffect(() => {
        if (!kvRunning) return;
        const tick = () => { void refreshKvCacheStats(); };
        tick();
        const id = setInterval(tick, 4000);
        return () => clearInterval(id);
    }, [kvRunning, refreshKvCacheStats]);

    // Fetch RunningCacheInfo whenever the proxy state flips on. Surfaces the
    // model identity (model_id, quant_signature, tokenizer_hash) so the user
    // can confirm the cache is bound to the model they think it is.
    useEffect(() => {
        if (!kvRunning) { setCacheStatus(null); return; }
        let cancelled = false;
        getKvCacheStatus()
            .then((info) => { if (!cancelled) setCacheStatus(info); })
            .catch(() => { if (!cancelled) setCacheStatus(null); });
        return () => { cancelled = true; };
    }, [kvRunning]);

    // Load the .aim neural VFS once on mount, then poll snapshot every 4 s so
    // the lifetime tallies stay roughly fresh as new completions roll in.
    useEffect(() => {
        let cancelled = false;
        loadAimTelemetry()
            .then((snap) => { if (!cancelled) setAim(snap); })
            .catch(() => { if (!cancelled) setAim(null); });
        const id = setInterval(() => {
            getAimTelemetrySnapshot()
                .then((snap) => { if (!cancelled) setAim(snap); })
                .catch(() => { /* keep last value */ });
        }, 4000);
        return () => { cancelled = true; clearInterval(id); };
    }, []);

    const onAimFlush = async () => {
        await wrap('Flushing telemetry.aim', async () => {
            await flushAimTelemetry();
            const snap = await getAimTelemetrySnapshot();
            setAim(snap);
        });
    };

    const onAimClearSamples = async () => {
        await wrap('Clearing telemetry samples', async () => {
            await clearAimTelemetrySamples();
            const snap = await getAimTelemetrySnapshot();
            setAim(snap);
        });
    };

    // Refresh CCET efficiency on every prop change to keep the UI fresh.
    useEffect(() => { refreshCcetEfficiency(); }, [refreshCcetEfficiency]);

    // ── AIM Workspace Index state ──────────────────────────────────────────
    const [aimIndexStatus, setAimIndexStatus] = useState<'idle' | 'indexing' | 'done' | 'error'>('idle');
    const [aimIndexStats, setAimIndexStats] = useState<{ files: number; slots: number; confidence: number } | null>(null);
    const activeRoot = useStore(s => (s as any).activeRoot ?? '');

    // Poll AIM trust manifest to show current index status
    useEffect(() => {
        import('../kortex/aim-vfs').then(({ getAimTrustManifest }) => {
            getAimTrustManifest().then((m: any) => {
                if (m && m.confidence > 0) {
                    setAimIndexStats({
                        files: m.total_files ?? m.file_count ?? 0,
                        slots: m.slot_count ?? 0,
                        confidence: m.confidence ?? 0,
                    });
                    setAimIndexStatus('done');
                }
            }).catch(() => {});
        });

        // Listen for index progress events
        import('@tauri-apps/api/event').then(({ listen }) => {
            const unlisten = listen('aim-index-progress', (event: any) => {
                const status = event.payload?.status;
                if (status === 'started') setAimIndexStatus('indexing');
                else if (status === 'complete') {
                    setAimIndexStatus('done');
                    // Refresh stats
                    import('../kortex/aim-vfs').then(({ getAimTrustManifest }) => {
                        getAimTrustManifest().then((m: any) => {
                            if (m) setAimIndexStats({ files: m.total_files ?? 0, slots: m.slot_count ?? 0, confidence: m.confidence ?? 0 });
                        }).catch(() => {});
                    });
                }
                else if (status === 'error') setAimIndexStatus('error');
            });
            return () => { unlisten.then(f => f()); };
        });
    }, []);

    const onIndexWorkspace = async () => {
        setAimIndexStatus('indexing');
        try {
            await invoke('trigger_workspace_index');
        } catch (e) {
            setAimIndexStatus('error');
            setErr(`Index failed: ${e}`);
        }
    };

    // ─── actions ───────────────────────────────────────────────────────────

    const wrap = async <T,>(label: string, fn: () => Promise<T>) => {
        setErr('');
        setBusy(label);
        try {
            return await fn();
        } catch (e) {
            const msg = e instanceof Error ? e.message : String(e);
            setErr(`${label} failed: ${msg}`);
            throw e;
        } finally {
            setBusy('');
        }
    };

    const onProfile = async () => {
        if (!llamaCppModelPath) { setErr('Set a model path first.'); return; }
        await wrap('Profiling model', async () => {
            const out = await profileModel(llamaCppModelPath, 256, 0xC0FFEE);
            setProfilePath(out);
        });
    };

    const onPreviewPlan = async () => {
        if (!llamaCppModelPath) { setErr('Set a model path first.'); return; }
        await wrap('Building plan', async () => {
            const p = await quickPlan(llamaCppModelPath, {
                vram_total_mb: kortexVramTotalMb,
                theta: kortexTheta,
                backend: kortexBackend as Backend,
            });
            setPlan(p);
        });
    };

    const onStart = async () => {
        if (!llamaCppModelPath) { setErr('Set a model path first.'); return; }
        await wrap('Starting Kortex stack', async () => {
            const cachedBase = kvCacheBaseDir && kvCacheBaseDir.trim() !== ''
                ? kvCacheBaseDir
                : undefined;
            const home = (typeof window !== 'undefined'
                && (window as { process?: { env?: { USERPROFILE?: string; HOME?: string } } }).process?.env?.USERPROFILE)
                || (typeof window !== 'undefined'
                    && (window as { process?: { env?: { USERPROFILE?: string; HOME?: string } } }).process?.env?.HOME)
                || '.';
            const baseDir = cachedBase ?? `${home}/.kortex/kvcache`;

            // 1. Plan + launch llama-server (with --slot-save-path so the proxy can save).
            const p = await quickPlan(llamaCppModelPath, {
                vram_total_mb: kortexVramTotalMb,
                theta: kortexTheta,
                backend: kortexBackend as Backend,
            });
            setPlan(p);
            await launchServer(p, llamaCppModelPath, {
                server_binary: kortexServerBinary && kortexServerBinary.trim() !== ''
                    ? kortexServerBinary
                    : undefined,
                slot_save_path: kvCacheEnabled ? `${baseDir}/slots` : undefined,
                wait_healthy_secs: 90,
            });
            setGacRunning(true);

            // 2. If KV cache is enabled, boot the proxy in front.
            if (kvCacheEnabled) {
                const cacheOpts = makeKvCacheOptions(baseDir, {
                    upstream_url: llamaCppUrl,
                    proxy_port: kvCacheProxyPort,
                    max_bytes: kvCacheMaxBytes,
                });
                await startKvCache(cacheOpts);
                setKvRunning(true);
            }
        });
    };

    const onStop = async () => {
        await wrap('Stopping Kortex stack', async () => {
            try { await stopKvCache(); } catch { /* may not be running */ }
            setKvRunning(false);
            try { await stopGacServer(); } catch { /* may not be running */ }
            setGacRunning(false);
        });
    };

    const onClearCache = async () => {
        await wrap('Clearing KV cache', async () => {
            await clearKvCache();
            await refreshKvCacheStats();
        });
    };

    // ─── render ────────────────────────────────────────────────────────────

    return (
        <div style={{ marginTop: 24 }}>

            {/* ── AIM Workspace Index ─────────────────────────────────────── */}
            <div style={{ ...sectionStyle, marginBottom: 8, borderColor: aimIndexStatus === 'done' ? 'rgba(74,222,128,0.3)' : 'var(--vscode-panel-border)' }}>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: 6 }}>
                    <span style={{ ...sectionTitleStyle, marginBottom: 0 }}>
                        ⚡ AIM Workspace Index
                    </span>
                    <span style={{ fontSize: 10, opacity: 0.6 }}>
                        {aimIndexStatus === 'idle' && 'Not indexed'}
                        {aimIndexStatus === 'indexing' && '⟳ Indexing…'}
                        {aimIndexStatus === 'done' && '✓ Ready'}
                        {aimIndexStatus === 'error' && '✗ Error'}
                    </span>
                </div>
                {aimIndexStats && aimIndexStatus === 'done' && (
                    <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 8 }}>
                        {aimIndexStats.files} files · {aimIndexStats.slots} slots · {aimIndexStats.confidence}% confidence
                        <span style={{ marginLeft: 8, color: '#4ade80' }}>Zero-grep mode active</span>
                    </div>
                )}
                {aimIndexStatus !== 'done' && (
                    <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 8 }}>
                        Index the workspace so AIRI understands your codebase without grepping every file.
                    </div>
                )}
                <button
                    style={btnStyle(aimIndexStatus === 'indexing' ? 'secondary' : 'primary')}
                    disabled={aimIndexStatus === 'indexing'}
                    onClick={onIndexWorkspace}
                >
                    {aimIndexStatus === 'indexing' ? '⟳ Indexing…' : '⚡ Index Workspace'}
                </button>
            </div>

            <h3 style={{
                fontSize: 14,
                fontWeight: 800,
                margin: 0,
                marginBottom: 4,
                color: 'var(--vscode-foreground)',
                letterSpacing: 0.4,
            }}>
                Kortex Inference Stack
            </h3>
            <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 8 }}>
                geometry-aware weight placement + ds4-style disk KV cache + CCET token router.
                Aimed at running 30B+ models on 8 GB consumer GPUs.
            </div>

            {/* Status banner */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 6, marginBottom: 8 }}>
                <span style={pillStyle(gacRunning ? '#4ade80' : 'rgba(255,255,255,0.5)')}>
                    GAC {gacRunning ? 'RUNNING' : 'IDLE'}
                </span>
                <span style={pillStyle(kvRunning ? '#4ade80' : 'rgba(255,255,255,0.5)')}>
                    KV CACHE {kvRunning ? 'RUNNING' : 'IDLE'}
                </span>
                <span style={pillStyle(ccetEnabled ? '#fbbf24' : 'rgba(255,255,255,0.5)')}>
                    CCET {ccetEnabled ? 'ON' : 'OFF'}
                </span>
                {busy && (
                    <span style={{ fontSize: 11, opacity: 0.8, marginLeft: 8 }}>
                        ⚙ {busy}…
                    </span>
                )}
            </div>

            {/* ── Backend feature matrix ───────────────────────────────── */}
            {(() => {
                // Render-time only, no state — derives what's active right
                // now from the current Zustand store. This is the honest
                // "what gains am I actually getting?" panel.
                const s = useStore.getState();
                const agentModel: string = s.agentModel || '';
                const lower = agentModel.toLowerCase();
                let activeBackend: 'ollama' | 'llama.cpp' | 'anthropic' | 'google' | 'openai' | 'unknown';
                if (lower.includes('claude') || lower.includes('anthropic')) activeBackend = 'anthropic';
                else if (lower.includes('gemini') || lower.includes('google')) activeBackend = 'google';
                else if (lower.startsWith('gpt') || lower.includes('openai')) activeBackend = 'openai';
                else if (lower.includes('llama.cpp') || s.llamaCppStatus === 'running') activeBackend = 'llama.cpp';
                else if (lower.includes('ollama') || agentModel.includes(':')) activeBackend = 'ollama';
                else activeBackend = 'unknown';

                type Row = { label: string; cells: ('on' | 'off' | 'n/a')[] };
                const ON: 'on' = 'on';
                const OFF: 'off' = 'off';
                const NA: 'n/a' = 'n/a';
                // Columns: Ollama | llama.cpp+KDKVC | Cloud
                const rows: Row[] = [
                    { label: 'CCET token routing', cells: [s.ccetEnabled ? ON : OFF, s.ccetEnabled ? ON : OFF, s.ccetEnabled ? ON : OFF] },
                    { label: 'tok/s telemetry', cells: [ON, ON, OFF] },
                    { label: '.aim neural VFS gist injection', cells: [ON, ON, NA] },
                    { label: '.aim durable telemetry', cells: [ON, ON, OFF] },
                    { label: 'KDKVC disk KV cache', cells: [NA, kvRunning ? ON : OFF, NA] },
                    { label: 'GAC geometry-aware tier placement', cells: [NA, gacRunning ? ON : OFF, NA] },
                ];
                const colIndex = activeBackend === 'ollama' ? 0
                    : activeBackend === 'llama.cpp' ? 1
                        : (activeBackend === 'anthropic' || activeBackend === 'google' || activeBackend === 'openai') ? 2
                            : -1;
                const cellColor = (v: 'on' | 'off' | 'n/a') =>
                    v === 'on' ? '#4ade80' : v === 'off' ? 'rgba(255,255,255,0.4)' : 'rgba(255,255,255,0.25)';
                const cellLabel = (v: 'on' | 'off' | 'n/a') => v === 'on' ? '●' : v === 'off' ? '○' : '–';

                return (
                    <div style={{
                        marginTop: 12,
                        padding: 12,
                        borderRadius: 6,
                        background: 'rgba(96, 165, 250, 0.04)',
                        border: '1px solid rgba(96, 165, 250, 0.18)',
                    }}>
                        <div style={{ fontSize: 12, fontWeight: 700, letterSpacing: 0.4, marginBottom: 8 }}>
                            Backend Feature Matrix
                            <span style={{
                                marginLeft: 8, fontSize: 10, fontWeight: 600,
                                padding: '2px 6px', borderRadius: 4,
                                background: 'rgba(96, 165, 250, 0.18)', color: '#60a5fa',
                                textTransform: 'uppercase', letterSpacing: 0.5,
                            }}>
                                active: {activeBackend}
                            </span>
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.7, marginBottom: 6 }}>
                            Which Kortex features apply to which inference path. "●" = active for that backend
                            given current settings; "○" = available but disabled; "–" = N/A.
                            The currently-active column is highlighted.
                        </div>
                        <div style={{
                            display: 'grid',
                            gridTemplateColumns: '1.6fr 1fr 1fr 1fr',
                            gap: 4,
                            fontSize: 11,
                            fontFamily: 'monospace',
                        }}>
                            <div style={{ opacity: 0.6, fontWeight: 700 }}>feature</div>
                            <div style={{
                                textAlign: 'center', fontWeight: 700, opacity: colIndex === 0 ? 1 : 0.5,
                                color: colIndex === 0 ? '#60a5fa' : undefined,
                            }}>Ollama</div>
                            <div style={{
                                textAlign: 'center', fontWeight: 700, opacity: colIndex === 1 ? 1 : 0.5,
                                color: colIndex === 1 ? '#60a5fa' : undefined,
                            }}>llama.cpp+KDKVC</div>
                            <div style={{
                                textAlign: 'center', fontWeight: 700, opacity: colIndex === 2 ? 1 : 0.5,
                                color: colIndex === 2 ? '#60a5fa' : undefined,
                            }}>Cloud (Claude/Gemini/OpenAI)</div>
                            {rows.map((r) => (
                                <React.Fragment key={r.label}>
                                    <div style={{ opacity: 0.85 }}>{r.label}</div>
                                    {r.cells.map((c, i) => (
                                        <div key={i} style={{
                                            textAlign: 'center',
                                            color: cellColor(c),
                                            opacity: colIndex === i ? 1 : 0.55,
                                            fontWeight: colIndex === i ? 700 : 400,
                                        }}>
                                            {cellLabel(c)}
                                        </div>
                                    ))}
                                </React.Fragment>
                            ))}
                        </div>
                        {activeBackend === 'ollama' && (
                            <div style={{ fontSize: 10, opacity: 0.75, marginTop: 8, lineHeight: 1.5 }}>
                                You're on Ollama. CCET, telemetry, and .aim gist injection work today.
                                For KDKVC + GAC, launch the Kortex stack above and point your agent
                                model at <code>llama.cpp</code> (e.g. pick a GGUF model in the GAC section).
                                An Ollama-shaped shim in front of KDKVC is on the roadmap.
                            </div>
                        )}
                        {activeBackend === 'llama.cpp' && (
                            <div style={{ fontSize: 10, opacity: 0.75, marginTop: 8 }}>
                                You're on the full Kortex stack: GAC tier placement + KDKVC disk
                                cache + CCET routing + .aim VFS. Everything applies.
                            </div>
                        )}
                        {(activeBackend === 'anthropic' || activeBackend === 'google' || activeBackend === 'openai') && (
                            <div style={{ fontSize: 10, opacity: 0.75, marginTop: 8 }}>
                                You're on a cloud provider. CCET routing still saves you money by
                                trimming prompts before they go out, but KDKVC/GAC are local-only.
                            </div>
                        )}
                    </div>
                );
            })()}

            {err && (
                <div style={{
                    background: 'rgba(220, 80, 80, 0.10)',
                    color: '#ff8080',
                    padding: '8px 10px',
                    borderRadius: 4,
                    fontSize: 11,
                    marginBottom: 8,
                    fontFamily: 'monospace',
                }}>
                    {err}
                </div>
            )}

            {/* ── Live throughput card ──────────────────────────────────── */}
            <div style={{
                marginTop: 12,
                padding: 14,
                background: 'linear-gradient(135deg, rgba(74, 222, 128, 0.05), rgba(74, 222, 128, 0.02))',
                border: '1px solid rgba(74, 222, 128, 0.25)',
                borderRadius: 6,
            }}>
                <div style={{
                    display: 'flex', alignItems: 'baseline', justifyContent: 'space-between',
                    marginBottom: 8,
                }}>
                    <div style={{ fontSize: 12, fontWeight: 700, letterSpacing: 0.4 }}>
                        Live Throughput
                    </div>
                    <div style={{ fontSize: 10, opacity: 0.65 }}>
                        rolling window · {telemetry?.sample_size ?? 0} samples
                    </div>
                </div>

                <div style={{
                    display: 'grid', gridTemplateColumns: '1fr 1fr 1fr', gap: 12,
                    alignItems: 'baseline',
                }}>
                    <div>
                        <div style={{ fontSize: 28, fontWeight: 800, color: '#4ade80', lineHeight: 1 }}>
                            {telemetry && telemetry.current_tps > 0
                                ? telemetry.current_tps.toFixed(1)
                                : '--'}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.7, marginTop: 2 }}>
                            tok/s (current)
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 18, fontWeight: 700, lineHeight: 1.2 }}>
                            {telemetry && telemetry.avg_tps > 0 ? telemetry.avg_tps.toFixed(1) : '--'}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.7 }}>
                            tok/s (avg)
                        </div>
                    </div>
                    <div>
                        <div style={{ fontSize: 18, fontWeight: 700, lineHeight: 1.2 }}>
                            {telemetry && Number.isFinite(telemetry.avg_prefill_tps) && telemetry.avg_prefill_tps > 0
                                ? telemetry.avg_prefill_tps.toFixed(0)
                                : '--'}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.7 }}>
                            prefill tok/s
                        </div>
                    </div>
                </div>

                {telemetry ? (
                    <div style={{
                        marginTop: 10,
                        fontFamily: 'monospace',
                        fontSize: 11,
                        opacity: 0.85,
                        lineHeight: 1.5,
                    }}>
                        <div>
                            last: {telemetry.last_output_tokens} out / {telemetry.last_input_tokens} in
                            · {(telemetry.last_wall_clock_ms / 1000).toFixed(2)}s
                            {telemetry.last_prefill_ms > 0
                                ? ` (ttft ${telemetry.last_prefill_ms}ms)`
                                : ''}
                            · {telemetry.last_backend || 'unknown'}
                        </div>
                        <div style={{ marginTop: 2 }}>
                            cache: {(telemetry.cache_hit_rate * 100).toFixed(0)}% hit rate
                            {telemetry.total_tokens_skipped > 0
                                ? ` · ${telemetry.total_tokens_skipped.toLocaleString()} tokens skipped`
                                : ''}
                            {telemetry.last_cache_hit ? ' · LAST WAS HIT' : ''}
                        </div>
                        {telemetry.last_model_id && (
                            <div style={{ marginTop: 2, opacity: 0.7 }}>
                                model: {telemetry.last_model_id}
                            </div>
                        )}
                    </div>
                ) : (
                    <div style={{ fontSize: 11, opacity: 0.6, marginTop: 8 }}>
                        Run a chat request to populate. Counts come from
                        llama-server's <code>timings</code> chunk and Ollama's
                        <code> eval_count</code> when available, falling back to
                        a char/4 approximation otherwise.
                    </div>
                )}

                {cacheStatus && (cacheStatus.model_id || cacheStatus.quant_signature) && (
                    <div style={{
                        marginTop: 10,
                        padding: '4px 8px',
                        background: 'rgba(74, 222, 128, 0.08)',
                        borderRadius: 4,
                        fontSize: 10,
                        fontFamily: 'monospace',
                        opacity: 0.85,
                    }}>
                        cache bound to: {cacheStatus.model_id || '(unknown)'}
                        {cacheStatus.quant_signature ? ` · ${cacheStatus.quant_signature}` : ''}
                        {cacheStatus.tokenizer_hash
                            ? ` · tok-sha ${cacheStatus.tokenizer_hash.slice(0, 8)}`
                            : ''}
                    </div>
                )}
            </div>

            {/* ── GAC section ───────────────────────────────────────────── */}
            <div style={sectionStyle}>
                <div style={sectionTitleStyle}>GAC — geometry-aware weight placement</div>

                <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 8 }}>
                    <input
                        type="checkbox"
                        checked={kortexGacEnabled}
                        onChange={e => setKortexGacEnabled(e.target.checked)}
                    />
                    Enable Kortex GAC scheduler when launching llama-server
                </label>

                <label style={labelStyle}>Model GGUF path</label>
                <input
                    style={inputStyle}
                    value={llamaCppModelPath}
                    onChange={e => setLlamaCppModelPath(e.target.value)}
                    placeholder="C:\\models\\qwen2.5-32b-coder.Q4_K_M.gguf"
                />
                <div style={{ marginTop: 8, display: 'flex', flexWrap: 'wrap', gap: 8, alignItems: 'center' }}>
                    <input
                        style={{ ...inputStyle, flex: '1 1 200px', maxWidth: 420 }}
                        value={ollamaTagForGguf}
                        onChange={e => setOllamaTagForGguf(e.target.value)}
                        placeholder="Ollama tag (e.g. llama3.2:latest) — empty uses agent model"
                    />
                    <button
                        type="button"
                        style={btnStyle('secondary')}
                        disabled={!!busy}
                        onClick={async () => {
                            setErr('');
                            const tag = (ollamaTagForGguf.trim() || parseOllamaModelTag(agentModel)).trim();
                            if (!tag) {
                                setErr('Set an Ollama model in the agent picker or enter a tag above.');
                                return;
                            }
                            setBusy('Resolving GGUF from Ollama…');
                            try {
                                const p = await resolveOllamaModelToGguf(tag);
                                if (!p) {
                                    setErr('No local .gguf path found for this model (check `ollama show` / Modelfile FROM).');
                                    return;
                                }
                                setLlamaCppModelPath(p);
                            } catch (e: unknown) {
                                setErr(e instanceof Error ? e.message : String(e));
                            } finally {
                                setBusy('');
                            }
                        }}
                    >
                        Resolve GGUF from Ollama
                    </button>
                </div>
                {profilePath && (
                    <div style={{ fontSize: 10, opacity: 0.6, marginTop: 4, fontFamily: 'monospace' }}>
                        profile → {profilePath}
                    </div>
                )}

                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: 12, marginTop: 8 }}>
                    <div>
                        <label style={labelStyle}>VRAM total (MB)</label>
                        <input
                            style={inputStyle}
                            type="number"
                            min={512}
                            max={131072}
                            value={kortexVramTotalMb}
                            onChange={e => setKortexVramTotalMb(parseInt(e.target.value || '0', 10))}
                        />
                    </div>
                    <div>
                        <label style={labelStyle}>θ (retrieval threshold) — {kortexTheta.toFixed(2)}</label>
                        <input
                            style={inputStyle}
                            type="range"
                            min={0.5}
                            max={0.99}
                            step={0.01}
                            value={kortexTheta}
                            onChange={e => setKortexTheta(parseFloat(e.target.value))}
                        />
                    </div>
                </div>

                <div style={{ display: 'grid', gridTemplateColumns: '1fr 2fr', gap: 12, marginTop: 8 }}>
                    <div>
                        <label style={labelStyle}>GPU backend</label>
                        <select
                            style={inputStyle}
                            value={kortexBackend}
                            onChange={e => setKortexBackend(e.target.value as Backend)}
                        >
                            <option value="vulkan">Vulkan (RX 580 / generic AMD)</option>
                            <option value="cuda">CUDA (NVIDIA)</option>
                            <option value="rocm">ROCm (AMD)</option>
                            <option value="metal">Metal (Apple)</option>
                            <option value="sycl">SYCL (Intel)</option>
                        </select>
                    </div>
                    <div>
                        <label style={labelStyle}>llama-server binary (optional)</label>
                        <input
                            style={inputStyle}
                            value={kortexServerBinary}
                            onChange={e => setKortexServerBinary(e.target.value)}
                            placeholder="auto-detect from PATH"
                        />
                    </div>
                </div>

                <div style={{ marginTop: 8 }}>
                    <button style={btnStyle('secondary')} onClick={onProfile} disabled={!!busy}>
                        Profile model
                    </button>
                    <button style={btnStyle('secondary')} onClick={onPreviewPlan} disabled={!!busy}>
                        Preview plan
                    </button>
                    <button style={btnStyle('primary')} onClick={onStart} disabled={!!busy}>
                        Start Kortex stack
                    </button>
                    <button style={btnStyle('danger')} onClick={onStop} disabled={!!busy || (!gacRunning && !kvRunning)}>
                        Stop
                    </button>
                </div>

                {plan && (
                    <div style={{ fontSize: 11, opacity: 0.85, marginTop: 8, fontFamily: 'monospace' }}>
                        plan: {summarizePlan(plan)}
                    </div>
                )}
            </div>

            {/* ── KV cache section ──────────────────────────────────────── */}
            <div style={sectionStyle}>
                <div style={sectionTitleStyle}>KV Cache — disk-persistent prefix reuse (ds4-style)</div>

                <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 8 }}>
                    <input
                        type="checkbox"
                        checked={kvCacheEnabled}
                        onChange={e => setKvCacheEnabled(e.target.checked)}
                    />
                    Boot the prefix-cache proxy in front of llama-server
                </label>

                <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 8 }}>
                    Each request's tokenised prefix is hashed to SHA-256; matching prefixes
                    skip the prefill entirely on subsequent calls — even after llama-server
                    restarts. Coding agents that resend 25 K-token system prompts win the most.
                </div>

                <label style={labelStyle}>Base directory (index/ + slots/ live underneath)</label>
                <input
                    style={inputStyle}
                    value={kvCacheBaseDir}
                    onChange={e => setKvCacheBaseDir(e.target.value)}
                    placeholder="(default) USERPROFILE/.kortex/kvcache"
                />

                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 12, marginTop: 8 }}>
                    <div>
                        <label style={labelStyle}>Max size (GB)</label>
                        <input
                            style={inputStyle}
                            type="number"
                            min={1}
                            max={1024}
                            value={Math.round(kvCacheMaxBytes / (1024 * 1024 * 1024))}
                            onChange={e => setKvCacheMaxBytes(parseInt(e.target.value || '0', 10) * 1024 * 1024 * 1024)}
                        />
                    </div>
                    <div>
                        <label style={labelStyle}>Proxy port</label>
                        <input
                            style={inputStyle}
                            type="number"
                            min={1024}
                            max={65535}
                            value={kvCacheProxyPort}
                            onChange={e => setKvCacheProxyPort(parseInt(e.target.value || '0', 10))}
                        />
                    </div>
                </div>

                <div style={{ marginTop: 8 }}>
                    <button style={btnStyle('secondary')} onClick={refreshKvCacheStats} disabled={!!busy}>
                        Refresh stats
                    </button>
                    <button style={btnStyle('danger')} onClick={onClearCache} disabled={!!busy || !kvRunning}>
                        Clear cache
                    </button>
                </div>

                {kvCacheStats && (
                    <div style={{
                        marginTop: 12,
                        padding: '8px 10px',
                        borderRadius: 4,
                        background: 'rgba(74, 222, 128, 0.06)',
                        fontFamily: 'monospace',
                        fontSize: 11,
                        lineHeight: 1.6,
                    }}>
                        {summarizeKvCache(kvCacheStats)}
                        <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 4, marginTop: 6, opacity: 0.85 }}>
                            <div>hits: {kvCacheStats.hits}</div>
                            <div>misses: {kvCacheStats.misses}</div>
                            <div>saves: {kvCacheStats.saves}</div>
                            <div>evictions: {kvCacheStats.evictions}</div>
                            <div>tokens skipped: {kvCacheStats.tokens_skipped}</div>
                            <div>size: {fmtBytes(kvCacheStats.total_bytes)}</div>
                        </div>
                    </div>
                )}
            </div>

            {/* ── CCET section ──────────────────────────────────────────── */}
            <div style={sectionStyle}>
                <div style={sectionTitleStyle}>CCET — context-compute efficiency router</div>

                <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 8 }}>
                    <input
                        type="checkbox"
                        checked={ccetEnabled}
                        onChange={e => setCcetEnabled(e.target.checked)}
                    />
                    Apply heuristic token routing before sending prompts
                </label>

                <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 8 }}>
                    Segments scoring above τ_compress are kept verbatim, between τ_skip and τ_compress
                    are replaced with a stub, below τ_skip are dropped. A skip-fraction cap protects
                    against pathological "drop everything" cases on low-novelty prompts.
                </div>

                <div style={{ display: 'grid', gridTemplateColumns: 'repeat(3, 1fr)', gap: 12, marginTop: 4 }}>
                    <div>
                        <label style={labelStyle}>τ_skip — {ccetTauSkip.toFixed(2)}</label>
                        <input
                            style={inputStyle}
                            type="range"
                            min={0}
                            max={0.5}
                            step={0.01}
                            value={ccetTauSkip}
                            onChange={e => setCcetTauSkip(parseFloat(e.target.value))}
                        />
                    </div>
                    <div>
                        <label style={labelStyle}>τ_compress — {ccetTauCompress.toFixed(2)}</label>
                        <input
                            style={inputStyle}
                            type="range"
                            min={0.1}
                            max={1.0}
                            step={0.01}
                            value={ccetTauCompress}
                            onChange={e => setCcetTauCompress(parseFloat(e.target.value))}
                        />
                    </div>
                    <div>
                        <label style={labelStyle}>max skip frac — {ccetMaxSkipFraction.toFixed(2)}</label>
                        <input
                            style={inputStyle}
                            type="range"
                            min={0}
                            max={0.9}
                            step={0.01}
                            value={ccetMaxSkipFraction}
                            onChange={e => setCcetMaxSkipFraction(parseFloat(e.target.value))}
                        />
                    </div>
                </div>

                <div style={{ marginTop: 8 }}>
                    <button style={btnStyle('secondary')} onClick={refreshCcetEfficiency}>
                        Refresh η
                    </button>
                </div>

                {ccetEfficiency ? (
                    <div style={{
                        marginTop: 12,
                        padding: '8px 10px',
                        borderRadius: 4,
                        background: 'rgba(251, 191, 36, 0.06)',
                        fontFamily: 'monospace',
                        fontSize: 11,
                        lineHeight: 1.6,
                    }}>
                        η = {ccetEfficiency.avg_eta.toFixed(3)}
                        {' · '}
                        avg saved = {(ccetEfficiency.avg_saved_fraction * 100).toFixed(1)}%
                        {' · '}
                        skipped segments (window): {ccetEfficiency.total_skipped_segments}
                        {' · '}
                        n = {ccetEfficiency.sample_size}
                    </div>
                ) : (
                    <div style={{ fontSize: 11, opacity: 0.6, marginTop: 8 }}>
                        No requests yet. η will populate after the first chat round-trip.
                    </div>
                )}
            </div>

            {/* ── Neural .aim VFS ───────────────────────────────────────── */}
            <div style={sectionStyle}>
                <div style={sectionTitleStyle}>
                    Neural VFS (.aim) — durable telemetry + bound-model record
                </div>

                <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 8 }}>
                    Every completion is appended to{' '}
                    <code>~/.kortex/telemetry.aim</code>. Lifetime tallies survive
                    the rolling window — even after old samples roll off, the
                    "you've routed 1.4M tokens through Kortex" number keeps growing.
                    The last KDKVC bound-model stamp is mirrored here so the IDE
                    can recover the cache identity even when the proxy is down.
                </div>

                {aim ? (
                    <>
                        <div style={{
                            padding: '8px 10px',
                            borderRadius: 4,
                            background: 'rgba(96, 165, 250, 0.06)',
                            border: '1px solid rgba(96, 165, 250, 0.18)',
                            fontFamily: 'monospace',
                            fontSize: 11,
                            lineHeight: 1.6,
                        }}>
                            {summarizeAimTelemetry(aim)}
                        </div>

                        <div style={{
                            display: 'grid',
                            gridTemplateColumns: 'repeat(3, 1fr)',
                            gap: 6,
                            marginTop: 8,
                            fontFamily: 'monospace',
                            fontSize: 10,
                            opacity: 0.85,
                        }}>
                            <div>schema: {aim.schema}</div>
                            <div>capacity: {aim.capacity}</div>
                            <div>in-window: {aim.samples.length}</div>
                            <div>lifetime out: {aim.lifetime_output_tokens.toLocaleString()}</div>
                            <div>lifetime in: {aim.lifetime_input_tokens.toLocaleString()}</div>
                            <div>lifetime skipped: {aim.lifetime_tokens_skipped.toLocaleString()}</div>
                            <div>cache hits: {aim.lifetime_cache_hits.toLocaleString()}</div>
                            <div>last quant: {aim.last_quant_signature ?? '—'}</div>
                            <div>tok-hash: {aim.last_tokenizer_hash ? aim.last_tokenizer_hash.slice(0, 8) : '—'}</div>
                        </div>
                    </>
                ) : (
                    <div style={{ fontSize: 11, opacity: 0.6 }}>
                        Loading neural VFS state…
                    </div>
                )}

                <div style={{ marginTop: 8 }}>
                    <button style={btnStyle('secondary')} onClick={onAimFlush} disabled={!!busy}>
                        Flush to disk
                    </button>
                    <button style={btnStyle('danger')} onClick={onAimClearSamples} disabled={!!busy}>
                        Clear samples (keep lifetime tallies)
                    </button>
                </div>
            </div>
        </div>
    );
};

export default KortexInferencePanel;
