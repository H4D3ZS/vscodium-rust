import React, { useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import {
    auditUrlSecurity,
    openBrowserTo,
    openResearchTerminal,
    scrapeUrl,
    searchWeb,
} from '../../application/research/runWebResearch';
import { runManusWebMission, type ManusMissionStep } from '../../application/research/runManusWebMission';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';

interface Symbol {
    type: string;
    name: string;
}

const panelStyle: React.CSSProperties = {
    padding: '16px', borderRadius: '14px',
    background: 'rgba(255,255,255,0.02)', border: '1px solid rgba(255,255,255,0.05)',
    backdropFilter: 'blur(20px)',
};

const ResearchCenter: React.FC = () => {
    const activeEditorPath = useStore(state => state.activeEditorPath);
    const activeRoot = useStore(state => state.activeRoot);
    const [status, setStatus] = useState<string | null>(null);
    const [missionQuery, setMissionQuery] = useState('');
    const [missionUrl, setMissionUrl] = useState('');
    const [missionSteps, setMissionSteps] = useState<ManusMissionStep[]>([]);
    const [missionReport, setMissionReport] = useState<string | null>(null);
    const [missionRunning, setMissionRunning] = useState(false);

    const [isAnalyzing, setIsAnalyzing] = useState(false);
    const [symbols, setSymbols] = useState<Symbol[]>([]);

    const [searchPattern, setSearchPattern] = useState('');
    const [searchResults, setSearchResults] = useState<string[]>([]);
    const [isSearching, setIsSearching] = useState(false);

    const [scrapeUrlInput, setScrapeUrlInput] = useState('');
    const [scrapeResult, setScrapeResult] = useState<string | null>(null);
    const [isScraping, setIsScraping] = useState(false);

    const [researchQuery, setResearchQuery] = useState('');
    const [researchResult, setResearchResult] = useState<string | null>(null);
    const [isResearching, setIsResearching] = useState(false);

    const [auditUrlInput, setAuditUrlInput] = useState('');
    const [auditResult, setAuditResult] = useState<string | null>(null);
    const [isAuditing, setIsAuditing] = useState(false);

    const onAnalyze = async () => {
        if (!activeEditorPath) return;
        setIsAnalyzing(true);
        setStatus('Extracting symbolic structure…');
        try {
            const res = await invoke('call_tool', {
                name: 'analyze_file_symbols',
                arguments: { path: activeEditorPath },
            }) as { symbols?: Symbol[]; symbols_count?: number };
            setSymbols(res.symbols || []);
            setStatus(`Found ${res.symbols_count ?? res.symbols?.length ?? 0} symbols`);
        } catch {
            setStatus('Symbol analysis failed.');
        } finally {
            setIsAnalyzing(false);
        }
    };

    const onSearch = async () => {
        if (!searchPattern) return;
        setIsSearching(true);
        setStatus('Searching project…');
        try {
            const res = await invoke('call_tool', {
                name: 'search_files',
                arguments: { pattern: searchPattern },
            }) as { files?: string[]; count?: number };
            setSearchResults(res.files || []);
            setStatus(`Found ${res.count ?? res.files?.length ?? 0} matches`);
        } catch {
            setStatus('Project search failed.');
        } finally {
            setIsSearching(false);
        }
    };

    const onScrape = async () => {
        const url = scrapeUrlInput.trim();
        if (!url) return;
        setIsScraping(true);
        setStatus(`Scraping ${url}…`);
        try {
            const text = await scrapeUrl(url);
            setScrapeResult(text.slice(0, 12000));
            setStatus('Page scraped.');
        } catch (e: unknown) {
            setStatus(`Scrape failed: ${e instanceof Error ? e.message : String(e)}`);
        } finally {
            setIsScraping(false);
        }
    };

    const onWebSearch = async () => {
        if (!researchQuery.trim()) return;
        setIsResearching(true);
        setStatus('Running web search…');
        try {
            const text = await searchWeb(researchQuery.trim());
            setResearchResult(text.slice(0, 12000));
            setStatus('Web search complete.');
        } catch (e: unknown) {
            setStatus(`Search failed: ${e instanceof Error ? e.message : String(e)}`);
        } finally {
            setIsResearching(false);
        }
    };

    const onSecurityAudit = async () => {
        const url = auditUrlInput.trim();
        if (!url) return;
        setIsAuditing(true);
        setStatus(`Security audit: ${url}…`);
        try {
            const report = await auditUrlSecurity(url);
            setAuditResult(report.slice(0, 16000));
            setStatus('Security audit complete.');
        } catch (e: unknown) {
            setStatus(`Audit failed: ${e instanceof Error ? e.message : String(e)}`);
        } finally {
            setIsAuditing(false);
        }
    };

    const onOpenBrowser = async () => {
        setStatus('Opening live browser…');
        try {
            await openBrowserTo(scrapeUrlInput.trim() || auditUrlInput.trim());
            setStatus('Browser ready — agent can navigate & screenshot.');
        } catch (e: unknown) {
            setStatus(`Browser failed: ${e instanceof Error ? e.message : String(e)}`);
        }
    };

    const onOpenTerminal = async () => {
        setStatus('Opening research terminal…');
        try {
            await openResearchTerminal();
            setStatus('Terminal ready in bottom panel.');
        } catch (e: unknown) {
            setStatus(`Terminal failed: ${e instanceof Error ? e.message : String(e)}`);
        }
    };

    const onRunMission = async () => {
        if (!missionQuery.trim()) return;
        setMissionRunning(true);
        setMissionReport(null);
        setMissionSteps([]);
        setStatus('Running web mission…');
        try {
            const result = await runManusWebMission({
                query: missionQuery.trim(),
                targetUrl: missionUrl.trim() || undefined,
                workspaceRoot: activeRoot || undefined,
                runCodebaseAudit: !!activeRoot,
                onStep: (step) => setMissionSteps(prev => {
                    const idx = prev.findIndex(s => s.id === step.id);
                    if (idx < 0) return [...prev, step];
                    return prev.map(s => s.id === step.id ? step : s);
                }),
            });
            setMissionReport(result.report);
            setStatus('Web mission complete.');
            if (activeRoot) {
                const path = `${activeRoot.replace(/\//g, '\\')}\\findings.md`;
                try {
                    const existing = await fileRepository.read(path).catch(() => '');
                    await fileRepository.write(path, `${existing}\n\n${result.report}`);
                } catch { /* */ }
            }
        } catch (e: unknown) {
            setStatus(`Mission failed: ${e instanceof Error ? e.message : String(e)}`);
        } finally {
            setMissionRunning(false);
        }
    };

    return (
        <div className="research-center" style={{ flex: 1, padding: '16px', display: 'flex', flexDirection: 'column', gap: '16px', overflowY: 'auto' }}>
            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="rocket" color="#a78bfa" title="Web Mission (autonomous)" />
                <input
                    value={missionQuery}
                    onChange={e => setMissionQuery(e.target.value)}
                    placeholder="Research query — e.g. CVE-2024-1234 exploit mitigations"
                    style={inputStyle}
                />
                <input
                    value={missionUrl}
                    onChange={e => setMissionUrl(e.target.value)}
                    placeholder="Optional target URL for scrape + security audit"
                    style={{ ...inputStyle, marginTop: 8 }}
                />
                <ActionButton onClick={onRunMission} disabled={missionRunning} color="#a78bfa" full>
                    {missionRunning ? 'MISSION RUNNING…' : 'RUN WEB MISSION'}
                </ActionButton>
                {missionSteps.length > 0 && (
                    <div style={{ marginTop: 10, display: 'flex', flexDirection: 'column', gap: 4 }}>
                        {missionSteps.map(s => (
                            <div key={s.id} style={{ fontSize: 10, display: 'flex', gap: 8, opacity: 0.85 }}>
                                <span style={{ color: s.status === 'done' ? '#4ade80' : s.status === 'error' ? '#f87171' : '#60a5fa' }}>{s.status}</span>
                                <span>{s.label}</span>
                                {s.detail && <span style={{ opacity: 0.5 }}>{s.detail}</span>}
                            </div>
                        ))}
                    </div>
                )}
                {missionReport && <ResultBox text={missionReport} />}
            </div>
            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="globe" color="#4ade80" title="Web Scrape" />
                <input
                    type="url"
                    value={scrapeUrlInput}
                    onChange={e => setScrapeUrlInput(e.target.value)}
                    placeholder="https://example.com/docs"
                    style={inputStyle}
                    onKeyDown={e => e.key === 'Enter' && onScrape()}
                />
                <ActionRow>
                    <ActionButton onClick={onScrape} disabled={isScraping} color="#4ade80">
                        {isScraping ? 'SCRAPING…' : 'SCRAPE PAGE'}
                    </ActionButton>
                    <ActionButton onClick={onOpenBrowser} color="#60a5fa">OPEN BROWSER</ActionButton>
                    <ActionButton onClick={onOpenTerminal} color="#fbbf24">OPEN TERMINAL</ActionButton>
                </ActionRow>
                {scrapeResult && <ResultBox text={scrapeResult} />}
            </div>

            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="shield" color="#f87171" title="Security Audit (live page)" />
                <input
                    type="url"
                    value={auditUrlInput}
                    onChange={e => setAuditUrlInput(e.target.value)}
                    placeholder="https://target.app — XSS, auth, injection scan"
                    style={inputStyle}
                    onKeyDown={e => e.key === 'Enter' && onSecurityAudit()}
                />
                <ActionButton onClick={onSecurityAudit} disabled={isAuditing} color="#f87171" full>
                    {isAuditing ? 'AUDITING…' : 'RUN SECURITY AUDIT'}
                </ActionButton>
                {auditResult && <ResultBox text={auditResult} />}
            </div>

            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="search" color="#a78bfa" title="Web Search" />
                <textarea
                    value={researchQuery}
                    onChange={e => setResearchQuery(e.target.value)}
                    placeholder="Search the web for docs, CVEs, patterns…"
                    style={{ ...inputStyle, minHeight: 56, resize: 'none', marginBottom: 8 }}
                />
                <ActionButton onClick={onWebSearch} disabled={isResearching} color="#a78bfa" full>
                    {isResearching ? 'SEARCHING…' : 'SEARCH WEB'}
                </ActionButton>
                {researchResult && <ResultBox text={researchResult} />}
            </div>

            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="symbol-class" color="#3b82f6" title="Code Symbols" />
                <ActionButton onClick={onAnalyze} disabled={isAnalyzing || !activeEditorPath} color="#60a5fa" full>
                    {isAnalyzing ? 'ANALYZING…' : 'ANALYZE ACTIVE FILE'}
                </ActionButton>
                {symbols.length > 0 && (
                    <div style={{ marginTop: 8, maxHeight: 100, overflowY: 'auto' }}>
                        {symbols.map((s, idx) => (
                            <div key={idx} style={{ fontSize: 10, marginBottom: 4, opacity: 0.8, fontFamily: 'var(--font-mono)' }}>
                                {s.type} {s.name}
                            </div>
                        ))}
                    </div>
                )}
            </div>

            <div className="glass-panel" style={panelStyle}>
                <SectionHeader icon="folder" color="#fbbf24" title="Project Search" />
                <div style={{ display: 'flex', gap: 8 }}>
                    <input
                        type="text"
                        value={searchPattern}
                        onChange={e => setSearchPattern(e.target.value)}
                        placeholder="*.rs, auth, config…"
                        style={{ ...inputStyle, flex: 1 }}
                        onKeyDown={e => e.key === 'Enter' && onSearch()}
                    />
                    <button onClick={onSearch} disabled={isSearching} style={iconBtn}>
                        {isSearching ? '…' : <i className="codicon codicon-arrow-right" />}
                    </button>
                </div>
                {searchResults.length > 0 && (
                    <div style={{ marginTop: 8, maxHeight: 80, overflowY: 'auto' }}>
                        {searchResults.map((f, idx) => (
                            <div key={idx} style={{ fontSize: 10, opacity: 0.6 }}>{f}</div>
                        ))}
                    </div>
                )}
            </div>

            {status && (
                <div style={{ fontSize: 10, opacity: 0.6, fontStyle: 'italic', display: 'flex', alignItems: 'center', gap: 8 }}>
                    <i className="codicon codicon-info" style={{ fontSize: 12 }} />
                    <span>{status}</span>
                </div>
            )}
        </div>
    );
};

const SectionHeader: React.FC<{ icon: string; color: string; title: string }> = ({ icon, color, title }) => (
    <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 10 }}>
        <i className={`codicon codicon-${icon}`} style={{ color, fontSize: 16 }} />
        <span style={{ fontSize: 11, fontWeight: 700, letterSpacing: '0.5px', textTransform: 'uppercase' }}>{title}</span>
    </div>
);

