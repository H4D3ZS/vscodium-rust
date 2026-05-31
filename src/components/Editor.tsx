import React, { useCallback, useEffect, useRef } from 'react';
import MonacoEditor from '@monaco-editor/react';
import type { OnMount, OnChange } from '@monaco-editor/react';
import { useStore } from '../store';
import DiffViewer from './DiffViewer';
import { FileJson, Database } from 'lucide-react';
import InlineEditOverlay from './InlineEditOverlay';
import Breadcrumbs from './Breadcrumbs';
import MarkdownPreview from './MarkdownPreview';
import WelcomePage from './WelcomePage';
import PredictiveEditOverlay from './PredictiveEditOverlay';
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

const CTRL_S = 2048 | 49;    // KeyMod.CtrlCmd | KeyCode.KeyS
const CTRL_I = 2048 | 40;    // KeyMod.CtrlCmd | KeyCode.KeyI
const CTRL_K = 2048 | 41;    // KeyMod.CtrlCmd | KeyCode.KeyK  (Cursor inline edit)
const CTRL_L = 2048 | 42;    // KeyMod.CtrlCmd | KeyCode.KeyL  (Cursor chat with selection)
const CTRL_D = 2048 | 33;    // KeyMod.CtrlCmd | KeyCode.KeyD  (multi-cursor: add next occurrence)
const CTRL_ALT_B = 2048 | 512 | 32; // CtrlCmd+Alt+B  (git blame toggle)
const ALT_LEFT = 512 | 15;   // KeyMod.Alt | KeyCode.LeftArrow
const ALT_RIGHT = 512 | 17;  // KeyMod.Alt | KeyCode.RightArrow

interface EditorProps {
    tabId?: string; // if provided, override activeTabId (for split pane)
}

