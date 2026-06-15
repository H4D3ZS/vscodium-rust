import React, { useEffect } from 'react';
import { useStore } from '../../store';

/**
 * Ollama vs llama.cpp inference backend selector.
 * Wired to `inferenceSlice` — the agent routing layer reads `inferenceBackend`.
 */
const InferenceBackendPanel: React.FC = () => {
    const backend = useStore((s) => s.inferenceBackend);
    const setBackend = useStore((s) => s.setInferenceBackend);
    const ollamaStatus = useStore((s) => s.ollamaStatus);
    const llamaCppStatus = useStore((s) => s.llamaCppStatus);
    const checkOllama = useStore((s) => s.checkOllamaStatus);
    const checkLlamaCpp = useStore((s) => s.checkLlamaCppStatus);
    const llamaCppUrl = useStore((s) => s.llamaCppUrl);
    const setLlamaCppUrl = useStore((s) => s.setLlamaCppUrl);
    const ollamaUrl = useStore((s) => s.ollamaUrl);

    useEffect(() => {
        void checkOllama();
        void checkLlamaCpp();
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
                Ollama is the default local stack. llama.cpp (Kortex) is the power tier for GGUF + AIM context.
            </p>

            <label style={{ display: 'flex', alignItems: 'center', gap: 8, fontSize: 12, cursor: 'pointer' }}>
                <input
                    type="radio"
                    name="inference-backend"
                    checked={backend === 'ollama'}
                    onChange={async () => {
                        setBackend('ollama');
                        // Auto-downgrade APEX models when Ollama is selected
                        try {
                            const { invoke } = await import('../../tauri_bridge');
                            await invoke('apex_set_local_mode', { smallModel: 'qwen3.5:2b' });
                        } catch (err) {
                            console.warn('Failed to auto-downgrade APEX models:', err);
                        }
                    }}
                />
                Ollama — {statusDot(ollamaStatus)} ({ollamaUrl})
            </label>

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
