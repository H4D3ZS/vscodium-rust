import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { initMonaco } from './monaco_setup';
import { installOllamaGuard } from './airi/ollama-guard';
import { scheduleDeferredInit } from './memory_budget';

// Install Ollama model-fallback interceptor before any AIRI subsystem
// has a chance to fire its first /api/generate request.
installOllamaGuard();

initMonaco();

// Kokoro/VRM error filters — only needed when AIRI avatar loads.
scheduleDeferredInit(() => { void import('./airi/kokoro-worker-wrapper'); }, 8_000);

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
