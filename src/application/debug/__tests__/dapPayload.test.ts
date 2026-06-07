import { describe, expect, it } from 'vitest';
import {
    isDapInitializeResponse,
    isDapStoppedEvent,
    parseDapPayload,
} from '../dapPayload';

describe('dapPayload', () => {
    it('parses string JSON payloads', () => {
        const msg = parseDapPayload('{"type":"event","event":"stopped","body":{}}');
        expect(msg?.event).toBe('stopped');
    });

    it('detects initialize response', () => {
        const msg = parseDapPayload({
            type: 'response',
            command: 'initialize',
            success: true,
        });
        expect(msg && isDapInitializeResponse(msg)).toBe(true);
    });

    it('detects stopped event', () => {
        const msg = parseDapPayload({ type: 'event', event: 'stopped', body: { threadId: 1 } });
        expect(msg && isDapStoppedEvent(msg)).toBe(true);
    });
});
