import React from 'react';
import { Brain, Cpu, Sparkles, Wand2 } from 'lucide-react';
import { useStore } from '../store';

export const ThoughtProcess: React.FC = () => {
    const { currentThought } = useStore();

    if (!currentThought) return null;

    return (
        <div className="fixed bottom-24 right-8 w-80 max-h-96 bg-slate-900/80 backdrop-blur-2xl border border-white/10 rounded-2xl shadow-2xl overflow-hidden animate-in slide-in-from-bottom-4 duration-500">
            <div className="flex items-center gap-3 px-4 py-3 bg-white/5 border-b border-white/5">
                <div className="relative">
                    <Brain className="w-5 h-5 text-purple-400" />
                    <div className="absolute inset-0 bg-purple-400 blur-lg opacity-20 animate-pulse" />
                </div>
                <span className="text-xs font-bold text-white tracking-tight uppercase">Cognitive Process</span>
                <div className="ml-auto flex gap-1">
                    <div className="w-1 h-1 rounded-full bg-purple-500 animate-bounce [animation-delay:-0.3s]" />
                    <div className="w-1 h-1 rounded-full bg-purple-500 animate-bounce [animation-delay:-0.15s]" />
                    <div className="w-1 h-1 rounded-full bg-purple-500 animate-bounce" />
                </div>
            </div>

            <div className="p-4 space-y-4 overflow-y-auto max-h-[calc(24rem-3rem)]">
                <div className="space-y-2">
                    <div className="flex items-center gap-2 text-[10px] font-bold text-slate-500 uppercase tracking-widest">
                        <Cpu className="w-3 h-3" />
                        Internal Logic
                    </div>
                    <p className="text-xs text-slate-300 leading-relaxed italic">
                        "{currentThought.logic}"
                    </p>
                </div>

                <div className="space-y-2">
                    <div className="flex items-center gap-2 text-[10px] font-bold text-slate-500 uppercase tracking-widest">
                        <Wand2 className="w-3 h-3" />
                        Proposed Action
                    </div>
                    <div className="p-2.5 bg-white/5 rounded-xl border border-white/5">
                        <p className="text-xs text-purple-200 font-medium">
                            {currentThought.action}
                        </p>
                    </div>
                </div>

                {currentThought.confidence !== undefined && (
                    <div className="space-y-2">
                        <div className="flex justify-between items-center text-[10px] font-bold text-slate-500 uppercase tracking-widest">
                            <div className="flex items-center gap-2">
                                <Sparkles className="w-3 h-3" />
                                Confidence
                            </div>
                            <span className="text-purple-400">{Math.round(currentThought.confidence * 100)}%</span>
                        </div>
                        <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                            <div
                                className="h-full bg-gradient-to-r from-purple-500 to-blue-500 transition-all duration-1000"
                                style={{ width: `${currentThought.confidence * 100}%` }}
                            />
                        </div>
                    </div>
                )}
            </div>
        </div>
    );
};
