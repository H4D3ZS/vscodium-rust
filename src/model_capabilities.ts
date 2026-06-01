// Model capabilities database — ported from Void editor (Glass Devtools)
// + Antigravity IDE extensions. Used by agent routing, UI display, and
// reasoning slider logic.

export type ProviderName =
    | 'anthropic' | 'openAI' | 'deepseek' | 'ollama' | 'vLLM' | 'openRouter'
    | 'gemini' | 'groq' | 'xAI' | 'mistral' | 'lmStudio' | 'liteLLM'
    | 'openAICompatible' | 'googleVertex' | 'microsoftAzure' | 'awsBedrock'
    | 'antigravity' | 'mimo' | 'cyberifrit';

export type FeatureName = 'Chat' | 'Apply' | 'Autocomplete' | 'QuickEdit' | 'SCM';

export interface ModelCapabilities {
    contextWindow: number;
    reservedOutputTokenSpace: number | null;
    supportsSystemMessage: false | 'system-role' | 'developer-role' | 'separated';
    specialToolFormat?: 'openai-style' | 'anthropic-style' | 'gemini-style';
    supportsFIM: boolean;
    reasoningCapabilities: false | {
        supportsReasoning: true;
        canTurnOffReasoning: boolean;
        canIOReasoning: boolean;
        reasoningReservedOutputTokenSpace?: number;
        reasoningSlider?:
            | { type: 'budget_slider'; min: number; max: number; default: number }
            | { type: 'effort_slider'; values: string[]; default: string };
        openSourceThinkTags?: [string, string];
    };
    cost: { input: number; output: number; cache_read?: number; cache_write?: number };
    downloadable: false | { sizeGb: number | 'not-known' };
}

export interface ModelSelection {
    providerName: ProviderName;
    modelName: string;
}

export interface ModelSelectionOptions {
    reasoningEnabled?: boolean;
    reasoningBudget?: number;
    reasoningEffort?: string;
}

export type ModelSelectionOfFeature = Partial<Record<FeatureName, ModelSelection | null>>;

export interface GlobalSettings {
    autoRefreshModels: boolean;
    aiInstructions: string;
    enableAutocomplete: boolean;
    syncApplyToChat: boolean;
    enableFastApply: boolean;
    chatMode: 'agent' | 'gather' | 'normal';
    autoApprove: Partial<Record<string, boolean>>;
    showInlineSuggestions: boolean;
    includeToolLintErrors: boolean;
    disableSystemMessage: boolean;
    autoAcceptLLMChanges: boolean;
    enableWorkflowEditor: boolean;
    enableRuleEditor: boolean;
    persistentLanguageServer: boolean;
}

export const defaultGlobalSettings: GlobalSettings = {
    autoRefreshModels: true,
    aiInstructions: '',
    enableAutocomplete: false,
    syncApplyToChat: true,
    enableFastApply: true,
    chatMode: 'agent',
    autoApprove: {},
    showInlineSuggestions: true,
    includeToolLintErrors: true,
    disableSystemMessage: false,
    autoAcceptLLMChanges: false,
    enableWorkflowEditor: true,
    enableRuleEditor: true,
    persistentLanguageServer: false,
};

export const defaultProviderEndpoints: Partial<Record<ProviderName, string>> = {
    ollama: 'http://127.0.0.1:11434',
    vLLM: 'http://localhost:8000',
    lmStudio: 'http://localhost:1234',
    antigravity: 'http://127.0.0.1:1536',
    mimo: 'https://api.xiaomimimo.com/v1',
    cyberifrit: 'https://api.cyberifrit.xyz',
};

export const localProviders: ProviderName[] = ['ollama', 'vLLM', 'lmStudio', 'antigravity'];

const defaultCaps: ModelCapabilities = {
    contextWindow: 4_096,
    reservedOutputTokenSpace: 4_096,
    supportsSystemMessage: false,
    supportsFIM: false,
    reasoningCapabilities: false,
    cost: { input: 0, output: 0 },
    downloadable: false,
};

