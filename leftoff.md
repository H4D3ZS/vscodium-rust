● Step 7: Extensions — expand ext-host/index.js with real IPC for completion, hover, definition, diagnostics, and workspace.getConfiguration reading real settings:

● Write(ext-host\index.js)
  ⎿  Added 645 lines, removed 144 lines
      1  /**
      2   * VSCodium Rust Extension Host
      3 - * This process executes third-party extensions in an isolated environment.
      3 + * Executes third-party extensions in an isolated environment.
      4 + * Provider requests (completion, hover, definition, diagnostics) are forwarded
      5 + * to the IDE via stdout JSON-line IPC.
      6   */
      7
      8  const readline = require('readline');
     ...
       15      terminal: false
       16  });
       17
       16 -// Deep Proxy to prevent crashes on unimplemented vscode APIs
       17 -function createStubProxy(target, path = 'vscode') {
       18 +// ── Deep Proxy (prevent crashes on unimplemented APIs) ────────────────────────
       19 +function createStubProxy(target, apiPath = 'vscode') {
       20      return new Proxy(target, {
       21          get(obj, prop) {
       20 -            if (prop in obj) {
       21 -                return obj[prop];
       22 -            }
       22 +            if (prop in obj) return obj[prop];
       23              if (typeof prop === 'string' && prop !== 'then') {
       24 -                // Return a callable object that also acts as a proxy
       24                  const stub = function () {
       26 -                    // console.warn(`[Stub Callback] ${path}.${prop}() was called with`, arguments);
       27 -                    return createStubProxy({}, `${path}.${prop}()`);
       25 +                    return createStubProxy({}, `${apiPath}.${prop}()`);
       26                  };
       29 -                return createStubProxy(stub, `${path}.${prop}`);
       27 +                return createStubProxy(stub, `${apiPath}.${prop}`);
       28              }
       29              return undefined;
       30          }
       31      });
       32  }
       33
       36 -// The global vscode API available to extensions
       34 +// ── Settings cache (refreshed on 'settingsUpdated' messages) ─────────────────
       35 +let currentSettings = { theme: 'vs-dark', font_size: 14, tab_size: 4 };
       36 +
       37 +// ── Provider registry ─────────────────────────────────────────────────────────
       38 +// Providers registered by extensions; keyed by a unique id.
       39 +const completionProviders = new Map();  // id → { selector, provider, triggerChars }
       40 +const hoverProviders      = new Map();  // id → { selector, provider }
       41 +const definitionProviders = new Map();  // id → { selector, provider }
       42 +const diagnosticCollections = new Map(); // name → Map<uri, diagnostics[]>
       43 +
       44 +let providerIdCounter = 0;
       45 +function nextProviderId() { return `p${++providerIdCounter}`; }
       46 +
       47 +// Match a document to a selector (language, pattern, scheme)
       48 +function selectorMatches(selector, doc) {
       49 +    if (!selector) return true;
       50 +    const sel = typeof selector === 'string' ? { language: selector } : selector;
       51 +    if (sel.language && doc.languageId !== sel.language) return false;
       52 +    return true;
       53 +}
       54 +
       55 +// ── Event emitter ─────────────────────────────────────────────────────────────
       56 +const eventHandlers = {
       57 +    handlers: new Map(),
       58 +    on(event, cb) {
       59 +        if (!this.handlers.has(event)) this.handlers.set(event, []);
       60 +        this.handlers.get(event).push(cb);
       61 +        return { dispose: () => {
       62 +            const arr = this.handlers.get(event) || [];
       63 +            const idx = arr.indexOf(cb);
       64 +            if (idx >= 0) arr.splice(idx, 1);
       65 +        }};
       66 +    },
       67 +    emit(event, ...args) {
       68 +        (this.handlers.get(event) || []).forEach(cb => {
       69 +            try { cb(...args); } catch(e) { /* swallow */ }
       70 +        });
       71 +    }
       72 +};
       73 +
       74 +// ── IPC ───────────────────────────────────────────────────────────────────────
       75 +function sendResponse(obj) {
       76 +    process.stdout.write(JSON.stringify(obj) + '\n');
       77 +}
       78 +
       79 +// Pending request map for request/response correlation
       80 +const pendingRequests = new Map();
       81 +let reqCounter = 0;
       82 +
       83 +function sendRequest(req) {
       84 +    return new Promise((resolve) => {
       85 +        const id = `req_${++reqCounter}`;
       86 +        pendingRequests.set(id, resolve);
       87 +        sendResponse({ ...req, _reqId: id });
       88 +        // Timeout after 5 s to avoid hangs
       89 +        setTimeout(() => {
       90 +            if (pendingRequests.has(id)) {
       91 +                pendingRequests.delete(id);
       92 +                resolve(null);
       93 +            }
       94 +        }, 5000);
       95 +    });
       96 +}
       97 +
       98 +// ── VSCode API implementation ─────────────────────────────────────────────────
       99  const vscodeImpl = {
      100      window: {
       39 -        showInformationMessage: (msg) => {
       40 -            sendResponse({ type: 'notification', level: 'info', message: msg });
      101 +        showInformationMessage: (msg, ...items) => {
      102 +            sendResponse({ type: 'notification', level: 'info', message: msg, items });
      103 +            return Promise.resolve(undefined);
      104          },
       42 -        showErrorMessage: (msg) => {
       43 -            sendResponse({ type: 'notification', level: 'error', message: msg });
      105 +        showErrorMessage: (msg, ...items) => {
      106 +            sendResponse({ type: 'notification', level: 'error', message: msg, items });
      107 +            return Promise.resolve(undefined);
      108          },
       45 -        showWarningMessage: (msg) => {
       46 -            sendResponse({ type: 'notification', level: 'warning', message: msg });
      109 +        showWarningMessage: (msg, ...items) => {
      110 +            sendResponse({ type: 'notification', level: 'warning', message: msg, items });
      111 +            return Promise.resolve(undefined);
      112          },
       48 -        createTextEditorDecorationType: () => ({ dispose: () => { } }),
      113 +        showInputBox: (options) => {
      114 +            return sendRequest({ type: 'showInputBox', options });
      115 +        },
      116 +        showQuickPick: (items, options) => {
      117 +            return sendRequest({ type: 'showQuickPick', items, options });
      118 +        },
      119 +        createTextEditorDecorationType: (options) => ({
      120 +            key: `deco_${Date.now()}`,
      121 +            dispose: () => {}
      122 +        }),
      123          createOutputChannel: (name) => ({
      124              name,
       51 -            append: (val) => console.error(`[Output:${name}] ${val}`),
       52 -            appendLine: (val) => console.error(`[Output:${name}] ${val}`),
       53 -            clear: () => { },
       54 -            show: () => { },
       55 -            hide: () => { },
       56 -            dispose: () => { }
      125 +            append: (val) => sendResponse({ type: 'outputChannel', channel: name, text: val }),
      126 +            appendLine: (val) => sendResponse({ type: 'outputChannel', channel: name, text: val + '\n' }),
      127 +            clear: () => {},
      128 +            show: () => {},
      129 +            hide: () => {},
      130 +            dispose: () => {}
      131          }),
      132 +        createWebviewPanel: (viewType, title, showOptions, options) => {
      133 +            const panel = {
      134 +                webview: {
      135 +                    html: '',
      136 +                    onDidReceiveMessage: (cb) => eventHandlers.on(`webview:${viewType}:message`, cb),
      137 +                    postMessage: (msg) => sendResponse({ type: 'webviewMessage', viewType, message: msg }),
      138 +                    asWebviewUri: (uri) => uri,
      139 +                    cspSource: 'self',
      140 +                },
      141 +                title,
      142 +                visible: true,
      143 +                onDidDispose: (cb) => eventHandlers.on(`webview:${viewType}:dispose`, cb),
      144 +                onDidChangeViewState: (cb) => eventHandlers.on(`webview:${viewType}:stateChange`, cb),
      145 +                reveal: () => {},
      146 +                dispose: () => {}
      147 +            };
      148 +            sendResponse({ type: 'createWebviewPanel', viewType, title });
      149 +            return panel;
      150 +        },
      151          get activeTextEditor() {
       59 -            if (vscodeImpl.workspace.textDocuments.length > 0) {
       60 -                return {
       61 -                    document: vscodeImpl.workspace.textDocuments[0],
       62 -                    setDecorations: () => {}
       63 -                };
       64 -            }
       65 -            return undefined;
      152 +            const docs = vscodeImpl.workspace.textDocuments;
      153 +            if (docs.length === 0) return undefined;
      154 +            const doc = docs[0];
      155 +            return {
      156 +                document: doc,
      157 +                selection: { active: { line: 0, character: 0 }, anchor: { line: 0, character: 0 }, isEmpty: true },
      158 +                selections: [],
      159 +                visibleRanges: [],
      160 +                options: { tabSize: currentSettings.tab_size ?? 4 },
      161 +                setDecorations: (type, ranges) => {
      162 +                    sendResponse({ type: 'setDecorations', key: type.key, ranges });
      163 +                },
      164 +                edit: (callback) => Promise.resolve(false),
      165 +            };
      166          },
      167          visibleTextEditors: [],
       68 -        onDidChangeActiveTextEditor: (cb) => {
       69 -            eventHandlers.on('onDidChangeActiveTextEditor', cb);
      168 +        onDidChangeActiveTextEditor: (cb) => eventHandlers.on('onDidChangeActiveTextEditor', cb),
      169 +        onDidChangeTextEditorSelection: (cb) => eventHandlers.on('onDidChangeTextEditorSelection', cb),
      170 +        onDidChangeVisibleTextEditors: (cb) => eventHandlers.on('onDidChangeVisibleTextEditors', cb),
      171 +        withProgress: (options, task) => {
      172 +            return task({ report: () => {} }, { isCancellationRequested: false, onCancellationRequested: () => ({dispose: () => {}}) });
      173 +        },
      174 +        registerTreeDataProvider: (viewId, provider) => {
      175 +            sendResponse({ type: 'registerTreeView', viewId });
      176              return { dispose: () => {} };
       71 -        }
      177 +        },
      178 +        createTreeView: (viewId, options) => {
      179 +            sendResponse({ type: 'createTreeView', viewId });
      180 +            return { visible: true, onDidChangeVisibility: () => ({dispose:()=>{}}), reveal: () => {}, dispose: () => {} };
      181 +        },
      182      },
      183 +
      184      commands: {
      185          registerCommand: (id, callback) => {
      186              commands.set(id, callback);
     ...
      188              return { dispose: () => commands.delete(id) };
      189          },
      190          executeCommand: async (id, ...args) => {
       80 -            return await sendRequest({ type: 'executeCommand', id, args });
       81 -        }
      191 +            // Try local first, then ask the IDE
      192 +            if (commands.has(id)) {
      193 +                try { return await commands.get(id)(...args); } catch(e) {}
      194 +            }
      195 +            return sendRequest({ type: 'executeCommand', id, args });
      196 +        },
      197 +        getCommands: () => Promise.resolve([...commands.keys()]),
      198      },
      199 +
      200      workspace: {
      201          textDocuments: [],
      202          rootPath: process.cwd(),
       86 -        workspaceFolders: [{ uri: { fsPath: process.cwd() }, name: path.basename(process.cwd()), index: 0 }],
      203 +        workspaceFolders: [{
      204 +            uri: { fsPath: process.cwd(), toString: () => `file:///${process.cwd().replace(/\\/g, '/')}` },
      205 +            name: path.basename(process.cwd()),
      206 +            index: 0
      207 +        }],
      208          fs: {
      209              readFile: async (uri) => {
       89 -                return await sendRequest({ type: 'workspace.readFile', uri });
      210 +                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
      211 +                try { return Buffer.from(fs.readFileSync(fpath)); } catch { return null; }
      212              },
      213 +            writeFile: async (uri, content) => {
      214 +                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
      215 +                fs.writeFileSync(fpath, Buffer.from(content));
      216 +            },
      217              stat: async (uri) => {
       92 -                return await sendRequest({ type: 'workspace.stat', uri });
      218 +                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
      219 +                try {
      220 +                    const st = fs.statSync(fpath);
      221 +                    return { type: st.isDirectory() ? 2 : 1, ctime: st.ctimeMs, mtime: st.mtimeMs, size: st.size };
      222 +                } catch { throw new Error('File not found'); }
      223 +            },
      224 +            delete: async (uri) => {
      225 +                const fpath = typeof uri === 'string' ? uri.replace(/^file:\/\/\/?/, '') : uri.fsPath;
      226 +                fs.unlinkSync(fpath);
      227              }
      228          },
      229          getConfiguration: (section) => {
      230 +            const config = {
      231 +                editor: {
      232 +                    tabSize: currentSettings.tab_size ?? 4,
      233 +                    fontSize: currentSettings.font_size ?? 14,
      234 +                    formatOnSave: true,
      235 +                    wordWrap: 'off',
      236 +                    minimap: { enabled: true },
      237 +                },
      238 +                [section]: {}
      239 +            };
      240 +            const sectionData = config[section] ?? {};
      241              return {
       97 -                get: (key) => undefined,
       98 -                has: (key) => false,
       99 -                update: (key, value) => { },
      100 -                inspect: (key) => undefined
      242 +                get: (key, defaultValue) => {
      243 +                    if (section && sectionData[key] !== undefined) return sectionData[key];
      244 +                    return defaultValue;
      245 +                },
      246 +                has: (key) => key in sectionData,
      247 +                update: (key, value, target) => {
      248 +                    sendResponse({ type: 'configurationUpdate', section, key, value, target });
      249 +                    return Promise.resolve();
      250 +                },
      251 +                inspect: (key) => ({ key, defaultValue: undefined, globalValue: sectionData[key] })
      252              };
      253          },
      103 -        onDidChangeTextDocument: (callback) => {
      104 -            eventHandlers.on('onDidChangeTextDocument', callback);
      105 -            return { dispose: () => { } };
      254 +        onDidChangeTextDocument: (callback) => eventHandlers.on('onDidChangeTextDocument', callback),
      255 +        onDidOpenTextDocument: (callback) => eventHandlers.on('onDidOpenTextDocument', callback),
      256 +        onDidCloseTextDocument: (callback) => eventHandlers.on('onDidCloseTextDocument', callback),
      257 +        onDidSaveTextDocument: (callback) => eventHandlers.on('onDidSaveTextDocument', callback),
      258 +        onDidChangeConfiguration: (callback) => eventHandlers.on('onDidChangeConfiguration', callback),
      259 +        onDidChangeWorkspaceFolders: (callback) => eventHandlers.on('onDidChangeWorkspaceFolders', callback),
      260 +        findFiles: async (include, exclude, maxResults) => {
      261 +            return sendRequest({ type: 'findFiles', include, exclude, maxResults });
      262          },
      107 -        onDidOpenTextDocument: (callback) => {
      108 -            eventHandlers.on('onDidOpenTextDocument', callback);
      109 -            return { dispose: () => { } };
      110 -        }
      263 +        openTextDocument: async (uriOrPath) => {
      264 +            const p = typeof uriOrPath === 'string' ? uriOrPath : uriOrPath.fsPath;
      265 +            const existing = vscodeImpl.workspace.textDocuments.find(d => d.uri === p || d.uri?.fsPath === p);
      266 +            if (existing) return existing;
      267 +            try {
      268 +                const content = fs.readFileSync(p.replace(/^file:\/\/\/?/, ''), 'utf8');
      269 +                const doc = { uri: { fsPath: p, toString: () => p }, fileName: p, getText: () => content, languageId: 'plaintext', version: 1 };
      270 +                vscodeImpl.workspace.textDocuments.push(doc);
      271 +                return doc;
      272 +            } catch { return null; }
      273 +        },
      274 +        applyEdit: async (edit) => {
      275 +            sendResponse({ type: 'applyEdit', edit: edit._edits ?? [] });
      276 +            return true;
      277 +        },
      278 +        createFileSystemWatcher: (globPattern) => ({
      279 +            onDidCreate: (cb) => eventHandlers.on('fileCreated', cb),
      280 +            onDidChange: (cb) => eventHandlers.on('fileChanged', cb),
      281 +            onDidDelete: (cb) => eventHandlers.on('fileDeleted', cb),
      282 +            dispose: () => {}
      283 +        }),
      284 +        name: path.basename(process.cwd()),
      285 +        isTrusted: true,
      286      },
      287 +
      288      languages: {
      113 -        registerCompletionItemProvider: () => ({ dispose: () => {} }),
      114 -        registerDefinitionProvider: () => ({ dispose: () => {} }),
      115 -        registerHoverProvider: () => ({ dispose: () => {} }),
      116 -        createDiagnosticCollection: () => ({
      117 -            set: () => {},
      118 -            delete: () => {},
      119 -            clear: () => {},
      120 -            dispose: () => {}
      121 -        })
      289 +        // ── Completion ────────────────────────────────────────────────────────
      290 +        registerCompletionItemProvider: (selector, provider, ...triggerChars) => {
      291 +            const id = nextProviderId();
      292 +            completionProviders.set(id, { selector, provider, triggerChars });
      293 +            sendResponse({ type: 'providerRegistered', kind: 'completion', id, selector, triggerChars });
      294 +            return { dispose: () => completionProviders.delete(id) };
      295 +        },
      296 +
      297 +        // ── Hover ─────────────────────────────────────────────────────────────
      298 +        registerHoverProvider: (selector, provider) => {
      299 +            const id = nextProviderId();
      300 +            hoverProviders.set(id, { selector, provider });
      301 +            sendResponse({ type: 'providerRegistered', kind: 'hover', id, selector });
      302 +            return { dispose: () => hoverProviders.delete(id) };
      303 +        },
      304 +
      305 +        // ── Definition / Declaration / TypeDefinition ─────────────────────────
      306 +        registerDefinitionProvider: (selector, provider) => {
      307 +            const id = nextProviderId();
      308 +            definitionProviders.set(id, { selector, provider });
      309 +            sendResponse({ type: 'providerRegistered', kind: 'definition', id, selector });
      310 +            return { dispose: () => definitionProviders.delete(id) };
      311 +        },
      312 +        registerDeclarationProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),
      313 +        registerTypeDefinitionProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),
      314 +        registerImplementationProvider: (selector, provider) => vscodeImpl.languages.registerDefinitionProvider(selector, provider),
      315 +
      316 +        // ── Diagnostics ───────────────────────────────────────────────────────
      317 +        createDiagnosticCollection: (name) => {
      318 +            const coll = name ?? `diag_${nextProviderId()}`;
      319 +            if (!diagnosticCollections.has(coll)) diagnosticCollections.set(coll, new Map());
      320 +            const store = diagnosticCollections.get(coll);
      321 +            return {
      322 +                name: coll,
      323 +                set: (uriOrEntries, diags) => {
      324 +                    if (Array.isArray(uriOrEntries)) {
      325 +                        // Array of [uri, diags] tuples
      326 +                        for (const [u, d] of uriOrEntries) {
      327 +                            const key = typeof u === 'string' ? u : u.toString();
      328 +                            store.set(key, d);
      329 +                            sendResponse({ type: 'diagnostics', uri: key, diagnostics: d, collection: coll });
      330 +                        }
      331 +                    } else {
      332 +                        const key = typeof uriOrEntries === 'string' ? uriOrEntries : uriOrEntries.toString();
      333 +                        store.set(key, diags ?? []);
      334 +                        sendResponse({ type: 'diagnostics', uri: key, diagnostics: diags ?? [], collection: coll });
      335 +                    }
      336 +                },
      337 +                delete: (uri) => {
      338 +                    const key = typeof uri === 'string' ? uri : uri.toString();
      339 +                    store.delete(key);
      340 +                    sendResponse({ type: 'diagnostics', uri: key, diagnostics: [], collection: coll });
      341 +                },
      342 +                clear: () => {
      343 +                    for (const [k] of store) {
      344 +                        sendResponse({ type: 'diagnostics', uri: k, diagnostics: [], collection: coll });
      345 +                    }
      346 +                    store.clear();
      347 +                },
      348 +                forEach: (callback) => store.forEach(callback),
      349 +                get: (uri) => store.get(typeof uri === 'string' ? uri : uri.toString()),
      350 +                has: (uri) => store.has(typeof uri === 'string' ? uri : uri.toString()),
      351 +                dispose: () => diagnosticCollections.delete(coll),
      352 +            };
      353 +        },
      354 +
      355 +        // ── References ────────────────────────────────────────────────────────
      356 +        registerReferenceProvider: (selector, provider) => ({ dispose: () => {} }),
      357 +        registerDocumentSymbolProvider: (selector, provider) => ({ dispose: () => {} }),
      358 +        registerWorkspaceSymbolProvider: (provider) => ({ dispose: () => {} }),
      359 +        registerSignatureHelpProvider: (selector, provider, ...triggerChars) => ({ dispose: () => {} }),
      360 +        registerDocumentFormattingEditProvider: (selector, provider) => {
      361 +            sendResponse({ type: 'providerRegistered', kind: 'formatting', selector });
      362 +            return { dispose: () => {} };
      363 +        },
      364 +        registerDocumentRangeFormattingEditProvider: (selector, provider) => ({ dispose: () => {} }),
      365 +        registerOnTypeFormattingEditProvider: (selector, provider, firstChar) => ({ dispose: () => {} }),
      366 +        registerRenameProvider: (selector, provider) => ({ dispose: () => {} }),
      367 +        registerCodeActionsProvider: (selector, provider, metadata) => ({ dispose: () => {} }),
      368 +        registerCodeLensProvider: (selector, provider) => ({ dispose: () => {} }),
      369 +        registerFoldingRangeProvider: (selector, provider) => ({ dispose: () => {} }),
      370 +        registerColorProvider: (selector, provider) => ({ dispose: () => {} }),
      371 +        registerDocumentLinkProvider: (selector, provider) => ({ dispose: () => {} }),
      372 +        registerSelectionRangeProvider: (selector, provider) => ({ dispose: () => {} }),
      373 +        registerInlayHintsProvider: (selector, provider) => ({ dispose: () => {} }),
      374 +        registerCallHierarchyProvider: (selector, provider) => ({ dispose: () => {} }),
      375 +        getDiagnostics: (uri) => {
      376 +            if (uri) {
      377 +                const key = uri.toString();
      378 +                const result = [];
      379 +                for (const [, store] of diagnosticCollections) {
      380 +                    const d = store.get(key);
      381 +                    if (d) result.push(...d);
      382 +                }
      383 +                return result;
      384 +            }
      385 +            const all = new Map();
      386 +            for (const [, store] of diagnosticCollections) {
      387 +                for (const [k, v] of store) {
      388 +                    if (!all.has(k)) all.set(k, []);
      389 +                    all.get(k).push(...v);
      390 +                }
      391 +            }
      392 +            return [...all.entries()];
      393 +        },
      394 +        onDidChangeDiagnostics: (cb) => eventHandlers.on('onDidChangeDiagnostics', cb),
      395 +        match: (selector, doc) => selectorMatches(selector, doc) ? 10 : 0,
      396 +        setTextDocumentLanguage: (doc, lang) => Promise.resolve(doc),
      397      },
      398 +
      399      env: {
      400          language: 'en',
      401          appName: 'VSCodium Rust',
      402          appRoot: __dirname,
      127 -        machineId: '1234',
      128 -        sessionId: '5678'
      403 +        machineId: require('crypto').randomBytes(16).toString('hex'),
      404 +        sessionId: require('crypto').randomBytes(16).toString('hex'),
      405 +        uriScheme: 'vscode',
      406 +        clipboard: {
      407 +            readText: () => Promise.resolve(''),
      408 +            writeText: (text) => Promise.resolve()
      409 +        },
      410 +        openExternal: (uri) => {
      411 +            sendResponse({ type: 'openExternal', uri: uri.toString() });
      412 +            return Promise.resolve(true);
      413 +        },
      414 +        asExternalUri: (uri) => Promise.resolve(uri),
      415      },
      416 +
      417      scm: {
      131 -        createSourceControl: (id, label) => ({
      132 -            id, label,
      133 -            createResourceGroup: () => ({ resourceStates: [], dispose: () => {} }),
      418 +        createSourceControl: (id, label, rootUri) => ({
      419 +            id, label, rootUri,
      420 +            inputBox: { value: '', placeholder: '' },
      421 +            count: 0,
      422 +            statusBarCommands: [],
      423 +            createResourceGroup: (id, label) => ({
      424 +                id, label,
      425 +                hideWhenEmpty: false,
      426 +                resourceStates: [],
      427 +                dispose: () => {}
      428 +            }),
      429              dispose: () => {}
      430          })
      431      },
      137 -    version: '1.85.0'
      138 -};
      432
      140 -// Simple event emitter
      141 -const eventHandlers = {
      142 -    handlers: new Map(),
      143 -    on(event, cb) {
      144 -        if (!this.handlers.has(event)) this.handlers.set(event, []);
      145 -        this.handlers.get(event).push(cb);
      433 +    extensions: {
      434 +        getExtension: (id) => {
      435 +            const meta = loadedExtensions.get(id);
      436 +            if (!meta) return undefined;
      437 +            return { id, extensionPath: meta.extensionPath, isActive: extensions.has(id), exports: extensions.get(id)?.instance };
      438 +        },
      439 +        all: [],
      440 +        onDidChange: (cb) => eventHandlers.on('extensionsChanged', cb),
      441      },
      147 -    emit(event, ...args) {
      148 -        (this.handlers.get(event) || []).forEach(cb => cb(...args));
      149 -    }
      442 +
      443 +    Uri: {
      444 +        file: (p) => ({ scheme: 'file', fsPath: p, path: p.replace(/\\/g, '/'), toString: () => `file:///${p.replace(/\\/g, '/')}` }),
      445 +        parse: (str) => {
      446 +            const fsPath = str.replace(/^file:\/\/\/?/, '');
      447 +            return { scheme: 'file', fsPath, path: '/' + fsPath.replace(/\\/g, '/'), toString: () => str };
      448 +        },
      449 +        joinPath: (uri, ...parts) => {
      450 +            const joined = path.join(uri.fsPath, ...parts);
      451 +            return { scheme: 'file', fsPath: joined, path: joined.replace(/\\/g, '/'), toString: () => `file:///${joined.replace(/\\/g, '/')}` };
      452 +        }
      453 +    },
      454 +
      455 +    Range: class {
      456 +        constructor(startLine, startChar, endLine, endChar) {
      457 +            this.start = { line: startLine, character: startChar };
      458 +            this.end   = { line: endLine,   character: endChar };
      459 +        }
      460 +        contains(pos) { return true; }
      461 +        intersection(other) { return this; }
      462 +        union(other) { return this; }
      463 +        isEmpty() { return this.start.line === this.end.line && this.start.character === this.end.character; }
      464 +        isSingleLine() { return this.start.line === this.end.line; }
      465 +    },
      466 +
      467 +    Position: class {
      468 +        constructor(line, character) { this.line = line; this.character = character; }
      469 +        isBefore(other) { return this.line < other.line || (this.line === other.line && this.character < other.character); }
      470 +        isAfter(other) { return !this.isBefore(other) && !this.isEqual(other); }
      471 +        isEqual(other) { return this.line === other.line && this.character === other.character; }
      472 +        translate(lineDelta, charDelta) { return new vscodeImpl.Position(this.line + (lineDelta||0), this.character + (charDelta||0)); }
      473 +        with(line, character) { return new vscodeImpl.Position(line ?? this.line, character ?? this.character); }
      474 +    },
      475 +
      476 +    Location: class {
      477 +        constructor(uri, rangeOrPosition) { this.uri = uri; this.range = rangeOrPosition; }
      478 +    },
      479 +
      480 +    Diagnostic: class {
      481 +        constructor(range, message, severity) {
      482 +            this.range = range; this.message = message; this.severity = severity ?? 0;
      483 +            this.source = undefined; this.code = undefined; this.relatedInformation = [];
      484 +        }
      485 +    },
      486 +
      487 +    DiagnosticSeverity: { Error: 0, Warning: 1, Information: 2, Hint: 3 },
      488 +
      489 +    CompletionItem: class {
      490 +        constructor(label, kind) { this.label = label; this.kind = kind; }
      491 +    },
      492 +
      493 +    CompletionItemKind: {
      494 +        Text: 0, Method: 1, Function: 2, Constructor: 3, Field: 4,
      495 +        Variable: 5, Class: 6, Interface: 7, Module: 8, Property: 9,
      496 +        Unit: 10, Value: 11, Enum: 12, Keyword: 13, Snippet: 14,
      497 +        Color: 15, File: 16, Reference: 17, Folder: 18, EnumMember: 19,
      498 +        Constant: 20, Struct: 21, Event: 22, Operator: 23, TypeParameter: 24
      499 +    },
      500 +
      501 +    CompletionList: class {
      502 +        constructor(items, isIncomplete) { this.items = items ?? []; this.isIncomplete = isIncomplete ?? false; }
      503 +    },
      504 +
      505 +    Hover: class {
      506 +        constructor(contents, range) { this.contents = Array.isArray(contents) ? contents : [contents]; this.range = range; }
      507 +    },
      508 +
      509 +    MarkdownString: class {
      510 +        constructor(value) { this.value = value ?? ''; this.isTrusted = false; }
      511 +        appendText(val) { this.value += val; return this; }
      512 +        appendMarkdown(val) { this.value += val; return this; }
      513 +        appendCodeblock(val, lang) { this.value += `\`\`\`${lang||''}\n${val}\n\`\`\``; return this; }
      514 +    },
      515 +
      516 +    TextEdit: {
      517 +        replace: (range, newText) => ({ range, newText }),
      518 +        insert: (position, newText) => ({ range: { start: position, end: position }, newText }),
      519 +        delete: (range) => ({ range, newText: '' }),
      520 +        setEndOfLine: (eol) => ({}),
      521 +    },
      522 +
      523 +    WorkspaceEdit: class {
      524 +        constructor() { this._edits = []; }
      525 +        replace(uri, range, newText) { this._edits.push({ uri: uri.toString(), range, newText }); }
      526 +        insert(uri, position, newText) { this._edits.push({ uri: uri.toString(), range: { start: position, end: position }, newText }); }
      527 +        delete(uri, range) { this._edits.push({ uri: uri.toString(), range, newText: '' }); }
      528 +        has(uri) { return this._edits.some(e => e.uri === uri.toString()); }
      529 +        set(uri, edits) { for (const e of edits) this._edits.push({ uri: uri.toString(), ...e }); }
      530 +        get(uri) { return this._edits.filter(e => e.uri === uri.toString()); }
      531 +        get size() { return this._edits.length; }
      532 +    },
      533 +
      534 +    EventEmitter: class {
      535 +        constructor() {
      536 +            this._listeners = [];
      537 +            this.event = (listener) => {
      538 +                this._listeners.push(listener);
      539 +                return { dispose: () => { const i = this._listeners.indexOf(listener); if (i >= 0) this._listeners.splice(i, 1); } };
      540 +            };
      541 +        }
      542 +        fire(data) { for (const l of this._listeners) try { l(data); } catch(e) {} }
      543 +        dispose() { this._listeners = []; }
      544 +    },
      545 +
      546 +    StatusBarAlignment: { Left: 1, Right: 2 },
      547 +    ViewColumn: { Active: -1, Beside: -2, One: 1, Two: 2, Three: 3 },
      548 +    EndOfLine: { LF: 1, CRLF: 2 },
      549 +    FileType: { Unknown: 0, File: 1, Directory: 2, SymbolicLink: 64 },
      550 +    ConfigurationTarget: { Global: 1, Workspace: 2, WorkspaceFolder: 3 },
      551 +    TreeItemCollapsibleState: { None: 0, Collapsed: 1, Expanded: 2 },
      552 +    TreeItem: class { constructor(label, collapsibleState) { this.label = label; this.collapsibleState = collapsibleState ?? 0; } },
      553 +    ThemeIcon: class { constructor(id, color) { this.id = id; this.color = color; } },
      554 +    ThemeColor: class { constructor(id) { this.id = id; } },
      555 +    SymbolKind: { File: 0, Module: 1, Namespace: 2, Package: 3, Class: 4, Method: 5, Property: 6, Field: 7, Constructor: 8, Enum: 9, Interface: 10, Function: 11, Variable: 12, Constant: 13 },
      556 +    DocumentSymbol: class { constructor(name, detail, kind, range, selectionRange) { this.name = name; this.detail = detail; this.kind = kind; this.range = range; this.selectionRange = selectionRange; this.c
          +hildren = []; } },
      557 +    SnippetString: class { constructor(value) { this.value = value ?? ''; } },
      558 +    CancellationTokenSource: class { constructor() { this.token = { isCancellationRequested: false, onCancellationRequested: () => ({dispose:()=>{}}) }; } cancel() { this.token.isCancellationRequested = true
          +; } dispose() {} },
      559 +    version: '1.96.0',
      560  };
      561
      562  const vscode = createStubProxy(vscodeImpl);
      153 -
      154 -// Global for extensions to access
      563  global.vscode = vscode;
      564 +global.acquireVsCodeApi = () => vscode; // Webview compat
      565
      566  const commands = new Map();
      567  const extensions = new Map();
      568 +const loadedExtensions = new Map();
      569
      160 -function sendResponse(obj) {
      161 -    process.stdout.write(JSON.stringify(obj) + '\n');
      162 -}
      163 -
      570 +// ── Message handlers ──────────────────────────────────────────────────────────
      571  rl.on('line', (line) => {
      572      try {
      573          const request = JSON.parse(line);
     ...
      577      }
      578  });
      579
      173 -function handleRequest(req) {
      580 +async function handleRequest(req) {
      581 +    // Handle response correlation first
      582 +    if (req._reqId && pendingRequests.has(req._reqId)) {
      583 +        pendingRequests.get(req._reqId)(req.result ?? null);
      584 +        pendingRequests.delete(req._reqId);
      585 +        return;
      586 +    }
      587 +
      588      switch (req.type) {
      589          case 'bootstrap':
      176 -            bootstrap(req.extensions);
      590 +            await bootstrap(req.extensions);
      591              break;
      592 +
      593          case 'activateExtension':
      179 -            const extId = req.id;
      180 -            const extension = loadedExtensions.get(extId);
      181 -            if (extension) {
      182 -                try {
      183 -                    const mainPath = path.join(extension.extensionPath, extension.main);
      184 -                    const extModule = require(mainPath);
      185 -                    if (extModule.activate) {
      186 -                        const context = { subscriptions: [] }; // Mock context
      187 -                        extModule.activate(context);
      188 -                        console.error(`Extension activated: ${extId}`);
      189 -                    }
      190 -                } catch (e) {
      191 -                    console.error(`Failed to activate extension ${extId}: ${e}`);
      192 -                }
      193 -            } else {
      194 -                console.error(`Extension not found: ${extId}`);
      195 -            }
      594 +            await activateExtension(req.id);
      595              break;
      197 -        case 'documentOpened':
      198 -            const doc = { uri: req.uri, content: req.content, languageId: req.languageId };
      199 -            vscode.workspace.textDocuments.push(doc);
      596 +
      597 +        case 'documentOpened': {
      598 +            const doc = {
      599 +                uri: { fsPath: req.uri, toString: () => req.uri },
      600 +                fileName: req.uri,
      601 +                languageId: req.languageId,
      602 +                version: req.version ?? 1,
      603 +                getText: () => req.content,
      604 +                lineCount: (req.content || '').split('\n').length,
      605 +                lineAt: (line) => ({ text: (req.content || '').split('\n')[line] ?? '', lineNumber: line }),
      606 +                content: req.content
      607 +            };
      608 +            vscodeImpl.workspace.textDocuments.push(doc);
      609              eventHandlers.emit('onDidOpenTextDocument', doc);
      610 +            eventHandlers.emit('onDidChangeActiveTextEditor', { document: doc });
      611              break;
      202 -        case 'documentChanged':
      203 -            const existingDoc = vscode.workspace.textDocuments.find(d => d.uri === req.uri);
      612 +        }
      613 +
      614 +        case 'documentChanged': {
      615 +            const existingDoc = vscodeImpl.workspace.textDocuments.find(d => d.uri?.toString() === req.uri || d.uri === req.uri);
      616              if (existingDoc) {
      617                  existingDoc.content = req.content;
      206 -                eventHandlers.emit('onDidChangeTextDocument', { document: existingDoc });
      618 +                existingDoc.getText = () => req.content;
      619 +                existingDoc.version = req.version ?? (existingDoc.version + 1);
      620 +                existingDoc.lineCount = (req.content || '').split('\n').length;
      621 +                eventHandlers.emit('onDidChangeTextDocument', { document: existingDoc, contentChanges: [{ text: req.content }] });
      622              }
      623              break;
      624 +        }
      625 +
      626 +        case 'documentSaved': {
      627 +            const savedDoc = vscodeImpl.workspace.textDocuments.find(d => d.uri?.toString() === req.uri || d.uri === req.uri);
      628 +            if (savedDoc) eventHandlers.emit('onDidSaveTextDocument', savedDoc);
      629 +            break;
      630 +        }
      631 +
      632 +        case 'documentClosed': {
      633 +            const idx = vscodeImpl.workspace.textDocuments.findIndex(d => d.uri?.toString() === req.uri || d.uri === req.uri);
      634 +            if (idx >= 0) {
      635 +                const [closed] = vscodeImpl.workspace.textDocuments.splice(idx, 1);
      636 +                eventHandlers.emit('onDidCloseTextDocument', closed);
      637 +            }
      638 +            break;
      639 +        }
      640 +
      641 +        // ── Provider request relay ─────────────────────────────────────────────
      642 +        case 'provideCompletions': {
      643 +            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
      644 +            const pos = new vscodeImpl.Position(req.line, req.character);
      645 +            const allItems = [];
      646 +            for (const [id, { selector, provider }] of completionProviders) {
      647 +                if (!selectorMatches(selector, doc)) continue;
      648 +                try {
      649 +                    const result = await provider.provideCompletionItems(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({dispose:()=>{}}) }, { triggerKind: 0, triggerCharacter: r
          +eq.triggerChar });
      650 +                    const items = result?.items ?? (Array.isArray(result) ? result : []);
      651 +                    allItems.push(...items);
      652 +                } catch(e) {}
      653 +            }
      654 +            sendResponse({ type: 'providerResult', kind: 'completion', reqId: req.reqId, items: allItems });
      655 +            break;
      656 +        }
      657 +
      658 +        case 'provideHover': {
      659 +            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
      660 +            const pos = new vscodeImpl.Position(req.line, req.character);
      661 +            for (const [id, { selector, provider }] of hoverProviders) {
      662 +                if (!selectorMatches(selector, doc)) continue;
      663 +                try {
      664 +                    const result = await provider.provideHover(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({dispose:()=>{}}) });
      665 +                    if (result) {
      666 +                        sendResponse({ type: 'providerResult', kind: 'hover', reqId: req.reqId, result });
      667 +                        return;
      668 +                    }
      669 +                } catch(e) {}
      670 +            }
      671 +            sendResponse({ type: 'providerResult', kind: 'hover', reqId: req.reqId, result: null });
      672 +            break;
      673 +        }
      674 +
      675 +        case 'provideDefinition': {
      676 +            const doc = { uri: req.uri, languageId: req.languageId, getText: () => req.text ?? '' };
      677 +            const pos = new vscodeImpl.Position(req.line, req.character);
      678 +            for (const [id, { selector, provider }] of definitionProviders) {
      679 +                if (!selectorMatches(selector, doc)) continue;
      680 +                try {
      681 +                    const result = await provider.provideDefinition(doc, pos, { isCancellationRequested: false, onCancellationRequested: () => ({dispose:()=>{}}) });
      682 +                    if (result) {
      683 +                        sendResponse({ type: 'providerResult', kind: 'definition', reqId: req.reqId, result });
      684 +                        return;
      685 +                    }
      686 +                } catch(e) {}
      687 +            }
      688 +            sendResponse({ type: 'providerResult', kind: 'definition', reqId: req.reqId, result: null });
      689 +            break;
      690 +        }
      691 +
      692 +        case 'settingsUpdated':
      693 +            currentSettings = { ...currentSettings, ...req.settings };
      694 +            eventHandlers.emit('onDidChangeConfiguration', { affectsConfiguration: () => true });
      695 +            break;
      696 +
      697          case 'ping':
      698              sendResponse({ type: 'pong' });
      699              break;
      212 -        case 'executeCommand':
      700 +
      701 +        case 'executeCommand': {
      702              const cmd = commands.get(req.id);
      703              if (cmd) {
      215 -                try {
      216 -                    cmd(...(req.args || []));
      217 -                } catch (e) {
      218 -                    sendResponse({ type: 'error', message: `Command ${req.id} failed: ${e.message}` });
      219 -                }
      704 +                try { await cmd(...(req.args || [])); }
      705 +                catch (e) { sendResponse({ type: 'error', message: `Command ${req.id} failed: ${e.message}` }); }
      706              } else {
      707                  sendResponse({ type: 'error', message: `Command ${req.id} not found` });
      708              }
      709              break;
      224 -        case 'load_extension':
      710 +        }
      711 +
      712 +        case 'load_extension': {
      713              const meta = req.metadata;
      226 -            if (meta && meta.id) {
      714 +            if (meta?.id) {
      715                  loadedExtensions.set(meta.id, meta);
      228 -                console.error(`Dynamic extension loaded: ${meta.id}`);
      229 -                // Check for eager activation
      230 -                if (meta.activationEvents && meta.activationEvents.includes('*')) {
      231 -                    activateExtension(meta.id);
      232 -                }
      716 +                if (meta.activationEvents?.includes('*')) await activateExtension(meta.id);
      717              }
      718              break;
      719 +        }
      720 +
      721 +        case 'webviewMessage': {
      722 +            eventHandlers.emit(`webview:${req.viewType}:message`, req.message);
      723 +            break;
      724 +        }
      725 +
      726          default:
      727              sendResponse({ type: 'error', message: `Unknown request type: ${req.type}` });
      728      }
      729  }
      730
      240 -const loadedExtensions = new Map();
      241 -
      731  async function bootstrap(extensionMetadataList) {
      243 -    for (const meta of extensionMetadataList) {
      732 +    for (const meta of (extensionMetadataList ?? [])) {
      733          loadedExtensions.set(meta.id, meta);
      245 -
      246 -        // Check for eager activation (e.g. *)
      247 -        if (meta.activationEvents && meta.activationEvents.includes('*')) {
      248 -            await activateExtension(meta.id);
      249 -        }
      734 +        if (meta.activationEvents?.includes('*')) await activateExtension(meta.id);
      735      }
      736      sendResponse({ type: 'ready', count: loadedExtensions.size });
      737  }
      738
      739  async function activateExtension(extId) {
      740      const meta = loadedExtensions.get(extId);
      256 -    if (!meta) return;
      741 +    if (!meta || extensions.has(extId)) return;
      742
      258 -    if (extensions.has(extId)) return; // Already activated
      259 -
      743      try {
      744          const extPath = meta.extensionPath;
      745          const mainFile = path.resolve(extPath, meta.main);
      263 -
      746          const extension = require(mainFile);
      747
      266 -        if (extension && typeof extension.activate === 'function') {
      748 +        if (typeof extension?.activate === 'function') {
      749              const context = {
      750                  subscriptions: [],
      269 -                extensionPath: extPath
      751 +                extensionPath: extPath,
      752 +                extensionUri: vscodeImpl.Uri.file(extPath),
      753 +                storagePath: path.join(extPath, '.storage'),
      754 +                storageUri: vscodeImpl.Uri.file(path.join(extPath, '.storage')),
      755 +                globalStoragePath: path.join(__dirname, 'global-storage', extId),
      756 +                globalStorageUri: vscodeImpl.Uri.file(path.join(__dirname, 'global-storage', extId)),
      757 +                logPath: path.join(__dirname, 'logs', extId),
      758 +                logUri: vscodeImpl.Uri.file(path.join(__dirname, 'logs', extId)),
      759 +                workspaceState: {
      760 +                    get: (key, def) => def,
      761 +                    update: (key, val) => Promise.resolve(),
      762 +                    keys: () => []
      763 +                },
      764 +                globalState: {
      765 +                    get: (key, def) => def,
      766 +                    update: (key, val) => Promise.resolve(),
      767 +                    keys: () => [],
      768 +                    setKeysForSync: () => {}
      769 +                },
      770 +                secrets: {
      771 +                    get: (key) => Promise.resolve(undefined),
      772 +                    store: (key, val) => Promise.resolve(),
      773 +                    delete: (key) => Promise.resolve(),
      774 +                    onDidChange: (cb) => eventHandlers.on('secretChanged', cb),
      775 +                },
      776 +                asAbsolutePath: (relPath) => path.join(extPath, relPath),
      777 +                environmentVariableCollection: createStubProxy({}, 'envVarCollection'),
      778 +                extension: { id: extId, extensionPath: extPath, isActive: true, packageJSON: meta, exports: undefined },
      779              };
      780              await extension.activate(context);
      781              extensions.set(extId, { metadata: meta, instance: extension, context });
      273 -            console.error(`Extension ${meta.id} activated`);
      782 +            sendResponse({ type: 'extensionActivated', id: extId });
      783          }
      784      } catch (e) {
      276 -        console.error(`Failed to activate extension ${meta.id}:`, e);
      785 +        console.error(`[ext-host] Failed to activate ${extId}:`, e.message);
      786 +        sendResponse({ type: 'extensionActivationFailed', id: extId, error: e.message });
      787      }
      788  }
      279 -
      280 -function sendRequest(req) {
      281 -    return new Promise((resolve, reject) => {
      282 -        // Simple one-way for now, assuming no response needed immediately or handled via events
      283 -        // In a real system we'd use IDs to correlate
      284 -        sendResponse(req);
      285 -        resolve();
      286 -    });
      287 -}
  ⎿  You've hit your limit · resets Apr 18, 12am (Asia/Singapore)
