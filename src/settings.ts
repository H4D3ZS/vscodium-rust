import { invoke } from './tauri_bridge.ts';
import { addTab } from './workspace.ts';

export async function openSettings() {
    const useStore = (window as any).useStore;
    if (useStore) {
        useStore.getState().openSettings();
    }
}

export function initSettings() {
    const settingsBtn = document.getElementById("activity-settings");
    if (settingsBtn) {
        settingsBtn.onclick = () => openSettings();
    }
}
