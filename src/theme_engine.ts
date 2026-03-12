import { invoke } from './tauri_bridge';

export interface VscodeTheme {
    label: string;
    path: string;
    extension: string;
}

export async function applyTheme(themePath: string) {
    try {
        const colors = await invoke<Record<string, string>>('load_extension_theme', { path: themePath });
        
        // Map VS Code color keys to our CSS variables
        const root = document.documentElement;
        for (const [key, value] of Object.entries(colors)) {
            // Convert 'editor.background' to '--vscode-editor-background'
            const cssVar = `--vscode-${key.replace(/\./g, '-')}`;
            root.style.setProperty(cssVar, value);
        }

        // Special handling for Monaco theme
        // For a full implementation, we'd define a custom Monaco theme using the colors.
        // For now, we'll just set it to 'vs-dark' or 'vs' based on the background.
        const bg = colors['editor.background'] || '#1e1e1e';
        const isDark = isColorDark(bg);
        
        return isDark ? 'vs-dark' : 'vs';
    } catch (e) {
        console.error("Failed to apply theme:", e);
        return 'vs-dark';
    }
}

function isColorDark(hex: string): boolean {
    if (!hex || hex[0] !== '#') return true;
    const r = parseInt(hex.slice(1, 3), 16);
    const g = parseInt(hex.slice(3, 5), 16);
    const b = parseInt(hex.slice(5, 7), 16);
    const luminance = (0.299 * r + 0.587 * g + 0.114 * b) / 255;
    return luminance < 0.5;
}
