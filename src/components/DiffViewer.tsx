import { useStore } from '../store';
import type { PendingChange } from '../store';
import { Sparkles, Check, X } from 'lucide-react';

const DiffViewer: React.FC = () => {
    const pendingChanges = useStore(state => state.pendingChanges);
    const acceptPendingChange = useStore(state => state.acceptPendingChange);
    const rejectPendingChange = useStore(state => state.rejectPendingChange);

    if (pendingChanges.length === 0) return null;

    const change = pendingChanges[0];

    return (
        <div className="diff-viewer-overlay">
            <div className="diff-viewer-header">
                <div className="diff-info">
                    <Sparkles className="spark-icon" size={14} />
                    <span className="file-path">{change.path}</span>
                    <span className="diff-desc">{change.description}</span>
                </div>
                <div className="diff-actions">
                    <button className="btn-reject" onClick={() => rejectPendingChange(change.id)}>
                        <X size={14} /> Reject
                    </button>
                    <button className="btn-accept" onClick={() => acceptPendingChange(change.id)}>
                        <Check size={14} /> Accept Changes
                    </button>
                </div>
            </div>
            <div className="diff-content">
                <pre className="diff-pre">
                    {change.proposedContent}
                </pre>
            </div>

            <style>{`
                .diff-viewer-overlay {
                    position: absolute;
                    top: 0;
                    left: 0;
                    right: 0;
                    z-index: 1000;
                    background: var(--vscode-editor-background, #1e1e1e);
                    border-bottom: 1px solid var(--vscode-panel-border, #454545);
                    box-shadow: 0 4px 20px rgba(0,0,0,0.5);
                    animation: slideDown 0.25s cubic-bezier(0, 0, 0.2, 1);
                    max-height: 40vh;
                    display: flex;
                    flex-direction: column;
                }

                @keyframes slideDown {
                    from { transform: translateY(-100%); }
                    to { transform: translateY(0); }
                }

                .diff-viewer-header {
                    display: flex;
                    justify-content: space-between;
                    align-items: center;
                    padding: 10px 20px;
                    background: var(--vscode-editorWidget-background, #252526);
                    border-bottom: 1px solid var(--vscode-panel-border, #454545);
                }

                .diff-info {
                    display: flex;
                    align-items: center;
                    gap: 12px;
                    font-size: 13px;
                }

                .spark-icon {
                    color: var(--terminator-accent, #00c6ff);
                }

                .file-path {
                    font-weight: 600;
                    color: var(--vscode-breadcrumb-foreground, #ccc);
                }

                .diff-desc {
                    color: var(--vscode-descriptionForeground, #888);
                    opacity: 0.8;
                }

                .diff-actions {
                    display: flex;
                    gap: 10px;
                }

                .diff-content {
                    flex: 1;
                    overflow: auto;
                    padding: 12px 20px;
                    background: var(--vscode-editor-background);
                }

                .diff-pre {
                    margin: 0;
                    font-family: var(--vscode-editor-font-family, 'Cascadia Code', monospace);
                    font-size: 12px;
                    color: var(--vscode-editor-foreground);
                    white-space: pre-wrap;
                }

                button {
                    display: flex;
                    align-items: center;
                    gap: 6px;
                    padding: 6px 14px;
                    border-radius: 4px;
                    border: none;
                    font-size: 12px;
                    font-weight: 600;
                    cursor: pointer;
                    transition: all 0.2s;
                }

                button:hover {
                    filter: brightness(1.1);
                    transform: translateY(-1px);
                }

                .btn-accept {
                    background: var(--vscode-button-background, #0e639c);
                    color: white;
                }

                .btn-reject {
                    background: transparent;
                    color: var(--vscode-errorForeground, #f48771);
                    border: 1px solid var(--vscode-errorForeground, #f48771);
                }
            `}</style>
        </div>
    );
};

export default DiffViewer;
