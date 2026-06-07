import { useStore } from '../../store';

/** Open PyTorch ML Studio full panel (activity bar layout). */
export function openPyTorchStudio(): void {
    useStore.getState().setLayoutMode?.('ml-studio');
}

/** Activity bar toggle — second click returns to the editor. */
export function togglePyTorchStudio(): void {
    const store = useStore.getState();
    store.setLayoutMode?.(store.layoutMode === 'ml-studio' ? 'editor' : 'ml-studio');
}

export function closePyTorchStudio(): void {
    useStore.getState().setLayoutMode?.('editor');
}
