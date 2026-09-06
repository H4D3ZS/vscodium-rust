import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { initMonaco } from './monaco_setup';
import { scheduleDeferredInit } from './memory_budget';
import { hydrate as hydrateUiSettings } from './infrastructure/SettingsRepository';

// HeroUI lazy-loaded via App.tsx to reduce initial bundle size

// Potato/low-end detection — flips on a CSS kill-switch that disables GPU-heavy
// effects (backdrop blur, animations, shadows) which balloon renderer/GPU memory
// on old integrated GPUs and RAM-starved machines. Auto-on for <=4 cores / <=4 GB,
// or force via localStorage 'lowEndMode' = '1' / '0'.
try {
    const forced = (typeof localStorage !== 'undefined') ? localStorage.getItem('lowEndMode') : null;
    const auto = ((navigator.hardwareConcurrency || 8) <= 4)
        || (((navigator as any).deviceMemory || 8) <= 4);
    if (forced === '1' || (forced !== '0' && auto)) {
        document.body.classList.add('low-end');
    }
} catch { /* noop */ }

initMonaco();

// Load ui_settings.json into the in-memory settings cache (and run the
// one-time localStorage migration) before panels read preferences.
void hydrateUiSettings();

// Warm up the selected inference backend — keeps the main model in GPU/VRAM
// so the first agent turn is fast. Backend-aware: only hits Ollama (:13305)
// when Ollama is the active backend; otherwise targets Lemonade (:13305) or
// whichever custom URL the user configured. Avoids the old hardcoded
// `:13305/api/generate` 404 that fired at boot no matter which backend was
// selected.
scheduleDeferredInit(() => {
    const backend = (() => {
        try { return localStorage.getItem('inferenceBackend') || 'ollama'; } catch { return 'ollama'; }
    })();
    const trim = (s: string) => (s || '').trim().replace(/\/+$/, '');

    if (backend === 'lemonade') {
        // Lemonade exposes the OpenAI-compat surface (`/v1/...`). Warm it with a
        // lightweight /v1/models probe to prime the connection. Do NOT send a
        // chat completion here — an empty `content: ""` causes a 500 on some
        // Lemonade backends, and the model may not be downloaded yet.
        let base = 'http://localhost:13305';
        try { base = trim(localStorage.getItem('provider.lemonade.url')) || base; } catch { /* ignore */ }
        // Normalize: the stored base often already ends in /v1 (it's the documented
        // client URL), which would warm `/v1/v1/models`.
        base = base.replace(/\/v1$/, '');
        // Only warm a LOCAL server. A cloud (https, non-localhost) Lemonade sits
        // behind the JWT gate — an unauthenticated probe just 401-spams the console.
        const isLocal = /^https?:\/\/(127\.0\.0\.1|localhost)/i.test(base);
        if (isLocal) fetch(`${base}/v1/models`).catch(() => {});
        return;
    }

    // Ollama (default) — keep the original keep-alive warmup.
    let base = 'http://localhost:13305';
    try { base = trim(localStorage.getItem('ollamaUrl')) || base; } catch { /* ignore */ }
    fetch(`${base}/api/v1/models`).catch(() => {});
    const agentModel = localStorage.getItem('agentModel') || '';
    const modelTag = agentModel.includes('|') ? agentModel.split('|').slice(1).join('|') : agentModel;
    if (modelTag) {
        fetch(`${base}/api/generate`, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ model: modelTag, prompt: '', stream: false, keep_alive: '60m' }),
        }).catch(() => {});
    }
}, 5_000);

// Kokoro/VRM error filters — only needed when AIRI avatar loads.
scheduleDeferredInit(() => { void import('./audio/kokoro-worker-wrapper'); }, 8_000);

// HeroUI reads its dark-theme CSS variables from a `.dark` ancestor. Put it on
// <html> so components portaled to <body> (modals, dropdowns, tooltips) are themed
// too. Harmless to the legacy CSS, which keys off --vscode-* vars, not `.dark`.
document.documentElement.classList.add('dark');

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
