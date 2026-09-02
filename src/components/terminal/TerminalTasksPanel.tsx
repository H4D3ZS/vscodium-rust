/**
 * cmder/Warp-style task launcher — saved workflows as clickable buttons.
 * Pure terminal feature (no AI).
 */
import React from 'react';
import { listTerminalWorkflows, runWorkflowCommand } from '../../application/terminal/runWorkflow';

const TerminalTasksPanel: React.FC = () => {
    const workflows = listTerminalWorkflows();

    if (workflows.length === 0) return null;

    return (
        <div
            className="terminal-tasks-panel"
            style={{
                borderTop: '1px solid var(--vscode-panel-border, rgba(128,128,128,0.35))',
                padding: '4px 8px',
                display: 'flex',
                flexWrap: 'wrap',
                gap: '4px',
                maxHeight: '72px',
                overflowY: 'auto',
                flexShrink: 0,
            }}
        >
            <span style={{ fontSize: '10px', opacity: 0.5, alignSelf: 'center', marginRight: '4px' }}>
                Tasks
            </span>
            {workflows.slice(0, 12).map((wf) => (
                <button
                    key={wf.id}
                    type="button"
                    title={wf.command}
                    onClick={() => void runWorkflowCommand(wf.command)}
                    style={{
                        fontSize: '10px',
                        padding: '2px 8px',
                        borderRadius: '3px',
                        border: '1px solid var(--vscode-panel-border, rgba(128,128,128,0.35))',
                        background: 'var(--vscode-button-secondaryBackground, rgba(128,128,128,0.15))',
                        color: 'var(--vscode-button-secondaryForeground, #ccc)',
                        cursor: 'pointer',
                        display: 'flex',
                        alignItems: 'center',
                        gap: '4px',
                    }}
                >
                    {wf.icon && <i className={`codicon codicon-${wf.icon}`} style={{ fontSize: '11px' }} />}
                    {wf.name}
                </button>
            ))}
        </div>
    );
};

export default TerminalTasksPanel;
