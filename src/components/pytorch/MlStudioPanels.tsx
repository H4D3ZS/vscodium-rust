import React, { useCallback, useEffect, useState } from 'react';
import {
    exportMlModel,
    exportMlReport,
    getMlDatasetStats,
    getMlModelSummary,
    getMlPretrainedGallery,
    listMlExperiments,
    runMlBenchmark,
    runMlGradCheck,
    runMlHpo,
    runMlLrFinder,
} from '../../application/ml/mlStudio';
import TrainingDashboard from './TrainingDashboard';

const card: React.CSSProperties = {
    padding: 10,
    borderRadius: 6,
    background: 'rgba(0,0,0,0.2)',
    border: '1px solid rgba(255,255,255,0.06)',
    marginBottom: 10,
};

const btn = 'settings-button';

const RunPicker: React.FC<{
    runs: { id: string }[];
    value: string | null;
    onChange: (id: string | null) => void;
}> = ({ runs, value, onChange }) => (
    <select className="settings-select" value={value ?? ''} onChange={(e) => onChange(e.target.value || null)} style={{ minWidth: 160 }}>
        <option value="">Select run…</option>
        {runs.map((r) => <option key={r.id} value={r.id}>{r.id}</option>)}
    </select>
);

export const DatasetManagerPanel: React.FC<{
    root: string;
    csvName: string;
    targetCol: string;
}> = ({ root, csvName, targetCol }) => {
    const [stats, setStats] = useState<Record<string, unknown> | null>(null);
    const [busy, setBusy] = useState(false);

    const load = useCallback(async () => {
        if (!csvName) return;
        setBusy(true);
        try {
            setStats(await getMlDatasetStats(root, csvName, targetCol || undefined));
        } catch { setStats(null); }
        finally { setBusy(false); }
    }, [root, csvName, targetCol]);

    useEffect(() => { void load(); }, [load]);

    const dist = (stats?.class_distribution ?? {}) as Record<string, number>;
    const distMax = Math.max(...Object.values(dist), 1);
    const preview = (stats?.preview ?? []) as Record<string, string>[];
    const colStats = (stats?.column_stats ?? []) as { column: string; mean?: number; std?: number; unique: number }[];

    return (
        <div>
            <button type="button" className={btn} onClick={() => void load()} disabled={busy || !csvName}>
                {busy ? 'Analyzing…' : 'Analyze dataset'}
            </button>
            {stats && (
                <>
                    <p className="afi-muted" style={{ marginTop: 8 }}>
                        {String(stats.rows)} rows · {String((stats.columns as string[])?.length ?? 0)} cols · {String(stats.memory_mb)} MB
                    </p>
                    {Object.keys(dist).length > 0 && (
                        <div style={card}>
                            <div style={{ fontWeight: 600, fontSize: 12, marginBottom: 8 }}>Class distribution</div>
                            {Object.entries(dist).slice(0, 12).map(([k, v]) => (
                                <div key={k} style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4, fontSize: 11 }}>
                                    <span style={{ width: 80, overflow: 'hidden', textOverflow: 'ellipsis' }}>{k}</span>
                                    <div style={{ flex: 1, height: 8, background: 'rgba(255,255,255,0.08)', borderRadius: 4 }}>
                                        <div style={{ width: `${(v / distMax) * 100}%`, height: '100%', background: '#ee4c2c', borderRadius: 4 }} />
                                    </div>
                                    <span style={{ opacity: 0.6 }}>{v}</span>
                                </div>
                            ))}
                        </div>
                    )}
                    {colStats.length > 0 && (
                        <div style={card}>
                            <div style={{ fontWeight: 600, fontSize: 12, marginBottom: 6 }}>Column statistics</div>
                            <div style={{ fontSize: 10, maxHeight: 120, overflow: 'auto' }}>
                                {colStats.map((c) => (
                                    <div key={c.column}>
                                        <b>{c.column}</b> — unique {c.unique}
                                        {c.mean != null && <> · μ={c.mean.toFixed(3)} σ={c.std?.toFixed(3)}</>}
                                    </div>
                                ))}
                            </div>
                        </div>
                    )}
                    {preview.length > 0 && (
                        <div style={{ overflow: 'auto', fontSize: 10 }}>
                            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                                <thead>
                                    <tr>{Object.keys(preview[0]).map((k) => (
                                        <th key={k} style={{ textAlign: 'left', padding: 4, borderBottom: '1px solid rgba(255,255,255,0.1)' }}>{k}</th>
                                    ))}</tr>
                                </thead>
                                <tbody>
                                    {preview.map((row, i) => (
                                        <tr key={i}>{Object.values(row).map((v, j) => (
                                            <td key={j} style={{ padding: 4, borderBottom: '1px solid rgba(255,255,255,0.04)' }}>{v}</td>
                                        ))}</tr>
                                    ))}
                                </tbody>
                            </table>
                        </div>
                    )}
                </>
            )}
        </div>
    );
};

