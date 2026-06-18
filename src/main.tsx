import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { initMonaco } from './monaco_setup';
import { installOllamaGuard } from './airi/ollama-guard';
import { scheduleDeferredInit } from './memory_budget';
import { hydrate as hydrateUiSettings } from './infrastructure/SettingsRepository';

// Install Ollama model-fallback interceptor before any AIRI subsystem
// has a chance to fire its first /api/generate request.
installOllamaGuard();

initMonaco();

// Load ui_settings.json into the in-memory settings cache (and run the
// one-time localStorage migration) before panels read preferences.
void hydrateUiSettings();

// Warm up Ollama — keeps main model in GPU so first agent turn is fast
scheduleDeferredInit(() => {
    // Ping Ollama to keep connection alive
    fetch('http://localhost:11434/api/tags').catch(() => {});
    // Pre-load the agent model (non-blocking, takes 30-60s first time)
    const agentModel = localStorage.getItem('agentModel') || '';
    const modelTag = agentModel.includes('|') ? agentModel.split('|').slice(1).join('|') : agentModel;
    if (modelTag) {
        fetch('http://localhost:11434/api/generate', {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ model: modelTag, prompt: '', stream: false, keep_alive: '60m' }),
        }).catch(() => {});
    }
}, 5_000);

// Kokoro/VRM error filters — only needed when AIRI avatar loads.
scheduleDeferredInit(() => { void import('./airi/kokoro-worker-wrapper'); }, 8_000);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
