import React, { useEffect, useRef, useState } from 'react';
import { useStore } from '../store';
import { invoke } from '../tauri_bridge';

type LspSymbol = {
    name: string;
    kind: number;
    range: { start: { line: number }; end: { line: number } };
    selectionRange?: { start: { line: number } };
    children?: LspSymbol[];
};

const KIND_ICON: Record<number, string> = {
    1: 'symbol-file', 2: 'symbol-module', 3: 'symbol-namespace',
    5: 'symbol-class', 6: 'symbol-method', 7: 'symbol-property',
    8: 'symbol-field', 9: 'symbol-enum-member', 10: 'symbol-interface',
    11: 'symbol-function', 12: 'symbol-variable', 13: 'symbol-constant',
    14: 'symbol-string', 15: 'symbol-numeric', 16: 'symbol-boolean',
    17: 'symbol-array', 18: 'symbol-object', 22: 'symbol-struct',
    23: 'symbol-event', 24: 'symbol-operator', 25: 'symbol-type-parameter',
};

function SymbolRow({ sym, depth, activeLine }: { sym: LspSymbol; depth: number; activeLine: number }) {
    const [collapsed, setCollapsed] = useState(false);
    const icon = KIND_ICON[sym.kind] ?? 'circle-outline';
    const startLine = (sym.range?.start?.line ?? 0) + 1;
    const endLine = (sym.range?.end?.line ?? 0) + 1;
    const isActive = activeLine >= startLine && activeLine <= endLine;
    const hasChildren = sym.children && sym.children.length > 0;

    const jump = () => {
        const line = (sym.selectionRange?.start?.line ?? sym.range?.start?.line ?? 0) + 1;
        window.dispatchEvent(new CustomEvent('editor:jump-to-line', { detail: { line, column: 1 } }));
    };

    return (
        <>
            <div
                onClick={jump}
                style={{
                    display: 'flex', alignItems: 'center', gap: '5px',
                    padding: `2px 8px 2px ${8 + depth * 12}px`,
                    cursor: 'pointer', fontSize: '12px',
                    background: isActive ? 'rgba(0,198,255,0.12)' : 'transparent',
                    color: isActive ? '#00c6ff' : 'var(--vscode-foreground)',
                    borderLeft: isActive ? '2px solid #00c6ff' : '2px solid transparent',
                }}
                onMouseEnter={e => { if (!isActive) e.currentTarget.style.background = 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.06))'; }}
                onMouseLeave={e => { if (!isActive) e.currentTarget.style.background = 'transparent'; }}
            >
                {hasChildren ? (
                    <i
                        className={`codicon codicon-chevron-${collapsed ? 'right' : 'down'}`}
                        style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '10px', opacity: 0.5, cursor: 'pointer', flexShrink: 0 }}
                        onClick={e => { e.stopPropagation(); setCollapsed(v => !v); }}
                    />
                ) : <span style={{ width: '12px', flexShrink: 0 }} />}
                <i
                    className={`codicon codicon-${icon}`}
                    style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px', flexShrink: 0, opacity: 0.75 }}
                />
                <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap', flex: 1 }}>{sym.name}</span>
                <span style={{ fontSize: '10px', opacity: 0.35, flexShrink: 0 }}>{startLine}</span>
            </div>
            {!collapsed && hasChildren && sym.children!.map((child, i) => (
                <SymbolRow key={i} sym={child} depth={depth + 1} activeLine={activeLine} />
            ))}
        </>
    );
}

const DocumentOutline: React.FC = () => {
    const activeEditorPath = useStore(state => (state as any).activeEditorPath ?? '');
    const isOutlinePanelOpen = useStore(state => (state as any).isOutlinePanelOpen ?? false);
    const toggleOutlinePanel = useStore(state => (state as any).toggleOutlinePanel);
    const [symbols, setSymbols] = useState<LspSymbol[]>([]);
    const [activeLine, setActiveLine] = useState(1);
    const pathRef = useRef('');

    useEffect(() => {
        if (!activeEditorPath || activeEditorPath === pathRef.current) return;
        pathRef.current = activeEditorPath;
        const normalized = activeEditorPath.replace(/\\/g, '/');
        const uri = normalized.startsWith('/') ? `file://${normalized}` : `file:///${normalized}`;
        invoke<LspSymbol[]>('lsp_document_symbols', { uri })
            .then(res => setSymbols(Array.isArray(res) ? res : []))
            .catch(() => setSymbols([]));
    }, [activeEditorPath]);

    useEffect(() => {
        const h = (e: Event) => {
            const { line } = (e as CustomEvent).detail ?? {};
            if (typeof line === 'number') setActiveLine(line);
        };
        window.addEventListener('editor:cursor-position', h);
        return () => window.removeEventListener('editor:cursor-position', h as any);
    }, []);

    if (!isOutlinePanelOpen) return null;

    return (
        <div style={{
            position: 'fixed', right: '0', top: '48px', width: '240px',
            height: 'calc(100vh - 70px)', zIndex: 500,
            background: 'var(--vscode-sideBar-background, #1e1e1e)',
            borderLeft: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.1))',
            display: 'flex', flexDirection: 'column', overflow: 'hidden',
            boxShadow: '-4px 0 16px rgba(0,0,0,0.4)',
        }}>
            <div style={{
                display: 'flex', alignItems: 'center', justifyContent: 'space-between',
                padding: '8px 10px', borderBottom: '1px solid rgba(255,255,255,0.06)',
                fontSize: '11px', fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.07em', opacity: 0.7,
            }}>
                <span>Outline</span>
                <i
                    className="codicon codicon-close"
                    onClick={toggleOutlinePanel}
                    style={{ fontFamily: 'codicon', fontStyle: 'normal', cursor: 'pointer', opacity: 0.6, fontSize: '13px' }}
                />
            </div>
            <div style={{ flex: 1, overflowY: 'auto', paddingBottom: '8px' }}>
                {symbols.length === 0 ? (
                    <div style={{ padding: '16px', opacity: 0.35, fontSize: '12px' }}>No symbols found</div>
                ) : (
                    symbols.map((sym, i) => <SymbolRow key={i} sym={sym} depth={0} activeLine={activeLine} />)
                )}
            </div>
        </div>
    );
};

export default DocumentOutline;
