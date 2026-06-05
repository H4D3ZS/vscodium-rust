/** Port — `.vscode/settings.json` for the active workspace root. */
export interface IWorkspaceSettingsRepository {
    load(root?: string): Promise<Record<string, unknown>>;
    save(settings: Record<string, unknown>, root?: string): Promise<void>;
}
