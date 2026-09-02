import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../store';
import {
    agDispatchLifecycleHooks,
    agLoadLifecycleHooks,
    agSaveLifecycleHooks,
} from '../infrastructure/antigravity/antigravityClient';

const EVENTS = ['PreToolUse', 'PostToolUse', 'PreInvocation', 'PostInvocation', 'Stop'] as const;

const AntigravityHooksPanel: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const [raw, setRaw] = useState('{}');
    const [error, setError] = useState<string | null>(null);
    const [saved, setSaved] = useState(false);

    const load = useCallback(async () => {
        if (!activeRoot) return;
        try {
            const hooks = await agLoadLifecycleHooks(activeRoot);
            setRaw(JSON.stringify(hooks, null, 2));
            setError(null);
        } catch (e: any) {
            setError(e?.message || String(e));
        }
    }, [activeRoot]);

    useEffect(() => { void load(); }, [load]);

    const save = async () => {
        if (!activeRoot) return;
        try {
            const parsed = JSON.parse(raw);
            await agSaveLifecycleHooks(activeRoot, parsed);
            setError(null);
            setSaved(true);
            setTimeout(() => setSaved(false), 2000);
        } catch (e: any) {
            setError(e?.message || 'Invalid JSON');
        }
    };

    const testDispatch = async (event: string) => {
        if (!activeRoot) return;
        try {
            const results = await agDispatchLifecycleHooks(activeRoot, event, '*');
            setError(null);
            alert(`Dispatched ${event}: ${results.length} hook(s) ran`);
        } catch (e: any) {
            setError(e?.message || String(e));
        }
    };

    return (
        <div style={{ maxWidth: 820 }}>
            <div className="settings-section-title" style={{ display: 'flex', alignItems: 'center', gap: 10 }}>
                Lifecycle hooks
                <span className="settings-badge new">Antigravity</span>
            </div>
            <p className="settings-section-subtitle">
                Project file <code>.agent/hooks.json</code> — PreToolUse, PostToolUse, PreInvocation, PostInvocation, Stop.
                Matchers support <code>*</code>, tool names, and <code>browser_.*</code> patterns.
            </p>
            {!activeRoot && (
                <div style={{ fontSize: 12, opacity: 0.5, marginBottom: 12 }}>Open a workspace folder to edit hooks.</div>
            )}
            {error && <div style={{ color: '#f87171', fontSize: 12, marginBottom: 8 }}>{error}</div>}
            <textarea
                className="settings-input"
                value={raw}
                onChange={e => setRaw(e.target.value)}
                spellCheck={false}
                style={{
                    width: '100%',
                    minHeight: 320,
                    fontFamily: 'monospace',
                    fontSize: 11,
                    boxSizing: 'border-box',
                }}
            />
            <div style={{ display: 'flex', gap: 8, marginTop: 12, flexWrap: 'wrap', alignItems: 'center' }}>
                <button className="settings-button" onClick={() => void save()} disabled={!activeRoot}>
                    {saved ? 'Saved ✓' : 'Save hooks.json'}
                </button>
                <button className="settings-button" onClick={() => void load()} disabled={!activeRoot} style={{ opacity: 0.7 }}>
                    Reload
                </button>
                {EVENTS.map(ev => (
                    <button
                        key={ev}
                        className="settings-button"
                        style={{ fontSize: 10, opacity: 0.75 }}
                        onClick={() => void testDispatch(ev)}
                        disabled={!activeRoot}
                    >
                        Test {ev}
                    </button>
                ))}
            </div>
        </div>
    );
};

export default AntigravityHooksPanel;
