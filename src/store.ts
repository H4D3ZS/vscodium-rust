import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { computeDiffBlocks, patchContentSelective } from './services/DiffService';
import { terminalManager } from './terminal';
import { initTheme } from './theme_engine';
import {
    type ProviderName,
    type FeatureName,
    type ModelSelection,
    type ModelSelectionOptions,
    type ModelSelectionOfFeature,
    type GlobalSettings,
    defaultGlobalSettings,
    defaultModelSelectionOfFeature,
} from './model_capabilities';

/** Bare hostnames break reqwest in Rust; infer https for public FQDNs, http for localhost/LAN. */
function ollamaShouldInferScheme(s: string): boolean {
    const head = (s.split('/')[0].split('?')[0] || '').replace(/^\/+/, '');
    if (!head) return false;
    const lower = head.toLowerCase();
    if (lower.startsWith('localhost')) return true;
    const first = head.charCodeAt(0);
    if ((first >= 48 && first <= 57) || head.startsWith('[')) {
        return head.includes('.') || head.includes(':');
    }
    if (!head.includes('.')) return false;
    const labels = head.split('.').filter(Boolean);
    if (labels.length < 2) return false;
    const tld = labels[labels.length - 1] || '';
    return tld.length >= 2;
}

export function normalizeOllamaUrl(raw: string): string {
    const s = raw.trim().replace(/\/+$/, '');
    if (!s) return 'http://127.0.0.1:11434';
    if (/^https?:\/\//i.test(s)) return s;
    if (s.startsWith('//')) return `https:${s}`.replace(/\/+$/, '');
    if (!ollamaShouldInferScheme(s)) return s;
    const hostish = s.replace(/^\/+/, '');
    const lower = hostish.toLowerCase();
    const useHttp =
        lower.startsWith('localhost') ||
        lower.startsWith('127.') ||
        lower.startsWith('0.0.0.0') ||
        lower.startsWith('[::1]') ||
        /^192\.168\.\d+\.\d+/.test(lower) ||
        /^10\.\d+\.\d+\.\d+/.test(lower) ||
        /^172\.(1[6-9]|2\d|3[0-1])\.\d+\.\d+/.test(lower);
    const scheme = useHttp ? 'http' : 'https';
    return `${scheme}://${hostish}`.replace(/\/+$/, '');
}

// Known dead/legacy hosts that may still be lingering in localStorage from
// older sessions. If we hit one of these on boot we drop it and fall back
// to direct localhost so the agent doesn't spend the first 30 seconds
// timing out against a host the user has already taken offline.
const DEAD_OLLAMA_HOSTS = [
    'ai.cyberifrit.xyz',
];

function readStoredOllamaUrl(): string {
    try {
        if (typeof localStorage === 'undefined') return 'http://127.0.0.1:11434';
        const raw = (localStorage.getItem('ollamaUrl') || '').trim();
        if (!raw) return 'http://127.0.0.1:11434';
        // Strip any of the known-dead hosts so we don't reconnect to them.
        // Users who legitimately want a custom host can re-enter it in
        // Settings → Ollama Integration; that codepath persists it back.
        for (const dead of DEAD_OLLAMA_HOSTS) {
            if (raw.includes(dead)) {
                try { localStorage.removeItem('ollamaUrl'); } catch { /* tracking prevention */ }
                return 'http://127.0.0.1:11434';
            }
        }
        return raw;
    } catch {
        return 'http://127.0.0.1:11434';
    }
}

interface EditorTab {
    id: string;
    filename: string;
    path: string;
    content: string;
    isModified: boolean;
    language: string;
    type?: 'file' | 'settings';
}

export interface AgentStep {
    name: string;
    status: 'running' | 'success' | 'error';
    args?: any;
    result?: string;
    summary?: string;
    type?: 'filesystem' | 'git' | 'terminal' | 'browser' | 'system' | 'other';
    callId?: string;
}

export interface PendingChange {
    id: string;
    path: string;
    originalContent: string;
    proposedContent: string;
    newContent: string; // The draft with some hunks possibly accepted/rejected
    description?: string;
    additions?: number;
    deletions?: number;
    acceptedHunkIds?: string[];
    rejectedHunkIds?: string[];
}

export interface Artifact {
    id: string;
    type: 'screenshot' | 'terminal_log' | 'diff' | 'task_plan' | 'walkthrough' | 'record';
    path: string;
    timestamp: number;
    title?: string;
    description?: string;
    metadata?: Record<string, any>;
}

export interface AgentMessage {
    role: 'user' | 'assistant';
    content: string;
    thoughts?: string;
    steps?: AgentStep[];
    files?: string[];
    artifacts?: Artifact[];
    context?: AttachedContext[];
    timestamp?: number;
    isSubAgentResponse?: boolean;
    /** Git checkpoint id captured immediately before this turn ran. Present
     *  only on `role === 'user'` messages. Drives the per-message
     *  "Restore to here" button in the chat. */
    checkpointId?: string;
    /** Short summary shown in the restore button tooltip / confirm dialog. */
    checkpointDescription?: string;
}

export interface DevWorkflowProject {
    id: string;
    name: string;
    currentPhase: string;
    progress: number;
    emulatorPreview: boolean;
    platform: 'ios' | 'android' | 'cross-platform';
}

export interface AttachedContext {
    type: 'attachment' | 'mention' | 'workflow' | 'file';
    id: string; // path or unique id
    name: string;
    data?: string;
    path?: string;
    thumbnail?: string;
}

export interface SemanticSlot {
    id: string;
    category: string;
    content: string;
    metadata?: any;
    timestamp: number;
}

export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    is_expanded?: boolean;
    children?: any[]; // Break type recursion for tsc performance
}

export interface McpServer {
    name: string;
    config: any;
}

export interface TerminalInstance {
    id: string;
    shell: string;
    parentGroupId: string;
}

export interface TerminalGroup {
    id: string;
    name: string;
    instances: string[]; // ids of terminal instances
    activeInstanceId: string;
    splitWeights?: number[]; // Percentages or relative weights
}

interface AppState {
    // Layout State
    isSidebarOpen: boolean;
    activeSidebarView: string;
    isBottomPanelOpen: boolean;
    activePanelTab: string;
    isRightSidebarOpen: boolean;
    theme: string;
    sidebarWidth: number;
    rightSidebarWidth: number;
    bottomPanelHeight: number;

    // Terminal State
    terminalGroups: any[];
    activeTerminalGroupId: string | null;

    // Editor State
    activeTabId: string | null;
    tabs: any[];
    fileTree: FileEntry[];
    aiStatus: 'alive' | 'dead';
    tokenUsage: number; // 0 to 100
    iconThemeMapping: any;
    agentMode: string;
    agentModel: string;
    activeRoot: string | null;
    activeEditorPath: string;
    activeRootName: string | null;
    activeDevice: string | null;
    emulators: string[];
    availableModels: any[];
    extensionContributions: any;
    mitmStatus: 'idle' | 'running' | 'error';
    mitmLogs: string[];
    mcpServers: any[];
    ollamaStatus: 'idle' | 'checking' | 'running' | 'error';
    llamaCppStatus: 'idle' | 'checking' | 'running' | 'error';
    agentMessages: any[];
    /** Last auto-checkpoint created at the start of an agent turn (Cursor-style restore). */
    lastAgentCheckpoint: { id: string; description: string; timestamp: number } | null;
    // List of file edits the agent performed during the current turn.
    // Populated from `ai-tool-call` events by the right sidebar; consumed
    // by the MultiFileReview panel that pops at end-of-turn so the user
    // can flip through the diffs and revert individual files.
    pendingAgentEdits: { path: string; tool: string; timestamp: number; preview?: string }[];
    isMultiFileReviewOpen: boolean;
    // ── Agent trajectory ────────────────────────────────────────────────
    // Persistent event log of what the agent did during the current
    // session (tool calls, content stream, errors). Powers the Antigravity-
    // style timeline panel that lets you replay a turn step-by-step.
    agentTrajectory: {
        id: string;
        ts: number;
        kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error';
        tool?: string;
        title: string;
        detail?: string;
        success?: boolean;
        turn?: number;
    }[];
    isTrajectoryOpen: boolean;
    currentTurnId: number;
    isMarkdownPreviewOpen: boolean;
    attachedContext: AttachedContext[];
    /** Independent background-agent runs that don't claim the main chat. */
    backgroundAgents: { id: string; prompt: string; status: 'pending' | 'running' | 'done' | 'error'; result: string; startedAt: number; finishedAt?: number }[];
    isAgentThinking: boolean;
    isAgentPaused: boolean;
    isYoloMode: boolean;
    isContinuousMode: boolean;   // 24/7 auto-loop: agent re-triggers itself until all tasks done
    // AIRI subsystem toggles persisted in localStorage so they survive reloads.
    airiVisionEnabled: boolean;
    airiVisionModel: string;
    airiConsciousnessEnabled: boolean;
    airiConsciousnessModel: string;
    agentUiMode: 'chat' | 'airi';
    ttsStrategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native';

