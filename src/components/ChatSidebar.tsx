import React, { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useStore } from '../store';
import { speak, stop, isSpeaking, initTTS, getProvider, type VoicePreset } from '../voice';
import { voiceActivation } from '../audio/voice-activation';

// VRM 3D avatar removed. No-op shim keeps the panel's lifecycle calls working;
// `initialize` rejects so the panel falls back to the lightweight orb.
const airiVRMAvatar = {
    initialize: (_el?: unknown, _url?: unknown) => Promise.reject(new Error('VRM avatar removed')),
    dispose: () => {},
    onResize: () => {},
    setSpeaking: (_v?: unknown) => {},
    setThinking: (_v?: unknown) => {},
    setListening: (_v?: unknown) => {},
    setEmotion: (_v?: unknown) => {},
};

interface ChatSidebarProps {
    className?: string;
    style?: React.CSSProperties;
    scale?: number;
    yOffset?: string;
    transparent?: boolean;
    character?: string; // Selected avatar character ID
}

// ── TTS Engine — ElevenLabs / OpenAI / Browser (via voice.ts) ───────────────

let _isInitialized = false;

function ttsOptedIn(): boolean {
    try { return localStorage.getItem('tts.enabled') === '1'; } catch { return false; }
}

async function ensureTtsInit() {
    if (_isInitialized || !ttsOptedIn()) return;
    _isInitialized = await initTTS();
}

// Split text into natural sentences for smooth streaming speech
function splitSentences(text: string): string[] {
    return text
        .replace(/([.!?])\s+/g, '$1\n')
        .split('\n')
        .map(s => s.trim())
        .filter(s => s.length > 3);
}

let _ttsSpeaking = false;
let _ttsPreset: VoicePreset = 'airi';

