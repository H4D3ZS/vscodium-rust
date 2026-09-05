/**
 * All Monaco LSP language provider registrations extracted from Editor.tsx.
 * Call registerMonacoProviders from handleMount; dispose returned handles on cleanup.
 */

function lspKindToMonaco(k: number): number {
    const map: Record<number, number> = {
        1: 17, 2: 0, 3: 1, 4: 2, 5: 3, 6: 4, 7: 6, 8: 7, 9: 8,
        10: 9, 12: 11, 13: 12, 14: 13, 15: 14, 16: 15, 17: 16, 18: 17, 25: 24,
    };
    return map[k] ?? 9;
}

/**
 * Disable Monaco's built-in TypeScript/JavaScript language service.
 *
 * It is entirely redundant here: completions, hover and diagnostics all come
 * from the IDE's own language servers (`registerMonacoProviders` below, plus
 * `setModelMarkers(model, 'lsp', …)` in Editor.tsx). Left enabled, Monaco
 * spawns `ts.worker` — a 6.7MB bundle — and builds a SECOND full TypeScript
 * program in JS: every lib.d.ts plus every project file it can reach, held in
 * the WebView2 renderer. On this repo (~2800 files) that is the single largest
 * consumer of renderer memory, and it duplicates work Rust already did.
 *
 * `setEagerModelSync(false)` matters as much as the diagnostics flags: with it
 * on, Monaco pushes every open model into the worker regardless of validation.
 *
 * Idempotent — safe to call from every mount.
 */
let languageServiceDisabled = false;

export function disableMonacoBuiltinLanguageService(monaco: any): void {
    if (languageServiceDisabled) return;
    const ts = monaco?.languages?.typescript;
    if (!ts) return;
    const off = {
        noSemanticValidation: true,
        noSyntaxValidation: true,
        noSuggestionDiagnostics: true,
    };
    try {
        for (const defaults of [ts.typescriptDefaults, ts.javascriptDefaults]) {
            if (!defaults) continue;
            defaults.setDiagnosticsOptions?.(off);
            defaults.setEagerModelSync?.(false);
        }
        languageServiceDisabled = true;
    } catch {
        /* Monaco build without the TS contribution — nothing to disable. */
    }
}

