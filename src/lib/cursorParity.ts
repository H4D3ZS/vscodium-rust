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
            { id: 'fast-apply', name: 'Fast Apply (SEARCH/REPLACE)', description: 'Surgical edits with diff review', cursorRef: 'Apply model', status: 'done', hadesPath: 'patch_engine.rs, AgentStreamSubscriber.ts' },
            { id: 'tool-permissions', name: 'Tool permissions & YOLO', description: 'Approve/deny destructive tools', cursorRef: 'cursor-agent-exec', status: 'done', hadesPath: 'AgentPermissionsPanel.tsx' },
            { id: 'subagents', name: 'Subagents / parallel workers', description: 'Spawn specialist child agents', cursorRef: 'spawn_subagent', status: 'done', hadesPath: 'apex_orchestrator.rs, workers/' },
            { id: 'background-agent', name: 'Background / cloud agent', description: 'Long-running local background composer + Jobs panel', cursorRef: 'cursor-always-local', status: 'done', hadesPath: 'agentSlice.ts, jobs.rs, ai_chat_oneshot' },
            { id: 'verify-before-done', name: 'Verify before done', description: 'Run checks before marking task complete', cursorRef: 'claude-map/prompts.ts', status: 'done', hadesPath: 'agent_harness.rs, hades_harness.rs' },
            { id: 'claude-code-harness', name: 'Claude Code harness', description: 'Anti-stuck, tool budget, verify contract (claude-map + claurst)', cursorRef: 'query.ts, claurst query', status: 'done', hadesPath: 'agent_harness.rs, claudeCodeHarness.ts' },
            { id: 'hermes-agent', name: 'Hermes skills + IDE shell', description: 'Native SKILL.md catalog + HADES Git Bash (no subprocess)', cursorRef: 'hermes-agent', status: 'done', hadesPath: 'hermes_skills.rs, ide_shell.rs' },
        ],
    },
    {
        id: 'retrieval',
        title: 'Codebase & @context',
        cursorRef: 'cursor-retrieval',
        features: [
            { id: 'codebase-index', name: '@codebase semantic index', description: 'Chunk + symbol index with Ollama embeddings', cursorRef: 'cursor-retrieval', status: 'done', hadesPath: 'vector_indexer.rs, embeddings.rs' },
            { id: 'semantic-search-tool', name: 'semantic_search agent tool', description: 'AIM + vector merged search with cosine similarity', cursorRef: 'semantic_search', status: 'done', hadesPath: 'ai_tools.rs, vector_indexer.rs' },
            { id: 'search-codebase-tool', name: 'search_codebase tool', description: 'Grep + symbols + vector chunks', cursorRef: 'grepClient', status: 'done', hadesPath: 'ai_tools.rs' },
            { id: 'embeddings', name: 'Vector embeddings (Ollama)', description: 'Cosine similarity via nomic-embed-text during index + search', cursorRef: 'retrieval embeddings', status: 'done', hadesPath: 'embeddings.rs, vector_indexer.rs' },
            { id: 'cursorignore', name: '.cursorignore / indexing ignore', description: 'Respect Cursor ignore files', cursorRef: '.cursorignore', status: 'done', hadesPath: 'cursor_compat.rs, ai_tools.rs' },
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
            { id: 'worktrees', name: 'Agent git worktrees', description: 'Isolated branches for agent edits', cursorRef: 'cursor-commits', status: 'done', hadesPath: 'cursor_commands.rs, WorktreesPanel.tsx' },
            { id: 'debug-log', name: 'Agent debug NDJSON log', description: 'Structured agent run logging', cursorRef: 'cursor-ndjson-ingest', status: 'done', hadesPath: 'cursor_compat.rs, ai_tools.rs' },
        ],
    },
    {
        id: 'edit',
        title: 'Inline edit & Tab',
        cursorRef: 'Tab model, Quick Edit',
        features: [
            { id: 'tab-autocomplete', name: 'Tab autocomplete (FIM)', description: 'Ollama native /api/generate FIM for coder models', cursorRef: 'Autocomplete feature', status: 'done', hadesPath: 'ai_commands.rs ai_inline_complete' },
            { id: 'quick-edit', name: 'Ctrl+K quick edit', description: 'Selection-scoped inline edit', cursorRef: 'QuickEdit', status: 'done', hadesPath: 'Editor.tsx, QuickEdit model slot' },
            { id: 'next-edit', name: 'Next-edit prediction', description: 'Jump to predicted next edit site', cursorRef: 'Tab prediction', status: 'done', hadesPath: 'PredictiveEditOverlay.tsx, predict_next_edit' },
            { id: 'inline-diff', name: 'Inline diff decorations', description: 'Monaco diff for pending agent edits', cursorRef: 'composer diff UI', status: 'done', hadesPath: 'pendingChanges store, Editor.tsx' },
        ],
    },
    {
        id: 'mcp',
        title: 'MCP & tools',
        cursorRef: 'cursor-mcp',
        features: [
            { id: 'mcp-client', name: 'MCP client (stdio/SSE)', description: 'Connect external MCP servers', cursorRef: 'cursor-mcp', status: 'done', hadesPath: 'mcp_client.rs, mcp_registry.rs' },
            { id: 'mcp-server', name: 'Built-in MCP server', description: 'Expose IDE tools over MCP', cursorRef: 'cursor-mcp', status: 'done', hadesPath: 'mcp_server.rs :1537' },
            { id: 'browser-mcp', name: 'Browser automation MCP', description: 'Automated browser for agent', cursorRef: 'cursor-browser-automation', status: 'done', hadesPath: 'browser.rs, browser_status' },
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
            { id: 'local-lemonade', name: 'Local Lemonade (no cloud)', description: 'Unlimited local agentic inference on real llama.cpp', cursorRef: 'Ollama', status: 'done', hadesPath: 'application/commands/ai.rs' },
            { id: 'feature-models', name: 'Per-feature model slots', description: 'Chat / Apply / Tab / SCM models', cursorRef: 'FEATURES', status: 'done', hadesPath: 'model_capabilities.ts' },
            { id: 'composer-2-amd3900', name: 'Composer 2 AMD 3900 hybrid', description: 'Local fast chat + remote MiniMax M2.7 agent with failover', cursorRef: 'Composer 2', status: 'done', hadesPath: 'composer2Stack.ts' },
            { id: 'composer-2', name: 'Composer 2 stack presets', description: 'Kimi K2.6-class hybrid + Composer 2 Fast chat routing', cursorRef: 'Composer 2', status: 'done', hadesPath: 'src/lib/composer2Stack.ts' },
            { id: 'composer-2-fast', name: 'Composer 2 Fast chat slot', description: 'Fast model for Q&A; heavy model for agent loop', cursorRef: 'Composer 2 Fast', status: 'done', hadesPath: 'composer2Stack.ts, agent.ts' },
        ],
    },
    {
        id: 'ui',
        title: 'UI & collaboration',
        cursorRef: 'workbench contributions',
        features: [
            { id: 'canvas', name: 'Canvas / Glass artifacts', description: 'Rich visual agent output panel', cursorRef: 'Canvas', status: 'done', hadesPath: 'CanvasArtifactPanel.tsx, ag_brain_*' },
            { id: 'remote-ssh', name: 'Remote SSH / dev containers', description: 'Mount remote workspace via rsync mirror + sync push/pull', cursorRef: 'anysphere.remote-*', status: 'done', hadesPath: 'remote_commands.rs, RemoteSshPanel.tsx' },
            { id: 'mobile', name: 'Mobile / iOS mirror', description: 'Device preview & stream', cursorRef: '—', status: 'done', hadesPath: 'ios_simulator.rs, UnifiedEmulatorPanel' },
            { id: 'scm-ai', name: 'AI commit messages', description: 'Generate git commit from diff', cursorRef: 'cursor.generateGitCommitMessage', status: 'done', hadesPath: 'gitSlice.ts, get_git_diff' },
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
