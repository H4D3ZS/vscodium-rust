import { useState, useEffect, useCallback } from 'react';
import { invoke } from '../tauri_bridge';
import { useStore } from '../store';
import {
    getKvCacheStatus, startKvCache, stopKvCache, getKvCacheStats,
    makeKvCacheOptions, resolveKvCacheBaseDir, summarizeKvCache,
    type RunningCacheInfo, type KvCacheStats,
} from '../kortex/kvcache-orchestrator';

interface SvcStatus { running: boolean; port: number | null }
interface RetrievalStatus extends SvcStatus {
    catalog_active?: boolean;
    chunks?: number;
    catalog_dir?: string | null;
}

/**
 * Kortex services — the three local processes the README describes.
 *
 * Port map: `:1536` AIM retrieval proxy (opt-in, experimental) ·
 * `:1537` KV-slot cache (the "route through kortex" path) · `:1538` aim-vfs.
 */
export function KortexPanel() {
    const inferenceUrl = useStore(s => s.inferenceUrl);
    const setInferenceUrl = useStore(s => s.setInferenceUrl);
    const lemonadeUrl = useStore(s => s.lemonadeUrl);
    const setLemonadeUrl = useStore(s => s.setLemonadeUrl);
    const kvProxyPort = useStore(s => s.kvCacheProxyPort) || 1537;

    // Point every routing path (store URLs + the Rust engine base) at `url`.
    const repointInference = useCallback(async (url: string) => {
        const clean = url.replace(/\/$/, '');
        setInferenceUrl?.(clean);
        setLemonadeUrl?.(clean);
        try { await invoke('set_lemonade_url', { url: clean }); } catch { /* engine offline */ }
    }, [setInferenceUrl, setLemonadeUrl]);

    const [retrieval, setRetrieval] = useState<RetrievalStatus | null>(null);
    const [vfs, setVfs] = useState<SvcStatus | null>(null);
    const [kv, setKv] = useState<RunningCacheInfo | null>(null);
    const [kvStats, setKvStats] = useState<KvCacheStats | null>(null);
    const [busy, setBusy] = useState('');
    const [error, setError] = useState('');
    const [indexMsg, setIndexMsg] = useState('');

    useEffect(() => {
        let un: (() => void) | undefined;
        import('@tauri-apps/api/event').then(({ listen }) =>
            listen<Record<string, unknown>>('aim-index-progress', (e) => {
                const p = e.payload;
                if (p.status === 'indexing') setIndexMsg(`indexing… ${p.files ?? 0} files, ${p.chunks ?? 0} chunks`);
                else if (p.status === 'complete') setIndexMsg(`indexed ${p.files_indexed ?? 0} files in ${Math.round(Number(p.elapsed_secs ?? 0))}s`);
                else if (p.status === 'started') setIndexMsg('building catalog…');
                else if (p.status === 'error') setIndexMsg(`catalog build failed: ${p.error ?? ''}`);
            }).then((u) => { un = u; }),
        );
        return () => { un?.(); };
    }, []);

    const refresh = useCallback(async () => {
        try {
            const [r, v, k] = await Promise.all([
                invoke<RetrievalStatus>('kortex_retrieval_status').catch(() => null),
                invoke<SvcStatus>('kortex_vfs_status').catch(() => null),
                getKvCacheStatus().catch(() => null),
            ]);
            setRetrieval(r);
            setVfs(v);
            setKv(k);
            if (k) setKvStats(await getKvCacheStats().catch(() => null));
            else setKvStats(null);
        } catch { /* ignore */ }
    }, []);

    useEffect(() => {
        void refresh();
        const t = setInterval(refresh, 5000);
        return () => clearInterval(t);
    }, [refresh]);

    const run = async (label: string, fn: () => Promise<unknown>) => {
        setBusy(label); setError('');
        try { await fn(); await refresh(); }
        catch (e) { setError(String(e)); }
        finally { setBusy(''); }
    };

    const startKv = () => run('kv', async () => {
        const base = await resolveKvCacheBaseDir();
        const upstream = (lemonadeUrl || inferenceUrl || 'http://127.0.0.1:13305').replace(/\/$/, '');
        if (upstream.includes(`:${kvProxyPort}`)) throw new Error('inference URL already points at the proxy port — set a real backend URL first');
        const opts = makeKvCacheOptions(base, { upstream_url: upstream, proxy_port: kvProxyPort });
        const port = await startKvCache(opts).catch((e) => {
            if (String(e).includes('already running')) return kvProxyPort;
            throw e;
        });
        try { localStorage.setItem('kvcache.upstream', upstream); } catch { /* */ }
        // This is the "prompts route through kortex" path: point inference at the proxy.
        await repointInference(`http://127.0.0.1:${port}`);
    });

    const stopKv = () => run('kv', async () => {
        let upstream = kv?.upstream_url || '';
        try { upstream = upstream || (localStorage.getItem('kvcache.upstream') || ''); } catch { /* */ }
        upstream = upstream || 'http://127.0.0.1:13305';
        await stopKvCache();
        if (inferenceUrl.includes(`:${kvProxyPort}`)) await repointInference(upstream);
    });

    const badge = (on: boolean): React.CSSProperties => ({
        display: 'inline-block', width: 8, height: 8, borderRadius: '50%',
        background: on ? '#4ec9b0' : '#808080', marginRight: 6, flexShrink: 0,
    });
    const btn: React.CSSProperties = {
        fontSize: 11, padding: '3px 9px', borderRadius: 3, cursor: busy ? 'wait' : 'pointer',
        background: 'transparent', color: 'var(--vscode-descriptionForeground)',
        border: '1px solid var(--vscode-panel-border)',
    };
    const sub: React.CSSProperties = { fontSize: 10, color: 'var(--vscode-descriptionForeground)', margin: '2px 0 8px 14px' };

    const svc = (
        on: boolean, name: string, port: number | string | null,
        note: string, actions: React.ReactNode,
    ) => (
        <div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                <span style={badge(on)} />
                <span style={{ fontSize: 11, flex: 1, minWidth: 0 }}>
                    {name} {on && port ? `(:${port})` : ''}
                </span>
                {actions}
            </div>
            <div style={sub}>{note}</div>
        </div>
    );

    return (
        <div style={{ display: 'flex', flexDirection: 'column', padding: 8, gap: 4, color: 'var(--vscode-foreground)' }}>
            <div style={{ fontSize: 12, fontWeight: 600, marginBottom: 2 }}>Kortex Services</div>
            {error && <div style={{ fontSize: 11, color: 'var(--vscode-errorForeground)', marginBottom: 4 }}>{error}</div>}

            {svc(
                !!kv, 'KV-slot cache', kv?.proxy_url?.split(':').pop() ?? kvProxyPort,
                kv
                    ? (kvStats ? summarizeKvCache(kvStats) : `proxy → ${kv.upstream_url}`)
                    : 'Prompts route through the proxy; repeated prefixes skip prefill. Opt-in.',
                kv
                    ? <button style={btn} onClick={stopKv} disabled={!!busy}>Stop</button>
                    : <button style={btn} onClick={startKv} disabled={!!busy}>{busy === 'kv' ? '…' : 'Start'}</button>,
            )}

            {svc(
                !!vfs?.running, 'VFS daemon', vfs?.port ?? null,
                'Auto-starts on boot. Manages .aim memory + file watching.',
                vfs?.running
                    ? <button style={btn} onClick={() => run('vfs', () => invoke('kortex_vfs_stop'))} disabled={!!busy}>Stop</button>
                    : <button style={btn} onClick={() => run('vfs', () => invoke('kortex_vfs_start'))} disabled={!!busy}>{busy === 'vfs' ? '…' : 'Start'}</button>,
            )}

            {svc(
                !!retrieval?.running, 'AIM retrieval proxy', retrieval?.port ?? null,
                retrieval?.running
                    ? (retrieval.catalog_active
                        ? `Active — ${retrieval.chunks ?? 0} chunks. Injects only the relevant .aim slices, shrinking every prompt.`
                        : `Pass-through — no usable catalog. Hit "Rebuild".`)
                    : (indexMsg || 'Injects relevant .aim workspace context into prompts so the model gets less, better context. Builds a dense catalog from the workspace on first Start.'),
                <div style={{ display: 'flex', gap: 6 }}>
                    {retrieval?.running && (
                        <button style={btn} onClick={() => run('ret', () => invoke('kortex_retrieval_start', { rebuild: true }))} disabled={!!busy} title="Re-index the workspace">
                            {busy === 'ret' ? '…' : 'Rebuild'}
                        </button>
                    )}
                    {retrieval?.running
                        ? <button style={btn} onClick={() => run('ret', () => invoke('kortex_retrieval_stop'))} disabled={!!busy}>Stop</button>
                        : <button style={btn} onClick={() => run('ret', () => invoke('kortex_retrieval_start'))} disabled={!!busy}>{busy === 'ret' ? '…' : 'Start'}</button>}
                </div>,
            )}
        </div>
    );
}