export function registerMonacoProviders(
    editor: any,
    monaco: any,
    lang: string,
    getFileUri: () => string,
    activeTabPath: string | undefined,
    invoke: <T>(cmd: string, args?: Record<string, any>) => Promise<T>
): any[] {
    const completionDisposable = monaco.languages.registerCompletionItemProvider(lang, {
        triggerCharacters: ['.', ':', '(', '<', '"', "'", '/', '@', '#'],
        provideCompletionItems: async (model: any, position: any) => {
            try {
                const res = await invoke<any>('lsp_completion', {
                    uri: getFileUri(),
                    line: position.lineNumber - 1,
                    character: position.column - 1,
                });
                const word = model.getWordUntilPosition(position);
                const range = {
                    startLineNumber: position.lineNumber, endLineNumber: position.lineNumber,
                    startColumn: word.startColumn, endColumn: word.endColumn,
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

    const hoverDisposable = monaco.languages.registerHoverProvider(lang, {
        provideHover: async (_model: any, position: any) => {
            try {
                const res = await invoke<any>('lsp_hover', {
                    uri: getFileUri(),
                    line: position.lineNumber - 1,
                    character: position.column - 1,
                });
                if (!res) return null;
                const raw = res.contents ?? res;
                const contents = Array.isArray(raw) ? raw : [raw];
                const mdParts = contents.map((c: any) => ({ value: typeof c === 'string' ? c : (c.value ?? String(c)) }));
                if (!mdParts.length || !mdParts[0].value) return null;
                const range = res.range ? {
                    startLineNumber: res.range.start.line + 1, startColumn: res.range.start.character + 1,
                    endLineNumber: res.range.end.line + 1, endColumn: res.range.end.character + 1,
                } : undefined;
                return { contents: mdParts, range };
            } catch { return null; }
        },
    });

    const definitionDisposable = monaco.languages.registerDefinitionProvider(lang, {
        provideDefinition: async (_model: any, position: any) => {
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
                            startLineNumber: (r.start?.line ?? 0) + 1, startColumn: (r.start?.character ?? 0) + 1,
                            endLineNumber: (r.end?.line ?? 0) + 1, endColumn: (r.end?.character ?? 0) + 1,
                        },
                    };
                });
            } catch { return []; }
        },
    });

    const referencesDisposable = monaco.languages.registerReferenceProvider(lang, {
        provideReferences: async (_model: any, position: any) => {
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
                        startLineNumber: (ref.range?.start?.line ?? 0) + 1, startColumn: (ref.range?.start?.character ?? 0) + 1,
                        endLineNumber: (ref.range?.end?.line ?? 0) + 1, endColumn: (ref.range?.end?.character ?? 0) + 1,
                    },
                }));
            } catch { return []; }
        },
    });

    const renameDisposable = monaco.languages.registerRenameProvider(lang, {
        provideRenameEdits: async (_model: any, position: any, newName: string) => {
            try {
                const res = await invoke<any>('lsp_rename_symbol', {
                    uri: getFileUri(), line: position.lineNumber - 1, character: position.column - 1, newName,
                });
                if (!res) return null;
                const edits: any[] = [];
                const toEdit = (u: string, e: any) => ({
                    resource: monaco.Uri.parse(u),
                    textEdit: {
                        range: {
                            startLineNumber: (e.range.start.line ?? 0) + 1, startColumn: (e.range.start.character ?? 0) + 1,
                            endLineNumber: (e.range.end.line ?? 0) + 1, endColumn: (e.range.end.character ?? 0) + 1,
                        },
                        text: e.newText,
                    },
                });
                if (res.documentChanges) {
                    for (const dc of res.documentChanges)
                        for (const e of (dc.edits ?? []))
                            edits.push(toEdit(dc.textDocument?.uri ?? dc.uri ?? '', e));
                } else if (res.changes) {
                    for (const [u, fileEdits] of Object.entries(res.changes as Record<string, any[]>))
                        for (const e of fileEdits) edits.push(toEdit(u, e));
                }
                return { edits };
            } catch { return null; }
        },
    });

    let inlineTimer: any = null;
    const inlineDisposable = monaco.languages.registerInlineCompletionsProvider(lang, {
        provideInlineCompletions: async (model: any, position: any, _ctx: any, token: any) => {
            const st = (window as any).useStore?.getState?.() || {};
            if (st.tabPredictionEnabled === false || st.voidGlobalSettings?.enableAutocomplete === false)
                return { items: [] };
            if (inlineTimer) clearTimeout(inlineTimer);
            const suggestion = await new Promise<string | null>(resolve => {
                inlineTimer = setTimeout(async () => {
                    if (token.isCancellationRequested) return resolve(null);
                    let textBefore = model.getValueInRange({
                        startLineNumber: Math.max(1, position.lineNumber - 60), startColumn: 1,
                        endLineNumber: position.lineNumber, endColumn: position.column,
                    });
                    const textAfter = model.getValueInRange({
                        startLineNumber: position.lineNumber, startColumn: position.column,
                        endLineNumber: Math.min(model.getLineCount(), position.lineNumber + 20),
                        endColumn: model.getLineMaxColumn(Math.min(model.getLineCount(), position.lineNumber + 20)),
                    });
                    // Prepend the file's import/use header when the cursor is deep in the
                    // file — gives the model type/symbol context it can't see in a 60-line
                    // window, at near-zero cost. Skipped when already near the top.
                    if (position.lineNumber > 70) {
                        const headerEnd = Math.min(30, position.lineNumber - 1);
                        const header = model.getValueInRange({
                            startLineNumber: 1, startColumn: 1, endLineNumber: headerEnd,
                            endColumn: model.getLineMaxColumn(headerEnd),
                        });
                        const importLines = header.split('\n')
                            .filter((l: string) => /^\s*(import|from|use|#include|require|using)\b/.test(l))
                            .slice(0, 40).join('\n');
                        if (importLines) textBefore = importLines + '\n\n' + textBefore;
                    }
                    if (textBefore.trim().length < 2) return resolve(null);
                    const acSel = st.modelSelectionOfFeature?.['Autocomplete'];
                    try {
                        const res = await invoke<string>('ai_inline_complete', {
                            prefix: textBefore, suffix: textAfter, language: lang,
                            filePath: activeTabPath || '',
                            ...(acSel?.modelName ? { model: acSel.modelName, provider: acSel.providerName } : {}),
                        });
                        resolve(res ?? null);
                    } catch { resolve(null); }
                }, 220);
            });
            if (!suggestion?.trim() || token.isCancellationRequested) return { items: [] };
            return {
                items: [{
                    insertText: suggestion,
                    range: new monaco.Range(position.lineNumber, position.column, position.lineNumber, position.column),
                }],
            };
        },
        handleItemDidShow: () => {},
        disposeInlineCompletions: () => { if (inlineTimer) clearTimeout(inlineTimer); },
    });

    const codeLensDisposable = monaco.languages.registerCodeLensProvider(lang, {
        provideCodeLenses: async () => {
            try {
                const res = await invoke<any>('lsp_code_lens', { uri: getFileUri() });
                if (!res || !Array.isArray(res)) return { lenses: [], dispose: () => {} };
                return {
                    lenses: res.map((cl: any) => ({
                        range: {
                            startLineNumber: (cl.range?.start?.line ?? 0) + 1, startColumn: (cl.range?.start?.character ?? 0) + 1,
                            endLineNumber: (cl.range?.end?.line ?? 0) + 1, endColumn: (cl.range?.end?.character ?? 0) + 1,
                        },
                        command: cl.command
                            ? { id: cl.command.command || '', title: cl.command.title || '', arguments: cl.command.arguments }
                            : { id: '', title: cl.data?.toString() ?? '' },
                    })),
                    dispose: () => {},
                };
            } catch { return { lenses: [], dispose: () => {} }; }
        },
        resolveCodeLens: (_: any, codeLens: any) => Promise.resolve(codeLens),
    });

    const formatDisposable = monaco.languages.registerDocumentFormattingEditProvider(lang, {
        provideDocumentFormattingEdits: async () => {
            try {
                const res = await invoke<any[]>('lsp_format_document', { uri: getFileUri() });
                if (!Array.isArray(res)) return [];
                return res.map((edit: any) => ({
                    range: {
                        startLineNumber: (edit.range?.start?.line ?? 0) + 1, startColumn: (edit.range?.start?.character ?? 0) + 1,
                        endLineNumber: (edit.range?.end?.line ?? 0) + 1, endColumn: (edit.range?.end?.character ?? 0) + 1,
                    },
                    text: edit.newText ?? '',
                }));
            } catch { return []; }
        },
    });

    return [
        completionDisposable, hoverDisposable, definitionDisposable,
        referencesDisposable, renameDisposable, inlineDisposable,
        codeLensDisposable, formatDisposable,
    ];
}
