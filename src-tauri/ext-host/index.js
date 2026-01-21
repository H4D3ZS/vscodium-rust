/**
 * VSCodium Rust Extension Host
 * This process executes third-party extensions in an isolated environment.
 */

const readline = require('readline');
const path = require('path');
const fs = require('fs');

const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
    terminal: false
});

// The global vscode API available to extensions
const vscode = {
    window: {
        showInformationMessage: (msg) => {
            sendResponse({ type: 'notification', level: 'info', message: msg });
        },
        showErrorMessage: (msg) => {
            sendResponse({ type: 'notification', level: 'error', message: msg });
        },
        get activeTextEditor() {
            if (vscode.workspace.textDocuments.length > 0) {
                return { document: vscode.workspace.textDocuments[0] };
            }
            return undefined;
        }
    },
    commands: {
        registerCommand: (id, callback) => {
            commands.set(id, callback);
            sendResponse({ type: 'commandRegistered', id });
        }
    },
    workspace: {
        textDocuments: [],
        fs: {
            readFile: async (uri) => {
                // Bridge to Rust
                return await sendRequest({ type: 'workspace.readFile', uri });
            }
        },
        onDidChangeTextDocument: (callback) => {
            eventHandlers.on('onDidChangeTextDocument', callback);
        },
        onDidOpenTextDocument: (callback) => {
            eventHandlers.on('onDidOpenTextDocument', callback);
        }
    },
    debug: {
        activeDebugSession: undefined,
        startDebugging: async (folder, nameOrConfiguration) => {
            // Bridge to Rust debug_start
            let adapter = "lldb-vscode";
            if (typeof nameOrConfiguration === 'object' && nameOrConfiguration.type) {
                adapter = nameOrConfiguration.type;
            }

            // We need a new RPC message type for this
            await sendRequest({ type: 'debug.start', adapter });
            return true;
        },
        onDidStartDebugSession: (callback) => {
            eventHandlers.on('onDidStartDebugSession', callback);
        }
    },
    version: '1.85.0'
};

// Simple event emitter
const eventHandlers = {
    handlers: new Map(),
    on(event, cb) {
        if (!this.handlers.has(event)) this.handlers.set(event, []);
        this.handlers.get(event).push(cb);
    },
    emit(event, ...args) {
        (this.handlers.get(event) || []).forEach(cb => cb(...args));
    }
};

// Global for extensions to access
global.vscode = vscode;

const commands = new Map();
const extensions = new Map();

function sendResponse(obj) {
    process.stdout.write(JSON.stringify(obj) + '\n');
}

rl.on('line', (line) => {
    try {
        const request = JSON.parse(line);
        handleRequest(request);
    } catch (e) {
        sendResponse({ type: 'error', message: 'Failed to parse request: ' + e.message });
    }
});

function handleRequest(req) {
    switch (req.type) {
        case 'bootstrap':
            bootstrap(req.extensions);
            break;
        case 'activateExtension':
            const extId = req.id;
            const extension = loadedExtensions.get(extId);
            if (extension) {
                try {
                    const mainPath = path.join(extension.path, extension.main);
                    const extModule = require(mainPath);
                    if (extModule.activate) {
                        const context = { subscriptions: [] }; // Mock context
                        extModule.activate(context);
                        console.error(`Extension activated: ${extId}`);
                    }
                } catch (e) {
                    console.error(`Failed to activate extension ${extId}: ${e}`);
                }
            } else {
                console.error(`Extension not found: ${extId}`);
            }
            break;
        case 'documentOpened':
            const doc = { uri: req.uri, content: req.content, languageId: req.languageId };
            vscode.workspace.textDocuments.push(doc);
            eventHandlers.emit('onDidOpenTextDocument', doc);
            break;
        case 'documentChanged':
            const existingDoc = vscode.workspace.textDocuments.find(d => d.uri === req.uri);
            if (existingDoc) {
                existingDoc.content = req.content;
                eventHandlers.emit('onDidChangeTextDocument', { document: existingDoc });
            }
            break;
        case 'ping':
            sendResponse({ type: 'pong' });
            break;
        case 'executeCommand':
            const cmd = commands.get(req.id);
            if (cmd) {
                try {
                    cmd(...(req.args || []));
                } catch (e) {
                    sendResponse({ type: 'error', message: `Command ${req.id} failed: ${e.message}` });
                }
            } else {
                sendResponse({ type: 'error', message: `Command ${req.id} not found` });
            }
            break;
        default:
            sendResponse({ type: 'error', message: `Unknown request type: ${req.type}` });
    }
}

const loadedExtensions = new Map();

async function bootstrap(extensionMetadataList) {
    for (const meta of extensionMetadataList) {
        loadedExtensions.set(meta.id, meta);

        // Check for eager activation (e.g. *)
        if (meta.activationEvents && meta.activationEvents.includes('*')) {
            await activateExtension(meta.id);
        }
    }
    sendResponse({ type: 'ready', count: loadedExtensions.size });
}

async function activateExtension(extId) {
    const meta = loadedExtensions.get(extId);
    if (!meta) return;

    if (extensions.has(extId)) return; // Already activated

    try {
        const extPath = meta.path;
        const mainFile = path.resolve(extPath, meta.main);

        const extension = require(mainFile);

        if (extension && typeof extension.activate === 'function') {
            const context = {
                subscriptions: [],
                extensionPath: extPath
            };
            await extension.activate(context);
            extensions.set(extId, { metadata: meta, instance: extension, context });
            console.error(`Extension ${meta.id} activated`);
        }
    } catch (e) {
        console.error(`Failed to activate extension ${meta.id}:`, e);
    }
}

function sendRequest(req) {
    return new Promise((resolve, reject) => {
        // Simple one-way for now, assuming no response needed immediately or handled via events
        // In a real system we'd use IDs to correlate
        sendResponse(req);
        resolve();
    });
}
