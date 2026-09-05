/**
 * BoundedStack — O(1) push with automatic eviction of oldest entries.
 *
 * Unlike ad-hoc `.slice(-N)` calls scattered across the codebase, this
 * encapsulates the eviction policy in one place with clear semantics:
 *   - `push()` appends and evicts the oldest entry if over capacity
 *   - `peek()` returns the newest entry without removal
 *   - `toArray()` returns entries in insertion order (oldest first)
 *   - `clear()` resets the stack
 *   - `size` is the current count
 *
 * Used by: agentSteps, agentToolBlocks, tabHistory, mitmLogs, debugOutput, etc.
 */
export class BoundedStack<T> {
    private items: T[];
    private capacity: number;

    constructor(capacity: number, initial: T[] = []) {
        this.capacity = capacity;
        this.items = initial.length > capacity ? initial.slice(-capacity) : [...initial];
    }

    get size(): number {
        return this.items.length;
    }

    push(item: T): void {
        this.items.push(item);
        if (this.items.length > this.capacity) {
            this.items.splice(0, this.items.length - this.capacity);
        }
    }

    pushMany(items: T[]): void {
        for (const item of items) this.push(item);
    }

    peek(): T | undefined {
        return this.items[this.items.length - 1];
    }

    peekN(n: number): T[] {
        return this.items.slice(-n);
    }

    map<U>(fn: (item: T, index: number) => U): U[] {
        return this.items.map(fn);
    }

    filter(fn: (item: T, index: number) => boolean): T[] {
        return this.items.filter(fn);
    }

    find(fn: (item: T, index: number) => boolean): T | undefined {
        return this.items.find(fn);
    }

    reduce<U>(fn: (acc: U, item: T, index: number) => U, initial: U): U {
        return this.items.reduce(fn, initial);
    }

    get length(): number {
        return this.items.length;
    }

    toArray(): T[] {
        return [...this.items];
    }

    [Symbol.iterator](): IterableIterator<T> {
        return this.items[Symbol.iterator]();
    }

    clear(): void {
        this.items.length = 0;
    }

    /** Immutable update: returns a new BoundedStack with the item appended. */
    with(item: T): BoundedStack<T> {
        const next = new BoundedStack<T>(this.capacity, this.items);
        next.push(item);
        return next;
    }
}
