import React, { useEffect } from 'react';
import { useStore } from '../../store';
import KortexLocalInferencePanel from './KortexLocalInferencePanel';

/**
 * Inference backend settings.
 *
 * "Kortex ROCmFPX" (the local AMD-GPU backend) is the card at the top and its
 * own selector — starting it makes it the active backend. Everything else is a
 * short radio list below.
 */
const InferenceBackendPanel: React.FC = () => {
    const backend = useStore((s) => s.inferenceBackend);
    const setBackend = useStore((s) => s.setInferenceBackend);
    const useClaudeCodeAgent = useStore((s) => s.useClaudeCodeAgent);
    const setUseClaudeCodeAgent = useStore((s) => s.setUseClaudeCodeAgent);
    const [claudeAvailable, setClaudeAvailable] = React.useState<boolean | null>(null);

    React.useEffect(() => {
        let cancelled = false;
        import('../../tauri_bridge')
            .then(({ invoke }) => invoke<boolean>('claude_code_available'))
            .then((ok) => { if (!cancelled) setClaudeAvailable(ok); })
            .catch(() => { if (!cancelled) setClaudeAvailable(false); });
        return () => { cancelled = true; };
    }, []);

    const lemonadeStatus = useStore((s) => s.lemonadeStatus);
    const checkLemonade = useStore((s) => s.checkLemonadeStatus);
    const checkLlamaCpp = useStore((s) => s.checkLlamaCppStatus);
    const lemonadeUrl = useStore((s) => s.lemonadeUrl);
    const setLemonadeUrl = useStore((s) => s.setLemonadeUrl);

    useEffect(() => {
        void checkLemonade();
        void checkLlamaCpp();
    }, [backend]);

    const dot = (s: string) => (
        <span style={{ color: s === 'running' ? '#9ece6a' : s === 'error' ? '#f7768e' : '#888' }}>{s}</span>
    );
    const input: React.CSSProperties = {
        fontSize: 12, padding: '4px 8px', borderRadius: 3,
        background: 'var(--vscode-input-background)', color: 'var(--vscode-input-foreground)',
        border: '1px solid var(--vscode-input-border)',
    };

    return (
        <div style={{ maxWidth: 640, display: 'flex', flexDirection: 'column', gap: 12 }}>
            <div>
                <div style={{ fontSize: 13, fontWeight: 600, marginBottom: 2 }}>AI Model</div>
                <p style={{ fontSize: 11, opacity: 0.6, margin: 0 }}>
                    One button to run a model on your AMD GPU, or pick another backend.
                </p>
            </div>

            <KortexLocalInferencePanel />

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input type="radio" name="inference-backend"
                    checked={backend === 'lemonade'} onChange={() => setBackend('lemonade' as any)} />
                <span style={{ fontWeight: 600 }}>Lemonade</span>
                <span style={{ fontSize: 11, opacity: 0.55, flex: 1 }}>{lemonadeStatus} · standard GGUF quants only</span>
            </label>
            {backend === 'lemonade' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginLeft: 24 }}>
                    <label style={{ fontSize: 11 }}>Lemonade server URL</label>
                    <input style={input} value={lemonadeUrl} onChange={(e) => setLemonadeUrl(e.target.value)} />
                </div>
            )}

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input type="radio" name="inference-backend"
                    checked={backend === 'openai'} onChange={() => setBackend('openai')} />
                <span style={{ fontWeight: 600 }}>OpenAI-compatible API</span>
                <span style={{ fontSize: 11, opacity: 0.55, flex: 1 }}>cloud / LiteLLM</span>
            </label>

            <div style={{ paddingTop: 10, marginTop: 2, borderTop: '1px solid var(--vscode-panel-border)' }}>
                <label style={{
                    display: 'flex', alignItems: 'center', gap: 8, fontSize: 12,
                    cursor: claudeAvailable === false ? 'not-allowed' : 'pointer',
                    opacity: claudeAvailable === false ? 0.5 : 1,
                }}>
                    <input type="checkbox" checked={useClaudeCodeAgent}
                        disabled={claudeAvailable === false}
                        onChange={(e) => setUseClaudeCodeAgent(e.target.checked)} />
                    Run chat through the Claude Code CLI
                </label>
                <p style={{ fontSize: 11, opacity: 0.6, margin: '4px 0 0 24px' }}>
                    Same model, but Claude Code's tools, hooks and skills.
                    {claudeAvailable === false && (
                        <> Not installed — <code>npm i -g @anthropic-ai/claude-code</code></>
                    )}
                </p>
            </div>
        </div>
    );
};

export default InferenceBackendPanel;
