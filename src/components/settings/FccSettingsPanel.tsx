import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../../store';

/**
 * FCC Settings Panel — configure and manage the Free Claude Code proxy sidecar.
 * Accessible from Settings → AI Models → Free Claude Code.
 */
const FccSettingsPanel: React.FC = () => {
    const fccUrl = useStore(s => s.fccUrl);
    const fccStatus = useStore(s => s.fccStatus);
    const fccEnabled = useStore(s => s.fccEnabled);
    const setFccUrl = useStore(s => s.setFccUrl);
    const setFccEnabled = useStore(s => s.setFccEnabled);
    const checkFccStatus = useStore(s => s.checkFccStatus);
    const inferenceBackend = useStore(s => s.inferenceBackend);
    const setInferenceBackend = useStore(s => s.setInferenceBackend);

    const [starting, setStarting] = useState(false);
    const [stopping, setStopping] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [envCheck, setEnvCheck] = useState<any>(null);

    useEffect(() => {
        if (fccEnabled) {
            checkFccStatus();
            // Check Python environment
            invoke<any>('fcc_check_env').then(setEnvCheck).catch(() => {});
        }
    }, [fccEnabled]);

    const handleStart = async () => {
        setStarting(true);
        setError(null);
        try {
            await invoke('fcc_start');
            await checkFccStatus();
        } catch (err: any) {
            setError(String(err));
        } finally {
            setStarting(false);
        }
    };

    const handleStop = async () => {
        setStopping(true);
        setError(null);
        try {
            await invoke('fcc_stop');
            await checkFccStatus();
        } catch (err: any) {
            setError(String(err));
        } finally {
            setStopping(false);
        }
    };

    const handleOpenAdmin = async () => {
        try {
            await invoke('fcc_open_admin');
        } catch (err: any) {
            setError(String(err));
        }
    };

    const statusColor = fccStatus === 'running'? '#10b981'
: fccStatus === 'checking'? '#f59e0b'
: fccStatus === 'error'? '#ef4444'
: 'rgba(255,255,255,0.4)';

    const statusLabel = fccStatus === 'running'? 'Running'
: fccStatus === 'checking'? 'Checking...'
: fccStatus === 'error'? 'Error'
: 'Stopped';

    return (
        <div style={{ padding: '16px', display: 'flex', flexDirection: 'column', gap: '16px' }}>
            {/* Header */}
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className="codicon codicon-server-process" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '16px', color: 'var(--vscode-textLink-foreground)' }} />
                    <span style={{ fontSize: '14px', fontWeight: 600 }}>Free Claude Code (FCC)</span>
                </div>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <div style={{
                        width: '8px', height: '8px', borderRadius: '50%',
                        background: statusColor,
                        boxShadow: fccStatus === 'running'? `0 0 6px ${statusColor}`: 'none',
                    }} />
                    <span style={{ fontSize: '11px', opacity: 0.7 }}>{statusLabel}</span>
                </div>
            </div>

            {/* Description */}
            <div style={{ fontSize: '12px', opacity: 0.7, lineHeight: 1.5 }}>
                FCC is a proxy that routes Claude Code / Codex traffic through 19+ providers
                (Lemonade, OpenRouter, NVIDIA NIM, etc.). Enable it to use your Claude
                Code subscription with VSCodium-Rust.
            </div>

            {/* Environment check */}
            {envCheck && (
                <div style={{
                    padding: '10px 12px', borderRadius: '4px', fontSize: '11px',
                    background: envCheck.ready? 'rgba(16,185,129,0.08)': 'rgba(251,191,36,0.08)',
                    border: `1px solid ${envCheck.ready? 'rgba(16,185,129,0.2)': 'rgba(251,191,36,0.2)'}`,
                }}>
                    <div style={{ fontWeight: 600, marginBottom: '6px', fontSize: '12px' }}>
                        {envCheck.ready? ' Environment ready': ' Setup required'}
                    </div>
                    <div style={{ display: 'flex', flexDirection: 'column', gap: '3px', opacity: 0.8 }}>
                        <div>uv: {envCheck.uv_available? ` ${envCheck.uv_version}`: ' not found'}</div>
                        <div>Python: {envCheck.python_available? ` ${envCheck.python_version}`: ' not found'}</div>
                        <div>FCC directory: {envCheck.fcc_dir_exists? ' found': ' not found'}</div>
                        <div>Dependencies: {envCheck.fcc_deps_installed? ' installed': ' not installed'}</div>
                    </div>
                    {!envCheck.ready && (
                        <div style={{ marginTop: '8px', lineHeight: 1.6 }}>
                            <div style={{ fontWeight: 600, marginBottom: '4px' }}>To set up FCC:</div>
                            <ol style={{ margin: '0', paddingLeft: '16px' }}>
                                <li>Install <a href="https://docs.astral.sh/uv/" target="_blank" rel="noopener" style={{ color: 'var(--vscode-textLink-foreground)' }}>uv</a> (Python package manager)</li>
                                <li>Run: <code style={{ background: 'rgba(0,0,0,0.2)', padding: '1px 4px', borderRadius: '2px' }}>uv python install 3.14.0</code></li>
                                <li>Run: <code style={{ background: 'rgba(0,0,0,0.2)', padding: '1px 4px', borderRadius: '2px' }}>cd third_party/free-claude-code && uv sync</code></li>
                            </ol>
                        </div>
                    )}
                </div>
            )}

            {/* Enable toggle */}
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '8px 12px', background: 'var(--vscode-input-background)', borderRadius: '4px' }}>
                <span style={{ fontSize: '12px' }}>Enable FCC sidecar</span>
                <div
                    onClick={() => setFccEnabled(!fccEnabled)}
                    style={{
                        width: '36px', height: '20px', borderRadius: '10px', cursor: 'pointer',
                        background: fccEnabled? 'var(--vscode-button-background)': 'rgba(255,255,255,0.1)',
                        position: 'relative', transition: 'background 0.2s',
                    }}
                >
                    <div style={{
                        width: '14px', height: '14px', borderRadius: '50%',
                        background: fccEnabled? 'var(--vscode-button-foreground)': 'rgba(255,255,255,0.4)',
                        position: 'absolute', top: '3px',
                        left: fccEnabled? '19px': '3px',
                        transition: 'left 0.2s',
                    }} />
                </div>
            </div>

            {/* URL */}
            <div>
                <label style={{ fontSize: '11px', opacity: 0.6, display: 'block', marginBottom: '4px' }}>Proxy URL</label>
                <input
                    value={fccUrl}
                    onChange={e => setFccUrl(e.target.value)}
                    placeholder="http://127.0.0.1:8082"
                    style={{
                        width: '100%', padding: '6px 10px', fontSize: '12px',
                        background: 'var(--vscode-input-background)',
                        color: 'var(--vscode-input-foreground)',
                        border: '1px solid var(--vscode-input-border)',
                        borderRadius: '4px', fontFamily: 'monospace',
                    }}
                />
            </div>

            {/* Actions */}
            <div style={{ display: 'flex', gap: '8px' }}>
                {fccStatus !== 'running'? (
                    <button
                        onClick={handleStart}
                        disabled={starting || !fccEnabled}
                        style={{
                            flex: 1, padding: '8px', fontSize: '12px', fontWeight: 600,
                            background: 'var(--vscode-button-background)',
                            color: 'var(--vscode-button-foreground)',
                            border: 'none', borderRadius: '4px', cursor: starting? 'wait': 'pointer',
                            opacity: starting || !fccEnabled? 0.5: 1,
                        }}
                    >
                        {starting? 'Starting...': 'Start FCC'}
                    </button>
                ): (
                    <button
                        onClick={handleStop}
                        disabled={stopping}
                        style={{
                            flex: 1, padding: '8px', fontSize: '12px', fontWeight: 600,
                            background: 'rgba(239,68,68,0.15)',
                            color: '#f87171',
                            border: '1px solid rgba(239,68,68,0.3)',
                            borderRadius: '4px', cursor: stopping? 'wait': 'pointer',
                            opacity: stopping? 0.5: 1,
                        }}
                    >
                        {stopping? 'Stopping...': 'Stop FCC'}
                    </button>
                )}
                <button
                    onClick={handleOpenAdmin}
                    disabled={fccStatus !== 'running'}
                    style={{
                        padding: '8px 16px', fontSize: '12px',
                        background: 'rgba(255,255,255,0.06)',
                        color: 'var(--vscode-foreground)',
                        border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '4px',
                        cursor: fccStatus !== 'running'? 'not-allowed': 'pointer',
                        opacity: fccStatus !== 'running'? 0.4: 1,
                    }}
                >
                    <i className="codicon codicon-globe" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }} />
                    Admin UI
                </button>
            </div>

            {/* Use as backend */}
            {fccStatus === 'running' && inferenceBackend !== 'fcc' && (
                <button
                    onClick={() => setInferenceBackend('fcc')}
                    style={{
                        padding: '8px', fontSize: '12px',
                        background: 'rgba(16,185,129,0.1)',
                        color: '#10b981',
                        border: '1px solid rgba(16,185,129,0.3)',
                        borderRadius: '4px', cursor: 'pointer',
                    }}
                >
                    <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginRight: '4px' }} />
                    Set as inference backend
                </button>
            )}

            {inferenceBackend === 'fcc' && (
                <div style={{ padding: '6px 10px', fontSize: '11px', background: 'rgba(16,185,129,0.08)', borderRadius: '4px', color: '#10b981' }}>
                    FCC is the active inference backend
                </div>
            )}

            {/* Error */}
            {error && (
                <div style={{ padding: '8px 12px', fontSize: '11px', background: 'rgba(239,68,68,0.1)', border: '1px solid rgba(239,68,68,0.2)', borderRadius: '4px', color: '#f87171' }}>
                    {error}
                </div>
            )}

            {/* Info */}
            <div style={{ fontSize: '11px', opacity: 0.5, lineHeight: 1.5 }}>
                FCC requires Python 3.14+ and uv. Configure providers and model routing
                through the Admin UI at <code>http://127.0.0.1:8082/admin</code>.
            </div>
        </div>
    );
};

export default FccSettingsPanel;
