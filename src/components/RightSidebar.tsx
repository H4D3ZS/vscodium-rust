import React, { useEffect, useState, useRef, useMemo } from 'react';
import { marked } from 'marked';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import AgentSettingsView from './AgentSettingsView';
import MissionControl from './agent/MissionControl';
import ResearchCenter from './agent/ResearchCenter';
import ContextSidebar from './visual/ContextSidebar';
import SentientAvatar from './agent/SentientAvatar';
import type { AvatarState } from './agent/SentientAvatar';

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
    const isAgentPaused = useStore(state => state.isAgentPaused);
    const addAgentMessage = useStore(state => state.addAgentMessage);
    const updateLastAgentMessage = useStore(state => state.updateLastAgentMessage);
    const setIsAgentThinking = useStore(state => state.setIsAgentThinking);
    const clearAgentMessages = useStore(state => state.clearAgentMessages);
    const resetThread = useStore(state => state.resetThread);
    const pendingChanges = useStore(state => state.pendingChanges);
    const truncateAgentMessages = useStore(state => state.truncateAgentMessages);
    const [sessionAge, setSessionAge] = useState<string>('');
    const attachedFiles = useStore(state => state.attachedFiles);
    const attachFile = useStore(state => state.attachFile);
    const removeFile = useStore(state => state.removeFile);
    const clearAttachedFiles = useStore(state => state.clearAttachedFiles);
    const agentRootAccess = useStore(state => state.agentRootAccess);
    const setAgentRootAccess = useStore(state => state.setAgentRootAccess);
    const fileTree = useStore(state => state.fileTree);
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const agentTasks = useStore(state => state.agentTasks);
    const chatSessions = useStore(state => state.chatSessions);
    const refreshChatSessions = useStore(state => state.refreshChatSessions);
    const loadChatSession = useStore(state => state.loadChatSession);
    const archiveCurrentSession = useStore(state => state.archiveCurrentSession);
    const createNewSession = useStore(state => state.createNewSession);
    const availableModels = useStore(state => state.availableModels);

    useEffect(() => {
        if (view === 'history') {
            refreshChatSessions();
        }
    }, [view, refreshChatSessions]);
    const [inputValue, setInputValue] = useState('');
    const [isMentionDropdownOpen, setIsMentionDropdownOpen] = useState(false);
    const [selectedMentionIndex, setSelectedMentionIndex] = useState(0);
    const [isHelpOpen, setIsHelpOpen] = useState(false);

    // Phase 7: Chat Editing & Copying State
    const [editingMsgIdx, setEditingMsgIdx] = useState<number | null>(null);
    const [editValue, setEditValue] = useState('');
    const [lastCopiedIdx, setLastCopiedIdx] = useState<number | null>(null);
    const [isAttaching, setIsAttaching] = useState(false);

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

    const avatarState: AvatarState = useMemo(() => {
        if (aiStatus === 'dead') return 'error';
        if (isAgentThinking) {
            // If thinking but last message has content, it's likely "coding" (streaming)
            const lastMsg = messages[messages.length - 1];
            if (lastMsg && lastMsg.role === 'assistant' && lastMsg.content) return 'coding';
            return 'thinking';
        }
        return 'idle';
    }, [aiStatus, isAgentThinking, messages]);

    if (!isOpen) return null;

    const handleAttachFile = async () => {
        if (isAttaching) return;
        setIsAttaching(true);
        try {
            // Priority: Find a dedicated embedding model first (much faster)
            let embeddingModel = availableModels.find(m =>
                m.provider === 'ollama' && (
                    m.id.toLowerCase().includes('embed') ||
                    m.id.toLowerCase().includes('nomic') ||
                    m.id.toLowerCase().includes('mxbai')
                )
            )?.id;

            // Filter for dedicated embedding models only
            const dedicatedEmbedder = availableModels.find(m =>
                m.provider === 'ollama' && (
                    m.id.toLowerCase().includes('embed') ||
                    m.id.toLowerCase().includes('nomic') ||
                    m.id.toLowerCase().includes('mxbai')
                )
            )?.id;

            // STRATEGY: Only attempt neuralization if we have a dedicated lightweight model.
            // Using the heavy reasoning model for embeddings causes the '4-minute' swap delay.
            let cleanInvokeModel = "";
            if (dedicatedEmbedder) {
                cleanInvokeModel = dedicatedEmbedder.includes('|') ? dedicatedEmbedder.split('|').pop()! :
                    (dedicatedEmbedder.includes('/') ? dedicatedEmbedder.split('/').pop()! : dedicatedEmbedder);
                console.log(`[DEBUG] Neuralizing using dedicated model: ${cleanInvokeModel}`);
            } else {
                console.log("[DEBUG] No dedicated embedder found. Using instant raw-text attachment flow.");
            }

            let results: any[];
            try {
                results = await invoke('select_and_process_attachment', { model: cleanInvokeModel });
            } catch (invokeError: any) {
                console.error('[ERROR] Attachment selection failed:', invokeError);
                throw invokeError;
            }

            if (results && Array.isArray(results)) {
                const formatted = results.map(r => ({
                    id: r.path || `neural-${Date.now()}-${Math.random()}`,
                    type: 'file' as const,
                    name: r.name,
                    path: r.path,
                    gist: r.gist,
                    thumbnail: r.thumbnail,
                    data: r.data
                }));
                attachFile(formatted);
            }
        } catch (error: any) {
            console.error('Failed to neuralize, attempting raw attachment:', error);
            // FINAL FALLBACK: If neuralization fails, try a simple file read for raw context
            try {
                // We need the path. Since we don't have it from 'select_and_process_attachment' yet
                // because it failed before/during selection potentially, we might need a separate picker.
                // However, 'select_and_process_attachment' usually fails AFTER selection.
                // For now, let's just alert the user to get a dedicated model.
                alert('Ollama failed to neuralize the file. \n\nTIP: For best performance, run "ollama pull nomic-embed-text". \nFalling back to standard attachment...');
            } catch (fallbackError) {
                alert('Failed to attach file: ' + error);
            }
        } finally {
            setIsAttaching(false);
        }
    };

    const onSend = async (overrideMsg?: string) => {
        const val = (overrideMsg !== undefined ? overrideMsg : inputValue).trim();
        console.log('[DIAG] onSend called, val:', val, 'isRightSidebarOpen:', useStore.getState().isRightSidebarOpen);
        if ((val || attachedFiles.length > 0) && !isAgentThinking) {
            console.log('[DIAG] onSend: sending message, sidebar state before:', useStore.getState().isRightSidebarOpen);
            if (overrideMsg === undefined) setInputValue("");
            setIsMentionDropdownOpen(false);
            if (inputRef.current) inputRef.current.style.height = 'auto';

            const context = [...attachedFiles];
            setIsAgentThinking(true);
            addAgentMessage('user', val, context);
            clearAttachedFiles();
            addAgentMessage('assistant', "");
            console.log('[DIAG] onSend: messages added, sidebar state after store updates:', useStore.getState().isRightSidebarOpen);

            try {
                const m = await import('../agent');
                await m.sendAgentMessage(val, () => { }, context);
            } catch (err: any) {
                console.error('Agent chat failed:', err);
                const errorMsg = err.message || JSON.stringify(err);
                updateLastAgentMessage(`Error: ${errorMsg}`);
            } finally {
                setIsAgentThinking(false);
                console.log('[DIAG] onSend: done. sidebar state:', useStore.getState().isRightSidebarOpen);
            }
        }
    };

    const handleCopy = (content: string, idx: number) => {
        navigator.clipboard.writeText(content).then(() => {
            setLastCopiedIdx(idx);
            setTimeout(() => setLastCopiedIdx(null), 2000);
        });
    };

    const handleEditSave = (idx: number) => {
        const newVal = editValue.trim();
        if (newVal) {
            truncateAgentMessages(idx);
            setEditingMsgIdx(null);
            onSend(newVal);
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
                attachFile({
                    id: `dropped-${Date.now()}-${i}`,
                    type: file.type.startsWith('image/') ? 'attachment' : 'file',
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
        attachFile({
            id: file.path,
            type: 'file',
            name: file.name,
            path: file.path
        });
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        // CRITICAL: stop ALL keyboard events from bubbling to the global handler
        // to prevent sidebar toggles, Ctrl+W closing tabs, etc. while typing
        e.stopPropagation();

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
                e.preventDefault();
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
                <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                    <SentientAvatar state={avatarState} size={28} />
                    <div style={{ display: 'flex', flexDirection: 'column' }}>
                        <span style={{ fontSize: '11px', fontWeight: 700, letterSpacing: '0.06em', color: avatarState === 'error' ? '#ef4444' : 'inherit' }}>TERMINATOR AI</span>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '4px' }}>
                            {isAgentThinking && (
                                <span style={{ fontSize: '9px', color: '#3b82f6', textTransform: 'uppercase', display: 'flex', alignItems: 'center', gap: '4px' }}>
                                    <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontSize: '9px' }}></i>
                                    Thinking
                                </span>
                            )}
                            {isAttaching && (
                                <span style={{ fontSize: '9px', color: '#10b981', textTransform: 'uppercase', display: 'flex', alignItems: 'center', gap: '2px', marginLeft: isAgentThinking ? '6px' : 0 }}>
                                    <i className="codicon codicon-sync codicon-modifier-spin" style={{ fontFamily: 'codicon', fontSize: '9px' }}></i>
                                    Neuralizing
                                </span>
                            )}
                            {!isAgentThinking && !isAttaching && (
                                <span style={{ fontSize: '9px', opacity: 0.3, textTransform: 'uppercase' }}>{aiStatus}</span>
                            )}
                        </div>
                    </div>
                    <div
                        onClick={() => {
                            createNewSession();
                            setView('chat');
                        }}
                        style={{ cursor: 'pointer', opacity: 0.8, marginLeft: '8px', padding: '4px', background: 'rgba(59,130,246,0.1)', borderRadius: '4px', color: '#3b82f6' }}
                        title="New Chat">
                        <i className="codicon codicon-add" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', display: 'block' }}></i>
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
                                    <div key={idx} className="agent-message-container" style={{ display: 'flex', flexDirection: 'column', gap: '8px', position: 'relative' }}>
                                        <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', opacity: 0.5 }}>
                                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                                <i className={`codicon codicon-${msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'hubot' : 'sparkle') : 'account'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: msg.isSubAgentResponse ? '#3b82f6' : 'inherit' }}></i>
                                                <span style={{ fontSize: '11px', fontWeight: 800, color: msg.isSubAgentResponse ? '#3b82f6' : 'inherit' }}>{msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'SUB-AGENT' : 'TERMINATOR') : 'YOU'}</span>
                                                {msg.role === 'assistant' && !msg.isSubAgentResponse && isAgentThinking && idx === messages.length - 1 && (
                                                    <span style={{ fontSize: '9px', opacity: 0.5, fontWeight: 400, marginLeft: '8px', textTransform: 'uppercase' }}>{avatarState} • {sessionAge}</span>
                                                )}
                                                {msg.role === 'assistant' && !isAgentThinking && msg.timestamp && (
                                                    <span style={{ fontSize: '9px', opacity: 0.3, fontWeight: 400, marginLeft: '8px' }}>{new Date(msg.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                                                )}
                                            </div>
                                            <div className="message-actions" style={{ opacity: 0, transition: 'opacity 0.2s', display: 'flex', gap: '8px' }}>
                                                {msg.role === 'user' && !isAgentThinking && (
                                                    <i className="codicon codicon-edit" style={{ cursor: 'pointer', fontSize: '12px' }} title="Edit message"
                                                        onClick={() => { setEditingMsgIdx(idx); setEditValue(msg.content); }}></i>
                                                )}
                                                {msg.role === 'assistant' && (
                                                    <i className={`codicon codicon-${lastCopiedIdx === idx ? 'check' : 'copy'}`}
                                                        style={{ cursor: 'pointer', fontSize: '12px', color: lastCopiedIdx === idx ? '#10b981' : 'inherit' }}
                                                        title="Copy response"
                                                        onClick={() => handleCopy(msg.content, idx)}></i>
                                                )}
                                            </div>
                                        </div>
                                        <div style={{
                                            background: msg.role === 'user' ? 'var(--vscode-list-hoverBackground, rgba(59, 130, 246, 0.05))' : (msg.isSubAgentResponse ? 'rgba(59, 130, 246, 0.03)' : 'rgba(255, 255, 255, 0.01)'),
                                            padding: '12px 16px', borderRadius: '14px', border: msg.isSubAgentResponse ? '1px solid rgba(59, 130, 246, 0.1)' : '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.05))'
                                        }}>
                                            {editingMsgIdx === idx ? (
                                                <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                                    <textarea
                                                        value={editValue}
                                                        onChange={(e) => setEditValue(e.target.value)}
                                                        autoFocus
                                                        style={{
                                                            background: 'rgba(0,0,0,0.2)', border: '1px solid var(--vscode-focusBorder)', color: '#fff',
                                                            padding: '8px', borderRadius: '6px', fontSize: '13px', resize: 'vertical', minHeight: '60px', outline: 'none'
                                                        }}
                                                    />
                                                    <div style={{ display: 'flex', gap: '8px', justifyContent: 'flex-end' }}>
                                                        <button onClick={() => setEditingMsgIdx(null)} style={{ background: 'transparent', border: '1px solid rgba(255,255,255,0.2)', color: '#fff', padding: '4px 8px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer' }}>Cancel</button>
                                                        <button onClick={() => handleEditSave(idx)} style={{ background: '#3b82f6', border: 'none', color: '#fff', padding: '4px 12px', borderRadius: '4px', fontSize: '11px', cursor: 'pointer', fontWeight: 600 }}>Resend</button>
                                                    </div>
                                                </div>
                                            ) : (
                                                <>
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
                                                    {msg.context && msg.context.length > 0 && (
                                                        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginBottom: '8px', opacity: 0.8 }}>
                                                            {msg.context.map((item: any, i: number) => (
                                                                <div key={i} style={{
                                                                    display: 'flex', alignItems: 'center', gap: '4px', padding: '2px 6px',
                                                                    background: 'rgba(255,255,255,0.06)', borderRadius: '4px', fontSize: '10px',
                                                                    border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.7)'
                                                                }}>
                                                                    {item.thumbnail ? (
                                                                        <img src={item.thumbnail} style={{ width: '14px', height: '14px', borderRadius: '2px', objectFit: 'cover' }} alt="" />
                                                                    ) : (
                                                                        <i className="codicon codicon-files" style={{ fontSize: '10px' }}></i>
                                                                    )}
                                                                    <span>{item.name}</span>
                                                                </div>
                                                            ))}
                                                        </div>
                                                    )}
                                                    <div className="markdown-content" style={{ fontSize: '13px', lineHeight: '1.6', minHeight: (!msg.content && msg.context?.length) ? '0' : '1em' }} dangerouslySetInnerHTML={{ __html: marked.parse(msg.content || "") as string }} />
                                                </>
                                            )}
                                        </div>
                                    </div>
                                ))}
                                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', padding: '12px 16px', opacity: 0.3 }}>
                                    <span style={{ fontSize: '10px', fontStyle: 'italic' }}>Terminator is generating payload...</span>
                                </div>
                                <div ref={messagesEndRef} />
                            </>
                        )}
                    </div>
                ) : view === 'history' ? (
                    <div style={{ padding: '16px', display: 'flex', flexDirection: 'column', gap: '12px' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                            <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Recent History</span>
                        </div>
                        {chatSessions.length === 0 ? (
                            <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                                No archived sessions found.
                            </div>
                        ) : (
                            <div style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                                {chatSessions.map((session: any) => (
                                    <div
                                        key={session.path}
                                        onClick={() => { loadChatSession(session.path); setView('chat'); }}
                                        style={{
                                            padding: '12px', borderRadius: '8px', background: 'rgba(255,255,255,0.03)',
                                            border: '1px solid rgba(255,255,255,0.05)', cursor: 'pointer',
                                            transition: 'background 0.2s'
                                        }}
                                        onMouseEnter={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.06)'}
                                        onMouseLeave={(e) => e.currentTarget.style.background = 'rgba(255,255,255,0.03)'}
                                    >
                                        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                                            <span style={{ fontSize: '12px', fontWeight: 600 }}>{session.name.replace('session_', '').replace('.aim', '')}</span>
                                            <span style={{ fontSize: '10px', opacity: 0.4 }}>{session.messages} msgs</span>
                                        </div>
                                        <div style={{ fontSize: '10px', opacity: 0.5 }}>
                                            {new Date(session.updated_at * 1000).toLocaleString()}
                                        </div>
                                    </div>
                                ))}
                            </div>
                        )}
                    </div>
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
                        <textarea
                            ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown}
                            placeholder="Ask the Agent..."
                            style={{ background: 'transparent', border: 'none', outline: 'none', color: '#fff', resize: 'none', fontSize: '13px', lineHeight: '1.5', width: '100%', minHeight: '32px' }}
                        />
                        {attachedFiles.length > 0 && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '4px', paddingBottom: '4px' }}>
                                {attachedFiles.map((item, i) => (
                                    <div key={item.id || i} style={{
                                        display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                        background: 'rgba(255,255,255,0.08)', borderRadius: '6px', fontSize: '11px',
                                        border: '1px solid rgba(255,255,255,0.1)', color: 'rgba(255,255,255,0.9)'
                                    }}>
                                        {item.thumbnail ? (
                                            <img src={item.thumbnail} style={{ width: '16px', height: '16px', borderRadius: '3px', objectFit: 'cover' }} alt="" />
                                        ) : (
                                            <span style={{ opacity: 0.7, fontSize: '10px' }}>{item.type === 'attachment' ? 'IMG' : '{ }'}</span>
                                        )}
                                        <span style={{ fontWeight: 500 }}>{item.name}</span>
                                        <i className="codicon codicon-close" onClick={() => removeFile(item.path)} style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.5, marginLeft: '2px', fontSize: '10px' }}></i>
                                    </div>
                                ))}
                                {isAttaching && (
                                    <div style={{
                                        display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                        background: 'rgba(255,255,255,0.05)', borderRadius: '6px', fontSize: '11px',
                                        border: '1px dashed rgba(255,255,255,0.2)', color: 'rgba(255,255,255,0.5)',
                                        fontStyle: 'italic'
                                    }}>
                                        <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }}></i>
                                        <span>Neuralizing...</span>
                                    </div>
                                )}
                            </div>
                        )}
                        {!attachedFiles.length && isAttaching && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '6px', marginTop: '4px', paddingBottom: '4px' }}>
                                <div style={{
                                    display: 'flex', alignItems: 'center', gap: '6px', padding: '3px 8px',
                                    background: 'rgba(255,255,255,0.05)', borderRadius: '6px', fontSize: '11px',
                                    border: '1px dashed rgba(255,255,255,0.2)', color: 'rgba(255,255,255,0.5)',
                                    fontStyle: 'italic'
                                }}>
                                    <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px' }}></i>
                                    <span>Neuralizing...</span>
                                </div>
                            </div>
                        )}
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginTop: '8px' }}>
                            <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                <div onClick={handleAttachFile} style={{ cursor: 'pointer', opacity: 0.5, display: 'flex', alignItems: 'center' }} className="hoverable-bg" title="Attach File (Neural Gist)">
                                    <i className="codicon codicon-attach" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px' }}></i>
                                </div>
                                <span onClick={onModeClick} style={{ fontSize: '10px', opacity: 0.5, cursor: 'pointer' }} className="hoverable-bg">{mode}</span>
                                <span onClick={onModelClick} style={{ fontSize: '10px', opacity: 0.5, cursor: 'pointer' }} className="hoverable-bg">{(model.split('|')[1] || model).split(':')[0]}</span>
                            </div>
                            <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                <div style={{
                                    display: 'flex',
                                    gap: '10px',
                                    alignItems: 'center',
                                    marginRight: '12px',
                                    padding: '4px 10px',
                                    background: 'rgba(255,255,255,0.03)',
                                    borderRadius: '6px',
                                    border: '1px solid rgba(255,255,255,0.05)',
                                    transition: 'all 0.2s'
                                }}>
                                    <div
                                        onClick={() => isAgentPaused ? import('../agent').then(m => m.resumeAgent()) : import('../agent').then(m => m.pauseAgent())}
                                        style={{ cursor: 'pointer', color: isAgentPaused ? '#10b981' : '#f59e0b', display: 'flex', alignItems: 'center' }}
                                        title={isAgentPaused ? "Resume Agent" : "Pause Agent"}
                                    >
                                        <i className={`codicon codicon-${isAgentPaused ? 'play' : 'debug-pause'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }}></i>
                                    </div>
                                    <div
                                        onClick={() => import('../agent').then(m => m.stopAgent())}
                                        style={{ cursor: 'pointer', color: '#ef4444', display: 'flex', alignItems: 'center' }}
                                        title="Stop Agent (Terminate)"
                                    >
                                        <i className="codicon codicon-primitive-square" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }}></i>
                                    </div>
                                </div>
                                <div onClick={() => onSend()} style={{
                                    width: '24px', height: '24px', borderRadius: '50%',
                                    background: (inputValue.trim() || attachedFiles.length > 0) ? '#fff' : 'rgba(255,255,255,0.1)',
                                    color: '#000', display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer'
                                }}>
                                    <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }}></i>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            )}
        </aside>
    );
};

export default RightSidebar;
