import React, { useCallback, useEffect, useState } from 'react';
import {
    AGENTSKILLS_SPEC,
    SKILLS_SH,
    auditBadge,
    skillStoreAudit,
    skillStoreInstall,
    skillStoreList,
    skillStoreRefresh,
    skillStoreStatus,
    skillStoreUninstall,
    type SkillAuditReport,
    type SkillInstallRecord,
    type SkillStoreStatus,
} from '../../lib/skillStore';
import { hermesIntegrationStatus } from '../../hermes/bridge';

const SkillStorePanel: React.FC = () => {
    const [status, setStatus] = useState<SkillStoreStatus | null>(null);
    const [skills, setSkills] = useState<SkillInstallRecord[]>([]);
    const [totalCatalog, setTotalCatalog] = useState<number | null>(null);
    const [source, setSource] = useState('');
    const [busy, setBusy] = useState(false);
    const [error, setError] = useState<string | null>(null);
    const [selectedAudit, setSelectedAudit] = useState<SkillAuditReport | null>(null);
    const [forceInstall, setForceInstall] = useState(false);
    const [notice, setNotice] = useState<string | null>(null);

    const refresh = useCallback(async () => {
        setError(null);
        try {
            const [st, list, hermes] = await Promise.all([
                skillStoreStatus(),
                skillStoreList(),
                hermesIntegrationStatus(),
            ]);
            setStatus(st);
            setSkills(list.skills);
            setTotalCatalog(hermes.skillsCount);
        } catch (e) {
            setError(String(e));
        }
    }, []);

    useEffect(() => {
        void refresh();
    }, [refresh]);

    const onInstall = async () => {
        const s = source.trim();
        if (!s) return;
        setBusy(true);
        setError(null);
        setSelectedAudit(null);
        try {
            const res = await skillStoreInstall(s, { force: forceInstall });
            setSelectedAudit(res.audit);
            if (res.installed_count > 1) {
                setNotice(`Installed ${res.installed_count} skills from this source`);
            } else {
                setNotice(null);
            }
            setSource('');
            await refresh();
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onUninstall = async (id: string) => {
        setBusy(true);
        setError(null);
        try {
            await skillStoreUninstall(id);
            if (selectedAudit?.skill_id === id) setSelectedAudit(null);
            await refresh();
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onAudit = async (id: string) => {
        setBusy(true);
        setError(null);
        try {
            setSelectedAudit(await skillStoreAudit(id));
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    };

    const onReauditAll = async () => {
        setBusy(true);
        try {
            await skillStoreRefresh();
            await refresh();
        } catch (e) {
            setError(String(e));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div style={{ maxWidth: 860, fontSize: 12, lineHeight: 1.55 }}>
            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Skill Store</div>
                <p style={{ opacity: 0.75, margin: '0 0 12px' }}>
                    Install <a href={AGENTSKILLS_SPEC} target="_blank" rel="noreferrer">agentskills.io</a> compatible
                    skills from <a href={SKILLS_SH} target="_blank" rel="noreferrer">skills.sh</a>. Installed skills
                    live in AppData and are loaded natively by Sentient via <code>use_skill</code> — no Python subprocess.
                </p>
                <div style={{ display: 'flex', gap: 12, flexWrap: 'wrap', marginBottom: 12 }}>
                    <Stat label="Catalog (bundled + store)" value={totalCatalog ?? '…'} />
                    <Stat label="User installed" value={status?.installedCount ?? '…'} />
                    <Stat label="Store path" value={status?.installedDir ?? '…'} mono />
                </div>
                <div style={{ display: 'flex', gap: 8, flexWrap: 'wrap', alignItems: 'center' }}>
                    <input
                        className="settings-input"
                        style={{ flex: 1, minWidth: 220 }}
                        placeholder="owner/repo  or  https://github.com/...  or  C:\path\to\skill"
                        value={source}
                        onChange={(e) => setSource(e.target.value)}
                        disabled={busy}
                    />
                    <label style={{ display: 'flex', alignItems: 'center', gap: 4, opacity: 0.85 }}>
                        <input type="checkbox" checked={forceInstall} onChange={(e) => setForceInstall(e.target.checked)} />
                        Force (override critical block)
                    </label>
                    <button type="button" disabled={busy || !source.trim()} onClick={onInstall}>
                        Install
                    </button>
                    <button type="button" disabled={busy} onClick={onReauditAll}>
                        Re-audit all
                    </button>
                    <button type="button" disabled={busy} onClick={refresh}>
                        Refresh
                    </button>
                </div>
                <p style={{ opacity: 0.65, margin: '10px 0 0', fontSize: 11 }}>
                    Example: <code>vercel-labs/skills</code> or <code>juliusbrussee/caveman</code> — same repos as{' '}
                    <code>npx skills add</code> on skills.sh.
                </p>
                {error && (
                    <pre style={{ color: '#f85149', marginTop: 10, whiteSpace: 'pre-wrap', fontSize: 11 }}>{error}</pre>
                )}
                {notice && (
                    <div style={{ color: '#9ece6a', marginTop: 10, fontSize: 11 }}>{notice}</div>
                )}
            </div>

            <div className="settings-card" style={{ marginBottom: 16 }}>
                <div className="settings-card-title">Security profiling</div>
                <p style={{ opacity: 0.75, margin: '0 0 10px' }}>
                    Each skill is scanned for executable payloads, obfuscation, and exfil patterns. Red-team /
                    pentest skills downgrade keyword hits (backdoor, payload, reverse shell) to <strong>info</strong>{' '}
                    instead of false-positive blocks. Critical findings block install unless Force is checked.
                </p>
                {selectedAudit && <AuditDetail report={selectedAudit} onClose={() => setSelectedAudit(null)} />}
            </div>

            <div className="settings-card">
                <div className="settings-card-title">Installed skills ({skills.length})</div>
                {skills.length === 0 ? (
                    <p style={{ opacity: 0.7 }}>No user-installed skills yet. Bundled Hermes skills are always available.</p>
                ) : (
                    <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                        {skills.map((sk) => {
                            const badge = auditBadge(sk);
                            return (
                                <div
                                    key={sk.id}
                                    style={{
                                        border: '1px solid var(--vscode-widget-border, rgba(255,255,255,0.1))',
                                        borderRadius: 6,
                                        padding: 10,
                                    }}
                                >
                                    <div style={{ display: 'flex', justifyContent: 'space-between', gap: 8, flexWrap: 'wrap' }}>
                                        <div>
                                            <strong>{sk.name}</strong>{' '}
                                            <span style={{ opacity: 0.6 }}>({sk.id})</span>
                                        </div>
                                        <span style={{ color: badge.color, fontWeight: 600 }}>{badge.label}</span>
                                    </div>
                                    {sk.description && <div style={{ opacity: 0.8, marginTop: 4 }}>{sk.description}</div>}
                                    <div style={{ opacity: 0.6, marginTop: 4, fontSize: 11 }}>{sk.audit_summary}</div>
                                    <div style={{ marginTop: 8, display: 'flex', gap: 8 }}>
                                        <button type="button" disabled={busy} onClick={() => onAudit(sk.id)}>
                                            Audit
                                        </button>
                                        <button type="button" disabled={busy} onClick={() => onUninstall(sk.id)}>
                                            Uninstall
                                        </button>
                                    </div>
                                </div>
                            );
                        })}
                    </div>
                )}
            </div>
        </div>
    );
};

const Stat: React.FC<{ label: string; value: string | number; mono?: boolean }> = ({ label, value, mono }) => (
    <div style={{ minWidth: 120 }}>
        <div style={{ opacity: 0.6, fontSize: 10 }}>{label}</div>
        <div style={{ fontWeight: 600, fontFamily: mono ? 'var(--vscode-editor-font-family, monospace)' : undefined, fontSize: 11 }}>
            {value}
        </div>
    </div>
);

const AuditDetail: React.FC<{ report: SkillAuditReport; onClose: () => void }> = ({ report, onClose }) => (
    <div style={{ marginTop: 8, padding: 10, background: 'rgba(255,255,255,0.03)', borderRadius: 6 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 8 }}>
            <strong>Audit: {report.skill_id}</strong>
            <button type="button" onClick={onClose}>Close</button>
        </div>
        <div style={{ marginBottom: 8 }}>{report.summary}</div>
        {report.findings.length === 0 ? (
            <div style={{ opacity: 0.7 }}>No findings.</div>
        ) : (
            <div style={{ maxHeight: 240, overflow: 'auto' }}>
                {report.findings.slice(0, 40).map((f, i) => (
                    <div key={i} style={{ marginBottom: 6, fontSize: 11 }}>
                        <span style={{
                            color: f.severity === 'critical' ? '#f85149' : f.severity === 'warning' ? '#d29922' : '#8b949e',
                            fontWeight: 600,
                        }}>
                            [{f.severity}]
                        </span>{' '}
                        {f.file}:{f.line} — {f.message}
                        {f.snippet && (
                            <pre style={{ margin: '4px 0 0', opacity: 0.75, whiteSpace: 'pre-wrap' }}>{f.snippet}</pre>
                        )}
                    </div>
                ))}
            </div>
        )}
    </div>
);

export default SkillStorePanel;
