import React, { useEffect, useState } from 'react';
import { DiffEditor } from '@monaco-editor/react';
import { listen } from '@tauri-apps/api/event';
import { Sparkles, Check, X, Disc } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';

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
                        <Sparkles className="text-cyan-400 animate-pulse" size={16} />
                        <span className="text-sm font-semibold text-gray-200">
                            Agentic Edit: {activeUpdate?.path || stagedPatch?.path}
                        </span>
                    </div>
                    <div className="flex items-center gap-3">
                        <button
                            onClick={() => setIsVisible(false)}
                            className="px-3 py-1 text-xs border border-red-500/50 text-red-400 rounded hover:bg-red-500/10 transition-colors flex items-center gap-1"
                        >
                            <X size={14} /> Discard
                        </button>
                        <button
                            className="px-3 py-1 text-xs bg-cyan-600 text-white rounded hover:bg-cyan-500 transition-colors flex items-center gap-1 font-bold shadow-lg shadow-cyan-900/20"
                        >
                            <Check size={14} /> Commit Changes
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
                        background: #1e1e1e;
                        border-bottom: 1px solid #333;
                        box-shadow: 0 10px 30px rgba(0,0,0,0.5);
                    }
                    .diff-header {
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        padding: 8px 16px;
                        background: #252526;
                        border-bottom: 1px solid #333;
                    }
                    .diff-editor-wrapper {
                        background: #1e1e1e;
                    }
                `}</style>
            </motion.div>
        </AnimatePresence>
    );
};

export default AgentDiffView;
