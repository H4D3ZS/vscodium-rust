/**
 * Android Emulator Stream Server
 * 
 * Captures emulator screen via scrcpy/adb and streams to IDE via WebSocket
 */

import { spawn, ChildProcess } from 'child_process';
import WebSocket, { WebSocketServer } from 'ws';
import { createServer, Server } from 'http';

export interface StreamServerConfig {
  port: number;
  emulatorPort: number;
  fps: number;
  bitrate: string;
}

export class EmulatorStreamServer {
  private config: StreamServerConfig;
  private wss: WebSocketServer | null = null;
  private httpServer: Server | null = null;
  private scrcpy: ChildProcess | null = null;
  private isRunning: boolean = false;

  constructor(config: Partial<StreamServerConfig> = {}) {
    this.config = {
      port: config.port || 8989,
      emulatorPort: config.emulatorPort || 5555,
      fps: config.fps || 10,
      bitrate: config.bitrate || '2M',
    };

    console.log(`📱 [EmulatorStream] Server configured on port ${this.config.port}`);
  }

  /**
   * Start the stream server
   */
  async start(): Promise<void> {
    if (this.isRunning) {
      console.warn('[EmulatorStream] Already running');
      return;
    }

    console.log(' [EmulatorStream] Starting stream server...');

    // Step 1: Create HTTP + WebSocket server
    this.httpServer = createServer((req, res) => {
      // Health check endpoint
      if (req.url === '/health') {
        res.writeHead(200);
        res.end(JSON.stringify({
          status: 'running',
          fps: this.config.fps,
          connected: this.wss?.clients.size || 0,
        }));
        return;
      }

      res.writeHead(404);
      res.end('Not found');
    });

    this.wss = new WebSocketServer({ server: this.httpServer });

    this.wss.on('connection', (ws) => {
      console.log('🔗 [EmulatorStream] Client connected');
      
      ws.on('close', () => {
        console.log('🔌 [EmulatorStream] Client disconnected');
      });

      ws.on('error', (error) => {
        console.error('[EmulatorStream] WebSocket error:', error);
      });
    });

    // Start HTTP server
    await new Promise<void>((resolve) => {
      this.httpServer!.listen(this.config.port, () => {
        console.log(`✅ [EmulatorStream] HTTP server listening on port ${this.config.port}`);
        resolve();
      });
    });

    // Step 2: Start scrcpy to capture emulator
    await this.startScrcpy();

    this.isRunning = true;
    console.log('✅ [EmulatorStream] Stream server ready');
  }

  /**
   * Start scrcpy capture
   */
  private async startScrcpy(): Promise<void> {
    return new Promise((resolve, reject) => {
      // Method 1: Use scrcpy with V4L2 output (Linux)
      // Method 2: Use adb screencap + convert (Windows fallback)
      
      if (process.platform === 'win32') {
        // Windows: Use adb screencap
        this.startAdbCapture(resolve);
      } else {
        // Linux/Mac: Use scrcpy
        this.startScrcpyCapture(resolve);
      }
    });
  }

  /**
   * Start scrcpy capture (Linux/Mac)
   */
  private startScrcpyCapture(resolve: () => void): void {
    this.scrcpy = spawn('scrcpy', [
      '--no-display',
      '--no-control',
      '--no-audio',
      '--bit-rate', this.config.bitrate,
      '--max-fps', this.config.fps.toString(),
      '--tcpip=5555',
      '--render-driver=software',
    ], {
      stdio: ['pipe', 'pipe', 'pipe'],
    });

    this.scrcpy.stdout.on('data', (data: Buffer) => {
      // Broadcast frame to all connected clients
      this.broadcastFrame(data);
    });

    this.scrcpy.stderr.on('data', (data: Buffer) => {
      console.error('[EmulatorStream] scrcpy error:', data.toString());
    });

    this.scrcpy.on('spawn', () => {
      console.log('✅ [EmulatorStream] scrcpy started');
      resolve();
    });

    this.scrcpy.on('error', (error) => {
      console.error('[EmulatorStream] scrcpy error:', error);
      reject(error);
    });
  }

  /**
   * Start ADB screencap (Windows)
   */
  private startAdbCapture(resolve: () => void): void {
    console.log('📱 [EmulatorStream] Starting ADB capture for Windows...');

    // Capture frame every 100ms (10fps)
    const captureInterval = setInterval(async () => {
      if (!this.isRunning || !this.wss) {
        clearInterval(captureInterval);
        return;
      }

      try {
        // Capture frame via adb
        const frame = await this.captureFrame();
        
        if (frame && this.wss.clients.size > 0) {
          // Broadcast to all clients
          this.wss.clients.forEach((client) => {
            if (client.readyState === WebSocket.OPEN) {
              client.send(frame);
            }
          });
        }
      } catch (error) {
        console.error('[EmulatorStream] Capture error:', error);
      }
    }, 1000 / this.config.fps);

    // Store interval for cleanup
    (this as any).captureInterval = captureInterval;

    resolve();
  }

  /**
   * Capture single frame via ADB
   */
  private async captureFrame(): Promise<Buffer | null> {
    return new Promise((resolve) => {
      const adb = spawn('adb', ['-s', 'emulator-5554', 'shell', 'screencap', '-p'], {
        stdio: ['pipe', 'pipe', 'pipe'],
      });

      let frameData = Buffer.alloc(0);

      adb.stdout.on('data', (data: Buffer) => {
        frameData = Buffer.concat([frameData, data]);
      });

      adb.on('close', (code) => {
        if (code === 0 && frameData.length > 0) {
          resolve(frameData);
        } else {
          resolve(null);
        }
      });

      adb.on('error', () => {
        resolve(null);
      });
    });
  }

  /**
   * Broadcast frame to all clients
   */
  private broadcastFrame(data: Buffer): void {
    if (!this.wss) return;

    this.wss.clients.forEach((client) => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(data);
      }
    });
  }

  /**
   * Stop the stream server
   */
  stop(): void {
    console.log('⏹️ [EmulatorStream] Stopping stream server...');

    this.isRunning = false;

    // Clear ADB capture interval
    if ((this as any).captureInterval) {
      clearInterval((this as any).captureInterval);
    }

    // Kill scrcpy
    if (this.scrcpy) {
      this.scrcpy.kill();
      this.scrcpy = null;
    }

    // Close WebSocket server
    if (this.wss) {
      this.wss.close();
      this.wss = null;
    }

    // Close HTTP server
    if (this.httpServer) {
      this.httpServer.close();
      this.httpServer = null;
    }

    console.log('✅ [EmulatorStream] Stopped');
  }

  /**
   * Get server status
   */
  getStatus(): {
    isRunning: boolean;
    port: number;
    fps: number;
    clientCount: number;
  } {
    return {
      isRunning: this.isRunning,
      port: this.config.port,
      fps: this.config.fps,
      clientCount: this.wss?.clients.size || 0,
    };
  }
}

// Singleton instance
export const emulatorStream = new EmulatorStreamServer();