export const ModelPanel: React.FC<{ root: string; runId: string | null; runs: { id: string }[]; onRun: (id: string | null) => void }> = ({
    root, runId, runs, onRun,
}) => {
    const [summary, setSummary] = useState<Record<string, unknown> | null>(null);
    const [gallery, setGallery] = useState<{ id: string; source: string; task: string; desc?: string }[]>([]);
    const [busy, setBusy] = useState(false);

    useEffect(() => {
        void getMlPretrainedGallery(root).then((g) => {
            setGallery((g.models as typeof gallery) ?? []);
        }).catch(() => {});
    }, [root]);

    useEffect(() => {
        if (!runId) { setSummary(null); return; }
        setBusy(true);
        getMlModelSummary(root, runId).then(setSummary).catch(() => setSummary(null)).finally(() => setBusy(false));
    }, [root, runId]);

    const layers = (summary?.layers ?? []) as { name: string; type: string; params: number; trainable: number }[];

    return (
        <div>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10, flexWrap: 'wrap' }}>
                <RunPicker runs={runs} value={runId} onChange={onRun} />
                {busy && <span className="afi-muted">Loading…</span>}
            </div>
            {summary && (
                <div style={card}>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 12, fontSize: 12, marginBottom: 10 }}>
                        <span>Params: <b>{String(summary.total_params)}</b></span>
                        <span>Trainable: <b>{String(summary.trainable_params)}</b></span>
                        <span>Size: <b>{String(summary.size_mb)} MB</b></span>
                        <span>In: <code>{JSON.stringify(summary.input_shape)}</code></span>
                        <span>Out: <code>{JSON.stringify(summary.output_shape)}</code></span>
                    </div>
                    <div style={{ fontWeight: 600, fontSize: 11, marginBottom: 6 }}>Layers</div>
                    {layers.map((l) => (
                        <div key={l.name} style={{ fontSize: 10, padding: '3px 0', borderBottom: '1px solid rgba(255,255,255,0.04)' }}>
                            <span style={{ color: '#ee4c2c' }}>{l.name}</span> · {l.type} · {l.params.toLocaleString()} params
                        </div>
                    ))}
                </div>
            )}
            <div style={{ fontWeight: 600, fontSize: 12, margin: '12px 0 6px' }}>Pre-trained gallery (torchvision)</div>
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6 }}>
                {gallery.slice(0, 24).map((m) => (
                    <div key={m.id} style={{ ...card, marginBottom: 0, flex: '1 1 140px', fontSize: 10 }}>
                        <div style={{ fontWeight: 700, color: '#ee4c2c' }}>{m.id}</div>
                        <div style={{ opacity: 0.6 }}>{m.source} · {m.task}</div>
                        {m.desc && <div style={{ marginTop: 4 }}>{m.desc}</div>}
                    </div>
                ))}
            </div>
        </div>
    );
};

export const ExperimentsPanel: React.FC<{ root: string }> = ({ root }) => {
    const [rows, setRows] = useState<Record<string, unknown>[]>([]);
    const [hpoMode, setHpoMode] = useState<'random' | 'grid'>('random');
    const [busy, setBusy] = useState<string | null>(null);
    const [msg, setMsg] = useState('');

    const refresh = useCallback(async () => {
        try { setRows(await listMlExperiments(root)); } catch { setRows([]); }
    }, [root]);

    useEffect(() => { void refresh(); }, [refresh]);

    const onHpo = async () => {
        setBusy('hpo');
        setMsg('');
        try {
            const r = await runMlHpo(root, hpoMode, 9);
            setMsg(`HPO done — best val_acc ${(r.best as Record<string, unknown>)?.val_acc ?? '?'}`);
            await refresh();
        } catch (e) {
            setMsg(e instanceof Error ? e.message : String(e));
        } finally { setBusy(null); }
    };

    return (
        <div>
            <button type="button" className={btn} onClick={() => void refresh()} style={{ marginBottom: 10 }}>Refresh experiments</button>
            <div style={{ overflow: 'auto', marginBottom: 12 }}>
                <table style={{ width: '100%', fontSize: 11, borderCollapse: 'collapse' }}>
                    <thead>
                        <tr style={{ textAlign: 'left', opacity: 0.7 }}>
                            <th>Run</th><th>Val acc</th><th>Git</th><th>Epochs</th>
                        </tr>
                    </thead>
                    <tbody>
                        {rows.map((r) => {
                            const m = r.metrics as Record<string, unknown> | undefined;
                            const e = r.experiment as Record<string, unknown> | undefined;
                            return (
                                <tr key={String(r.id)}>
                                    <td style={{ padding: 4 }}><code>{String(r.id)}</code></td>
                                    <td>{m?.val_acc != null ? Number(m.val_acc).toFixed(4) : '—'}</td>
                                    <td>{String(e?.git_hash ?? '—')}</td>
                                    <td>{String(m?.epochs_run ?? '—')}</td>
                                </tr>
                            );
                        })}
                    </tbody>
                </table>
            </div>
            <div style={card}>
                <div style={{ fontWeight: 600, marginBottom: 8 }}>Hyperparameter search</div>
                <select className="settings-select" value={hpoMode} onChange={(e) => setHpoMode(e.target.value as 'random' | 'grid')} style={{ marginRight: 8 }}>
                    <option value="random">Random search</option>
                    <option value="grid">Grid search</option>
                </select>
                <button type="button" className={`${btn} success`} disabled={!!busy} onClick={() => void onHpo()}>
                    {busy === 'hpo' ? 'Running…' : 'Run HPO (9 trials)'}
                </button>
                {msg && <p className="afi-subtle" style={{ marginTop: 8 }}>{msg}</p>}
            </div>
        </div>
    );
};

