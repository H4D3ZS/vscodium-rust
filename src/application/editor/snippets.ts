/**
 * User snippet support — VS Code `.code-snippets` (project) files.
 *
 * Reads `<root>/.vscode/*.code-snippets` (all JSONC), merges them, and exposes
 * Monaco completion suggestions filtered by the file's language. Global
 * snippets (`<config>/snippets/*.json`) are read too when the backend exposes
 * the config dir via `read_user_snippets`.
 */
import { invoke } from '../../tauri_bridge';
import { tryParseJsonc } from '../debug/runConfigUtils';

export interface SnippetDef {
    prefix: string | string[];
    body: string | string[];
    description?: string;
    /** comma-separated language ids; empty = all languages */
    scope?: string;
}

type SnippetMap = Record<string, SnippetDef>;

let cache: SnippetMap = {};
let loadedFor: string | null = null;

function bodyToString(body: string | string[]): string {
    return Array.isArray(body) ? body.join('\n') : body;
}

/** (Re)load snippets for a workspace root. Cheap no-op if already loaded. */
export async function refreshSnippets(root: string | null | undefined, force = false): Promise<void> {
    if (!root) { cache = {}; loadedFor = null; return; }
    if (!force && loadedFor === root) return;
    loadedFor = root;
    const merged: SnippetMap = {};

    // Project snippets: .vscode/*.code-snippets
    try {
        const files = await invoke<{ name: string }[] | string[]>('list_directory', { path: `${root}/.vscode` }).catch(() => []);
        const names = (files as any[])
            .map((f) => (typeof f === 'string' ? f : f.name))
            .filter((n: string) => n && n.endsWith('.code-snippets'));
        for (const n of names) {
            const raw = await invoke<string>('read_file', { path: `${root}/.vscode/${n}` }).catch(() => '');
            const parsed = raw ? tryParseJsonc<SnippetMap>(raw) : null;
            if (parsed) Object.assign(merged, parsed);
        }
    } catch { /* no .vscode dir */ }

    // Global snippets, if the backend surfaces them.
    try {
        const globalRaw = await invoke<Record<string, SnippetMap>>('read_user_snippets').catch(() => null);
        if (globalRaw) {
            for (const m of Object.values(globalRaw)) Object.assign(merged, m);
        }
    } catch { /* command not present */ }

    cache = merged;
}

/** Pure: build Monaco suggestions from an explicit snippet map. */
export function buildSnippetSuggestions(map: SnippetMap, langId: string, monaco: any, range: any): any[] {
    const out: any[] = [];
    for (const [name, def] of Object.entries(map)) {
        if (!def || !def.body) continue;
        if (def.scope) {
            const scopes = def.scope.split(',').map((s) => s.trim()).filter(Boolean);
            if (scopes.length && !scopes.includes(langId)) continue;
        }
        const prefixes = Array.isArray(def.prefix) ? def.prefix : [def.prefix ?? name];
        for (const prefix of prefixes) {
            if (!prefix) continue;
            out.push({
                label: prefix,
                kind: monaco.languages.CompletionItemKind.Snippet,
                insertText: bodyToString(def.body),
                insertTextRules: monaco.languages.CompletionItemInsertTextRule.InsertAsSnippet,
                detail: def.description || name,
                documentation: { value: '```\n' + bodyToString(def.body) + '\n```' },
                sortText: `~snip~${prefix}`, // sort just after LSP items
                range,
            });
        }
    }
    return out;
}

/** Monaco completion suggestions for `langId` at `range`, from the live cache. */
export function snippetSuggestions(langId: string, monaco: any, range: any): any[] {
    return buildSnippetSuggestions(cache, langId, monaco, range);
}

export function hasSnippets(): boolean {
    return Object.keys(cache).length > 0;
}
