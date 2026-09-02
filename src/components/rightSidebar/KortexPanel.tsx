/**
 * KortexPanel — .AIM brain knowledge viewer.
 * Shows stored knowledge slots, categories, and tags.
 * Extracted from RightSidebar.tsx (A2 decomposition).
 */
import React, { useEffect, useState, useCallback } from 'react';
import { useStore } from '../../store';
import { invoke } from '../../tauri_bridge';

interface KortexSlot {
    id: string;
    category: string;
    content: string;
    timestamp: number;
    tags?: string[];
}

const KortexPanel: React.FC = () => {
    const [slots, setSlots] = useState<KortexSlot[]>([]);
    const [loading, setLoading] = useState(false);

    const refresh = useCallback(async () => {
        setLoading(true);
        try {
            const result = await invoke<KortexSlot[]>('get_all_memory_slots');
            setSlots(result || []);
        } catch (e) {
            console.error('[Kortex] Failed to load memory slots:', e);
        } finally {
            setLoading(false);
        }
    }, []);

    useEffect(() => { refresh(); }, [refresh]);

    // Category breakdown
    const categories = React.useMemo(() => {
        const cats: Record<string, number> = {};
        slots.forEach(s => { cats[s.category] = (cats[s.category] || 0) + 1; });
        return cats;
    }, [slots]);

    return (
        <div className="right-sidebar-active-surface">
            {/* Header */}
            <div style={{ padding: '16px 16px 8px', display: 'flex', alignItems: 'center', justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
                    <span style={{ fontSize: '16px' }}></span>
                    <div>
                        <div style={{ fontSize: '11px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.08em' }}>Kortex Brain</div>
                        <div style={{ fontSize: '10px', opacity: 0.4 }}>{slots.length} knowledge slots</div>
                    </div>
                </div>
                <button
                    onClick={refresh}
                    disabled={loading}
                    style={{
                        background: 'rgba(255,255,255,0.06)',
                        border: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.15))',
                        color: 'var(--vscode-textLink-foreground, #3794ff)',
                        padding: '4px 10px', borderRadius: '6px', fontSize: '10px',
                        cursor: loading ? 'not-allowed' : 'pointer', fontWeight: 600,
                    }}
                >
                    {loading ? '...' : 'Refresh'}
                </button>
            </div>

            {/* Category chips */}
            {slots.length > 0 && (
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '4px', padding: '0 16px 12px' }}>
                    {Object.entries(categories).map(([cat, count]) => (
                        <span key={cat} style={{
                            fontSize: '9px', padding: '2px 7px', borderRadius: '10px',
                            background: 'rgba(255,255,255,0.06)', border: '1px solid rgba(255,255,255,0.12)',
                            color: 'var(--vscode-focusBorder, #007acc)', fontWeight: 600,
                        }}>{cat} ({count})</span>
                    ))}
                </div>
            )}

            {/* Slot list */}
            <div style={{ flex: 1, overflowY: 'auto', padding: '0 12px 12px', display: 'flex', flexDirection: 'column', gap: '6px' }}>
                {loading ? (
                    <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                        <i className="codicon codicon-loading codicon-modifier-spin" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: '20px', display: 'block', marginBottom: '8px' }} />
                        Loading neural weights...
                    </div>
                ) : slots.length === 0 ? (
                    <div style={{ padding: '40px 20px', textAlign: 'center', opacity: 0.4, fontSize: '12px' }}>
                        <span style={{ fontSize: '32px', display: 'block', marginBottom: '8px' }}></span>
                        No knowledge stored yet.<br />
                        <span style={{ fontSize: '10px', opacity: 0.6 }}>Run a mission to populate the brain.</span>
                    </div>
                ) : (
                    slots.map((slot, i) => (
                        <div key={slot.id || i} style={{
                            background: 'var(--vscode-list-hoverBackground, rgba(255,255,255,0.04))',
                            border: '1px solid rgba(255,255,255,0.08)',
                            borderRadius: '8px', padding: '8px 12px',
                        }}>
                            <div style={{ display: 'flex', alignItems: 'center', gap: '6px', marginBottom: '4px' }}>
                                <span style={{
                                    fontSize: '8px', padding: '1px 5px', borderRadius: '8px',
                                    background: 'var(--vscode-panel-border, rgba(255,255,255,0.12))',
                                    color: 'var(--vscode-focusBorder, #007acc)', fontWeight: 700,
                                    textTransform: 'uppercase',
                                }}>{slot.category}</span>
                                <span style={{ fontSize: '9px', opacity: 0.3, marginLeft: 'auto' }}>
                                    {new Date(slot.timestamp * 1000).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                                </span>
                            </div>
                            <div style={{ fontSize: '11px', opacity: 0.8, lineHeight: 1.4, fontFamily: 'var(--font-mono)' }}>
                                {slot.content.slice(0, 140)}{slot.content.length > 140 ? '…' : ''}
                            </div>
                            {slot.tags && slot.tags.length > 0 && (
                                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '3px', marginTop: '6px' }}>
                                    {slot.tags.map((tag: string, ti: number) => (
                                        <span key={ti} style={{ fontSize: '9px', opacity: 0.4, padding: '1px 4px', background: 'rgba(255,255,255,0.05)', borderRadius: '4px' }}>#{tag}</span>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default KortexPanel;
