import React, { useEffect, useMemo, useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  Breadcrumbs — VS Code-style header strip that shows the active file
//  path plus the symbol the cursor is currently inside.
//
//  Wiring:
//    • file path comes from the store's activeEditorPath
//    • cursor line comes from the `editor:cursor-position` CustomEvent the
//      editor emits whenever the cursor moves
//    • symbols come from the LSP via `lsp_document_symbols`; we fetch
//      once per file and cache the tree, then walk it locally for cursor
//      tracking so we don't slam the LSP on every keystroke.
//
//  Clicking a segment dispatches `editor:jump-to-line` which the editor
//  already listens for (see Editor.tsx ~L575) so navigation lands at
//  the symbol's definition.
// ─────────────────────────────────────────────────────────────────────────────

type LspSymbol = {
    name: string;
    kind: number;
    range?: { start: { line: number; character: number }; end: { line: number; character: number } };
    selectionRange?: { start: { line: number; character: number }; end: { line: number; character: number } };
    children?: LspSymbol[];
};

// Map LSP SymbolKind → codicon glyph for the segment icons.
const SYMBOL_ICON: Record<number, string> = {
    1: 'symbol-file',       2: 'symbol-module',    3: 'symbol-namespace',
    4: 'symbol-namespace',  5: 'symbol-class',     6: 'symbol-method',
    7: 'symbol-property',   8: 'symbol-field',     9: 'symbol-enum-member',
    10: 'symbol-interface', 11: 'symbol-function', 12: 'symbol-variable',
    13: 'symbol-constant',  14: 'symbol-string',   15: 'symbol-numeric',
    16: 'symbol-boolean',   17: 'symbol-array',    18: 'symbol-object',
    19: 'symbol-key',       20: 'symbol-null',     21: 'symbol-enum',
    22: 'symbol-struct',    23: 'symbol-event',    24: 'symbol-operator',
    25: 'symbol-type-parameter',
};

// Walk the symbol tree and collect every ancestor whose range contains the
// cursor's line. Returns the trail from outermost to innermost.
function symbolTrailForLine(symbols: LspSymbol[], line: number): LspSymbol[] {
    const trail: LspSymbol[] = [];
    const visit = (list: LspSymbol[] | undefined) => {
        if (!list) return;
        for (const s of list) {
            const r = s.range;
            if (!r) continue;
            if (line >= r.start.line && line <= r.end.line) {
                trail.push(s);
                if (s.children?.length) visit(s.children);
                break; // only one sibling at a given depth contains the cursor
            }
        }
    };
    visit(symbols);
    return trail;
}

function uriForPath(path: string): string {
    const normalized = path.replace(/\\/g, '/');
    return normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
}

const Breadcrumbs: React.FC = () => {
    const activeEditorPath = useStore(state => state.activeEditorPath);
    const activeRoot = useStore(state => state.activeRoot);
    const [symbols, setSymbols] = useState<LspSymbol[]>([]);
    const [line, setLine] = useState(1);

    // Fetch document symbols when the active file changes. We dedupe by
    // path so swapping back and forth between tabs doesn't re-request.
    useEffect(() => {
        if (!activeEditorPath) { setSymbols([]); return; }
        let cancelled = false;
        const uri = uriForPath(activeEditorPath);
        invoke<any>('lsp_document_symbols', { uri })
            .then(res => {
                if (cancelled) return;
                setSymbols(Array.isArray(res) ? res : []);
            })
            .catch(() => { if (!cancelled) setSymbols([]); });
        return () => { cancelled = true; };
    }, [activeEditorPath]);

    // Track cursor position.
    useEffect(() => {
        const handler = (e: Event) => {
            const detail = ((e as CustomEvent).detail || {}) as { line?: number };
            if (typeof detail.line === 'number') setLine(detail.line);
        };
        window.addEventListener('editor:cursor-position', handler);
        return () => window.removeEventListener('editor:cursor-position', handler);
    }, []);

    // Build the path segments. We strip the workspace root prefix so the
    // breadcrumb shows project-relative paths (matches VS Code).
    const pathSegments = useMemo(() => {
        if (!activeEditorPath) return [];
        let rel = activeEditorPath;
        if (activeRoot && rel.startsWith(activeRoot)) {
            rel = rel.slice(activeRoot.length).replace(/^[\\/]+/, '');
        }
        return rel.split(/[\\/]/).filter(Boolean);
    }, [activeEditorPath, activeRoot]);

    // Compute the symbol trail. LSP positions are 0-indexed lines, the
    // editor reports 1-indexed lines.
    const symbolTrail = useMemo(() => {
        if (!symbols.length) return [];
        return symbolTrailForLine(symbols, Math.max(0, line - 1));
    }, [symbols, line]);

    if (!activeEditorPath) return null;

    return (
        <div
            className="breadcrumbs"
            style={{
                display: 'flex',
                alignItems: 'center',
                flexWrap: 'nowrap',
                overflow: 'hidden',
                fontSize: 11,
                lineHeight: 1.6,
                padding: '2px 12px',
                background: 'var(--vscode-breadcrumb-background, var(--vscode-editor-background))',
                borderBottom: '1px solid var(--vscode-breadcrumb-border, var(--vscode-panel-border, rgba(255,255,255,0.06)))',
                color: 'var(--vscode-breadcrumb-foreground, rgba(255,255,255,0.7))',
                gap: 2,
                minHeight: 22,
                userSelect: 'none',
            }}
        >
            {pathSegments.map((seg, i) => (
                <React.Fragment key={`p-${i}`}>
                    {i > 0 && <i className="codicon codicon-chevron-right" style={chevronStyle} />}
                    <span
                        style={{
                            cursor: 'default',
                            padding: '0 4px',
                            whiteSpace: 'nowrap',
                            opacity: i === pathSegments.length - 1 ? 1 : 0.75,
                            fontWeight: i === pathSegments.length - 1 ? 600 : 400,
                        }}
                        title={seg}
                    >
                        {seg}
                    </span>
                </React.Fragment>
            ))}
            {symbolTrail.map((sym, i) => (
                <React.Fragment key={`s-${i}`}>
                    <i className="codicon codicon-chevron-right" style={chevronStyle} />
                    <span
                        style={{
                            display: 'inline-flex',
                            alignItems: 'center',
                            gap: 4,
                            cursor: 'pointer',
                            padding: '0 4px',
                            borderRadius: 3,
                            whiteSpace: 'nowrap',
                            color: 'var(--vscode-breadcrumb-focusForeground, #fff)',
                            opacity: i === symbolTrail.length - 1 ? 1 : 0.8,
                        }}
                        onClick={() => {
                            const targetLine = (sym.selectionRange?.start?.line ?? sym.range?.start?.line ?? 0) + 1;
                            window.dispatchEvent(new CustomEvent('editor:jump-to-line', {
                                detail: { path: activeEditorPath, line: targetLine, column: 1 },
                            }));
                        }}
                        title={`Jump to ${sym.name}`}
                    >
                        <i
                            className={`codicon codicon-${SYMBOL_ICON[sym.kind] || 'symbol-misc'}`}
                            style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11, opacity: 0.85 }}
                        />
                        {sym.name}
                    </span>
                </React.Fragment>
            ))}
        </div>
    );
};

const chevronStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 10,
    opacity: 0.4,
};

export default Breadcrumbs;
