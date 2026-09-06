/**
 * Remote agent bridge — frontend bindings.
 *
 * Starts/stops the localhost WebSocket in `src-tauri/src/remote_bridge.rs` that
 * lets a browser or companion app drive the built-in autonomous agent.
 *
 * Connect (from anywhere on the machine):
 *   const ws = new WebSocket(info.url);                 // ws://127.0.0.1:<port>/agent?token=...
 *   ws.onmessage = e => console.log(JSON.parse(e.data)); // {type:"delta"|"done"|"error"|"ready"|"pong"}
 *   ws.send(JSON.stringify({ type: "prompt", text: "…", mode: "Agent", root_access: false }));
 */

import { invoke } from '../tauri_bridge';

export interface RemoteInfo {
    running: boolean;
    port: number;
    /** Only populated by `start`; `status` never returns it. */
    token: string;
    url: string;
}

/** Start the bridge (idempotent). Returns the connection URL incl. the token. */
export function startRemoteBridge(port?: number): Promise<RemoteInfo> {
    return invoke<RemoteInfo>('remote_bridge_start', { port: port ?? null });
}

export function stopRemoteBridge(): Promise<void> {
    return invoke<void>('remote_bridge_stop');
}

/** Current state — `token` is always empty here (never leaked from status). */
export function remoteBridgeStatus(): Promise<RemoteInfo> {
    return invoke<RemoteInfo>('remote_bridge_status');
}
