// LSP service — typed wrappers around Tauri LSP commands.

import { invoke } from '@tauri-apps/api/core';

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
    return invoke<LspCompletionItem[]>('lsp_completion', { uri, line, character: char }).catch(() => []);
}

export async function hover(uri: string, line: number, char: number): Promise<string | null> {
    return invoke<string | null>('lsp_hover', { uri, line, character: char }).catch(() => null);
}

export async function definition(uri: string, line: number, char: number): Promise<LspLocation | null> {
    return invoke<LspLocation | null>('lsp_definition', { uri, line, character: char }).catch(() => null);
}

export async function references(uri: string, line: number, char: number): Promise<LspLocation[]> {
    return invoke<LspLocation[]>('lsp_references', { uri, line, character: char }).catch(() => []);
}

export async function rename(uri: string, line: number, char: number, newName: string): Promise<Record<string, any> | null> {
    return invoke<Record<string, any> | null>('lsp_rename', { uri, line, character: char, newName }).catch(() => null);
}

export async function documentSymbols(uri: string): Promise<LspSymbol[]> {
    return invoke<LspSymbol[]>('lsp_document_symbols', { uri }).catch(() => []);
}

export async function diagnostics(uri: string): Promise<LspDiagnostic[]> {
    return invoke<LspDiagnostic[]>('lsp_diagnostics', { uri }).catch(() => []);
}

export async function format(uri: string): Promise<void> {
    await invoke('lsp_format', { uri }).catch(() => { });
}
