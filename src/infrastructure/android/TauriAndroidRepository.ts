import { invoke } from '../../tauri_bridge';
import type { AndroidDevice, AndroidSdkConfig, IAndroidRepository } from '../../domain/android/IAndroidRepository';

export class TauriAndroidRepository implements IAndroidRepository {
    async getConfig(): Promise<AndroidSdkConfig> {
        return invoke<AndroidSdkConfig>('get_android_config');
    }

    async setSdkPath(path: string): Promise<void> {
        await invoke('set_android_sdk_path', { path });
    }

    async listDevices(): Promise<AndroidDevice[]> {
        return invoke<AndroidDevice[]>('adb_list_devices');
    }

    async listAvds(): Promise<string[]> {
        return invoke<string[]>('adb_list_emulators');
    }

    async spawnEmulator(avd: string): Promise<void> {
        await invoke('spawn_emulator', { avd });
    }

    async setActiveDevice(device: string): Promise<void> {
        await invoke('set_active_device', { device });
    }

    async installAndRun(apkPath: string, packageName?: string, activity?: string): Promise<void> {
        await invoke('adb_install_and_run', {
            apkPath,
            package: packageName,
            activity,
        });
    }
}

export const androidRepository = new TauriAndroidRepository();
