import type { FileSymbol } from '../../domain/symbols/FileSymbol';
import { TauriSymbolRepository } from '../../infrastructure/symbols/TauriSymbolRepository';

const repo = new TauriSymbolRepository();

function pathToUri(path: string): string {
    const normalized = path.replace(/\\/g, '/');
    return normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
}

/**
 * Fetch outline symbols for the active file.
 * Tries LSP first; falls back to static analyze_file_symbols when LSP returns empty.
 */
export async function fetchFileSymbols(filePath: string): Promise<FileSymbol[]> {
    if (!filePath || filePath.startsWith('vscode://')) return [];

    const uri = pathToUri(filePath);
    const lsp = await repo.fetchLspSymbols(uri);
    if (lsp.length > 0) return lsp;

    return repo.analyzeFileSymbols(filePath);
}
