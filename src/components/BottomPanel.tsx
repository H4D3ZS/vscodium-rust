import React, { useEffect, useState, useRef, useCallback } from 'react';
import { useStore } from '../store';
import TerminalView from './terminal/TerminalView';
import Composer from './Composer';
import { listen } from '@tauri-apps/api/event';
import { invoke } from '../tauri_bridge';

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

    // Listen for output log events from backend
    useEffect(() => {
        const unlisten = listen<string>('output-log', (e) => {
            setOutputLogs(prev => [...prev.slice(-500), e.payload]);
        });
        // Also capture console-style agent step messages
        const unlisten2 = listen<any>('agent-step', (e) => {
            const { name, status } = e.payload ?? {};
            if (name) setOutputLogs(prev => [...prev.slice(-500), `[agent] ${status === 'running' ? '▶' : status === 'success' ? '✓' : '✗'} ${name}`]);
        });
        return () => { unlisten.then(f => f()); unlisten2.then(f => f()); };
    }, []);

    useEffect(() => {
        if (activeTab === 'OUTPUT') outputEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [outputLogs, activeTab]);

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
        if (!path) return 'shell';
        const name = path.split(/[\\/]/).pop() || path;
        if (name.toLowerCase().startsWith('pwsh')) return 'pwsh';
        if (name.toLowerCase() === 'powershell.exe') return 'powershell';
        if (name.toLowerCase() === 'cmd.exe') return 'cmd';
        if (name.toLowerCase() === 'wsl.exe') return 'wsl';
        if (name.toLowerCase() === 'bash.exe' || name === 'bash') return 'bash';
        if (name.toLowerCase() === 'zsh') return 'zsh';
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
            {/* Header / Tabs */}
            <div className="panel-header" style={{
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                height: '35px',
                padding: '0 8px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                fontSize: '11px',
                textTransform: 'uppercase',
                letterSpacing: '0.05em'
            }}>
                <div className="panel-tabs" style={{ display: 'flex', gap: '2px', height: '100%', alignItems: 'center' }}>
                    {['Problems', 'Output', 'Debug Console', 'Terminal', 'Composer', 'Ports'].map(tab => (
                        <div
                            key={tab}
                            className={`panel-tab ${activeTab === tab.toUpperCase() ? 'active' : ''}`}
                            onClick={() => setActiveTab(tab.toUpperCase() as any)}
                        >
                            {tab}
                            {tab === 'Composer' && (
                                <span style={{
                                    background: 'var(--vscode-badge-background)',
                                    color: 'var(--vscode-badge-foreground)',
                                    padding: '0px 6px',
                                    borderRadius: '10px',
                                    fontSize: '9px',
                                    fontWeight: 700,
                                    height: '14px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    marginLeft: '6px'
                                }}>0</span>
                            )}
                            {tab === 'Problems' && (() => {
                                const total = Object.values(diagnosticsMap).reduce((s, d) => s + d.length, 0);
                                return total > 0 ? (
                                    <span style={{
                                        background: 'var(--terminator-accent)',
                                        color: '#ffffff',
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

                <div className="panel-toolbar" style={{ display: 'flex', alignItems: 'center', gap: '8px', paddingRight: '12px' }}>
                    {activeTab === 'TERMINAL' && (
                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                            {/* Shell selector dropdown */}
                            <div ref={shellDropdownRef} style={{ position: 'relative' }}>
                                <div
                                    className="toolbar-item"
                                    onClick={() => setShellDropdownOpen(o => !o)}
                                    style={{
                                        display: 'flex', alignItems: 'center', gap: '6px',
                                        padding: '2px 8px', borderRadius: '4px',
                                        background: shellDropdownOpen
                                            ? 'var(--vscode-toolbar-hoverBackground, rgba(255,255,255,0.1))'
                                            : 'var(--vscode-badge-background, rgba(255,255,255,0.05))',
                                        fontSize: '11px', cursor: 'pointer',
                                        color: 'var(--vscode-badge-foreground, #ccc)',
                                        userSelect: 'none',
                                    }}
                                    title="Select shell"
                                >
                                    <i className="codicon codicon-terminal" style={{ fontSize: '13px', opacity: 0.7 }} />
                                    <span>{selectedShell ? shellLabel(selectedShell) : 'shell'}</span>
                                    <i className="codicon codicon-chevron-down" style={{ fontSize: '10px', opacity: 0.5 }} />
                                </div>
                                {shellDropdownOpen && availableShells.length > 0 && (
                                    <div style={{
                                        position: 'absolute', bottom: '100%', left: 0, marginBottom: '4px',
                                        background: 'var(--vscode-menu-background, #252526)',
                                        border: '1px solid var(--vscode-menu-border, #454545)',
                                        borderRadius: '4px', zIndex: 9999, minWidth: '180px',
                                        boxShadow: '0 4px 16px rgba(0,0,0,0.4)',
                                    }}>
                                        <div style={{ padding: '4px 0' }}>
                                            <div style={{ padding: '4px 12px', fontSize: '10px', color: '#888', textTransform: 'uppercase', letterSpacing: '0.08em' }}>
                                                Select Default Shell
                                            </div>
                                            {availableShells.map(shell => (
                                                <div
                                                    key={shell}
                                                    onClick={() => { setSelectedShell(shell); setShellDropdownOpen(false); }}
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
                            <div className="toolbar-icon" onClick={addAiriActivityTerminal} title="Open AIRI Activity Terminal">
                                <i className="codicon codicon-radio-tower" />
                            </div>
                            <div className="toolbar-icon" onClick={() => activeGroup && splitTerminal(activeGroup.id, activeGroup.activeInstanceId)} title="Split Terminal">
                                <i className="codicon codicon-split-horizontal" />
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
                {activeTab === 'TERMINAL' && <TerminalView />}
                {activeTab === 'COMPOSER' && <Composer />}
                {activeTab === 'OUTPUT' && (
                    <div style={{ height: '100%', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ padding: '4px 12px', borderBottom: '1px solid #333', display: 'flex', justifyContent: 'space-between', alignItems: 'center', flexShrink: 0 }}>
                            <select style={{ background: 'transparent', color: '#ccc', border: 'none', fontSize: '11px', cursor: 'pointer', outline: 'none' }}>
                                <option>Main</option>
                                <option>Agent</option>
                                <option>Extension Host</option>
                            </select>
                            <div style={{ display: 'flex', gap: '8px', color: '#666' }}>
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
                    <div style={{ height: '100%', display: 'flex', flexDirection: 'column', padding: '12px 16px' }}>
                        <div style={{ color: '#666', fontSize: '12px', fontStyle: 'italic', marginBottom: '8px' }}>
                            Debug Console is ready. No active debug session.
                        </div>
                        <div style={{ display: 'flex', alignItems: 'center', color: '#ccc', fontSize: '12px', marginTop: 'auto' }}>
                            <i className="codicon codicon-chevron-right" style={{ fontSize: '12px', marginRight: '8px', color: '#3794ef' }}></i>
                            <input
                                type="text"
                                placeholder="Filter or evaluate expression"
                                style={{ background: 'transparent', border: 'none', color: '#fff', fontSize: '12px', width: '100%', outline: 'none' }}
                            />
                        </div>
                    </div>
                )}
                {activeTab === 'PROBLEMS' && (() => {
                    const entries = Object.entries(diagnosticsMap).filter(([, d]) => d.length > 0);
                    if (entries.length === 0) return (
                        <div style={{ padding: '32px', color: '#666', fontSize: '12px', textAlign: 'center' }}>
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
                    <div style={{ padding: '32px', color: '#666', fontSize: '12px', textAlign: 'center' }}>
                        PORTS view is currently empty.
                    </div>
                )}
            </div>
        </div>
    );
};

export default BottomPanel;
