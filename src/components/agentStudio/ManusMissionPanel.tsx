import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../../store';
import { runManusWebMission, type ManusMissionStep } from '../../application/research/runManusWebMission';
import { openFile } from '../../application/editor/openFile';

const STATUS_COLOR: Record<string, string> = {
    pending: '#888',
    running: '#60a5fa',
    done: '#4ade80',
    error: '#f87171',
    skipped: '#666',
};

const ManusMissionPanel: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const setActiveSidebarView = useStore(s => s.setActiveSidebarView);
    const [query, setQuery] = useState('');
    const [url, setUrl] = useState('');
    const [codebaseAudit, setCodebaseAudit] = useState(true);
    const [running, setRunning] = useState(false);
    const [steps, setSteps] = useState<ManusMissionStep[]>([]);
    const [report, setReport] = useState<string | null>(null);

    useEffect(() => {
        const onPrefill = (e: Event) => {
            const detail = (e as CustomEvent<{ query?: string; url?: string }>).detail;
            if (detail?.query) setQuery(detail.query);
            if (detail?.url) setUrl(detail.url);
        };
        window.addEventListener('manus:prefill', onPrefill);
        return () => window.removeEventListener('manus:prefill', onPrefill);
    }, []);

    const run = useCallback(async () => {
        if (!query.trim()) return;
        setRunning(true);
        setReport(null);
        setSteps([]);
        try {
            const result = await runManusWebMission({
                query: query.trim(),
                targetUrl: url.trim() || undefined,
                workspaceRoot: activeRoot || undefined,
                runCodebaseAudit: codebaseAudit && !!activeRoot,
                onStep: (step) => setSteps(prev => {
                    const i = prev.findIndex(s => s.id === step.id);
                    if (i < 0) return [...prev, step];
                    return prev.map(s => s.id === step.id ? step : s);
                }),
            });
            setReport(result.report);
        } finally {
            setRunning(false);
        }
    }, [query, url, activeRoot, codebaseAudit]);

    const openFindings = () => {
        if (!activeRoot) return;
        void openFile(`${activeRoot.replace(/\//g, '\\')}\\findings.md`);
    };

    return (
        <div className="manus-panel">
            <div className="manus-panel-header">
                <div>
                    <div className="manus-panel-title">
                        <i className="codicon codicon-globe" />
                        Web Research Agent
                    </div>
                    <div className="manus-panel-sub">
                        invisible_playwright stealth Firefox · search · scrape · audit · terminal · findings
                    </div>
                </div>
                {!activeRoot && (
                    <button className="manus-link-btn" onClick={() => setActiveSidebarView('explorer-view')}>
                        Open folder first
                    </button>
                )}
            </div>

            <div className="manus-form">
                <label className="manus-label">Mission query</label>
                <textarea
                    className="manus-input"
                    value={query}
                    onChange={e => setQuery(e.target.value)}
                    placeholder="e.g. CVE-2024-XXXX mitigations, competitor pricing, API docs for OAuth2…"
                    rows={3}
                />
                <label className="manus-label">Target URL (optional)</label>
                <input
                    className="manus-input manus-input-single"
                    value={url}
                    onChange={e => setUrl(e.target.value)}
                    placeholder="https://example.com — triggers scrape + live security audit"
                />
                <label className="manus-check">
                    <input type="checkbox" checked={codebaseAudit} onChange={e => setCodebaseAudit(e.target.checked)} />
                    Run codebase security scan when workspace is open
                </label>
                <div className="manus-actions">
                    <button className="manus-run-btn" onClick={() => void run()} disabled={running || !query.trim()}>
                        {running ? 'Mission running…' : 'Run full web mission'}
                    </button>
                    {report && activeRoot && (
                        <button className="manus-secondary-btn" onClick={openFindings}>Open findings.md</button>
                    )}
                </div>
            </div>

            {steps.length > 0 && (
                <div className="manus-steps">
                    <div className="manus-label">Pipeline</div>
                    <div className="manus-step-grid">
                        {steps.map(s => (
                            <div key={s.id} className={`manus-step manus-step--${s.status}`}>
                                <span className="manus-step-dot" style={{ background: STATUS_COLOR[s.status] || '#888' }} />
                                <span className="manus-step-label">{s.label}</span>
                                {s.detail && <span className="manus-step-detail">{s.detail}</span>}
                            </div>
                        ))}
                    </div>
                </div>
            )}

            {report && (
                <div className="manus-report">
                    <div className="manus-label">Mission report</div>
                    <pre className="manus-report-body">{report}</pre>
                </div>
            )}

            <div className="manus-footer-hint">
                Tip: type <code>/manus &lt;query&gt;</code> in chat or use Quick Mission → Web Research
            </div>
        </div>
    );
};

export default ManusMissionPanel;
