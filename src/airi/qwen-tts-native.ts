/**
 * Qwen3-TTS Integration - Local Python TTS Server
 * 
 * Uses the local Qwen3-TTS Python package
 * Runs as a subprocess, no Ollama needed
 * 
 * GitHub: https://github.com/QwenLM/Qwen3-TTS
 * 
 * IMPORTANT: This module is lazy — it does NOT auto-start or poll the server.
 * The health check only runs when the user explicitly selects the "qwen-native"
 * TTS strategy in Settings. This prevents console spam when the server isn't
 * running (which is the default case).
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
    /** Circuit breaker: if the server has failed N times, stop polling. */
    private consecutiveFailures = 0;
    private static readonly MAX_FAILURES = 3;

    constructor(config?: Partial<QwenTTSConfig>) {
        this.config = {
            pythonPath: 'python',
            // No default install location — pass modelPath explicitly when
            // running a local Qwen3-TTS checkout (server is external anyway).
            modelPath: '',
            port: 8080,
            speed: 1.0,
            pitch: 1.0,
            ...config,
        };
        this.serverUrl = `http://localhost:${this.config.port}`;
    }

    /**
     * Start Qwen3-TTS server (only checks if the external server is reachable)
     */
    async start(): Promise<boolean> {
        if (this.isRunning) {
            return true;
        }

        // Circuit breaker — avoid hammering a dead server
        if (this.consecutiveFailures >= Qwen3TTSServer.MAX_FAILURES) {
            return false;
        }

        try {
            const ready = await this.checkOnce();

            if (ready) {
                this.isRunning = true;
                this.consecutiveFailures = 0;
                console.log('[Qwen3-TTS] ✅ Native server connected on port', this.config.port);
                return true;
            } else {
                this.consecutiveFailures++;
                if (this.consecutiveFailures >= Qwen3TTSServer.MAX_FAILURES) {
                    console.warn('[Qwen3-TTS] ⚠ Server unreachable after', this.consecutiveFailures, 'attempts. Will not retry until reset.');
                }
                return false;
            }
        } catch (error) {
            this.consecutiveFailures++;
            console.error('[Qwen3-TTS] Start error:', error);
            return false;
        }
    }

    /**
     * Single health check — no loop, no retry, no spam.
     */
    private async checkOnce(): Promise<boolean> {
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
     * Reset the circuit breaker (e.g. when user re-selects qwen-native in Settings)
     */
    reset(): void {
        this.consecutiveFailures = 0;
        this.isRunning = false;
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
            this.isRunning = false; // Mark as down so next call retries
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
        return this.checkOnce();
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
export const qwenNativeTTS = new Qwen3TTSServer();

// NO auto-start on module load. The server is only checked when the user
// selects "qwen-native" as their TTS strategy in Settings. This prevents
// the flood of CORS errors in the console when the Python server isn't running.

// Clean up on page unload
if (typeof window !== 'undefined') {
    window.addEventListener('beforeunload', () => {
        qwenNativeTTS.shutdown();
    });
}
