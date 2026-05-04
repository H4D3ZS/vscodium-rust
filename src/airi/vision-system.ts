/**
 * AIRI Vision System
 * Real-time screen capture and AI visual analysis
 */

import { invoke } from '../tauri_bridge';

export interface VisionConfig {
  captureFps: number;
  diffThreshold: number;
}

export interface FrameData {
  timestamp: number;
  buffer: string; // Base64 PNG
  metadata: { width: number; height: number };
}

export class AIRIVisionSystem {
  private isRunning: boolean = false;
  private captureTimer: any = null;
  private config: VisionConfig;

  constructor(config: Partial<VisionConfig> = {}) {
    this.config = {
      captureFps: config.captureFps || 0.1, // Once every 10 seconds
      diffThreshold: config.diffThreshold || 0.05,
    };
  }

  async start(): Promise<void> {
    if (this.isRunning) return;
    this.isRunning = true;
    this.startCaptureLoop();
  }

  private startCaptureLoop(): void {
    const intervalMs = 1000 / this.config.captureFps;
    this.captureTimer = setInterval(async () => {
      try { await this.captureAndProcessFrame(); } catch { }
    }, intervalMs);
  }

  private async captureAndProcessFrame(): Promise<void> {
    try {
      const pngBytes = await invoke<Uint8Array>('airi_vision_capture_screen');
      // Minimal processing
      this.emitStatus('frame_captured');
    } catch { }
  }

  private emitStatus(status: string): void {
    // placeholder for event system integration
  }

  stop(): void {
    if (this.captureTimer) {
      clearInterval(this.captureTimer);
      this.captureTimer = null;
    }
    this.isRunning = false;
  }

  on(event: string, cb: any): void { }
  getState(): any { return { isRunning: this.isRunning }; }
  getLatestFrame(): any { return null; }
}

export const airiVision = new AIRIVisionSystem();
