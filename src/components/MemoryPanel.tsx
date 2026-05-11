import React, { useEffect, useState } from 'react';
import { invoke } from '../tauri_bridge';

interface SemanticSlot {
    id: string;
    category: string;
    content: string;
    tags: string[];
    metadata?: any;
    timestamp: number;
}

interface BrainTelemetry {
    slot_count: number;
    message_count: number;
    entity_count: number;
    categories: Record<string, number>;
    last_slots: SemanticSlot[];
    recent_context: any[];
}

const MemoryPanel: React.FC = () => {
    const [slots, setSlots] = useState<SemanticSlot[]>([]);
    const [telemetry, setTelemetry] = useState<BrainTelemetry | null>(null);
    const [filter, setFilter] = useState<string>('');
    const [activeCategory, setActiveCategory] = useState<string>('');
    const [loading, setLoading] = useState(false);

    const refresh = async () => {
        setLoading(true);
        try {
            const [s, t] = await Promise.all([
                invoke<SemanticSlot[]>('get_all_memory_slots').catch(() => []),
                invoke<BrainTelemetry>('get_brain_telemetry').catch(() => null),
            ]);
            setSlots(s || []);
            setTelemetry(t);
        } catch (e) {
            console.error('[MemoryPanel] refresh failed', e);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => { refresh(); }, []);

    const filtered = slots
        .filter(s => !activeCategory || s.category === activeCategory)
        .filter(s => !filter || s.content.toLowerCase().includes(filter.toLowerCase()) || s.tags.some(t => t.toLowerCase().includes(filter.toLowerCase())))
        .sort((a, b) => b.timestamp - a.timestamp);

    const categories = telemetry?.categories || {};

    return (
        <div style={{ display: 'flex', flexDirection: 'column', gap: 12 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 8, justifyContent: 'space-between' }}>
                <div style={{ display: 'flex', gap: 12, fontSize: 11 }}>
                    <span title="Total memory slots persisted to .aim">
                        <strong style={{ color: '#c084fc' }}>{telemetry?.slot_count ?? 0}</strong> slots
                    </span>
                    <span title="Conversation messages in the current session">
                        <strong style={{ color: '#60a5fa' }}>{telemetry?.message_count ?? 0}</strong> msgs
                    </span>
                    <span title="Entity references">
                        <strong style={{ color: '#34d399' }}>{telemetry?.entity_count ?? 0}</strong> entities
                    </span>
                </div>
                <button
                    onClick={refresh}
                    disabled={loading}
                    style={{ background: 'transparent', border: '1px solid rgba(255,255,255,0.15)', color: '#fff', padding: '3px 10px', fontSize: 11, borderRadius: 4, cursor: loading ? 'wait' : 'pointer' }}
                    title="Reload memory from .aim"
                >
                    {loading ? '…' : 'Reload'}
                </button>
            </div>

            <input
                value={filter}
                onChange={e => setFilter(e.target.value)}
                placeholder="Filter slots by content or tag…"
                style={{ background: 'rgba(0,0,0,0.25)', border: '1px solid rgba(255,255,255,0.1)', color: '#fff', padding: '4px 8px', fontSize: 11, borderRadius: 4 }}
            />

            {Object.keys(categories).length > 0 && (
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4 }}>
                    <button
                        onClick={() => setActiveCategory('')}
                        style={{
                            background: !activeCategory ? '#c084fc' : 'transparent',
                            color: !activeCategory ? '#000' : '#fff',
                            border: '1px solid rgba(255,255,255,0.15)',
                            padding: '2px 8px', fontSize: 10, borderRadius: 12, cursor: 'pointer',
                        }}
                    >
                        all ({telemetry?.slot_count ?? 0})
                    </button>
                    {Object.entries(categories)
                        .sort((a, b) => b[1] - a[1])
                        .map(([cat, count]) => (
                            <button
                                key={cat}
                                onClick={() => setActiveCategory(cat === activeCategory ? '' : cat)}
                                style={{
                                    background: cat === activeCategory ? '#c084fc' : 'transparent',
                                    color: cat === activeCategory ? '#000' : '#fff',
                                    border: '1px solid rgba(255,255,255,0.15)',
                                    padding: '2px 8px', fontSize: 10, borderRadius: 12, cursor: 'pointer',
                                }}
                            >
                                {cat} ({count})
                            </button>
                        ))}
                </div>
            )}

            <div style={{ display: 'flex', flexDirection: 'column', gap: 6, maxHeight: 480, overflowY: 'auto' }}>
                {filtered.length === 0 && (
                    <div style={{ fontSize: 11, opacity: 0.5, fontStyle: 'italic', padding: 12, textAlign: 'center' }}>
                        {slots.length === 0
                            ? 'Memory is empty. Slots are written automatically as the agent learns from your turns.'
                            : 'No slots match the current filter.'}
                    </div>
                )}
                {filtered.slice(0, 200).map(slot => (
                    <div key={slot.id} style={{
                        padding: '6px 10px', background: 'rgba(255,255,255,0.04)',
                        border: '1px solid rgba(255,255,255,0.08)', borderRadius: 4,
                    }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 4 }}>
                            <span style={{ fontSize: 10, fontWeight: 600, color: '#c084fc', textTransform: 'uppercase', letterSpacing: 0.4 }}>
                                {slot.category}
                            </span>
                            <span style={{ fontSize: 9, opacity: 0.4 }}>
                                {new Date(slot.timestamp * 1000).toLocaleString()}
                            </span>
                        </div>
                        <div style={{ fontSize: 11, opacity: 0.9, whiteSpace: 'pre-wrap', wordBreak: 'break-word' }}>
                            {slot.content.length > 320 ? `${slot.content.slice(0, 320)}…` : slot.content}
                        </div>
                        {slot.tags.length > 0 && (
                            <div style={{ display: 'flex', flexWrap: 'wrap', gap: 4, marginTop: 4 }}>
                                {slot.tags.slice(0, 6).map((t, i) => (
                                    <span key={i} style={{ fontSize: 9, padding: '1px 6px', background: 'rgba(96,165,250,0.15)', color: '#60a5fa', borderRadius: 8 }}>
                                        {t}
                                    </span>
                                ))}
                            </div>
                        )}
                    </div>
                ))}
                {filtered.length > 200 && (
                    <div style={{ fontSize: 10, opacity: 0.5, fontStyle: 'italic', textAlign: 'center', padding: 6 }}>
                        Showing 200 of {filtered.length} matching slots.
                    </div>
                )}
            </div>
        </div>
    );
};

export default MemoryPanel;
