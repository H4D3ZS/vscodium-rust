import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { computeDiffBlocks, patchContentSelective } from './services/DiffService';
import { terminalManager, getVSCodeTheme } from './terminal';
import { initTheme } from './theme_engine';

const DEFAULT_REMOTE_OLLAMA_URL = 'http://129.212.185.15:11434/v1';
const LOCAL_PROXY_URL = 'http://localhost:1536';
const LOCAL_DIRECT_URL = 'http://localhost:11434';

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
    isAgentThinking: boolean;
    isAgentPaused: boolean;
    isYoloMode: boolean;
    agentUiMode: 'chat' | 'airi';
    avatarCharacter: string; // Selected AI avatar character
    avatarCustomConfig?: { stickerUrl?: string; wallpaperUrl?: string; enabled?: boolean }; // Custom 2D avatar URLs
    avatar3dConfig?: { modelUrl?: string; modelId?: string; customModels?: Array<{ id: string; name: string; url: string }> }; // 3D VRM avatar config
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

    // Inference Backend Configuration
    inferenceBackend: 'ollama' | 'llama-cpp' | 'openai';
    llamaCppUrl: string;
    llamaCppModelPath: string;
    llamaCppNgl: number;
    llamaCppHadesEnabled: boolean;
    ollamaConnectionMode: 'proxy' | 'direct';  // proxy=1536 (AIM), direct=11434
    ollamaMode: 'local' | 'cloud' | 'auto';  // Hybrid backend mode

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
    openAiriPanel: () => void;
    closeAiriPanel: () => void;
    openEmulatorPanel: () => void;
    closeEmulatorPanel: () => void;
    toggleAiriPanel: () => void;
    toggleEmulatorPanel: () => void;
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
    checkOllamaStatus: () => Promise<void>;
    pullOllamaModel: (name: string) => Promise<void>;
    setInferenceBackend: (backend: 'ollama' | 'llama-cpp' | 'openai') => void;
    setLlamaCppUrl: (url: string) => void;
    setLlamaCppModelPath: (path: string) => void;
    setLlamaCppNgl: (ngl: number) => void;
    setLlamaCppHadesEnabled: (enabled: boolean) => void;
    checkLlamaCppStatus: () => Promise<void>;
    openSettings: () => void;
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
    setEmulatorPosition: (pos: 'android' | 'iphone') => void;
    setYoloMode: (enabled: boolean) => void;
    setAgentUiMode: (mode: 'chat' | 'airi') => void;
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
    setAvatarCharacter: (id: string) => void;
    setAvatarCustomConfig: (config: any) => void;
    setAvatar3dConfig: (config: any) => void;
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
    agentMode: 'Chat',
    agentModel: 'Ollama|huihui_ai/qwen3.5-abliterated:35b', // Default to 35B model which exists in registry
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
    isAgentThinking: false,
    isAgentPaused: false,
    isYoloMode: false,
    agentUiMode: (localStorage.getItem('agentUiMode') as 'chat' | 'airi') || 'airi',
    avatarCharacter: localStorage.getItem('avatarCharacter') || 'airi',
    avatarCustomConfig: JSON.parse(localStorage.getItem('avatarCustomConfig') || '{}'),
    avatar3dConfig: JSON.parse(localStorage.getItem('avatar3dConfig') || '{}'),
    ollamaConnectionMode: (localStorage.getItem('ollamaConnectionMode') as 'proxy' | 'direct') || 'proxy',
    ollamaMode: (localStorage.getItem('ollamaMode') as 'local' | 'cloud' | 'auto') || 'auto',

    // Right sidebar panels
    isAiriPanelOpen: true,  // AIRI open by default
    isEmulatorPanelOpen: false,  // Emulator closed by default
    emulatorPanelPosition: 'android',

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

    ollamaUrl: localStorage.getItem('ollamaUrl') || DEFAULT_REMOTE_OLLAMA_URL,
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
    taskPlannerState: null,
    ghostRuntimeResults: [],
    currentThought: null,

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
        set((state) => {
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
    setAgentMode: (agentMode) => set({ agentMode }),
    setAgentModel: (agentModel) => set({ agentModel }),
    setAgentRootAccess: (_rootAccess: boolean) => {
        // Root access is now permanent and cannot be disabled
        set({ agentRootAccess: true });
    },
    setActiveRoot: (path) => {
        if (path) {
            const name = path.replace(/\\/g, '/').split('/').pop() || path;
            // Persist to localStorage so we can restore on next launch
            localStorage.setItem('activeRoot', path);
            localStorage.setItem('activeRootName', name);
            set({ activeRoot: path, activeRootName: name });
            // Sync with backend
            invoke('set_active_root', { path }).then(() => {
                get().refreshFileTree();
                get().fetchActiveProjectSpec();
            }).catch(console.error);
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
        set({ ollamaUrl: url });
        localStorage.setItem('ollamaUrl', url);
        if (url !== LOCAL_PROXY_URL) {
            localStorage.setItem('preferredDirectOllamaUrl', url);
        }
        invoke('set_ollama_url', { url }).catch(console.error);
    },
    setOllamaConnectionMode: (mode: 'proxy' | 'direct') => {
        const preferredDirectUrl = localStorage.getItem('preferredDirectOllamaUrl') || DEFAULT_REMOTE_OLLAMA_URL;
        const url = mode === 'proxy' ? LOCAL_PROXY_URL : preferredDirectUrl;
        set({ ollamaConnectionMode: mode, ollamaUrl: url });
        localStorage.setItem('ollamaUrl', url);
        invoke('set_ollama_url', { url }).catch(console.error);
        // Persist mode to localStorage
        localStorage.setItem('ollamaConnectionMode', mode);
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
    },
    setLlamaCppUrl: (url: string) => {
        localStorage.setItem('llamaCppUrl', url);
        set({ llamaCppUrl: url });
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

    openSettings: () => {
        const settingsTab = get().tabs.find(t => t.type === 'settings');
        if (settingsTab) {
            set({ activeTabId: settingsTab.id });
            return;
        }
        const id = 'settings-tab';
        const tab: EditorTab = {
            id,
            filename: 'Settings',
            path: 'vscode://settings',
            content: '',
            isModified: false,
            language: '',
            type: 'settings'
        };
        set((state) => ({ tabs: [...state.tabs, tab], activeTabId: id }));
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
            if (keys.groq) providers.push('Groq');
            if (keys.xai) providers.push('xAI');
            if (keys.alibaba) providers.push('Alibaba');
            providers.push('ApiRadar'); // Always include for aggregated view

            let allModels: { id: string, provider: string }[] = [];

            // Fix case sensitivity and provider mapping
            const activeProviders = targetProvider
                ? [targetProvider.toLowerCase() === 'apiradar' ? 'ApiRadar' : targetProvider.charAt(0).toUpperCase() + targetProvider.slice(1).toLowerCase()]
                : providers;

            for (const p of activeProviders) {
                try {
                    if (p.toLowerCase() === 'ollama') {
                        // Prefer user-configured endpoint first (supports remote VPS), then local fallbacks.
                        const candidates = [ollamaUrl, LOCAL_DIRECT_URL, LOCAL_PROXY_URL].filter(
                            (value, index, self) => Boolean(value) && self.indexOf(value) === index
                        );
                        let ollamaToUse = ollamaUrl;
                        let connected = false;

                        for (const candidate of candidates) {
                            const probeUrl = candidate.replace(/\/$/, '');
                            const controller = new AbortController();
                            const timeout = setTimeout(() => controller.abort(), 2500);
                            try {
                                const response = await fetch(`${probeUrl}/api/tags`, { signal: controller.signal });
                                clearTimeout(timeout);
                                if (response.ok) {
                                    ollamaToUse = candidate;
                                    connected = true;
                                    break;
                                }
                            } catch {
                                clearTimeout(timeout);
                            }
                        }

                        if (!connected) {
                            set({ ollamaStatus: 'error', ollamaMode: 'local' });
                        } else {
                            const usesProxy = ollamaToUse === LOCAL_PROXY_URL;
                            set({
                                ollamaConnectionMode: usesProxy ? 'proxy' : 'direct',
                                ollamaMode: usesProxy ? 'auto' : 'cloud'
                            });
                        }

                        await invoke('set_ollama_url', { url: ollamaToUse });
                        localStorage.setItem('ollamaUrl', ollamaToUse);
                        if (ollamaToUse !== LOCAL_PROXY_URL) {
                            localStorage.setItem('preferredDirectOllamaUrl', ollamaToUse);
                        }
                        set({ ollamaUrl: ollamaToUse });
                    }
                    const models = await invoke<string[]>('list_provider_models', { provider: p });
                    allModels = [...allModels, ...models.map(m => ({ id: m, provider: p.toLowerCase() }))];

                    if (p.toLowerCase() === 'ollama') {
                        if (models.length > 0) {
                            set({ ollamaStatus: 'running' });
                            // Auto-select first Ollama model if none selected or if current is invalid
                            const currentModel = get().agentModel;
                            const hasValidOllama = currentModel && currentModel.startsWith('Ollama|') && currentModel.split('|')[1];

                            const validModels = models.filter(m => m !== 'hades:latest');

                            if (models.length > 0 && !hasValidOllama) {
                                if (validModels.length > 0) {
                                    set({ agentModel: `Ollama|${validModels[0]}` });
                                }
                            }

                            // Explicitly clear legacy hades:latest if it's the current selection
                            if (get().agentModel === 'Ollama|hades:latest') {
                                console.warn('[Registry] Clearing legacy hades:latest model reference');
                                if (validModels.length > 0) {
                                    set({ agentModel: `Ollama|${validModels[0]}` });
                                } else {
                                    set({ agentModel: 'Ollama|huihui_ai/qwen3.5-abliterated:35b' });
                                }
                            }
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

        return { agentMessages: [...state.agentMessages, newMessage] };
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

            // Robust Overlap Merger:
            // Find the longest suffix of currentRaw that is a prefix of delta.
            let overlapLength = 0;
            const maxCheck = Math.min(currentRaw.length, delta.length);
            for (let i = 1; i <= maxCheck; i++) {
                if (currentRaw.slice(-i) === delta.slice(0, i)) {
                    overlapLength = i;
                }
            }

            const cleanDelta = delta.slice(overlapLength);
            if (cleanDelta.length === 0) return state;

            const fullRaw = currentRaw + cleanDelta;
            let newContent = currentContent;
            let newThoughts = currentThoughts;

            // Strip thinking blocks from visible content - thoughts are internal only
            if (fullRaw.includes('<think>')) {
                const thinkMatch = fullRaw.match(/<think>([\s\S]*?)<\/think>/);
                if (thinkMatch) {
                    // Store thought internally for UI state, but DON'T add to message
                    set({ currentThought: thinkMatch[1].trim() });
                    newContent = fullRaw.replace(/<think>[\s\S]*?<\/think>/g, '').trim();
                } else {
                    const parts = fullRaw.split('<think>');
                    newContent = parts[0] || '';
                }
            } else {
                newContent += cleanDelta;
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
    setEmulatorPosition: (emulatorPanelPosition) => set({ emulatorPanelPosition }),
    setYoloMode: (isYoloMode) => set({ isYoloMode }),
    setAgentUiMode: (agentUiMode) => { localStorage.setItem('agentUiMode', agentUiMode); set({ agentUiMode }); },
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
                // console.log("[Persistence] History refreshed from backend core.");
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

        // Create the terminal instance in the manager
        // createTerminal(profileId, groupId, cwd)
        const actualId = await terminalManager.createTerminal(shell, id);
        // Sync our local instanceId with the one generated/used by the manager if needed
        // but store.ts seems to want to use its own generated instanceId.
        // Actually, let's use the one from createTerminal.

        const newGroup: TerminalGroup = {
            id,
            name: `${name}`,
            instances: [actualId],
            activeInstanceId: actualId
        };

        set((state) => ({
            terminalGroups: [...state.terminalGroups, newGroup],
            activeTerminalGroupId: id,
            activePanelTab: 'TERMINAL',
            isBottomPanelOpen: true
        }));

        return id;
    },

    splitTerminal: async (groupId, instanceId) => {
        const newInstanceId = `term-${Date.now()}-${Math.random().toString(36).substr(2, 4)}`;

        // Get shell of current instance if possible
        const currentInstance = terminalManager.getTerminal(instanceId);
        const shell = currentInstance?.shell;

        // Create the terminal instance in the manager
        const actualNewId = await terminalManager.createTerminal(shell, groupId);

        set((state) => {
            const groups = state.terminalGroups.map(g => {
                if (g.id === groupId) {
                    return {
                        ...g,
                        instances: [...g.instances, actualNewId],
                        activeInstanceId: actualNewId
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

    listen('kairos://suggestion', (event: any) => {
        // console.log('[KAIROS] Suggestion received:', event.payload);
        const state = useStore.getState() as any;
        state.addKairosSuggestion(event.payload);
    });

    (window as any).useStore = useStore;
}
