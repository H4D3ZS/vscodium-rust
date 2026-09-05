import React, { useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';

const SidebarPane: React.FC<{ title: string; children: React.ReactNode; defaultCollapsed?: boolean; actions?: React.ReactNode }> = ({ title, children, defaultCollapsed = false, actions }) => {
    const [isCollapsed, setIsCollapsed] = useState(defaultCollapsed);
    return (
        <div className="sidebar-pane" style={{ display: 'flex', flexDirection: 'column', flexShrink: 0, borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))' }}>
            <div
                className={`pane-header${isCollapsed ? ' collapsed' : ''}`}
                onClick={() => setIsCollapsed(!isCollapsed)}
                style={{
                    padding: '6px 10px',
                    display: 'flex',
                    alignItems: 'center',
                    cursor: 'pointer',
                    background: 'var(--vscode-sideBarSectionHeader-background, rgba(255,255,255,0.02))',
                    fontSize: '11px',
                    fontWeight: 600,
                    textTransform: 'uppercase',
                    letterSpacing: '0.05em',
                    color: 'var(--vscode-sideBar-foreground)',
                    opacity: 0.8
                }}
            >
                <i className={`codicon codicon-chevron-${isCollapsed ? 'right' : 'down'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', marginRight: '8px', fontSize: '12px' }}></i>
                <span style={{ flex: 1 }}>{title}</span>
                {actions && <div className="pane-actions" onClick={e => e.stopPropagation()}>{actions}</div>}
            </div>
            {!isCollapsed && <div className="pane-content" style={{ padding: '8px 0' }}>{children}</div>}
        </div>
    );
};

const AnePerformancePane: React.FC = () => {
    const [benchResult, setBenchResult] = useState<any>(null);
    const [history, setHistory] = useState<any[]>([]);
    const [isRunning, setIsRunning] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [deviceMode, setDeviceMode] = useState<'ANE' | 'CPU' | 'GPU'>('ANE');
    const [ollamaModels, setOllamaModels] = useState<any[]>([]);

    const runBenchmark = async () => {
        setIsRunning(true);
        setError(null);
        try {
            const result: any = await invoke('benchmark_ane', { device: deviceMode });
            setBenchResult(result);
            fetchHistory();
        } catch (e: any) {
            setError(e.toString());
        } finally {
            setIsRunning(false);
        }
    };

    const fetchHistory = async () => {
        try {
            const data: any = await invoke('get_inference_history');
            setHistory(data);
        } catch (e) {
            console.error('Failed to fetch history:', e);
        }
    };

    // Loaded-model list comes straight from Lemonade's health endpoint; there is
    // no Rust command for it (the old `get_ollama_ps` went with Ollama).
    const fetchOllamaPs = async () => {
        try {
            const base = localStorage.getItem('provider.lemonade.url') || 'http://127.0.0.1:13305';
            const res = await fetch(`${base.replace(/\/$/, '')}/api/v1/health`);
            const data: any = await res.json();
            setOllamaModels(Array.isArray(data?.all_models_loaded) ? data.all_models_loaded : []);
        } catch (e) {
            console.error('Failed to fetch loaded Lemonade models:', e);
            setOllamaModels([]);
        }
    };

    useEffect(() => {
        fetchHistory();
        const interval = setInterval(fetchHistory, 5000);
        return () => clearInterval(interval);
    }, []);

    useEffect(() => {
        if (deviceMode === 'GPU') {
            fetchOllamaPs();
            const interval = setInterval(fetchOllamaPs, 3000);
            return () => clearInterval(interval);
        }
    }, [deviceMode]);

    return (
        <SidebarPane title="H4RDW4RE UNL3ASHED" defaultCollapsed={false}>
            <div style={{ padding: '0 12px 10px', fontSize: '12px', color: 'var(--vscode-sideBar-foreground)' }}>
                <div style={{ marginBottom: '12px', opacity: 0.8, lineHeight: '1.4' }}>
                    Monitor local AI inference performance across hardware backends.
                </div>

                <div style={{ display: 'flex', gap: '4px', marginBottom: '12px', background: 'rgba(0,0,0,0.2)', padding: '2px', borderRadius: '4px' }}>
                    <button
                        onClick={() => setDeviceMode('ANE')}
                        style={{
                            flex: 1,
                            padding: '4px',
                            fontSize: '10px',
                            border: 'none',
                            borderRadius: '3px',
                            background: deviceMode === 'ANE' ? 'var(--vscode-button-background)' : 'transparent',
                            color: deviceMode === 'ANE' ? 'var(--vscode-button-foreground)' : 'inherit',
                            cursor: 'pointer'
                        }}
                    >
                        Silicon (ANE)
                    </button>
                    <button
                        onClick={() => setDeviceMode('CPU')}
                        style={{
                            flex: 1,
                            padding: '4px',
                            fontSize: '10px',
                            border: 'none',
                            borderRadius: '3px',
                            background: deviceMode === 'CPU' ? 'var(--vscode-button-background)' : 'transparent',
                            color: deviceMode === 'CPU' ? 'var(--vscode-button-foreground)' : 'inherit',
                            cursor: 'pointer'
                        }}
                    >
                        Intel/PC (CPU)
                    </button>
                    <button
                        onClick={() => setDeviceMode('GPU')}
                        style={{
                            flex: 1,
                            padding: '4px',
                            fontSize: '10px',
                            border: 'none',
                            borderRadius: '3px',
                            background: deviceMode === 'GPU' ? 'var(--vscode-button-background)' : 'transparent',
                            color: deviceMode === 'GPU' ? 'var(--vscode-button-foreground)' : 'inherit',
                            cursor: 'pointer'
                        }}
                    >
                        PC (GPU)
                    </button>
                </div>

                {benchResult ? (
                    <div style={{ background: 'rgba(255,255,255,0.03)', padding: '10px', borderRadius: '4px', marginBottom: '12px', border: '1px solid rgba(255,255,255,0.1)' }}>
                        <div style={{ fontWeight: 600, marginBottom: '8px', color: 'var(--vscode-charts-green)', display: 'flex', alignItems: 'center' }}>
                            <i className="codicon codicon-pulse" style={{ marginRight: '6px' }}></i>
                            {benchResult.device} Benchmark
                        </div>
                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px', opacity: 0.9 }}>
                            <div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Latency</div>
                                <div>{benchResult.eval_us > 1000 ? `${(benchResult.eval_us / 1000).toFixed(2)} ms` : `${benchResult.eval_us} μs`}</div>
                            </div>
                            <div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Backend</div>
                                <div style={{ fontSize: '10px' }}>{deviceMode}</div>
                            </div>
                            <div style={{ gridColumn: 'span 2', marginTop: '4px' }}>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Est. Raw Throughput</div>
                                <div style={{ color: 'var(--vscode-charts-blue)', fontWeight: 600 }}>
                                    {deviceMode === 'ANE' ?
                                        `~${(0.7 * 0.7 * 2 / (benchResult.eval_us / 1000000)).toFixed(2)} GFLOPS` :
                                        deviceMode === 'GPU' ?
                                            `~${(12.4 * 2 / (benchResult.eval_us / 1000000)).toFixed(2)} GFLOPS` :
                                            `~${(0.1 * 1 / (benchResult.eval_us / 1000)).toFixed(2)} GFLOPS`
                                    }
                                </div>
                            </div>
                        </div>
                    </div>
                ) : (
                    <div style={{ padding: '10px', textAlign: 'center', background: 'rgba(255,255,255,0.02)', borderRadius: '4px', marginBottom: '12px', border: '1px dashed rgba(255,255,255,0.1)' }}>
                        <button
                            onClick={runBenchmark}
                            disabled={isRunning}
                            style={{
                                background: 'var(--vscode-button-background)',
                                color: 'var(--vscode-button-foreground)',
                                border: 'none',
                                padding: '4px 12px',
                                borderRadius: '2px',
                                cursor: isRunning ? 'wait' : 'pointer',
                                fontSize: '11px',
                                width: '100%'
                            }}
                        >
                            {isRunning ? 'Running Benchmark...' : `Benchmark ${deviceMode}`}
                        </button>
                    </div>
                )}

                {history.length > 0 && (
                    <div style={{ marginTop: '12px' }}>
                        <div style={{ fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5, marginBottom: '6px', display: 'flex', justifyContent: 'space-between' }}>
                            <span>Inference History</span>
                            <span onClick={runBenchmark} style={{ cursor: 'pointer', color: 'var(--vscode-charts-blue)' }}>Test Again</span>
                        </div>
                        <div style={{ maxHeight: '120px', overflowY: 'auto', border: '1px solid rgba(255,255,255,0.05)', borderRadius: '4px' }}>
                            {history.slice().reverse().map((stat, i) => (
                                <div key={i} style={{ padding: '4px 8px', fontSize: '10px', display: 'flex', justifyContent: 'space-between', borderBottom: '1px solid rgba(255,255,255,0.03)', opacity: 0.8 }}>
                                    <span style={{ color: stat.device === 'ANE' ? '#f472b6' : stat.device === 'GPU' ? '#00f2ff' : '#60a5fa' }}>{stat.device}</span>
                                    <span>{stat.latency_ms} ms</span>
                                </div>
                            ))}
                        </div>
                    </div>
                )}

                {error && (
                    <div style={{ color: 'var(--vscode-errorForeground)', fontSize: '11px', marginTop: '8px', padding: '8px', background: 'rgba(255,0,0,0.1)', borderRadius: '4px' }}>
                        {error}
                    </div>
                )}

                {deviceMode === 'GPU' && ollamaModels.length > 0 && (
                    <div style={{ marginTop: '16px', borderTop: '1px solid rgba(255,255,255,0.05)', paddingTop: '12px' }}>
                        <div style={{ fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5, marginBottom: '8px', display: 'flex', alignItems: 'center' }}>
                            <i className="codicon codicon-layers" style={{ marginRight: '6px' }}></i>
                            Active Ollama Models
                        </div>
                        <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                            {ollamaModels.map((model, i) => {
                                const vramGB = (model.size_vram / (1024 * 1024 * 1024)).toFixed(1);
                                const totalGB = (model.size / (1024 * 1024 * 1024)).toFixed(1);
                                const gpuPct = model.size > 0 ? Math.round((model.size_vram / model.size) * 100) : 0;

                                return (
                                    <div key={i} style={{ background: 'rgba(255,255,255,0.03)', padding: '8px', borderRadius: '4px', border: '1px solid rgba(255,255,255,0.05)' }}>
                                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                                            <span style={{ fontWeight: 600, color: 'var(--vscode-charts-blue)', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', marginRight: '8px' }}>
                                                {model.name}
                                            </span>
                                            <span style={{ fontSize: '9px', opacity: 0.6 }}>{totalGB} GB</span>
                                        </div>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '9px' }}>
                                            <div style={{ flex: 1, height: '4px', background: 'rgba(255,255,255,0.05)', borderRadius: '2px', overflow: 'hidden' }}>
                                                <div style={{ height: '100%', width: `${gpuPct}%`, background: gpuPct > 90 ? 'var(--vscode-charts-red)' : 'var(--vscode-charts-green)', transition: 'width 0.3s ease' }}></div>
                                            </div>
                                            <span style={{ minWidth: '40px', textAlign: 'right', color: gpuPct > 0 ? 'var(--vscode-charts-green)' : 'inherit' }}>
                                                {gpuPct}% GPU
                                            </span>
                                        </div>
                                        <div style={{ marginTop: '4px', fontSize: '9px', opacity: 0.5, display: 'flex', justifyContent: 'space-between' }}>
                                            <span>VRAM: {vramGB} GB</span>
                                            {model.expires_at && <span>Live</span>}
                                        </div>
                                    </div>
                                );
                            })}
                        </div>
                    </div>
                )}
            </div>
        </SidebarPane>
    );
};

export default AnePerformancePane;
