/**
 * Emulator Preview Component
 * 
 * Embeds live Android emulator stream in VSCodium-Rust side panel
 * Uses WebSocket to receive frames from backend scrcpy/adb capture
 */

import React, { useEffect, useRef, useState, useCallback } from 'react';
// Note: Vision system runs in backend (Tauri/Node), not browser
// import { airiVision } from '../airi/vision-system';

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
  const [streamState, setStreamState] = useState<{frameCount: number, lastFrameTime: number | null}>({frameCount: 0, lastFrameTime: null});

  // Connect to emulator stream via WebSocket
  useEffect(() => {
    const ws = new WebSocket(streamUrl);
    
    ws.onopen = () => {
      console.log('📺 [EmulatorPreview] Connected to stream server');
      setIsConnected(true);
    };
    
    ws.onmessage = (event) => {
      // Receive frame data (base64 or binary)
      if (canvasRef.current) {
        const ctx = canvasRef.current.getContext('2d');
        if (ctx) {
          // Create image from received data
          const img = new Image();
          img.onload = () => {
            ctx.drawImage(img, 0, 0, width, height);
          };
          
          if (typeof event.data === 'string') {
            // Base64 encoded image
            img.src = 'data:image/png;base64,' + event.data;
          } else {
            // Binary data - would need to convert to blob
            console.warn('[EmulatorPreview] Binary frame data not yet supported');
          }
        }
      }
      
      // Update frame counter
      setStreamState(prev => ({
        frameCount: prev.frameCount + 1,
        lastFrameTime: Date.now(),
      }));
    };
    
    ws.onclose = () => {
      console.log('📺 [EmulatorPreview] Disconnected from stream server');
      setIsConnected(false);
    };
    
    ws.onerror = (error) => {
      console.error('[EmulatorPreview] WebSocket error:', error);
      setIsConnected(false);
    };
    
    // FPS monitoring
    const fpsInterval = setInterval(() => {
      // Simple FPS calculation based on frame updates
      setFps(prev => {
        // This is a placeholder - real FPS would track actual frame times
        return streamState.frameCount > 0 ? Math.round(streamState.frameCount / (Date.now() / 1000)) : 0;
      });
    }, 1000);

    return () => {
      clearInterval(fpsInterval);
      ws.close();
    };
  }, [streamUrl, width, height]);

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

      {/* Stream State Info */}
      {streamState.lastFrameTime && (
        <div style={styles.infoPanel}>
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Frames:</span>
            <span style={styles.infoValue}>{streamState.frameCount}</span>
          </div>
          <div style={styles.infoRow}>
            <span style={styles.infoLabel}>Last Frame:</span>
            <span style={styles.infoValue}>
              {new Date(streamState.lastFrameTime).toLocaleTimeString()}
            </span>
          </div>
        </div>
      )}
    </div>
  );
};

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
