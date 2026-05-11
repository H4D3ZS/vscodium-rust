/**
 * HADES-Ollama Integration Service
 * 
 * Wraps Ollama API with HADES intelligence layer
 */

import { useStore } from './store';
import { invoke } from './tauri_bridge';
import { routePrompt, recordRequest, type RoutingResult } from './kortex/ccet';

const DEMO_FALLBACK_OLLAMA_TOKEN = '94d92f5148bb721b16d310e8bcedac54ceeca26428feefd01bdd89bc07592a76';

const DEMO_FALLBACK_OLLAMA_TOKEN = '94d92f5148bb721b16d310e8bcedac54ceeca26428feefd01bdd89bc07592a76';

export interface HadesOllamaConfig {
  baseUrl: string;
  model: string;
  aimVfsEnabled: boolean;
}

export interface OllamaResponse {
  model: string;
  response?: string;
  message?: { role: string; content: string };
  done: boolean;
}

class HadesOllamaService {
  private config: HadesOllamaConfig = {
    baseUrl: 'http://localhost:11434',
    model: 'llama3.2',
    aimVfsEnabled: true,
  };

  private activeRequests = 0;
  private readonly MAX_CONCURRENT = 4;
  private modelCache: any = null;
  private lastCacheTime = 0;

  constructor() {
    this.loadConfig();
  }

  private getAuthHeader(): Record<string, string> {
    try {
      const token = localStorage.getItem('ollamaBearerToken') || '';
      if (token.trim()) {
        return { Authorization: `Bearer ${token.trim()}` };
      }
      const rawUrl = useStore.getState().ollamaUrl || '';
      const parsed = new URL(rawUrl);
      const tokenFromUrl =
        parsed.searchParams.get('token') ||
        parsed.searchParams.get('api_key') ||
        parsed.searchParams.get('bearer');
      if (tokenFromUrl && tokenFromUrl.trim()) {
        localStorage.setItem('ollamaBearerToken', tokenFromUrl.trim());
        return { Authorization: `Bearer ${tokenFromUrl.trim()}` };
      }
      if (DEMO_FALLBACK_OLLAMA_TOKEN.trim()) {
        return { Authorization: `Bearer ${DEMO_FALLBACK_OLLAMA_TOKEN}` };
      }
    } catch { }
    return {};
  }

  private loadConfig() {
    const s = useStore.getState();
    const rawModel = s.agentModel || '';
    const resolvedModel = rawModel.includes('|') ? rawModel.split('|')[1] : rawModel;
    let normalizedBaseUrl = s.ollamaUrl || 'http://localhost:11434';
    try {
      const parsed = new URL(normalizedBaseUrl);
      parsed.search = '';
      parsed.hash = '';
      normalizedBaseUrl = parsed.toString().replace(/\/$/, '');
    } catch { }
    this.config = {
      baseUrl: normalizedBaseUrl,
      model: resolvedModel || 'llama3.2',
      aimVfsEnabled: true,
    };
  }

  async generate(prompt: string, options?: any): Promise<OllamaResponse> {
    this.loadConfig();
    while (this.activeRequests >= this.MAX_CONCURRENT) {
      await new Promise(r => setTimeout(r, 100));
    }
    this.activeRequests++;
    try {
      const targetModel = options?.model || this.config.model;
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), options?.timeout || 300000);

      let finalPrompt = prompt;
      if (this.config.aimVfsEnabled) {
        try {
          const aimContext = await invoke<string>('hades_aim_get_context', { input: prompt.substring(0, 500) });
          if (aimContext) finalPrompt = `[Context: ${aimContext}]\n\n${prompt}`;
        } catch { }
      }