    // ── Cursor-style IDE preferences ────────────────────────────────────
    // Each of these maps to a category in Settings → vscodium-rust
    // Settings. They're persisted in localStorage and read at boot. We
    // keep them flat (no nesting) so the settings page can wire each one
    // to a primitive `set*` action without ceremony.
    //
    // tabPredictionEnabled    — Cursor Tab inline AI completion master switch
    // tabMultilineSuggestions — whether tab predictions can span multiple lines
    // tabAcceptKey            — which key accepts a suggestion ('Tab' | 'Enter')
    // betaFastApply           — enable the experimental fast_apply tool in the catalog
    // betaSemanticSearch      — enable semantic_search tool in the catalog
    // betaShadowWorkspace     — preview edits in a shadow VFS before commit
    // networkProxyUrl         — http(s)://host:port for outbound IDE network calls
    // networkAllowInsecureTls — accept self-signed TLS for self-hosted endpoints
    // indexingEnabled         — let the context_indexer scan the workspace
    // indexingDocsUrls        — list of documentation URLs the agent can browse
    tabPredictionEnabled: boolean;
    tabMultilineSuggestions: boolean;
    tabAcceptKey: 'Tab' | 'Enter';
    betaFastApply: boolean;
    betaSemanticSearch: boolean;
    betaShadowWorkspace: boolean;
    aimVfsEnabled: boolean;
    thermalGovernorEnabled: boolean;
    jitDecompressionEnabled: boolean;
    jitThreshold: number;
    networkProxyUrl: string;
    networkAllowInsecureTls: boolean;
    indexingEnabled: boolean;
    indexingDocsUrls: string[];
    avatarCharacter: string; // Selected AI avatar character
    avatarCustomConfig?: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean }; // Custom 2D avatar URLs
    avatar3dConfig?: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> }; // 3D VRM avatar config
    showVrmAvatar: boolean;
    agentCurrentAction: string | null;
    isCommandPaletteOpen: boolean;
    isContextMenuOpen: boolean;
    isDebugToolbarOpen: boolean;
    isAgentBlocked: boolean;
    contextMenuPosition: { x: number, y: number };
    commandPaletteQuery: string;

    // Right sidebar panels (independent toggles)
    isAiriPanelOpen: boolean;
    isEmulatorPanelOpen: boolean;
    emulatorPanelPosition: 'android' | 'iphone';
    emulatorLayout: 'left' | 'right' | 'hidden';

    // Inference Backend Configuration
    inferenceBackend: 'ollama' | 'llama-cpp' | 'openai';
    llamaCppUrl: string;
    llamaCppModelPath: string;
    llamaCppNgl: number;
    llamaCppHadesEnabled: boolean;
    ollamaConnectionMode: 'proxy' | 'direct';  // proxy=1536 (AIM), direct=11434
    ollamaMode: 'local' | 'cloud' | 'auto';  // Hybrid backend mode
    // Cursor-style server-mode picker:
    //   local  → http://127.0.0.1:11434 (no questions asked)
    //   remote → user-supplied `customOllamaUrl`
    //   auto   → probe remote first; on failure fall back to local
    // The resolved/active URL lives in `ollamaUrl` (unchanged); these fields
    // are the user-facing config that drive what `ollamaUrl` gets set to.
    ollamaServerMode: 'local' | 'remote' | 'auto';
    customOllamaUrl: string;

    // Dev Workflow State
    isDevWorkflowActive: boolean;
    currentDevProject: DevWorkflowProject | null;
    emulatorPlatform: 'ios' | 'android';
    ollamaUrl: string;
    isPullingModel: boolean;
    pullProgress: number;
    pendingChanges: any[];
    autoAcceptChanges: boolean;
    checkpoint: Record<string, string> | null; // path → original content snapshot
    agentRootAccess: boolean;
    chatSessions: any[];
    brainTelemetry: any | null;
    attachedFiles: { id: string, path: string, name: string, gist?: string, thumbnail?: string, type: 'file' | 'attachment' | 'mention' }[];
    kairosSuggestions: any[];
    kairosStatus: 'idle' | 'indexing' | 'dreaming';
    webuiSessions: { session_id: string; provider: string; display_name: string; has_token: boolean; is_active: boolean }[];
    activeWebuiSessionId: string | null;

    // Phase 8: Agentic State
    taskPlannerState: any | null;
    ghostRuntimeResults: any[];
    currentThought: { logic: string, action: string, confidence?: number } | null;

    // Google Antigravity Expanded State
    layoutMode: 'editor' | 'manager' | 'browser';
    artifactReviewPolicy: 'always_proceed' | 'request_review';
    terminalAutoExecution: 'always_proceed' | 'request_review';
    agentThreads: Record<string, {
        id: string;
        name: string;
        messages: any[];
        isThinking: boolean;
        tasks: any[];
        artifacts: any[];
    }>;
    activeAgentThreadId: string;
    processStats: { memory_mb: number, cpu_usage: number, total_ram_gb: number, available_ram_gb: number } | null;
    memorySavings: { original: number, compressed: number } | null;
    contextSlots: SemanticSlot[];
    activeFileContext: {
        symbols: string[];
        related_files: string[];
        relevant_lessons: SemanticSlot[];
    } | null;
    activeProjectSpec: any | null;

    // Extension State
    installedExtensions: any[];
    marketExtensions: any[];
    popularExtensions: any[];
    isSearchingExtensions: boolean;
    extensionTrustRequest: { publisher: string, name: string, version: string, onResolve: (trusted: boolean) => void } | null;
    trustedPublishers: string[];
    selectedExtensionId: string | null;
    extensionDetails: Record<string, any>;

    // Project Memory (spec-kit / AGENTS.md / CLAUDE.md)
    projectMemory: string;
    memoryFiles: string[];

    // Agent Task Tracking
    agentTask: AgentTask | null;
    agentTasks: any[];
    agentFiles: string[];
    agentSteps: any[];
    currentPhase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE';
    currentPhaseStatus: string;

    // Visual Lab State
    visualLabMode: 'none' | 'json' | 'flow' | 'erd' | 'summary';
    visualLabData: any;
    isVisualLabFullScreen: boolean;
    isVisualLabOpen: boolean;
    isVisualLabSplitView: boolean;

    isAiriOpen: boolean;
    isComposerOpen: boolean;

    // Tab History (Alt+←/→)
    tabHistory: string[];
    tabHistoryIndex: number;
    navigateBack: () => void;
    navigateForward: () => void;

    // Split Editor
    splitEditorTabId: string | null;
    isSplitEditorOpen: boolean;
    setSplitEditorTab: (tabId: string | null) => void;
    toggleSplitEditor: () => void;

    // Specs-to-Code State
    isSpecsWizardOpen: boolean;
    specsWizardStep: 'generator' | 'status' | 'project';
    currentSpecProjectId: number | null;

    // LSP Diagnostics (for Problems panel)
    diagnosticsMap: Record<string, { severity: number; message: string; startLine: number; startCol: number; endLine: number; endCol: number; source: string; code: string }[]>;
    setDiagnosticsForUri: (uri: string, diags: any[]) => void;

    // Actions
    setVisualLabMode: (mode: 'none' | 'json' | 'flow' | 'erd' | 'summary') => void;
    setVisualLabData: (data: any) => void;
    setIsVisualLabFullScreen: (isFullScreen: boolean) => void;
    setIsVisualLabSplitView: (isSplit: boolean) => void;
    toggleVisualLab: (open?: boolean) => void;
    toggleAiri: (open?: boolean) => void;
    toggleComposer: (open?: boolean) => void;
    setLayoutMode: (mode: 'editor' | 'manager' | 'browser') => void;
    setArtifactReviewPolicy: (policy: 'always_proceed' | 'request_review') => void;
    setTerminalAutoExecution: (policy: 'always_proceed' | 'request_review') => void;
    createAgentThread: (name: string) => string;
    setActiveAgentThread: (id: string) => void;
    approveArtifact: (threadId: string, artifactId: string) => void;
    rejectArtifact: (threadId: string, artifactId: string) => void;
    setActiveSidebarView: (view: string) => void;
    toggleBottomPanel: () => void;
    setActivePanelTab: (tab: string) => void;
    toggleRightSidebar: () => void;
    setTheme: (theme: string) => void;
    setSidebarWidth: (width: number) => void;
    setRightSidebarWidth: (width: number) => void;
    setBottomPanelHeight: (height: number) => void;
    setFileTree: (tree: FileEntry[]) => void;
    setAiStatus: (status: 'alive' | 'dead') => void;
    setTokenUsage: (usage: number) => void;
    setIconThemeMapping: (mapping: any) => void;
    setAgentMode: (mode: string) => void;
    setAgentModel: (model: string) => void;
    setAgentRootAccess: (rootAccess: boolean) => void;
    setActiveRoot: (path: string | null) => void;
    setActiveEditorPath: (path: string) => void;
    setActiveDevice: (id: string | null) => void;
    setEmulators: (ems: string[]) => void;
    setAvatarCharacter: (avatarCharacter: string) => void;
    setAvatarCustomConfig: (config: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean }) => void;
    setAvatar3dConfig: (config: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> }) => void;
    setShowVrmAvatar: (show: boolean) => void;
    setExtensionContributions: (contributions: any) => void;
    refreshAvailableModels: (provider?: string) => Promise<void>;
    refreshFileTree: () => Promise<void>;
    toggleDirectory: (path: string) => Promise<void>;
    closeFolder: () => void;
    openFile: (path: string) => Promise<void>;
    closeTab: (id: string) => void;
    setActiveTab: (id: string) => void;
    updateTabContent: (id: string, content: string) => void;
    saveActiveFile: () => Promise<void>;
    setOllamaUrl: (url: string) => void;
    setOllamaConnectionMode: (mode: 'proxy' | 'direct') => void;
    setOllamaServerMode: (mode: 'local' | 'remote' | 'auto') => void;
    setCustomOllamaUrl: (url: string) => void;
    checkOllamaStatus: () => Promise<void>;
    pullOllamaModel: (name: string) => Promise<void>;
    setInferenceBackend: (backend: 'ollama' | 'llama-cpp' | 'openai') => void;
    setLlamaCppUrl: (url: string) => void;
    setLlamaCppModelPath: (path: string) => void;
    setLlamaCppNgl: (ngl: number) => void;
    setLlamaCppHadesEnabled: (enabled: boolean) => void;
    checkLlamaCppStatus: () => Promise<void>;
    openSettings: (tab?: 'user' | 'workspace' | 'agent') => void;
    setProjectMemory: (content: string, files?: string[]) => void;

    // Dev Workflow Actions
    setDevWorkflowActive: (active: boolean) => void;
    updateDevProject: (project: Partial<DevWorkflowProject>) => void;
    setEmulatorPlatform: (platform: 'ios' | 'android') => void;

    // Backend Actions
    backendPing: () => Promise<string>;
    startMitm: () => Promise<void>;
    stopMitm: () => Promise<void>;
    addMitmLog: (log: string) => void;
    addMcpServer: (name: string, config: any) => Promise<void>;
    removeMcpServer: (name: string) => Promise<void>;
    listMcpServers: () => Promise<void>;
    setMcpServerEnabled: (name: string, enabled: boolean) => Promise<void>;
    setLastAgentCheckpoint: (cp: { id: string; description: string; timestamp: number } | null) => void;
    rollbackLastAgentCheckpoint: () => Promise<{ ok: boolean; message: string }>;
    // Multi-file review setters — single source of truth for the diff
    // carousel so multiple components stay in sync.
    addPendingAgentEdit: (edit: { path: string; tool: string; preview?: string }) => void;
    clearPendingAgentEdits: () => void;
    openMultiFileReview: () => void;
    closeMultiFileReview: () => void;
    // Trajectory setters — used by RightSidebar's event listeners to log
    // each agent step, and by the trajectory panel to clear / replay.
    pushTrajectoryEvent: (evt: { kind: 'tool_call' | 'tool_result' | 'content' | 'phase' | 'error'; tool?: string; title: string; detail?: string; success?: boolean }) => void;
    clearTrajectory: () => void;
    openTrajectory: () => void;
    closeTrajectory: () => void;
    beginNewTurn: () => void;
    openMarkdownPreview: () => void;
    closeMarkdownPreview: () => void;
    toggleMarkdownPreview: () => void;
    /** Attach a Git checkpoint id to the most-recent `user` message in
     *  `agentMessages` so the chat can render a per-turn "Restore to here"
     *  button next to that message. */
    setLastUserMessageCheckpoint: (checkpointId: string, description?: string) => void;
    /** Restore the workspace to the checkpoint attached to a specific user
     *  message and truncate the chat to that turn so the conversation state
     *  matches the on-disk state. */
    restoreToMessageCheckpoint: (timestamp: number) => Promise<{ ok: boolean; message: string }>;
    runBackgroundAgent: (prompt: string) => Promise<string>;
    removeBackgroundAgent: (id: string) => void;
    clearBackgroundAgents: () => void;
    refreshProcessStats: () => Promise<void>;
    compressSessionData: (key: string, data: string) => Promise<void>;
    refreshMemorySavings: () => Promise<void>;
    addAgentMessage: (role: 'user' | 'assistant', content: string, context?: AttachedContext[] | boolean) => void;
    updateLastAgentMessage: (content: string) => void;
    appendLastAgentMessage: (delta: string) => void;
    updateLastAgentThought: (thought: string) => void;
    updateAgentStepStatus: (name: string, status: 'running' | 'success' | 'error', result?: string, summary?: string, callId?: string) => void;
    addAgentStep: (name: string, type?: AgentStep['type'], args?: any, callId?: string) => void;
    addAgentFile: (path: string) => void;
    addAgentArtifact: (artifact: Omit<Artifact, 'id' | 'timestamp'>) => void;
    setIsAgentThinking: (isThinking: boolean) => void;
    setIsAgentPaused: (paused: boolean) => void;
    setEmulatorPanelPosition: (pos: 'android' | 'iphone') => void;
    setEmulatorLayout: (layout: 'left' | 'right' | 'hidden') => void;
    setYoloMode: (enabled: boolean) => void;
    setContinuousMode: (enabled: boolean) => void;
    setAiriVisionEnabled: (enabled: boolean) => void;
    setAiriVisionModel: (model: string) => void;
    setAiriConsciousnessEnabled: (enabled: boolean) => void;
    setAiriConsciousnessModel: (model: string) => void;
    setAgentUiMode: (mode: 'chat' | 'airi') => void;
    setTtsStrategy: (strategy: 'elevenlabs' | 'openai' | 'browser' | 'qwen' | 'qwen-native') => void;
    // Cursor-style IDE preference setters — see the matching state fields
    // above for what each one controls.
    setTabPredictionEnabled: (v: boolean) => void;
    setTabMultilineSuggestions: (v: boolean) => void;
    setTabAcceptKey: (k: 'Tab' | 'Enter') => void;
    setBetaFastApply: (v: boolean) => void;
    setBetaSemanticSearch: (v: boolean) => void;
    setBetaShadowWorkspace: (v: boolean) => void;
    setAimVfsEnabled: (v: boolean) => void;
    setThermalGovernorEnabled: (v: boolean) => void;
    setJitDecompressionEnabled: (v: boolean) => void;
    setJitThreshold: (v: number) => void;
    setNetworkProxyUrl: (url: string) => void;
    setNetworkAllowInsecureTls: (v: boolean) => void;
    setIndexingEnabled: (v: boolean) => void;
    setIndexingDocsUrls: (urls: string[]) => void;
    setAgentCurrentAction: (action: string | null) => void;
    attachFile: (file: any | any[]) => void;
    removeFile: (path: string) => void;
    clearAttachedFiles: () => void;
    clearAgentMessages: () => void;
    resetThread: () => void;
    truncateAgentMessages: (index: number) => void;
    updateAgentTask: (task: Partial<AgentTask> & { id: string }) => void;
    setAgentBlocked: (blocked: boolean) => void;
    setAgentMessages: (messages: any[]) => void;
    setAgentTasks: (tasks: any[]) => void;
    setPhase: (phase: 'ANALYZE' | 'PLAN' | 'EXECUTE' | 'VERIFY' | 'REPORT' | 'IDLE', status: string) => void;

    // Diff Review Actions
    proposePendingChange: (change: Omit<PendingChange, 'id'>) => void;
    acceptPendingChange: (id: string) => Promise<void>;
    rejectPendingChange: (id: string) => void;
    acceptAllPendingChanges: () => Promise<void>;
    rejectAllPendingChanges: () => void;
    setAutoAcceptChanges: (v: boolean) => void;
    snapshotCheckpoint: (paths: string[]) => Promise<void>;
    revertToCheckpoint: () => Promise<void>;
    acceptHunk: (changeId: string, hunkId: string) => Promise<void>;
    rejectHunk: (changeId: string, hunkId: string) => Promise<void>;
    setCommandPaletteOpen: (open: boolean) => void;
    setContextMenuOpen: (open: boolean, x?: number, y?: number) => void;
    setDebugToolbarOpen: (open: boolean) => void;
    setCommandPaletteQuery: (query: string) => void;
    addAttachedContext: (item: AttachedContext) => void;
    removeAttachedContext: (index: number) => void;
    clearAttachedContext: () => void;

    // Terminal Actions
    addTerminalGroup: (shell?: string) => Promise<string>;
    addAiriActivityTerminal: () => Promise<string>;
    splitTerminal: (groupId: string, instanceId: string) => Promise<string>;
    closeTerminalInstance: (groupId: string, instanceId: string) => Promise<void>;
    setActiveTerminalGroup: (id: string) => void;
    setActiveTerminalInstance: (groupId: string, instanceId: string) => void;
    renameTerminalGroup: (groupId: string, name: string) => void;
    closeTerminalGroup: (groupId: string) => Promise<void>;
    updateTerminalSplitWeights: (groupId: string, weights: number[]) => void;

    // Agent Tasks
    setAgentTask: (task: AgentTask | null) => void;
    setAgentFiles: (files: string[]) => void;
    setAgentSteps: (steps: AgentStep[]) => void;

    // Extension Actions
    setInstalledExtensions: (exts: any[]) => void;
    setMarketExtensions: (exts: any[]) => void;
    setSearchingExtensions: (searching: boolean) => void;
    refreshInstalledExtensions: () => Promise<void>;
    addInstalledExtension: (extension: any) => void;
    refreshPopularExtensions: () => Promise<void>;
    searchExtensions: (query: string) => Promise<void>;
    requestExtensionTrust: (publisher: string, name: string, version: string) => Promise<boolean>;
    resolveExtensionTrust: (trusted: boolean, always?: boolean) => void;
    addTrustedPublisher: (publisher: string) => void;
    removeTrustedPublisher: (publisher: string) => void;
    setSelectedExtensionId: (id: string | null) => void;
    fetchExtensionDetails: (id: string) => Promise<void>;
    installExtension: (publisher: string, name: string, version: string) => Promise<boolean>;
    uninstallExtension: (publisher: string, name: string, version?: string) => Promise<boolean>;
    fetchWorkspaceMemory: (category: string) => Promise<void>;
    fetchFileContext: (path: string) => Promise<void>;
    fetchActiveProjectSpec: () => Promise<void>;
    getFlattenedFiles: () => FileEntry[];

    setSpecsWizardOpen: (open: boolean) => void;
    setSpecsWizardStep: (step: 'generator' | 'status' | 'project') => void;
    setCurrentSpecProjectId: (id: number | null) => void;
    refreshChatSessions: () => Promise<void>;
    loadChatSession: (path: string) => Promise<void>;
    archiveCurrentSession: () => Promise<void>;
    createNewSession: () => Promise<void>;
    refreshBrainTelemetry: () => Promise<void>;
    addKairosSuggestion: (suggestion: any) => void;
    refreshWebuiSessions: (provider?: string) => Promise<void>;
    switchWebuiSession: (sessionId: string) => Promise<void>;
    deleteWebuiSession: (sessionId: string) => Promise<void>;

    // Right sidebar panel actions
    toggleAiriPanel: () => void;
    toggleEmulatorPanel: () => void;
    openAiriPanel: () => void;
    closeAiriPanel: () => void;
    openEmulatorPanel: () => void;
    closeEmulatorPanel: () => void;

    // ── Void-style per-feature model selection ────────────────────────────
    modelSelectionOfFeature: ModelSelectionOfFeature;
    modelSelectionOptions: Partial<Record<FeatureName, Partial<Record<ProviderName, Record<string, ModelSelectionOptions>>>>>;
    setModelSelectionForFeature: (feature: FeatureName, selection: ModelSelection | null) => void;
    setModelSelectionOptions: (feature: FeatureName, providerName: ProviderName, modelName: string, opts: ModelSelectionOptions) => void;

    // ── Void-style global settings ────────────────────────────────────────
    voidGlobalSettings: GlobalSettings;
    setVoidGlobalSetting: <K extends keyof GlobalSettings>(key: K, value: GlobalSettings[K]) => void;

    // ── Reasoning state (per active chat turn) ────────────────────────────
    currentReasoningBudget: number;
    currentReasoningEffort: string;
    isReasoningEnabled: boolean;
    setReasoningBudget: (v: number) => void;
    setReasoningEffort: (v: string) => void;
    setReasoningEnabled: (v: boolean) => void;

    // ── Extra provider settings (vLLM, LM Studio, liteLLM, Vertex, Azure, Bedrock) ──
    vllmUrl: string;
    lmStudioUrl: string;
    liteLLMUrl: string;
    liteLLMApiKey: string;
    googleVertexProject: string;
    googleVertexRegion: string;
    azureProject: string;
    azureApiKey: string;
    azureApiVersion: string;
    awsBedrockApiKey: string;
    awsBedrockRegion: string;
    awsBedrockEndpoint: string;
    setVllmUrl: (url: string) => void;
    setLmStudioUrl: (url: string) => void;
    setLiteLLMUrl: (url: string) => void;
    setLiteLLMApiKey: (k: string) => void;
    setGoogleVertexProject: (v: string) => void;
    setGoogleVertexRegion: (v: string) => void;
    setAzureProject: (v: string) => void;
    setAzureApiKey: (k: string) => void;
    setAzureApiVersion: (v: string) => void;
    setAwsBedrockApiKey: (k: string) => void;
    setAwsBedrockRegion: (v: string) => void;
    setAwsBedrockEndpoint: (v: string) => void;

    // ── Antigravity: Workflow/Rule editor state ───────────────────────────
    openWorkflowFiles: string[];
    openRuleFiles: string[];
    openWorkflowFile: (path: string) => void;
    closeWorkflowFile: (path: string) => void;
    openRuleFile: (path: string) => void;
    closeRuleFile: (path: string) => void;

    // ── Antigravity: Agent hunk navigation ───────────────────────────────
    focusedHunkId: string | null;
    setFocusedHunk: (id: string | null) => void;
    acceptFocusedHunk: () => void;
    rejectFocusedHunk: () => void;

    // ── Antigravity: AI commit message ────────────────────────────────────
    isGeneratingCommitMessage: boolean;
    lastGeneratedCommitMessage: string;
    generateAiCommitMessage: () => Promise<string>;

    // ── Antigravity: Import settings ─────────────────────────────────────
    importFromEditor: (source: 'vscode' | 'cursor' | 'windsurf' | 'cider') => Promise<void>;

    // ── Antigravity: Demo mode ────────────────────────────────────────────
    isDemoMode: boolean;
    startDemoMode: () => void;
    endDemoMode: () => void;
}

export interface AgentTask {
    id: string;
    title: string;
    summary: string;
    message?: string; // Progress or details
    status: 'running' | 'completed' | 'error' | 'pending' | 'failed' | 'blocked';
    progress: number;
    createdAt: number;
    updatedAt: number;
    artifacts: Artifact[];
    mode?: string;
    task_status?: string;
}

