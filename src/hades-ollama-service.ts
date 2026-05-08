/**
 * HADES-Ollama Integration Service
 * 
 * Wraps Ollama API with HADES intelligence layer
 */

import { useStore } from './store';
import { invoke } from './tauri_bridge';

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
    } catch { }
    return {};
  }

  private loadConfig() {
    const s = useStore.getState();
    const rawModel = s.agentModel || '';
    const resolvedModel = rawModel.includes('|') ? rawModel.split('|')[1] : rawModel;
    this.config = {
      baseUrl: s.ollamaUrl || 'http://localhost:11434',
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
    try {
      const targetModel = options?.model || this.config.model;
      const response = await fetch(`${this.config.baseUrl}/api/chat`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...this.getAuthHeader() },
        body: JSON.stringify({
          model: targetModel,
          messages,
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
              yield json.message.content;
            }
            if (json.done) return;
          } catch {
            // console.warn('Failed to parse chunk:', line);
          }
        }
      }
    } finally {
      this.activeRequests--;
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
