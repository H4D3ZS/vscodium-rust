import React, { useState, useRef, useEffect } from 'react';
import { useStore } from '../../store';
import { Bot, FileText, CheckCircle, Clock, Plus, Zap, Shield, Check, X, Info, Send, Activity, Search, Code, GlassWater, Milestone } from 'lucide-react';
import type { Artifact } from '../../store';
import { sendAgentMessage } from '../../agent';
import { listen } from '@tauri-apps/api/event';

const TaskRoadmap: React.FC = () => {
    const currentPhase = useStore(state => state.currentPhase);
    const status = useStore(state => state.currentPhaseStatus);
    const isThinking = useStore(state => state.isAgentThinking);

    if (currentPhase === 'IDLE' || !isThinking) return null;

    const phases = ['ANALYZE', 'PLAN', 'EXECUTE', 'VERIFY', 'REPORT'];
    const activeIndex = phases.indexOf(currentPhase);

    return (
        <div className="task-roadmap" style={{
            margin: '16px 24px',
            padding: '16px',
            background: 'rgba(var(--terminator-accent-rgb), 0.05)',
            border: '1px solid rgba(var(--terminator-accent-rgb), 0.2)',
            borderRadius: '8px',
            animation: 'fadeIn 0.3s ease-out'
        }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '16px' }}>
                {phases.map((p, i) => (
                    <div key={p} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '4px', flex: 1, position: 'relative' }}>
                        <div style={{
                            width: '24px',
                            height: '24px',
                            borderRadius: '50%',
                            background: i <= activeIndex ? 'var(--terminator-accent)' : 'var(--vscode-panel-border)',
                            color: i <= activeIndex ? '#fff' : 'var(--vscode-foreground)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontSize: '10px',
                            fontWeight: 700,
                            zIndex: 2,
                            boxShadow: i === activeIndex ? '0 0 10px var(--terminator-accent)' : 'none'
                        }}>
                            {i < activeIndex ? <Check size={14} /> : i + 1}
                        </div>
                        <span style={{ fontSize: '9px', fontWeight: 600, opacity: i <= activeIndex ? 1 : 0.4 }}>{p}</span>
                        {i < phases.length - 1 && (
                            <div style={{
                                position: 'absolute',
                                left: '50%',
                                top: '12px',
                                width: '100%',
                                height: '2px',
                                background: i < activeIndex ? 'var(--terminator-accent)' : 'var(--vscode-panel-border)',
                                zIndex: 1
                            }} />
                        )}
                    </div>
                ))}
            </div>
            <div style={{ fontSize: '11px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                <Activity size={14} style={{ color: 'var(--terminator-accent)' }} className="animate-pulse" />
                <span style={{ fontWeight: 600 }}>{status}</span>
            </div>
        </div>
    );
};

