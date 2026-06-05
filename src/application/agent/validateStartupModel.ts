import { useStore } from '../../store';

const CLOUD_PROVIDERS = new Set([
    'google', 'anthropic', 'openai', 'azure', 'bedrock', 'vertex',
    'cyberifrit', 'mimo', 'deepseek', 'groq', 'mistral', 'cohere', 'xai', 'litellm',
    'openrouter', 'cerebras', 'highwayapi', 'interfaceai', 'jiekou',
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
        }
    } catch (e) {
        console.warn('[validateStartupModel] skipped:', e);
    }
}
