import { useStore } from '../../store';
import { attachAgentStreamSubscriber, registerAgentKeyboardShortcuts } from '../../infrastructure/agent/AgentStreamSubscriber';
import { validateStartupModel } from './validateStartupModel';
import { tryActivateAiriCompanion } from './tryActivateAiriCompanion';
import { bootstrapHeavyFeaturesDefaults } from './bootstrapHeavyFeaturesDefaults';

/**
 * Use-case: boot the agent event spine once per app session.
 *
 * WHY defer AIRI + legacy engine?
 * Chat listeners are lightweight; sentient-core + tool_registry load only when
 * companion mode is on or the user sends a message.
 */
export async function bootstrapAgentRuntime(): Promise<void> {
    bootstrapHeavyFeaturesDefaults();
    console.log('[bootstrapAgentRuntime] attaching stream subscriber');
    registerAgentKeyboardShortcuts();
    await attachAgentStreamSubscriber();
    await validateStartupModel();
    void tryActivateAiriCompanion();
    // Fresh chat surface on cold boot — persisted sessions load via History tab.
    useStore.getState().setAgentMessages([]);
}
