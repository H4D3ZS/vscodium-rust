import React, { useEffect, useState, useRef, useMemo, useCallback } from 'react';
import { marked } from 'marked';
import { useStore } from '../store';
import type { FileEntry } from '../store';
import { invoke } from '../tauri_bridge';
import AgentSettingsView from './AgentSettingsView';
import MissionControl from './agent/MissionControl';
import ResearchCenter from './agent/ResearchCenter';
import ContextSidebar from './visual/ContextSidebar';
import { AiriPanel } from './AiriPanel';
import SentientAvatar from './agent/SentientAvatar';
import type { AvatarState } from './agent/SentientAvatar';

/**
 * Strips raw tool-call JSON/XML from AI content so the user sees only
 * natural language. The backend still processes the full JSON — this is
 * purely a display transform.
 */
/** Returns true if a JSON string looks like a tool call object */
function isToolCallJson(text: string): boolean {
    try {
        const t = text.trim();
        if (!t.startsWith('{') && !t.startsWith('[')) return false;
        const parsed = JSON.parse(t);
        if (Array.isArray(parsed)) return parsed.some(isToolCallJson);
        if (parsed && typeof parsed === 'object') {
            return ('name' in parsed && ('arguments' in parsed || 'parameters' in parsed || 'input' in parsed))
                || 'tool_calls' in parsed
                || 'function_call' in parsed;
        }
    } catch { /* not valid JSON */ }
    return false;
}

/**
 * Strips raw tool-call JSON/XML from AI content so the user sees only
 * natural language. The backend still processes the full JSON — this is
 * purely a display transform.
 */
