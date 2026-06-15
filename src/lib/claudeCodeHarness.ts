/**
 * Claude Code parity harness — maps claude-map + claurst patterns to vscodium-rust.
 *
 * Reference trees (do not import at runtime):
 * - claude-map/query.ts, query/stopHooks.ts, query/tokenBudget.ts
 * - claurst/src-rust/crates/query/src/lib.rs
 * - src-tauri/src/agent_harness.rs (wired harness)
 */

export const CLAUDE_MAP_ROOT = 'claude-map';
export const CLAURST_ROOT = 'claurst/src-rust';

export type HarnessFeatureStatus = 'wired' | 'partial' | 'reference-only';

export interface ClaudeCodeHarnessFeature {
    id: string;
    name: string;
    claudeRef: string;
    hadesPath: string;
    status: HarnessFeatureStatus;
}

/** What we ship vs what lives in the reference maps. */
export const CLAUDE_CODE_HARNESS: ClaudeCodeHarnessFeature[] = [
    {
        id: 'query-loop',
        name: 'Agent query loop',
        claudeRef: 'claude-map/query.ts → queryLoop',
        hadesPath: 'src-tauri/src/ai_engine.rs',
        status: 'wired',
    },
    {
        id: 'verify-before-done',
        name: 'Verify before done',
        claudeRef: 'claude-map/constants/prompts.ts',
        hadesPath: 'src-tauri/src/agent_harness.rs + verify gate in ai_engine.rs',
        status: 'wired',
    },
    {
        id: 'stuck-loop',
        name: 'Stuck / repeat-tool guard',
        claudeRef: 'claude-map/query.ts death-spiral guards',
        hadesPath: 'agent_harness.rs detect_stuck_loop',
        status: 'wired',
    },
    {
        id: 'tool-result-budget',
        name: 'Tool result budget',
        claudeRef: 'claurst query apply_tool_result_budget',
        hadesPath: 'agent_harness.rs apply_tool_result_budget',
        status: 'wired',
    },
    {
        id: 'shadow-verify',
        name: 'Shadow VFS + cargo check',
        claudeRef: 'claude-map/skills/bundled/verify.ts',
        hadesPath: 'hades_harness.rs, ai_tools verify_implementation',
        status: 'wired',
    },
    {
        id: 'claurst-backend',
        name: 'Claurst external agent',
        claudeRef: 'claurst run_query_loop',
        hadesPath: 'src/claurst/bridge.ts, agentBackend=claurst',
        status: 'partial',
    },
    {
        id: 'stop-hooks',
        name: 'User Stop hooks',
        claudeRef: 'claude-map/query/stopHooks.ts',
        hadesPath: '—',
        status: 'reference-only',
    },
    {
        id: 'streaming-tool-exec',
        name: 'Streaming tool executor',
        claudeRef: 'claude-map/services/tools/StreamingToolExecutor.ts',
        hadesPath: '—',
        status: 'reference-only',
    },
];

/** Models that need the full agent loop (never fast-chat only). */
const HEAVY_AGENT_PATTERN =
    /(?:^|[/:\-_])(40|35|32|70|72|128|229)(?:b|-)|deck-opus|neo-code|qwen3\.6|qwen3-6|qwen3\.5|minimax-m2|kimi-k2/i;

const QWEN3_PATTERN = /qwen3(?:\.\d+)?(?:-|\/|:|$)/i;

export function isHeavyAgentModel(modelTag: string): boolean {
    return HEAVY_AGENT_PATTERN.test(modelTag.toLowerCase());
}

export function isQwen3Family(modelTag: string): boolean {
    return QWEN3_PATTERN.test(modelTag);
}

/**
 * Agent-mode fast path: only for plain questions. Action verbs and Qwen3.6-class
 * models must enter the Rust tool loop (Claude Code parity).
 */
export function shouldUseAgentToolLoop(
    userPrompt: string,
    agentMode: string,
    modelTag: string,
): boolean {
    const mode = agentMode || 'Agent';
    if (mode === 'Chat') return false;
    if (/BugBounty|Red Team|Blue Team|Sentient/i.test(mode)) return true;
    if (/^\s*\[INTENT\s*:/i.test(userPrompt)) return true;
    if (/\bhttps?:\/\S+/i.test(userPrompt)) return true;
    if (/\b(write|create|build|implement|fix|run|execute|deploy|audit|scan|patch)\b/i.test(userPrompt)) {
        return true;
    }
    if (isHeavyAgentModel(modelTag) && mode === 'Agent') return true;
    return false;
}

export function harnessCoveragePercent(features = CLAUDE_CODE_HARNESS): number {
    const scored = features.filter((f) => f.status !== 'reference-only');
    if (!scored.length) return 0;
    const wired = scored.filter((f) => f.status === 'wired').length;
    const partial = scored.filter((f) => f.status === 'partial').length;
    return Math.round(((wired + partial * 0.5) / scored.length) * 100);
}
