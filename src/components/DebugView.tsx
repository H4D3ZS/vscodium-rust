import React from 'react';
import { useStore } from '../store';
import RunConfigsPanel from './RunConfigsPanel';
import {
    debugContinue,
    debugStepOver,
    debugStepInto,
    debugStepOut,
} from '../application/debug/bootstrapDebugRuntime';
import { stopDebugSession } from '../application/debug/sendDapRequest';

const DebugView: React.FC = () => {
    const isDebugging = useStore((s) => s.isDebugging);
    const sessionName = useStore((s) => s.debugSessionName);
    const threads = useStore((s) => s.debugThreads);
    const stackFrames = useStore((s) => s.debugStackFrames);
    const variables = useStore((s) => s.debugVariables);
    const breakpoints = useStore((s) => s.debugBreakpoints);
    const debugWatch = useStore((s) => s.debugWatch);
    const addDebugWatch = useStore((s) => s.addDebugWatch);
    const removeDebugWatch = useStore((s) => s.removeDebugWatch);
    const output = useStore((s) => s.debugOutput);
    const setDebugging = useStore((s) => s.setDebugging);
    const [watchExpr, setWatchExpr] = React.useState('');

    const handleStop = async () => {
        try {
            await stopDebugSession();
        } catch (e) {
            console.error('debug_stop failed', e);
        }
        setDebugging(false);
    };

    return (
        <div className="debug-view" style={{ display: 'flex', flexDirection: 'column', height: '100%' }}>
            <div style={{ flex: '0 0 auto', maxHeight: '45%', overflow: 'hidden', borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))' }}>
                <RunConfigsPanel />
            </div>

            <div style={{ padding: '8px 10px', borderBottom: '1px solid var(--vscode-panel-border, rgba(255,255,255,0.06))' }}>
                {isDebugging ? (
                    <div style={{ display: 'flex', gap: 4, alignItems: 'center' }}>
                        <span style={{ fontSize: 11, opacity: 0.7, marginRight: 8 }}>{sessionName || 'Debugging'}</span>
                        <button type="button" style={ctrl} onClick={() => void debugContinue()} title="Continue (F5)">
                            <i className="codicon codicon-debug-continue" style={icon} />
                        </button>
                        <button type="button" style={ctrl} onClick={() => void debugStepOver()} title="Step Over">
                            <i className="codicon codicon-debug-step-over" style={icon} />
                        </button>
                        <button type="button" style={ctrl} onClick={() => void debugStepInto()} title="Step Into">
                            <i className="codicon codicon-debug-step-into" style={icon} />
                        </button>
                        <button type="button" style={ctrl} onClick={() => void debugStepOut()} title="Step Out">
                            <i className="codicon codicon-debug-step-out" style={icon} />
                        </button>
                        <button type="button" onClick={() => void handleStop()} style={ctrlStop} title="Stop">
                            <i className="codicon codicon-debug-stop" style={icon} />
                        </button>
                    </div>
                ) : (
                    <div style={{ fontSize: 11, opacity: 0.55 }}>
                        Pick a launch configuration above, or toggle breakpoints in the editor gutter (F9).
                    </div>
                )}
            </div>

            <div style={{ flex: 1, overflowY: 'auto', fontSize: 11 }}>
                <Panel title="WATCH">
                    {debugWatch.length === 0 ? (
                        <Empty>Add a watch expression below</Empty>
                    ) : (
                        debugWatch.map((w) => (
                            <Row key={w.id} label={w.expression} value={w.value} onRemove={() => removeDebugWatch(w.id)} />
                        ))
                    )}
                    <div style={{ display: 'flex', gap: 4, padding: '4px 8px' }}>
                        <input
                            value={watchExpr}
                            onChange={(e) => setWatchExpr(e.target.value)}
                            onKeyDown={async (e) => {
                                if (e.key === 'Enter' && watchExpr.trim()) {
                                    addDebugWatch(watchExpr.trim());
                                    if (isDebugging) {
                                        const { evaluateDebugExpression } = await import('../application/debug/evaluateExpression');
                                        const val = await evaluateDebugExpression(watchExpr.trim(), 'watch');
                                        useStore.getState().updateDebugWatchValue(
                                            useStore.getState().debugWatch.at(-1)?.id ?? '',
                                            val,
                                        );
                                    }
                                    setWatchExpr('');
                                }
                            }}
                            placeholder="Add watch…"
                            style={{ flex: 1, fontSize: 11, background: 'rgba(255,255,255,0.04)', border: '1px solid rgba(255,255,255,0.1)', borderRadius: 3, padding: '2px 6px', color: 'inherit' }}
                        />
                    </div>
                </Panel>

                <Panel title="VARIABLES">
                    {variables.length === 0 ? (
                        <Empty>Not debugging</Empty>
                    ) : (
                        variables.map((v) => (
                            <Row key={v.name} label={v.name} value={v.value} />
                        ))
                    )}
                </Panel>

                <Panel title="CALL STACK">
                    {stackFrames.length === 0 ? (
                        <Empty>Not debugging</Empty>
                    ) : (
                        stackFrames.map((f) => (
                            <Row
                                key={f.id}
                                label={f.name}
                                value={`${f.source?.path || ''}:${f.line}`}
                            />
                        ))
                    )}
                </Panel>

                <Panel title="THREADS">
                    {threads.length === 0 ? (
                        <Empty>Not debugging</Empty>
                    ) : (
                        threads.map((t) => <Row key={t.id} label={t.name} value={`#${t.id}`} />)
                    )}
                </Panel>

                <Panel title="BREAKPOINTS">
                    {breakpoints.length === 0 ? (
                        <Empty>No breakpoints — click gutter or press F9</Empty>
                    ) : (
                        breakpoints.map((b) => (
                            <Row
                                key={b.id}
                                label={b.path.split(/[\\/]/).pop() || b.path}
                                value={`line ${b.line}`}
                            />
                        ))
                    )}
                </Panel>

                {output.length > 0 && (
                    <Panel title="DEBUG OUTPUT">
                        {output.slice(-30).map((line, i) => (
                            <div key={i} style={{ padding: '2px 10px', opacity: 0.65, fontFamily: 'monospace', fontSize: 10 }}>
                                {line}
                            </div>
                        ))}
                    </Panel>
                )}
            </div>
        </div>
    );
};

