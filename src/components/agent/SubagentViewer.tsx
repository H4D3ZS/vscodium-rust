import React from 'react';
import type { SubagentState, TrajectoryStep } from '../../infrastructure/antigravity/antigravityClient';

export interface SubagentViewerProps {
    subagent: SubagentState | null;
    steps: TrajectoryStep[];
    onClose: () => void;
}

const SubagentViewer: React.FC<SubagentViewerProps> = ({ subagent, steps, onClose }) => {
    if (!subagent) return null;
    const filtered = steps.filter(s => s.subagent_id === subagent.id);

    return (
        <div style={{
            marginBottom: 16,
            padding: 12,
            borderRadius: 10,
            background: 'rgba(88,86,214,0.08)',
            border: '1px solid rgba(88,86,214,0.25)',
        }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 8 }}>
                <i className="codicon codicon-organization" style={{ color: '#5856d6' }} />
                <div style={{ flex: 1 }}>
                    <div style={{ fontSize: 13, fontWeight: 700 }}>{subagent.name}</div>
                    <div style={{ fontSize: 10, opacity: 0.55 }}>
                        {subagent.role || 'Subagent'} · {subagent.status}
                    </div>
                </div>
                <button
                    type="button"
                    onClick={onClose}
                    style={{ border: 'none', background: 'transparent', cursor: 'pointer', opacity: 0.6 }}
                >
                    <i className="codicon codicon-close" />
                </button>
            </div>
            {subagent.summary && (
                <p style={{ fontSize: 11, opacity: 0.7, margin: '0 0 8px' }}>{subagent.summary}</p>
            )}
            <div style={{ fontSize: 10, fontWeight: 700, opacity: 0.45, marginBottom: 4, textTransform: 'uppercase' }}>
                Sub-trajectory ({filtered.length} steps)
            </div>
            {filtered.length === 0 ? (
                <div style={{ fontSize: 11, opacity: 0.4 }}>No steps recorded yet.</div>
            ) : (
                filtered.slice(-8).map(s => (
                    <div key={s.id} style={{ fontSize: 11, opacity: 0.75, padding: '2px 0' }}>
                        <i className={`codicon codicon-${s.success === false ? 'error' : 'pass'}`} style={{ marginRight: 6, fontSize: 10 }} />
                        {s.title}
                    </div>
                ))
            )}
        </div>
    );
};

export default SubagentViewer;
