import React, { useState, useRef, useEffect } from 'react';
import { useStore } from '../../store';
import { 
    Bot, FileText, CheckCircle, Clock, Plus, Zap, Shield, Check, X, Info, 
    Send, Activity, Search, Code, ChevronDown, ChevronUp, Terminal, 
    Eye, GitBranch, Globe, Paperclip, Mic, MicOff, Volume2, VolumeX, 
    Sparkles, Cpu, Play, Square, Settings 
} from 'lucide-react';
import type { Artifact } from '../../store';
import { sendAgentMessage, stopAgent } from '../../agent';
import { listen } from '@tauri-apps/api/event';
import * as voice from '../../voice';

// =============================================================================
// Premium Visual Sub-Components
// =============================================================================

const EqualizerBars: React.FC<{ active: boolean }> = ({ active }) => {
    return (
        <div style={{ display: 'flex', gap: '3px', alignItems: 'center', height: '20px', justifyContent: 'center' }}>
            {[1, 2, 3, 4, 5, 6, 7, 8].map((i) => {
                const duration = 0.4 + Math.random() * 0.6;
                return (
                    <div
                        key={i}
                        style={{
                            width: '2px',
                            background: 'linear-gradient(to top, hsl(270, 85%, 60%), hsl(320, 95%, 60%))',
                            borderRadius: '1px',
                            height: active ? '100%' : '3px',
                            animation: active ? `pulseEqualizer ${duration}s ease-in-out infinite alternate` : 'none',
                            transition: 'height 0.2s ease',
                            opacity: active ? 0.95 : 0.3
                        }}
                    />
                );
            })}
        </div>
    );
};

