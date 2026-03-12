import { invoke } from './tauri_bridge.ts';

export async function initMobile() {
    const mobileContent = document.getElementById("mobile-content");

    if (mobileContent) {
        mobileContent.innerHTML = `
            <div class="mobile-config-section" style="padding: 10px; border-bottom: 1px solid var(--vscode-panel-border);">
                <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; color: var(--vscode-descriptionForeground); text-transform: uppercase;">Android SDK Configuration</div>
                <div style="display: flex; gap: 8px; margin-bottom: 8px;">
                    <input type="text" id="android-sdk-path" placeholder="SDK Path (e.g. ~/Library/Android/sdk)" style="flex: 1; background: var(--vscode-editor-background); border: 1px solid var(--vscode-panel-border); color: var(--vscode-editor-foreground); padding: 4px 8px; font-size: 11px; border-radius: 4px;">
                    <button id="set-sdk-btn" style="background: var(--vscode-button-background); color: var(--vscode-button-foreground); border: none; padding: 4px 10px; font-size: 11px; border-radius: 4px; cursor: pointer;">Set</button>
                </div>
                <div id="sdk-status" style="font-size: 10px; opacity: 0.8;">Checking SDK...</div>
            </div>
            
            <div class="mobile-devices-section" style="padding: 10px;">
                <div style="display: flex; justify-content: space-between; align-items: center; margin-bottom: 8px;">
                    <div style="font-size: 11px; font-weight: 600; color: var(--vscode-descriptionForeground); text-transform: uppercase;">Connected Devices (Android)</div>
                </div>
                <div id="android-device-list" style="display: flex; flex-direction: column; gap: 4px;"></div>
            </div>

            <div class="mobile-emulators-section" style="padding: 10px; border-top: 1px solid var(--vscode-panel-border);">
                <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; color: var(--vscode-descriptionForeground); text-transform: uppercase;">Android Emulators</div>
                <div id="emulator-list" style="display: flex; flex-direction: column; gap: 4px;"></div>
            </div>

            <div class="mobile-ios-section" style="padding: 10px; border-top: 1px solid var(--vscode-panel-border);">
                <div style="font-size: 11px; font-weight: 600; margin-bottom: 8px; color: var(--vscode-descriptionForeground); text-transform: uppercase;">iOS Simulators</div>
                <div id="ios-list" style="display: flex; flex-direction: column; gap: 4px;">
                    <div class="ios-item hoverable" id="vphone-ios-item" style="padding: 6px 10px; background: rgba(255,255,255,0.03); border-radius: 4px; font-size: 12px; display: flex; align-items: center; gap: 8px; cursor: pointer;">
                        <i class="codicon codicon-device-mobile"></i> <span>Virtual iPhone Emulator 26.3.1 JB</span>
                        <span id="vphone-status" style="font-size: 10px; opacity: 0.6; margin-left: auto;">Ready</span>
                    </div>
                </div>
            </div>
        `;

        const vphoneItem = document.getElementById("vphone-ios-item");
        if (vphoneItem) {
            vphoneItem.onclick = async () => {
                const status = document.getElementById("vphone-status");
                if (status) status.innerText = "Launching...";
                try {
                    await invoke('launch_vphone');
                    await invoke('set_active_device', { device: "Virtual iPhone Emulator 26.3.1 JB" });
                    if (status) status.innerText = "Running";
                    resetSelections();
                    vphoneItem.style.outline = '1px solid var(--vscode-focusBorder)';
                } catch (err) {
                    console.error("Failed to launch VPhone:", err);
                    if (status) {
                        status.innerText = "Error";
                        status.style.color = "#f87171";
                    }
                }
            };
        }

        const sdkInput = document.getElementById("android-sdk-path") as HTMLInputElement;
        const setSdkBtn = document.getElementById("set-sdk-btn");
        const refreshBtn = document.getElementById("refresh-mobile");

        if (setSdkBtn) {
            setSdkBtn.onclick = async () => {
                const path = sdkInput.value.trim();
                if (path) {
                    await invoke('set_android_sdk_path', { path });
                    await refreshMobileView();
                }
            };
        }

        if (refreshBtn) {
            refreshBtn.onclick = () => refreshMobileView();
        }
    }

    await refreshMobileView();
}

