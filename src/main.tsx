import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { useStore } from './store';
import { initMonaco } from './monaco_setup';
import './airi/kokoro-worker-wrapper'; // Suppress expected Kokoro errors

initMonaco();
(window as any).useStore = useStore;

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
