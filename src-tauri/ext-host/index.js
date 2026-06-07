/**
 * VSCodium Rust Extension Host
 * Executes third-party extensions in an isolated environment.
 * Provider requests (completion, hover, definition, diagnostics) are forwarded
 * to the IDE via stdout JSON-line IPC.
 */

const readline = require('readline');
const path = require('path');
const fs = require('fs');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

// ── Deep Proxy (prevent crashes on unimplemented APIs) ────────────────────────
function createStubProxy(target, apiPath = 'vscode') {
    return new Proxy(target, {
        get(obj, prop) {
            if (prop in obj) return obj[prop];
            if (typeof prop === 'string' && prop !== 'then') {
                const stub = function () {
                    return createStubProxy({}, `${apiPath}.${prop}()`);
                };
                return createStubProxy(stub, `${apiPath}.${prop}`);
            }
            return undefined;
        }
    });
}

// ── Settings cache (refreshed on 'settingsUpdated' messages) ─────────────────
let currentSettings = { theme: 'vs-dark', font_size: 14, tab_size: 4 };

// ── Provider registry ─────────────────────────────────────────────────────────
// Providers registered by extensions; keyed by a unique id.
const completionProviders = new Map();  // id → { selector, provider, triggerChars }
const hoverProviders = new Map();  // id → { selector, provider }
const definitionProviders = new Map();  // id → { selector, provider }
const diagnosticCollections = new Map(); // name → Map<uri, diagnostics[]>

let providerIdCounter = 0;
function nextProviderId() { return `p${++providerIdCounter}`; }

// Match a document to a selector (language, pattern, scheme)
function selectorMatches(selector, doc) {
    if (!selector) return true;
    const sel = typeof selector === 'string' ? { language: selector } : selector;
    if (sel.language && doc.languageId !== sel.language) return false;
    return true;
}

// ── Event emitter ─────────────────────────────────────────────────────────────
const eventHandlers = {
    handlers: new Map(),
    on(event, cb) {
        if (!this.handlers.has(event)) this.handlers.set(event, []);
        this.handlers.get(event).push(cb);
        return {
            dispose: () => {
                const arr = this.handlers.get(event) || [];
                const idx = arr.indexOf(cb);
                if (idx >= 0) arr.splice(idx, 1);
            }
        };
    },
    emit(event, ...args) {
        (this.handlers.get(event) || []).forEach(cb => {
            try { cb(...args); } catch (e) { /* swallow */ }
        });
    }
};

// ── IPC ───────────────────────────────────────────────────────────────────────
function sendResponse(obj) {
    process.stdout.write(JSON.stringify(obj) + '\n');
}

// Pending request map for request/response correlation
const pendingRequests = new Map();
let reqCounter = 0;

function sendRequest(req) {
    return new Promise((resolve) => {
        const id = `req_${++reqCounter}`;
        pendingRequests.set(id, resolve);
        sendResponse({ ...req, _reqId: id });
        // Timeout after 5 s to avoid hangs
        setTimeout(() => {
            if (pendingRequests.has(id)) {
                pendingRequests.delete(id);
                resolve(null);
            }
        }, 5000);
    });
}

