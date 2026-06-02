import React, { useEffect, useMemo, useState } from 'react';
import { invoke } from '../tauri_bridge';

// Viewer for `.aim` Neural Weight-Map binaries (memmap2). These are NOT text, so
// Monaco can't show them. We call the backend `aim_inspect` which parses the
// header + entries (key / gist / mtime / weight) and render a readable table.

interface AimEntry { key: string; gist: string; mtime: number; weight: number; }
interface AimInspection {
    format?: 'json' | 'aim-binary';
    // binary AIM\x01
    magic_ok?: boolean;
    version?: number;
    written_at?: number;
    entry_count?: number;
    entries?: AimEntry[];
    // JSON kortex memory
    pretty?: string;
    truncated?: boolean;
    entities?: number | null;
    tree_count?: number | null;
    // common
    size_bytes: number;
}

function fmtBytes(n: number): string {
    if (n < 1024) return `${n} B`;
    if (n < 1024 * 1024) return `${(n / 1024).toFixed(1)} KB`;
    return `${(n / 1024 / 1024).toFixed(2)} MB`;
}
function fmtTime(unixSecs: number): string {
    if (!unixSecs) return '—';
    try { return new Date(unixSecs * 1000).toLocaleString(); } catch { return String(unixSecs); }
}

const AimViewer: React.FC<{ path: string }> = ({ path }) => {
    const [data, setData] = useState<AimInspection | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [loading, setLoading] = useState(true);
    const [query, setQuery] = useState('');
    const [sortBy, setSortBy] = useState<'weight' | 'key' | 'mtime'>('weight');

    const load = () => {
        setLoading(true); setError(null);
        invoke<AimInspection>('aim_inspect', { path })
            .then((d) => setData(d))
            .catch((e) => setError(typeof e === 'string' ? e : (e?.message || String(e))))
            .finally(() => setLoading(false));
    };
    useEffect(() => { load(); /* eslint-disable-next-line */ }, [path]);

    const filename = path.replace(/\\/g, '/').split('/').pop() || path;

    const rows = useMemo(() => {
        if (!data || !data.entries) return [];
        const q = query.trim().toLowerCase();
        let r = data.entries;
        if (q) r = r.filter((e) => e.key.toLowerCase().includes(q) || e.gist.toLowerCase().includes(q));
        const s = [...r];
        if (sortBy === 'weight') s.sort((a, b) => b.weight - a.weight);
        else if (sortBy === 'key') s.sort((a, b) => a.key.localeCompare(b.key));
        else s.sort((a, b) => b.mtime - a.mtime);
        return s;
    }, [data, query, sortBy]);

    return (
        <div style={{
            width: '100%', height: '100%', overflow: 'auto',
            background: 'var(--vscode-editor-background, #1e1e1e)',
            color: 'var(--vscode-editor-foreground, #ddd)',
            fontFamily: 'var(--font-ui)', padding: '20px 24px',
        }}>
            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 6 }}>
                <i className="codicon codicon-chip" style={{ fontSize: 22, color: '#c084fc' }} />
                <div style={{ fontSize: 18, fontWeight: 600 }}>{filename}</div>
                <span style={{ fontSize: 11, padding: '2px 8px', borderRadius: 10, background: 'rgba(192,132,252,0.15)', color: '#c084fc' }}>
                    AIM · Neural Weight-Map
                </span>
                <button onClick={load} title="Reload" style={{ marginLeft: 'auto', background: 'transparent', border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))', color: 'inherit', borderRadius: 6, padding: '4px 8px', cursor: 'pointer', fontSize: 12 }}>
                    <i className="codicon codicon-refresh" />
                </button>
            </div>

            {loading && <div style={{ opacity: 0.6, padding: '24px 0' }}>Reading .aim…</div>}
            {error && (
                <div style={{ padding: 16, borderRadius: 8, background: 'rgba(248,81,73,0.1)', border: '1px solid rgba(248,81,73,0.3)', color: '#ff7b72', fontSize: 13 }}>
                    Could not read this .aim file: {error}
                </div>
            )}

            {/* JSON Kortex memory format */}
            {data && !loading && data.format === 'json' && (
                <>
                    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: 10, margin: '14px 0 16px' }}>
                        {[
                            ['Format', 'Kortex JSON'],
                            ['Size', fmtBytes(data.size_bytes)],
                            ['Entities', data.entities != null ? String(data.entities) : '—'],
                            ['Project tree', data.tree_count != null ? `${data.tree_count} paths` : '—'],
                        ].map(([label, value]) => (
                            <div key={label} style={{ padding: '10px 14px', borderRadius: 8, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))', background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))' }}>
                                <div style={{ fontSize: 10, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.06em' }}>{label}</div>
                                <div style={{ fontSize: 14, fontWeight: 600, marginTop: 3 }}>{value}</div>
                            </div>
                        ))}
                    </div>
                    <div style={{ fontSize: 11, opacity: 0.55, marginBottom: 8 }}>
                        Kortex memory map (JSON). {data.truncated ? 'Large file — preview truncated.' : ''}
                    </div>
                    <pre style={{
                        margin: 0, padding: '14px 16px', borderRadius: 8, overflow: 'auto', maxHeight: '60vh',
                        background: 'var(--vscode-textCodeBlock-background, rgba(0,0,0,0.25))',
                        border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
                        fontFamily: 'var(--font-mono)', fontSize: 12, lineHeight: 1.5, whiteSpace: 'pre',
                    }}>{data.pretty}</pre>
                </>
            )}

            {/* Binary AIM\x01 format */}
            {data && !loading && data.format !== 'json' && (
                <>
                    {/* Stat cards */}
                    <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(150px, 1fr))', gap: 10, margin: '14px 0 20px' }}>
                        {[
                            ['Entries', String(data.entry_count ?? 0)],
                            ['Size', fmtBytes(data.size_bytes)],
                            ['Format', data.magic_ok ? `AIM v${data.version}` : 'invalid magic'],
                            ['Written', fmtTime(data.written_at ?? 0)],
                        ].map(([label, value]) => (
                            <div key={label} style={{ padding: '10px 14px', borderRadius: 8, border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))', background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))' }}>
                                <div style={{ fontSize: 10, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.06em' }}>{label}</div>
                                <div style={{ fontSize: 14, fontWeight: 600, marginTop: 3 }}>{value}</div>
                            </div>
                        ))}
                    </div>

                    {/* Controls */}
                    <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginBottom: 10 }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 6, flex: 1, background: 'var(--vscode-input-background, rgba(255,255,255,0.04))', border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))', borderRadius: 6, padding: '4px 10px' }}>
                            <i className="codicon codicon-search" style={{ opacity: 0.5, fontSize: 13 }} />
                            <input
                                value={query}
                                onChange={(e) => setQuery(e.target.value)}
                                placeholder="Filter entries (key or gist)…"
                                style={{ flex: 1, background: 'transparent', border: 'none', outline: 'none', color: 'inherit', fontSize: 12 }}
                            />
                        </div>
                        <select value={sortBy} onChange={(e) => setSortBy(e.target.value as any)}
                            style={{ background: 'var(--vscode-dropdown-background, #2a2a2a)', color: 'inherit', border: '1px solid var(--vscode-dropdown-border, rgba(255,255,255,0.15))', borderRadius: 6, padding: '5px 8px', fontSize: 12 }}>
                            <option value="weight">Sort: weight</option>
                            <option value="mtime">Sort: modified</option>
                            <option value="key">Sort: key</option>
                        </select>
                        <span style={{ fontSize: 11, opacity: 0.5 }}>{rows.length} shown</span>
                    </div>

                    {/* Entry table */}
                    <div style={{ border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))', borderRadius: 8, overflow: 'hidden' }}>
                        {rows.length === 0 && <div style={{ padding: 20, opacity: 0.5, fontSize: 13 }}>No entries{query ? ' match the filter.' : ' in this map.'}</div>}
                        {rows.map((e, i) => (
                            <div key={i} style={{ display: 'flex', gap: 12, padding: '8px 12px', borderTop: i === 0 ? 'none' : '1px solid var(--vscode-panel-border, rgba(255,255,255,0.05))', alignItems: 'flex-start' }}>
                                {/* weight bar */}
                                <div style={{ width: 46, flexShrink: 0, paddingTop: 2 }}>
                                    <div style={{ height: 4, borderRadius: 2, background: 'rgba(255,255,255,0.1)', overflow: 'hidden' }}>
                                        <div style={{ width: `${Math.max(0, Math.min(1, e.weight)) * 100}%`, height: '100%', background: '#c084fc' }} />
                                    </div>
                                    <div style={{ fontSize: 9, opacity: 0.5, marginTop: 2, textAlign: 'right', fontFamily: 'monospace' }}>{e.weight.toFixed(2)}</div>
                                </div>
                                <div style={{ minWidth: 0, flex: 1 }}>
                                    <div style={{ fontSize: 12, fontWeight: 600, fontFamily: 'var(--font-mono)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{e.key}</div>
                                    {e.gist && <div style={{ fontSize: 11, opacity: 0.6, marginTop: 2, whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}>{e.gist}</div>}
                                </div>
                                <div style={{ fontSize: 10, opacity: 0.4, flexShrink: 0, whiteSpace: 'nowrap', paddingTop: 2 }}>{fmtTime(e.mtime)}</div>
                            </div>
                        ))}
                    </div>
                    {data.entry_count > data.entries.length && (
                        <div style={{ fontSize: 11, opacity: 0.5, marginTop: 8 }}>
                            Showing first {data.entries.length} of {data.entry_count} entries.
                        </div>
                    )}
                </>
            )}
        </div>
    );
};

export default AimViewer;
