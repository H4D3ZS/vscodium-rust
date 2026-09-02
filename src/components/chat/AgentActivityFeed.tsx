/**
 * Cursor-style live agent activity — compact tool/recon lines in the chat pane.
 */
import React from 'react';
import { useStore } from '../../store';
import { formatCursorActivityLine } from '../../domain/agent/cleanAgentContent';

interface AgentActivityFeedProps {
    isAgentThinking: boolean;
    maxItems?: number;
}

const AgentActivityFeed: React.FC<AgentActivityFeedProps> = ({ isAgentThinking, maxItems = 10 }) => {
    const trajectory = useStore((s) => s.agentTrajectory);
    const currentAction = useStore((s) => s.agentCurrentAction);

    const events = trajectory.slice(-maxItems);
    if (!isAgentThinking && events.length === 0) return null;

    return (
        <div
            className="agent-activity-feed"
            style={{
                margin: '4px 10px 8px',
                padding: '8px 10px',
                borderRadius: '8px',
                border: '1px solid rgba(255,255,255,0.06)',
                background: 'rgba(255,255,255,0.02)',
            }}
        >
            {isAgentThinking && (
                <div style={{
                    display: 'flex',
                    alignItems: 'center',
                    gap: '6px',
                    fontSize: '11px',
                    opacity: 0.55,
                    marginBottom: events.length ? '8px' : 0,
                }}>
                    <span style={{
                        width: '6px',
                        height: '6px',
                        borderRadius: '50%',
                        background: '#3794ff',
                        display: 'inline-block',
                        animation: 'hubPulse 1s infinite',
                        flexShrink: 0,
                    }} />
                    <span>{(typeof currentAction === 'string' ? currentAction : '') || 'Working…'}</span>
                </div>
            )}

            {events.map((evt) => {
                const line = formatCursorActivityLine(evt.tool, evt.title, evt.detail, evt.success);
                const running = evt.kind === 'tool_call' && evt.success === undefined;
                return (
                    <div
                        key={evt.id}
                        style={{
                            display: 'flex',
                            alignItems: 'flex-start',
                            gap: '8px',
                            fontSize: '11px',
                            lineHeight: 1.45,
                            padding: '2px 0',
                            color: evt.success === false
                                ? '#f48771'
                                : running
                                    ? 'rgba(255,255,255,0.82)'
                                    : 'rgba(255,255,255,0.45)',
                        }}
                    >
                        <span style={{
                            marginTop: '5px',
                            width: '5px',
                            height: '5px',
                            borderRadius: '50%',
                            flexShrink: 0,
                            background: running ? '#3794ff' : evt.success === false ? '#f48771' : 'rgba(255,255,255,0.25)',
                            animation: running ? 'hubPulse 1s infinite' : undefined,
                        }} />
                        <span style={{ wordBreak: 'break-word' }}>{line}</span>
                    </div>
                );
            })}
        </div>
    );
};

export default AgentActivityFeed;
