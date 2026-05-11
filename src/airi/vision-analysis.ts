/**
 * AIRI Vision Analysis
 * Vision-LLM integration for error detection from IDE screenshots.
 *
 * The model + host are now configurable through localStorage / window globals
 * so we don't hardcode `qwen2.5-vl:72b` and spam 404s when the user hasn't
 * pulled it. The analyzer also disables itself after enough consecutive
 * failures instead of looping forever.
 */

import { Ollama } from 'ollama';

export interface ErrorDetectionResult {
  hasError: boolean;
  errorMessage: string;
  confidence: number;
}

const DEFAULT_HOST = 'http://localhost:11434';
// Default to the bare-tag form your local Ollama actually uses ("qwen2.5vl"
// not "qwen2.5-vl"). Users override via localStorage.airi.vision.model.
const DEFAULT_MODEL = 'qwen2.5vl:72b';
const FAILURE_BUDGET = 3;

function resolveHost(): string {
  try {
    const w: any = typeof window !== 'undefined' ? window : null;
    return (
      w?.localStorage?.getItem?.('airi.vision.host') ||
      w?.AIRI_VISION_HOST ||
      DEFAULT_HOST
    );
  } catch {
    return DEFAULT_HOST;
  }
}

function resolveModel(): string {
  try {
    const w: any = typeof window !== 'undefined' ? window : null;
    return (
      w?.localStorage?.getItem?.('airi.vision.model') ||
      w?.AIRI_VISION_MODEL ||
      DEFAULT_MODEL
    );
  } catch {
    return DEFAULT_MODEL;
  }
}

export class VisionAnalyzer {
  private ollama: any;
  private model: string;
  private host: string;
  private failureCount = 0;
  private disabled = false;

  constructor(host?: string, model?: string) {
    this.host = host || resolveHost();
    this.model = model || resolveModel();
    this.ollama = new Ollama({ host: this.host });
  }

  isAvailable(): boolean {
    return !this.disabled;
  }

  reconfigure(opts: { model?: string; host?: string }): void {
    if (opts.host && opts.host !== this.host) {
      this.host = opts.host;
      this.ollama = new Ollama({ host: this.host });
    }
    if (opts.model) {
      this.model = opts.model;
    }
    this.failureCount = 0;
    this.disabled = false;
  }

  getModel(): string {
    return this.model;
  }

  getHost(): string {
    return this.host;
  }

  async analyzeFrame(frame: any, question: string): Promise<any> {
    if (this.disabled) {
      return { answer: '', confidence: 0 };
    }
    try {
      const response = await this.ollama.generate({
        model: this.model,
        prompt: question,
        images: [frame.buffer],
        stream: false,
        options: { temperature: 0.3, num_predict: 512 },
      });
      this.failureCount = 0;
      return { answer: response.response, confidence: 0.7 };
    } catch (err: any) {
      const msg = String(err?.message || err || '');
      const modelMissing = msg.includes('not found') || msg.includes('404');
      this.failureCount += 1;
      if (modelMissing || this.failureCount >= FAILURE_BUDGET) {
        this.disabled = true;
        console.warn(
          `[AIRI Vision] Disabling vision analyzer — model "${this.model}" not available at ${this.host}.`,
          'Set localStorage.airi.vision.model to an installed Ollama vision tag (e.g. "llava" or "qwen2-vl") to re-enable.'
        );
      }
      return { answer: '', confidence: 0 };
    }
  }

  async detectErrors(frame: any): Promise<ErrorDetectionResult> {
    const question = `Look at this IDE screenshot. Identify any compiler errors, linter warnings, or stack traces. Return JSON: {"hasError":boolean,"errorMessage":"text"}`;
    const resp = await this.analyzeFrame(frame, question);
    
    try {
      const jsonStart = resp.answer.indexOf('{');
      const jsonEnd = resp.answer.lastIndexOf('}') + 1;
      const jsonStr = resp.answer.substring(jsonStart, jsonEnd);
      const parsed = JSON.parse(jsonStr);
      return { 
        hasError: parsed.hasError || false, 
        errorMessage: parsed.errorMessage || '', 
        confidence: resp.confidence 
      };
    } catch {
      const lower = resp.answer.toLowerCase();
      const hasError = /error|syntax|compile|failed/.test(lower);
      return { 
         hasError, 
         errorMessage: hasError ? resp.answer.substring(0,200) : '', 
         confidence: resp.confidence 
      };
    }
  }
}

export const visionAnalyzer = new VisionAnalyzer();
