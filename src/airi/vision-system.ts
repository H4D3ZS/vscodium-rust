/**
 * AIRI Vision System
 * Real-time desktop capture with Qwen2.5-VL analysis for error detection
 */

import { invoke } from '@tauri-apps/api/core';
import { VisionAnalyzer } from './vision-analysis';

export interface VisionConfig {
  captureFps: number;
  diffThreshold: number;
  thermalThrottleTemp: number;
  enableStreaming: boolean;
  maxBufferSize: number;
  ollamaHost: string;
}

export interface FrameData {
  buffer: string; // base64 PNG
  timestamp: number;
  width: number;
  height: number;
  format: 'png' | 'jpeg';
  analysis?: any;
}

export interface VisionAnalysis {
  code?: { language: string | null; function: string | null; snippet: string | null; errors: string[] };
  ui?: { panels: string[]; activeFile: string | null; lineNumbers: { start: number; end: number } | null };
  attention?: { focus: string; reason: string };
}

export class AIRIVisionSystem {
  private config: VisionConfig;
  private frameBuffer: FrameData[] = [];
  private isRunning: boolean = false;
  private captureTimer: NodeJS.Timeout | null = null;
  private analysisQueue: FrameData[] = [];
  private isAnalyzing: boolean = false;
  private lastAnalysisTime: number = 0;
  private readonly ANALYSIS_INTERVAL_MS = 150;
  private unsupportedPlatform = false;
  private listeners = new Map<string, Array<(data: any) => void>>();
  private regionsOfInterest: Array<{ name: string; x: number; y: number; w: number; h: number; priority: number }> = [
    { name: 'editor', x: 0, y: 0, w: 1920, h: 1080, priority: 10 },
  ];

  on(event: string, callback: (data: any) => void): void {
    if (!this.listeners.has(event)) this.listeners.set(event, []);
    this.listeners.get(event)!.push(callback);
  }

  private emit(event: string, data: any): void {
    this.listeners.get(event)?.forEach(fn => fn(data));
  }

   constructor(config: Partial<VisionConfig> = {}) {
     this.config = {
      captureFps: config.captureFps || 10,
      diffThreshold: config.diffThreshold || 0.02,
      thermalThrottleTemp: config.thermalThrottleTemp || 72,
      enableStreaming: config.enableStreaming ?? true,
      maxBufferSize: config.maxBufferSize || 30,
      ollamaHost: config.ollamaHost || 'http://localhost:13305',
    };
  }

  async start(): Promise<void> {
    if (this.isRunning) return;
    if (this.unsupportedPlatform) return;
    // Honour the user's toggle; the store persists this under airi.vision.enabled.
    try {
      if (typeof localStorage !== 'undefined' && localStorage.getItem('airi.vision.enabled') !== '1') {
        console.log('[AIRI Vision] Skipping start — disabled in settings (airi.vision.enabled!=1).');
        return;
      }
      const model = typeof localStorage !== 'undefined'? localStorage.getItem('airi.vision.model'): '';
      if (!model || model.trim() === '') {
        console.log('[AIRI Vision] Skipping start — no vision model configured in settings.');
        return;
      }
    } catch { /* no localStorage */ }
    try {
      // Test capture capability by attempting a single capture
      await invoke<number[]>('airi_vision_capture_screen');
      this.isRunning = true;
      this.startCaptureLoop();
      console.log(`[AIRI Vision] Started (${this.config.captureFps} FPS)`);
    } catch (error: any) {
      const msg = String(error?.message || error || '');
      if (msg.toLowerCase().includes('screen capture not supported')) {
        this.unsupportedPlatform = true;
        console.warn('[AIRI Vision] Screen capture unsupported on this platform; vision disabled.');
        return;
      }
 console.error('[AIRI Vision] Failed:', msg);
      throw error;
    }
  }

  private startCaptureLoop(): void {
    const intervalMs = 1000 / this.config.captureFps;
    this.captureTimer = setInterval(async () => {
      try { await this.captureAndProcessFrame(); } catch (error) { console.error('[AIRI Vision] Capture error:', error); }
    }, intervalMs);
  }

  private async captureAndProcessFrame(): Promise<void> {
    try {
      const pngBytes = await invoke<number[]>('airi_vision_capture_screen');
      const buffer = Buffer.from(pngBytes);
      const base64 = buffer.toString('base64');
      const frame: FrameData = { buffer: base64, timestamp: Date.now(), width: 1920, height: 1080, format: 'png' };

      if (!this.shouldProcessFrame(frame)) {
        this.frameBuffer.push(frame);
        if (this.frameBuffer.length > this.config.maxBufferSize) this.frameBuffer.shift();
        return;
      }

      this.frameBuffer.push(frame);
      if (this.frameBuffer.length > this.config.maxBufferSize) this.frameBuffer.shift();
      this.analysisQueue.push(frame);
      this.emit('frame', frame);
      
      if (!this.isAnalyzing) this.processAnalysisQueue();
    } catch (error) {
      console.error('[AIRI Vision] Capture failed:', error);
    }
  }

