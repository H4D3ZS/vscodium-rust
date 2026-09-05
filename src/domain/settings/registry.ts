// Declarative settings registry — single source of truth for the Settings UI
// (Milestone C, ARCHITECTURE.md). Sections, search, and routing
// all derive from this data. PURE DATA: the domain layer must not import
// React — SettingsPage.tsx maps each item's `panel` key to a component.

export type SettingsSectionId =
    | 'editor'
    | 'appearance'
    | 'ai-models'
    | 'agent'
    | 'extensions'
    | 'privacy-account'
    | 'keyboard'
    | 'advanced';

export interface SettingsSection {
    id: SettingsSectionId;
    label: string;
    /** codicon name */
    icon: string;
}

export interface SettingsItem {
    id: string;
    label: string;
    section: SettingsSectionId;
    /** codicon name */
    icon: string;
    /** Search terms beyond the label — include old section names users knew. */
    keywords: string[];
    /** Component key resolved by SettingsPage's PANEL_COMPONENTS map. */
    panel: string;
}

export const SETTINGS_SECTIONS: SettingsSection[] = [
    { id: 'editor', label: 'Editor', icon: 'edit' },
    { id: 'appearance', label: 'Appearance', icon: 'symbol-color' },
    { id: 'ai-models', label: 'AI Models', icon: 'circuit-board' },
    { id: 'agent', label: 'Agent', icon: 'comment-discussion' },
    { id: 'extensions', label: 'Additional Modules', icon: 'extensions' },
    { id: 'privacy-account', label: 'Privacy & Account', icon: 'account' },
    { id: 'keyboard', label: 'Keyboard', icon: 'keyboard' },
    { id: 'advanced', label: 'Advanced', icon: 'beaker' },
];

export const SETTINGS_ITEMS: SettingsItem[] = [
    // ── Editor ──────────────────────────────────────────────────────────
    { id: 'editor', label: 'Editor', section: 'editor', icon: 'edit', panel: 'editor', keywords: ['font', 'tab size', 'autosave', 'monaco'] },
    { id: 'workspace', label: 'Workspace', section: 'editor', icon: 'folder', panel: 'workspace', keywords: ['project', 'folders', 'root'] },

    // ── Appearance ──────────────────────────────────────────────────────
    { id: 'theme', label: 'Theme', section: 'appearance', icon: 'symbol-color', panel: 'theme', keywords: ['color', 'dark', 'light', 'high contrast'] },
    { id: 'avatar', label: 'AI Avatar', section: 'appearance', icon: 'person', panel: 'agent-view:avatar', keywords: ['airi', 'vrm', '3d', 'companion', 'visibility'] },

    // ── AI Models ───────────────────────────────────────────────────────
    { id: 'models', label: 'Models & API Keys', section: 'ai-models', icon: 'circuit-board', panel: 'models', keywords: ['provider', 'anthropic', 'openai', 'gemini', 'groq', 'openrouter', 'deepseek', 'mistral', 'api key', 'base url'] },
    { id: 'model-selection', label: 'Model Selection', section: 'ai-models', icon: 'chip', panel: 'model-selection', keywords: ['auto-detect', 'engines', 'apply to all'] },
    { id: 'lemonade', label: 'Lemonade', section: 'ai-models', icon: 'server-environment', panel: 'lemonade', keywords: ['local', 'nvidia', 'rocm', 'amd', 'gpu', 'glm'] },
    { id: 'fcc', label: 'Free Claude Code', section: 'ai-models', icon: 'server-process', panel: 'fcc', keywords: ['claude', 'codex', 'proxy', 'openrouter', 'nvidia nim', 'provider routing'] },
    { id: 'inference', label: 'Inference Backend', section: 'ai-models', icon: 'server-process', panel: 'inference-backend', keywords: ['vllm', 'lm studio', 'litellm', 'endpoint', 'kortex', 'rocmfpx', 'rocm', 'amd', 'local ai', 'llama.cpp', 'lemonade'] },

    // ── Agent ───────────────────────────────────────────────────────────
    { id: 'chat', label: 'Chat & Agent', section: 'agent', icon: 'comment-discussion', panel: 'chat', keywords: ['yolo', 'modes', 'backend', 'planner', 'reasoning'] },
    { id: 'permissions', label: 'Permissions', section: 'agent', icon: 'shield', panel: 'permissions', keywords: ['tool approval', 'allow', 'deny', 'safety'] },
    { id: 'skill-store', label: 'Skill Store', section: 'agent', icon: 'library', panel: 'skill-store', keywords: ['skills', 'install', 'agentskills', 'SKILL.md'] },
    { id: 'workflow', label: 'Specs & Workflow', section: 'agent', icon: 'tasklist', panel: 'workflow', keywords: ['specs', 'tasks', 'work items'] },

    // ── Extensions ──────────────────────────────────────────────────────
    { id: 'modules', label: 'Module Installer', section: 'extensions', icon: 'extensions', panel: 'modules', keywords: ['install', 'catalog', 'add-on', 'module', 'extension modules'] },
    { id: 'platform', label: 'Platform Status', section: 'extensions', icon: 'checklist', panel: 'platform', keywords: ['health', 'integrations', 'status'] },

    // ── Privacy & Account ───────────────────────────────────────────────
    { id: 'account', label: 'Account', section: 'privacy-account', icon: 'account', panel: 'account', keywords: ['login', 'subscription', 'pro', 'sign in'] },
    { id: 'enterprise', label: 'Enterprise', section: 'privacy-account', icon: 'organization', panel: 'enterprise', keywords: ['governance', 'audit', 'sso'] },
    { id: 'privacy', label: 'Privacy', section: 'privacy-account', icon: 'shield', panel: 'privacy', keywords: ['telemetry', 'data', 'offline'] },

    // ── Keyboard ────────────────────────────────────────────────────────
    { id: 'keybindings', label: 'Keyboard Shortcuts', section: 'keyboard', icon: 'keyboard', panel: 'keybindings', keywords: ['keys', 'shortcuts', 'chords', 'bindings'] },

    // ── Advanced ────────────────────────────────────────────────────────
    { id: 'apex', label: 'APEX Intelligence', section: 'advanced', icon: 'circuit-board', panel: 'apex', keywords: ['engines', 'architect', 'threat', 'red team', 'specialist'] },
    { id: 'airi-core', label: 'Sentient Core', section: 'advanced', icon: 'beaker', panel: 'airi-core', keywords: ['airi', 'consciousness', 'biology', 'autonomous'] },
    { id: 'ane', label: 'ANE Acceleration', section: 'advanced', icon: 'rocket', panel: 'ane', keywords: ['neural engine', 'apple silicon', 'npu', 'similarity', 'hardware'] },
    { id: 'kortex', label: 'Kortex / AIM', section: 'advanced', icon: 'database', panel: 'kortex', keywords: ['aim', 'memory map', 'proxy', 'weight'] },
    { id: 'memory', label: 'Memory (.aim)', section: 'advanced', icon: 'archive', panel: 'agent-view:memory', keywords: ['context', 'offload', 'quantizer'] },
    { id: 'voice', label: 'Voice & TTS', section: 'advanced', icon: 'unmute', panel: 'agent-view:voice', keywords: ['speech', 'tts', 'microphone', 'speak'] },
    { id: 'steering', label: 'Steering', section: 'advanced', icon: 'symbol-keyword', panel: 'steering', keywords: ['rules', 'system prompt', 'guidance'] },
    { id: 'hooks', label: 'Hooks', section: 'advanced', icon: 'zap', panel: 'hooks', keywords: ['automation', 'on save', 'triggers'] },
    { id: 'ag-hooks', label: 'Lifecycle Hooks', section: 'advanced', icon: 'debug-breakpoint-log', panel: 'ag-hooks', keywords: ['antigravity', 'lifecycle', 'events'] },
    { id: 'lsp', label: 'Language Servers', section: 'advanced', icon: 'symbol-class', panel: 'lsp', keywords: ['lsp', 'diagnostics', 'rust-analyzer', 'tsserver', 'intellisense'] },
    { id: 'mcps', label: 'MCP & Tools', section: 'advanced', icon: 'plug', panel: 'agent-view:mcps', keywords: ['mcp', 'servers', 'tools', 'protocol'] },
    { id: 'pytorch', label: 'PyTorch ML Studio', section: 'advanced', icon: 'flame', panel: 'pytorch', keywords: ['ml', 'training', 'thermal', 'studio', 'torch'] },
];

