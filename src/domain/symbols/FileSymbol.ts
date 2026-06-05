/** Outline / document symbol from LSP or static analysis. */
export interface FileSymbol {
    name: string;
    kind: string;
    line: number;
    column?: number;
    children?: FileSymbol[];
}

export function symbolKindToCodicon(kind: string): string {
    const k = kind.toLowerCase();
    if (k.includes('class') || k === 'struct') return 'codicon-symbol-class';
    if (k.includes('interface') || k === 'trait') return 'codicon-symbol-interface';
    if (k.includes('function') || k === 'method') return 'codicon-symbol-method';
    if (k.includes('enum')) return 'codicon-symbol-enum';
    if (k.includes('variable') || k === 'field') return 'codicon-symbol-field';
    if (k.includes('component')) return 'codicon-symbol-method';
    if (k === 'impl') return 'codicon-symbol-constructor';
    return 'codicon-symbol-property';
}
