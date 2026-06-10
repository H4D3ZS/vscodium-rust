import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import type { SecurityPanelTab } from '../../application/security/runCodebaseSecurityReview';

type Discipline = {
    id: string;
    label: string;
    icon: string;
    native: string[];
    external: string[];
};

type Props = {
    onNavigate: (tab: SecurityPanelTab) => void;
    onOpenModules: () => void;
};

const DISCIPLINE_TAB: Record<string, SecurityPanelTab> = {
    'web-pentest': 'vega',
    'red-team': 'arsenal',
    'malware-re': 'review',
    'mobile-pentest': 'modules',
};

const QUICK_ACTIONS: { tab: SecurityPanelTab; label: string; desc: string }[] = [
    { tab: 'vega', label: 'Vega DAST', desc: 'SQLi · XSS · CMDi · SSRF' },
    { tab: 'proxy', label: 'Intercept Proxy', desc: 'Capture · inspect · replay' },
    { tab: 'repeater', label: 'Repeater', desc: 'Craft · resend · inspect' },
    { tab: 'intruder', label: 'Intruder', desc: 'Payload fuzzing · anomalies' },
    { tab: 'oast', label: 'OAST Collaborator', desc: 'Blind SSRF/RCE callbacks' },
    { tab: 'chunks', label: 'Bundle Intel', desc: 'JS chunk secrets + XSS' },
    { tab: 'review', label: 'Code Audit', desc: 'Secrets · CWE patterns' },
    { tab: 'arsenal', label: 'Arsenal', desc: 'Shells · payloads · cheatsheets' },
];

const cardStyle: React.CSSProperties = {
    padding: '12px 14px',
    borderRadius: 8,
    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.12))',
    background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.03))',
    cursor: 'pointer',
    textAlign: 'left',
    color: 'inherit',
    width: '100%',
};

const CyberOpsOverview: React.FC<Props> = ({ onNavigate, onOpenModules }) => {
    const [disciplines, setDisciplines] = useState<Discipline[]>([]);

    useEffect(() => {
        invoke<{ disciplines: Discipline[] }>('vega_disciplines')
            .then((d) => setDisciplines(d.disciplines ?? []))
            .catch(() => {});
    }, []);

    const go = useCallback((tab: SecurityPanelTab) => () => onNavigate(tab), [onNavigate]);

    return (
        <div style={{ flex: 1, overflowY: 'auto', padding: '10px 12px' }}>
            <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 12, lineHeight: 1.5 }}>
                Cyber Ops hub — Rust-native tools load instantly. External modules install on demand from Settings.
            </div>

            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(2, 1fr)', gap: 8, marginBottom: 16 }}>
                {QUICK_ACTIONS.map((a) => (
                    <button key={a.tab} type="button" style={cardStyle} onClick={go(a.tab)}>
                        <div style={{ fontWeight: 700, fontSize: 12, marginBottom: 4 }}>{a.label}</div>
                        <div style={{ fontSize: 10, opacity: 0.55 }}>{a.desc}</div>
                    </button>
                ))}
            </div>

            <div style={{ fontSize: 10, fontWeight: 700, opacity: 0.5, marginBottom: 8, letterSpacing: 0.5 }}>
                DISCIPLINES
            </div>
            <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                {disciplines.map((d) => (
                    <button
                        key={d.id}
                        type="button"
                        style={cardStyle}
                        onClick={() => {
                            const tab = DISCIPLINE_TAB[d.id] ?? 'overview';
                            if (tab === 'modules') onOpenModules();
                            else onNavigate(tab);
                        }}
                    >
                        <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 6 }}>
                            <i className={`codicon codicon-${d.icon}`} style={{ fontFamily: 'codicon', fontStyle: 'normal' }} />
                            <span style={{ fontWeight: 700, fontSize: 12 }}>{d.label}</span>
                            {d.native.length > 0 && (
                                <span style={{ fontSize: 9, padding: '2px 6px', borderRadius: 8, background: '#1d4ed8', marginLeft: 'auto' }}>
                                    Rust-native
                                </span>
                            )}
                        </div>
                        <div style={{ fontSize: 10, opacity: 0.55 }}>
                            Built-in: {d.native.join(', ') || '—'}
                            {d.external.length > 0 && ` · Add-ons: ${d.external.slice(0, 3).join(', ')}…`}
                        </div>
                    </button>
                ))}
            </div>

            <button
                type="button"
                style={{ ...cardStyle, marginTop: 12, textAlign: 'center', opacity: 0.85 }}
                onClick={onOpenModules}
            >
                <div style={{ fontWeight: 600, fontSize: 11 }}>Install cyber modules → Settings</div>
            </button>
        </div>
    );
};

export default CyberOpsOverview;
