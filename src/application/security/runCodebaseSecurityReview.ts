import type { SecurityAuditReport } from '../../domain/security/SecurityAuditReport';
import type { SecurityAuditOptions } from '../../domain/security/ISecurityAuditRepository';
import { securityAuditRepository } from '../../infrastructure/security/TauriSecurityAuditRepository';
import { useStore } from '../../store';

export type ReviewPhase = 'idle' | 'secrets' | 'patterns' | 'dependencies' | 'done' | 'error';

/**
 * On-demand comprehensive codebase security review (CodeRabbit-style, click-to-run).
 * NOT real-time — user triggers when they want a full audit.
 */
export async function runCodebaseSecurityReview(
    options: SecurityAuditOptions = {},
): Promise<SecurityAuditReport> {
    const store = useStore.getState();
    const root = store.activeRoot;
    if (!root) {
        throw new Error('Open a folder first — security review needs a workspace root.');
    }

    store.setSecurityReviewPhase('secrets');
    store.setSecurityReviewRunning(true);
    store.setSecurityReviewError(null);

    try {
        // UI phase ticks while Rust runs all passes in one call.
        const phaseTimer = window.setTimeout(() => {
            useStore.getState().setSecurityReviewPhase('patterns');
        }, 400);
        const phaseTimer2 = window.setTimeout(() => {
            useStore.getState().setSecurityReviewPhase('dependencies');
        }, 1200);

        const report = await securityAuditRepository.runDeepAudit({
            path: options.path ?? '.',
            depth: options.depth ?? 'deep',
            writeReport: true,
            maxFindings: options.maxFindings ?? 500,
        });

        window.clearTimeout(phaseTimer);
        window.clearTimeout(phaseTimer2);

        store.setSecurityReviewReport(report);
        store.setSecurityReviewPhase('done');
        return report;
    } catch (e: unknown) {
        const msg = e instanceof Error ? e.message : String(e);
        store.setSecurityReviewError(msg);
        store.setSecurityReviewPhase('error');
        throw e;
    } finally {
        store.setSecurityReviewRunning(false);
    }
}

export type SecurityPanelTab = 'overview' | 'review' | 'vega' | 'chunks' | 'proxy' | 'arsenal' | 'modules';

export function openSecurityReviewPanel(tab: SecurityPanelTab = 'overview'): void {
    useStore.getState().setActiveSidebarView('security-view');
    useStore.getState().setSecurityReviewPanelOpen(true);
    window.dispatchEvent(new CustomEvent('hades:security-tab', { detail: { tab } }));
}
