import React, { useCallback, useEffect, useRef, useState } from 'react';
import { useStore } from '../../store';
import { SettingsRow } from './panels';

/**
 * Kortex Local Inference — the "one button" demo flow.
 *
 *   pick GGUF + server binary
 *        -> kortex_gac  : profile -> geometry-aware tier plan -> launch llama-server
 *        -> kortex_kvcache : KV-slot prefix-cache proxy in front of it
 *        -> repoint the IDE's inference backend at the proxy
 *
 * Built for the Escha W2 35B-A3B Q2_0_ROCMFPX model on an RX 9060 XT via a
 * ROCmFPX-built llama-server (see scripts/build-rocmfpx-windows.ps1). Any GGUF +
 * any llama-server works too; the ROCmFPX pieces are only needed for the
 * Q2_0_ROCMFPX quant.
 */

type Phase = 'idle' | 'profiling' | 'launching' | 'caching' | 'running' | 'error';

const ESCHA_HINT =
    '%USERPROFILE%\\.cache\\huggingface\\hub\\models--cafonez--Escha-W2-35B-A3B-ROCmFP2' +
    '\\snapshots\\*\\Qwen3.6-35B-A3B-Escha-W2-ROCmFP2.gguf';

const inputStyle: React.CSSProperties = {
    fontSize: 12, padding: '4px 8px', flex: 1, minWidth: 0,
    background: 'var(--vscode-input-background)',
    color: 'var(--vscode-input-foreground)',
    border: '1px solid var(--vscode-input-border)',
};

async function pickFile(filters: { name: string; extensions: string[] }[]): Promise<string | null> {
    try {
        const { open } = await import('@tauri-apps/plugin-dialog');
        const picked = await open({ multiple: false, directory: false, filters });
        return typeof picked === 'string' ? picked : null;
    } catch {
        return null;
    }
}

async function defaultBaseDir(): Promise<string> {
    try {
        const { appLocalDataDir, join } = await import('@tauri-apps/api/path');
        return await join(await appLocalDataDir(), 'kortex-kv');
    } catch {
        return '';
    }
}