const ActionRow: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap' }}>{children}</div>
);

const ActionButton: React.FC<{
    onClick: () => void;
    disabled?: boolean;
    color: string;
    full?: boolean;
    children: React.ReactNode;
}> = ({ onClick, disabled, color, full, children }) => (
    <button
        onClick={onClick}
        disabled={disabled}
        style={{
            width: full ? '100%' : undefined,
            flex: full ? undefined : 1,
            padding: '8px 10px', borderRadius: 8, fontSize: 10, fontWeight: 600,
            background: `color-mix(in srgb, ${color} 12%, transparent)`,
            border: `1px solid color-mix(in srgb, ${color} 35%, transparent)`,
            color, cursor: disabled ? 'not-allowed' : 'pointer', opacity: disabled ? 0.5 : 1,
            marginTop: full ? 0 : 8,
        }}
    >
        {children}
    </button>
);

const ResultBox: React.FC<{ text: string }> = ({ text }) => (
    <pre style={{
        marginTop: 10, padding: 10, borderRadius: 8, fontSize: 10, lineHeight: 1.45,
        background: 'rgba(0,0,0,0.25)', border: '1px solid rgba(255,255,255,0.05)',
        maxHeight: 200, overflowY: 'auto', whiteSpace: 'pre-wrap', wordBreak: 'break-word',
    }}>
        {text}
    </pre>
);

const inputStyle: React.CSSProperties = {
    width: '100%', background: 'rgba(0,0,0,0.2)', border: '1px solid rgba(255,255,255,0.1)',
    borderRadius: 6, padding: '8px 10px', color: 'var(--vscode-editor-foreground, #fff)', fontSize: 11,
};

const iconBtn: React.CSSProperties = {
    padding: '6px 12px', borderRadius: 6, background: 'rgba(255,255,255,0.05)',
    border: '1px solid rgba(255,255,255,0.1)', color: '#fff', fontSize: 11, cursor: 'pointer',
};

export default ResearchCenter;
