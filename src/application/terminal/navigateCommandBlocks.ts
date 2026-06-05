import { getTerminalManager } from './getTerminalManager';

export async function scrollToPreviousCommand(instanceId?: string): Promise<void> {
    const mgr = await getTerminalManager();
    const id = instanceId ?? mgr.getActiveGroup()?.activeInstanceId;
    if (id) mgr.scrollToPreviousCommand(id);
}

export async function scrollToNextCommand(instanceId?: string): Promise<void> {
    const mgr = await getTerminalManager();
    const id = instanceId ?? mgr.getActiveGroup()?.activeInstanceId;
    if (id) mgr.scrollToNextCommand(id);
}

export async function rerunLastCommand(instanceId?: string): Promise<void> {
    const mgr = await getTerminalManager();
    const id = instanceId ?? mgr.getActiveGroup()?.activeInstanceId;
    if (id) mgr.rerunLastCommand(id);
}
