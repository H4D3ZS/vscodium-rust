import React, { useState } from 'react';
import { searchCodebase, findSymbol, getIndexStats, type IndexStats, type SearchResult } from '../services/vectorSearch';

const VectorSearchPanel: React.FC = () => {
    const [query, setQuery] = useState('');
    const [results, setResults] = useState<SearchResult[]>([]);
    const [loading, setLoading] = useState(false);
    const [stats, setStats] = useState<IndexStats | null>(null);
    const [searchMode, setSearchMode] = useState<'code' | 'symbol'>('code');
    const [selectedResult, setSelectedResult] = useState<SearchResult | null>(null);

    const handleSearch = async () => {
        if (!query.trim()) return;

        setLoading(true);
        try {
            if (searchMode === 'code') {
                const res = await searchCodebase(query, 20);
                setResults(res.results);
            } else {
                const res = await findSymbol(query);
                setResults(res.results);
            }
        } catch (e) {
            console.error('Search failed:', e);
        } finally {
            setLoading(false);
        }
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === 'Enter') {
            handleSearch();
        }
    };

    const handleRefreshStats = async () => {
        try {
            const s = await getIndexStats();
            setStats(s);
        } catch (e) {
            console.error('Failed to get stats:', e);
        }
    };

    return (
        <div style={{
            display: 'flex',
            flexDirection: 'column',
            height: '100%',
            background: 'var(--vscode-editor-background)',
            color: 'var(--vscode-foreground)',
        }}>
            {/* Header */}
            <div style={{
                height: '35px',
                display: 'flex',
                alignItems: 'center',
                padding: '0 12px',
                borderBottom: '1px solid var(--vscode-panel-border)',
                background: 'var(--vscode-panel-background)',
                justifyContent: 'space-between',
            }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <i className="codicon codicon-search" style={{ fontSize: '14px' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 600 }}>CODEBASE SEARCH</span>
                </div>
                <button
                    onClick={handleRefreshStats}
                    style={{
                        background: 'transparent',
                        border: 'none',
                        color: 'var(--vscode-foreground)',
                        cursor: 'pointer',
                        padding: '4px',
                        borderRadius: '2px',
                        fontSize: '10px',
                    }}
                    title="Refresh stats"
                >
                    <i className="codicon codicon-refresh" style={{ fontSize: '14px' }}></i>
                </button>
            </div>

            {/* Stats */}
            {stats && (
                <div style={{
                    padding: '8px 12px',
                    borderBottom: '1px solid var(--vscode-panel-border)',
                    fontSize: '10px',
                    opacity: 0.7,
                    display: 'flex',
                    gap: '12px',
                }}>
                    <span>{stats.total_files} files</span>
                    <span>{stats.total_chunks} chunks</span>
                    <span>{stats.total_symbols} symbols</span>
                    {stats.languages && Object.keys(stats.languages).length > 0 && (
                        <span>{Object.keys(stats.languages).join(', ')}</span>
                    )}
                </div>
            )}

            {/* Search Input */}
            <div style={{ padding: '12px' }}>
                <div style={{ display: 'flex', gap: '4px', marginBottom: '8px' }}>
                    <button
                        onClick={() => setSearchMode('code')}
                        style={{
                            background: searchMode === 'code' ? 'var(--vscode-button-background)' : 'transparent',
                            color: searchMode === 'code' ? 'var(--vscode-button-foreground)' : 'var(--vscode-foreground)',
                            border: '1px solid var(--vscode-button-background)',
                            padding: '4px 8px',
                            fontSize: '11px',
                            borderRadius: '2px',
                            cursor: 'pointer',
                            flex: 1,
                        }}
                    >
                        Code Search
                    </button>
                    <button
                        onClick={() => setSearchMode('symbol')}
                        style={{
                            background: searchMode === 'symbol' ? 'var(--vscode-button-background)' : 'transparent',
                            color: searchMode === 'symbol' ? 'var(--vscode-button-foreground)' : 'var(--vscode-foreground)',
                            border: '1px solid var(--vscode-button-background)',
                            padding: '4px 8px',
                            fontSize: '11px',
                            borderRadius: '2px',
                            cursor: 'pointer',
                            flex: 1,
                        }}
                    >
                        Symbol Search
                    </button>
                </div>
                <div style={{ display: 'flex', gap: '4px' }}>
                    <input
                        type="text"
                        value={query}
                        onChange={e => setQuery(e.target.value)}
                        onKeyDown={handleKeyDown}
                        placeholder={`Search ${searchMode === 'code' ? 'code' : 'symbols'}...`}
                        style={{
                            flex: 1,
                            padding: '6px 8px',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border)',
                            borderRadius: '2px',
                            fontSize: '12px',
                            outline: 'none',
                        }}
                    />
                    <button
                        onClick={handleSearch}
                        disabled={loading || !query.trim()}
                        style={{
                            background: loading || !query.trim()
                                ? 'var(--vscode-button-background)'
                                : 'var(--vscode-button-background)',
                            color: 'var(--vscode-button-foreground)',
                            border: 'none',
                            padding: '6px 12px',
                            fontSize: '12px',
                            borderRadius: '2px',
                            cursor: loading ? 'wait' : 'pointer',
                            opacity: loading || !query.trim() ? 0.5 : 1,
                            display: 'flex',
                            alignItems: 'center',
                            gap: '4px',
                        }}
                    >
                        {loading ? (
                            <i className="codicon codicon-loading" style={{ animation: 'spin 1s linear infinite' }}></i>
                        ) : (
                            <i className="codicon codicon-search"></i>
                        )}
                    </button>
                </div>
            </div>

            {/* Results */}
            <div style={{
                flex: 1,
                overflowY: 'auto',
                borderTop: '1px solid var(--vscode-panel-border)',
            }}>
                {results.length === 0 && !loading ? (
                    <div style={{
                        padding: '24px',
                        textAlign: 'center',
                        opacity: 0.5,
                        fontSize: '12px',
                    }}>
                        <i className="codicon codicon-search" style={{ fontSize: '32px', marginBottom: '8px', display: 'block', opacity: 0.3 }}></i>
                        Search your codebase semantically.<br />
                        Find code by meaning, context, and symbols.
                    </div>
                ) : (
                    results.map((result, idx) => (
                        <div
                            key={idx}
                            onClick={() => setSelectedResult(result)}
                            style={{
                                padding: '12px',
                                borderBottom: '1px solid var(--vscode-panel-border)',
                                cursor: 'pointer',
                                background: selectedResult === result
                                    ? 'var(--vscode-list-activeSelectionBackground)'
                                    : 'transparent',
                                color: selectedResult === result
                                    ? 'var(--vscode-list-activeSelectionForeground)'
                                    : 'var(--vscode-foreground)',
                            }}
                        >
                            <div style={{
                                display: 'flex',
                                justifyContent: 'space-between',
                                alignItems: 'center',
                                marginBottom: '4px',
                            }}>
                                <div style={{
                                    fontSize: '12px',
                                    fontWeight: 500,
                                    display: 'flex',
                                    alignItems: 'center',
                                    gap: '4px',
                                }}>
                                    <i className="codicon codicon-file" style={{ fontSize: '12px' }}></i>
                                    {result.file_path}
                                </div>
                                <div style={{
                                    fontSize: '10px',
                                    background: 'var(--vscode-badge-background)',
                                    color: 'var(--vscode-badge-foreground)',
                                    padding: '2px 6px',
                                    borderRadius: '10px',
                                }}>
                                    L{result.start_line}-L{result.end_line}
                                </div>
                            </div>
                            {result.context && (
                                <pre style={{
                                    fontSize: '11px',
                                    fontFamily: 'var(--vscode-editor-font-family)',
                                    background: 'var(--vscode-textCodeBlock-background)',
                                    padding: '8px',
                                    borderRadius: '4px',
                                    marginTop: '6px',
                                    overflow: 'auto',
                                    maxHeight: '100px',
                                    whiteSpace: 'pre-wrap',
                                    wordBreak: 'break-word',
                                }}>
                                    {result.context}
                                </pre>
                            )}
                            {result.relevance_score > 0 && (
                                <div style={{
                                    fontSize: '10px',
                                    opacity: 0.6,
                                    marginTop: '4px',
                                }}>
                                    Relevance: {(result.relevance_score * 10).toFixed(1)}%
                                </div>
                            )}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default VectorSearchPanel;
