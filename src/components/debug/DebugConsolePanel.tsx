import React, { useState } from 'react';
import { useStore } from '../../store';

const DebugConsolePanel: React.FC = () => {
    const isDebugging = useStore((s) => s.isDebugging);
    const debugOutput = useStore((s) => s.debugOutput);
    const [expr, setExpr] = useState('');

    const submitEval = async () => {
        const trimmed = expr.trim();
        if (!trimmed || !isDebugging) return;
        const { evaluateDebugExpression } = await import('../../application/debug/evaluateExpression');
        await evaluateDebugExpression(trimmed, 'repl');
        setExpr('');
    };

    return (
        <div style={{ height: '100%', display: 'flex', flexDirection: 'column', padding: '8px 12px', fontFamily: 'var(--font-mono)', fontSize: 11 }}>
            <div style={{ flex: 1, overflowY: 'auto', marginBottom: 8, opacity: isDebugging ? 1 : 0.6 }}>
                {!isDebugging && (
                    <div style={{ fontStyle: 'italic', opacity: 0.6, marginBottom: 6 }}>
                        Start a debug session to evaluate expressions.
                    </div>
                )}
                {debugOutput.slice(-80).map((line, i) => (
                    <div key={i} style={{ whiteSpace: 'pre-wrap', wordBreak: 'break-all', lineHeight: '16px' }}>{line}</div>
                ))}
            </div>
            <div style={{ display: 'flex', alignItems: 'center', borderTop: '1px solid rgba(255,255,255,0.08)', paddingTop: 6 }}>
                <i className="codicon codicon-chevron-right" style={{ fontFamily: 'codicon', fontStyle: 'normal', fontSize: 12, marginRight: 6, color: '#3794ef' }} />
                <input
                    type="text"
                    value={expr}
                    onChange={(e) => setExpr(e.target.value)}
                    onKeyDown={(e) => { if (e.key === 'Enter') void submitEval(); }}
                    placeholder="Evaluate expression (Enter)"
                    disabled={!isDebugging}
                    style={{ background: 'transparent', border: 'none', color: 'inherit', fontSize: 12, width: '100%', outline: 'none', fontFamily: 'inherit' }}
                />
            </div>
        </div>
    );
};

export default DebugConsolePanel;
