import React from 'react';
import { useStore, type AgentTask, type Artifact, type AgentStep } from '../../store';
import AnePerformancePane from '../visual/AnePerformancePane';

const ArtifactCard: React.FC<{ artifact: Artifact }> = ({ artifact }) => {
    const isImage = artifact.type === 'screenshot';

    return (
        <div className="artifact-card" style={{
            background: 'rgba(255,255,255,0.03)',
            borderRadius: '8px',
            border: '1px solid rgba(255,255,255,0.05)',
            padding: '8px',
            marginBottom: '8px',
            cursor: 'pointer',
            transition: 'transform 0.2s, border-color 0.2s',
        }}
            onMouseEnter={(e) => {
                e.currentTarget.style.borderColor = 'rgba(0,122,255,0.4)';
                e.currentTarget.style.transform = 'translateY(-1px)';
            }}
            onMouseLeave={(e) => {
                e.currentTarget.style.borderColor = 'rgba(255,255,255,0.05)';
                e.currentTarget.style.transform = 'translateY(0)';
            }}
        >
            <div style={{ display: 'flex', alignItems: 'center', marginBottom: '4px' }}>
                <i className={`codicon codicon-${artifact.type === 'screenshot' ? 'device-camera' : 'file-code'}`} style={{ marginRight: '6px', fontSize: '14px', color: '#007aff' }}></i>
                <span style={{ fontSize: '11px', fontWeight: 600, opacity: 0.9 }}>{artifact.title || artifact.type.toUpperCase()}</span>
                <span style={{ marginLeft: 'auto', fontSize: '10px', opacity: 0.5 }}>{new Date(artifact.timestamp).toLocaleTimeString()}</span>
            </div>
            {isImage ? (
                <div style={{ width: '100%', height: '80px', borderRadius: '4px', overflow: 'hidden', background: '#000', position: 'relative' }}>
                    <img src={`asset://${artifact.path}`} alt="Artifact" style={{ width: '100%', height: '100%', objectFit: 'cover', opacity: 0.7 }} />
                </div>
            ) : (
                <div style={{ fontSize: '10px', opacity: 0.6, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>
                    {artifact.path.split('/').pop()}
                </div>
            )}
        </div>
    );
};

const MissionControl: React.FC = () => {
    const activeTask = useStore(state => state.agentTask);
    const tasks = useStore(state => state.agentTasks);
    const steps = useStore(state => state.agentSteps);

    return (
        <div className="mission-control" style={{ padding: '0 12px', height: '100%', overflowY: 'auto' }}>
            <div style={{ marginBottom: '20px' }}>
                <AnePerformancePane />
            </div>

            {activeTask ? (
                <div className="active-task" style={{ marginBottom: '24px' }}>
                    <div style={{ display: 'flex', alignItems: 'center', marginBottom: '8px', gap: '8px' }}>
                        <div style={{
                            width: '8px',
                            height: '8px',
                            borderRadius: '50%',
                            background: activeTask.status === 'blocked' ? '#f1c40f' : '#00ff00',
                            boxShadow: `0 0 10px ${activeTask.status === 'blocked' ? '#f1c40f' : '#00ff00'}`
                        }}></div>
                        <h3 style={{ margin: 0, fontSize: '14px', fontWeight: 600 }}>{activeTask.title}</h3>
                        {activeTask.mode && (
                            <span style={{
                                fontSize: '9px',
                                padding: '2px 6px',
                                borderRadius: '4px',
                                background: 'rgba(0,122,255,0.2)',
                                color: '#007aff',
                                fontWeight: 700,
                                textTransform: 'uppercase'
                            }}>{activeTask.mode}</span>
                        )}
                    </div>

                    <p style={{ margin: '0 0 8px 0', fontSize: '12px', opacity: 0.7, lineHeight: '1.4' }}>{activeTask.summary}</p>

                    {activeTask.task_status && (
                        <div style={{
                            fontSize: '11px',
                            fontWeight: 500,
                            color: activeTask.status === 'blocked' ? '#f1c40f' : '#3498db',
                            marginBottom: '12px',
                            display: 'flex',
                            alignItems: 'center',
                            gap: '4px'
                        }}>
                            <i className={`codicon codicon-${activeTask.status === 'blocked' ? 'warning' : 'play'}`} style={{ fontSize: '12px' }}></i>
                            {activeTask.task_status}
                        </div>
                    )}

                    <div className="progress-container" style={{ marginBottom: '16px' }}>
                        <div style={{ display: 'flex', justifyContent: 'space-between', fontSize: '10px', marginBottom: '4px', opacity: 0.6 }}>
                            <span>PROGRESS</span>
                            <span>{Math.round(activeTask.progress)}%</span>
                        </div>
                        <div style={{ height: '4px', background: 'rgba(255,255,255,0.1)', borderRadius: '2px', overflow: 'hidden' }}>
                            <div
                                style={{
                                    height: '100%',
                                    width: `${activeTask.progress}%`,
                                    background: activeTask.status === 'blocked' ? '#f1c40f' : 'linear-gradient(90deg, #007aff, #5856d6)',
                                    transition: 'width 0.5s ease'
                                }}
                            />
                        </div>
                    </div>

                    <div className="steps-section" style={{ marginBottom: '16px' }}>
                        <h4 style={{ margin: '0 0 8px 0', fontSize: '11px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Execution Steps</h4>
                        {steps.map((step, i) => (
                            <div key={i} style={{ display: 'flex', alignItems: 'flex-start', marginBottom: '6px', fontSize: '12px' }}>
                                <i className={`codicon codicon-${step.status === 'success' ? 'pass-filled' : step.status === 'error' ? 'error' : 'loading'}`}
                                    style={{
                                        marginRight: '10px',
                                        fontSize: '14px',
                                        color: step.status === 'success' ? '#2ecc71' : step.status === 'error' ? '#e74c3c' : '#3498db',
                                        marginTop: '2px'
                                    }}
                                ></i>
                                <span style={{ opacity: step.status === 'running' ? 1 : 0.7 }}>{step.name}</span>
                            </div>
                        ))}
                    </div>

                    <div className="artifacts-section">
                        <h4 style={{ margin: '0 0 8px 0', fontSize: '11px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Verifiable Artifacts</h4>
                        <div style={{ display: 'grid', gridTemplateColumns: '1fr 1fr', gap: '8px' }}>
                            {activeTask.artifacts.map((art, i) => (
                                <ArtifactCard key={art.id} artifact={art} />
                            ))}
                        </div>
                        {activeTask.artifacts.length === 0 && (
                            <div style={{ padding: '12px', border: '1px dashed rgba(255,255,255,0.1)', borderRadius: '8px', textAlign: 'center', fontSize: '11px', opacity: 0.4 }}>
                                No artifacts generated yet
                            </div>
                        )}
                    </div>
                </div>
            ) : (
                <div style={{ height: '200px', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', opacity: 0.3 }}>
                    <i className="codicon codicon-target" style={{ fontSize: '48px', marginBottom: '16px' }}></i>
                    <span style={{ fontSize: '13px' }}>No Active Mission</span>
                </div>
            )}

            {tasks.length > 0 && (
                <div className="mission-history" style={{ marginTop: '24px', borderTop: '1px solid rgba(255,255,255,0.05)', paddingTop: '16px' }}>
                    <h4 style={{ margin: '0 0 12px 0', fontSize: '11px', fontWeight: 700, opacity: 0.5, textTransform: 'uppercase', letterSpacing: '0.05em' }}>Mission History</h4>
                    {tasks.map((task) => (
                        <div key={task.id} style={{
                            padding: '10px',
                            background: 'rgba(255,255,255,0.02)',
                            borderRadius: '6px',
                            marginBottom: '8px',
                            fontSize: '12px',
                            border: '1px solid transparent'
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '4px' }}>
                                <span style={{ fontWeight: 600 }}>{task.title}</span>
                                <span style={{ opacity: 0.5, fontSize: '10px' }}>{new Date(task.createdAt).toLocaleDateString()}</span>
                            </div>
                            <div style={{ fontSize: '11px', opacity: 0.6, whiteSpace: 'nowrap', overflow: 'hidden', textOverflow: 'ellipsis' }}>
                                {task.summary}
                            </div>
                        </div>
                    ))}
                </div>
            )}
        </div>
    );
};

export default MissionControl;
