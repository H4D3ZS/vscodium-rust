import React, { useEffect, useState, useRef, useMemo } from 'react';
import { marked } from 'marked';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import AgentSettingsView from './AgentSettingsView';
import MissionControl from './agent/MissionControl';
import ResearchCenter from './agent/ResearchCenter';

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

// Configure marked options
marked.setOptions({
    gfm: true,
    breaks: true,
    silent: true
});

const RightSidebar: React.FC = () => {
    const isOpen = useStore(state => state.isRightSidebarOpen);
    const toggle = useStore(state => state.toggleRightSidebar);
    const aiStatus = useStore(state => state.aiStatus || 'idle');
    const [view, setView] = useState<'chat' | 'history' | 'settings' | 'dashboard' | 'research'>('chat');
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
    const attachedContext = useStore(state => state.attachedContext);
    const addAttachedContext = useStore(state => state.addAttachedContext);
    const removeAttachedContext = useStore(state => state.removeAttachedContext);
    const clearAttachedContext = useStore(state => state.clearAttachedContext);
    const agentRootAccess = useStore(state => state.agentRootAccess);
    const setAgentRootAccess = useStore(state => state.setAgentRootAccess);
    const fileTree = useStore(state => state.fileTree);
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const [inputValue, setInputValue] = useState('');
    const [isMentionDropdownOpen, setIsMentionDropdownOpen] = useState(false);
    const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);

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
                    <span style={{ fontSize: '11px', fontWeight: 600, letterSpacing: '0.05em', opacity: 0.8 }}>TERMINATOR</span>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                    <div onClick={() => setView('chat')} style={{ cursor: 'pointer', opacity: view === 'chat' ? 1 : 0.4 }} title="Chat"><i className="codicon codicon-comment-discussion" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('history')} style={{ cursor: 'pointer', opacity: view === 'history' ? 1 : 0.4 }} title="History"><i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('dashboard')} style={{ cursor: 'pointer', opacity: view === 'dashboard' ? 1 : 0.4 }} title="Dashboard"><i className="codicon codicon-dashboard" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('settings')} style={{ cursor: 'pointer', opacity: view === 'settings' ? 1 : 0.4 }} title="Settings"><i className="codicon codicon-settings-gear" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={toggle} style={{ cursor: 'pointer', opacity: 0.5 }} title="Close"><i className="codicon codicon-close" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                </div>
            </div>

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
                            messages.map((msg, idx) => (
                                <div key={idx} className="agent-message-container" style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px', opacity: 0.5 }}>
                                        <i className={`codicon codicon-${msg.role === 'assistant' ? 'sparkle' : 'account'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                        <span style={{ fontSize: '11px', fontWeight: 800 }}>{msg.role === 'assistant' ? 'TERMINATOR' : 'YOU'}</span>
                                    </div>
                                    <div style={{
                                        background: msg.role === 'user' ? 'var(--vscode-list-hoverBackground, rgba(59, 130, 246, 0.05))' : 'rgba(255, 255, 255, 0.01)',
                                        padding: '12px 16px', borderRadius: '14px', border: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))'
                                    }}>
                                        {msg.thoughts && (
                                            <details style={{ marginBottom: '8px', opacity: 0.6 }}>
                                                <summary style={{ fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Thoughts process...</summary>
                                                <div style={{ fontSize: '11px', padding: '8px', background: 'rgba(0,0,0,0.2)', borderRadius: '6px', marginTop: '4px' }}>{msg.thoughts}</div>
                                            </details>
                                        )}
                                        <div className="markdown-content" style={{ fontSize: '13px', lineHeight: '1.6' }} dangerouslySetInnerHTML={{ __html: marked.parse(msg.content || "") as string }} />
                                    </div>
                                </div>
                            ))
                        )}
                        {isAgentThinking && (
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '12px 16px', opacity: 0.5 }}>
                                <i className="codicon codicon-sync codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                <span style={{ fontSize: '11px', fontWeight: 600 }}>Thinking...</span>
                            </div>
                        )}
                        <div ref={messagesEndRef} />
                    </div>
                ) : view === 'history' ? (
                    <div style={{ padding: '20px', opacity: 0.5, textAlign: 'center', fontSize: '12px' }}>No chat history found.</div>
                ) : view === 'dashboard' ? (
                    <MissionControl />
                ) : view === 'research' ? (
                    <ResearchCenter />
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
