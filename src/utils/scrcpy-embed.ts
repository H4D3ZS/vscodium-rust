/**
 * scrcpy Embed - Embed Android emulator screen directly in IDE
 * Uses scrcpy to capture and stream emulator display
 */

import { invoke } from '@tauri-apps/api/core';

export interface ScrcpyConfig {
  deviceId: string;
  width: number;
  height: number;
  bitrate: number;
  maxFps: number;
}

export class ScrcpyEmbed {
  private config: ScrcpyConfig;
  private videoElement: HTMLVideoElement | null = null;
  private canvasElement: HTMLCanvasElement | null = null;
  private mediaSource: MediaSource | null = null;
  private sourceBuffer: SourceBuffer | null = null;
  private eventSource: EventSource | null = null;

  constructor(config: Partial<ScrcpyConfig> = {}) {
    this.config = {
      deviceId: config.deviceId || 'emulator-5554',
      width: config.width || 360,
      height: config.height || 640,
      bitrate: config.bitrate || 2000000, // 2Mbps
      maxFps: config.maxFps || 30,
    };
  }

  /**
   * Start scrcpy stream and embed in container
   */
  async start(containerElement: HTMLElement): Promise<void> {

    // Clear container
    containerElement.innerHTML = '';

    // Create canvas for frame rendering (more reliable than video stream)
    this.canvasElement = document.createElement('canvas');
    this.canvasElement.width = this.config.width;
    this.canvasElement.height = this.config.height;
    this.canvasElement.style.width = '100%';
    this.canvasElement.style.height = '100%';
    this.canvasElement.style.objectFit = 'contain';
    
    containerElement.appendChild(this.canvasElement);

    // Start canvas-based frame capture (works without scrcpy server)
    await this.startCanvasFallback();
  }

  /**
   * Fallback: Canvas-based frame capture via ADB screencap
   */
  private async startCanvasFallback(): Promise<void> {

    if (!this.canvasElement) return;

    const ctx = this.canvasElement.getContext('2d');
    if (!ctx) return;

    // Show loading message
    ctx.fillStyle = '#000';
    ctx.fillRect(0, 0, this.canvasElement.width, this.canvasElement.height);
    ctx.fillStyle = '#888';
    ctx.font = '14px Arial';
    ctx.textAlign = 'center';
    ctx.fillText('Connecting to emulator...', this.canvasElement.width / 2, this.canvasElement.height / 2 - 20);
    ctx.font = '12px Arial';
    ctx.fillText('Make sure emulator is running', this.canvasElement.width / 2, this.canvasElement.height / 2 + 10);

    let frameCount = 0;
    let lastSuccessfulFrame = 0;

    // Capture frames via adb screencap (through Tauri command)
    const captureFrame = async () => {
      try {
        const { invoke } = await import('@tauri-apps/api/core');
        const base64Image = await invoke<string>('capture_emulator_frame', {
          deviceId: this.config.deviceId,
        });

        if (base64Image && base64Image.length > 100) {
          const img = new Image();
          img.onload = () => {
            if (ctx && this.canvasElement) {
              ctx.drawImage(img, 0, 0, this.canvasElement.width, this.canvasElement.height);
              frameCount++;
              lastSuccessfulFrame = Date.now();
            }
          };
          img.onerror = () => {
          };
          img.src = 'data:image/png;base64,' + base64Image;
        } else {
          // No frame returned - emulator might not be ready
          if (frameCount === 0) {
            // Still showing loading message
            ctx.fillStyle = '#000';
            ctx.fillRect(0, 0, this.canvasElement.width, this.canvasElement.height);
            ctx.fillStyle = '#888';
            ctx.font = '12px Arial';
            ctx.textAlign = 'center';
            ctx.fillText('Waiting for emulator to boot...', this.canvasElement.width / 2, this.canvasElement.height / 2);
          }
        }
      } catch (error: any) {
        // Silently fail - emulator might not be ready yet
        if (frameCount === 0 && Date.now() - lastSuccessfulFrame > 5000) {
          // Show error after 5 seconds of no frames
          ctx.fillStyle = '#000';
          ctx.fillRect(0, 0, this.canvasElement.width, this.canvasElement.height);
          ctx.fillStyle = '#f44';
          ctx.font = '14px Arial';
          ctx.textAlign = 'center';
          ctx.fillText('No screen data from emulator', this.canvasElement.width / 2, this.canvasElement.height / 2 - 30);
          ctx.fillStyle = '#f88';
          ctx.font = '12px Arial';
          ctx.fillText('Possible issues:', this.canvasElement.width / 2, this.canvasElement.height / 2 - 10);
          ctx.fillStyle = '#888';
          ctx.font = '11px Arial';
          ctx.fillText('1. Emulator not running - click "Launch & Embed"', this.canvasElement.width / 2, this.canvasElement.height / 2 + 10);
          ctx.fillText('2. ADB not found - install Android SDK', this.canvasElement.width / 2, this.canvasElement.height / 2 + 30);
          ctx.fillText('3. Emulator still booting - wait 30-60 seconds', this.canvasElement.width / 2, this.canvasElement.height / 2 + 50);
        }
      }

      // Continue capturing at 10fps
      setTimeout(captureFrame, 100);
    };

    captureFrame();
  }

  /**
   * Stop scrcpy stream
   */
  async stop(): Promise<void> {

    if (this.videoElement) {
      this.videoElement.src = '';
      this.videoElement = null;
    }

    if (this.canvasElement) {
      this.canvasElement = null;
    }

    if (this.eventSource) {
      this.eventSource.close();
      this.eventSource = null;
    }

    // Stop scrcpy server
    try {
      await invoke('stop_scrcpy_stream');
    } catch (error) {
    }
  }

  /**
   * Send tap event to emulator
   */
  async sendTap(x: number, y: number): Promise<void> {
    try {
      await invoke('send_emulator_tap', {
        deviceId: this.config.deviceId,
        x: Math.round(x),
        y: Math.round(y),
      });
    } catch (error) {
    }
  }

  /**
   * Send swipe event to emulator
   */
  async sendSwipe(x1: number, y1: number, x2: number, y2: number): Promise<void> {
    try {
      await invoke('send_emulator_swipe', {
        deviceId: this.config.deviceId,
        x1: Math.round(x1),
        y1: Math.round(y1),
        x2: Math.round(x2),
        y2: Math.round(y2),
        duration: 300,
      });
    } catch (error) {
    }
  }

  /**
   * Send text input to emulator
   */
  async sendText(text: string): Promise<void> {
    try {
      await invoke('send_emulator_text', {
        deviceId: this.config.deviceId,
        text,
      });
    } catch (error) {
    }
  }

  /**
   * Send key event to emulator
   */
  async sendKey(keycode: number): Promise<void> {
    try {
      await invoke('send_emulator_key', {
        deviceId: this.config.deviceId,
        keycode,
      });
    } catch (error) {
    }
  }
}

// Singleton instance manager
const activeScrcpyInstances = new Map<string, ScrcpyEmbed>();

/**
 * Get or create scrcpy instance for device
 */
export function getScrcpyForDevice(deviceId: string): ScrcpyEmbed {
  if (!activeScrcpyInstances.has(deviceId)) {
    activeScrcpyInstances.set(deviceId, new ScrcpyEmbed({ deviceId }));
  }
  return activeScrcpyInstances.get(deviceId)!;
}

/**
 * Stop all scrcpy instances
 */
export async function stopAllScrcpyInstances(): Promise<void> {
  for (const [deviceId, instance] of activeScrcpyInstances.entries()) {
    await instance.stop();
    activeScrcpyInstances.delete(deviceId);
  }
}
