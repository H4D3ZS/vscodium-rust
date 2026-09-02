/**
 * HADES-Ollama Integration Service
 * 
 * Wraps Ollama API with HADES intelligence layer:
 * - .aim VFS context injection
 * - Thermal governor monitoring
 * - JIT decompression triggers
 * - Semantic gist enhancement
 */

import { useStore, normalizeOllamaUrl } from './store';
import { invoke } from './tauri_bridge';

export interface HadesOllamaConfig {
  baseUrl: string;
  model: string;
  aimVfsEnabled: boolean;
  thermalGovernorEnabled: boolean;
  jitDecompressionEnabled: boolean;
  jitThreshold: number;
}

export interface OllamaResponse {
  model: string;
  response?: string;  // For /api/generate
  message?: {         // For /api/chat
    content: string;
  };
  done: boolean;
  total_duration: number;
  load_duration: number;
  prompt_eval_count: number;
  eval_count: number;
  eval_duration: number;
}

class HadesOllamaService {
  private config: HadesOllamaConfig = {
    baseUrl: 'http://localhost:13305',
    model: '',
    aimVfsEnabled: true,
    thermalGovernorEnabled: true,
    jitDecompressionEnabled: true,
    jitThreshold: 0.85,
  };

  constructor() {
    this.loadConfig();
  }

  private loadConfig() {
    const s = useStore.getState();
    this.config = {
      baseUrl: s.ollamaUrl || 'http://localhost:13305',
      model: s.agentModel?.split('|')[1] || s.agentModel || '',
      aimVfsEnabled: (s as { aimVfsEnabled?: boolean }).aimVfsEnabled ?? true,
      thermalGovernorEnabled: (s as { thermalGovernorEnabled?: boolean }).thermalGovernorEnabled ?? true,
      jitDecompressionEnabled: (s as { jitDecompressionEnabled?: boolean }).jitDecompressionEnabled ?? true,
      jitThreshold: (s as { jitThreshold?: number }).jitThreshold ?? 0.85,
    };
  }

  private async postOllama(path: '/api/generate' | '/api/chat', body: Record<string, unknown>): Promise<OllamaResponse> {
    if (!this.config.model.trim()) {
      throw new Error('No Ollama model selected. Choose a local model in Settings before starting local inference.');
    }
    const raw = (this.config.baseUrl || '').trim() || 'http://localhost:13305';
    let base: string;
    try {
      base = normalizeOllamaUrl(raw);
    } catch {
      base = 'http://localhost:13305';
    }
    // Lemonade sends `Access-Control-Allow-Origin: *`, so the webview calls it
    // directly — the old Rust CORS-bypass bridge is gone.
    const headers: Record<string, string> = { 'Content-Type': 'application/json' };
    try {
      const keys = await invoke<Record<string, string>>('get_api_keys');
      const tok = keys?.lemonade?.trim();
      if (tok) headers.Authorization = `Bearer ${tok}`;
    } catch {
      /* no Tauri / no keys */
    }
    // Same-origin dev proxy (see vite.config.ts) when running in a browser surface.
    // Falls through to the configured base URL only for fully-local Ollama (matches origin).
    let url = `${base}${path}`;
    if (typeof window !== 'undefined') {
      try {
        const u = new URL(base);
        if (u.origin !== window.location.origin) {
          url = `${base}${path}`;
        }
      } catch {
        /* malformed base → keep as-is */
      }
    }
    const response = await fetch(url, {
      method: 'POST',
      headers,
      body: JSON.stringify(body),
    });
    if (!response.ok) {
      throw new Error(`Ollama error: ${response.statusText}`);
    }
    return (await response.json()) as OllamaResponse;
  }

