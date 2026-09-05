import React, { useEffect } from 'react';
import { useStore } from '../../store';

/**
 * Ollama vs llama.cpp vs Lemonade inference backend selector.
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
    const ollamaStatus = useStore((s) => s.ollamaStatus);
    const llamaCppStatus = useStore((s) => s.llamaCppStatus);
    const lemonadeStatus = useStore((s) => s.lemonadeStatus);
    const checkOllama = useStore((s) => s.checkOllamaStatus);
    const checkLlamaCpp = useStore((s) => s.checkLlamaCppStatus);
    const checkLemonade = useStore((s) => s.checkLemonadeStatus);
    const llamaCppUrl = useStore((s) => s.llamaCppUrl);
    const setLlamaCppUrl = useStore((s) => s.setLlamaCppUrl);
    const lemonadeUrl = useStore((s) => s.lemonadeUrl);
    const setLemonadeUrl = useStore((s) => s.setLemonadeUrl);
    const ollamaUrl = useStore((s) => s.ollamaUrl);

    useEffect(() => {
        void checkOllama();
        void checkLlamaCpp();
        void checkLemonade();
    }, [backend]);

    const statusDot = (s: string) => (
        <span style={{ color: s === 'running' ? '#9ece6a' : s === 'error' ? '#f7768e' : '#888' }}>
            {s}
        </span>
    );

    return (
        <div style={{ maxWidth: 640, display: 'flex', flexDirection: 'column', gap: 14 }}>
            <div style={{ fontSize: 13, fontWeight: 600 }}>Inference Backend</div>
            <p style={{ fontSize: 11, opacity: 0.65, margin: 0 }}>
                Lemonade is the local backend — real llama.cpp, auto-detects models on NVIDIA/ROCm.
            </p>

            <div style={{ marginTop: 4, paddingTop: 12, borderTop: '1px solid var(--vscode-panel-border)' }}>
                <label style={{
                    display: 'flex', alignItems: 'center', gap: 8, fontSize: 12,
                    cursor: claudeAvailable === false ? 'not-allowed' : 'pointer',
                    opacity: claudeAvailable === false ? 0.5 : 1,
                }}>
                    <input
                        type="checkbox"
                        checked={useClaudeCodeAgent}
                        disabled={claudeAvailable === false}
                        onChange={(e) => setUseClaudeCodeAgent(e.target.checked)}
                    />
                    Use Claude Code as the chat agent
                </label>
                <p style={{ fontSize: 11, opacity: 0.65, margin: '4px 0 0 24px' }}>
                    Chat prompts run through the Claude Code CLI instead of the built-in agent
                    loop — same local model via Lemonade, but Claude Code's tools, hooks and
                    skills. Each reply continues the same Claude Code session.
                </p>
                {claudeAvailable === false && (
                    <p style={{ fontSize: 11, opacity: 0.7, margin: '4px 0 0 24px', color: '#e5a00d' }}>
                        Claude Code CLI not found. Install it with
                        <code style={{ marginLeft: 4 }}>npm i -g @anthropic-ai/claude-code</code>
                    </p>
                )}
            </div>

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input
                    type="radio"
                    name="inference-backend"
                    checked={backend === 'lemonade'}
                    onChange={() => setBackend('lemonade' as any)}
                />
                Lemonade — {statusDot(lemonadeStatus)} ({lemonadeUrl})
                <span style={{ fontSize: 10, opacity: 0.5, marginLeft: 4 }}>(NVIDIA/ROCm, auto-detect models)</span>
            </label>

            {backend === 'lemonade' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginLeft: 24 }}>
                    <label style={{ fontSize: 11 }}>Lemonade server URL</label>
                    <input
                        value={lemonadeUrl}
                        onChange={(e) => setLemonadeUrl(e.target.value)}
                        style={{
                            fontSize: 12,
                            padding: '4px 8px',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border)',
                        }}
                    />
                </div>
            )}

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input
                    type="radio"
                    name="inference-backend"
                    checked={backend === 'llama-cpp'}
                    onChange={() => setBackend('llama-cpp')}
                />
                llama.cpp / Kortex — {statusDot(llamaCppStatus)}
            </label>

            {backend === 'llama-cpp' && (
                <div style={{ display: 'flex', flexDirection: 'column', gap: 6, marginLeft: 24 }}>
                    <label style={{ fontSize: 11 }}>llama.cpp server URL</label>
                    <input
                        value={llamaCppUrl}
                        onChange={(e) => setLlamaCppUrl(e.target.value)}
                        style={{
                            fontSize: 12,
                            padding: '4px 8px',
                            background: 'var(--vscode-input-background)',
                            color: 'var(--vscode-input-foreground)',
                            border: '1px solid var(--vscode-input-border)',
                        }}
                    />
                </div>
            )}

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input
                    type="radio"
                    name="inference-backend"
                    checked={backend === 'openai'}
                    onChange={() => setBackend('openai')}
                />
                OpenAI-compatible API (cloud / LiteLLM)
            </label>
        </div>
    );
};

export default InferenceBackendPanel;
