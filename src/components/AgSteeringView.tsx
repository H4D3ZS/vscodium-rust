import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useStore } from '../store';

interface SteeringFile {
    name: string;
    path: string;
    preview: string;
}

interface HookDef {
    id: string;
    name: string;
    trigger: string;
    enabled: boolean;
}

const HOOKS_STORAGE_KEY = 'hades.hooks';

const AgSteeringView: React.FC = () => {
    const [files, setFiles] = useState<SteeringFile[]>([]);
    const [hooks, setHooks] = useState<HookDef[]>([]);
    const [loading, setLoading] = useState(false);
    const [tab, setTab] = useState<'steering' | 'hooks'>('steering');
    const activeRoot = useStore(s => s.activeRoot);

    const loadSteering = async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const dir = `${activeRoot}/.agent/steering`;
            await invoke('create_directory', { path: dir }).catch(() => null);
            const entries = await invoke<any[]>('list_directory', { path: dir }).catch(() => []);
            const md = (entries || []).filter((e: any) => !e.is_dir && e.name.endsWith('.md'));
            const loaded: SteeringFile[] = await Promise.all(
                md.map(async (e: any) => {
                    const content = await invoke<string>('read_file', { path: e.path }).catch(() => '');
                    return {
                        name: e.name.replace('.md', ''),
                        path: e.path,
                        preview: content.slice(0, 120).replace(/\n+/g, ' '),
                    };
                })
            );
            setFiles(loaded);
        } finally {
            setLoading(false);
        }
    };

    const loadHooks = () => {
        try {
            const stored = localStorage.getItem(HOOKS_STORAGE_KEY);
            if (stored) setHooks(JSON.parse(stored));
        } catch { /* corrupted localStorage */ }
    };

    useEffect(() => { loadSteering(); loadHooks(); }, [activeRoot]);

    const openSettings = (section: string) => {
        const store = (window as any).useStore?.getState();
        if (store?.openSettings) {
            store.openSettings();
            setTimeout(() => {
                const event = new CustomEvent('settings-navigate', { detail: section });
                window.dispatchEvent(event);
            }, 150);
        }
    };

    const openFile = async (path: string) => {
        const store = (window as any).useStore?.getState();
        if (store?.openFile) {
            try {
                const content = await invoke<string>('read_file', { path });
                store.openFile(path, content, 'markdown');
    } catch { /* non-fatal */ }
        }
    };

    const TRIGGER_LABEL: Record<string, string> = {
        on_save: 'Save',
        on_commit: 'Commit',
        on_file_create: 'Create',
        on_test_fail: 'Test Fail',
        manual: 'Manual',
    };

    if (!activeRoot) {
        return (
            <div style={{ padding: '20px', textAlign: 'center', opacity: 0.5, fontSize: '12px' }}>
                Open a folder to view steering
            </div>
        );
    }

    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', overflow: 'hidden' }}>
            {/* Tab bar */}
            <div style={{ display: 'flex', borderBottom: '1px solid rgba(255,255,255,0.06)', padding: '0 8px' }}>
                {(['steering', 'hooks'] as const).map(t => (
                    <div
                        key={t}
                        onClick={() => setTab(t)}
                        style={{
                            padding: '6px 10px', fontSize: 11, cursor: 'pointer',
                            borderBottom: `2px solid ${tab === t ? '#4f8ef7' : 'transparent'}`,
                            color: tab === t ? '#4f8ef7' : 'rgba(255,255,255,0.5)',
                            textTransform: 'uppercase', letterSpacing: '0.5px', fontWeight: 500,
                        }}
                    >
                        {t === 'steering' ? `Steering (${files.length})` : `Hooks (${hooks.filter(h => h.enabled).length}/${hooks.length})`}
                    </div>
                ))}
                <button
                    onClick={() => tab === 'steering' ? loadSteering() : loadHooks()}
                    style={{ background: 'none', border: 'none', color: 'rgba(255,255,255,0.35)', cursor: 'pointer', marginLeft: 'auto', padding: '4px 6px', fontSize: 12 }}
                    title="Refresh"
                >
                    <i className={`codicon codicon-${loading ? 'loading~spin' : 'refresh'}`} />
                </button>
            </div>

            <div style={{ flex: 1, overflowY: 'auto', padding: '6px 0' }}>
                {tab === 'steering' ? (
                    <>
                        {files.length === 0 ? (
                            <div style={{ padding: '16px', textAlign: 'center', opacity: 0.4, fontSize: '11px' }}>
                                No steering files in .agent/steering/
                            </div>
                        ) : (
                            files.map(f => (
                                <div
                                    key={f.path}
                                    onClick={() => openFile(f.path)}
                                    style={{ padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.04)', cursor: 'pointer' }}
                                    className="sidebar-hover-row"
                                >
                                    <div style={{ display: 'flex', alignItems: 'center', gap: 6 }}>
                                        <i className="codicon codicon-symbol-file" style={{ color: '#c084fc', fontSize: 12 }} />
                                        <span style={{ fontSize: 12, fontWeight: 600 }}>{f.name}</span>
                                        <span style={{ fontSize: 9, opacity: 0.35, marginLeft: 'auto' }}>always on</span>
                                    </div>
                                    <div style={{ fontSize: 10, opacity: 0.45, marginTop: 3, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                        {f.preview}
                                    </div>
                                </div>
                            ))
                        )}
                    </>
                ) : (
                    <>
                        {hooks.length === 0 ? (
                            <div style={{ padding: '16px', textAlign: 'center', opacity: 0.4, fontSize: '11px' }}>
                                No hooks configured yet
                            </div>
                        ) : (
                            hooks.map(h => (
                                <div
                                    key={h.id}
                                    style={{ padding: '8px 12px', borderBottom: '1px solid rgba(255,255,255,0.04)', display: 'flex', alignItems: 'center', gap: 8, opacity: h.enabled ? 1 : 0.4 }}
                                >
                                    <div style={{ width: 6, height: 6, borderRadius: '50%', background: h.enabled ? '#3fb950' : 'rgba(255,255,255,0.2)', flexShrink: 0 }} />
                                    <div style={{ flex: 1, minWidth: 0 }}>
                                        <div style={{ fontSize: 12, fontWeight: 500, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{h.name}</div>
                                        <div style={{ fontSize: 10, opacity: 0.45, marginTop: 1 }}>
                                            {TRIGGER_LABEL[h.trigger] || h.trigger}
                                        </div>
                                    </div>
                                </div>
                            ))
                        )}
                    </>
                )}
            </div>

            {/* Footer */}
            <div style={{ padding: '8px 12px', borderTop: '1px solid rgba(255,255,255,0.06)' }}>
                <button
                    style={{ width: '100%', fontSize: 11, padding: '5px', background: 'rgba(255,255,255,0.04)', border: '1px dashed rgba(255,255,255,0.12)', borderRadius: 5, color: 'rgba(255,255,255,0.5)', cursor: 'pointer' }}
                    onClick={() => openSettings(tab === 'steering' ? 'steering' : 'hooks')}
                >
                    {tab === 'steering' ? 'Manage Steering Files →' : 'Manage Hooks →'}
                </button>
            </div>
        </div>
    );
};

export default AgSteeringView;
