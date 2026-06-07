import React, { useCallback, useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';

interface Policy {
    org_name: string;
    audit_enabled: boolean;
    require_secure_mode: boolean;
    audit_retention_days: number;
}

interface AuditEntry {
    ts: number;
    actor: string;
    action: string;
    detail?: Record<string, unknown>;
}

const EnterprisePanel: React.FC = () => {
    const [policy, setPolicy] = useState<Policy | null>(null);
    const [audit, setAudit] = useState<AuditEntry[]>([]);
    const [msg, setMsg] = useState('');
    const [busy, setBusy] = useState(false);

    const refresh = useCallback(() => {
        invoke<Policy>('enterprise_get_policy').then(setPolicy).catch(() => setPolicy(null));
        invoke<AuditEntry[]>('enterprise_audit_list', { limit: 40 }).then(setAudit).catch(() => setAudit([]));
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
        setMsg('Exporting…');
        invoke<string>('enterprise_audit_export')
            .then((p) => setMsg(`Exported to ${p}`))
            .catch((e) => setMsg(String(e)));
    };

    if (!policy) {
        return (
            <div style={{ padding: 8, opacity: 0.7, fontSize: 12 }}>
                Enterprise policy unavailable (offline or not entitled).
            </div>
        );
    }

    return (
        <div style={{ padding: '4px 4px 32px', color: 'var(--vscode-foreground)' }}>
            <h2 style={{ fontSize: 18, fontWeight: 600, margin: '0 0 4px' }}>Enterprise</h2>
            <p style={{ fontSize: 12, opacity: 0.6, margin: '0 0 16px' }}>
                Org policy, secure defaults, and compliance audit trail (JSONL in your config dir).
            </p>

            <label style={{ display: 'block', fontSize: 11, opacity: 0.7, marginBottom: 4 }}>Organization name</label>
            <input
                value={policy.org_name}
                onChange={(e) => setPolicy({ ...policy, org_name: e.target.value })}
                style={{ width: '100%', maxWidth: 420, marginBottom: 12, padding: '8px 10px', borderRadius: 6, border: '1px solid var(--vscode-panel-border)', background: 'var(--vscode-input-background)', color: 'inherit' }}
            />

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 8 }}>
                <input
                    type="checkbox"
                    checked={policy.require_secure_mode}
                    onChange={(e) => setPolicy({ ...policy, require_secure_mode: e.target.checked })}
                />
                Require Secure autonomy mode (prompt before risky actions)
            </label>
            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, marginBottom: 8 }}>
                <input
                    type="checkbox"
                    checked={policy.audit_enabled}
                    onChange={(e) => setPolicy({ ...policy, audit_enabled: e.target.checked })}
                />
                Enable audit logging
            </label>

            <div style={{ display: 'flex', gap: 8, marginBottom: 20 }}>
                <button type="button" disabled={busy} onClick={save} className="settings-button success">
                    Save policy
                </button>
                <button type="button" onClick={exportLog} className="settings-button">
                    Export audit log
                </button>
            </div>
            {msg && <div style={{ fontSize: 11, opacity: 0.75, marginBottom: 16 }}>{msg}</div>}

            <h3 style={{ fontSize: 13, fontWeight: 600, margin: '0 0 8px' }}>Recent audit events</h3>
            <div style={{ maxHeight: 280, overflow: 'auto', fontSize: 11, fontFamily: 'var(--vscode-editor-font-family, monospace)', border: '1px solid var(--vscode-panel-border)', borderRadius: 8, padding: 8 }}>
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
