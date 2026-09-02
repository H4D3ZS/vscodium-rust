import { invoke as bridgeInvoke } from '../tauri_bridge';

export function invoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
    return bridgeInvoke<T>(cmd, args);
}