// ── Open-source model patterns (shared across Ollama / vLLM / etc.) ──────────

const openSourceModels: Record<string, Partial<ModelCapabilities>> = {
    'deepseek-r1': {
        supportsSystemMessage: false,
        supportsFIM: false,
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] },
        contextWindow: 32_000, reservedOutputTokenSpace: 4_096,
    },
    'deepseek-v3': {
        supportsSystemMessage: false,
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 32_000, reservedOutputTokenSpace: 4_096,
    },
    'codestral': {
        supportsFIM: true,
        supportsSystemMessage: 'system-role',
        reasoningCapabilities: false,
        contextWindow: 256_000, reservedOutputTokenSpace: 4_096,
    },
    'devstral': {
        supportsFIM: false,
        supportsSystemMessage: 'system-role',
        reasoningCapabilities: false,
        contextWindow: 131_000, reservedOutputTokenSpace: 8_192,
    },
    'phi4': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] },
        contextWindow: 16_000, reservedOutputTokenSpace: 4_096,
    },
    'gemma': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 32_000, reservedOutputTokenSpace: 4_096,
    },
    'llama3': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 32_000, reservedOutputTokenSpace: 4_096,
    },
    'llama3.1': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 4_096,
    },
    'llama3.2': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 4_096,
    },
    'llama3.3': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 4_096,
    },
    'llama4-scout': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 10_000_000, reservedOutputTokenSpace: 4_096,
    },
    'qwen2.5-coder': {
        supportsFIM: true,
        supportsSystemMessage: 'system-role',
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 4_096,
    },
    'qwq': {
        supportsFIM: false,
        supportsSystemMessage: 'system-role',
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] },
        contextWindow: 128_000, reservedOutputTokenSpace: 8_192,
    },
    'qwen3': {
        supportsFIM: false,
        supportsSystemMessage: 'system-role',
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] },
        contextWindow: 32_768, reservedOutputTokenSpace: 8_192,
    },
    'starcoder2': {
        supportsFIM: true,
        supportsSystemMessage: false,
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 8_192,
    },
    'openhands': {
        supportsSystemMessage: 'system-role',
        supportsFIM: false,
        reasoningCapabilities: false,
        contextWindow: 128_000, reservedOutputTokenSpace: 4_096,
    },
};

// ── Per-provider model databases ──────────────────────────────────────────────

