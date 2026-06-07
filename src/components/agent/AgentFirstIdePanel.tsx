import React, { useCallback, useEffect, useMemo, useState } from 'react';
import {
    AGENT_FIRST_IDE_FEATURES,
    TIER_COLOR,
    TIER_LABEL,
    agentFirstIdeSummary,
    type IdeFeatureTier,
} from '../../lib/agentFirstIdeParity';
import { ideShellStatus, hermesIntegrationStatus } from '../../hermes/bridge';

const AgentFirstIdePanel: React.FC = () => {
    const summary = useMemo(() => agentFirstIdeSummary(), []);
    const [filter, setFilter] = useState<IdeFeatureTier | 'all'>('all');
    const [shell, setShell] = useState<any>(null);
    const [hermes, setHermes] = useState<any>(null);

    const refreshHermes = useCallback(async () => {
        const [s, h] = await Promise.all([ideShellStatus(), hermesIntegrationStatus()]);
        setShell(s);
        setHermes(h);
    }, []);

    useEffect(() => {
        void refreshHermes();
    }, [refreshHermes]);

    const filtered = useMemo(() => {
        if (filter === 'all') return AGENT_FIRST_IDE_FEATURES;
        return AGENT_FIRST_IDE_FEATURES.filter((f) => f.tier === filter);
    }, [filter]);

    return (
        <div className="agent-first-ide-panel" style={{ maxWidth: 820 }}>
            <div style={{ marginBottom: 16 }}>
                <div style={{ fontWeight: 700, fontSize: 14, marginBottom: 6 }}>
                    Agent-first IDE roadmap
                </div>
                <p className="afi-lead">
                    HADES combines <strong>claude-map</strong> harness, <strong>claurst</strong> query loop,
                    and <strong>Hermes skills</strong> (native SKILL.md catalog + HADES Git Bash) — plus .aim memory and shadow verify
                    that none of them ship natively.
                </p>
                <div style={{ display: 'flex', gap: 10, flexWrap: 'wrap', marginBottom: 12 }}>
                    <Pill label="HADES edge" value={String(summary['hades-advantage'])} color={TIER_COLOR['hades-advantage']} />
                    <Pill label="Wired" value={String(summary.wired)} color={TIER_COLOR.wired} />
                    <Pill label="Partial" value={String(summary.partial)} color={TIER_COLOR.partial} />
                    <Pill label="Next" value={String(summary['integrate-next'])} color={TIER_COLOR['integrate-next']} />
                    <Pill label="Cursor %" value={`${summary.cursorPct}%`} color="#79c0ff" />
                    <Pill label="Hermes %" value={`${summary.hermesPct}%`} color="#56d364" />
                </div>
            </div>

            <div className="settings-card" style={{ marginBottom: 16, padding: 12 }}>
                <div style={{ fontWeight: 600, marginBottom: 6 }}>Hermes skills (native in Sentient)</div>
                {hermes?.skillsCount != null ? (
                    <div className="afi-muted">
                        ✅ {hermes.skillsCount} SKILL.md files from <code>hermes-agent/</code> — no Python subprocess.
                        Git Bash: {shell?.ready ? shell.gitBash : 'not configured'}
                    </div>
                ) : (
                    <div className="afi-subtle">Loading…</div>
                )}
                <button type="button" className="settings-button" style={{ marginTop: 8 }} onClick={refreshHermes}>
                    Re-check Hermes
                </button>
            </div>

            <div style={{ display: 'flex', gap: 6, flexWrap: 'wrap', marginBottom: 12 }}>
                {(['all', 'hades-advantage', 'wired', 'partial', 'integrate-next', 'missing'] as const).map((t) => (
                    <button
                        key={t}
                        type="button"
                        className={`afi-filter-btn${filter === t ? ' is-active' : ''}`}
                        onClick={() => setFilter(t)}
                        style={{ fontSize: 11, padding: '4px 10px', borderRadius: 4, cursor: 'pointer' }}
                    >
                        {t === 'all' ? 'All' : TIER_LABEL[t]}
                    </button>
                ))}
            </div>

            <table style={{ width: '100%', borderCollapse: 'collapse' }}>
                <thead>
                    <tr className="afi-table-head" style={{ textAlign: 'left' }}>
                        <th style={{ padding: '6px 10px' }}>Feature</th>
                        <th style={{ padding: '6px 10px' }}>Tier</th>
                        <th style={{ padding: '6px 10px' }}>Path</th>
                    </tr>
                </thead>
                <tbody>
                    {filtered.map((f) => (
                        <tr key={f.id}>
                            <td style={{ padding: '8px 10px', verticalAlign: 'top' }}>
                                <div style={{ fontWeight: 600 }}>{f.name}</div>
                                <div className="afi-desc">{f.description}</div>
                            </td>
                            <td className="afi-tier" style={{ padding: '8px 10px', verticalAlign: 'top', color: TIER_COLOR[f.tier], whiteSpace: 'nowrap' }}>
                                {TIER_LABEL[f.tier]}
                            </td>
                            <td className="afi-path" style={{ padding: '8px 10px', verticalAlign: 'top' }}>
                                {f.hadesPath || f.sources.join(', ')}
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    );
};

function Pill({ label, value, color }: { label: string; value: string; color: string }) {
    return (
        <span style={{
            padding: '5px 11px',
            borderRadius: 6,
            border: `1px solid ${color}66`,
            background: `${color}22`,
            color,
            fontWeight: 500,
            fontSize: 12,
        }}>
            {label}: <strong>{value}</strong>
        </span>
    );
}

export default AgentFirstIdePanel;
