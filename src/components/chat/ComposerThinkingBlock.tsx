import React, { useState, useEffect } from 'react';

interface ComposerThinkingBlockProps {
    thoughts: string;
    durationMs?: number;
    isStreaming?: boolean;
}

function formatDuration(ms?: number): string {
    if (!ms || ms < 500) return '';
    const sec = Math.round(ms / 1000);
    if (sec < 60) return `${sec}s`;
    return `${Math.floor(sec / 60)}m ${sec % 60}s`;
}

const ComposerThinkingBlock: React.FC<ComposerThinkingBlockProps> = ({ thoughts, durationMs, isStreaming }) => {
    // Cursor-style: ALWAYS collapsed. The reasoning never streams into the chat;
    // you only see the live "Thinking…" / final "Thought for Ns" label. Click to
    // expand and read the reasoning on demand.
    const [open, setOpen] = useState(false);

    // Live elapsed counter while the model reasons (no durationMs yet).
    const [liveMs, setLiveMs] = useState(0);
    useEffect(() => {
        if (!isStreaming) return;
        const start = Date.now();
        setLiveMs(0);
        const t = setInterval(() => setLiveMs(Date.now() - start), 500);
        return () => clearInterval(t);
    }, [isStreaming]);

    const label = isStreaming
        ? (liveMs >= 1000 ? `Thinking for ${formatDuration(liveMs)}` : 'Thinking…')
        : durationMs
            ? `Thought for ${formatDuration(durationMs)}`
            : 'Thought process';

    if (!thoughts?.trim()) return null;

    return (
        <details
            className="composer-thinking"
            open={open}
            onToggle={(e) => setOpen((e.target as HTMLDetailsElement).open)}
        >
            <summary className="composer-thinking__summary">
                <span className={`composer-thinking__dot${isStreaming ? ' composer-thinking__dot--live' : ''}`} />
                <span>{label}</span>
                <i className={`codicon codicon-chevron-${open ? 'up' : 'down'}`} style={{ fontSize: 10, opacity: 0.45, marginLeft: 'auto' }} />
            </summary>
            <pre className="composer-thinking__body">{thoughts.trim()}</pre>
        </details>
    );
};

export default ComposerThinkingBlock;
