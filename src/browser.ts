import { invoke } from './tauri_bridge.ts';

export interface BrowserActionResult {
    success: boolean;
    message: string;
    data?: string;
}

export async function browserOpen(): Promise<BrowserActionResult> {
    try {
        const message = await invoke<string>("browser_open");
        return { success: true, message };
    } catch (e) {
        return { success: false, message: String(e) };
    }
}

export async function browserNavigate(url: string): Promise<BrowserActionResult> {
    try {
        const message = await invoke<string>("browser_navigate", { url });
        return { success: true, message };
    } catch (e) {
        return { success: false, message: String(e) };
    }
}

export async function browserScreenshot(): Promise<BrowserActionResult> {
    try {
        const data = await invoke<string>("browser_screenshot");
        return { success: true, message: "Screenshot captured", data };
    } catch (e) {
        return { success: false, message: String(e) };
    }
}

export async function browserClose(): Promise<BrowserActionResult> {
    try {
        const message = await invoke<string>("browser_close");
        return { success: true, message };
    } catch (e) {
        return { success: false, message: String(e) };
    }
}
