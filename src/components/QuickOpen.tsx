import React, { useState, useEffect, useRef, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

// Fuzzy match: returns score > 0 if query chars appear in order in str
function fuzzyScore(str: string, query: string): number {
    if (!query) return 1;
    const s = str.toLowerCase();
    const q = query.toLowerCase();
    let si = 0, qi = 0, score = 0;
    while (si < s.length && qi < q.length) {
        if (s[si] === q[qi]) {
            score += (si === 0 || s[si - 1] === '/' || s[si - 1] === '\\') ? 3 : 1;
            qi++;
        }
        si++;
    }
    return qi === q.length ? score : 0;
}

// Highlight matching chars in label
function HighlightMatch({ text, query }: { text: string; query: string }) {
    if (!query) return <span>{text}</span>;
    const q = query.toLowerCase();
    const chars: React.JSX.Element[] = [];
    let qi = 0;
    for (let i = 0; i < text.length; i++) {
        const match = qi < q.length && text[i].toLowerCase() === q[qi];
        if (match) {
            chars.push(<span key={i} style={{ color: 'var(--vscode-list-highlightForeground, #18a3ff)', fontWeight: 700 }}>{text[i]}</span>);
            qi++;
        } else {
            chars.push(<span key={i}>{text[i]}</span>);
        }
    }
    return <>{chars}</>;
}

const QuickOpen: React.FC = () => {
    const [isOpen, setIsOpen] = useState(false);
    const [query, setQuery] = useState('');
    const [allFiles, setAllFiles] = useState<string[]>([]);
    const [filtered, setFiltered] = useState<string[]>([]);
    const [selectedIndex, setSelectedIndex] = useState(0);
    const [loading, setLoading] = useState(false);
    const inputRef = useRef<HTMLInputElement>(null);
    const listRef = useRef<HTMLDivElement>(null);
    const openFile = useStore(state => state.openFile);
    const activeRoot = useStore(state => state.activeRoot);

    // Global Ctrl+P handler
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && !e.shiftKey && e.key === 'p') {
                e.preventDefault();
                e.stopImmediatePropagation();
                setIsOpen(true);
            }
            if (e.key === 'Escape' && isOpen) {
                setIsOpen(false);
            }
        };
        window.addEventListener('keydown', handler, true);
        return () => window.removeEventListener('keydown', handler, true);
    }, [isOpen]);

    // Load files when opened
    useEffect(() => {
        if (!isOpen) return;
        setQuery('');
        setSelectedIndex(0);
        setLoading(true);
        invoke<string[]>('list_project_files')
            .then(files => { setAllFiles(files); setFiltered(files.slice(0, 100)); })
            .catch(() => setAllFiles([]))
            .finally(() => setLoading(false));
        setTimeout(() => inputRef.current?.focus(), 50);
    }, [isOpen, activeRoot]);

    // Filter on query change
    useEffect(() => {
        if (!query) {
            setFiltered(allFiles.slice(0, 100));
            setSelectedIndex(0);
            return;
        }
        const scored = allFiles
            .map(f => ({ f, score: fuzzyScore(f, query) }))
            .filter(x => x.score > 0)
            .sort((a, b) => b.score - a.score)
            .slice(0, 100)
            .map(x => x.f);
        setFiltered(scored);
        setSelectedIndex(0);
    }, [query, allFiles]);

    // Scroll selected into view
    useEffect(() => {
        if (listRef.current) {
            const item = listRef.current.querySelector(`[data-idx="${selectedIndex}"]`) as HTMLElement;
            item?.scrollIntoView({ block: 'nearest' });
        }
    }, [selectedIndex]);

    const open = useCallback((path: string) => {
        setIsOpen(false);
        const fullPath = activeRoot ? `${activeRoot}/${path}`.replace(/\//g, '\\') : path;
        openFile(fullPath);
    }, [activeRoot, openFile]);

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Escape') { e.preventDefault(); setIsOpen(false); }
        else if (e.key === 'ArrowDown') { e.preventDefault(); setSelectedIndex(p => Math.min(p + 1, filtered.length - 1)); }
        else if (e.key === 'ArrowUp') { e.preventDefault(); setSelectedIndex(p => Math.max(p - 1, 0)); }
        else if (e.key === 'Enter') { e.preventDefault(); if (filtered[selectedIndex]) open(filtered[selectedIndex]); }
    };

    if (!isOpen) return null;

    return (
        <>
            <div
                style={{ position: 'fixed', inset: 0, zIndex: 9999, background: 'rgba(0,0,0,0.4)' }}
                onClick={() => setIsOpen(false)}
            />
            <div style={{
                position: 'fixed',
                top: '20%',
                left: '50%',
                transform: 'translateX(-50%)',
                width: 'min(640px, 90vw)',
                background: 'var(--vscode-quickInput-background, #252526)',
                border: '1px solid var(--vscode-widget-border, #454545)',
                borderRadius: '6px',
                boxShadow: '0 8px 32px rgba(0,0,0,0.6)',
                zIndex: 10000,
                display: 'flex',
                flexDirection: 'column',
                maxHeight: '60vh',
                overflow: 'hidden',
            }}>
                {/* Input */}
                <div style={{ padding: '8px', borderBottom: '1px solid var(--vscode-panel-border)', display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className="codicon codicon-file" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px', opacity: 0.5 }} />
                    <input
                        ref={inputRef}
                        type="text"
                        value={query}
                        onChange={e => setQuery(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder="Go to file..."
                        style={{
                            flex: 1,
                            background: 'transparent',
                            border: 'none',
                            outline: 'none',
                            color: 'var(--vscode-foreground)',
                            fontSize: '14px',
                        }}
                    />
                    {loading && <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', opacity: 0.5 }} />}
                    <kbd style={{ fontSize: '10px', padding: '2px 5px', background: 'rgba(255,255,255,0.08)', border: '1px solid rgba(255,255,255,0.15)', borderRadius: '3px', color: 'rgba(255,255,255,0.4)', pointerEvents: 'none' }}>Esc</kbd>
                </div>

                {/* Results */}
                <div ref={listRef} style={{ overflowY: 'auto', flex: 1 }}>
                    {filtered.length === 0 && !loading ? (
                        <div style={{ padding: '16px', textAlign: 'center', color: 'rgba(255,255,255,0.3)', fontSize: '12px' }}>
                            {query ? `No files match "${query}"` : 'No files in project'}
                        </div>
                    ) : filtered.map((file, idx) => {
                        const isSelected = idx === selectedIndex;
                        const parts = file.replace(/\\/g, '/').split('/');
                        const filename = parts.pop() ?? file;
                        const dir = parts.join('/');
                        return (
                            <div
                                key={file}
                                data-idx={idx}
                                onClick={() => open(file)}
                                onMouseEnter={() => setSelectedIndex(idx)}
                                style={{
                                    padding: '5px 12px',
                                    cursor: 'pointer',
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '8px',
                                    background: isSelected ? 'var(--vscode-list-activeSelectionBackground)' : 'transparent',
                                    color: isSelected ? 'var(--vscode-list-activeSelectionForeground)' : 'inherit',
                                }}
                            >
                                <i className="codicon codicon-file" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '13px', opacity: 0.5, flexShrink: 0 }} />
                                <div style={{ flex: 1, minWidth: 0 }}>
                                    <div style={{ fontSize: '13px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                        <HighlightMatch text={filename} query={query} />
                                    </div>
                                    {dir && <div style={{ fontSize: '10px', opacity: 0.45, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{dir}</div>}
                                </div>
                            </div>
                        );
                    })}
                </div>

                {/* Footer */}
                <div style={{ display: 'flex', gap: '16px', padding: '4px 12px', borderTop: '1px solid var(--vscode-panel-border)', fontSize: '10px', color: 'rgba(255,255,255,0.25)', background: 'rgba(0,0,0,0.2)' }}>
                    <span><kbd style={{ padding: '1px 4px', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '3px', marginRight: '4px' }}>↑↓</kbd>navigate</span>
                    <span><kbd style={{ padding: '1px 4px', border: '1px solid rgba(255,255,255,0.1)', borderRadius: '3px', marginRight: '4px' }}>↵</kbd>open</span>
                    {filtered.length > 0 && <span style={{ marginLeft: 'auto' }}>{filtered.length} file{filtered.length !== 1 ? 's' : ''}</span>}
                </div>
            </div>
        </>
    );
};

export default QuickOpen;
