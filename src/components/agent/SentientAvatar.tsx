import React from 'react';
import idleImg    from '../../assets/sentient_avatar/idle.png';
import thinkingImg from '../../assets/sentient_avatar/thinking.png';
import codingImg  from '../../assets/sentient_avatar/coding.png';
import errorImg   from '../../assets/sentient_avatar/error.png';

export type AvatarState = 'idle' | 'thinking' | 'coding' | 'error' | 'success' | 'executing' | 'browsing';

interface SentientAvatarProps {
    state: AvatarState;
    size?: number;
}

const STATE_CONFIG: Record<AvatarState, {
    img: string;
    animation: string;
    glow: string;
    border: string;
    badge?: string;
}> = {
    idle:      { img: idleImg,     animation: 'sa-float 3s ease-in-out infinite',     glow: 'rgba(59,130,246,0.2)',   border: 'rgba(255,255,255,0.1)' },
    thinking:  { img: thinkingImg, animation: 'sa-pulse 1s ease-in-out infinite',     glow: 'rgba(167,139,250,0.4)', border: 'rgba(167,139,250,0.5)', badge: '◎' },
    coding:    { img: codingImg,   animation: 'sa-spin 10s linear infinite',          glow: 'rgba(52,211,153,0.35)', border: 'rgba(52,211,153,0.4)',  badge: '⌨' },
    executing: { img: codingImg,   animation: 'sa-flash 0.6s ease-in-out infinite',   glow: 'rgba(245,158,11,0.4)',  border: 'rgba(245,158,11,0.6)',  badge: '⚡' },
    browsing:  { img: thinkingImg, animation: 'sa-float 2s ease-in-out infinite',     glow: 'rgba(56,189,248,0.35)', border: 'rgba(56,189,248,0.4)',  badge: '🌐' },
    success:   { img: idleImg,     animation: 'sa-bounce 0.4s ease-out 2',            glow: 'rgba(16,185,129,0.45)', border: 'rgba(16,185,129,0.6)',  badge: '✓' },
    error:     { img: errorImg,    animation: 'sa-shake 0.5s linear infinite',        glow: 'rgba(239,68,68,0.4)',   border: '#ef4444',               badge: '✗' },
};

const SentientAvatar: React.FC<SentientAvatarProps> = ({ state, size = 32 }) => {
    const cfg = STATE_CONFIG[state] || STATE_CONFIG.idle;
    return (
        <div style={{
            width: size, height: size, borderRadius: '50%', overflow: 'hidden',
            flexShrink: 0, position: 'relative',
            boxShadow: `0 0 ${size * 0.5}px ${cfg.glow}`,
            border: `1px solid ${cfg.border}`,
            background: '#000',
            animation: cfg.animation,
            transition: 'box-shadow 0.4s ease, border-color 0.4s ease',
        }}>
            <style>{`
                @keyframes sa-float   { 0%,100%{transform:translateY(0)}   50%{transform:translateY(-3px)} }
                @keyframes sa-pulse   { 0%,100%{transform:scale(1);opacity:.8} 50%{transform:scale(1.06);opacity:1} }
                @keyframes sa-spin    { from{transform:rotate(0deg)} to{transform:rotate(360deg)} }
                @keyframes sa-shake   { 0%,100%{transform:translateX(0)} 25%{transform:translateX(-2px)} 75%{transform:translateX(2px)} }
                @keyframes sa-flash   { 0%,100%{opacity:1} 50%{opacity:0.55} }
                @keyframes sa-bounce  { 0%,100%{transform:translateY(0)} 40%{transform:translateY(-6px)} 70%{transform:translateY(-3px)} }
            `}</style>
            <img
                src={cfg.img}
                alt={`AIRI: ${state}`}
                style={{ width: '100%', height: '100%', objectFit: 'cover', transition: 'all 0.5s ease-in-out' }}
            />
            {cfg.badge && size > 24 && (
                <div style={{
                    position: 'absolute', bottom: -1, right: -1,
                    width: Math.max(12, size * 0.38), height: Math.max(12, size * 0.38),
                    borderRadius: '50%', background: '#0d0d1a',
                    display: 'flex', alignItems: 'center', justifyContent: 'center',
                    fontSize: size * 0.22, border: `1px solid ${cfg.border}`,
                    lineHeight: 1,
                }}>
                    {cfg.badge}
                </div>
            )}
        </div>
    );
};

export default SentientAvatar;
