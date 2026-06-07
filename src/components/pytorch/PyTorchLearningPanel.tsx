import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../../store';
import { detectPyTorch, installPyTorch, verifyPyTorch } from '../../application/pytorch/pytorchSetup';
import type { PyTorchDetectResult, PyTorchLesson, PyTorchVerifyResult } from '../../domain/pytorch/IPyTorchRepository';
import { PYTORCH_BEGINNER_LESSONS } from '../../lib/pytorchLessons';

const PyTorchLearningPanel: React.FC = () => {
    const [detected, setDetected] = useState<PyTorchDetectResult | null>(null);
    const [verify, setVerify] = useState<PyTorchVerifyResult | null>(null);
    const [busy, setBusy] = useState<string | null>(null);
    const [error, setError] = useState<string | null>(null);
    const [activeLesson, setActiveLesson] = useState<PyTorchLesson>(PYTORCH_BEGINNER_LESSONS[0]);

    const refresh = useCallback(async () => {
        try {
            setDetected(await detectPyTorch());
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        }
    }, []);

    useEffect(() => { void refresh(); }, [refresh]);

    const onInstall = async (variant: 'cpu' | 'cu121' | 'cu124') => {
        setBusy(variant);
        setError(null);
        try {
            await installPyTorch(variant);
            await refresh();
            setVerify(await verifyPyTorch());
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const onVerify = async () => {
        setBusy('verify');
        setError(null);
        try {
            setVerify(await verifyPyTorch());
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(null);
        }
    };

    const runInTerminal = async (code: string) => {
        const root = useStore.getState().activeRoot || '.';
        useStore.getState().setActivePanelTab('TERMINAL');
        try {
            const groupId = await useStore.getState().addTerminalGroup();
            const group = useStore.getState().terminalGroups.find((g) => g.id === groupId);
            const termId = group?.activeInstanceId;
            if (!termId) return;
            const { invoke } = await import('../../tauri_bridge');
            if (root) {
                await invoke('terminal_send_data', { id: termId, data: `cd /d "${root}"\r` });
            }
            const oneLiner = code.split('\n').filter((l) => l.trim() && !l.trim().startsWith('#')).join('; ');
            await invoke('terminal_send_data', { id: termId, data: `python -c "${oneLiner.replace(/"/g, '\\"')}"\r` });
        } catch (e) {
            console.error(e);
        }
    };

    return (
        <div className="agent-first-ide-panel" style={{ maxWidth: 860, display: 'flex', gap: 20 }}>
            <div style={{ flex: 1, minWidth: 0 }}>
                <div style={{ fontWeight: 700, fontSize: 14, marginBottom: 6 }}>PyTorch learning hub</div>
                <p className="afi-lead">
                    One-click install for beginners, then a guided path from tensors → autograd → training loops.
                    Built for PyTorch ambassadors teaching in the IDE.
                </p>

                <div className="settings-card" style={{ marginBottom: 16 }}>
                    <div style={{ fontWeight: 600, marginBottom: 8 }}>Environment</div>
                    <div className="afi-muted" style={{ marginBottom: 10 }}>
                        Python: {detected?.python || 'not found — install Python 3.10+'}<br />
                        PyTorch: {detected?.torch_version || 'not installed'}<br />
                        GPU: {detected?.cuda_available ? detected.gpu_name || 'CUDA detected' : 'CPU only (or no NVIDIA driver)'}
                    </div>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8 }}>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cpu')}>
                            {busy === 'cpu' ? 'Installing…' : 'Install CPU'}
                        </button>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cu124')}>
                            {busy === 'cu124' ? 'Installing…' : 'Install CUDA 12.4'}
                        </button>
                        <button type="button" className="settings-button" disabled={!!busy} onClick={() => void onInstall('cu121')}>
                            {busy === 'cu121' ? 'Installing…' : 'Install CUDA 12.1'}
                        </button>
                        <button type="button" className="settings-button success" disabled={!!busy} onClick={() => void onVerify()}>
                            {busy === 'verify' ? 'Checking…' : 'Verify install'}
                        </button>
                    </div>
                    {verify && (
                        <pre style={{ marginTop: 10, fontSize: 11, padding: 10, background: 'rgba(0,0,0,0.2)', borderRadius: 6, overflow: 'auto' }}>
                            {JSON.stringify(verify, null, 2)}
                        </pre>
                    )}
                    {error && <div style={{ marginTop: 10, color: '#ff8b80', fontSize: 12 }}>{error}</div>}
                </div>

                <div style={{ fontWeight: 600, marginBottom: 8 }}>{activeLesson.title}</div>
                <p className="afi-desc">{activeLesson.summary}</p>
                <ol className="afi-muted" style={{ paddingLeft: 18, margin: '10px 0' }}>
                    {activeLesson.steps.map((s, i) => <li key={i} style={{ marginBottom: 4 }}>{s}</li>)}
                </ol>
                <pre style={{
                    fontSize: 11, padding: 12, borderRadius: 6,
                    background: 'rgba(0,0,0,0.25)', border: '1px solid var(--readability-panel-border)',
                    overflow: 'auto', whiteSpace: 'pre-wrap',
                }}>{activeLesson.code}</pre>
                <div style={{ display: 'flex', gap: 8, marginTop: 10 }}>
                    <button type="button" className="settings-button" onClick={() => runInTerminal(activeLesson.code)}>
                        Run in terminal
                    </button>
                    {activeLesson.runHint && (
                        <span className="afi-subtle" style={{ alignSelf: 'center' }}>{activeLesson.runHint}</span>
                    )}
                </div>
            </div>

            <aside style={{ width: 220, flexShrink: 0 }}>
                <div className="afi-table-head" style={{ marginBottom: 8 }}>Guided path</div>
                {PYTORCH_BEGINNER_LESSONS.map((lesson) => (
                    <button
                        key={lesson.id}
                        type="button"
                        onClick={() => setActiveLesson(lesson)}
                        style={{
                            display: 'block', width: '100%', textAlign: 'left',
                            padding: '8px 10px', marginBottom: 4, borderRadius: 6, cursor: 'pointer',
                            border: activeLesson.id === lesson.id ? '1px solid #ee4c2c' : '1px solid var(--readability-panel-border)',
                            background: activeLesson.id === lesson.id ? 'rgba(238,76,44,0.12)' : 'transparent',
                            color: 'inherit', fontSize: 12,
                        }}
                    >
                        <div style={{ fontWeight: 600 }}>{lesson.title}</div>
                        <div className="afi-subtle">{lesson.level} · {lesson.minutes} min</div>
                    </button>
                ))}
                <p className="afi-subtle" style={{ marginTop: 12 }}>
                    Official docs:{' '}
                    <a href="https://pytorch.org/tutorials/" target="_blank" rel="noreferrer" style={{ color: '#ee4c2c' }}>
                        pytorch.org/tutorials
                    </a>
                </p>
            </aside>
        </div>
    );
};

export default PyTorchLearningPanel;
