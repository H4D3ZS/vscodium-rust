/**
 * Emulator Preview Component
 *
 * Renders real-time emulator framebuffer via Tauri events.
 * Backend captures emulator window via BitBlt at ~15fps,
 * emits `emulator:frame` events with base64 PNG frames.
 */

import React, { useEffect, useRef, useState, useCallback } from 'react';
import { invoke } from '../tauri_bridge';

interface EmulatorPreviewProps {
  width?: number;
  height?: number;
  showFps?: boolean;
  showControls?: boolean;
  onFrameCapture?: (frame: ImageData) => void;
}

export const EmulatorPreview: React.FC<EmulatorPreviewProps> = ({
  width = 360,
  height = 640,
  showFps = true,
  showControls = true,
  onFrameCapture,
}) => {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [fps, setFps] = useState(0);
  const [isPaused, setIsPaused] = useState(false);
  const [streamState, setStreamState] = useState<{
    frameCount: number;
    lastFrameTime: number | null;
  }>({ frameCount: 0, lastFrameTime: null });

  const frameTimes = useRef<number[]>([]);
  const startTime = useRef(Date.now());

  // Start/stop emulator stream via backend
  const startStream = useCallback(async () => {
    try {
      await invoke('start_emulator_stream', { deviceId: 'emulator-5554' });
      setIsConnected(true);
    } catch (err: any) {
      console.error('[EmulatorPreview] Failed to start stream:', err);
      setIsConnected(false);
    }
  }, []);

  const stopStream = useCallback(async () => {
    try {
      await invoke('stop_emulator_stream');
      setIsConnected(false);
    } catch (err: any) {
      console.error('[EmulatorPreview] Failed to stop stream:', err);
    }
  }, []);

  // Toggle stream
  const toggleStream = useCallback(async () => {
    if (isPaused) {
      await startStream();
      setIsPaused(false);
    } else {
      await stopStream();
      setIsPaused(true);
    }
  }, [isPaused, startStream, stopStream]);

  // Listen for emulator frames from backend
  useEffect(() => {
    let unsubFns: (() => void)[] = [];

    import('@tauri-apps/api/event').then(({ listen }) => {
      listen<{
        base64: string;
        width: number;
        height: number;
        frame: number;
        timestamp: number;
      }>('emulator:frame', (event) => {
        const { base64: b64, width: fw, height: fh } = event.payload;
        if (!canvasRef.current) return;
        const ctx = canvasRef.current.getContext('2d');
        if (!ctx) return;
        const img = new Image();
        img.onload = () => {
          ctx.drawImage(img, 0, 0, width, height);
        };
        img.src = 'data:image/png;base64,' + b64;

        // FPS tracking
        const now = Date.now();
        frameTimes.current.push(now);
        const oneSecAgo = now - 1000;
        frameTimes.current = frameTimes.current.filter(t => t > oneSecAgo);
        setFps(frameTimes.current.length);

        setStreamState(prev => ({
          frameCount: prev.frameCount + 1,
          lastFrameTime: now,
        }));

        // Fire capture callback if provided
        if (onFrameCapture) {
          const imageData = ctx.getImageData(0, 0, width, height);
          onFrameCapture(imageData);
        }
      }).then(u => unsubFns.push(u));

      // Auto-start stream
      invoke('start_emulator_stream', { deviceId: 'emulator-5554' }).then(() => {
        setIsConnected(true);
      }).catch((err: any) => {
        console.warn('[EmulatorPreview] Stream start (expected if no emulator):', err);
      });
    });

    return () => {
      unsubFns.forEach(u => u());
      invoke('stop_emulator_stream').catch(() => {});
    };
  }, [width, height, onFrameCapture]);

  // Capture screenshot
  const captureScreenshot = useCallback(() => {
    if (canvasRef.current && onFrameCapture) {
      const ctx = canvasRef.current.getContext('2d');
      if (ctx) {
        const imageData = ctx.getImageData(0, 0, width, height);
        onFrameCapture(imageData);
      }
    }
  }, [onFrameCapture, width, height]);

  return (
    <div className="emulator-preview" style={styles.container}>
      {/* Header */}
      <div style={styles.header}>
        <div style={styles.statusIndicator}>
          <span style={{
            ...styles.statusDot,
            background: isConnected ? '#4ade80' : '#ef4444',
          }} />
          <span style={styles.statusText}>
            {isConnected ? 'Live' : 'Disconnected'}
          </span>
        </div>

        {showFps && (
          <div style={styles.fpsCounter}>
            {fps} fps
          </div>
        )}
      </div>

      {/* Canvas */}
      <div style={styles.videoContainer}>
        <canvas
          ref={canvasRef}
          width={width}
          height={height}
          style={styles.canvas}
        />

        {!isConnected && (
          <div style={styles.overlay}>
            <div style={styles.overlayText}>
              {isPaused ? 'Paused' : 'Starting stream...'}
            </div>
          </div>
        )}
      </div>

      {/* Controls */}
      {showControls && (
        <div style={styles.controls}>
          <button
            onClick={captureScreenshot}
            style={styles.button}
            disabled={!isConnected}
          >
            📸 Screenshot
          </button>

          <button
            onClick={toggleStream}
            style={styles.button}
          >
            {isPaused ? '▶ Resume' : '⏸ Pause'}
          </button>
        </div>
      )}
    </div>
  );
};

const styles: Record<string, React.CSSProperties> = {
  container: {
    display: 'flex', flexDirection: 'column', gap: '8px',
    padding: '12px', background: 'var(--vscode-sideBar-background)',
    height: '100%',
  },
  header: {
    display: 'flex', justifyContent: 'space-between', alignItems: 'center',
    fontSize: '12px', color: 'var(--vscode-descriptionForeground)',
  },
  statusIndicator: { display: 'flex', alignItems: 'center', gap: '4px' },
  statusDot: { width: '8px', height: '8px', borderRadius: '50%' },
  statusText: { fontSize: '12px' },
  fpsCounter: { fontSize: '11px', opacity: 0.6 },
  videoContainer: {
    position: 'relative', flex: 1,
    background: '#000', borderRadius: '4px', overflow: 'hidden',
    display: 'flex', alignItems: 'center', justifyContent: 'center',
  },
  canvas: { width: '100%', height: '100%', objectFit: 'contain' },
  overlay: {
    position: 'absolute', inset: 0,
    display: 'flex', alignItems: 'center', justifyContent: 'center',
    background: 'rgba(0,0,0,0.6)',
  },
  overlayText: { color: 'var(--vscode-editor-foreground, #fff)', fontSize: '14px', opacity: 0.7 },
  controls: {
    display: 'flex', gap: '8px', justifyContent: 'center',
  },
  button: {
    padding: '6px 12px',
    background: 'var(--vscode-button-background, #0e639c)',
    color: 'var(--vscode-button-foreground, #ffffff)',
    border: 'none', borderRadius: '4px', fontSize: '12px',
    fontWeight: 500, cursor: 'pointer',
  },
};
