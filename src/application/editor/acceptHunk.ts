import { computeDiffBlocks, patchContentAccepted } from '../../domain/editor/DiffService';
import { fileRepository } from '../../infrastructure/editor/TauriFileRepository';
import { patchRepository } from '../../infrastructure/editor/TauriPatchRepository';
import { useStore } from '../../store';

/**
 * Accept a single diff hunk within a pending change.
 * When all hunks are accepted, writes the file and clears the pending entry.
 */
export async function acceptHunk(changeId: string, hunkId: string): Promise<void> {
    const st = useStore.getState();
    const change = st.pendingChanges.find((c) => c.id === changeId);
    if (!change) return;

    const original = change.originalContent || change.oldContent || '';
    const proposed = change.proposedContent || change.newContent || '';
    const accepted = [...(change.acceptedHunkIds || [])];
    if (accepted.includes(hunkId)) return;
    accepted.push(hunkId);

    const allHunks = computeDiffBlocks(original, proposed);
    const mergedContent = patchContentAccepted(original, proposed, accepted);

    useStore.setState((s) => ({
        pendingChanges: s.pendingChanges.map((c) =>
            c.id === changeId
                ? { ...c, acceptedHunkIds: accepted, newContent: mergedContent }
                : c,
        ),
    }));

    if (accepted.length >= allHunks.length) {
        try {
            if (change.applied) {
                await fileRepository.write(change.path, mergedContent);
            } else {
                await patchRepository.acceptPatch(change.path);
            }
            const tab = st.tabs.find((t) => t.path === change.path);
            if (tab) {
                const disk = await fileRepository.read(change.path);
                st.updateTabContent(tab.id, disk);
            }
            useStore.setState((s) => ({
                pendingChanges: s.pendingChanges.filter((c) => c.id !== changeId),
                focusedHunkId: s.focusedHunkId === changeId ? null : s.focusedHunkId,
            }));
            await st.refreshFileTree();
        } catch (e) {
            console.error('[editor] acceptHunk finalize failed:', e);
        }
    }
}
