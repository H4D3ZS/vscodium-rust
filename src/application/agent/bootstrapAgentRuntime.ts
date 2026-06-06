import { useStore } from '../../store';
import { mapBackendChatMessages } from '../../store/agentSlice';
import { attachAgentStreamSubscriber, registerAgentKeyboardShortcuts } from '../../infrastructure/agent/AgentStreamSubscriber';
import { attachAgentToolStream } from '../../infrastructure/agent/attachAgentToolStream';
import { validateStartupModel } from './validateStartupModel';
import { tryActivateAiriCompanion } from './tryActivateAiriCompanion';
import { bootstrapHeavyFeaturesDefaults } from './bootstrapHeavyFeaturesDefaults';
import { invoke } from '../../tauri_bridge';

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
    attachAgentToolStream();
    await validateStartupModel();
    void tryActivateAiriCompanion();
    const root = useStore.getState().activeRoot;
    if (root) {
        import('../../infrastructure/workspace/workspaceProject').then(m => m.syncWorkspaceCompat(root));
    }
    // Restore workspace conversation from memory.aim (if any) instead of wiping on boot.
    try {
        const raw = await invoke<unknown>('get_agent_messages');
        const messages = mapBackendChatMessages(raw);
        useStore.getState().setAgentMessages(messages.length > 0 ? messages : []);
    } catch {
        useStore.getState().setAgentMessages([]);
    }
}
