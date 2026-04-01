import React, { useEffect, useState, useRef, useMemo } from 'react';
import { marked } from 'marked';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import AgentSettingsView from './AgentSettingsView';
import MissionControl from './agent/MissionControl';
import ResearchCenter from './agent/ResearchCenter';
import ContextSidebar from './visual/ContextSidebar';

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

marked.setOptions({
    gfm: true,
    breaks: true,
    silent: true
});

const AnePerformancePane: React.FC = () => {
    const [benchResult, setBenchResult] = useState<any>(null);
    const [isRunning, setIsRunning] = useState(false);
    const [error, setError] = useState<string | null>(null);

    const runBenchmark = async () => {
        setIsRunning(true);
        setError(null);
        try {
            const result = await invoke('benchmark_ane');
            setBenchResult(result);
        } catch (e: any) {
            setError(e.toString());
        } finally {
            setIsRunning(false);
        }
    };

    return (
        <SidebarPane title="Apple Neural Engine" defaultCollapsed={false}>
            <div style={{ padding: '0 12px 10px', fontSize: '12px', color: 'var(--vscode-sideBar-foreground)' }}>
                <div style={{ marginBottom: '12px', opacity: 0.8, lineHeight: '1.4' }}>
                    Utilize the NPU for high-efficiency local AI inference (M1/M2/M3/M4).
                </div>

                {benchResult ? (
                    <div style={{ background: 'rgba(255,255,255,0.03)', padding: '10px', borderRadius: '4px', marginBottom: '12px', border: '1px solid rgba(255,255,255,0.1)' }}>
                        <div style={{ fontWeight: 600, marginBottom: '8px', color: 'var(--vscode-charts-green)', display: 'flex', alignItems: 'center' }}>
                            <i className="codicon codicon-check" style={{ marginRight: '6px' }}></i>
                            ANE Active
                        </div>
                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px', opacity: 0.9 }}>
                            <div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Compile Time</div>
                                <div>{benchResult.compile_ms || '-'} ms</div>
                            </div>
                            <div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Eval Latency</div>
                                <div>{benchResult.eval_us} μs</div>
                            </div>
                            <div style={{ gridColumn: 'span 2' }}>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>Est. Throughput</div>
                                <div style={{ color: 'var(--vscode-charts-blue)' }}>~{(0.7 * 0.7 * 2 / (benchResult.eval_us / 1000000)).toFixed(2)} GFLOPS</div>
                            </div>
                        </div>
                    </div>
                ) : (
                    <div style={{ padding: '10px', textAlign: 'center', background: 'rgba(255,255,255,0.02)', borderRadius: '4px', marginBottom: '12px', border: '1px dashed rgba(255,255,255,0.1)' }}>
                        <div style={{ opacity: 0.5, marginBottom: '10px' }}>No active ANE session</div>
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
                                fontSize: '11px'
                            }}
                        >
                            {isRunning ? 'Initializing ANE...' : 'Initialize & Benchmark'}
                        </button>
                    </div>
                )}

                {error && (
                    <div style={{ color: 'var(--vscode-errorForeground)', fontSize: '11px', marginTop: '8px', padding: '8px', background: 'rgba(255,0,0,0.1)', borderRadius: '4px' }}>
                        {error}
                    </div>
                )}
            </div>
        </SidebarPane>
    );
};

