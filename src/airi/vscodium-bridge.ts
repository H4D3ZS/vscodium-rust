/**
 * VSCodium-Rust AIRI Integration Bridge
 *
 * Emits AIRI cognitive events to the frontend (React/Tauri window)
 * for real-time HUD overlay, status panels, and editor decorations.
 *
 * Events (subscribe on frontend via `window.addEventListener('airi:*')):
 *   - airi:status         → periodic full status
 *   - airi:vision_frame   → new screenshot + analysis
 *   - airi:phase_wrap    → reflection cycle complete
 *   - airi:edit_proposed → code change staged
 *   - airi:edit_committed → edit applied
 *   - airi:error_detected → vision spotted an error
 *   - airi:thought       → new consciousness thought
 *
 * VS Code extension / webview UI can listen and render overlays.
 */

import { airi } from './core';
import { EventEmitter } from 'events';

export class VSCodiumBridge extends EventEmitter {
  private static instance: VSCodiumBridge;
  private listeners: Map<string, Array<(data: any) => void>> = new Map();

  private constructor() {
    super();
    this.setupAIRISubscriptions();
  }

  static getInstance(): VSCodiumBridge {
    if (!VSCodiumBridge.instance) {
      VSCodiumBridge.instance = new VSCodiumBridge();
    }
    return VSCodiumBridge.instance;
  }

  /**
   * Wire AIRI events → forward to frontend
   */
  private setupAIRISubscriptions(): void {
    // Vision frames
    airi.vision.on('frame', (data: any) => {
      this.emit('airi:vision_frame', {
        timestamp: Date.now(),
        image: data.frame.buffer.substring(0, 200) + '...', // Truncated for demo
        analysis: data.analysis,
      });

      // Also invoke Tauri event for React overlay
      this.notifyFrontend('vision_frame', {
        frame: { image: data.frame.buffer, width: data.frame.width, height: data.frame.height },
        analysis: data.analysis,
      });
    });

    // Phase wrap complete
    airi.phaseWrap.on('phase_wrap_complete', (data: any) => {
      this.emit('airi:phase_wrap', data);
      this.notifyFrontend('phase_wrap', data);
    });

    // Edit proposal created
    airi.surgicalEditor.on('edit_proposed', (data: any) => {
      this.emit('airi:edit_proposed', data);
      this.notifyFrontend('edit_proposed', data);
    });

    // Edit committed
    airi.surgicalEditor.on('edit_committed', (data: any) => {
      this.emit('airi:edit_committed', data);
      this.notifyFrontend('edit_committed', data);
    });

    // Error detected by vision
    airi.vision.on('error_detected', (data: any) => {
      this.emit('airi:error_detected', data);
      this.notifyFrontend('error_detected', data);
    });

    // New thought (observation, planning, etc.)
    airi.consciousness.on('thought', (thought: any) => {
      this.emit('airi:thought', thought);
      // Debounce frequent thought events
    });
  }

  /**
   * Send event to VSCodium frontend (React overlay)
   */
  private async notifyFrontend(event: string, data: any): Promise<void> {
    try {
      // Tauri window emit (frontend can listen via `window.__TAURI__.event.listen`)
      await invoke('airi_broadcast', { event, payload: data });
    } catch {
      // Not in Tauri context (dev mode) — fallback to window event
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent(`airi:${event}`, { detail: data }));
      }
    }
  }

  /**
   * Subscribe to AIRI events
   */
  on(event: string, callback: (data: any) => void): void {
    if (!this.listeners.has(event)) {
      this.listeners.set(event, []);
    }
    this.listeners.get(event)!.push(callback);
    this.on(event, callback); // forward from EventEmitter
  }

  /**
   * Get current AIRI state snapshot
   */
  getStateSnapshot(): any {
    return {
      status: airi.getStatus(),
      vision: airi.vision.getState(),
      phaseWrap: airi.phaseWrap.getReports(3),
      editing: {
        pending: airi.surgicalEditor.getPending().length,
        recent: airi.surgicalEditor.getHistory().slice(0, 5),
      },
      consciousness: airi.consciousness.getState(),
      biology: airi.biology.getState(),
    };
  }
}

// Global singleton
export const vsCodiumBridge = VSCodiumBridge.getInstance();

// Export for use in UI components
if (typeof window !== 'undefined') {
  (window as any).__AIRI_BRIDGE__ = vsCodiumBridge;
}