// ── VSCode API implementation ─────────────────────────────────────────────────
const vscodeImpl = {
    window: {
        showInformationMessage: (msg, ...items) => {
            sendResponse({ type: 'notification', level: 'info', message: msg, items });
            return Promise.resolve(undefined);
        },
        showErrorMessage: (msg, ...items) => {
            sendResponse({ type: 'notification', level: 'error', message: msg, items });
            return Promise.resolve(undefined);
        },
        showWarningMessage: (msg, ...items) => {
            sendResponse({ type: 'notification', level: 'warning', message: msg, items });
            return Promise.resolve(undefined);
        },
        showInputBox: (options) => {
            return sendRequest({ type: 'showInputBox', options });
        },
        showQuickPick: (items, options) => {
            return sendRequest({ type: 'showQuickPick', items, options });
        },
        createTextEditorDecorationType: (options) => ({
            key: `deco_${Date.now()}`,
            dispose: () => { }
        }),
        createOutputChannel: (name) => ({
            name,
            append: (val) => sendResponse({ type: 'outputChannel', channel: name, text: val }),
            appendLine: (val) => sendResponse({ type: 'outputChannel', channel: name, text: val + '\n' }),
            clear: () => { },
            show: () => { },
            hide: () => { },
            dispose: () => { }
        }),
        createWebviewPanel: (viewType, title, showOptions, options) => {
            const panel = {
                webview: {
                    html: '',
                    onDidReceiveMessage: (cb) => eventHandlers.on(`webview:${viewType}:message`, cb),
                    postMessage: (msg) => sendResponse({ type: 'webviewMessage', viewType, message: msg }),
                    asWebviewUri: (uri) => uri,
                    cspSource: 'self',
                },
                title,
                visible: true,
                onDidDispose: (cb) => eventHandlers.on(`webview:${viewType}:dispose`, cb),
                onDidChangeViewState: (cb) => eventHandlers.on(`webview:${viewType}:stateChange`, cb),
                reveal: () => { },
                dispose: () => { }
            };
            sendResponse({ type: 'createWebviewPanel', viewType, title });
            return panel;
        },
        get activeTextEditor() {
            const docs = vscodeImpl.workspace.textDocuments;
            if (docs.length === 0) return undefined;
            const doc = docs[0];
            return {
                document: doc,
                selection: { active: { line: 0, character: 0 }, anchor: { line: 0, character: 0 }, isEmpty: true },
                selections: [],
                visibleRanges: [],
                options: { tabSize: currentSettings.tab_size ?? 4 },
                setDecorations: (type, ranges) => {
                    sendResponse({ type: 'setDecorations', key: type.key, ranges });
                },
                edit: (callback) => Promise.resolve(false),
            };
        },
        visibleTextEditors: [],
        onDidChangeActiveTextEditor: (cb) => eventHandlers.on('onDidChangeActiveTextEditor', cb),
        onDidChangeTextEditorSelection: (cb) => eventHandlers.on('onDidChangeTextEditorSelection', cb),
        onDidChangeVisibleTextEditors: (cb) => eventHandlers.on('onDidChangeVisibleTextEditors', cb),
        withProgress: (options, task) => {
            return task({ report: () => { } }, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
        },
        registerTreeDataProvider: (viewId, provider) => {
            sendResponse({ type: 'registerTreeView', viewId });
            return { dispose: () => { } };
        },
        createTreeView: (viewId, options) => {
            sendResponse({ type: 'createTreeView', viewId });
            return { visible: true, onDidChangeVisibility: () => ({ dispose: () => { } }), reveal: () => { }, dispose: () => { } };
        },
        showTextDocument: async (docOrUri, options) => {
            const p = typeof docOrUri === 'string'
                ? docOrUri
                : (docOrUri?.uri?.fsPath || docOrUri?.fileName || String(docOrUri));
            sendResponse({ type: 'openFile', path: p.replace(/^file:\/\/\/?/, '') });
            const opened = await vscodeImpl.workspace.openTextDocument(p);
            return {
                document: opened,
                selection: { active: { line: 0, character: 0 }, anchor: { line: 0, character: 0 }, isEmpty: true },
                selections: [],
                visibleRanges: [],
                options: { tabSize: currentSettings.tab_size ?? 4 },
                edit: (callback) => Promise.resolve(false),
            };
        },
        createTerminal: (name, shellPath, shellArgs) => {
            const id = `ext-term-${Date.now()}`;
            sendResponse({ type: 'createTerminal', id, name: name || 'Extension Terminal', shellPath, shellArgs });
            return {
                name: name || 'Terminal',
                processId: Promise.resolve(undefined),
                sendText: (text, addNewLine = true) => {
                    sendResponse({ type: 'terminalSendText', id, text: addNewLine ? text + '\r' : text });
                },
                show: () => sendResponse({ type: 'showTerminal', id }),
                hide: () => { },
                dispose: () => sendResponse({ type: 'disposeTerminal', id }),
            };
        },
        createStatusBarItem: (alignment, priority) => {
            const id = `sbi-${Date.now()}`;
            const item = {
                id,
                text: '',
                tooltip: '',
                command: undefined,
                show: () => sendResponse({ type: 'statusBarShow', id, text: item.text, tooltip: item.tooltip }),
                hide: () => sendResponse({ type: 'statusBarHide', id }),
                dispose: () => sendResponse({ type: 'statusBarDispose', id }),
            };
            return item;
        },
    },

    commands: {
        registerCommand: (id, callback) => {
            commands.set(id, callback);
            sendResponse({ type: 'commandRegistered', id });
            return { dispose: () => commands.delete(id) };
        },
        executeCommand: async (id, ...args) => {
            // Try local first, then ask the IDE
            if (commands.has(id)) {
                try { return await commands.get(id)(...args); } catch (e) { }
            }
            return sendRequest({ type: 'executeCommand', id, args });
        },
        getCommands: () => Promise.resolve([...commands.keys()]),
    },

    workspace: {
        textDocuments: [],
        rootPath: process.cwd(),
        workspaceFolders: [{
            uri: { fsPath: process.cwd(), toString: () => `file:///${process.cwd().replace(/\\/g, '/')}` },
            name: path.basename(process.cwd()),
            index: 0
        }],
        fs: {
            readFile: async (uri) => {
                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
                try { return Buffer.from(fs.readFileSync(fpath)); } catch { return null; }
            },
            writeFile: async (uri, content) => {
                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
                fs.writeFileSync(fpath, Buffer.from(content));
            },
            stat: async (uri) => {
                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
                try {
                    const st = fs.statSync(fpath);
                    return { type: st.isDirectory() ? 2 : 1, ctime: st.ctimeMs, mtime: st.mtimeMs, size: st.size };
                } catch { throw new Error('File not found'); }
            },
            delete: async (uri) => {
                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
                fs.unlinkSync(fpath);
            }
        },
        getConfiguration: (section) => {
            return {
                get: (key, defaultValue) => {
                    const fullKey = section ? `${section}.${key}` : key;
                    if (currentSettings[fullKey] !== undefined) return currentSettings[fullKey];
                    // Dot-walk currentSettings for nested keys
                    if (section && currentSettings[section]) {
                        const sectionObj = currentSettings[section];
                        if (typeof sectionObj === 'object' && sectionObj[key] !== undefined) return sectionObj[key];
                    }
                    // Handle common conversions (e.g. extension.section.key)
                    if (currentSettings[key] !== undefined) return currentSettings[key];

                    // Fallback for editor settings
                    if (fullKey === 'editor.tabSize') return currentSettings.tab_size ?? 4;
                    if (fullKey === 'editor.fontSize') return currentSettings.font_size ?? 14;

                    return defaultValue;
                },
                has: (key) => {
                    const fullKey = section ? `${section}.${key}` : key;
                    return currentSettings[fullKey] !== undefined || (section && currentSettings[section] && currentSettings[section][key] !== undefined);
                },
                update: (key, value, target) => {
                    sendResponse({ type: 'configurationUpdate', section, key, value, target });
                    return Promise.resolve();
                },
                inspect: (key) => {
                    const fullKey = section ? `${section}.${key}` : key;
                    return { key, defaultValue: undefined, globalValue: currentSettings[fullKey] };
                }
            };
        },
        onDidChangeTextDocument: (callback) => eventHandlers.on('onDidChangeTextDocument', callback),
        onDidOpenTextDocument: (callback) => eventHandlers.on('onDidOpenTextDocument', callback),
        onDidCloseTextDocument: (callback) => eventHandlers.on('onDidCloseTextDocument', callback),
        onDidSaveTextDocument: (callback) => eventHandlers.on('onDidSaveTextDocument', callback),
        onDidChangeConfiguration: (callback) => eventHandlers.on('onDidChangeConfiguration', callback),
        onDidChangeWorkspaceFolders: (callback) => eventHandlers.on('onDidChangeWorkspaceFolders', callback),
        findFiles: async (include, exclude, maxResults) => {
            return sendRequest({ type: 'findFiles', include, exclude, maxResults });
        },
        openTextDocument: async (uriOrPath) => {
            const p = typeof uriOrPath === 'string' ? uriOrPath : uriOrPath.fsPath;
            const existing = vscodeImpl.workspace.textDocuments.find(d => d.uri === p || d.uri?.fsPath === p);
            if (existing) return existing;
            try {
                const content = fs.readFileSync(p.replace(/^file:\/\/\/?/, ''), 'utf8');
                const doc = { uri: { fsPath: p, toString: () => p }, fileName: p, getText: () => content, languageId: 'plaintext', version: 1 };
                vscodeImpl.workspace.textDocuments.push(doc);
                return doc;
            } catch { return null; }
        },
        applyEdit: async (edit) => {
            sendResponse({ type: 'applyEdit', edit: edit._edits ?? [] });
            return true;
        },
        createFileSystemWatcher: (globPattern) => ({
            onDidCreate: (cb) => eventHandlers.on('fileCreated', cb),
            onDidChange: (cb) => eventHandlers.on('fileChanged', cb),
            onDidDelete: (cb) => eventHandlers.on('fileDeleted', cb),
            dispose: () => { }
        }),
        name: path.basename(process.cwd()),
        isTrusted: true,
    },

    languages: {
        // ── Completion ────────────────────────────────────────────────────────
        registerCompletionItemProvider: (selector, provider, ...triggerChars) => {
            const id = nextProviderId();
            completionProviders.set(id, { selector, provider, triggerChars });
            sendResponse({ type: 'providerRegistered', kind: 'completion', id, selector, triggerChars });
            return { dispose: () => completionProviders.delete(id) };
        },

        // ── Hover ─────────────────────────────────────────────────────────────
        registerHoverProvider: (selector, provider) => {
            const id = nextProviderId();
            hoverProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'hover', id, selector });
            return { dispose: () => hoverProviders.delete(id) };
        },

        // ── Definition ────────────────────────────────────────────────────────
        registerDefinitionProvider: (selector, provider) => {
            const id = nextProviderId();
            definitionProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'definition', id, selector });
            return { dispose: () => definitionProviders.delete(id) };
        },
        registerDeclarationProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),
        registerTypeDefinitionProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),
        registerImplementationProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),

        // ── References ────────────────────────────────────────────────────────
        registerReferenceProvider: (selector, provider) => {
            const id = nextProviderId();
            referenceProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'reference', id, selector });
            return { dispose: () => referenceProviders.delete(id) };
        },

        // ── Symbols ──────────────────────────────────────────────────────────
        registerDocumentSymbolProvider: (selector, provider) => {
            const id = nextProviderId();
            documentSymbolProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'documentSymbol', id, selector });
            return { dispose: () => documentSymbolProviders.delete(id) };
        },
        registerWorkspaceSymbolProvider: (provider) => {
            const id = nextProviderId();
            workspaceSymbolProviders.set(id, { provider });
            sendResponse({ type: 'providerRegistered', kind: 'workspaceSymbol', id });
            return { dispose: () => workspaceSymbolProviders.delete(id) };
        },

        // ── Formatting ────────────────────────────────────────────────────────
        registerDocumentFormattingEditProvider: (selector, provider) => {
            const id = nextProviderId();
            formattingProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'formatting', id, selector });
            return { dispose: () => formattingProviders.delete(id) };
        },
        registerDocumentRangeFormattingEditProvider: (selector, provider) => {
            const id = nextProviderId();
            rangeFormattingProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'rangeFormatting', id, selector });
            return { dispose: () => rangeFormattingProviders.delete(id) };
        },

        // ── Other Providers ───────────────────────────────────────────────────
        registerSignatureHelpProvider: (selector, provider, ...triggerChars) => {
            const id = nextProviderId();
            signatureHelpProviders.set(id, { selector, provider, triggerChars });
            sendResponse({ type: 'providerRegistered', kind: 'signatureHelp', id, selector, triggerChars });
            return { dispose: () => signatureHelpProviders.delete(id) };
        },
        registerRenameProvider: (selector, provider) => {
            const id = nextProviderId();
            renameProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'rename', id, selector });
            return { dispose: () => renameProviders.delete(id) };
        },
        registerCodeActionsProvider: (selector, provider, metadata) => {
            const id = nextProviderId();
            codeActionProviders.set(id, { selector, provider, metadata });
            sendResponse({ type: 'providerRegistered', kind: 'codeAction', id, selector });
            return { dispose: () => codeActionProviders.delete(id) };
        },
        registerCodeLensProvider: (selector, provider) => {
            const id = nextProviderId();
            codeLensProviders.set(id, { selector, provider });
            sendResponse({ type: 'providerRegistered', kind: 'codeLens', id, selector });
            return { dispose: () => codeLensProviders.delete(id) };
        },

        // ── Diagnostics ───────────────────────────────────────────────────────
        createDiagnosticCollection: (name) => {
            const coll = name ?? `diag_${nextProviderId()}`;
            if (!diagnosticCollections.has(coll)) diagnosticCollections.set(coll, new Map());
            const store = diagnosticCollections.get(coll);
            return {
                name: coll,
                set: (uriOrEntries, diags) => {
                    if (Array.isArray(uriOrEntries)) {
                        for (const [u, d] of uriOrEntries) {
                            const key = typeof u === 'string' ? u : u.toString();
                            store.set(key, d);
                            sendResponse({ type: 'diagnostics', uri: key, diagnostics: d, collection: coll });
                        }
                    } else {
                        const key = typeof uriOrEntries === 'string' ? uriOrEntries : uriOrEntries.toString();
                        store.set(key, diags ?? []);
                        sendResponse({ type: 'diagnostics', uri: key, diagnostics: diags ?? [], collection: coll });
                    }
                },
                delete: (uri) => {
                    const key = typeof uri === 'string' ? uri : uri.toString();
                    store.delete(key);
                    sendResponse({ type: 'diagnostics', uri: key, diagnostics: [], collection: coll });
                },
                clear: () => {
                    for (const [k] of store) {
                        sendResponse({ type: 'diagnostics', uri: k, diagnostics: [], collection: coll });
                    }
                    store.clear();
                },
                forEach: (callback) => store.forEach(callback),
                get: (uri) => store.get(typeof uri === 'string' ? uri : uri.toString()),
                has: (uri) => store.has(typeof uri === 'string' ? uri : uri.toString()),
                dispose: () => diagnosticCollections.delete(coll),
            };
        },

        getDiagnostics: (uri) => {
            if (uri) {
                const key = uri.toString();
                const result = [];
                for (const [, store] of diagnosticCollections) {
                    const d = store.get(key);
                    if (d) result.push(...d);
                }
                return result;
            }
            const all = new Map();
            for (const [, store] of diagnosticCollections) {
                for (const [k, v] of store) {
                    if (!all.has(k)) all.set(k, []);
                    all.get(k).push(...v);
                }
            }
            return [...all.entries()];
        },
        onDidChangeDiagnostics: (cb) => eventHandlers.on('onDidChangeDiagnostics', cb),
        match: (selector, doc) => selectorMatches(selector, doc) ? 10 : 0,
        setTextDocumentLanguage: (doc, lang) => Promise.resolve(doc),
    },

    env: {
        language: 'en',
        appName: 'VSCodium Rust',
        appRoot: __dirname,
        machineId: require('crypto').randomBytes(16).toString('hex'),
        sessionId: require('crypto').randomBytes(16).toString('hex'),
        uriScheme: 'vscode',
        clipboard: {
            readText: () => Promise.resolve(''),
            writeText: (text) => Promise.resolve()
        },
        openExternal: (uri) => {
            sendResponse({ type: 'openExternal', uri: uri.toString() });
            return Promise.resolve(true);
        },
        asExternalUri: (uri) => Promise.resolve(uri),
    },

    scm: {
        createSourceControl: (id, label, rootUri) => ({
            id, label, rootUri,
            inputBox: { value: '', placeholder: '' },
            count: 0,
            statusBarCommands: [],
            createResourceGroup: (id, label) => ({
                id, label,
                hideWhenEmpty: false,
                resourceStates: [],
                dispose: () => { }
            }),
            dispose: () => { }
        })
    },

    extensions: {
        getExtension: (id) => {
            const meta = loadedExtensions.get(id);
            if (!meta) return undefined;
            return { id, extensionPath: meta.extensionPath, isActive: extensions.has(id), exports: extensions.get(id)?.instance };
        },
        all: [],
        onDidChange: (cb) => eventHandlers.on('extensionsChanged', cb),
    },

    Uri: {
        file: (p) => ({ scheme: 'file', fsPath: p, path: p.replace(/\\/g, '/'), toString: () => `file:///${p.replace(/\\/g, '/')}` }),
        parse: (str) => {
            const fsPath = str.replace(/^file:\/\/\/?/, '');
            return { scheme: 'file', fsPath, path: '/' + fsPath.replace(/\\/g, '/'), toString: () => str };
        },
        joinPath: (uri, ...parts) => {
            const joined = path.join(uri.fsPath, ...parts);
            return { scheme: 'file', fsPath: joined, path: joined.replace(/\\/g, '/'), toString: () => `file:///${joined.replace(/\\/g, '/')}` };
        }
    },

    Range: class {
        constructor(startLine, startChar, endLine, endChar) {
            this.start = { line: startLine, character: startChar };
            this.end = { line: endLine, character: endChar };
        }
        contains(pos) { return true; }
        intersection(other) { return this; }
        union(other) { return this; }
        isEmpty() { return this.start.line === this.end.line && this.start.character === this.end.character; }
        isSingleLine() { return this.start.line === this.end.line; }
    },

    Position: class {
        constructor(line, character) { this.line = line; this.character = character; }
        isBefore(other) { return this.line < other.line || (this.line === other.line && this.character < other.character); }
        isAfter(other) { return !this.isBefore(other) && !this.isEqual(other); }
        isEqual(other) { return this.line === other.line && this.character === other.character; }
        translate(lineDelta, charDelta) { return new vscodeImpl.Position(this.line + (lineDelta || 0), this.character + (charDelta || 0)); }
        with(line, character) { return new vscodeImpl.Position(line ?? this.line, character ?? this.character); }
    },

    Location: class {
        constructor(uri, rangeOrPosition) { this.uri = uri; this.range = rangeOrPosition; }
    },

    Diagnostic: class {
        constructor(range, message, severity) {
            this.range = range; this.message = message; this.severity = severity ?? 0;
            this.source = undefined; this.code = undefined; this.relatedInformation = [];
        }
    },

    DiagnosticSeverity: { Error: 0, Warning: 1, Information: 2, Hint: 3 },

    CompletionItem: class {
        constructor(label, kind) { this.label = label; this.kind = kind; }
    },

    CompletionItemKind: {
        Text: 0, Method: 1, Function: 2, Constructor: 3, Field: 4,
        Variable: 5, Class: 6, Interface: 7, Module: 8, Property: 9,
        Unit: 10, Value: 11, Enum: 12, Keyword: 13, Snippet: 14,
        Color: 15, File: 16, Reference: 17, Folder: 18, EnumMember: 19,
        Constant: 20, Struct: 21, Event: 22, Operator: 23, TypeParameter: 24
    },

    CompletionList: class {
        constructor(items, isIncomplete) { this.items = items ?? []; this.isIncomplete = isIncomplete ?? false; }
    },

    Hover: class {
        constructor(contents, range) { this.contents = Array.isArray(contents) ? contents : [contents]; this.range = range; }
    },

    MarkdownString: class {
        constructor(value) { this.value = value ?? ''; this.isTrusted = false; }
        appendText(val) { this.value += val; return this; }
        appendMarkdown(val) { this.value += val; return this; }
        appendCodeblock(val, lang) { this.value += `\`\`\`${lang || ''}\n${val}\n\`\`\``; return this; }
    },

    TextEdit: {
        replace: (range, newText) => ({ range, newText }),
        insert: (position, newText) => ({ range: { start: position, end: position }, newText }),
        delete: (range) => ({ range, newText: '' }),
        setEndOfLine: (eol) => ({}),
    },

    WorkspaceEdit: class {
        constructor() { this._edits = []; }
        replace(uri, range, newText) { this._edits.push({ uri: uri.toString(), range, newText }); }
        insert(uri, position, newText) { this._edits.push({ uri: uri.toString(), range: { start: position, end: position }, newText }); }
        delete(uri, range) { this._edits.push({ uri: uri.toString(), range, newText: '' }); }
        has(uri) { return this._edits.some(e => e.uri === uri.toString()); }
        set(uri, edits) { for (const e of edits) this._edits.push({ uri: uri.toString(), ...e }); }
        get(uri) { return this._edits.filter(e => e.uri === uri.toString()); }
        get size() { return this._edits.length; }
    },

    EventEmitter: class {
        constructor() {
            this._listeners = [];
            this.event = (listener) => {
                this._listeners.push(listener);
                return { dispose: () => { const i = this._listeners.indexOf(listener); if (i >= 0) this._listeners.splice(i, 1); } };
            };
        }
        fire(data) { for (const l of this._listeners) try { l(data); } catch (e) { } }
        dispose() { this._listeners = []; }
    },

    StatusBarAlignment: { Left: 1, Right: 2 },
    ViewColumn: { Active: -1, Beside: -2, One: 1, Two: 2, Three: 3 },
    EndOfLine: { LF: 1, CRLF: 2 },
    FileType: { Unknown: 0, File: 1, Directory: 2, SymbolicLink: 64 },
    ConfigurationTarget: { Global: 1, Workspace: 2, WorkspaceFolder: 3 },
    TreeItemCollapsibleState: { None: 0, Collapsed: 1, Expanded: 2 },
    TreeItem: class { constructor(label, collapsibleState) { this.label = label; this.collapsibleState = collapsibleState ?? 0; } },
    ThemeIcon: class { constructor(id, color) { this.id = id; this.color = color; } },
    ThemeColor: class { constructor(id) { this.id = id; } },
    SymbolKind: { File: 0, Module: 1, Namespace: 2, Package: 3, Class: 4, Method: 5, Property: 6, Field: 7, Constructor: 8, Enum: 9, Interface: 10, Function: 11, Variable: 12, Constant: 13 },
    DocumentSymbol: class { constructor(name, detail, kind, range, selectionRange) { this.name = name; this.detail = detail; this.kind = kind; this.range = range; this.selectionRange = selectionRange; this.children = []; } },
    SnippetString: class { constructor(value) { this.value = value ?? ''; } },
    CancellationTokenSource: class { constructor() { this.token = { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) }; } cancel() { this.token.isCancellationRequested = true; } dispose() { } },
    // ── Commonly-used classes/enums real extensions need on load ──
    Disposable: Object.assign(
        class { constructor(fn) { this._fn = fn; } dispose() { try { this._fn && this._fn(); } catch { } } },
        { from: (...items) => ({ dispose: () => items.forEach(i => { try { i && i.dispose && i.dispose(); } catch { } }) }) }
    ),
    Selection: class { constructor(a, b, c, d) { if (typeof a === 'object') { this.anchor = a; this.active = b; this.start = a; this.end = b; } else { this.anchor = { line: a, character: b }; this.active = { line: c, character: d }; this.start = this.anchor; this.end = this.active; } this.isEmpty = false; } },
    ProgressLocation: { SourceControl: 1, Window: 10, Notification: 15 },
    CodeActionKind: { Empty: '', QuickFix: 'quickfix', Refactor: 'refactor', RefactorExtract: 'refactor.extract', RefactorInline: 'refactor.inline', RefactorRewrite: 'refactor.rewrite', Source: 'source', SourceOrganizeImports: 'source.organizeImports', SourceFixAll: 'source.fixAll' },
    CodeAction: class { constructor(title, kind) { this.title = title; this.kind = kind; this.diagnostics = []; this.command = undefined; this.edit = undefined; this.isPreferred = false; } },
    RelativePattern: class { constructor(base, pattern) { this.base = base; this.baseUri = base; this.pattern = pattern; } },
    SymbolInformation: class { constructor(name, kind, containerName, location) { this.name = name; this.kind = kind; this.containerName = containerName; this.location = location; } },
    CodeLens: class { constructor(range, command) { this.range = range; this.command = command; this.isResolved = !!command; } },
    version: '1.96.0',
};

