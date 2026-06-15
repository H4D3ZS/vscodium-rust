import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';

interface Policy {
    org_name: string;
    audit_enabled: boolean;
    require_secure_mode: boolean;
    audit_retention_days: number;
    offline_only: boolean;
    allowed_models: string[];
    blocked_models: string[];
    allowed_mcp_servers: string[];
    engagement_targets: string[];
    engagement_id: string;
    require_engagement_scope: boolean;
    block_private_network_scan: boolean;
    tool_allowlist: string[];
    tool_denylist: string[];
    siem_webhook_url: string;
    dlp_redact_secrets: boolean;
    audit_tool_calls: boolean;
    audit_file_writes: boolean;
    audit_model_calls: boolean;
}

interface AuditEntry {
    ts: number;
    actor: string;
    action: string;
    detail?: Record<string, unknown>;
}

const inputStyle: React.CSSProperties = {
    width: '100%',
    maxWidth: 520,
    marginBottom: 10,
    padding: '8px 10px',
    borderRadius: 6,
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-input-background)',
    color: 'inherit',
    fontSize: 12,
};

const listToText = (arr: string[]) => arr.join('\n');
const textToList = (s: string) =>
    s.split(/[\n,]/).map((x) => x.trim()).filter(Boolean);

const EnterprisePanel: React.FC = () => {
    const [policy, setPolicy] = useState<Policy | null>(null);
    const [audit, setAudit] = useState<AuditEntry[]>([]);
    const [msg, setMsg] = useState('');
    const [busy, setBusy] = useState(false);
    const [engagementId, setEngagementId] = useState('');
    const [engagementTargets, setEngagementTargets] = useState('');

    const refresh = useCallback(() => {
        invoke<Policy>('enterprise_get_policy').then(setPolicy).catch(() => setPolicy(null));
        invoke<AuditEntry[]>('enterprise_audit_list', { limit: 50 }).then(setAudit).catch(() => setAudit([]));
    }, []);

    useEffect(() => {
        refresh();
    }, [refresh]);

    const save = () => {
        if (!policy) return;
        setBusy(true);
        invoke('enterprise_set_policy', { policy })
            .then(() => {
                setMsg('Policy saved.');
                refresh();
            })
            .catch((e) => setMsg(String(e)))
            .finally(() => setBusy(false));
    };

    const exportLog = () => {
        setMsg('Exporting audit…');
        invoke<string>('enterprise_audit_export')
            .then((p) => setMsg(`Audit exported: ${p}`))
            .catch((e) => setMsg(String(e)));
    };

    const exportSarif = () => {
        setMsg('Building SARIF…');
        invoke<string>('enterprise_export_sarif')
            .then((p) => setMsg(`SARIF exported: ${p}`))
            .catch((e) => setMsg(String(e)));
    };

    const seedCyber = () => {
        setBusy(true);
        invoke<Policy>('enterprise_seed_cyber_policy', { orgName: policy?.org_name || 'Security Team' })
            .then((p) => {
                setPolicy(p);
                setMsg('Cyber-enterprise defaults applied.');
            })
            .catch((e) => setMsg(String(e)))
            .finally(() => setBusy(false));
    };

    const startEngagement = () => {
        const id = engagementId.trim() || `eng-${Date.now()}`;
        const targets = textToList(engagementTargets);
        if (targets.length === 0) {
            setMsg('Add at least one in-scope host (e.g. *.customer.com)');
            return;
        }
        setBusy(true);
        invoke('enterprise_init_engagement', { engagementId: id, targets })
            .then(() => {
                setMsg(`Engagement "${id}" initialized — reports/recon/exploits folders created.`);
                refresh();
            })
            .catch((e) => setMsg(String(e)))
            .finally(() => setBusy(false));
    };

    if (!policy) {
        return (
            <div style={{ padding: 8, opacity: 0.7, fontSize: 12 }}>
                Enterprise policy unavailable.{' '}
                <button type="button" className="settings-button" onClick={seedCyber}>Seed defaults</button>
            </div>
        );
    }

    return (
        <div style={{ padding: '4px 4px 32px', color: 'var(--vscode-foreground)' }}>
            <h2 style={{ fontSize: 18, fontWeight: 600, margin: '0 0 4px' }}>Enterprise</h2>
            <p style={{ fontSize: 12, opacity: 0.6, margin: '0 0 16px' }}>
                Org policy, engagement scope, audit trail, DLP, and compliance exports.
            </p>

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '16px 0 8px' }}>Organization</h3>
            <input
                value={policy.org_name}
                onChange={(e) => setPolicy({ ...policy, org_name: e.target.value })}
                placeholder="Organization name"
                style={inputStyle}
            />

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '16px 0 8px' }}>Security posture</h3>
            {[
                ['require_secure_mode', 'Require secure autonomy (prompt before risky actions)'],
                ['audit_enabled', 'Enable audit logging'],
                ['offline_only', 'Offline only — block cloud models'],
                ['block_private_network_scan', 'Block private/localhost scans in offensive modes'],
                ['require_engagement_scope', 'Require in-scope targets for offensive tools'],
                ['dlp_redact_secrets', 'DLP — redact secrets in audit logs'],
                ['audit_tool_calls', 'Audit every tool execution'],
                ['audit_model_calls', 'Audit model invocations'],
            ].map(([key, label]) => (
                <label key={key} style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 6 }}>
                    <input
                        type="checkbox"
                        checked={!!policy[key as keyof Policy]}
                        onChange={(e) => setPolicy({ ...policy, [key]: e.target.checked })}
                    />
                    {label}
                </label>
            ))}

            <label style={{ display: 'block', fontSize: 11, opacity: 0.7, marginTop: 12 }}>SIEM webhook URL (optional)</label>
            <input
                value={policy.siem_webhook_url}
                onChange={(e) => setPolicy({ ...policy, siem_webhook_url: e.target.value })}
                placeholder="https://siem.example.com/ingest"
                style={inputStyle}
            />

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '16px 0 8px' }}>Engagement (pentest scope)</h3>
            <div style={{ fontSize: 11, opacity: 0.65, marginBottom: 8 }}>
                Current: {policy.engagement_id || '—'} · Targets: {policy.engagement_targets.join(', ') || 'none'}
            </div>
            <input value={engagementId} onChange={(e) => setEngagementId(e.target.value)} placeholder="engagement-id" style={inputStyle} />
            <textarea
                value={engagementTargets}
                onChange={(e) => setEngagementTargets(e.target.value)}
                placeholder="In-scope hosts (one per line):&#10;*.customer.com&#10;api.target.io"
                rows={4}
                style={{ ...inputStyle, fontFamily: 'monospace' }}
            />
            <button type="button" className="settings-button" disabled={busy} onClick={startEngagement} style={{ marginBottom: 16 }}>
                Initialize engagement folders + lock scope
            </button>

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '16px 0 8px' }}>Allowlists</h3>
            <label style={{ fontSize: 11, opacity: 0.7 }}>Allowed models (empty = all)</label>
            <textarea
                value={listToText(policy.allowed_models)}
                onChange={(e) => setPolicy({ ...policy, allowed_models: textToList(e.target.value) })}
                rows={2}
                style={{ ...inputStyle, fontFamily: 'monospace' }}
            />
            <label style={{ fontSize: 11, opacity: 0.7 }}>Blocked models</label>
            <textarea
                value={listToText(policy.blocked_models)}
                onChange={(e) => setPolicy({ ...policy, blocked_models: textToList(e.target.value) })}
                rows={2}
                style={{ ...inputStyle, fontFamily: 'monospace' }}
            />
            <label style={{ fontSize: 11, opacity: 0.7 }}>Allowed MCP servers (empty = all)</label>
            <textarea
                value={listToText(policy.allowed_mcp_servers)}
                onChange={(e) => setPolicy({ ...policy, allowed_mcp_servers: textToList(e.target.value) })}
                rows={2}
                style={{ ...inputStyle, fontFamily: 'monospace' }}
            />
            <label style={{ fontSize: 11, opacity: 0.7 }}>Tool denylist</label>
            <textarea
                value={listToText(policy.tool_denylist)}
                onChange={(e) => setPolicy({ ...policy, tool_denylist: textToList(e.target.value) })}
                rows={2}
                style={{ ...inputStyle, fontFamily: 'monospace' }}
            />

            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 8, margin: '20px 0' }}>
                <button type="button" disabled={busy} onClick={save} className="settings-button success">Save policy</button>
                <button type="button" onClick={exportLog} className="settings-button">Export audit JSONL</button>
                <button type="button" onClick={exportSarif} className="settings-button">Export SARIF</button>
                <button type="button" onClick={seedCyber} className="settings-button">Reset cyber defaults</button>
            </div>
            {msg && <div style={{ fontSize: 11, opacity: 0.75, marginBottom: 16 }}>{msg}</div>}

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '0 0 8px' }}>Recent audit events</h3>
            <div style={{ maxHeight: 300, overflow: 'auto', fontSize: 11, fontFamily: 'var(--vscode-editor-font-family, monospace)', border: '1px solid var(--vscode-panel-border)', borderRadius: 8, padding: 8 }}>
                {audit.length === 0 ? (
                    <span style={{ opacity: 0.5 }}>No events yet.</span>
                ) : (
                    audit.map((e, i) => (
                        <div key={i} style={{ marginBottom: 6, borderBottom: '1px solid rgba(255,255,255,0.06)', paddingBottom: 4 }}>
                            <span style={{ opacity: 0.5 }}>{new Date(e.ts * 1000).toLocaleString()}</span>{' '}
                            <strong>{e.action}</strong>{' '}
                            <span style={{ opacity: 0.65 }}>({e.actor})</span>
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default EnterprisePanel;
