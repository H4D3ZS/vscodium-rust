/**
 * Local the local backend rigs (4b–14b on a desk/laptop) should use a single fast executor.
 * Hybrid planner (iter-0 on a second / heavier model) is for remote GPU or cloud stacks.
 */

import { isHeavyLocalModel } from '../model_capabilities';

type AgentDefaultsStore = {
    setPlannerEnabled?: (v: boolean) => void;
    setPlannerModel?: (v: string) => void;
    setHybridAuto?: (v: boolean) => void;
    inferenceServerMode?: string;
    plannerEnabled?: boolean;
    plannerModel?: string;
};

export function isLocalInferenceServerMode(mode: string | undefined): boolean {
    return String(mode || '').toLowerCase() === 'local';
}

/** Hybrid planner is only meaningful on remote/cloud GPU — not localhost the local backend. */
export function hybridPlannerAllowed(state: {
    plannerEnabled?: boolean;
    inferenceServerMode?: string;
}): boolean {
    if (!state.plannerEnabled) return false;
    if (isLocalInferenceServerMode(state.inferenceServerMode)) return false;
    return true;
}

/** Apply when user picks Local the local backend in Settings — single-model agent, no 40B planner pass. */
export function applyLocalAgentDefaults(store: AgentDefaultsStore): void {
    store.setPlannerEnabled?.(false);
    store.setPlannerModel?.('');
    try {
        localStorage.setItem('agent.plannerEnabled', '0');
        localStorage.removeItem('agent.plannerModel');
    } catch { /* ignore */ }
}

/** One-time cleanup for profiles that had hybrid enabled while on local the local backend. */
export function migrateLocalPlannerSettings(store: AgentDefaultsStore): void {
    if (!isLocalInferenceServerMode(store.inferenceServerMode)) return;
    const storedOn = (() => {
        try { return localStorage.getItem('agent.plannerEnabled') === '1'; } catch { return false; }
    })();
    const storedPlanner = (() => {
        try { return localStorage.getItem('agent.plannerModel') || ''; } catch { return ''; }
    })();
    const plannerId = storedPlanner.includes('|')
        ? storedPlanner.split('|').slice(1).join('|')
        : storedPlanner;
    if (storedOn || store.plannerEnabled || isHeavyLocalModel(plannerId)) {
        applyLocalAgentDefaults(store);
    }
}
