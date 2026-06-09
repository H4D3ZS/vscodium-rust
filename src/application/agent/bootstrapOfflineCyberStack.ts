import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';
import { applyLocalOllamaAgentDefaults } from '../../lib/localOllamaAgentDefaults';

const MIGRATION_KEY = 'ide.offline-cyber-boot-v1';

const isMac = typeof navigator !== 'undefined' && /Mac|iPhone|iPad/i.test(navigator.userAgent);

/** One-time defaults for 8 GB rigs: local inference, SSD KV cache, Metal GAC, pentest mode. */
export function applyOfflineCyberDefaults(): void {
    if (typeof localStorage === 'undefined') return;
    if (localStorage.getItem(MIGRATION_KEY)) return;

    const cloudOnboarded = localStorage.getItem('cyberifrit.cloudOnboarded') === '1';
    if (!cloudOnboarded && localStorage.getItem('ollamaServerMode') !== 'cloud') {
        localStorage.setItem('ollamaServerMode', 'local');
        localStorage.setItem('inferenceBackend', 'ollama');
    }

    localStorage.setItem('kortex.gacEnabled', '1');
    localStorage.setItem('kvcache.enabled', '1');
    localStorage.setItem('ccet.enabled', '1');
    localStorage.setItem('indexing.enabled', '1');
    localStorage.setItem('ollamaConnectionMode', 'proxy');

    if (!localStorage.getItem('kortex.backend')) {
        localStorage.setItem('kortex.backend', isMac ? 'metal' : 'vulkan');
    }
    if (!localStorage.getItem('kortex.vramTotalMb')) {
        localStorage.setItem('kortex.vramTotalMb', '8192');
    }
    // Disk budget on SSD — not RAM. 32 GB prefix cache is safe on modern SSDs.
    if (!localStorage.getItem('kvcache.maxBytes')) {
        localStorage.setItem('kvcache.maxBytes', String(32 * 1024 * 1024 * 1024));
    }

    if (!localStorage.getItem('agent.mode')) {
        localStorage.setItem('agent.mode', 'BugBounty');
    }
    localStorage.setItem('agent.plannerEnabled', '0');

    localStorage.setItem(MIGRATION_KEY, '1');
}

async function ensureKvCacheDirs(baseDir: string): Promise<void> {
    for (const sub of ['index', 'slots']) {
        try {
            await invoke('create_dir', { path: `${baseDir}/${sub}` });
        } catch { /* already exists */ }
    }
}

function resolveHomeDir(): string {
    const env = typeof window !== 'undefined'
        ? (window as { process?: { env?: { USERPROFILE?: string; HOME?: string } } }).process?.env
        : undefined;
    return env?.HOME || env?.USERPROFILE || '.';
}

async function resolveKvCacheBaseDir(): Promise<string> {
    const stored = (() => {
        try { return localStorage.getItem('kvcache.baseDir')?.trim() || ''; } catch { return ''; }
    })();
    if (stored) return stored;
    return `${resolveHomeDir()}/.kortex/kvcache`;
}

async function probeAimProxy(): Promise<void> {
    const proxyUrl = 'http://127.0.0.1:1536/api/tags';
    try {
        const res = await fetch(proxyUrl, { signal: AbortSignal.timeout(2500) });
        if (res.ok) {
            console.log('[offline-cyber] AIM proxy (:1536) reachable — zero-grep context injection active');
            return;
        }
    } catch { /* proxy down */ }

    try {
        const direct = await fetch('http://127.0.0.1:11434/api/tags', { signal: AbortSignal.timeout(2500) });
        if (direct.ok) {
            console.warn(
                '[offline-cyber] Ollama direct (:11434) OK but AIM proxy (:1536) offline. '
                + 'Run `kortex/target/release/aim-proxy` for .aim context injection and KV prefix caching.',
            );
        }
    } catch {
        console.warn('[offline-cyber] Ollama not reachable — start `ollama serve` for offline inference');
    }
}

/**
 * Boot local cyber stack: ANE, AIM/VFS index, offline model pick, SSD KV dirs.
 * Lightweight — does not spin up llama-server (that stays on-demand via GAC panel).
 */
function syncStoreFromOfflineDefaults(): void {
    const store = useStore.getState();
    try {
        const mode = localStorage.getItem('ollamaServerMode') as 'local' | 'cloud' | 'remote' | null;
        if (mode && store.ollamaServerMode !== mode) {
            store.setOllamaServerMode?.(mode);
        }
        store.setKvCacheEnabled?.(localStorage.getItem('kvcache.enabled') !== '0');
        store.setKortexGacEnabled?.(localStorage.getItem('kortex.gacEnabled') !== '0');
        const backend = localStorage.getItem('kortex.backend');
        if (backend) store.setKortexBackend?.(backend);
        const vram = parseInt(localStorage.getItem('kortex.vramTotalMb') || '8192', 10);
        if (!Number.isNaN(vram)) store.setKortexVramTotalMb?.(vram);
        const agentMode = localStorage.getItem('agent.mode');
        if (agentMode && store.agentMode !== agentMode) {
            store.setAgentMode?.(agentMode);
        }
    } catch { /* */ }
}

export async function bootstrapOfflineCyberStack(opts?: { heavy?: boolean }): Promise<void> {
    applyOfflineCyberDefaults();
    syncStoreFromOfflineDefaults();

    const store = useStore.getState();

    if (store.ollamaServerMode === 'local') {
        applyLocalOllamaAgentDefaults(store);
    }

    const mode = (() => {
        try { return localStorage.getItem('agent.mode') || store.agentMode; } catch { return store.agentMode; }
    })();
    if (mode === 'BugBounty' && store.agentMode !== 'BugBounty') {
        store.setAgentMode?.('BugBounty');
    }

    if (!opts?.heavy) return;

    if (store.ollamaServerMode === 'local') {
        void store.syncOllamaEndpoint?.();
    }

    if (isMac) {
        try {
            await invoke('ane_init_inference', {});
        } catch { /* lazy ANE */ }
    }

    void probeAimProxy();

    const indexingOn = (() => {
        try { return localStorage.getItem('indexing.enabled') !== '0'; } catch { return true; }
    })();
    if (indexingOn && store.activeRoot) {
        void invoke('trigger_workspace_index').catch(() => {});
        void invoke('aim_trust_manifest', { root: store.activeRoot, path: null }).catch(() => { });
    }

    if (store.kvCacheEnabled) {
        const baseDir = await resolveKvCacheBaseDir();
        try { localStorage.setItem('kvcache.baseDir', baseDir); } catch { /* */ }
        await ensureKvCacheDirs(baseDir);
    }

    const currentModel = (store.agentModel || '').trim();
    if (!currentModel && store.inferenceBackend === 'ollama' && store.ollamaServerMode === 'local') {
        try {
            const best = await invoke<string>('detect_best_model');
            if (best) {
                const tag = `Ollama|${best}`;
                store.setAgentModel?.(tag);
                try { localStorage.setItem('agentModel', tag); } catch { /* */ }
            }
        } catch { /* Ollama offline */ }
    }
}
