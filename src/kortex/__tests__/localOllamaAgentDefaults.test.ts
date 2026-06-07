import { describe, it, expect, vi } from 'vitest';
import {
    applyLocalOllamaAgentDefaults,
    hybridPlannerAllowed,
    migrateLocalOllamaPlannerSettings,
} from '../../lib/localOllamaAgentDefaults';

describe('localOllamaAgentDefaults', () => {
    it('disables hybrid planner when applying local defaults', () => {
        const setPlannerEnabled = vi.fn();
        const setPlannerModel = vi.fn();
        applyLocalOllamaAgentDefaults({ setPlannerEnabled, setPlannerModel });
        expect(setPlannerEnabled).toHaveBeenCalledWith(false);
        expect(setPlannerModel).toHaveBeenCalledWith('');
    });

    it('blocks hybrid planner on local Ollama even if checkbox was on', () => {
        expect(hybridPlannerAllowed({ plannerEnabled: true, ollamaServerMode: 'local' })).toBe(false);
        expect(hybridPlannerAllowed({ plannerEnabled: true, ollamaServerMode: 'remote' })).toBe(true);
    });

    it('migrates stale hybrid-on-local profiles', () => {
        const setPlannerEnabled = vi.fn();
        const setPlannerModel = vi.fn();
        migrateLocalOllamaPlannerSettings({
            ollamaServerMode: 'local',
            plannerEnabled: true,
            setPlannerEnabled,
            setPlannerModel,
        });
        expect(setPlannerEnabled).toHaveBeenCalledWith(false);
        expect(setPlannerModel).toHaveBeenCalledWith('');
    });
});