const Editor: React.FC<EditorProps> = React.memo(({ tabId: forcedTabId }) => {
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const updateTabContent = useStore(state => state.updateTabContent);
    const saveActiveFile = useStore(state => state.saveActiveFile);
    const theme = useStore(state => state.theme);
    const setTheme = useStore(state => state.setTheme);
    const setActiveEditorPath = useStore(state => state.setActiveEditorPath);
    const setVisualLabData = useStore(state => state.setVisualLabData);
    const toggleVisualLab = useStore(state => state.toggleVisualLab);
    const isGitBlameVisible = useStore(state => (state as any).isGitBlameVisible ?? false);
    const toggleGitBlame = useStore(state => (state as any).toggleGitBlame);
    const setVisualLabMode = useStore(state => state.setVisualLabMode);

    const [isInlineEditOpen, setIsInlineEditOpen] = React.useState(false);
    const [inlineEditPosition, setInlineEditPosition] = React.useState({ top: 0, left: 0 });
    const [inlineEditSelection, setInlineEditSelection] = React.useState<{ text: string; startLine: number; endLine: number } | null>(null);

    const effectiveTabId = forcedTabId ?? activeTabId;
    const activeTab = tabs.find(t => t.id === effectiveTabId) ?? null;

    const editorRef = useRef<any>(null);
    const autoSaveTimer = useRef<ReturnType<typeof setTimeout> | null>(null);

    // Update active editor path in store whenever tab changes (only for primary pane)
    useEffect(() => {
        if (!forcedTabId && activeTab?.path) {
            setActiveEditorPath(activeTab.path);
        }
    }, [effectiveTabId, activeTab?.path, setActiveEditorPath, forcedTabId]);

    // Enforce hard cap of 12 active Monaco models to save RAM (~5MB each = 60MB max)
    useEffect(() => {
        if (editorRef.current) {
            import('monaco-editor').then((monaco) => {
                const models = monaco.editor.getModels();
                if (models.length > 12) {
                    const openTabUris = new Set(tabs.map(t => pathToUri(t.path)));
                    const inactiveModels = models.filter(m => !openTabUris.has(m.uri.toString()));
                    
                    if (inactiveModels.length > 0) {
                        const toEvictCount = models.length - 12;
                        const toEvict = inactiveModels.slice(0, toEvictCount);
                        toEvict.forEach(m => {
                            m.dispose();
                            console.log(`[Monaco Eviction] Disposed inactive model: ${m.uri.toString()} to conserve RAM.`);
                        });
                    }
                }
            }).catch(console.error);
        }
    }, [activeTab?.path, tabs]);

    const handleMount: OnMount = useCallback((editor, monaco) => {
        editorRef.current = editor;
        // Announce the live editor so add-on overlays (PredictiveEditOverlay,
        // future inline widgets) can attach onDidChangeModelContent listeners
        // without us having to thread the handle through React props.
        window.dispatchEvent(new CustomEvent('editor:registered', { detail: { editor, monaco } }));
        editor.addCommand(CTRL_S, () => saveActiveFile());

        // ── Tab to accept Ghost Text (inline AI completion) ────────────────
        editor.addCommand(monaco.KeyCode.Tab, () => {
            // Check if there's an inline completion to accept
            const inlineSuggestions = (editor as any).getInlineCompletions?.();
            if (inlineSuggestions && inlineSuggestions.items?.length > 0) {
                // Accept the first inline suggestion
                editor.trigger('source', 'editor.action.inlineCompletions.accept', null);
                return;
            }
            // Otherwise, let Tab do its normal thing
            return false;
        });

        // Shared logic: open inline edit with selection context
        const openInlineEdit = () => {
            const position = editor.getPosition();
            const selection = editor.getSelection();
            let selText = '';
            let startLine = position?.lineNumber ?? 1;
            let endLine = position?.lineNumber ?? 1;
            if (selection && !selection.isEmpty()) {
                selText = editor.getModel()?.getValueInRange(selection) || '';
                startLine = selection.startLineNumber;
                endLine = selection.endLineNumber;
            } else if (position) {
                // Default: grab 10 lines around cursor
                const model = editor.getModel();
                startLine = Math.max(1, position.lineNumber - 3);
                endLine = Math.min(model?.getLineCount() ?? position.lineNumber, position.lineNumber + 6);
                selText = model?.getValueInRange({
                    startLineNumber: startLine, startColumn: 1,
                    endLineNumber: endLine, endColumn: model.getLineMaxColumn(endLine),
                }) || '';
            }
            setInlineEditSelection({ text: selText, startLine, endLine });
            if (position) {
                const pixelCoords = editor.getScrolledVisiblePosition(position);
                if (pixelCoords) {
                    setInlineEditPosition({ top: pixelCoords.top + 20, left: pixelCoords.left });
                }
            }
            setIsInlineEditOpen(true);
        };

        // Alt+Left/Right = tab history navigation
        editor.addCommand(ALT_LEFT, () => useStore.getState().navigateBack?.());
        editor.addCommand(ALT_RIGHT, () => useStore.getState().navigateForward?.());

        // Ctrl+D = add next occurrence to selection (multi-cursor)
        editor.addCommand(CTRL_D, () => {
            editor.trigger('keyboard', 'editor.action.addSelectionToNextFindMatch', null);
        });

        // Ctrl+Alt+B = toggle git blame gutter
        editor.addCommand(CTRL_ALT_B, () => {
            (useStore.getState() as any).toggleGitBlame?.();
        });

        // Track cursor position → emit for breadcrumb symbol context and StatusBar
        editor.onDidChangeCursorPosition((e) => {
            const selection = editor.getSelection();
            const selectionLength = selection && !selection.isEmpty()
                ? editor.getModel()?.getValueInRange(selection)?.length ?? 0
                : 0;
            window.dispatchEvent(new CustomEvent('editor:cursor-position', {
                detail: { line: e.position.lineNumber, column: e.position.column, selectionLength }
            }));
        });

        // StatusBar: Go to line
        const gotoHandler = (ev: Event) => {
            const { line } = (ev as CustomEvent).detail ?? {};
            if (typeof line === 'number') {
                editor.revealLineInCenter(line);
                editor.setPosition({ lineNumber: line, column: 1 });
                editor.focus();
            }
        };
        window.addEventListener('editor:goto-line', gotoHandler);

        // StatusBar: Set indentation
        const indentHandler = (ev: Event) => {
            const { insertSpaces } = (ev as CustomEvent).detail ?? {};
            editor.getModel()?.updateOptions({ insertSpaces });
        };
        window.addEventListener('editor:set-indent', indentHandler);

        const tabSizeHandler = (ev: Event) => {
            const { size } = (ev as CustomEvent).detail ?? {};
            if (typeof size === 'number') editor.getModel()?.updateOptions({ tabSize: size });
        };
        window.addEventListener('editor:set-tab-size', tabSizeHandler);

        // Ctrl+K = Cursor-style inline edit (primary binding)
        editor.addCommand(CTRL_K, openInlineEdit);
        // Ctrl+I = legacy binding, same action
        editor.addCommand(CTRL_I, openInlineEdit);

        // Ctrl+L = send selected code to chat (like Cursor)
        editor.addCommand(CTRL_L, () => {
            const selection = editor.getSelection();
            const selText = selection && !selection.isEmpty()
                ? editor.getModel()?.getValueInRange(selection) || ''
                : editor.getModel()?.getValue() || '';
            if (selText) {
                const store = useStore.getState();
                store.attachFile?.({
                    id: `sel-${Date.now()}`,
                    type: 'file' as any,
                    name: `${activeTab?.path?.split(/[/\\]/).pop() ?? 'selection'} (selected)`,
                    path: activeTab?.path ?? '',
                    data: selText,
                });
                // Open right sidebar if closed
                if (!store.isRightSidebarOpen) store.toggleRightSidebar?.();
            }
        });

        // ── Phase 4: Kiro Diagnose ─────────────────────────────────────
        editor.addAction({
            id: 'kiro.diagnose',
            label: 'Diagnose with Kiro',
            contextMenuGroupId: 'navigation',
            contextMenuOrder: 1.5,
            run: (ed) => {
                const pos = ed.getPosition();
                if (!pos) return;
                const model = ed.getModel();
                if (!model) return;
                
                // Get the global monaco object
                import('monaco-editor').then(monaco => {
                    const markers = monaco.editor.getModelMarkers({ resource: model.uri });
                    const marker = markers.find(m => m.startLineNumber <= pos.lineNumber && m.endLineNumber >= pos.lineNumber);
                    if (marker) {
                        const markerRange = {
                            startLineNumber: marker.startLineNumber,
                            startColumn: marker.startColumn,
                            endLineNumber: marker.endLineNumber,
                            endColumn: marker.endColumn
                        };
                        const text = model.getValueInRange(markerRange);
                        const prompt = `Diagnose and fix this error: "${marker.message}" on line ${marker.startLineNumber} in ${activeTab?.path}\n\nCode context:\n\`\`\`\n${text}\n\`\`\``;
                        useStore.getState().runBackgroundAgent(prompt).catch(console.error);
                    } else {
                        // Fallback if no specific marker under cursor: just diagnose the line
                        const text = model.getLineContent(pos.lineNumber);
                        const prompt = `Diagnose potential issues on line ${pos.lineNumber} in ${activeTab?.path}\n\nCode context:\n\`\`\`\n${text}\n\`\`\``;
                        useStore.getState().runBackgroundAgent(prompt).catch(console.error);
                    }
                });
            }
        });

        // Register Inline Completions (Ghost Text) for 'Tab' modality
        import('monaco-editor').then(monaco => {
            const lang = activeTab?.language || 'plaintext';

            // LSP kind → Monaco CompletionItemKind
            const lspKindToMonaco = (k: number) => {
                const map: Record<number, number> = {
                    1: 17, 2: 0, 3: 1, 4: 2, 5: 3, 6: 4, 7: 6, 8: 7, 9: 8,
                    10: 9, 12: 11, 13: 12, 14: 13, 15: 14, 16: 15, 17: 16, 18: 17, 25: 24,
                };
                return map[k] ?? 9;
            };

            // Use pathToUri to get the canonical URI the LSP server knows about
            const getFileUri = () => pathToUri(activeTab?.path || '');

            // ── Completion (Intellisense) ──────────────────────────────────
            const completionDisposable = monaco.languages.registerCompletionItemProvider(lang, {
                triggerCharacters: ['.', ':', '(', '<', '"', "'", '/', '@', '#'],
                provideCompletionItems: async (model, position) => {
                    try {
                        const res = await invoke<any>('lsp_completion', {
                            uri: getFileUri(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                        });
                        const word = model.getWordUntilPosition(position);
                        const range = {
                            startLineNumber: position.lineNumber,
                            endLineNumber: position.lineNumber,
                            startColumn: word.startColumn,
                            endColumn: word.endColumn,
                        };
                        return {
                            suggestions: (res?.items ?? []).map((item: any) => ({
                                label: item.label,
                                kind: lspKindToMonaco(item.kind ?? 1),
                                insertText: item.insertText ?? item.textEdit?.newText ?? item.label,
                                insertTextRules: item.insertTextFormat === 2 ? 4 : 0,
                                detail: item.detail ?? '',
                                documentation: { value: item.documentation?.value ?? item.documentation ?? '' },
                                sortText: item.sortText ?? item.label,
                                filterText: item.filterText ?? item.label,
                                preselect: item.preselect ?? false,
                                range,
                            })),
                            incomplete: false,
                        };
                    } catch { return { suggestions: [] }; }
                },
            });

            // ── Hover (doc on mouseover) ───────────────────────────────────
            const hoverDisposable = monaco.languages.registerHoverProvider(lang, {
                provideHover: async (_model, position) => {
                    try {
                        const res = await invoke<any>('lsp_hover', {
                            uri: getFileUri(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                        });
                        if (!res) return null;
                        const raw = res.contents ?? res;
                        const contents = Array.isArray(raw) ? raw : [raw];
                        const mdParts = contents.map((c: any) => ({
                            value: typeof c === 'string' ? c : (c.value ?? String(c)),
                        }));
                        if (!mdParts.length || !mdParts[0].value) return null;
                        const range = res.range ? {
                            startLineNumber: res.range.start.line + 1,
                            startColumn: res.range.start.character + 1,
                            endLineNumber: res.range.end.line + 1,
                            endColumn: res.range.end.character + 1,
                        } : undefined;
                        return { contents: mdParts, range };
                    } catch { return null; }
                },
            });

            // ── Go To Definition ───────────────────────────────────────────
            const definitionDisposable = monaco.languages.registerDefinitionProvider(lang, {
                provideDefinition: async (_model, position) => {
                    try {
                        const res = await invoke<any>('lsp_goto_definition', {
                            uri: getFileUri(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                        });
                        if (!res) return [];
                        const locs = Array.isArray(res) ? res : [res];
                        return locs.map((loc: any) => {
                            const locUri = loc.uri ?? loc.targetUri ?? '';
                            const r = loc.range ?? loc.targetSelectionRange ?? {};
                            return {
                                uri: monaco.Uri.parse(locUri.startsWith('file:') ? locUri : `file:///${locUri.replace(/\\/g, '/')}`),
                                range: {
                                    startLineNumber: (r.start?.line ?? 0) + 1,
                                    startColumn: (r.start?.character ?? 0) + 1,
                                    endLineNumber: (r.end?.line ?? 0) + 1,
                                    endColumn: (r.end?.character ?? 0) + 1,
                                },
                            };
                        });
                    } catch { return []; }
                },
            });

            // ── Find All References ────────────────────────────────────────
            const referencesDisposable = monaco.languages.registerReferenceProvider(lang, {
                provideReferences: async (_model, position) => {
                    try {
                        const res = await invoke<any[]>('lsp_find_references', {
                            uri: getFileUri(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                        });
                        if (!res) return [];
                        return res.map((ref: any) => ({
                            uri: monaco.Uri.parse(ref.uri.startsWith('file:') ? ref.uri : `file:///${ref.uri.replace(/\\/g, '/')}`),
                            range: {
                                startLineNumber: (ref.range?.start?.line ?? 0) + 1,
                                startColumn: (ref.range?.start?.character ?? 0) + 1,
                                endLineNumber: (ref.range?.end?.line ?? 0) + 1,
                                endColumn: (ref.range?.end?.character ?? 0) + 1,
                            },
                        }));
                    } catch { return []; }
                },
            });

            // ── Rename Symbol (F2) ─────────────────────────────────────────
            const renameDisposable = monaco.languages.registerRenameProvider(lang, {
                provideRenameEdits: async (_model, position, newName) => {
                    try {
                        const res = await invoke<any>('lsp_rename_symbol', {
                            uri: getFileUri(),
                            line: position.lineNumber - 1,
                            character: position.column - 1,
                            newName,
                        });
                        if (!res) return null;
                        // Handle both documentChanges and changes formats
                        const edits: any[] = [];
                        if (res.documentChanges) {
                            for (const dc of res.documentChanges) {
                                const dcUri = monaco.Uri.parse(dc.textDocument?.uri ?? dc.uri ?? '');
                                for (const e of (dc.edits ?? [])) {
                                    edits.push({
                                        resource: dcUri, textEdit: {
                                            range: {
                                                startLineNumber: (e.range.start.line ?? 0) + 1,
                                                startColumn: (e.range.start.character ?? 0) + 1,
                                                endLineNumber: (e.range.end.line ?? 0) + 1,
                                                endColumn: (e.range.end.character ?? 0) + 1,
                                            },
                                            text: e.newText,
                                        }
                                    });
                                }
                            }
                        } else if (res.changes) {
                            for (const [u, fileEdits] of Object.entries(res.changes as Record<string, any[]>)) {
                                const dcUri = monaco.Uri.parse(u);
                                for (const e of fileEdits) {
                                    edits.push({
                                        resource: dcUri, textEdit: {
                                            range: {
                                                startLineNumber: (e.range.start.line ?? 0) + 1,
                                                startColumn: (e.range.start.character ?? 0) + 1,
                                                endLineNumber: (e.range.end.line ?? 0) + 1,
                                                endColumn: (e.range.end.character ?? 0) + 1,
                                            },
                                            text: e.newText,
                                        }
                                    });
                                }
                            }
                        }
                        return { edits };
                    } catch { return null; }
                },
            });

            // ── Ghost Text / Inline Completions (Tab to accept) ────────────
            // Respects tabPredictionEnabled / voidGlobalSettings.enableAutocomplete.
            // Uses the per-feature Autocomplete model when configured.
            let inlineTimer: any = null;
            const inlineDisposable = monaco.languages.registerInlineCompletionsProvider(lang, {
                provideInlineCompletions: async (model, position, _ctx, token) => {
                    // Check kill switches
                    const st = (window as any).useStore?.getState?.() || {};
                    const autocompleteOn = st.tabPredictionEnabled !== false
                        && st.voidGlobalSettings?.enableAutocomplete !== false;
                    if (!autocompleteOn) return { items: [] };

                    if (inlineTimer) clearTimeout(inlineTimer);
                    const suggestion = await new Promise<string | null>(resolve => {
                        inlineTimer = setTimeout(async () => {
                            if (token.isCancellationRequested) return resolve(null);
                            const textBefore = model.getValueInRange({
                                startLineNumber: Math.max(1, position.lineNumber - 40),
                                startColumn: 1,
                                endLineNumber: position.lineNumber,
                                endColumn: position.column,
                            });
                            const textAfter = model.getValueInRange({
                                startLineNumber: position.lineNumber,
                                startColumn: position.column,
                                endLineNumber: Math.min(model.getLineCount(), position.lineNumber + 15),
                                endColumn: model.getLineMaxColumn(Math.min(model.getLineCount(), position.lineNumber + 15)),
                            });
                            if (textBefore.trim().length < 3) return resolve(null);
                            // Void: use per-feature Autocomplete model if configured
                            const acSel = st.modelSelectionOfFeature?.['Autocomplete'];
                            try {
                                const res = await invoke<string>('ai_inline_complete', {
                                    prefix: textBefore,
                                    suffix: textAfter,
                                    language: lang,
                                    filePath: activeTab?.path || '',
                                    ...(acSel?.modelName ? { model: acSel.modelName, provider: acSel.providerName } : {}),
                                });
                                resolve(res ?? null);
                            } catch { resolve(null); }
                        }, 600); // 600ms debounce — avoids firing on every keystroke
                    });
                    if (!suggestion?.trim() || token.isCancellationRequested) return { items: [] };
                    return {
                        items: [{
                            insertText: suggestion,
                            range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column),
                        }],
                    };
                },
                handleItemDidShow: () => { },
                disposeInlineCompletions: () => { if (inlineTimer) clearTimeout(inlineTimer); },
            });

            // ── Fix with AIRI action (right-click or Ctrl+.) ───────────────
            const fixAction = editor.addAction({
                id: 'airi-fix-with-ai',
                label: '✦ Fix with AIRI',
                contextMenuGroupId: 'navigation',
                contextMenuOrder: 0.5,
                run: async (ed) => {
                    const pos = ed.getPosition();
                    const model = ed.getModel();
                    if (!pos || !model) return;
                    const markers = (monaco.editor.getModelMarkers({ resource: model.uri }))
                        .filter((m: any) => m.severity >= 4); // errors + warnings
                    const errMsg = markers.length
                        ? markers.map((m: any) => `Line ${m.startLineNumber}: [${m.source}] ${m.message}`).join('\n')
                        : `Code at line ${pos.lineNumber}`;
                    const code = model.getValue();
                    const store = (window as any).useStore?.getState?.();
                    if (!store) return;
                    store.addAgentMessage('user', `[Fix with AIRI]\n${errMsg}`);
                    store.addAgentMessage('assistant', '');
                    store.setIsAgentThinking(true);
                    store.setActivePanelTab?.('TERMINAL');
                    import('../agent').then(({ sendAgentMessage }) => {
                        sendAgentMessage(
                            `Fix the following issues in \`${activeTab?.path || 'the file'}\`:\n\n${errMsg}\n\nFile content (${code.split('\n').length} lines):\n\`\`\`${lang}\n${code.slice(0, 8000)}\n\`\`\`\n\nUse search_replace_edit or patch_file_content to apply the fix directly. Do not just describe it.`,
                            () => { }
                        ).finally(() => store.setIsAgentThinking(false));
                    });
                },
            });

            // ── Windsurf-style slash command editor actions ───────────────
            // These appear in the right-click context menu and can be triggered
            // via the command palette. Each pre-fills the chat with the selected
            // text + a command directive (matching the / commands in the sidebar).
            const agentSlashAction = (id: string, label: string, command: string, order: number) =>
                editor.addAction({
                    id,
                    label,
                    contextMenuGroupId: 'airi-slash',
                    contextMenuOrder: order,
                    run: (ed) => {
                        const selection = ed.getSelection();
                        const model = ed.getModel();
                        const selText = selection && !selection.isEmpty() && model
                            ? model.getValueInRange(selection)
                            : model?.getValue()?.slice(0, 4000) ?? '';
                        const filePath = activeTab?.path ?? '';
                        const store = (window as any).useStore?.getState?.();
                        if (!store) return;
                        const prompt = `${command}\n\nFile: \`${filePath}\`\n\`\`\`${lang}\n${selText}\n\`\`\``;
                        store.addAgentMessage('user', prompt);
                        store.addAgentMessage('assistant', '');
                        store.setIsAgentThinking(true);
                        if (!store.isRightSidebarOpen) store.toggleRightSidebar?.();
                        import('../agent').then(({ sendAgentMessage }) => {
                            sendAgentMessage(prompt).finally(() => store.setIsAgentThinking(false));
                        });
                    },
                });
            agentSlashAction('airi.explain', '✦ /explain — Explain this code', '/explain the following code in detail', 1);
            agentSlashAction('airi.refactor', '✦ /refactor — Refactor this code', '/refactor the following code for clarity and performance', 2);
            agentSlashAction('airi.test', '✦ /test — Generate tests', '/test generate comprehensive unit tests for the following code', 3);
            agentSlashAction('airi.document', '✦ /document — Add documentation', '/document add inline documentation to the following code', 4);

            // ── Code Lens ────────────────────────────────────────────────
            const codeLensDisposable = monaco.languages.registerCodeLensProvider(lang, {
                provideCodeLenses: async (model) => {
                    try {
                        const res = await invoke<any>('lsp_code_lens', { uri: getFileUri() });
                        if (!res || !Array.isArray(res)) return { lenses: [], dispose: () => { } };
                        const lenses = res.map((cl: any) => ({
                            range: {
                                startLineNumber: (cl.range?.start?.line ?? 0) + 1,
                                startColumn: (cl.range?.start?.character ?? 0) + 1,
                                endLineNumber: (cl.range?.end?.line ?? 0) + 1,
                                endColumn: (cl.range?.end?.character ?? 0) + 1,
                            },
                            command: cl.command ? {
                                id: cl.command.command || '',
                                title: cl.command.title || '',
                                arguments: cl.command.arguments,
                            } : { id: '', title: cl.data?.toString() ?? '' },
                        }));
                        return { lenses, dispose: () => { } };
                    } catch { return { lenses: [], dispose: () => { } }; }
                },
                resolveCodeLens: (_, codeLens) => Promise.resolve(codeLens),
            });

            // ── Format Document (Shift+Alt+F) ────────────────────────────
            const formatDisposable = monaco.languages.registerDocumentFormattingEditProvider(lang, {
                provideDocumentFormattingEdits: async (model) => {
                    try {
                        const res = await invoke<any[]>('lsp_format_document', { uri: getFileUri() });
                        if (!Array.isArray(res)) return [];
                        return res.map((edit: any) => ({
                            range: {
                                startLineNumber: (edit.range?.start?.line ?? 0) + 1,
                                startColumn: (edit.range?.start?.character ?? 0) + 1,
                                endLineNumber: (edit.range?.end?.line ?? 0) + 1,
                                endColumn: (edit.range?.end?.character ?? 0) + 1,
                            },
                            text: edit.newText ?? '',
                        }));
                    } catch { return []; }
                },
            });

            (editor as any)._lspDisposables = [
                completionDisposable, hoverDisposable, definitionDisposable,
                referencesDisposable, renameDisposable, inlineDisposable, fixAction,
                codeLensDisposable, formatDisposable,
            ];

            // ── Code Actions Lightbulb (Ctrl+.) ───────────────────────────────
            monaco.languages.registerCodeActionProvider(lang, {
                provideCodeActions: async (model, range) => {
                    const markers = monaco.editor.getModelMarkers({ resource: model.uri })
                        .filter((m: any) => m.severity >= 4);
                    if (!markers.length) return { actions: [], dispose: () => {} };
                    
                    const actions = markers.slice(0, 5).map((m: any) => ({
                        title: m.message.slice(0, 100),
                        id: 'airi-fix-' + m.startLineNumber,
                        edit: null,
                        command: {
                            id: 'airi-fix-command',
                            title: '✦ Fix with AI',
                            arguments: [{ resource: model.uri, marker: m }],
                        },
                    }));
                    return { actions, dispose: () => {} };
                },
            });
        });
    }, [saveActiveFile, activeTab?.language, activeTab?.path]);

    const handleChange: OnChange = useCallback((value) => {
        if (effectiveTabId && value !== undefined) {
            updateTabContent(effectiveTabId, value);
            // Auto-save after 1 second of inactivity (afterDelay mode)
            if (autoSaveTimer.current) clearTimeout(autoSaveTimer.current);
            autoSaveTimer.current = setTimeout(() => {
                useStore.getState().saveActiveFile();
            }, 1000);
        }
    }, [effectiveTabId, updateTabContent]);

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
            const targetContent = activeFilePendingChange ? activeFilePendingChange.newContent : activeTab.content;
            if (currentValue !== targetContent) {
                editorRef.current.setValue(targetContent);
            }
        }
    }, [effectiveTabId, activeFilePendingChange]);

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
    }, [effectiveTabId, activeTab?.path]);

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

    // Jump-to-line: listen for custom events from SearchView / SymbolOutline
    useEffect(() => {
        const handler = (e: Event) => {
            const { path, line, column } = (e as CustomEvent).detail;
            if (!editorRef.current) return;
            if (path && activeTab?.path && path !== activeTab.path) return;
            editorRef.current.revealLineInCenter(line);
            editorRef.current.setPosition({ lineNumber: line, column: column ?? 1 });
            editorRef.current.focus();
        };
        window.addEventListener('editor:jump-to-line', handler);
        return () => window.removeEventListener('editor:jump-to-line', handler);
    }, [activeTab?.path]);

    // LSP: listen for publishDiagnostics → Monaco markers + Problems panel store
    useEffect(() => {
        const unlisten = listen('lsp-diagnostics', (event) => {
            const { uri, diagnostics } = event.payload;
            // Push to Problems panel store
            useStore.getState().setDiagnosticsForUri(uri, diagnostics);
            import('monaco-editor').then(monaco => {
                const models = monaco.editor.getModels();
                const targetPath = uriToPath(uri);
                const model = models.find(m => {
                    const mp = m.uri.path.replace(/^\//, '').replace(/\\/g, '/');
                    return mp === targetPath.replace(/\\/g, '/') || m.uri.toString() === uri;
                });
                if (!model) return;
                const severityMap: Record<number, number> = { 1: 8, 2: 4, 3: 2, 4: 1 };
                const markers = diagnostics.map((d: any) => ({
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

    // Agent "hands" indicator (Windsurf-style): show animated gutter decoration
    // on the file the agent is currently editing.
    const agentEditDecorationsRef = useRef<string[]>([]);
    useEffect(() => {
        const unlistenAgent = listen('agent_editing_file', (event: any) => {
            const { path: editingPath } = event.payload ?? {};
            if (!editorRef.current) return;
            const editor = editorRef.current;
            const model = editor.getModel();
            if (!model) return;
            // Only decorate if this editor's file matches what the agent is editing
            const currentPath = model.uri.path.replace(/^\//, '').replace(/\//g, '\\');
            const normalizedEditing = (editingPath as string ?? '').replace(/\//g, '\\');
            if (!currentPath.toLowerCase().endsWith(normalizedEditing.toLowerCase()) &&
                !normalizedEditing.toLowerCase().endsWith(currentPath.toLowerCase())) return;
            import('monaco-editor').then(monaco => {
                // Place a full-line highlight decoration on line 1 as a "writing" indicator.
                // It will be cleared 3s after the last event.
                agentEditDecorationsRef.current = editor.deltaDecorations(
                    agentEditDecorationsRef.current,
                    [{
                        range: new monaco.Range(1, 1, 1, 1),
                        options: {
                            isWholeLine: true,
                            linesDecorationsClassName: 'agent-editing-gutter',
                            className: 'agent-editing-line',
                        },
                    }]
                );
                // Auto-clear after 3s of no new events
                setTimeout(() => {
                    if (editorRef.current && agentEditDecorationsRef.current.length) {
                        agentEditDecorationsRef.current = editorRef.current.deltaDecorations(
                            agentEditDecorationsRef.current, []
                        );
                    }
                }, 3000);
            });
        });
        return () => { unlistenAgent.then(f => f()); };
    }, []);

    // ── Cursor-style per-hunk gutter diff decorations + accept/reject ─────
    // Computes line-level hunks, renders gutter decorations, and registers
    // context-menu actions so each hunk can be accepted/rejected individually.
    const diffDecorationsRef = useRef<string[]>([]);
    const hunkDataRef = useRef<Array<{ startLine: number; endLine: number; type: 'added' | 'removed'; newContent: string; oldContent: string }>>([]);
    const hunkActionDisposablesRef = useRef<any[]>([]);

    useEffect(() => {
        if (!editorRef.current) return;
        const editor = editorRef.current;

        // Clear old decorations and actions
        if (diffDecorationsRef.current.length) {
            diffDecorationsRef.current = editor.deltaDecorations(diffDecorationsRef.current, []);
        }
        hunkActionDisposablesRef.current.forEach(d => d?.dispose?.());
        hunkActionDisposablesRef.current = [];
        hunkDataRef.current = [];

        if (!activeFilePendingChange) return;

        const oldText = activeFilePendingChange.originalContent ?? activeFilePendingChange.oldContent ?? '';
        const newText = activeFilePendingChange.newContent ?? activeFilePendingChange.proposedContent ?? '';
        if (!oldText || !newText || oldText === newText) return;

        import('monaco-editor').then(monaco => {
            import('diff').then(({ diffLines }) => {
                const hunks = diffLines(oldText, newText);
                const decorations: any[] = [];
                const hunkData: typeof hunkDataRef.current = [];
                let newLine = 1;
                let oldLine = 1;

                // Group consecutive added/removed hunks into logical hunks
                for (let i = 0; i < hunks.length; i++) {
                    const hunk = hunks[i];
                    const count = hunk.count ?? 1;

                    if (hunk.added) {
                        const startLine = newLine;
                        for (let j = 0; j < count; j++) {
                            decorations.push({
                                range: new monaco.Range(newLine + j, 1, newLine + j, 1),
                                options: {
                                    isWholeLine: true,
                                    className: 'agent-diff-added-line',
                                    linesDecorationsClassName: 'agent-diff-added-gutter',
                                    glyphMarginHoverMessage: { value: `**Added line** — right-click to accept/reject this hunk` },
                                },
                            });
                        }
                        hunkData.push({ startLine, endLine: newLine + count - 1, type: 'added', newContent: hunk.value, oldContent: '' });
                        newLine += count;
                    } else if (hunk.removed) {
                        decorations.push({
                            range: new monaco.Range(Math.max(1, newLine - 1), 1, Math.max(1, newLine - 1), 1),
                            options: {
                                linesDecorationsClassName: 'agent-diff-deleted-gutter',
                                glyphMarginHoverMessage: { value: `**Deleted ${count} line(s)** — right-click to accept/reject` },
                            },
                        });
                        hunkData.push({ startLine: Math.max(1, newLine - 1), endLine: Math.max(1, newLine - 1), type: 'removed', newContent: '', oldContent: hunk.value });
                        // Removed lines don't exist in new file — don't advance newLine
                    } else {
                        newLine += count;
                        oldLine += count;
                    }
                }

                hunkDataRef.current = hunkData;

                if (decorations.length > 0) {
                    diffDecorationsRef.current = editor.deltaDecorations(
                        diffDecorationsRef.current,
                        decorations
                    );
                }

                // Register context-menu actions for per-hunk accept/reject
                const acceptAction = editor.addAction({
                    id: 'agent-diff-accept-hunk',
                    label: '✓ Accept this hunk',
                    contextMenuGroupId: 'agent-diff',
                    contextMenuOrder: 0,
                    precondition: null,
                    run: (ed) => {
                        const line = ed.getPosition()?.lineNumber ?? 1;
                        const hunk = hunkDataRef.current.find(h => line >= h.startLine && line <= h.endLine + 1);
                        if (!hunk || !activeFilePendingChange) return;
                        // For added hunks: accept = keep as-is (already in editor). Just mark hunk done.
                        // For removed hunks: accept = remove those lines from original that were deleted.
                        // Since the editor already shows the new content, accept means keep it.
                        // Full accept of the whole file for now — per-hunk partial apply is complex.
                        useStore.getState().acceptPendingChange(activeFilePendingChange.id).catch(console.error);
                    },
                });

                const rejectAction = editor.addAction({
                    id: 'agent-diff-reject-hunk',
                    label: '✕ Reject this hunk',
                    contextMenuGroupId: 'agent-diff',
                    contextMenuOrder: 1,
                    precondition: null,
                    run: (ed) => {
                        const line = ed.getPosition()?.lineNumber ?? 1;
                        const hunk = hunkDataRef.current.find(h => line >= h.startLine && line <= h.endLine + 1);
                        if (!hunk || !activeFilePendingChange) return;
                        // Rejecting a hunk: revert that hunk's lines to original content
                        // Get current model value, find the hunk lines, replace with original
                        const model = ed.getModel();
                        if (!model) return;
                        if (hunk.type === 'added') {
                            // Remove the added lines
                            const range = new monaco.Range(hunk.startLine, 1, hunk.endLine + 1, 1);
                            model.applyEdits([{ range, text: '' }]);
                        } else if (hunk.type === 'removed') {
                            // Re-insert the removed lines at the deletion point
                            const pos = new monaco.Range(hunk.startLine, 1, hunk.startLine, 1);
                            model.applyEdits([{ range: pos, text: hunk.oldContent }]);
                        }
                    },
                });

                hunkActionDisposablesRef.current = [acceptAction, rejectAction];
            });
        });
    }, [activeFilePendingChange]);

    // Git blame — show "author · date · summary" as ghost text on the cursor line (GitLens style)
    const blameDataRef = useRef<string[]>([]);
    const blameDecorationsRef = useRef<string[]>([]);
    useEffect(() => {
        if (!activeTab?.path) return;
        const root = useStore.getState().activeRoot ?? '';
        invoke<string[]>('git_blame', { path: root, filePath: activeTab.path })
            .then(lines => { blameDataRef.current = lines ?? []; })
            .catch(() => { blameDataRef.current = []; });
    }, [activeTab?.path]);

    useEffect(() => {
        if (!editorRef.current) return;
        const editor = editorRef.current;
        // Clear decorations when blame is toggled off
        if (!isGitBlameVisible) {
            import('monaco-editor').then(() => {
                if (editorRef.current && blameDecorationsRef.current.length) {
                    blameDecorationsRef.current = editorRef.current.deltaDecorations(blameDecorationsRef.current, []);
                }
            });
            return;
        }
        const disposable = editor.onDidChangeCursorPosition((e: any) => {
            const line = e.position.lineNumber;
            const entry = blameDataRef.current[line - 1];
            import('monaco-editor').then(monaco => {
                if (!editorRef.current) return;
                const [, author, date, summary] = (entry ?? '').split('|');
                const newDecorations = (!author || author === 'Not Committed Yet') ? [] : [{
                    range: new monaco.Range(line, 1, line, 1),
                    options: {
                        isWholeLine: true,
                        after: {
                            content: `  ${author} · ${date} · ${summary ?? ''}`,
                            inlineClassName: 'git-blame-ghost',
                        },
                    },
                }];
                blameDecorationsRef.current = editorRef.current.deltaDecorations(
                    blameDecorationsRef.current,
                    newDecorations
                );
            });
        });
        return () => disposable.dispose();
    }, [isGitBlameVisible]);

    // Git gutter decorations — green added, blue modified, red deleted
    const gitDecorationsRef = useRef<string[]>([]);
    const refreshGitGutter = useCallback(() => {
        if (!editorRef.current || !activeTab?.path) return;
        invoke<{ added: number[]; modified: number[]; deleted: number[]; new_file: boolean }>(
            'get_git_file_hunks', { path: activeTab.path }
        ).then(hunks => {
            import('monaco-editor').then(monaco => {
                const editor = editorRef.current;
                if (!editor) return;
                const decorations: any[] = [
                    ...hunks.added.map((line: number) => ({
                        range: new monaco.Range(line, 1, line, 1),
                        options: { isWholeLine: false, linesDecorationsClassName: 'git-gutter-added' },
                    })),
                    ...hunks.modified.map((line: number) => ({
                        range: new monaco.Range(line, 1, line, 1),
                        options: { isWholeLine: false, linesDecorationsClassName: 'git-gutter-modified' },
                    })),
                    ...hunks.deleted.map((line: number) => ({
                        range: new monaco.Range(line, 1, line, 1),
                        options: { isWholeLine: false, linesDecorationsClassName: 'git-gutter-deleted' },
                    })),
                ];
                gitDecorationsRef.current = editor.deltaDecorations(
                    gitDecorationsRef.current,
                    decorations
                );
            });
        }).catch(() => { });
    }, [activeTab?.path]);

    useEffect(() => {
        refreshGitGutter();
        const unlisten = listen('file-changed', (event: any) => {
            if (event.payload?.path === activeTab?.path) {
                refreshGitGutter();
            }
        });
        return () => { unlisten.then(f => f()); };
    }, [activeTab?.path, refreshGitGutter]);

    if (!activeTab) {
        // Show the Welcome page on first open (unless dismissed) plus the
        // AIRI orb behind it. WelcomePage handles its own dismissal so
        // once the user closes it they get the legacy avatar view.
        return (
            <div style={{
                height: '100%', width: '100%',
                position: 'relative',
                background: 'var(--vscode-editor-background)',
                overflow: 'hidden',
            }}>
                <WelcomePage />
                <div style={{
                    height: '100%',
                    width: '100%',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    flexDirection: 'column',
                    gap: '20px',
                    pointerEvents: 'none',
                }}>
                <div style={{ 
                    width: '200px', 
                    height: '200px', 
                    borderRadius: '50%', 
                    background: 'radial-gradient(circle, rgba(124, 58, 237, 0.3) 0%, rgba(0, 0, 0, 0) 70%)',
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    animation: 'pulse 2s ease-in-out infinite'
                }}>
                    <div style={{ 
                        width: '120px', 
                        height: '120px', 
                        borderRadius: '50%', 
                        background: 'linear-gradient(135deg, #7c3aed 0%, #4c1d95 100%)',
                        boxShadow: '0 0 60px rgba(124, 58, 237, 0.5)',
                        animation: 'float 3s ease-in-out infinite'
                    }} />
                </div>
                <div style={{ color: 'var(--vscode-editor-foreground)', fontSize: '14px', opacity: 0.7 }}>
                    AIRI is ready for your mission
                </div>
                <div style={{ color: 'var(--vscode-descriptionForeground)', fontSize: '12px' }}>
                    Speak to AIRI or start a new project
                </div>
                <style>{`
                    @keyframes pulse {
                        0%, 100% { transform: scale(1); opacity: 0.5; }
                        50% { transform: scale(1.1); opacity: 0.8; }
                    }
                    @keyframes float {
                        0%, 100% { transform: translateY(0); }
                        50% { transform: translateY(-20px); }
                    }
                `}</style>
                </div>
            </div>
        );
    }

    return (
        <div style={{ position: 'relative', height: '100%', width: '100%', display: 'flex', flexDirection: 'column' }}>
            {/* Breadcrumbs — file path + LSP symbol trail. We render it
                above the Monaco editor so it stays anchored regardless of
                editor scrolling. The component reads the active path from
                the store and the cursor line from a CustomEvent the Monaco
                onDidChangeCursorPosition handler already emits. */}
            <Breadcrumbs />
            <div style={{ flex: 1, display: 'flex', minHeight: 0, position: 'relative' }}>
              <div style={{ flex: 1, minWidth: 0, position: 'relative' }}>
                <MonacoEditor
                    height="100%"
                    width="100%"
                theme={theme}
                language={activeTab.language}
                value={activeFilePendingChange ? activeFilePendingChange.newContent : activeTab.content}
                onMount={handleMount}
                onChange={(value, ev) => { handleChange(value, ev); handleChangeLsp(value); }}
                loading={<div className="editor-loading" style={{ background: 'var(--vscode-editor-background)', color: 'var(--vscode-editor-foreground)', height: '100%', display: 'flex', alignItems: 'center', justifyContent: 'center', fontSize: '12px', opacity: 0.5 }}>Loading IDE Editor Assets...</div>}
                options={{
                    fontSize: 13,
                    fontFamily: 'var(--font-mono)',
                    lineNumbers: 'on',
                    lineNumbersMinChars: 3,
                    glyphMargin: true,
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
                    stickyScroll: { enabled: true },
                    codeLens: true,
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
                <>
                    <div style={{
                        position: 'absolute',
                        top: '40px',
                        left: '50%',
                        transform: 'translateX(-50%)',
                        zIndex: 2000,
                        display: 'flex',
                        alignItems: 'center',
                        gap: '16px',
                        padding: '10px 20px',
                        borderRadius: '12px',
                        background: 'rgba(18, 18, 29, 0.82)',
                        backdropFilter: 'blur(16px)',
                        WebkitBackdropFilter: 'blur(16px)',
                        border: '1px solid rgba(0, 198, 255, 0.35)',
                        boxShadow: '0 8px 32px rgba(0, 0, 0, 0.5), 0 0 20px rgba(0, 198, 255, 0.2), inset 0 0 0 1px rgba(255, 255, 255, 0.05)',
                        color: 'var(--vscode-editor-foreground, #ffffff)',
                        fontSize: '12px',
                        fontFamily: 'var(--font-ui, sans-serif)',
                        animation: 'bannerSlideIn 0.35s cubic-bezier(0.16, 1, 0.3, 1)'
                    }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: '10px' }}>
                            <span style={{
                                display: 'flex',
                                alignItems: 'center',
                                justifyContent: 'center',
                                width: '24px',
                                height: '24px',
                                borderRadius: '50%',
                                background: 'rgba(0, 198, 255, 0.18)',
                                color: '#00c6ff',
                                fontSize: '13px',
                                textShadow: '0 0 8px #00c6ff',
                                alignSelf: 'center',
                                paddingLeft: '4px'
                            }}>✨</span>
                            <span style={{ fontWeight: 500, letterSpacing: '0.3px', textShadow: '0 1px 4px rgba(0,0,0,0.5)' }}>
                                AI suggested edits for <span style={{ color: '#00c6ff', fontWeight: 600 }}>{activeTab?.filename}</span> are pending review
                            </span>
                        </div>
                        <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                            <button
                                onClick={async () => {
                                    await useStore.getState().acceptPendingChange(activeFilePendingChange.id);
                                }}
                                style={{
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '6px',
                                    padding: '6px 14px',
                                    borderRadius: '6px',
                                    border: 'none',
                                    background: 'linear-gradient(135deg, #00c6ff 0%, #0072ff 100%)',
                                    color: 'var(--vscode-editor-foreground, #ffffff)',
                                    fontSize: '11px',
                                    fontWeight: 700,
                                    cursor: 'pointer',
                                    boxShadow: '0 2px 10px rgba(0, 198, 255, 0.35)',
                                    transition: 'all 0.2s',
                                    letterSpacing: '0.3px'
                                }}
                                onMouseEnter={(e) => {
                                    e.currentTarget.style.filter = 'brightness(1.15)';
                                    e.currentTarget.style.boxShadow = '0 4px 14px rgba(0, 198, 255, 0.5)';
                                    e.currentTarget.style.transform = 'translateY(-1px)';
                                }}
                                onMouseLeave={(e) => {
                                    e.currentTarget.style.filter = 'none';
                                    e.currentTarget.style.boxShadow = '0 2px 10px rgba(0, 198, 255, 0.35)';
                                    e.currentTarget.style.transform = 'none';
                                }}
                            >
                                ✓ Accept Changes
                            </button>
                            <button
                                onClick={() => {
                                    useStore.getState().rejectPendingChange(activeFilePendingChange.id);
                                }}
                                style={{
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '6px',
                                    padding: '6px 14px',
                                    borderRadius: '6px',
                                    border: '1px solid rgba(244, 63, 94, 0.45)',
                                    background: 'rgba(244, 63, 94, 0.15)',
                                    color: '#f43f5e',
                                    fontSize: '11px',
                                    fontWeight: 700,
                                    cursor: 'pointer',
                                    transition: 'all 0.2s',
                                    letterSpacing: '0.3px'
                                }}
                                onMouseEnter={(e) => {
                                    e.currentTarget.style.background = 'rgba(244, 63, 94, 0.25)';
                                    e.currentTarget.style.borderColor = 'rgba(244, 63, 94, 0.6)';
                                    e.currentTarget.style.transform = 'translateY(-1px)';
                                }}
                                onMouseLeave={(e) => {
                                    e.currentTarget.style.background = 'rgba(244, 63, 94, 0.15)';
                                    e.currentTarget.style.borderColor = 'rgba(244, 63, 94, 0.45)';
                                    e.currentTarget.style.transform = 'none';
                                }}
                            >
                                ✗ Reject Changes
                            </button>
                        </div>
                        <style>{`
                            @keyframes bannerSlideIn {
                                from { transform: translate(-50%, -30px); opacity: 0; }
                                to { transform: translate(-50%, 0); opacity: 1; }
                            }
                        `}</style>
                    </div>
                    <DiffViewer />
                </>
            )}

            {isInlineEditOpen && (
                <InlineEditOverlay
                    position={inlineEditPosition}
                    onClose={() => { setIsInlineEditOpen(false); setInlineEditSelection(null); }}
                    onSubmit={async (prompt) => {
                        setIsInlineEditOpen(false);
                        const sel = inlineEditSelection;
                        setInlineEditSelection(null);

                        const filePath = activeTab?.path || 'unknown';
                        const lang = activeTab?.language || 'unknown';
                        const selectedText = sel?.text || '';

                        // Fast path: bypass orchestrator, use ai_chat_fast directly
                        const fastPrompt = `You are a surgical AI editor.
Task: Modify the selected code based on the instruction.
Return ONLY the modified selected code. DO NOT wrap it in markdown block. DO NOT explain. DO NOT output the rest of the file.

File: ${filePath}
Language: ${lang}

Instruction: ${prompt}

Selected code to modify:
\`\`\`${lang}
${selectedText}
\`\`\`
`;

                        const store = useStore.getState();
                        store.setIsAgentThinking?.(true);

                        try {
                            const { invoke } = await import('@tauri-apps/api/core');
                            
                            // Send fast inference request
                            let replacement = await invoke<string>('ai_chat_fast', {
                                request: {
                                    messages: [{ role: 'user', content: fastPrompt }],
                                    max_tokens: 4096,
                                    temperature: 0.1,
                                    provider: 'google', // auto-resolved in backend
                                    model: store.agentModel || 'gemini-2.5-pro'
                                }
                            });
                            
                            // Clean up any markdown blocks if the model ignored instructions
                            if (replacement.startsWith('```')) {
                                const lines = replacement.split('\\n');
                                if (lines[0].startsWith('```')) lines.shift();
                                if (lines[lines.length - 1].startsWith('```')) lines.pop();
                                replacement = lines.join('\\n');
                            }

                            const oldContent = activeTab?.content || '';
                            const newContent = oldContent.replace(selectedText, replacement);

                            // Push to backend shadow workspace so it can be accepted/rejected
                            await invoke('propose_fast_edit', {
                                path: filePath,
                                newContent: newContent
                            });

                            // Tell frontend state to render DiffViewer
                            store.proposePendingChange({
                                path: filePath,
                                description: prompt,
                                oldContent: oldContent,
                                newContent: newContent,
                            });
                        } catch (error: any) {
                            store.updateLastAgentMessage(`**Error:** ${error.message || error}`);
                        } finally {
                            store.setIsAgentThinking?.(false);
                        }
                    }}
                />
            )}
                {/* Cursor-style next-edit prediction. After the user
                    renames an identifier we surface remaining sites with
                    one-Tab apply. Mounted inside the editor pane so its
                    absolute positioning anchors to the editor, not the
                    full window. */}
                <PredictiveEditOverlay />
              </div>
              {/* Side-by-side markdown preview. The component returns null
                  unless the active file is .md and the toggle is on, so
                  the layout collapses cleanly for other file types. */}
              <MarkdownPreview />
            </div>
        </div>
    );
});

export default Editor;
