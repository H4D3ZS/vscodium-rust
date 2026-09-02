import { useStore } from '../../store';
import { startDebugSession, stopDebugSession } from './sendDapRequest';

export interface LaunchConfig {
    name: string;
    type?: string;
    request?: string;
    program?: string;
    args?: string[];
    cwd?: string;
    env?: Record<string, string>;
    adapter_path?: string;
}

export async function startLaunchConfig(config: LaunchConfig): Promise<void> {
    await startDebugSession({ ...config } as Record<string, unknown>);
    useStore.getState().setDebugging(true, config.name);
    const { pushBreakpointsToAdapter } = await import('./bootstrapDebugRuntime');
    await pushBreakpointsToAdapter();
}

export async function stopLaunchConfig(): Promise<void> {
    await stopDebugSession();
}
