export interface LogcatEntry {
    raw: string;
    level?: string | null;
    tag?: string | null;
    message: string;
}

export interface ILogcatRepository {
    start(device?: string, filter?: string): Promise<void>;
    stop(): Promise<void>;
    status(): Promise<{ running: boolean }>;
}
