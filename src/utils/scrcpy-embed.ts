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
    console.log(`[ScrcpyEmbed] Starting stream for ${this.config.deviceId}...`);

    // Create video element
    this.videoElement = document.createElement('video');
    this.videoElement.style.width = '100%';
    this.videoElement.style.height = '100%';
    this.videoElement.style.objectFit = 'contain';
    this.videoElement.autoplay = true;
    this.videoElement.muted = true; // scrcpy doesn't stream audio by default

    // Create canvas for frame rendering
    this.canvasElement = document.createElement('canvas');
    this.canvasElement.width = this.config.width;
    this.canvasElement.height = this.config.height;

    // Clear container and add video
    containerElement.innerHTML = '';
    containerElement.appendChild(this.videoElement);

    // Start scrcpy server stream
    await this.startScrcpyStream();
  }

  /**
   * Start scrcpy HTTP stream
   */
  private async startScrcpyStream(): Promise<void> {
    try {
      // Use scrcpy-web or custom scrcpy server
      // For now, use Tauri command to start scrcpy
      const streamUrl = await invoke<string>('start_scrcpy_stream', {
        deviceId: this.config.deviceId,
        port: 8989,
      });

      console.log(`[ScrcpyEmbed] Stream URL: ${streamUrl}`);

      // Connect to stream
      if (this.videoElement) {
        this.videoElement.src = streamUrl;
      }
    } catch (error) {
      console.error('[ScrcpyEmbed] Failed to start stream:', error);
      
      // Fallback: Use canvas-based frame capture
      await this.startCanvasFallback();
    }
  }

  /**
   * Fallback: Canvas-based frame capture via ADB
   */
  private async startCanvasFallback(): Promise<void> {
    console.log('[ScrcpyEmbed] Using canvas fallback...');

    if (!this.canvasElement) return;

    const ctx = this.canvasElement.getContext('2d');
    if (!ctx) return;

    // Capture frames via adb screencap
    const captureFrame = async () => {
      try {
        const base64Image = await invoke<string>('capture_emulator_frame', {
          deviceId: this.config.deviceId,
        });

        const img = new Image();
        img.onload = () => {
          if (ctx && this.canvasElement) {
            ctx.drawImage(img, 0, 0, this.canvasElement.width, this.canvasElement.height);
          }
        };
        img.src = 'data:image/png;base64,' + base64Image;
      } catch (error) {
        console.error('[ScrcpyEmbed] Frame capture error:', error);
      }

      // Continue capturing at 10fps
      setTimeout(captureFrame, 100);
    };

    captureFrame();

    // Show canvas in video element container
    if (this.videoElement && this.canvasElement) {
      this.videoElement.parentNode?.replaceChild(this.canvasElement, this.videoElement);
    }
  }

  /**
   * Stop scrcpy stream
   */
  async stop(): Promise<void> {
    console.log('[ScrcpyEmbed] Stopping stream...');

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
      console.error('[ScrcpyEmbed] Failed to stop stream:', error);
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
      console.error('[ScrcpyEmbed] Tap error:', error);
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
      console.error('[ScrcpyEmbed] Swipe error:', error);
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
      console.error('[ScrcpyEmbed] Text error:', error);
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
      console.error('[ScrcpyEmbed] Key error:', error);
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
