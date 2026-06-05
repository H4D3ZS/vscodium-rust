/**
 * Port: backend PTY operations.
 * Implementations live in `infrastructure/terminal/` (Tauri invoke adapter).
 */
export interface ITerminalRepository {
    spawn(id: string, shell?: string): Promise<void>;
    close(id: string): Promise<void>;
    send(id: string, data: string): Promise<void>;
    resize(id: string, rows: number, cols: number): Promise<void>;
    takePending(id: string): Promise<string>;
    readOutput(id: string): Promise<string>;
    getStatus(id: string): Promise<{ active: boolean; success?: boolean }>;
    listAvailableShells(): Promise<string[]>;
}
