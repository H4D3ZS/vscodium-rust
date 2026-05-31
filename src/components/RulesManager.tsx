import React from 'react';
import { useStore } from '../store';

interface RulesManagerProps {
    onClose?: () => void;
}

export const RulesManager: React.FC<RulesManagerProps> = ({ onClose }) => {
    const globalRule = useStore(state => state.globalSteeringRule);
    const setGlobalRule = useStore(state => state.setGlobalSteeringRule);
    const hooks = useStore(state => state.agentHooks);
    const setHooks = useStore(state => state.setAgentHooks);
    
    const addHook = () => {
        setHooks([...hooks, { id: Math.random().toString(), pattern: '*.ts', prompt: 'Refactor this code', enabled: true }]);
    };

    const removeHook = (id: string) => {
        setHooks(hooks.filter(h => h.id !== id));
    };

    return (
        <div style={{ padding: '16px', color: 'var(--vscode-foreground)', height: '100%', overflowY: 'auto' }}>
            <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '16px' }}>
                <h2 style={{ margin: 0, fontSize: '14px', textTransform: 'uppercase', letterSpacing: '1px' }}>Steering Rules & Hooks</h2>
                {onClose && <button onClick={onClose} style={{ background: 'none', border: 'none', color: 'inherit', cursor: 'pointer' }}>×</button>}
            </div>

            <div style={{ marginBottom: '24px' }}>
                <div style={{ fontSize: '12px', fontWeight: 600, marginBottom: '8px' }}>Global Steering Rules</div>
                <div style={{ fontSize: '11px', opacity: 0.7, marginBottom: '8px' }}>
                    Define guidelines the agent must always follow across the entire workspace (e.g., "Always use functional components" or "Never use any").
                </div>
                <textarea
                    value={globalRule}
                    onChange={(e) => setGlobalRule(e.target.value)}
                    placeholder="Enter global prompt instructions..."
                    style={{
                        width: '100%',
                        minHeight: '80px',
                        background: 'rgba(0,0,0,0.2)',
                        border: '1px solid rgba(255,255,255,0.1)',
                        color: 'var(--vscode-editor-foreground, #fff)',
                        padding: '8px',
                        borderRadius: '4px',
                        fontFamily: 'inherit',
                        resize: 'vertical'
                    }}
                />
            </div>

            <div>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '8px' }}>
                    <div style={{ fontSize: '12px', fontWeight: 600 }}>Automated Agent Hooks</div>
                    <button onClick={addHook} style={{ background: 'rgba(255,255,255,0.1)', border: 'none', color: 'white', padding: '4px 8px', borderRadius: '4px', cursor: 'pointer', fontSize: '10px' }}>
                        + Add Hook
                    </button>
                </div>
                <div style={{ fontSize: '11px', opacity: 0.7, marginBottom: '12px' }}>
                    Agents execute in the background when files matching the pattern are saved.
                </div>
                
                {hooks.length === 0 ? (
                    <div style={{ opacity: 0.5, fontSize: '11px' }}>No hooks defined.</div>
                ) : (
                    hooks.map(hook => (
                        <div key={hook.id} style={{
                            background: 'rgba(255,255,255,0.03)',
                            border: '1px solid rgba(255,255,255,0.05)',
                            padding: '10px',
                            borderRadius: '6px',
                            marginBottom: '8px'
                        }}>
                            <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: '8px' }}>
                                <input 
                                    type="text" 
                                    value={hook.pattern}
                                    onChange={(e) => {
                                        const newHooks = [...hooks];
                                        const idx = newHooks.findIndex(h => h.id === hook.id);
                                        newHooks[idx] = { ...newHooks[idx], pattern: e.target.value };
                                        setHooks(newHooks);
                                    }}
                                    style={{ background: 'rgba(0,0,0,0.3)', border: '1px solid rgba(255,255,255,0.1)', color: 'white', padding: '4px 8px', borderRadius: '4px', fontSize: '11px', width: '120px' }}
                                />
                                <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                                    <label style={{ fontSize: '10px', display: 'flex', alignItems: 'center', gap: '4px', cursor: 'pointer' }}>
                                        <input 
                                            type="checkbox" 
                                            checked={hook.enabled}
                                            onChange={(e) => {
                                                const newHooks = [...hooks];
                                                const idx = newHooks.findIndex(h => h.id === hook.id);
                                                newHooks[idx] = { ...newHooks[idx], enabled: e.target.checked };
                                                setHooks(newHooks);
                                            }}
                                        />
                                        Enabled
                                    </label>
                                    <button onClick={() => removeHook(hook.id)} style={{ background: 'none', border: 'none', color: '#ef4444', cursor: 'pointer', fontSize: '14px', padding: 0 }}>×</button>
                                </div>
                            </div>
                            <input 
                                type="text"
                                placeholder="Agent prompt to execute on save..."
                                value={hook.prompt}
                                onChange={(e) => {
                                    const newHooks = [...hooks];
                                    const idx = newHooks.findIndex(h => h.id === hook.id);
                                    newHooks[idx] = { ...newHooks[idx], prompt: e.target.value };
                                    setHooks(newHooks);
                                }}
                                style={{ width: '100%', background: 'rgba(0,0,0,0.3)', border: '1px solid rgba(255,255,255,0.1)', color: 'white', padding: '6px 8px', borderRadius: '4px', fontSize: '11px' }}
                            />
                        </div>
                    ))
                )}
            </div>
        </div>
    );
};

export default RulesManager;
