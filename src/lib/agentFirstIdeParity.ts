/**
 * Master parity map: HADES IDE vs Claude Code (claude-map) vs Claurst vs Hermes Agent.
 * Open-source integration targets — MIT/GPL boundaries respected via subprocess bridges.
 */

import { CLAUDE_CODE_HARNESS, harnessCoveragePercent } from './claudeCodeHarness';
import { HERMES_INTEGRATION, hermesCoveragePercent } from './hermesIntegration';
import { paritySummary, CURSOR_PARITY_GROUPS } from './cursorParity';

export type IdeFeatureTier = 'hades-advantage' | 'wired' | 'partial' | 'integrate-next' | 'missing';

export interface AgentFirstIdeFeature {
    id: string;
    name: string;
    description: string;
    tier: IdeFeatureTier;
    sources: string[];
    hadesPath?: string;
}

export const AGENT_FIRST_IDE_FEATURES: AgentFirstIdeFeature[] = [
    // ── HADES advantages (already ahead) ──
    {
        id: 'aim-memory',
        name: '.aim binary memory + AIM proxy',
        description: 'Zero-grep project brain, memmap context injection on :1536',
        tier: 'hades-advantage',
        sources: ['kortex/libaim', 'aim-proxy'],
        hadesPath: 'memory_store.rs, aim_store.rs, kortex/',
    },
    {
        id: 'shadow-mcts',
        name: 'Shadow VFS + MCTS verify',
        description: 'Surgical patches + cargo-check before commit — stronger than Hermes soft verify skill',
        tier: 'hades-advantage',
        sources: ['hades_harness'],
        hadesPath: 'shadow_workspace.rs, hades_harness.rs, patch_engine.rs',
    },
    {
        id: 'apex-red-team',
        name: 'APEX offensive orchestrator',
        description: '7-model specialist routing for security research modes',
        tier: 'hades-advantage',
        sources: ['vscodium-rust'],
        hadesPath: 'apex_orchestrator.rs, apex_red_team.rs',
    },
    {
        id: 'composer2-hybrid',
        name: 'Composer 2 hybrid routing',
        description: 'Local 7B chat + remote MiniMax/Kimi agent on GPU server',
        tier: 'hades-advantage',
        sources: ['composer2Stack'],
        hadesPath: 'src/lib/composer2Stack.ts',
    },
    // ── Wired (Claude Code / harness) ──
    {
        id: 'agent-loop',
        name: 'Autonomous agent loop',
        description: 'Tool loop with verify gate, stuck detection, action-tool enforcement',
        tier: 'wired',
        sources: ['claude-map/query.ts', 'claurst/query', 'ai_engine.rs'],
        hadesPath: 'ai_engine.rs, agent_harness.rs',
    },
    {
        id: 'claude-harness',
        name: 'Claude Code harness contract',
        description: 'Verify-before-done, anti-loop, tool-result budget, Qwen3 protocol',
        tier: 'wired',
        sources: ['claude-map', 'claurst'],
        hadesPath: 'agent_harness.rs, claudeCodeHarness.ts',
    },
    {
        id: 'cursor-compat',
        name: 'Cursor project layout',
        description: '.cursor/rules, environment.json, worktrees, ignore files',
        tier: 'wired',
        sources: ['cursor-retrieval'],
        hadesPath: 'cursor_compat.rs',
    },
    // ── Partial integrations ──
    {
        id: 'claurst-backend',
        name: 'Claurst external backend',
        description: 'GPL subprocess agent — session-id streaming via claurst-stream events',
        tier: 'wired',
        sources: ['claurst'],
        hadesPath: 'claurst_bridge.rs, claurst/bridge.ts',
    },
    {
        id: 'hermes-native',
        name: 'Hermes skills (native in Sentient)',
        description: 'SKILL.md catalog from hermes-agent/ — use_skill, search_skills, prompt injection',
        tier: 'wired',
        sources: ['hermes-agent/skills'],
        hadesPath: 'hermes_skills.rs, ai_tools.rs, ai_engine.rs',
    },
    {
        id: 'mcp-dual',
        name: 'MCP client + server',
        description: 'IDE-native MCP; Hermes also has MCP — needs unified registry UI',
        tier: 'wired',
        sources: ['cursor-mcp', 'hermes mcp_tool.py'],
        hadesPath: 'mcp_client.rs, mcp_server.rs, mcp_registry.rs',
    },
    {
        id: 'vector-index',
        name: 'Semantic codebase index',
        description: 'Vector + AIM search with Ollama embeddings at index time',
        tier: 'wired',
        sources: ['cursor-retrieval'],
        hadesPath: 'vector_indexer.rs, embeddings.rs',
    },
    // ── Integrate next (from Hermes + Claude map) ──
    {
        id: 'hermes-skills',
        name: 'Hermes skills hub (74 bundled + optional)',
        description: 'agentskills.io SKILL.md scanned natively — no HERMES_HOME symlink',
        tier: 'wired',
        sources: ['hermes-agent/skills'],
        hadesPath: 'hermes_skills.rs',
    },
    {
        id: 'hermes-git-bash',
        name: 'HADES Portable Git Bash bundle',
        description: 'IDE-owned MinGit — agent shell + terminal on Windows without admin',
        tier: 'wired',
        sources: ['Git for Windows portable'],
        hadesPath: 'ide_shell.rs, terminal_commands.rs, ai_tools.rs',
    },
    {
        id: 'hermes-acp',
        name: 'ACP streaming (native port)',
        description: 'Tool streaming patterns from Hermes acp — port to ai_engine, no subprocess',
        tier: 'integrate-next',
        sources: ['hermes acp_adapter'],
        hadesPath: 'ai_engine.rs streaming (planned)',
    },
    {
        id: 'pytorch-ml-studio',
        name: 'PyTorch ML Studio',
        description: 'Friend-of-ML-engineers pipeline: datasets, Optuna HPO, graphs, export — TorchStudio-class',
        tier: 'hades-advantage',
        sources: ['pytorch.org', 'torchstudio.ai'],
        hadesPath: 'ml_studio.rs, PyTorchStudioPanel.tsx, scripts/ml/',
    },
    {
        id: 'hermes-gateway',
        name: 'Hermes gateway + cron',
        description: 'Telegram/Discord/scheduled agents + OpenAI-compatible :8642 API',
        tier: 'integrate-next',
        sources: ['gateway/run.py'],
        hadesPath: 'planned native gateway',
    },
    {
        id: 'hermes-voice',
        name: 'Hermes voice mode',
        description: 'Browser SpeechRecognition STT in chat + ElevenLabs/Qwen TTS',
        tier: 'wired',
        sources: ['tools/voice_mode.py'],
        hadesPath: 'RightSidebar.tsx, voice.ts, AIRI',
    },
    {
        id: 'hermes-soul',
        name: 'SOUL.md personality tier',
        description: 'Stable persona file — SOUL.md + .hades/SOUL.md merged into rules',
        tier: 'wired',
        sources: ['agent/system_prompt.py'],
        hadesPath: 'rules_engine.rs, RulesManager',
    },
    {
        id: 'stop-hooks',
        name: 'User Stop hooks',
        description: 'Claude Code stopHooks — .hades/stop_hooks.json at turn end',
        tier: 'wired',
        sources: ['claude-map/query/stopHooks.ts'],
        hadesPath: 'stop_hooks.rs, ai_chat_oneshot',
    },
    {
        id: 'streaming-tools',
        name: 'Streaming tool executor',
        description: 'Parallel safe tools during model stream',
        tier: 'integrate-next',
        sources: ['claude-map/StreamingToolExecutor.ts'],
        hadesPath: 'ai_engine.rs',
    },
    {
        id: 'trajectory-train',
        name: 'Trajectory export for fine-tune',
        description: 'Export agent trajectories as JSONL for SFT/DPO pipelines',
        tier: 'wired',
        sources: ['hermes batch_runner', 'hades_harness rewards'],
        hadesPath: 'ag_export_trajectory_jsonl, TrajectoryPanel.tsx',
    },
    // ── Missing (Cursor cloud class) ──
    {
        id: 'background-local',
        name: 'Background local agent',
        description: 'Long-running oneshot agent + Jobs panel progress',
        tier: 'wired',
        sources: ['cursor-always-local'],
        hadesPath: 'jobs.rs, agentSlice.ts',
    },
    {
        id: 'tab-model',
        name: 'Tab FIM autocomplete',
        description: 'Ollama /api/generate FIM for coder models',
        tier: 'wired',
        sources: ['Tab model'],
        hadesPath: 'ai_commands.rs ai_inline_complete',
    },
    {
        id: 'remote-ssh',
        name: 'Remote SSH / dev containers',
        description: 'SSH probe + remote directory listing (full workspace mount planned)',
        tier: 'partial',
        sources: ['anysphere.remote-*'],
        hadesPath: 'remote_commands.rs, RemoteSshPanel.tsx',
    },
];

export function agentFirstIdeSummary() {
    const tiers = { 'hades-advantage': 0, wired: 0, partial: 0, 'integrate-next': 0, missing: 0 };
    for (const f of AGENT_FIRST_IDE_FEATURES) tiers[f.tier]++;
    const cursor = paritySummary(CURSOR_PARITY_GROUPS);
    return {
        ...tiers,
        total: AGENT_FIRST_IDE_FEATURES.length,
        claudeHarnessPct: harnessCoveragePercent(),
        hermesPct: hermesCoveragePercent(),
        cursorPct: cursor.coveragePercent,
    };
}

export const TIER_LABEL: Record<IdeFeatureTier, string> = {
    'hades-advantage': 'HADES edge',
    wired: 'Shipped',
    partial: 'Partial',
    'integrate-next': 'Next integrate',
    missing: 'Not yet',
};

export const TIER_COLOR: Record<IdeFeatureTier, string> = {
    'hades-advantage': '#c297ff',
    wired: '#56d364',
    partial: '#e3b341',
    'integrate-next': '#79c0ff',
    missing: '#ff8b80',
};
