/**
 * HADES Extension API — v1 surface (Milestone E, deliberately small).
 *
 * Extensions run in a sidecar process and talk JSON-RPC over the existing
 * ext_host IPC. Every capability is permission-gated: calls into a namespace
 * the manifest didn't declare reject with PermissionDenied and surface a
 * prompt to the user (tool_permission flow).
 *
 * Runtime shim: require('hades') inside the sidecar resolves to an object
 * implementing this interface.
 */

export interface Disposable {
    dispose(): void;
}

export interface HadesEventEmitter<T> {
    (listener: (e: T) => void): Disposable;
}

// ── commands ──────────────────────────────────────────────────────────────
export namespace commands {
    /** Register a command id (must be declared in the manifest `contributes.commands`). */
    function register(id: string, handler: (...args: unknown[]) => unknown): Disposable;
    /** Execute any registered command (built-in or extension-contributed). */
    function execute(id: string, ...args: unknown[]): Promise<unknown>;
}

// ── window ────────────────────────────────────────────────────────────────
export interface StatusBarItem extends Disposable {
    text: string;
    tooltip?: string;
    show(): void;
    hide(): void;
}

export namespace window {
    function showMessage(message: string, kind?: 'info' | 'warn' | 'error'): Promise<void>;
    function createStatusBarItem(alignment?: 'left' | 'right', priority?: number): StatusBarItem;
}

// ── workspace ─────────────────────────────────────────────────────────────
export interface TextDocumentChange {
    /** Workspace-relative path. */
    path: string;
    /** Full new content (v1 keeps the protocol simple; deltas can come later). */
    content: string;
}

export namespace workspace {
    /** Scoped fs: paths resolve inside the workspace root only. Capability: `fs`. */
    namespace fs {
        function readFile(path: string): Promise<string>;
        function writeFile(path: string, content: string): Promise<void>;
        function readDir(path: string): Promise<string[]>;
    }
    const onDidChangeTextDocument: HadesEventEmitter<TextDocumentChange>;
    /** Workspace root absolute path (read-only). */
    const rootPath: string;
}

// ── languages ─────────────────────────────────────────────────────────────
export interface CompletionItem {
    label: string;
    insertText?: string;
    detail?: string;
    kind?: 'text' | 'method' | 'function' | 'keyword' | 'snippet';
}

export interface CompletionContext {
    path: string;
    line: number;
    character: number;
    linePrefix: string;
}

export namespace languages {
    /** Bridges to extHostProviders.ts; languageId uses Monaco ids. Capability: `languages`. */
    function registerCompletionProvider(
        languageId: string,
        provider: { provideCompletionItems(ctx: CompletionContext): Promise<CompletionItem[]> },
        triggerCharacters?: string[],
    ): Disposable;
}

// ── settings ──────────────────────────────────────────────────────────────
export namespace settings {
    /** Reads from the Milestone C registry (ui_settings.json). Capability: `settings`. */
    function get<T = unknown>(key: string, fallback?: T): Promise<T | undefined>;
    const onChange: HadesEventEmitter<{ key: string; value: unknown }>;
}

// ── manifest shape (package.json "hades" field) ───────────────────────────
export interface HadesManifest {
    name: string;
    version: string;
    main: string;
    /** Declared capabilities — anything not listed is denied at runtime. */
    capabilities: Array<'commands' | 'window' | 'fs' | 'languages' | 'settings'>;
    contributes?: {
        commands?: Array<{ id: string; title: string }>;
        keybindings?: Array<{ command: string; key: string }>;
        themes?: Array<{ label: string; path: string }>;
        views?: Array<{ id: string; title: string; location: 'sidebar' }>;
        settings?: Array<{ id: string; label: string; type: 'string' | 'boolean' | 'number'; default?: unknown }>;
    };
}
