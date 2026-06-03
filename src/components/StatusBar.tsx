import React, { useState, useRef, useEffect, useCallback } from 'react';
import { useStore } from '../store';
import { invoke } from '@tauri-apps/api/core';

// ──────────────────────────────────────────────────────────────────────────────
// Helpers
// ──────────────────────────────────────────────────────────────────────────────
function languageLabel(lang: string): string {
    const map: Record<string, string> = {
        typescript: 'TypeScript', typescriptreact: 'TSX', javascript: 'JavaScript',
        javascriptreact: 'JSX', rust: 'Rust', python: 'Python', go: 'Go',
        java: 'Java', cpp: 'C++', c: 'C', csharp: 'C#', html: 'HTML',
        css: 'CSS', scss: 'SCSS', json: 'JSON', yaml: 'YAML', toml: 'TOML',
        markdown: 'Markdown', shell: 'Shell', bash: 'Shell', powershell: 'PowerShell',
        sql: 'SQL', lua: 'Lua', php: 'PHP', ruby: 'Ruby', swift: 'Swift',
        kotlin: 'Kotlin', dart: 'Dart', vue: 'Vue', svelte: 'Svelte',
        xml: 'XML', dockerfile: 'Dockerfile', makefile: 'Makefile', plaintext: 'Plain Text',
    };
    return map[lang?.toLowerCase()] ?? (lang ? lang.charAt(0).toUpperCase() + lang.slice(1) : 'Plain Text');
}

// ──────────────────────────────────────────────────────────────────────────────
// Sub-components
// ──────────────────────────────────────────────────────────────────────────────
interface StatusItemProps {
    onClick?: () => void;
    title?: string;
    children: React.ReactNode;
    accent?: boolean;
    danger?: boolean;
}

const StatusItem: React.FC<StatusItemProps> = ({ onClick, title, children, accent, danger }) => (
    <div
        className="status-item hoverable"
        onClick={onClick}
        title={title}
        style={{
            cursor: onClick ? 'pointer' : 'default',
            height: '100%',
            display: 'flex',
            alignItems: 'center',
            padding: '0 8px',
            gap: '4px',
            color: danger ? '#f87171' : accent ? '#4ade80' : 'inherit',
            background: accent ? 'rgba(74, 222, 128, 0.08)' : undefined,
        }}
    >
        {children}
    </div>
);

