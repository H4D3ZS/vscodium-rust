import { invoke } from '../tauri_bridge';

export interface SearchResult {
    file_path: string;
    content: string;
    start_line: number;
    end_line: number;
    relevance_score: number;
    context: string;
}

export interface IndexStats {
    total_files: number;
    total_chunks: number;
    total_symbols: number;
    languages: Record<string, number>;
    last_indexed: number | null;
    index_size_bytes: number;
}

export interface CodeChunk {
    id: string;
    file_path: string;
    content: string;
    start_line: number;
    end_line: number;
    language: string;
    symbols: string[];
    timestamp: number;
}

export async function indexCodebase(): Promise<string> {
    return invoke<string>('vector_index_codebase');
}

export async function searchCodebase(query: string, limit?: number): Promise<{
    query: string;
    results: SearchResult[];
    count: number;
}> {
    return invoke<any>('vector_search_codebase', { query, limit: limit || 10 });
}

export async function findSymbol(symbolName: string): Promise<{
    symbol: string;
    results: SearchResult[];
    count: number;
}> {
    return invoke<any>('vector_find_symbol', { symbol_name: symbolName });
}

export async function getIndexStats(): Promise<IndexStats> {
    return invoke<any>('vector_get_index_stats');
}

export async function getFileChunks(filePath: string): Promise<{
    file: string;
    chunks: CodeChunk[];
    count: number;
}> {
    return invoke<any>('vector_get_file_chunks', { file_path: filePath });
}