/** Old flat category id (pre-Milestone-C) → new item id. Keeps deep links
 *  (sessionStorage settings.initialTab / settings.category.*) working. */
export const LEGACY_CATEGORY_MAP: Record<string, string> = {
    chat: 'chat', permissions: 'permissions', models: 'models', workspace: 'workspace',
    lsp: 'lsp', 'skill-store': 'skill-store', modules: 'modules', platform: 'platform',
    pytorch: 'pytorch', steering: 'steering', hooks: 'hooks',
    'ag-hooks': 'ag-hooks', workflow: 'workflow', mcps: 'mcps', airi: 'airi-core',
    apex: 'apex', ane: 'ane', 'model-selection': 'model-selection', hades: 'kortex',
    memory: 'memory', voice: 'voice', avatar: 'avatar', privacy: 'privacy',
    account: 'account', enterprise: 'enterprise', editor: 'editor', theme: 'theme',
    keybindings: 'keybindings',
};

/** Substring search over label + keywords + section label. */
export function searchSettings(query: string): SettingsItem[] {
    const q = query.trim().toLowerCase();
    if (!q) return [];
    const sectionLabel = new Map(SETTINGS_SECTIONS.map((s) => [s.id, s.label.toLowerCase()]));
    return SETTINGS_ITEMS.filter(
        (it) =>
            it.label.toLowerCase().includes(q) ||
            it.keywords.some((k) => k.toLowerCase().includes(q)) ||
            (sectionLabel.get(it.section) ?? '').includes(q),
    );
}

export function itemsForSection(section: SettingsSectionId): SettingsItem[] {
    return SETTINGS_ITEMS.filter((it) => it.section === section);
}

export function findItem(id: string): SettingsItem | undefined {
    return SETTINGS_ITEMS.find((it) => it.id === id);
}
