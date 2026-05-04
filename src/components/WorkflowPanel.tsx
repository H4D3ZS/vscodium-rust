import React, { useState, useEffect } from 'react';
import { useStore, type FileEntry } from '../store';
import { invoke } from '@tauri-apps/api/core';

const WorkflowPanel: React.FC = () => {
    const [workflows, setWorkflows] = useState<FileEntry[]>([]);
    const [loading, setLoading] = useState(false);
    const activeRoot = useStore(state => state.activeRoot);

    const loadWorkflows = async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const paths = [`${activeRoot}/.agent/workflows`, `${activeRoot}/.agents/workflows`];
            let allWfs: FileEntry[] = [];
            for (const p of paths) {
                try {
                    const entries = await invoke<FileEntry[]>('list_directory', { path: p });
                    allWfs = [...allWfs, ...entries.filter(e => !e.is_dir && e.name.endsWith('.md'))];
                } catch (e) {}
            }
            // Remove duplicates by path
            const uniqueWfs = Array.from(new Map(allWfs.map(item => [item.path, item])).values());
            setWorkflows(uniqueWfs);
        } catch (e) {
            console.error("Failed to load workflows:", e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        loadWorkflows();
    }, [activeRoot]);

    const runWorkflow = (path: string) => {
        // Feed the workflow to the agent
        const input = document.querySelector('.agent-input-section textarea') as HTMLTextAreaElement;
        if (input) {
            input.value = `Execute the workflow defined in: ${path}`;
            // Trigger send (assuming the event listener handles it)
            const event = new KeyboardEvent('keydown', { key: 'Enter', bubbles: true });
            input.dispatchEvent(event);
        }
    };

    return (
        <div className="workflow-panel" style={{ display: 'flex', flexDirection: 'column', height: '100%', padding: '12px' }}>
            <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '12px' }}>
                <span style={{ fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', opacity: 0.8 }}>Available Workflows</span>
                <i className="codicon codicon-refresh" onClick={loadWorkflows} style={{ cursor: 'pointer', fontSize: '14px', opacity: 0.6 }}></i>
            </div>

            {loading ? (
                <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>Loading workflows...</div>
            ) : workflows.length === 0 ? (
                <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px', fontStyle: 'italic' }}>
                    No workflows found in .agent/workflows.
                </div>
            ) : (
                <div className="workflow-list" style={{ display: 'flex', flexDirection: 'column', gap: '8px' }}>
                    {workflows.map(wf => (
                        <div 
                            key={wf.path} 
                            className="workflow-item"
                            style={{
                                padding: '10px',
                                background: 'rgba(255,255,255,0.03)',
                                border: '1px solid var(--vscode-panel-border)',
                                borderRadius: '4px',
                                display: 'flex',
                                alignItems: 'center',
                                gap: '10px',
                                cursor: 'pointer',
                                transition: 'background 0.2s'
                            }}
                            onMouseOver={e => e.currentTarget.style.background = 'rgba(255,255,255,0.06)'}
                            onMouseOut={e => e.currentTarget.style.background = 'rgba(255,255,255,0.03)'}
                            onClick={() => runWorkflow(wf.path)}
                        >
                            <i className="codicon codicon-play" style={{ fontSize: '16px', color: '#4ade80', opacity: 0.8 }}></i>
                            <div style={{ flex: 1, minWidth: 0 }}>
                                <div style={{ fontSize: '13px', fontWeight: 500, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {wf.name.replace('.md', '')}
                                </div>
                                <div style={{ fontSize: '11px', opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                    {wf.path.split(/[/\\]/).pop()}
                                </div>
                            </div>
                        </div>
                    ))}
                </div>
            )}

            <div style={{ marginTop: 'auto', padding: '12px', background: 'rgba(59, 130, 246, 0.05)', borderRadius: '6px', border: '1px solid rgba(59, 130, 246, 0.2)' }}>
                <div style={{ display: 'flex', alignItems: 'center', justifyContent: 'space-between', marginBottom: '8px' }}>
                    <div style={{ fontSize: '12px', fontWeight: 600, color: '#60a5fa' }}>Autonomous Execution</div>
                </div>
                <div style={{ fontSize: '11px', opacity: 0.8, lineHeight: '1.4' }}>
                    Clicking a workflow triggers the AI Agent to autonomously evaluate and execute the steps defined in the workflow file.
                </div>
            </div>
        </div>
    );
};

export default WorkflowPanel;
