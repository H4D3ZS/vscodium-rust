import React, { useEffect, useMemo, useState } from 'react';
import { invoke } from '../tauri_bridge';

// ─────────────────────────────────────────────────────────────────────────────
//  KeybindingsPanel — VS Code-style keybindings editor.
//
//  Reads every registered keybinding from the backend, lets the user
//  filter by command/key/when, and edit a binding by typing a new chord
//  into a capture input. Empty key + Enter unbinds. We persist via the
//  `update_keybinding` Tauri command which mutates the in-memory
//  KeybindingRegistry; persistence to disk lives there.
// ─────────────────────────────────────────────────────────────────────────────

interface Keybinding {
    key: string;
    command: string;
    when?: string | null;
}

const KeybindingsPanel: React.FC = () => {
    const [bindings, setBindings] = useState<Keybinding[]>([]);
    const [filter, setFilter] = useState('');
    const [editing, setEditing] = useState<{ command: string; when?: string | null } | null>(null);
    const [draftKey, setDraftKey] = useState('');
    const [loading, setLoading] = useState(false);
    const [savingError, setSavingError] = useState<string | null>(null);

    const reload = async () => {
        setLoading(true);
        try {
            const res = await invoke<Keybinding[]>('list_keybindings');
            setBindings(Array.isArray(res) ? res : []);
        } catch (e) {
            console.error('list_keybindings failed', e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { reload(); }, []);

    const filtered = useMemo(() => {
        const q = filter.trim().toLowerCase();
        if (!q) return bindings;
        return bindings.filter(b =>
            b.command.toLowerCase().includes(q) ||
            b.key.toLowerCase().includes(q) ||
            (b.when || '').toLowerCase().includes(q)
        );
    }, [bindings, filter]);

    const startEdit = (b: Keybinding) => {
        setEditing({ command: b.command, when: b.when ?? null });
        setDraftKey(b.key);
        setSavingError(null);
    };

    const commit = async () => {
        if (!editing) return;
        try {
            await invoke('update_keybinding', {
                key: draftKey.trim(),
                command: editing.command,
                when: editing.when || null,
            });
            setEditing(null);
            setDraftKey('');
            await reload();
        } catch (e: any) {
            setSavingError(String(e?.message ?? e));
        }
    };

    const cancel = () => {
        setEditing(null);
        setDraftKey('');
    };

    // Capture key chords inline. We render textual input fallback in
    // case the user prefers typing. The capture is intentionally simple
    // — VS Code's keybinding spec is huge; we cover the common ground:
    // modifier prefixes joined with `+`, lowercase key names.
    const onKeyCapture = (e: React.KeyboardEvent<HTMLInputElement>) => {
        e.preventDefault();
        const parts: string[] = [];
        if (e.ctrlKey || e.metaKey) parts.push('ctrl');
        if (e.altKey) parts.push('alt');
        if (e.shiftKey) parts.push('shift');
        // Letters / digits / arrows / function keys; skip pure modifier presses.
        const k = e.key;
        if (['Control', 'Shift', 'Alt', 'Meta', 'Dead'].includes(k)) return;
        const lower = k.length === 1 ? k.toLowerCase() : k.toLowerCase();
        parts.push(lower);
        setDraftKey(parts.join('+'));
    };

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
            <div style={{ display: 'flex', gap: 8, alignItems: 'center' }}>
                <input
                    type="text"
                    placeholder="Filter by command, key, or when-clause…"
                    value={filter}
                    onChange={(e) => setFilter(e.target.value)}
                    style={input}
                />
                <button onClick={reload} style={btn} title="Reload">
                    <i className="codicon codicon-refresh" style={iconStyle} />
                </button>
            </div>

            {loading && <div style={{ fontSize: 12, opacity: 0.55 }}>Loading…</div>}

            <div style={{ border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))', borderRadius: 4, overflow: 'hidden' }}>
                <div style={{ ...row, opacity: 0.55, background: 'rgba(255,255,255,0.03)', fontSize: 10, textTransform: 'uppercase' }}>
                    <div style={{ flex: '0 0 180px' }}>Keybinding</div>
                    <div style={{ flex: 1, minWidth: 0 }}>Command</div>
                    <div style={{ flex: '0 0 220px' }}>When</div>
                    <div style={{ flex: '0 0 80px', textAlign: 'right' }}></div>
                </div>
                {filtered.map((b, i) => {
                    const isEditing = editing && editing.command === b.command && (editing.when ?? null) === (b.when ?? null);
                    return (
                        <div key={`${b.command}-${b.when || ''}-${i}`} style={row}>
                            <div style={{ flex: '0 0 180px' }}>
                                {isEditing ? (
                                    <input
                                        autoFocus
                                        value={draftKey}
                                        onChange={(e) => setDraftKey(e.target.value)}
                                        onKeyDown={onKeyCapture}
                                        placeholder="Press chord…"
                                        style={inputCompact}
                                    />
                                ) : (
                                    <code style={kbd}>{b.key}</code>
                                )}
                            </div>
                            <div style={{ flex: 1, minWidth: 0, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {b.command}
                            </div>
                            <div style={{ flex: '0 0 220px', opacity: 0.65, fontSize: 11, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {b.when || '—'}
                            </div>
                            <div style={{ flex: '0 0 80px', textAlign: 'right' }}>
                                {isEditing ? (
                                    <>
                                        <button onClick={commit} style={btnGood}><i className="codicon codicon-check" style={iconStyle} /></button>
                                        <button onClick={cancel} style={btn}><i className="codicon codicon-close" style={iconStyle} /></button>
                                    </>
                                ) : (
                                    <button onClick={() => startEdit(b)} style={btn} title="Edit binding">
                                        <i className="codicon codicon-edit" style={iconStyle} />
                                    </button>
                                )}
                            </div>
                        </div>
                    );
                })}
                {filtered.length === 0 && !loading && (
                    <div style={{ padding: 12, fontSize: 12, opacity: 0.5, textAlign: 'center' }}>
                        No keybindings match the filter.
                    </div>
                )}
            </div>

            {savingError && (
                <div style={{ fontSize: 12, color: '#f87171' }}>Save failed: {savingError}</div>
            )}
            <div style={{ fontSize: 11, opacity: 0.55 }}>
                Tip: click the pencil, then press the desired chord (Ctrl+Shift+K). Leave blank to unbind.
            </div>
        </div>
    );
};

const row: React.CSSProperties = {
    display: 'flex',
    alignItems: 'center',
    gap: 8,
    padding: '4px 10px',
    borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.04))',
    fontSize: 12,
};

const input: React.CSSProperties = {
    flex: 1,
    background: 'var(--vscode-input-background, rgba(0,0,0,0.2))',
    color: 'inherit',
    border: '1px solid rgba(255,255,255,0.1)',
    padding: '4px 8px',
    borderRadius: 3,
    fontSize: 12,
};

const inputCompact: React.CSSProperties = {
    ...input,
    flex: 'none',
    width: '100%',
    fontFamily: 'var(--font-mono, monospace)',
};

const btn: React.CSSProperties = {
    background: 'transparent',
    border: 'none',
    color: 'inherit',
    cursor: 'pointer',
    padding: 2,
    fontSize: 12,
};

const btnGood: React.CSSProperties = {
    ...btn,
    color: '#22c55e',
};

const kbd: React.CSSProperties = {
    background: 'rgba(255,255,255,0.06)',
    border: '1px solid rgba(255,255,255,0.1)',
    padding: '1px 6px',
    borderRadius: 3,
    fontFamily: 'var(--font-mono, monospace)',
    fontSize: 11,
};

const iconStyle: React.CSSProperties = {
    fontFamily: 'codicon',
    fontStyle: 'normal',
    fontSize: 12,
};

export default KeybindingsPanel;
