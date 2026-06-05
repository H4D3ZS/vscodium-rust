/** Shell profile — which executable to spawn and how to label it in the UI. */
export interface TerminalProfile {
    id: string;
    name: string;
    path: string;
    args?: string[];
    icon: string;
    isDefault: boolean;
    platform?: 'win32' | 'linux' | 'darwin';
}
