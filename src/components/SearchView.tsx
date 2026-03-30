import React, { useState, useEffect } from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';

interface SearchResult {
    path: string;
    line: number;
    content: string;
}

const SearchView: React.FC = () => {
    const [query, setQuery] = useState('');
    const [results, setResults] = useState<SearchResult[]>([]);
    const [isSearching, setIsSearching] = useState(false);
    const openFile = useStore(state => state.openFile);

    useEffect(() => {
        const delayDebounce = setTimeout(() => {
            if (query.length > 2) {
                handleSearch(query);
            } else {
                setResults([]);
            }
        }, 300);

        return () => clearTimeout(delayDebounce);
    }, [query]);

    const handleSearch = async (searchQuery: string) => {
        setIsSearching(true);
        try {
            const searchResults = await invoke<SearchResult[]>('search_project', { query: searchQuery });
            setResults(searchResults);
        } catch (error) {
            console.error('Search failed:', error);
        } finally {
            setIsSearching(false);
        }
    };

    const handleResultClick = (result: SearchResult) => {
        openFile(result.path);
        // Ideally we'd jump to the line here, but since Monaco manages its own cursor, 
        // we'd need a more advanced way to pass line number to the editor.
        // For now, opening the file is the core backend functionality.
    };

    return (
        <div className="search-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            <div className="sidebar-search-container" style={{ padding: '10px' }}>
                <div style={{ position: 'relative' }}>
                    <input
                        type="text"
                        value={query}
                        onChange={(e) => setQuery(e.target.value)}
                        placeholder="Search (min 3 chars)"
                        style={{
                            width: '100%',
                            boxSizing: 'border-box',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border, var(--vscode-panel-border))',
                            padding: '4px 24px 4px 6px',
                            fontSize: '12px',
                            outline: 'none',
                            borderRadius: '2px'
                        }}
                    />
                    {isSearching && (
                        <div style={{ position: 'absolute', right: '6px', top: '50%', transform: 'translateY(-50%)' }}>
                            <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontSize: '12px', opacity: 0.6 }}></i>
                        </div>
                    )}
                </div>
            </div>
            <div className="search-results" style={{ flex: 1, overflowY: 'auto', padding: '0 10px' }}>
                {results.length > 0 ? (
                    results.map((result, index) => (
                        <div
                            key={`${result.path}-${index}`}
                            className="search-result-item"
                            onClick={() => handleResultClick(result)}
                            style={{
                                padding: '6px 0',
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                cursor: 'pointer',
                                fontSize: '12px'
                            }}
                        >
                            <div style={{ fontWeight: 'bold', color: 'var(--vscode-textLink-foreground)', marginBottom: '2px', overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {result.path.split('/').pop()} <span style={{ fontWeight: 'normal', opacity: 0.5, fontSize: '10px' }}>{result.path}</span>
                            </div>
                            <div style={{ opacity: 0.8, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                                <span style={{ color: 'var(--vscode-descriptionForeground)', marginRight: '6px' }}>{result.line}:</span>
                                {result.content}
                            </div>
                        </div>
                    ))
                ) : (
                    query.length > 2 && !isSearching && (
                        <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                            No results found.
                        </div>
                    )
                )}
            </div>
        </div>
    );
};

export default SearchView;