export function KortexLocalInferencePanel() {
    const serverBinary = useStore(s => s.kortexServerBinary);
    const setServerBinary = useStore(s => (s as any).setKortexServerBinary);
    const vramMb = useStore(s => s.kortexVramTotalMb);
    const setVramMb = useStore(s => (s as any).setKortexVramTotalMb);
    const baseDir = useStore(s => s.kvCacheBaseDir);
    const setBaseDir = useStore(s => (s as any).setKvCacheBaseDir);
    const proxyPort = useStore(s => s.kvCacheProxyPort);
    const kvStats = useStore(s => s.kvCacheStats);
    const refreshKvStats = useStore(s => (s as any).refreshKvCacheStats);
    const setInferenceBackend = useStore(s => (s as any).setInferenceBackend);
    const setLlamaCppUrl = useStore(s => (s as any).setLlamaCppUrl);

    const [modelPath, setModelPath] = useState<string>(() => {
        try { return localStorage.getItem('kortex.localModelPath') || ''; } catch { return ''; }
    });
    const [phase, setPhase] = useState<Phase>('idle');
    const [msg, setMsg] = useState('');
    const [planLine, setPlanLine] = useState('');
    const [statsLine, setStatsLine] = useState('');
    const pollRef = useRef<ReturnType<typeof setInterval> | null>(null);

    const busy = phase === 'profiling' || phase === 'launching' || phase === 'caching';

    // Resume UI state if the proxy is already up (panel remounts on tab switch).
    useEffect(() => {
        let cancelled = false;
        (async () => {
            try {
                const { getKvCacheStatus } = await import('../../kortex/kvcache-orchestrator');
                const info = await getKvCacheStatus();
                if (!cancelled && info) {
                    setPhase('running');
                    setMsg(`proxy :${proxyPort} -> ${info.upstream_url}  (${info.model_id || 'model'})`);
                }
            } catch { /* not running */ }
        })();
        return () => { cancelled = true; };
    }, [proxyPort]);

    useEffect(() => {
        if (phase !== 'running') {
            if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; }
            return;
        }
        const tick = async () => {
            try {
                await refreshKvStats?.();
                const { summarizeKvCache } = await import('../../kortex/kvcache-orchestrator');
                const s = useStore.getState().kvCacheStats;
                if (s) setStatsLine(summarizeKvCache(s));
            } catch { /* ignore */ }
        };
        void tick();
        pollRef.current = setInterval(tick, 3000);
        return () => { if (pollRef.current) { clearInterval(pollRef.current); pollRef.current = null; } };
    }, [phase, refreshKvStats]);

    const launch = useCallback(async () => {
        if (!modelPath) { setPhase('error'); setMsg('Pick a GGUF model first.'); return; }
        setPhase('profiling'); setMsg('Profiling model geometry…'); setPlanLine(''); setStatsLine('');
        try {
            try { localStorage.setItem('kortex.localModelPath', modelPath); } catch { /* ignore */ }

            let base = baseDir || (await defaultBaseDir());
            if (!base) throw new Error('Could not resolve a cache directory.');
            if (base !== baseDir) setBaseDir?.(base);
            const slotDir = `${base}/slots`;

            const gac = await import('../../kortex/gac-orchestrator');
            const kv = await import('../../kortex/kvcache-orchestrator');

            setPhase('launching'); setMsg('Planning tensor tiers and launching llama-server…');
            const boot = await gac.startKortexInference({
                model_path: modelPath,
                vram_total_mb: vramMb || 16384,
                backend: 'rocm',
                refresh_profile: false,
                launch: {
                    server_binary: serverBinary || undefined,
                    port: 8081,
                    ctx_size: 8192,
                    slot_save_path: slotDir,
                    wait_healthy_secs: 120,
                },
            });
            setPlanLine(gac.summarizePlan(boot.plan));

            setPhase('caching'); setMsg('Starting KV-slot cache proxy…');
            const opts = kv.makeKvCacheOptions(base, {
                upstream_url: boot.base_url,
                proxy_port: proxyPort || 1537,
                max_bytes: (vramMb && vramMb > 0 ? vramMb : 16384) * 1024 * 1024,
            });
            const boundPort = await kv.startKvCache(opts);

            const proxyUrl = `http://127.0.0.1:${boundPort}`;
            setLlamaCppUrl?.(proxyUrl);
            setInferenceBackend?.('llama-cpp');

            setPhase('running');
            setMsg(`Live. IDE inference -> ${proxyUrl} -> llama-server ${boot.base_url}`);
        } catch (e) {
            setPhase('error');
            setMsg(String((e as any)?.message ?? e));
        }
    }, [modelPath, baseDir, vramMb, serverBinary, proxyPort, setBaseDir, setLlamaCppUrl, setInferenceBackend]);

    const stop = useCallback(async () => {
        setMsg('Stopping…');
        try {
            const kv = await import('../../kortex/kvcache-orchestrator');
            const gac = await import('../../kortex/gac-orchestrator');
            await kv.stopKvCache().catch(() => {});
            await gac.stopServer().catch(() => {});
        } finally {
            setPhase('idle'); setMsg(''); setPlanLine(''); setStatsLine('');
        }
    }, []);

    const dot = phase === 'running' ? '#4ec9b0' : phase === 'error' ? '#f7768e' : busy ? '#e5a00d' : '#808080';

    return (
        <div className="settings-card">
            <div className="settings-card-title">Local Inference (Kortex + ROCmFPX)</div>
            <p style={{ fontSize: 11, opacity: 0.6, margin: '0 0 10px' }}>
                Profile → geometry-aware tier plan → launch llama-server → KV-slot prefix cache →
                repoint the IDE. Needs a ROCmFPX-built llama-server for the Escha Q2_0_ROCMFPX quant
                (<code>scripts/build-rocmfpx-windows.ps1</code>).
            </p>

            <SettingsRow
                label="Model (GGUF)"
                description={modelPath || `e.g. ${ESCHA_HINT}`}
                control={
                    <div style={{ display: 'flex', gap: 6, width: 320 }}>
                        <input style={inputStyle} value={modelPath}
                            placeholder="path to .gguf"
                            onChange={e => setModelPath(e.target.value)} />
                        <button type="button" className="settings-button" disabled={busy}
                            onClick={async () => {
                                const p = await pickFile([{ name: 'GGUF', extensions: ['gguf'] }]);
                                if (p) setModelPath(p);
                            }}>Browse</button>
                    </div>
                }
            />

            <SettingsRow
                label="Server binary"
                description={serverBinary || 'blank = resolve llama-server from PATH / bundled binaries'}
                control={
                    <div style={{ display: 'flex', gap: 6, width: 320 }}>
                        <input style={inputStyle} value={serverBinary}
                            placeholder="…\\src-tauri\\binaries\\rocmfpx\\llama-server.exe"
                            onChange={e => setServerBinary?.(e.target.value)} />
                        <button type="button" className="settings-button" disabled={busy}
                            onClick={async () => {
                                const p = await pickFile([{ name: 'Executable', extensions: ['exe', ''] }]);
                                if (p) setServerBinary?.(p);
                            }}>Browse</button>
                    </div>
                }
            />

            <SettingsRow
                label="VRAM budget (MB)"
                description="Tier planner keeps GPU tensors under this; the rest go to system RAM."
                control={
                    <input type="number" min={2048} step={512} style={{ ...inputStyle, flex: 'none', width: 120 }}
                        value={vramMb || 16384}
                        onChange={e => setVramMb?.(parseInt(e.target.value) || 16384)} />
                }
            />

            <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginTop: 10 }}>
                {phase === 'running'
                    ? <button type="button" className="settings-button" onClick={stop}>Stop</button>
                    : <button type="button" className="settings-button success" disabled={busy} onClick={launch}>
                        {busy ? 'Working…' : 'Launch'}
                      </button>}
                <span style={{ display: 'inline-flex', alignItems: 'center', gap: 6, fontSize: 11 }}>
                    <span style={{ width: 8, height: 8, borderRadius: '50%', background: dot, display: 'inline-block' }} />
                    {phase}
                </span>
            </div>

            {msg && (
                <div style={{ fontSize: 11, marginTop: 8, color: phase === 'error' ? 'var(--vscode-errorForeground)' : 'var(--vscode-descriptionForeground)' }}>
                    {msg}
                </div>
            )}
            {planLine && <div style={{ fontSize: 10, marginTop: 6, opacity: 0.7, fontFamily: 'var(--vscode-editor-font-family, monospace)' }}>{planLine}</div>}
            {phase === 'running' && (
                <div style={{ fontSize: 10, marginTop: 4, opacity: 0.8, fontFamily: 'var(--vscode-editor-font-family, monospace)' }}>
                    {statsLine || 'cache: warming up…'}
                </div>
            )}
        </div>
    );
}

export default KortexLocalInferencePanel;
