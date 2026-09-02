/**
 * Bounded array operations for Zustand stores.
 *
 * Zustand state must be plain serializable objects, so we can't use class
 * instances (BoundedStack) directly in store state. These helpers encapsulate
 * the same eviction semantics as a clean function API.
 *
 * Every capacity constant lives here — no magic numbers scattered across slices.
 */

// ── Capacity constants ────────────────────────────────────────────────────────
export const MAX_AGENT_STEPS_PER_MSG = 100;
export const MAX_TRAJECTORY_EVENTS = 200;
export const MAX_TOOL_BLOCKS = 25;
export const MAX_TOOL_OUTPUT_LINES = 40;
export const MAX_COMPLETED_TASKS = 20;
export const MAX_TAB_HISTORY = 100;
export const MAX_PENDING_CHANGES = 30;
export const MAX_PENDING_CHANGE_CONTENT = 50_000;
export const MAX_THREAD_COUNT = 5;
export const MAX_MITM_LOGS = 100;
export const MAX_DEBUG_OUTPUT = 200;
export const MAX_RAW_BUFFER = 32_000;
export const MAX_MESSAGE_CONTENT = 80_000;

// ── Helpers ───────────────────────────────────────────────────────────────────

/** Append an item, evicting the oldest if over capacity. Returns a new array. */
export function boundedPush<T>(arr: T[], item: T, capacity: number): T[] {
    if (arr.length < capacity) return [...arr, item];
    const result = arr.slice(arr.length - capacity + 1);
    result.push(item);
    return result;
}

/** Append many items with a per-item capacity check. */
export function boundedPushMany<T>(arr: T[], items: T[], capacity: number): T[] {
    let result = [...arr, ...items];
    if (result.length > capacity) result = result.slice(result.length - capacity);
    return result;
}

/** Keep only the last N items. */
export function boundedTail<T>(arr: T[], capacity: number): T[] {
    if (arr.length <= capacity) return arr;
    return arr.slice(arr.length - capacity);
}
