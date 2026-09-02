/**
 * Use-case: opt-in AIRI sentient companion (default OFF — saves RAM).
 */
export async function tryActivateAiriCompanion(): Promise<void> {
    if (typeof localStorage === 'undefined' || localStorage.getItem('airi.companion') !== '1') {
        console.log('[Agent] AIRI companion disabled (localStorage airi.companion=1 to enable)');
        return;
    }
    try {
        const { activateAIRIAgent } = await import('../../airi_agent_bridge');
        console.log('[Agent] Activating AIRI Sentient Core…');
        await activateAIRIAgent({
            fullAutonomy: true,
            selfLearning: true,
            biology: true,
            consciousness: true,
            voice: false,
        });
        console.log('[Agent] AIRI Sentient Core active');
    } catch (err) {
        console.error('[Agent] AIRI activation failed:', err);
    }
}
