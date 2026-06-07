/** Parse DAP JSON from Tauri event payload (string or object). */
export function parseDapPayload(raw: unknown): Record<string, unknown> | null {
    if (typeof raw === 'string') {
        try {
            return JSON.parse(raw) as Record<string, unknown>;
        } catch {
            return null;
        }
    }
    if (raw && typeof raw === 'object') return raw as Record<string, unknown>;
    return null;
}

export function isDapStoppedEvent(msg: Record<string, unknown>): boolean {
    return String(msg.type ?? '') === 'event' && String(msg.event ?? '') === 'stopped';
}

export function isDapInitializeResponse(msg: Record<string, unknown>): boolean {
    return String(msg.type ?? '') === 'response'
        && String(msg.command ?? '') === 'initialize';
}
