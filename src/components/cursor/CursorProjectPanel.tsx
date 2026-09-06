import React from 'react';
import { invoke } from '../../tauri_bridge';

/**
 * `.cursor/` project surface — mirrors what Cursor keeps in a repo:
 * `rules/*.mdc`, `environment.json`, `mcp.json`, `.cursorignore` /
 * `.cursorindexingignore`, plus `skills/` `commands/` `hooks/` `worktrees/`.
 *
 * Backed by the `cursor_scan_project` / `cursor_init_project` Tauri commands
 * (`domain/compat/cursor_compat.rs`).
 */

interface CursorRuleFrontmatter {
    description?: string | null;
    globs?: string | null;
    alwaysApply?: boolean | null;
}

interface CursorRule {
    name: string;
    content: string;
    file_path: string;
    frontmatter?: CursorRuleFrontmatter | null;
}

interface CursorProjectScan {
    root: string;
    has_cursor_dir: boolean;
    rules_count: number;
    rules: CursorRule[];
    has_environment: boolean;
    has_mcp: boolean;
    mcp_server_count: number;
    has_cursorignore: boolean;
    has_cursorindexingignore: boolean;
    worktree_count: number;
    skills_count: number;
    commands_count: number;
    hooks_count: number;
}

const chipStyle = (on: boolean): React.CSSProperties => ({
    display: 'inline-flex',
    alignItems: 'center',
    gap: 4,
    fontSize: 10,
    padding: '2px 7px',
    borderRadius: 10,
    border: '1px solid rgba(255,255,255,0.12)',
    background: on ? 'rgba(76,175,80,0.18)' : 'rgba(255,255,255,0.04)',
    color: on ? '#8fd694' : 'var(--vscode-descriptionForeground, #999)',
});

const CursorProjectPanel: React.FC = () => {
    const [scan, setScan] = React.useState<CursorProjectScan | null>(null);
    const [loading, setLoading] = React.useState(true);
    const [busy, setBusy] = React.useState(false);
    const [error, setError] = React.useState<string | null>(null);

    const rescan = React.useCallback(async () => {
        setLoading(true);
        setError(null);
        try {
            const res = await invoke<CursorProjectScan>('cursor_scan_project', { root: null });
            setScan(res);
        } catch (e) {
            setError(typeof e === 'string' ? e : String(e ?? 'scan failed'));
        } finally {
            setLoading(false);
        }
    }, []);

    React.useEffect(() => { void rescan(); }, [rescan]);

    const init = async () => {
        setBusy(true);
        try {
            await invoke('cursor_init_project', { root: null });
            await rescan();
        } catch (e) {
            setError(typeof e === 'string' ? e : String(e ?? 'init failed'));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
                <div style={{ fontSize: 12, fontWeight: 600 }}>Cursor project (<code style={{ fontSize: 11 }}>.cursor/</code>)</div>
                <div style={{ display: 'flex', gap: 6 }}>
                    <button
                        onClick={() => void rescan()}
                        disabled={loading}
                        style={{ background: 'rgba(255,255,255,0.08)', border: 'none', color: 'white', padding: '3px 8px', borderRadius: 4, cursor: 'pointer', fontSize: 10 }}
                    >
                        {loading ? 'Scanning…' : 'Rescan'}
                    </button>
                    {scan && !scan.has_cursor_dir && (
                        <button
                            onClick={() => void init()}
                            disabled={busy}
                            style={{ background: 'rgba(80,160,255,0.25)', border: 'none', color: 'white', padding: '3px 8px', borderRadius: 4, cursor: 'pointer', fontSize: 10 }}
                        >
                            {busy ? 'Creating…' : 'Initialize .cursor/'}
                        </button>
                    )}
                </div>
            </div>

            <div style={{ fontSize: 11, opacity: 0.7, marginBottom: 10 }}>
                Rules, MCP servers, ignore files and worktrees Cursor keeps in the repo. The agent reads
                <code style={{ margin: '0 3px', fontSize: 10 }}>.cursor/rules/*.mdc</code>
                (with <code style={{ fontSize: 10 }}>globs</code> / <code style={{ fontSize: 10 }}>alwaysApply</code> frontmatter) as steering context.
            </div>

            {error && (
                <div style={{ fontSize: 11, color: '#ef8a8a', marginBottom: 10 }}>{error}</div>
            )}

            {scan && (
                <>
                    <div style={{ display: 'flex', flexWrap: 'wrap', gap: 6, marginBottom: 12 }}>
                        <span style={chipStyle(scan.has_cursor_dir)}>.cursor/ {scan.has_cursor_dir ? '✓' : '—'}</span>
                        <span style={chipStyle(scan.rules_count > 0)}>{scan.rules_count} rule{scan.rules_count === 1 ? '' : 's'}</span>
                        <span style={chipStyle(scan.has_mcp)}>MCP {scan.has_mcp ? `· ${scan.mcp_server_count}` : '—'}</span>
                        <span style={chipStyle(scan.has_environment)}>environment.json {scan.has_environment ? '✓' : '—'}</span>
                        <span style={chipStyle(scan.has_cursorignore)}>.cursorignore {scan.has_cursorignore ? '✓' : '—'}</span>
                        <span style={chipStyle(scan.has_cursorindexingignore)}>.cursorindexingignore {scan.has_cursorindexingignore ? '✓' : '—'}</span>
                        <span style={chipStyle(scan.skills_count > 0)}>{scan.skills_count} skill{scan.skills_count === 1 ? '' : 's'}</span>
                        <span style={chipStyle(scan.commands_count > 0)}>{scan.commands_count} command{scan.commands_count === 1 ? '' : 's'}</span>
                        <span style={chipStyle(scan.hooks_count > 0)}>{scan.hooks_count} hook{scan.hooks_count === 1 ? '' : 's'}</span>
                        <span style={chipStyle(scan.worktree_count > 0)}>{scan.worktree_count} worktree{scan.worktree_count === 1 ? '' : 's'}</span>
                    </div>

                    {scan.rules.length > 0 && (
                        <div style={{ display: 'flex', flexDirection: 'column', gap: 6 }}>
                            {scan.rules.map((r) => (
                                <div
                                    key={r.file_path}
                                    style={{ background: 'rgba(255,255,255,0.03)', border: '1px solid rgba(255,255,255,0.06)', borderRadius: 6, padding: '8px 10px' }}
                                >
                                    <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'baseline', gap: 8 }}>
                                        <span style={{ fontSize: 11, fontWeight: 600 }}>{r.name}</span>
                                        {r.frontmatter?.alwaysApply ? (
                                            <span style={{ fontSize: 9, opacity: 0.7 }}>always applied</span>
                                        ) : r.frontmatter?.globs ? (
                                            <code style={{ fontSize: 9, opacity: 0.7 }}>{r.frontmatter.globs}</code>
                                        ) : null}
                                    </div>
                                    {r.frontmatter?.description && (
                                        <div style={{ fontSize: 10, opacity: 0.65, marginTop: 3 }}>{r.frontmatter.description}</div>
                                    )}
                                </div>
                            ))}
                        </div>
                    )}

                    {scan.has_cursor_dir && scan.rules.length === 0 && (
                        <div style={{ fontSize: 11, opacity: 0.5 }}>
                            No <code style={{ fontSize: 10 }}>.cursor/rules/*.mdc</code> files yet.
                        </div>
                    )}
                </>
            )}
        </div>
    );
};

export default CursorProjectPanel;
