import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

const BOOT_KEY = 'ide.enterprise-governance-v1';

/**
 * One-time + per-session enterprise control plane bootstrap:
 * policy seed, offline defaults, secure mode, audit.
 */
export async function bootstrapEnterpriseGovernance(): Promise<void> {
    const store = useStore.getState();
    const firstRun = typeof localStorage !== 'undefined' && !localStorage.getItem(BOOT_KEY);

    try {
        const policy = await invoke<{
            offline_only?: boolean;
            require_secure_mode?: boolean;
            block_private_network_scan?: boolean;
        }>('enterprise_seed_cyber_policy', { orgName: null });

        if (policy?.offline_only) {
            store.setOllamaServerMode?.('local');
            try { localStorage.setItem('inferenceBackend', 'lemonade'); } catch { /* */ }
        }
        if (policy?.require_secure_mode) {
            try {
                localStorage.setItem('agent.secureModeEnabled', '1');
                localStorage.setItem('agent.securityMode', 'secure');
                localStorage.setItem('agent.browserJsPolicy', 'always_ask');
                localStorage.setItem('agent.reviewPolicy', 'always_ask');
            } catch { /* */ }
        }
    } catch {
        /* offline — policy file still usable when seeded manually */
    }

    if (firstRun) {
        try { localStorage.setItem(BOOT_KEY, '1'); } catch { /* */ }
    }

    void invoke('enterprise_audit_log', {
        action: 'enterprise.ide_boot',
        detail: { first_run: firstRun },
    }).catch(() => { });
}

export async function initEngagement(
    engagementId: string,
    targets: string[],
): Promise<{ engagement_id?: string }> {
    return invoke('enterprise_init_engagement', {
        engagementId,
        targets,
    });
}
