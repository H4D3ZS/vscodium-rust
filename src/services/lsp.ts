// LSP service — typed wrappers around Tauri LSP commands.

import { invoke } from '../tauri_bridge';

export interface LspPosition { line: number; character: number; }
export interface LspRange { start: LspPosition; end: LspPosition; }
export interface LspLocation { uri: string; range: LspRange; }

export interface LspCompletionItem {
    label: string;
    kind?: number;
    detail?: string;
    documentation?: string;
    insertText?: string;
}

export interface LspDiagnostic {
    range: LspRange;
    severity?: number;
    message: string;
    source?: string;
    code?: string | number;
}

export interface LspSymbol {
    name: string;
    kind: number;
    range: LspRange;
    selectionRange: LspRange;
    children?: LspSymbol[];
}

export async function completion(uri: string, line: number, char: number): Promise<LspCompletionItem[]> {
    const res = await invoke<{ items: LspCompletionItem[] }>('lsp_completion', { uri, line, character: char }).catch(() => ({ items: [] }));
    return res.items ?? [];
}

export async function hover(uri: string, line: number, char: number): Promise<unknown> {
    return invoke<unknown>('lsp_hover', { uri, line, character: char }).catch(() => null);
}

export async function definition(uri: string, line: number, char: number): Promise<LspLocation | null> {
    return invoke<LspLocation | null>('lsp_goto_definition', { uri, line, character: char }).catch(() => null);
}

export async function references(uri: string, line: number, char: number): Promise<LspLocation[]> {
    const res = await invoke<LspLocation[] | null>('lsp_find_references', { uri, line, character: char }).catch(() => []);
    return res ?? [];
}

export async function rename(uri: string, line: number, char: number, newName: string): Promise<Record<string, unknown> | null> {
    return invoke<Record<string, unknown> | null>('lsp_rename_symbol', { uri, line, character: char, newName }).catch(() => null);
}

export async function documentSymbols(uri: string): Promise<LspSymbol[]> {
    const res = await invoke<LspSymbol[] | null>('lsp_document_symbols', { uri }).catch(() => []);
    return res ?? [];
}

export async function diagnostics(path?: string): Promise<LspDiagnostic[]> {
    const res = await invoke<Array<{ uri: string; diagnostics: LspDiagnostic[] }>>('lsp_get_diagnostics', { path }).catch(() => []);
    return res.flatMap((r) => r.diagnostics ?? []);
}

export async function format(uri: string): Promise<unknown> {
    return invoke<unknown>('lsp_format_document', { uri }).catch(() => null);
}

export async function ensureForFile(opts: {
    root: string;
    path: string;
    languageId: string;
    version: number;
    text: string;
}): Promise<{ serverId?: string; status?: string } | null> {
    return invoke('lsp_ensure_for_file', opts).catch(() => null);
}
