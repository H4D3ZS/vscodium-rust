import { describe, it, expect } from 'vitest';
import { buildSnippetSuggestions } from '../snippets';

const monaco = {
    languages: {
        CompletionItemKind: { Snippet: 27 },
        CompletionItemInsertTextRule: { InsertAsSnippet: 4 },
    },
};
const range = { startLineNumber: 1, endLineNumber: 1, startColumn: 1, endColumn: 1 };

describe('buildSnippetSuggestions', () => {
    const map = {
        'For Loop': { prefix: 'for', body: ['for (let i = 0; i < $1; i++) {', '\t$0', '}'], description: 'C-style for' },
        'Log': { prefix: ['log', 'cl'], body: 'console.log($1)' },
        'Rust main': { prefix: 'main', body: 'fn main() {}', scope: 'rust' },
    };

    it('emits an InsertAsSnippet suggestion per prefix', () => {
        const s = buildSnippetSuggestions(map, 'typescript', monaco, range);
        const labels = s.map((x) => x.label).sort();
        expect(labels).toEqual(['cl', 'for', 'log']); // rust-scoped one excluded
        expect(s.every((x) => x.insertTextRules === 4 && x.kind === 27)).toBe(true);
        expect(s.find((x) => x.label === 'for')!.insertText).toContain('for (let i = 0');
    });

    it('applies scope filtering', () => {
        const rs = buildSnippetSuggestions(map, 'rust', monaco, range).map((x) => x.label).sort();
        expect(rs).toEqual(['cl', 'for', 'log', 'main']);
    });

    it('skips defs with no body', () => {
        expect(buildSnippetSuggestions({ x: { prefix: 'x', body: '' } as any }, 'ts', monaco, range)).toEqual([]);
    });
});
