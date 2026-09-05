import React from 'react';
import { useStore } from '../../store';

// Antigravity-style plan gate: when the agent outputs a task plan and stops at
// AWAITING_APPROVAL, show a Proceed / Review card instead of making the user
// retype "[PROCEED]".

const PLAN_RE = /<TASK_PLAN>([\s\S]*?)<\/TASK_PLAN>/i;

export function messageHasPlanGate(content: string): boolean {
    if (!content) return false;
    return /AWAITING_APPROVAL/i.test(content) || PLAN_RE.test(content);
}

const PlanProceedCard: React.FC<{ content: string }> = ({ content }) => {
    const isAgentThinking = useStore(s => s.isAgentThinking);
    const activeRoot = useStore(s => s.activeRoot);

    const steps = React.useMemo(() => {
        const m = content.match(PLAN_RE);
        const body = m ? m[1] : content;
        return body
            .split('\n')
            .map(l => l.trim())
            .filter(l => /^\d+[.)]\s+/.test(l))
            .map(l => l.replace(/^\d+[.)]\s+/, ''))
            .slice(0, 12);
    }, [content]);

    const proceed = async () => {
        const { sendAgentMessage } = await import('../../agent');
        await sendAgentMessage('[PROCEED] Proceed with the plan above. Execute every step.');
    };
    const review = async () => {
        if (!activeRoot) return;
        try {
            const { openFile } = await import('../../application/editor/openFile');
            await openFile(`${activeRoot}/task_plan.md`);
        } catch { /* plan file may not exist yet */ }
    };

    return (
        <div className="ac-card">
            <div className="ac-card__title">
                <i className="codicon codicon-checklist" style={{ color: 'var(--ac-accent)' }} />
                Implementation plan — review before proceeding
            </div>
            {steps.length > 0 && (
                <ol className="ac-card__steps">
                    {steps.map((s, i) => <li key={i}>{s}</li>)}
                </ol>
            )}
            <div className="ac-card__actions">
                <button className="ac-btn ac-btn--primary" onClick={proceed} disabled={isAgentThinking}>
                    <i className="codicon codicon-play" /> Proceed
                </button>
                <button className="ac-btn ac-btn--ghost" onClick={review}>
                    <i className="codicon codicon-eye" /> Review plan
                </button>
            </div>
        </div>
    );
};

export default PlanProceedCard;
