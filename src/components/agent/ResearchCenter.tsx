import React, { useState, useEffect } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

interface Symbol {
    type: string;
    name: string;
}

const ResearchCenter: React.FC = () => {
    const activeEditorPath = useStore(state => state.activeEditorPath);
    const [isAnalyzing, setIsAnalyzing] = useState(false);
    const [symbols, setSymbols] = useState<Symbol[]>([]);
    const [status, setStatus] = useState<string | null>(null);

    // Search State
    const [searchPattern, setSearchPattern] = useState('');
    const [searchResults, setSearchResults] = useState<string[]>([]);
    const [isSearching, setIsSearching] = useState(false);

    // Perplexity State
    const [researchQuery, setResearchQuery] = useState('');
    const [researchResult, setResearchResult] = useState<string | null>(null);
    const [isResearching, setIsResearching] = useState(false);

    const onAnalyze = async () => {
        if (!activeEditorPath) return;
        setIsAnalyzing(true);
        setStatus("Extracting symbolic structure...");
        try {
            const res = await invoke('call_tool', {
                name: 'analyze_file_symbols',
                arguments: { path: activeEditorPath }
            }) as any;
            setSymbols(res.symbols || []);
            setStatus(`Found ${res.symbols_count} symbols in ${activeEditorPath.split('/').pop()}`);
        } catch (err) {
            console.error(err);
            setStatus("Analysis failed.");
        } finally {
            setIsAnalyzing(false);
        }
    };

    const onSearch = async () => {
        if (!searchPattern) return;
        setIsSearching(true);
        setStatus("Searching project...");
        try {
            const res = await invoke('call_tool', {
                name: 'search_files',
                arguments: { pattern: searchPattern }
            }) as any;
            setSearchResults(res.files || []);
            setStatus(`Found ${res.count} matches for "${searchPattern}"`);
        } catch (err) {
            console.error(err);
            setStatus("Search failed.");
        } finally {
            setIsSearching(false);
        }
    };

    const onResearch = async () => {
        if (!researchQuery) return;
        setIsResearching(true);
        setStatus("Querying Perplexity AI...");
        try {
            const res = await invoke('call_tool', {
                name: 'perplexity_ask',
                arguments: { query: researchQuery }
            }) as any;
            setResearchResult(res.answer || res.response);
            setStatus("Research complete.");
        } catch (err) {
            console.error(err);
            setStatus("Research failed.");
        } finally {
            setIsResearching(false);
        }
    };

    return (
        <div className="research-center" style={{ flex: 1, padding: '16px', display: 'flex', flexDirection: 'column', gap: '24px', overflowY: 'auto' }}>
            {/* Symbol Analysis */}
            <div className="glass-panel" style={{ padding: '16px', borderRadius: '14px', background: 'rgba(255,255,255,0.02)', border: '1px solid rgba(255,255,255,0.05)', backdropFilter: 'blur(20px)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
                    <i className="codicon codicon-symbol-class" style={{ color: '#3b82f6', fontSize: '16px' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 700, color: 'var(--vscode-editor-foreground, #fff)', letterSpacing: '0.5px', textTransform: 'uppercase' }}>Symbolic Analysis</span>
                </div>
                <button
                    onClick={onAnalyze}
                    disabled={isAnalyzing || !activeEditorPath}
                    className="hoverable-scale"
                    style={{
                        width: '100%', padding: '10px', borderRadius: '8px',
                        background: isAnalyzing ? 'rgba(255,255,255,0.05)' : 'rgba(59, 130, 246, 0.1)',
                        border: '1px solid rgba(59, 130, 246, 0.2)',
                        color: '#60a5fa', fontSize: '11px', fontWeight: 600,
                        cursor: activeEditorPath ? 'pointer' : 'not-allowed', transition: 'all 0.2s',
                        opacity: activeEditorPath ? 1 : 0.5
                    }}>
                    {isAnalyzing ? 'ANALYZING...' : 'ANALYZE ACTIVE BUFFER'}
                </button>

                {symbols.length > 0 && (
                    <div style={{ marginTop: '12px', maxHeight: '120px', overflowY: 'auto', borderTop: '1px solid rgba(255,255,255,0.05)', paddingTop: '8px' }}>
                        {symbols.map((s, idx) => (
                            <div key={idx} style={{ display: 'flex', alignItems: 'center', gap: '6px', fontSize: '10px', marginBottom: '4px', opacity: 0.8 }}>
                                <i className={`codicon codicon-symbol-${s.type}`} style={{ fontSize: '10px', opacity: 0.6 }}></i>
                                <span style={{ fontFamily: 'var(--font-mono)' }}>{s.name}</span>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {/* Project Search */}
            <div className="glass-panel" style={{ padding: '16px', borderRadius: '14px', background: 'rgba(255,255,255,0.02)', border: '1px solid rgba(255,255,255,0.05)', backdropFilter: 'blur(20px)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
                    <i className="codicon codicon-search" style={{ color: '#fbbf24', fontSize: '16px' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 700, color: 'var(--vscode-editor-foreground, #fff)', letterSpacing: '0.5px', textTransform: 'uppercase' }}>Project Search</span>
                </div>
                <div style={{ display: 'flex', gap: '8px' }}>
                    <input
                        type="text"
                        value={searchPattern}
                        onChange={(e) => setSearchPattern(e.target.value)}
                        placeholder="Search files (e.g. *.rs)"
                        style={{
                            flex: 1, background: 'rgba(0,0,0,0.2)', border: '1px solid rgba(255,255,255,0.1)',
                            borderRadius: '6px', padding: '6px 10px', color: 'var(--vscode-editor-foreground, #fff)', fontSize: '11px'
                        }}
                        onKeyDown={(e) => e.key === 'Enter' && onSearch()}
                    />
                    <button
                        onClick={onSearch}
                        disabled={isSearching}
                        style={{
                            padding: '6px 12px', borderRadius: '6px', background: 'rgba(255,255,255,0.05)',
                            border: '1px solid rgba(255,255,255,0.1)', color: 'var(--vscode-editor-foreground, #fff)', fontSize: '11px', cursor: 'pointer'
                        }}>
                        {isSearching ? '...' : <i className="codicon codicon-arrow-right"></i>}
                    </button>
                </div>
                {searchResults.length > 0 && (
                    <div style={{ marginTop: '12px', maxHeight: '100px', overflowY: 'auto', borderTop: '1px solid rgba(255,255,255,0.05)', paddingTop: '8px' }}>
                        {searchResults.map((f, idx) => (
                            <div key={idx} style={{ fontSize: '10px', marginBottom: '4px', opacity: 0.6, cursor: 'pointer' }} className="hoverable-text">
                                {f}
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {/* AI Research */}
            <div className="glass-panel" style={{ padding: '16px', borderRadius: '14px', background: 'rgba(255,255,255,0.02)', border: '1px solid rgba(255,255,255,0.05)', backdropFilter: 'blur(20px)' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px', marginBottom: '12px' }}>
                    <i className="codicon codicon-globe" style={{ color: '#4ade80', fontSize: '16px' }}></i>
                    <span style={{ fontSize: '11px', fontWeight: 700, color: 'var(--vscode-editor-foreground, #fff)', letterSpacing: '0.5px', textTransform: 'uppercase' }}>Advanced Research</span>
                </div>
                <textarea
                    value={researchQuery}
                    onChange={(e) => setResearchQuery(e.target.value)}
                    placeholder="Ask anything (docs, news, code patterns)..."
                    style={{
                        width: '100%', minHeight: '60px', background: 'rgba(0,0,0,0.2)', border: '1px solid rgba(255,255,255,0.1)',
                        borderRadius: '8px', padding: '10px', color: 'var(--vscode-editor-foreground, #fff)', fontSize: '11px', marginBottom: '12px', resize: 'none',
                        fontFamily: 'inherit'
                    }}
                />
                <button
                    onClick={onResearch}
                    disabled={isResearching}
                    className="hoverable-scale"
                    style={{
                        width: '100%', padding: '10px', borderRadius: '8px',
                        background: isResearching ? 'rgba(255,255,255,0.05)' : 'rgba(74, 222, 128, 0.1)',
                        border: '1px solid rgba(74, 222, 128, 0.2)',
                        color: '#4ade80', fontSize: '11px', fontWeight: 600, cursor: 'pointer', transition: 'all 0.2s'
                    }}>
                    {isResearching ? 'RESEARCHING...' : 'RUN DEEP SEARCH'}
                </button>
                {researchResult && (
                    <div style={{
                        marginTop: '16px', padding: '12px', borderRadius: '8px',
                        background: 'rgba(0,0,0,0.2)', border: '1px solid rgba(255,255,255,0.05)',
                        fontSize: '11px', lineHeight: '1.5', opacity: 0.9, maxHeight: '200px', overflowY: 'auto'
                    }}>
                        {researchResult}
                    </div>
                )}
            </div>

            {status && (
                <div style={{
                    marginTop: 'auto', padding: '10px', borderRadius: '8px',
                    background: 'rgba(0,0,0,0.2)', border: '1px solid rgba(255,255,255,0.05)',
                    fontSize: '10px', opacity: 0.6, fontStyle: 'italic', display: 'flex', alignItems: 'center', gap: '8px'
                }}>
                    <i className="codicon codicon-info" style={{ fontSize: '12px' }}></i>
                    <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{status}</span>
                </div>
            )}
        </div>
    );
};

export default ResearchCenter;