function detectLanguage(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase() ?? '';
    const map: Record<string, string> = {
        rs: 'rust', ts: 'typescript', tsx: 'typescript', js: 'javascript',
        jsx: 'javascript', json: 'json', css: 'css', html: 'html',
        md: 'markdown', toml: 'toml', yaml: 'yaml', yml: 'yaml',
        sh: 'shell', py: 'python', go: 'go', c: 'c', cpp: 'cpp',
        h: 'c', hpp: 'cpp', txt: 'plaintext',
    };
    return map[ext] ?? 'plaintext';
}

const storeImplementation: any = (set: any, get: any) => ({
    // Initial Layout State
    isSidebarOpen: true,
    activeSidebarView: 'explorer-view',
    // Start with a clean workspace: panel closed by default
    isBottomPanelOpen: false,
    activePanelTab: 'TERMINAL',
    isRightSidebarOpen: false,
    theme: localStorage.getItem('active-monaco-theme') || 'vs-dark',
    kairosSuggestions: [],
    kairosStatus: 'idle',
    sidebarWidth: parseInt(localStorage.getItem('sidebarWidth') || '260'),
    rightSidebarWidth: parseInt(localStorage.getItem('rightSidebarWidth') || '300'),
    bottomPanelHeight: parseInt(localStorage.getItem('bottomPanelHeight') || '240'),

    // Initial Editor State
    activeTabId: null,
    tabs: [],
    fileTree: [],
    aiStatus: 'alive',
    tokenUsage: 0,
    iconThemeMapping: null,
    // Default to ACTION mode so the agent actually writes files, runs
    // commands and ships PoCs — critical for cybersecurity/bug-bounty work.
    // Chat mode forbids tool calls, which was the previous (broken) default.
    agentMode: (typeof localStorage !== 'undefined' && localStorage.getItem('agent.mode')) || 'Harness',
    agentModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('agentModel') || '';
        const oldDefaults = new Set([
            'Ollama|airi-fast:latest',
            'Ollama|qwen3:35b',
            'qwen3:35b',
            'huihui_ai/qwen2.5-coder-abliterate:7b',
            'Ollama|huihui_ai/qwen2.5-coder-abliterate:7b',
        ]);
        if (oldDefaults.has(saved)) {
            localStorage.removeItem('agentModel');
            return '';
        }
        return saved;
    })(),
    trustedPublishers: JSON.parse(localStorage.getItem('trustedPublishers') || '[]'),
    activeRoot: localStorage.getItem('activeRoot') || null,
    activeEditorPath: '',
    activeRootName: localStorage.getItem('activeRootName') || null,
    activeDevice: null,
    emulators: [],
    availableModels: [],
    extensionContributions: {
        viewsContainers: { activitybar: [] },
        views: {}
    },
    mitmStatus: 'idle',
    mitmLogs: [],
    mcpServers: [],
    ollamaStatus: 'idle',
    llamaCppStatus: 'idle',
    agentMessages: [],
    lastAgentCheckpoint: null,
    pendingAgentEdits: [],
    isMultiFileReviewOpen: false,
    agentTrajectory: [],
    isTrajectoryOpen: false,
    currentTurnId: 0,
    isMarkdownPreviewOpen: false,
    backgroundAgents: [],
    isAgentThinking: false,
    isAgentPaused: false,
    isYoloMode: false,
    isContinuousMode: false,
    // Default local subsystems OFF/empty. They become available only after the
    // user selects models in Settings, so startup never silently loads a 35B/VL
    // model or burns GPU on consumer machines.
    airiVisionEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('airi.vision.enabled') === '1') || false,
    airiVisionModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('airi.vision.model') || '';
        if (saved === 'qwen2.5vl:72b') {
            localStorage.removeItem('airi.vision.model');
            return '';
        }
        return saved;
    })(),
    airiConsciousnessEnabled: (typeof localStorage !== 'undefined' && localStorage.getItem('airi.consciousness.enabled') === '1') || false,
    airiConsciousnessModel: (() => {
        if (typeof localStorage === 'undefined') return '';
        const saved = localStorage.getItem('airi.consciousness.model') || '';
        if (saved === 'airi-fast:latest') {
            localStorage.removeItem('airi.consciousness.model');
            return '';
        }
        return saved;
    })(),
    agentUiMode: (localStorage.getItem('agentUiMode') as 'chat' | 'airi') || 'chat',
    ttsStrategy: (localStorage.getItem('ttsStrategy') as any) || 'elevenlabs',
    // ── Cursor-style IDE preference defaults ──────────────────────────
    // Tab predictions / fast_apply / shadow workspace default ON because
    // the existing agent loop already references those tools when they
    // exist. Toggling them OFF gives the user a way to opt out without
    // editing source. Proxy + insecure TLS default empty/off so we don't
    // surprise users who never configured a proxy.
    tabPredictionEnabled: (() => { try { return localStorage.getItem('tab.predictionEnabled') !== '0'; } catch { return true; } })(),
    tabMultilineSuggestions: (() => { try { return localStorage.getItem('tab.multilineSuggestions') !== '0'; } catch { return true; } })(),
    tabAcceptKey: (() => { try { return (localStorage.getItem('tab.acceptKey') as 'Tab' | 'Enter') || 'Tab'; } catch { return 'Tab' as const; } })(),
    betaFastApply: (() => { try { return localStorage.getItem('beta.fastApply') !== '0'; } catch { return true; } })(),
    betaSemanticSearch: (() => { try { return localStorage.getItem('beta.semanticSearch') !== '0'; } catch { return true; } })(),
    betaShadowWorkspace: (() => { try { return localStorage.getItem('beta.shadowWorkspace') === '1'; } catch { return false; } })(),
    networkProxyUrl: (() => { try { return localStorage.getItem('network.proxyUrl') || ''; } catch { return ''; } })(),
    networkAllowInsecureTls: (() => { try { return localStorage.getItem('network.allowInsecureTls') === '1'; } catch { return false; } })(),
    indexingEnabled: (() => { try { return localStorage.getItem('indexing.enabled') !== '0'; } catch { return true; } })(),
    indexingDocsUrls: (() => {
        try {
            const raw = localStorage.getItem('indexing.docsUrls');
            if (!raw) return [];
            const arr = JSON.parse(raw);
            return Array.isArray(arr) ? arr.filter((s: any) => typeof s === 'string') : [];
        } catch { return []; }
    })(),
    avatarCharacter: localStorage.getItem('avatarCharacter') || 'airi',
    avatarCustomConfig: JSON.parse(localStorage.getItem('avatarCustomConfig') || '{}'),
    avatar3dConfig: JSON.parse(localStorage.getItem('avatar3dConfig') || '{}'),
    showVrmAvatar: (() => {
        try {
            return localStorage.getItem('showVrmAvatar') === 'true';
        } catch {
            return false;
        }
    })(),
    ollamaConnectionMode: (localStorage.getItem('ollamaConnectionMode') as 'proxy' | 'direct') || 'proxy',
    ollamaMode: (localStorage.getItem('ollamaMode') as 'local' | 'cloud' | 'auto') || 'auto',
    ollamaServerMode: (() => {
        try {
            return (localStorage.getItem('ollamaServerMode') as 'local' | 'remote' | 'auto') || 'local';
        } catch { return 'local'; }
    })(),
    customOllamaUrl: (() => {
        try { return localStorage.getItem('customOllamaUrl') || ''; } catch { return ''; }
    })(),

    // Right sidebar panels
    isAiriPanelOpen: true,  // AIRI open by default
    isEmulatorPanelOpen: false,  // Emulator closed by default
    emulatorPanelPosition: 'android',
    emulatorLayout: (localStorage.getItem('emulatorLayout') as 'left' | 'right' | 'hidden') || 'left',

    // Inference Backend Configuration
    inferenceBackend: (localStorage.getItem('inferenceBackend') as 'ollama' | 'llama-cpp' | 'openai') || 'ollama',  // Default to Ollama (GPU-accelerated via DirectML)
    llamaCppUrl: localStorage.getItem('llamaCppUrl') || 'http://localhost:8081',
    llamaCppModelPath: localStorage.getItem('llamaCppModelPath') || '',
    llamaCppNgl: parseInt(localStorage.getItem('llamaCppNgl') || '99'),
    llamaCppHadesEnabled: localStorage.getItem('llamaCppHadesEnabled') !== 'false',

    // HADES Intelligence Layer (works with any backend)
    aimVfsEnabled: localStorage.getItem('aimVfsEnabled') !== 'false',  // .aim VFS context injection
    thermalGovernorEnabled: localStorage.getItem('thermalGovernorEnabled') !== 'false',  // RX 580 thermal monitoring
    jitDecompressionEnabled: localStorage.getItem('jitDecompressionEnabled') === 'true',  // JIT code inflation
    jitThreshold: parseFloat(localStorage.getItem('jitThreshold') || '0.85'),  // Attention trigger threshold

    agentCurrentAction: null,
    isCommandPaletteOpen: false,
    isContextMenuOpen: false,
    isDebugToolbarOpen: false,
    isAgentBlocked: false,
    contextMenuPosition: { x: 0, y: 0 },
    commandPaletteQuery: '',

    // Dev Workflow State (initial values)
    isDevWorkflowActive: false,
    currentDevProject: null,
    emulatorPlatform: 'ios',

    // Default to local Ollama. Users can switch to a hosted endpoint from the
    // settings UI; we no longer pin a dead cloud host here.
    ollamaUrl: normalizeOllamaUrl(readStoredOllamaUrl()),
    isPullingModel: false,
    pullProgress: 0,
    pendingChanges: [],
    agentRootAccess: true,
    processStats: null,
    memorySavings: null,
    contextSlots: [],
    activeFileContext: null,
    activeProjectSpec: null,
    chatSessions: [],
    brainTelemetry: null,
    attachedFiles: [],
    attachedContext: [],
    taskPlannerState: null,
    ghostRuntimeResults: [],
    currentThought: null,
    webuiSessions: [],
    activeWebuiSessionId: null,

    // Terminal Initial State
    terminalGroups: [],
    activeTerminalGroupId: null,

    // Project Memory
    projectMemory: '',
    memoryFiles: [],

    // Agent Tasks
    agentTask: null,
    agentTasks: [],
    agentFiles: [],
    agentSteps: [],
    currentPhase: 'IDLE',
    currentPhaseStatus: 'Waiting for task...',

    // Visual Lab Initial State
    visualLabMode: 'none',
    visualLabData: null,
    isVisualLabFullScreen: false,
    isVisualLabOpen: false,
    isVisualLabSplitView: false,

    isAiriOpen: true, // Default to true for the sentient oracle experience

    // Tab History
    tabHistory: [],
    tabHistoryIndex: -1,

    // Split Editor
    splitEditorTabId: null,
    isSplitEditorOpen: false,

    // Initial Specs-to-Code State
    isSpecsWizardOpen: false,
    specsWizardStep: 'generator',
    currentSpecProjectId: null,

    // LSP Diagnostics
    diagnosticsMap: {},
    setDiagnosticsForUri: (uri: string, diags: any[]) => set(state => {
        const severityMap: Record<number, number> = { 1: 8, 2: 4, 3: 2, 4: 1 };
        const mapped = diags.map((d: any) => ({
            severity: severityMap[d.severity ?? 1] ?? 8,
            message: d.message ?? '',
            startLine: (d.range?.start?.line ?? 0) + 1,
            startCol: (d.range?.start?.character ?? 0) + 1,
            endLine: (d.range?.end?.line ?? 0) + 1,
            endCol: (d.range?.end?.character ?? 0) + 1,
            source: d.source ?? 'lsp',
            code: d.code?.toString() ?? '',
        }));
        // Use path as key (strip file:// prefix)
        const key = uri.replace(/^file:\/\/\//, '').replace(/^file:\/\//, '').replace(/\//g, '\\');
        return { diagnosticsMap: { ...state.diagnosticsMap, [key]: mapped } };
    }),

    // Initial Extension State
    installedExtensions: [],
    marketExtensions: [],
    popularExtensions: [],
    isSearchingExtensions: false,
    extensionTrustRequest: null,
    extensionDetails: {},
    layoutMode: 'editor',
    artifactReviewPolicy: 'request_review',
    terminalAutoExecution: 'request_review',
    autoAcceptChanges: false,
    checkpoint: null,
    agentThreads: {},
    activeAgentThreadId: '',

    // Actions
    setVisualLabMode: (mode: any) => set({ visualLabMode: mode }),
    setVisualLabData: (data: any) => set({ visualLabData: data }),
    setIsVisualLabFullScreen: (isFullScreen: boolean) => set({ isVisualLabFullScreen: isFullScreen }),
    setIsVisualLabSplitView: (isSplit: boolean) => set({ isVisualLabSplitView: isSplit }),
    toggleVisualLab: (open: any) => set((state: any) => ({ isVisualLabOpen: open !== undefined ? open : !state.isVisualLabOpen, isVisualLabSplitView: false })),
    toggleAiri: (open: any) => set((state: any) => ({ isAiriOpen: open !== undefined ? open : !state.isAiriOpen })),
    setProjectMemory: (content: any, files = []) => set(() => ({ projectMemory: content, memoryFiles: files })),
    toggleSidebar: () => set((state) => ({ isSidebarOpen: !state.isSidebarOpen })),
    setActiveSidebarView: (view) => set(() => ({ activeSidebarView: view, isSidebarOpen: true })),
    toggleBottomPanel: () => set((state) => ({ isBottomPanelOpen: !state.isBottomPanelOpen })),
    setActivePanelTab: (tab) => set(() => ({ activePanelTab: tab, isBottomPanelOpen: true })),
    toggleRightSidebar: () => {
        console.trace('[DIAG] toggleRightSidebar called — full call stack above');
        set((state) => {
            console.log('[DIAG] isRightSidebarOpen:', state.isRightSidebarOpen, '→', !state.isRightSidebarOpen);
            return { isRightSidebarOpen: !state.isRightSidebarOpen };
        });
    },
    setTheme: (theme) => {
        set({ theme });
        localStorage.setItem('active-monaco-theme', theme);
        if (['vs', 'vs-dark', 'hc-black'].includes(theme)) {
            localStorage.removeItem('active-theme-path');
        }
    },
    setSidebarWidth: (sidebarWidth) => {
        localStorage.setItem('sidebarWidth', sidebarWidth.toString());
        set({ sidebarWidth });
    },
    setRightSidebarWidth: (rightSidebarWidth) => {
        localStorage.setItem('rightSidebarWidth', rightSidebarWidth.toString());
        set({ rightSidebarWidth });
    },
    setBottomPanelHeight: (bottomPanelHeight) => {
        localStorage.setItem('bottomPanelHeight', bottomPanelHeight.toString());
        set({ bottomPanelHeight });
    },
    setFileTree: (tree) => set({ fileTree: tree }),
    setAiStatus: (aiStatus) => set({ aiStatus }),
    setTokenUsage: (tokenUsage) => set({ tokenUsage }),
    setIconThemeMapping: (iconThemeMapping) => set({ iconThemeMapping }),
    setAgentMode: (agentMode) => {
        try { localStorage.setItem('agent.mode', agentMode); } catch { /* quota */ }
        set({ agentMode });
    },
    setAgentModel: (agentModel) => {
        try {
            localStorage.setItem('agentModel', agentModel);
        } catch {
            /* quota / tracking prevention */
        }
        set({ agentModel });
    },
    setAgentRootAccess: (_rootAccess: boolean) => {
        // Root access is now permanent and cannot be disabled
        set({ agentRootAccess: true });
    },
    setActiveRoot: (path) => {
        // Defensive cleanup: a corrupted localStorage entry once stored a
        // path with a trailing NUL byte, which then poisoned CreateProcessW
        // and broke every new PTY. Always normalise before persisting.
        const cleaned = (path ?? '').split('\0')[0].trim();
        if (cleaned) {
            const name = cleaned.replace(/\\/g, '/').split('/').pop() || cleaned;
            localStorage.setItem('activeRoot', cleaned);
            localStorage.setItem('activeRootName', name);
            set({ activeRoot: cleaned, activeRootName: name });
            invoke('set_active_root', { path: cleaned })
                .then(() => {
                    get().refreshFileTree();
                    get().fetchActiveProjectSpec();
                })
                .catch((err) => {
                    // Backend rejected the path (e.g. it no longer exists).
                    // Surface it instead of leaving a ghost root that the
                    // terminal subsystem will then choke on.
                    console.warn('[store] set_active_root rejected:', err);
                    localStorage.removeItem('activeRoot');
                    localStorage.removeItem('activeRootName');
                    set({ activeRoot: null, activeRootName: null, fileTree: [], activeProjectSpec: null });
                });
        } else {
            localStorage.removeItem('activeRoot');
            localStorage.removeItem('activeRootName');
            invoke('set_active_root', { path: null }).catch(console.error);
            set({ activeRoot: null, activeRootName: null, fileTree: [], activeProjectSpec: null });
        }
    },
    setActiveDevice: (activeDevice) => set({ activeDevice }),
    setEmulators: (emulators) => set({ emulators }),
    setExtensionContributions: (extensionContributions) => set({ extensionContributions }),
    setLayoutMode: (mode) => set({ layoutMode: mode }),
    setArtifactReviewPolicy: (policy) => set({ artifactReviewPolicy: policy }),
    setTerminalAutoExecution: (policy) => set({ terminalAutoExecution: policy }),
    setActiveAgentThreadId: (activeAgentThreadId) => set({ activeAgentThreadId }),
    createAgentThread: (name: string) => {
        const id = `agent-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
        set((state: any) => ({
            agentThreads: {
                ...state.agentThreads,
                [id]: {
                    id,
                    name,
                    messages: [],
                    isThinking: false,
                    tasks: [],
                    artifacts: [],
                }
            },
            activeAgentThreadId: id,
            agentMessages: [], // Reset active view for the new thread
        }));
        return id;
    },
    fetchActiveProjectSpec: async () => {
        try {
            const projects: any = await invoke('cmd_specs_get_projects');
            const activeRoot = get().activeRoot;
            if (activeRoot && projects) {
                const activeSpec = projects.find((p: any) => p.root_path === activeRoot || p.path === activeRoot);
                set({ activeProjectSpec: activeSpec || null });
            }
        } catch (error) {
            console.error('Failed to fetch active project spec:', error);
        }
    },
    setActiveAgentThread: (id: string) => {
        set((state: any) => {
            const thread = state.agentThreads[id];
            if (!thread) return state;
            return {
                activeAgentThreadId: id,
                agentMessages: thread.messages,
                agentTasks: thread.tasks,
            };
        });
    },
    approveArtifact: (threadId: string, artifactId: string) => {
        set((state: any) => {
            const thread = state.agentThreads[threadId];
            if (!thread) return state;
            const updatedArtifacts = thread.artifacts.map((a: any) =>
                a.id === artifactId ? { ...a, metadata: { ...a.metadata, reviewed: true, status: 'approved' } } : a
            );
            return {
                agentThreads: {
                    ...state.agentThreads,
                    [threadId]: { ...thread, artifacts: updatedArtifacts }
                }
            };
        });
    },
    rejectArtifact: (threadId: string, artifactId: string) => {
        set((state: any) => {
            const thread = state.agentThreads[threadId];
            if (!thread) return state;
            const updatedArtifacts = thread.artifacts.map((a: any) =>
                a.id === artifactId ? { ...a, metadata: { ...a.metadata, reviewed: true, status: 'rejected' } } : a
            );
            return {
                agentThreads: {
                    ...state.agentThreads,
                    [threadId]: { ...thread, artifacts: updatedArtifacts }
                }
            };
        });
    },
    setOllamaUrl: (url: string) => {
        const normalized = normalizeOllamaUrl(url);
        set({ ollamaUrl: normalized });
        try {
            localStorage.setItem('ollamaUrl', normalized);
        } catch {
            /* quota / private mode / Tracking Prevention */
        }
        invoke('set_ollama_url', { url: normalized }).catch(console.error);
    },
    setOllamaConnectionMode: (mode: 'proxy' | 'direct') => {
        // The two toggle buttons are EXPLICIT port presets — clicking
        // them MUST change the backend URL to the matching port. The
        // previous version reused whatever was in localStorage, which
        // meant switching to "Direct Ollama (11434)" silently kept a
        // stale VPS URL alive and every subsequent /api/tags call timed
        // out against a host that was no longer running. Now the toggle
        // is canonical: proxy → 1536 (AIM), direct → 11434 (raw Ollama).
        // Users who want a custom host edit the textbox; the textbox
        // calls `setOllamaUrl` which doesn't touch the mode.
        const PROXY_DEFAULT = 'http://127.0.0.1:1536';
        const DIRECT_DEFAULT = 'http://127.0.0.1:11434';
        const url = mode === 'proxy' ? PROXY_DEFAULT : DIRECT_DEFAULT;

        set({ ollamaConnectionMode: mode, ollamaUrl: url });

        // Push the URL into the Rust engine immediately, then re-list
        // models so the picker reflects the new endpoint without the
        // user having to hit Reconnect.
        (async () => {
            try {
                await invoke('set_ollama_url', { url });
            } catch (err) {
                console.error('[ollama] set_ollama_url failed:', err);
            }
            try {
                await get().refreshAvailableModels?.('ollama');
            } catch { /* surface via the green/red dot, no toast spam */ }
            try {
                await get().checkOllamaStatus?.();
            } catch { /* ditto */ }
        })();

        try {
            localStorage.setItem('ollamaConnectionMode', mode);
            localStorage.setItem('ollamaUrl', url);
        } catch {
            /* quota / private mode / Tracking Prevention — fine, it'll
             * fall back to the in-memory `set` above. */
        }
    },
    // ── Cursor-style server mode picker ─────────────────────────────────
    // `setOllamaServerMode` is the top-level selector the user clicks in
    // Settings → Ollama. It resolves to the concrete URL (`local`,
    // `remote`, or `auto`), pushes that URL into the Rust engine via
    // `set_ollama_url`, then re-lists models so the picker is in sync.
    //
    // - local  → http://127.0.0.1:11434 (raw Ollama)
    // - remote → customOllamaUrl (whatever the user typed in the input)
    // - auto   → probe customOllamaUrl /api/tags; on success use remote,
    //            else fall back to local. Same idea as Cursor's "Auto"
    //            balanced mode.
    setOllamaServerMode: (mode: 'local' | 'remote' | 'auto') => {
        const LOCAL = 'http://127.0.0.1:11434';
        const custom = (get().customOllamaUrl || '').trim();
        const remote = custom ? normalizeOllamaUrl(custom) : '';

        // Resolve to a concrete URL synchronously for `local` / `remote`;
        // `auto` needs an async probe so we set local first and upgrade
        // to remote in the background if it pings back.
        const resolveAuto = async (): Promise<string> => {
            if (!remote) return LOCAL;
            try {
                const ctrl = new AbortController();
                const t = setTimeout(() => ctrl.abort(), 3000);
                const r = await fetch(`${remote.replace(/\/$/, '')}/api/tags`, {
                    method: 'GET',
                    signal: ctrl.signal,
                });
                clearTimeout(t);
                if (r.ok || r.status === 401) return remote; // 401 = auth needed but reachable
            } catch { /* unreachable */ }
            return LOCAL;
        };

        let initialUrl = LOCAL;
        if (mode === 'remote' && remote) initialUrl = remote;

        set({ ollamaServerMode: mode, ollamaUrl: initialUrl });
        try {
            localStorage.setItem('ollamaServerMode', mode);
            localStorage.setItem('ollamaUrl', initialUrl);
        } catch { /* tracking prevention */ }

        (async () => {
            let finalUrl = initialUrl;
            if (mode === 'auto') {
                finalUrl = await resolveAuto();
                if (finalUrl !== initialUrl) {
                    set({ ollamaUrl: finalUrl });
                    try { localStorage.setItem('ollamaUrl', finalUrl); } catch { /* ignore */ }
                }
            }
            try { await invoke('set_ollama_url', { url: finalUrl }); }
            catch (err) { console.error('[ollama] set_ollama_url failed:', err); }
            try { await get().refreshAvailableModels?.('ollama'); } catch { /* surfaced via status dot */ }
            try { await get().checkOllamaStatus?.(); } catch { /* same */ }
        })();
    },
    setCustomOllamaUrl: (url: string) => {
        const trimmed = url.trim();
        set({ customOllamaUrl: trimmed });
        try { localStorage.setItem('customOllamaUrl', trimmed); } catch { /* ignore */ }
        // If we're currently in remote/auto mode, re-resolve so the URL
        // change takes effect immediately without a second click.
        const mode = get().ollamaServerMode;
        if (mode === 'remote' || mode === 'auto') {
            get().setOllamaServerMode?.(mode);
        }
    },
    setDevWorkflowActive: (active: boolean) => {
        set({ isDevWorkflowActive: active });
    },
    updateDevProject: (project: Partial<DevWorkflowProject>) => {
        set((state) => ({
            currentDevProject: state.currentDevProject
                ? { ...state.currentDevProject, ...project }
                : { id: `dev_${Date.now()}`, name: 'New App', currentPhase: 'requirements', progress: 0, emulatorPreview: true, platform: 'cross-platform', ...project },
        }));
    },
    setEmulatorPlatform: (platform: 'ios' | 'android') => {
        set({ emulatorPlatform: platform });
    },
    checkOllamaStatus: async () => {
        set({ ollamaStatus: 'checking' });
        try {
            const isRunning = await invoke<boolean>('check_ollama_status');
            set({ ollamaStatus: isRunning ? 'running' : 'error' });
        } catch (e) {
            set({ ollamaStatus: 'error' });
        }
    },
    pullOllamaModel: async (name: string) => {
        set({ isPullingModel: true, pullProgress: 0 });
        try {
            await invoke('pull_ollama_model', { name });
            get().refreshAvailableModels('ollama');
        } catch (e) {
            console.error('Failed to pull model:', e);
        } finally {
            set({ isPullingModel: false });
        }
    },
    setInferenceBackend: (backend: 'ollama' | 'llama-cpp' | 'openai') => {
        localStorage.setItem('inferenceBackend', backend);
        set({ inferenceBackend: backend });
        const st = get();
        // Keep Rust EditorState.ollama_url in sync so `ai_inline_complete` and
        // other commands that read `state.ollama_url` hit the same endpoint as
        // the agent chat loop (`ai_chat` passes `ollama_url` in the request,
        // but inline completion uses the mutex on AiEngine).
        if (backend === 'llama-cpp') {
            import('./tauri_bridge').then(({ invoke }) =>
                invoke('set_ollama_url', { url: st.llamaCppUrl }).catch(() => { })
            );
        } else if (backend === 'ollama') {
            import('./tauri_bridge').then(({ invoke }) =>
                invoke('set_ollama_url', { url: st.ollamaUrl }).catch(() => { })
            );
        }
    },
    setLlamaCppUrl: (url: string) => {
        localStorage.setItem('llamaCppUrl', url);
        set({ llamaCppUrl: url });
        if (get().inferenceBackend === 'llama-cpp') {
            import('./tauri_bridge').then(({ invoke }) =>
                invoke('set_ollama_url', { url }).catch(() => { })
            );
        }
    },
    setLlamaCppModelPath: (path: string) => {
        localStorage.setItem('llamaCppModelPath', path);
        set({ llamaCppModelPath: path });
    },
    setLlamaCppNgl: (ngl: number) => {
        localStorage.setItem('llamaCppNgl', ngl.toString());
        set({ llamaCppNgl: ngl });
    },
    setLlamaCppHadesEnabled: (enabled: boolean) => {
        localStorage.setItem('llamaCppHadesEnabled', enabled.toString());
        set({ llamaCppHadesEnabled: enabled });
    },
    // Right sidebar panel toggles
    toggleAiriPanel: () => set((state) => ({
        isRightSidebarOpen: true,  // Open sidebar when toggling AIRI
        isAiriPanelOpen: !state.isAiriPanelOpen,
        isEmulatorPanelOpen: false  // Close emulator when opening AIRI
    })),
    toggleEmulatorPanel: () => set((state) => ({
        isRightSidebarOpen: true,  // Open sidebar when toggling Emulator
        isEmulatorPanelOpen: !state.isEmulatorPanelOpen,
        isAiriPanelOpen: false  // Close AIRI when opening emulator
    })),
    setEmulatorPanelPosition: (position: 'android' | 'iphone') => set({ emulatorPanelPosition: position }),
    setEmulatorLayout: (layout: 'left' | 'right' | 'hidden') => {
        try { localStorage.setItem('emulatorLayout', layout); } catch { /* ignore quota errors */ }
        set({ emulatorLayout: layout });
    },
    openAiriPanel: () => set({ isRightSidebarOpen: true, isAiriPanelOpen: true, isEmulatorPanelOpen: false }),
    closeAiriPanel: () => set({ isAiriPanelOpen: false }),
    openEmulatorPanel: () => set({ isRightSidebarOpen: true, isEmulatorPanelOpen: true, isAiriPanelOpen: false }),
    closeEmulatorPanel: () => set({ isEmulatorPanelOpen: false }),
    checkLlamaCppStatus: async () => {
        set({ llamaCppStatus: 'checking' });
        try {
            const response = await fetch(`${get().llamaCppUrl}/health`, {
                method: 'GET',
                signal: AbortSignal.timeout(3000),
            });
            set({ llamaCppStatus: response.ok ? 'running' : 'error' });
        } catch (e) {
            set({ llamaCppStatus: 'error' });
        }
    },

    refreshFileTree: async () => {
        try {
            const tree = await invoke<FileEntry[]>('get_file_tree');
            set({ fileTree: tree });
        } catch (error) {
            console.error('Refresh File Tree Error:', error);
            // If it fails because no root is set, clear the tree
            set({ fileTree: [] });
        }
    },

    closeFolder: () => {
        invoke('set_active_root', { path: null });
        set({ activeRoot: null, activeRootName: null, fileTree: [] });
    },
    showWelcomeTab: () => {
        const { openFile } = get().activeTabId !== undefined ? get() : { openFile: (p: string) => { } };
        (get() as any).openFile('Welcome');
    },

    openFile: async (path: string) => {
        const existingTab = get().tabs.find(t => t.path === path);
        if (existingTab) {
            get().setActiveTab(existingTab.id);
            return;
        }
        try {
            const content = await invoke<string>('read_file', { path });
            const filename = path.replace(/\\/g, '/').split('/').pop() ?? path;
            const id = `tab-${Date.now()}-${Math.random()}`;
            const tab: EditorTab = { id, filename, path, content, isModified: false, language: detectLanguage(filename) };
            set((state) => {
                const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
                history.push(id);
                return { tabs: [...state.tabs, tab], activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
            });
        } catch (error) {
            console.error('Open File Error:', error);
        }
    },

    closeTab: (id: string) => {
        set((state) => {
            const tabs = state.tabs.filter(t => t.id !== id);
            let activeTabId = state.activeTabId;
            if (activeTabId === id) {
                activeTabId = tabs.length > 0 ? tabs[tabs.length - 1].id : null;
            }
            return { tabs, activeTabId };
        });
    },

    setActiveTab: (id: string) => set((state) => {
        if (state.activeTabId === id) return {};
        const history = state.tabHistory.slice(0, state.tabHistoryIndex + 1);
        history.push(id);
        return { activeTabId: id, tabHistory: history, tabHistoryIndex: history.length - 1 };
    }),

    navigateBack: () => set((state) => {
        if (state.tabHistoryIndex <= 0) return {};
        const newIndex = state.tabHistoryIndex - 1;
        const tabId = state.tabHistory[newIndex];
        if (!state.tabs.find((t: any) => t.id === tabId)) return {};
        return { activeTabId: tabId, tabHistoryIndex: newIndex };
    }),

    navigateForward: () => set((state) => {
        if (state.tabHistoryIndex >= state.tabHistory.length - 1) return {};
        const newIndex = state.tabHistoryIndex + 1;
        const tabId = state.tabHistory[newIndex];
        if (!state.tabs.find((t: any) => t.id === tabId)) return {};
        return { activeTabId: tabId, tabHistoryIndex: newIndex };
    }),

    setSplitEditorTab: (tabId: string | null) => set({ splitEditorTabId: tabId, isSplitEditorOpen: tabId !== null }),
    toggleSplitEditor: () => set((state: any) => {
        if (state.isSplitEditorOpen) return { isSplitEditorOpen: false, splitEditorTabId: null };
        // default: clone active tab in split
        return { isSplitEditorOpen: true, splitEditorTabId: state.activeTabId };
    }),

    updateTabContent: (id: string, content: string) => {
        set((state: any) => {
            const isVisualizing = state.isVisualLabOpen && state.activeTabId === id;
            return {
                tabs: state.tabs.map((t: any) => t.id === id ? { ...t, content, isModified: true } : t),
                visualLabData: isVisualizing ? content : state.visualLabData
            };
        });
    },

    saveActiveFile: async () => {
        const { tabs, activeTabId } = get();
        const tab = tabs.find(t => t.id === activeTabId);
        if (!tab || tab.type === 'settings') return;
        try {
            await invoke('write_file', { path: tab.path, content: tab.content });
            set((state) => ({
                tabs: state.tabs.map(t => t.id === activeTabId ? { ...t, isModified: false } : t),
            }));
        } catch (error) {
            console.error('Save File Error:', error);
        }
    },

    openSettings: (tab?: 'user' | 'workspace' | 'agent') => {
        // Stash the requested initial tab so SettingsPage can read it as it
        // mounts (or re-mounts when the user returns to the existing tab).
        try {
            if (tab) sessionStorage.setItem('settings.initialTab', tab);
        } catch { /* sessionStorage unavailable */ }
        try {
            // Broadcast for the case where the Settings tab is already open
            // and we just want it to switch its inner section.
            window.dispatchEvent(new CustomEvent('settings:focus-tab', { detail: { tab } }));
        } catch { /* no-op */ }

        const settingsTab = get().tabs.find(t => t.type === 'settings');
        if (settingsTab) {
            set({ activeTabId: settingsTab.id });
            return;
        }
        const id = 'settings-tab';
        const newTab: EditorTab = {
            id,
            filename: 'Settings',
            path: 'vscode://settings',
            content: '',
            isModified: false,
            language: '',
            type: 'settings'
        };
        set((state) => ({ tabs: [...state.tabs, newTab], activeTabId: id }));
    },

    // Backend Actions
    backendPing: async () => {
        try {
            return await invoke<string>('backend_ping');
        } catch (error) {
            console.error('Backend Ping Error:', error);
            return `Error: ${error}`;
        }
    },

    refreshAvailableModels: async (targetProvider?: string) => {
        const { ollamaUrl } = get();
        try {
            const keys: any = await invoke('get_api_keys');
            const providers: string[] = [];

            // ALWAYS try Ollama first (default to local models)
            providers.push('Ollama');

            if (keys.google) providers.push('Google');
            if (keys.anthropic) providers.push('Anthropic');
            if (keys.openai) providers.push('OpenAI');
            if (keys.openrouter) providers.push('Openrouter');
            if (keys.mistral) providers.push('Mistral');
            if ((keys as any).deepseek) providers.push('Deepseek');
            // Local DeepSeek V2 on Apple Silicon — always available because
            // the server is local and keyless. If it isn't running, model
            // listing will fail with a clear message and the user gets a
            // pointer to `tools/deepseek-ane/start-server.sh`. Mac-only.
            if (typeof navigator !== 'undefined' && /Mac/i.test(navigator.platform || navigator.userAgent || '')) {
                providers.push('Deepseek-ANE');
            }
            if (keys.groq) providers.push('Groq');
            if (keys.xai) providers.push('xAI');
            if (keys.cerebras) providers.push('Cerebras');
            if (keys.alibaba) providers.push('Alibaba');
            if ((keys as any).nvidia) providers.push('Nvidia');
            providers.push('ApiRadar'); // Always include for aggregated view
            providers.push(
                'OpenWebUI',
                'Claude (WebUI)',
                'Gemini (WebUI)',
                'OpenAI (WebUI)',
                'DeepSeek (WebUI)',
                'Qwen (WebUI)',
            ); // WebUI integrations

            let allModels: { id: string, provider: string }[] = [];

            // Fix case sensitivity and provider mapping
            const activeProviders = targetProvider
                ? [targetProvider.toLowerCase() === 'apiradar' ? 'ApiRadar' : targetProvider.charAt(0).toUpperCase() + targetProvider.slice(1).toLowerCase()]
                : providers;

            for (const p of activeProviders) {
                try {
                    if (p.toLowerCase() === 'ollama') {
                        // Use whatever Ollama URL the store already has (set by
                        // the user / persisted in localStorage). Falling back
                        // to the dead community cloud endpoint just spams 404s.
                        const raw = get().ollamaUrl || 'http://localhost:11434';
                        const ollamaToUse = normalizeOllamaUrl(raw);
                        await invoke('set_ollama_url', { url: ollamaToUse });
                        set({ ollamaUrl: ollamaToUse });
                        try {
                            localStorage.setItem('ollamaUrl', ollamaToUse);
                        } catch {
                            /* Tracking Prevention / private mode */
                        }
                        const isLocal = /localhost|127\.|0\.0\.0\.0/.test(ollamaToUse);
                        set({
                            ollamaConnectionMode: 'direct',
                            ollamaMode: isLocal ? 'local' : 'cloud',
                        });
                    } // <-- Added missing brace here
                    let models: string[] = [];
                    if (p.includes('WebUI') && p !== 'OpenWebUI') {
                        const baseProvider = p.split(' ')[0].toLowerCase();
                        allModels.push({ id: `WebUI Session (${baseProvider})`, provider: p.toLowerCase() });
                        continue;
                    } else {
                        models = await invoke<string[]>('list_provider_models', { provider: p });
                        allModels = [...allModels, ...models.map(m => ({ id: m, provider: p.toLowerCase() }))];
                    }

                    if (p.toLowerCase() === 'ollama') {
                        if (models.length > 0) {
                            set({ ollamaStatus: 'running' });
                        }
                    }
                } catch (e: any) {
                    // Suppress common error when a provider key is simply missing
                    if (e && typeof e === 'string' && e.includes('API key not found')) {
                        // Silent skip
                    } else {
                        console.error(`Failed to fetch models for ${p}:`, e);
                    }
                    if (p.toLowerCase() === 'ollama') set({ ollamaStatus: 'error' });
                }
            }

            // Manifest Antigravity as a completely separate, first-class provider
            // This ensures ZERO interference with the native Ollama model list
            allModels.push({ id: 'antigravity-sentient', provider: 'antigravity' });

            set((state) => {
                let currentModels = [...state.availableModels];

                if (targetProvider) {
                    // Refreshing only ONE provider: remove its old models
                    currentModels = currentModels.filter(m => m.provider !== targetProvider.toLowerCase());
                } else {
                    // Refreshing ALL: remove everything except Ollama if it was already running and not being refreshed
                    // Actually, since practitioners often have many Ollama models, we should only keep them if they are still valid.
                    // But for simplicity, if targetProvider is null (full refresh), we start fresh except for Ollama which we might want to preserve 
                    // if it takes long to fetch. However, list_provider_models is fast.
                    currentModels = [];
                }

                // Add newly fetched models, ensuring NO duplicates by ID
                const newModels = allModels.filter(nm => !currentModels.some(cm => cm.id === nm.id && cm.provider === nm.provider));

                return {
                    availableModels: [...currentModels, ...newModels],
                    lastRefresh: Date.now()
                };
            });
        } catch (e) {
            console.error('Refresh Available Models Error:', e);
        }
    },

    refreshWebuiSessions: async (provider?: string) => {
        try {
            const sessions: any = await invoke('list_webui_sessions', { provider: provider || null });
            set({ webuiSessions: sessions || [] });
            const active = sessions?.find((s: any) => s.is_active);
            if (active) {
                set({ activeWebuiSessionId: active.session_id });
                try {
                    localStorage.setItem(`hades.webui.account.${active.provider}`, active.display_name || 'default');
                } catch { /* non-fatal */ }
            } else if (sessions && sessions.length > 0) {
                set({ activeWebuiSessionId: sessions[0].session_id });
                try {
                    localStorage.setItem(`hades.webui.account.${sessions[0].provider}`, sessions[0].display_name || 'default');
                } catch { /* non-fatal */ }
            } else {
                set({ activeWebuiSessionId: null });
            }
        } catch (error) {
            console.error('Refresh WebUI sessions error:', error);
        }
    },

    switchWebuiSession: async (sessionId: string) => {
        try {
            await invoke('switch_webui_session', { sessionId });
            set({ activeWebuiSessionId: sessionId });
            const session = get().webuiSessions.find((s: any) => s.session_id === sessionId);
            if (session) {
                try {
                    localStorage.setItem(`hades.webui.account.${session.provider}`, session.display_name || 'default');
                } catch { /* non-fatal */ }
            }
            // Refresh models list
            await get().refreshAvailableModels();
        } catch (error) {
            console.error('Switch WebUI session error:', error);
        }
    },

    deleteWebuiSession: async (sessionId: string) => {
        try {
            await invoke('delete_webui_session', { sessionId });
            await get().refreshWebuiSessions();
        } catch (error) {
            console.error('Delete WebUI session error:', error);
        }
    },

    startMitm: async () => {
        try {
            set({ mitmStatus: 'running' });
            await invoke('start_mitm_server');
            get().addMitmLog('Proxy server started on port 8080');
        } catch (e: any) {
            set({ mitmStatus: 'error' });
            get().addMitmLog(`Error: ${e}`);
        }
    },

    stopMitm: async () => {
        try {
            await invoke('stop_mitm_server');
            set({ mitmStatus: 'idle' });
            get().addMitmLog('Proxy server stopped');
        } catch (e: any) {
            get().addMitmLog(`Error stopping server: ${e}`);
        }
    },
    addMitmLog: (log) => set((state) => ({
        mitmLogs: [...state.mitmLogs, `[${new Date().toLocaleTimeString()}] ${log}`].slice(-100)
    })),

    addMcpServer: async (name, config) => {
        try {
            await invoke('add_mcp_server', { name, config });
            await get().listMcpServers();
        } catch (e) {
            console.error('Add MCP Server Error:', e);
        }
    },
    removeMcpServer: async (name) => {
        try {
            await invoke('remove_mcp_server', { name });
            await get().listMcpServers();
        } catch (e) {
            console.error('Remove MCP Server Error:', e);
        }
    },
    listMcpServers: async () => {
        try {
            const servers = await invoke<McpServer[]>('list_mcp_servers');
            set({ mcpServers: servers });
        } catch (e) {
            console.error('List MCP Servers Error:', e);
        }
    },
    setMcpServerEnabled: async (name: string, enabled: boolean) => {
        try {
            await invoke('set_mcp_server_enabled', { name, enabled });
            await get().listMcpServers();
        } catch (e) {
            console.error('Toggle MCP Server Error:', e);
        }
    },

    // ── Per-turn restore points ─────────────────────────────────────────
    setLastAgentCheckpoint: (cp) => set({ lastAgentCheckpoint: cp }),
    setLastUserMessageCheckpoint: (checkpointId: string, description?: string) => set((state) => {
        const messages = [...state.agentMessages];
        // Walk backwards to find the most recent user message (the one
        // sendAgentMessage just appended) and stamp the checkpoint id on
        // it. Idempotent: stamping the same id twice is a no-op.
        for (let i = messages.length - 1; i >= 0; i--) {
            if (messages[i].role === 'user') {
                if (messages[i].checkpointId === checkpointId) break;
                messages[i] = { ...messages[i], checkpointId, checkpointDescription: description };
                break;
            }
        }
        return { agentMessages: messages };
    }),
    restoreToMessageCheckpoint: async (timestamp: number) => {
        const state = get();
        const idx = state.agentMessages.findIndex((m: any) => m.timestamp === timestamp);
        if (idx < 0) return { ok: false, message: 'Message not found in chat history.' };
        const target: any = state.agentMessages[idx];
        if (!target?.checkpointId) return { ok: false, message: 'This message has no checkpoint attached.' };
        try {
            const message = await invoke<string>('git_rollback_checkpoint', { checkpointId: target.checkpointId });
            // Truncate the chat above and including the restored user
            // message — anything below it was generated against state we
            // just rewound, so keeping it would be misleading.
            set({
                agentMessages: state.agentMessages.slice(0, idx),
                lastAgentCheckpoint: null,
            });
            return { ok: true, message };
        } catch (e: any) {
            return { ok: false, message: String(e?.message ?? e) };
        }
    },
    rollbackLastAgentCheckpoint: async () => {
        const cp = get().lastAgentCheckpoint;
        if (!cp) return { ok: false, message: 'No checkpoint available to roll back.' };
        try {
            const message = await invoke<string>('git_rollback_checkpoint', { checkpointId: cp.id });
            set({ lastAgentCheckpoint: null });
            return { ok: true, message };
        } catch (e: any) {
            return { ok: false, message: String(e?.message ?? e) };
        }
    },
    // Multi-file review state — populated by the right sidebar's
    // ai-tool-call listener when the agent calls a write/patch tool,
    // consumed by MultiFileReview.tsx. We dedupe by path so the panel
    // shows one entry per file even if the agent edited it twice.
    addPendingAgentEdit: (edit) => set(s => {
        const existing = s.pendingAgentEdits.find(e => e.path === edit.path);
        if (existing) {
            return {
                pendingAgentEdits: s.pendingAgentEdits.map(e =>
                    e.path === edit.path
                        ? { ...e, tool: edit.tool, timestamp: Date.now(), preview: edit.preview ?? e.preview }
                        : e
                ),
            };
        }
        return {
            pendingAgentEdits: [...s.pendingAgentEdits, { ...edit, timestamp: Date.now() }],
        };
    }),
    clearPendingAgentEdits: () => set({ pendingAgentEdits: [] }),
    openMultiFileReview: () => set({ isMultiFileReviewOpen: true }),
    closeMultiFileReview: () => set({ isMultiFileReviewOpen: false }),
    // Trajectory log. We bound at 1000 entries so a runaway agent loop
    // can't balloon the heap. The current turn id is bumped each user
    // send so the panel can group by turn for replay.
    pushTrajectoryEvent: (evt) => set(s => ({
        agentTrajectory: [
            ...s.agentTrajectory.slice(-999),
            {
                id: `tr-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
                ts: Date.now(),
                turn: s.currentTurnId,
                ...evt,
            },
        ],
    })),
    clearTrajectory: () => set({ agentTrajectory: [] }),
    openTrajectory: () => set({ isTrajectoryOpen: true }),
    closeTrajectory: () => set({ isTrajectoryOpen: false }),
    beginNewTurn: () => set(s => ({ currentTurnId: s.currentTurnId + 1 })),
    openMarkdownPreview: () => set({ isMarkdownPreviewOpen: true }),
    closeMarkdownPreview: () => set({ isMarkdownPreviewOpen: false }),
    toggleMarkdownPreview: () => set(s => ({ isMarkdownPreviewOpen: !s.isMarkdownPreviewOpen })),

    // ── Background agents ───────────────────────────────────────────────
    runBackgroundAgent: async (prompt: string) => {
        const id = `bg-${Date.now()}-${Math.floor(Math.random() * 1000)}`;
        set(state => ({
            backgroundAgents: [
                ...state.backgroundAgents,
                { id, prompt, status: 'running', result: '', startedAt: Date.now() },
            ],
        }));
        try {
            // Background runs use the same autonomous backend as the foreground
            // agent, but through `ai_chat_oneshot` so UI events do not cross-talk
            // with the active chat. Keep this a real agent mission: tools,
            // workspace writes, verification, and completion contract enabled.
            const state = get();
            const provider = state.agentModel.includes('|')
                ? state.agentModel.split('|')[0]
                : 'ollama';
            const model = state.agentModel.includes('|')
                ? state.agentModel.split('|')[1]
                : state.agentModel;
            // Background runs use the dedicated `ai_chat_oneshot` Tauri
            // command, which mirrors `ai_chat` except every `emit_event`
            // call inside the engine is suppressed for the duration of
            // the run. That way the foreground chat keeps streaming its
            // own phases/content without any cross-talk.
            const resultText = await invoke<string>('ai_chat_oneshot', {
                request: {
                    provider,
                    model,
                    messages: [
                        {
                            role: 'system',
                            content:
                                'You are a persistent autonomous coding agent inside VSCodium-Rust. Execute tools, inspect the workspace, write fixes, run verification, and continue until the requested goal is actually finished. Only stop when you can emit MISSION_ACCOMPLISHED or TASK_COMPLETE with a blocker.',
                        },
                        { role: 'user', content: prompt },
                    ],
                    autonomous: true,
                    root_access: true,
                    mode: state.agentMode === 'Chat' ? 'Agent' : state.agentMode,
                    ollama_url: state.ollamaUrl,
                },
            });
            set(s => ({
                backgroundAgents: s.backgroundAgents.map(b =>
                    b.id === id
                        ? { ...b, status: 'done', result: resultText, finishedAt: Date.now() }
                        : b,
                ),
            }));
            return id;
        } catch (e: any) {
            set(s => ({
                backgroundAgents: s.backgroundAgents.map(b =>
                    b.id === id
                        ? { ...b, status: 'error', result: String(e?.message ?? e), finishedAt: Date.now() }
                        : b,
                ),
            }));
            return id;
        }
    },
    removeBackgroundAgent: (id: string) => set(s => ({
        backgroundAgents: s.backgroundAgents.filter(b => b.id !== id),
    })),
    clearBackgroundAgents: () => set({ backgroundAgents: [] }),

    refreshProcessStats: async () => {
        try {
            const stats = await invoke<any>('get_process_stats');
            set({ processStats: stats });
        } catch (e) {
            console.error('Refresh Process Stats Error:', e);
        }
    },
    compressSessionData: async (key: string, data: string) => {
        try {
            await invoke('compress_session_data', { key, data });
            get().refreshMemorySavings();
        } catch (e) {
            console.error('Compress Session Data Error:', e);
        }
    },
    refreshMemorySavings: async () => {
        try {
            const [original, compressed] = await invoke<[number, number]>('get_memory_savings');
            set({ memorySavings: { original, compressed } });
        } catch (e) {
            console.error('Refresh Memory Savings Error:', e);
        }
    },
    attachFile: (file: any | any[]) => set((state: any) => {
        const files = Array.isArray(file) ? file : [file];
        const newAttached = [...state.attachedFiles];
        for (const f of files) {
            if (!newAttached.find(existing => existing.path === f.path)) {
                newAttached.push({ ...f, type: f.type || 'file' });
            }
        }
        return { attachedFiles: newAttached };
    }),
    removeFile: (path: string) => set((state: any) => ({ attachedFiles: state.attachedFiles.filter((f: any) => f.path !== path) })),
    clearAttachedFiles: () => set({ attachedFiles: [] }),
    addAttachedContext: (item: AttachedContext) => set((state: any) => ({
        attachedContext: [...state.attachedContext, item]
    })),
    removeAttachedContext: (index: number) => set((state: any) => ({
        attachedContext: state.attachedContext.filter((_: any, i: number) => i !== index)
    })),
    clearAttachedContext: () => set({ attachedContext: [] }),
    addAgentMessage: (role: any, content: any, contextOrSubAgent: any) => set((state: any) => {
        const isSubAgent = typeof contextOrSubAgent === 'boolean' ? contextOrSubAgent : false;
        const context = Array.isArray(contextOrSubAgent) ? contextOrSubAgent : [];
        const timestamp = Date.now();
        const newMessage: any = {
            role,
            content,
            context,
            timestamp,
            isSubAgentResponse: isSubAgent,
            steps: role === 'assistant' ? [] : undefined
        };

        // Phase 25: Sync to Backend Persistence
        invoke('store_message', {
            role,
            content: typeof content === 'string' ? content : JSON.stringify(content),
            timestamp
        }).catch((err: any) => console.error("[Persistence] Sync failed:", err));

        let newMessages = [...state.agentMessages, newMessage];
        if (newMessages.length > 100) {
            newMessages = newMessages.slice(newMessages.length - 100);
        }
        return { agentMessages: newMessages };
    }),
    updateLastAgentMessage: (content: any) => set((state) => {
        const messages = [...state.agentMessages];
        const lastIndex = messages.length - 1;
        const last = messages[lastIndex];
        if (last && last.role === 'assistant') {
            // Defensive: ensure content is a string even if backend/streaming emits an object
            const rawContent = typeof content === 'string'
                ? content
                : (content && typeof content === 'object' && content.content ? content.content : String(content));

            let newContent = rawContent;
            let newThoughts = last.thoughts;

            // Extract <think> blocks if present in the new content
            const thinkMatch = rawContent.match(/<think>([\s\S]*?)<\/think>/);
            if (thinkMatch) {
                newThoughts = thinkMatch[1].trim();
                newContent = rawContent.replace(/<think>[\s\S]*?<\/think>/, '').trim();
            } else if (rawContent.startsWith('<think>') && !rawContent.includes('</think>')) {
                // Partial thinking block
                newThoughts = rawContent.replace('<think>', '').trim();
                newContent = '';
            }

            messages[lastIndex] = { ...last, content: newContent, thoughts: newThoughts };
        }
        return { agentMessages: messages };
    }),
    updateLastAgentThought: (thought: string) => {
        // HADES FIX: Thoughts are internal - don't store them in visible messages
        // Only update the internal thought state for UI display (THINKING indicator)
        return { currentThought: thought };
    },
    appendLastAgentMessage: (delta: string) => set((state: any) => {
        const messages = [...state.agentMessages];
        const lastIndex = messages.length - 1;
        const last = messages[lastIndex];
        if (last && last.role === 'assistant') {
            const currentContent = last.content || '';
            const currentThoughts = last.thoughts || '';
            const currentRaw = last.raw_buffer || (currentContent + currentThoughts);

            // We use the OpenAI-compat /v1/chat/completions endpoint which emits
            // true deltas, not accumulated content. The old "Overlap Merger" caused
            // data loss: common English suffixes ("ing", "s", "ly") matched
            // cross-token boundaries and were stripped. Just append directly.
            // Guard: skip exact full-duplicate only (e.g. provider resent the chunk).
            if (delta.length > 0 && currentRaw.endsWith(delta)) return state;

            const fullRaw = currentRaw + delta;
            let newContent = currentContent;
            let newThoughts = currentThoughts;

            // Strip thinking blocks from visible content - thoughts are internal only
            if (fullRaw.includes('<think>')) {
                const thinkMatch = fullRaw.match(/<think>([\s\S]*?)<\/think>/);
                if (thinkMatch) {
                    // Update global UI state with a summary of the thought
                    const thoughtText = thinkMatch[1].trim();
                    if (thoughtText) set({ currentThought: thoughtText });

                    // The visible content is everything EXCEPT the thinking blocks
                    newContent = fullRaw.replace(/<think>[\s\S]*?<\/think>/g, '').trim();
                } else {
                    // Think block is open but not closed yet.
                    // Keep existing content, and only show what's BEFORE the <think> tag.
                    const parts = fullRaw.split('<think>');
                    newContent = parts[0] || '';

                    // Update current thought with the partial content from inside the tag
                    const partialThought = parts[1] || '';
                    if (partialThought) set({ currentThought: partialThought.trim() });
                }
            } else {
                // If there's no think block, or we are AFTER the think block
                // Just use the tail of fullRaw if a think block was previously stripped
                if (fullRaw.includes('</think>')) {
                    newContent = fullRaw.split('</think>').pop()?.trim() || '';
                } else {
                    newContent += delta;
                }
            }

            messages[lastIndex] = {
                ...last,
                content: newContent,
                thoughts: newThoughts,
                raw_buffer: fullRaw
            };
            return { agentMessages: messages };
        }
        return state;
    }),
    addAgentStep: (name, type, args, callId) => set((state) => {
        const messages = [...state.agentMessages];
        if (messages.length === 0) return state;
        const last = messages[messages.length - 1];
        if (last && last.role === 'assistant') {
            const steps = last.steps || [];
            // Avoid duplicate steps if redelivered, check callId if present
            if (!steps.find(s => (callId && s.callId === callId) || (!callId && s.name === name && s.status === 'running'))) {
                last.steps = [...steps, { name, status: 'running', type, args, callId }];
            }
        }
        return { agentMessages: messages };
    }),
    updateAgentStepStatus: (name, status, result, summary, callId) => set((state) => {
        const messages = [...state.agentMessages];
        if (messages.length === 0) return state;
        const last = messages[messages.length - 1];
        if (last && last.role === 'assistant' && last.steps) {
            // Priority 1: Match by callId
            // Priority 2: Match by name if still running
            const step = last.steps.find(s => (callId && s.callId === callId) || (!callId && s.name === name && s.status === 'running'));
            if (step) {
                step.status = status;
                if (result !== undefined) step.result = result;
                if (summary !== undefined) step.summary = summary;
            }
        }
        return { agentMessages: messages };
    }),
    setActiveEditorPath: (activeEditorPath) => set({ activeEditorPath }),
    setIsAgentThinking: (isAgentThinking) => set({ isAgentThinking }),
    setIsAgentPaused: (isAgentPaused) => set({ isAgentPaused }),
    setYoloMode: (isYoloMode) => set({ isYoloMode }),
    setContinuousMode: (isContinuousMode: boolean) => set({ isContinuousMode }),
    setAiriVisionEnabled: (enabled) => {
        try { localStorage.setItem('airi.vision.enabled', enabled ? '1' : '0'); } catch { /* quota */ }
        set({ airiVisionEnabled: enabled });
        // Best-effort start/stop; vision-system is lazy-loaded.
        (async () => {
            try {
                const mod = await import('./airi/vision-system');
                if (enabled) {
                    await mod.airiVision.start();
                } else if (typeof (mod.airiVision as any).stop === 'function') {
                    (mod.airiVision as any).stop();
                }
            } catch (err) {
                console.warn('[store] toggling AIRI vision failed:', err);
            }
        })();
    },
    setAiriVisionModel: (model) => {
        try { localStorage.setItem('airi.vision.model', model); } catch { /* quota */ }
        set({ airiVisionModel: model });
        (async () => {
            try {
                const { visionAnalyzer } = await import('./airi/vision-analysis');
                if (typeof (visionAnalyzer as any).reconfigure === 'function') {
                    (visionAnalyzer as any).reconfigure({ model });
                }
            } catch { /* analyzer not loaded yet */ }
        })();
    },
    setAiriConsciousnessEnabled: (enabled) => {
        try { localStorage.setItem('airi.consciousness.enabled', enabled ? '1' : '0'); } catch { /* quota */ }
        set({ airiConsciousnessEnabled: enabled });
        (async () => {
            try {
                const { airiConsciousness } = await import('./airi/consciousness');
                if (enabled) {
                    if (typeof (airiConsciousness as any).resumeThoughts === 'function') {
                        (airiConsciousness as any).resumeThoughts();
                    } else if (typeof (airiConsciousness as any).wakeUp === 'function') {
                        (airiConsciousness as any).wakeUp();
                    } else if (typeof (airiConsciousness as any).start === 'function') {
                        (airiConsciousness as any).start();
                    }
                } else if (typeof (airiConsciousness as any).pauseThoughts === 'function') {
                    (airiConsciousness as any).pauseThoughts();
                }
            } catch (err) {
                console.warn('[store] toggling consciousness failed:', err);
            }
        })();
    },
    setAiriConsciousnessModel: (model) => {
        try { localStorage.setItem('airi.consciousness.model', model); } catch { /* quota */ }
        set({ airiConsciousnessModel: model });
        (async () => {
            try {
                const { airiConsciousness } = await import('./airi/consciousness');
                if (typeof (airiConsciousness as any).reconfigure === 'function') {
                    (airiConsciousness as any).reconfigure({ model });
                }
            } catch { /* not loaded yet */ }
        })();
    },
    setAgentUiMode: (agentUiMode) => { localStorage.setItem('agentUiMode', agentUiMode); set({ agentUiMode }); },
    setTtsStrategy: (strategy) => {
        localStorage.setItem('ttsStrategy', strategy);
        set({ ttsStrategy: strategy });
        import('./voice').then(({ setProvider }) => setProvider(strategy)).catch(() => { });
    },
    // Cursor-style IDE preference setters. Each one persists to
    // localStorage under a stable key so toggling lives across reloads.
    // Boolean keys use '1'/'0' to stay compatible with the rest of the
    // store; lists are JSON-encoded.
    // Cursor-style IDE preference setters. Each one persists to
    // localStorage under a stable key so toggling lives across reloads.
    // Boolean keys use '1'/'0' to stay compatible with the rest of the
    // store; lists are JSON-encoded.
    setTabPredictionEnabled: (v) => { try { localStorage.setItem('tab.predictionEnabled', v ? '1' : '0'); } catch { } set({ tabPredictionEnabled: v }); },
    setTabMultilineSuggestions: (v) => { try { localStorage.setItem('tab.multilineSuggestions', v ? '1' : '0'); } catch { } set({ tabMultilineSuggestions: v }); },
    setTabAcceptKey: (k) => { try { localStorage.setItem('tab.acceptKey', k); } catch { } set({ tabAcceptKey: k }); },
    setBetaFastApply: (v) => { try { localStorage.setItem('beta.fastApply', v ? '1' : '0'); } catch { } set({ betaFastApply: v }); },
    setBetaSemanticSearch: (v) => { try { localStorage.setItem('beta.semanticSearch', v ? '1' : '0'); } catch { } set({ betaSemanticSearch: v }); },
    setBetaShadowWorkspace: (v) => { try { localStorage.setItem('beta.shadowWorkspace', v ? '1' : '0'); } catch { } set({ betaShadowWorkspace: v }); },
    setAimVfsEnabled: (v) => { try { localStorage.setItem('aimVfsEnabled', v ? 'true' : 'false'); } catch { } set({ aimVfsEnabled: v }); },
    setThermalGovernorEnabled: (v) => { try { localStorage.setItem('thermalGovernorEnabled', v ? 'true' : 'false'); } catch { } set({ thermalGovernorEnabled: v }); },
    setJitDecompressionEnabled: (v) => { try { localStorage.setItem('jitDecompressionEnabled', v ? 'true' : 'false'); } catch { } set({ jitDecompressionEnabled: v }); },
    setJitThreshold: (v) => { try { localStorage.setItem('jitThreshold', v.toString()); } catch { } set({ jitThreshold: v }); },
    setNetworkProxyUrl: (url: string) => { try { localStorage.setItem('network.proxyUrl', url); } catch { } set({ networkProxyUrl: url }); },
    setNetworkAllowInsecureTls: (v) => { try { localStorage.setItem('network.allowInsecureTls', v ? '1' : '0'); } catch { } set({ networkAllowInsecureTls: v }); },
    setIndexingEnabled: (v) => { try { localStorage.setItem('indexing.enabled', v ? '1' : '0'); } catch { } set({ indexingEnabled: v }); },
    setIndexingDocsUrls: (urls) => { try { localStorage.setItem('indexing.docsUrls', JSON.stringify(urls)); } catch { } set({ indexingDocsUrls: urls }); },
    setAvatarCharacter: (avatarCharacter) => { localStorage.setItem('avatarCharacter', avatarCharacter); set({ avatarCharacter }); },
    setAvatarCustomConfig: (config: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean }) => {
        const existing = JSON.parse(localStorage.getItem('avatarCustomConfig') || '{}');
        const updated = { ...existing, ...config };
        localStorage.setItem('avatarCustomConfig', JSON.stringify(updated));
        set({ avatarCustomConfig: updated });
    },
    setAvatar3dConfig: (config: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> }) => {
        const existing = JSON.parse(localStorage.getItem('avatar3dConfig') || '{}');
        const updated = { ...existing, ...config };
        localStorage.setItem('avatar3dConfig', JSON.stringify(updated));
        set({ avatar3dConfig: updated });
    },
    setShowVrmAvatar: (show) => {
        try {
            localStorage.setItem('showVrmAvatar', show ? 'true' : 'false');
        } catch { }
        set({ showVrmAvatar: show });
    },
    setAgentCurrentAction: (agentCurrentAction) => set({ agentCurrentAction }),
    addAgentFile: (path: string) => {
        set((state) => {
            const last = state.agentMessages[state.agentMessages.length - 1];
            if (last && last.role === 'assistant') {
                const files = last.files || [];
                if (!files.includes(path)) {
                    const newMessages = [...state.agentMessages];
                    newMessages[newMessages.length - 1] = { ...last, files: [...files, path] };
                    return { agentMessages: newMessages };
                }
            }
            return state;
        });
    },
    addAgentArtifact: (art) => {
        set((state) => {
            const artifact: Artifact = {
                ...art,
                id: Math.random().toString(36).substring(7),
                timestamp: Date.now()
            };
            const last = state.agentMessages[state.agentMessages.length - 1];
            if (last && last.role === 'assistant') {
                const artifacts = last.artifacts || [];
                if (!artifacts.find(a => a.path === artifact.path)) {
                    const newMessages = [...state.agentMessages];
                    newMessages[newMessages.length - 1] = { ...last, artifacts: [...artifacts, artifact] };

                    // Also add to current task if exists
                    const currentTask = state.agentTask ? {
                        ...state.agentTask,
                        artifacts: [...state.agentTask.artifacts, artifact],
                        updatedAt: Date.now()
                    } : null;

                    return { agentMessages: newMessages, agentTask: currentTask };
                }
            }
            return state;
        });
    },
    setAgentMessages: (agentMessages) => set({ agentMessages }),
    refreshAgentHistory: async () => {
        try {
            const messages = await invoke<any[]>('get_agent_messages');
            if (messages && Array.isArray(messages) && messages.length > 0) {
                // Map backend format to frontend format if needed
                const mapped = messages.map((m: any) => ({
                    role: m.role,
                    content: typeof m.content === 'object' ? (m.content.text || '') : (m.content || ''),
                    timestamp: m.metadata?.timestamp || Date.now(),
                    metadata: m.metadata || {},
                    context: []
                }));
                set({ agentMessages: mapped });
                console.log("[Persistence] History refreshed from backend core.");
            }
        } catch (err) {
            console.error("[Persistence] Refresh failed:", err);
        }
    },
    setAgentTasks: (agentTasks) => set({ agentTasks }),
    clearAgentMessages: () => set({ agentMessages: [] }),
    resetThread: () => {
        set({ agentMessages: [], pendingChanges: [], attachedContext: [] });
        invoke('set_ai_status', { status: 'alive' }).catch(console.error);
    },
    truncateAgentMessages: (index: number) => set((state) => ({
        agentMessages: state.agentMessages.slice(0, index)
    })),
    updateAgentTask: (taskUpdate) => set((state) => {
        const existingTasks = [...state.agentTasks];
        const index = existingTasks.findIndex(t => t.id === taskUpdate.id);

        let updatedTask: AgentTask;
        if (index > -1) {
            existingTasks[index] = {
                ...existingTasks[index],
                ...taskUpdate,
                updatedAt: Date.now()
            } as AgentTask;
        } else {
            updatedTask = {
                id: taskUpdate.id,
                title: taskUpdate.title || 'Agent Task',
                summary: taskUpdate.summary || '',
                status: (taskUpdate.status as any) || 'running',
                progress: taskUpdate.progress || 0,
                createdAt: Date.now(),
                updatedAt: Date.now(),
                artifacts: []
            };
            existingTasks.push(updatedTask);
        }

        return {
            agentTasks: existingTasks,
            agentTask: updatedTask // Set as current active task
        };
    }),
    setCommandPaletteOpen: (isCommandPaletteOpen) => set({ isCommandPaletteOpen }),
    setContextMenuOpen: (isContextMenuOpen, x = 0, y = 0) => set({ isContextMenuOpen, contextMenuPosition: { x, y } }),
    setDebugToolbarOpen: (isDebugToolbarOpen) => set({ isDebugToolbarOpen }),
    setCommandPaletteQuery: (commandPaletteQuery) => set({ commandPaletteQuery }),
    toggleDirectory: async (path: string) => {
        const state = get();
        const node = findNodeRecursive(state.fileTree, path);
        if (!node) return;

        const is_now_expanded = !node.is_expanded;

        const updateExpansionRecursive = (nodes: FileEntry[]): FileEntry[] => {
            return nodes.map(n => {
                if (n.path === path) return { ...n, is_expanded: is_now_expanded };
                if (n.children) return { ...n, children: updateExpansionRecursive(n.children) };
                return n;
            });
        };

        if (is_now_expanded && (!node.children || node.children.length === 0)) {
            try {
                const children = await invoke<FileEntry[]>('list_dir_flat', { path });
                const treeWithChildren = injectChildrenRecursive(state.fileTree, path, children);
                set({ fileTree: updateExpansionRecursive(treeWithChildren) });
            } catch (e) {
                console.error('Lazy load directory failed:', e);
                set({ fileTree: updateExpansionRecursive(state.fileTree) });
            }
        } else {
            set({ fileTree: updateExpansionRecursive(state.fileTree) });
        }
    },

    setAutoAcceptChanges: (v) => set({ autoAcceptChanges: v }),

    snapshotCheckpoint: async (paths) => {
        const snap: Record<string, string> = {};
        for (const p of paths) {
            try {
                const content = await invoke<string>('read_file', { path: p });
                snap[p] = content;
            } catch (_) { /* file may not exist yet */ }
        }
        set({ checkpoint: Object.keys(snap).length > 0 ? snap : null });
    },

    revertToCheckpoint: async () => {
        const { checkpoint } = get();
        if (!checkpoint) return;
        for (const [path, content] of Object.entries(checkpoint)) {
            try {
                await invoke('write_file', { path, content });
                const tab = get().tabs.find((t: any) => t.path === path);
                if (tab) get().updateTabContent(tab.id, content);
            } catch (e) {
                console.error('Checkpoint revert failed for', path, e);
            }
        }
        set({ checkpoint: null, pendingChanges: [] });
        await get().refreshFileTree();
    },

    // Diff Review Implementation
    proposePendingChange: (change) => {
        const id = Math.random().toString(36).substring(7);
        const newChange = {
            id,
            path: change.path,
            originalContent: (change as any).oldContent || '',
            proposedContent: (change as any).newContent || '',
            newContent: (change as any).newContent || '',
            description: change.description,
            additions: change.additions,
            deletions: change.deletions,
            acceptedHunkIds: [],
            rejectedHunkIds: [],
        };
        // Auto-accept mode: skip the review queue, apply immediately
        if (get().autoAcceptChanges) {
            set((state) => ({ pendingChanges: [...state.pendingChanges, newChange] }));
            // Fire-and-forget accept — errors logged but not surfaced
            get().acceptPendingChange(id).catch(console.error);
        } else {
            set((state) => ({ pendingChanges: [...state.pendingChanges, newChange] }));
        }
    },

    acceptPendingChange: async (id) => {
        const change = get().pendingChanges.find(c => c.id === id);
        if (!change) return;

        try {
            await invoke('accept_sentient_patch', { path: change.path });

            // Update open tabs if necessary
            const tab = get().tabs.find(t => t.path === change.path);
            if (tab) {
                const updatedContent = await invoke<string>('read_file', { path: change.path });
                get().updateTabContent(tab.id, updatedContent);
            }

            set((state) => ({
                pendingChanges: state.pendingChanges.filter(c => c.id !== id)
            }));

            await get().refreshFileTree();
        } catch (error) {
            console.error('Failed to accept sentient patch:', error);
        }
    },

    rejectPendingChange: async (id) => {
        const { pendingChanges } = get();
        const change = pendingChanges.find(c => c.id === id);
        if (!change) return;

        try {
            await invoke('reject_sentient_patch', { path: change.path });
            set((state) => ({
                pendingChanges: state.pendingChanges.filter(c => c.id !== id)
            }));
        } catch (error) {
            console.error('Failed to reject sentient patch:', error);
        }
    },

    acceptAllPendingChanges: async () => {
        const changes = get().pendingChanges;
        for (const change of changes) {
            await get().acceptPendingChange(change.id);
        }
    },

    rejectAllPendingChanges: () => {
        set({ pendingChanges: [] });
    },

    acceptHunk: async (changeId: string, hunkId: string) => {
        set((state) => {
            const change = state.pendingChanges.find(c => c.id === changeId);
            if (!change) return state;
            const accepted = change.acceptedHunkIds || [];
            if (accepted.includes(hunkId)) return state;

            return {
                pendingChanges: state.pendingChanges.map(c =>
                    c.id === changeId ? { ...c, acceptedHunkIds: [...accepted, hunkId] } : c
                )
            };
        });
    },

    rejectHunk: (changeId: string, hunkId: string) => {
        set((state) => {
            const change = state.pendingChanges.find(c => c.id === changeId);
            if (!change) return state;
            const rejected = change.rejectedHunkIds || [];
            if (rejected.includes(hunkId)) return state;

            const newRejected = [...rejected, hunkId];
            const newContent = patchContentSelective(change.originalContent, change.proposedContent, newRejected);

            return {
                pendingChanges: state.pendingChanges.map(c =>
                    c.id === changeId ? { ...c, rejectedHunkIds: newRejected, newContent } : c
                )
            };
        });
    },

    // Terminal Actions Implementation
    addTerminalGroup: async (shell) => {
        const id = `group-${Date.now()}`;
        const instanceId = `term-${Date.now()}-${Math.random().toString(36).substr(2, 4)}`;
        const name = shell ? shell.split(/[\\/]/).pop() || 'shell' : 'terminal';

        // Pass explicit id so React `TerminalInstance` and PTY `spawn_terminal`
        // use the same key. (Previously `getVSCodeTheme()` was passed as
        // `groupId` and the store id as `cwd`, so the manager created `term-1`
        // while the UI looked up `term-<timestamp>-…` → blank terminal.)
        await terminalManager.createTerminal(shell || undefined, undefined, undefined, instanceId);

        const newGroup: TerminalGroup = {
            id,
            name: `${name}`,
            instances: [instanceId],
            activeInstanceId: instanceId
        };

        set((state) => ({
            terminalGroups: [...state.terminalGroups, newGroup],
            activeTerminalGroupId: id,
            activePanelTab: 'TERMINAL',
            isBottomPanelOpen: true
        }));

        return id;
    },

    /**
     * Open the live AIRI activity terminal in the bottom panel. Idempotent —
     * if one is already open we just focus its group. Lets the user actually
     * *see* every tool call streaming instead of only the right-sidebar card.
     */
    addAiriActivityTerminal: async () => {
        const existing = get().terminalGroups.find(g => (g as any).name === 'AIRI');
        if (existing) {
            set({
                activeTerminalGroupId: existing.id,
                activePanelTab: 'TERMINAL',
                isBottomPanelOpen: true,
            });
            return existing.id;
        }
        const groupId = `group-airi-${Date.now()}`;
        const instanceId = await terminalManager.createAiriActivityTerminal(`airi-activity-${Date.now()}`);
        const newGroup: TerminalGroup = {
            id: groupId,
            name: 'AIRI',
            instances: [instanceId],
            activeInstanceId: instanceId,
        };
        set((state) => ({
            terminalGroups: [...state.terminalGroups, newGroup],
            activeTerminalGroupId: groupId,
            activePanelTab: 'TERMINAL',
            isBottomPanelOpen: true,
        }));
        return groupId;
    },

    splitTerminal: async (groupId, instanceId) => {
        const newInstanceId = `term-${Date.now()}-${Math.random().toString(36).substr(2, 4)}`;

        // Get shell of current instance if possible
        const currentInstance = terminalManager.getTerminal(instanceId);
        const shell = currentInstance?.shell;

        await terminalManager.createTerminal(shell, undefined, undefined, newInstanceId);

        set((state) => {
            const groups = state.terminalGroups.map(g => {
                if (g.id === groupId) {
                    return {
                        ...g,
                        instances: [...g.instances, newInstanceId],
                        activeInstanceId: newInstanceId
                    };
                }
                return g;
            });
            return { terminalGroups: groups };
        });

        return newInstanceId;
    },

    closeTerminalInstance: async (groupId, instanceId) => {
        await terminalManager.closeTerminal(instanceId);
        set((state) => {
            const groups = state.terminalGroups.map(g => {
                if (g.id === groupId) {
                    const newInstances = g.instances.filter(id => id !== instanceId);
                    return {
                        ...g,
                        instances: newInstances,
                        activeInstanceId: g.activeInstanceId === instanceId
                            ? (newInstances.length > 0 ? newInstances[newInstances.length - 1] : '')
                            : g.activeInstanceId
                    };
                }
                return g;
            }).filter(g => g.instances.length > 0);

            let activeId = state.activeTerminalGroupId;
            if (activeId === groupId && !groups.find(g => g.id === groupId)) {
                activeId = groups.length > 0 ? groups[groups.length - 1].id : null;
            }

            return { terminalGroups: groups, activeTerminalGroupId: activeId };
        });
    },

    setActiveTerminalGroup: (id) => set({ activeTerminalGroupId: id }),

    setActiveTerminalInstance: (groupId, instanceId) => set((state) => ({
        terminalGroups: state.terminalGroups.map(g =>
            g.id === groupId ? { ...g, activeInstanceId: instanceId } : g
        )
    })),

    renameTerminalGroup: (groupId, name) => set((state) => ({
        terminalGroups: state.terminalGroups.map(g =>
            g.id === groupId ? { ...g, name } : g
        )
    })),

    closeTerminalGroup: async (groupId) => {
        const group = get().terminalGroups.find(g => g.id === groupId);
        if (group) {
            for (const instanceId of group.instances) {
                await terminalManager.closeTerminal(instanceId);
            }
        }
        set((state) => {
            const nextGroups = state.terminalGroups.filter(g => g.id !== groupId);
            let nextActiveId = state.activeTerminalGroupId;
            if (nextActiveId === groupId) {
                nextActiveId = nextGroups.length > 0 ? nextGroups[0].id : null;
            }
            return {
                terminalGroups: nextGroups,
                activeTerminalGroupId: nextActiveId
            };
        });
    },

    updateTerminalSplitWeights: (groupId, weights) => set((state) => ({
        terminalGroups: state.terminalGroups.map(g =>
            g.id === groupId ? { ...g, splitWeights: weights } : g
        )
    })),

    setAgentTask: (agentTask) => set({ agentTask }),
    setAgentFiles: (agentFiles) => set({ agentFiles }),
    setAgentSteps: (agentSteps) => set({ agentSteps }),
    setAgentBlocked: (isAgentBlocked) => set({ isAgentBlocked }),

    // Extension Actions Implementation
    setInstalledExtensions: (installedExtensions) => set({ installedExtensions }),
    setMarketExtensions: (marketExtensions) => set({ marketExtensions }),
    setSearchingExtensions: (isSearchingExtensions) => set({ isSearchingExtensions }),
    addInstalledExtension: (extension) => set((state) => ({
        installedExtensions: [...state.installedExtensions.filter(e => e.id !== extension.id), extension]
    })),

    refreshInstalledExtensions: async () => {
        try {
            const extensions = await invoke<any[]>("get_running_extensions");
            set({ installedExtensions: extensions });

            // Also refresh icon theme and contributions as they depend on extensions
            const iconThemeMapping = await invoke<any>("get_icon_theme_mapping");
            if (iconThemeMapping && iconThemeMapping.iconDefinitions) {
                set({ iconThemeMapping });
            }

            const contributions = await invoke<any>("get_extension_contributions");
            if (contributions) {
                set({ extensionContributions: contributions });
            }
        } catch (err) {
            console.error("Failed to refresh installed extensions:", err);
        }
    },

    refreshPopularExtensions: async () => {
        try {
            const extensions = await invoke<any[]>("get_popular_extensions");
            set({ popularExtensions: extensions });
        } catch (err) {
            console.error("Failed to refresh popular extensions:", err);
        }
    },

    searchExtensions: async (query: string) => {
        if (!query) {
            set({ marketExtensions: [], isSearchingExtensions: false });
            return;
        }
        set({ isSearchingExtensions: true });
        try {
            const results = await invoke<any[]>("search_extensions", { query });
            set({ marketExtensions: results, isSearchingExtensions: false });
        } catch (err) {
            console.error("Marketplace search failed:", err);
            set({ isSearchingExtensions: false });
        }
    },

    requestExtensionTrust: (publisher, name, version) => {
        // Check if publisher is already trusted
        if (get().trustedPublishers.includes(publisher)) {
            return Promise.resolve(true);
        }

        return new Promise((resolve) => {
            set({ extensionTrustRequest: { publisher, name, version, onResolve: resolve } });
        });
    },

    resolveExtensionTrust: (trusted, always) => {
        const { extensionTrustRequest } = get();
        if (extensionTrustRequest) {
            if (trusted && always) {
                get().addTrustedPublisher(extensionTrustRequest.publisher);
            }
            extensionTrustRequest.onResolve(trusted);
            set({ extensionTrustRequest: null });
        }
    },

    addTrustedPublisher: (publisher) => set((state) => {
        const trustedPublishers = [...new Set([...state.trustedPublishers, publisher])];
        localStorage.setItem('trustedPublishers', JSON.stringify(trustedPublishers));
        return { trustedPublishers };
    }),

    removeTrustedPublisher: (publisher) => set((state) => {
        const trustedPublishers = state.trustedPublishers.filter(p => p !== publisher);
        localStorage.setItem('trustedPublishers', JSON.stringify(trustedPublishers));
        return { trustedPublishers };
    }),

    setSelectedExtensionId: (id) => set({ selectedExtensionId: id }),

    fetchExtensionDetails: async (id) => {
        try {
            const details = await invoke<any>("get_extension_details", { id });
            set((state) => ({
                extensionDetails: { ...state.extensionDetails, [id]: details }
            }));
        } catch (err) {
            console.error("Failed to fetch extension details:", err);
        }
    },

    installExtension: async (publisher, name, version) => {
        try {
            await invoke("install_extension", { publisher, name, version });
            await get().refreshInstalledExtensions();

            // Auto-apply if it's a Doki theme
            if (name.toLowerCase().includes('doki')) {
                console.log(`Auto-applying Doki theme: ${name}`);
                setTimeout(() => {
                    initTheme();
                }, 500); // Small delay to ensure extension files are ready
            }

            return true;
        } catch (err) {
            console.error("Installation failed:", err);
            return false;
        }
    },

    uninstallExtension: async (publisher, name, version) => {
        try {
            await invoke("uninstall_extension", { publisher, name, version });
            await get().refreshInstalledExtensions();
            return true;
        } catch (err) {
            console.error("Uninstallation failed:", err);
            return false;
        }
    },
    getFlattenedFiles: () => {
        const flatten = (entries: FileEntry[]): FileEntry[] => {
            let res: FileEntry[] = [];
            for (const e of entries) {
                if (!e.is_dir) res.push(e);
                if (e.children) res.push(...flatten(e.children));
            }
            return res;
        };
        return flatten(get().fileTree);
    },

    fetchWorkspaceMemory: async (category: string) => {
        try {
            const slots = await invoke<SemanticSlot[]>('query_workspace_memory', { category });
            set({ contextSlots: slots });
        } catch (error) {
            console.error('Fetch Workspace Memory Error:', error);
        }
    },

    fetchFileContext: async (path: string) => {
        try {
            const context = await invoke<any>('get_file_context', { filePath: path });
            set({ activeFileContext: context });
        } catch (error) {
            console.error('Fetch File Context Error:', error);
        }
    },

    setSpecsWizardOpen: (open: boolean) => set({ isSpecsWizardOpen: open }),
    setSpecsWizardStep: (step: 'generator' | 'status' | 'project') => set({ specsWizardStep: step }),
    setCurrentSpecProjectId: (id: number | null) => set({ currentSpecProjectId: id }),
    setPhase: (phase, status) => set({ currentPhase: phase, currentPhaseStatus: status }),

    refreshChatSessions: async () => {
        try {
            const sessions = await invoke<any[]>('list_chat_sessions');
            set({ chatSessions: sessions });
        } catch (error) {
            console.error('Refresh Chat Sessions Error:', error);
        }
    },

    loadChatSession: async (path: string) => {
        try {
            await invoke('load_chat_session', { path });
            // Retrieve messages from the newly loaded store
            const messages = await invoke<any[]>('get_agent_messages'); // Assuming this exists or I need to add it
            set({ agentMessages: messages });
            // Refresh sessions list to show current
            get().refreshChatSessions();
        } catch (error) {
            console.error('Load Chat Session Error:', error);
        }
    },

    archiveCurrentSession: async () => {
        try {
            await invoke('archive_chat_session');
            get().refreshChatSessions();
        } catch (error) {
            console.error('Archive Chat Session Error:', error);
        }
    },

    createNewSession: async () => {
        try {
            await invoke('create_new_session');
            get().clearAgentMessages();
            get().refreshChatSessions();
        } catch (error) {
            console.error('Create New Session Error:', error);
        }
    },

    refreshBrainTelemetry: async () => {
        try {
            const telemetry = await invoke<any>('get_brain_telemetry');
            set({ brainTelemetry: telemetry });
        } catch (error) {
            console.error('Refresh Brain Telemetry Error:', error);
        }
    },

    addKairosSuggestion: (suggestion: any) => {
        set((state: any) => ({
            kairosSuggestions: [suggestion, ...state.kairosSuggestions].slice(0, 50)
        }));
    },

    // ── Void-style per-feature model selection ────────────────────────────
    modelSelectionOfFeature: (() => {
        try {
            const saved = localStorage.getItem('void.modelSelectionOfFeature');
            return saved ? JSON.parse(saved) : { ...defaultModelSelectionOfFeature };
        } catch { return { ...defaultModelSelectionOfFeature }; }
    })(),
    modelSelectionOptions: {},
    setModelSelectionForFeature: (feature: FeatureName, selection: ModelSelection | null) => {
        set((state: any) => {
            const next = { ...state.modelSelectionOfFeature, [feature]: selection };
            try { localStorage.setItem('void.modelSelectionOfFeature', JSON.stringify(next)); } catch { /* */ }
            return { modelSelectionOfFeature: next };
        });
    },
    setModelSelectionOptions: (feature: FeatureName, providerName: ProviderName, modelName: string, opts: ModelSelectionOptions) => {
        set((state: any) => ({
            modelSelectionOptions: {
                ...state.modelSelectionOptions,
                [feature]: {
                    ...(state.modelSelectionOptions[feature] || {}),
                    [providerName]: {
                        ...((state.modelSelectionOptions[feature] || {})[providerName] || {}),
                        [modelName]: opts,
                    },
                },
            },
        }));
    },

    // ── Void-style global settings ────────────────────────────────────────
    voidGlobalSettings: (() => {
        try {
            const saved = localStorage.getItem('void.globalSettings');
            return saved ? { ...defaultGlobalSettings, ...JSON.parse(saved) } : { ...defaultGlobalSettings };
        } catch { return { ...defaultGlobalSettings }; }
    })(),
    setVoidGlobalSetting: <K extends keyof GlobalSettings>(key: K, value: GlobalSettings[K]) => {
        set((state: any) => {
            const next = { ...state.voidGlobalSettings, [key]: value };
            try { localStorage.setItem('void.globalSettings', JSON.stringify(next)); } catch { /* */ }
            return { voidGlobalSettings: next };
        });
    },

    // ── Reasoning state ───────────────────────────────────────────────────
    currentReasoningBudget: parseInt(localStorage.getItem('reasoning.budget') || '1024'),
    currentReasoningEffort: localStorage.getItem('reasoning.effort') || 'low',
    isReasoningEnabled: localStorage.getItem('reasoning.enabled') === '1',
    setReasoningBudget: (v: number) => {
        try { localStorage.setItem('reasoning.budget', String(v)); } catch { /* */ }
        set({ currentReasoningBudget: v });
    },
    setReasoningEffort: (v: string) => {
        try { localStorage.setItem('reasoning.effort', v); } catch { /* */ }
        set({ currentReasoningEffort: v });
    },
    setReasoningEnabled: (v: boolean) => {
        try { localStorage.setItem('reasoning.enabled', v ? '1' : '0'); } catch { /* */ }
        set({ isReasoningEnabled: v });
    },

    // ── Extra provider settings ────────────────────────────────────────────
    vllmUrl: localStorage.getItem('provider.vllm.url') || 'http://localhost:8000',
    lmStudioUrl: localStorage.getItem('provider.lmstudio.url') || 'http://localhost:1234',
    liteLLMUrl: localStorage.getItem('provider.litellm.url') || '',
    liteLLMApiKey: localStorage.getItem('provider.litellm.apikey') || '',
    googleVertexProject: localStorage.getItem('provider.vertex.project') || '',
    googleVertexRegion: localStorage.getItem('provider.vertex.region') || 'us-west2',
    azureProject: localStorage.getItem('provider.azure.project') || '',
    azureApiKey: localStorage.getItem('provider.azure.apikey') || '',
    azureApiVersion: localStorage.getItem('provider.azure.apiversion') || '2024-05-01-preview',
    awsBedrockApiKey: localStorage.getItem('provider.bedrock.apikey') || '',
    awsBedrockRegion: localStorage.getItem('provider.bedrock.region') || 'us-east-1',
    awsBedrockEndpoint: localStorage.getItem('provider.bedrock.endpoint') || '',
    setVllmUrl: (url: string) => { try { localStorage.setItem('provider.vllm.url', url); } catch { /* */ } set({ vllmUrl: url }); },
    setLmStudioUrl: (url: string) => { try { localStorage.setItem('provider.lmstudio.url', url); } catch { /* */ } set({ lmStudioUrl: url }); },
    setLiteLLMUrl: (url: string) => { try { localStorage.setItem('provider.litellm.url', url); } catch { /* */ } set({ liteLLMUrl: url }); },
    setLiteLLMApiKey: (k: string) => { try { localStorage.setItem('provider.litellm.apikey', k); } catch { /* */ } set({ liteLLMApiKey: k }); },
    setGoogleVertexProject: (v: string) => { try { localStorage.setItem('provider.vertex.project', v); } catch { /* */ } set({ googleVertexProject: v }); },
    setGoogleVertexRegion: (v: string) => { try { localStorage.setItem('provider.vertex.region', v); } catch { /* */ } set({ googleVertexRegion: v }); },
    setAzureProject: (v: string) => { try { localStorage.setItem('provider.azure.project', v); } catch { /* */ } set({ azureProject: v }); },
    setAzureApiKey: (k: string) => { try { localStorage.setItem('provider.azure.apikey', k); } catch { /* */ } set({ azureApiKey: k }); },
    setAzureApiVersion: (v: string) => { try { localStorage.setItem('provider.azure.apiversion', v); } catch { /* */ } set({ azureApiVersion: v }); },
    setAwsBedrockApiKey: (k: string) => { try { localStorage.setItem('provider.bedrock.apikey', k); } catch { /* */ } set({ awsBedrockApiKey: k }); },
    setAwsBedrockRegion: (v: string) => { try { localStorage.setItem('provider.bedrock.region', v); } catch { /* */ } set({ awsBedrockRegion: v }); },
    setAwsBedrockEndpoint: (v: string) => { try { localStorage.setItem('provider.bedrock.endpoint', v); } catch { /* */ } set({ awsBedrockEndpoint: v }); },

    // ── Antigravity: Workflow/Rule editor ────────────────────────────────
    openWorkflowFiles: [],
    openRuleFiles: [],
    openWorkflowFile: (path: string) => set((state: any) => ({
        openWorkflowFiles: state.openWorkflowFiles.includes(path) ? state.openWorkflowFiles : [...state.openWorkflowFiles, path]
    })),
    closeWorkflowFile: (path: string) => set((state: any) => ({
        openWorkflowFiles: state.openWorkflowFiles.filter((p: string) => p !== path)
    })),
    openRuleFile: (path: string) => set((state: any) => ({
        openRuleFiles: state.openRuleFiles.includes(path) ? state.openRuleFiles : [...state.openRuleFiles, path]
    })),
    closeRuleFile: (path: string) => set((state: any) => ({
        openRuleFiles: state.openRuleFiles.filter((p: string) => p !== path)
    })),

    // ── Antigravity: Agent hunk navigation ───────────────────────────────
    focusedHunkId: null,
    setFocusedHunk: (id: string | null) => set({ focusedHunkId: id }),
    acceptFocusedHunk: () => {
        const { focusedHunkId, pendingChanges } = get();
        if (!focusedHunkId) return;
        // find the change containing this hunk and accept it
        for (const change of pendingChanges) {
            if (change.id === focusedHunkId) {
                get().acceptPendingChange(change.id);
                break;
            }
        }
    },
    rejectFocusedHunk: () => {
        const { focusedHunkId, pendingChanges } = get();
        if (!focusedHunkId) return;
        for (const change of pendingChanges) {
            if (change.id === focusedHunkId) {
                get().rejectPendingChange(change.id);
                break;
            }
        }
        set({ focusedHunkId: null });
    },

    // ── Antigravity: AI commit message ────────────────────────────────────
    isGeneratingCommitMessage: false,
    lastGeneratedCommitMessage: '',
    generateAiCommitMessage: async () => {
        set({ isGeneratingCommitMessage: true });
        try {
            const diff = await invoke<string>('get_git_diff', { staged: true }).catch(() => '');
            if (!diff.trim()) {
                set({ isGeneratingCommitMessage: false });
                return '';
            }
            const state = get();
            const model = state.agentModel || '';
            const provider = model.includes('|') ? model.split('|')[0].toLowerCase() : (state.inferenceBackend === 'ollama' ? 'ollama' : 'openai');
            const result = await invoke<{ content: string }>('ai_chat_fast', {
                request: {
                    messages: [{
                        role: 'user',
                        content: `Write a concise git commit message for this diff. Format: <type>(<scope>): <description>\\n\\n<body if needed>\\n\\nTypes: feat|fix|refactor|docs|test|chore\\n\\nDiff:\\n${diff.slice(0, 4000)}`,
                    }],
                    model: model.includes('|') ? model.split('|').slice(1).join('|') : model,
                    provider,
                    temperature: 0.3,
                    ollama_url: state.ollamaUrl,
                }
            }).catch(() => null);
            const msg = result?.content?.trim() || '';
            set({ lastGeneratedCommitMessage: msg, isGeneratingCommitMessage: false });
            return msg;
        } catch {
            set({ isGeneratingCommitMessage: false });
            return '';
        }
    },

    // ── Antigravity: Import settings ─────────────────────────────────────
    importFromEditor: async (source: 'vscode' | 'cursor' | 'windsurf' | 'cider') => {
        try {
            await invoke('import_editor_settings', { source });
        } catch (e) {
            console.warn('[ImportSettings] not implemented yet:', e);
        }
    },

    // ── Antigravity: Demo mode ────────────────────────────────────────────
    isDemoMode: false,
    startDemoMode: () => set({ isDemoMode: true }),
    endDemoMode: () => set({ isDemoMode: false }),
});

