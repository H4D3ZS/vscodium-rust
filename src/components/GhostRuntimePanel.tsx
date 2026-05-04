import React from 'react';
import { Terminal, Shield, Play, CheckCircle, XCircle, AlertTriangle } from 'lucide-react';
import { useStore } from '../store';

export const GhostRuntimePanel: React.FC = () => {
    const { ghostRuntimeResults } = useStore();

    if (!ghostRuntimeResults || ghostRuntimeResults.length === 0) {
        return (
            <div className="flex flex-col items-center justify-center h-full text-slate-500 gap-3 opacity-50">
                <Shield className="w-12 h-12 stroke-[1px]" />
                <span className="text-xs font-medium uppercase tracking-widest">Ghost Runtime Idle</span>
            </div>
        );
    }

    const lastResult = ghostRuntimeResults[0];

    return (
        <div className="flex flex-col h-full bg-slate-950 border border-white/5 rounded-xl overflow-hidden shadow-2xl">
            <div className="flex items-center justify-between px-4 py-3 bg-white/5 border-b border-white/5">
                <div className="flex items-center gap-2">
                    <Play className="w-4 h-4 text-blue-400 fill-blue-400" />
                    <span className="text-xs font-bold text-white tracking-tight uppercase">Ghost Verification</span>
                </div>
                <div className="flex items-center gap-3">
                    <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/20">
                        <CheckCircle className="w-3 h-3 text-emerald-400" />
                        <span className="text-[10px] font-bold text-emerald-400">{lastResult.passCount}</span>
                    </div>
                    <div className="flex items-center gap-1.5 px-2 py-0.5 rounded-full bg-rose-500/10 border border-rose-500/20">
                        <XCircle className="w-3 h-3 text-rose-400" />
                        <span className="text-[10px] font-bold text-rose-400">{lastResult.failCount}</span>
                    </div>
                </div>
            </div>

            <div className="flex-1 overflow-y-auto p-4 font-mono text-[11px] leading-relaxed selection:bg-blue-500/30">
                <div className="space-y-2">
                    {lastResult.output.split('\n').map((line, i) => (
                        <div key={i} className="flex gap-3">
                            <span className="text-slate-600 select-none w-4 text-right">{i + 1}</span>
                            <span className={
                                line.includes('FAILED') ? 'text-rose-400' :
                                    line.includes('PASSED') ? 'text-emerald-400' :
                                        line.startsWith('$') ? 'text-blue-400 font-bold' : 'text-slate-300'
                            }>
                                {line}
                            </span>
                        </div>
                    ))}
                </div>
            </div>

            {lastResult.failCount > 0 && (
                <div className="p-4 bg-rose-500/5 border-t border-rose-500/20">
                    <div className="flex items-start gap-3">
                        <AlertTriangle className="w-5 h-5 text-rose-400 mt-0.5" />
                        <div className="flex-1">
                            <h4 className="text-xs font-bold text-rose-400 mb-1">Fault Detected</h4>
                            <p className="text-[11px] text-rose-200/60 leading-normal mb-3">
                                Agentic self-healing has been triggered to resolve these failures before commit.
                            </p>
                            <button className="px-3 py-1.5 bg-rose-500 text-white rounded-lg text-[10px] font-bold hover:bg-rose-400 transition-colors shadow-lg shadow-rose-500/20">
                                Fix Regression
                            </button>
                        </div>
                    </div>
                </div>
            )}
        </div>
    );
};
