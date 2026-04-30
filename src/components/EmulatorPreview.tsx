/**
 * Emulator Preview Component
 * 
 * Embeds live Android emulator stream in VSCodium-Rust side panel
 */

import React, { useEffect, useRef, useState, useCallback } from 'react';
import { airiVision } from '../airi/vision-system';

interface EmulatorPreviewProps {
  streamUrl?: string;
  width?: number;
  height?: number;
  showFps?: boolean;
  showControls?: boolean;
  onFrameCapture?: (frame: ImageData) => void;
}

export const EmulatorPreview: React.FC<EmulatorPreviewProps> = ({
  streamUrl = 'ws://localhost:8989',
  width = 360,
  height = 640,
  showFps = true,
  showControls = true,
  onFrameCapture,
}) => {
  const videoRef = useRef<HTMLVideoElement>(null);
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const [isConnected, setIsConnected] = useState(false);
  const [fps, setFps] = useState(0);
  const [isPaused, setIsPaused] = useState(false);
  const [visionState, setVisionState] = useState<any>(null);

  // Connect to emulator stream
  useEffect(() => {
    const connectStream = async () => {
      try {
        // Start vision capture
        await airiVision.start();
        
        // Set up frame listener
        airiVision.on('frame', (frame) => {
          if (videoRef.current && canvasRef.current) {
            // Convert YUV frame to RGB and display
            const ctx = canvasRef.current.getContext('2d');
            if (ctx) {
              const imageData = ctx.createImageData(frame.width, frame.height);
              // Convert YUV to RGB (simplified - production would use proper conversion)
              convertYuvToRgb(frame.buffer, imageData.data);
              ctx.putImageData(imageData, 0, 0);
            }
          }
        });

        // FPS monitoring
        const fpsInterval = setInterval(() => {
          const state = airiVision.getState();
          setFps(state.fps);
          setVisionState(state);
          setIsConnected(state.isRunning);
        }, 1000);

        return () => {
          clearInterval(fpsInterval);
          airiVision.stop();
        };
      } catch (error) {
        console.error('Failed to connect emulator stream:', error);
        setIsConnected(false);
      }
    };

    connectStream();
  }, [streamUrl]);

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

  // Toggle stream
  const toggleStream = useCallback(() => {
    if (isPaused) {
      airiVision.start();
      setIsPaused(false);
    } else {
      airiVision.stop();
      setIsPaused(true);
    }
  }, [isPaused]);

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

      {/* Video/Canvas */}
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
              {isPaused ? 'Paused' : 'Connecting...'}
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
          
          <button
            onClick={() => {
              // Refresh frame
              airiVision.emit('refresh');
            }}
            style={styles.button}
            disabled={!isConnected}
          >
            🔄 Refresh
          </button>
        </div>
      )}

      {/* Vision State Info */}
      {visionState && (
        <div style={styles.infoPanel}>
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Frames:</span>
            <span style={styles.infoValue}>{visionState.frameCount}</span>
          </div>
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Last Frame:</span>
            <span style={styles.infoValue}>
              {visionState.lastFrameTime 
                ? new Date(visionState.lastFrameTime).toLocaleTimeString()
                : 'N/A'}
            </span>
          </div>
        </div>
      )}
    </div>
  );
};

/**
 * Convert YUV to RGB
 * Simplified implementation - production would use proper color space conversion
 */
function convertYuvToRgb(yuvData: Uint8Array, rgbData: Uint8ClampedArray): void {
  const len = yuvData.length;
  
  for (let i = 0; i < len; i += 4) {
    const y = yuvData[i];
    const u = yuvData[i + 1];
    const v = yuvData[i + 2];
    
    // YUV to RGB conversion
    const r = Math.max(0, Math.min(255, y + 1.402 * (v - 128)));
    const g = Math.max(0, Math.min(255, y - 0.344 * (u - 128) - 0.714 * (v - 128)));
    const b = Math.max(0, Math.min(255, y + 1.772 * (u - 128)));
    
    rgbData[i] = r;
    rgbData[i + 1] = g;
    rgbData[i + 2] = b;
    rgbData[i + 3] = 255;  // Alpha
  }
}

// Styles
const styles: Record<string, React.CSSProperties> = {
  container: {
    display: 'flex',
    flexDirection: 'column',
    gap: '8px',
    padding: '12px',
    background: 'var(--vscode-sideBar-background)',
    height: '100%',
  },
  header: {
    display: 'flex',
    justifyContent: 'space-between',
    alignItems: 'center',
    fontSize: '12px',
    color: 'var(--vscode-descriptionForeground)',
  },
  statusIndicator: {
    display: 'flex',
    alignItems: 'center',
    gap: '4px',
  },
  statusDot: {
    width: '8px',
    height: '8px',
    borderRadius: '50%',
  },
  statusText: {
    fontSize: '12px',
  },
  fpsCounter: {
    fontFamily: 'var(--vscode-editor-font-family)',
    background: 'var(--vscode-badge-background)',
    color: 'var(--vscode-badge-foreground)',
    padding: '2px 6px',
    borderRadius: '4px',
    fontSize: '11px',
  },
  videoContainer: {
    position: 'relative',
    flex: 1,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    background: '#000',
    borderRadius: '8px',
    overflow: 'hidden',
  },
  canvas: {
    maxWidth: '100%',
    maxHeight: '100%',
    objectFit: 'contain',
  },
  overlay: {
    position: 'absolute',
    top: 0,
    left: 0,
    right: 0,
    bottom: 0,
    display: 'flex',
    alignItems: 'center',
    justifyContent: 'center',
    background: 'rgba(0, 0, 0, 0.7)',
  },
  overlayText: {
    color: '#fff',
    fontSize: '14px',
    fontWeight: 500,
  },
  controls: {
    display: 'flex',
    gap: '8px',
  },
  button: {
    flex: 1,
    padding: '6px',
    border: '1px solid var(--vscode-panel-border)',
    background: 'var(--vscode-button-background)',
    color: 'var(--vscode-button-foreground)',
    borderRadius: '4px',
    cursor: 'pointer',
    fontSize: '12px',
  },
  infoPanel: {
    padding: '8px',
    background: 'var(--vscode-textBlockQuote-background)',
    borderRadius: '4px',
    fontSize: '11px',
  },
  infoRow: {
    display: 'flex',
    justifyContent: 'space-between',
    marginBottom: '4px',
  },
  infoLabel: {
    color: 'var(--vscode-descriptionForeground)',
  },
  infoValue: {
    fontFamily: 'var(--vscode-editor-font-family)',
  },
};

export default EmulatorPreview;
