import { listen as bridgeListen } from '../tauri_bridge';

export function listen(event: string, handler: (event: any) => void): Promise<() => void> {
    return bridgeListen(event, handler);
}