export async function refreshMobileView() {
    const sdkInput = document.getElementById("android-sdk-path") as HTMLInputElement;
    const sdkStatus = document.getElementById("sdk-status");
    const deviceList = document.getElementById("android-device-list");
    const emulatorList = document.getElementById("emulator-list");

    try {
        const config = await invoke<any>('get_android_config');
        if (sdkInput) sdkInput.value = config.sdk_path || "";
        if (sdkStatus) {
            sdkStatus.innerHTML = config.adb_found 
                ? `<span style="color: #4ade80;"><i class="codicon codicon-check"></i> ADB Found</span>`
                : `<span style="color: #f87171;"><i class="codicon codicon-error"></i> ADB not found</span>`;
        }

        const devices = await invoke<string[]>('adb_list_devices');
        if (deviceList) {
            deviceList.innerHTML = devices.length > 0 
                ? devices.map(d => `<div class="device-item hoverable" style="padding: 6px 10px; background: rgba(255,255,255,0.03); border-radius: 4px; font-size: 12px; display: flex; align-items: center; gap: 8px; cursor: pointer;">
                    <i class="codicon codicon-device-mobile"></i> <span>${d}</span>
                  </div>`).join("")
                : `<div style="font-size: 11px; opacity: 0.6; padding: 5px;">No devices connected</div>`;
            
            deviceList.querySelectorAll('.device-item').forEach((item, index) => {
                const el = item as HTMLElement;
                el.onclick = async () => {
                   await invoke('set_active_device', { device: devices[index] });
                   resetSelections();
                   el.style.outline = '1px solid var(--vscode-focusBorder)';
                };
            });
        }

        const emulators = await invoke<string[]>('adb_list_emulators');
        if (emulatorList) {
            emulatorList.innerHTML = emulators.length > 0
                ? emulators.map(e => `<div class="emulator-item hoverable" style="padding: 6px 10px; background: rgba(255,255,255,0.03); border-radius: 4px; font-size: 12px; display: flex; align-items: center; gap: 8px; cursor: pointer;">
                    <i class="codicon codicon-play"></i> <span>${e}</span>
                    <span class="emulator-status" style="font-size: 10px; opacity: 0.6; margin-left: auto;"></span>
                    <i class="codicon codicon-link-external" style="margin-left: 8px; opacity: 0.4;"></i>
                  </div>`).join("")
                : `<div style="font-size: 11px; opacity: 0.6; padding: 5px;">No emulators found</div>`;

            emulatorList.querySelectorAll('.emulator-item').forEach((item, index) => {
                const el = item as HTMLElement;
                el.onclick = async () => {
                    const status = el.querySelector('.emulator-status') as HTMLElement;
                    if (status) status.innerHTML = "Starting...";
                    try {
                        await invoke('spawn_emulator', { avd: emulators[index] });
                        // Auto-select the device (it will eventually show up in adb_list_devices, but for now we iframe it via name matching)
                        await invoke('set_active_device', { device: emulators[index] });
                        resetSelections();
                        el.style.outline = '1px solid var(--vscode-focusBorder)';
                        
                        if (status) status.innerHTML = "Launching...";
                        setTimeout(() => { if (status) status.innerHTML = ""; }, 3000);
                    } catch (err) {
                        console.error("Failed to spawn emulator:", err);
                        if (status) {
                            status.innerHTML = "Error";
                            status.style.color = "#f87171";
                        }
                    }
                };
            });
        }
    } catch (err) {
        console.error("Mobile refresh failed:", err);
    }
}

function resetSelections() {
    document.querySelectorAll('.device-item, .emulator-item, .ios-item').forEach(e => {
        (e as HTMLElement).style.outline = 'none';
    });
}
