import { logcatRepository } from '../../infrastructure/android/TauriLogcatRepository';

export const startLogcat = (device?: string, filter?: string) => logcatRepository.start(device, filter);
export const stopLogcat = () => logcatRepository.stop();
export const logcatStatus = () => logcatRepository.status();
