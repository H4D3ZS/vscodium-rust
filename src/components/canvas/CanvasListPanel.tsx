/**
 * CanvasListPanel — sidebar browser for agent-generated canvases.
 * Lets the user reopen or delete durable canvas artifacts.
 */
import React from 'react';
import { useStore } from '../../store';

const CanvasListPanel: React.FC = () => {
    const canvases = useStore((s) => s.canvases);
    const openCanvasTab = useStore((s) => s.openCanvasTab);
    const removeCanvas = useStore((s) => s.removeCanvas);

    if (canvases.length === 0) {
        return (
            <div style={{ padding: '24px 16px', textAlign: 'center', opacity: 0.55, fontSize: 12, lineHeight: 1.6 }}>
                <i className="codicon codicon-dashboard" style={{ fontSize: 28, display: 'block', marginBottom: 10, opacity: 0.5, fontFamily: 'codicon', fontStyle: 'normal' }} />
                No canvases yet.
                <div style={{ marginTop: 8, fontSize: 11, opacity: 0.8 }}>
                    Ask the agent to visualize results — e.g. “show the scan findings as a dashboard”.
                </div>
            </div>
        );
    }

    const sorted = [...canvases].sort((a, b) => b.updatedAt - a.updatedAt);

    return (
        <div style={{ display: 'flex', flexDirection: 'column', overflowY: 'auto', flex: 1 }}>
            {sorted.map((c) => (
                <div
                    key={c.id}
                    onClick={() => openCanvasTab(c.id)}
                    style={{
                        display: 'flex', alignItems: 'center', gap: 8,
                        padding: '8px 12px', cursor: 'pointer', fontSize: 12.5,
                        borderBottom: '1px solid rgba(255,255,255,0.04)',
                    }}
                    onMouseEnter={(e) => (e.currentTarget.style.background = 'rgba(255,255,255,0.04)')}
                    onMouseLeave={(e) => (e.currentTarget.style.background = 'transparent')}
                >
                    <i className="codicon codicon-dashboard" style={{ opacity: 0.6, fontFamily: 'codicon', fontStyle: 'normal' }} />
                    <div style={{ flex: 1, minWidth: 0 }}>
                        <div style={{ whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{c.title}</div>
                        <div style={{ fontSize: 10, opacity: 0.45 }}>
                            {c.blocks.length} block{c.blocks.length === 1 ? '' : 's'} · {new Date(c.updatedAt).toLocaleDateString()}
                        </div>
                    </div>
                    <i
                        className="codicon codicon-trash"
                        title="Delete canvas"
                        onClick={(e) => { e.stopPropagation(); removeCanvas(c.id); }}
                        style={{ opacity: 0.4, fontFamily: 'codicon', fontStyle: 'normal' }}
                    />
                </div>
            ))}
        </div>
    );
};

export default CanvasListPanel;