const vscode = createStubProxy(vscodeImpl);
global.vscode = vscode;
global.acquireVsCodeApi = () => vscode; // Webview compat

// Extensions call require('vscode') — hook Module._load before any extension activates.
const Module = require('module');
const originalLoad = Module._load;
Module._load = function vscodeModuleShim(request, parent, isMain) {
    if (request === 'vscode') return vscode;
    return originalLoad.call(this, request, parent, isMain);
};

const commands = new Map();
const extensions = new Map();
const loadedExtensions = new Map();

// ── Additional Registry ──────────────────────────────────────────────────────
const referenceProviders = new Map();
const documentSymbolProviders = new Map();
const workspaceSymbolProviders = new Map();
const signatureHelpProviders = new Map();
const renameProviders = new Map();
const codeActionProviders = new Map();
const codeLensProviders = new Map();
const formattingProviders = new Map();
const rangeFormattingProviders = new Map();

function getLanguageId(filePath) {
    const ext = filePath.split('.').pop().toLowerCase();
    const map = { rs: 'rust', ts: 'typescript', js: 'javascript', py: 'python', c: 'c', cpp: 'cpp', cs: 'csharp', go: 'go', md: 'markdown', json: 'json' };
    return map[ext] ?? 'plaintext';
}

// ── Message handlers ──────────────────────────────────────────────────────────
rl.on('line', (line) => {
    try {
        const request = JSON.parse(line);
        handleRequest(request);
    } catch (e) {
        sendResponse({ type: 'error', message: 'Failed to parse request: ' + e.message });
    }
});

