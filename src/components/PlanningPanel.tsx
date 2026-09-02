import React from 'react';
import { CheckCircle2, Circle, Clock, Loader2, AlertCircle } from 'lucide-react';
import { useStore } from '../store';

export const PlanningPanel: React.FC = () => {
    const taskPlannerState = useStore(s => s.taskPlannerState);

    if (!taskPlannerState || taskPlannerState.state === 'Complete' || taskPlannerState.state === 'Inactive') {
        return null;
    }

    const getStatusIcon = (stepStatus: string) => {
        switch (stepStatus) {
            case 'Complete': return <CheckCircle2 className="w-4 h-4 text-emerald-400" />;
            case 'Executing': return <Loader2 className="w-4 h-4 text-blue-400 animate-spin" />;
            case 'Failed': return <AlertCircle className="w-4 h-4 text-rose-400" />;
            default: return <Circle className="w-4 h-4 text-slate-500" />;
        }
    };

    return (
        <div className="flex flex-col h-full bg-slate-900/50 backdrop-blur-xl border-l border-white/5 p-4 overflow-y-auto">
            <div className="flex items-center gap-2 mb-6">
                <div className="p-2 bg-blue-500/10 rounded-lg">
                    <Clock className="w-5 h-5 text-blue-400" />
                </div>
                <div>
                    <h2 className="text-sm font-semibold text-white">Mission Control</h2>
                    <p className="text-xs text-slate-400">Current Phase: {taskPlannerState.state}</p>
                </div>
            </div>

            <div className="space-y-4">
                {taskPlannerState.steps.map((step, idx) => (
                    <div
                        key={idx}
                        className={`p-3 rounded-xl border transition-all duration-300 ${step.status === 'Executing'
                                ? 'bg-blue-500/5 border-blue-500/20 ring-1 ring-blue-500/20'
                                : 'bg-white/5 border-transparent'
                            }`}
                    >
                        <div className="flex items-start gap-3">
                            <div className="mt-0.5">
                                {getStatusIcon(step.status)}
                            </div>
                            <div className="flex-1 min-w-0">
                                <p className={`text-xs font-medium leading-tight ${step.status === 'Complete' ? 'text-slate-400 line-through' : 'text-slate-200'
                                    }`}>
                                    {step.description}
                                </p>
                                {step.status === 'Executing' && (
                                    <div className="mt-2 space-y-1">
                                        <div className="h-1 bg-slate-800 rounded-full overflow-hidden">
                                            <div className="h-full bg-blue-500 animate-pulse w-1/2" />
                                        </div>
                                    </div>
                                )}
                                {step.error && (
                                    <p className="mt-2 text-[10px] text-rose-400 bg-rose-400/10 p-2 rounded-md font-mono border border-rose-400/20 line-clamp-2">
                                        {step.error}
                                    </p>
                                )}
                            </div>
                        </div>
                    </div>
                ))}
            </div>

            {taskPlannerState.state === 'SelfHealing' && (
                <div className="mt-6 p-4 rounded-xl bg-amber-500/10 border border-amber-500/20 animate-pulse">
                    <div className="flex items-center gap-2 mb-2">
                        <AlertCircle className="w-4 h-4 text-amber-400" />
                        <span className="text-xs font-bold text-amber-400 uppercase tracking-wider">Self-Healing Active</span>
                    </div>
                    <p className="text-xs text-amber-200/80 leading-relaxed">
                        AI detected an execution fault. Re-planning intervention strategy...
                    </p>
                </div>
            )}
        </div>
    );
};
