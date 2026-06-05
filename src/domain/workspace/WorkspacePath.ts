/**
 * Value object: validated workspace root path.
 * WHY separate from string? Path normalization (strip NUL, trim) belongs in domain,
 * not scattered across App.tsx and editorSlice.
 */
export type WorkspacePath = { readonly value: string };

export function parseWorkspacePath(raw: string | null | undefined): WorkspacePath | null {
    if (!raw) return null;
    const cleaned = raw.split('\0')[0].trim();
    return cleaned.length > 0 ? { value: cleaned } : null;
}

export function workspacePathToString(path: WorkspacePath): string {
    return path.value;
}