export const ToolsPanel: React.FC<{ root: string; runId: string | null; runs: { id: string }[]; onRun: (id: string | null) => void }> = ({
    root, runId, runs, onRun,
}) => {
    const [log, setLog] = useState('');
    const [busy, setBusy] = useState(false);

    const needRun = () => {
        if (!runId) { setLog('Select a run first.'); return false; }
        return true;
    };

    const act = async (label: string, fn: () => Promise<unknown>) => {
        if (!needRun()) return;
        setBusy(true);
        setLog(`${label}…`);
        try {
            const r = await fn();
            setLog(JSON.stringify(r, null, 2));
        } catch (e) {
            setLog(e instanceof Error ? e.message : String(e));
        } finally { setBusy(false); }
    };

    return (
        <div>
            <RunPicker runs={runs} value={runId} onChange={onRun} />
            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, margin: '10px 0' }}>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('ONNX export', () => exportMlModel(root, runId!, 'onnx'))}>Export ONNX</button>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('TorchScript', () => exportMlModel(root, runId!, 'torchscript'))}>TorchScript</button>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('All formats', () => exportMlModel(root, runId!, 'all'))}>Export all</button>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('Grad check', () => runMlGradCheck(root, runId!))}>Grad check</button>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('LR finder', () => runMlLrFinder(root, 20))}>LR finder</button>
                <button type="button" className={btn} disabled={busy} onClick={() => void act('Benchmark', () => runMlBenchmark(root, runId!, 200))}>Benchmark infer</button>
                <button type="button" className={btn} disabled={busy} onClick={async () => {
                    if (!needRun()) return;
                    setBusy(true);
                    try { setLog(await exportMlReport(root, runId!)); } catch (e) { setLog(String(e)); }
                    finally { setBusy(false); }
                }}>Export report (MD)</button>
            </div>
            {log && <pre style={{ fontSize: 10, padding: 10, background: 'rgba(0,0,0,0.2)', borderRadius: 6, maxHeight: 240, overflow: 'auto', whiteSpace: 'pre-wrap' }}>{log}</pre>}
        </div>
    );
};

export const DashboardPanel: React.FC<{ root: string; runId: string | null; runs: { id: string }[]; onRun: (id: string | null) => void }> = (props) => (
    <div>
        <RunPicker runs={props.runs} value={props.runId} onChange={props.onRun} />
        <div style={{ marginTop: 10 }}>
            <TrainingDashboard root={props.root} runId={props.runId} />
        </div>
        <ConfusionBlock root={props.root} runId={props.runId} />
    </div>
);

const ConfusionBlock: React.FC<{ root: string; runId: string | null }> = ({ root, runId }) => {
    const [cm, setCm] = useState<{ labels: string[]; matrix: number[][] } | null>(null);
    useEffect(() => {
        if (!runId) return;
        import('../../application/ml/mlStudio').then(({ getMlRunMetrics }) =>
            getMlRunMetrics(root, runId).then((m) => setCm(m.confusion_matrix ?? null)).catch(() => setCm(null)),
        );
    }, [root, runId]);
    if (!cm?.matrix?.length) return null;
    const max = Math.max(...cm.matrix.flat(), 1);
    return (
        <div style={{ ...card, marginTop: 12 }}>
            <div style={{ fontWeight: 600, fontSize: 12, marginBottom: 8 }}>Confusion matrix</div>
            <div style={{ overflow: 'auto' }}>
                <table style={{ fontSize: 10, borderCollapse: 'collapse' }}>
                    <thead><tr><th /><th colSpan={cm.labels.length}>Predicted</th></tr></thead>
                    <tbody>
                        {cm.matrix.map((row, i) => (
                            <tr key={i}>
                                <td style={{ padding: 4, opacity: 0.7 }}>{cm.labels[i]}</td>
                                {row.map((v, j) => (
                                    <td key={j} style={{
                                        padding: 6, textAlign: 'center',
                                        background: `rgba(238,76,44,${0.15 + (v / max) * 0.75})`,
                                    }}>{v}</td>
                                ))}
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </div>
    );
};
