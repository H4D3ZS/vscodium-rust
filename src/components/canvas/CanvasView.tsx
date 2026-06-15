/**
 * CanvasView — editor-tab host for an agent-generated canvas.
 * Resolves the canvas id from the tab path (canvas://<id>), subscribes to the
 * store so live agent updates re-render in place.
 */
import React from 'react';
import { useStore } from '../../store';
import CanvasRenderer from './CanvasRenderer';

const CanvasView: React.FC<{ path: string }> = ({ path }) => {
    const id = path.replace(/^canvas:\/\//, '');
    const spec = useStore((s) => s.canvases.find((c) => c.id === id));
    const removeCanvas = useStore((s) => s.removeCanvas);

    if (!spec) {
        return (
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'center', height: '100%', opacity: 0.5, fontSize: 13 }}>
                Canvas not found — it may have been deleted.
            </div>
        );
    }

    return (
        <div style={{ height: '100%', overflowY: 'auto', background: 'var(--vscode-editor-background)' }}>
            <div style={{ padding: '24px 28px 48px', position: 'relative' }}>
                <div style={{ position: 'absolute', top: 14, right: 20, display: 'flex', gap: 8 }}>
                    <button
                        title="Delete canvas"
                        onClick={() => removeCanvas(spec.id)}
                        style={{
                            background: 'transparent', border: '1px solid rgba(255,255,255,0.15)',
                            borderRadius: 5, color: 'inherit', opacity: 0.55, cursor: 'pointer',
                            fontSize: 11, padding: '3px 8px',
                        }}
                    >
                        Delete
                    </button>
                </div>
                <CanvasRenderer spec={spec} />
            </div>
        </div>
    );
};

export default CanvasView;