  private shouldProcessFrame(current: FrameData): boolean {
    const previous = this.frameBuffer[this.frameBuffer.length - 1];
    if (!previous) return true;
    const diff = this.hammingDistance(current.buffer, previous.buffer);
    const maxLen = Math.max(current.buffer.length, previous.buffer.length);
    const changeRatio = diff / (maxLen / 100);
    return changeRatio > this.config.diffThreshold * 100;
  }

  private hammingDistance(a: string, b: string): number {
    let diff = 0;
    const step = 100;
    const len = Math.min(a.length, b.length);
    for (let i = 0; i < len; i += step) if (a[i] !== b[i]) diff++;
    return diff;
  }

  private async processAnalysisQueue(): Promise<void> {
    this.isAnalyzing = true;
    while (this.analysisQueue.length > 0) {
      const frame = this.analysisQueue.shift()!;
      this.lastAnalysisTime = Date.now();
      try {
        const { visionAnalyzer } = await import('./vision-analysis');
        if (typeof visionAnalyzer.isAvailable === 'function' && !visionAnalyzer.isAvailable()) {
          this.analysisQueue.length = 0;
          break;
        }
        const [codeCtx, errors] = await Promise.all([
          this.analyzeForCode(frame, visionAnalyzer),
          this.analyzeForErrors(frame, visionAnalyzer),
        ]);

        const analysis: VisionAnalysis = {
          code: {
            language: this.parseAnswer(codeCtx.answer, 'LANGUAGE'),
            function: this.parseAnswer(codeCtx.answer, 'FUNCTION'),
            snippet: this.parseAnswer(codeCtx.answer, 'CODE_SNIPPET'),
            errors: this.parseErrors(errors.answer),
          },
          ui: {
            panels: ['editor'],
            activeFile: null,
            lineNumbers: null,
          },
          attention: { focus: null, reason: null },
        };

        frame.analysis = analysis;
        this.emit('analysis', { frame, analysis });

        if (analysis.code?.errors?.length > 0) {
          this.emit('error_detected', { analysis });
        }
      } catch (error) {
        console.error('[AIRI Vision] Analysis failed:', error);
      }
    }
    this.isAnalyzing = false;
  }

  private async analyzeForCode(frame: FrameData, analyzer: any): Promise<any> {
    return await analyzer.analyzeFrame(frame, `Code visible? Return:\nLANGUAGE: [lang]\nFUNCTION: [name]\nCODE_SNIPPET: [5 lines]\nERRORS: [list]\nCARET: [cursor pos]`);
  }

  private async analyzeForErrors(frame: FrameData, analyzer: any): Promise<any> {
    return await analyzer.analyzeFrame(frame, `Errors/stack traces visible? "YES" + describe or "NO".`);
  }

  private parseAnswer(text: string, key: string): string | null {
    const m = text.match(new RegExp(`${key}:\\s*([^\\n]+)`, 'i'));
    return m? m[1].trim(): null;
  }

   private parseErrors(text: string): string[] {
     if (!text) return [];
     const lower = text.toLowerCase();
     // If explicitly says no error, return empty
     if (lower.includes('no') && lower.length < 20) return [];
     
     // Strip leading YES/no confirmation to isolate error description
     let cleaned = text.trim();
     if (lower.startsWith('yes')) {
       // Remove leading YES and any punctuation/whitespace after it
       cleaned = cleaned.replace(/^yes\s*[:\-]?\s*/i, '');
     }
     
     // If after stripping it's empty, return empty
     if (!cleaned.trim()) return [];
     
     // Return single error message (first non-empty line)
     const lines = cleaned.split('\n').filter(l => l.trim().length > 0);
     return [lines[0].trim()].slice(0, 3);
   }

  getLatestFrame(): FrameData | null {
    if (this.frameBuffer.length === 0) return null;
    return this.frameBuffer[this.frameBuffer.length - 1];
  }

  stop(): void {
    if (this.captureTimer) {
      clearInterval(this.captureTimer);
      this.captureTimer = null;
    }
    this.analysisQueue.length = 0;
    this.isRunning = false;
    this.isAnalyzing = false;
    console.log('[AIRI Vision] Stopped.');
  }

  getState(): { isRunning: boolean; fps: number; frameCount: number; lastFrameTime: number | null } {
    return {
      isRunning: this.isRunning,
      fps: this.config.captureFps,
      frameCount: this.frameBuffer.length,
      lastFrameTime: this.frameBuffer.length > 0? this.frameBuffer[this.frameBuffer.length - 1].timestamp: null,
    };
  }
}

// Singleton instance
export const airiVision = new AIRIVisionSystem();