const AgentOrb: React.FC<{ state: 'idle' | 'listening' | 'speaking' | 'thinking' }> = ({ state }) => {
    const getColors = () => {
        switch (state) {
            case 'listening':
                return {
                    bg: 'rgba(16, 185, 129, 0.08)',
                    border: 'rgba(16, 185, 129, 0.4)',
                    glow: 'rgba(16, 185, 129, 0.6)',
                    label: 'VRD LISTENING'
                };
            case 'thinking':
                return {
                    bg: 'rgba(168, 85, 247, 0.08)',
                    border: 'rgba(168, 85, 247, 0.4)',
                    glow: 'rgba(168, 85, 247, 0.6)',
                    label: 'NEURAL INFERENCE'
                };
            case 'speaking':
                return {
                    bg: 'rgba(236, 72, 153, 0.08)',
                    border: 'rgba(236, 72, 153, 0.4)',
                    glow: 'rgba(236, 72, 153, 0.6)',
                    label: 'SPEECH SYNTHESIS'
                };
            default:
                return {
                    bg: 'rgba(139, 92, 246, 0.03)',
                    border: 'rgba(139, 92, 246, 0.2)',
                    glow: 'rgba(139, 92, 246, 0.35)',
                    label: 'AGENT STANDBY'
                };
        }
    };

    const colors = getColors();

    return (
        <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', margin: '24px 0', gap: '14px' }}>
            <div style={{ position: 'relative', width: '100px', height: '100px', display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                {/* Outermost pulsing ring */}
                <div style={{
                    position: 'absolute',
                    width: '100%',
                    height: '100%',
                    borderRadius: '50%',
                    background: colors.bg,
                    border: `1px solid ${colors.border}`,
                    boxShadow: `0 0 20px ${colors.glow}`,
                    animation: state !== 'idle' ? 'orbPulse 1.5s ease-in-out infinite' : 'orbPulseIdle 3s ease-in-out infinite',
                    transition: 'all 0.5s ease'
                }} />
                
                {/* Inner glowing core */}
                <div style={{
                    position: 'absolute',
                    width: '50px',
                    height: '50px',
                    borderRadius: '50%',
                    background: `linear-gradient(135deg, ${colors.border}, ${colors.glow})`,
                    boxShadow: `0 0 14px ${colors.glow}`,
                    animation: state === 'thinking' ? 'orbRotate 2s linear infinite' : 'none',
                    transition: 'all 0.5s ease',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center'
                }}>
                    <Bot size={20} color="#ffffff" style={{ opacity: 0.95 }} />
                </div>
            </div>
            
            <div style={{
                fontSize: '10px',
                fontWeight: 700,
                letterSpacing: '2px',
                color: colors.glow,
                textTransform: 'uppercase',
                transition: 'color 0.5s ease',
                display: 'flex',
                alignItems: 'center',
                gap: '6px'
            }}>
                <span style={{
                    width: '5px',
                    height: '5px',
                    borderRadius: '50%',
                    background: colors.glow,
                    display: 'inline-block',
                    animation: state !== 'idle' ? 'pulse 1s infinite' : 'none'
                }} />
                {colors.label}
            </div>
        </div>
    );
};

// =============================================================================
// Core Components
// =============================================================================

const TaskRoadmap: React.FC = () => {
    const currentPhase = useStore(state => state.currentPhase);
    const status = useStore(state => state.currentPhaseStatus);
    const isThinking = useStore(state => state.isAgentThinking);

    if (currentPhase === 'IDLE' || !isThinking) return null;

    const phases = ['ANALYZE', 'PLAN', 'EXECUTE', 'VERIFY', 'REPORT'];
    const activeIndex = phases.indexOf(currentPhase);

    return (
        <div className="task-roadmap" style={{
            margin: '12px 16px',
            padding: '12px',
            background: 'rgba(var(--terminator-accent-rgb), 0.04)',
            border: '1px solid rgba(var(--terminator-accent-rgb), 0.15)',
            borderRadius: '6px',
            animation: 'fadeIn 0.3s ease-out'
        }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '12px' }}>
                {phases.map((p, i) => (
                    <div key={p} style={{ display: 'flex', flexDirection: 'column', alignItems: 'center', gap: '3px', flex: 1, position: 'relative' }}>
                        <div style={{
                            width: '20px',
                            height: '20px',
                            borderRadius: '50%',
                            background: i <= activeIndex ? 'var(--terminator-accent)' : 'var(--vscode-panel-border)',
                            color: i <= activeIndex ? '#fff' : 'var(--vscode-foreground)',
                            display: 'flex',
                            alignItems: 'center',
                            justifyContent: 'center',
                            fontSize: '9px',
                            fontWeight: 700,
                            zIndex: 2,
                            boxShadow: i === activeIndex ? '0 0 8px var(--terminator-accent)' : 'none'
                        }}>
                            {i < activeIndex ? <Check size={12} /> : i + 1}
                        </div>
                        <span style={{ fontSize: '8px', fontWeight: 600, opacity: i <= activeIndex ? 1 : 0.4 }}>{p}</span>
                        {i < phases.length - 1 && (
                            <div style={{
                                position: 'absolute',
                                left: '50%',
                                top: '10px',
                                width: '100%',
                                height: '2px',
                                background: i < activeIndex ? 'var(--terminator-accent)' : 'var(--vscode-panel-border)',
                                zIndex: 1
                            }} />
                        )}
                    </div>
                ))}
            </div>
            <div style={{ fontSize: '10px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                <Activity size={12} style={{ color: 'var(--terminator-accent)' }} className="animate-pulse" />
                <span style={{ fontWeight: 600 }}>{status}</span>
            </div>
        </div>
    );
};

const ToolStepItem: React.FC<{ step: any }> = ({ step }) => {
    const [isExpanded, setIsExpanded] = useState(false);
    const hasDetails = step.args || step.result;

    const getIcon = () => {
        const name = (step.name || '').toLowerCase();
        if (name.includes('bash') || name.includes('run_command') || name.includes('sh')) return <Terminal size={12} />;
        if (name.includes('view') || name.includes('read')) return <Eye size={12} />;
        if (name.includes('git')) return <GitBranch size={12} />;
        if (name.includes('search') || name.includes('grep')) return <Search size={12} />;
        if (name.includes('browser') || name.includes('web')) return <Globe size={12} />;
        return <Activity size={12} />;
    };

    const getStatusColor = () => {
        if (step.status === 'success') return '#22c55e';
        if (step.status === 'error') return '#ef4444';
        return 'var(--terminator-accent)';
    };

    return (
        <div className={`tool-step-item ${step.status}`} style={{
            margin: '3px 0',
            background: 'rgba(255,255,255,0.02)',
            border: `1px solid ${isExpanded ? 'rgba(255,255,255,0.08)' : 'transparent'}`,
            borderRadius: '4px',
            overflow: 'hidden',
            transition: 'all 0.2s ease',
            fontSize: '11px'
        }}>
            <div
                onClick={() => hasDetails && setIsExpanded(!isExpanded)}
                style={{
                    padding: '5px 8px',
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    cursor: hasDetails ? 'pointer' : 'default',
                    background: isExpanded ? 'rgba(255,255,255,0.04)' : 'transparent',
                    userSelect: 'none'
                }}
            >
                <div style={{ color: getStatusColor(), display: 'flex', alignItems: 'center' }}>
                    {getIcon()}
                </div>
                <div style={{ flex: 1, fontWeight: 500, color: 'var(--vscode-foreground)', opacity: 0.85 }}>
                    {step.summary || `Executing ${step.name}...`}
                </div>
                {step.status === 'running' && (
                    <div className="animate-spin" style={{ width: '10px', height: '10px', border: '1.5px solid var(--terminator-accent)', borderTopColor: 'transparent', borderRadius: '50%' }} />
                )}
                {hasDetails && (
                    <div style={{ opacity: 0.4 }}>
                        {isExpanded ? <ChevronUp size={12} /> : <ChevronDown size={12} />}
                    </div>
                )}
            </div>

            {isExpanded && (
                <div style={{ padding: '0 8px 8px 8px', borderTop: '1px solid rgba(255,255,255,0.04)', background: 'rgba(0,0,0,0.15)' }}>
                    {step.args && (
                        <div style={{ marginTop: '6px' }}>
                            <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', opacity: 0.35, marginBottom: '3px' }}>Arguments</div>
                            <pre style={{ margin: 0, padding: '5px', fontSize: '10px', background: 'rgba(0,0,0,0.25)', borderRadius: '3px', overflowX: 'auto', color: '#6fb3fa' }}>
                                {JSON.stringify(step.args, null, 2)}
                            </pre>
                        </div>
                    )}
                    {step.result && (
                        <div style={{ marginTop: '6px' }}>
                            <div style={{ fontSize: '9px', fontWeight: 700, textTransform: 'uppercase', opacity: 0.35, marginBottom: '3px' }}>Result</div>
                            <pre style={{
                                margin: 0,
                                padding: '5px',
                                fontSize: '10px',
                                background: 'rgba(0,0,0,0.25)',
                                borderRadius: '3px',
                                overflowX: 'auto',
                                maxHeight: '150px',
                                color: (step.status === 'error' ? '#f87171' : '#9ce19c')
                            }}>
                                {typeof step.result === 'string' ? step.result : JSON.stringify(step.result, null, 2)}
                            </pre>
                        </div>
                    )}
                </div>
            )}
        </div>
    );
};

const ArtifactCard: React.FC<{ artifact: Artifact; onApprove: () => void; onReject: () => void }> = ({ artifact, onApprove, onReject }) => {
    return (
        <div className="artifact-card" style={{ marginBottom: '12px' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '6px' }}>
                <span style={{ fontSize: '11px', fontWeight: 600, color: 'var(--terminator-accent)' }}>
                    {artifact.type.toUpperCase()}
                </span>
                <span style={{ fontSize: '9px', opacity: 0.4 }}>
                    {new Date(artifact.timestamp).toLocaleTimeString()}
                </span>
            </div>
            <div style={{ fontSize: '12px', fontWeight: 500, marginBottom: '3px' }}>{artifact.title || 'Untitled Artifact'}</div>
            <div style={{ fontSize: '10px', opacity: 0.65, marginBottom: '10px', lineHeight: 1.35 }}>{artifact.description}</div>

            <div style={{ display: 'flex', gap: '6px' }}>
                <button
                    onClick={onApprove}
                    style={{ flex: 1, padding: '5px', background: 'var(--terminator-success)', border: 'none', borderRadius: '3px', color: 'white', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '3px', fontSize: '10px', fontWeight: 600 }}
                >
                    <Check size={12} /> Approve
                </button>
                <button
                    onClick={onReject}
                    style={{ flex: 1, padding: '5px', background: 'var(--vscode-button-secondaryBackground)', border: 'none', borderRadius: '3px', color: 'white', cursor: 'pointer', display: 'flex', alignItems: 'center', justifyContent: 'center', gap: '3px', fontSize: '10px', fontWeight: 600 }}
                >
                    <X size={12} /> Reject
                </button>
            </div>
        </div>
    );
};

// =============================================================================
// Main AgentManager Component
// =============================================================================

const AgentManager: React.FC = () => {
    const agentThreads = useStore(state => state.agentThreads);
    const activeId = useStore(state => state.activeAgentThreadId);
    const setActiveThread = useStore(state => state.setActiveAgentThread);
    const createThread = useStore(state => state.createAgentThread);
    const agentMessages = useStore(state => state.agentMessages);
    const isThinking = useStore(state => state.isAgentThinking);
    const isPaused = useStore(state => state.isAgentPaused);
    const currentAction = useStore(state => state.agentCurrentAction);

    const [inputValue, setInputValue] = useState('');
    const messagesEndRef = useRef<HTMLDivElement>(null);
    const inputRef = useRef<HTMLTextAreaElement>(null);
    const recognitionRef = useRef<any>(null);

    // Active Interface mode (Standard / Premium)
    const [activeInterface, setActiveInterface] = useState<'standard' | 'premium'>(
        () => (localStorage.getItem('jetski_interface_mode') as 'standard' | 'premium') || 'standard'
    );

    // Voice Synthesis Settings (Muted status, preset persona)
    const [isMuted, setIsMuted] = useState<boolean>(() => {
        return localStorage.getItem('jetski_voice_muted') === 'true';
    });
    const [voicePreset, setVoicePreset] = useState<voice.VoicePreset>(() => {
        return (localStorage.getItem('jetski_voice_preset') as voice.VoicePreset) || 'sage';
    });

    // Voice listening state (Speech Recognition)
    const [isVoiceListening, setIsVoiceListening] = useState(false);
    const [isSpeakingOutLoud, setIsSpeakingOutLoud] = useState(false);

    const toggleInterface = (mode: 'standard' | 'premium') => {
        setActiveInterface(mode);
        localStorage.setItem('jetski_interface_mode', mode);
    };

    // Save states to local storage and bind to global TTS Selection
    useEffect(() => {
        localStorage.setItem('jetski_voice_muted', String(isMuted));
        if (isMuted) {
            voice.stop();
        }
    }, [isMuted]);

    useEffect(() => {
        localStorage.setItem('jetski_voice_preset', voicePreset);
    }, [voicePreset]);

    // Periodically sync "Speaking out loud" indicator from the global speaker
    useEffect(() => {
        const interval = setInterval(() => {
            setIsSpeakingOutLoud(voice.isSpeaking());
        }, 300);

        // Global voice init
        voice.initTTS().catch(err => console.warn('[TTS] Failed to init voice:', err));

        return () => {
            clearInterval(interval);
            voice.stop();
        };
    }, []);

    const threads = Object.values(agentThreads);
    const activeThread = agentThreads[activeId];
    const attachedFiles = useStore(state => state.attachedFiles);
    const attachFile = useStore(state => state.attachFile);
    const removeFile = useStore(state => state.removeFile);
    const clearAttachedFiles = useStore(state => state.clearAttachedFiles);

    const renderContent = (content: string, steps: any[]) => {
        if (!content) return null;
        const jsonBlockRegex = /\{[\s\S]*?"name"[\s\S]*?("arguments"|"function"|"args")[\s\S]*?\}/g;
        let cleanedContent = content;
        const matches = content.match(jsonBlockRegex);
        if (matches) {
            for (const match of matches) {
                try {
                    const parsed = JSON.parse(match);
                    const name = parsed.name || (parsed.function && parsed.function.name);
                    if (name && steps?.find(s => s.name === name)) {
                        cleanedContent = cleanedContent.replace(match, '').trim();
                    }
                } catch (e) {
                    // ignore
                }
            }
        }
        if (!cleanedContent && content) return null;
        return cleanedContent;
    };

    const setPhase = useStore(state => state.setPhase);

    // Auto-scroll to bottom
    useEffect(() => {
        messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
    }, [agentMessages]);

    // Listen for phase updates from Sentient engine
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

    const handleAttachFile = async () => {
        try {
            const { invoke } = await import('@tauri-apps/api/core');
            const result: any = await invoke('select_and_process_attachment');
            attachFile(result);
        } catch (error) {
            console.error('Failed to attach file:', error);
        }
    };

    // Voice dictation interface (Speech Recognition)
    const toggleVoiceInput = () => {
        if (isVoiceListening) {
            stopVoiceInput();
        } else {
            startVoiceInput();
        }
    };

    const startVoiceInput = () => {
        voice.stop(); // Stop agent speaking before user speaks
        const SpeechRecognition = (window as any).SpeechRecognition || (window as any).webkitSpeechRecognition;
        if (!SpeechRecognition) {
            console.warn('[VRD] Web Speech Recognition not supported in this environment.');
            return;
        }

        const recognition = new SpeechRecognition();
        recognition.continuous = false;
        recognition.interimResults = true;
        recognition.lang = 'en-US';

        recognition.onstart = () => {
            setIsVoiceListening(true);
        };

        recognition.onresult = (event: any) => {
            const transcript = Array.from(event.results)
                .map((result: any) => result[0].transcript)
                .join('');
            setInputValue(transcript);
        };

        recognition.onend = () => {
            setIsVoiceListening(false);
            setTimeout(() => {
                const text = inputRef.current?.value || '';
                if (text.trim()) {
                    handleSendWithText(text);
                }
            }, 200);
        };

        recognition.onerror = (e: any) => {
            console.error('[VRD] Recognition error:', e);
            setIsVoiceListening(false);
        };

        recognitionRef.current = recognition;
        recognition.start();
    };

    const stopVoiceInput = () => {
        if (recognitionRef.current) {
            recognitionRef.current.stop();
        }
        setIsVoiceListening(false);
    };

    const handleSend = () => {
        handleSendWithText(inputValue);
    };

    const handleSendWithText = async (textToSend: string) => {
        const prompt = textToSend.trim();
        if (!prompt || isThinking) return;

        setInputValue('');
        clearAttachedFiles();
        voice.stop(); // Clear any active speech and speech queue
        voice.clearTtsQueue();

        // If no active thread, create one
        if (!activeId || !activeThread) {
            const threadName = prompt.slice(0, 40) + (prompt.length > 40 ? '...' : '');
            createThread(threadName);
        }

        // Add user message to global store
        const store = useStore.getState();
        store.addAgentMessage('user', prompt);
        store.addAgentMessage('assistant', '');
        store.setIsAgentThinking(true);

        try {
            let sentenceBuffer = "";
            let spokenLength = 0;

            await sendAgentMessage(prompt, (fullText) => {
                // If not muted, feed text incrementally into real-time speech queue
                if (!isMuted) {
                    const newText = fullText.slice(spokenLength);
                    sentenceBuffer += newText;
                    spokenLength = fullText.length;

                    // Detect complete sentence boundary (. ! ? or newline)
                    const sentences = sentenceBuffer.split(/([.!?\n]+)/);
                    if (sentences.length > 2) {
                        const incomplete = sentences.pop() || "";
                        const completeSentence = sentences.join("").trim();
                        sentenceBuffer = incomplete;

                        if (completeSentence) {
                            voice.queueSpeechChunk(completeSentence, voicePreset);
                        }
                    }
                }
            });

            // Enqueue any remaining trailing text
            if (!isMuted && sentenceBuffer.trim()) {
                voice.queueSpeechChunk(sentenceBuffer.trim(), voicePreset);
            }

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

    const getAgentState = () => {
        if (isVoiceListening) return 'listening';
        if (isThinking) return 'thinking';
        if (isSpeakingOutLoud) return 'speaking';
        return 'idle';
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', flex: 1, height: '100%', overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
            
            {/* Sleek Switcher Header */}
            <div style={{
                display: 'flex',
                justifyContent: 'space-between',
                alignItems: 'center',
                padding: '6px 12px',
                background: 'rgba(25, 25, 30, 0.75)',
                backdropFilter: 'blur(12px)',
                borderBottom: '1px solid rgba(255, 255, 255, 0.08)',
                zIndex: 10,
                gap: '8px',
                userSelect: 'none'
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                    <Bot size={14} color="var(--terminator-accent)" />
                    <span style={{ fontSize: '10px', fontWeight: 700, letterSpacing: '0.5px', textTransform: 'uppercase', color: 'rgba(255, 255, 255, 0.85)' }}>
                        Agent Workspace
                    </span>
                </div>
                
                <div style={{
                    display: 'flex',
                    background: 'rgba(0, 0, 0, 0.3)',
                    borderRadius: '16px',
                    padding: '2px',
                    border: '1px solid rgba(255, 255, 255, 0.05)',
                    position: 'relative'
                }}>
                    <button
                        onClick={() => toggleInterface('standard')}
                        style={{
                            padding: '4px 10px',
                            borderRadius: '12px',
                            border: 'none',
                            background: activeInterface === 'standard' ? 'rgba(255, 255, 255, 0.08)' : 'transparent',
                            color: activeInterface === 'standard' ? 'var(--vscode-foreground)' : 'rgba(255, 255, 255, 0.45)',
                            fontSize: '10px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '4px',
                            transition: 'all 0.2s ease'
                        }}
                    >
                        <Activity size={10} />
                        Sentient Standard
                    </button>
                    <button
                        onClick={() => toggleInterface('premium')}
                        style={{
                            padding: '4px 10px',
                            borderRadius: '12px',
                            border: 'none',
                            background: activeInterface === 'premium' ? 'linear-gradient(135deg, hsl(270, 75%, 45%), hsl(320, 85%, 45%))' : 'transparent',
                            color: activeInterface === 'premium' ? '#ffffff' : 'rgba(255, 255, 255, 0.45)',
                            fontSize: '10px',
                            fontWeight: 600,
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '4px',
                            transition: 'all 0.2s ease',
                            boxShadow: activeInterface === 'premium' ? '0 0 10px rgba(186, 85, 211, 0.45)' : 'none'
                        }}
                    >
                        <Zap size={10} style={{ color: activeInterface === 'premium' ? '#fff' : 'rgba(255, 255, 255, 0.45)' }} />
                        Antigravity Premium
                    </button>
                </div>
            </div>

            {/* Standard Mode View */}
            {activeInterface === 'standard' ? (
                <div className="agent-manager-surface" style={{ display: 'flex', flex: 1, height: '100%', overflow: 'hidden', background: 'var(--vscode-editor-background)' }}>
                    
                    {/* Sidebar Threads */}
                    <div className="agent-threads-sidebar" style={{ width: '220px', borderRight: '1px solid var(--vscode-panel-border)', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ padding: '8px 12px', display: 'flex', justifyContent: 'space-between', alignItems: 'center', borderBottom: '1px solid var(--vscode-panel-border)' }}>
                            <span style={{ fontSize: '9px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.5 }}>Active Agents</span>
                            <button onClick={() => createThread('New Mission')} style={{ background: 'none', border: 'none', color: 'var(--terminator-accent)', cursor: 'pointer' }}><Plus size={14} /></button>
                        </div>
                        <div style={{ flex: 1, overflowY: 'auto' }}>
                            {threads.map(thread => (
                                <div
                                    key={thread.id}
                                    onClick={() => setActiveThread(thread.id)}
                                    style={{
                                        padding: '10px 12px',
                                        borderBottom: '1px solid var(--vscode-panel-border)',
                                        cursor: 'pointer',
                                        background: activeId === thread.id ? 'var(--vscode-list-activeSelectionBackground)' : 'transparent',
                                        borderLeft: activeId === thread.id ? '2px solid var(--terminator-accent)' : 'none'
                                    }}
                                >
                                    <div style={{ fontWeight: 500, fontSize: '11px', whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{thread.name}</div>
                                    <div style={{ fontSize: '9px', opacity: 0.4, marginTop: '3px', display: 'flex', alignItems: 'center', gap: '3px' }}>
                                        {thread.isThinking ? <Clock size={10} className="spinning" /> : <CheckCircle size={10} />}
                                        {thread.messages.length} messages
                                    </div>
                                </div>
                            ))}
                            {threads.length === 0 && (
                                <div style={{ padding: '30px 10px', textAlign: 'center', opacity: 0.25 }}>
                                    <Bot size={36} style={{ display: 'block', margin: '0 auto 8px' }} />
                                    <div style={{ fontSize: '11px' }}>No thread.</div>
                                </div>
                            )}
                        </div>
                    </div>

                    {/* Chat pane */}
                    <div className="agent-conversation-area" style={{ flex: 1, display: 'flex', flexDirection: 'column', background: 'var(--vscode-sideBar-background)', position: 'relative' }}>
                        
                        {/* Thread Header */}
                        <div style={{ padding: '12px 16px', borderBottom: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                            <h2 style={{ margin: 0, fontSize: '14px', display: 'flex', alignItems: 'center', gap: '6px' }}>
                                <Bot size={16} color="var(--terminator-accent)" />
                                {activeThread ? activeThread.name : 'Antigravity Agent'}
                            </h2>
                            
                            <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                <span style={{ fontSize: '10px', opacity: 0.5 }}>Speech Synthesizer:</span>
                                <button
                                    onClick={() => setIsMuted(!isMuted)}
                                    style={{
                                        background: isMuted ? 'rgba(255,255,255,0.05)' : 'rgba(186, 85, 211, 0.15)',
                                        border: '1px solid rgba(186, 85, 211, 0.3)',
                                        borderRadius: '4px',
                                        padding: '3px 6px',
                                        color: isMuted ? 'rgba(255,255,255,0.4)' : '#d946ef',
                                        cursor: 'pointer',
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: '4px',
                                        fontSize: '9px',
                                        fontWeight: 600
                                    }}
                                >
                                    {isMuted ? <VolumeX size={10} /> : <Volume2 size={10} />}
                                    {isMuted ? 'MUTED' : 'VOCAL'}
                                </button>
                            </div>
                        </div>

                        {/* Sentient Roadmap UI */}
                        <TaskRoadmap />

                        {/* Conversation messages */}
                        <div style={{ flex: 1, overflowY: 'auto', padding: '16px' }}>
                            {agentMessages.length === 0 && (
                                <div style={{ textAlign: 'center', opacity: 0.15, marginTop: '60px' }}>
                                    <Zap size={48} style={{ marginBottom: '12px' }} />
                                    <h4>What do you want to build?</h4>
                                    <p style={{ fontSize: '11px' }}>Speak your instructions or type below</p>
                                </div>
                            )}
                            {agentMessages.map((m: any, i: number) => (
                                <div key={i} style={{ marginBottom: '16px', maxWidth: '800px', margin: '0 auto 16px' }}>
                                    <div style={{ fontWeight: 600, marginBottom: '4px', fontSize: '10px', color: m.role === 'user' ? 'var(--terminator-accent)' : 'var(--terminator-success)' }}>
                                        {m.role === 'user' ? 'YOU' : 'ANTIGRAVITY'}
                                        {isThinking && i === agentMessages.length - 1 && m.role === 'assistant' && (
                                            <Clock size={10} className="spinning" style={{ marginLeft: '4px' }} />
                                        )}
                                    </div>
                                    {m.context && m.context.length > 0 && (
                                        <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', marginBottom: '6px', opacity: 0.7 }}>
                                            {m.context.map((item: any, ci: number) => (
                                                <div key={ci} style={{
                                                    display: 'flex', alignItems: 'center', gap: '3px', padding: '1px 6px',
                                                    background: 'rgba(var(--terminator-accent-rgb), 0.08)', borderRadius: '3px', fontSize: '9px',
                                                    border: '1px solid rgba(var(--terminator-accent-rgb), 0.15)', color: 'var(--vscode-foreground)'
                                                }}>
                                                    <FileText size={10} />
                                                    <span>{item.name}</span>
                                                </div>
                                            ))}
                                        </div>
                                    )}
                                    {(renderContent(m.content, m.steps) || (isThinking && i === agentMessages.length - 1 && m.role === 'assistant')) && (
                                        <div className={`agent-bubble ${m.role}`} style={{
                                            lineHeight: 1.5,
                                            whiteSpace: 'pre-wrap',
                                            fontSize: '11.5px',
                                            background: m.role === 'user' ? 'rgba(var(--terminator-accent-rgb), 0.03)' : 'rgba(255,255,255,0.01)',
                                            padding: '10px',
                                            borderRadius: '6px',
                                            border: `1px solid ${m.role === 'user' ? 'rgba(var(--terminator-accent-rgb), 0.08)' : 'rgba(255,255,255,0.04)'}`
                                        }}>
                                            {renderContent(m.content, m.steps) || (
                                                <div style={{ display: 'flex', alignItems: 'center', gap: '6px', opacity: 0.4 }}>
                                                    <div className="animate-spin" style={{ width: '8px', height: '8px', border: '1.5px solid var(--terminator-accent)', borderTopColor: 'transparent', borderRadius: '50%' }} />
                                                    Thinking...
                                                </div>
                                            )}
                                        </div>
                                    )}
                                    {m.steps && m.steps.length > 0 && (
                                        <div style={{ marginTop: '6px', display: 'flex', flexDirection: 'column', gap: '3px' }}>
                                            {m.steps.map((step: any, si: number) => (
                                                <ToolStepItem key={si} step={step} />
                                            ))}
                                        </div>
                                    )}
                                </div>
                            ))}
                            <div ref={messagesEndRef} />
                        </div>

                        {/* File upload chips */}
                        {attachedFiles.length > 0 && (
                            <div style={{ padding: '6px 16px', display: 'flex', gap: '6px', flexWrap: 'wrap', background: 'rgba(var(--terminator-accent-rgb), 0.03)', borderTop: '1px solid var(--vscode-panel-border)' }}>
                                {attachedFiles.map(file => (
                                    <div key={file.path} style={{
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: '4px',
                                        padding: '3px 6px',
                                        background: 'var(--vscode-badge-background)',
                                        color: 'var(--vscode-badge-foreground)',
                                        borderRadius: '3px',
                                        fontSize: '9px',
                                        border: '1px solid rgba(var(--terminator-accent-rgb), 0.15)'
                                    }}>
                                        <FileText size={10} />
                                        <span style={{ maxWidth: '120px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{file.name}</span>
                                        <button
                                            onClick={() => removeFile(file.path)}
                                            style={{ background: 'none', border: 'none', color: 'inherit', cursor: 'pointer', padding: 0, display: 'flex', alignItems: 'center', opacity: 0.5 }}
                                        >
                                            <X size={10} />
                                        </button>
                                    </div>
                                ))}
                            </div>
                        )}

                        {/* Input row */}
                        <div style={{ padding: '12px 16px', borderTop: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)' }}>
                            <div style={{ display: 'flex', gap: '6px', maxWidth: '800px', margin: '0 auto' }}>
                                <textarea
                                    ref={inputRef}
                                    value={inputValue}
                                    onChange={(e) => setInputValue(e.target.value)}
                                    onKeyDown={handleKeyDown}
                                    placeholder={
                                        isVoiceListening 
                                            ? "VRD listening... speak clearly..." 
                                            : isThinking 
                                                ? `Agent: ${currentAction || 'Thinking...'}` 
                                                : isPaused 
                                                    ? "Agent paused. Add instructions or type 'continue'..." 
                                                    : "Type instruction or click mic to talk..."
                                    }
                                    disabled={isThinking && !isPaused}
                                    rows={1}
                                    style={{
                                        flex: 1,
                                        background: isVoiceListening ? 'rgba(16, 185, 129, 0.08)' : 'var(--vscode-sideBar-background)',
                                        color: 'inherit',
                                        border: isVoiceListening ? '1px solid rgba(16, 185, 129, 0.3)' : '1px solid var(--vscode-panel-border)',
                                        borderRadius: '6px',
                                        padding: '8px 12px',
                                        fontSize: '12px',
                                        resize: 'none',
                                        outline: 'none',
                                        fontFamily: 'inherit',
                                        minHeight: '32px',
                                        maxHeight: '100px',
                                        opacity: (isThinking && !isPaused) ? 0.5 : 1,
                                        transition: 'all 0.2s ease'
                                    }}
                                />
                                
                                {/* Micro Dictation Trigger */}
                                <button
                                    onClick={toggleVoiceInput}
                                    disabled={isThinking && !isPaused}
                                    style={{
                                        background: isVoiceListening ? 'rgba(16, 185, 129, 0.2)' : 'var(--vscode-button-secondaryBackground)',
                                        border: isVoiceListening ? '1px solid rgba(16, 185, 129, 0.5)' : 'none',
                                        borderRadius: '6px',
                                        color: isVoiceListening ? '#10b981' : 'white',
                                        width: '32px',
                                        height: '32px',
                                        cursor: 'pointer',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                        transition: 'all 0.2s'
                                    }}
                                    title={isVoiceListening ? "Stop Listening" : "VRD Voice Control"}
                                >
                                    {isVoiceListening ? <Mic size={14} className="animate-pulse" /> : <MicOff size={14} />}
                                </button>

                                <button
                                    onClick={handleAttachFile}
                                    disabled={isThinking}
                                    title="Attach File"
                                    style={{
                                        background: 'var(--vscode-button-secondaryBackground)',
                                        border: 'none',
                                        borderRadius: '6px',
                                        color: 'white',
                                        width: '32px',
                                        height: '32px',
                                        cursor: isThinking ? 'default' : 'pointer',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                        opacity: isThinking ? 0.5 : 1
                                    }}
                                >
                                    <Paperclip size={14} />
                                </button>
                                
                                <button
                                    onClick={handleSend}
                                    disabled={isThinking || !inputValue.trim()}
                                    style={{
                                        background: inputValue.trim() && !isThinking ? 'var(--terminator-accent)' : 'var(--vscode-button-secondaryBackground)',
                                        border: 'none',
                                        borderRadius: '6px',
                                        color: 'white',
                                        width: '32px',
                                        height: '32px',
                                        cursor: inputValue.trim() && !isThinking ? 'pointer' : 'default',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                    }}
                                >
                                    <Send size={14} />
                                </button>
                                
                                {isThinking && (
                                    <button
                                        onClick={stopAgent}
                                        style={{
                                            background: 'var(--vscode-errorForeground, #f48771)',
                                            border: 'none',
                                            borderRadius: '6px',
                                            color: 'white',
                                            width: '32px',
                                            height: '32px',
                                            cursor: 'pointer',
                                            display: 'flex',
                                            alignItems: 'center',
                                            justifyContent: 'center',
                                        }}
                                        title="Stop Agent"
                                    >
                                        <X size={16} />
                                    </button>
                                )}
                            </div>
                        </div>
                    </div>

                    {/* Right pane review */}
                    <div className="agent-inspector-sidebar" style={{ width: '260px', borderLeft: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-editor-background)', display: 'flex', flexDirection: 'column' }}>
                        <div style={{ padding: '8px 12px', borderBottom: '1px solid var(--vscode-panel-border)', fontWeight: 600, fontSize: '9px', textTransform: 'uppercase', opacity: 0.5 }}>
                            Artifacts & Reviews
                        </div>
                        <div style={{ flex: 1, overflowY: 'auto', padding: '12px' }}>
                            <div style={{ marginBottom: '16px' }}>
                                <h4 style={{ fontSize: '9px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '4px', opacity: 0.6 }}>
                                    <Info size={12} /> Pending reviews
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
                                    <div style={{ padding: '16px', background: 'var(--vscode-sideBar-background)', borderRadius: '4px', border: '1px dashed var(--vscode-panel-border)', fontSize: '10px', opacity: 0.35, textAlign: 'center' }}>
                                        No pending reviews.
                                    </div>
                                )}
                            </div>

                            <div style={{ marginBottom: '16px' }}>
                                <h4 style={{ fontSize: '9px', fontWeight: 600, textTransform: 'uppercase', marginBottom: '8px', display: 'flex', alignItems: 'center', gap: '4px', opacity: 0.6 }}>
                                    <FileText size={12} /> History
                                </h4>
                                {activeThread?.tasks.map(task => (
                                    <div key={task.id} style={{ padding: '8px', background: 'var(--vscode-sideBar-background)', borderRadius: '4px', border: '1px solid var(--vscode-panel-border)', marginBottom: '6px' }}>
                                        <div style={{ fontSize: '11px', fontWeight: 500 }}>{task.title}</div>
                                        <div style={{ fontSize: '9px', opacity: 0.4, marginTop: '2px' }}>{task.status}</div>
                                    </div>
                                ))}
                            </div>
                        </div>
                    </div>

                </div>
            ) : (
                
                // =============================================================================
                // Premium Native Mode Dashboard
                // =============================================================================
                <div style={{
                    flex: 1,
                    display: 'flex',
                    flexDirection: 'column',
                    height: '100%',
                    overflow: 'hidden',
                    background: 'radial-gradient(circle at top, #181024 0%, #0d0a14 100%)',
                    fontFamily: 'system-ui, -apple-system, sans-serif'
                }}>
                    
                    {/* VRD Glowing Control Panel */}
                    <div style={{
                        padding: '12px 16px',
                        background: 'rgba(20, 16, 28, 0.4)',
                        backdropFilter: 'blur(16px)',
                        borderBottom: '1px solid rgba(139, 92, 246, 0.15)',
                        display: 'flex',
                        flexWrap: 'wrap',
                        justifyContent: 'space-between',
                        alignItems: 'center',
                        gap: '12px'
                    }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '12px' }}>
                            {/* Voice Synthesizer Toggle */}
                            <button
                                onClick={() => setIsMuted(!isMuted)}
                                style={{
                                    background: isMuted ? 'rgba(255, 255, 255, 0.03)' : 'rgba(236, 72, 153, 0.1)',
                                    border: `1px solid ${isMuted ? 'rgba(255,255,255,0.08)' : 'rgba(236, 72, 153, 0.3)'}`,
                                    borderRadius: '20px',
                                    padding: '5px 14px',
                                    color: isMuted ? 'rgba(255, 255, 255, 0.5)' : '#ec4899',
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '6px',
                                    fontSize: '10px',
                                    fontWeight: 700,
                                    letterSpacing: '1px',
                                    transition: 'all 0.3s ease',
                                    boxShadow: isMuted ? 'none' : '0 0 10px rgba(236, 72, 153, 0.2)'
                                }}
                            >
                                {isMuted ? <VolumeX size={12} /> : <Volume2 size={12} />}
                                {isMuted ? 'AUTO-SPEECH OFF' : 'SPEECH SYNTHESIS'}
                            </button>

                            {/* Voice Preset Selection Dropdown */}
                            <div style={{ display: 'flex', alignItems: 'center', gap: '6px' }}>
                                <span style={{ fontSize: '10px', color: 'rgba(255, 255, 255, 0.4)', fontWeight: 600 }}>Persona:</span>
                                <select
                                    value={voicePreset}
                                    onChange={(e) => setVoicePreset(e.target.value as voice.VoicePreset)}
                                    style={{
                                        background: 'rgba(20, 16, 28, 0.7)',
                                        color: '#b55fe6',
                                        border: '1px solid rgba(139, 92, 246, 0.25)',
                                        borderRadius: '12px',
                                        padding: '4px 10px',
                                        fontSize: '10px',
                                        fontWeight: 600,
                                        outline: 'none',
                                        cursor: 'pointer',
                                        transition: 'all 0.2s ease'
                                    }}
                                >
                                    <option value="sage">Sage (Mature Calm)</option>
                                    <option value="airi">Airi (Anime Girl)</option>
                                    <option value="nova">Nova (Youthful)</option>
                                    <option value="zero">Zero (Deep Command)</option>
                                    <option value="yamato">Yamato (Male Japanese)</option>
                                    <option value="hana">Hana (Gentle female)</option>
                                    <option value="ren">Ren (Male Professional)</option>
                                    <option value="filipino">Filipino (Native Accent)</option>
                                </select>
                            </div>
                        </div>

                        {/* Real-time frequency equalizer */}
                        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                            <span style={{ fontSize: '9px', color: 'rgba(255,255,255,0.3)', fontWeight: 600, letterSpacing: '1.5px' }}>VRD STATE:</span>
                            <EqualizerBars active={isSpeakingOutLoud || isVoiceListening} />
                        </div>
                    </div>

                    {/* Central matrix display */}
                    <div style={{
                        flex: 1,
                        display: 'flex',
                        flexDirection: 'column',
                        overflowY: 'auto',
                        padding: '24px 20px',
                        alignItems: 'center',
                        justifyContent: 'flex-start'
                    }}>
                        
                        {/* Interactive Agent Mind Orb */}
                        <AgentOrb state={getAgentState()} />

                        {/* VRD Speech Live Feedback Card */}
                        {isVoiceListening && (
                            <div style={{
                                width: '100%',
                                maxWidth: '500px',
                                background: 'rgba(16, 185, 129, 0.05)',
                                border: '1px dashed rgba(16, 185, 129, 0.25)',
                                borderRadius: '12px',
                                padding: '14px',
                                color: '#10b981',
                                fontSize: '11px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '6px',
                                marginBottom: '16px',
                                backdropFilter: 'blur(8px)',
                                animation: 'fadeIn 0.3s ease-out'
                            }}>
                                <div style={{ fontWeight: 700, fontSize: '9px', letterSpacing: '1px', textTransform: 'uppercase', display: 'flex', alignItems: 'center', gap: '6px' }}>
                                    <Mic size={10} className="animate-pulse" /> Live Speech Capture
                                </div>
                                <div style={{ fontStyle: 'italic', opacity: 0.85 }}>
                                    "{inputValue || 'Listening to your command...'}"
                                </div>
                            </div>
                        )}

                        {/* High-Tech System Parameters Overlay */}
                        <div style={{
                            display: 'grid',
                            gridTemplateColumns: 'repeat(auto-fit, minmax(130px, 1fr))',
                            gap: '10px',
                            width: '100%',
                            maxWidth: '500px',
                            marginBottom: '20px'
                        }}>
                            <div style={{
                                background: 'rgba(255, 255, 255, 0.02)',
                                border: '1px solid rgba(139, 92, 246, 0.1)',
                                padding: '10px',
                                borderRadius: '8px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '3px'
                            }}>
                                <div style={{ fontSize: '8px', opacity: 0.4, fontWeight: 700, letterSpacing: '1px' }}>COGNITIVE INTERFACE</div>
                                <div style={{ color: '#c084fc', fontWeight: 600, fontSize: '11px', display: 'flex', alignItems: 'center', gap: '4px' }}>
                                    <Cpu size={10} /> VRD Active Drive
                                </div>
                            </div>
                            <div style={{
                                background: 'rgba(255, 255, 255, 0.02)',
                                border: '1px solid rgba(139, 92, 246, 0.1)',
                                padding: '10px',
                                borderRadius: '8px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '3px'
                            }}>
                                <div style={{ fontSize: '8px', opacity: 0.4, fontWeight: 700, letterSpacing: '1px' }}>SPEECH RATE</div>
                                <div style={{ color: '#d946ef', fontWeight: 600, fontSize: '11px' }}>
                                    {isMuted ? 'N/A' : 'Real-time (Multilingual)'}
                                </div>
                            </div>
                            <div style={{
                                background: 'rgba(255, 255, 255, 0.02)',
                                border: '1px solid rgba(139, 92, 246, 0.1)',
                                padding: '10px',
                                borderRadius: '8px',
                                display: 'flex',
                                flexDirection: 'column',
                                gap: '3px'
                            }}>
                                <div style={{ fontSize: '8px', opacity: 0.4, fontWeight: 700, letterSpacing: '1px' }}>SYSTEM THROTTLE</div>
                                <div style={{ color: '#3b82f6', fontWeight: 600, fontSize: '11px' }}>
                                    {isThinking ? 'Processing...' : '0ms Latency'}
                                </div>
                            </div>
                        </div>

                        {/* Sentient Roadmap UI */}
                        <div style={{ width: '100%', maxWidth: '500px' }}>
                            <TaskRoadmap />
                        </div>

                        {/* Hacker Chat History Window */}
                        <div style={{
                            width: '100%',
                            maxWidth: '500px',
                            display: 'flex',
                            flexDirection: 'column',
                            gap: '12px',
                            marginTop: '10px'
                        }}>
                            {agentMessages.slice(-5).map((m: any, i: number) => (
                                <div
                                    key={i}
                                    style={{
                                        background: m.role === 'user' ? 'rgba(139, 92, 246, 0.05)' : 'rgba(25, 20, 35, 0.5)',
                                        border: `1px solid ${m.role === 'user' ? 'rgba(139, 92, 246, 0.2)' : 'rgba(139, 92, 246, 0.08)'}`,
                                        borderRadius: '8px',
                                        padding: '12px',
                                        animation: 'fadeIn 0.3s ease-out',
                                        boxShadow: '0 4px 12px rgba(0, 0, 0, 0.15)'
                                    }}
                                >
                                    <div style={{
                                        fontWeight: 700,
                                        fontSize: '9px',
                                        letterSpacing: '1px',
                                        color: m.role === 'user' ? '#a78bfa' : '#f472b6',
                                        marginBottom: '6px'
                                    }}>
                                        {m.role === 'user' ? 'NEURAL LINK PROMPT' : 'AGENT CONSCIOUSNESS'}
                                    </div>
                                    <div style={{
                                        fontSize: '11.5px',
                                        color: 'rgba(255, 255, 255, 0.85)',
                                        lineHeight: 1.5,
                                        whiteSpace: 'pre-wrap'
                                    }}>
                                        {renderContent(m.content, m.steps) || (
                                            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', opacity: 0.4 }}>
                                                <div className="animate-spin" style={{ width: '8px', height: '8px', border: '1.5px solid var(--terminator-accent)', borderTopColor: 'transparent', borderRadius: '50%' }} />
                                                Inference Stream...
                                            </div>
                                        )}
                                    </div>
                                    {m.steps && m.steps.length > 0 && (
                                        <div style={{ marginTop: '8px', display: 'flex', flexDirection: 'column', gap: '4px' }}>
                                            {m.steps.slice(-3).map((step: any, si: number) => (
                                                <ToolStepItem key={si} step={step} />
                                            ))}
                                        </div>
                                    )}
                                </div>
                            ))}
                        </div>
                    </div>

                    {/* Bottom voice control & dictation panel */}
                    <div style={{
                        padding: '16px 20px',
                        background: 'rgba(20, 16, 28, 0.6)',
                        borderTop: '1px solid rgba(139, 92, 246, 0.15)',
                        backdropFilter: 'blur(16px)'
                    }}>
                        <div style={{ display: 'flex', gap: '8px', maxWidth: '500px', margin: '0 auto', alignItems: 'center' }}>
                            
                            {/* Big Futuristic Neon Microphone Button */}
                            <button
                                onClick={toggleVoiceInput}
                                disabled={isThinking && !isPaused}
                                style={{
                                    background: isVoiceListening 
                                        ? 'linear-gradient(135deg, #10b981, #059669)'
                                        : 'linear-gradient(135deg, hsl(270, 70%, 55%), hsl(320, 80%, 55%))',
                                    border: 'none',
                                    borderRadius: '50%',
                                    color: 'white',
                                    width: '46px',
                                    height: '46px',
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    transition: 'all 0.3s cubic-bezier(0.4, 0, 0.2, 1)',
                                    boxShadow: isVoiceListening 
                                        ? '0 0 16px rgba(16, 185, 129, 0.6)' 
                                        : '0 0 16px rgba(186, 85, 211, 0.4)',
                                    flexShrink: 0
                                }}
                                title={isVoiceListening ? "Stop voice listening" : "Talk via VRD Voice Link"}
                            >
                                {isVoiceListening ? <Mic size={20} className="animate-pulse" /> : <MicOff size={20} />}
                            </button>

                            {/* Supplementary Textarea */}
                            <textarea
                                ref={inputRef}
                                value={inputValue}
                                onChange={(e) => setInputValue(e.target.value)}
                                onKeyDown={handleKeyDown}
                                placeholder={
                                    isVoiceListening 
                                        ? "VRD listening... Speak clearly..." 
                                        : isThinking 
                                            ? "Agent is running mission..." 
                                            : "Type instruction or speak to agent..."
                                }
                                disabled={isThinking && !isPaused}
                                rows={1}
                                style={{
                                    flex: 1,
                                    background: 'rgba(0, 0, 0, 0.4)',
                                    color: '#ffffff',
                                    border: isVoiceListening ? '1px solid #10b981' : '1px solid rgba(139, 92, 246, 0.25)',
                                    borderRadius: '20px',
                                    padding: '10px 16px',
                                    fontSize: '12px',
                                    resize: 'none',
                                    outline: 'none',
                                    fontFamily: 'inherit',
                                    minHeight: '36px',
                                    maxHeight: '80px',
                                    transition: 'all 0.2s ease'
                                }}
                            />

                            {/* Direct Trigger */}
                            <button
                                onClick={handleSend}
                                disabled={isThinking || !inputValue.trim()}
                                style={{
                                    background: inputValue.trim() && !isThinking 
                                        ? 'linear-gradient(135deg, hsl(270, 75%, 45%), hsl(320, 85%, 45%))' 
                                        : 'rgba(255, 255, 255, 0.05)',
                                    border: 'none',
                                    borderRadius: '50%',
                                    color: 'white',
                                    width: '36px',
                                    height: '36px',
                                    cursor: inputValue.trim() && !isThinking ? 'pointer' : 'default',
                                    display: 'flex',
                                    alignItems: 'center',
                                    justifyContent: 'center',
                                    transition: 'all 0.2s',
                                    flexShrink: 0
                                }}
                            >
                                <Send size={14} />
                            </button>

                            {isThinking && (
                                <button
                                    onClick={stopAgent}
                                    style={{
                                        background: 'rgba(239, 68, 68, 0.2)',
                                        border: '1px solid rgba(239, 68, 68, 0.4)',
                                        borderRadius: '50%',
                                        color: '#ef4444',
                                        width: '36px',
                                        height: '36px',
                                        cursor: 'pointer',
                                        display: 'flex',
                                        alignItems: 'center',
                                        justifyContent: 'center',
                                        flexShrink: 0
                                    }}
                                    title="Stop Agent"
                                >
                                    <X size={16} />
                                </button>
                            )}
                        </div>
                    </div>
                </div>
            )}

            {/* Embedded styles for beautiful animations */}
            <style>{`
                @keyframes pulseEqualizer {
                    0% { transform: scaleY(0.15); }
                    100% { transform: scaleY(1.0); }
                }
                @keyframes orbPulse {
                    0% { transform: scale(0.96); box-shadow: 0 0 16px var(--glow); opacity: 0.8; }
                    50% { transform: scale(1.06); box-shadow: 0 0 28px var(--glow); opacity: 1; }
                    100% { transform: scale(0.96); box-shadow: 0 0 16px var(--glow); opacity: 0.8; }
                }
                @keyframes orbPulseIdle {
                    0% { transform: scale(0.98); opacity: 0.45; }
                    50% { transform: scale(1.02); opacity: 0.65; }
                    100% { transform: scale(0.98); opacity: 0.45; }
                }
                @keyframes orbRotate {
                    0% { transform: rotate(0deg); }
                    100% { transform: rotate(360deg); }
                }
                @keyframes fadeIn {
                    from { opacity: 0; transform: translateY(4px); }
                    to { opacity: 1; transform: translateY(0); }
                }
            `}</style>
        </div>
    );
};

export default AgentManager;
