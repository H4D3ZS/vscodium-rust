import React, { useState, useEffect } from 'react';
import { invoke } from '../tauri_bridge';

interface ModelInfo {
    name: string;
    context_window: number;
    recommended: boolean;
    supports_12b_and_below: boolean;
}

const ModelSelectorPanel: React.FC = () => {
    const [models, setModels] = useState<ModelInfo[]>([]);
    const [currentModel, setCurrentModel] = useState<string>('');
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        loadModels();
    }, []);

    const loadModels = async () => {
        setLoading(true);
        try {
            const [modelList, current] = await Promise.all([
                invoke<ModelInfo[]>('list_ollama_models', {}),
                invoke<string>('get_current_model', {}),
            ]);
            setModels(modelList);
            setCurrentModel(current);
            setError(null);
        } catch (err) {
            setError(`${err}`);
        } finally {
            setLoading(false);
        }
    };

    const handleSelectModel = async (modelName: string) => {
        try {
            await invoke('set_current_model', { modelName });
            setCurrentModel(modelName);
        } catch (err) {
            setError(`Failed to select model: ${err}`);
        }
    };

    const handleAutoDetect = async () => {
        try {
            const detected = await invoke<string>('detect_best_model', {});
            setCurrentModel(detected);
        } catch (err) {
            setError(`Auto-detection failed: ${err}`);
        }
    };

    if (loading) {
        return <div style={{ padding: 16, opacity: 0.6 }}>Loading Ollama models...</div>;
    }

    if (error) {
        return (
            <div style={{ maxWidth: 680 }}>
                <div style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>Model Selection</div>
                <div style={{ padding: 12, background: '#f7768e20', borderRadius: 4, color: '#f7768e' }}>
                    {error}
                </div>
                <button
                    type="button"
                    className="settings-button"
                    onClick={loadModels}
                    style={{ marginTop: 12 }}
                >
                    Retry
                </button>
            </div>
        );
    }

    return (
        <div style={{ maxWidth: 680 }}>
            <div style={{ fontSize: 14, fontWeight: 600, marginBottom: 12 }}>
                Local Ollama Models (12b and below)
            </div>

            <div className="settings-card">
                <div style={{ display: 'flex', gap: 8, marginBottom: 16 }}>
                    <button
                        type="button"
                        className="settings-button success"
                        onClick={handleAutoDetect}
                        style={{ flex: 1 }}
                    >
                        Auto-Detect Best Model
                    </button>
                    <button
                        type="button"
                        className="settings-button"
                        onClick={loadModels}
                        style={{ flex: 1 }}
                    >
                        Refresh List
                    </button>
                </div>

                <div style={{ fontSize: 11, opacity: 0.6, marginBottom: 12 }}>
                    Current Model: <strong>{currentModel || 'None selected'}</strong>
                </div>

                <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
                    {models.length === 0 ? (
                        <div style={{ opacity: 0.5, fontSize: 12, padding: 8 }}>
                            No models found. Run: <code>ollama pull qwen3.5:12b</code>
                        </div>
                    ) : (
                        models.map((model) => (
                            <div
                                key={model.name}
                                style={{
                                    padding: 12,
                                    background: currentModel === model.name ? '#9ece6a20' : '#ffffff05',
                                    border: `1px solid ${currentModel === model.name ? '#9ece6a' : '#ffffff10'}`,
                                    borderRadius: 4,
                                    cursor: 'pointer',
                                    transition: 'all 0.2s',
                                }}
                                onClick={() => handleSelectModel(model.name)}
                            >
                                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'start', marginBottom: 4 }}>
                                    <div style={{ fontWeight: 500, fontSize: 12 }}>
                                        {model.name}
                                        {model.recommended && <span style={{ marginLeft: 8, color: '#9ece6a' }}>⭐ Recommended</span>}
                                        {currentModel === model.name && <span style={{ marginLeft: 8, color: '#9ece6a' }}>✓ Active</span>}
                                    </div>
                                </div>
                                <div style={{ fontSize: 11, opacity: 0.6 }}>
                                    Context: {model.context_window / 1024}K tokens • Offline: {model.supports_12b_and_below ? '✓' : '✗'}
                                </div>
                            </div>
                        ))
                    )}
                </div>
            </div>

            <div className="settings-card" style={{ marginTop: 16 }}>
                <div style={{ fontSize: 12, opacity: 0.7, lineHeight: 1.6 }}>
                    <p style={{ margin: '0 0 8px 0' }}>
                        💡 <strong>Recommended for offline work:</strong> 12b models (qwen3.5:12b, mistral:7b)
                    </p>
                    <p style={{ margin: '0 0 8px 0' }}>
                        ⚡ <strong>Speed:</strong> ~12-15 tokens/sec on M1 CPU (faster with ANE acceleration)
                    </p>
                    <p style={{ margin: 0 }}>
                        🔗 <strong>Pull new models:</strong> Run <code style={{ background: '#ffffff10', padding: '2px 4px', borderRadius: 2 }}>ollama pull mistral:7b</code>
                    </p>
                </div>
            </div>
        </div>
    );
};

export default ModelSelectorPanel;
