import React, { useEffect, useState } from 'react';
import { DiffEditor } from '@monaco-editor/react';
import { listen } from '@tauri-apps/api/event';
import { motion, AnimatePresence } from '../../lib/motionShim';

interface ShadowUpdate {
    path: string;
    content: string;
    diff: string;
}

interface StagedPatch {
    path: string;
    diff: string;
    originalContent: string;
}

const AgentDiffView: React.FC = () => {
    const [activeUpdate, setActiveUpdate] = useState<ShadowUpdate | null>(null);
    const [stagedPatch, setStagedPatch] = useState<StagedPatch | null>(null);
    const [isVisible, setIsVisible] = useState(false);

    useEffect(() => {
        const unlistenUpdate = listen<ShadowUpdate>('shadow-file-updated', (event) => {
            setActiveUpdate(event.payload);
            setIsVisible(true);
        });

        const unlistenStaged = listen<StagedPatch>('sentient://patch_staged', (event) => {
            setStagedPatch(event.payload);
            setIsVisible(true);
        });

        return () => {
            unlistenUpdate.then(f => f());
            unlistenStaged.then(f => f());
        };
    }, []);

    if (!isVisible) return null;

    const original = stagedPatch?.originalContent || ""; // Logic to fetch original if possible
    const modified = activeUpdate?.content || "";

    return (
        <AnimatePresence>
            <motion.div
                initial={{ y: -100, opacity: 0 }}
                animate={{ y: 0, opacity: 1 }}
                exit={{ y: -100, opacity: 0 }}
                className="agent-diff-container"
            >
                <div className="diff-header">
                    <div className="flex items-center gap-2">
                        <i className="codicon codicon-diff" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 14 }} />
                        <span className="text-sm font-semibold" style={{ color: 'var(--vscode-foreground)' }}>
                            Agent edit: {activeUpdate?.path || stagedPatch?.path}
                        </span>
                    </div>
                    <div className="flex items-center gap-3">
                        <button
                            onClick={() => setIsVisible(false)}
                            className="px-3 py-1 text-xs rounded"
                            style={{ border: '1px solid var(--vscode-inputValidation-errorBorder)', color: 'var(--vscode-errorForeground, #f48771)', background: 'transparent' }}
                        >
                            Dismiss
                        </button>
                    </div>
                </div>

                <div className="diff-editor-wrapper">
                    <DiffEditor
                        height="40vh"
                        language="rust"
                        theme="vs-dark"
                        original={original}
                        modified={modified}
                        options={{
                            renderSideBySide: true,
                            readOnly: true,
                            minimap: { enabled: false },
                            scrollBeyondLastLine: false,
                            fontSize: 12,
                            lineNumbers: 'on',
                            folding: true,
                        }}
                    />
                </div>

                <style>{`
                    .agent-diff-container {
                        position: absolute;
                        top: 0;
                        left: 0;
                        right: 0;
                        z-index: 2000;
                        background: var(--vscode-editor-background);
                        border-bottom: 1px solid var(--vscode-panel-border);
                        box-shadow: 0 4px 16px rgba(0,0,0,0.35);
                    }
                    .diff-header {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        padding: 8px 16px;
                        background: var(--vscode-titleBar-activeBackground);
                        border-bottom: 1px solid var(--vscode-panel-border);
                    }
                    .diff-editor-wrapper {
                        background: var(--vscode-editor-background);
                    }
                `}</style>
            </motion.div>
        </AnimatePresence>
    );
};

export default AgentDiffView;
