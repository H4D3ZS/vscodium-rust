import { invoke } from './tauri_bridge.ts';
import { useStore } from './store.ts';

export async function initExtensions() {
    console.log("DEBUG: initExtensions called");
    try {
        await invoke("ext_host_init");
        console.log("Extension host initialized");
    } catch (err) {
        console.error("Failed to initialize extension host:", err);
    }

    // Refresh everything via the store
    const { refreshInstalledExtensions, refreshPopularExtensions } = useStore.getState();
    await refreshInstalledExtensions();
    await refreshPopularExtensions();
}

export async function refreshInstalledExtensions() {
    await useStore.getState().refreshInstalledExtensions();
}

// Keep helper for manual installation if needed by other legacy parts
export async function installExtension(publisher: string, name: string, version: string) {
    try {
        await invoke("install_extension", { publisher, name, version });
        await refreshInstalledExtensions();
        return true;
    } catch (err) {
        console.error("Installation failed:", err);
        throw err;
    }
}
