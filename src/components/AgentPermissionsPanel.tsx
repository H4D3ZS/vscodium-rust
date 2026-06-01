import React, { useState } from 'react';
import { useStore } from '../store';

// Agent Permissions — security mode + granular controls (original implementation,
// referencing the Antigravity "Settings — Permissions" layout). The security mode
// maps onto our real autonomy flags (YOLO / auto-accept) so it isn't cosmetic;
// the granular toggles persist to localStorage for the relevant subsystems.

type SecMode = 'full' | 'sandboxed' | 'strict';

function getLS(key: string, fallback: string): string {
    try { return localStorage.getItem(key) ?? fallback; } catch { return fallback; }
}
function setLS(key: string, value: string) {
    try { localStorage.setItem(key, value); } catch { /* */ }
}

const MODES: { id: SecMode; title: string; desc: string }[] = [
    { id: 'full', title: 'Full access', desc: 'Agents have full access to your machine and external resources.' },
    { id: 'sandboxed', title: 'Sandboxed', desc: 'Agents run in a secure sandbox that restricts access to external resources outside your trusted folders.' },
    { id: 'strict', title: 'Strict', desc: 'Terminal commands always require review and the agent cannot access files outside its given workspaces.' },
];

const AgentPermissionsPanel: React.FC = () => {
    const setYoloMode = useStore(s => s.setYoloMode);
    const setAutoAccept = useStore(s => (s as any).setAutoAcceptChanges);

    const [mode, setMode] = useState<SecMode>(() => getLS('agent.securityMode', 'full') as SecMode);
    const [termAuto, setTermAuto] = useState(() => getLS('agent.terminalAutoExec', 'proceed'));
    const [shellInt, setShellInt] = useState(() => getLS('agent.shellIntegration', '1') === '1');
    const [nonWsFiles, setNonWsFiles] = useState(() => getLS('agent.nonWorkspaceFileAccess', '0') === '1');
    const [autoOpen, setAutoOpen] = useState(() => getLS('agent.autoOpenEdited', '1') === '1');
    const [reviewPolicy, setReviewPolicy] = useState(() => getLS('agent.reviewPolicy', 'proceed'));

    const applyMode = (m: SecMode) => {
        setMode(m);
        setLS('agent.securityMode', m);
        // Map to real autonomy flags.
        if (m === 'full') {
            setYoloMode?.(true); setAutoAccept?.(true);
            setTermAuto('proceed'); setLS('agent.terminalAutoExec', 'proceed');
            setReviewPolicy('proceed'); setLS('agent.reviewPolicy', 'proceed');
            setNonWsFiles(true); setLS('agent.nonWorkspaceFileAccess', '1');
        } else if (m === 'sandboxed') {
            setYoloMode?.(false); setAutoAccept?.(true);
            setTermAuto('ask'); setLS('agent.terminalAutoExec', 'ask');
            setReviewPolicy('proceed'); setLS('agent.reviewPolicy', 'proceed');
            setNonWsFiles(false); setLS('agent.nonWorkspaceFileAccess', '0');
        } else {
            setYoloMode?.(false); setAutoAccept?.(false);
            setTermAuto('ask'); setLS('agent.terminalAutoExec', 'ask');
            setReviewPolicy('ask'); setLS('agent.reviewPolicy', 'ask');
            setNonWsFiles(false); setLS('agent.nonWorkspaceFileAccess', '0');
        }
    };

    return (
        <div style={{ padding: '4px 4px 40px', color: 'var(--vscode-foreground)' }}>
            <h2 style={{ fontSize: 18, fontWeight: 600, margin: '0 0 4px' }}>Agent security mode</h2>
            <p style={{ fontSize: 12, opacity: 0.65, margin: '0 0 16px' }}>
                Select one of the three options. Agent settings and permissions can be further customized below.
            </p>

            {/* Mode cards */}
            <div style={{ display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(220px, 1fr))', gap: 12, marginBottom: 28 }}>
                {MODES.map(m => {
                    const active = mode === m.id;
                    return (
                        <div
                            key={m.id}
                            onClick={() => applyMode(m.id)}
                            style={{
                                cursor: 'pointer',
                                padding: '14px 16px',
                                borderRadius: 8,
                                border: `1px solid ${active ? 'var(--vscode-focusBorder, #4daafc)' : 'var(--vscode-panel-border, rgba(255,255,255,0.12))'}`,
                                background: active ? 'var(--vscode-list-activeSelectionBackground, rgba(77,170,252,0.12))' : 'var(--vscode-editorWidget-background, rgba(255,255,255,0.02))',
                                transition: 'border-color .12s, background .12s',
                            }}
                        >
                            <div style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 14, fontWeight: 600, marginBottom: 6 }}>
                                <i className={`codicon codicon-${m.id === 'full' ? 'unlock' : m.id === 'sandboxed' ? 'shield' : 'lock'}`} style={{ opacity: 0.8 }} />
                                {m.title}
                            </div>
                            <div style={{ fontSize: 12, opacity: 0.6, lineHeight: 1.45 }}>{m.desc}</div>
                        </div>
                    );
                })}
            </div>

            <Section title="Terminal">
                <RowSelect
                    title="Terminal Command Auto Execution"
                    desc="Controls whether terminal commands require your approval before running."
                    value={termAuto}
                    options={[['proceed', 'Always Proceed'], ['ask', 'Ask First']]}
                    onChange={(v) => { setTermAuto(v); setLS('agent.terminalAutoExec', v); }}
                />
                <RowToggle
                    title="Enable Shell Integration"
                    desc="When enabled, Agent will use the IDE's shell integration to detect and report terminal command execution."
                    value={shellInt}
                    onChange={(v) => { setShellInt(v); setLS('agent.shellIntegration', v ? '1' : '0'); }}
                />
            </Section>

            <Section title="File Access">
                <RowToggle
                    title="Agent Non-Workspace File Access"
                    desc="Allows the agent to access files outside of your current workspace."
                    value={nonWsFiles}
                    onChange={(v) => { setNonWsFiles(v); setLS('agent.nonWorkspaceFileAccess', v ? '1' : '0'); }}
                />
                <RowToggle
                    title="Auto-Open Edited Files"
                    desc="Open files in the background if Agent creates or edits them."
                    value={autoOpen}
                    onChange={(v) => { setAutoOpen(v); setLS('agent.autoOpenEdited', v ? '1' : '0'); }}
                />
            </Section>

            <Section title="Planning">
                <RowSelect
                    title="Review Policy"
                    desc="Specifies the Agent's behavior when asking for review on artifacts it creates."
                    value={reviewPolicy}
                    options={[['proceed', 'Always Proceed'], ['ask', 'Ask First']]}
                    onChange={(v) => { setReviewPolicy(v); setLS('agent.reviewPolicy', v); }}
                />
            </Section>
        </div>
    );
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ marginBottom: 24 }}>
        <div style={{ fontSize: 11, textTransform: 'uppercase', letterSpacing: '0.06em', opacity: 0.5, fontWeight: 600, marginBottom: 10 }}>{title}</div>
        <div style={{ display: 'flex', flexDirection: 'column', gap: 10 }}>{children}</div>
    </div>
);

