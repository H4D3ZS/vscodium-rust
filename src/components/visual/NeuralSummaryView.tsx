import React, { useEffect } from 'react';
import { useStore } from '../../store';
import { motion } from 'framer-motion';
import { Brain, MessageSquare, Database, List, Clock, Activity } from 'lucide-react';

const NeuralSummaryView: React.FC = () => {
    const { brainTelemetry: data, refreshBrainTelemetry } = useStore();

    useEffect(() => {
        refreshBrainTelemetry();
        const interval = setInterval(refreshBrainTelemetry, 15000); // Reduce CPU pressure
        return () => clearInterval(interval);
    }, [refreshBrainTelemetry]);

    if (!data) {
        return (
            <div style={{ padding: '40px', textAlign: 'center', opacity: 0.5 }}>
                <Activity className="animate-pulse" size={48} style={{ margin: '0 auto 16px' }} />
                <div style={{ fontSize: '14px' }}>Synchronizing Neural Context...</div>
            </div>
        );
    }

    return (
        <div style={{ padding: '24px', display: 'flex', flexDirection: 'column', gap: '24px', overflowY: 'auto', height: '100%', color: '#fff' }}>
            <div style={{ display: 'flex', gap: '16px' }}>
                <StatCard icon={<Database size={16} color="#3b82f6" />} label="Slots" value={data.slot_count} />
                <StatCard icon={<MessageSquare size={16} color="#10b981" />} label="Messages" value={data.message_count} />
                <StatCard icon={<Brain size={16} color="#a855f7" />} label="Entities" value={data.entity_count} />
            </div>

            <section>
                <SectionHeader icon={<Clock size={14} />} title="Recent Memory Slots" />
                <div style={{ display: 'flex', flexDirection: 'column', gap: '10px', marginTop: '12px' }}>
                    {data.last_slots.map((slot: any) => (
                        <div key={slot.id} style={{
                            padding: '12px', borderRadius: '8px', background: 'rgba(255,255,255,0.03)',
                            border: '1px solid rgba(255,255,255,0.05)', fontSize: '12px'
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '6px' }}>
                                <span style={{ fontWeight: 700, color: '#3b82f6', textTransform: 'uppercase', fontSize: '10px', letterSpacing: '0.05em' }}>{slot.category}</span>
                                <span style={{ opacity: 0.4, fontSize: '10px' }}>{new Date(slot.timestamp * 1000).toLocaleTimeString()}</span>
                            </div>
                            <div style={{ opacity: 0.8, lineHeight: '1.5' }}>{slot.content.length > 200 ? slot.content.substring(0, 200) + '...' : slot.content}</div>
                            {slot.tags.length > 0 && (
                                <div style={{ display: 'flex', gap: '6px', marginTop: '8px', flexWrap: 'wrap' }}>
                                    {slot.tags.map((tag: string) => (
                                        <span key={tag} style={{ padding: '2px 6px', background: 'rgba(59, 130, 246, 0.1)', color: '#3b82f6', borderRadius: '4px', fontSize: '9px' }}>#{tag}</span>
                                    ))}
                                </div>
                            )}
                        </div>
                    ))}
                </div>
            </section>

            <section>
                <SectionHeader icon={<List size={14} />} title="Knowledge Categories" />
                <div style={{ display: 'flex', flexWrap: 'wrap', gap: '8px', marginTop: '12px' }}>
                    {Object.entries(data.categories).map(([cat, count]: [string, any]) => (
                        <div key={cat} style={{
                            padding: '8px 12px', background: 'rgba(59, 130, 246, 0.05)',
                            border: '1px solid rgba(59, 130, 246, 0.1)', borderRadius: '12px',
                            display: 'flex', alignItems: 'center', gap: '8px'
                        }}>
                            <span style={{ fontSize: '11px', fontWeight: 600 }}>{cat.toUpperCase()}</span>
                            <span style={{ fontSize: '10px', opacity: 0.5, padding: '1px 5px', background: 'rgba(0,0,0,0.2)', borderRadius: '10px' }}>{count}</span>
                        </div>
                    ))}
                </div>
            </section>

            <section style={{ marginBottom: '24px' }}>
                <SectionHeader icon={<Activity size={14} />} title="Context Horizon" />
                <div style={{ marginTop: '12px', opacity: 0.6, fontSize: '11px', fontStyle: 'italic' }}>
                    Tracking the last {data.recent_context.length} interaction patterns for optimized reasoning depth.
                </div>
            </section>
        </div>
    );
};

const StatCard = ({ icon, label, value }: any) => (
    <div style={{
        flex: 1, padding: '16px', borderRadius: '12px', background: 'rgba(255,255,255,0.02)',
        border: '1px solid rgba(255,255,255,0.05)', display: 'flex', flexDirection: 'column', gap: '8px'
    }}>
        <div style={{ display: 'flex', alignItems: 'center', gap: '8px' }}>
            {icon}
            <span style={{ fontSize: '11px', opacity: 0.5, fontWeight: 600, textTransform: 'uppercase' }}>{label}</span>
        </div>
        <div style={{ fontSize: '24px', fontWeight: 700 }}>{value}</div>
    </div>
);

const SectionHeader = ({ icon, title }: any) => (
    <div style={{ display: 'flex', alignItems: 'center', gap: '10px', borderBottom: '1px solid rgba(255,255,255,0.05)', paddingBottom: '8px' }}>
        <span style={{ opacity: 0.5 }}>{icon}</span>
        <h3 style={{ margin: 0, fontSize: '13px', fontWeight: 700, textTransform: 'uppercase', letterSpacing: '0.05em' }}>{title}</h3>
    </div>
);

export default NeuralSummaryView;
