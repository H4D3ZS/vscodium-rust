import React, { Suspense, lazy } from 'react';
import type { AgentStudioSubView } from '../../domain/agentStudio/AgentStudioSubView';
import { AGENT_STUDIO_SUB_VIEWS } from '../../application/agentStudio/agentStudioSubViews';
import MissionControl from '../agent/MissionControl';
import AgTasksView from '../AgTasksView';
import AgSteeringView from '../AgSteeringView';
import SessionPlanPane from './SessionPlanPane';
import AgentManagerPanel from './AgentManagerPanel';

const ManusMissionPanel = lazy(() => import('./ManusMissionPanel'));
const SpecsManager = lazy(() => import('../SpecsManager'));
const RulesManager = lazy(() => import('../RulesManager'));

export interface AgentStudioPanelProps {
    activeSubView: AgentStudioSubView;
    onSubViewChange: (view: AgentStudioSubView) => void;
}

const AgentStudioPanel: React.FC<AgentStudioPanelProps> = ({ activeSubView, onSubViewChange }) => {
    return (
        <div style={{ display: 'flex', flexDirection: 'column', height: '100%', minHeight: 0 }}>
            <div style={{
                display: 'flex', alignItems: 'center', gap: 6, padding: '6px 10px',
                borderBottom: '1px solid rgba(255,255,255,0.06)',
                background: 'rgba(0,122,255,0.04)', flexShrink: 0,
            }}>
                <i className="codicon codicon-rocket" style={{ fontFamily: 'codicon', fontSize: 14, color: '#007aff' }} />
                <span style={{ fontSize: 11, fontWeight: 600, letterSpacing: '0.04em' }}>AGENT STUDIO</span>
                <span style={{ fontSize: 10, opacity: 0.45, marginLeft: 4 }}>specs · research · security</span>
            </div>
            <div style={{
                display: 'flex', gap: 2, padding: '4px 8px', overflowX: 'auto',
                borderBottom: '1px solid rgba(255,255,255,0.06)', flexShrink: 0,
            }}>
                {AGENT_STUDIO_SUB_VIEWS.map(v => (
                    <button
                        key={v.id}
                        title={v.hint}
                        onClick={() => onSubViewChange(v.id)}
                        style={{
                            border: 'none',
                            borderBottom: activeSubView === v.id ? '2px solid var(--vscode-focusBorder, #007acc)' : '2px solid transparent',
                            background: 'transparent',
                            color: activeSubView === v.id ? '#e7e7e7' : 'rgba(231,231,231,0.55)',
                            padding: '5px 8px', fontSize: 10, fontWeight: 500, cursor: 'pointer',
                            whiteSpace: 'nowrap', display: 'flex', alignItems: 'center', gap: 4,
                        }}
                    >
                        <i className={`codicon codicon-${v.icon}`} style={{ fontFamily: 'codicon', fontSize: 11 }} />
                        {v.label}
                    </button>
                ))}
            </div>
            <div style={{ flex: 1, minHeight: 0, overflow: 'hidden' }}>
                {activeSubView === 'dashboard' && <MissionControl />}
                {activeSubView === 'agents' && <AgentManagerPanel />}
                {activeSubView === 'tasks' && <AgTasksView />}
                {activeSubView === 'steering' && <AgSteeringView />}
                {activeSubView === 'planning' && <SessionPlanPane />}
                {activeSubView === 'research' && (
                    <Suspense fallback={<StudioLoading label="web agent" />}>
                        <ManusMissionPanel />
                    </Suspense>
                )}
                {activeSubView === 'specs' && (
                    <Suspense fallback={<StudioLoading label="specs" />}>
                        <SpecsManager />
                    </Suspense>
                )}
                {activeSubView === 'rules' && (
                    <Suspense fallback={<StudioLoading label="rules" />}>
                        <RulesManager />
                    </Suspense>
                )}
            </div>
        </div>
    );
};

const StudioLoading: React.FC<{ label: string }> = ({ label }) => (
    <div style={{ padding: 20, opacity: 0.5, fontSize: 11 }}>Loading {label}…</div>
);

export default AgentStudioPanel;
