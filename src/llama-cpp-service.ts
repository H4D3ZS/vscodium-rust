/**
 * Llama.cpp Backend Integration for VSCodium-Rust
 * 
 * Provides direct llama.cpp inference as an alternative to Ollama.
 * Integrates with Kortex GAC for geometry-aware 8GB VRAM scheduling.
 */

import {
  startKortexInference,
  stopServer as stopKortexServer,
  getRunningServer as getKortexRunningServer,
  summarizePlan,
  type Backend as KortexBackend,
  type KortexBootResult,
  type TierPlan,
} from './kortex/gac-orchestrator';
import {
  startKvCache,
  stopKvCache,
  makeKvCacheOptions,
  type KvCacheOptions,
} from './kortex/kvcache-orchestrator';
import { routePrompt, recordRequest, type RoutingResult } from './kortex/ccet';
import { useStore } from './store';

export interface LlamaCppConfig {
  enabled: boolean;
  modelPath: string;
  ngl: number;  // Number of layers to GPU (ignored when kortexEnabled — GAC computes the override-tensor layout instead)
  nThreads: number;
  nCtx: number;  // Context size
  batchSize: number;
  /** Legacy name; mirrored by `kortexEnabled` for the new GAC path. */
  hadesEnabled: boolean;
  hadesGistPath?: string;
  /** Use the Kortex GAC scheduler (geometry-of-consolidation profile + tier planner). */
  kortexEnabled?: boolean;
  /** Total physical VRAM in MB (e.g. 8192 for RX 580 8GB). */
  vramTotalMb?: number;
  /** Retrieval threshold theta from the GAC paper. Default 0.85. */
  kortexTheta?: number;
  /** GPU backend: vulkan (RX 580), cuda, rocm, metal, sycl. */
  kortexBackend?: KortexBackend;
  /** Optional explicit path to llama-server binary. */
  serverBinary?: string;
  /** Enable the Kortex Disk KV Cache proxy (ds4-style prefix reuse).
   *  When on, the IDE talks to the proxy instead of llama-server directly. */
  kvCacheEnabled?: boolean;
  /** Base directory for the proxy's index + slot binaries. Defaults to
   *  `<userprofile>/.kortex/kvcache`. Override per-machine if your slow disk
   *  needs a different location. */
  kvCacheBaseDir?: string;
  /** Total bytes budget for the cache. Default 16 GB. */
  kvCacheMaxBytes?: number;
  /** Port the KV cache proxy listens on. Default 8090. The IDE auto-points
   *  its inference URL at this port when the proxy is up. */
  kvCacheProxyPort?: number;
}

export interface LlamaCppStatus {
  status: 'disconnected' | 'connecting' | 'connected' | 'error';
  model?: string;
  vramUsage?: number;
  temp?: number;
  error?: string;
}

const DEFAULT_CONFIG: LlamaCppConfig = {
  enabled: false,
  modelPath: '',
  ngl: 99,  // Offload all layers to GPU
  nThreads: 8,
  nCtx: 4096,
  batchSize: 512,
  hadesEnabled: true,
  hadesGistPath: '',
  kortexEnabled: true,
  vramTotalMb: 8192,
  kortexTheta: 0.85,
  kortexBackend: 'vulkan',
  serverBinary: '',
  kvCacheEnabled: true,
  kvCacheBaseDir: '',
  kvCacheMaxBytes: 16 * 1024 * 1024 * 1024,
  kvCacheProxyPort: 8090,
};

class LlamaCppService {
  private config: LlamaCppConfig = DEFAULT_CONFIG;
  private status: LlamaCppStatus = { status: 'disconnected' };
  private baseUrl: string = 'http://localhost:8081';  // llama.cpp server default (changed from 8080)

  constructor() {
    this.loadConfig();
  }

  private loadConfig() {
    const saved = localStorage.getItem('llamaCppConfig');
    if (saved) {
      try {
        this.config = JSON.parse(saved);
      } catch (e) {
        console.error('Failed to load llama.cpp config:', e);
      }
    }
  }

  private saveConfig() {
    localStorage.setItem('llamaCppConfig', JSON.stringify(this.config));
  }

  /**
   * Configure llama.cpp backend
   */
  configure(config: Partial<LlamaCppConfig>) {
    this.config = { ...this.config, ...config };
    this.saveConfig();
  }

  /**
   * Check if llama.cpp server is running
   */
  async checkStatus(): Promise<LlamaCppStatus> {
    this.status = { status: 'connecting' };
    useStore.setState({ llamaCppStatus: 'checking' });

    try {
      const response = await fetch(`${this.baseUrl}/health`, {
        method: 'GET',
        signal: AbortSignal.timeout(3000),
      });

      if (response.ok) {
        this.status = {
          status: 'connected',
          model: 'llama.cpp',
        };
        useStore.setState({ llamaCppStatus: 'running' });
      } else {
        throw new Error('Server returned unhealthy status');
      }
    } catch (error) {
      this.status = {
        status: 'error',
        error: error instanceof Error ? error.message : 'Connection failed',
      };
      useStore.setState({ llamaCppStatus: 'error' });
    }

    return this.status;
  }

