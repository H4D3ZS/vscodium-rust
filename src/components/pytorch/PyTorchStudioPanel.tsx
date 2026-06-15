import React, { useCallback, useEffect, useMemo, useState } from 'react';
import { useStore } from '../../store';
import { detectPyTorch, installPyTorch, verifyPyTorch } from '../../application/pytorch/pytorchSetup';
import {
    cancelMlTrain,
    getMlConfig,
    getMlActiveRun,
    initMlStudio,
    installMlDeps,
    listMlData,
    listMlRuns,
    prepareMlDataset,
    runMlInference,
    saveMlConfig,
    trainMlModel,
    listMlWorkers,
} from '../../application/ml/mlStudio';
import type { PyTorchDetectResult, PyTorchInstallVariant, PyTorchVerifyResult } from '../../domain/pytorch/IPyTorchRepository';
import type { MlDatasetEntry, MlRunSummary, MlStudioConfig } from '../../domain/ml/IMlStudioRepository';
import { PYTORCH_BEGINNER_LESSONS } from '../../lib/pytorchLessons';
import { AI_ENGINEER_LEAD, AI_ENGINEER_TAGLINE } from '../../lib/aiEngineerManifesto';
import PyTorchLogo from './PyTorchLogo';
import { closeCenterWorkbench } from '../../application/layout/closeCenterWorkbench';

import {
    DashboardPanel,
    DatasetManagerPanel,
    ExperimentsPanel,
    ModelPanel,
    ToolsPanel,
} from './MlStudioPanels';

type Tab = 'setup' | 'data' | 'train' | 'dashboard' | 'model' | 'experiments' | 'tools' | 'infer' | 'learn';

const PYTORCH_DOCS = 'https://docs.pytorch.org/docs/2.12/index.html';
const ROCM_DOCS = 'https://rocm.docs.amd.com/projects/radeon-ryzen/en/latest/docs/install/installrad/windows/install-pytorch.html';

const DEFAULT_CONFIG: MlStudioConfig = {
    epochs: 20,
    learning_rate: 0.001,
    hidden_size: 64,
    val_ratio: 0.2,
    embed_model: 'nomic-embed-text',
    early_stop_patience: 3,
    model_template: 'tabular_mlp',
    model_source: 'builtin',
};

const gpuLabel = (d: PyTorchDetectResult | null) => {
    if (!d) return 'Detecting…';
    if (d.nvidia_gpu_name) return `NVIDIA · ${d.nvidia_gpu_name}`;
    if (d.amd_gpu_name) return `AMD · ${d.amd_gpu_name}`;
    return 'CPU only';
};

