import React, { useState, useCallback, useRef, useEffect } from 'react';
import { useStore } from '../store';
import { FileDiff } from './DiffViewer';

interface ComposerMessage {
    id: string;
    role: 'user' | 'assistant' | 'system';
    content: string;
    timestamp: number;
    files?: string[];
    isRunning?: boolean;
}

interface ComposerFile {
    path: string;
    originalContent: string;
    newContent: string;
    status: 'pending' | 'accepted' | 'rejected';
}

const Composer: React.FC = () => {
    const pendingChanges = useStore(state => state.pendingChanges);
    const acceptChange = useStore(state => state.acceptPendingChange);
    const rejectChange = useStore(state => state.rejectPendingChange);
    const workspacePath = useStore(state => state.activeRoot ?? '');
    
    const isSpecModeActive = useStore(state => state.isSpecModeActive);
    const setSpecModeActive = useStore(state => state.setSpecModeActive);
    const setSpecsPrompt = useStore(state => state.setSpecsPrompt);

    // Multi-pane workflow state
    const [activePane, setActivePane] = useState<'chat' | 'files' | 'diff' | 'preview'>('chat');
    const [messages, setMessages] = useState<ComposerMessage[]>([]);
    const [input, setInput] = useState('');
    const [isProcessing, setIsProcessing] = useState(false);
    const [files, setFiles] = useState<ComposerFile[]>([]);
    const [selectedFile, setSelectedFile] = useState<string | null>(null);
    const [workflowMode, setWorkflowMode] = useState<'edit' | 'create' | 'refactor' | 'debug'>('edit');
    const [conversationHistory, setConversationHistory] = useState<Array<{ role: string; content: string }>>([]);
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const inputRef = useRef<HTMLTextAreaElement>(null);

    // Auto-scroll to latest message
    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [messages]);

    const handleSend = useCallback(async () => {
        if (!input.trim() || isProcessing) return;

        const userText = input;
        if (isSpecModeActive) {
            setSpecsPrompt(userText);
            useStore.getState().setSpecsWizardOpen(true);
            useStore.getState().setSpecsWizardStep('generator');
            setInput('');
            return;
        }
        const userMessage: ComposerMessage = {
            id: Date.now().toString(),
            role: 'user',
            content: userText,
            timestamp: Date.now(),
        };
        const assistantId = (Date.now() + 1).toString();
        const assistantPlaceholder: ComposerMessage = {
            id: assistantId,
            role: 'assistant',
            content: '',
            timestamp: Date.now(),
            isRunning: true,
        };

        setMessages(prev => [...prev, userMessage, assistantPlaceholder]);
        setInput('');
        setIsProcessing(true);
        setConversationHistory(prev => [...prev, { role: 'user', content: userText }]);

        // Workflow mode is prepended as a soft directive — the real agent loop
        // already handles tool calls, planning, diffs, and accept/reject via
        // the global pendingChanges store, so Composer just streams the
        // assistant text into its local message list.
        const modePrefix =
            workflowMode === 'create'   ? '[Workflow: CREATE] '   :
            workflowMode === 'refactor' ? '[Workflow: REFACTOR] ' :
            workflowMode === 'debug'    ? '[Workflow: DEBUG] '    :
                                          '';
        const prompt = modePrefix + userText;

        try {
            const { sendAgentMessage } = await import('../agent');
            await sendAgentMessage(prompt, (msg: string) => {
                setMessages(prev => prev.map(m =>
                    m.id === assistantId ? { ...m, content: msg, isRunning: true } : m
                ));
            });
        } catch (err: any) {
            setMessages(prev => prev.map(m =>
                m.id === assistantId
                    ? { ...m, content: `Error: ${err?.message ?? String(err)}`, isRunning: false }
                    : m
            ));
        } finally {
            setMessages(prev => prev.map(m =>
                m.id === assistantId ? { ...m, isRunning: false } : m
            ));
            setIsProcessing(false);
        }
    }, [input, isProcessing, workflowMode]);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && (e.metaKey || e.ctrlKey)) {
            handleSend();
        }
    };

    const acceptAllChanges = () => {
        pendingChanges.forEach(change => acceptChange(change.id));
    };

    const rejectAllChanges = () => {
        pendingChanges.forEach(change => rejectChange(change.id));
    };

    const modeIcons = {
        edit: 'codicon-edit',
        create: 'codicon-add',
        refactor: 'codicon-wand',
        debug: 'codicon-bug',
    };

    const modeLabels = {
        edit: 'Edit',
        create: 'Create',
        refactor: 'Refactor',
        debug: 'Debug',
    };

    if (pendingChanges.length === 0 && messages.length === 0) {
        return (
            <div className="composer-container" style={{
                display: 'flex',
                flexDirection: 'column',
                height: '100%',
                background: 'var(--vscode-editor-background)',
                color: 'var(--vscode-foreground)'
            }}>
                {/* Composer Header with Mode Selector */}
                <div className="composer-header" style={{
                    height: '40px',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 12px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    background: 'var(--vscode-panel-background)',
                    justifyContent: 'space-between'
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <i className="codicon codicon-composer" style={{ fontSize: '14px' }}></i>
                        <span style={{ fontSize: '11px', fontWeight: 600, opacity: 0.8 }}>COMPOSER</span>
                    </div>
                    
                    {/* Workflow Mode Selector */}
                    <div style={{ display: 'flex', gap: '4px' }}>
                        {(Object.keys(modeIcons) as Array<keyof typeof modeIcons>).map(mode => (
                            <button
                                key={mode}
                                onClick={() => setWorkflowMode(mode)}
                                title={modeLabels[mode]}
                                style={{
                                    background: workflowMode === mode ? 'var(--vscode-button-background)' : 'transparent',
                                    color: workflowMode === mode ? 'var(--vscode-button-foreground)' : 'var(--vscode-foreground)',
                                    border: 'none',
                                    padding: '4px 8px',
                                    borderRadius: '2px',
                                    cursor: 'pointer',
                                    fontSize: '10px',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                }}
                            >
                                <i className={`codicon ${modeIcons[mode]}`} style={{ fontSize: '12px' }}></i>
                                {modeLabels[mode]}
                            </button>
                        ))}
                    </div>
                </div>

                {/* Empty State */}
                <div className="composer-body" style={{
                    flex: 1,
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    justifyContent: 'center',
                    opacity: 0.5,
                    padding: '24px'
                }}>
                    <i className="codicon codicon-composer" style={{ fontSize: '64px', marginBottom: '16px', opacity: 0.2 }}></i>
                    <div style={{ fontSize: '14px', marginBottom: '8px', fontWeight: 500 }}>Composer</div>
                    <div style={{ fontSize: '12px', textAlign: 'center', maxWidth: '400px', lineHeight: 1.6 }}>
                        Multi-file editing workspace with iterative refinement.<br />
                        Describe what you want to build or change.
                    </div>
                    <div style={{ fontSize: '11px', marginTop: '16px', opacity: 0.6 }}>
                        Press <kbd style={{ padding: '2px 6px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '3px' }}>⌘</kbd>+<kbd style={{ padding: '2px 6px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '3px' }}>Enter</kbd> to send
                    </div>
                </div>

                {/* Input Area */}
                <div className="composer-input" style={{
                    borderTop: '1px solid var(--vscode-panel-border)',
                    padding: '12px',
                    background: 'var(--vscode-input-background)'
                }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px', padding: '0 4px' }}>
                        <div 
                            onClick={() => setSpecModeActive(!isSpecModeActive)}
                            style={{ 
                                display: 'flex', 
                                alignItems: 'center', 
                                gap: '6px', 
                                cursor: 'pointer',
                                fontSize: '11px',
                                fontWeight: 600,
                                color: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                transition: 'all 0.2s',
                                userSelect: 'none'
                            }}
                        >
                            <span style={{
                                width: '28px',
                                height: '14px',
                                background: isSpecModeActive ? 'rgba(0, 198, 255, 0.2)' : 'rgba(255,255,255,0.1)',
                                borderRadius: '10px',
                                position: 'relative',
                                display: 'inline-block',
                                border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.15)'}`
                            }}>
                                <span style={{
                                    width: '10px',
                                    height: '10px',
                                    background: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                    borderRadius: '50%',
                                    position: 'absolute',
                                    top: '1px',
                                    left: isSpecModeActive ? '15px' : '2px',
                                    transition: 'all 0.2s'
                                }} />
                            </span>
                            <span>📋 SPEC MODE</span>
                        </div>
                    </div>
                    <textarea
                        ref={inputRef}
                        value={input}
                        onChange={e => setInput(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder={isSpecModeActive ? 'Describe the feature to auto-generate requirements & tasks...' : `Describe what you want to ${workflowMode}...`}
                        rows={3}
                        style={{
                            width: '100%',
                            padding: '8px 12px',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'var(--vscode-input-border)'}`,
                            boxShadow: isSpecModeActive ? '0 0 10px rgba(0, 198, 255, 0.2)' : 'none',
                            borderRadius: '4px',
                            fontSize: '13px',
                            resize: 'vertical',
                            fontFamily: 'var(--vscode-font-family)',
                            transition: 'all 0.2s'
                        }}
                    />
                </div>
            </div>
        );
    }

    return (
        <div className="composer-container" style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: 'var(--vscode-editor-background)',
            color: 'var(--vscode-foreground)'
        }}>
            {/* Header with Pane Tabs */}
            <div className="composer-header" style={{
                height: '40px',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                justifyContent: 'space-between'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, opacity: 0.8 }}>COMPOSER</span>
                    {pendingChanges.length > 0 && (
                        <span style={{
                            background: 'var(--vscode-badge-background)',
                            color: 'var(--vscode-badge-foreground)',
                            padding: '1px 6px',
                            borderRadius: '10px',
                            fontSize: '10px'
                        }}>{pendingChanges.length}</span>
                    )}
                </div>

                {/* Pane Tabs */}
                <div style={{ display: 'flex', gap: '2px' }}>
                    {(['chat', 'files', 'diff', 'preview'] as const).map(pane => (
                        <button
                            key={pane}
                            onClick={() => setActivePane(pane)}
                            style={{
                                background: activePane === pane ? 'var(--vscode-tab-activeBackground)' : 'transparent',
                                color: activePane === pane ? 'var(--vscode-tab-activeForeground)' : 'var(--vscode-foreground)',
                                border: 'none',
                                borderBottom: activePane === pane ? '2px solid var(--vscode-tab-activeBorder)' : '2px solid transparent',
                                padding: '4px 12px',
                                fontSize: '11px',
                                cursor: 'pointer',
                                textTransform: 'capitalize',
                                borderRadius: '2px 2px 0 0',
                            }}
                        >
                            {pane}
                        </button>
                    ))}
                </div>

                {/* Actions */}
                {pendingChanges.length > 0 && (
                    <div className="composer-actions" style={{ display: 'flex', gap: '8px' }}>
                        <button
                            onClick={acceptAllChanges}
                            style={{
                                background: 'var(--vscode-button-background)',
                                color: 'var(--vscode-button-foreground)',
                                border: 'none',
                                padding: '2px 8px',
                                fontSize: '11px',
                                borderRadius: '2px',
                                cursor: 'pointer'
                            }}
                        >
                            Accept All
                        </button>
                        <button
                            onClick={rejectAllChanges}
                            style={{
                                background: 'transparent',
                                color: 'var(--vscode-testing-iconFailed)',
                                border: '1px solid var(--vscode-testing-iconFailed)',
                                padding: '2px 8px',
                                fontSize: '11px',
                                borderRadius: '2px',
                                cursor: 'pointer'
                            }}
                        >
                            Reject All
                        </button>
                    </div>
                )}
            </div>

            {/* Multi-Pane Body */}
            <div className="composer-body" style={{
                flex: 1,
                display: 'flex',
                overflow: 'hidden'
            }}>
                {/* Chat Pane */}
                {activePane === 'chat' && (
                    <div style={{
                        flex: 1,
                        display: 'flex',
                        flexDirection: 'column',
                        overflow: 'hidden'
                    }}>
                        {/* Messages */}
                        <div style={{
                            flex: 1,
                            overflowY: 'auto',
                            padding: '12px',
                        }}>
                            {messages.map(msg => (
                                <div key={msg.id} style={{
                                    marginBottom: '16px',
                                    padding: '12px',
                                    background: msg.role === 'user' 
                                        ? 'var(--vscode-input-background)' 
                                        : 'var(--vscode-textBlockQuote-background)',
                                    borderRadius: '4px',
                                    borderLeft: msg.role === 'user' 
                                        ? '3px solid var(--vscode-button-background)' 
                                        : '3px solid var(--vscode-testing-iconPassed)',
                                }}>
                                    <div style={{
                                        fontSize: '10px',
                                        opacity: 0.6,
                                        marginBottom: '4px',
                                        textTransform: 'uppercase',
                                    }}>
                                        {msg.role}
                                    </div>
                                    <div style={{
                                        fontSize: '13px',
                                        lineHeight: 1.5,
                                        whiteSpace: 'pre-wrap',
                                    }}>
                                        {msg.content}
                                    </div>
                                    {msg.isRunning && (
                                        <div style={{
                                            marginTop: '8px',
                                            fontSize: '11px',
                                            color: 'var(--vscode-progressBar-background)',
                                            display: 'flex',
                                            alignItems: 'center',
                                            gap: '6px',
                                        }}>
                                            <i className="codicon codicon-loading" style={{ animation: 'spin 1s linear infinite' }}></i>
                                            Processing...
                                        </div>
                                    )}
                                </div>
                            ))}
                            <div ref={messagesEndRef} />
                        </div>

                        {/* Input */}
                        <div style={{
                            borderTop: '1px solid var(--vscode-panel-border)',
                            padding: '12px',
                            background: 'var(--vscode-input-background)'
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px', padding: '0 4px' }}>
                                <div 
                                    onClick={() => setSpecModeActive(!isSpecModeActive)}
                                    style={{ 
                                        display: 'flex', 
                                        alignItems: 'center', 
                                        gap: '6px', 
                                        cursor: 'pointer',
                                        fontSize: '11px',
                                        fontWeight: 600,
                                        color: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                        transition: 'all 0.2s',
                                        userSelect: 'none'
                                    }}
                                >
                                    <span style={{
                                        width: '28px',
                                        height: '14px',
                                        background: isSpecModeActive ? 'rgba(0, 198, 255, 0.2)' : 'rgba(255,255,255,0.1)',
                                        borderRadius: '10px',
                                        position: 'relative',
                                        display: 'inline-block',
                                        border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.15)'}`
                                    }}>
                                        <span style={{
                                            width: '10px',
                                            height: '10px',
                                            background: isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'rgba(255,255,255,0.4)',
                                            borderRadius: '50%',
                                            position: 'absolute',
                                            top: '1px',
                                            left: isSpecModeActive ? '15px' : '2px',
                                            transition: 'all 0.2s'
                                        }} />
                                    </span>
                                    <span>📋 SPEC MODE</span>
                                </div>
                            </div>
                            <textarea
                                ref={inputRef}
                                value={input}
                                onChange={e => setInput(e.target.value)}
                                onKeyDown={handleKeyDown}
                                placeholder={isSpecModeActive ? 'Describe the feature to auto-generate requirements & tasks...' : 'Continue refining...'}
                                rows={3}
                                style={{
                                    width: '100%',
                                    padding: '8px 12px',
                                    background: 'var(--vscode-input-background)',
                                    color: 'var(--vscode-input-foreground)',
                                    border: `1px solid ${isSpecModeActive ? 'var(--terminator-accent, #00c6ff)' : 'var(--vscode-input-border)'}`,
                                    boxShadow: isSpecModeActive ? '0 0 10px rgba(0, 198, 255, 0.2)' : 'none',
                                    borderRadius: '4px',
                                    fontSize: '13px',
                                    resize: 'vertical',
                                    fontFamily: 'var(--vscode-font-family)',
                                    transition: 'all 0.2s'
                                }}
                            />
                            <div style={{
                                marginTop: '8px',
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'center',
                                fontSize: '11px',
                                opacity: 0.6,
                            }}>
                                <span>
                                    <kbd style={{ padding: '2px 6px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '3px' }}>⌘</kbd>
                                    +
                                    <kbd style={{ padding: '2px 6px', background: 'var(--vscode-keybindingLabel-background)', borderRadius: '3px' }}>Enter</kbd>
                                    to send
                                </span>
                                <button
                                    onClick={handleSend}
                                    disabled={isProcessing || !input.trim()}
                                    style={{
                                        background: isProcessing || !input.trim() 
                                            ? 'var(--vscode-button-background)' 
                                            : 'var(--vscode-button-background)',
                                        color: 'var(--vscode-button-foreground)',
                                        border: 'none',
                                        padding: '4px 12px',
                                        borderRadius: '2px',
                                        cursor: isProcessing ? 'wait' : 'pointer',
                                        opacity: isProcessing || !input.trim() ? 0.5 : 1,
                                    }}
                                >
                                    {isProcessing ? 'Processing...' : 'Send'}
                                </button>
                            </div>
                        </div>
                    </div>
                )}

                {/* Files Pane */}
                {activePane === 'files' && (
                    <div style={{
                        flex: 1,
                        display: 'flex',
                        overflow: 'hidden'
                    }}>
                        {/* File List */}
                        <div style={{
                            width: '250px',
                            borderRight: '1px solid var(--vscode-panel-border)',
                            overflowY: 'auto',
                            padding: '8px',
                        }}>
                            <div style={{
                                fontSize: '11px',
                                fontWeight: 600,
                                marginBottom: '8px',
                                opacity: 0.8,
                            }}>
                                AFFECTED FILES
                            </div>
                            {pendingChanges.map(change => (
                                <div
                                    key={change.id}
                                    onClick={() => setSelectedFile(change.id)}
                                    style={{
                                        padding: '6px 8px',
                                        cursor: 'pointer',
                                        borderRadius: '2px',
                                        background: selectedFile === change.id 
                                            ? 'var(--vscode-list-activeSelectionBackground)' 
                                            : 'transparent',
                                        color: selectedFile === change.id 
                                            ? 'var(--vscode-list-activeSelectionForeground)' 
                                            : 'var(--vscode-foreground)',
                                        fontSize: '12px',
                                        marginBottom: '2px',
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: '6px',
                                    }}
                                >
                                    <i className="codicon codicon-file" style={{ fontSize: '12px' }}></i>
                                    <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                        {change.path}
                                    </span>
                                </div>
                            ))}
                        </div>

                        {/* File Preview */}
                        <div style={{
                            flex: 1,
                            overflow: 'auto',
                            padding: '12px',
                        }}>
                            {selectedFile ? (
                                <>
                                    <div style={{
                                        fontSize: '12px',
                                        fontWeight: 600,
                                        marginBottom: '12px',
                                    }}>
                                        {pendingChanges.find(c => c.id === selectedFile)?.path}
                                    </div>
                                    <div style={{ height: '400px', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', overflow: 'hidden' }}>
                                        <FileDiff change={pendingChanges.find(c => c.id === selectedFile)!} />
                                    </div>
                                </>
                            ) : (
                                <div style={{
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    height: '100%',
                                    opacity: 0.5,
                                    fontSize: '12px',
                                }}>
                                    Select a file to preview
                                </div>
                            )}
                        </div>
                    </div>
                )}

                {/* Diff Pane */}
                {activePane === 'diff' && (
                    <div style={{
                        flex: 1,
                        overflow: 'auto',
                        padding: '12px',
                    }}>
                        {pendingChanges.map(change => (
                            <div key={change.id} style={{ marginBottom: '24px' }}>
                                <div style={{
                                    display: 'flex',
                                    justifyContent: 'space-between',
                                    alignItems: 'center',
                                    marginBottom: '8px',
                                    paddingBottom: '8px',
                                    borderBottom: '1px solid var(--vscode-panel-border)',
                                }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                        <i className="codicon codicon-file-diff" style={{ fontSize: '14px' }}></i>
                                        <span style={{ fontSize: '13px', fontWeight: 600 }}>{change.path}</span>
                                    </div>
                                    <div style={{ display: 'flex', gap: '4px' }}>
                                        <button
                                            onClick={() => acceptChange(change.id)}
                                            style={{
                                                background: 'var(--vscode-button-background)',
                                                color: 'var(--vscode-button-foreground)',
                                                border: 'none',
                                                padding: '4px 12px',
                                                fontSize: '11px',
                                                borderRadius: '2px',
                                                cursor: 'pointer',
                                            }}
                                        >
                                            ✓ Accept
                                        </button>
                                        <button
                                            onClick={() => rejectChange(change.id)}
                                            style={{
                                                background: 'transparent',
                                                color: 'var(--vscode-testing-iconFailed)',
                                                border: '1px solid var(--vscode-testing-iconFailed)',
                                                padding: '4px 12px',
                                                fontSize: '11px',
                                                borderRadius: '2px',
                                                cursor: 'pointer',
                                            }}
                                        >
                                            ✗ Reject
                                        </button>
                                    </div>
                                </div>
                                {change.description && (
                                    <div style={{
                                        fontSize: '11px',
                                        opacity: 0.7,
                                        marginBottom: '8px',
                                        fontStyle: 'italic',
                                    }}>
                                        {change.description}
                                    </div>
                                )}
                                <div style={{ height: '300px', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', overflow: 'hidden' }}>
                                    <FileDiff change={change} />
                                </div>
                            </div>
                        ))}
                    </div>
                )}

                {/* Preview Pane */}
                {activePane === 'preview' && (
                    <div style={{ flex: 1, overflow: 'auto', padding: 12 }}>
                        {pendingChanges.length === 0 ? (
                            <div style={{ textAlign: 'center', opacity: 0.5, padding: 40, fontSize: 12 }}>
                                No pending changes to preview. Send a composer prompt first.
                            </div>
                        ) : (
                            pendingChanges.map((change) => (
                                <div key={change.id} style={{ marginBottom: 20 }}>
                                    <div style={{ fontSize: 12, fontWeight: 600, marginBottom: 6, display: 'flex', alignItems: 'center', gap: 6 }}>
                                        <i className="codicon codicon-file" style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                                        {change.path}
                                    </div>
                                    {change.description && (
                                        <div style={{ fontSize: 11, opacity: 0.65, marginBottom: 8, fontStyle: 'italic' }}>{change.description}</div>
                                    )}
                                    <div style={{ height: 280, border: '1px solid var(--vscode-panel-border)', borderRadius: 4, overflow: 'hidden' }}>
                                        <FileDiff change={change} />
                                    </div>
                                </div>
                            ))
                        )}
                    </div>
                )}
            </div>
        </div>
    );
};

export default Composer;
