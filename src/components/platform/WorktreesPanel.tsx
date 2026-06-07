import React, { useCallback, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

const WorktreesPanel: React.FC<{ root: string }> = ({ root }) => {
    const setActiveRoot = useStore((s) => s.setActiveRoot);
    const [worktrees, setWorktrees] = useState<{ path: string; branch?: string }[]>([]);
    const [branch, setBranch] = useState('agent-worktree');
    const [busy, setBusy] = useState(false);
    const [msg, setMsg] = useState('');

    const refresh = useCallback(async () => {
        try {
            const rows = await invoke<{ path: string; branch: string }[]>('cursor_list_worktrees', { root });
            setWorktrees(rows ?? []);
        } catch {
            setWorktrees([]);
        }
    }, [root]);

    const onCreate = async () => {
        setBusy(true);
        setMsg('');
        try {
            const res = await invoke<{ path: string; branch: string }>('cursor_create_worktree', { root, branch });
            if (res?.path) {
                setActiveRoot(res.path);
                setMsg(`Switched to worktree: ${res.path}`);
            }
            await refresh();
        } catch (e) {
            setMsg(e instanceof Error ? e.message : String(e));
        } finally {
            setBusy(false);
        }
    };

    return (
        <div className="settings-card" style={{ maxWidth: 520 }}>
            <div style={{ fontWeight: 700, marginBottom: 8 }}>Agent git worktrees</div>
            <p className="afi-desc">Isolated branches under <code>.cursor/worktrees/</code> for safe agent edits.</p>
            <div style={{ display: 'flex', gap: 8, marginBottom: 10 }}>
                <input className="settings-input" value={branch} onChange={(e) => setBranch(e.target.value)} placeholder="branch name" />
                <button type="button" className="settings-button success" disabled={busy} onClick={() => void onCreate()}>Create & open</button>
                <button type="button" className="settings-button" disabled={busy} onClick={() => void refresh()}>Refresh</button>
            </div>
            {msg && <p className="afi-subtle" style={{ fontSize: 11 }}>{msg}</p>}
            <ul style={{ fontSize: 11, margin: 0, paddingLeft: 18 }}>
                {worktrees.map((w) => (
                    <li key={w.path}>
                        <button type="button" className="settings-button" style={{ padding: '2px 6px', fontSize: 10 }}
                            onClick={() => setActiveRoot(w.path)}>
                            Open
                        </button>
                        {' '}<code>{w.branch ?? w.path}</code>
                    </li>
                ))}
            </ul>
        </div>
    );
};

export default WorktreesPanel;