      const response = await fetch(`${this.config.baseUrl}/api/generate`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...this.getAuthHeader() },
        signal: controller.signal,
        body: JSON.stringify({
          model: targetModel,
          prompt: finalPrompt,
          system: options?.system,
          stream: false,
          images: options?.images,
          keep_alive: -1
        }),
      });
      clearTimeout(timeoutId);
      if (!response.ok) throw new Error(`Ollama error: ${response.status}`);
      return await response.json();
    } finally {
      this.activeRequests--;
    }
  }

  async chat(messages: any[], options?: any): Promise<OllamaResponse> {
    this.loadConfig();
    while (this.activeRequests >= this.MAX_CONCURRENT) {
      await new Promise(r => setTimeout(r, 100));
    }
    this.activeRequests++;
    try {
      const targetModel = options?.model || this.config.model;
      const response = await fetch(`${this.config.baseUrl}/api/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...this.getAuthHeader() },
        body: JSON.stringify({
          model: targetModel,
          messages,
          stream: false,
          keep_alive: -1
        }),
      });
      if (!response.ok) throw new Error(`Ollama chat error: ${response.status}`);
      return await response.json();
    } finally {
      this.activeRequests--;
    }
  }

  async *chatStream(messages: any[], options?: any): AsyncGenerator<string> {
    this.loadConfig();
    while (this.activeRequests >= this.MAX_CONCURRENT) {
      await new Promise(r => setTimeout(r, 100));
    }
    this.activeRequests++;
    const t0 = (typeof performance !== 'undefined' ? performance.now() : Date.now());
    let inputChars = 0;
    let activeChars = 0;
    let outputChars = 0;
    let route: RoutingResult | null = null;
    // Ollama's terminal `done:true` chunk carries precise token stats. We
    // capture them here for the throughput tracker.
    let evalCount = 0;
    let evalDurationNs = 0;
    let promptEvalCount = 0;
    let promptEvalDurationNs = 0;
    let ttftMs = 0;
    let sawFirstToken = false;
    const preStats = useStore.getState().kvCacheStats;

    try {
      const targetModel = options?.model || this.config.model;

      // Apply CCET token routing if the user has it enabled. We only touch
      // the trailing user turn — the assistant/system turns are preserved
      // exactly so the model's role conditioning isn't broken. The shipped
      // routing is a heuristic v1 (see src/kortex/ccet.ts).
      let routedMessages = messages;
      try {
        const s = useStore.getState();
        if (s.ccetEnabled && Array.isArray(messages) && messages.length > 0) {
          const last = messages[messages.length - 1];
          if (last && typeof last.content === 'string' && last.content.length > 800) {
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
        // CCET is best-effort; routing failures fall through to the original prompt.
      }

      const response = await fetch(`${this.config.baseUrl}/api/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...this.getAuthHeader() },
        body: JSON.stringify({
          model: targetModel,
          messages: routedMessages,
          stream: true,
          keep_alive: -1
        }),
      });

      if (!response.ok) throw new Error(`Ollama stream error: ${response.status}`);

      const reader = response.body?.getReader();
      if (!reader) return;

      const decoder = new TextDecoder();
      while (true) {
        const { done, value } = await reader.read();
        if (done) break;

        const chunk = decoder.decode(value, { stream: true });
        const lines = chunk.split('\n');

        for (const line of lines) {
          if (!line.trim()) continue;
          try {
            const json = JSON.parse(line);
            if (json.message?.content) {
              if (!sawFirstToken && json.message.content.length > 0) {
                sawFirstToken = true;
                const now = (typeof performance !== 'undefined' ? performance.now() : Date.now());
                ttftMs = Math.max(1, Math.round(now - t0));
              }
              outputChars += json.message.content.length;
              yield json.message.content;
            }
            if (json.done) {
              evalCount = Number(json.eval_count ?? 0);
              evalDurationNs = Number(json.eval_duration ?? 0);
              promptEvalCount = Number(json.prompt_eval_count ?? 0);
              promptEvalDurationNs = Number(json.prompt_eval_duration ?? 0);
              return;
            }
          } catch {
            // console.warn('Failed to parse chunk:', line);
          }
        }
      }
    } finally {
      this.activeRequests--;
      const t1 = (typeof performance !== 'undefined' ? performance.now() : Date.now());
      const wallMs = Math.max(1, Math.round(t1 - t0));

      // Record an η sample if CCET routing actually applied to this request.
      if (route) {
        try {
          recordRequest({
            request_id: `${Date.now()}-${Math.floor(Math.random() * 1e6)}`,
            model: options?.model || this.config.model,
            input_chars: inputChars,
            active_chars: activeChars,
            output_chars: outputChars,
            wall_clock_ms: wallMs,
            routing_counts: route.counts,
            saved_fraction: route.saved_fraction,
          });
        } catch {
          // Don't let metric bookkeeping leak into the chat result.
        }
      }

      // Live throughput sample. Ollama gives us ground-truth eval_count and
      // eval_duration on the terminal chunk — use them when present, fall back
      // to character approximation otherwise.
      try {
        const s = useStore.getState();
        let outputTokens = evalCount > 0 ? evalCount : Math.max(1, Math.round(outputChars / 4));
        let inputTokens = promptEvalCount > 0
          ? promptEvalCount
          : Math.max(0, Math.round((activeChars || inputChars) / 4));

        // Prefer Ollama's prompt_eval_duration for prefill timing; fall back
        // to the TTFT we measured client-side.
        let prefillMs = 0;
        if (promptEvalDurationNs > 0) {
          prefillMs = Math.max(1, Math.round(promptEvalDurationNs / 1_000_000));
        } else if (ttftMs > 0) {
          prefillMs = ttftMs;
        }

        // Effective wall clock for decode-only tok/s when ollama reports it.
        const effectiveWallMs = evalDurationNs > 0
          ? Math.max(1, Math.round(evalDurationNs / 1_000_000))
          : wallMs;

        const postStats = useStore.getState().kvCacheStats;
        let cacheHit = false;
        let tokensSkipped = 0;
        if (preStats && postStats) {
          if (postStats.hits > preStats.hits) cacheHit = true;
          tokensSkipped = Math.max(0, postStats.tokens_skipped - preStats.tokens_skipped);
        }

        s.recordKortexCompletion({
          wall_clock_ms: effectiveWallMs,
          prefill_ms: prefillMs > 0 ? prefillMs : undefined,
          output_tokens: outputTokens,
          input_tokens: inputTokens,
          backend: 'ollama',
          cache_hit: cacheHit,
          tokens_skipped: tokensSkipped,
          model_id: options?.model || this.config.model,
        });
      } catch {
        // Telemetry must never break the inference path.
      }
    }
  }

  async list(): Promise<any> {
    if (this.modelCache && Date.now() - this.lastCacheTime < 60000) {
      return this.modelCache;
    }
    this.loadConfig();
    try {
      const response = await fetch(`${this.config.baseUrl}/api/tags`, { headers: { ...this.getAuthHeader() } });
      if (response.ok) {
        this.modelCache = await response.json();
        this.lastCacheTime = Date.now();
        return this.modelCache;
      }
    } catch { }
    return { models: [] };
  }

  async tags(): Promise<any> {
    return this.list();
  }
}

export const hadesOllama = new HadesOllamaService();
