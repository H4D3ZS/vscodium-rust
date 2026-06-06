import React, { useCallback, useEffect, useState } from 'react';
import { useStore } from '../../store';
import RunningItemsPanel from './RunningItemsPanel';
import SubagentViewer from './SubagentViewer';
import AnePerformancePane from '../visual/AnePerformancePane';
import {
    agBrainList,
    agGetTrajectory,
    agListTrajectories,
    type BrainArtifactInfo,
    type TrajectoryRecord,
    type TrajectoryStep,
} from '../../infrastructure/antigravity/antigravityClient';
import type { TaskArtifact } from '../../store/types';

const ArtifactCard: React.FC<{ artifact: TaskArtifact | BrainArtifactInfo; isMedia?: boolean }> = ({ artifact, isMedia }) => {
    const path = 'path' in artifact ? artifact.path : '';
    const title = 'title' in artifact ? (artifact.title || artifact.type) : artifact.name;
    const ts = 'timestamp' in artifact ? artifact.timestamp : Date.parse(artifact.updated_at) || Date.now();

    return (
        <div className="artifact-card" style={{
            background: 'rgba(255,255,255,0.03)',
            borderRadius: 8,
            border: '1px solid rgba(255,255,255,0.05)',
            padding: 8,
            marginBottom: 8,
        }}>
            <div style={{ display: 'flex', alignItems: 'center', marginBottom: 4 }}>
                <i className={`codicon codicon-${isMedia ? 'device-camera' : 'file-code'}`} style={{ marginRight: 6, fontSize: 14, color: '#007aff' }} />
                <span style={{ fontSize: 11, fontWeight: 600, opacity: 0.9 }}>{title}</span>
                <span style={{ marginLeft: 'auto', fontSize: 10, opacity: 0.5 }}>{new Date(ts).toLocaleTimeString()}</span>
            </div>
            {isMedia ? (
                <div style={{ width: '100%', height: 80, borderRadius: 4, overflow: 'hidden', background: '#000' }}>
                    <img src={`asset://${path}`} alt="" style={{ width: '100%', height: '100%', objectFit: 'cover', opacity: 0.85 }} />
                </div>
            ) : (
                <div style={{ fontSize: 10, opacity: 0.6, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {path.split(/[/\\]/).pop()}
                </div>
            )}
        </div>
    );
};

const MissionControl: React.FC = () => {
    const activeRoot = useStore(s => s.activeRoot);
    const activeCascadeId = useStore(s => s.activeCascadeId);
    const activeTask = useStore(s => s.agentTask);
    const steps = useStore(s => s.agentSteps);
    const trajectoryEvents = useStore(s => s.agentTrajectory);

    const [trajectories, setTrajectories] = useState<TrajectoryRecord[]>([]);
    const [trajectory, setTrajectory] = useState<TrajectoryRecord | null>(null);
    const [brainArtifacts, setBrainArtifacts] = useState<BrainArtifactInfo[]>([]);
    const [selectedSubagentId, setSelectedSubagentId] = useState<string | null>(null);
    const [loading, setLoading] = useState(false);

    const cascadeId = activeCascadeId || trajectories[0]?.id;

    const refresh = useCallback(async () => {
        if (!activeRoot) return;
        setLoading(true);
        try {
            const [runs, brain] = await Promise.all([
                agListTrajectories(activeRoot),
                cascadeId ? agBrainList(activeRoot, cascadeId).catch(() => []) : Promise.resolve([]),
            ]);
            setTrajectories(runs);
            setBrainArtifacts(brain);
            if (cascadeId) {
                const t = await agGetTrajectory(activeRoot, cascadeId);
                setTrajectory(t);
            }
        } finally {
            setLoading(false);
        }
    }, [activeRoot, cascadeId]);

    useEffect(() => { void refresh(); }, [refresh]);
    useEffect(() => {
        const t = setInterval(() => void refresh(), 8000);
        return () => clearInterval(t);
    }, [refresh]);

    const trajSteps: TrajectoryStep[] = trajectory?.steps?.length
        ? trajectory.steps
        : (trajectoryEvents || []).map((e, i) => ({
            id: `live-${i}`,
            kind: e.kind,
            title: e.title,
            detail: e.detail,
            tool: e.tool,
            timestamp: Date.now(),
            success: e.success,
        }));

    const subagents = trajectory?.subagents || [];
    const selectedSub = subagents.find(s => s.id === selectedSubagentId) || null;
    const mediaArtifacts = brainArtifacts.filter(a => a.is_media);
    const docArtifacts = brainArtifacts.filter(a => !a.is_media);

    return (
        <div className="mission-control" style={{ padding: '0 12px', height: '100%', overflowY: 'auto' }}>
            <div style={{ marginBottom: 16 }}>
                <AnePerformancePane />
            </div>

            <Section title={`Running items${loading ? ' · …' : ''}`}>
                <RunningItemsPanel
                    subagents={subagents}
                    onSelectSubagent={setSelectedSubagentId}
                />
            </Section>

            {selectedSub && (
                <SubagentViewer
                    subagent={selectedSub}
                    steps={trajSteps}
                    onClose={() => setSelectedSubagentId(null)}
                />
            )}

            {(activeTask || trajectory) && (
                <Section title="Active mission">
                    <div style={{ marginBottom: 12 }}>
                        <div style={{ display: 'flex', alignItems: 'center', gap: 8, marginBottom: 6 }}>
                            <div style={{
                                width: 8, height: 8, borderRadius: '50%',
                                background: '#00ff00', boxShadow: '0 0 10px #00ff00',
                            }} />
                            <h3 style={{ margin: 0, fontSize: 14, fontWeight: 600 }}>
                                {activeTask?.title || trajectory?.objective || 'Agent mission'}
                            </h3>
                        </div>
                        <p style={{ margin: 0, fontSize: 12, opacity: 0.65 }}>
                            {activeTask?.summary || trajectory?.summary || 'Autonomous execution in progress'}
                        </p>
                        {cascadeId && (
                            <div style={{ fontSize: 10, opacity: 0.4, marginTop: 4, fontFamily: 'monospace' }}>
                                cascade: {cascadeId.slice(0, 24)}…
                            </div>
                        )}
                    </div>

                    <div style={{ marginBottom: 12 }}>
                        <h4 style={sectionLabel}>Execution steps</h4>
                        {(trajSteps.length ? trajSteps.slice(-12) : steps).map((step: any, i: number) => (
                            <div key={step.id || i} style={{ display: 'flex', alignItems: 'flex-start', marginBottom: 6, fontSize: 12 }}>
                                <i className={`codicon codicon-${step.success === false || step.status === 'error' ? 'error' : step.status === 'running' ? 'loading' : 'pass-filled'}`}
                                    style={{ marginRight: 10, fontSize: 14, color: step.success === false ? '#e74c3c' : '#2ecc71', marginTop: 2 }}
                                />
                                <span>{step.title || step.name}</span>
                            </div>
                        ))}
                    </div>
                </Section>
            )}

            <Section title="Verifiable artifacts (.agent/brain/)">
                {mediaArtifacts.length > 0 && (
                    <>
                        <h4 style={sectionLabel}>Visual verification</h4>
                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8, marginBottom: 12 }}>
                            {mediaArtifacts.slice(-4).map(a => (
                                <ArtifactCard key={a.path} artifact={a} isMedia />
                            ))}
                        </div>
                    </>
                )}
                {docArtifacts.length > 0 ? (
                    <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
                        {docArtifacts.map(a => (
                            <ArtifactCard key={a.path} artifact={a} />
                        ))}
                    </div>
                ) : activeTask?.artifacts?.length ? (
                    <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: 8 }}>
                        {activeTask.artifacts.map(art => (
                            <ArtifactCard key={art.id} artifact={art} isMedia={art.type === 'screenshot'} />
                        ))}
                    </div>
                ) : (
                    <div style={{ padding: 12, border: '1px dashed rgba(255,255,255,0.1)', borderRadius: 8, textAlign: 'center', fontSize: 11, opacity: 0.4 }}>
                        Screenshots, task.md, walkthrough.md appear here during agent runs
                    </div>
                )}
            </Section>

            {trajectories.length > 0 && (
                <Section title="Mission history (.agent/runs/)">
                    {trajectories.slice(0, 8).map(run => (
                        <div key={run.id} style={{
                            padding: 10,
                            background: run.id === cascadeId ? 'rgba(0,122,255,0.08)' : 'rgba(255,255,255,0.02)',
                            borderRadius: 6,
                            marginBottom: 8,
                            fontSize: 12,
                            border: `1px solid ${run.id === cascadeId ? 'rgba(0,122,255,0.2)' : 'transparent'}`,
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 4 }}>
                                <span style={{ fontWeight: 600 }}>{run.status}</span>
                                <span style={{ opacity: 0.5, fontSize: 10 }}>
                                    {run.steps?.length ?? 0} steps · {run.subagents?.length ?? 0} subagents
                                </span>
                            </div>
                            <div style={{ fontSize: 11, opacity: 0.65, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                                {run.objective}
                            </div>
                        </div>
                    ))}
                </Section>
            )}

            {!activeTask && !trajectory && trajectories.length === 0 && (
                <div style={{ height: 120, display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', opacity: 0.3 }}>
                    <i className="codicon codicon-target" style={{ fontSize: 40, marginBottom: 12 }} />
                    <span style={{ fontSize: 13 }}>No active mission — send a prompt or run /next</span>
                </div>
            )}
        </div>
    );
};

const sectionLabel: React.CSSProperties = {
    margin: '0 0 8px 0',
    fontSize: 11,
    fontWeight: 700,
    opacity: 0.5,
    textTransform: 'uppercase',
    letterSpacing: '0.05em',
};

const Section: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ marginBottom: 20 }}>
        <div style={{ fontSize: 11, fontWeight: 700, opacity: 0.55, marginBottom: 8, textTransform: 'uppercase', letterSpacing: '0.06em' }}>
            {title}
        </div>
        {children}
    </div>
);

export default MissionControl;
