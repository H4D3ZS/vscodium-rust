import { requestExtHostProvider } from './ExtHostBridge';

function normalizePath(p: string): string {
    return p.replace(/^file:\/\/\/?/, '').replace(/\\/g, '/').toLowerCase();
}

export async function extHostCompletions(
    uri: string,
    path: string,
    languageId: string,
    text: string,
    line: number,
    character: number,
    triggerChar?: string,
): Promise<any[]> {
    const result = await requestExtHostProvider('provideCompletions', {
        uri,
        languageId,
        text,
        line,
        character,
        triggerChar,
    });
    if (!result || typeof result !== 'object') return [];
    const payload = result as { items?: unknown[] };
    return Array.isArray(payload.items) ? payload.items : [];
}

export async function extHostHover(
    uri: string,
    languageId: string,
    text: string,
    line: number,
    character: number,
): Promise<any | null> {
    const result = await requestExtHostProvider('provideHover', {
        uri,
        languageId,
        text,
        line,
        character,
    });
    return result ?? null;
}

export async function extHostDefinition(
    uri: string,
    languageId: string,
    text: string,
    line: number,
    character: number,
): Promise<any | null> {
    return requestExtHostProvider('provideDefinition', {
        uri,
        languageId,
        text,
        line,
        character,
    });
}

export function mapExtCompletionItems(items: any[], range: any): any[] {
    return items.map((item) => ({
        label: typeof item.label === 'string' ? item.label : (item.label?.label ?? item.insertText ?? ''),
        kind: typeof item.kind === 'number' ? item.kind : 9,
        insertText: item.insertText ?? (typeof item.label === 'string' ? item.label : item.label?.label ?? ''),
        insertTextRules: item.insertTextFormat === 2 ? 4 : 0,
        detail: item.detail ?? '',
        documentation: item.documentation
            ? { value: typeof item.documentation === 'string' ? item.documentation : (item.documentation.value ?? '') }
            : undefined,
        sortText: item.sortText,
        filterText: item.filterText,
        range,
    }));
}

export function mapExtHoverToMonaco(result: any, monaco: any): { contents: { value: string }[]; range?: any } | null {
    if (!result) return null;
    const raw = result.contents ?? result;
    const parts = Array.isArray(raw) ? raw : [raw];
    const contents = parts.map((c: any) => ({
        value: typeof c === 'string' ? c : (c?.value ?? String(c)),
    })).filter((c) => c.value);
    if (!contents.length) return null;
    const range = result.range ? {
        startLineNumber: (result.range.start?.line ?? 0) + 1,
        startColumn: (result.range.start?.character ?? 0) + 1,
        endLineNumber: (result.range.end?.line ?? 0) + 1,
        endColumn: (result.range.end?.character ?? 0) + 1,
    } : undefined;
    return { contents, range };
}

export function mapExtDefinitionToMonaco(result: any, monaco: any): any[] {
    if (!result) return [];
    const locs = Array.isArray(result) ? result : [result];
    return locs.map((loc: any) => {
        const locUri = loc.uri ?? loc.targetUri ?? '';
        const uriStr = typeof locUri === 'string' ? locUri : (locUri?.fsPath ?? locUri?.path ?? '');
        const r = loc.range ?? loc.targetSelectionRange ?? {};
        const parsed = uriStr.startsWith('file:')
            ? uriStr
            : `file:///${uriStr.replace(/\\/g, '/')}`;
        return {
            uri: monaco.Uri.parse(parsed),
            range: {
                startLineNumber: (r.start?.line ?? 0) + 1,
                startColumn: (r.start?.character ?? 0) + 1,
                endLineNumber: (r.end?.line ?? 0) + 1,
                endColumn: (r.end?.character ?? 0) + 1,
            },
        };
    });
}

export function pathsMatch(a: string, b: string): boolean {
    return normalizePath(a) === normalizePath(b);
}
