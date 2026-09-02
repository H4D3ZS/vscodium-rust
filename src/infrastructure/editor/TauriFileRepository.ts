import { invoke } from '../../tauri_bridge';
import type { IFileRepository } from '../../domain/editor/IFileRepository';

export interface FileReadResult {
    large?: boolean;
    size?: number;
    lines?: number;
    content?: string;
    preview?: string;
}

export class TauriFileRepository implements IFileRepository {
    async read(path: string): Promise<string> {
        return invoke<string>('read_file', { path });
    }

    async readRaw(path: string): Promise<FileReadResult> {
        const content = await invoke<string>('read_file', { path });
        return { large: false, content };
    }

    write(path: string, content: string): Promise<void> {
        return invoke('write_file', { path, content });
    }

    listDirectory(path: string): Promise<unknown[]> {
        return invoke<unknown[]>('get_file_tree', { path });
    }
}

export const fileRepository = new TauriFileRepository();
