import { useState, useEffect, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';

export interface UseInvokeResult<T> {
    data: T | null;
    loading: boolean;
    error: string | null;
    refetch: () => void;
}

export function useInvoke<T>(cmd: string, args?: Record<string, any>): UseInvokeResult<T> {
    const [data, setData] = useState<T | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [tick, setTick] = useState(0);

    const refetch = useCallback(() => setTick(t => t + 1), []);

    useEffect(() => {
        setLoading(true);
        setError(null);
        invoke<T>(cmd, args ?? {})
            .then(result => { setData(result); setLoading(false); })
            .catch(err => { setError(String(err)); setLoading(false); });
    // eslint-disable-next-line react-hooks/exhaustive-deps
    }, [cmd, tick]);

    return { data, loading, error, refetch };
}
