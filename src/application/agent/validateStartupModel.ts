import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

const CLOUD_PROVIDERS = new Set([
    'google', 'gemini', 'anthropic', 'openai', 'azure', 'bedrock', 'vertex',
    'COMMUNITYAI', 'mimo', 'deepseek', 'groq', 'mistral', 'cohere', 'xai', 'litellm',
    'openrouter', 'cerebras', 'highwayapi', 'interfaceai', 'jiekou', 'antigravity',
]);

const STALE_AGENT_MODELS = new Set([
    'antigravity|antigravity-sentient',
    'Antigravity|antigravity-sentient',
]);

const isHighwayApiModel = (model: unknown): boolean =>
    String(model || '').toLowerCase().includes('claude-opus-4-8');

/**
 * Use-case: fix stale localStorage model tags on boot.
 * WHY application layer? It's a business rule (cloud vs Ollama), not a UI concern.
 */
export async function validateStartupModel(): Promise<void> {
    try {
        const st = useStore.getState();
        const currentModel = st.agentModel || '';
        if (STALE_AGENT_MODELS.has(currentModel)) {
            st.setAgentModel?.('');
            try { localStorage.removeItem('agentModel'); } catch { /* */ }
            console.warn('[validateStartupModel] Cleared legacy antigravity-sentient model — pick a real model in the chat toolbar.');
            return;
        }
        const providerPrefix = currentModel.includes('|')
            ? currentModel.split('|')[0].toLowerCase()
            : '';
        const modelTag = currentModel.includes('|')
            ? currentModel.split('|').slice(1).join('|').trim()
            : currentModel.trim();

        if (modelTag && isHighwayApiModel(modelTag) && providerPrefix !== 'highwayapi') {
            const normalized = `highwayapi|${modelTag}`;
            st.setAgentModel?.(normalized);
            try { localStorage.setItem('agentModel', normalized); } catch { /* */ }
        }

        if (
            modelTag
            && st.inferenceBackend === 'ollama'
            && !CLOUD_PROVIDERS.has(providerPrefix)
            && !isHighwayApiModel(modelTag)
        ) {
            const { resolveOllamaModelTag } = await import('../../airi/shared-ollama');
            const resolved = await resolveOllamaModelTag(modelTag);
            if (resolved && resolved !== modelTag) {
                console.warn(`[validateStartupModel] "${modelTag}" → "${resolved}"`);
                st.setAgentModel?.(`Ollama|${resolved}`);
                try { localStorage.setItem('agentModel', `Ollama|${resolved}`); } catch { /* */ }
            }
        } else if (
            !modelTag
            && st.inferenceBackend === 'ollama'
            && st.ollamaServerMode === 'local'
        ) {
            try {
                const best = await invoke<string>('detect_best_model');
                if (best) {
                    const tag = `Ollama|${best}`;
                    st.setAgentModel?.(tag);
                    try { localStorage.setItem('agentModel', tag); } catch { /* */ }
                    console.log(`[validateStartupModel] offline auto-pick: ${best}`);
                }
            } catch { /* Ollama offline */ }
        }
    } catch (e) {
        console.warn('[validateStartupModel] skipped:', e);
    }
}