const Panel: React.FC<{ title: string; children: React.ReactNode }> = ({ title, children }) => (
    <div style={{ marginBottom: 10 }}>
        <div style={{ fontSize: 11, fontWeight: 600, opacity: 0.85, background: 'rgba(255,255,255,0.05)', padding: '4px 8px' }}>
            {title}
        </div>
        {children}
    </div>
);

const Empty: React.FC<{ children: React.ReactNode }> = ({ children }) => (
    <div style={{ padding: '4px 10px', opacity: 0.45 }}>{children}</div>
);

const Row: React.FC<{ label: string; value: string; onRemove?: () => void }> = ({ label, value, onRemove }) => (
    <div style={{ padding: '3px 10px', display: 'flex', gap: 8, alignItems: 'center' }}>
        <span style={{ flex: 1, overflow: 'hidden', textOverflow: 'ellipsis', whiteSpace: 'nowrap' }}>{label}</span>
        <span style={{ opacity: 0.55, fontFamily: 'monospace', fontSize: 10 }}>{value}</span>
        {onRemove && (
            <i
                className="codicon codicon-close"
                onClick={onRemove}
                style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 11, opacity: 0.5, cursor: 'pointer' }}
            />
        )}
    </div>
);

const ctrl: React.CSSProperties = {
    flex: '0 0 auto',
    background: 'var(--vscode-button-background, #444)',
    color: 'white',
    border: 'none',
    padding: 6,
    cursor: 'pointer',
    borderRadius: 2,
};

const ctrlStop: React.CSSProperties = { ...ctrl, background: '#a1260d' };
const icon: React.CSSProperties = { fontFamily: 'codicon', fontStyle: 'normal' };

export default DebugView;
