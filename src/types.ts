export interface FileEntry {
    name: string;
    path: string;
    is_dir: boolean;
    children: FileEntry[] | null;
}

export interface Settings {
    theme: string;
    font_size: number;
}

export interface GitStatus {
    path: string;
    status: string;
}

export interface SearchResult {
    path: string;
    line: number;
    content: string;
}

export interface Extension {
    id: string;
    name: string;
    description: string;
    version: string;
    is_installed: boolean;
}

// Ensure monaco is available on window
declare global {
    interface Window {
        __TAURI__: {
            core: {
                invoke: <T>(cmd: string, args?: Record<string, unknown>) => Promise<T>;
            };
            dialog: {
                open: (options: any) => Promise<string | string[] | null>;
            };
            event: {
                listen: (event: string, handler: (event: any) => void) => Promise<any>;
            };
            window: {
                getCurrentWindow: () => any;
            };
        };
        monacoEditor: any;
        extSyncTimeout: any;
        activeFilePath: string;
        activeRoot: string;
        TerminalManager: any;
        switchSidebarView: (view: string) => void;
    }
}
