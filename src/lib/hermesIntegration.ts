/**
 * Hermes Agent (MIT) native integration map for HADES IDE.
 * Submodule: hermes-agent/ — skills catalog + reference; no Python subprocess backend.
 */

export const HERMES_REPO = 'hermes-agent';
export const HERMES_DOCS = 'https://hermes-agent.nousresearch.com/docs/';
export const HERMES_LICENSE = 'MIT';

export type HermesIntegrationStatus = 'wired' | 'partial' | 'planned' | 'reference';

export interface HermesIntegrationFeature {
    id: string;
    name: string;
    hermesPath: string;
    hadesPath: string;
    status: HermesIntegrationStatus;
    note?: string;
}

export const HERMES_INTEGRATION: HermesIntegrationFeature[] = [
    {
        id: 'native-skills',
        name: 'Hermes SKILL.md catalog (native)',
        hermesPath: 'hermes-agent/skills/, optional-skills/',
        hadesPath: 'hermes_skills.rs, ai_tools use_skill/search_skills',
        status: 'wired',
        note: 'Scanned at runtime; injected into Sentient system prompt. No hermes CLI.',
    },
    {
        id: 'ide-git-bash',
        name: 'HADES Portable Git Bash bundle',
        hermesPath: 'scripts/install.ps1 (reference only)',
        hadesPath: 'ide_shell.rs, terminal_commands.rs, ai_tools run_command',
        status: 'wired',
        note: '%LOCALAPPDATA%\\HADES\\git or bundles/portable-git — IDE-owned, not Hermes subprocess.',
    },
    {
        id: 'sentient-loop',
        name: 'Agent loop (Sentient, not subprocess)',
        hermesPath: 'agent/conversation_loop.py',
        hadesPath: 'ai_engine.rs + agent_harness.rs',
        status: 'wired',
        note: 'Hermes conversation loop is reference; HADES runs native Rust loop with .aim + shadow verify.',
    },
    {
        id: 'optional-skills-ui',
        name: 'Skill Store UI',
        hermesPath: 'hermes skills install, skills.sh npx skills add',
        hadesPath: 'skill_store.rs, SkillStorePanel.tsx',
        status: 'wired',
        note: 'Install owner/repo or git URL to %LOCALAPPDATA%\\HADES\\skills with security audit.',
    },
    {
        id: 'acp-streaming',
        name: 'ACP editor protocol',
        hermesPath: 'acp_adapter/entry.py',
        hadesPath: 'Port ACP patterns to ai_engine streaming (planned)',
        status: 'planned',
        note: 'Native tool cards + parallel tools — no hermes acp subprocess.',
    },
    {
        id: 'gateway-cron',
        name: 'Gateway + cron scheduler',
        hermesPath: 'gateway/run.py, tools/cronjob_tools.py',
        hadesPath: 'hermes_gateway.rs, HermesGatewayPanel.tsx',
        status: 'partial',
        note: 'OpenAI-compatible :8642 gateway wired; cron job stub accepts POST /v1/cron/jobs.',
    },
    {
        id: 'memory-plugins',
        name: 'Memory plugins (Honcho, mem0, …)',
        hermesPath: 'plugins/memory/',
        hadesPath: 'HADES .aim is primary; optional bridges later',
        status: 'planned',
    },
    {
        id: 'voice',
        name: 'Voice mode STT/TTS',
        hermesPath: 'tools/voice_mode.py, tools/tts_tool.py',
        hadesPath: 'voice.ts, AIRI',
        status: 'planned',
    },
    {
        id: 'environments',
        name: 'Docker / SSH / Modal terminals',
        hermesPath: 'tools/environments/',
        hadesPath: 'Extend ai_tools run_command backends',
        status: 'planned',
    },
    {
        id: 'trajectory-batch',
        name: 'Batch trajectory generation',
        hermesPath: 'batch_runner.py',
        hadesPath: 'Fine-tune pipeline with hades_harness rewards',
        status: 'planned',
    },
    {
        id: 'oneshot-backend',
        name: 'Hermes CLI subprocess backend',
        hermesPath: 'hermes_cli/oneshot.py (-z)',
        hadesPath: 'Removed — use Sentient native loop',
        status: 'reference',
        note: 'Intentionally not bridged; full IDE integration instead.',
    },
];

export function hermesCoveragePercent(features = HERMES_INTEGRATION): number {
    const scored = features.filter((f) => f.status !== 'reference');
    if (!scored.length) return 0;
    const wired = scored.filter((f) => f.status === 'wired').length;
    const partial = scored.filter((f) => f.status === 'partial').length;
    const planned = scored.filter((f) => f.status === 'planned').length;
    return Math.round(((wired + partial * 0.5 + planned * 0.15) / scored.length) * 100);
}

export const HADES_GIT_BASH_HINT =
    'Extract PortableGit to %LOCALAPPDATA%\\HADES\\git\\bin\\bash.exe or set HADES_GIT_BASH_PATH';

export const HERMES_INSTALL_DEV = 'Skills vendored in hermes-agent/ — no separate Hermes install required';
