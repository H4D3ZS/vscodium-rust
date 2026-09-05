/**
 * AiriOverlay — Ambient sentient presence that floats over the editor.
 * Reacts to IDE events, shows live thoughts, and proactively suggests actions.
 * Not a chat window — AIRI is watching, thinking, and acting in real-time.
 */
import React, { useEffect, useRef, useState, useCallback } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useStore } from '../store';

interface Thought {
    id: string;
    text: string;
    type: 'suggestion' | 'action' | 'warning' | 'success';
    action?: () => void;
    actionLabel?: string;
    ttl?: number; // ms before auto-dismiss
}

interface LiveAction {
    id: string;
    tool: string;
    label: string;
    status: 'running' | 'done' | 'error';
    detail?: string;
}

const TOOL_ICONS: Record<string, string> = {
    write_to_file: '',
    search_replace_edit: '',
    apply_shadow_patch: '',
    view_file: '',
    run_command: '',
    verify_implementation: '',
    ghost_test: '',
    list_files: '',
    grep: '',
    git_commit: '',
    git_add: '',
    save_knowledge_brief: '',
    create_mission_plan: '',
};

const TOOL_LABELS: Record<string, string> = {
    write_to_file: 'Writing file',
    search_replace_edit: 'Patching code',
    apply_shadow_patch: 'Committing edit',
    view_file: 'Reading file',
    run_command: 'Running command',
    verify_implementation: 'Verifying build',
    ghost_test: 'Running tests',
    list_files: 'Scanning directory',
    grep: 'Searching codebase',
    git_commit: 'Committing changes',
    git_add: 'Staging files',
    save_knowledge_brief: 'Storing memory',
    create_mission_plan: 'Planning mission',
};

