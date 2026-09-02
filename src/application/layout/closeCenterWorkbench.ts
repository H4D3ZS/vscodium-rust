import { useStore } from '../../store';

export function closeCenterWorkbench(): void {
    useStore.getState().setLayoutMode?.('editor');
}
