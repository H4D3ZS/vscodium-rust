import React, { useMemo, useState } from 'react';
import {
    AGENT_FIRST_IDE_FEATURES,
    TIER_COLOR,
    TIER_LABEL,
    agentFirstIdeSummary,
} from '../../lib/agentFirstIdeParity';
import {
    CURSOR_PARITY_GROUPS,
    STATUS_COLOR,
    STATUS_LABEL,
    paritySummary,
} from '../../lib/cursorParity';

type HubTab = 'shipped' | 'roadmap';

/**
 * Single platform status view — replaces separate "Agent-First IDE" and "Cursor Parity" menus.
 * Default tab shows only what is shipped; roadmap is for internal planning.
 */
const PlatformHubPanel: React.FC = () => {
    const [tab, setTab] = useState<HubTab>('shipped');
    const agentSummary = useMemo(() => agentFirstIdeSummary(), []);
    const cursorSummary = useMemo(() => paritySummary(), []);

    const shippedAgent = useMemo(
        () => AGENT_FIRST_IDE_FEATURES.filter((f) => f.tier === 'hades-advantage' || f.tier === 'wired'),
        [],
    );
    const roadmapAgent = useMemo(
        () => AGENT_FIRST_IDE_FEATURES.filter((f) => f.tier === 'partial' || f.tier === 'integrate-next' || f.tier === 'missing'),
        [],
    );

    const shippedCursor = useMemo(
        () => CURSOR_PARITY_GROUPS.map((g) => ({
            ...g,
            features: g.features.filter((f) => f.status === 'done'),
        })).filter((g) => g.features.length > 0),
        [],
    );
    const roadmapCursor = useMemo(
        () => CURSOR_PARITY_GROUPS.map((g) => ({
            ...g,
            features: g.features.filter((f) => f.status === 'partial' || f.status === 'missing'),
        })).filter((g) => g.features.length > 0),
        [],
    );

    return (
        <div className="agent-first-ide-panel" style={{ maxWidth: 820 }}>
            <div style={{ marginBottom: 16 }}>
                <div style={{ fontWeight: 700, fontSize: 14, marginBottom: 6 }}>Platform status</div>
                <p className="afi-lead">
                    This page tracks what HADES ships today vs what is still on the roadmap.
                    It is an <strong>internal checklist</strong>, not a promise that every Cursor/Hermes feature is cloned.
                    Use <strong>Shipped</strong> to see working capabilities; open <strong>Roadmap</strong> only when planning work.
                </p>
                <div style={{ display: 'flex', gap: 10, flexWrap: 'wrap', marginBottom: 12 }}>
                    <Pill label="Cursor shipped" value={String(cursorSummary.done)} color={STATUS_COLOR.done} />
                    <Pill label="HADES edge" value={String(agentSummary['hades-advantage'])} color={TIER_COLOR['hades-advantage']} />
                    <Pill label="Agent wired" value={String(agentSummary.wired)} color={TIER_COLOR.wired} />
                    <Pill label="Roadmap items" value={String(roadmapAgent.length + cursorSummary.partial + cursorSummary.missing)} color="#79c0ff" />
                </div>
            </div>

            <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
                <TabBtn active={tab === 'shipped'} onClick={() => setTab('shipped')}>✓ Shipped ({shippedAgent.length + cursorSummary.done})</TabBtn>
                <TabBtn active={tab === 'roadmap'} onClick={() => setTab('roadmap')}>📋 Roadmap</TabBtn>
            </div>

            {tab === 'shipped' && (
                <>
                    <Section title="HADES differentiators & wired agent stack">
                        <AgentTable features={shippedAgent} />
                    </Section>
                    <Section title="Cursor-class features (shipped)">
                        <CursorGroups groups={shippedCursor} />
                    </Section>
                </>
            )}

            {tab === 'roadmap' && (
                <>
                    <p className="afi-subtle" style={{ marginBottom: 12 }}>
                        These are planned or partial — not user-facing promises. Prioritize shipping over listing.
                    </p>
                    <Section title="Agent / Hermes / Claude integrations">
                        <AgentTable features={roadmapAgent} />
                    </Section>
                    <Section title="Cursor parity gaps">
                        <CursorGroups groups={roadmapCursor} showStatus />
                    </Section>
                </>
            )}
        </div>
    );
};

function Section({ title, children }: { title: string; children: React.ReactNode }) {
    return (
        <section style={{ marginBottom: 24 }}>
            <h3 className="afi-table-head" style={{ margin: '0 0 10px' }}>{title}</h3>
            {children}
        </section>
    );
}

function AgentTable({ features }: { features: typeof AGENT_FIRST_IDE_FEATURES }) {
    if (!features.length) return <p className="afi-subtle">Nothing in this bucket.</p>;
    return (
        <table style={{ width: '100%', borderCollapse: 'collapse' }}>
            <thead>
                <tr className="afi-table-head">
                    <th style={{ padding: '6px 10px', textAlign: 'left' }}>Feature</th>
                    <th style={{ padding: '6px 10px', textAlign: 'left' }}>Status</th>
                </tr>
            </thead>
            <tbody>
                {features.map((f) => (
                    <tr key={f.id}>
                        <td style={{ padding: '8px 10px', verticalAlign: 'top' }}>
                            <div style={{ fontWeight: 600 }}>{f.name}</div>
                            <div className="afi-desc">{f.description}</div>
                        </td>
                        <td className="afi-tier" style={{ padding: '8px 10px', color: TIER_COLOR[f.tier], whiteSpace: 'nowrap' }}>
                            {TIER_LABEL[f.tier]}
                        </td>
                    </tr>
                ))}
            </tbody>
        </table>
    );
}

function CursorGroups({ groups, showStatus }: { groups: typeof CURSOR_PARITY_GROUPS; showStatus?: boolean }) {
    return groups.map((group) => (
        <div key={group.id} style={{ marginBottom: 14 }}>
            <div className="afi-subtle" style={{ marginBottom: 6, fontWeight: 600 }}>{group.title}</div>
            {group.features.map((f) => (
                <div key={f.id} style={{ padding: '8px 10px', marginBottom: 4, border: '1px solid var(--readability-panel-border)', borderRadius: 6 }}>
                    <div style={{ display: 'flex', justifyContent: 'space-between', gap: 8 }}>
                        <span style={{ fontWeight: 600 }}>{f.name}</span>
                        {showStatus && (
                            <span style={{ color: STATUS_COLOR[f.status], fontSize: 11, fontWeight: 600 }}>
                                {STATUS_LABEL[f.status]}
                            </span>
                        )}
                    </div>
                    <div className="afi-desc">{f.description}</div>
                </div>
            ))}
        </div>
    ));
}

function TabBtn({ active, onClick, children }: { active: boolean; onClick: () => void; children: React.ReactNode }) {
    return (
        <button
            type="button"
            className={`afi-filter-btn${active ? ' is-active' : ''}`}
            onClick={onClick}
            style={{ fontSize: 12, padding: '6px 14px', borderRadius: 6, cursor: 'pointer' }}
        >
            {children}
        </button>
    );
}

function Pill({ label, value, color }: { label: string; value: string; color: string }) {
    return (
        <span style={{ padding: '5px 11px', borderRadius: 6, border: `1px solid ${color}66`, background: `${color}22`, color, fontSize: 12 }}>
            {label}: <strong>{value}</strong>
        </span>
    );
}

export default PlatformHubPanel;