const PyTorchStudioPanel: React.FC<{ mode?: 'dock' | 'settings' }> = ({ mode = 'settings' }) => {
    const activeRoot = useStore((s) => s.activeRoot);
    const root = activeRoot || '.';

    const [tab, setTab] = useState<Tab>('setup');
    const [detected, setDetected] = useState<PyTorchDetectResult | null>(null);
    const [verify, setVerify] = useState<PyTorchVerifyResult | null>(null);
    const [config, setConfig] = useState<MlStudioConfig>(DEFAULT_CONFIG);
    const [datasets, setDatasets] = useState<MlDatasetEntry[]>([]);
    const [runs, setRuns] = useState<MlRunSummary[]>([]);
    const [selectedCsv, setSelectedCsv] = useState('');
    const [targetCol, setTargetCol] = useState('');
    const [selectedRun, setSelectedRun] = useState('');
    const [inferInput, setInferInput] = useState('{}');
    const [inferResult, setInferResult] = useState<Record<string, unknown> | null>(null);
    const [busy, setBusy] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [log, setLog] = useState<string>('');
    const [activeRunId, setActiveRunId] = useState<string | null>(null);
    const [resumeRunId, setResumeRunId] = useState('');
    const [workerId, setWorkerId] = useState('');
    const [workers, setWorkers] = useState<{ id: string; host: string }[]>([]);

    const columns = useMemo(() => {
        const d = datasets.find((x) => x.name === selectedCsv);
        return d?.columns ?? [];
    }, [datasets, selectedCsv]);

    const refreshAll = useCallback(async () => {
        if (!root) return;
        try {
            setDetected(await detectPyTorch());
            await initMlStudio(root);
            setConfig(await getMlConfig(root));
            const data = await listMlData(root);
            setDatasets(data);
            if (!selectedCsv && data[0]) setSelectedCsv(data[0].name);
            const r = await listMlRuns(root);
            setRuns(r);
            if (!selectedRun && r[0]) setSelectedRun(r[0].id);
            const active = await getMlActiveRun(root);
            if (active) setActiveRunId(active);
            setWorkers(await listMlWorkers(root));
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        }
    }, [root, selectedCsv, selectedRun]);

    useEffect(() => { void refreshAll(); }, [refreshAll]);

    const onInstall = async (variant: PyTorchInstallVariant) => {
        setBusy(variant);
        setError(null);
        try {
            await installPyTorch(variant);
            await installMlDeps();
            await refreshAll();
            setVerify(await verifyPyTorch());
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const onPrepare = async () => {
        if (!selectedCsv || !targetCol) return;
        setBusy('prepare');
        setError(null);
        try {
            const res = await prepareMlDataset(root, selectedCsv, targetCol, config.val_ratio);
            setLog(JSON.stringify(res, null, 2));
            await refreshAll();
            setTab('train');
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const onTrain = async (resume?: string) => {
        setBusy('train');
        setError(null);
        try {
            await saveMlConfig(root, config);
            const res = await trainMlModel(root, resume || undefined, workerId || undefined);
            setActiveRunId(res.run_id ?? null);
            const action = res.resumed ? 'Resumed' : 'Started';
            setLog(`${action} training: job ${res.job_id}, run ${res.run_id}\nOpen Dashboard tab for live charts.`);
            setTab('dashboard');
            setTimeout(() => void refreshAll(), 3000);
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const onCancelTrain = async () => {
        if (!activeRunId) return;
        setBusy('cancel');
        try {
            await cancelMlTrain(root, activeRunId);
            setLog(`Cancelled training for ${activeRunId}`);
            await refreshAll();
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const onInfer = async () => {
        if (!selectedRun) return;
        setBusy('infer');
        setError(null);
        try {
            const input = JSON.parse(inferInput) as Record<string, number>;
            const res = await runMlInference(root, selectedRun, input);
            setInferResult(res);
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const tabs: { id: Tab; label: string }[] = [
        { id: 'setup', label: 'Setup' },
        { id: 'data', label: 'Data' },
        { id: 'train', label: 'Train' },
        { id: 'dashboard', label: 'Dashboard' },
        { id: 'model', label: 'Model' },
        { id: 'experiments', label: 'Experiments' },
        { id: 'tools', label: 'Export/Debug' },
        { id: 'infer', label: 'Inference' },
        { id: 'learn', label: 'Learn' },
    ];

    const isDock = mode === 'dock';
    const recommended = detected?.recommended_variant;

    return (
        <div
            className="agent-first-ide-panel pytorch-ml-studio"
            style={{
                maxWidth: isDock ? '100%' : 920,
                padding: isDock ? '10px 14px 16px' : undefined,
                height: isDock ? '100%' : undefined,
                boxSizing: 'border-box',
            }}
        >
            <div style={{ display: 'flex', alignItems: 'center', gap: 10, marginBottom: isDock ? 8 : 6 }}>
                <PyTorchLogo size={isDock ? 32 : 28} />
                <div style={{ flex: 1, minWidth: 0 }}>
                    <div style={{ fontWeight: 700, fontSize: isDock ? 15 : 14, display: 'flex', alignItems: 'center', gap: 8, flexWrap: 'wrap' }}>
                        PyTorch ML Studio
                        <a href={PYTORCH_DOCS} target="_blank" rel="noreferrer" style={{ fontSize: 11, fontWeight: 500, color: '#ee4c2c' }}>
                            docs ↗
                        </a>
                    </div>
                    {!isDock && (
                        <p className="afi-lead" style={{ margin: '4px 0 0' }}>
                            {AI_ENGINEER_TAGLINE} Project: <code>{root}</code>
                        </p>
                    )}
                    {isDock && (
                        <p className="afi-subtle" style={{ margin: '2px 0 0', fontSize: 11 }}>
                            {AI_ENGINEER_LEAD} · <code>{root}</code>
                        </p>
                    )}
                </div>
                {isDock && (
                    <button type="button" className="settings-button" onClick={() => closeCenterWorkbench()} title="Back to editor">
                        <i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                    </button>
                )}
            </div>

            <div style={{ display: 'flex', gap: 6, marginBottom: 16, flexWrap: 'wrap' }}>
                {tabs.map((t) => (
                    <button
                        key={t.id}
                        type="button"
                        className="settings-button"
                        onClick={() => setTab(t.id)}
                        style={{
                            borderColor: tab === t.id ? '#ee4c2c' : undefined,
                            background: tab === t.id ? 'rgba(238,76,44,0.12)' : undefined,
                        }}
                    >
                        {t.label}
                    </button>
                ))}
            </div>

            {error && <div style={{ color: '#ff8b80', fontSize: 12, marginBottom: 10 }}>{error}</div>}

            {tab === 'setup' && (
                <div className="settings-card">
                    <div style={{ fontWeight: 600, marginBottom: 8 }}>Environment</div>
                    <div className="afi-muted" style={{ marginBottom: 10, lineHeight: 1.6 }}>
                        Python: {detected?.python || 'not found'}{detected?.python_version ? ` (${detected.python_version})` : ''}<br />
                        PyTorch: {detected?.torch_version || 'not installed'}{detected?.torch_backend ? ` · ${detected.torch_backend}` : ''}<br />
                        GPU: {gpuLabel(detected)}
                        {recommended && (
                            <span style={{ marginLeft: 8, color: '#ee4c2c', fontSize: 11 }}>
                                recommended: {recommended}
                            </span>
                        )}
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, marginBottom: 8 }}>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cpu')}>
                            {busy === 'cpu' ? 'Installing…' : 'CPU'}
                        </button>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cu124')} title="NVIDIA CUDA 12.4">
                            {busy === 'cu124' ? 'Installing…' : 'NVIDIA CUDA 12.4'}
                        </button>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cu121')} title="NVIDIA CUDA 12.1">
                            {busy === 'cu121' ? 'Installing…' : 'CUDA 12.1'}
                        </button>
                        <button
                            type="button"
                            className="settings-button"
                            disabled={!!busy}
                            onClick={() => void onInstall('rocm721')}
                            title="AMD ROCm 7.2.1 on Windows — requires Python 3.12+ and Radeon driver 26.2.2+"
                        >
                            {busy === 'rocm721' ? 'Installing…' : 'AMD ROCm 7.2.1'}
                        </button>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('rocm62')} title="AMD ROCm on Linux">
                            {busy === 'rocm62' ? 'Installing…' : 'ROCm 6.2 (Linux)'}
                        </button>
                        <button type="button" className="settings-button success" disabled={!!busy} onClick={async () => { setBusy('v'); setVerify(await verifyPyTorch()); setBusy(null); }}>
                            Verify
                        </button>
                    </div>
                    <p className="afi-subtle" style={{ fontSize: 11, margin: 0 }}>
                        NVIDIA: <a href="https://pytorch.org/get-started/locally/" target="_blank" rel="noreferrer" style={{ color: '#ee4c2c' }}>pytorch.org/get-started</a>
                        {' · '}
                        AMD: <a href={ROCM_DOCS} target="_blank" rel="noreferrer" style={{ color: '#ee4c2c' }}>ROCm Windows PyTorch</a>
                        {detected?.gpu_vendor === 'amd' && detected.python_version && parseFloat(detected.python_version) < 3.12 && (
                            <span style={{ color: '#ff8b80' }}> · ROCm 7.2.1 needs Python 3.12+</span>
                        )}
                    </p>
                    {verify && (
                        <pre style={{ marginTop: 10, fontSize: 11, padding: 10, background: 'rgba(0,0,0,0.2)', borderRadius: 6, overflow: 'auto' }}>{JSON.stringify(verify, null, 2)}</pre>
                    )}
                </div>
            )}

            {tab === 'data' && (
                <div className="settings-card">
                    <div style={{ fontWeight: 600, marginBottom: 8 }}>Datasets in .hades/ml/data/</div>
                    <p className="afi-desc">Copy a CSV into that folder, refresh, pick target column, then prepare train/val split.</p>
                    <button type="button" className="settings-button" onClick={() => void refreshAll()} style={{ marginBottom: 10 }}>Refresh</button>
                    {datasets.length === 0 ? (
                        <p className="afi-muted">No CSV files yet.</p>
                    ) : (
                        <>
                            <select className="settings-select" value={selectedCsv} onChange={(e) => setSelectedCsv(e.target.value)} style={{ marginBottom: 8, width: '100%' }}>
                                {datasets.map((d) => <option key={d.name} value={d.name}>{d.name} ({d.columns.length} cols)</option>)}
                            </select>
                            <select className="settings-select" value={targetCol} onChange={(e) => setTargetCol(e.target.value)} style={{ marginBottom: 8, width: '100%' }}>
                                <option value="">Target column…</option>
                                {columns.map((c) => <option key={c} value={c}>{c}</option>)}
                            </select>
                            <label className="afi-muted">Val ratio: {config.val_ratio}</label>
                            <input type="range" min={0.05} max={0.5} step={0.05} value={config.val_ratio}
                                onChange={(e) => setConfig({ ...config, val_ratio: parseFloat(e.target.value) })}
                                style={{ width: '100%', marginBottom: 10 }} />
                            <button type="button" className="settings-button success" disabled={!!busy || !targetCol} onClick={() => void onPrepare()}>
                                {busy === 'prepare' ? 'Preparing…' : 'Prepare train/val split'}
                            </button>
                            <div style={{ marginTop: 14 }}>
                                <DatasetManagerPanel root={root} csvName={selectedCsv} targetCol={targetCol} />
                            </div>
                        </>
                    )}
                </div>
            )}

            {tab === 'train' && (
                <div className="settings-card">
                    <div style={{ fontWeight: 600, marginBottom: 8 }}>Training config</div>
                    <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 10, marginBottom: 12 }}>
                        <label className="afi-muted">Epochs<input type="number" className="settings-input" value={config.epochs} min={1} max={200}
                            onChange={(e) => setConfig({ ...config, epochs: parseInt(e.target.value, 10) || 20 })} /></label>
                        <label className="afi-muted">Learning rate<input type="number" step={0.0001} className="settings-input" value={config.learning_rate}
                            onChange={(e) => setConfig({ ...config, learning_rate: parseFloat(e.target.value) || 0.001 })} /></label>
                        <label className="afi-muted">Hidden size<input type="number" className="settings-input" value={config.hidden_size} min={8} max={512}
                            onChange={(e) => setConfig({ ...config, hidden_size: parseInt(e.target.value, 10) || 64 })} /></label>
                        <label className="afi-muted">Early-stop patience<input type="number" className="settings-input" value={config.early_stop_patience} min={1} max={20}
                            onChange={(e) => setConfig({ ...config, early_stop_patience: parseInt(e.target.value, 10) || 3 })} /></label>
                    </div>
                    {config.model_template !== 'tabular_mlp' && (
                        <p className="afi-subtle" style={{ fontSize: 11, marginBottom: 8 }}>
                            Pretrained template: <code>{config.model_source}/{config.model_template}</code> (tabular CSV still uses MLP; vision models apply when image data is prepared)
                        </p>
                    )}
                    <div style={{ marginBottom: 12 }}>
                        <label className="afi-muted" style={{ display: 'block', marginBottom: 4 }}>Remote worker (optional)</label>
                        <select className="settings-select" value={workerId} onChange={(e) => setWorkerId(e.target.value)} style={{ width: '100%' }}>
                            <option value="">Local GPU/CPU</option>
                            {workers.map((w) => <option key={w.id} value={w.id}>{w.id} @ {w.host}</option>)}
                        </select>
                        <p className="afi-subtle" style={{ fontSize: 10, marginTop: 4 }}>
                            Add workers in <code>.hades/ml/workers.json</code> or via Settings → Workspace remote SSH host.
                        </p>
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, marginBottom: 10 }}>
                        <button type="button" className="settings-button success" disabled={!!busy} onClick={() => void onTrain()}>
                            {busy === 'train' ? 'Starting…' : 'Start new training'}
                        </button>
                        {activeRunId && (
                            <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onCancelTrain()}>
                                {busy === 'cancel' ? 'Cancelling…' : 'Cancel active run'}
                            </button>
                        )}
                    </div>
                    {runs.length > 0 && (
                        <div style={{ marginBottom: 12 }}>
                            <label className="afi-muted" style={{ display: 'block', marginBottom: 4 }}>Resume from checkpoint</label>
                            <select className="settings-select" value={resumeRunId} onChange={(e) => setResumeRunId(e.target.value)} style={{ width: '100%', marginBottom: 6 }}>
                                <option value="">Select run with checkpoint.pt…</option>
                                {runs.map((r) => <option key={r.id} value={r.id}>{r.id} (acc {r.val_acc?.toFixed(3) ?? '?'})</option>)}
                            </select>
                            <button type="button" className="settings-button" disabled={!!busy || !resumeRunId}
                                onClick={() => void onTrain(resumeRunId)}>
                                Resume training
                            </button>
                        </div>
                    )}
                    <p className="afi-subtle" style={{ marginTop: 8 }}>Runs under .hades/ml/runs/ — live charts on Dashboard tab.</p>
                    {runs.length > 0 && (
                        <ul className="afi-muted" style={{ marginTop: 10 }}>
                            {runs.slice(0, 5).map((r) => (
                                <li key={r.id}>
                                    <button type="button" className="settings-button" style={{ padding: '2px 6px', fontSize: 11 }}
                                        onClick={() => { setActiveRunId(r.id); setTab('dashboard'); }}>
                                        {r.id}
                                    </button>
                                    {' '}— val_acc {r.val_acc?.toFixed(4) ?? 'n/a'}
                                </li>
                            ))}
                        </ul>
                    )}
                </div>
            )}

            {tab === 'dashboard' && (
                <div className="settings-card">
                    <DashboardPanel
                        root={root}
                        runId={activeRunId}
                        runs={runs}
                        onRun={setActiveRunId}
                    />
                </div>
            )}

            {tab === 'model' && (
                <div className="settings-card">
                    <ModelPanel root={root} runId={activeRunId} runs={runs} onRun={setActiveRunId} />
                </div>
            )}

            {tab === 'experiments' && (
                <div className="settings-card">
                    <ExperimentsPanel root={root} runs={runs} />
                </div>
            )}

            {tab === 'tools' && (
                <div className="settings-card">
                    <ToolsPanel root={root} runId={activeRunId} runs={runs} onRun={setActiveRunId} />
                </div>
            )}

            {tab === 'infer' && (
                <div className="settings-card">
                    <div style={{ fontWeight: 600, marginBottom: 8 }}>Inference</div>
                    <button type="button" className="settings-button" onClick={() => void refreshAll()} style={{ marginBottom: 8 }}>Refresh runs</button>
                    <select className="settings-select" value={selectedRun} onChange={(e) => setSelectedRun(e.target.value)} style={{ width: '100%', marginBottom: 8 }}>
                        {runs.length === 0 && <option value="">No runs yet</option>}
                        {runs.map((r) => <option key={r.id} value={r.id}>{r.id} (acc {r.val_acc?.toFixed(3) ?? '?'})</option>)}
                    </select>
                    <label className="afi-muted">Input JSON (feature column → number)</label>
                    <textarea className="settings-input" rows={4} value={inferInput} onChange={(e) => setInferInput(e.target.value)} style={{ width: '100%', marginBottom: 8, fontFamily: 'monospace' }} />
                    <button type="button" className="settings-button success" disabled={!!busy || !selectedRun} onClick={() => void onInfer()}>Run inference</button>
                    {inferResult && (
                        <pre style={{ marginTop: 10, fontSize: 11, padding: 10, background: 'rgba(0,0,0,0.2)', borderRadius: 6 }}>{JSON.stringify(inferResult, null, 2)}</pre>
                    )}
                </div>
            )}

            {tab === 'learn' && (
                <div style={{ fontSize: 12 }}>
                    {PYTORCH_BEGINNER_LESSONS.map((lesson) => (
                        <details key={lesson.id} style={{ marginBottom: 8 }}>
                            <summary style={{ cursor: 'pointer', fontWeight: 600 }}>{lesson.title}</summary>
                            <p className="afi-muted">{lesson.summary}</p>
                            <pre style={{ fontSize: 11, padding: 8, background: 'rgba(0,0,0,0.2)', borderRadius: 4, overflow: 'auto' }}>{lesson.code}</pre>
                        </details>
                    ))}
                </div>
            )}

            {log && tab !== 'learn' && (
                <pre style={{ marginTop: 12, fontSize: 11, padding: 10, background: 'rgba(0,0,0,0.15)', borderRadius: 6, whiteSpace: 'pre-wrap' }}>{log}</pre>
            )}
        </div>
    );
};

export default PyTorchStudioPanel;
