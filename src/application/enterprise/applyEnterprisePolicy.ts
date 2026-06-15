/**
 * Apply org policy when Enterprise tier is active (secure mode, audit defaults).
 */
import { invoke } from '../../tauri_bridge';

export async function applyEnterprisePolicyFromAccount(acct: {
    account?: { tier?: string };
    entitlements?: { features?: string[] };
} | null): Promise<void> {
    if (!acct) return;
    const tier = String(acct.account?.tier || '').toLowerCase();
    const features = acct.entitlements?.features || [];
    const isEnterprise =
        tier === 'enterprise' || features.includes('team') || features.includes('amd_backend');
    if (!isEnterprise) return;

    try {
        let policy = await invoke<{
            require_secure_mode?: boolean;
            audit_enabled?: boolean;
            org_name?: string;
        }>('enterprise_get_policy');
        const seeded = localStorage.getItem('enterprise.policy.seeded') === '1';
        if (!seeded) {
            policy = {
                ...policy,
                require_secure_mode: true,
                audit_enabled: true,
            };
            await invoke('enterprise_set_policy', { policy });
            try { localStorage.setItem('enterprise.policy.seeded', '1'); } catch { /* */ }
        }
        if (policy.require_secure_mode) {
            try {
                localStorage.setItem('agent.secureModeEnabled', '1');
                localStorage.setItem('agent.securityMode', 'secure');
                localStorage.setItem('agent.browserJsPolicy', 'always_ask');
                localStorage.setItem('agent.reviewPolicy', 'always_ask');
            } catch { /* */ }
        }
        const { bootstrapEnterpriseGovernance } = await import('./bootstrapEnterpriseGovernance');
        await bootstrapEnterpriseGovernance();
        await invoke('enterprise_audit_log', {
            action: 'enterprise.session_start',
            detail: { org: policy.org_name || null, tier },
        }).catch(() => { });
    } catch { /* offline */ }
}