const RightSidebar: React.FC = () => {
    const isOpen = useStore(state => state.isRightSidebarOpen);
    const toggle = useStore(state => state.toggleRightSidebar);
    const aiStatus = useStore(state => state.aiStatus || 'idle');
    const [view, setView] = useState<'chat' | 'history' | 'settings' | 'dashboard' | 'research' | 'context'>('chat');
    const inputRef = useRef<HTMLTextAreaElement>(null);
    const mode = useStore(state => state.agentMode);
    const model = useStore(state => state.agentModel);
    const messages = useStore(state => state.agentMessages);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const addAgentMessage = useStore(state => state.addAgentMessage);
    const updateLastAgentMessage = useStore(state => state.updateLastAgentMessage);
    const setIsAgentThinking = useStore(state => state.setIsAgentThinking);
    const clearAgentMessages = useStore(state => state.clearAgentMessages);
    const resetThread = useStore(state => state.resetThread);
    const pendingChanges = useStore(state => state.pendingChanges);
    const truncateAgentMessages = useStore(state => state.truncateAgentMessages);
    const [sessionAge, setSessionAge] = useState<string>('');
    const attachedContext = useStore(state => state.attachedContext);
    const addAttachedContext = useStore(state => state.addAttachedContext);
    const removeAttachedContext = useStore(state => state.removeAttachedContext);
    const clearAttachedContext = useStore(state => state.clearAttachedContext);
    const agentRootAccess = useStore(state => state.agentRootAccess);
    const setAgentRootAccess = useStore(state => state.setAgentRootAccess);
    const fileTree = useStore(state => state.fileTree);
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const agentTasks = useStore(state => state.agentTasks);
    const [inputValue, setInputValue] = useState('');
    const [isMentionDropdownOpen, setIsMentionDropdownOpen] = useState(false);
    const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);
    const [isHelpOpen, setIsHelpOpen] = useState(false);

    const allFiles = useMemo(() => {
        const flatten = (entries: FileEntry[]): FileEntry[] => {
            let res: FileEntry[] = [];
            for (const e of entries) {
                if (!e.is_dir) res.push(e);
                if (e.children) res.push(...flatten(e.children));
            }
            return res;
        };
        return flatten(fileTree);
    }, [fileTree]);

    const filteredSuggestions = useMemo(() => {
        const lastWord = inputValue.split(/\s+/).pop() || '';
        if (!lastWord.startsWith('@')) return [];
        const query = lastWord.slice(1).toLowerCase();
        return allFiles.filter(f => f.name.toLowerCase().includes(query)).slice(0, 10);
    }, [inputValue, allFiles]);

    useEffect(() => {
        if (messages.length > 0 && messages[0].timestamp) {
            const updateAge = () => {
                const diff = Date.now() - messages[0].timestamp;
                const mins = Math.floor(diff / 60000);
                const hrs = Math.floor(mins / 60);
                if (hrs > 0) setSessionAge(`${hrs}h ${mins % 60}m`);
                else setSessionAge(`${mins}m`);
            };
            updateAge();
            const interval = setInterval(updateAge, 60000);
            return () => clearInterval(interval);
        } else {
            setSessionAge('');
        }
    }, [messages]);

    useEffect(() => {
        const container = document.querySelector('.right-sidebar-messages');
        if (container) {
            const isNearBottom = container.scrollHeight - container.scrollTop - container.clientHeight < 100;
            if (isNearBottom) {
                messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
            }
        }
    }, [messages]);

    if (!isOpen) return null;

    const onSend = async () => {
        const val = inputValue.trim();
        if (val && !isAgentThinking) {
            setInputValue("");
            setIsMentionDropdownOpen(false);
            if (inputRef.current) inputRef.current.style.height = 'auto';

            const context = [...attachedContext];
            setIsAgentThinking(true);
            addAgentMessage('user', val, context);
            clearAttachedContext();
            addAgentMessage('assistant', "");

            try {
                const m = await import('../agent');
                await m.sendAgentMessage(val, () => { });
            } catch (err: any) {
                console.error('Agent chat failed:', err);
                const errorMsg = err.message || JSON.stringify(err);
                updateLastAgentMessage(`Error: ${errorMsg}`);
            } finally {
                setIsAgentThinking(false);
            }
        }
    };

    const onModeClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModeDropdown(target, () => { }));
    };

    const onModelClick = (e: React.MouseEvent) => {
        const target = e.currentTarget as HTMLElement;
        import('../agent').then(m => m.openModelDropdown(target, () => { }));
    };

    const handleDragOver = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
    };

    const handleDrop = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                addAttachedContext({
                    type: file.type.startsWith('image/') ? 'attachment' : 'file',
                    id: `dropped-${Date.now()}-${i}`,
                    name: file.name,
                    path: (file as any).path || file.name
                });
            }
        }
    };

    const handleMentionSelect = (file: FileEntry) => {
        const words = inputValue.split(/\s+/);
        words[words.length - 1] = `@${file.name}`;
        const newValue = words.join(' ') + ' ';
        setInputValue(newValue);
        setIsMentionDropdownOpen(false);
        addAttachedContext({
            id: file.path,
            type: 'file',
            name: file.name,
            path: file.path
        });
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (isMentionDropdownOpen && filteredSuggestions.length > 0) {
            if (e.key === 'ArrowDown') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev + 1) % filteredSuggestions.length);
            } else if (e.key === 'ArrowUp') {
                e.preventDefault();
                setSelectedMentionIndex(prev => (prev - 1 + filteredSuggestions.length) % filteredSuggestions.length);
            } else if (e.key === 'Enter') {
                e.preventDefault();
                handleMentionSelect(filteredSuggestions[selectedMentionIndex]);
            } else if (e.key === 'Escape') {
                setIsMentionDropdownOpen(false);
            }
        } else if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            onSend();
        }
    };

    const handleInputChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
        const val = e.target.value;
        setInputValue(val);

        if (inputRef.current) {
            inputRef.current.style.height = 'auto';
            inputRef.current.style.height = `${Math.min(e.target.scrollHeight, 200)}px`;
        }

        const lastWord = val.split(/\s+/).pop() || '';
        setIsMentionDropdownOpen(lastWord.startsWith('@'));
        setSelectedMentionIndex(0);
    };

    return (
        <aside
            onDragOver={handleDragOver}
            onDrop={handleDrop}
            className="right-sidebar" id="right-sidebar"
            style={{
                background: 'var(--vscode-sideBar-background)',
                borderLeft: '1px solid var(--vscode-sideBar-border, rgba(0,0,0,0.2))',
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                overflow: 'hidden',
                position: 'relative'
            }}>
            <style>{`
                .agent-message-container:hover .message-actions {
                    opacity: 1 !important;
                }
                .hoverable:hover {
                    background: rgba(255,255,255,0.1) !important;
                    color: #fff;
                }
                .markdown-content p { margin: 0 0 1em 0; }
                .markdown-content p:last-child { margin-bottom: 0; }
                .markdown-content pre { 
                    background: rgba(0,0,0,0.3); 
                    padding: 12px; 
                    border-radius: 8px; 
                    overflow-x: auto;
                    border: 1px solid rgba(255,255,255,0.05);
                    margin: 12px 0;
                }
                .markdown-content code {
                    font-family: var(--font-mono);
                    background: rgba(255,255,255,0.1);
                    padding: 2px 4px;
                    border-radius: 4px;
                    font-size: 0.9em;
                }
                .markdown-content h1, .markdown-content h2, .markdown-content h3 {
                    margin: 1.5em 0 0.5em 0;
                    font-weight: 600;
                    color: #fff;
                }
                .help-modal-overlay {
                    position: absolute;
                    top: 0; left: 0; right: 0; bottom: 0;
                    background: rgba(0,0,0,0.8);
                    backdrop-filter: blur(4px);
                    z-index: 1000;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 20px;
                }
                .help-modal-content {
                    background: var(--vscode-sideBar-background);
                    border: 1px solid var(--vscode-sideBar-border);
                    border-radius: 12px;
                    width: 100%;
                    max-height: 80%;
                    overflow-y: auto;
                    padding: 24px;
                    box-shadow: 0 10px 40px rgba(0,0,0,0.5);
                }
            `}</style>

            <div className="sidebar-header" style={{
                padding: '12px 16px',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'space-between',
                borderBottom: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))',
                background: 'var(--vscode-sideBar-background)'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className={`codicon codicon-shield${isAgentThinking ? ' codicon-modifier-spin' : ''}`} style={{
                        fontFamily: 'codicon',
                        fontStyle: 'normal',
                        fontSize: '14px',
                        color: '#ff4d4f'
                    }}></i>
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                        <span style={{ fontSize: '11px', fontWeight: 600, letterSpacing: '0.05em', opacity: 0.8 }}>VSCODIUM-RUST</span>
                        {sessionAge && <span style={{ fontSize: '9px', opacity: 0.4, fontWeight: 400 }}>Active: {sessionAge}</span>}
                    </div>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                    <div onClick={() => setView('chat')} style={{ cursor: 'pointer', opacity: view === 'chat' ? 1 : 0.4 }} title="Chat"><i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('history')} style={{ cursor: 'pointer', opacity: view === 'history' ? 1 : 0.4 }} title="History"><i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('dashboard')} style={{ cursor: 'pointer', opacity: view === 'dashboard' ? 1 : 0.4 }} title="Dashboard"><i className="codicon codicon-dashboard" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('context')} style={{ cursor: 'pointer', opacity: view === 'context' ? 1 : 0.4 }} title="Workspace Context"><i className="codicon codicon-hubot" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('settings')} style={{ cursor: 'pointer', opacity: view === 'settings' ? 1 : 0.4 }} title="Settings"><i className="codicon codicon-settings-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setIsHelpOpen(true)} style={{ cursor: 'pointer', opacity: 0.8, color: '#3b82f6' }} title="Command Help"><i className="codicon codicon-question" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={toggle} style={{ cursor: 'pointer', opacity: 0.5 }} title="Close"><i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                </div>
            </div>

            {isHelpOpen && (
                <div className="help-modal-overlay" onClick={() => setIsHelpOpen(false)}>
                    <div className="help-modal-content" onClick={e => e.stopPropagation()}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '20px' }}>
                            <h3 style={{ margin: 0, fontSize: '14px', textTransform: 'uppercase', letterSpacing: '0.1em' }}>Command Reference</h3>
                            <i className="codicon codicon-close" style={{ cursor: 'pointer', fontFamily: 'codicon', fontStyle: 'normal' }} onClick={() => setIsHelpOpen(false)}></i>
                        </div>
                        <div className="markdown-content" style={{ fontSize: '12px', opacity: 0.9 }}>
                            <p><strong>CORE COMMANDS:</strong></p>
                            <ul>
                                <li><code>/doctor</code> - Run system environment diagnostics.</li>
                                <li><code>/help</code> - Show this reference in chat.</li>
                                <li><code>/tools</code> - List all available tools & schemas.</li>
                                <li><code>/clear</code> - Reset conversation context.</li>
                                <li><code>/resume</code> - Restore last persistent session.</li>
                            </ul>
                            <p><strong>ENGINEERING COMMANDS:</strong></p>
                            <ul>
                                <li><code>/diff</code> - View workspace changes.</li>
                                <li><code>/commit</code> - Automated staging and committing.</li>
                                <li><code>/compact</code> - Compress chat history.</li>
                            </ul>
                            <p><strong>REASONING TIERS:</strong></p>
                            <ul>
                                <li><code>/advisor &lt;model&gt;</code> - Delegate planning to high-tier model.</li>
                                <li><code>/ultraplan</code> - Trigger deep architectural reasoning loop.</li>
                                <li><code>/insights</code> - Generate project architectural report.</li>
                            </ul>
                            <p style={{ marginTop: '20px', fontSize: '10px', opacity: 0.5 }}><em>Integrates all features from the original Claude Code architecture.</em></p>
                        </div>
                    </div>
                </div>
            )}

            <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column' }}>
                {view === 'chat' ? (
                    <div style={{ display: 'flex', flexDirection: 'column', padding: '16px', gap: '20px' }}>
                        {messages.length === 0 ? (
                            <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.5 }}>
                                <i className="codicon codicon-copilot" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '48px', marginBottom: '16px', display: 'block' }}></i>
                                <div style={{ fontSize: '14px', fontWeight: 600, marginBottom: '8px' }}>How can I help you today?</div>
                                <div style={{ fontSize: '12px' }}>Ask me to write code, explain logic, or debug errors.</div>
                            </div>
                        ) : (
                            <>
                                {agentTasks.filter((t: any) => t.status === 'running' && t.id.includes('-')).map((task: any) => (
                                    <div key={task.id} style={{
                                        background: 'rgba(59, 130, 246, 0.05)',
                                        border: '1px solid rgba(59, 130, 246, 0.2)',
                                        padding: '12px',
                                        borderRadius: '8px',
                                        marginBottom: '12px',
                                        animation: 'pulse 2s infinite'
                                    }}>
                                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: '#3b82f6' }}></i>
                                                <span style={{ fontSize: '11px', fontWeight: 600 }}>{task.title}</span>
                                            </div>
                                            <span style={{ fontSize: '10px', opacity: 0.6 }}>{task.progress}%</span>
                                        </div>
                                        <div style={{ background: 'rgba(0,0,0,0.2)', height: '4px', borderRadius: '2px', overflow: 'hidden' }}>
                                            <div style={{ background: '#3b82f6', width: `${task.progress}%`, height: '100%', transition: 'width 0.3s ease' }}></div>
                                        </div>
                                        {task.message && <div style={{ fontSize: '10px', marginTop: '6px', opacity: 0.6 }}>{task.message}</div>}
                                    </div>
                                ))}
                                {messages.map((msg, idx) => (
                                    <div key={idx} className="agent-message-container" style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px', opacity: 0.5 }}>
                                            <i className={`codicon codicon-${msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'hubot' : 'sparkle') : 'account'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: msg.isSubAgentResponse ? '#3b82f6' : 'inherit' }}></i>
                                            <span style={{ fontSize: '11px', fontWeight: 800, color: msg.isSubAgentResponse ? '#3b82f6' : 'inherit' }}>{msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'SUB-AGENT' : 'TERMINATOR') : 'YOU'}</span>
                                        </div>
                                        <div style={{
                                            background: msg.role === 'user' ? 'var(--vscode-list-hoverBackground, rgba(59, 130, 246, 0.05))' : (msg.isSubAgentResponse ? 'rgba(59, 130, 246, 0.03)' : 'rgba(255, 255, 255, 0.01)'),
                                            padding: '12px 16px', borderRadius: '14px', border: msg.isSubAgentResponse ? '1px solid rgba(59, 130, 246, 0.1)' : '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))'
                                        }}>
                                            {msg.thoughts && (
                                                <details style={{ marginBottom: '8px', opacity: 0.6 }}>
                                                    <summary style={{ fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Thoughts process...</summary>
                                                    <div style={{ fontSize: '11px', padding: '8px', background: 'rgba(0,0,0,0.2)', borderRadius: '6px', marginTop: '4px' }}>{msg.thoughts}</div>
                                                </details>
                                            )}
                                            {msg.steps && msg.steps.length > 0 && (
                                                <div style={{ display: 'flex', flexDirection: 'column', gap: '4px', marginBottom: '8px' }}>
                                                    {msg.steps.map((step: any, sIdx: number) => {
                                                        const getIcon = (type?: string) => {
                                                            switch (type) {
                                                                case 'git': return 'git-branch';
                                                                case 'terminal': return 'terminal';
                                                                case 'filesystem': return 'file-code';
                                                                case 'browser': return 'browser';
                                                                case 'system': return 'server-process';
                                                                default: return 'gear';
                                                            }
                                                        };
                                                        const getStatusColor = (status: string) => {
                                                            if (status === 'running') return '#3b82f6';
                                                            if (status === 'success') return '#10b981';
                                                            if (status === 'error') return '#ef4444';
                                                            return '#666';
                                                        };
                                                        return (
                                                            <div key={sIdx} style={{ display: 'flex', alignItems: 'center', gap: '8px', fontSize: '11px', opacity: 0.8 }}>
                                                                <i className={`codicon codicon-${getIcon(step.type)}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', color: getStatusColor(step.status) }}></i>
                                                                <span style={{ fontFamily: 'var(--font-mono)', opacity: 0.7 }}>{step.name}</span>
                                                                {step.status === 'running' && <i className="codicon codicon-sync codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', opacity: 0.4 }}></i>}
                                                            </div>
                                                        );
                                                    })}
                                                </div>
                                            )}
                                            <div className="markdown-content" style={{ fontSize: '13px', lineHeight: '1.6' }} dangerouslySetInnerHTML={{ __html: marked.parse(msg.content || "") as string }} />
                                        </div>
                                    </div>
                                ))}
                                {isAgentThinking && (
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '12px 16px', opacity: 0.5 }}>
                                        <i className="codicon codicon-sync codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                        <span style={{ fontSize: '11px', fontWeight: 600 }}>Thinking...</span>
                                    </div>
                                )}
                                <div ref={messagesEndRef} />
                            </>
                        )}
                    </div>
                ) : view === 'history' ? (
                    <div style={{ padding: '20px', opacity: 0.5, textAlign: 'center', fontSize: '12px' }}>No chat history found.</div>
                ) : view === 'dashboard' ? (
                    <MissionControl />
                ) : view === 'research' ? (
                    <ResearchCenter />
                ) : view === 'context' ? (
                    <ContextSidebar />
                ) : (
                    <AgentSettingsView />
                )}
            </div>
            {view === 'chat' && (
                <div style={{ padding: '16px', borderTop: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.1))' }}>
                    <div style={{
                        background: 'var(--vscode-input-background)', border: '1px solid var(--vscode-input-border, transparent)',
                        borderRadius: '12px', padding: '8px 12px', display: 'flex', flexDirection: 'column'
                    }}>
                        {attachedContext.length > 0 && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', marginBottom: '8px' }}>
                                {attachedContext.map((item, i) => (
                                    <div key={i} style={{ display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px', fontSize: '10px' }}>
                                        <span>{item.name}</span>
                                        <i className="codicon codicon-close" onClick={() => removeAttachedContext(i)} style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5 }}></i>
                                    </div>
                                ))}
                            </div>
                        )}
                        <textarea
                            ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown}
                            placeholder="Ask the Agent..."
                            style={{ background: 'transparent', border: 'none', outline: 'none', color: '#fff', resize: 'none', fontSize: '13px', lineHeight: '1.5', width: '100%', minHeight: '24px' }}
                        />
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '8px' }}>
                            <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                <span onClick={onModeClick} style={{ fontSize: '10px', opacity: 0.5, cursor: 'pointer' }} className="hoverable-bg">{mode}</span>
                                <span onClick={onModelClick} style={{ fontSize: '10px', opacity: 0.5, cursor: 'pointer' }} className="hoverable-bg">{(model.split('|')[1] || model).split(':')[0]}</span>
                            </div>
                            <div onClick={onSend} style={{ width: '24px', height: '24px', borderRadius: '50%', background: inputValue.trim() ? '#fff' : 'rgba(255,255,255,0.1)', color: '#000', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer' }}>
                                <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </aside>
    );
};

export default RightSidebar;
