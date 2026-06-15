import React, { useCallback, useMemo, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

type Finding = {
    kind: string;
    severity: string;
    file: string;
    line: number;
    column: number;
    snippet: string;
    redacted: string;
    bounty_hint: string;
};

type ScanResult = {
    files_scanned: number;
    bytes_scanned: number;
    findings: Finding[];
    source_maps_found: number;
    script_urls: string[];
};

type XssHit = {
    param: string;
    payload: string;
    severity: string;
    url: string;
    bounty_hint: string;
};

type BountyResult = {
    chunk: ScanResult;
    xss?: { hits: XssHit[]; params_tested: number; target: string } | null;
};

const SEV_COLOR: Record<string, string> = {
    CRITICAL: '#ef4444',
    HIGH: '#f97316',
    MEDIUM: '#eab308',
    LOW: '#94a3b8',
    INFO: '#64748b',
};

const ChunkSecretScannerPanel: React.FC = () => {
    const activeRoot = useStore((s) => s.activeRoot);
    const [mode, setMode] = useState<'workspace' | 'url' | 'bounty'>('workspace');
    const [url, setUrl] = useState('https://');
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState('');
    const [result, setResult] = useState<ScanResult | null>(null);
    const [xssHits, setXssHits] = useState<XssHit[]>([]);
    const [expanded, setExpanded] = useState<string | null>(null);

    const runScan = useCallback(async () => {
        setError('');
        setBusy(true);
        setResult(null);
        setXssHits([]);
        try {
            if (mode === 'workspace') {
                const data = await invoke<ScanResult>('chunk_secrets_scan_path', {
                    path: activeRoot,
                    maxFiles: 1500,
                });
                setResult(data);
            } else if (mode === 'bounty') {
                const data = await invoke<BountyResult>('security_bounty_scan_url', {
                    url: url.trim(),
                    includeXss: true,
                });
                setResult(data.chunk);
                setXssHits(data.xss?.hits ?? []);
            } else {
                const data = await invoke<ScanResult>('chunk_secrets_scan_url', { url: url.trim() });
                setResult(data);
            }
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    }, [activeRoot, mode, url]);

    const criticalCount = useMemo(
        () => result?.findings.filter((f) => f.severity === 'CRITICAL').length ?? 0,
        [result],
    );

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            <div style={{ padding: '10px 12px', borderBottom: '1px solid var(--vscode-panel-border)', flexShrink: 0 }}>
                <div style={{ fontSize: 11, lineHeight: 1.5, opacity: 0.75, marginBottom: 10 }}>
                    Hunt secrets in minified <code>.js</code> chunks (Rust-native, parallel scan).
                    <b> Bounty URL</b> mode adds XSS reflection probing — no DalFox/Moxy required for triage.
                </div>

                <div style={{ display: 'flex', gap: 6, marginBottom: 8, flexWrap: 'wrap' }}>
                    {([
                        ['workspace', 'Workspace'],
                        ['url', 'URL chunks'],
                        ['bounty', 'Bounty URL'],
                    ] as const).map(([m, label]) => (
                        <button
                            key={m}
                            type="button"
                            onClick={() => setMode(m)}
                            style={{
                                flex: 1,
                                minWidth: 90,
                                padding: '4px 8px',
                                fontSize: 10,
                                borderRadius: 4,
                                border: '1px solid var(--vscode-panel-border)',
                                background: mode === m ? 'rgba(59,130,246,0.15)' : 'transparent',
                                color: 'inherit',
                                cursor: 'pointer',
                            }}
                        >
                            {label}
                        </button>
                    ))}
                </div>

                {mode !== 'workspace' && (
                    <input
                        value={url}
                        onChange={(e) => setUrl(e.target.value)}
                        placeholder="https://target.app/"
                        style={{
                            width: '100%',
                            marginBottom: 8,
                            padding: '6px 8px',
                            fontSize: 11,
                            borderRadius: 4,
                            border: '1px solid var(--vscode-panel-border)',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                        }}
                    />
                )}

                <button
                    type="button"
                    disabled={busy || (mode === 'workspace' && !activeRoot) || ((mode === 'url' || mode === 'bounty') && url.trim().length < 10)}
                    onClick={() => void runScan()}
                    style={{
                        width: '100%',
                        padding: '8px 12px',
                        border: 'none',
                        borderRadius: 4,
                        cursor: busy ? 'wait' : 'pointer',
                        background: 'var(--vscode-button-background, #0e639c)',
                        color: 'var(--vscode-button-foreground, #fff)',
                        fontSize: 12,
                        fontWeight: 600,
                    }}
                >
                    {busy ? 'Scanning…' : mode === 'bounty' ? 'Run Rust-native Bounty Scan' : 'Scan JS Chunks for Secrets'}
                </button>

                {mode === 'workspace' && !activeRoot && (
                    <div style={{ marginTop: 8, fontSize: 10, opacity: 0.6 }}>
                        Open a folder first (scans <code>dist/</code>, <code>build/</code>,{' '}
                        <code>.next/</code> JS, etc.).
                    </div>
                )}
                {error && <div style={{ marginTop: 8, fontSize: 10, color: '#f87171' }}>{error}</div>}
            </div>

            {result && (
                <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', fontSize: 10, flexShrink: 0 }}>
                    <span>
                        {result.files_scanned} files · {(result.bytes_scanned / 1024).toFixed(0)} KB ·{' '}
                        {result.findings.length} hits
                        {criticalCount > 0 && (
                            <span style={{ color: SEV_COLOR.CRITICAL, marginLeft: 8, fontWeight: 700 }}>
                                {criticalCount} CRITICAL
                            </span>
                        )}
                    </span>
                    {result.source_maps_found > 0 && (
                        <span style={{ marginLeft: 8, opacity: 0.65 }}>
                            · {result.source_maps_found} source maps
                        </span>
                    )}
                    {result.script_urls.length > 0 && (
                        <div style={{ marginTop: 4, opacity: 0.55 }}>
                            Scripts: {result.script_urls.slice(0, 4).join(', ')}
                            {result.script_urls.length > 4 ? '…' : ''}
                        </div>
                    )}
                </div>
            )}

            {xssHits.length > 0 && (
                <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', fontSize: 10, flexShrink: 0 }}>
                    <div style={{ fontWeight: 700, marginBottom: 6, color: SEV_COLOR.HIGH }}>
                        XSS reflection hits ({xssHits.length})
                    </div>
                    {xssHits.map((h, i) => (
                        <div key={i} style={{ marginBottom: 6, opacity: 0.9 }}>
                            <code>{h.param}</code> · {h.severity} · {h.bounty_hint}
                        </div>
                    ))}
                </div>
            )}

            <div style={{ flex: 1, overflowY: 'auto', minHeight: 0 }}>
                {!result && !busy && (
                    <div style={{ padding: 16, fontSize: 11, lineHeight: 1.55, opacity: 0.6 }}>
                        Production SPAs often ship API keys in <code>*.chunk.js</code>. This scanner
                        targets high-signal patterns for instant bounty triage — not a replacement for
                        authorized DAST on in-scope targets.
                    </div>
                )}
                {result?.findings.map((f, i) => {
                    const id = `${f.file}:${f.line}:${i}`;
                    return (
                        <div
                            key={id}
                            style={{
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                padding: '8px 12px',
                                cursor: 'pointer',
                                background: expanded === id ? 'var(--vscode-list-hoverBackground)' : 'transparent',
                            }}
                            onClick={() => setExpanded(expanded === id ? null : id)}
                        >
                            <div style={{ display: 'flex', gap: 8, alignItems: 'flex-start' }}>
                                <span
                                    style={{
                                        fontSize: 8,
                                        fontWeight: 800,
                                        padding: '2px 5px',
                                        borderRadius: 3,
                                        background: `${SEV_COLOR[f.severity] ?? '#888'}33`,
                                        color: SEV_COLOR[f.severity] ?? '#888',
                                        flexShrink: 0,
                                    }}
                                >
                                    {f.severity}
                                </span>
                                <div style={{ flex: 1, minWidth: 0 }}>
                                    <div style={{ fontSize: 11, fontWeight: 600 }}>{f.kind.replace(/_/g, ' ')}</div>
                                    <div
                                        style={{
                                            fontSize: 10,
                                            opacity: 0.5,
                                            overflow: 'hidden',
                                            textOverflow: 'ellipsis',
                                            whiteSpace: 'nowrap',
                                        }}
                                    >
                                        {f.file}
                                        {f.line ? `:${f.line}` : ''} · {f.redacted}
                                    </div>
                                </div>
                            </div>
                            {expanded === id && (
                                <div style={{ marginTop: 8, fontSize: 10, lineHeight: 1.5 }}>
                                    <div
                                        style={{
                                            fontFamily: 'monospace',
                                            background: 'rgba(0,0,0,0.2)',
                                            padding: 6,
                                            borderRadius: 4,
                                            wordBreak: 'break-all',
                                            marginBottom: 6,
                                        }}
                                    >
                                        {f.snippet}
                                    </div>
                                    <div style={{ opacity: 0.85 }}>
                                        <b>Bounty angle:</b> {f.bounty_hint}
                                    </div>
                                </div>
                            )}
                        </div>
                    );
                })}
                {result && result.findings.length === 0 && (
                    <div style={{ padding: 20, textAlign: 'center', fontSize: 11, opacity: 0.5 }}>
                        No high-signal secrets in scanned bundles.
                    </div>
                )}
            </div>
        </div>
    );
};

export default ChunkSecretScannerPanel;
