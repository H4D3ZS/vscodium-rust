/**
 * LRUMap — Least-Recently-Used cache with O(1) get/set and automatic eviction.
 *
 * Unlike ad-hoc `Map.size > N → delete(firstKey)` scattered across the codebase,
 * this encapsulates LRU eviction in one reusable, testable class.
 *
 * - `get(key)` returns the value and marks it as recently used
 * - `set(key, value)` inserts or updates and evicts the LRU entry if over capacity
 * - `has(key)` checks existence without affecting recency
 * - `delete(key)` removes an entry
 * - `size` is the current count
 * - `keys()`, `values()`, `entries()` iterate in LRU order (oldest first)
 */
export class LRUMap<K, V> {
    private map: Map<K, V>;
    private capacity: number;

    constructor(capacity: number, entries?: [K, V][]) {
        this.capacity = capacity;
        this.map = new Map(entries);
        this.evictToCap();
    }

    get size(): number {
        return this.map.size;
    }

    get(key: K): V | undefined {
        const val = this.map.get(key);
        if (val !== undefined) {
            this.map.delete(key);
            this.map.set(key, val);
        }
        return val;
    }

    has(key: K): boolean {
        return this.map.has(key);
    }

    set(key: K, value: V): void {
        if (this.map.has(key)) {
            this.map.delete(key);
        }
        this.map.set(key, value);
        this.evictToCap();
    }

    delete(key: K): boolean {
        return this.map.delete(key);
    }

    *keys(): IterableIterator<K> {
        yield* this.map.keys();
    }

    *values(): IterableIterator<V> {
        yield* this.map.values();
    }

    *entries(): IterableIterator<[K, V]> {
        yield* this.map.entries();
    }

    forEach(callback: (value: V, key: K, map: Map<K, V>) => void): void {
        this.map.forEach(callback);
    }

    clear(): void {
        this.map.clear();
    }

    /** Snapshot as plain object (for Zustand state serialization). */
    toObject(): Record<string, V> {
        const out: Record<string, V> = {};
        for (const [k, v] of this.map) {
            out[k as any] = v;
        }
        return out;
    }

    private evictToCap(): void {
        while (this.map.size > this.capacity) {
            const oldest = this.map.keys().next().value;
            if (oldest !== undefined) this.map.delete(oldest);
        }
    }
}
