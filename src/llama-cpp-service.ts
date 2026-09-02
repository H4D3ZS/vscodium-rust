/**
 * Llama.cpp Backend Integration for VSCodium-Rust
 * 
 * Provides direct llama.cpp inference as an alternative to Ollama.
 * Integrates with HADES Bridge for 8GB VRAM optimization.
 */

import { useStore } from './store';

export interface LlamaCppConfig {
  enabled: boolean;
  modelPath: string;
  ngl: number;  // Number of layers to GPU
  nThreads: number;
  nCtx: number;  // Context size
  batchSize: number;
  hadesEnabled: boolean;
  hadesGistPath?: string;
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
   * Generate chat completion
   */
  async *generateChat(
    messages: Array<{ role: string; content: string }>,
    options?: {
      maxTokens?: number;
      temperature?: number;
    }
  ): AsyncGenerator<string, void, unknown> {
    // Convert messages to llama.cpp format
    const prompt = messages
      .map((m) => `${m.role}: ${m.content}`)
      .join('\n\n') + '\nassistant:';

    yield* this.generateCompletion(prompt, options);
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
   * Start llama.cpp server (requires backend binary)
   */
  async startServer(): Promise<void> {
    // This would call the native llama.cpp binary
    // For now, we assume the server is started externally
    console.log('Starting llama.cpp server...');
    console.log('Model:', this.config.modelPath);
    console.log('NGPU layers:', this.config.ngl);
    console.log('HADES:', this.config.hadesEnabled ? 'enabled' : 'disabled');

    // In production, this would spawn the llama.cpp server process
    // with HADES Bridge integration
    throw new Error('Server start not implemented - start llama.cpp externally');
  }

  /**
   * Stop llama.cpp server
   */
  async stopServer(): Promise<void> {
    try {
      await fetch(`${this.baseUrl}/shutdown`, { method: 'POST' });
    } catch {
      // Server might already be stopped
    }
    this.status = { status: 'disconnected' };
  }
}

export const llamaCppService = new LlamaCppService();