const rowStyle: React.CSSProperties = {
    display: 'flex', alignItems: 'center', justifyContent: 'space-between', gap: 16,
    padding: '12px 14px', borderRadius: 8,
    border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.08))',
    background: 'var(--vscode-editorWidget-background, rgba(255,255,255,0.015))',
};

const RowToggle: React.FC<{ title: string; desc: string; value: boolean; onChange: (v: boolean) => void }> = ({ title, desc, value, onChange }) => (
    <div style={rowStyle}>
        <div style={{ minWidth: 0 }}>
            <div style={{ fontSize: 13, fontWeight: 600 }}>{title}</div>
            <div style={{ fontSize: 12, opacity: 0.55, marginTop: 2 }}>{desc}</div>
        </div>
        <div
            onClick={() => onChange(!value)}
            role="switch"
            aria-checked={value}
            style={{
                cursor: 'pointer', flexShrink: 0,
                width: 38, height: 20, borderRadius: 999, padding: 2,
                background: value ? 'var(--vscode-focusBorder, #4daafc)' : 'rgba(255,255,255,0.18)',
                transition: 'background .15s',
            }}
        >
            <div style={{
                width: 16, height: 16, borderRadius: '50%', background: '#fff',
                transform: value ? 'translateX(18px)' : 'translateX(0)', transition: 'transform .15s',
            }} />
        </div>
    </div>
);

const RowSelect: React.FC<{ title: string; desc: string; value: string; options: [string, string][]; onChange: (v: string) => void }> = ({ title, desc, value, options, onChange }) => (
    <div style={rowStyle}>
        <div style={{ minWidth: 0 }}>
            <div style={{ fontSize: 13, fontWeight: 600 }}>{title}</div>
            <div style={{ fontSize: 12, opacity: 0.55, marginTop: 2 }}>{desc}</div>
        </div>
        <select
            value={value}
            onChange={(e) => onChange(e.target.value)}
            style={{
                flexShrink: 0,
                background: 'var(--vscode-dropdown-background, #2a2a2a)',
                color: 'var(--vscode-dropdown-foreground, #ddd)',
                border: '1px solid var(--vscode-dropdown-border, rgba(255,255,255,0.15))',
                borderRadius: 6, padding: '5px 10px', fontSize: 12, outline: 'none', cursor: 'pointer',
            }}
        >
            {options.map(([v, label]) => <option key={v} value={v}>{label}</option>)}
        </select>
    </div>
);

export default AgentPermissionsPanel;
