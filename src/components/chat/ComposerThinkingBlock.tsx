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
    const [open, setOpen] = useState(isStreaming ?? false);
    const [userToggled, setUserToggled] = useState(false);
    // Auto-expand while the model is reasoning so you see it stream; auto-collapse
    // when it finishes — unless the user manually toggled it (then respect that).
    useEffect(() => {
        if (!userToggled) setOpen(!!isStreaming);
    }, [isStreaming, userToggled]);
    const label = isStreaming
        ? 'Thinking…'
        : durationMs
            ? `Thought for ${formatDuration(durationMs)}`
            : 'Thought process';

    if (!thoughts?.trim()) return null;

    return (
        <details
            className="composer-thinking"
            open={open}
            onToggle={(e) => { setUserToggled(true); setOpen((e.target as HTMLDetailsElement).open); }}
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