function cleanAiContent(raw: string): string {
    if (!raw) return '';
    let s = raw;

    // Strip XML tool-call tags (Qwen / DeepSeek style)
    s = s.replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '');
    s = s.replace(/<function_calls>[\s\S]*?<\/function_calls>/g, '');
    s = s.replace(/<function>[\s\S]*?<\/function>/g, '');
    s = s.replace(/<invoke>[\s\S]*?<\/invoke>/g, '');

    // Strip markdown fenced code blocks that contain tool-call JSON
    // Use block parser to correctly handle nested braces
    s = s.replace(/```[a-z]*\n([\s\S]*?)```/gi, (match, inner) => {
        return isToolCallJson(inner.trim()) ? '' : match;
    });
    // Also handle ``` without language tag followed immediately by {
    s = s.replace(/```\s*(\{[\s\S]*?\})\s*```/g, (match, inner) => {
        return isToolCallJson(inner) ? '' : match;
    });

    // Strip bare JSON tool-call lines (outside code blocks)
    s = s.split('\n').filter(line => {
        const t = line.trim();
        return !isToolCallJson(t);
    }).join('\n');

    // Strip SEARCH/REPLACE edit blocks (code diffs, not conversation)
    s = s.replace(/<<<< SEARCH[\s\S]*?>>>>/g, '');
    s = s.replace(/<<<<<<[\s\S]*?>>>>>>>/g, '');

    // Strip MISSION_ACCOMPLISHED marker
    s = s.replace(/MISSION_ACCOMPLISHED/g, '');

    // Clean up excessive blank lines
    s = s.replace(/\n{3,}/g, '\n\n').trim();
    return s;
}

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
    const [view, setView] = useState<'chat' | 'history' | 'settings' | 'dashboard' | 'research' | 'context' | 'kortex'>('chat');
    const inputRef = useRef<HTMLTextAreaElement>(null);
    const mode = useStore(state => state.agentMode);
    const model = useStore(state => state.agentModel);
    const messages = useStore(state => state.agentMessages);
    const isAgentThinking = useStore(state => state.isAgentThinking);
    const isAgentPaused = useStore(state => state.isAgentPaused);
    const isYoloMode = useStore(state => state.isYoloMode);
    const setYoloMode = useStore(state => state.setYoloMode);
    const agentUiMode = useStore(state => state.agentUiMode);
    const setAgentUiMode = useStore(state => state.setAgentUiMode);
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
    const autoAcceptChanges = useStore(state => state.autoAcceptChanges);
    const setAutoAcceptChanges = useStore(state => state.setAutoAcceptChanges);
    const checkpoint = useStore(state => state.checkpoint);
    const revertToCheckpoint = useStore(state => state.revertToCheckpoint);
    const snapshotCheckpoint = useStore(state => state.snapshotCheckpoint);

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

    // AIRI full mode — last spoken response and speech state
    const [airiSpeech, setAiriSpeech] = useState<string>('');
    const [airiSpeaking, setAiriSpeaking] = useState(false);

    // Kortex .aim memory panel state
    const [kortexSlots, setKortexSlots] = useState<any[]>([]);
    const [kortexLoading, setKortexLoading] = useState(false);
    const [liveToolCalls, setLiveToolCalls] = useState<Array<{ id: string; tool: string; label: string; status: 'running' | 'done' | 'error'; detail?: string }>>([]);

    const refreshKortex = useCallback(async () => {
        setKortexLoading(true);
        try {
            const slots = await invoke<any[]>('get_all_memory_slots');
            setKortexSlots(slots || []);
        } catch (e) {
            console.error('[Kortex] Failed to load memory slots:', e);
        } finally {
            setKortexLoading(false);
        }
    }, []);

    useEffect(() => {
        if (view === 'kortex') refreshKortex();
    }, [view, refreshKortex]);

    // Live tool-call feed for the Mission Hub
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            const TOOL_ICONS: Record<string, string> = {
                write_to_file: '📝', search_replace_edit: '✂️', apply_shadow_patch: '💾',
                view_file: '👁️', run_command: '⚡', verify_implementation: '🔬',
                ghost_test: '👻', list_files: '📁', grep: '🔍', git_commit: '📦',
            };
            const TOOL_LABELS: Record<string, string> = {
                write_to_file: 'Writing file', search_replace_edit: 'Patching code',
                apply_shadow_patch: 'Committing edit', view_file: 'Reading file',
                run_command: 'Running command', verify_implementation: 'Verifying',
                ghost_test: 'Running tests', list_files: 'Scanning directory',
                grep: 'Searching', git_commit: 'Committing',
            };
            listen<any>('ai-tool-call', (e) => {
                const name = e.payload?.name || 'unknown';
                const id = `tc-${Date.now()}-${Math.random()}`;
                const label = `${TOOL_ICONS[name] || '⚙️'} ${TOOL_LABELS[name] || name.replace(/_/g, ' ')}`;
                setLiveToolCalls(prev => [{
                    id, tool: name, label, status: 'running' as const,
                    detail: e.payload?.args ? (typeof e.payload.args === 'string' ? e.payload.args.slice(0, 50) : JSON.stringify(e.payload.args).slice(0, 50)) : undefined
                }, ...prev].slice(0, 8));
            }).then(u => subs.push(u));
            listen<any>('ai-tool-result', (e) => {
                const name = e.payload?.name;
                setLiveToolCalls(prev => prev.map(a => a.tool === name && a.status === 'running' ? { ...a, status: 'done' } : a));
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, []);

    // Clear tool calls 2.5s after agent stops
    const wasThinkingRef = useRef(false);
    useEffect(() => {
        if (isAgentThinking) {
            wasThinkingRef.current = true;
        } else if (wasThinkingRef.current) {
            wasThinkingRef.current = false;
            const t = setTimeout(() => setLiveToolCalls([]), 2500);
            return () => clearTimeout(t);
        }
    }, [isAgentThinking]);

    // Capture streaming AI content — used for thought bubble (both modes) and speech overlay (AIRI mode)
    useEffect(() => {
        let buffer = '';
        const unsub: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen }) => {
            listen<any>('ai-stream', (e) => {
                const chunk: string = e.payload?.chunk || e.payload?.content || '';
                if (chunk) {
                    buffer += chunk;
                    if (agentUiMode === 'airi') setAiriSpeech(buffer);
                }
            }).then(u => unsub.push(u));
            listen<any>('ai-content', (e) => {
                const content: string = e.payload?.content || '';
                if (content) {
                    buffer = content;
                    if (agentUiMode === 'airi') { setAiriSpeech(content); setAiriSpeaking(false); }
                }
            }).then(u => unsub.push(u));
        });
        return () => { unsub.forEach(u => u()); };
    }, [agentUiMode]);

    useEffect(() => {
        if (agentUiMode === 'airi') {
            if (isAgentThinking) { setAiriSpeaking(true); }
            else { setAiriSpeaking(false); }
        }
    }, [isAgentThinking, agentUiMode]);

    // Neural Sync Bridge: Forward IDE state to the AIRI manifold
    useEffect(() => {
        const syncPayload = {
            messages,
            agentInfo: {
                name: "AIRI",
                status: aiStatus,
                context: "vscodium-rust"
            }
        };
        // Emit native event for AiriPanel to pick up
        import('@tauri-apps/api/event').then(({ emit }) => {
            emit('hades-sync', syncPayload);
        }).catch(err => console.error('[HADES] Sync Broadcast Failed:', err));
    }, [messages, aiStatus]);
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

    // Track current activity from live tool calls
    const [currentActivity, setCurrentActivity] = React.useState<AvatarState>('idle');
    React.useEffect(() => {
        const running = liveToolCalls.find(t => t.status === 'running');
        if (!isAgentThinking) { setCurrentActivity(aiStatus === 'dead' ? 'error' : 'idle'); return; }
        if (!running) { setCurrentActivity('thinking'); return; }
        const toolActivity: Record<string, AvatarState> = {
            write_to_file: 'coding', search_replace_edit: 'coding', patch_file_content: 'coding',
            apply_shadow_patch: 'coding', run_command: 'executing', ghost_test: 'executing',
            dev_cargo_diagnostics: 'executing', browser_open: 'browsing', browser_navigate: 'browsing',
            view_file: 'thinking', grep: 'thinking', search_codebase: 'thinking',
        };
        setCurrentActivity(toolActivity[running.tool] || 'thinking');
    }, [isAgentThinking, liveToolCalls, aiStatus]);

    const avatarState: AvatarState = useMemo(() => {
        if (aiStatus === 'dead') return 'error';
        return currentActivity;
    }, [aiStatus, currentActivity]);

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
            const openPaths = useStore.getState().tabs.map((t: any) => t.path).filter(Boolean);
            snapshotCheckpoint(openPaths);
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

    const readFileAsDataUrl = (file: File): Promise<string> =>
        new Promise((resolve, reject) => {
            const reader = new FileReader();
            reader.onload = () => resolve(reader.result as string);
            reader.onerror = reject;
            reader.readAsDataURL(file);
        });

    const handleDrop = (e: React.DragEvent) => {
        e.preventDefault();
        e.stopPropagation();
        const files = e.dataTransfer.files;
        if (files.length > 0) {
            for (let i = 0; i < files.length; i++) {
                const file = files[i];
                const isImage = file.type.startsWith('image/');
                if (isImage) {
                    // Read image as base64 data URL so vision models can receive it
                    readFileAsDataUrl(file).then(dataUrl => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                            data: dataUrl,
                            thumbnail: dataUrl,
                        } as any);
                    }).catch(() => {
                        attachFile({
                            id: `dropped-${Date.now()}-${i}`,
                            type: 'attachment',
                            name: file.name,
                            path: (file as any).path || file.name,
                        });
                    });
                } else {
                    attachFile({
                        id: `dropped-${Date.now()}-${i}`,
                        type: 'file',
                        name: file.name,
                        path: (file as any).path || file.name,
                    });
                }
            }
        }
    };

    const handlePaste = (e: React.ClipboardEvent) => {
        const items = Array.from(e.clipboardData.items);
        const imageItem = items.find(item => item.type.startsWith('image/'));
        if (imageItem) {
            const file = imageItem.getAsFile();
            if (file) {
                e.preventDefault();
                readFileAsDataUrl(file).then(dataUrl => {
                    attachFile({
                        id: `pasted-${Date.now()}`,
                        type: 'attachment',
                        name: `pasted-image-${Date.now()}.png`,
                        path: `pasted-${Date.now()}.png`,
                        data: dataUrl,
                        thumbnail: dataUrl,
                    } as any);
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
                @keyframes hubPulse {
                    0%, 100% { opacity: 1; transform: scale(1); }
                    50% { opacity: 0.4; transform: scale(1.3); }
                }
                @keyframes thoughtCloudIn {
                    from { opacity: 0; transform: translateX(-50%) scale(0.75) translateY(12px); }
                    to   { opacity: 1; transform: translateX(-50%) scale(1) translateY(0); }
                }
                @keyframes thoughtFloat {
                    0%, 100% { transform: translateX(-50%) translateY(0); }
                    50%      { transform: translateX(-50%) translateY(-7px); }
                }
                /* Anime-style cloud thought bubble for AIRI 3D mode */
                .airi-thought-cloud-3d {
                    background: rgba(232, 245, 255, 0.94);
                    border: 1.5px solid rgba(180, 215, 255, 0.65);
                    border-radius: 22px;
                    padding: 11px 16px;
                    max-width: 210px;
                    min-width: 110px;
                    position: relative;
                    box-shadow: 0 4px 20px rgba(0,0,0,0.25),
                                inset 0 1px 0 rgba(255,255,255,0.9);
                    animation: thoughtFloat 2.8s ease-in-out infinite;
                }
                .airi-thought-cloud-3d::before {
                    content: '';
                    position: absolute;
                    width: 42px; height: 28px;
                    background: rgba(232,245,255,0.94);
                    border: 1.5px solid rgba(180,215,255,0.65);
                    border-radius: 50%;
                    top: -16px;
                    left: 14%;
                }
                .airi-thought-cloud-3d::after {
                    content: '';
                    position: absolute;
                    width: 32px; height: 22px;
                    background: rgba(232,245,255,0.94);
                    border: 1.5px solid rgba(180,215,255,0.65);
                    border-radius: 50%;
                    top: -12px;
                    right: 18%;
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
                        <span style={{ fontSize: '11px', fontWeight: 700, letterSpacing: '0.06em', color: avatarState === 'error' ? '#ef4444' : 'inherit' }}>AIRI CORE</span>
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
                    {/* UI Mode toggle: Chat ↔ AIRI 3D */}
                    <div
                        onClick={() => setAgentUiMode(agentUiMode === 'chat' ? 'airi' : 'chat')}
                        style={{
                            cursor: 'pointer',
                            display: 'flex', alignItems: 'center', gap: '3px',
                            fontSize: '9px', fontWeight: 700,
                            padding: '2px 6px', borderRadius: '5px',
                            background: agentUiMode === 'airi' ? 'rgba(168,85,247,0.15)' : 'rgba(255,255,255,0.06)',
                            border: agentUiMode === 'airi' ? '1px solid rgba(168,85,247,0.4)' : '1px solid rgba(255,255,255,0.1)',
                            color: agentUiMode === 'airi' ? '#c084fc' : 'rgba(255,255,255,0.5)',
                            transition: 'all 0.2s'
                        }}
                        title={agentUiMode === 'airi' ? 'Switch to Chat mode' : 'Switch to AIRI 3D mode'}
                    >
                        <span>{agentUiMode === 'airi' ? '🎭' : '💬'}</span>
                        <span>{agentUiMode === 'airi' ? 'AIRI' : 'CHAT'}</span>
                    </div>
                    <div onClick={() => setView('chat')} style={{ cursor: 'pointer', opacity: view === 'chat' ? 1 : 0.4, color: view === 'chat' ? '#3b82f6' : 'inherit' }} title="Mission Hub"><i className="codicon codicon-rocket" style={{ fontFamily: 'codicon', fontStyle: 'normal' }}></i></div>
                    <div onClick={() => setView('kortex')} style={{ cursor: 'pointer', opacity: view === 'kortex' ? 1 : 0.4, color: view === 'kortex' ? '#a855f7' : 'inherit' }} title="Kortex Brain (.aim)">🧠</div>
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

            <div style={{ flex: 1, display: 'flex', flexDirection: 'column', overflow: 'hidden', minHeight: 0 }}>

                {/* ── AIRI 3D FULL MODE ── */}
                {agentUiMode === 'airi' ? (
                    <div style={{ display: 'flex', flexDirection: 'column', flex: 1, overflow: 'hidden', position: 'relative' }}>

                        {/* Full-height 3D avatar */}
                        <div style={{ flex: 1, minHeight: 0, position: 'relative', overflow: 'hidden' }}>
                            <AiriPanel style={{ width: '100%', height: '100%' }} transparent={true} />

                            {/* ── Anime Thought Bubble — what AIRI is currently thinking ── */}
                            {isAgentThinking && (
                                <div style={{
                                    position: 'absolute',
                                    top: '16%',
                                    left: '50%',
                                    transform: 'translateX(-50%)',
                                    zIndex: 10,
                                    pointerEvents: 'none',
                                    display: 'flex',
                                    flexDirection: 'column',
                                    alignItems: 'center',
                                    animation: 'thoughtCloudIn 0.35s ease-out',
                                }}>
                                    {/* Cloud body */}
                                    <div className="airi-thought-cloud-3d">
                                        <span style={{
                                            display: 'block',
                                            fontSize: '11px',
                                            color: '#080516',
                                            lineHeight: 1.5,
                                            textAlign: 'center',
                                            maxWidth: '190px',
                                            wordBreak: 'break-word',
                                        }}>
                                            {(() => {
                                                const raw = cleanAiContent(airiSpeech).slice(-100);
                                                if (raw) return raw;
                                                if (liveToolCalls[0]) return liveToolCalls[0].label;
                                                return '...';
                                            })()}
                                        </span>
                                    </div>
                                    {/* Trail dots from cloud down toward avatar head */}
                                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '3px', marginTop: '6px' }}>
                                        <div style={{ width: '9px', height: '9px', borderRadius: '50%', background: 'rgba(210,235,255,0.82)' }} />
                                        <div style={{ width: '6px', height: '6px', borderRadius: '50%', background: 'rgba(210,235,255,0.65)' }} />
                                        <div style={{ width: '4px', height: '4px', borderRadius: '50%', background: 'rgba(210,235,255,0.48)' }} />
                                    </div>
                                </div>
                            )}

                            {/* Active tool pill — bottom of avatar, not top */}
                            {liveToolCalls[0] && liveToolCalls[0].status === 'running' && (
                                <div style={{
                                    position: 'absolute', bottom: '12px', left: '50%', transform: 'translateX(-50%)',
                                    background: 'rgba(0,0,0,0.6)', backdropFilter: 'blur(8px)',
                                    border: '1px solid rgba(168,85,247,0.3)',
                                    borderRadius: '20px', padding: '4px 14px',
                                    display: 'flex', alignItems: 'center', gap: '8px',
                                    fontSize: '10px', color: '#c084fc', fontWeight: 700,
                                    pointerEvents: 'none', whiteSpace: 'nowrap',
                                    maxWidth: '90%', overflow: 'hidden', textOverflow: 'ellipsis',
                                }}>
                                    <span style={{ width: '6px', height: '6px', borderRadius: '50%', background: '#c084fc', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />
                                    {liveToolCalls[0].label}
                                </div>
                            )}
                        </div>

                        {/* Speech / response overlay at the bottom */}
                        <div style={{
                            flexShrink: 0,
                            background: 'linear-gradient(0deg, rgba(10,8,20,0.97) 60%, transparent)',
                            padding: '12px 14px 6px',
                            maxHeight: '32%',
                            overflowY: 'auto',
                            transition: 'max-height 0.3s ease',
                        }}>
                            {(() => {
                                const cleaned = cleanAiContent(airiSpeech);
                                if (cleaned) return (
                                    <div>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '6px' }}>
                                            <span style={{ fontSize: '10px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: '#c084fc' }}>
                                                {airiSpeaking ? '◉ Speaking' : '✦ AIRI'}
                                            </span>
                                            {airiSpeaking && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#c084fc', display: 'inline-block', animation: 'hubPulse 0.8s infinite' }} />}
                                        </div>
                                        <div
                                            className="markdown-content"
                                            style={{ fontSize: '12.5px', lineHeight: '1.55', color: 'rgba(255,255,255,0.88)' }}
                                            dangerouslySetInnerHTML={{ __html: marked.parse(cleaned.slice(-1000) || '') as string }}
                                        />
                                    </div>
                                );
                                if (isAgentThinking) return (
                                    <div style={{ fontSize: '10px', opacity: 0.3, textAlign: 'center', padding: '6px 0', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '6px' }}>
                                        <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: 'rgba(168,85,247,0.5)', display: 'inline-block', animation: 'hubPulse 1.2s infinite' }} />
                                        thinking...
                                    </div>
                                );
                                return <div style={{ fontSize: '10px', opacity: 0.2, textAlign: 'center', padding: '6px 0' }}>Awaiting mission</div>;
                            })()}
                        </div>

                        {/* Mission input at the very bottom */}
                        <div style={{ flexShrink: 0, padding: '8px 12px', borderTop: '1px solid rgba(168,85,247,0.15)' }}>
                            <div style={{
                                background: 'rgba(168,85,247,0.06)', border: '1px solid rgba(168,85,247,0.2)',
                                borderRadius: '10px', padding: '6px 10px', display: 'flex', gap: '8px', alignItems: 'flex-end'
                            }}>
                                <textarea
                                    ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown} onPaste={handlePaste}
                                    className="agent-mission-input"
                                    placeholder={isAgentThinking ? 'Processing...' : 'Speak to AIRI...'}
                                    disabled={isAgentThinking}
                                    rows={1}
                                    style={{
                                        background: 'transparent', border: 'none', outline: 'none', color: '#fff',
                                        resize: 'none', fontSize: '13px', lineHeight: '1.5', flex: 1, minHeight: '22px',
                                        opacity: isAgentThinking ? 0.4 : 1
                                    }}
                                />
                                <div onClick={() => onSend()} style={{
                                    width: '28px', height: '28px', borderRadius: '50%', flexShrink: 0,
                                    background: (inputValue.trim() && !isAgentThinking) ? '#c084fc' : 'rgba(168,85,247,0.2)',
                                    display: 'flex', alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                                    transition: 'background 0.2s'
                                }}>
                                    <i className="codicon codicon-arrow-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: '#000' }}></i>
                                </div>
                            </div>
                            <div style={{ display: 'flex', gap: '8px', justifyContent: 'center', marginTop: '6px' }}>
                                <span onClick={onModeClick} style={{ fontSize: '9px', opacity: 0.35, cursor: 'pointer' }}>{mode}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span onClick={onModelClick} style={{ fontSize: '9px', opacity: 0.35, cursor: 'pointer' }}>{(model.split('|')[1] || model).split(':')[0]}</span>
                                <span style={{ fontSize: '9px', opacity: 0.2 }}>·</span>
                                <span
                                    onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                    style={{ fontSize: '9px', cursor: 'pointer', color: isYoloMode ? '#f97316' : 'rgba(255,255,255,0.3)', fontWeight: isYoloMode ? 700 : 400 }}
                                >⚡ YOLO</span>
                            </div>
                        </div>
                    </div>
                ) : (

                /* ── CHAT / MISSION HUB MODE ── */
                <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column' }}>
                {view === 'chat' ? (
                    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>

                        {/* AIRI Sentient Header — shrinks when mission is active */}
                        <div style={{
                            position: 'sticky', top: 0, zIndex: 10,
                            background: 'linear-gradient(180deg, var(--vscode-sideBar-background) 60%, transparent)',
                            display: 'flex', flexDirection: 'column', alignItems: 'center',
                            transition: 'all 0.3s ease-in-out',
                            height: messages.length === 0 ? '320px' : '100px',
                            minHeight: messages.length === 0 ? '320px' : '100px',
                            paddingTop: messages.length === 0 ? '24px' : '8px',
                            overflow: 'hidden', pointerEvents: 'none'
                        }}>
                            <div style={{
                                width: messages.length === 0 ? '240px' : '80px',
                                height: messages.length === 0 ? '240px' : '80px',
                                transition: 'all 0.3s ease-in-out'
                            }}>
                                <AiriPanel style={{ width: '100%', height: '100%' }} scale={0.8} yOffset={"-25%"} transparent={true} />
                            </div>
                            {messages.length === 0 && (
                                <div style={{ marginTop: '16px', textAlign: 'center', pointerEvents: 'auto' }}>
                                    <div style={{ fontSize: '13px', fontWeight: 700, marginBottom: '4px', letterSpacing: '0.05em' }}>AIRI SENTIENT CORE</div>
                                    <div style={{ fontSize: '11px', opacity: 0.4 }}>
                                        {isYoloMode ? '⚡ YOLO — Full autonomy' : 'Ready for your mission'}
                                    </div>
                                </div>
                            )}
                        </div>

                        {/* Quick Mission Workflows — shown only on fresh session */}
                        {messages.length === 0 && (
                            <div style={{ padding: '0 12px 12px' }}>
                                <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', opacity: 0.35, marginBottom: '8px' }}>Quick Missions</div>
                                <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '6px' }}>
                                    {[
                                        { icon: '🔍', label: 'Audit Codebase', mission: 'Audit the entire codebase for bugs, dead code, and architectural issues. List findings.' },
                                        { icon: '🛠️', label: 'Fix All Errors', mission: 'Find all compiler errors and runtime issues in this project. Fix them one by one.' },
                                        { icon: '📦', label: 'Git Commit', mission: 'Stage all modified files and create a meaningful commit message based on the changes.' },
                                        { icon: '🚀', label: 'Build & Verify', mission: 'Run cargo build, fix any errors, then verify the implementation is correct.' },
                                        { icon: '📝', label: 'Write Tests', mission: 'Write comprehensive unit tests for the most critical functions in this project.' },
                                        { icon: '🧹', label: 'Refactor', mission: 'Identify and refactor the most complex functions for clarity and performance.' },
                                    ].map(({ icon, label, mission }) => (
                                        <button
                                            key={label}
                                            onClick={() => !isAgentThinking && onSend(mission)}
                                            disabled={isAgentThinking}
                                            style={{
                                                background: 'rgba(255,255,255,0.03)',
                                                border: '1px solid rgba(255,255,255,0.07)',
                                                borderRadius: '8px', padding: '8px 10px',
                                                cursor: isAgentThinking ? 'not-allowed' : 'pointer',
                                                color: 'rgba(255,255,255,0.75)', fontSize: '11px',
                                                textAlign: 'left', display: 'flex', alignItems: 'center', gap: '6px',
                                                transition: 'all 0.15s',
                                                opacity: isAgentThinking ? 0.4 : 1,
                                            }}
                                            onMouseEnter={(e) => { if (!isAgentThinking) { e.currentTarget.style.background = 'rgba(59,130,246,0.08)'; e.currentTarget.style.borderColor = 'rgba(59,130,246,0.3)'; }}}
                                            onMouseLeave={(e) => { e.currentTarget.style.background = 'rgba(255,255,255,0.03)'; e.currentTarget.style.borderColor = 'rgba(255,255,255,0.07)'; }}
                                        >
                                            <span style={{ fontSize: '14px' }}>{icon}</span>
                                            <span style={{ fontWeight: 600 }}>{label}</span>
                                        </button>
                                    ))}
                                </div>
                            </div>
                        )}

                        {/* Live Tool-Call Feed — shows while agent is active */}
                        {(isAgentThinking || liveToolCalls.length > 0) && (() => {
                            // Deduplicate consecutive same-tool calls → show label + count badge
                            const deduped: { id: string; label: string; tool: string; status: 'running' | 'done' | 'error'; count: number }[] = [];
                            for (const tc of liveToolCalls.slice(0, 12)) {
                                const last = deduped[deduped.length - 1];
                                if (last && last.tool === tc.tool) {
                                    last.count++;
                                    if (tc.status === 'running') last.status = 'running';
                                } else {
                                    deduped.push({ ...tc, count: 1 });
                                }
                            }
                            return (
                            <div style={{
                                margin: '0 12px 8px',
                                background: 'rgba(15,15,25,0.8)',
                                border: `1px solid ${isAgentThinking ? 'rgba(249,115,22,0.25)' : 'rgba(16,185,129,0.2)'}`,
                                borderRadius: '10px', padding: '8px 12px',
                                transition: 'border-color 0.5s'
                            }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                    <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.1em', color: isAgentThinking ? 'rgba(249,115,22,0.7)' : 'rgba(16,185,129,0.6)' }}>
                                        {isAgentThinking ? (isYoloMode ? '⚡ YOLO Executing' : '● Live Actions') : '✓ Completed'}
                                    </div>
                                    <div
                                        onClick={() => setLiveToolCalls([])}
                                        style={{ cursor: 'pointer', fontSize: '11px', opacity: 0.35, padding: '0 2px', lineHeight: 1 }}
                                        title="Clear feed"
                                    >✕</div>
                                </div>
                                {deduped.length === 0 && isAgentThinking && (
                                    <div style={{ fontSize: '11px', opacity: 0.4, fontStyle: 'italic' }}>Thinking...</div>
                                )}
                                {deduped.slice(0, 6).map(tc => (
                                    <div key={tc.id} style={{
                                        display: 'flex', alignItems: 'center', gap: '6px',
                                        fontSize: '11px', padding: '2px 0',
                                        color: tc.status === 'done' ? 'rgba(255,255,255,0.25)' : tc.status === 'error' ? '#ef4444' : 'rgba(255,255,255,0.8)',
                                    }}>
                                        {tc.status === 'running' && <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#f97316', display: 'inline-block', animation: 'hubPulse 1s infinite', flexShrink: 0 }} />}
                                        {tc.status === 'done' && <span style={{ color: '#10b981', fontSize: '10px', flexShrink: 0 }}>✓</span>}
                                        {tc.status === 'error' && <span style={{ fontSize: '10px', flexShrink: 0 }}>✗</span>}
                                        <span style={{ flex: 1, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{tc.label}</span>
                                        {tc.count > 1 && (
                                            <span style={{
                                                fontSize: '9px', fontWeight: 700, padding: '0 5px', borderRadius: '10px',
                                                background: 'rgba(255,255,255,0.08)', color: 'rgba(255,255,255,0.4)',
                                                flexShrink: 0
                                            }}>×{tc.count}</span>
                                        )}
                                    </div>
                                ))}
                            </div>
                            );
                        })()}

                        {/* Agent task progress bars */}
                        {agentTasks.filter((t: any) => t.status === 'running' && t.id.includes('-')).map((task: any) => (
                            <div key={task.id} style={{
                                margin: '0 12px 8px',
                                background: 'rgba(59,130,246,0.05)',
                                border: '1px solid rgba(59,130,246,0.2)',
                                padding: '10px 12px', borderRadius: '8px',
                            }}>
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                        <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', color: '#3b82f6' }}></i>
                                        <span style={{ fontSize: '11px', fontWeight: 600 }}>{task.title}</span>
                                    </div>
                                    <span style={{ fontSize: '10px', opacity: 0.5 }}>{task.progress}%</span>
                                </div>
                                <div style={{ background: 'rgba(0,0,0,0.3)', height: '3px', borderRadius: '2px', overflow: 'hidden' }}>
                                    <div style={{ background: '#3b82f6', width: `${task.progress}%`, height: '100%', transition: 'width 0.3s ease' }} />
                                </div>
                                {task.message && <div style={{ fontSize: '10px', marginTop: '4px', opacity: 0.5 }}>{task.message}</div>}
                            </div>
                        ))}

                        {/* Mission log — agent messages */}
                        <div style={{ display: 'flex', flexDirection: 'column', padding: '8px 12px', gap: '12px', flex: 1 }}>
                            {messages.length > 0 && (
                                <>
                                    {messages.map((msg, idx) => (
                                        <div key={idx} className="agent-message-container" style={{ display: 'flex', flexDirection: 'column', gap: '6px', position: 'relative' }}>
                                            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', opacity: 0.5 }}>
                                                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                                    <span style={{ fontSize: '10px' }}>{msg.role === 'assistant' ? (msg.isSubAgentResponse ? '🤖' : '✦') : '▸'}</span>
                                                    <span style={{ fontSize: '10px', fontWeight: 800, textTransform: 'uppercase', letterSpacing: '0.06em', color: msg.isSubAgentResponse ? '#3b82f6' : msg.role === 'assistant' ? 'rgba(255,255,255,0.6)' : '#3b82f6' }}>
                                                        {msg.role === 'assistant' ? (msg.isSubAgentResponse ? 'AIRI-MODULE' : 'AIRI') : 'MISSION'}
                                                    </span>
                                                    {msg.role === 'assistant' && isAgentThinking && idx === messages.length - 1 && (
                                                        <span style={{ fontSize: '9px', opacity: 0.4, textTransform: 'uppercase' }}>{avatarState}</span>
                                                    )}
                                                    {msg.role === 'assistant' && !isAgentThinking && msg.timestamp && (
                                                        <span style={{ fontSize: '9px', opacity: 0.25 }}>{new Date(msg.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}</span>
                                                    )}
                                                </div>
                                                <div className="message-actions" style={{ opacity: 0, transition: 'opacity 0.2s', display: 'flex', gap: '8px' }}>
                                                    {msg.role === 'user' && !isAgentThinking && (
                                                        <i className="codicon codicon-edit" style={{ cursor: 'pointer', fontSize: '11px' }} title="Edit"
                                                            onClick={() => { setEditingMsgIdx(idx); setEditValue(msg.content); }}></i>
                                                    )}
                                                    {msg.role === 'assistant' && (
                                                        <i className={`codicon codicon-${lastCopiedIdx === idx ? 'check' : 'copy'}`}
                                                            style={{ cursor: 'pointer', fontSize: '11px', color: lastCopiedIdx === idx ? '#10b981' : 'inherit' }}
                                                            onClick={() => handleCopy(msg.content, idx)}></i>
                                                    )}
                                                </div>
                                            </div>
                                            <div style={{
                                                background: msg.role === 'user'
                                                    ? 'rgba(59,130,246,0.06)'
                                                    : (msg.isSubAgentResponse ? 'rgba(59,130,246,0.03)' : 'rgba(255,255,255,0.01)'),
                                                padding: '10px 14px', borderRadius: '10px',
                                                border: msg.role === 'user'
                                                    ? '1px solid rgba(59,130,246,0.15)'
                                                    : '1px solid rgba(255,255,255,0.04)'
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
                                                                <summary style={{ fontSize: '10px', cursor: 'pointer', fontWeight: 600 }}>Cognitive trace...</summary>
                                                                <div style={{ fontSize: '10px', padding: '8px', background: 'rgba(0,0,0,0.2)', borderRadius: '6px', marginTop: '4px', fontFamily: 'var(--font-mono)' }}>{msg.thoughts}</div>
                                                            </details>
                                                        )}
                                                        {msg.steps && msg.steps.length > 0 && (
                                                            <div style={{ display: 'flex', flexDirection: 'column', gap: '3px', marginBottom: '8px' }}>
                                                                {msg.steps.map((step: any, sIdx: number) => {
                                                                    const statusColor = step.status === 'running' ? '#3b82f6' : step.status === 'success' ? '#10b981' : step.status === 'error' ? '#ef4444' : '#555';
                                                                    return (
                                                                        <div key={sIdx} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', opacity: 0.7 }}>
                                                                            <span style={{ color: statusColor, fontSize: '8px' }}>●</span>
                                                                            <span style={{ fontFamily: 'var(--font-mono)' }}>{step.name}</span>
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
                                                        {(() => {
                                                            const cleaned = cleanAiContent(msg.content || '');
                                                            if (!cleaned && msg.role === 'assistant') {
                                                                // Content is still being built (tool calls in progress)
                                                                return (
                                                                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', opacity: 0.4 }}>
                                                                        <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#3b82f6', display: 'inline-block', animation: 'hubPulse 1s infinite' }} />
                                                                        <span style={{ fontSize: '11px', fontStyle: 'italic' }}>
                                                                            {isAgentThinking ? 'Working...' : 'Processing response...'}
                                                                        </span>
                                                                    </div>
                                                                );
                                                            }
                                                            return <div className="markdown-content" style={{ fontSize: '13px', lineHeight: '1.6' }} dangerouslySetInnerHTML={{ __html: marked.parse(cleaned) as string }} />;
                                                        })()}
                                                    </>
                                                )}
                                            </div>
                                        </div>
                                    ))}
                                    {isAgentThinking && (
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', padding: '8px 12px', opacity: 0.3 }}>
                                            <span style={{ width: '5px', height: '5px', borderRadius: '50%', background: '#3b82f6', display: 'inline-block', animation: 'hubPulse 1s infinite' }} />
                                            <span style={{ fontSize: '10px', fontStyle: 'italic', fontFamily: 'var(--font-mono)' }}>processing...</span>
                                        </div>
                                    )}
                                    <div ref={messagesEndRef} />
                                </>
                            )}
                        </div>
                    </div>
                ) : view === 'kortex' ? (
                    /* Kortex .aim Brain Panel */
                    <div style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
                        <div style={{ padding: '16px 16px 8px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                                <span style={{ fontSize: '16px' }}>🧠</span>
                                <div>
                                    <div style={{ fontSize: '11px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em' }}>Kortex Brain</div>
                                    <div style={{ fontSize: '10px', opacity: 0.4 }}>{kortexSlots.length} knowledge slots</div>
                                </div>
                            </div>
                            <button
                                onClick={refreshKortex}
                                disabled={kortexLoading}
                                style={{ background: 'rgba(168,85,247,0.1)', border: '1px solid rgba(168,85,247,0.3)', color: '#a855f7', padding: '4px 10px', borderRadius: '6px', fontSize: '10px', cursor: kortexLoading ? 'not-allowed' : 'pointer', fontWeight: 600 }}
                            >
                                {kortexLoading ? '...' : 'Refresh'}
                            </button>
                        </div>

                        {/* Category breakdown */}
                        {kortexSlots.length > 0 && (() => {
                            const cats: Record<string, number> = {};
                            kortexSlots.forEach(s => { cats[s.category] = (cats[s.category] || 0) + 1; });
                            return (
                                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', padding: '0 16px 12px' }}>
                                    {Object.entries(cats).map(([cat, count]) => (
                                        <span key={cat} style={{
                                            fontSize: '9px', padding: '2px 7px', borderRadius: '10px',
                                            background: 'rgba(168,85,247,0.1)', border: '1px solid rgba(168,85,247,0.25)',
                                            color: '#c084fc', fontWeight: 600
                                        }}>{cat} ({count})</span>
                                    ))}
                                </div>
                            );
                        })()}

                        <div style={{ flex: 1, overflowY: 'auto', padding: '0 12px 12px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                            {kortexLoading ? (
                                <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                    <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '20px', display: 'block', marginBottom: '8px' }}></i>
                                    Loading neural weights...
                                </div>
                            ) : kortexSlots.length === 0 ? (
                                <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                                    <span style={{ fontSize: '32px', display: 'block', marginBottom: '8px' }}>🧠</span>
                                    No knowledge stored yet.<br />
                                    <span style={{ fontSize: '10px', opacity: 0.6 }}>Run a mission to populate the brain.</span>
                                </div>
                            ) : (
                                kortexSlots.map((slot, i) => (
                                    <div key={slot.id || i} style={{
                                        background: 'rgba(168,85,247,0.04)',
                                        border: '1px solid rgba(168,85,247,0.12)',
                                        borderRadius: '8px', padding: '8px 12px',
                                    }}>
                                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '4px' }}>
                                            <span style={{
                                                fontSize: '8px', padding: '1px 5px', borderRadius: '8px',
                                                background: 'rgba(168,85,247,0.15)', color: '#c084fc', fontWeight: 700, textTransform: 'uppercase'
                                            }}>{slot.category}</span>
                                            <span style={{ fontSize: '9px', opacity: 0.3, marginLeft: 'auto' }}>
                                                {new Date(slot.timestamp * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                            </span>
                                        </div>
                                        <div style={{ fontSize: '11px', opacity: 0.8, lineHeight: 1.4, fontFamily: 'var(--font-mono)' }}>
                                            {slot.content.slice(0, 140)}{slot.content.length > 140 ? '…' : ''}
                                        </div>
                                        {slot.tags && slot.tags.length > 0 && (
                                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px', marginTop: '6px' }}>
                                                {slot.tags.map((tag: string, ti: number) => (
                                                    <span key={ti} style={{ fontSize: '9px', opacity: 0.4, padding: '1px 4px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>#{tag}</span>
                                                ))}
                                            </div>
                                        )}
                                    </div>
                                ))
                            )}
                        </div>
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
                )}
            </div>
            {
                view === 'chat' && agentUiMode === 'chat' && (
                    <div style={{ padding: '16px', borderTop: '1px solid var(--vscode-sideBar-border, rgba(255,255,255,0.1))', position: 'relative' }}>
                        {/* @file mention dropdown */}
                        {isMentionDropdownOpen && filteredSuggestions.length > 0 && (
                            <div style={{
                                position: 'absolute', bottom: '100%', left: '16px', right: '16px',
                                background: 'var(--vscode-menu-background, #1e1e2e)',
                                border: '1px solid rgba(168,85,247,0.35)',
                                borderRadius: '10px', overflow: 'hidden',
                                boxShadow: '0 -8px 32px rgba(0,0,0,0.5)',
                                zIndex: 100, marginBottom: '4px',
                            }}>
                                <div style={{ padding: '4px 10px 2px', fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em', color: 'rgba(168,85,247,0.8)', borderBottom: '1px solid rgba(255,255,255,0.06)' }}>
                                    @ Files — type to filter
                                </div>
                                {filteredSuggestions.map((file, i) => (
                                    <div
                                        key={file.path}
                                        onMouseDown={() => handleMentionSelect(file)}
                                        style={{
                                            padding: '6px 12px', cursor: 'pointer', fontSize: '12px',
                                            background: i === selectedMentionIndex ? 'rgba(168,85,247,0.15)' : 'transparent',
                                            color: i === selectedMentionIndex ? '#c084fc' : 'rgba(255,255,255,0.75)',
                                            display: 'flex', alignItems: 'center', gap: '8px',
                                            borderLeft: i === selectedMentionIndex ? '2px solid #c084fc' : '2px solid transparent',
                                            transition: 'all 0.1s',
                                        }}
                                    >
                                        <i className="codicon codicon-file" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px', opacity: 0.6 }} />
                                        <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{file.name}</span>
                                        <span style={{ fontSize: '9px', opacity: 0.35, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', maxWidth: '80px' }}>
                                            {file.path.split(/[\\/]/).slice(-3, -1).join('/')}
                                        </span>
                                    </div>
                                ))}
                            </div>
                        )}
                        <div style={{
                            background: 'var(--vscode-input-background)', border: '1px solid var(--vscode-input-border, transparent)',
                            borderRadius: '12px', padding: '8px 12px', display: 'flex', flexDirection: 'column'
                        }}>
                            <textarea
                                ref={inputRef} value={inputValue} onChange={handleInputChange} onKeyDown={handleKeyDown}
                                className="agent-mission-input"
                                placeholder={isAgentThinking ? 'Agent executing...' : 'Launch a mission...  (type @ to mention a file)'}
                                disabled={isAgentThinking}
                                style={{ background: 'transparent', border: 'none', outline: 'none', color: '#fff', resize: 'none', fontSize: '13px', lineHeight: '1.5', width: '100%', minHeight: '32px', opacity: isAgentThinking ? 0.5 : 1 }}
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
                                    {/* Token counter */}
                                    <span style={{ fontSize: '9px', opacity: 0.35, fontVariantNumeric: 'tabular-nums' }} title="Estimated context tokens">
                                        ~{Math.round(messages.reduce((n, m) => n + (typeof m.content === 'string' ? m.content.length : 0), 0) / 4).toLocaleString()}t
                                    </span>
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
                                        {/* Auto-accept toggle */}
                                        <div
                                            onClick={() => setAutoAcceptChanges(!autoAcceptChanges)}
                                            style={{
                                                cursor: 'pointer',
                                                color: autoAcceptChanges ? '#10b981' : 'rgba(255,255,255,0.35)',
                                                display: 'flex', alignItems: 'center', gap: '3px',
                                                fontSize: '10px', fontWeight: 600,
                                                padding: '1px 5px',
                                                borderRadius: '4px',
                                                background: autoAcceptChanges ? 'rgba(16,185,129,0.15)' : 'transparent',
                                                border: autoAcceptChanges ? '1px solid rgba(16,185,129,0.4)' : '1px solid transparent',
                                                transition: 'all 0.2s'
                                            }}
                                            title={autoAcceptChanges ? "Auto-accept ON — agent edits apply instantly" : "Auto-accept OFF — review each diff before applying"}
                                        >
                                            <i className="codicon codicon-check-all" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }}></i>
                                            <span>AUTO</span>
                                        </div>
                                        {/* Checkpoint restore */}
                                        {checkpoint && (
                                            <div
                                                onClick={() => { revertToCheckpoint(); }}
                                                style={{
                                                    cursor: 'pointer',
                                                    color: '#f59e0b',
                                                    display: 'flex', alignItems: 'center', gap: '3px',
                                                    fontSize: '10px', fontWeight: 600,
                                                    padding: '1px 5px',
                                                    borderRadius: '4px',
                                                    background: 'rgba(245,158,11,0.12)',
                                                    border: '1px solid rgba(245,158,11,0.35)',
                                                    transition: 'all 0.2s'
                                                }}
                                                title="Restore checkpoint — undo all agent file changes"
                                            >
                                                <i className="codicon codicon-history" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }}></i>
                                                <span>UNDO</span>
                                            </div>
                                        )}
                                        <div
                                            onClick={() => import('../agent').then(m => m.setYoloMode(!isYoloMode).then(() => setYoloMode(!isYoloMode)))}
                                            style={{
                                                cursor: 'pointer',
                                                color: isYoloMode ? '#f97316' : 'rgba(255,255,255,0.35)',
                                                display: 'flex', alignItems: 'center', gap: '3px',
                                                fontSize: '10px', fontWeight: 600,
                                                padding: '1px 5px',
                                                borderRadius: '4px',
                                                background: isYoloMode ? 'rgba(249,115,22,0.15)' : 'transparent',
                                                border: isYoloMode ? '1px solid rgba(249,115,22,0.4)' : '1px solid transparent',
                                                transition: 'all 0.2s'
                                            }}
                                            title={isYoloMode ? "YOLO MODE ON — Click to disable" : "Enable YOLO Mode (full autonomy, no blockers)"}
                                        >
                                            <span style={{ fontStyle: 'normal' }}>⚡</span>
                                            <span>YOLO</span>
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
                )
            }
        </aside >
    );
};

export default RightSidebar;
