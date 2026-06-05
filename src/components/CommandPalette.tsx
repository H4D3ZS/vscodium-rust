import React, { useState, useEffect, useRef, useCallback } from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';

interface Command {
    id: string;
    label: string;
    run: () => void;
}

// Derive a category from command ID prefix
function getCategory(id: string): string {
    if (id.startsWith('workbench.action.files') || id.startsWith('explorer') || id.startsWith('git')) return 'File';
    if (id.startsWith('workbench.view') || id.startsWith('workbench.action.toggle') || id.startsWith('workbench.action.show')) return 'View';
    if (id.startsWith('terminal')) return 'Terminal';
    if (id.startsWith('git')) return 'Git';
    return 'Command';
}

// Map category to a codicon name
function getCategoryIcon(id: string): string {
    if (id.startsWith('explorer') || id.startsWith('workbench.action.files')) return 'codicon-file';
    if (id.startsWith('git')) return 'codicon-source-control';
    if (id.startsWith('terminal')) return 'codicon-terminal';
    if (id.includes('toggle') || id.includes('view')) return 'codicon-layout';
    return 'codicon-run';
}

// Keybinding hints (Ctrl on Windows)
const KEYBINDING_MAP: Record<string, string> = {
    'workbench.action.toggleSidebarVisibility': 'Ctrl+B',
    'workbench.action.togglePanel': 'Ctrl+J',
    'workbench.action.toggleAuxiliaryBar': 'Ctrl+Alt+B',
    'workbench.action.closeActiveEditor': 'Ctrl+W',
    'workbench.action.files.save': 'Ctrl+S',
    'workbench.action.showCommands': 'Ctrl+Shift+P',
    'explorer.openFolder': 'Ctrl+K Ctrl+O',
    'terminal.new': 'Ctrl+`',
};

