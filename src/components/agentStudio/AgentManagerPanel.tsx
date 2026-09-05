import React, { useState } from 'react';
import { useStore } from '../../store';

const AgentManagerPanel: React.FC = () => {
    const bgAgents = useStore(s => s.backgroundAgents);
    const runBackground = useStore(s => s.runBackgroundAgent);
    const remove = useStore(s => s.removeBackgroundAgent);
    const clearAll = useStore(s => s.clearBackgroundAgents);
    const [prompt, setPrompt] = useState('');
    const [spawning, setSpawning] = useState(false);
    const [expanded, setExpanded] = useState<string | null>(null);

    const spawn = async () => {
        if (!prompt.trim()) return;
        setSpawning(true);
        try {
            await runBackground(prompt.trim());
            setPrompt('');
        } finally {
            setSpawning(false);
        }
    };

    const running = bgAgents.filter(b => b.status === 'running').length;

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: 12, gap: 10 }}>
            <div style={{ fontSize: 11, opacity: 0.7 }}>
                Parallel background agents — each runs an autonomous tool loop independently.
            </div>
            <div style={{ display: 'flex', gap: 6 }}>
                <input
                    value={prompt}
                    onChange={e => setPrompt(e.target.value)}
                    onKeyDown={e => e.key === 'Enter' && spawn()}
                    placeholder="Spawn background agent task…"
                    style={{
                        flex: 1, fontSize: 11, padding: '8px 10px', borderRadius: 6,
                        border: '1px solid rgba(255,255,255,0.12)', background: 'rgba(0,0,0,0.25)',
                        color: 'var(--vscode-editor-foreground, #fff)', outline: 'none',
                    }}
                />
                <button
                    onClick={() => void spawn()}
                    disabled={spawning || !prompt.trim()}
                    style={primaryBtn}
                >
                    {spawning? '…': 'Spawn'}
                </button>
            </div>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 10, opacity: 0.6 }}>
                <span>{bgAgents.length} total</span>
                {running > 0 && <span style={{ color: '#60a5fa' }}>{running} running</span>}
                {bgAgents.length > 0 && (
                    <button onClick={clearAll} style={ghostBtn}>Clear finished</button>
                )}
            </div>
            <div style={{ flex: 1, overflowY: 'auto', display: 'flex', flexDirection: 'column', gap: 6 }}>
                {bgAgents.length === 0 && (
                    <div style={{ fontSize: 11, opacity: 0.45, padding: 16, textAlign: 'center' }}>
                        No background agents. Use /bg in chat or spawn above.
                    </div>
                )}
                {[...bgAgents].reverse().map(agent => (
                    <div
                        key={agent.id}
                        style={{
                            border: '1px solid rgba(255,255,255,0.08)', borderRadius: 8,
                            padding: '8px 10px', background: 'rgba(255,255,255,0.02)',
                        }}
                    >
                        <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 4 }}>
                            <StatusDot status={agent.status} />
                            <span style={{ fontSize: 11, fontWeight: 600, flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {agent.prompt.slice(0, 80)}
                            </span>
                            <button onClick={() => setExpanded(expanded === agent.id? null: agent.id)} style={ghostBtn}>
                                {expanded === agent.id? 'Hide': 'Log'}
                            </button>
                            <button onClick={() => remove(agent.id)} style={ghostBtn}></button>
                        </div>
                        <div style={{ fontSize: 9, opacity: 0.45 }}>
                            {agent.status} · {new Date(agent.startedAt).toLocaleTimeString()}
                        </div>
                        {expanded === agent.id && agent.result && (
                            <pre style={{
                                marginTop: 8, fontSize: 10, lineHeight: 1.4, maxHeight: 160,
                                overflowY: 'auto', whiteSpace: 'pre-wrap', wordBreak: 'break-word',
                                background: 'rgba(0,0,0,0.3)', padding: 8, borderRadius: 4,
                            }}>
                                {agent.result}
                            </pre>
                        )}
                    </div>
                ))}
            </div>
        </div>
    );
};

const StatusDot: React.FC<{ status: string }> = ({ status }) => {
    const color = status === 'running'? '#60a5fa': status === 'done'? '#4ade80': status === 'error'? '#f87171': '#888';
    return <span style={{ width: 8, height: 8, borderRadius: '50%', background: color, flexShrink: 0 }} />;
};

const primaryBtn: React.CSSProperties = {
    padding: '8px 14px', fontSize: 11, fontWeight: 600, borderRadius: 6,
    border: '1px solid rgba(96,165,250,0.5)', background: 'rgba(96,165,250,0.15)',
    color: '#93c5fd', cursor: 'pointer',
};

const ghostBtn: React.CSSProperties = {
    padding: '2px 6px', fontSize: 9, border: 'none', background: 'transparent',
    color: 'rgba(255,255,255,0.5)', cursor: 'pointer',
};

export default AgentManagerPanel;
