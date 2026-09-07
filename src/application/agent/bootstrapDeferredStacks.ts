import { scheduleDeferredInit } from '../../memory_budget';
import { HEAVY_STACK_DEFER_MS } from '../../memory_budget';

let scheduled = false;

/** ANE, AIM index, enterprise policy, the local backend probes — never on critical path. */
export function scheduleDeferredHeavyStacks(): void {
    if (scheduled) return;
    scheduled = true;

    scheduleDeferredInit(async () => {
        const [{ bootstrapOfflineCyberStack }, { bootstrapEnterpriseGovernance }] = await Promise.all([
            import('./bootstrapOfflineCyberStack'),
            import('../enterprise/bootstrapEnterpriseGovernance'),
        ]);
        await bootstrapOfflineCyberStack({ heavy: true });
        await bootstrapEnterpriseGovernance();
    }, HEAVY_STACK_DEFER_MS);
}
