import React, { useEffect, useState } from 'react';
import { useStore } from '../../store';
import { loadWorkspaceSettings, saveWorkspaceSettings } from '../../application/workspace/workspaceSettings';

const WorkspaceSettingsPanel: React.FC = () => {
    const activeRoot = useStore((s) => s.activeRoot);
    const [json, setJson] = useState('{}');
    const [error, setError] = useState('');
    const [saved, setSaved] = useState(false);

    useEffect(() => {
        if (!activeRoot) return;
        setError('');
        loadWorkspaceSettings()
            .then((s) => setJson(JSON.stringify(s, null, 2)))
            .catch((e) => setError(String(e)));
    }, [activeRoot]);

    const onSave = async () => {
        setError('');
        setSaved(false);
        try {
            const parsed = JSON.parse(json);
            await saveWorkspaceSettings(parsed);
            setSaved(true);
        } catch (e) {
            setError(e instanceof Error ? e.message : String(e));
        }
    };

    if (!activeRoot) {
        return (
            <div style={{ padding: 16, opacity: 0.7, fontSize: 12 }}>
                Open a folder to edit workspace settings (`.vscode/settings.json`).
            </div>
        );
    }

    return (
        <div style={{ maxWidth: 720, display: 'flex', flexDirection: 'column', gap: 12 }}>
            <div style={{ fontSize: 13, fontWeight: 600 }}>Workspace Settings</div>
            <div style={{ fontSize: 11, opacity: 0.6 }}>
                {activeRoot}/.vscode/settings.json
            </div>
            <textarea
                value={json}
                onChange={(e) => setJson(e.target.value)}
                spellCheck={false}
                style={{
                    width: '100%',
                    minHeight: 320,
                    fontFamily: 'Consolas, monospace',
                    fontSize: 12,
                    background: 'var(--vscode-editor-background)',
                    color: 'var(--vscode-editor-foreground)',
                    border: '1px solid var(--vscode-panel-border)',
                    padding: 8,
                    resize: 'vertical',
                }}
            />
            {error && <div style={{ color: '#f7768e', fontSize: 11 }}>{error}</div>}
            {saved && <div style={{ color: '#9ece6a', fontSize: 11 }}>Saved.</div>}
            <button
                type="button"
                onClick={() => void onSave()}
                style={{
                    alignSelf: 'flex-start',
                    padding: '4px 14px',
                    fontSize: 12,
                    background: 'var(--vscode-button-background)',
                    color: 'var(--vscode-button-foreground)',
                    border: 'none',
                    cursor: 'pointer',
                }}
            >
                Save Workspace Settings
            </button>
        </div>
    );
};

export default WorkspaceSettingsPanel;
