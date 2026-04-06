import React, { useCallback, useEffect, useRef } from 'react';
import MonacoEditor from '@monaco-editor/react';
import type { OnMount, OnChange } from '@monaco-editor/react';
import { useStore } from '../store';
import DiffViewer from './DiffViewer';
import { FileJson, Database } from 'lucide-react';
import InlineEditOverlay from './InlineEditOverlay';
import { invoke } from '../tauri_bridge';
import { sendAgentMessage } from '../agent';

const CTRL_S = 2048 | 49; // KeyMod.CtrlCmd | KeyCode.KeyS
const CTRL_I = 2048 | 40; // KeyMod.CtrlCmd | KeyCode.KeyI

const Editor: React.FC = () => {
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const updateTabContent = useStore(state => state.updateTabContent);
    const saveActiveFile = useStore(state => state.saveActiveFile);
    const theme = useStore(state => state.theme);
    const setTheme = useStore(state => state.setTheme);
    const setActiveEditorPath = useStore(state => state.setActiveEditorPath);
    const setVisualLabData = useStore(state => state.setVisualLabData);
    const toggleVisualLab = useStore(state => state.toggleVisualLab);
    const setVisualLabMode = useStore(state => state.setVisualLabMode);

    const [isInlineEditOpen, setIsInlineEditOpen] = React.useState(false);
    const [inlineEditPosition, setInlineEditPosition] = React.useState({ top: 0, left: 0 });

    const activeTab = tabs.find(t => t.id === activeTabId) ?? null;

    const editorRef = useRef<any>(null);

    // Update active editor path in store whenever tab changes
    useEffect(() => {
        if (activeTab?.path) {
            setActiveEditorPath(activeTab.path);
        }
    }, [activeTabId, activeTab?.path, setActiveEditorPath]);

    const handleMount: OnMount = useCallback((editor) => {
        editorRef.current = editor;
        editor.addCommand(CTRL_S, () => saveActiveFile());

        editor.addCommand(CTRL_I, () => {
            const position = editor.getPosition();
            if (position) {
                const pixelCoords = editor.getScrolledVisiblePosition(position);
                if (pixelCoords) {
                    setInlineEditPosition({ top: pixelCoords.top + 20, left: pixelCoords.left });
                    setIsInlineEditOpen(true);
                }
            }
        });

        // Register Inline Completions (Ghost Text) for 'Tab' modality
        import('monaco-editor').then(monaco => {
            monaco.languages.registerInlineCompletionsProvider(activeTab?.language || 'plaintext', {
                provideInlineCompletions: async (model, position) => {
                    // Get the text before cursor for context
                    const textBefore = model.getValueInRange({
                        startLineNumber: Math.max(1, position.lineNumber - 10),
                        startColumn: 1,
                        endLineNumber: position.lineNumber,
                        endColumn: position.column
                    });
                    const textAfter = model.getValueInRange({
                        startLineNumber: position.lineNumber,
                        startColumn: position.column,
                        endLineNumber: Math.min(model.getLineCount(), position.lineNumber + 5),
                        endColumn: model.getLineMaxColumn(Math.min(model.getLineCount(), position.lineNumber + 5))
                    });

                    // Don't call on very short context
                    if (textBefore.trim().length < 5) return { items: [] };

                    try {
                        const suggestion = await invoke<string>('ai_inline_complete', {
                            prefix: textBefore,
                            suffix: textAfter,
                            language: activeTab?.language || 'plaintext',
                            filePath: activeTab?.path || '',
                        });

                        if (!suggestion || suggestion.trim().length === 0) return { items: [] };

                        return {
                            items: [{
                                insertText: suggestion,
                                range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column)
                            }]
                        };
                    } catch (e) {
                        // Silently fail — inline completions are best-effort
                        return { items: [] };
                    }
                },
                handleItemDidShow: () => { },
                disposeInlineCompletions: () => { }
            });
        });
    }, [saveActiveFile, activeTab?.language]);

    const handleChange: OnChange = useCallback((value) => {
        if (activeTabId && value !== undefined) {
            updateTabContent(activeTabId, value);
        }
    }, [activeTabId, updateTabContent]);

    const pendingChanges = useStore(state => state.pendingChanges);
    const activeFilePendingChange = pendingChanges.find(c => c.path === activeTab?.path);

    // When theme changes in store, ensure it's applied correctly to monaco instance
    useEffect(() => {
        import('@monaco-editor/react').then(({ loader }) => {
            loader.init().then(monaco => {
                if (theme) {
                    try {
                        // Check if it's a custom theme name
                        if (theme.startsWith('vscode-theme-')) {
                            // If registration is known to be successful (or at least attempted), apply it.
                            // Monaco might throw if the theme name doesn't exist yet.
                            monaco.editor.setTheme(theme);
                        } else {
                            monaco.editor.setTheme(theme);
                        }
                    } catch (e) {
                        console.warn("[Editor] Custom theme not ready, falling back to vs-dark");
                        monaco.editor.setTheme('vs-dark');
                    }
                }
            });
        });
    }, [theme]);

    // When switching tabs, sync the editor value
    useEffect(() => {
        if (editorRef.current && activeTab) {
            const currentValue = editorRef.current.getValue();
            // If there's a pending change, show the new content
            const targetContent = activeFilePendingChange ? activeFilePendingChange.newContent : activeTab.content;
            if (currentValue !== targetContent) {
                editorRef.current.setValue(targetContent);
            }
        }
    }, [activeTabId, activeFilePendingChange]);

    if (!activeTab) {
        return null;
    }

    return (
        <div style={{ position: 'relative', height: '100%', width: '100%' }}>
            <MonacoEditor
                height="100%"
                width="100%"
                theme={theme}
                language={activeTab.language}
                value={activeFilePendingChange ? activeFilePendingChange.newContent : activeTab.content}
                onMount={handleMount}
                onChange={handleChange}
                loading={<div className="editor-loading" style={{ background: 'var(--vscode-editor-background)', color: 'var(--vscode-editor-foreground)', height: '100%', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '12px', opacity: 0.5 }}>Loading IDE Editor Assets...</div>}
                options={{
                    fontSize: 13,
                    fontFamily: 'var(--font-mono)',
                    lineNumbers: 'on',
                    lineNumbersMinChars: 3,
                    glyphMargin: false,
                    folding: true,
                    lineDecorationsWidth: 10,
                    minimap: { enabled: true },
                    scrollBeyondLastLine: false,
                    wordWrap: 'off',
                    tabSize: 4,
                    insertSpaces: true,
                    automaticLayout: true,
                    renderWhitespace: 'selection',
                    smoothScrolling: true,
                    cursorBlinking: 'smooth',
                    cursorSmoothCaretAnimation: 'on',
                    bracketPairColorization: { enabled: true },
                }}
            />

            {/* Visual Lab Quick Action */}
            {(activeTab.language === 'json' || activeTab.path.endsWith('.json') || activeTab.path.endsWith('.sql') || activeTab.path.endsWith('.mongodb')) && (
                <button
                    onClick={() => {
                        setVisualLabData(activeTab.content);
                        const mode = activeTab.path.endsWith('.sql') ? 'erd' : 'json';
                        setVisualLabMode(mode);
                        toggleVisualLab(true);
                    }}
                    style={{
                        position: 'absolute',
                        top: '10px',
                        right: '40px',
                        zIndex: 100,
                        background: activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.15)' : 'rgba(168, 85, 247, 0.15)',
                        border: `1px solid ${activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.3)' : 'rgba(168, 85, 247, 0.3)'}`,
                        borderRadius: '6px',
                        color: activeTab.path.endsWith('.sql') ? '#10b981' : '#c084fc',
                        padding: '4px 10px',
                        fontSize: '11px',
                        fontWeight: 600,
                        display: 'flex',
                        alignItems: 'center',
                        gap: '6px',
                        cursor: 'pointer',
                        backdropFilter: 'blur(4px)',
                        transition: 'all 0.2s'
                    }}
                    onMouseEnter={(e) => {
                        e.currentTarget.style.background = activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.25)' : 'rgba(168, 85, 247, 0.25)';
                        e.currentTarget.style.borderColor = activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.5)' : 'rgba(168, 85, 247, 0.5)';
                    }}
                    onMouseLeave={(e) => {
                        e.currentTarget.style.background = activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.15)' : 'rgba(168, 85, 247, 0.15)';
                        e.currentTarget.style.borderColor = activeTab.path.endsWith('.sql') ? 'rgba(16, 185, 129, 0.3)' : 'rgba(168, 85, 247, 0.3)';
                    }}
                >
                    {activeTab.path.endsWith('.sql') ? <Database size={12} /> : <FileJson size={12} />}
                    {activeTab.path.endsWith('.sql') ? 'Visualize Schema' : 'Visualize Content'}
                </button>
            )}

            {isInlineEditOpen && (
                <InlineEditOverlay
                    position={inlineEditPosition}
                    onClose={() => setIsInlineEditOpen(false)}
                    onSubmit={async (prompt) => {
                        setIsInlineEditOpen(false);

                        const editor = editorRef.current;
                        if (!editor) return;

                        // Get selected text or surrounding context for the AI
                        const selection = editor.getSelection();
                        let selectedText = '';
                        if (selection && !selection.isEmpty()) {
                            selectedText = editor.getModel()?.getValueInRange(selection) || '';
                        } else {
                            // Get the current line and nearby context
                            const pos = editor.getPosition();
                            if (pos) {
                                const startLine = Math.max(1, pos.lineNumber - 5);
                                const endLine = Math.min(editor.getModel()?.getLineCount() || pos.lineNumber, pos.lineNumber + 5);
                                selectedText = editor.getModel()?.getValueInRange({
                                    startLineNumber: startLine, startColumn: 1,
                                    endLineNumber: endLine, endColumn: editor.getModel()?.getLineMaxColumn(endLine) || 1
                                }) || '';
                            }
                        }

                        // Build the full prompt with context and send to agent
                        const fullPrompt = `INLINE EDIT REQUEST (Ctrl+I).\nFile: ${activeTab?.path || 'unknown'}\nLanguage: ${activeTab?.language || 'unknown'}\n\nUser instruction: ${prompt}\n\nCode context:\n\`\`\`${activeTab?.language || ''}\n${selectedText}\n\`\`\`\n\nApply the edit using the replace_file_content or patch_file_content tool. Only modify what's needed.`;

                        const store = useStore.getState();
                        store.addAgentMessage('user', `[Inline Edit] ${prompt}`);
                        store.addAgentMessage('assistant', '');
                        store.setIsAgentThinking(true);

                        try {
                            await sendAgentMessage(fullPrompt, () => { });
                        } catch (error: any) {
                            store.setIsAgentThinking(false);
                            store.updateLastAgentMessage(`**Error:** ${error.message || error}`);
                        }
                    }}
                />
            )}
        </div>
    );
};

export default Editor;
