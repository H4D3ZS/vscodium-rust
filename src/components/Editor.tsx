import React, { useCallback, useEffect, useRef } from 'react';
import MonacoEditor from '@monaco-editor/react';
import type { OnMount, OnChange } from '@monaco-editor/react';
import { useStore } from '../store';
import DiffViewer from './DiffViewer';
import { FileJson, Database } from 'lucide-react';
import InlineEditOverlay from './InlineEditOverlay';
import { invoke, listen } from '../tauri_bridge';
import { sendAgentMessage } from '../agent';

// Map file extension → LSP language id
function getLspLanguageId(path: string): string {
    const ext = path.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'typescriptreact',
        js: 'javascript', jsx: 'javascriptreact', py: 'python',
        go: 'go', java: 'java', c: 'c', cpp: 'cpp', cs: 'csharp',
        json: 'json', md: 'markdown', toml: 'toml', yaml: 'yaml', yml: 'yaml',
    };
    return map[ext] ?? 'plaintext';
}

// Convert LSP URI (file:///...) to a path string
function uriToPath(uri: string): string {
    return uri.replace(/^file:\/\/\//, '').replace(/^file:\/\//, '');
}

// file path → file:// URI
function pathToUri(path: string): string {
    const normalized = path.replace(/\\/g, '/');
    return normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
}

const CTRL_S = 2048 | 49; // KeyMod.CtrlCmd | KeyCode.KeyS
const CTRL_I = 2048 | 40; // KeyMod.CtrlCmd | KeyCode.KeyI

const Editor: React.FC = React.memo(() => {
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
            const lang = activeTab?.language || 'plaintext';

            // Register Completion Item Provider (Intellisense)
            const completionDisposable = monaco.languages.registerCompletionItemProvider(lang, {
                triggerCharacters: ['.', ':', '/', '@'],
                provideCompletionItems: async (model, position) => {
                    if (model.uri.toString() !== pathToUri(activeTab?.path || '')) return { suggestions: [] };
                    try {
                        const res = await invoke<any>('lsp_completion', {
                            uri: model.uri.toString(),
                            line: position.lineNumber - 1,
                            character: position.column - 1
                        });
                        const items = res.items || [];
                        return {
                            suggestions: items.map((item: any) => ({
                                label: item.label,
                                kind: item.kind ?? monaco.languages.CompletionItemKind.Property,
                                insertText: item.insertText || item.label,
                                detail: item.detail,
                                documentation: item.documentation,
                                range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column)
                            }))
                        };
                    } catch (e) { return { suggestions: [] }; }
                }
            });

            // Register Hover Provider
            const hoverDisposable = monaco.languages.registerHoverProvider(lang, {
                provideHover: async (model, position) => {
                    try {
                        const res = await invoke<any>('lsp_hover', {
                            uri: model.uri.toString(),
                            line: position.lineNumber - 1,
                            character: position.column - 1
                        });
                        if (!res || !res.contents) return null;
                        const contents = Array.isArray(res.contents) ? res.contents : [res.contents];
                        return {
                            contents: contents.map((c: any) => {
                                if (typeof c === 'string') return { value: c };
                                return { value: c.value };
                            })
                        };
                    } catch (e) { return null; }
                }
            });

            // Register Definition Provider
            const definitionDisposable = monaco.languages.registerDefinitionProvider(lang, {
                provideDefinition: async (model, position) => {
                    try {
                        const res = await invoke<any>('lsp_goto_definition', {
                            uri: model.uri.toString(),
                            line: position.lineNumber - 1,
                            character: position.column - 1
                        });
                        if (!res) return null;
                        const links = Array.isArray(res) ? res : [res];
                        return links.map((link: any) => ({
                            uri: monaco.Uri.parse(link.uri || link.targetUri),
                            range: {
                                startLineNumber: (link.range?.start?.line ?? link.targetRange?.start?.line ?? 0) + 1,
                                startColumn: (link.range?.start?.character ?? link.targetRange?.start?.character ?? 0) + 1,
                                endLineNumber: (link.range?.end?.line ?? link.targetRange?.end?.line ?? 0) + 1,
                                endColumn: (link.range?.end?.character ?? link.targetRange?.end?.character ?? 0) + 1,
                            }
                        }));
                    } catch (e) { return null; }
                }
            });

            // Register References Provider
            const referencesDisposable = monaco.languages.registerReferenceProvider(lang, {
                provideReferences: async (model, position) => {
                    try {
                        const res = await invoke<any[]>('lsp_find_references', {
                            uri: model.uri.toString(),
                            line: position.lineNumber - 1,
                            character: position.column - 1
                        });
                        if (!res) return [];
                        return res.map((ref: any) => ({
                            uri: monaco.Uri.parse(ref.uri),
                            range: {
                                startLineNumber: (ref.range?.start?.line ?? 0) + 1,
                                startColumn: (ref.range?.start?.character ?? 0) + 1,
                                endLineNumber: (ref.range?.end?.line ?? 0) + 1,
                                endColumn: (ref.range?.end?.character ?? 0) + 1,
                            }
                        }));
                    } catch (e) { return []; }
                }
            });

            // Register Rename Provider
            const renameDisposable = monaco.languages.registerRenameProvider(lang, {
                provideRenameEdits: async (model, position, newName) => {
                    try {
                        const res = await invoke<any>('lsp_rename_symbol', {
                            uri: model.uri.toString(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                            newName
                        });
                        if (!res || !res.documentChanges) return null;
                        return {
                            edits: res.documentChanges.flatMap((dc: any) =>
                                dc.edits.map((e: any) => ({
                                    resource: monaco.Uri.parse(dc.textDocument.uri),
                                    edit: {
                                        range: {
                                            startLineNumber: (e.range.start.line ?? 0) + 1,
                                            startColumn: (e.range.start.character ?? 0) + 1,
                                            endLineNumber: (e.range.end.line ?? 0) + 1,
                                            endColumn: (e.range.end.character ?? 0) + 1,
                                        },
                                        text: e.newText
                                    }
                                }))
                            )
                        };
                    } catch (e) { return null; }
                }
            });

            let inlineTimer: any = null;
            const inlineDisposable = monaco.languages.registerInlineCompletionsProvider(lang, {
                provideInlineCompletions: async (model, position, context, token) => {
                    // Debounce: wait for 350ms of inactivity
                    if (inlineTimer) clearTimeout(inlineTimer);

                    const suggestion = await new Promise<string | null>(resolve => {
                        inlineTimer = setTimeout(async () => {
                            if (token.isCancellationRequested) return resolve(null);

                            const textBefore = model.getValueInRange({
                                startLineNumber: Math.max(1, position.lineNumber - 50),
                                startColumn: 1,
                                endLineNumber: position.lineNumber,
                                endColumn: position.column
                            });
                            const textAfter = model.getValueInRange({
                                startLineNumber: position.lineNumber,
                                startColumn: position.column,
                                endLineNumber: Math.min(model.getLineCount(), position.lineNumber + 20),
                                endColumn: model.getLineMaxColumn(Math.min(model.getLineCount(), position.lineNumber + 20))
                            });

                            if (textBefore.trim().length < 2) return resolve(null);

                            try {
                                const res = await invoke<string>('ai_inline_complete', {
                                    prefix: textBefore,
                                    suffix: textAfter,
                                    language: lang,
                                    filePath: activeTab?.path || '',
                                });
                                resolve(res);
                            } catch (e) { resolve(null); }
                        }, 350);
                    });

                    if (!suggestion || suggestion.trim().length === 0 || token.isCancellationRequested) {
                        return { items: [] };
                    }

                    return {
                        items: [{
                            insertText: suggestion,
                            range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column)
                        }]
                    };
                },
                handleItemDidShow: () => { },
                disposeInlineCompletions: () => { if (inlineTimer) clearTimeout(inlineTimer); }
            });

            // Store disposables to clean up if needed
            (editor as any)._lspDisposables = [
                completionDisposable, hoverDisposable, definitionDisposable,
                referencesDisposable, renameDisposable, inlineDisposable
            ];
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

    // LSP: notify did_open when a file is opened / tab switches
    const lspVersionRef = useRef<Record<string, number>>({});
    useEffect(() => {
        if (!activeTab?.path) return;
        const uri = pathToUri(activeTab.path);
        const langId = getLspLanguageId(activeTab.path);
        const ver = (lspVersionRef.current[uri] ?? 0) + 1;
        lspVersionRef.current[uri] = ver;
        invoke('lsp_did_open', {
            uri,
            languageId: langId,
            version: ver,
            text: activeTab.content ?? '',
        }).catch(() => { /* LSP may not be running */ });
    }, [activeTabId, activeTab?.path]);

    // LSP: notify did_change when content changes (debounced 300 ms)
    const lspChangeTimer = useRef<ReturnType<typeof setTimeout> | null>(null);
    const handleChangeLsp = useCallback((value: string | undefined) => {
        if (!activeTab?.path || value === undefined) return;
        if (lspChangeTimer.current) clearTimeout(lspChangeTimer.current);
        lspChangeTimer.current = setTimeout(() => {
            const uri = pathToUri(activeTab.path);
            const ver = (lspVersionRef.current[uri] ?? 0) + 1;
            lspVersionRef.current[uri] = ver;
            invoke('lsp_did_change', { uri, version: ver, text: value }).catch(() => { });
        }, 300);
    }, [activeTab?.path]);

    // LSP: listen for publishDiagnostics and set Monaco markers
    useEffect(() => {
        const unlisten = listen('lsp-diagnostics', (event) => {
            const { uri, diagnostics } = event.payload;
            import('monaco-editor').then(monaco => {
                // Find the model matching this URI
                const models = monaco.editor.getModels();
                const targetPath = uriToPath(uri);
                const model = models.find(m => {
                    const mp = m.uri.path.replace(/^\//, '').replace(/\\/g, '/');
                    return mp === targetPath.replace(/\\/g, '/') || m.uri.toString() === uri;
                });
                if (!model) return;

                // Map LSP severity (1=error, 2=warning, 3=info, 4=hint) → Monaco MarkerSeverity
                const severityMap: Record<number, number> = { 1: 8, 2: 4, 3: 2, 4: 1 };

                const markers = diagnostics.map(d => ({
                    severity: severityMap[d.severity ?? 1] ?? 8,
                    message: d.message ?? '',
                    startLineNumber: (d.range?.start?.line ?? 0) + 1,
                    startColumn: (d.range?.start?.character ?? 0) + 1,
                    endLineNumber: (d.range?.end?.line ?? 0) + 1,
                    endColumn: (d.range?.end?.character ?? 0) + 1,
                    source: d.source ?? 'lsp',
                    code: d.code?.toString() ?? '',
                }));

                monaco.editor.setModelMarkers(model, 'lsp', markers);
            });
        });
        return () => { unlisten.then(f => f()); };
    }, []);

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
                onChange={(value) => { handleChange(value); handleChangeLsp(value); }}
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

            {activeFilePendingChange && (
                <DiffViewer />
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
});

export default Editor;