const anthropicModels: Record<string, ModelCapabilities> = {
    'claude-opus-4-0': {
        contextWindow: 200_000, reservedOutputTokenSpace: 8_192,
        cost: { input: 15.00, cache_read: 1.50, cache_write: 18.75, output: 75.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, reasoningReservedOutputTokenSpace: 8192, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } },
    },
    'claude-sonnet-4-0': {
        contextWindow: 200_000, reservedOutputTokenSpace: 8_192,
        cost: { input: 3.00, cache_read: 0.30, cache_write: 3.75, output: 6.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, reasoningReservedOutputTokenSpace: 8192, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } },
    },
    'claude-3-7-sonnet-latest': {
        contextWindow: 200_000, reservedOutputTokenSpace: 8_192,
        cost: { input: 3.00, cache_read: 0.30, cache_write: 3.75, output: 15.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, reasoningReservedOutputTokenSpace: 8192, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } },
    },
    'claude-3-5-sonnet-latest': {
        contextWindow: 200_000, reservedOutputTokenSpace: 8_192,
        cost: { input: 3.00, cache_read: 0.30, cache_write: 3.75, output: 15.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: false,
    },
    'claude-3-5-haiku-latest': {
        contextWindow: 200_000, reservedOutputTokenSpace: 8_192,
        cost: { input: 0.80, cache_read: 0.08, cache_write: 1.00, output: 4.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: false,
    },
    'claude-3-opus-latest': {
        contextWindow: 200_000, reservedOutputTokenSpace: 4_096,
        cost: { input: 15.00, cache_read: 1.50, cache_write: 18.75, output: 75.00 },
        downloadable: false, supportsFIM: false,
        specialToolFormat: 'anthropic-style', supportsSystemMessage: 'separated',
        reasoningCapabilities: false,
    },
};

const openAIModels: Record<string, ModelCapabilities> = {
    'gpt-4.1': { contextWindow: 1_047_576, reservedOutputTokenSpace: 32_768, cost: { input: 2.00, output: 8.00, cache_read: 0.50 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'developer-role', reasoningCapabilities: false },
    'gpt-4.1-mini': { contextWindow: 1_047_576, reservedOutputTokenSpace: 32_768, cost: { input: 0.40, output: 1.60, cache_read: 0.10 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'developer-role', reasoningCapabilities: false },
    'gpt-4.1-nano': { contextWindow: 1_047_576, reservedOutputTokenSpace: 32_768, cost: { input: 0.10, output: 0.40, cache_read: 0.03 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'developer-role', reasoningCapabilities: false },
    'o3': { contextWindow: 1_047_576, reservedOutputTokenSpace: 32_768, cost: { input: 10.00, output: 40.00, cache_read: 2.50 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'developer-role', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: false, reasoningSlider: { type: 'effort_slider', values: ['low', 'medium', 'high'], default: 'low' } } },
    'o4-mini': { contextWindow: 1_047_576, reservedOutputTokenSpace: 32_768, cost: { input: 1.10, output: 4.40, cache_read: 0.275 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'developer-role', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: false, reasoningSlider: { type: 'effort_slider', values: ['low', 'medium', 'high'], default: 'low' } } },
    'gpt-4o': { contextWindow: 128_000, reservedOutputTokenSpace: 16_384, cost: { input: 2.50, cache_read: 1.25, output: 10.00 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'gpt-4o-mini': { contextWindow: 128_000, reservedOutputTokenSpace: 16_384, cost: { input: 0.15, cache_read: 0.075, output: 0.60 }, downloadable: false, supportsFIM: false, specialToolFormat: 'openai-style', supportsSystemMessage: 'system-role', reasoningCapabilities: false },
};

const geminiModels: Record<string, ModelCapabilities> = {
    'gemini-2.5-pro-preview-05-06': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0, output: 0 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: false, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } } },
    'gemini-2.5-flash-preview-04-17': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0.15, output: 0.60 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: false, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } } },
    'gemini-2.5-pro-exp-03-25': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0, output: 0 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: false, reasoningSlider: { type: 'budget_slider', min: 1024, max: 8192, default: 1024 } } },
    'gemini-2.0-flash': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0.10, output: 0.40 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: false },
    'gemini-2.0-flash-lite': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0, output: 0 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: false },
    'gemini-1.5-pro': { contextWindow: 2_097_152, reservedOutputTokenSpace: 8_192, cost: { input: 1.25, output: 5.00 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: false },
    'gemini-1.5-flash': { contextWindow: 1_048_576, reservedOutputTokenSpace: 8_192, cost: { input: 0.075, output: 0.30 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'separated', specialToolFormat: 'gemini-style', reasoningCapabilities: false },
};

const xAIModels: Record<string, ModelCapabilities> = {
    'grok-3': { contextWindow: 131_072, reservedOutputTokenSpace: null, cost: { input: 3.00, output: 15.00 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: false },
    'grok-3-mini': { contextWindow: 131_072, reservedOutputTokenSpace: null, cost: { input: 0.30, output: 0.50 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: false, reasoningSlider: { type: 'effort_slider', values: ['low', 'high'], default: 'low' } } },
    'grok-3-fast': { contextWindow: 131_072, reservedOutputTokenSpace: null, cost: { input: 5.00, output: 25.00 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: false },
    'grok-2': { contextWindow: 131_072, reservedOutputTokenSpace: null, cost: { input: 2.00, output: 10.00 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: false },
};

const deepseekModels: Record<string, ModelCapabilities> = {
    'deepseek-chat': { contextWindow: 64_000, reservedOutputTokenSpace: 8_000, cost: { cache_read: .07, input: .27, output: 1.10 }, downloadable: false, supportsFIM: false, supportsSystemMessage: false, reasoningCapabilities: false },
    'deepseek-reasoner': { contextWindow: 64_000, reservedOutputTokenSpace: 8_000, cost: { cache_read: .14, input: .55, output: 2.19 }, downloadable: false, supportsFIM: false, supportsSystemMessage: false, reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: false, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] } },
    // DeepSeek-V4-Pro — permanent 75%-off pricing, automatic server-side context
    // caching (cache-hit input is ~120x cheaper). Ideal resell/rebrand executor.
    'deepseek-v4-pro': { contextWindow: 128_000, reservedOutputTokenSpace: 8_192, cost: { cache_read: 0.003625, input: 0.435, output: 0.87 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] } },
};

// Xiaomi MiMo — OpenAI/Anthropic-compatible coding models (Token Plan subscription
// or metered). v2.5-pro is the current flagship; v2/Omni auto-route to v2.5.
const mimoModels: Record<string, ModelCapabilities> = {
    'mimo-v2.5-pro': { contextWindow: 256_000, reservedOutputTokenSpace: 16_384, cost: { input: 0.3, output: 1.2 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] } },
    'mimo-v2.5': { contextWindow: 256_000, reservedOutputTokenSpace: 16_384, cost: { input: 0.15, output: 0.6 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', specialToolFormat: 'openai-style', reasoningCapabilities: { supportsReasoning: true, canTurnOffReasoning: true, canIOReasoning: true, openSourceThinkTags: ['<think>', '</think>'] } },
};

const mistralModels: Record<string, ModelCapabilities> = {
    'codestral-latest': { contextWindow: 256_000, reservedOutputTokenSpace: 8_192, cost: { input: 0.30, output: 0.90 }, supportsFIM: true, downloadable: { sizeGb: 13 }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'devstral-small-latest': { contextWindow: 131_000, reservedOutputTokenSpace: 8_192, cost: { input: 0, output: 0 }, supportsFIM: false, downloadable: { sizeGb: 14 }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'mistral-large-latest': { contextWindow: 131_000, reservedOutputTokenSpace: 8_192, cost: { input: 2.00, output: 6.00 }, supportsFIM: false, downloadable: { sizeGb: 73 }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'mistral-medium-latest': { contextWindow: 131_000, reservedOutputTokenSpace: 8_192, cost: { input: 0.40, output: 2.00 }, supportsFIM: false, downloadable: { sizeGb: 'not-known' }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'ministral-8b-latest': { contextWindow: 131_000, reservedOutputTokenSpace: 4_096, cost: { input: 0.10, output: 0.10 }, supportsFIM: false, downloadable: { sizeGb: 4.1 }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'ministral-3b-latest': { contextWindow: 131_000, reservedOutputTokenSpace: 4_096, cost: { input: 0.04, output: 0.04 }, supportsFIM: false, downloadable: { sizeGb: 'not-known' }, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
};

const groqModels: Record<string, ModelCapabilities> = {
    'llama-3.3-70b-versatile': { contextWindow: 128_000, reservedOutputTokenSpace: 32_768, cost: { input: 0.59, output: 0.79 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'llama-3.1-8b-instant': { contextWindow: 128_000, reservedOutputTokenSpace: 8_192, cost: { input: 0.05, output: 0.08 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'qwen-qwq-32b': { contextWindow: 128_000, reservedOutputTokenSpace: null, cost: { input: 0.29, output: 0.39 }, downloadable: false, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: { supportsReasoning: true, canIOReasoning: true, canTurnOffReasoning: false, openSourceThinkTags: ['<think>', '</think>'] } },
};

const ollamaModels: Record<string, ModelCapabilities> = {
    'qwen2.5-coder:7b': { contextWindow: 32_000, reservedOutputTokenSpace: null, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 1.9 }, supportsFIM: true, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'qwen2.5-coder:3b': { contextWindow: 32_000, reservedOutputTokenSpace: null, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 1.9 }, supportsFIM: true, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'llama3.1': { contextWindow: 128_000, reservedOutputTokenSpace: null, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 4.9 }, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
    'qwq': { contextWindow: 128_000, reservedOutputTokenSpace: 32_000, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 20 }, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: { supportsReasoning: true, canIOReasoning: false, canTurnOffReasoning: false, openSourceThinkTags: ['<think>', '</think>'] } },
    'deepseek-r1': { contextWindow: 128_000, reservedOutputTokenSpace: null, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 4.7 }, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: { supportsReasoning: true, canIOReasoning: false, canTurnOffReasoning: false, openSourceThinkTags: ['<think>', '</think>'] } },
    'devstral:latest': { contextWindow: 131_000, reservedOutputTokenSpace: 8_192, cost: { input: 0, output: 0 }, downloadable: { sizeGb: 14 }, supportsFIM: false, supportsSystemMessage: 'system-role', reasoningCapabilities: false },
};

export const ollamaRecommendedModels = ['qwen2.5-coder:7b', 'llama3.1', 'qwq', 'deepseek-r1', 'devstral:latest'];

const providerModelDbs: Partial<Record<ProviderName, Record<string, ModelCapabilities>>> = {
    anthropic: anthropicModels,
    openAI: openAIModels,
    gemini: geminiModels,
    xAI: xAIModels,
    deepseek: deepseekModels,
    mimo: mimoModels,
    mistral: mistralModels,
    groq: groqModels,
    ollama: ollamaModels,
    // Cyber-Ifrit hosts arbitrary named models (your AMD backend), so there's no
    // fixed catalog — names resolve via fuzzyLookup / defaultCaps at runtime.
    cyberifrit: {},
};

// ── Fuzzy fallback lookup ─────────────────────────────────────────────────────

function fuzzyLookup(modelName: string): Partial<ModelCapabilities> | null {
    const m = modelName.toLowerCase();

    if (m.includes('claude')) {
        const base = anthropicModels['claude-3-5-sonnet-latest'];
        if (m.includes('opus')) return { ...anthropicModels['claude-opus-4-0'] };
        if (m.includes('3-7') || m.includes('3.7')) return { ...anthropicModels['claude-3-7-sonnet-latest'] };
        if (m.includes('haiku')) return { ...anthropicModels['claude-3-5-haiku-latest'] };
        return { ...base };
    }
    if (m.includes('gpt-4.1') && m.includes('mini')) return { ...openAIModels['gpt-4.1-mini'] };
    if (m.includes('gpt-4.1') && m.includes('nano')) return { ...openAIModels['gpt-4.1-nano'] };
    if (m.includes('gpt-4.1')) return { ...openAIModels['gpt-4.1'] };
    if (m.includes('gpt-4o') && m.includes('mini')) return { ...openAIModels['gpt-4o-mini'] };
    if (m.includes('gpt-4o')) return { ...openAIModels['gpt-4o'] };
    if (m.includes('o4-mini')) return { ...openAIModels['o4-mini'] };
    if (m.includes('o3')) return { ...openAIModels['o3'] };

    if (m.includes('gemini-2.5-pro')) return { ...geminiModels['gemini-2.5-pro-preview-05-06'] };
    if (m.includes('gemini-2.5-flash')) return { ...geminiModels['gemini-2.5-flash-preview-04-17'] };
    if (m.includes('gemini-2.0-flash')) return { ...geminiModels['gemini-2.0-flash'] };
    if (m.includes('gemini')) return { ...geminiModels['gemini-1.5-flash'] };

    if (m.includes('grok-3-mini')) return { ...xAIModels['grok-3-mini'] };
    if (m.includes('grok-3-fast')) return { ...xAIModels['grok-3-fast'] };
    if (m.includes('grok-3') || m.includes('grok3')) return { ...xAIModels['grok-3'] };
    if (m.includes('grok')) return { ...xAIModels['grok-2'] };

    if (m.includes('deepseek-v4') || m.includes('v4-pro')) return { ...deepseekModels['deepseek-v4-pro'] };
    if (m.includes('deepseek-reasoner') || m.includes('deepseek-r1')) return { ...deepseekModels['deepseek-reasoner'] };
    if (m.includes('deepseek')) return { ...deepseekModels['deepseek-chat'] };

    if (m.includes('codestral')) return openSourceModels.codestral as ModelCapabilities;
    if (m.includes('devstral')) return openSourceModels.devstral as ModelCapabilities;
    if (m.includes('phi4') || m.includes('phi-4')) return openSourceModels.phi4 as ModelCapabilities;
    if (m.includes('gemma')) return openSourceModels.gemma as ModelCapabilities;
    if (m.includes('llama3.3')) return openSourceModels['llama3.3'] as ModelCapabilities;
    if (m.includes('llama3.2')) return openSourceModels['llama3.2'] as ModelCapabilities;
    if (m.includes('llama3.1')) return openSourceModels['llama3.1'] as ModelCapabilities;
    if (m.includes('llama3') || m.includes('llama-3')) return openSourceModels.llama3 as ModelCapabilities;
    if (m.includes('llama') && (m.includes('scout') || m.includes('maverick'))) return openSourceModels['llama4-scout'] as ModelCapabilities;
    if (m.includes('llama')) return openSourceModels.llama3 as ModelCapabilities;

    if (m.includes('qwen3') || m.includes('qwen-3')) return openSourceModels.qwen3 as ModelCapabilities;
    if (m.includes('qwq')) return openSourceModels.qwq as ModelCapabilities;
    if (m.includes('qwen2.5') && m.includes('coder')) return openSourceModels['qwen2.5-coder'] as ModelCapabilities;
    if (m.includes('qwen')) return openSourceModels.qwen3 as ModelCapabilities;

    if (m.includes('starcoder2')) return openSourceModels.starcoder2 as ModelCapabilities;
    if (m.includes('openhands')) return openSourceModels.openhands as ModelCapabilities;

    if (m.includes('mistral-large')) return { ...mistralModels['mistral-large-latest'] };
    if (m.includes('mistral-medium')) return { ...mistralModels['mistral-medium-latest'] };
    if (m.includes('ministral-8b') || m.includes('ministral8b')) return { ...mistralModels['ministral-8b-latest'] };
    if (m.includes('ministral-3b') || m.includes('ministral3b')) return { ...mistralModels['ministral-3b-latest'] };
    if (m.includes('mistral')) return { ...mistralModels['ministral-8b-latest'] };

    return null;
}

// ── Public API ────────────────────────────────────────────────────────────────

export function getModelCapabilities(
    providerName: ProviderName,
    modelName: string
): ModelCapabilities & { isUnrecognized: boolean } {
    const db = providerModelDbs[providerName];
    if (db) {
        const direct = db[modelName] ?? db[modelName.toLowerCase()];
        if (direct) return { ...direct, isUnrecognized: false };
    }
    const fuzzy = fuzzyLookup(modelName);
    if (fuzzy) return { ...defaultCaps, ...fuzzy, isUnrecognized: false } as ModelCapabilities & { isUnrecognized: boolean };
    return { ...defaultCaps, isUnrecognized: true };
}

export function modelSupportsReasoning(providerName: ProviderName, modelName: string): boolean {
    const caps = getModelCapabilities(providerName, modelName);
    return !!(caps.reasoningCapabilities && caps.reasoningCapabilities.supportsReasoning);
}

export function modelSupportsFIM(providerName: ProviderName, modelName: string): boolean {
    return getModelCapabilities(providerName, modelName).supportsFIM;
}

export function getReasoningSlider(providerName: ProviderName, modelName: string) {
    const caps = getModelCapabilities(providerName, modelName);
    if (!caps.reasoningCapabilities || !caps.reasoningCapabilities.supportsReasoning) return null;
    return caps.reasoningCapabilities.reasoningSlider ?? null;
}

export function getThinkTags(providerName: ProviderName, modelName: string): [string, string] | null {
    const caps = getModelCapabilities(providerName, modelName);
    if (!caps.reasoningCapabilities || !caps.reasoningCapabilities.supportsReasoning) return null;
    return caps.reasoningCapabilities.openSourceThinkTags ?? null;
}

export function getContextWindow(providerName: ProviderName, modelName: string): number {
    return getModelCapabilities(providerName, modelName).contextWindow;
}

// ── Default model selections per feature ─────────────────────────────────────

export const defaultModelSelectionOfFeature: ModelSelectionOfFeature = {
    Chat: { providerName: 'gemini', modelName: 'gemini-2.5-pro' },
    Apply: { providerName: 'anthropic', modelName: 'claude-3-5-sonnet-latest' },
    Autocomplete: { providerName: 'ollama', modelName: 'qwen2.5-coder:7b' },
    QuickEdit: { providerName: 'gemini', modelName: 'gemini-2.5-flash' },
    SCM: { providerName: 'openAI', modelName: 'gpt-4.1-mini' },
};

export const featureDisplayNames: Record<FeatureName, string> = {
    Chat: 'Chat',
    Apply: 'Apply (Fast Apply)',
    Autocomplete: 'Autocomplete (FIM)',
    QuickEdit: 'Quick Edit (Ctrl+K)',
    SCM: 'Commit Message Generator',
};

// ── Provider display info ─────────────────────────────────────────────────────

export const providerDisplayInfo: Record<ProviderName, { title: string; icon?: string }> = {
    anthropic: { title: 'Anthropic' },
    openAI: { title: 'OpenAI' },
    deepseek: { title: 'DeepSeek' },
    ollama: { title: 'Ollama (Local)' },
    vLLM: { title: 'vLLM (Local)' },
    openRouter: { title: 'OpenRouter' },
    gemini: { title: 'Gemini' },
    groq: { title: 'Groq' },
    xAI: { title: 'Grok (xAI)' },
    mistral: { title: 'Mistral' },
    lmStudio: { title: 'LM Studio (Local)' },
    liteLLM: { title: 'LiteLLM' },
    openAICompatible: { title: 'OpenAI-Compatible' },
    googleVertex: { title: 'Google Vertex AI' },
    microsoftAzure: { title: 'Microsoft Azure OpenAI' },
    awsBedrock: { title: 'AWS Bedrock' },
    antigravity: { title: 'Antigravity / AIM Proxy' },
    mimo: { title: 'Xiaomi MiMo' },
    cyberifrit: { title: 'Cyber-Ifrit Cloud' },
};

// ── Fast Apply search/replace block parser (from Void) ────────────────────────
// Format:
//   <<<<<<< ORIGINAL
//   ... old code ...
//   =======
//   ... new code ...
//   >>>>>>> UPDATED

export interface SearchReplaceBlock {
    original: string;
    updated: string;
}

export function extractSearchReplaceBlocks(text: string): SearchReplaceBlock[] {
    const blocks: SearchReplaceBlock[] = [];
    const pattern = /<<<<<<< ORIGINAL\n([\s\S]*?)\n?=======\n([\s\S]*?)\n?>>>>>>> UPDATED/g;
    let match: RegExpExecArray | null;
    while ((match = pattern.exec(text)) !== null) {
        blocks.push({ original: match[1], updated: match[2] });
    }
    return blocks;
}

// ── Hybrid planner/executor auto-detection ────────────────────────────────────
// Used by the agent's hybrid pipeline: the strongest available model PLANS
// (deep reasoning), the fastest capable model EXECUTES (acts/edits). Input is
// `store.availableModels`, which only contains models for providers that have a
// key/install — so a pick is always "actually reachable".

export interface AvailableModelLite { id: string; provider: string }
export interface PlannerExecutorPick {
    planner: AvailableModelLite | null;
    executor: AvailableModelLite | null;
}

/** `provider|id` key, lowercased provider — matches the format set_advisor_model expects. */
export function modelKey(m: AvailableModelLite): string {
    return `${String(m.provider || '').toLowerCase()}|${m.id}`;
}

const BIG_LOCAL = /(?::|-)(?:30b|32b|34b|65b|70b|72b|110b)\b/;
const SMALL_LOCAL = /(?::|-)(?:0\.5b|1\.5b|1b|2b|3b|4b|7b|8b|9b|13b|14b)\b/;

/** Higher = stronger reasoner. Frontier cloud > large local > mid local. */
function plannerScore(m: AvailableModelLite): number {
    const id = String(m.id || '').toLowerCase();
    if (id.includes('opus')) return 100;
    if (id.includes('o3')) return 90;
    if (id.includes('claude') && id.includes('sonnet')) return 89;
    if (id.includes('gemini') && id.includes('2.5') && id.includes('pro')) return 88;
    if (id.includes('gemini') && id.includes('pro')) return 84;
    if (id.includes('o1')) return 86;
    if (id.includes('gpt-4.1') && !id.includes('mini') && !id.includes('nano')) return 82;
    if (id.includes('deepseek') && (id.includes('r1') || id.includes('reason'))) return 80;
    if (id.includes('gpt-4o') && !id.includes('mini')) return 78;
    if (id.includes('grok-3') && !id.includes('mini')) return 74;
    if (id.includes('claude')) return 70;
    if (id.includes('qwq')) return BIG_LOCAL.test(id) ? 68 : 58;
    if (id.includes('gemini')) return 60;
    if (BIG_LOCAL.test(id)) return 56;
    if (id.includes('qwen') || id.includes('llama') || id.includes('mistral') || id.includes('devstral')) return 40;
    return 20;
}

/** Higher = better fast executor. Coding-specialized + small/fast wins. */
function executorScore(m: AvailableModelLite): number {
    const id = String(m.id || '').toLowerCase();
    if (id.includes('coder')) return 100;
    if (id.includes('codestral') || id.includes('starcoder')) return 95;
    if (id.includes('devstral')) return 88;
    if (id.includes('haiku')) return 85;
    if (id.includes('flash')) return 84;
    if (id.includes('mini') || id.includes('nano') || id.includes('instant')) return 80;
    if (SMALL_LOCAL.test(id)) return 70;
    if (id.includes('sonnet')) return 60;
    if (id.includes('qwen') || id.includes('llama') || id.includes('mistral')) return 55;
    if (id.includes('gemini') && id.includes('pro')) return 45;
    if (id.includes('opus') || id.includes('o3') || id.includes('o1')) return 35; // capable but slow/costly
    return 30;
}

/**
 * Pick a (planner, executor) pair from the available models. The executor is the
 * best fast model that differs from the planner; if only one model exists, both
 * roles collapse to it (degrades to today's single-model behavior).
 */
export function classifyModels(models: AvailableModelLite[]): PlannerExecutorPick {
    // Never pick apiradar (removed: leaked-key aggregator, unreachable → hangs the loop).
    const list = (models || []).filter((m) => m && m.id && String(m.provider || '').toLowerCase() !== 'apiradar');
    if (list.length === 0) return { planner: null, executor: null };
    if (list.length === 1) return { planner: list[0], executor: list[0] };

    const planner = [...list].sort((a, b) => plannerScore(b) - plannerScore(a))[0];
    const byExecutor = [...list].sort((a, b) => executorScore(b) - executorScore(a));
    const executor = byExecutor.find((m) => modelKey(m) !== modelKey(planner)) || byExecutor[0];
    return { planner, executor };
}

export function buildSearchReplacePrompt(originalFile: string, instruction: string): string {
    return `You are applying a code edit. Output ONLY search/replace blocks in this exact format:

<<<<<<< ORIGINAL
(exact original code to replace)
=======
(new code to put in its place)
>>>>>>> UPDATED

File content:
\`\`\`
${originalFile}
\`\`\`

Instruction: ${instruction}

Output ONLY the search/replace blocks. No explanation.`;
}
