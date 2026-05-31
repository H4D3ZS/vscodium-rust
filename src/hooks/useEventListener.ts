import { useEffect, useRef } from 'react';

export function useEventListener<K extends keyof WindowEventMap>(
    event: K,
    handler: (e: WindowEventMap[K]) => void,
    deps: any[] = [],
): void {
    const savedHandler = useRef(handler);
    useEffect(() => { savedHandler.current = handler; });
    useEffect(() => {
        const fn = (e: WindowEventMap[K]) => savedHandler.current(e);
        window.addEventListener(event, fn as EventListener);
        return () => window.removeEventListener(event, fn as EventListener);
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [event, ...deps]);
}
