import React, { useState, useEffect, useRef, useCallback } from 'react';
import { useStore } from '../store';

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
    const inputRef = useRef<HTMLInputElement>(null);
    const listRef = useRef<HTMLDivElement>(null);

    // Load commands fresh each time palette opens (fixes race condition)
    useEffect(() => {
        if (isOpen) {
            const registry: Command[] = (window as any).commandRegistry || [];
            setCommands(registry);
            setSelectedIndex(0);
            setQuery('');
            setTimeout(() => inputRef.current?.focus(), 50);
        }
    }, [isOpen, setQuery]);

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

                {/* Results */}
                <div className="command-list" ref={listRef}>
                    {filtered.length === 0 ? (
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
