import React, { useEffect, useState, useRef, useCallback, lazy, Suspense } from 'react';
import { useStore } from '../store';
import TerminalCommandPalette from './terminal/TerminalCommandPalette';
import {
    getActiveTerminalThemeMode,
    setActiveTerminalThemeMode,
} from '../application/terminal/refreshTerminalTheme';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '../tauri_bridge';

const TerminalView = lazy(() => import('./terminal/TerminalView'));
const DebugConsolePanel = lazy(() => import('./debug/DebugConsolePanel'));
const PortsPanel = lazy(() => import('./ports/PortsPanel'));
const LogcatPanel = lazy(() => import('./android/LogcatPanel'));

const PanelChunkFallback = () => (
    <div style={{ padding: 12, fontSize: 11, opacity: 0.45 }}>Loading panel…</div>
);

const BottomPanel: React.FC = () => {
    const isOpen = useStore(state => state.isBottomPanelOpen);
    const activeTab = useStore(state => state.activePanelTab);
    const setActiveTab = useStore(state => state.setActivePanelTab);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);

    const diagnosticsMap = useStore(state => state.diagnosticsMap);
    const terminalGroups = useStore(state => state.terminalGroups);
    const activeTerminalGroupId = useStore(state => state.activeTerminalGroupId);
    const addTerminalGroup = useStore(state => state.addTerminalGroup);
    const addAiriActivityTerminal = useStore(state => state.addAiriActivityTerminal);
    const splitTerminal = useStore(state => state.splitTerminal);
    const closeTerminalGroup = useStore(state => state.closeTerminalGroup);

    const [availableShells, setAvailableShells] = useState<string[]>([]);
    const [selectedShell, setSelectedShell] = useState<string>('');
    const [shellDropdownOpen, setShellDropdownOpen] = useState(false);
    const shellDropdownRef = useRef<HTMLDivElement>(null);
    const [outputLogs, setOutputLogs] = useState<string[]>(['[info] vscodium-rust initialized', '[info] Tauri backend connected']);
    const outputEndRef = useRef<HTMLDivElement>(null);
    const [backgroundJobs, setBackgroundJobs] = useState<any[]>([]);

    // Listen for output log events from backend
    useEffect(() => {
        const unlisten = listen<string>('output-log', (e) => {
            setOutputLogs(prev => [...prev.slice(-500), e.payload]);
        });
        // Also capture console-style agent step messages
        const unlisten2 = listen<any>('agent-step', (e) => {
            const { name, status } = e.payload ?? {};
            if (name) setOutputLogs(prev => [...prev.slice(-500), `[agent] ${status === 'running' ? '' : status === 'success' ? '✓' : '✗'} ${name}`]);
        });
        return () => { unlisten.then(f => f()); unlisten2.then(f => f()); };
    }, []);

    useEffect(() => {
        if (activeTab === 'OUTPUT') outputEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [outputLogs, activeTab]);

    useEffect(() => {
        let interval: NodeJS.Timeout;
        if (activeTab === 'JOBS') {
            const fetchJobs = () => {
                invoke<any[]>('get_background_jobs').then(jobs => {
                    setBackgroundJobs(jobs);
                }).catch(() => {});
            };
            fetchJobs();
            interval = setInterval(fetchJobs, 5000);
        }
        return () => clearInterval(interval);
    }, [activeTab]);

    // Load available shells on mount
    useEffect(() => {
        invoke<string[]>('get_available_shells').then(shells => {
            setAvailableShells(shells);
            if (shells.length > 0) setSelectedShell(shells[0]);
        }).catch(() => setAvailableShells([]));
    }, []);

    // Close dropdown on outside click
    useEffect(() => {
        const handler = (e: MouseEvent) => {
            if (shellDropdownRef.current && !shellDropdownRef.current.contains(e.target as Node))
                setShellDropdownOpen(false);
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, []);

    useEffect(() => {
        const unlisten = listen<boolean>('toggle-terminal', (event) => {
            if (event.payload) {
                setActiveTab('TERMINAL');
            } else if (isOpen) {
                toggleBottomPanel();
            }
        });
        return () => { unlisten.then(f => f()); };
    }, [isOpen, setActiveTab, toggleBottomPanel]);

    const activeGroup = terminalGroups.find((g: any) => g.id === activeTerminalGroupId);
    const agentTask = useStore(state => state.agentTask);

    // Short display name for a shell path
    function shellLabel(path: string) {
        if (!path) return 'Shell';
        const name = path.split(/[\\/]/).pop() || path;
        const lc = name.toLowerCase();
        if (lc.startsWith('pwsh')) return 'PowerShell 7';
        if (lc === 'powershell.exe') return 'PowerShell';
        if (lc === 'cmd.exe') return 'Command Prompt';
        if (lc === 'wsl.exe') return 'WSL';
        if (lc === 'bash.exe' || name === 'bash') return 'Bash';
        if (lc === 'zsh') return 'Zsh';
        return name.replace(/\.exe$/i, '');
    }

    if (!isOpen) return null;

    return (
        <div
            className="bottom-panel"
            style={{
                height: '100%',
                width: '100%',
                background: 'var(--vscode-panel-background)',
                borderTop: '1px solid var(--vscode-panel-border)',
                display: 'flex',
                flexDirection: 'column',
                zIndex: 10,
                position: 'relative',
                overflow: 'hidden'
            }}
        >
            {/* Terminal command palette (Ctrl+Shift+R) — fixed overlay, self-hides. */}
            <TerminalCommandPalette />

            <style>{`
                .bottom-panel .panel-tabs::-webkit-scrollbar { height: 0; display: none; }
                .bottom-panel .panel-tab { white-space: nowrap; flex-shrink: 0; }
                .bottom-panel .vscr-shell-pill:hover {
                    background: rgba(122,162,247,0.14) !important;
                    border-color: rgba(122,162,247,0.4) !important;
                }
                .bottom-panel .toolbar-icon {
                    display: flex; align-items: center; justify-content: center;
                    width: 26px; height: 26px; border-radius: 6px; cursor: pointer;
                    color: var(--vscode-foreground, #d5d8e0); opacity: 0.7;
                    transition: background .12s ease, opacity .12s ease;
                    flex-shrink: 0;
                }
                .bottom-panel .toolbar-icon:hover {
                    opacity: 1; background: rgba(255,255,255,0.08);
                }
                .bottom-panel .toolbar-icon i { font-size: 14px; }
            `}</style>

            {/* Header / Tabs */}
            <div className="panel-header" style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                height: '34px',
                padding: '0 8px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                fontSize: '12px',
                // Cursor-style: normal Title Case tabs, not UPPERCASE.
                textTransform: 'none',
                letterSpacing: '0'
            }}>
                <div className="panel-tabs" style={{ display: 'flex', gap: '2px', height: '100%', alignItems: 'center', minWidth: 0, flexShrink: 1, overflowX: 'auto', overflowY: 'hidden', scrollbarWidth: 'none' as any }}>
                    {['Problems', 'Output', 'Debug Console', 'Logcat', 'Terminal', 'Ports', 'Jobs'].map(tab => (
                        <div
                            key={tab}
                            className={`panel-tab ${activeTab === tab.toUpperCase() ? 'active' : ''}`}
                            onClick={() => setActiveTab(tab.toUpperCase() as any)}
                        >
                            {tab}
                            {tab === 'Problems' && (() => {
                                const total = Object.values(diagnosticsMap).reduce((s, d) => s + d.length, 0);
                                return total > 0 ? (
                                    <span style={{
                                        background: 'var(--terminator-accent)',
                                        color: 'var(--vscode-editor-foreground, #ffffff)',
                                        padding: '0px 6px',
                                        borderRadius: '10px',
                                        fontSize: '9px',
                                        fontWeight: 700,
                                        height: '14px',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                        marginLeft: '6px'
                                    }}>{total}</span>
                                ) : null;
                            })()}
                        </div>
                    ))}
                </div>

                <div className="panel-toolbar" style={{ display: 'flex', alignItems: 'center', gap: '8px', paddingRight: '12px', flexShrink: 0, marginLeft: '8px' }}>
                    {activeTab === 'TERMINAL' && (
                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                            {/* Shell selector dropdown */}
                            <div ref={shellDropdownRef} style={{ position: 'relative' }}>
                                <div
                                    className="vscr-shell-pill"
                                    onClick={() => setShellDropdownOpen(o => !o)}
                                    style={{
                                        display: 'flex', alignItems: 'center', gap: '6px',
                                        padding: '3px 8px', borderRadius: '6px',
                                        border: '1px solid ' + (shellDropdownOpen
                                            ? 'rgba(122,162,247,0.5)'
                                            : 'var(--vscode-panel-border, rgba(255,255,255,0.12))'),
                                        background: shellDropdownOpen
                                            ? 'rgba(122,162,247,0.18)'
                                            : 'rgba(255,255,255,0.04)',
                                        // The panel-header sets text-transform:uppercase + letter-spacing,
                                        // which cascades here and mangles the shell name — reset it.
                                        textTransform: 'none', letterSpacing: 'normal',
                                        fontSize: '11px', fontWeight: 500, lineHeight: 1.4,
                                        cursor: 'pointer', whiteSpace: 'nowrap',
                                        color: 'var(--vscode-foreground, #d5d8e0)',
                                        userSelect: 'none',
                                        transition: 'background .12s ease, border-color .12s ease',
                                    }}
                                    title="Select shell"
                                >
                                    <i className="codicon codicon-terminal" style={{ fontSize: '13px', opacity: 0.75, color: '#7aa2f7' }} />
                                    <span>{selectedShell ? shellLabel(selectedShell) : 'Shell'}</span>
                                    <i className="codicon codicon-chevron-down" style={{ fontSize: '10px', opacity: 0.5 }} />
                                </div>
                                {shellDropdownOpen && availableShells.length > 0 && (
                                    <div style={{
                                        // Open DOWNWARD (into the panel). Opening upward (bottom:100%)
                                        // put the menu above the panel header where the bottom-panel's
                                        // overflow:hidden clipped it — so it was invisible/unclickable.
                                        position: 'absolute', top: '100%', left: 0, marginTop: '6px',
                                        background: 'var(--vscode-menu-background, #252526)',
                                        color: 'var(--vscode-menu-foreground, #cccccc)',
                                        border: '1px solid var(--vscode-menu-border, var(--vscode-panel-border, #454545))',
                                        borderRadius: '6px', zIndex: 99999, minWidth: '200px',
                                        boxShadow: '0 8px 24px rgba(0,0,0,0.45)',
                                        // reset inherited uppercase/letter-spacing from panel-header
                                        textTransform: 'none', letterSpacing: 'normal',
                                        overflow: 'hidden',
                                    }}>
                                        <div style={{ padding: '4px 0' }}>
                                            <div style={{ padding: '4px 12px', fontSize: '10px', color: 'var(--vscode-editor-foreground, #888)', textTransform: 'uppercase', letterSpacing: '0.08em' }}>
                                                Select Default Shell
                                            </div>
                                            {availableShells.map(shell => (
                                                <div
                                                    key={shell}
                                                    onClick={() => {
                                                        // VSCode behavior: picking a profile sets the default AND
                                                        // immediately opens a NEW terminal with that shell.
                                                        setSelectedShell(shell);
                                                        setShellDropdownOpen(false);
                                                        addTerminalGroup(shell);
                                                    }}
                                                    style={{
                                                        padding: '5px 12px', fontSize: '12px', cursor: 'pointer',
                                                        color: shell === selectedShell ? 'var(--vscode-list-activeSelectionForeground, #fff)' : 'var(--vscode-menu-foreground, #cccccc)',
                                                        background: shell === selectedShell ? 'var(--vscode-list-activeSelectionBackground, #04395e)' : 'transparent',
                                                        display: 'flex', alignItems: 'center', gap: '8px',
                                                    }}
                                                    onMouseEnter={e => { if (shell !== selectedShell) (e.currentTarget as HTMLElement).style.background = 'var(--vscode-menu-selectionBackground, rgba(255,255,255,0.08))'; }}
                                                    onMouseLeave={e => { if (shell !== selectedShell) (e.currentTarget as HTMLElement).style.background = 'transparent'; }}
                                                >
                                                    <i className="codicon codicon-terminal" style={{ fontSize: '12px', opacity: 0.7 }} />
                                                    <span>{shellLabel(shell)}</span>
                                                    <span style={{ marginLeft: 'auto', fontSize: '10px', opacity: 0.4, fontFamily: 'monospace' }}>
                                                        {shell.split(/[\\/]/).pop()}
                                                    </span>
                                                </div>
                                            ))}
                                        </div>
                                    </div>
                                )}
                            </div>
                            <div className="toolbar-icon" onClick={() => addTerminalGroup(selectedShell || undefined)} title="New Terminal (Ctrl+`)">
                                <i className="codicon codicon-add" />
                            </div>
                            <div className="toolbar-icon" onClick={() => addAiriActivityTerminal({ focus: true })} title="Open AIRI Activity Terminal">
                                <i className="codicon codicon-radio-tower" />
                            </div>
                            <div className="toolbar-icon" onClick={() => activeGroup && splitTerminal(activeGroup.id, activeGroup.activeInstanceId, 'horizontal')} title="Split Right (Ctrl+Shift+5)">
                                <i className="codicon codicon-split-horizontal" />
                            </div>
                            <div className="toolbar-icon" onClick={() => activeGroup && splitTerminal(activeGroup.id, activeGroup.activeInstanceId, 'vertical')} title="Split Down (Ctrl+Alt+Shift+5)">
                                <i className="codicon codicon-split-vertical" />
                            </div>
                            <div
                                className="toolbar-icon"
                                onClick={() => {
                                    const next = getActiveTerminalThemeMode() === 'native' ? 'vscode' : 'native';
                                    void setActiveTerminalThemeMode(next);
                                }}
                                title="Toggle terminal theme (IDE / native)"
                            >
                                <i className="codicon codicon-color-mode" />
                            </div>
                            <div className="toolbar-icon" onClick={() => activeTerminalGroupId && closeTerminalGroup(activeTerminalGroupId)} title="Kill Terminal">
                                <i className="codicon codicon-trash" />
                            </div>
                        </div>
                    )}
                    <span style={{ height: '14px', width: '1px', background: 'var(--vscode-panel-border, rgba(255,255,255,0.1))', margin: '0 4px' }}></span>
                    <div className="toolbar-icon" title="Maximize Panel Size"><i className="codicon codicon-chevron-up"></i></div>
                    <div className="toolbar-icon" title="Close Panel" onClick={toggleBottomPanel}><i className="codicon codicon-close"></i></div>
                </div>
            </div>

            {/* Content Area */}
            <div className="panel-content" style={{ flex: 1, overflow: 'hidden', background: 'var(--vscode-terminal-background, var(--vscode-panel-background))' }}>
                {activeTab === 'TERMINAL' && (
                    <Suspense fallback={<PanelChunkFallback />}>
                        <TerminalView />
                    </Suspense>
                )}
                {activeTab === 'OUTPUT' && (
                    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ padding: '4px 12px', borderBottom: '1px solid var(--vscode-panel-border, #333)', display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexShrink: 0 }}>
                            <select style={{ background: 'transparent', color: 'var(--vscode-editor-foreground, #ccc)', border: 'none', fontSize: '11px', cursor: 'pointer', outline: 'none' }}>
                                <option>Main</option>
                                <option>Agent</option>
                                <option>Extension Host</option>
                            </select>
                            <div style={{ display: 'flex', gap: '8px', color: 'var(--vscode-editor-foreground, #666)' }}>
                                <i className="codicon codicon-clear-all" style={{ fontSize: '12px', cursor: 'pointer' }} title="Clear" onClick={() => setOutputLogs([])}></i>
                            </div>
                        </div>
                        <div style={{ flex: 1, overflowY: 'auto', padding: '8px 16px', fontFamily: 'var(--font-mono)', fontSize: '12px', lineHeight: '18px' }}>
                            {outputLogs.map((line, i) => {
                                const color = line.startsWith('[error]') ? '#f87171'
                                    : line.startsWith('[warn]') ? '#fbbf24'
                                    : line.startsWith('[agent]') ? '#60a5fa'
                                    : '#888';
                                return <div key={i} style={{ color, whiteSpace: 'pre-wrap', wordBreak: 'break-all' }}>{line}</div>;
                            })}
                            <div ref={outputEndRef} />
                        </div>
                    </div>
                )}
                {activeTab === 'DEBUG CONSOLE' && (
                    <Suspense fallback={<PanelChunkFallback />}>
                        <DebugConsolePanel />
                    </Suspense>
                )}
                {activeTab === 'LOGCAT' && (
                    <Suspense fallback={<PanelChunkFallback />}>
                        <LogcatPanel />
                    </Suspense>
                )}
                {activeTab === 'PROBLEMS' && (() => {
                    const entries = Object.entries(diagnosticsMap).filter(([, d]) => d.length > 0);
                    if (entries.length === 0) return (
                        <div style={{ padding: '32px', color: 'var(--vscode-editor-foreground, #666)', fontSize: '12px', textAlign: 'center' }}>
                            No problems detected.
                        </div>
                    );
                    const openFile = useStore.getState().openFile;
                    return (
                        <div style={{ overflowY: 'auto', height: '100%', fontFamily: 'var(--font-mono)', fontSize: '12px' }}>
                            {entries.map(([filePath, diags]) => (
                                <div key={filePath}>
                                    <div style={{ padding: '4px 12px', fontWeight: 600, fontSize: '11px', opacity: 0.7, background: 'rgba(255,255,255,0.03)', borderBottom: '1px solid rgba(255,255,255,0.05)' }}>
                                        {filePath.replace(/\\/g, '/').split('/').pop()}
                                        <span style={{ fontWeight: 400, opacity: 0.45, marginLeft: '6px', fontSize: '10px' }}>{filePath.replace(/\\/g, '/')}</span>
                                    </div>
                                    {diags.map((d, i) => {
                                        const isError = d.severity === 8;
                                        const isWarn = d.severity === 4;
                                        return (
                                            <div
                                                key={i}
                                                onClick={() => openFile(filePath).then(() => setTimeout(() => window.dispatchEvent(new CustomEvent('editor:jump-to-line', { detail: { path: filePath, line: d.startLine, column: d.startCol } })), 100))}
                                                style={{ padding: '3px 12px 3px 24px', cursor: 'pointer', display: 'flex', gap: '8px', alignItems: 'flex-start', borderBottom: '1px solid rgba(255,255,255,0.03)' }}
                                                onMouseEnter={e => (e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)')}
                                                onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
                                            >
                                                <i className={`codicon codicon-${isError ? 'error' : isWarn ? 'warning' : 'info'}`} style={{ color: isError ? '#f87171' : isWarn ? '#fbbf24' : '#60a5fa', fontSize: '12px', marginTop: '2px', flexShrink: 0 }} />
                                                <span style={{ flex: 1 }}>{d.message}</span>
                                                <span style={{ opacity: 0.4, flexShrink: 0, fontSize: '10px' }}>{d.startLine}:{d.startCol}</span>
                                                {d.source && <span style={{ opacity: 0.35, flexShrink: 0, fontSize: '10px' }}>[{d.source}]</span>}
                                            </div>
                                        );
                                    })}
                                </div>
                            ))}
                        </div>
                    );
                })()}
                {activeTab === 'PORTS' && (
                    <Suspense fallback={<PanelChunkFallback />}>
                        <PortsPanel />
                    </Suspense>
                )}
                {activeTab === 'JOBS' && (
                    <div style={{ padding: '16px', color: 'var(--vscode-foreground)', fontSize: '12px', height: '100%', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '16px', borderBottom: '1px solid var(--vscode-panel-border)', paddingBottom: '8px' }}>
                            <span style={{ fontWeight: 600 }}>Active Background Jobs</span>
                            <div style={{ display: 'flex', gap: '8px' }}>
                                <i className="codicon codicon-refresh" style={{ cursor: 'pointer', opacity: 0.7 }} title="Refresh Jobs"></i>
                                <i className="codicon codicon-clear-all" style={{ cursor: 'pointer', opacity: 0.7 }} title="Clear Completed"></i>
                            </div>
                        </div>
                        <div style={{ flex: 1, display: 'flex', flexDirection: 'column', gap: '8px' }}>
                            {backgroundJobs.length === 0 ? (
                                <div style={{ opacity: 0.5, fontStyle: 'italic', padding: '8px' }}>No active background jobs</div>
                            ) : backgroundJobs.map(job => (
                                <div key={job.id} style={{ display: 'flex', alignItems: 'center', gap: '12px', background: 'var(--vscode-list-hoverBackground)', padding: '8px 12px', borderRadius: '4px' }}>
                                    <div className={job.status === 'running' ? 'animate-spin' : ''} style={{ width: '12px', height: '12px', border: '2px solid var(--terminator-accent)', borderTopColor: 'transparent', borderRadius: '50%' }}></div>
                                    <div style={{ flex: 1 }}>
                                        <div style={{ fontWeight: 500 }}>{job.name}</div>
                                        <div style={{ opacity: 0.5, fontSize: '10px' }}>{job.status}... {job.progress}%</div>
                                    </div>
                                    <i className="codicon codicon-stop" style={{ cursor: 'pointer', color: '#f87171' }} title="Stop Job"></i>
                                </div>
                            ))}
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};

export default BottomPanel;
