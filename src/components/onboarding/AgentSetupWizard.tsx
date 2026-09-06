import React, { useEffect, useState } from 'react';
import { invoke } from '../../tauri_bridge';
import { useStore } from '../../store';

const SETUP_KEY = 'agent.setup.done';

type Persona = 'dev' | 'bounty' | 'ml';

const PERSONAS: { id: Persona; title: string; desc: string; mode: string; modelHint: string }[] = [
    {
        id: 'dev',
        title: 'Software engineer',
        desc: 'Ship features with the engineering harness loop.',
        mode: 'Harness',
        modelHint: 'Local: load a strong SWE/agentic model (~16 GB) into your local server. Cloud: Cyber-Ifrit or BugTraceAI.',
    },
    {
        id: 'bounty',
        title: 'Bug bounty / pentest',
        desc: 'Offensive tooling, browser recon, structured reports.',
        mode: 'BugBounty',
        modelHint: 'Use BugTraceAI or Cyber-Ifrit cloud — not “Antigravity” (that is workflow memory, not a model).',
    },
    {
        id: 'ml',
        title: 'ML / PyTorch',
        desc: 'Local training + agent-assisted notebooks.',
        mode: 'Agent',
        modelHint: 'A local backend or cloud — open ML Studio from the activity bar.',
    },
];

const AgentSetupWizard: React.FC = () => {
    const [open, setOpen] = useState(false);
    const [step, setStep] = useState<'persona' | 'explain' | 'done'>('persona');
    const [persona, setPersona] = useState<Persona>('dev');
    const setAgentMode = useStore((s) => s.setAgentMode);

    useEffect(() => {
        try {
            if (localStorage.getItem(SETUP_KEY) === '1') return;
        } catch {
            return;
        }
        const t = window.setTimeout(() => setOpen(true), 800);
        return () => clearTimeout(t);
    }, []);

    const chosen = PERSONAS.find((p) => p.id === persona)!;

    const finish = async () => {
        setAgentMode(chosen.mode);
        try {
            localStorage.setItem(SETUP_KEY, '1');
            localStorage.setItem('agent.mode', chosen.mode);
        } catch { /* */ }
        await invoke('enterprise_audit_log', {
            action: 'onboarding.agent_setup',
            detail: { persona, mode: chosen.mode },
        }).catch(() => { });
        setStep('done');
        window.setTimeout(() => setOpen(false), 2400);
    };

    const dismiss = () => {
        try {
            localStorage.setItem(SETUP_KEY, '1');
        } catch { /* */ }
        setOpen(false);
    };

    if (!open) return null;

    return (
        <div className="agent-wizard-overlay" role="dialog" aria-modal="true" aria-labelledby="agent-setup-title">
            <div className="agent-wizard-card">
                <header className="agent-wizard-header">
                    <span className="agent-wizard-badge">Enterprise-ready agent IDE</span>
                    <h2 id="agent-setup-title">How the agent works</h2>
                    <p className="agent-wizard-sub">
                        Three separate knobs — do not mix them up:
                        <strong> Mode</strong> (behavior) · <strong> Model</strong> (which LLM) ·{' '}
                        <strong> Antigravity</strong> (mission memory on disk, not a model name).
                    </p>
                </header>

                {step === 'persona' && (
                    <div className="agent-wizard-body">
                        <p style={{ fontSize: 12, opacity: 0.75, marginBottom: 10 }}>What are you here to do?</p>
                        <div className="agent-wizard-models">
                            {PERSONAS.map((p) => (
                                <button
                                    key={p.id}
                                    type="button"
                                    className={`agent-wizard-model${persona === p.id? ' is-selected': ''}`}
                                    onClick={() => setPersona(p.id)}
                                >
                                    <span className="agent-wizard-model-name">{p.title}</span>
                                    <span className="agent-wizard-model-desc">{p.desc}</span>
                                </button>
                            ))}
                        </div>
                        <div className="agent-wizard-actions">
                            <button type="button" className="agent-wizard-primary" onClick={() => setStep('explain')}>
                                Continue
                            </button>
                            <button type="button" className="agent-wizard-ghost" onClick={dismiss}>
                                Skip
                            </button>
                        </div>
                    </div>
                )}

                {step === 'explain' && (
                    <div className="agent-wizard-body">
                        <ul className="agent-wizard-list">
                            <li>
                                <strong>Mode → {chosen.mode}</strong> — set in the chat toolbar pill (Bug Bounty, Harness, Sentient…).
                            </li>
                            <li>
                                <strong>Model</strong> — second pill in chat; must be a real local or Cyber-Ifrit model.
                            </li>
                            <li>{chosen.modelHint}</li>
                            <li>
                                <strong>YOLO / AUTO</strong> — auto-approve tools during autonomous runs (on for Bug Bounty).
                            </li>
                        </ul>
                        <div className="agent-wizard-actions">
                            <button type="button" className="agent-wizard-primary" onClick={() => void finish()}>
                                Set mode to {chosen.mode} &amp; start
                            </button>
                            <button type="button" className="agent-wizard-ghost" onClick={() => setStep('persona')}>
                                Back
                            </button>
                        </div>
                    </div>
                )}

                {step === 'done' && (
                    <div className="agent-wizard-body agent-wizard-center">
                        <p className="agent-wizard-success">
                             Mode set to <strong>{chosen.mode}</strong> — open chat with <kbd>Ctrl+L</kbd>, pick a model, send a mission.
                        </p>
                    </div>
                )}
            </div>
        </div>
    );
};

export default AgentSetupWizard;