  /**
   * Generate completion using llama.cpp
   */
  async *generateCompletion(
    prompt: string,
    options?: {
      maxTokens?: number;
      temperature?: number;
      topP?: number;
      stop?: string[];
    }
  ): AsyncGenerator<string, void, unknown> {
    if (this.status.status !== 'connected') {
      throw new Error('llama.cpp not connected');
    }

    const response = await fetch(`${this.baseUrl}/completion`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        prompt,
        n_predict: options?.maxTokens || 512,
        temperature: options?.temperature ?? 0.7,
        top_p: options?.topP ?? 0.9,
        stop: options?.stop ?? [],
        stream: true,
      }),
    });

    if (!response.ok) {
      throw new Error(`llama.cpp error: ${response.statusText}`);
    }

    const reader = response.body?.getReader();
    if (!reader) return;

    const decoder = new TextDecoder();
    let buffer = '';

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });

      // Process SSE format
      const lines = buffer.split('\n');
      buffer = lines.pop() || '';

      for (const line of lines) {
        if (line.startsWith('data: ')) {
          const data = line.slice(6);
          if (data === '[DONE]') continue;

          try {
            const parsed = JSON.parse(data);
            if (parsed.content) {
              yield parsed.content;
            }
          } catch (e) {
            console.warn('Failed to parse SSE data:', e);
          }
        }
      }
    }
  }

  /**
   * Generate chat completion. Honours the CCET routing flag from the store —
   * when enabled and the trailing user turn is large, the prompt goes through
   * `routePrompt` first and an η metric is recorded after the stream closes.
   */
  async *generateChat(
    messages: Array<{ role: string; content: string }>,
    options?: {
      maxTokens?: number;
      temperature?: number;
    }
  ): AsyncGenerator<string, void, unknown> {
    let routedMessages = messages;
    let route: RoutingResult | null = null;
    let inputChars = 0;
    let activeChars = 0;
    let outputChars = 0;
    const t0 = (typeof performance !== 'undefined' ? performance.now() : Date.now());

    try {
      const s = useStore.getState();
      if (s.ccetEnabled && messages.length > 0) {
        const last = messages[messages.length - 1];
        if (last && last.content.length > 800) {
          inputChars = last.content.length;
          route = routePrompt(last.content, {
            tau_skip: s.ccetTauSkip,
            tau_compress: s.ccetTauCompress,
            max_skip_fraction: s.ccetMaxSkipFraction,
          });
          activeChars = route.output_text.length;
          routedMessages = [
            ...messages.slice(0, -1),
            { ...last, content: route.output_text },
          ];
        }
      }
    } catch {
      // CCET routing failures degrade to the original prompt.
    }

    // Convert messages to llama.cpp format
    const prompt = routedMessages
      .map((m) => `${m.role}: ${m.content}`)
      .join('\n\n') + '\nassistant:';

    try {
      for await (const chunk of this.generateCompletion(prompt, options)) {
        outputChars += chunk.length;
        yield chunk;
      }
    } finally {
      if (route) {
        const t1 = (typeof performance !== 'undefined' ? performance.now() : Date.now());
        try {
          recordRequest({
            request_id: `${Date.now()}-${Math.floor(Math.random() * 1e6)}`,
            model: this.config.modelPath.split(/[/\\]/).pop() ?? 'llama.cpp',
            input_chars: inputChars,
            active_chars: activeChars,
            output_chars: outputChars,
            wall_clock_ms: Math.max(1, Math.round(t1 - t0)),
            routing_counts: route.counts,
            saved_fraction: route.saved_fraction,
          });
        } catch {
          // Bookkeeping; ignore.
        }
      }
    }
  }

  /**
   * Get available models from llama.cpp
   */
  async getModels(): Promise<string[]> {
    try {
      const response = await fetch(`${this.baseUrl}/models`);
      if (!response.ok) return [];
      const data = await response.json();
      return data.models?.map((m: any) => m.name) || [];
    } catch {
      return [];
    }
  }

  /**
   * Enable/disable HADES Bridge integration
   */
  setHadesEnabled(enabled: boolean) {
    this.config.hadesEnabled = enabled;
    this.saveConfig();
  }

  /**
   * Get current configuration
   */
  getConfig(): LlamaCppConfig {
    return { ...this.config };
  }

  /**
   * Get current status
   */
  getStatus(): LlamaCppStatus {
    return { ...this.status };
  }

  /**
   * Start llama.cpp server with Kortex GAC scheduling.
   *
   * Flow: profile (or read cached) GGUF -> plan tier assignment against the
   * configured VRAM budget -> spawn llama-server with --override-tensor flags
   * derived from the geometry of each weight matrix.
   *
   * The first call on a new model takes ~30-60s for profiling. Subsequent
   * launches reuse the cached `<model>.geometry.aim` profile and start in
   * a few seconds.
   */
  async startServer(): Promise<KortexBootResult | void> {
    if (!this.config.modelPath) {
      throw new Error('startServer: modelPath is empty — set llamaCppModelPath first.');
    }
    if (this.config.kortexEnabled === false) {
      // Legacy path: no programmatic launch, the user must start llama.cpp themselves.
      console.warn('[llama-cpp] kortexEnabled=false; expecting an externally-managed llama-server on', this.baseUrl);
      throw new Error('Server start without Kortex GAC is no longer supported in-process — enable Kortex GAC or start llama-server manually.');
    }

    console.log('[llama-cpp] booting Kortex GAC inference path...');

    const kvCacheBase = this.kvCacheBaseDirOrDefault();
    const slotSavePath = this.config.kvCacheEnabled !== false
      ? `${kvCacheBase}/slots`
      : undefined;

    const opts = {
      model_path: this.config.modelPath,
      vram_total_mb: this.config.vramTotalMb ?? 8192,
      theta: this.config.kortexTheta ?? 0.85,
      backend: (this.config.kortexBackend ?? 'vulkan') as KortexBackend,
      launch: {
        port: parsePortFromUrl(this.baseUrl) ?? 8081,
        ctx_size: this.config.nCtx,
        n_threads: this.config.nThreads,
        batch_size: this.config.batchSize,
        server_binary: this.config.serverBinary && this.config.serverBinary.trim() !== ''
          ? this.config.serverBinary
          : undefined,
        slot_save_path: slotSavePath,
        wait_healthy_secs: 90,
      },
    };

    const result = await startKortexInference(opts);
    let baseUrl = result.base_url;

    // Boot the KDKVC proxy in front of llama-server so prefix reuse persists
    // across sessions. Failure here is non-fatal — the IDE can still talk
    // directly to llama-server, just without disk-resident KV cache hits.
    if (this.config.kvCacheEnabled !== false) {
      try {
        const upstream = result.base_url;
        const cacheOpts: KvCacheOptions = makeKvCacheOptions(kvCacheBase, {
          upstream_url: upstream,
          proxy_port: this.config.kvCacheProxyPort ?? 8090,
          max_bytes: this.config.kvCacheMaxBytes ?? 16 * 1024 * 1024 * 1024,
        });
        const port = await startKvCache(cacheOpts);
        baseUrl = `http://${cacheOpts.proxy_host}:${port}`;
        console.log(`[llama-cpp] Kortex KV cache proxy live at ${baseUrl} → ${upstream}`);
      } catch (e) {
        console.warn('[llama-cpp] KV cache proxy failed to start, falling through to direct llama-server:', e);
      }
    }

    this.baseUrl = baseUrl;
    this.status = {
      status: 'connected',
      model: this.config.modelPath.split(/[/\\]/).pop() ?? 'llama.cpp',
    };
    useStore.setState({ llamaCppStatus: 'running' });
    console.log('[llama-cpp] Kortex GAC plan:', summarizePlan(result.plan));
    return result;
  }

  /**
   * Resolve the on-disk base directory for the KV cache. Picks the user-set
   * value when present, otherwise puts everything under
   * `<USERPROFILE|HOME>/.kortex/kvcache`.
   */
  private kvCacheBaseDirOrDefault(): string {
    if (this.config.kvCacheBaseDir && this.config.kvCacheBaseDir.trim() !== '') {
      return this.config.kvCacheBaseDir;
    }
    const home = (typeof window !== 'undefined' && (window as any).process?.env?.USERPROFILE)
      || (typeof window !== 'undefined' && (window as any).process?.env?.HOME)
      || '.';
    return `${home}/.kortex/kvcache`;
  }

  /**
   * Stop the Kortex-managed llama-server, then fall back to /shutdown if a
   * different server was started externally.
   */
  async stopServer(): Promise<void> {
    // Tear the proxy down first so it doesn't try to talk to a dying upstream.
    try {
      await stopKvCache();
    } catch {
      // Proxy might not be running.
    }
    try {
      const running = await getKortexRunningServer();
      if (running) {
        await stopKortexServer();
      } else {
        await fetch(`${this.baseUrl}/shutdown`, { method: 'POST' });
      }
    } catch {
      // Server might already be stopped.
    }
    this.status = { status: 'disconnected' };
  }

  /** Returns the most recent GAC tier plan, if a Kortex-managed server is running. */
  async getKortexPlan(): Promise<TierPlan | null> {
    const info = await getKortexRunningServer();
    if (!info) return null;
    // We don't currently round-trip the plan through the running server info.
    // The caller can re-derive it via planTiers + the cached profile if needed.
    return null;
  }
}

function parsePortFromUrl(url: string): number | null {
  try {
    const u = new URL(url);
    if (u.port) return parseInt(u.port, 10);
    return null;
  } catch {
    return null;
  }
}

export const llamaCppService = new LlamaCppService();
