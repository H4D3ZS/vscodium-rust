import { invoke } from '../tauri_bridge';

export interface SystemProfile {
    total_ram_gb: number;
    lite_mode: boolean;
    source: string;
    threshold_gb: number;
}

let cached: SystemProfile | null = null;
let inflight: Promise<SystemProfile> | null = null;

/**
 * RAM-keyed runtime profile ("potato mode"), detected once by the Rust host.
 * lite_mode=true on <9GB machines (or HADES_LITE=1): heavy bootstraps must
 * defer or skip. Falls back to full mode if the backend is unreachable.
 */
export function getSystemProfile(): Promise<SystemProfile> {
    if (cached) return Promise.resolve(cached);
    if (inflight) return inflight;
    const fallback: SystemProfile = { total_ram_gb: 0, lite_mode: false, source: 'fallback', threshold_gb: 9 };
    inflight = invoke<SystemProfile>('get_system_profile')
        .then((p) => {
            // Mock bridge outside the Tauri shell resolves null — treat as full mode.
            cached = p ?? fallback;
            return cached;
        })
        .catch(() => {
            cached = fallback;
            return fallback;
        })
        .finally(() => {
            inflight = null;
        });
    return inflight;
}

export async function isLiteMode(): Promise<boolean> {
    return (await getSystemProfile()).lite_mode;
}
