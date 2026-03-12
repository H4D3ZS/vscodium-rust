import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import { useStore } from './store';

(window as any).useStore = useStore;

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
    <React.StrictMode>
        <App />
    </React.StrictMode>,
);
