import { androidRepository } from '../../infrastructure/android/TauriAndroidRepository';

export const refreshAndroidDevices = () => androidRepository.listDevices();
export const refreshAndroidAvds = () => androidRepository.listAvds();
export const spawnAndroidEmulator = (avd: string) => androidRepository.spawnEmulator(avd);
export const setActiveAndroidDevice = (device: string) => androidRepository.setActiveDevice(device);
export const getAndroidConfig = () => androidRepository.getConfig();