// ──────────────────────────────────────────────────────────────────────────────
// Main StatusBar
// ──────────────────────────────────────────────────────────────────────────────
const StatusBar: React.FC = () => {
    const theme = useStore(state => state.theme);
    const setActiveSidebarView = useStore(state => state.setActiveSidebarView);
    const toggleBottomPanel = useStore(state => state.toggleBottomPanel);
    const setActivePanelTab = useStore(state => state.setActivePanelTab);
    const agentModel = useStore(state => state.agentModel);
    const setAgentModel = useStore(state => state.setAgentModel);
    const availableModels = useStore(state => state.availableModels);
    const refreshAvailableModels = useStore(state => state.refreshAvailableModels);
    const ollamaStatus = useStore(state => state.ollamaStatus);
    const toggleRightSidebar = useStore(state => state.toggleRightSidebar);
    const diagnosticsMap = useStore(state => state.diagnosticsMap);
    const gitBranch = useStore(state => state.gitBranch);
    const activeTabId = useStore(state => state.activeTabId);
    const tabs = useStore(state => state.tabs);
    const processStats = useStore(state => state.processStats);
    const memorySavings = useStore(state => state.memorySavings);
    const refreshProcessStats = useStore(state => state.refreshProcessStats);
    const refreshMemorySavings = useStore(state => state.refreshMemorySavings);
    const isIndexingCodebase = useStore(state => state.isIndexingCodebase);
    const indexingProgress = useStore(state => state.indexingProgress);
    const startIndexingCodebase = useStore(state => state.startIndexingCodebase);

    // ── Live cursor position ──────────────────────────────────────────────────
    const [cursorLine, setCursorLine] = useState(1);
    const [cursorCol, setCursorCol] = useState(1);
    const [selectionCount, setSelectionCount] = useState(0);

    useEffect(() => {
        const handler = (e: Event) => {
            const { line, column, selectionLength } = (e as CustomEvent).detail ?? {};
            if (line !== undefined) setCursorLine(line);
            if (column !== undefined) setCursorCol(column);
            setSelectionCount(selectionLength ?? 0);
        };
        window.addEventListener('editor:cursor-position', handler);
        return () => window.removeEventListener('editor:cursor-position', handler as any);
    }, []);

    // Reset on tab change
    useEffect(() => {
        setCursorLine(1);
        setCursorCol(1);
        setSelectionCount(0);
    }, [activeTabId]);

    // ── Zen mode toggle ──────────────────────────────────────────────────────
    const isZenMode = useStore(state => (state as any).isZenMode ?? false);
    const toggleZenMode = useStore(state => (state as any).toggleZenMode);
    const toggleOutlinePanel = useStore(state => (state as any).toggleOutlinePanel);

    // ── Token budget ─────────────────────────────────────────────────────────
    const agentMessages = useStore(state => (state as any).agentMessages ?? []);
    const tokenBudget = React.useMemo(() => {
        const used = agentMessages.reduce((sum: number, m: any) => {
            const txt = typeof m.content === 'string' ? m.content : JSON.stringify(m.content ?? '');
            return sum + Math.ceil(txt.length / 4);
        }, 0);
        const model = (useStore.getState() as any).agentModel ?? '';
        const max = model.toLowerCase().includes('gemini-2.5') ? 1048576
            : model.toLowerCase().includes('gemini') ? 131072
            : model.toLowerCase().includes('claude') ? 200000
            : model.toLowerCase().includes('gpt-4') ? 128000
            : 128000;
        return { used, max, pct: Math.min(100, Math.round((used / max) * 100)) };
    }, [agentMessages]);

    // ── Git blame ─────────────────────────────────────────────────────────────
    const isGitBlameVisible = useStore(state => (state as any).isGitBlameVisible ?? false);
    const toggleGitBlame = useStore(state => (state as any).toggleGitBlame);

    // ── Git branch ───────────────────────────────────────────────────────────
    const setGitBranch = useStore(state => state.setGitBranch);
    const activeRoot = useStore(state => state.activeRoot);

    useEffect(() => {
        if (!activeRoot) { setGitBranch(''); return; }
        const refresh = () =>
            invoke<string>('get_git_branch')
                .then(b => setGitBranch(b?.trim() ?? ''))
                .catch(() => setGitBranch(''));
        refresh();
        // Refresh on SCM events
        const handler = () => refresh();
        window.addEventListener('scm:changed', handler);
        return () => window.removeEventListener('scm:changed', handler);
    }, [activeRoot, setGitBranch]);

    // ── Diagnostics counts ───────────────────────────────────────────────────
    const errorCount = Object.values(diagnosticsMap).reduce((s, d) => s + d.filter(x => x.severity === 8).length, 0);
    const warnCount = Object.values(diagnosticsMap).reduce((s, d) => s + d.filter(x => x.severity === 4).length, 0);

    // ── Active tab language ───────────────────────────────────────────────────
    const activeTab = tabs.find(t => t.id === activeTabId);
    const language = activeTab?.language ?? '';

    // ── Model picker ─────────────────────────────────────────────────────────
    const [modelPickerOpen, setModelPickerOpen] = useState(false);
    const [pickerPos, setPickerPos] = useState<{ left: number; bottom: number } | null>(null);
    const modelPickerRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (!modelPickerOpen) return;
        const handler = (e: MouseEvent) => {
            if (modelPickerRef.current && !modelPickerRef.current.contains(e.target as Node))
                setModelPickerOpen(false);
        };
        document.addEventListener('mousedown', handler);
        return () => document.removeEventListener('mousedown', handler);
    }, [modelPickerOpen]);

    const openModelPicker = () => {
        if (!modelPickerOpen && modelPickerRef.current) {
            const rect = modelPickerRef.current.getBoundingClientRect();
            setPickerPos({ left: rect.left, bottom: window.innerHeight - rect.top });
        }
        refreshAvailableModels();
        setModelPickerOpen(v => !v);
    };

    // ── Go-to-line dialog ────────────────────────────────────────────────────
    const [gotoLineOpen, setGotoLineOpen] = useState(false);
    const [gotoLineValue, setGotoLineValue] = useState('');

    const openGotoLine = () => {
        setGotoLineValue(String(cursorLine));
        setGotoLineOpen(true);
    };

    const submitGotoLine = () => {
        const n = parseInt(gotoLineValue, 10);
        if (!isNaN(n)) {
            window.dispatchEvent(new CustomEvent('editor:goto-line', { detail: { line: n } }));
        }
        setGotoLineOpen(false);
    };

    // ── Memory / stats ───────────────────────────────────────────────────────
    const handleOptimize = async () => {
        try { await invoke('optimize_memory'); refreshMemorySavings(); }
        catch (e) { console.error('optimize_memory failed:', e); }
    };

    useEffect(() => {
        refreshProcessStats();
        refreshMemorySavings();
        const t = setInterval(() => {
            refreshProcessStats();
            if (processStats?.available_ram_gb != null && processStats.available_ram_gb < 1) {
                handleOptimize();
            }
        }, 20000);
        return () => clearInterval(t);
    }, [processStats?.available_ram_gb]);

    // ── Account / usage chip (SaaS) ───────────────────────────────────────────
    const openSettings = useStore(state => (state as any).openSettings);
    const [acct, setAcct] = useState<{ signedIn: boolean; tier: string; usedMonth: number; limitMonth: number; usedDay: number; limitDay: number } | null>(null);
    useEffect(() => {
        let alive = true;
        const load = async () => {
            try {
                const [a, u] = await Promise.all([
                    invoke<any>('account_get'),
                    invoke<any>('account_usage'),
                ]);
                if (!alive) return;
                setAcct({
                    signedIn: !!a.signed_in,
                    tier: u.tier || a.tier_label || 'Community',
                    usedMonth: u.used_month ?? 0, limitMonth: u.limit_month ?? 0,
                    usedDay: u.used_day ?? 0, limitDay: u.limit_day ?? 0,
                });
            } catch { /* backend not ready */ }
        };
        load();
        const t = setInterval(load, 30000);
        const h = () => load();
        window.addEventListener('account:changed', h);
        return () => { alive = false; clearInterval(t); window.removeEventListener('account:changed', h); };
    }, []);

    // ── Open Problems panel ───────────────────────────────────────────────────
    const openProblems = useCallback(() => {
        if (!useStore.getState().isBottomPanelOpen) toggleBottomPanel();
        setActivePanelTab('PROBLEMS');
    }, [toggleBottomPanel, setActivePanelTab]);

    // ── Indentation picker ────────────────────────────────────────────────────
    const [indentOpen, setIndentOpen] = useState(false);
    const [indentPos, setIndentPos] = useState<{ left: number; bottom: number } | null>(null);
    const indentRef = useRef<HTMLDivElement>(null);
    const tabSize = useStore(state => (state as any).tabSize ?? 4);
    const insertSpaces = useStore(state => (state as any).insertSpaces ?? true);

    useEffect(() => {
        if (!indentOpen) return;
        const h = (e: MouseEvent) => {
            if (indentRef.current && !indentRef.current.contains(e.target as Node)) setIndentOpen(false);
        };
        document.addEventListener('mousedown', h);
        return () => document.removeEventListener('mousedown', h);
    }, [indentOpen]);

    return (
        <footer className="status-bar" style={{
            backgroundColor: 'var(--vscode-statusBar-background, #007acc)',
            color: 'var(--vscode-statusBar-foreground, #ffffff)',
            height: '22px',
            display: 'flex',
            justifyContent: 'space-between',
            alignItems: 'center',
            fontSize: '12px',
            fontFamily: 'var(--font-ui)',
            userSelect: 'none',
            zIndex: 1000,
            flexShrink: 0,
        }}>
            {/* ── LEFT ─────────────────────────────────────────────────────── */}
            <div className="status-left" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>
                {/* Remote / workspace indicator */}
                <div style={{
                    background: 'var(--vscode-statusBarItem-remoteBackground, #16825d)',
                    height: '100%',
                    display: 'flex',
                    alignItems: 'center',
                    padding: '0 8px',
                    marginRight: '4px',
                    cursor: 'pointer',
                    gap: '4px',
                }}>
                    <i className="codicon codicon-remote" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '14px' }} />
                </div>

                {/* Workspace name */}
                <StatusItem onClick={() => setActiveSidebarView('explorer-view')} title="Open Explorer">
                    <i className="codicon codicon-layout-sidebar-left" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }} />
                    <span>{useStore.getState().activeRootName || 'vscodium-rust'}</span>
                </StatusItem>

                {/* Git branch */}
                {gitBranch && (
                    <StatusItem onClick={() => setActiveSidebarView('scm-view')} title="Source Control">
                        <i className="codicon codicon-source-control" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }} />
                        <span>{gitBranch}</span>
                    </StatusItem>
                )}

                {/* Sync icon */}
                <StatusItem title="Synchronize Changes">
                    <i className="codicon codicon-sync" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }} />
                </StatusItem>

                {/* Errors / Warnings — click to open Problems panel */}
                <StatusItem
                    onClick={openProblems}
                    title={`${errorCount} errors, ${warnCount} warnings`}
                    danger={errorCount > 0}
                >
                    <i className="codicon codicon-error" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', color: errorCount > 0 ? '#f87171' : 'inherit' }} />
                    <span style={{ color: errorCount > 0 ? '#f87171' : 'inherit' }}>{errorCount}</span>
                    <i className="codicon codicon-warning" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px', marginLeft: '4px', color: warnCount > 0 ? '#fbbf24' : 'inherit' }} />
                    <span style={{ color: warnCount > 0 ? '#fbbf24' : 'inherit' }}>{warnCount}</span>
                </StatusItem>

                {/* Codebase Indexing Progress Indicator */}
                {activeRoot && (
                    <StatusItem
                        onClick={async () => {
                            await startIndexingCodebase();
                        }}
                        title={
                            isIndexingCodebase
                                ? `Indexing in progress: ${indexingProgress?.files_processed ?? 0}/${indexingProgress?.total_files ?? 0} files. Click to re-index.`
                                : "Codebase fully indexed. Click to re-index."
                        }
                    >
                        <i
                            className={`codicon ${isIndexingCodebase ? 'codicon-sync animate-spin' : 'codicon-database'}`}
                            style={{
                                fontFamily: 'codicon',
                                fontStyle: 'normal',
                                fontSize: '12px',
                                color: isIndexingCodebase ? '#00c6ff' : '#4ade80',
                                display: 'inline-block'
                            }}
                        />
                        <span style={{ fontSize: '11px', display: 'flex', alignItems: 'center', gap: '4px' }}>
                            {isIndexingCodebase ? (
                                <>
                                    <span>Indexing</span>
                                    <span style={{ fontWeight: 600, color: '#00c6ff' }}>
                                        {indexingProgress?.progress_percent != null ? `${indexingProgress.progress_percent}%` : '...'}
                                    </span>
                                </>
                            ) : (
                                <span style={{ opacity: 0.9 }}>Codebase Indexed</span>
                            )}
                        </span>
                    </StatusItem>
                )}

                {/* Model picker */}
                <div ref={modelPickerRef} style={{ position: 'relative' }}>
                    <div
                        className="status-item hoverable"
                        onClick={openModelPicker}
                        style={{
                            cursor: 'pointer',
                            display: 'flex',
                            alignItems: 'center',
                            height: '22px',
                            padding: '0 8px',
                            marginLeft: '4px',
                            borderLeft: '1px solid rgba(255,255,255,0.1)',
                            gap: '6px',
                        }}
                        title="Click to change AI model"
                    >
                        <i className="codicon codicon-sparkle" style={{
                            fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px',
                            color: useStore.getState().isAgentThinking ? '#4ade80' : 'rgba(255,255,255,0.7)',
                            animation: useStore.getState().isAgentThinking ? 'spin 2s linear infinite' : 'none',
                        }} />
                        <span style={{ fontSize: '11px', opacity: 0.9 }}>
                            {agentModel.split('|').pop()?.split(':')[0].toUpperCase() || 'AGENT'}
                        </span>
                        <i className="codicon codicon-chevron-up" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '9px', opacity: 0.5 }} />
                        {agentModel.toLowerCase().includes('ollama') && (
                            <div
                                title={ollamaStatus === 'running' ? 'Ollama: Connected' : 'Ollama: Not Connected'}
                                style={{
                                    width: '6px', height: '6px', borderRadius: '50%',
                                    background: ollamaStatus === 'running' ? '#10b981' : '#f43f5e',
                                    boxShadow: ollamaStatus === 'running' ? '0 0 4px #10b981' : 'none',
                                }}
                            />
                        )}
                    </div>

                    {modelPickerOpen && pickerPos && (
                        <div style={{
                            position: 'fixed',
                            bottom: pickerPos.bottom,
                            left: pickerPos.left,
                            width: '300px',
                            maxHeight: '360px',
                            overflowY: 'auto',
                            background: 'var(--vscode-menu-background, #1e1e1e)',
                            border: '1px solid var(--vscode-menu-border, rgba(255,255,255,0.15))',
                            borderRadius: '6px',
                            boxShadow: '0 8px 32px rgba(0,0,0,0.5)',
                            zIndex: 99999,
                            padding: '4px 0',
                            fontSize: '12px',
                        }}>
                            <div style={{ padding: '6px 12px', fontSize: '10px', opacity: 0.5, fontWeight: 600, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Select Model</div>
                            {availableModels.length === 0 && (
                                <div style={{ padding: '8px 12px', opacity: 0.5, fontSize: '11px' }}>Loading models…</div>
                            )}
                            {Object.entries(
                                availableModels.reduce((acc: Record<string, any[]>, m: any) => {
                                    const p = m.provider || 'other';
                                    if (!acc[p]) acc[p] = [];
                                    acc[p].push(m);
                                    return acc;
                                }, {})
                            ).map(([provider, models]) => (
                                <div key={provider}>
                                    <div style={{ padding: '4px 12px', fontSize: '10px', opacity: 0.45, fontWeight: 600, textTransform: 'uppercase' }}>{provider}</div>
                                    {(models as any[]).map((m: any) => {
                                        const modelValue = `${m.provider.charAt(0).toUpperCase() + m.provider.slice(1)}|${m.id}`;
                                        const isActive = agentModel === modelValue || agentModel.endsWith(`|${m.id}`);
                                        return (
                                            <div
                                                key={m.id}
                                                onClick={() => { setAgentModel(modelValue); setModelPickerOpen(false); }}
                                                style={{
                                                    padding: '5px 12px 5px 20px',
                                                    cursor: 'pointer',
                                                    background: isActive ? 'var(--vscode-list-activeSelectionBackground, rgba(0,122,204,0.3))' : 'transparent',
                                                    color: isActive ? 'var(--vscode-list-activeSelectionForeground, #fff)' : 'inherit',
                                                    display: 'flex', alignItems: 'center', gap: '6px',
                                                }}
                                                onMouseEnter={e => { if (!isActive) e.currentTarget.style.background = 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.05))'; }}
                                                onMouseLeave={e => { if (!isActive) e.currentTarget.style.background = 'transparent'; }}
                                            >
                                                {isActive && <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />}
                                                {!isActive && <span style={{ width: '13px' }} />}
                                                <span style={{ overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{m.id}</span>
                                            </div>
                                        );
                                    })}
                                </div>
                            ))}
                        </div>
                    )}
                </div>
            </div>

            {/* ── RIGHT ────────────────────────────────────────────────────── */}
            <div className="status-right" style={{ display: 'flex', alignItems: 'center', height: '100%' }}>

                {/* Account / plan + usage (click → Account settings) */}
                <StatusItem
                    onClick={() => openSettings?.('agent')}
                    title={acct?.signedIn ? `Signed in · ${acct.tier} plan — click for Account & Subscription` : 'Not signed in — click to sign in / subscribe'}
                    accent={!!acct?.signedIn}
                >
                    <i className="codicon codicon-account" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }} />
                    {acct && (acct.signedIn ? (
                        <span style={{ fontSize: '11px' }}>
                            {acct.tier}
                            {(() => {
                                const useDay = acct.limitDay > 0; // Community is day-capped; paid tiers month-capped
                                const used = useDay ? acct.usedDay : acct.usedMonth;
                                const lim = useDay ? acct.limitDay : acct.limitMonth;
                                if (lim <= 0) return null; // unlimited
                                const fmt = (n: number) => (n >= 1000 ? `${(n / 1000).toFixed(n % 1000 === 0 ? 0 : 1)}k` : `${n}`);
                                return <span style={{ opacity: 0.6 }}> · {fmt(used)}/{fmt(lim)}</span>;
                            })()}
                        </span>
                    ) : <span style={{ fontSize: '11px' }}>Sign in</span>)}
                </StatusItem>

                {/* Token budget */}
                {agentMessages.length > 0 && (
                    <StatusItem
                        title={`Context: ~${tokenBudget.used.toLocaleString()} / ${tokenBudget.max.toLocaleString()} tokens (${tokenBudget.pct}%)`}
                        danger={tokenBudget.pct > 85}
                        accent={tokenBudget.pct < 50}
                    >
                        <i className="codicon codicon-symbol-keyword" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />
                        <span style={{ fontSize: '11px' }}>
                            {tokenBudget.used >= 1000 ? `${(tokenBudget.used / 1000).toFixed(1)}k` : tokenBudget.used}
                            <span style={{ opacity: 0.5 }}>/{tokenBudget.max >= 1000 ? `${(tokenBudget.max / 1000).toFixed(0)}k` : tokenBudget.max}</span>
                        </span>
                        <div style={{
                            width: '36px', height: '3px', borderRadius: '2px', overflow: 'hidden',
                            background: 'rgba(255,255,255,0.15)', marginLeft: '2px',
                        }}>
                            <div style={{
                                width: `${tokenBudget.pct}%`, height: '100%',
                                background: tokenBudget.pct > 85 ? '#f87171' : tokenBudget.pct > 60 ? '#fbbf24' : '#4ade80',
                                transition: 'width 0.3s',
                            }} />
                        </div>
                    </StatusItem>
                )}

                {/* Go to Line / Column */}
                <div style={{ position: 'relative' }}>
                    <StatusItem onClick={openGotoLine} title="Go to Line/Column (Ctrl+G)">
                        <span>Ln {cursorLine}, Col {cursorCol}</span>
                        {selectionCount > 0 && <span style={{ opacity: 0.6 }}>({selectionCount} selected)</span>}
                    </StatusItem>

                    {gotoLineOpen && (
                        <div
                            style={{
                                position: 'fixed',
                                bottom: '26px',
                                right: '240px',
                                background: 'var(--vscode-menu-background, #252526)',
                                border: '1px solid var(--vscode-menu-border, var(--vscode-panel-border, #454545))',
                                borderRadius: '6px',
                                padding: '8px',
                                zIndex: 99999,
                                display: 'flex',
                                gap: '6px',
                                alignItems: 'center',
                                boxShadow: '0 4px 16px rgba(0,0,0,0.5)',
                            }}
                        >
                            <input
                                autoFocus
                                type="number"
                                value={gotoLineValue}
                                onChange={e => setGotoLineValue(e.target.value)}
                                onKeyDown={e => { if (e.key === 'Enter') submitGotoLine(); if (e.key === 'Escape') setGotoLineOpen(false); }}
                                placeholder="Line number"
                                style={{
                                    width: '100px',
                                    background: 'var(--vscode-input-background)',
                                    border: '1px solid var(--vscode-focusBorder)',
                                    color: 'var(--vscode-editor-foreground)',
                                    padding: '3px 6px',
                                    fontSize: '12px',
                                    outline: 'none',
                                    borderRadius: '3px',
                                }}
                            />
                            <button
                                onClick={submitGotoLine}
                                style={{
                                    background: 'var(--vscode-button-background, #0e639c)',
                                    color: 'var(--vscode-editor-foreground, #fff)',
                                    border: 'none',
                                    borderRadius: '3px',
                                    padding: '3px 8px',
                                    fontSize: '11px',
                                    cursor: 'pointer',
                                }}
                            >Go</button>
                        </div>
                    )}
                </div>

                {/* Indentation */}
                <div ref={indentRef} style={{ position: 'relative' }}>
                    <StatusItem
                        onClick={() => {
                            if (indentRef.current) {
                                const rect = indentRef.current.getBoundingClientRect();
                                setIndentPos({ left: rect.left, bottom: window.innerHeight - rect.top });
                            }
                            setIndentOpen(v => !v);
                        }}
                        title="Select indentation"
                    >
                        <span>{insertSpaces ? `Spaces: ${tabSize}` : `Tab Size: ${tabSize}`}</span>
                    </StatusItem>
                    {indentOpen && indentPos && (
                        <div style={{
                            position: 'fixed', bottom: indentPos.bottom, left: indentPos.left,
                            background: 'var(--vscode-menu-background, #252526)',
                            border: '1px solid var(--vscode-menu-border, var(--vscode-panel-border, #454545))',
                            borderRadius: '6px', zIndex: 99999,
                            boxShadow: '0 4px 16px rgba(0,0,0,0.5)',
                            minWidth: '200px', padding: '4px 0',
                        }}>
                            <div style={{ padding: '4px 12px', fontSize: '10px', opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Indent Using</div>
                            {['Spaces', 'Tabs'].map(mode => (
                                <div key={mode}
                                    onClick={() => {
                                        window.dispatchEvent(new CustomEvent('editor:set-indent', { detail: { insertSpaces: mode === 'Spaces' } }));
                                        setIndentOpen(false);
                                    }}
                                    style={{ padding: '5px 12px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '8px' }}
                                    onMouseEnter={e => e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'}
                                    onMouseLeave={e => e.currentTarget.style.background = 'transparent'}
                                >
                                    {(mode === 'Spaces') === insertSpaces && <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />}
                                    {!((mode === 'Spaces') === insertSpaces) && <span style={{ width: '13px' }} />}
                                    {mode}
                                </div>
                            ))}
                            <div style={{ height: '1px', background: 'rgba(255,255,255,0.08)', margin: '3px 0' }} />
                            <div style={{ padding: '4px 12px', fontSize: '10px', opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Indent Size</div>
                            {[2, 4, 8].map(size => (
                                <div key={size}
                                    onClick={() => {
                                        window.dispatchEvent(new CustomEvent('editor:set-tab-size', { detail: { size } }));
                                        setIndentOpen(false);
                                    }}
                                    style={{ padding: '5px 12px', cursor: 'pointer', display: 'flex', alignItems: 'center', gap: '8px' }}
                                    onMouseEnter={e => e.currentTarget.style.background = 'var(--vscode-list-hoverBackground)'}
                                    onMouseLeave={e => e.currentTarget.style.background = 'transparent'}
                                >
                                    {tabSize === size && <i className="codicon codicon-check" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '11px' }} />}
                                    {tabSize !== size && <span style={{ width: '13px' }} />}
                                    {size}
                                </div>
                            ))}
                        </div>
                    )}
                </div>

                {/* Encoding */}
                <StatusItem title="Select Encoding">
                    <span>UTF-8</span>
                </StatusItem>

                {/* Language mode */}
                {activeTab && (
                    <StatusItem title="Select Language Mode" onClick={() => window.dispatchEvent(new CustomEvent('editor:pick-language'))}>
                        <span>{languageLabel(language)}</span>
                    </StatusItem>
                )}

                {/* Memory — subtle RAM readout; click to optimize (folds the old
                    RAM/CPU + KB-saved + Optimize trio into one native-looking item). */}
                {processStats && (
                    <StatusItem
                        onClick={handleOptimize}
                        title={`RAM ${processStats.memory_mb.toFixed(0)}MB · CPU ${processStats.cpu_usage.toFixed(0)}% · ${processStats.available_ram_gb}GB free — click to optimize memory`}
                        danger={processStats.available_ram_gb > 0 && processStats.available_ram_gb < 1}
                    >
                        <i className="codicon codicon-pulse" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '12px' }} />
                        <span>{processStats.memory_mb.toFixed(0)} MB</span>
                    </StatusItem>
                )}
            </div>
        </footer>
    );
};

export default StatusBar;