const ArtifactCard: React.FC<{ artifact: Artifact; onApprove: () => void; onReject: () => void }> = ({ artifact, onApprove, onReject }) => {
    return (
        <div className="artifact-card" style={{ marginBottom: '16px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                <span style={{ fontSize: '12px', fontWeight: 600, color: 'var(--terminator-accent)' }}>
                    {artifact.type.toUpperCase()}
                </span>
                <span style={{ fontSize: '10px', opacity: 0.5 }}>
                    {new Date(artifact.timestamp).toLocaleTimeString()}
                </span>
            </div>
            <div style={{ fontSize: '13px', fontWeight: 500, marginBottom: '4px' }}>{artifact.title || 'Untitled Artifact'}</div>
            <div style={{ fontSize: '11px', opacity: 0.7, marginBottom: '12px', lineHeight: 1.4 }}>{artifact.description}</div>

            <div style={{ display: 'flex', gap: '8px' }}>
                <button
                    onClick={onApprove}
                    style={{ flex: 1, padding: '6px', background: 'var(--terminator-success)', border: 'none', borderRadius: '4px', color: 'white', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '4px', fontSize: '11px' }}
                >
                    <Check size={14} /> Approve
                </button>
                <button
                    onClick={onReject}
                    style={{ flex: 1, padding: '6px', background: 'var(--vscode-button-secondaryBackground)', border: 'none', borderRadius: '4px', color: 'white', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '4px', fontSize: '11px' }}
                >
                    <X size={14} /> Reject
                </button>
            </div>
        </div>
    );
};

const AgentManager: React.FC = () => {
    const agentThreads = useStore(state => state.agentThreads);
    const activeId = useStore(state => state.activeAgentThreadId);
    const setActiveThread = useStore(state => state.setActiveAgentThread);
    const createThread = useStore(state => state.createAgentThread);
    const agentMessages = useStore(state => state.agentMessages);
    const isThinking = useStore(state => state.isAgentThinking);

    const [inputValue, setInputValue] = useState('');
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const inputRef = useRef<HTMLTextAreaElement>(null);

    const threads = Object.values(agentThreads);
    const activeThread = agentThreads[activeId];

    const setPhase = useStore(state => state.setPhase);

    // Auto-scroll to bottom when messages change
    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [agentMessages]);

    // Listen for phase updates from the Sentient engine
    useEffect(() => {
        let unlisten: any;
        const setup = async () => {
            unlisten = await listen<any>('task-phase-update', (event) => {
                setPhase(event.payload.phase, event.payload.status);
            });
        };
        setup();
        return () => { if (unlisten) unlisten(); };
    }, [setPhase]);

    const handleSend = async () => {
        const prompt = inputValue.trim();
        if (!prompt || isThinking) return;

        setInputValue('');

        // If no active thread, create one
        if (!activeId || !activeThread) {
            createThread(prompt.slice(0, 40) + (prompt.length > 40 ? '...' : ''));
        }

        // Add user message to the global store
        const store = useStore.getState();
        store.addAgentMessage('user', prompt);
        store.addAgentMessage('assistant', '');
        store.setIsAgentThinking(true);

        try {
            await sendAgentMessage(prompt, () => { });
        } catch (error: any) {
            console.error('Agent Manager chat error:', error);
            store.setIsAgentThinking(false);
            store.updateLastAgentMessage(`**Error:** ${error.message || error}`);
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter' && !e.shiftKey) {
            e.preventDefault();
            handleSend();
        }
    };

    return (
        <div className="agent-manager-surface" style={{ display: 'flex', flex: 1, height: '100%', overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
            {/* Sidebar for Thread Management */}
            <div className="agent-threads-sidebar" style={{ width: '260px', borderRight: '1px solid var(--vscode-panel-border)', display: 'flex', flexDirection: 'column' }}>
                <div style={{ padding: '12px 16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                    <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.6 }}>Active Agents</span>
                    <button onClick={() => createThread('New Mission')} style={{ background: 'none', border: 'none', color: 'var(--terminator-accent)', cursor: 'pointer' }}><Plus size={16} /></button>
                </div>
                <div style={{ flex: 1, overflowY: 'auto' }}>
                    {threads.map(thread => (
                        <div
                            key={thread.id}
                            onClick={() => setActiveThread(thread.id)}
                            style={{
                                padding: '12px 16px',
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                cursor: 'pointer',
                                background: activeId === thread.id ? 'var(--vscode-list-activeSelectionBackground)' : 'transparent',
                                borderLeft: activeId === thread.id ? '2px solid var(--terminator-accent)' : 'none'
                            }}
                        >
                            <div style={{ fontWeight: 500, fontSize: '13px' }}>{thread.name}</div>
                            <div style={{ fontSize: '11px', opacity: 0.5, marginTop: '4px', display: 'flex', alignItems: 'center', gap: '4px' }}>
                                {thread.isThinking ? <Clock size={12} className="spinning" /> : <CheckCircle size={12} />}
                                {thread.messages.length} messages
                            </div>
                        </div>
                    ))}
                    {threads.length === 0 && (
                        <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.3 }}>
                            <Bot size={48} style={{ display: 'block', margin: '0 auto 12px' }} />
                            <div style={{ fontSize: '12px' }}>No active agent threads.</div>
                            <div style={{ fontSize: '11px', marginTop: '8px' }}>Start by typing a message below.</div>
                        </div>
                    )}
                </div>
            </div>

            {/* Main Conversation & Planning Area */}
            <div className="agent-conversation-area" style={{ flex: 1, display: 'flex', flexDirection: 'column', background: 'var(--vscode-sideBar-background)', position: 'relative' }}>
                {/* Header */}
                <div style={{ padding: '16px 24px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)' }}>
                    <h2 style={{ margin: 0, fontSize: '18px', display: 'flex', alignItems: 'center', gap: '8px' }}>
                        <Bot size={20} color="var(--terminator-accent)" />
                        {activeThread ? activeThread.name : 'Antigravity Agent'}
                    </h2>
                    <div style={{ display: 'flex', gap: '16px', marginTop: '12px' }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '11px', opacity: 0.7 }}>
                            <Shield size={14} /> Artifact Review:
                            <select
                                value={useStore.getState().artifactReviewPolicy}
                                onChange={(e) => useStore.getState().setArtifactReviewPolicy(e.target.value as any)}
                                style={{ background: 'var(--vscode-sideBar-background)', color: 'inherit', border: '1px solid var(--vscode-panel-border)', borderRadius: '4px', padding: '2px 4px' }}
                            >
                                <option value="request_review">Request Review</option>
                                <option value="always_proceed">Always Proceed (Unsafe)</option>
                            </select>
                        </div>
                    </div>
                </div>

                {/* Sentient Roadmap UI */}
                <TaskRoadmap />

                {/* Messages */}
                <div style={{ flex: 1, overflowY: 'auto', padding: '24px' }}>
                    {agentMessages.length === 0 && (
                        <div style={{ textAlign: 'center', opacity: 0.2, marginTop: '80px' }}>
                            <Zap size={64} style={{ marginBottom: '16px' }} />
                            <h3>What do you want to build?</h3>
                            <p style={{ fontSize: '12px' }}>Describe your task or use a slash command like /specify</p>
                        </div>
                    )}
                    {agentMessages.map((m: any, i: number) => (
                        <div key={i} style={{ marginBottom: '24px', maxWidth: '800px', margin: '0 auto 24px' }}>
                            <div style={{ fontWeight: 600, marginBottom: '8px', fontSize: '12px', color: m.role === 'user' ? 'var(--terminator-accent)' : 'var(--terminator-success)' }}>
                                {m.role === 'user' ? 'YOU' : 'ANTIGRAVITY'}
                                {isThinking && i === agentMessages.length - 1 && m.role === 'assistant' && (
                                    <Clock size={12} className="spinning" style={{ marginLeft: '8px' }} />
                                )}
                            </div>
                            {m.content && (
                                <div className={`agent-bubble ${m.role}`} style={{ lineHeight: 1.6, whiteSpace: 'pre-wrap', fontSize: '13px' }}>
                                    {m.content}
                                </div>
                            )}
                            {m.steps && m.steps.length > 0 && (
                                <div style={{ marginTop: '8px', display: 'flex', flexWrap: 'wrap', gap: '4px' }}>
                                    {m.steps.map((step: any, si: number) => (
                                        <span key={si} style={{
                                            fontSize: '10px', padding: '2px 6px', borderRadius: '3px',
                                            background: step.status === 'success' ? 'rgba(34,197,94,0.15)' : step.status === 'error' ? 'rgba(248,113,113,0.15)' : 'rgba(255,255,255,0.05)',
                                            color: step.status === 'success' ? '#22c55e' : step.status === 'error' ? '#f87171' : 'inherit',
                                            border: `1px solid ${step.status === 'success' ? 'rgba(34,197,94,0.3)' : step.status === 'error' ? 'rgba(248,113,113,0.3)' : 'rgba(255,255,255,0.1)'}`
                                        }}>
                                            {step.name}
                                        </span>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))}
                    <div ref={messagesEndRef} />
                </div>

                {/* Input Area */}
                <div style={{ padding: '16px 24px', borderTop: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)' }}>
                    <div style={{ display: 'flex', gap: '8px', maxWidth: '800px', margin: '0 auto' }}>
                        <textarea
                            ref={inputRef}
                            value={inputValue}
                            onChange={(e) => setInputValue(e.target.value)}
                            onKeyDown={handleKeyDown}
                            placeholder={isThinking ? "Agent is executing..." : "Describe what you want to build, or use /help for commands..."}
                            disabled={isThinking}
                            rows={1}
                            style={{
                                flex: 1,
                                background: 'var(--vscode-sideBar-background)',
                                color: 'inherit',
                                border: '1px solid var(--vscode-panel-border)',
                                borderRadius: '8px',
                                padding: '10px 14px',
                                fontSize: '13px',
                                resize: 'none',
                                outline: 'none',
                                fontFamily: 'inherit',
                                minHeight: '40px',
                                maxHeight: '120px',
                                opacity: isThinking ? 0.5 : 1,
                            }}
                        />
                        <button
                            onClick={handleSend}
                            disabled={isThinking || !inputValue.trim()}
                            style={{
                                background: inputValue.trim() && !isThinking ? 'var(--terminator-accent)' : 'var(--vscode-button-secondaryBackground)',
                                border: 'none',
                                borderRadius: '8px',
                                color: 'white',
                                width: '40px',
                                height: '40px',
                                cursor: inputValue.trim() && !isThinking ? 'pointer' : 'default',
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                                transition: 'all 0.2s',
                            }}
                        >
                            <Send size={16} />
                        </button>
                    </div>
                </div>
            </div>

            {/* Artifact Review & Task View (Right Panel) */}
            <div className="agent-inspector-sidebar" style={{ width: '320px', borderLeft: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)', display: 'flex', flexDirection: 'column' }}>
                <div style={{ padding: '12px 16px', borderBottom: '1px solid var(--vscode-panel-border)', fontWeight: 600, fontSize: '11px', textTransform: 'uppercase', opacity: 0.6 }}>
                    Artifacts & Tasks
                </div>
                <div style={{ flex: 1, overflowY: 'auto', padding: '16px' }}>
                    <div style={{ marginBottom: '24px' }}>
                        <h4 style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '12px', display: 'flex', alignItems: 'center', gap: '6px', opacity: 0.7 }}>
                            <Info size={14} /> Pending Reviews
                        </h4>
                        {activeThread?.artifacts.filter(a => !a.metadata?.reviewed).map(artifact => (
                            <ArtifactCard
                                key={artifact.id}
                                artifact={artifact}
                                onApprove={() => useStore.getState().approveArtifact(activeId, artifact.id)}
                                onReject={() => useStore.getState().rejectArtifact(activeId, artifact.id)}
                            />
                        ))}
                        {(!activeThread || activeThread.artifacts.filter(a => !a.metadata?.reviewed).length === 0) && (
                            <div style={{ padding: '20px', background: 'var(--vscode-sideBar-background)', borderRadius: '6px', border: '1px dashed var(--vscode-panel-border)', fontSize: '11px', opacity: 0.4, textAlign: 'center' }}>
                                No pending artifacts for review.
                            </div>
                        )}
                    </div>

                    <div style={{ marginBottom: '24px' }}>
                        <h4 style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '12px', display: 'flex', alignItems: 'center', gap: '6px', opacity: 0.7 }}>
                            <FileText size={14} /> Mission History
                        </h4>
                        {activeThread?.tasks.map(task => (
                            <div key={task.id} style={{ padding: '10px', background: 'var(--vscode-sideBar-background)', borderRadius: '6px', border: '1px solid var(--vscode-panel-border)', marginBottom: '8px' }}>
                                <div style={{ fontSize: '12px', fontWeight: 500 }}>{task.title}</div>
                                <div style={{ fontSize: '10px', opacity: 0.5, marginTop: '2px' }}>{task.status}</div>
                            </div>
                        ))}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default AgentManager;
