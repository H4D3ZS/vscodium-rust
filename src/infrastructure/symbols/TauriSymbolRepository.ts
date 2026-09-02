import { invoke } from '../../tauri_bridge';
import type { ISymbolRepository } from '../../domain/symbols/ISymbolRepository';
import type { FileSymbol } from '../../domain/symbols/FileSymbol';

const LSP_KIND: Record<number, string> = {
    1: 'file', 2: 'module', 3: 'namespace', 4: 'package', 5: 'class',
    6: 'method', 7: 'property', 8: 'field', 9: 'constructor', 10: 'enum',
    11: 'interface', 12: 'function', 13: 'variable', 14: 'constant',
    15: 'string', 16: 'number', 17: 'boolean', 18: 'array', 19: 'object',
    20: 'key', 21: 'null', 22: 'enumMember', 23: 'struct', 24: 'event',
    25: 'operator', 26: 'typeParameter',
};

function mapLspNode(node: any): FileSymbol {
    const range = node.range ?? node.location?.range;
    const line = (range?.start?.line ?? 0) + 1;
    const column = (range?.start?.character ?? 0) + 1;
    const kind = typeof node.kind === 'number' ? (LSP_KIND[node.kind] ?? 'symbol') : String(node.kind ?? 'symbol');
    const children = Array.isArray(node.children)
        ? node.children.map(mapLspNode)
        : undefined;
    return { name: node.name ?? 'unknown', kind, line, column, children };
}

function mapStaticSymbol(s: { type?: string; name?: string; line?: number }): FileSymbol {
    return {
        name: s.name ?? 'unknown',
        kind: s.type ?? 'symbol',
        line: s.line ?? 1,
    };
}

export class TauriSymbolRepository implements ISymbolRepository {
    async fetchLspSymbols(uri: string): Promise<FileSymbol[]> {
        const res = await invoke<any[]>('lsp_document_symbols', { uri }).catch(() => []);
        if (!Array.isArray(res) || res.length === 0) return [];
        return res.map(mapLspNode);
    }

    async analyzeFileSymbols(path: string): Promise<FileSymbol[]> {
        const res = await invoke<{ symbols?: { type?: string; name?: string; line?: number }[] }>(
            'analyze_file_symbols',
            { path },
        ).catch(() => ({ symbols: [] }));
        const list = res?.symbols ?? [];
        return list.map(mapStaticSymbol);
    }
}
