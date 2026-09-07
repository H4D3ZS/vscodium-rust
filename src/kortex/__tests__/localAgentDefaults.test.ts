import { describe, it, expect, vi } from 'vitest';
import {
    applyLocalAgentDefaults,
    hybridPlannerAllowed,
    migrateLocalPlannerSettings,
} from '../../lib/localAgentDefaults';

describe('localAgentDefaults', () => {
    it('disables hybrid planner when applying local defaults', () => {
        const setPlannerEnabled = vi.fn();
        const setPlannerModel = vi.fn();
        applyLocalAgentDefaults({ setPlannerEnabled, setPlannerModel });
        expect(setPlannerEnabled).toHaveBeenCalledWith(false);
        expect(setPlannerModel).toHaveBeenCalledWith('');
    });

    it('blocks hybrid planner on a local backend even if checkbox was on', () => {
        expect(hybridPlannerAllowed({ plannerEnabled: true, inferenceServerMode: 'local' })).toBe(false);
        expect(hybridPlannerAllowed({ plannerEnabled: true, inferenceServerMode: 'remote' })).toBe(true);
    });

    it('migrates stale hybrid-on-local profiles', () => {
        const setPlannerEnabled = vi.fn();
        const setPlannerModel = vi.fn();
        migrateLocalPlannerSettings({
            inferenceServerMode: 'local',
            plannerEnabled: true,
            setPlannerEnabled,
            setPlannerModel,
        });
        expect(setPlannerEnabled).toHaveBeenCalledWith(false);
        expect(setPlannerModel).toHaveBeenCalledWith('');
    });
});
