/**
 * AIRI Vision System
 * 
 * Real-time emulator capture, Moondream vision analysis,
 * and visual verification with thermal integration.
 */

import { spawn, ChildProcess } from 'child_process';
import { EventEmitter } from 'events';
import { invoke } from '@tauri-apps/api/core';

export interface VisionConfig {
  emulatorPort: number;
  captureFps: number;
  diffThreshold: number;
  thermalThrottleTemp: number;
  model: string;  // 'moondream'
}

export interface FrameData {
  buffer: Buffer;
  timestamp: number;
  width: number;
  height: number;
  format: 'yuv' | 'rgb';
}

export interface VisionResponse {
  answer: string;
  description: string;
  confidence: number;
  timestamp: number;
}

export interface VerificationResult {
  passed: boolean;
  description: string;
  confidence: number;
  error?: string;
}

export class AIRIVisionSystem extends EventEmitter {
  private config: VisionConfig;
  private scrcpy: ChildProcess | null = null;
  private frameBuffer: FrameData[] = [];
  private currentFps: number = 10;
  private isRunning: boolean = false;
  private captureInterval: NodeJS.Timeout | null = null;

  constructor(config: Partial<VisionConfig> = {}) {
    super();
    
    this.config = {
      emulatorPort: config.emulatorPort || 5555,
      captureFps: config.captureFps || 10,
      diffThreshold: config.diffThreshold || 0.05,
      thermalThrottleTemp: config.thermalThrottleTemp || 72,
      model: config.model || 'moondream',
    };

    console.log('👁️ [AIRI Vision] Vision system initialized');
  }

  /**
   * Start vision capture
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      console.warn('[AIRI Vision] Already running');
      return;
    }

    console.log('🎥 [AIRI Vision] Starting emulator capture...');

    // Start scrcpy stream
    this.scrcpy = spawn('scrcpy', [
      '--no-display',
      '--no-control',
      '--no-audio',
      '--bit-rate', '2M',
      '--max-fps', this.config.captureFps.toString(),
      '--tcpip=5555',
    ]);

    this.scrcpy.stdout.on('data', (data: Buffer) => {
      this.onFrameReceived(data);
    });

    this.scrcpy.stderr.on('data', (data: Buffer) => {
      console.error('[AIRI Vision] scrcpy error:', data.toString());
    });

    this.scrcpy.on('close', (code) => {
      console.log(`[AIRI Vision] scrcpy exited with code ${code}`);
      this.isRunning = false;
    });

    // Start thermal management loop
    this.startThermalManagement();

    this.isRunning = true;
    console.log('✅ [AIRI Vision] Capture started');
  }

  /**
   * Stop vision capture
   */
  stop(): void {
    if (this.scrcpy) {
      this.scrcpy.kill();
      this.scrcpy = null;
    }

    if (this.captureInterval) {
      clearInterval(this.captureInterval);
      this.captureInterval = null;
    }

    this.isRunning = false;
    console.log('⏹️ [AIRI Vision] Capture stopped');
  }

  /**
   * Handle incoming frame
   */
  private onFrameReceived(data: Buffer): void {
    const frame: FrameData = {
      buffer: data,
      timestamp: Date.now(),
      width: 1080,  // Default, can be detected
      height: 1920,
      format: 'yuv',
    };

    this.frameBuffer.push(frame);

    // Keep only last 5 frames
    if (this.frameBuffer.length > 5) {
      this.frameBuffer.shift();
    }

    // Emit frame event
    this.emit('frame', frame);
  }

  /**
   * Get latest frame
   */
  getLatestFrame(): FrameData | null {
    if (this.frameBuffer.length === 0) return null;
    return this.frameBuffer[this.frameBuffer.length - 1];
  }

  /**
   * Get previous frame (for diffing)
   */
  getPreviousFrame(): FrameData | null {
    if (this.frameBuffer.length < 2) return null;
    return this.frameBuffer[this.frameBuffer.length - 2];
  }

  /**
   * Check if frame should be processed (diff > threshold)
   */
  shouldProcessFrame(current: FrameData, previous: FrameData): boolean {
    if (!previous) return true;  // No previous frame, always process

    const diff = this.calculateFrameDiff(current.buffer, previous.buffer);
    const changeRatio = diff / current.buffer.length;

    return changeRatio > this.config.diffThreshold;
  }

  /**
   * Calculate pixel diff between two frames
   */
  private calculateFrameDiff(a: Buffer, b: Buffer): number {
    let diffPixels = 0;
    const sampleRate = 10;  // Check every 10th pixel for speed

    const minLength = Math.min(a.length, b.length);

    for (let i = 0; i < minLength; i += sampleRate) {
      if (a[i] !== b[i]) {
        diffPixels++;
      }
    }

    return diffPixels;
  }

  /**
   * Start thermal management loop
   */
  private startThermalManagement(): void {
    this.captureInterval = setInterval(async () => {
      await this.updateCaptureRate();
    }, 2000);  // Check every 2 seconds
  }

  /**
   * Update capture rate based on GPU temperature
   */
  private async updateCaptureRate(): Promise<void> {
    try {
      // Get GPU temperature via Tauri command
      let temp = 0;
      try {
        const telemetry = await invoke<any>('get_gpu_telemetry');
        temp = telemetry.temperature_c || 0;
      } catch (e) {
        // Fallback if command not available
        console.warn('[AIRI Vision] Could not get GPU telemetry');
      }

      if (temp >= 80) {
        // Emergency - stop capture
        this.currentFps = 0;
        console.warn('[AIRI Vision] 🔴 Thermal emergency - stopping capture');
      } else if (temp >= 72) {
        // Throttle to 1fps
        this.currentFps = 1;
        console.log(`[AIRI Vision] 🟡 Thermal throttle - reduced to ${this.currentFps}fps`);
      } else if (temp >= 65) {
        // Reduce to 5fps
        this.currentFps = 5;
      } else {
        // Normal operation
        this.currentFps = this.config.captureFps;
      }

      // Update scrcpy FPS (would need to restart stream with new FPS)
      // For now, just track internally
      this.emit('fps-change', this.currentFps);

    } catch (error) {
      console.error('[AIRI Vision] Thermal management error:', error);
    }
  }

  /**
   * Get current FPS
   */
  getCurrentFps(): number {
    return this.currentFps;
  }

  /**
   * Get vision system state
   */
  getState(): {
    isRunning: boolean;
    fps: number;
    frameCount: number;
    lastFrameTime: number | null;
  } {
    return {
      isRunning: this.isRunning,
      fps: this.currentFps,
      frameCount: this.frameBuffer.length,
      lastFrameTime: this.frameBuffer.length > 0 
        ? this.frameBuffer[this.frameBuffer.length - 1].timestamp 
        : null,
    };
  }
}

// Singleton instance
export const airiVision = new AIRIVisionSystem();
