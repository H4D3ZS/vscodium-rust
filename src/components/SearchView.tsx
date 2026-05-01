import React, { useState, useEffect, useRef } from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';

interface SearchResult {
    path: string;
    line: number;
    content: string;
}

const SearchView: React.FC = () => {
    const [query, setQuery] = useState('');
    const [replaceValue, setReplaceValue] = useState('');
    const [results, setResults] = useState<SearchResult[]>([]);
    const [isSearching, setIsSearching] = useState(false);
    const [isReplacing, setIsReplacing] = useState(false);
    const [replaceCount, setReplaceCount] = useState<number | null>(null);
    const [showReplace, setShowReplace] = useState(false);
    const [caseSensitive, setCaseSensitive] = useState(false);
    const openFile = useStore(state => state.openFile);
    const setActiveSidebarView = useStore(state => state.setActiveSidebarView);
    const inputRef = useRef<HTMLInputElement>(null);
    const replaceRef = useRef<HTMLInputElement>(null);

    useEffect(() => {
        const delayDebounce = setTimeout(() => {
            if (query.length > 2) handleSearch(query);
            else setResults([]);
        }, 300);
        return () => clearTimeout(delayDebounce);
    }, [query, caseSensitive]);

    const handleSearch = async (q: string) => {
        setIsSearching(true);
        setReplaceCount(null);
        try {
            const searchResults = await invoke<SearchResult[]>('search_project', { query: q });
            setResults(searchResults);
        } catch (e) {
            console.error('Search failed:', e);
        } finally {
            setIsSearching(false);
        }
    };

    const handleResultClick = (result: SearchResult) => {
        openFile(result.path).then(() => {
            setTimeout(() => {
                window.dispatchEvent(new CustomEvent('editor:jump-to-line', {
                    detail: { path: result.path, line: result.line, column: 1 }
                }));
            }, 100);
        });
    };

    const handleReplaceAll = async () => {
        if (!query || query.length < 2) return;
        setIsReplacing(true);
        try {
            const count = await invoke<number>('replace_in_files', {
                query,
                replacement: replaceValue,
                caseSensitive,
            });
            setReplaceCount(count);
            // Re-run search to show updated matches
            await handleSearch(query);
        } catch (e) {
            console.error('Replace failed:', e);
        } finally {
            setIsReplacing(false);
        }
    };

    // Ctrl+Shift+F: focus search from anywhere
    // Ctrl+Shift+H: focus search + open replace
    useEffect(() => {
        const handler = (e: KeyboardEvent) => {
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'F') {
                e.preventDefault();
                setActiveSidebarView('search-view');
                setTimeout(() => inputRef.current?.focus(), 50);
            }
            if ((e.ctrlKey || e.metaKey) && e.shiftKey && e.key === 'H') {
                e.preventDefault();
                setActiveSidebarView('search-view');
                setShowReplace(true);
                setTimeout(() => replaceRef.current?.focus(), 50);
            }
        };
        window.addEventListener('keydown', handler);
        return () => window.removeEventListener('keydown', handler);
    }, [setActiveSidebarView]);

    const inputStyle: React.CSSProperties = {
        width: '100%',
        boxSizing: 'border-box',
        background: 'var(--vscode-input-background)',
        color: 'var(--vscode-input-foreground)',
        border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
        padding: '4px 6px',
        fontSize: '12px',
        outline: 'none',
        borderRadius: '2px',
    };

    // Group results by file
    const grouped = results.reduce((acc: Record<string, SearchResult[]>, r) => {
        const key = r.path;
        if (!acc[key]) acc[key] = [];
        acc[key].push(r);
        return acc;
    }, {});

    return (
        <div className="search-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            {/* Search/Replace inputs */}
            <div style={{ padding: '8px 10px', flexShrink: 0 }}>
                {/* Toggle replace */}
                <div style={{ display: 'flex', alignItems: 'center', marginBottom: '6px', gap: '4px' }}>
                    <div
                        onClick={() => setShowReplace(v => !v)}
                        title={showReplace ? 'Hide replace' : 'Toggle replace (Ctrl+Shift+H)'}
                        style={{ cursor: 'pointer', opacity: 0.6, fontSize: '12px' }}
                    >
                        <i className={`codicon codicon-chevron-${showReplace ? 'down' : 'right'}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                    </div>
                    <div style={{ position: 'relative', flex: 1 }}>
                        <input
                            ref={inputRef}
                            type="text"
                            value={query}
                            onChange={e => setQuery(e.target.value)}
                            placeholder="Search  (Ctrl+Shift+F)"
                            style={{ ...inputStyle, paddingRight: '52px' }}
                            onKeyDown={e => { if (e.key === 'Enter' && query.length > 2) handleSearch(query); }}
                        />
                        <div style={{ position: 'absolute', right: '4px', top: '50%', transform: 'translateY(-50%)', display: 'flex', gap: '2px' }}>
                            <div
                                title="Case sensitive"
                                onClick={() => setCaseSensitive(v => !v)}
                                style={{
                                    cursor: 'pointer', padding: '1px 3px', borderRadius: '2px', fontSize: '10px', fontWeight: 700,
                                    background: caseSensitive ? 'var(--vscode-inputOption-activeBackground, rgba(0,122,204,0.4))' : 'transparent',
                                    color: caseSensitive ? 'var(--vscode-inputOption-activeForeground, #fff)' : 'rgba(255,255,255,0.5)',
                                    border: '1px solid transparent',
                                    lineHeight: '14px',
                                }}
                            >Aa</div>
                            {isSearching && (
                                <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', opacity: 0.6 }} />
                            )}
                        </div>
                    </div>
                </div>

                {showReplace && (
                    <div style={{ display: 'flex', gap: '4px', paddingLeft: '18px' }}>
                        <input
                            ref={replaceRef}
                            type="text"
                            value={replaceValue}
                            onChange={e => setReplaceValue(e.target.value)}
                            placeholder="Replace"
                            style={{ ...inputStyle, flex: 1 }}
                            onKeyDown={e => { if (e.key === 'Enter') handleReplaceAll(); }}
                        />
                        <button
                            onClick={handleReplaceAll}
                            disabled={isReplacing || !query}
                            title="Replace All (Enter)"
                            style={{
                                background: 'var(--vscode-button-background, #0e639c)',
                                color: 'white', border: 'none', borderRadius: '2px',
                                padding: '2px 8px', fontSize: '11px', cursor: 'pointer',
                                opacity: isReplacing || !query ? 0.5 : 1, whiteSpace: 'nowrap',
                            }}
                        >
                            {isReplacing ? '…' : 'All'}
                        </button>
                    </div>
                )}

                {replaceCount !== null && (
                    <div style={{ marginTop: '6px', fontSize: '11px', color: '#4ec9b0', paddingLeft: '18px' }}>
                        Replaced in {replaceCount} file{replaceCount !== 1 ? 's' : ''}
                    </div>
                )}

                {results.length > 0 && (
                    <div style={{ marginTop: '4px', fontSize: '10px', opacity: 0.5, paddingLeft: '18px' }}>
                        {results.length} result{results.length !== 1 ? 's' : ''} in {Object.keys(grouped).length} file{Object.keys(grouped).length !== 1 ? 's' : ''}
                    </div>
                )}
            </div>

            {/* Results grouped by file */}
            <div className="search-results" style={{ flex: 1, overflowY: 'auto', padding: '0 10px 10px' }}>
                {Object.entries(grouped).map(([filePath, fileResults]) => (
                    <div key={filePath} style={{ marginBottom: '8px' }}>
                        <div style={{
                            fontSize: '11px', fontWeight: 600, padding: '4px 0 2px',
                            color: 'var(--vscode-textLink-foreground)',
                            overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap',
                        }} title={filePath}>
                            {filePath.replace(/\\/g, '/').split('/').pop()}
                            <span style={{ fontWeight: 400, opacity: 0.45, fontSize: '10px', marginLeft: '6px' }}>
                                {filePath.replace(/\\/g, '/').split('/').slice(-2, -1)[0]}
                            </span>
                        </div>
                        {fileResults.map((result, i) => (
                            <div
                                key={i}
                                onClick={() => handleResultClick(result)}
                                style={{
                                    padding: '2px 0 2px 10px',
                                    cursor: 'pointer',
                                    fontSize: '12px',
                                    display: 'flex',
                                    gap: '6px',
                                    borderRadius: '2px',
                                }}
                                onMouseEnter={e => (e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)')}
                                onMouseLeave={e => (e.currentTarget.style.background = 'transparent')}
                            >
                                <span style={{ color: 'var(--vscode-descriptionForeground)', minWidth: '28px', textAlign: 'right', opacity: 0.6, flexShrink: 0 }}>{result.line}</span>
                                <span style={{ opacity: 0.85, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>{result.content}</span>
                            </div>
                        ))}
                    </div>
                ))}
                {query.length > 2 && !isSearching && results.length === 0 && (
                    <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                        No results found.
                    </div>
                )}
            </div>
        </div>
    );
};

export default SearchView;