export const useStore = create<AppState>(storeImplementation);

function findNodeRecursive(nodes: FileEntry[], path: string): FileEntry | undefined {
    for (const node of nodes) {
        if (node.path === path) return node;
        if (node.children) {
            const found = findNodeRecursive(node.children, path);
            if (found) return found;
        }
    }
    return null;
}

function injectChildrenRecursive(nodes: FileEntry[], path: string, children: FileEntry[]): FileEntry[] {
    return nodes.map(node => {
        if (node.path === path) {
            return { ...node, children };
        }
        if (node.children) {
            return { ...node, children: injectChildrenRecursive(node.children, path, children) };
        }
        return node;
    });
}

// Initialize listeners
if (typeof window !== 'undefined') {
    listen('sentient://patch_staged', (event: any) => {
        console.log('Patch staged event received:', event);
        const { path, diff, originalContent } = event.payload;
        const state = useStore.getState() as any;

        // Find existing pending change for this path
        const existingIndex = state.pendingChanges.findIndex((c: any) => c.path === path);

        if (existingIndex !== -1) {
            // Update existing change
            const updatedChanges = [...state.pendingChanges];
            updatedChanges[existingIndex] = {
                ...updatedChanges[existingIndex],
                proposedContent: diff,
                newContent: diff,
                originalContent
            };
            useStore.setState({ pendingChanges: updatedChanges });
        } else {
            // Add new pending change
            state.proposePendingChange({
                path,
                originalContent,
                proposedContent: diff,
                description: 'Sentient AI Surgical Patch'
            });
        }
    });

    // KAIROS fires suggestions on every idle tick — logging each one floods
    // the console. Keep the log but throttle to once per minute, and skip
    // entirely unless localStorage.kairos.debug is enabled.
    let _kairosLastLog = 0;
    listen('kairos://suggestion', (event: any) => {
        const debugOn = (() => {
            try { return localStorage.getItem('kairos.debug') === '1'; } catch { return false; }
        })();
        const now = Date.now();
        if (debugOn && now - _kairosLastLog > 60_000) {
            _kairosLastLog = now;
            console.log('[KAIROS] Suggestion received (throttled, 1/min):', event.payload);
        }
        const state = useStore.getState() as any;
        state.addKairosSuggestion(event.payload);
    });

    (window as any).useStore = useStore;
}
