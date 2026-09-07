/**
 * Agent Resilience — auto-reconnect, message queuing, and backend health
 * monitoring for bulletproof 24/7 offline AI agent operation.
 *
 * When the inference backend (the local backend/Lemonade) goes down, this module:
 *  1. Queues outgoing messages for later delivery
 *  2. Probes the backend on an exponential-backoff schedule
 *  3. Replays queued messages when the backend recovers
 *  4. Exposes reactive status for UI components (OfflineBanner)
 */

import { invoke } from '@tauri-apps/api/core';

// ── Types ────────────────────────────────────────────────────────────────────

export type BackendStatus = 'connected' | 'reconnecting' | 'offline';

export interface QueuedMessage {
    id: string;
    prompt: string;
    context?: Record<string, unknown>;
    queuedAt: number;
}

export type StatusListener = (status: BackendStatus, queuedCount: number) => void;

// ── Singleton ────────────────────────────────────────────────────────────────

const BACKOFF_SCHEDULE_MS = [5_000, 10_000, 20_000, 40_000, 60_000];
const MAX_QUEUE_SIZE = 20;

class AgentResilience {
    private status: BackendStatus = 'connected';
    private queue: QueuedMessage[] = [];
    private listeners: Set<StatusListener> = new Set();
    private reconnectTimer: ReturnType<typeof setTimeout> | null = null;
    private backoffIndex = 0;
    private probing = false;

    /** Current backend status. */
    getStatus(): BackendStatus { return this.status; }

    /** Number of queued messages waiting for replay. */
    getQueuedCount(): number { return this.queue.length; }

    /** Get a copy of queued messages (for UI display). */
    getQueuedMessages(): QueuedMessage[] { return [...this.queue]; }

    /** Subscribe to status changes. Returns an unsubscribe function. */
    onStatusChange(listener: StatusListener): () => void {
        this.listeners.add(listener);
        return () => this.listeners.delete(listener);
    }

    // ── Status transitions ───────────────────────────────────────────────────

    private setStatus(s: BackendStatus) {
        if (this.status === s) return;
        this.status = s;
        for (const fn of this.listeners) {
            try { fn(s, this.queue.length); } catch { /* listener error */ }
        }
    }

    /** Called by the chat sender when a turn fails with a connection error. */
    markOffline() {
        this.setStatus('reconnecting');
        this.startReconnectLoop();
    }

    /** Called when a probe succeeds — reconnects and replays. */
    private async markConnected() {
        this.setStatus('connected');
        this.backoffIndex = 0;
        this.cancelReconnect();

        // Replay queued messages
        if (this.queue.length > 0) {
            const pending = [...this.queue];
            this.queue = [];
            this.notifyQueueChange();

            for (const msg of pending) {
                try {
                    // Re-dispatch through the Tauri AI chat command.
                    // The caller is responsible for rendering the replayed
                    // messages into the chat panel.
                    await invoke('ai_chat_fast', {
                        prompt: msg.prompt,
                        context: msg.context || {},
                    });
                } catch (err) {
                    console.warn('[AgentResilience] Replay failed for queued message:', err);
                    // Re-queue if still failing
                    if (this.queue.length < MAX_QUEUE_SIZE) {
                        this.queue.push(msg);
                    }
                }
            }
            this.notifyQueueChange();
        }
    }

    // ── Queue management ─────────────────────────────────────────────────────

    /** Queue a message for later delivery. Returns true if queued. */
    queueMessage(prompt: string, context?: Record<string, unknown>): boolean {
        if (this.queue.length >= MAX_QUEUE_SIZE) return false;
        this.queue.push({
            id: `q-${Date.now()}-${Math.random().toString(36).slice(2, 6)}`,
            prompt,
            context,
            queuedAt: Date.now(),
        });
        this.notifyQueueChange();
        return true;
    }

    /** Remove a specific message from the queue. */
    unqueueMessage(id: string) {
        this.queue = this.queue.filter(m => m.id !== id);
        this.notifyQueueChange();
    }

    /** Clear all queued messages. */
    clearQueue() {
        this.queue = [];
        this.notifyQueueChange();
    }

    private notifyQueueChange() {
        for (const fn of this.listeners) {
            try { fn(this.status, this.queue.length); } catch { /* listener error */ }
        }
    }

    // ── Reconnect loop ───────────────────────────────────────────────────────

    private startReconnectLoop() {
        if (this.reconnectTimer || this.probing) return;
        this.probe();
    }

    private cancelReconnect() {
        if (this.reconnectTimer) {
            clearTimeout(this.reconnectTimer);
            this.reconnectTimer = null;
        }
    }

    private async probe() {
        if (this.probing) return;
        this.probing = true;

        try {
            const backend = localStorage.getItem('inferenceBackend') || 'lemonade';
            // Both status commands resolve with a boolean (false = unreachable)
            // rather than rejecting, so check the value, not just the resolve.
            const up = backend === 'lemonade'
                ? await invoke<boolean>('check_lemonade_status')
                : await invoke<boolean>('check_lemonade_status');
            if (!up) throw new Error(`${backend} unreachable`);
            this.markConnected();
        } catch {
            this.probing = false;
            this.scheduleNextProbe();
        }
    }

    private scheduleNextProbe() {
        const delay = BACKOFF_SCHEDULE_MS[Math.min(this.backoffIndex, BACKOFF_SCHEDULE_MS.length - 1)];
        this.backoffIndex++;
        this.reconnectTimer = setTimeout(() => {
            this.reconnectTimer = null;
            this.probe();
        }, delay);
    }

    /** Force an immediate reconnect attempt (e.g. user clicked retry). */
    forceReconnect() {
        this.cancelReconnect();
        this.backoffIndex = 0;
        this.setStatus('reconnecting');
        this.probe();
    }

    /** Clean shutdown. */
    destroy() {
        this.cancelReconnect();
        this.listeners.clear();
    }
}

/** Singleton instance — import and use across the app. */
export const agentResilience = new AgentResilience();
