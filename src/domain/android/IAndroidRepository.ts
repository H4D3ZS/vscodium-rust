export interface AndroidDevice {
    id: string;
    state: string;
}

export interface AndroidSdkConfig {
    sdk_path?: string | null;
    adb_found: boolean;
    emulator_found?: boolean;
}

export interface IAndroidRepository {
    getConfig(): Promise<AndroidSdkConfig>;
    setSdkPath(path: string): Promise<void>;
    listDevices(): Promise<AndroidDevice[]>;
    listAvds(): Promise<string[]>;
    spawnEmulator(avd: string): Promise<void>;
    setActiveDevice(device: string): Promise<void>;
    installAndRun(apkPath: string, packageName?: string, activity?: string): Promise<void>;
}