const CommandPalette: React.FC = () => {
    const isOpen = useStore(state => state.isCommandPaletteOpen);
    const setOpen = useStore(state => state.setCommandPaletteOpen);
    const query = useStore(state => state.commandPaletteQuery);
    const setQuery = useStore(state => state.setCommandPaletteQuery);

    const [commands, setCommands] = useState<Command[]>([]);
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [symbolResults, setSymbolResults] = useState<any[]>([]);
    const [isSymbolMode, setIsSymbolMode] = useState(false);
    const inputRef = useRef<HTMLInputElement>(null);
    const listRef = useRef<HTMLDivElement>(null);
    const symbolTimer = useRef<any>(null);

    // Ctrl+Shift+O → workspace symbol search (Ctrl+T reserved for new chat)
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key.toLowerCase() === 'o') {
                e.preventDefault();
                useStore.getState().setCommandPaletteOpen(true);
                setTimeout(() => {
                    setQuery('#');
                    inputRef.current?.focus();
                }, 60);
            }
        };
        window.addEventListener('keydown', handler);
        return () => window.removeEventListener('keydown', handler);
    }, []);

    // Load commands fresh each time palette opens
    useEffect(() => {
        if (isOpen) {
            const registry: Command[] = (window as any).commandRegistry || [];
            setCommands(registry);
            setSelectedIndex(0);
            if (!query.startsWith('#')) setQuery('');
            setTimeout(() => inputRef.current?.focus(), 50);
        }
    }, [isOpen]);

    // Symbol mode: when query starts with #, search workspace symbols via LSP
    useEffect(() => {
        const sym = query.startsWith('#');
        setIsSymbolMode(sym);
        if (!sym) { setSymbolResults([]); return; }
        const q = query.slice(1);
        if (symbolTimer.current) clearTimeout(symbolTimer.current);
        symbolTimer.current = setTimeout(async () => {
            try {
                const res = await invoke<any>('lsp_workspace_symbols', { query: q });
                setSymbolResults(Array.isArray(res) ? res.slice(0, 50) : []);
            } catch { setSymbolResults([]); }
        }, 200);
    }, [query]);

    const filtered = commands.filter(c =>
        query.length === 0 || c.label.toLowerCase().includes(query.toLowerCase())
    );

    // Scroll selected item into view
    useEffect(() => {
        if (listRef.current) {
            const item = listRef.current.querySelector(`[data-idx="${selectedIndex}"]`) as HTMLElement;
            item?.scrollIntoView({ block: 'nearest' });
        }
    }, [selectedIndex]);

    const execute = useCallback((cmd: Command) => {
        setOpen(false);
        setQuery('');
        setTimeout(() => cmd.run(), 50);
    }, [setOpen, setQuery]);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') {
            e.preventDefault();
            setOpen(false);
            setQuery('');
        } else if (e.key === 'ArrowDown') {
            e.preventDefault();
            setSelectedIndex(prev => Math.min(prev + 1, filtered.length - 1));
        } else if (e.key === 'ArrowUp') {
            e.preventDefault();
            setSelectedIndex(prev => Math.max(prev - 1, 0));
        } else if (e.key === 'Enter') {
            e.preventDefault();
            const cmd = filtered[selectedIndex];
            if (cmd) execute(cmd);
        }
    };

    // Reset selection when filter changes
    useEffect(() => {
        setSelectedIndex(0);
    }, [query]);

    if (!isOpen) return null;

    return (
        <>
            {/* Backdrop */}
            <div
                style={{
                    position: 'fixed',
                    inset: 0,
                    zIndex: 9999,
                    background: 'rgba(0,0,0,0.4)',
                }}
                onClick={() => { setOpen(false); setQuery(''); }}
            />

            {/* Palette */}
            <div
                id="command-palette"
                className="command-palette"
                style={{ zIndex: 10000 }}
            >
                {/* Search Input */}
                <div className="command-input-container" style={{ borderBottom: '1px solid var(--vscode-panel-border)' }}>
                    <div style={{ position: 'relative', display: 'flex', alignItems: 'center' }}>
                        <i
                            className="codicon codicon-search"
                            style={{
                                fontFamily: 'codicon',
                                fontStyle: 'normal',
                                position: 'absolute',
                                left: '10px',
                                fontSize: '14px',
                                opacity: 0.5,
                                pointerEvents: 'none',
                            }}
                        />
                        <input
                            ref={inputRef}
                            type="text"
                            placeholder="Type a command or search..."
                            value={query}
                            onChange={e => setQuery(e.target.value)}
                            onKeyDown={handleKeyDown}
                            style={{ paddingLeft: '32px', width: '100%', boxSizing: 'border-box' }}
                        />
                        <kbd style={{
                            fontFamily: 'var(--font-ui)',
                            position: 'absolute',
                            right: '10px',
                            fontSize: '10px',
                            padding: '2px 5px',
                            background: 'rgba(255,255,255,0.08)',
                            border: '1px solid rgba(255,255,255,0.15)',
                            borderRadius: '3px',
                            color: 'rgba(255,255,255,0.4)',
                            pointerEvents: 'none',
                        }}>
                            Esc
                        </kbd>
                    </div>
                </div>

                {isSymbolMode && (
                    <div style={{ padding: '4px 12px', fontSize: '10px', opacity: 0.5, borderBottom: '1px solid var(--vscode-panel-border)' }}>
                        Workspace symbols — type to filter  <kbd style={{ fontSize: '9px', opacity: 0.6, padding: '1px 4px', background: 'rgba(255,255,255,0.08)', border: '1px solid rgba(255,255,255,0.15)', borderRadius: '2px' }}>Ctrl+T</kbd>
                    </div>
                )}

                {/* Results */}
                <div className="command-list" ref={listRef}>
                    {isSymbolMode ? (
                        symbolResults.length === 0 ? (
                            <div style={{ padding: '16px', textAlign: 'center', color: 'rgba(255,255,255,0.3)', fontSize: '12px' }}>
                                {query === '#' ? 'Type to search symbols…' : `No symbols found for "${query.slice(1)}"`}
                            </div>
                        ) : symbolResults.map((sym, idx) => {
                            const isSelected = idx === selectedIndex;
                            const locUri = sym.location?.uri ?? '';
                            const filename = locUri.replace(/\\/g, '/').split('/').pop() ?? locUri;
                            const line = (sym.location?.range?.start?.line ?? 0) + 1;
                            return (
                                <div
                                    key={idx}
                                    data-idx={idx}
                                    style={{
                                        padding: '6px 16px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '10px',
                                        background: isSelected ? 'var(--vscode-list-activeSelectionBackground)' : 'transparent',
                                        color: isSelected ? 'var(--vscode-list-activeSelectionForeground)' : 'inherit',
                                    }}
                                    onMouseEnter={e => { if (!isSelected) e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'; }}
                                    onMouseLeave={e => { if (!isSelected) e.currentTarget.style.background = 'transparent'; }}
                                    onClick={() => {
                                        setOpen(false);
                                        const path = locUri.replace('file:///', '').replace('file://', '');
                                        useStore.getState().openFile(path).then(() => {
                                            setTimeout(() => window.dispatchEvent(new CustomEvent('editor:jump-to-line', { detail: { path, line, column: 1 } })), 100);
                                        });
                                    }}
                                >
                                    <i className="codicon codicon-symbol-method" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', opacity: 0.7 }} />
                                    <div style={{ flex: 1, minWidth: 0 }}>
                                        <div style={{ fontSize: '13px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{sym.name}</div>
                                        <div style={{ fontSize: '10px', opacity: 0.5, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{filename}:{line}</div>
                                    </div>
                                    {sym.containerName && <span style={{ fontSize: '10px', opacity: 0.4, flexShrink: 0 }}>{sym.containerName}</span>}
                                </div>
                            );
                        })
                    ) : filtered.length === 0 ? (
                        <div style={{
                            padding: '24px 16px',
                            textAlign: 'center',
                            color: 'rgba(255,255,255,0.3)',
                            fontSize: '12px',
                        }}>
                            <i className="codicon codicon-search-stop" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '24px', display: 'block', marginBottom: '8px', opacity: 0.4 }} />
                            No commands match "{query}"
                        </div>
                    ) : (
                        filtered.map((cmd, idx) => {
                            const isSelected = idx === selectedIndex;
                            const kb = KEYBINDING_MAP[cmd.id];
                            const icon = getCategoryIcon(cmd.id);

                            return (
                                <div
                                    key={cmd.id}
                                    data-idx={idx}
                                    className={`command-item${isSelected ? ' selected' : ''}`}
                                    onClick={() => execute(cmd)}
                                    onMouseEnter={() => setSelectedIndex(idx)}
                                    style={{
                                        display: 'flex',
                                        alignItems: 'center',
                                        gap: '10px',
                                        padding: '5px 12px',
                                        justifyContent: 'space-between',
                                    }}
                                >
                                    <div style={{ display: 'flex', alignItems: 'center', gap: '10px', minWidth: 0 }}>
                                        <i
                                            className={`codicon ${icon}`}
                                            style={{
                                                fontFamily: 'codicon',
                                                fontStyle: 'normal',
                                                fontSize: '14px',
                                                opacity: 0.5,
                                                flexShrink: 0,
                                            }}
                                        />
                                        <span style={{
                                            overflow: 'hidden',
                                            textOverflow: 'ellipsis',
                                            whiteSpace: 'nowrap',
                                            fontSize: '13px',
                                        }}>
                                            {cmd.label}
                                        </span>
                                    </div>
                                    {kb && (
                                        <kbd style={{
                                            fontFamily: 'var(--font-ui)',
                                            fontSize: '10px',
                                            padding: '1px 5px',
                                            background: 'rgba(255,255,255,0.07)',
                                            border: '1px solid rgba(255,255,255,0.12)',
                                            borderRadius: '3px',
                                            color: 'rgba(255,255,255,0.45)',
                                            flexShrink: 0,
                                            whiteSpace: 'nowrap',
                                        }}>
                                            {kb}
                                        </kbd>
                                    )}
                                </div>
                            );
                        })
                    )}
                </div>

                {/* Footer hint */}
                <div style={{
                    display: 'flex',
                    gap: '16px',
                    padding: '4px 12px',
                    borderTop: '1px solid var(--vscode-panel-border)',
                    fontSize: '10px',
                    color: 'rgba(255,255,255,0.25)',
                    background: 'rgba(0,0,0,0.2)',
                }}>
                    <span><kbd style={{ fontFamily: 'var(--font-ui)', padding: '1px 4px', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '3px', marginRight: '4px' }}>↑↓</kbd>navigate</span>
                    <span><kbd style={{ fontFamily: 'var(--font-ui)', padding: '1px 4px', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '3px', marginRight: '4px' }}>↵</kbd>select</span>
                    <span><kbd style={{ fontFamily: 'var(--font-ui)', padding: '1px 4px', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '3px', marginRight: '4px' }}>Esc</kbd>close</span>
                    {filtered.length > 0 && (
                        <span style={{ marginLeft: 'auto' }}>{filtered.length} result{filtered.length !== 1 ? 's' : ''}</span>
                    )}
                </div>
            </div>
        </>
    );
};

export default CommandPalette;
