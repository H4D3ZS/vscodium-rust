import { invoke } from '../../tauri_bridge';
import type { IFileRepository } from '../../domain/editor/IFileRepository';

export class TauriFileRepository implements IFileRepository {
    read(path: string): Promise<string> {
        return invoke<string>('read_file', { path });
    }

    write(path: string, content: string): Promise<void> {
        return invoke('write_file', { path, content });
    }

    listDirectory(path: string): Promise<unknown[]> {
        return invoke<unknown[]>('get_file_tree', { path });
    }
}

export const fileRepository = new TauriFileRepository();
