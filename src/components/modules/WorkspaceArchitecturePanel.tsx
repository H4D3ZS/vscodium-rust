// Standalone Architecture Visualizer panel — the proper home for the
// installed module (it previously hijacked the Specs-to-Code wizard).
// Full-screen overlay over the workbench, real workspace data via
// workspace_architecture_layout.

import React, { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { fetchWorkspaceArchitecture } from '../../infrastructure/workspaceArchitecture';
import { ArchitectureVisualizer } from '../ArchitectureVisualizer';

const WorkspaceArchitecturePanel: React.FC = () => {
    const activeModulePanel = useStore((s) => s.activeModulePanel);
    const setActiveModulePanel = useStore((s) => s.setActiveModulePanel);
    const [files, setFiles] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const open = activeModulePanel === 'architecture-visualizer';

    useEffect(() => {
        if (!open) return;
        setLoading(true);
        setError(null);
        fetchWorkspaceArchitecture()
            .then(setFiles)
            .catch((e) => setError(String(e)))
            .finally(() => setLoading(false));
    }, [open]);

    if (!open) return null;

    return (
        <div style={{
            position: 'fixed', inset: 0, zIndex: 900,
            background: 'var(--color-surface, #1e1e1e)',
            display: 'flex', flexDirection: 'column',
        }}>
            <div style={{
                display: 'flex', alignItems: 'center', justifyContent: 'space-between',
                padding: '8px 14px',
                borderBottom: '1px solid var(--color-border, rgba(255,255,255,0.08))',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: 8 }}>
                    <i className="codicon codicon-type-hierarchy-sub" />
                    <span style={{ fontSize: 13, fontWeight: 600 }}>Architecture Visualizer</span>
                    <span style={{ fontSize: 11, opacity: 0.6 }}>
                        {loading ? 'analyzing workspace…' : `${files.length} source files`}
                    </span>
                </div>
                <button
                    type="button"
                    className="settings-button"
                    onClick={() => setActiveModulePanel(null)}
                    title="Close (module panel)"
                >
                    <i className="codicon codicon-close" /> Close
                </button>
            </div>

            <div style={{ flex: 1, position: 'relative' }}>
                {error && (
                    <div style={{ padding: 24, color: '#f87171', fontSize: 12 }}>
                        Could not analyze the workspace: {error}
                    </div>
                )}
                {!error && loading && (
                    <div style={{ position: 'absolute', inset: 0, display: 'flex', alignItems: 'center', justifyContent: 'center' }}>
                        <div className="spinner" style={{
                            width: 36, height: 36,
                            border: '4px solid rgba(255,255,255,0.1)',
                            borderTop: '4px solid var(--color-accent, #007acc)',
                            borderRadius: '50%', animation: 'spin 1s linear infinite',
                        }} />
                    </div>
                )}
                {!error && !loading && files.length === 0 && (
                    <div style={{ padding: 24, opacity: 0.6, fontSize: 12 }}>
                        No source files with symbols found in this workspace.
                    </div>
                )}
                {!error && !loading && files.length > 0 && (
                    <ArchitectureVisualizer files={files} tasks={[]} />
                )}
            </div>
        </div>
    );
};

export default WorkspaceArchitecturePanel;
