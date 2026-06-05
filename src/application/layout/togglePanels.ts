import { useStore } from '../../store';

export function setBottomPanelOpen(open: boolean): void {
    useStore.setState({ isBottomPanelOpen: open });
}

export function openTerminalPanel(): void {
    useStore.setState({ isBottomPanelOpen: true, activePanelTab: 'TERMINAL' });
}

export function setChatSidebarOpen(open: boolean): void {
    useStore.setState({ isRightSidebarOpen: open });
}

export function setComposerOpen(open: boolean): void {
    useStore.getState().toggleComposer(open);
}
