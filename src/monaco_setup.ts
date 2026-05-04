import { loader } from '@monaco-editor/react';

// This allows Monaco to work in production environments by loading from local scripts
// instead of the default jsdelivr CDN which is often blocked by CSP.
export function initMonaco() {
    // We use LOCAL assets for absolute reliability and speed.
    // This eliminates "white screen" issues caused by CDN load failures.
    loader.config({
        paths: {
            vs: '/vs' // Points to src/public/vs in dev and /vs in built app
        }
    });
}

