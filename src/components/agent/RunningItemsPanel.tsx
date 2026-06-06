import React from 'react';
import { useStore } from '../../store';
import type { SubagentState } from '../../infrastructure/antigravity/antigravityClient';

export interface RunningItemsPanelProps {
    subagents?: SubagentState[];
    onSelectSubagent?: (id: string) => void;
}

const RunningItemsPanel: React.FC<RunningItemsPanelProps> = ({ subagents = [], onSelectSubagent }) => {
    const backgroundAgents = useStore(s => s.backgroundAgents);
    const isThinking = useStore(s => s.isAgentThinking);

    const runningBg = backgroundAgents.filter(b => b.status === 'running' || b.status === 'pending');
    const runningSubs = subagents.filter(s => s.status === 'running' || s.status === 'pending');
    const total = runningBg.length + runningSubs.length + (isThinking ? 1 : 0);

    if (total === 0) {
        return (
            <div style={{ fontSize: 11, opacity: 0.4, padding: '8px 0' }}>
                No background tasks or subagents running.
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
            {isThinking && (
                <RunningRow
                    icon="sync"
                    title="Main agent"
                    status="running"
                    detail="Tool loop active"
                    spin
                />
            )}
            {runningBg.map(b => (
                <RunningRow
                    key={b.id}
                    icon="server-process"
                    title={b.prompt.slice(0, 48) + (b.prompt.length > 48 ? '…' : '')}
                    status={b.status}
                    detail={`Background · ${b.id.slice(0, 8)}`}
                />
            ))}
            {runningSubs.map(s => (
                <RunningRow
                    key={s.id}
                    icon="organization"
                    title={s.name}
                    status={s.status}
                    detail={s.summary || s.role || 'Subagent'}
                    progress={s.progress}
                    onClick={() => onSelectSubagent?.(s.id)}
                />
            ))}
        </div>
    );
};

const RunningRow: React.FC<{
    icon: string;
    title: string;
    status: string;
    detail: string;
    progress?: number;
    spin?: boolean;
    onClick?: () => void;
}> = ({ icon, title, status, detail, progress, spin, onClick }) => (
    <div
        onClick={onClick}
        style={{
            display: 'flex',
            alignItems: 'center',
            gap: 8,
            padding: '8px 10px',
            borderRadius: 8,
            background: 'rgba(0,122,255,0.06)',
            border: '1px solid rgba(0,122,255,0.15)',
            cursor: onClick ? 'pointer' : 'default',
        }}
    >
        <i
            className={`codicon codicon-${icon}${spin ? ' codicon-modifier-spin' : ''}`}
            style={{ color: '#007aff', fontSize: 14 }}
        />
        <div style={{ flex: 1, minWidth: 0 }}>
            <div style={{ fontSize: 12, fontWeight: 600, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                {title}
            </div>
            <div style={{ fontSize: 10, opacity: 0.55 }}>{detail}</div>
        </div>
        <span style={{
            fontSize: 9,
            padding: '2px 6px',
            borderRadius: 4,
            background: status === 'running' ? 'rgba(46,204,113,0.15)' : 'rgba(255,255,255,0.08)',
            color: status === 'running' ? '#2ecc71' : 'rgba(255,255,255,0.6)',
            textTransform: 'uppercase',
            fontWeight: 700,
        }}>
            {progress != null ? `${progress}%` : status}
        </span>
    </div>
);

export default RunningItemsPanel;