async function handleRequest(req) {
    // Handle response correlation first
    if (req._reqId && pendingRequests.has(req._reqId)) {
        pendingRequests.get(req._reqId)(req.result ?? null);
        pendingRequests.delete(req._reqId);
        return;
    }

    switch (req.type) {
        case 'bootstrap':
            await bootstrap(req.extensions);
            break;

        case 'syncWorkspaceFolders': {
            if (Array.isArray(req.folders) && req.folders.length > 0) {
                vscodeImpl.workspace.workspaceFolders = req.folders.map((f, i) => {
                    const fpath = f.path || (f.uri || '').replace(/^file:\/\/\/?/, '');
                    const uri = f.uri || `file:///${fpath.replace(/\\/g, '/')}`;
                    return {
                        uri: { fsPath: fpath, toString: () => uri },
                        name: f.name || path.basename(fpath),
                        index: i,
                    };
                });
                vscodeImpl.workspace.rootPath = vscodeImpl.workspace.workspaceFolders[0].uri.fsPath;
                eventHandlers.emit('onDidChangeWorkspaceFolders', {
                    added: vscodeImpl.workspace.workspaceFolders,
                    removed: [],
                });
            }
            break;
        }

        case 'activateExtension':
            await activateExtension(req.id);
            break;

        case 'documentOpened': {
            const normUri = normalizeDocUri(req.uri);
            const existingIdx = vscodeImpl.workspace.textDocuments.findIndex(d =>
                normalizeDocUri(d.uri?.toString?.() ?? d.uri) === normUri
            );
            if (existingIdx >= 0) {
                vscodeImpl.workspace.textDocuments.splice(existingIdx, 1);
            }
            const doc = {
                uri: { fsPath: normUri, toString: () => req.uri || normUri },
                fileName: normUri,
                languageId: req.languageId,
                version: req.version ?? 1,
                getText: () => req.content,
                lineCount: (req.content || '').split('\n').length,
                lineAt: (line) => ({ text: (req.content || '').split('\n')[line] ?? '', lineNumber: line }),
                content: req.content
            };
            vscodeImpl.workspace.textDocuments.push(doc);
            eventHandlers.emit('onDidOpenTextDocument', doc);
            eventHandlers.emit('onDidChangeActiveTextEditor', { document: doc });
            break;
        }

        case 'documentChanged': {
            const normUri = normalizeDocUri(req.uri);
            const existingDoc = vscodeImpl.workspace.textDocuments.find(d =>
                normalizeDocUri(d.uri?.toString?.() ?? d.uri) === normUri
            );
            if (existingDoc) {
                existingDoc.content = req.content;
                existingDoc.getText = () => req.content;
                existingDoc.version = req.version ?? (existingDoc.version + 1);
                existingDoc.lineCount = (req.content || '').split('\n').length;
                eventHandlers.emit('onDidChangeTextDocument', { document: existingDoc, contentChanges: [{ text: req.content }] });
            }
            break;
        }

        case 'documentSaved': {
            const normUri = normalizeDocUri(req.uri);
            const savedDoc = vscodeImpl.workspace.textDocuments.find(d =>
                normalizeDocUri(d.uri?.toString?.() ?? d.uri) === normUri
            );
            if (savedDoc) eventHandlers.emit('onDidSaveTextDocument', savedDoc);
            break;
        }

        case 'documentClosed': {
            const normUri = normalizeDocUri(req.uri);
            const idx = vscodeImpl.workspace.textDocuments.findIndex(d =>
                normalizeDocUri(d.uri?.toString?.() ?? d.uri) === normUri
            );
            if (idx >= 0) {
                const [closed] = vscodeImpl.workspace.textDocuments.splice(idx, 1);
                eventHandlers.emit('onDidCloseTextDocument', closed);
            }
            break;
        }

        // ── Provider request relay ─────────────────────────────────────────────
        case 'provideCompletions': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            const allItems = [];
            for (const [id, { selector, provider }] of completionProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideCompletionItems(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) }, { triggerKind: 0, triggerCharacter: req.triggerChar });
                    const items = result?.items ?? (Array.isArray(result) ? result : []);
                    allItems.push(...items);
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'completion', reqId: req.reqId, items: allItems });
            break;
        }

        case 'provideHover': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            for (const [id, { selector, provider }] of hoverProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideHover(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'hover', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'hover', reqId: req.reqId, result: null });
            break;
        }

        case 'provideDefinition': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            for (const [id, { selector, provider }] of definitionProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideDefinition(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'definition', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'definition', reqId: req.reqId, result: null });
            break;
        }

        case 'provideReferences': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            const allRefs = [];
            for (const [id, { selector, provider }] of referenceProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideReferences(doc, pos, { includeDeclaration: true }, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) allRefs.push(...(Array.isArray(result) ? result : [result]));
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'reference', reqId: req.reqId, result: allRefs });
            break;
        }

        case 'provideDocumentSymbols': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            for (const [id, { selector, provider }] of documentSymbolProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideDocumentSymbols(doc, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'documentSymbol', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'documentSymbol', reqId: req.reqId, result: [] });
            break;
        }

        case 'provideSignatureHelp': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            for (const [id, { selector, provider }] of signatureHelpProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideSignatureHelp(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) }, { triggerKind: 1 });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'signatureHelp', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'signatureHelp', reqId: req.reqId, result: null });
            break;
        }

        case 'provideRename': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            const pos = new vscodeImpl.Position(req.line, req.character);
            for (const [id, { selector, provider }] of renameProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideRenameEdits(doc, pos, req.newName, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'rename', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'rename', reqId: req.reqId, result: null });
            break;
        }

        case 'provideFormatting': {
            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
            for (const [id, { selector, provider }] of formattingProviders) {
                if (!selectorMatches(selector, doc)) continue;
                try {
                    const result = await provider.provideDocumentFormattingEdits(doc, { tabSize: req.tabSize, insertSpaces: req.insertSpaces }, { isCancellationRequested: false, onCancellationRequested: () => ({ dispose: () => { } }) });
                    if (result) {
                        sendResponse({ type: 'providerResult', kind: 'formatting', reqId: req.reqId, result });
                        return;
                    }
                } catch (e) { }
            }
            sendResponse({ type: 'providerResult', kind: 'formatting', reqId: req.reqId, result: [] });
            break;
        }

        case 'settingsUpdated':
            currentSettings = { ...currentSettings, ...req.settings };
            eventHandlers.emit('onDidChangeConfiguration', { affectsConfiguration: () => true });
            break;

        case 'ping':
            sendResponse({ type: 'pong' });
            break;

        case 'executeCommand': {
            const cmd = commands.get(req.id);
            if (cmd) {
                try { await cmd(...(req.args || [])); }
                catch (e) { sendResponse({ type: 'error', message: `Command ${req.id} failed: ${e.message}` }); }
            } else {
                sendResponse({ type: 'error', message: `Command ${req.id} not found` });
            }
            break;
        }

        case 'load_extension': {
            const meta = req.metadata;
            if (meta?.id) {
                loadedExtensions.set(meta.id, meta);
                if (meta.activationEvents?.includes('*')) await activateExtension(meta.id);
            }
            break;
        }

        case 'webviewMessage': {
            eventHandlers.emit(`webview:${req.viewType}:message`, req.message);
            break;
        }

        default:
            sendResponse({ type: 'error', message: `Unknown request type: ${req.type}` });
    }
}