async function ttsSpeak(text: string) {
    if (!text.trim()) return;

    // Strip markdown for cleaner speech synthesis
    const clean = text
        .replace(/```[\s\S]*?```/g, ' code block. ')
        .replace(/`[^`]+`/g, '')
        .replace(/#{1,6}\s/g, '')
        .replace(/[*_>[\]]/g, '')
        .replace(/https?:\/\/\S+/g, 'link')
        .replace(/\s+/g, ' ')
        .trim();

    if (!clean) return;

    ensureTtsInit().then(() => {
        speak(clean, 'airi');
    }).catch(err => console.error('[ChatSidebar] TTS speak failed:', err));
}

function ttsStop() {
    stop();
    _ttsSpeaking = false;
}

// ── ANSI color strip ─────────────────────────────────────────────────────────
function stripAnsi(str: string) {
    return str.replace(/\x1B\[[0-9;]*[mGKHF]/g, '');
}

// ── Activity State Machine ───────────────────────────────────────────────────
type AiriActivity =
    | 'idle'
    | 'thinking'
    | 'coding'       // write_to_file / search_replace_edit
    | 'reading'      // view_file / grep / search_codebase
    | 'executing'    // run_command / ghost_test
    | 'browsing'     // browser_*
    | 'committing'   // git_commit
    | 'patching'     // apply_shadow_patch
    | 'success'
    | 'error';

const ACTIVITY_META: Record<AiriActivity, { color: string; glow: string; label: string; emoji: string }> = {
    idle: { color: '#818cf8', glow: 'rgba(129,140,248,0.3)', label: 'Idle', emoji: '' },
    thinking: { color: '#c084fc', glow: 'rgba(192,132,252,0.4)', label: 'Thinking', emoji: '◎' },
    coding: { color: '#34d399', glow: 'rgba(52,211,153,0.4)', label: 'Coding', emoji: '⌨' },
    reading: { color: '#60a5fa', glow: 'rgba(96,165,250,0.35)', label: 'Reading', emoji: '' },
    executing: { color: '#f59e0b', glow: 'rgba(245,158,11,0.4)', label: 'Executing', emoji: '' },
    browsing: { color: '#38bdf8', glow: 'rgba(56,189,248,0.35)', label: 'Browsing', emoji: '' },
    committing: { color: '#a3e635', glow: 'rgba(163,230,53,0.35)', label: 'Committing', emoji: '' },
    patching: { color: '#fb923c', glow: 'rgba(251,146,60,0.35)', label: 'Patching', emoji: '' },
    success: { color: '#10b981', glow: 'rgba(16,185,129,0.4)', label: 'Done', emoji: '' },
    error: { color: '#ef4444', glow: 'rgba(239,68,68,0.4)', label: 'Error', emoji: '' },
};

const TOOL_TO_ACTIVITY: Record<string, AiriActivity> = {
    write_to_file: 'coding', search_replace_edit: 'coding', patch_file_content: 'patching',
    apply_shadow_patch: 'patching', view_file: 'reading', grep: 'reading',
    search_codebase: 'reading', find_symbols: 'reading', list_files: 'reading',
    run_command: 'executing', ghost_test: 'executing', dev_cargo_diagnostics: 'executing',
    git_commit: 'committing', git_push: 'committing',
    browser_open: 'browsing', browser_navigate: 'browsing', browser_screenshot: 'browsing',
};

// ── Typewriter hook ───────────────────────────────────────────────────────────
function useTypewriter(text: string, speed = 18): string {
    const [displayed, setDisplayed] = useState('');
    const prevRef = useRef('');
    useEffect(() => {
        if (text === prevRef.current) return;
        // If new text extends old, continue from where we stopped
        const base = text.startsWith(prevRef.current)? prevRef.current: '';
        const remaining = text.slice(base.length);
        prevRef.current = text;
        if (!remaining) return;
        let i = 0;
        const iv = setInterval(() => {
            i++;
            setDisplayed(base + remaining.slice(0, i));
            if (i >= remaining.length) clearInterval(iv);
        }, speed);
        return () => clearInterval(iv);
    }, [text, speed]);
    return displayed;
}

export const ChatSidebar: React.FC<ChatSidebarProps> = ({ className, style, scale, yOffset, transparent, character = 'airi' }) => {
    const vrmContainerRef = useRef<HTMLDivElement>(null);
    const avatar3dConfig = useStore(state => state.avatar3dConfig);
    const showVrmAvatar = useStore(state => state.showVrmAvatar);
    const [isAiriLoading, setAiriLoading] = useState(true);
    const [vrmFailed, setVrmFailed] = useState(false);
    const [isHibernating, setIsHibernating] = useState(false);
    const [lastActivityTime, setLastActivityTime] = useState(Date.now());
    // TTS is opt-in — silent unless the user turns it on. Persisted, and shared
    // with voice.ts's speak()/initTTS() gate via the 'tts.enabled' key.
    const [isTtsEnabled, setTtsEnabledState] = useState(() => {
        try { return localStorage.getItem('tts.enabled') === '1'; } catch { return false; }
    });
    const setTtsEnabled = useCallback((v: boolean) => {
        try { localStorage.setItem('tts.enabled', v ? '1' : '0'); } catch { /* quota */ }
        setTtsEnabledState(v);
    }, []);
    const [isListening, setIsListening] = useState(false);

    const IDLE_TIMEOUT = 60000; // 1 minute

    // Use 3D model from config if available
    const selectedCharacter = avatar3dConfig?.modelId || character;
    const selectedModelUrl = avatar3dConfig?.modelUrl;

    // Wake up AIRI from hibernation
    const wakeUp = useCallback(() => {
        setLastActivityTime(Date.now());
        if (isHibernating) {
            setIsHibernating(false);
            setAiriLoading(true);
        }
    }, [isHibernating]);

    // ── Live agent state from store ──────────────────────────────────────────
    const isAgentThinking = useStore(s => s.isAgentThinking);
    const agentMessages = useStore(s => s.agentMessages);

    // Track activity and manage hibernation.
    const lastActivityRef = useRef(lastActivityTime);
    useEffect(() => {
        lastActivityRef.current = lastActivityTime;
    }, [lastActivityTime]);

    useEffect(() => {
        if (isAgentThinking) {
            wakeUp();
            return;
        }
        const iv = setInterval(() => {
            if (Date.now() - lastActivityRef.current > IDLE_TIMEOUT && !isHibernating) {
                setIsHibernating(true);
            }
        }, 10000);
        return () => clearInterval(iv);
    }, [isAgentThinking, isHibernating]);

    const uiStatus = useStore(s => s.aiStatus);

    // ── Initialize Native VRM Avatar ─────────────────────────────────────────
    useEffect(() => {
        if (isHibernating) return;

        if (!showVrmAvatar) {
            setAiriLoading(false);
            setVrmFailed(true);
            return;
        }

        let active = true;
        if (vrmContainerRef.current) {
            setAiriLoading(true);
            setVrmFailed(false);
            const modelUrl = selectedModelUrl || '/models/airi.vrm';
            airiVRMAvatar.initialize(vrmContainerRef.current, modelUrl)
                .then((success) => {
                    if (active) {
                        setAiriLoading(false);
                        if (!success) setVrmFailed(true);
                    }
                })
                .catch(_err => {
                    if (active) {
                        setAiriLoading(false);
                        setVrmFailed(true);
                    }
                });
        }

        // Setup ResizeObserver to adapt to side-panel sizing
        let resizeObserver: ResizeObserver | null = null;
        if (vrmContainerRef.current) {
            resizeObserver = new ResizeObserver(() => {
                airiVRMAvatar.onResize();
            });
            resizeObserver.observe(vrmContainerRef.current);
        }

        return () => {
            active = false;
            if (resizeObserver) {
                resizeObserver.disconnect();
            }
            airiVRMAvatar.dispose();
        };
    }, [selectedModelUrl, isHibernating, showVrmAvatar]);

    // ── Lip Sync Integration ──────────────────────────────────────────────
    useEffect(() => {
        const handleLipSyncStart = (e: any) => {
            airiVRMAvatar.setSpeaking(true);
        };

        const handleLipSyncStop = () => {
            airiVRMAvatar.setSpeaking(false);
        };

        const handleModelChange = (e: any) => {
            if (vrmContainerRef.current) {
                setAiriLoading(true);
                airiVRMAvatar.initialize(vrmContainerRef.current, e.detail.modelUrl)
                    .then(() => setAiriLoading(false))
                    .catch(err => {
                        console.error('[ChatSidebar] Model change failed:', err);
                        setAiriLoading(false);
                    });
            }
        };

        window.addEventListener('airi-lipsync-start', handleLipSyncStart as any);
        window.addEventListener('airi-lipsync-stop', handleLipSyncStop);
        window.addEventListener('airi-vrm-model-change', handleModelChange);

        return () => {
            window.removeEventListener('airi-lipsync-start', handleLipSyncStart as any);
            window.removeEventListener('airi-lipsync-stop', handleLipSyncStop);
            window.removeEventListener('airi-vrm-model-change', handleModelChange);
        };
    }, []);

    // ── Live tool-call tracking ──────────────────────────────────────────────
    const [currentTool, setCurrentTool] = useState<string | null>(null);
    const [currentFile, setCurrentFile] = useState<string | null>(null);
    const [currentCommand, setCurrentCommand] = useState<string | null>(null);
    const [activity, setActivity] = useState<AiriActivity>('idle');
    const [thoughtText, setThoughtText] = useState('');
    const lastSpokenIndexRef = useRef(0);
    const prevThinkingRef = useRef(false);
    const [completedCount, setCompletedCount] = useState(0);
    const [errorText, setErrorText] = useState<string | null>(null);

    // ── Derive activity from agent state ─────────────────────────────────────
    useEffect(() => {
        airiVRMAvatar.setThinking(isAgentThinking);

        if (isAgentThinking !== prevThinkingRef.current) {
            if (isAgentThinking) {
                lastSpokenIndexRef.current = 0;
                setThoughtText(''); // Clear previous turn's text
                ttsStop();  // Kill any lingering speech
            }
            prevThinkingRef.current = isAgentThinking;
        }

        if (!isAgentThinking) {
            if (uiStatus === 'dead') {
                setActivity('error');
            } else if (completedCount > 0) {
                setActivity('success');
                const t = setTimeout(() => setActivity('idle'), 3500);
                return () => clearTimeout(t);
            } else {
                setActivity('idle');
            }
            setCurrentTool(null);
            setCurrentFile(null);
            setCurrentCommand(null);
        } else {
            if (activity === 'idle' || activity === 'success') {
                setActivity('thinking');
            }
        }
    }, [isAgentThinking, uiStatus]);

    useEffect(() => {
        const last = agentMessages[agentMessages.length - 1];
        if (last?.role === 'assistant' && typeof last.content === 'string') {
            const fullContent = last.content;

            // UI Thought Bubble (Typewriter)
            const cleanUI = fullContent
                .replace(/```[\s\S]*?```/g, '')
                .replace(/[*_#>[\]]/g, '')
                .replace(/\s+/g, ' ')
                .trim();
            if (cleanUI.length > 5) {
                setThoughtText(cleanUI.slice(-500));
            }

            // High-quality Voice (Incremental)
            if (isTtsEnabled && fullContent.length > lastSpokenIndexRef.current) {
                const newPart = fullContent.slice(lastSpokenIndexRef.current);
                ttsSpeak(newPart);
                lastSpokenIndexRef.current = fullContent.length;
            }
        }
    }, [agentMessages, isTtsEnabled]);

    // ── Listen for tool events ────────────────────────────────────────────────
    useEffect(() => {
        const subs: (() => void)[] = [];
        let isValid = true;

        async function setupSubscriptions() {
            const { listen: listenEv } = await import('@tauri-apps/api/event');
            if (!isValid) return;

            const u1 = await listenEv<any>('ai-tool-call', (e) => {
                const name = e.payload?.name || '';
                const args = e.payload?.args || {};
                const act = TOOL_TO_ACTIVITY[name] || 'thinking';
                setActivity(act);
                setCurrentTool(name);
                setErrorText(null);
                wakeUp(); // Interaction wake

                // Extract meaningful context from args
                const file = args.path || args.file_path || args.uri || args.target_file || null;
                const cmd = args.command || args.cmd || null;
                if (file) setCurrentFile(stripAnsi(typeof file === 'string'? file.split(/[\\/]/).slice(-2).join('/'): ''));
                if (cmd) setCurrentCommand(stripAnsi(typeof cmd === 'string'? cmd.slice(0, 60): ''));

                // Inject a live status into thought
                const label = ACTIVITY_META[act]?.label || name;
                if (file) setThoughtText(`${label}: ${file.split(/[\\/]/).slice(-1)[0]}`);
                else if (cmd) setThoughtText(`${label}: ${cmd.slice(0, 60)}`);
                else setThoughtText(`${label}...`);

                // Set avatar emotion directly
                airiVRMAvatar.setEmotion(act === 'coding'? 'focused': 'thinking');
            });
            subs.push(u1);

            const u2 = await listenEv<any>('ai-tool-result', (e) => {
                const name = e.payload?.name || '';
                const act = TOOL_TO_ACTIVITY[name] || 'thinking';
                if (act !== 'thinking') {
                    setCompletedCount(c => c + 1);
                }
                setActivity(isAgentThinking? 'thinking': 'success');
                wakeUp(); // Interaction wake
            });
            subs.push(u2);
        }

        setupSubscriptions();

        // ── Inbound from Voice (Transcription/Input) ──────────────────────
        const handleMessage = (e: MessageEvent) => {
            if (e.data?.type === 'airi-transcription') {
                const { text, isFinal } = e.data.payload || {};
                if (text && isFinal) {
                    window.dispatchEvent(new CustomEvent('airi-voice-mission', { detail: { text } }));
                }
            }
            if (e.data?.type === 'airi-hearing-state') {
                setIsListening(!!e.data.payload?.enabled);
                airiVRMAvatar.setListening(!!e.data.payload?.enabled);
            }
        };
        window.addEventListener('message', handleMessage);

        return () => {
            isValid = false;
            subs.forEach(u => u());
            window.removeEventListener('message', handleMessage);
        };
    }, [isAgentThinking, uiStatus, wakeUp]);

    // ── Mouse look-at ────────────────────────────────────────────────────────
    useEffect(() => {
        const handleMouseMove = (e: MouseEvent) => {
            wakeUp();
        };
        window.addEventListener('mousemove', handleMouseMove);
        return () => window.removeEventListener('mousemove', handleMouseMove);
    }, []);

    // ── TTS stop when agent stops ─────────────────────────────────────────────
    useEffect(() => {
        if (!isAgentThinking) ttsStop();
    }, [isAgentThinking]);

    const meta = ACTIVITY_META[activity];
    const isSmall = (style?.width as number || 400) < 200;

    return (
        <div
            className={`airi-panel ${className || ''}`}
            style={{ width: '100%', height: '100%', position: 'relative', background: 'transparent', border: 'none', ...style }}
        >
            {/* ── VRM Manifold Canvas ────────────────────────────────────── */}
            {isHibernating? (
                <div
                    onClick={wakeUp}
                    style={{
                        position: 'absolute', inset: 0, display: 'flex', flexDirection: 'column',
                        alignItems: 'center', justifyContent: 'center', cursor: 'pointer',
                        background: 'rgba(0,0,0,0.2)', backdropFilter: 'blur(10px)',
                        borderRadius: '12px', border: '1px dashed rgba(255,255,255,0.1)'
                    }}
                >
                    <div style={{ fontSize: '32px', marginBottom: '8px', opacity: 0.6 }}></div>
                    <div style={{ color: meta.color, fontSize: '10px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '1px' }}>
                        Eco Mode Active
                    </div>
                    <div style={{ color: 'rgba(255,255,255,0.4)', fontSize: '9px', marginTop: '4px' }}>
                        Click to wake AIRI
                    </div>
                </div>
            ): vrmFailed? (
                /* ── Glowing orb fallback when VRM model isn't available ────── */
                <div style={{
                    position: 'absolute', inset: 0,
                    display: 'flex', flexDirection: 'column',
                    alignItems: 'center', justifyContent: 'flex-start',
                    paddingTop: '16px',
                    pointerEvents: 'none',
                }}>
                    <div style={{ position: 'relative', width: '88px', height: '88px' }}>
                        {/* Outer glow ring */}
                        <div style={{
                            position: 'absolute', inset: '-12px',
                            borderRadius: '50%',
                            background: `radial-gradient(circle, ${meta.glow} 0%, transparent 70%)`,
                            animation: activity === 'thinking'? 'airiGlowPulse 1.4s ease-in-out infinite': 'none',
                        }} />
                        {/* Core orb */}
                        <div style={{
                            width: '88px', height: '88px', borderRadius: '50%',
                            background: `radial-gradient(circle at 35% 35%, ${meta.color}cc, ${meta.color}44 60%, transparent)`,
                            border: `1.5px solid ${meta.color}66`,
                            boxShadow: `0 0 24px 6px ${meta.glow}, inset 0 0 16px ${meta.color}33`,
                            display: 'flex', alignItems: 'center', justifyContent: 'center',
                            transition: 'all 0.5s ease',
                        }}>
                            <span style={{ fontSize: '28px', filter: 'drop-shadow(0 0 8px currentColor)', color: meta.color }}>
                                {meta.emoji}
                            </span>
                        </div>
                        {/* Small inner sparkle */}
                        <div style={{
                            position: 'absolute', top: '14px', left: '18px',
                            width: '14px', height: '14px', borderRadius: '50%',
                            background: 'rgba(255,255,255,0.55)',
                            filter: 'blur(3px)',
                        }} />
                    </div>
                    {/* AIRI label under orb */}
                    <div style={{
                        marginTop: '10px', fontSize: '9px', fontWeight: 700,
                        letterSpacing: '2px', textTransform: 'uppercase',
                        color: meta.color, opacity: 0.7,
                    }}>AIRI</div>
                </div>
            ): (
                <>
                    {isAiriLoading && (
                        <div style={{
                            position: 'absolute', inset: 0, display: 'flex', alignItems: 'center', justifyContent: 'center',
                            color: meta.color, fontSize: '9px', textTransform: 'uppercase', letterSpacing: '1px', opacity: 0.5
                        }}>
                            {isSmall? '...': 'Syncing Manifold...'}
                        </div>
                    )}
                    <div
                        ref={vrmContainerRef}
                        style={{
                            width: '100%',
                            height: '100%',
                            opacity: isAiriLoading? 0: 1,
                            background: 'transparent',
                            transition: 'opacity 0.3s ease',
                            transform: `scale(${scale ?? 1}) translateY(${yOffset ?? '0px'})`,
                            transformOrigin: 'top center'
                        }}
                    />
                </>
            )}

            {/* ── Activity Glow Ring (behind avatar) ─────────────────────── */}
            {!isSmall && (
                <div style={{
                    position: 'absolute', bottom: '18%', left: '50%', transform: 'translateX(-50%)',
                    width: '90px', height: '90px', borderRadius: '50%',
                    boxShadow: `0 0 40px 20px ${meta.glow}`,
                    background: 'transparent', pointerEvents: 'none',
                    transition: 'box-shadow 0.8s ease',
                    animation: activity === 'thinking'? 'airiGlowPulse 1.4s ease-in-out infinite': 'none',
                }} />
            )}

            {/* ── Computer-Use Overlay: current tool activity ─────────────── */}
            {!isSmall && (isAgentThinking || activity === 'success') && (currentFile || currentCommand || currentTool) && (
                <div style={{
                    position: 'absolute', top: '8px', left: '8px', right: '8px',
                    background: 'rgba(0,0,0,0.72)', backdropFilter: 'blur(12px)',
                    border: `1px solid ${meta.color}44`,
                    borderRadius: '10px', padding: '8px 10px',
                    pointerEvents: 'none',
                    animation: 'airiSlideDown 0.25s ease-out',
                }}>
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: currentFile || currentCommand? '4px': 0 }}>
                        <span style={{ fontSize: '13px' }}>{meta.emoji}</span>
                        <span style={{ fontSize: '10px', fontWeight: 700, color: meta.color, textTransform: 'uppercase', letterSpacing: '0.08em' }}>
                            {meta.label}
                        </span>
                        <span style={{
                            marginLeft: 'auto', fontSize: '8px', fontWeight: 700, padding: '1px 5px', borderRadius: '4px',
                            background: `${meta.color}22`, color: meta.color, textTransform: 'uppercase'
                        }}>{currentTool?.replace(/_/g, ' ')}</span>
                    </div>
                    {currentFile && (
                        <div style={{
                            fontSize: '10px', color: 'rgba(255,255,255,0.75)',
                            fontFamily: 'var(--font-mono)', overflow: 'hidden',
                            textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                            padding: '2px 4px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px',
                            borderLeft: `2px solid ${meta.color}88`,
                        }}>
                            {currentFile}
                        </div>
                    )}
                    {currentCommand && (
                        <div style={{
                            fontSize: '10px', color: '#a3e635',
                            fontFamily: 'var(--font-mono)', overflow: 'hidden',
                            textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                            padding: '2px 4px', background: 'rgba(0,0,0,0.3)', borderRadius: '4px',
                            borderLeft: '2px solid #a3e63588',
                        }}>
                            $ {currentCommand}
                        </div>
                    )}
                </div>
            )}

            {/* ── Thought Bubble ──────────────────────────────────────────── */}
            {!isSmall && isAgentThinking && thoughtText && (
                <div style={{
                    position: 'absolute', bottom: '48%', left: '50%', transform: 'translateX(-50%)',
                    maxWidth: '88%', pointerEvents: 'none',
                    animation: 'airiSlideUp 0.3s ease-out',
                }}>
                    <div style={{
                        background: 'rgba(230,240,255,0.92)', backdropFilter: 'blur(6px)',
                        borderRadius: '14px 14px 14px 4px',
                        padding: '8px 12px',
                        boxShadow: '0 4px 24px rgba(0,0,0,0.4)',
                        minWidth: '140px',
                        minHeight: '48px',
                        display: 'flex',
                        alignItems: 'center',
                        justifyContent: 'center',
                    }}>
                        <div style={{
                            fontSize: '11px', color: '#0d0921', lineHeight: 1.5,
                            maxWidth: '200px', minHeight: '32px', wordBreak: 'break-word',
                        }}>
                            {thoughtText}
                        </div>
                    </div>
                    <div style={{ display: 'flex', flexDirection: 'column', alignItems: 'flex-start', gap: '2px', marginTop: '4px', paddingLeft: '12px' }}>
                        <div style={{ width: '7px', height: '7px', borderRadius: '50%', background: 'rgba(220,230,255,0.75)' }} />
                        <div style={{ width: '4px', height: '4px', borderRadius: '50%', background: 'rgba(220,230,255,0.5)' }} />
                    </div>
                </div>
            )}

            {/* ── Status Pill (bottom) ─────────────────────────────────────── */}
            {!isSmall && (
                <div style={{
                    position: 'absolute', bottom: '8px', left: '50%', transform: 'translateX(-50%)',
                    display: 'flex', alignItems: 'center', gap: '8px',
                    background: 'rgba(13, 9, 33, 0.85)', backdropFilter: 'blur(12px)',
                    border: `1px solid ${meta.color}66`,
                    borderRadius: '24px', padding: '5px 14px',
                    pointerEvents: 'auto',
                    boxShadow: `0 4px 12px rgba(0,0,0,0.4), 0 0 10px ${meta.glow}`,
                    transition: 'all 0.5s cubic-bezier(0.4, 0, 0.2, 1)',
                }}>
                    {activity !== 'idle' && (
                        <div style={{
                            width: '12px', height: '12px', borderRadius: '50%',
                            border: `2px solid ${meta.color}33`,
                            borderTopColor: meta.color,
                            animation: 'airiSpinner 0.8s linear infinite',
                            flexShrink: 0
                        }} />
                    )}
                    <span style={{
                        fontSize: '10px', fontWeight: 800, color: 'var(--vscode-editor-foreground, #fff)',
                        textTransform: 'uppercase', letterSpacing: '0.1em',
                        whiteSpace: 'nowrap', textShadow: `0 0 8px ${meta.color}`
                    }}>
                        {meta.label}
                    </span>

                    <div style={{ width: '1px', height: '10px', background: 'rgba(255,255,255,0.1)', margin: '0 4px' }} />

                    {/* TTS toggle */}
                    <div
                        onClick={() => {
                            const newState = !isTtsEnabled;
                            setTtsEnabled(newState);
                            if (!newState) ttsStop();
                        }}
                        style={{
                            marginLeft: '4px', cursor: 'pointer', fontSize: '10px',
                            opacity: isTtsEnabled? 1: 0.35,
                            color: isTtsEnabled? '#c084fc': 'rgba(255,255,255,0.5)',
                            display: 'flex', alignItems: 'center'
                        }}
                        title={isTtsEnabled? 'Mute AIRI voice': 'Enable AIRI voice'}
                    >
                        {isTtsEnabled? '': ''}
                    </div>

                    {/* Microphone toggle - Voice Activation */}
                    <div
                        onClick={async () => {
                            const newState = !isListening;
                            setIsListening(newState);

                            // Use voice activation system
                            if (newState) {
                                await voiceActivation.startConversation();
                            } else {
                                voiceActivation.stopListening();
                            }

                            airiVRMAvatar.setListening(newState);
                        }}
                        style={{
                            marginLeft: '8px', cursor: 'pointer', fontSize: '10px',
                            opacity: isListening? 1: 0.35,
                            color: isListening? '#f97316': 'rgba(255,255,255,0.5)',
                            display: 'flex', alignItems: 'center'
                        }}
                        title={isListening? 'Stop voice interaction': 'Start voice interaction (say "Hey AIRI" or click to talk)'}
                    >

                    </div>

                    {/* Error badge */}
                    {errorText && (
                        <span style={{ fontSize: '9px', color: '#ef4444', maxWidth: '100px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                            {errorText}
                        </span>
                    )}
                </div>
            )}

            {/* ── CSS animations ──────────────────────────────────────────── */}
            <style>{`
                @keyframes airiSpinner {
                    from { transform: rotate(0deg); }
                    to   { transform: rotate(360deg); }
                }
                @keyframes airiGlowPulse {
                    0%, 100% { opacity: 0.6; transform: translateX(-50%) scale(1); }
                    50%       { opacity: 1;   transform: translateX(-50%) scale(1.12); }
                }
                @keyframes airiSlideDown {
                    from { opacity: 0; transform: translateY(-8px); }
                    to   { opacity: 1; transform: translateY(0); }
                }
                @keyframes airiSlideUp {
                    from { opacity: 0; transform: translateX(-50%) translateY(8px); }
                    to   { opacity: 1; transform: translateX(-50%) translateY(0); }
                }
                @keyframes blinkCursor {
                    0%, 100% { opacity: 1; } 50% { opacity: 0; }
                }
                @keyframes hubPulse {
                    0%, 100% { opacity: 1; }
                    50%       { opacity: 0.3; }
                }
            `}</style>
        </div>
    );
};

export default ChatSidebar;
