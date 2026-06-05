/** Port — read/write files and workspace tree via Tauri backend. */
export interface IFileRepository {
    read(path: string): Promise<string>;
    write(path: string, content: string): Promise<void>;
    listDirectory(path: string): Promise<unknown[]>;
}
