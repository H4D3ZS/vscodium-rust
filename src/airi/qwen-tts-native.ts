/**
 * Qwen3-TTS Integration - Local Python TTS Server
 * 
 * Uses the local Qwen3-TTS Python package
 * Runs as a subprocess, no Ollama needed
 * 
 * GitHub: https://github.com/QwenLM/Qwen3-TTS
 */

export interface QwenTTSConfig {
    pythonPath: string;
    modelPath: string;
    port: number;
    speed: number;
    pitch: number;
}

export class Qwen3TTSServer {
    private config: QwenTTSConfig;
    private serverUrl: string;
    private isRunning = false;
    private process: any = null;

    constructor(config?: Partial<QwenTTSConfig>) {
        this.config = {
            pythonPath: 'python',
            modelPath: 'C:/Users/HADES/Desktop/vscodium-rust/Qwen3-TTS',
            port: 8080,
            speed: 1.0,
            pitch: 1.0,
            ...config,
        };
        this.serverUrl = `http://localhost:${this.config.port}`;
    }

    /**
     * Start Qwen3-TTS server
     */
    async start(): Promise<boolean> {
        if (this.isRunning) {
            return true;
        }


        try {
            // For Tauri, we'll use fetch to a Python HTTP server
            // The Python script needs to be started separately or via Tauri command
            
            // Check if server is ready
            const ready = await this.waitForServer();
            
            if (ready) {
                this.isRunning = true;
                return true;
            } else {
                console.error('[Qwen3-TTS] ❌ Server failed to start');
                return false;
            }
        } catch (error) {
            console.error('[Qwen3-TTS] Start error:', error);
            return false;
        }
    }

    /**
     * Wait for server to be ready
     */
    private async waitForServer(maxAttempts: number = 30): Promise<boolean> {
        for (let i = 0; i < maxAttempts; i++) {
            try {
                const response = await fetch(`${this.serverUrl}/health`, {
                    method: 'GET',
                    signal: AbortSignal.timeout(1000),
                });
                
                if (response.ok) {
                    return true;
                }
            } catch {
                // Server not ready yet
            }
            
            await new Promise(resolve => setTimeout(resolve, 500));
        }
        
        return false;
    }

    /**
     * Speak text using Qwen3-TTS
     */
    async speak(text: string, emotion?: string): Promise<boolean> {
        if (!this.isRunning) {
            const started = await this.start();
            if (!started) {
                return false;
            }
        }

        try {
            const response = await fetch(`${this.serverUrl}/tts`, {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify({
                    text,
                    emotion: emotion || 'neutral',
                    speed: this.config.speed,
                    pitch: this.config.pitch,
                }),
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}`);
            }

            // Get audio blob and play it
            const audioBlob = await response.blob();
            const audioUrl = URL.createObjectURL(audioBlob);
            const audio = new Audio(audioUrl);
            
            return new Promise((resolve) => {
                audio.onended = () => {
                    URL.revokeObjectURL(audioUrl);
                    resolve(true);
                };
                audio.onerror = () => {
                    URL.revokeObjectURL(audioUrl);
                    resolve(false);
                };
                audio.play();
            });
        } catch (error) {
            console.error('[Qwen3-TTS] Speak error:', error);
            return false;
        }
    }

    /**
     * Stop speaking
     */
    async stop(): Promise<void> {
        // No-op for now - audio will finish naturally
    }

    /**
     * Check if server is running
     */
    async isHealthy(): Promise<boolean> {
        try {
            const response = await fetch(`${this.serverUrl}/health`, {
                method: 'GET',
                signal: AbortSignal.timeout(2000),
            });
            return response.ok;
        } catch {
            return false;
        }
    }

    /**
     * Stop server
     */
    async shutdown(): Promise<void> {
        if (this.process) {
            this.process.kill();
            this.process = null;
        }
        this.isRunning = false;
    }
}

// Export singleton
export const qwenTTS = new Qwen3TTSServer();

// Auto-start on module load (for Tauri)
if (typeof window !== 'undefined') {
    // Try to start server
    qwenTTS.start().catch(console.error);
    
    // Clean up on page unload
    window.addEventListener('beforeunload', () => {
        qwenTTS.shutdown();
    });
}
