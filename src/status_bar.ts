import { invoke } from './tauri_bridge.ts';

export function initStatusBar() {
    const modelSelector = document.getElementById("model-selector");
    const deviceSelector = document.getElementById("device-selector");

    if (modelSelector) {
        modelSelector.onclick = () => switchModel();
    }

    if (deviceSelector) {
        deviceSelector.onclick = () => switchDevice();
    }

    // Start stats refresh
    setInterval(updateStats, 2000);
}

async function switchModel() {
    const models = ["gpt-4o", "gpt-4-turbo", "claude-3-5-sonnet", "gemini-1.5-pro", "grok-beta"];
    const current = document.getElementById("current-model")?.innerText || "gpt-4o";

    // Simple cycle for now, ideally a context menu
    const nextIndex = (models.indexOf(current.toLowerCase()) + 1) % models.length;
    const nextModel = models[nextIndex];

    const modelSpan = document.getElementById("current-model");
    if (modelSpan) modelSpan.innerText = nextModel.toUpperCase();

    try {
        await invoke("set_ai_model", { model: nextModel });
    } catch (e) {
        console.error("Failed to set model:", e);
    }
}

async function switchDevice() {
    try {
        const devices = await invoke<string[]>("adb_list_devices");
        if (devices.length === 0) {
            alert("No Android devices found via ADB.");
            return;
        }

        const device = prompt(`Select Device:\n${devices.join("\n")}`, devices[0]);
        if (device) {
            const deviceSpan = document.getElementById("current-device");
            if (deviceSpan) deviceSpan.innerText = device;
            await invoke("set_active_device", { device });
        }
    } catch (e) {
        console.error("Failed to list devices:", e);
    }
}

async function updateStats() {
    try {
        const stats = await invoke<any>("get_process_stats");
        const perfSpan = document.getElementById("perf-stats");
        if (perfSpan && stats) {
            perfSpan.innerText = `RAM: ${(stats.memory_usage / 1024 / 1024).toFixed(1)}MB | CPU: ${stats.cpu_usage.toFixed(1)}%`;
        }
    } catch (e) {
        // Silently fail stats if backend not ready
    }
}
