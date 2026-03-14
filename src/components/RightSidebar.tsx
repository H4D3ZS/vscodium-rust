import React, { useEffect } from 'react';
import { useStore } from '../store';
import { setupAgentUI } from '../agent';

const RightSidebar: React.FC = () => {
    const isOpen = useStore(state => state.isRightSidebarOpen);
    const toggle = useStore(state => state.toggleRightSidebar);
    const aiStatus = useStore(state => state.aiStatus);
    const tokenUsage = useStore(state => state.tokenUsage);
    const mode = useStore(state => state.agentMode);
    const setMode = useStore(state => state.setAgentMode);
    const model = useStore(state => state.agentModel);
    const setModel = useStore(state => state.setAgentModel);
    const inputRef = React.useRef<HTMLTextAreaElement>(null);

    const onSend = () => {
        if (inputRef.current) {
            import('../agent').then(m => m.handleAgentChat(inputRef.current!));
        }
    };

    const onKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            onSend();
        }
    };

    const onContextClick = (e: React.MouseEvent) => {
        import('../agent').then(m => m.openContextDropdown(e.currentTarget as HTMLElement, (ctx) => {
            if (inputRef.current) {
                inputRef.current.value += ` [Context: ${ctx}] `;
                inputRef.current.focus();
            }
        }));
    };

    const [showModeDropdown, setShowModeDropdown] = React.useState(false);
    const [showModelDropdown, setShowModelDropdown] = React.useState(false);

    const onModeClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        setShowModeDropdown(!showModeDropdown);
        setShowModelDropdown(false);
    };

    const onModelClick = (e: React.MouseEvent) => {
        e.stopPropagation();
        setShowModelDropdown(!showModelDropdown);
        setShowModeDropdown(false);
    };

    const handleModeSelect = (m: string) => {
        setMode(m);
        setShowModeDropdown(false);
        if (m.includes("Source Control")) {
            useStore.getState().setActiveSidebarView('planning-view');
        }
    };

    const handleModelSelect = (m: string) => {
        setModel(m);
        setShowModelDropdown(false);
    };

    useEffect(() => {
        const handleClickOutside = () => {
            setShowModeDropdown(false);
            setShowModelDropdown(false);
        };
        document.addEventListener('click', handleClickOutside);
        return () => document.removeEventListener('click', handleClickOutside);
    }, []);

    if (!isOpen) return null;

    const modes = [
        { label: "Planning", value: "Planning", desc: "Plan before executing tasks" },
        { label: "Planning (Source Control)", value: "Planning (Source Control)", desc: "Deep dive into git history" },
        { label: "Fast", value: "Fast", desc: "Execute tasks directly" }
    ];

    const models = [
        { label: "Gemini 1.5 Pro (High)", value: "Google|gemini-1.5-pro" },
        { label: "Gemini 1.5 Flash (Low)", value: "Google|gemini-1.5-flash" },
        { label: "Claude 3.5 Sonnet", value: "Anthropic|claude-3-5-sonnet-20241022" },
        { label: "Qwen 3.5 Plus (Fast)", value: "Alibaba|qwen3.5-plus" },
        { label: "GPT4-o (Router)", value: "OpenRouter|openai/gpt-4o" }
    ];

    return (
        <aside className="right-sidebar" id="right-sidebar" style={{ display: 'flex', width: '100%', height: '100%', flexDirection: 'column', background: 'var(--vscode-sideBar-background)', borderLeft: '1px solid var(--vscode-panel-border)', zIndex: 10 }}>
            {/* ... rest of header ... */}
            <div className="right-sidebar-header" style={{ height: '35px', display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '0 10px', background: 'var(--vscode-editorGroupHeader-tabsBackground)', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span style={{ fontSize: '11px', fontWeight: 400, fontFamily: 'var(--font-ui)', color: 'var(--vscode-sideBar-foreground)', opacity: 0.8 }}>Agent</span>
                    <i className={`codicon ${aiStatus === 'alive' ? 'codicon-leaf' : 'codicon-skull'}`} 
                       title={aiStatus === 'alive' ? 'Agent is Alive' : 'Agent is Offline'}
                       style={{ color: aiStatus === 'alive' ? '#4ade80' : '#f87171', fontSize: '13px' }}></i>
                </div>
                <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                    <i className="codicon codicon-close hoverable" title="Toggle Agent (⌥⌘B)" onClick={toggle} style={{ color: 'var(--vscode-sideBar-foreground)', cursor: 'pointer', fontSize: '16px', marginLeft: '4px' }}></i>
                </div>
            </div>

            <div className="agent-tabs" style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', padding: '12px 15px' }}>
                <div style={{ fontSize: '18px', fontWeight: 600, color: 'var(--vscode-sideBar-foreground)', letterSpacing: '-0.3px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>vscodium-rust</div>
            </div>

            <div id="agent-messages" style={{ flex: 1, overflowY: 'auto', padding: '15px', fontSize: '13px', display: 'flex', flexDirection: 'column', gap: '15px', color: 'var(--vscode-editor-foreground)' }}>
            </div>

            <div className="agent-input-section" style={{ padding: '10px', borderTop: '1px solid var(--vscode-panel-border)', position: 'relative' }}>
                {showModeDropdown && (
                    <div style={{ position: 'absolute', bottom: '100%', left: '10px', width: '220px', background: '#252526', border: '1px solid #454545', borderRadius: '6px', boxShadow: '0 4px 14px rgba(0,0,0,0.5)', zIndex: 100, padding: '4px 0' }}>
                        {modes.map(m => (
                            <div key={m.value} onClick={() => handleModeSelect(m.value)} style={{ padding: '8px 12px', cursor: 'pointer', color: '#ccc', fontSize: '12px' }} className="hoverable">
                                <div style={{ color: '#fff', fontWeight: 500 }}>{m.label}</div>
                                <div style={{ fontSize: '10px', opacity: 0.6 }}>{m.desc}</div>
                            </div>
                        ))}
                    </div>
                )}
                {showModelDropdown && (
                    <div style={{ position: 'absolute', bottom: '100%', left: '80px', width: '220px', background: '#252526', border: '1px solid #454545', borderRadius: '6px', boxShadow: '0 4px 14px rgba(0,0,0,0.5)', zIndex: 100, padding: '4px 0' }}>
                        {models.map(m => (
                            <div key={m.value} onClick={() => handleModelSelect(m.value)} style={{ padding: '8px 12px', cursor: 'pointer', color: '#fff', fontSize: '12px' }} className="hoverable">
                                {m.label}
                            </div>
                        ))}
                    </div>
                )}

                <div className="agent-input-wrapper" style={{ background: 'var(--vscode-editor-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '8px', display: 'flex', flexDirection: 'column', overflow: 'hidden' }}>
                    <textarea 
                        ref={inputRef}
                        id="agent-input" 
                        onKeyDown={onKeyDown}
                        placeholder="Ask anything..." 
                        style={{ width: '100%', minHeight: '80px', maxHeight: '250px', background: 'transparent', border: 'none', color: 'var(--vscode-editor-foreground)', padding: '10px', resize: 'none', fontSize: '13px', outline: 'none' }}
                    ></textarea>

                    <div className="agent-input-toolbar" style={{ display: 'flex', alignItems: 'center', padding: '6px 8px', background: 'var(--vscode-editorGroupHeader-tabsBackground)', borderTop: '1px solid var(--vscode-panel-border)', gap: '4px' }}>
                        <div className="agent-dropdown" onClick={onModeClick} style={{ display: 'flex', alignItems: 'center', background: 'var(--vscode-activityBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', cursor: 'pointer', fontSize: '11px', gap: '4px', padding: '2px 6px', color: 'var(--vscode-sideBar-foreground)', maxWidth: '100px', overflow: 'hidden' }}>
                            <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{mode}</span>
                            <i className="codicon codicon-chevron-up" style={{ fontSize: '10px' }}></i>
                        </div>

                        <div className="agent-dropdown" onClick={onModelClick} style={{ display: 'flex', alignItems: 'center', background: 'var(--vscode-activityBar-background)', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', cursor: 'pointer', fontSize: '11px', gap: '4px', padding: '2px 6px', color: 'var(--vscode-sideBar-foreground)', maxWidth: '120px', overflow: 'hidden' }}>
                            <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{model.split('|')[1] || model}</span>
                            <i className="codicon codicon-chevron-up" style={{ fontSize: '10px' }}></i>
                        </div>

                        <div style={{ flex: 1 }}></div>
                        <button className="agent-icon-btn" onClick={onSend} id="agent-send" style={{ color: '#f85b5b', border: 'none', background: 'none', cursor: 'pointer', padding: '4px' }}><i className="codicon codicon-debug-stop"></i></button>
                    </div>
                </div>
            </div>
        </aside>
    );
};

export default RightSidebar;
