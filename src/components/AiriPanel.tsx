import React, { useEffect, useRef, useState, useCallback, useMemo } from 'react';
import { listen } from '@tauri-apps/api/event';
import { useStore } from '../store';

interface AiriPanelProps {
    className?: string;
    style?: React.CSSProperties;
    scale?: number;
    yOffset?: string;
    transparent?: boolean;
}

// ── TTS Engine ───────────────────────────────────────────────────────────────
let ttsQueue: string[] = [];
let ttsSpeaking = false;

function ttsSpeak(text: string, rate = 1.0, pitch = 1.1, volume = 0.85) {
    if (!window.speechSynthesis || !text.trim()) return;
    // Strip markdown / code blocks for natural speech
    const clean = text
        .replace(/```[\s\S]*?```/g, ' code block ')
        .replace(/`[^`]+`/g, '')
        .replace(/[*_#>[\]]/g, '')
        .replace(/https?:\/\/\S+/g, ' link ')
        .replace(/\s+/g, ' ')
        .trim()
        .slice(0, 300); // Keep it short

    if (!clean) return;
    ttsQueue.push(clean);
    if (!ttsSpeaking) drainTtsQueue(rate, pitch, volume);
}

function drainTtsQueue(rate: number, pitch: number, volume: number) {
    if (ttsQueue.length === 0) { ttsSpeaking = false; return; }
    ttsSpeaking = true;
    const utt = new SpeechSynthesisUtterance(ttsQueue.shift()!);
    utt.rate = rate;
    utt.pitch = pitch;
    utt.volume = volume;

    // Try to pick a female voice
    const voices = window.speechSynthesis.getVoices();
    const preferred = voices.find(v =>
        /female|zira|hazel|samantha|victoria|karen|moira|fiona|tessa|aria|jenny|sonia/i.test(v.name)
    );
    if (preferred) utt.voice = preferred;

    utt.onend = () => drainTtsQueue(rate, pitch, volume);
    utt.onerror = () => drainTtsQueue(rate, pitch, volume);
    window.speechSynthesis.speak(utt);
}

function ttsStop() {
    window.speechSynthesis?.cancel();
    ttsQueue = [];
    ttsSpeaking = false;
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
    idle:       { color: '#818cf8', glow: 'rgba(129,140,248,0.3)', label: 'Idle',      emoji: '✦' },
    thinking:   { color: '#c084fc', glow: 'rgba(192,132,252,0.4)', label: 'Thinking',  emoji: '◎' },
    coding:     { color: '#34d399', glow: 'rgba(52,211,153,0.4)',  label: 'Coding',    emoji: '⌨' },
    reading:    { color: '#60a5fa', glow: 'rgba(96,165,250,0.35)', label: 'Reading',   emoji: '👁' },
    executing:  { color: '#f59e0b', glow: 'rgba(245,158,11,0.4)',  label: 'Executing', emoji: '⚡' },
    browsing:   { color: '#38bdf8', glow: 'rgba(56,189,248,0.35)', label: 'Browsing',  emoji: '🌐' },
    committing: { color: '#a3e635', glow: 'rgba(163,230,53,0.35)', label: 'Committing',emoji: '📦' },
    patching:   { color: '#fb923c', glow: 'rgba(251,146,60,0.35)', label: 'Patching',  emoji: '🔧' },
    success:    { color: '#10b981', glow: 'rgba(16,185,129,0.4)',  label: 'Done',      emoji: '✓' },
    error:      { color: '#ef4444', glow: 'rgba(239,68,68,0.4)',   label: 'Error',     emoji: '✗' },
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
        const base = text.startsWith(prevRef.current) ? prevRef.current : '';
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

export const AiriPanel: React.FC<AiriPanelProps> = ({ className, style, scale, yOffset, transparent }) => {
    const iframeRef = useRef<HTMLIFrameElement>(null);
    const [isAiriLoading, setAiriLoading] = useState(true);
    const [isTtsEnabled, setTtsEnabled] = useState(false);

    // ── Live agent state from store ──────────────────────────────────────────
    const isAgentThinking = useStore(s => s.isAgentThinking);
    const agentMessages   = useStore(s => s.agentMessages);
    const aiStatus        = useStore(s => s.aiStatus);

    // ── Live tool-call tracking ──────────────────────────────────────────────
    const [currentTool,    setCurrentTool]    = useState<string | null>(null);
    const [currentFile,    setCurrentFile]    = useState<string | null>(null);
    const [currentCommand, setCurrentCommand] = useState<string | null>(null);
    const [activity,       setActivity]       = useState<AiriActivity>('idle');
    const [thoughtText,    setThoughtText]    = useState('');
    const [completedCount, setCompletedCount] = useState(0);
    const [errorText,      setErrorText]      = useState<string | null>(null);

    const typedThought = useTypewriter(thoughtText, 14);

    // ── Derive activity from agent state ─────────────────────────────────────
    useEffect(() => {
        if (!isAgentThinking) {
            if (aiStatus === 'dead') {
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
    }, [isAgentThinking, aiStatus]);

    // ── Capture latest AI response text for thought bubble ───────────────────
    useEffect(() => {
        const last = agentMessages[agentMessages.length - 1];
        if (last?.role === 'assistant' && typeof last.content === 'string') {
            const clean = last.content
                .replace(/```[\s\S]*?```/g, '')
                .replace(/[*_#>[\]]/g, '')
                .replace(/\s+/g, ' ')
                .trim();
            if (clean.length > 5) {
                const snippet = clean.slice(-180);
                setThoughtText(snippet);
                if (isTtsEnabled && snippet.length > 20) {
                    ttsSpeak(snippet);
                }
            }
        }
    }, [agentMessages, isTtsEnabled]);

    // ── Listen for tool events ────────────────────────────────────────────────
    useEffect(() => {
        const subs: (() => void)[] = [];
        import('@tauri-apps/api/event').then(({ listen: listenEv }) => {
            listenEv<any>('ai-tool-call', (e) => {
                const name  = e.payload?.name  || '';
                const args  = e.payload?.args  || {};
                const act   = TOOL_TO_ACTIVITY[name] || 'thinking';
                setActivity(act);
                setCurrentTool(name);
                setErrorText(null);

                // Extract meaningful context from args
                const file = args.path || args.file_path || args.uri || args.target_file || null;
                const cmd  = args.command || args.cmd || null;
                if (file) setCurrentFile(stripAnsi(typeof file === 'string' ? file.split(/[\\/]/).slice(-2).join('/') : ''));
                if (cmd)  setCurrentCommand(stripAnsi(typeof cmd  === 'string' ? cmd.slice(0, 60) : ''));

                // Inject a live status into thought
                const label = ACTIVITY_META[act]?.label || name;
                if (file) setThoughtText(`${label}: ${file.split(/[\\/]/).slice(-1)[0]}`);
                else if (cmd) setThoughtText(`${label}: ${cmd.slice(0, 60)}`);
                else setThoughtText(`${label}...`);

                // Forward to VRM manifold
                iframeRef.current?.contentWindow?.postMessage({
                    type: 'airi-activity', payload: { activity: act, tool: name, file, cmd }
                }, '*');
            }).then(u => subs.push(u));

            listenEv<any>('ai-tool-result', (e) => {
                const name = e.payload?.name || '';
                const act  = TOOL_TO_ACTIVITY[name] || 'thinking';
                if (act !== 'thinking') {
                    setCompletedCount(c => c + 1);
                }
                setActivity(isAgentThinking ? 'thinking' : 'success');
            }).then(u => subs.push(u));

            listenEv('hades-sync', (event) => {
                iframeRef.current?.contentWindow?.postMessage(
                    { type: 'hades-sync', payload: event.payload }, '*'
                );
            }).then(u => subs.push(u));
        });
        return () => subs.forEach(u => u());
    }, [isAgentThinking]);

    // ── Mouse look-at ────────────────────────────────────────────────────────
    useEffect(() => {
        const handleMouseMove = (e: MouseEvent) => {
            if (!iframeRef.current?.contentWindow) return;
            const rect = iframeRef.current.getBoundingClientRect();
            iframeRef.current.contentWindow.postMessage({
                type: 'hades-focus',
                payload: { x: e.clientX - rect.left, y: e.clientY - rect.top }
            }, '*');
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

    const url = useMemo(() => {
        const base = "http://localhost:5174/?headless=true";
        const scaleParam       = scale       ? `&scale=${scale}` : "";
        const yOffsetParam     = yOffset     ? `&yOffset=${encodeURIComponent(yOffset)}` : "";
        const transparentParam = transparent ? `&transparent=true` : "";
        return `${base}${scaleParam}${yOffsetParam}${transparentParam}`;
    }, [scale, yOffset, transparent]);

    return (
        <div
            className={`airi-panel ${className || ''}`}
            style={{ width: '100%', height: '100%', position: 'relative', background: 'transparent', border: 'none', ...style }}
        >
            {/* ── VRM Manifold Iframe ────────────────────────────────────── */}
            {isAiriLoading && (
                <div style={{
                    position: 'absolute', inset: 0, display: 'flex', alignItems: 'center', justifyContent: 'center',
                    color: meta.color, fontSize: '9px', textTransform: 'uppercase', letterSpacing: '1px', opacity: 0.5
                }}>
                    {isSmall ? '...' : 'Syncing Manifold...'}
                </div>
            )}
            <iframe
                ref={iframeRef}
                src={url}
                allowTransparency={true}
                style={{ width: '100%', height: '100%', border: 'none', opacity: isAiriLoading ? 0 : 1, background: 'transparent' }}
                onLoad={() => setAiriLoading(false)}
            />

            {/* ── Activity Glow Ring (behind avatar) ─────────────────────── */}
            {!isSmall && (
                <div style={{
                    position: 'absolute', bottom: '18%', left: '50%', transform: 'translateX(-50%)',
                    width: '90px', height: '90px', borderRadius: '50%',
                    boxShadow: `0 0 40px 20px ${meta.glow}`,
                    background: 'transparent', pointerEvents: 'none',
                    transition: 'box-shadow 0.8s ease',
                    animation: activity === 'thinking' ? 'airiGlowPulse 1.4s ease-in-out infinite' : 'none',
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
                    <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: currentFile || currentCommand ? '4px' : 0 }}>
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
            {!isSmall && isAgentThinking && typedThought && (
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
                    }}>
                        <div style={{
                            fontSize: '11px', color: '#0d0921', lineHeight: 1.5,
                            maxWidth: '200px', wordBreak: 'break-word',
                        }}>
                            {typedThought}
                            <span style={{ animation: 'blinkCursor 0.8s step-end infinite', marginLeft: '1px' }}>|</span>
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
                    display: 'flex', alignItems: 'center', gap: '6px',
                    background: 'rgba(0,0,0,0.55)', backdropFilter: 'blur(8px)',
                    border: `1px solid ${meta.color}44`,
                    borderRadius: '20px', padding: '3px 10px',
                    pointerEvents: 'auto',
                }}>
                    <span style={{
                        width: '6px', height: '6px', borderRadius: '50%', background: meta.color,
                        display: 'inline-block', flexShrink: 0,
                        animation: (activity === 'thinking' || activity === 'coding' || activity === 'executing')
                            ? 'hubPulse 1s infinite' : 'none'
                    }} />
                    <span style={{ fontSize: '9px', fontWeight: 700, color: meta.color, textTransform: 'uppercase', letterSpacing: '0.06em', whiteSpace: 'nowrap' }}>
                        {meta.label}
                    </span>

                    {/* TTS toggle */}
                    <div
                        onClick={() => { setTtsEnabled(v => !v); if (isTtsEnabled) ttsStop(); }}
                        style={{
                            marginLeft: '4px', cursor: 'pointer', fontSize: '10px',
                            opacity: isTtsEnabled ? 1 : 0.35,
                            color: isTtsEnabled ? '#c084fc' : 'rgba(255,255,255,0.5)',
                        }}
                        title={isTtsEnabled ? 'Mute AIRI voice' : 'Enable AIRI voice'}
                    >
                        {isTtsEnabled ? '🔊' : '🔇'}
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

export default AiriPanel;
