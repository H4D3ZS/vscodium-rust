/**
 * Local Ollama rigs (4b–14b on a desk/laptop) should use a single fast executor.
 * Hybrid planner (iter-0 on a second / heavier model) is for remote GPU or cloud stacks.
 */

import { isHeavyLocalModel } from '../model_capabilities';

type AgentDefaultsStore = {
    setPlannerEnabled?: (v: boolean) => void;
    setPlannerModel?: (v: string) => void;
    setHybridAuto?: (v: boolean) => void;
    ollamaServerMode?: string;
    plannerEnabled?: boolean;
    plannerModel?: string;
};

export function isLocalOllamaServerMode(mode: string | undefined): boolean {
    return String(mode || '').toLowerCase() === 'local';
}

/** Hybrid planner is only meaningful on remote/cloud GPU — not localhost Ollama. */
export function hybridPlannerAllowed(state: {
    plannerEnabled?: boolean;
    ollamaServerMode?: string;
}): boolean {
    if (!state.plannerEnabled) return false;
    if (isLocalOllamaServerMode(state.ollamaServerMode)) return false;
    return true;
}

/** Apply when user picks Local Ollama in Settings — single-model agent, no 40B planner pass. */
export function applyLocalOllamaAgentDefaults(store: AgentDefaultsStore): void {
    store.setPlannerEnabled?.(false);
    store.setPlannerModel?.('');
    try {
        localStorage.setItem('agent.plannerEnabled', '0');
        localStorage.removeItem('agent.plannerModel');
    } catch { /* ignore */ }
}

/** One-time cleanup for profiles that had hybrid enabled while on local Ollama. */
export function migrateLocalOllamaPlannerSettings(store: AgentDefaultsStore): void {
    if (!isLocalOllamaServerMode(store.ollamaServerMode)) return;
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
        applyLocalOllamaAgentDefaults(store);
    }
}
