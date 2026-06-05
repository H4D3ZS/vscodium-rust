import { useStore } from '../../store';
import type { PendingChange } from '../../store/types';

/** Composer-style navigation between pending file changes (Alt+J / Alt+K). */
export function navigatePendingChange(direction: 'next' | 'prev'): void {
    const st = useStore.getState();
    const changes: PendingChange[] = st.pendingChanges || [];
    if (changes.length === 0) return;

    const focusedId = st.focusedHunkId;
    const idx = focusedId ? changes.findIndex((c) => c.id === focusedId) : -1;

    let next: PendingChange | undefined;
    if (direction === 'next') {
        next = changes[idx + 1] ?? changes[0];
    } else {
        next = changes[idx <= 0 ? changes.length - 1 : idx - 1];
    }

    if (next) {
        st.setFocusedHunk(next.id);
        // Open the file so the gutter diff is visible.
        void st.openFile(next.path);
    }
}

export function acceptFocusedPendingChange(): void {
    const st = useStore.getState();
    const id = st.focusedHunkId;
    if (!id) return;
    const change = st.pendingChanges.find((c) => c.id === id);
    if (change) void st.acceptPendingChange(change.id);
}

export function rejectFocusedPendingChange(): void {
    const st = useStore.getState();
    const id = st.focusedHunkId;
    if (!id) return;
    const change = st.pendingChanges.find((c) => c.id === id);
    if (change) {
        void st.rejectPendingChange(change.id);
        st.setFocusedHunk(null);
    }
}