  /**
   * Generate response with HADES intelligence layer
   */
  async generate(prompt: string, options?: {
    stream?: boolean;
    context?: string[];
  }): Promise<OllamaResponse> {
    this.loadConfig();

    // Step 1: Check thermal state (RX 580 protection)
    if (this.config.thermalGovernorEnabled) {
      const throttle = await this.checkThermalThrottle();
      if (throttle < 0.3) {
        throw new Error(`Thermal emergency: GPU throttling at ${(throttle * 100).toFixed(0)}%`);
      }
      
      // Apply back-pressure delay if throttling
      if (throttle < 1.0) {
        const delayMs = (1 - throttle) * 50;
        await this.sleep(delayMs);
      }
    }

    // Step 2: Get .aim VFS context (6KB semantic gist)
    let enhancedPrompt = prompt;
    if (this.config.aimVfsEnabled) {
      const aimContext = await this.getAimContext(prompt);
      if (aimContext) {
        enhancedPrompt = `[Context: ${aimContext}]\n\n${prompt}`;
      }
    }

    // Step 3: Check for JIT decompression trigger
    if (this.config.jitDecompressionEnabled) {
      const attentionScore = await this.estimateAttentionScore(prompt);
      if (attentionScore >= this.config.jitThreshold) {
        // Trigger JIT inflation of relevant code blocks
        await this.triggerJitInflation(prompt);
      }
    }

    const data = await this.postOllama('/api/generate', {
      model: this.config.model,
      prompt: enhancedPrompt,
      stream: options?.stream ?? false,
      context: options?.context,
    });
    
    // Normalize chat response to have 'response' field
    if (data.message && data.message.content) {
      data.response = data.message.content;
    }

    // Step 5: Update .aim VFS with new context
    if (this.config.aimVfsEnabled) {
      await this.updateAimContext(prompt, data.response);
    }

    return data;
  }

  /**
   * Chat completion with HADES intelligence
   */
  async chat(messages: Array<{ role: string; content: string }>, options?: {
    stream?: boolean;
  }): Promise<OllamaResponse> {
    this.loadConfig();

    // Thermal check
    if (this.config.thermalGovernorEnabled) {
      const throttle = await this.checkThermalThrottle();
      if (throttle < 0.3) {
        throw new Error('Thermal emergency');
      }
      if (throttle < 1.0) {
        await this.sleep((1 - throttle) * 50);
      }
    }

    // Get .aim context for latest message
    const lastMessage = messages[messages.length - 1];
    let enhancedMessages = messages;
    
    if (this.config.aimVfsEnabled) {
      const aimContext = await this.getAimContext(lastMessage.content);
      if (aimContext) {
        enhancedMessages = [
          ...messages.slice(0, -1),
          {
            role: lastMessage.role,
            content: `[Context: ${aimContext}]\n\n${lastMessage.content}`,
          },
        ];
      }
    }

    const data = await this.postOllama('/api/chat', {
      model: this.config.model,
      messages: enhancedMessages,
      stream: options?.stream ?? false,
    });
    
    // Normalize chat response to have 'response' field
    if (data.message && data.message.content) {
      data.response = data.message.content;
    }
    
    return data;
  }

  /**
   * Check thermal throttle ratio (0.0-1.0)
   */
  private async checkThermalThrottle(): Promise<number> {
    // In production, this would call the HADES thermal governor API
    // For now, return 1.0 (no throttling)
    return 1.0;
  }

  /**
   * Get .aim VFS semantic context
   */
  private async getAimContext(prompt: string): Promise<string | null> {
    // Extract keywords from prompt
    const keywords = this.extractKeywords(prompt);
    
    // Query .aim VFS for relevant context
    // This would call the actual .aim VFS API
    console.log('[HADES] Querying .aim VFS for:', keywords);
    
    // Placeholder - would return 6KB gist context
    return null;
  }

  /**
   * Estimate attention score for JIT trigger
   */
  private async estimateAttentionScore(prompt: string): Promise<number> {
    // Simple heuristic: check for code-related keywords
    const codeKeywords = ['function', 'class', 'interface', 'code', 'file', 'path', 'buffer'];
    const hasCode = codeKeywords.some(k => prompt.toLowerCase().includes(k));
    
    // Return score 0.0-1.0
    return hasCode ? 0.9 : 0.5;
  }

  /**
   * Trigger JIT inflation of code blocks
   */
  private async triggerJitInflation(prompt: string): Promise<void> {
    console.log('[HADES] JIT inflation triggered for:', prompt);
    // Would inflate relevant code blocks from SSD to VRAM
  }

  /**
   * Update .aim VFS with new context
   */
  private async updateAimContext(prompt: string, response: string): Promise<void> {
    console.log('[HADES] Updating .aim VFS with new context');
    // Would update the semantic gist with new information
  }

  /**
   * Extract keywords from prompt
   */
  private extractKeywords(prompt: string): string[] {
    // Simple keyword extraction
    return prompt
      .toLowerCase()
      .match(/\b\w+\b/g)
      ?.filter(w => w.length > 3) || [];
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}

export const hadesOllama = new HadesOllamaService();
