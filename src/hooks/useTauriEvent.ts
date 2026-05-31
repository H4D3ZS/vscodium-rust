import { useEffect } from 'react';
import { listen } from '@tauri-apps/api/event';

export function useTauriEvent<T>(
    event: string,
    handler: (payload: T) => void,
): void {
    useEffect(() => {
        let unlisten: (() => void) | null = null;
        listen<T>(event, (e) => handler(e.payload)).then(fn => { unlisten = fn; });
        return () => { unlisten?.(); };
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [event]);
}
