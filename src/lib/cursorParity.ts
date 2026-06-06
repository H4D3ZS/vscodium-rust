/**
 * Cursor 3.x feature parity matrix (reference: bundled extensions in cursor/resources/app).
 * Status is maintained manually — update when shipping parity work.
 */

export type ParityStatus = 'done' | 'partial' | 'missing' | 'na';

export interface CursorParityFeature {
    id: string;
    name: string;
    description: string;
    /** Cursor extension or convention this maps to */
    cursorRef: string;
    status: ParityStatus;
    /** Where we implement it in vscodium-rust (if any) */
    hadesPath?: string;
}

export interface CursorParityGroup {
    id: string;
    title: string;
    cursorRef: string;
    features: CursorParityFeature[];
}

export const CURSOR_PARITY_GROUPS: CursorParityGroup[] = [
    {
        id: 'agent',
        title: 'Agent & Composer',
        cursorRef: 'cursor-agent-exec, cursor-agent-worker',
        features: [
            { id: 'composer-chat', name: 'Composer / Agent chat', description: 'Multi-turn agent with tool loop', cursorRef: 'workbench agent', status: 'done', hadesPath: 'src/agent.ts, ai_engine.rs' },
            { id: 'fast-apply', name: 'Fast Apply (SEARCH/REPLACE)', description: 'Surgical edits with diff review', cursorRef: 'Apply model', status: 'partial', hadesPath: 'patch_engine.rs, AgentStreamSubscriber.ts' },
            { id: 'tool-permissions', name: 'Tool permissions & YOLO', description: 'Approve/deny destructive tools', cursorRef: 'cursor-agent-exec', status: 'done', hadesPath: 'AgentPermissionsPanel.tsx' },
            { id: 'subagents', name: 'Subagents / parallel workers', description: 'Spawn specialist child agents', cursorRef: 'spawn_subagent', status: 'partial', hadesPath: 'apex_orchestrator.rs, workers/' },
            { id: 'background-agent', name: 'Background / cloud agent', description: 'Long-running remote composer', cursorRef: 'cursor-always-local', status: 'missing' },
            { id: 'verify-before-done', name: 'Verify before done', description: 'Run checks before marking task complete', cursorRef: 'agent harness', status: 'partial', hadesPath: 'hades_harness.rs' },
        ],
    },
    {
        id: 'retrieval',
        title: 'Codebase & @context',
        cursorRef: 'cursor-retrieval',
        features: [
            { id: 'codebase-index', name: '@codebase semantic index', description: 'Chunk + symbol index for retrieval', cursorRef: 'cursor-retrieval', status: 'partial', hadesPath: 'vector_indexer.rs' },
            { id: 'semantic-search-tool', name: 'semantic_search agent tool', description: 'AIM + vector merged search', cursorRef: 'semantic_search', status: 'partial', hadesPath: 'ai_tools.rs' },
            { id: 'search-codebase-tool', name: 'search_codebase tool', description: 'Grep + symbols + vector chunks', cursorRef: 'grepClient', status: 'partial', hadesPath: 'ai_tools.rs' },
            { id: 'embeddings', name: 'Vector embeddings (Ollama)', description: 'True cosine similarity vs keyword rank', cursorRef: 'retrieval embeddings', status: 'missing', hadesPath: 'vector_indexer.rs (embedding: None)' },
            { id: 'cursorignore', name: '.cursorignore / indexing ignore', description: 'Respect Cursor ignore files', cursorRef: '.cursorignore', status: 'partial', hadesPath: 'cursor_compat.rs' },
            { id: 'github-augment', name: 'GitHub-augmented retrieval', description: 'Login to enrich index from GitHub', cursorRef: 'cursor-retrieval.canAttemptGithubLogin', status: 'na' },
        ],
    },
    {
        id: 'project',
        title: 'Project layout & rules',
        cursorRef: 'cursor-always-local, cursor-retrieval',
        features: [
            { id: 'cursor-rules', name: '.cursor/rules/*.mdc', description: 'Project rules with frontmatter', cursorRef: '.cursor/rules', status: 'done', hadesPath: 'cursor_compat.rs, RulesManager.tsx' },
            { id: 'environment-json', name: '.cursor/environment.json', description: 'Agent environment manifest', cursorRef: 'environment.schema.json', status: 'done', hadesPath: 'cursor_compat.rs' },
            { id: 'worktrees', name: 'Agent git worktrees', description: 'Isolated branches for agent edits', cursorRef: 'cursor-commits', status: 'partial', hadesPath: 'cursor_commands.rs, shadow_workspace.rs' },
            { id: 'debug-log', name: 'Agent debug NDJSON log', description: 'Structured agent run logging', cursorRef: 'cursor-ndjson-ingest', status: 'partial', hadesPath: 'cursor_append_debug_log' },
        ],
    },
    {
        id: 'edit',
        title: 'Inline edit & Tab',
        cursorRef: 'Tab model, Quick Edit',
        features: [
            { id: 'tab-autocomplete', name: 'Tab autocomplete (FIM)', description: 'Inline ghost completions', cursorRef: 'Autocomplete feature', status: 'partial', hadesPath: 'model_capabilities.ts, inline providers' },
            { id: 'quick-edit', name: 'Ctrl+K quick edit', description: 'Selection-scoped inline edit', cursorRef: 'QuickEdit', status: 'partial', hadesPath: 'QuickEdit / PredictiveEditOverlay' },
            { id: 'next-edit', name: 'Next-edit prediction', description: 'Jump to predicted next edit site', cursorRef: 'Tab prediction', status: 'partial', hadesPath: 'tabPredictionEnabled store' },
            { id: 'inline-diff', name: 'Inline diff decorations', description: 'Monaco diff for pending agent edits', cursorRef: 'composer diff UI', status: 'partial', hadesPath: 'pendingChanges store' },
        ],
    },
    {
        id: 'mcp',
        title: 'MCP & tools',
        cursorRef: 'cursor-mcp',
        features: [
            { id: 'mcp-client', name: 'MCP client (stdio/SSE)', description: 'Connect external MCP servers', cursorRef: 'cursor-mcp', status: 'done', hadesPath: 'mcp_client.rs, mcp_registry.rs' },
            { id: 'mcp-server', name: 'Built-in MCP server', description: 'Expose IDE tools over MCP', cursorRef: 'cursor-mcp', status: 'done', hadesPath: 'mcp_server.rs :1537' },
            { id: 'browser-mcp', name: 'Browser automation MCP', description: 'Automated browser for agent', cursorRef: 'cursor-browser-automation', status: 'partial', hadesPath: 'browser_state, browser tools' },
        ],
    },
    {
        id: 'shadow',
        title: 'Shadow workspace',
        cursorRef: 'cursor-shadow-workspace',
        features: [
            { id: 'shadow-vfs', name: 'Shadow branch / VFS', description: 'Safe mutation before commit', cursorRef: 'cursor-shadow-workspace', status: 'done', hadesPath: 'shadow_workspace.rs' },
            { id: 'checkpoints', name: 'Git checkpoints', description: 'Auto snapshot before AI edits', cursorRef: 'checkpoints', status: 'done', hadesPath: 'git_checkpoints.rs' },
        ],
    },
    {
        id: 'models',
        title: 'Models & inference',
        cursorRef: 'Cursor model routing',
        features: [
            { id: 'multi-provider', name: 'Multi-provider routing', description: 'Anthropic, OpenAI, Ollama, etc.', cursorRef: 'model picker', status: 'done', hadesPath: 'inferenceSlice.ts, ai_engine.rs' },
            { id: 'local-ollama', name: 'Local Ollama (no cloud)', description: 'Unlimited local agentic inference', cursorRef: 'Ollama', status: 'done', hadesPath: 'localOllamaRegistry.ts' },
            { id: 'feature-models', name: 'Per-feature model slots', description: 'Chat / Apply / Tab / SCM models', cursorRef: 'FEATURES', status: 'done', hadesPath: 'model_capabilities.ts' },
            { id: 'composer-2', name: 'Composer 2 planner speed', description: 'Fast planner iteration timeouts', cursorRef: 'Composer 2', status: 'partial', hadesPath: 'ai_engine.rs planner timeout' },
        ],
    },
    {
        id: 'ui',
        title: 'UI & collaboration',
        cursorRef: 'workbench contributions',
        features: [
            { id: 'canvas', name: 'Canvas / Glass artifacts', description: 'Rich visual agent output panel', cursorRef: 'Canvas', status: 'missing' },
            { id: 'remote-ssh', name: 'Remote SSH / dev containers', description: 'Remote workspace development', cursorRef: 'anysphere.remote-*', status: 'missing' },
            { id: 'mobile', name: 'Mobile / iOS mirror', description: 'Device preview & stream', cursorRef: '—', status: 'partial', hadesPath: 'ios_simulator.rs (SaaS)' },
            { id: 'scm-ai', name: 'AI commit messages', description: 'Generate git commit from diff', cursorRef: 'cursor.generateGitCommitMessage', status: 'partial', hadesPath: 'SCM feature slot' },
        ],
    },
];

export function paritySummary(groups: CursorParityGroup[] = CURSOR_PARITY_GROUPS) {
    const counts = { done: 0, partial: 0, missing: 0, na: 0, total: 0 };
    for (const g of groups) {
        for (const f of g.features) {
            counts[f.status]++;
            if (f.status !== 'na') counts.total++;
        }
    }
    const pct = counts.total
        ? Math.round(((counts.done + counts.partial * 0.5) / counts.total) * 100)
        : 0;
    return { ...counts, coveragePercent: pct };
}

export const STATUS_LABEL: Record<ParityStatus, string> = {
    done: 'Shipped',
    partial: 'Partial',
    missing: 'Not yet',
    na: 'N/A',
};

export const STATUS_COLOR: Record<ParityStatus, string> = {
    done: '#3fb950',
    partial: '#d29922',
    missing: '#f85149',
    na: '#8b949e',
};
