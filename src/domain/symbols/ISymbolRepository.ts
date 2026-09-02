import type { FileSymbol } from './FileSymbol';

export interface ISymbolRepository {
    /** LSP document symbols for a file URI. */
    fetchLspSymbols(uri: string): Promise<FileSymbol[]>;
    /** Regex/static fallback when LSP is unavailable. */
    analyzeFileSymbols(path: string): Promise<FileSymbol[]>;
}