export const AiriOverlay: React.FC = () => {
    const [thoughts, setThoughts] = useState<Thought[]>([]);
    const [liveActions, setLiveActions] = useState<LiveAction[]>([]);
    const [isExpanded, setIsExpanded] = useState(false);
    const [verity, setVerity] = useState(1.0);
    const [mood, setMood] = useState<'idle' | 'thinking' | 'coding' | 'success' | 'error'>('idle');
    const [pulseCount, setPulseCount] = useState(0);
    const [thinkingText, setThinkingText] = useState('');
    const isAgentThinking = useStore(s => s.isAgentThinking);
    const activeFilePath = useStore(s => s.activeEditorPath);
    const isYoloMode = useStore(s => s.isYoloMode);
    const thoughtTimers = useRef<Map<string, ReturnType<typeof setTimeout>>>(new Map());
    const wasThinking = useRef(false);
    const clearActionsTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

    const addThought = useCallback((t: Thought) => {
        setThoughts(prev => [t, ...prev].slice(0, 4));
        if (t.ttl) {
            const timer = setTimeout(() => {
                setThoughts(prev => prev.filter(x => x.id !== t.id));
                thoughtTimers.current.delete(t.id);
            }, t.ttl);
            thoughtTimers.current.set(t.id, timer);
        }
    }, []);

    const dismissThought = useCallback((id: string) => {
        setThoughts(prev => prev.filter(t => t.id !== id));
        const timer = thoughtTimers.current.get(id);
        if (timer) { clearTimeout(timer); thoughtTimers.current.delete(id); }
    }, []);

    useEffect(() => {
        const subs: (() => void)[] = [];
        // Listen to Rust AIRI events (new unified event bus)
        listen<any>('airi:vision_frame', (e) => {
            const { analysis } = e.payload;
            if (analysis?.code?.errors?.length) {
                setMood('error');
                addThought({
                    id: `vision-err-${Date.now()}`,
                    text: ` Vision: ${analysis.code.errors[0].substring(0, 60)}`,
                    type: 'warning',
                    ttl: 4000
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('airi:phase_wrap', (e) => {
            const reports = e.payload?.reports || [];
            if (reports.length > 0) {
                addThought({
                    id: `phase-${Date.now()}`,
                    text: ` Phase-Wrap: ${reports[0].summary?.substring(0, 60) || 'Reflection complete'}`,
                    type: 'action',
                    ttl: 3000
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('airi:edit_proposed', (e) => {
            const { file, description } = e.payload;
            addThought({
                id: `edit-prop-${Date.now()}`,
                text: ` Edit proposed: ${description?.substring(0, 50)}`,
                type: 'suggestion',
                ttl: 5000
            });
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('airi:edit_committed', (e) => {
            const { file, success } = e.payload;
            if (success) {
                setMood('success');
                addThought({
                    id: `edit-ok-${Date.now()}`,
                    text: ` Edit applied to ${file?.split('/').pop()}`,
                    type: 'success',
                    ttl: 3000
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('airi:error_detected', (e) => {
            const { errors } = e.payload;
            if (errors?.length) {
                setMood('error');
                addThought({
                    id: `err-${Date.now()}`,
                    text: ` Detected: ${errors[0].substring(0, 60)}`,
                    type: 'warning',
                    ttl: 5000,
                    actionLabel: 'Fix',
                    action: () => {
                        // Trigger self-healing
                    }
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('airi:thought', (e) => {
            // Dedupe frequent thoughts — only show significant ones
            const t = e.payload;
            if (t?.type === 'observation' && t.content?.includes(' Vision:')) {
                // Low-priority visual observation, don't clutter overlay
                return;
            }
            addThought({
                id: `thought-${Date.now()}`,
                text: t.content?.substring(0, 80) || 'Thinking...',
                type: t.type === 'warning'? 'warning': 'action',
                ttl: 4000
            });
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('ai-tool-result', (e) => {
            const name = e.payload?.name;
            setLiveActions(prev => prev.map(a =>
                a.tool === name && a.status === 'running'
? { ...a, status: 'done' }
: a
            ));
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('hades://verity', (e) => {
            const score = e.payload?.score ?? 1.0;
            setVerity(score);
            if (score < 1.0) {
                setMood('error');
                addThought({
                    id: `verity-${Date.now()}`,
                    text: `Compiler found issues — I'll self-correct`,
                    type: 'warning',
                    ttl: 4000
                });
            } else {
                setMood(prev => prev === 'coding'? 'success': prev);
                setTimeout(() => setMood('idle'), 2000);
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('ai-content', (e) => {
            const content: string = e.payload?.content || '';
            if (content.includes('MISSION_ACCOMPLISHED')) {
                setMood('success');
                addThought({
                    id: `done-${Date.now()}`,
                    text: 'Mission accomplished ',
                    type: 'success',
                    ttl: 5000
                });
                setTimeout(() => {
                    setMood('idle');
                    setLiveActions([]);
                }, 3000);
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('task-phase-update', (e) => {
            const phase = e.payload?.phase;
            const status = e.payload?.status;
            if (phase && status) {
                addThought({
                    id: `phase-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
                    text: status,
                    type: 'action',
                    ttl: 3000
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('memory-update', (e) => {
            const slots = e.payload?.slots ?? 0;
            if (slots > 0 && slots % 5 === 0) {
                // Date.now() ticks once per ms, but two memory-update events
                // can fire in the same millisecond. Append a random suffix so
                // React doesn't see duplicate keys.
                addThought({
                    id: `mem-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
                    text: `Kortex brain: ${slots} knowledge slots stored`,
                    type: 'action',
                    ttl: 3000
                });
            }
        }).then(u => subs.push(u)).catch(() => { });

        // Capture streaming text for the thought bubble
        let streamBuf = '';
        listen<any>('ai-stream', (e) => {
            const chunk: string = e.payload?.chunk || e.payload?.content || '';
            if (chunk) {
                streamBuf += chunk;
                // Show last 90 chars — strip tool XML noise
                const cleaned = streamBuf
                    .replace(/<tool_call>[\s\S]*?<\/tool_call>/g, '')
                    .replace(/<function_calls>[\s\S]*?<\/function_calls>/g, '')
                    .replace(/```json[\s\S]*?```/g, '')
                    .replace(/\n{3,}/g, '\n')
                    .trim();
                setThinkingText(cleaned.slice(-90));
            }
        }).then(u => subs.push(u)).catch(() => { });

        listen<any>('ai-content', () => {
            streamBuf = '';
            setThinkingText('');
        }).then(u => subs.push(u)).catch(() => { });

        return () => subs.forEach(u => u());
    }, [addThought]);

    // React to isAgentThinking changes — auto-clear liveActions on completion
    useEffect(() => {
        if (isAgentThinking) {
            wasThinking.current = true;
            if (clearActionsTimer.current) { clearTimeout(clearActionsTimer.current); clearActionsTimer.current = null; }
            setMood('thinking');
        } else if (wasThinking.current) {
            wasThinking.current = false;
            setThinkingText('');
            // Mark all running actions as done, then fade out after 2.5s
            setLiveActions(prev => prev.map(a => a.status === 'running'? { ...a, status: 'done' as const }: a));
            clearActionsTimer.current = setTimeout(() => {
                setLiveActions([]);
                setMood('idle');
                clearActionsTimer.current = null;
            }, 2500);
        }
    }, [isAgentThinking]);

    // Proactive file suggestion
    useEffect(() => {
        if (!activeFilePath || isAgentThinking) return;
        const ext = activeFilePath.split('.').pop()?.toLowerCase();
        if (ext === 'rs') {
            const timer = setTimeout(() => {
                addThought({
                    id: `suggest-rs-${Date.now()}`,
                    text: `Open Rust file detected. Want me to check for improvements?`,
                    type: 'suggestion',
                    actionLabel: 'Analyze',
                    ttl: 8000,
                    action: () => {
                        const input = document.querySelector<HTMLTextAreaElement>('.agent-mission-input');
                        if (input) {
                            input.value = `Review ${activeFilePath} for issues, improvements, and missing error handling.`;
                            input.dispatchEvent(new Event('input', { bubbles: true }));
                        }
                    }
                });
            }, 5000);
            return () => clearTimeout(timer);
        }
    }, [activeFilePath]);

    const moodColor = {
        idle: 'rgba(255,255,255,0.2)',
        thinking: '#a855f7',
        coding: '#7c3aed',
        success: '#10b981',
        error: '#ef4444',
    }[mood];

    const moodGlow = {
        idle: 'none',
        thinking: '0 0 12px rgba(168, 85, 247, 0.4)',
        coding: '0 0 12px rgba(124, 58, 237, 0.5)',
        success: '0 0 16px rgba(16, 185, 129, 0.5)',
        error: '0 0 12px rgba(239, 68, 68, 0.4)',
    }[mood];

    if (!isAgentThinking && liveActions.length === 0 && thoughts.length === 0 && mood === 'idle') {
        return null; // Completely invisible when truly idle
    }

    return (
        <div style={{
            position: 'fixed',
            bottom: '48px',
            right: isExpanded? '320px': '12px',
            zIndex: 9000,
            display: 'flex',
            flexDirection: 'column',
            alignItems: 'flex-end',
            gap: '8px',
            pointerEvents: 'none',
            transition: 'right 0.3s ease'
        }}>
            {/* ── Anime Thought Bubble — shows what AIRI is currently thinking ── */}
            {(mood === 'thinking' || mood === 'coding') && thinkingText && (
                <div style={{
                    pointerEvents: 'none',
                    display: 'flex',
                    flexDirection: 'column',
                    alignItems: 'center',
                    animation: 'thoughtCloudIn 0.3s ease-out',
                }}>
                    {/* Cloud body */}
                    <div className="airi-thought-cloud">
                        <span style={{
                            display: 'block',
                            fontSize: '10px',
                            color: '#0a0618',
                            lineHeight: 1.45,
                            textAlign: 'center',
                            maxWidth: '180px',
                            wordBreak: 'break-word'
                        }}>
                            {thinkingText || '...'}
                        </span>
                    </div>
                    {/* Trail dots connecting cloud down to the orb */}
                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '3px', marginTop: '5px' }}>
                        <div style={{ width: '8px', height: '8px', borderRadius: '50%', background: 'rgba(210,235,255,0.82)' }} />
                        <div style={{ width: '6px', height: '6px', borderRadius: '50%', background: 'rgba(210,235,255,0.68)' }} />
                        <div style={{ width: '4px', height: '4px', borderRadius: '50%', background: 'rgba(210,235,255,0.5)' }} />
                    </div>
                </div>
            )}

            {/* Thought bubbles */}
            {thoughts.map(thought => (
                <div key={thought.id} style={{
                    pointerEvents: 'auto',
                    background: thought.type === 'success'? 'rgba(16,185,129,0.12)'
: thought.type === 'warning'? 'rgba(239,68,68,0.12)'
: thought.type === 'suggestion'? 'rgba(59,130,246,0.12)'
: 'rgba(30,30,40,0.85)',
                    border: `1px solid ${thought.type === 'success'? 'rgba(16,185,129,0.3)'
: thought.type === 'warning'? 'rgba(239,68,68,0.3)'
: thought.type === 'suggestion'? 'rgba(59,130,246,0.3)'
: 'rgba(255,255,255,0.08)'}`,
                    borderRadius: '10px',
                    padding: '8px 12px',
                    maxWidth: '260px',
                    fontSize: '11px',
                    color: 'rgba(255,255,255,0.85)',
                    backdropFilter: 'blur(12px)',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '8px',
                    boxShadow: '0 4px 16px rgba(0,0,0,0.3)',
                    animation: 'airiThoughtIn 0.2s ease-out',
                }}>
                    <span style={{ flex: 1 }}>{thought.text}</span>
                    {thought.actionLabel && (
                        <button onClick={thought.action} style={{
                            background: 'rgba(168, 85, 247, 0.3)', border: '1px solid rgba(168, 85, 247, 0.5)',
                            color: '#e9d5ff', fontSize: '10px', padding: '2px 8px',
                            borderRadius: '5px', cursor: 'pointer', whiteSpace: 'nowrap', fontWeight: 600
                        }}>{thought.actionLabel}</button>
                    )}
                    <span onClick={() => dismissThought(thought.id)}
                        style={{ cursor: 'pointer', opacity: 0.4, fontSize: '10px' }}></span>
                </div>
            ))}

            {/* Thought bubbles and suggestions handled here */}

            <style>{`
                @keyframes airiPulse {
                    0%, 100% { opacity: 1; transform: scale(1); }
                    50% { opacity: 0.7; transform: scale(1.08); }
                }
                @keyframes airiThoughtIn {
                    from { opacity: 0; transform: translateY(8px) scale(0.95); }
                    to { opacity: 1; transform: translateY(0) scale(1); }
                }
                @keyframes thoughtCloudIn {
                    from { opacity: 0; transform: scale(0.75) translateY(10px); }
                    to   { opacity: 1; transform: scale(1) translateY(0); }
                }
                @keyframes thoughtFloat {
                    0%, 100% { transform: translateY(0); }
                    50%      { transform: translateY(-5px); }
                }
                /* Anime-style cloud thought bubble */
                .airi-thought-cloud {
                    background: rgba(232, 245, 255, 0.94);
                    border: 1.5px solid rgba(180, 215, 255, 0.65);
                    border-radius: 20px;
                    padding: 10px 14px;
                    max-width: 200px;
                    min-width: 100px;
                    position: relative;
                    box-shadow: 0 3px 16px rgba(0,0,0,0.22),
                                inset 0 1px 0 rgba(255,255,255,0.88);
                    animation: thoughtFloat 2.8s ease-in-out infinite;
                }
                /* Cloud bump — left lobe */
                .airi-thought-cloud::before {
                    content: '';
                    position: absolute;
                    width: 38px; height: 26px;
                    background: rgba(232, 245, 255, 0.94);
                    border: 1.5px solid rgba(180, 215, 255, 0.65);
                    border-radius: 50%;
                    top: -14px;
                    left: 14%;
                }
                /* Cloud bump — right lobe */
                .airi-thought-cloud::after {
                    content: '';
                    position: absolute;
                    width: 30px; height: 22px;
                    background: rgba(232, 245, 255, 0.94);
                    border: 1.5px solid rgba(180, 215, 255, 0.65);
                    border-radius: 50%;
                    top: -12px;
                    right: 18%;
                }
            `}</style>
        </div>
    );
};