async function bootstrap(extensionMetadataList) {
    for (const meta of (extensionMetadataList ?? [])) {
        loadedExtensions.set(meta.id, meta);
        if (meta.activationEvents?.includes('*')) await activateExtension(meta.id);
    }
    sendResponse({ type: 'ready', count: loadedExtensions.size });
}

function normalizeDocUri(uri) {
    if (!uri) return '';
    return String(uri).replace(/^file:\/\/\/?/, '').replace(/\\/g, '/');
}

async function activateExtension(extId) {
    const meta = loadedExtensions.get(extId);
    if (!meta || extensions.has(extId)) return;

    try {
        const extPath = meta.extensionPath || meta.extension_path;
        if (!extPath) throw new Error('extensionPath missing from metadata');
        const mainRel = meta.main || './extension.js';
        const mainFile = path.resolve(extPath, mainRel);
        if (!fs.existsSync(mainFile)) {
            throw new Error(`Extension entry not found: ${mainFile}`);
        }
        const extension = require(mainFile);

        if (typeof extension?.activate === 'function') {
            const context = {
                subscriptions: [],
                extensionPath: extPath,
                extensionUri: vscodeImpl.Uri.file(extPath),
                storagePath: path.join(extPath, '.storage'),
                storageUri: vscodeImpl.Uri.file(path.join(extPath, '.storage')),
                globalStoragePath: path.join(__dirname, 'global-storage', extId),
                globalStorageUri: vscodeImpl.Uri.file(path.join(__dirname, 'global-storage', extId)),
                logPath: path.join(__dirname, 'logs', extId),
                logUri: vscodeImpl.Uri.file(path.join(__dirname, 'logs', extId)),
                workspaceState: {
                    get: (key, def) => def,
                    update: (key, val) => Promise.resolve(),
                    keys: () => []
                },
                globalState: {
                    get: (key, def) => def,
                    update: (key, val) => Promise.resolve(),
                    keys: () => [],
                    setKeysForSync: () => { }
                },
                secrets: {
                    get: (key) => Promise.resolve(undefined),
                    store: (key, val) => Promise.resolve(),
                    delete: (key) => Promise.resolve(),
                    onDidChange: (cb) => eventHandlers.on('secretChanged', cb),
                },
                asAbsolutePath: (relPath) => path.join(extPath, relPath),
                environmentVariableCollection: createStubProxy({}, 'envVarCollection'),
                extension: { id: extId, extensionPath: extPath, isActive: true, packageJSON: meta, exports: undefined },
            };
            await extension.activate(context);
            extensions.set(extId, { metadata: meta, instance: extension, context });
            sendResponse({ type: 'extensionActivated', id: extId });
        }
    } catch (e) {
        console.error(`[ext-host] Failed to activate ${extId}:`, e.message);
        sendResponse({ type: 'extensionActivationFailed', id: extId, error: e.message });
    }
}
